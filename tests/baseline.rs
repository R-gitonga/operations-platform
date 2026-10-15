//! Run explicitly against a disposable PostgreSQL server; SQLx creates a test DB.
#![allow(dead_code, unused_imports, unused_variables)]
#[path = "../src/config.rs"]
mod config;
#[path = "../src/database/mod.rs"]
mod database;
#[path = "../src/errors/mod.rs"]
mod errors;
#[path = "../src/models/mod.rs"]
mod models;
#[path = "../src/repositories/mod.rs"]
mod repositories;
#[path = "../src/services/mod.rs"]
mod services;

use serde_json::json;
use sqlx::{Executor, PgPool};

fn config() -> config::Config {
    config::Config {
        jwt_secret: "test-only".into(),
        jwt_expiration_hours: 1,
        smtp_host: "localhost".into(),
        smtp_port: 1,
        smtp_username: "test".into(),
        smtp_password: "test".into(),
        smtp_from: "test@example.invalid".into(),
        system_notification_name: "Test".into(),
        system_notification_email: "test@example.invalid".into(),
        frontend_base_url: "http://localhost".into(),
    }
}

#[sqlx::test(migrations = "./migrations")]
#[ignore = "requires DATABASE_URL pointing to a disposable PostgreSQL server"]
async fn baseline_workflows(pool: PgPool) {
    let cfg = config();
    assert_eq!(
        repositories::notification_event::find_all(&pool)
            .await
            .unwrap()
            .len(),
        18
    );
    assert_eq!(
        repositories::notification_setting::find_all(&pool)
            .await
            .unwrap()
            .len(),
        18
    );
    assert_eq!(
        repositories::partial_receiving_settings::get(&pool)
            .await
            .unwrap()
            .attention_after_days,
        3
    );
    assert_eq!(
        repositories::po_settings::get(&pool)
            .await
            .unwrap()
            .alert_sweep_interval_minutes,
        60
    );
    let actor = services::users::bootstrap_first_admin(
        &pool,
        "Test",
        "test@example.invalid",
        "test-password",
    )
    .await
    .unwrap();
    assert_eq!(
        services::auth::authenticate(&pool, &actor.email, "test-password")
            .await
            .unwrap()
            .id,
        actor.id
    );
    assert!(services::auth::authenticate(&pool, &actor.email, "wrong")
        .await
        .is_err());
    let hash = services::password::hash_password("reset-token").unwrap();
    repositories::password_reset_token::create(
        &pool,
        actor.id,
        &hash,
        chrono::Utc::now() + chrono::Duration::minutes(30),
    )
    .await
    .unwrap();
    services::password_reset::reset(&pool, "reset-token", "changed-password")
        .await
        .unwrap();
    assert!(
        services::password_reset::reset(&pool, "reset-token", "changed-password")
            .await
            .is_err()
    );
    services::auth::authenticate(&pool, &actor.email, "changed-password")
        .await
        .unwrap();

    let cat = repositories::category::find_all(&pool)
        .await
        .unwrap()
        .remove(0);
    let bt = repositories::branding_type::create(&pool, "test", "Test", None, 0)
        .await
        .unwrap();
    let bl = repositories::branding_location::create(&pool, "test", "Test", None, 0)
        .await
        .unwrap();
    let request = serde_json::from_value(json!({
        "wso_number": "TEST-WSO", "items": [{
            "category_id": cat.id, "description": "Test product", "design_code": "D", "fabric_code": "F",
            "branding_required": true,
            "branding": [{"branding_type_id": bt.id, "branding_location_id": bl.id, "quantity": 10}],
            "line_items": [{"size": "M", "qty_raised": 10}]
        }]
    })).unwrap();
    let wso = services::wso_create::create_complete_wso(&cfg, &pool, &request, &actor)
        .await
        .unwrap();
    let item = &wso.items[0];
    let line = &item.line_items[0];
    let history = repositories::wso_stage_history::find_by_wso_item(&pool, item.id)
        .await
        .unwrap();
    assert_eq!(history.len(), 1);
    assert_eq!(history[0].stage_name, "Not Started");
    assert_eq!(item.branding.len(), 1);
    let cutting = repositories::production_stage::find_by_code(&pool, "cutting")
        .await
        .unwrap()
        .unwrap();
    services::production_item_stage_history::create(
        &pool,
        item.id,
        serde_json::from_value(json!({"production_stage_id": cutting.id, "notes": "Started"}))
            .unwrap(),
        &actor,
    )
    .await
    .unwrap();
    assert_eq!(
        repositories::wso_stage_history::find_by_wso_item(&pool, item.id)
            .await
            .unwrap()
            .len(),
        2
    );
    // Exercise the standalone insert, which previously omitted wso_order_id.
    let extra = repositories::line_item::create(
        &pool,
        item.id,
        &serde_json::from_value(json!({"size": "L", "qty_raised": 2})).unwrap(),
    )
    .await
    .unwrap();
    assert_eq!(extra.wso_order_id, wso.id);
    repositories::line_item::delete(&pool, extra.id)
        .await
        .unwrap();
    let received = services::line_item::receive(
        &pool,
        &cfg,
        line.id,
        models::line_item::ReceiveLineItemRequest { quantity: 4 },
        &actor,
    )
    .await
    .unwrap();
    assert_eq!(received.balance, 6);
    // Optional item text must still decode in dashboard/attention projections.
    sqlx::query("UPDATE wso_items SET description = NULL, design_code = NULL, fabric_code = NULL WHERE id = $1")
        .bind(item.id).execute(&pool).await.unwrap();
    sqlx::query("UPDATE partial_receiving_tracking SET first_partial_received_at = now() - interval '5 days' WHERE wso_item_id = $1")
        .bind(item.id).execute(&pool).await.unwrap();
    assert_eq!(
        repositories::partial_receiving_attention::find_attention_required(&pool)
            .await
            .unwrap()[0]
            .description,
        ""
    );
    assert!(
        repositories::partial_receiving_tracking::find_active_by_wso_item(&pool, item.id)
            .await
            .unwrap()
            .is_some()
    );
    assert_eq!(
        repositories::wso_stage_history::find_by_wso_item(&pool, item.id)
            .await
            .unwrap()
            .len(),
        3
    );
    services::dashboard::get_dashboard(&pool, 1, 20)
        .await
        .unwrap();
    services::wso::get_wso_summary(&pool).await.unwrap();
    repositories::wso_item::find_by_category(&pool, cat.id)
        .await
        .unwrap();
    repositories::production_stage::find_items_in_stage(&pool, cutting.id)
        .await
        .unwrap();
    services::line_item::receive(
        &pool,
        &cfg,
        line.id,
        models::line_item::ReceiveLineItemRequest { quantity: 6 },
        &actor,
    )
    .await
    .unwrap();
    assert_eq!(
        services::wso::get_wso_detail(&pool, wso.id)
            .await
            .unwrap()
            .status,
        "completed"
    );
    assert!(
        repositories::partial_receiving_tracking::find_active_by_wso_item(&pool, item.id)
            .await
            .unwrap()
            .is_none()
    );

    let supplier = repositories::supplier::create(&pool, "Test supplier", None, None)
        .await
        .unwrap();
    repositories::supplier::update(&pool, supplier.id, "Updated supplier", None, None)
        .await
        .unwrap();
    let po_request = serde_json::from_value(json!({
        "erp_reference": "TEST-ERP", "accounts_reference": "TEST-ACC", "supplier_id": supplier.id,
        "items": [{"category_id": cat.id, "description": "Test PO item", "branding_required": false,
            "line_items": [{"size": "M", "qty_ordered": 10}]}]
    }))
    .unwrap();
    let po = services::purchase_order::create_complete(&pool, &po_request, &actor.name)
        .await
        .unwrap();
    let pi = &po.items[0];
    let pli = pi.line_items[0].line_item.id;
    for qty in [4, 6] {
        let receipt = serde_json::from_value(
            json!({"lines": [{"po_line_item_id": pli, "qty_delivered": qty}]}),
        )
        .unwrap();
        services::po_receipt::record_receipt(&pool, &cfg, po.id, receipt, &actor)
            .await
            .unwrap();
    }
    let defect = services::po_defect::report_defect(
        &pool,
        pli,
        serde_json::from_value(json!({"qty_defective": 2, "reason": "Test"})).unwrap(),
        &actor,
    )
    .await
    .unwrap();
    assert_eq!(
        services::purchase_order::get_detail(&pool, po.id)
            .await
            .unwrap()
            .total_outstanding,
        2
    );
    services::po_defect::resolve_defect(
        &pool,
        defect.id,
        serde_json::from_value(json!({"resolution_type": "accepted"})).unwrap(),
        &actor,
    )
    .await
    .unwrap();
    assert_eq!(
        services::purchase_order::get_detail(&pool, po.id)
            .await
            .unwrap()
            .derived_status,
        "completed"
    );
    repositories::po_item_note::create(&pool, pi.id, "Test note", &actor.name)
        .await
        .unwrap();
    assert_eq!(
        services::po_procurement_timeline::list(&pool, pi.id)
            .await
            .unwrap()
            .len(),
        4
    );
    services::po_dashboard::get_dashboard(&pool, 1, 20)
        .await
        .unwrap();
    services::po_overdue::get_overdue_items(&pool)
        .await
        .unwrap();

    // Persistence only: no SMTP worker is started by these tests.
    let event = repositories::notification_event::find_by_code(&pool, "wso_created")
        .await
        .unwrap();
    let log = repositories::notification_log::create_pending(
        &pool,
        event.id,
        "test@example.invalid",
        "email",
    )
    .await
    .unwrap();
    let job = repositories::notification_log::enqueue(
        &pool,
        models::enqueue_notification_job::EnqueueNotificationJob {
            notification_log_id: log.id,
            sender_name: "Test".into(),
            sender_email: "test@example.invalid".into(),
            recipient_email: "test@example.invalid".into(),
            subject: "Test".into(),
            html_body: "Test".into(),
        },
    )
    .await
    .unwrap();
    repositories::notification_job::mark_failed(&pool, job.id, "test error")
        .await
        .unwrap();
    repositories::notification_log::mark_failed(&pool, log.id, "test error")
        .await
        .unwrap();
    repositories::notification_log::mark_sent(&pool, log.id)
        .await
        .unwrap();
    let status: String = sqlx::query_scalar("SELECT status FROM notification_logs WHERE id = $1")
        .bind(log.id)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(status, "sent");
    let attempts: i32 = sqlx::query_scalar("SELECT attempts FROM notification_jobs WHERE id = $1")
        .bind(job.id)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(
        attempts, 1,
        "log updates must not modify the job a second time"
    );

    // Stage overdue calculations and occurrence deduplication use item history.
    sqlx::query("UPDATE wso_orders SET status = 'active' WHERE id = $1")
        .bind(wso.id)
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("UPDATE production_stages SET expected_duration_hours = 1 WHERE id = $1")
        .bind(cutting.id)
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("UPDATE wso_stage_history SET changed_at = now() - interval '2 days' WHERE wso_item_id = $1").bind(item.id).execute(&pool).await.unwrap();
    let overdue = repositories::production_stage::find_attention_required_items(&pool)
        .await
        .unwrap();
    assert_eq!(overdue.len(), 1);
    assert_eq!(overdue[0].description, "");
    for _ in 0..2 {
        repositories::attention_required_notification::record_notification(
            &pool,
            item.id,
            cutting.id,
            overdue[0].stage_started_at,
            event.id,
        )
        .await
        .unwrap();
    }
    assert!(
        repositories::attention_required_notification::has_been_notified(
            &pool,
            item.id,
            cutting.id,
            overdue[0].stage_started_at
        )
        .await
        .unwrap()
    );

    // Database invariants, independently of service validation.
    assert!(pool
        .execute("INSERT INTO partial_receiving_settings(id) VALUES (2)")
        .await
        .is_err());
    assert!(
        sqlx::query("UPDATE wso_line_items SET qty_received = qty_raised + 1 WHERE id = $1")
            .bind(line.id)
            .execute(&pool)
            .await
            .is_err()
    );
    assert!(
        sqlx::query("UPDATE wso_line_items SET wso_order_id = -1 WHERE id = $1")
            .bind(line.id)
            .execute(&pool)
            .await
            .is_err()
    );
    let legacy: i64 = sqlx::query_scalar("SELECT count(*) FROM information_schema.columns WHERE table_schema = 'public' AND table_name = 'wso_stage_history' AND column_name = 'wso_id'").fetch_one(&pool).await.unwrap();
    assert_eq!(legacy, 0);
    let naive: i64 = sqlx::query_scalar("SELECT count(*) FROM information_schema.columns WHERE table_schema = 'public' AND data_type = 'timestamp without time zone'").fetch_one(&pool).await.unwrap();
    assert_eq!(naive, 0);
}

