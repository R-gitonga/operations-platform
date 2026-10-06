-- ============================================================
-- PO Notification Events
-- ============================================================
--
-- Ten events for the first cut (Tier 1 + Tier 2, per the
-- brainstorm). Settings default to enabled/email_enabled = TRUE
-- via notification_settings' own column defaults, matching how
-- WSO events were seeded -- but deliberately NO recipients are
-- seeded here, so nothing actually sends until someone adds one
-- in Settings.
-- ============================================================

INSERT INTO notification_events
(code, display_name, description, module)

VALUES

(
'po_created',
'Purchase Order Created',
'Raised whenever a new Purchase Order is created.',
'po'
),

(
'po_cancelled',
'Purchase Order Cancelled',
'Raised when a Purchase Order is cancelled.',
'po'
),

(
'po_reactivated',
'Purchase Order Reactivated',
'Raised when a cancelled Purchase Order is restored.',
'po'
),

(
'po_completed',
'Purchase Order Completed',
'Raised when every item on a Purchase Order has been fully received and accepted.',
'po'
),

(
'po_defect_reported',
'Defect Reported',
'Raised when a defect is reported against a delivered line item.',
'po'
),

(
'po_delivery_overdue',
'Delivery Overdue',
'Raised when a Purchase Order item passes its expected delivery date with quantity still outstanding.',
'po'
),

(
'po_delivery_approaching',
'Delivery Approaching Deadline',
'Raised when a Purchase Order item is nearing its expected delivery date.',
'po'
),

(
'po_defect_unresolved',
'Defect Unresolved',
'Raised when a reported defect remains open beyond the configured threshold.',
'po'
),

(
'po_partial_stalled',
'Partial Delivery Stalled',
'Raised when a partially received Purchase Order item has had no new deliveries for a while.',
'po'
),

(
'po_branding_goods_ready',
'Branding Goods Ready',
'Raised when a branded Purchase Order item has been fully received and accepted and is ready for branding.',
'po'
)

ON CONFLICT (code) DO NOTHING;

INSERT INTO notification_settings
(notification_event_id)

SELECT id
FROM notification_events
WHERE module = 'po'

ON CONFLICT (notification_event_id) DO NOTHING;