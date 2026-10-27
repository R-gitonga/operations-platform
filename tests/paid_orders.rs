//! Disposable database tests; no production SMTP server or inbox is contacted.
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
#[path = "../src/app_state.rs"]
mod app_state;
#[path = "../src/authenticated_user.rs"]
mod authenticated_user;
#[path = "../src/handlers/paid_orders.rs"]
mod paid_handlers;
use serde_json::json;
use services::{notification_worker as worker, po_notifications as emails};
use sqlx::{Executor, PgPool};

fn config() -> config::Config {
    config::Config {
        jwt_secret: "test".into(),
        jwt_expiration_hours: 1,
        smtp_host: "localhost".into(),
        smtp_port: 1,
        smtp_username: "test".into(),
        smtp_password: "test".into(),
        smtp_from: "sender@example.invalid".into(),
        system_notification_name: "Operations".into(),
        system_notification_email: "sender@example.invalid".into(),
        frontend_base_url: "http://localhost:5173".into(),
    }
}
fn actor() -> models::user::User {
    models::user::User {
        id: 1,
        name: "Test actor".into(),
        email: "actor@example.invalid".into(),
        password_hash: "unused".into(),
        role: "admin".into(),
        active: true,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    }
}

use services::paid_orders::{self as paid, Classification, Filters};

async fn fixture(
    pool: &PgPool,
) -> (
    i32,
    models::wso_detail::WsoDetail,
    models::purchase_order_detail::PurchaseOrderDetail,
) {
    let branch: i32 = sqlx::query_scalar(
        "INSERT INTO branches(code,name) VALUES('TEST','Test branch') RETURNING id",
    )
    .fetch_one(pool)
    .await
    .unwrap();
    let categories: Vec<i32> = sqlx::query_scalar("SELECT id FROM categories ORDER BY id LIMIT 2")
        .fetch_all(pool)
        .await
        .unwrap();
    let wso=models::create_complete_wso::CreateCompleteWsoRequest::deserialize(json!({
        "wso_number":"PAID-WSO", "is_paid_order":true,"branch_id":branch,
        "items":[
            {"category_id":categories[0],"description":"Trousers","design_code":"","fabric_code":"","branding_required":false,"branding":[],"paid_order_reason":"mtm","line_items":[{"size":"Custom","qty_raised":2}]},
            {"category_id":categories[1],"description":"Fleece","design_code":"","fabric_code":"","branding_required":false,"branding":[],"paid_order_reason":"stock_out","line_items":[{"size":"M","qty_raised":3}]}
        ]})).unwrap();
    let wso = services::wso_create::create_complete_wso(&config(), pool, &wso, &actor())
        .await
        .unwrap();
    let supplier = repositories::supplier::create(pool, "Test supplier", None, None)
        .await
        .unwrap();
    let po=serde_json::from_value(json!({"erp_reference":"PAID-ERP","accounts_reference":"PAID-ACC","supplier_id":supplier.id,"is_paid_order":true,"branch_id":branch,
        "items":[
            {"category_id":categories[0],"description":"Trousers","branding_required":false,"paid_order_reason":"mtm","line_items":[{"size":"Custom","qty_ordered":2}]},
            {"category_id":categories[1],"description":"Fleece","branding_required":false,"paid_order_reason":"stock_out","line_items":[{"size":"M","qty_ordered":3}]}
        ]})).unwrap();
    let po = services::purchase_order::create_complete(pool, &po, "Test actor")
        .await
        .unwrap();
    (branch, wso, po)
}
use serde::Deserialize;