#[sqlx::test(migrations = "./migrations")]
#[ignore = "requires DATABASE_URL pointing to a disposable PostgreSQL server"]
async fn repository_sql_prepares(pool: PgPool) {
    // Prepare standalone raw SQL directly from source so new columns/joins cannot
    // silently drift. Formatted fragments are covered by workflow calls above.
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/repositories");
    let mut checked = 0;
    for entry in std::fs::read_dir(root).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().and_then(|s| s.to_str()) != Some("rs") {
            continue;
        }
        let source = std::fs::read_to_string(&path).unwrap();
        let mut remaining = source.as_str();
        while let Some((_, tail)) = remaining.split_once("r#\"") {
            let (sql, tail) = tail.split_once("\"#").unwrap();
            remaining = tail;
            let mut sql = sql.trim().to_string();
            if sql.contains('{') || sql.contains("__SUPPLIER_NAME__") {
                continue;
            }
            if sql.starts_with('(') {
                sql = format!("SELECT * FROM {sql}");
            }
            if !(sql.starts_with("SELECT")
                || sql.starts_with("INSERT")
                || sql.starts_with("UPDATE")
                || sql.starts_with("DELETE"))
            {
                continue;
            }
            if path.file_name().unwrap() == "po_status.rs" {
                sql.push_str(" GROUP BY purchase_orders.id, purchase_orders.accounts_reference, purchase_orders.supplier_id, suppliers.name, purchase_orders.status");
            }
            pool.prepare(&sql)
                .await
                .unwrap_or_else(|e| panic!("{}: {e}\n{sql}", path.display()));
            checked += 1;
        }
    }
    assert!(
        checked > 100,
        "unexpected SQL coverage regression: {checked}"
    );
    println!("Prepared {checked} standalone repository SQL statements/fragments");
}
