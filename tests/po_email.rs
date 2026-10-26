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
async fn order(pool: &PgPool) -> models::purchase_order_detail::PurchaseOrderDetail {
    let supplier = repositories::supplier::create(pool, "Supplier <&>", None, None)
        .await
        .unwrap();
    let request=serde_json::from_value(json!({"erp_reference":format!("ERP-{}",supplier.id),"accounts_reference":format!("ACC-{}",supplier.id),"supplier_id":supplier.id,
        "items":[{"description":"Shirts <&>","branding_required":false,"line_items":[{"size":"M","qty_ordered":10},{"size":"L","qty_ordered":10}]}]})).unwrap();
    services::purchase_order::create_complete(pool, &request, "Creator")
        .await
        .unwrap()
}
async fn count(pool: &PgPool, code: &str) -> i64 {
    sqlx::query_scalar("SELECT count(*) FROM po_notification_occurrences WHERE event_code=$1")
        .bind(code)
        .fetch_one(pool)
        .await
        .unwrap()
}
async fn recipients(pool: &PgPool) {
    pool.execute("INSERT INTO notification_recipients(notification_event_id,display_name,email) SELECT id,'Test','receiver@example.invalid' FROM notification_events").await.unwrap();
}
async fn receipt(pool: &PgPool, po: &models::purchase_order_detail::PurchaseOrderDetail, qty: i32) {
    let lines: Vec<_> = po.items[0]
        .line_items
        .iter()
        .map(|l| json!({"po_line_item_id":l.line_item.id,"qty_delivered":qty}))
        .collect();
    services::po_receipt::record_receipt(
        pool,
        &config(),
        po.id,
        serde_json::from_value(json!({"lines":lines})).unwrap(),
        &actor(),
    )
    .await
    .unwrap();
}