#[sqlx::test(migrations = "./migrations")]
#[ignore = "requires disposable PostgreSQL"]
async fn paid_receiving_uses_source_lifecycle_and_notification_settings(pool: PgPool) {
    pool.execute("INSERT INTO notification_recipients(notification_event_id,display_name,email) SELECT id,'Test','receiver@example.invalid' FROM notification_events").await.unwrap();
    let (branch, wso, po) = fixture(&pool).await;
    sqlx::query("UPDATE branches SET name='Branch <East>' WHERE id=$1").bind(branch).execute(&pool).await.unwrap();
    let receive = |quantity| models::line_item::ReceiveLineItemRequest { quantity };
    let first = wso.items[0].line_items[0].id;
    assert!(services::line_item::receive(&pool, &config(), first, receive(3), &actor()).await.is_err());
    services::line_item::receive(&pool, &config(), first, receive(1), &actor()).await.unwrap();
    let rows = paid::list(&pool, &Filters::default()).await.unwrap();
    let row = rows.items.iter().find(|r| r["order_type"] == "wso").unwrap();
    assert_eq!(row["fulfilled"], 1);
    assert_eq!(row["outstanding"], 4);
    assert_eq!(row["status"], "partial");
    services::line_item::receive(&pool, &config(), first, receive(1), &actor()).await.unwrap();
    services::line_item::receive(&pool, &config(), wso.items[1].line_items[0].id, receive(3), &actor()).await.unwrap();
    assert!(services::line_item::receive(&pool, &config(), first, receive(1), &actor()).await.is_err());

    let lines: Vec<_> = po.items.iter().flat_map(|i| &i.line_items).map(|l| json!({"po_line_item_id":l.line_item.id,"qty_delivered":l.line_item.qty_ordered})).collect();
    services::po_receipt::record_receipt(&pool, &config(), po.id, serde_json::from_value(json!({"lines":lines})).unwrap(), &actor()).await.unwrap();
    // Snapshot captures the branch at receipt time, even if subsequently renamed.
    sqlx::query("UPDATE branches SET name='Renamed branch' WHERE id=$1").bind(branch).execute(&pool).await.unwrap();
    emails::enqueue_occurrences(&pool, &config()).await.unwrap();
    emails::enqueue_occurrences(&pool, &config()).await.unwrap();
    for (code, expected) in [("wso_completed", 1_i64), ("product_fully_received", 2), ("po_completed", 1)] {
        let messages: Vec<(String,String)> = sqlx::query_as("SELECT j.subject,j.html_body FROM notification_jobs j JOIN notification_logs l ON l.id=j.notification_log_id JOIN notification_events e ON e.id=l.notification_event_id WHERE e.code=$1").bind(code).fetch_all(&pool).await.unwrap();
        assert_eq!(messages.len() as i64, expected, "{code}");
        for (subject, body) in messages {
            assert!(subject.starts_with("[Paid order]"));
            assert!(body.contains("Branch &lt;East&gt;"));
            assert!(body.contains("Make to measure (MTM)") && body.contains("Stock-out"));
        }
    }
    let complete = paid::list(&pool, &Filters { status: Some("completed".into()), ..Default::default() }).await.unwrap();
    assert_eq!(complete.total, 2);
    assert!(complete.items.iter().all(|r| r["outstanding"] == 0));
    assert_eq!(paid::list(&pool, &Filters::default()).await.unwrap().total, 0);
    let before: i64 = sqlx::query_scalar("SELECT count(*) FROM notification_jobs").fetch_one(&pool).await.unwrap();
    pool.execute("UPDATE notification_settings SET enabled=false").await.unwrap();
    let mut context = models::notification_context::NotificationContext { event_code:"wso_completed".into(), actor_name:"Test".into(),actor_email:"test@example.invalid".into(),variables:Default::default() };
    context.insert("wso_number", wso.wso_number);
    services::notifications::dispatch(&pool, &config(), context).await.unwrap();
    assert_eq!(sqlx::query_scalar::<_, i64>("SELECT count(*) FROM notification_jobs").fetch_one(&pool).await.unwrap(), before);
}

#[sqlx::test(migrations = "./migrations")]
#[ignore = "requires disposable PostgreSQL"]
async fn mixed_items_filters_corrections_and_constraints(pool: PgPool) {
    let (branch, wso, po) = fixture(&pool).await;
    assert!(wso.is_paid_order && po.is_paid_order);
    assert_eq!(wso.items[0].paid_order_reason.as_deref(), Some("mtm"));
    let all = paid::list(&pool, &Filters::default()).await.unwrap();
    assert_eq!(all.total, 2);
    for row in &all.items {
        assert_eq!(row["ordered"], 5);
        assert_eq!(row["reasons"].as_array().unwrap().len(), 2);
    }
    let mut f = Filters {
        reason: Some("mtm".into()),
        ..Default::default()
    };
    let mtm = paid::list(&pool, &f).await.unwrap();
    assert_eq!(mtm.total, 2);
    assert!(mtm.items.iter().all(|r| r["ordered"] == 2));
    f.category_id = wso.items[1].category_id;
    assert_eq!(
        paid::list(&pool, &f).await.unwrap().total,
        0,
        "reason and category must match the same item"
    );
    assert!(
        sqlx::query("UPDATE wso_items SET paid_order_reason=NULL WHERE id=$1")
            .bind(wso.items[0].id)
            .execute(&pool)
            .await
            .is_err()
    );
    assert!(
        sqlx::query("INSERT INTO branches(code,name) VALUES('test','Duplicate')")
            .execute(&pool)
            .await
            .is_err()
    );
    sqlx::query("UPDATE branches SET active=false WHERE id=$1")
        .bind(branch)
        .execute(&pool)
        .await
        .unwrap();
    assert_eq!(
        paid::list(&pool, &Filters::default()).await.unwrap().total,
        2
    );
    let mut metadata = paid::metadata(&pool, "wso", wso.id).await.unwrap();
    let correction: Classification = serde_json::from_value(metadata.clone()).unwrap();
    paid::correct(&pool, "wso", wso.id, &correction, "Editor")
        .await
        .unwrap(); // unchanged inactive assignment is allowed
    metadata["is_paid_order"] = json!(false);
    metadata["branch_id"] = json!(null);
    for item in metadata["items"].as_array_mut().unwrap() {
        item["paid_order_reason"] = json!(null);
    }
    paid::correct(
        &pool,
        "wso",
        wso.id,
        &serde_json::from_value(metadata).unwrap(),
        "Editor",
    )
    .await
    .unwrap();
    assert_eq!(
        paid::list(&pool, &Filters::default()).await.unwrap().total,
        1
    );
    assert!(paid::correct(&pool, "wso", wso.id, &correction, "Editor")
        .await
        .is_err());
    let audit: i64 =
        sqlx::query_scalar("SELECT count(*) FROM paid_order_audit WHERE actor='Editor'")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(audit, 3);
    // A failed reclassification rolls back both header and item changes.
    assert!(
        !services::wso::get_wso_detail(&pool, wso.id)
            .await
            .unwrap()
            .is_paid_order
    );
    // New items on paid POs must include a valid reason.
    let invalid = serde_json::from_value(
        json!({"branding_required":false,"line_items":[{"size":"S","qty_ordered":1}]}),
    )
    .unwrap();
    assert!(
        services::po_item::add_item(&pool, po.id, &invalid, "Editor")
            .await
            .is_err()
    );
    let valid=serde_json::from_value(json!({"branding_required":false,"paid_order_reason":"stock_out","line_items":[{"size":"S","qty_ordered":1}]})).unwrap();
    services::po_item::add_item(&pool, po.id, &valid, "Editor")
        .await
        .unwrap();
    // Ordinary header and item edits preserve metadata.
    let request = serde_json::from_value(json!({"description":"Changed"})).unwrap();
    services::purchase_order::update(&pool, po.id, &request)
        .await
        .unwrap();
    assert!(
        services::purchase_order::get_detail(&pool, po.id)
            .await
            .unwrap()
            .is_paid_order
    );
}