#[sqlx::test(migrations = "./migrations")]
#[ignore = "requires disposable PostgreSQL"]
async fn lifecycle_snapshots_rollback_and_reentry(pool: PgPool) {
    let po = order(&pool).await;
    assert_eq!(count(&pool, "po_created").await, 1);
    let items:i64=sqlx::query_scalar("SELECT jsonb_array_length(payload->'items')::bigint FROM po_notification_occurrences WHERE event_code='po_created'").fetch_one(&pool).await.unwrap();
    assert_eq!(items, 1, "deferred creation captures the complete order");
    sqlx::query("UPDATE po_items SET branding_required=true WHERE id=$1")
        .bind(po.items[0].id)
        .execute(&pool)
        .await
        .unwrap();
    let mut tx = pool.begin().await.unwrap();
    sqlx::query("UPDATE purchase_orders SET status='cancelled' WHERE id=$1")
        .bind(po.id)
        .execute(&mut *tx)
        .await
        .unwrap();
    sqlx::query("SET CONSTRAINTS ALL IMMEDIATE")
        .execute(&mut *tx)
        .await
        .unwrap();
    tx.rollback().await.unwrap();
    assert_eq!(count(&pool, "po_cancelled").await, 0);
    services::purchase_order::cancel(&pool, po.id, "Canceller")
        .await
        .unwrap();
    assert!(services::purchase_order::cancel(&pool, po.id, "Canceller")
        .await
        .is_err());
    services::purchase_order::reactivate(&pool, po.id, "Reactivator")
        .await
        .unwrap();
    assert_eq!(count(&pool, "po_cancelled").await, 1);
    assert_eq!(count(&pool, "po_reactivated").await, 1);
    let who: String = sqlx::query_scalar(
        "SELECT payload->>'actor' FROM po_notification_occurrences WHERE event_code='po_cancelled'",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(who, "Canceller");
    receipt(&pool, &po, 10).await;
    assert_eq!(count(&pool, "po_completed").await, 1);
    assert_eq!(count(&pool, "po_branding_goods_ready").await, 1);
    let defect = services::po_defect::report_defect(
        &pool,
        po.items[0].line_items[0].line_item.id,
        serde_json::from_value(json!({"qty_defective":2,"reason":"Bad <stitch>"})).unwrap(),
        &actor(),
    )
    .await
    .unwrap();
    assert_eq!(count(&pool, "po_defect_reported").await, 1);
    services::po_defect::resolve_defect(
        &pool,
        defect.id,
        serde_json::from_value(json!({"resolution_type":"accepted"})).unwrap(),
        &actor(),
    )
    .await
    .unwrap();
    assert_eq!(count(&pool, "po_completed").await, 2);
    assert_eq!(count(&pool, "po_branding_goods_ready").await, 2);
    pool.execute("UPDATE po_items SET description='Edited description'")
        .await
        .unwrap();
    assert_eq!(count(&pool, "po_completed").await, 2);
    // Aggregate totals still equal 20, but size L is now short. Both status
    // and notifications must use the same per-line qualification.
    let mut tx = pool.begin().await.unwrap();
    sqlx::query("UPDATE po_line_items SET qty_ordered=CASE WHEN size='M' THEN 5 ELSE 15 END")
        .execute(&mut *tx)
        .await
        .unwrap();
    tx.commit().await.unwrap();
    assert_eq!(
        services::purchase_order::get_detail(&pool, po.id)
            .await
            .unwrap()
            .derived_status,
        "partial"
    );
    pool.execute("UPDATE po_line_items SET qty_ordered=10")
        .await
        .unwrap();
    assert_eq!(count(&pool, "po_completed").await, 3);
    assert_eq!(count(&pool, "po_branding_goods_ready").await, 3);
    recipients(&pool).await;
    let cfg = config();
    let (a, b) = tokio::join!(
        emails::enqueue_occurrences(&pool, &cfg),
        emails::enqueue_occurrences(&pool, &cfg)
    );
    a.unwrap();
    b.unwrap();
    let duplicates:i64=sqlx::query_scalar("SELECT count(*) FROM (SELECT occurrence_id,recipient_email FROM notification_jobs GROUP BY 1,2 HAVING count(*)>1) d").fetch_one(&pool).await.unwrap();
    assert_eq!(duplicates, 0);
}

#[sqlx::test(migrations = "./migrations")]
#[ignore = "requires disposable PostgreSQL"]
async fn timed_policies_and_grace(pool: PgPool) {
    let po = order(&pool).await;
    recipients(&pool).await;
    pool.execute(
        "UPDATE po_items SET expected_delivery_date=(now() AT TIME ZONE 'Africa/Nairobi')::date+2",
    )
    .await
    .unwrap();
    emails::sweep(&pool).await.unwrap();
    emails::enqueue_occurrences(&pool, &config()).await.unwrap();
    emails::sweep(&pool).await.unwrap();
    assert_eq!(count(&pool, "po_delivery_approaching").await, 1);
    pool.execute("UPDATE po_settings SET approaching_window_days=5")
        .await
        .unwrap();
    emails::sweep(&pool).await.unwrap();
    assert_eq!(count(&pool, "po_delivery_approaching").await, 1);
    pool.execute("UPDATE po_items SET expected_delivery_date=(now() AT TIME ZONE 'Africa/Nairobi')::date-1; UPDATE po_settings SET overdue_grace_days=1").await.unwrap();
    emails::sweep(&pool).await.unwrap();
    assert_eq!(count(&pool, "po_delivery_overdue").await, 0);
    assert!(repositories::po_overdue::find_overdue(&pool)
        .await
        .unwrap()
        .is_empty());
    pool.execute("UPDATE po_settings SET overdue_grace_days=0; UPDATE notification_settings SET email_enabled=false WHERE notification_event_id=(SELECT id FROM notification_events WHERE code='po_delivery_overdue')").await.unwrap();
    emails::sweep(&pool).await.unwrap();
    assert_eq!(count(&pool, "po_delivery_overdue").await, 0);
    pool.execute("UPDATE notification_settings SET email_enabled=true")
        .await
        .unwrap();
    emails::sweep(&pool).await.unwrap();
    assert_eq!(count(&pool, "po_delivery_overdue").await, 1);
    assert_eq!(
        repositories::po_overdue::find_overdue(&pool)
            .await
            .unwrap()
            .len(),
        2
    );
    // Stale timed occurrence is suppressed before queueing, then re-evaluated
    // when it qualifies again. It was never recorded as delivered.
    services::purchase_order::cancel(&pool, po.id, "Test")
        .await
        .unwrap();
    emails::enqueue_occurrences(&pool, &config()).await.unwrap();
    services::purchase_order::reactivate(&pool, po.id, "Test")
        .await
        .unwrap();
    emails::sweep(&pool).await.unwrap();
    emails::enqueue_occurrences(&pool, &config()).await.unwrap();
    emails::sweep(&pool).await.unwrap();
    assert_eq!(count(&pool, "po_delivery_overdue").await, 1);
    pool.execute("UPDATE po_items SET expected_delivery_date=expected_delivery_date-1")
        .await
        .unwrap();
    emails::sweep(&pool).await.unwrap();
    assert_eq!(count(&pool, "po_delivery_overdue").await, 2);
    pool.execute("UPDATE po_items SET expected_delivery_date=expected_delivery_date+1")
        .await
        .unwrap();
    emails::sweep(&pool).await.unwrap();
    assert_eq!(
        count(&pool, "po_delivery_overdue").await,
        3,
        "reverting a date is still a new revision"
    );
    receipt(&pool, &po, 2).await;
    pool.execute("UPDATE po_receipts SET received_at=now()-interval '4 days'")
        .await
        .unwrap();
    emails::sweep(&pool).await.unwrap();
    emails::enqueue_occurrences(&pool, &config()).await.unwrap();
    assert_eq!(count(&pool, "po_partial_stalled").await, 1);
    receipt(&pool, &po, 1).await;
    emails::sweep(&pool).await.unwrap();
    assert_eq!(
        count(&pool, "po_partial_stalled").await,
        1,
        "a recent receipt resets the item clock"
    );
    let defect = services::po_defect::report_defect(
        &pool,
        po.items[0].line_items[0].line_item.id,
        serde_json::from_value(json!({"qty_defective":1,"reason":"Damaged"})).unwrap(),
        &actor(),
    )
    .await
    .unwrap();
    pool.execute("UPDATE po_defects SET reported_at=now()-interval '4 days'")
        .await
        .unwrap();
    emails::sweep(&pool).await.unwrap();
    emails::enqueue_occurrences(&pool, &config()).await.unwrap();
    emails::sweep(&pool).await.unwrap();
    assert_eq!(count(&pool, "po_defect_unresolved").await, 1);
    services::po_defect::resolve_defect(
        &pool,
        defect.id,
        serde_json::from_value(json!({"resolution_type":"written_off"})).unwrap(),
        &actor(),
    )
    .await
    .unwrap();
    emails::sweep(&pool).await.unwrap();
    assert_eq!(count(&pool, "po_defect_unresolved").await, 1);
}

#[sqlx::test(migrations = "./migrations")]
#[ignore = "requires disposable PostgreSQL"]
async fn delivery_flags_claims_retries_and_fencing(pool: PgPool) {
    let po = order(&pool).await;
    recipients(&pool).await;
    pool.execute("INSERT INTO notification_recipients(notification_event_id,display_name,email) SELECT id,'Duplicate',' RECEIVER@EXAMPLE.INVALID ' FROM notification_events WHERE code='po_created'").await.unwrap();
    emails::enqueue_occurrences(&pool, &config()).await.unwrap();
    let jobs: i64 = sqlx::query_scalar("SELECT count(*) FROM notification_jobs")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(jobs, 1);
    let (a, b) = tokio::join!(worker::claim(&pool), worker::claim(&pool));
    let claimed: Vec<_> = [a.unwrap(), b.unwrap()].into_iter().flatten().collect();
    assert_eq!(claimed.len(), 1);
    let first = &claimed[0];
    assert!(worker::allowed(&pool, first).await.unwrap());
    pool.execute("UPDATE notification_jobs SET lease_until=now()-interval '1 second'")
        .await
        .unwrap();
    let second = worker::claim(&pool).await.unwrap().unwrap();
    worker::finish(&pool, first, "sent", None).await.unwrap();
    let state: String = sqlx::query_scalar("SELECT status FROM notification_jobs")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(state, "processing");
    worker::finish(&pool, &second, "retry", Some("Transient test failure"))
        .await
        .unwrap();
    assert!(worker::claim(&pool).await.unwrap().is_none());
    pool.execute("UPDATE notification_jobs SET available_at=now(); UPDATE notification_settings SET email_enabled=false").await.unwrap();
    worker::process_with_sender(&pool, &config(), |_| async {
        panic!("disabled email must not send")
    })
    .await
    .unwrap();
    let statuses:(String,String)=sqlx::query_as("SELECT j.status,l.status FROM notification_jobs j JOIN notification_logs l ON l.id=j.notification_log_id").fetch_one(&pool).await.unwrap();
    assert_eq!(statuses, ("suppressed".into(), "suppressed".into()));
    // Existing WSO dispatch must also respect the email switch.
    let ctx = models::notification_context::NotificationContext {
        event_code: "wso_created".into(),
        actor_name: "Test".into(),
        actor_email: "test@example.invalid".into(),
        variables: Default::default(),
    };
    services::notifications::dispatch(&pool, &config(), ctx.clone())
        .await
        .unwrap();
    let jobs: i64 = sqlx::query_scalar("SELECT count(*) FROM notification_jobs")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(jobs, 1);
    pool.execute("UPDATE notification_settings SET email_enabled=true")
        .await
        .unwrap();
    services::notifications::dispatch(&pool, &config(), ctx)
        .await
        .unwrap();
    for attempt in 1..=5 {
        worker::process_with_sender(&pool, &config(), |_| async {
            Err("Temporary SMTP failure".into())
        })
        .await
        .unwrap();
        let state: String =
            sqlx::query_scalar("SELECT status FROM notification_jobs ORDER BY id DESC LIMIT 1")
                .fetch_one(&pool)
                .await
                .unwrap();
        assert_eq!(state, if attempt == 5 { "failed" } else { "retry" });
        pool.execute("UPDATE notification_jobs SET available_at=now()")
            .await
            .unwrap();
    }
    services::purchase_order::cancel(&pool, po.id, "Test")
        .await
        .unwrap();
    worker::process_with_sender(&pool, &config(), |_| async {
        Err("Permanent SMTP failure: 550".into())
    })
    .await
    .unwrap();
    let attempts: i32 =
        sqlx::query_scalar("SELECT attempts FROM notification_jobs ORDER BY id DESC LIMIT 1")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(attempts, 1);
}

#[sqlx::test(migrations = "./migrations")]
#[ignore = "requires disposable PostgreSQL"]
async fn smtp_capture_receives_po_email(pool: PgPool) {
    use lettre::{AsyncSmtpTransport, AsyncTransport, Tokio1Executor};
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    let capture = tokio::spawn(async move {
        let (socket, _) = listener.accept().await.unwrap();
        let (read, mut write) = socket.into_split();
        let mut read = BufReader::new(read);
        let mut line = String::new();
        let mut body = String::new();
        let mut data = false;
        write
            .write_all(b"220 localhost test SMTP\r\n")
            .await
            .unwrap();
        loop {
            line.clear();
            if read.read_line(&mut line).await.unwrap() == 0 {
                break;
            }
            if data {
                if line == ".\r\n" {
                    write.write_all(b"250 queued\r\n").await.unwrap();
                    break;
                }
                body.push_str(&line);
                continue;
            }
            let response = if line.starts_with("EHLO") {
                "250 localhost\r\n"
            } else if line.starts_with("DATA") {
                data = true;
                "354 End with dot\r\n"
            } else {
                "250 OK\r\n"
            };
            write.write_all(response.as_bytes()).await.unwrap();
        }
        body
    });
    order(&pool).await;
    recipients(&pool).await;
    worker::process_with_sender(&pool, &config(), |message| async move {
        // Plain local transport is confined to this test; production requires STARTTLS.
        let mailer = AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous("127.0.0.1")
            .port(port)
            .build();
        mailer
            .send(services::email_sender::build_message(message)?)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    })
    .await
    .unwrap();
    let body = tokio::time::timeout(std::time::Duration::from_secs(10), capture)
        .await
        .unwrap()
        .unwrap();
    assert!(body.contains("Subject: Purchase order created: ACC-"));
    assert!(body.contains("receiver@example.invalid"));
    assert!(body.contains("supplier") || body.contains("Supplier"));
    let status: String = sqlx::query_scalar("SELECT status FROM notification_jobs")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(status, "sent");
}

#[sqlx::test(migrations = false)]
#[ignore = "requires disposable PostgreSQL"]
async fn upgrade_baselines_historical_alerts(pool: PgPool) {
    for sql in [
        include_str!("../migrations/202610150001_reference_and_users.sql"),
        include_str!("../migrations/202610150002_wso.sql"),
        include_str!("../migrations/202610150003_notifications.sql"),
        include_str!("../migrations/202610150004_purchasing.sql"),
        include_str!("../migrations/202610150005_reference_data.sql"),
    ] {
        sqlx::raw_sql(sql).execute(&pool).await.unwrap();
    }
    // Seed via SQL because the current status projection needs the new view/fields.
    pool.execute("INSERT INTO suppliers(name) VALUES('Historical'); INSERT INTO purchase_orders(erp_reference,accounts_reference,supplier_id,created_by) VALUES('OLD','OLD',1,'Old creator'),('OLD2','OLD2',1,'Old creator'); INSERT INTO po_items(purchase_order_id,created_by,expected_delivery_date,branding_required) VALUES(1,'Old creator',CURRENT_DATE-10,true),(2,'Old creator',CURRENT_DATE-10,false); INSERT INTO po_line_items(po_item_id,size,qty_ordered) VALUES(1,'M',10),(2,'M',10); INSERT INTO po_receipts(purchase_order_id,received_by,received_at) VALUES(1,'Old actor',now()-interval '10 days'),(2,'Old actor',now()-interval '10 days'); INSERT INTO po_receipt_lines(po_receipt_id,po_line_item_id,qty_delivered,total_delivered_to_date,delivered_balance) VALUES(1,1,10,10,0),(2,2,5,5,5); INSERT INTO po_defects(po_line_item_id,qty_defective,reason,reported_by,reported_at) VALUES(2,1,'Old defect','Old actor',now()-interval '10 days')").await.unwrap();
    sqlx::raw_sql(include_str!(
        "../migrations/202610150006_po_email_notifications.sql"
    ))
    .execute(&pool)
    .await
    .unwrap();
    recipients(&pool).await;
    emails::sweep(&pool).await.unwrap();
    emails::enqueue_occurrences(&pool, &config()).await.unwrap();
    let count: i64 = sqlx::query_scalar("SELECT count(*) FROM po_notification_occurrences")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(count, 0);
    pool.execute("UPDATE po_items SET expected_delivery_date=expected_delivery_date-1")
        .await
        .unwrap();
    emails::sweep(&pool).await.unwrap();
    let count: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM po_notification_occurrences WHERE event_code='po_delivery_overdue'",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(count, 1);
}

#[sqlx::test(migrations = "./migrations")]
#[ignore = "requires disposable PostgreSQL"]
async fn concurrent_receipts_capture_completion_once(pool: PgPool) {
    let po = order(&pool).await;
    let cfg = config();
    let user = actor();
    let first=serde_json::from_value(json!({"lines":[{"po_line_item_id":po.items[0].line_items[0].line_item.id,"qty_delivered":10}]})).unwrap();
    let second=serde_json::from_value(json!({"lines":[{"po_line_item_id":po.items[0].line_items[1].line_item.id,"qty_delivered":10}]})).unwrap();
    let (a, b) = tokio::join!(
        services::po_receipt::record_receipt(&pool, &cfg, po.id, first, &user),
        services::po_receipt::record_receipt(&pool, &cfg, po.id, second, &user)
    );
    a.unwrap();
    b.unwrap();
    assert_eq!(count(&pool, "po_completed").await, 1);
}

#[sqlx::test(migrations = "./migrations")]
#[ignore = "requires disposable PostgreSQL"]
async fn missing_recipients_and_master_switch_do_not_send(pool: PgPool) {
    order(&pool).await;
    emails::enqueue_occurrences(&pool, &config()).await.unwrap();
    recipients(&pool).await;
    emails::enqueue_occurrences(&pool, &config()).await.unwrap();
    let jobs: i64 = sqlx::query_scalar("SELECT count(*) FROM notification_jobs")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(
        jobs, 0,
        "past immediate events are not replayed on enabling recipients"
    );
    order(&pool).await;
    pool.execute("UPDATE notification_settings SET enabled=false")
        .await
        .unwrap();
    emails::enqueue_occurrences(&pool, &config()).await.unwrap();
    let jobs: i64 = sqlx::query_scalar("SELECT count(*) FROM notification_jobs")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(jobs, 0);
    pool.execute("UPDATE notification_settings SET enabled=true")
        .await
        .unwrap();
    order(&pool).await;
    emails::enqueue_occurrences(&pool, &config()).await.unwrap();
    pool.execute("UPDATE notification_recipients SET enabled=false")
        .await
        .unwrap();
    worker::process_with_sender(&pool, &config(), |_| async {
        panic!("disabled recipient must not be sent an email")
    })
    .await
    .unwrap();
    let status: String = sqlx::query_scalar("SELECT status FROM notification_jobs")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(status, "suppressed");
}

#[sqlx::test(migrations = "./migrations")]
#[ignore = "requires disposable PostgreSQL"]
async fn enqueue_failure_leaves_no_orphan_log(pool: PgPool) {
    order(&pool).await;
    recipients(&pool).await;
    let mut cfg = config();
    cfg.system_notification_name = "x".repeat(300);
    assert!(emails::enqueue_occurrences(&pool, &cfg).await.is_err());
    let logs: i64 = sqlx::query_scalar("SELECT count(*) FROM notification_logs")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(logs, 0);
    let state: String = sqlx::query_scalar("SELECT state FROM po_notification_occurrences")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(state, "pending");
    emails::enqueue_occurrences(&pool, &config()).await.unwrap();
    let jobs: i64 = sqlx::query_scalar("SELECT count(*) FROM notification_jobs")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(jobs, 1);
}