#[sqlx::test(migrations = "./migrations")]
#[ignore = "requires disposable PostgreSQL"]
async fn fulfilment_and_pagination(pool: PgPool) {
    let (branch, _, po) = fixture(&pool).await;
    let lines: Vec<_> = po
        .items
        .iter()
        .flat_map(|i| i.line_items.iter())
        .map(|l| json!({"po_line_item_id":l.line_item.id,"qty_delivered":l.line_item.qty_ordered}))
        .collect();
    services::po_receipt::record_receipt(
        &pool,
        &config(),
        po.id,
        serde_json::from_value(json!({"lines":lines})).unwrap(),
        &actor(),
    )
    .await
    .unwrap();
    let f = Filters {
        order_type: Some("po".into()),
        status: Some("all".into()),
        ..Default::default()
    };
    let page = paid::list(&pool, &f).await.unwrap();
    assert_eq!(page.items[0]["status"], "completed");
    assert_eq!(page.items[0]["fulfilled"], 5);
    assert_eq!(
        paid::list(
            &pool,
            &Filters {
                order_type: Some("po".into()),
                ..Default::default()
            }
        )
        .await
        .unwrap()
        .total,
        0
    );
    sqlx::query("INSERT INTO po_defects(po_line_item_id,qty_defective,reason,reported_by) VALUES($1,1,'test','Tester')").bind(po.items[0].line_items[0].line_item.id).execute(&pool).await.unwrap();
    let page = paid::list(&pool, &f).await.unwrap();
    assert_eq!(page.items[0]["status"], "partial");
    assert_eq!(page.items[0]["fulfilled"], 4);
    assert_eq!(page.items[0]["outstanding"], 1);
    sqlx::query("UPDATE purchase_orders SET created_at='2026-10-25 22:00:00+00' WHERE id=$1")
        .bind(po.id)
        .execute(&pool)
        .await
        .unwrap();
    let day = chrono::NaiveDate::from_ymd_opt(2026, 10, 26).unwrap();
    assert_eq!(
        paid::list(
            &pool,
            &Filters {
                order_type: Some("po".into()),
                date_from: Some(day),
                date_to: Some(day),
                ..Default::default()
            }
        )
        .await
        .unwrap()
        .total,
        1
    );
    services::purchase_order::cancel(&pool, po.id, "Tester")
        .await
        .unwrap();
    assert_eq!(
        paid::list(
            &pool,
            &Filters {
                order_type: Some("po".into()),
                ..Default::default()
            }
        )
        .await
        .unwrap()
        .total,
        0
    );
    services::purchase_order::reactivate(&pool, po.id, "Tester")
        .await
        .unwrap();
    for n in 0..27 {
        let mut tx = pool.begin().await.unwrap();
        let id:i32=sqlx::query_scalar("INSERT INTO wso_orders(wso_number,is_paid_order,branch_id) VALUES($1,true,$2) RETURNING id").bind(format!("PAGE-{n}")).bind(branch).fetch_one(&mut *tx).await.unwrap();
        sqlx::query("INSERT INTO wso_items(wso_order_id,paid_order_reason) VALUES($1,'mtm')")
            .bind(id)
            .execute(&mut *tx)
            .await
            .unwrap();
        tx.commit().await.unwrap();
    }
    let first = paid::list(&pool, &Filters::default()).await.unwrap();
    let second = paid::list(
        &pool,
        &Filters {
            page: Some(2),
            ..Default::default()
        },
    )
    .await
    .unwrap();
    assert_eq!(first.total, 29);
    assert_eq!(first.items.len(), 25);
    assert_eq!(second.items.len(), 4);
    for a in first.items {
        assert!(!second
            .items
            .iter()
            .any(|b| a["order_type"] == b["order_type"] && a["order_id"] == b["order_id"]));
    }
}

#[sqlx::test(migrations = false)]
#[ignore = "requires disposable PostgreSQL"]
async fn upgrade_preserves_existing_bulk_orders(pool: PgPool) {
    for sql in [
        include_str!("../migrations/202610150001_reference_and_users.sql"),
        include_str!("../migrations/202610150002_wso.sql"),
        include_str!("../migrations/202610150003_notifications.sql"),
        include_str!("../migrations/202610150004_purchasing.sql"),
        include_str!("../migrations/202610150005_reference_data.sql"),
        include_str!("../migrations/202610150006_po_email_notifications.sql"),
    ] {
        pool.execute(sql).await.unwrap();
    }
    pool.execute("INSERT INTO wso_orders(wso_number,status) VALUES('LEGACY','partial'); INSERT INTO wso_items(wso_order_id) VALUES(1); INSERT INTO wso_line_items(wso_order_id,wso_item_id,size,qty_raised,qty_received) VALUES(1,1,'M',10,3)").await.unwrap();
    pool.execute(include_str!("../migrations/202610260001_paid_orders.sql"))
        .await
        .unwrap();
    let order = services::wso::get_wso_detail(&pool, 1).await.unwrap();
    assert!(!order.is_paid_order);
    assert_eq!(order.branch_id, None);
    assert_eq!(order.status, "partial");
    assert_eq!(order.total_qty_received, 3);
    assert_eq!(order.total_balance, 7);
}
