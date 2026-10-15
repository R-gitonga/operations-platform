-- Reference configuration only: no accounts, recipients or operational records.
INSERT INTO categories(name) VALUES
    ('Shirts'), ('Trousers'), ('Fleeces'), ('Blazers'), ('Sportswear')
ON CONFLICT (name) DO NOTHING;

INSERT INTO production_stages(code, display_name, display_order, color, expected_duration_hours)
VALUES
    ('NOT_STARTED', 'Not Started', 0, '#64748b', 24),
    ('cad_room', 'CAD Room', 1, 'rgb(124, 58, 237)', NULL),
    ('fabric_allocation', 'Fabric Allocation', 2, 'rgb(37, 99, 235)', NULL),
    ('cutting', 'Cutting', 3, 'rgb(234, 88, 12)', NULL),
    ('embroidery', 'Embroidery', 4, '#db2777', NULL),
    ('printing', 'Printing', 5, '#0891b2', NULL),
    ('production', 'Production Floor', 6, '#16a34a', NULL),
    ('finishing', 'Finishing', 7, '#65a30d', NULL),
    ('quality_control', 'Quality Control', 8, '#ca8a04', NULL),
    ('packing', 'Packing', 9, '#0f766e', NULL),
    ('dispatch', 'Ready For Dispatch', 10, '#1d4ed8', NULL)
ON CONFLICT DO NOTHING;

INSERT INTO notification_events(code, display_name, description, module) VALUES
    ('wso_created', 'Workshop Order Created', 'Raised whenever a new workshop order is created.', 'wso'),
    ('wso_completed', 'Workshop Order Completed', 'Raised when every line item has been fully received.', 'wso'),
    ('wso_cancelled', 'Workshop Order Cancelled', 'Raised when a workshop order is cancelled.', 'wso'),
    ('wso_reactivated', 'Workshop Order Reactivated', 'Raised when a cancelled workshop order is restored.', 'wso'),
    ('attachment_uploaded', 'Attachment Uploaded', 'Raised whenever a document is attached to a workshop order.', 'wso'),
    ('attention_required', 'Product Attention Required', 'Raised when a product remains in a production stage beyond its expected duration.', 'wso'),
    ('product_fully_received', 'Product Fully Received', 'Raised when all line items for a product have been fully received.', 'wso'),
    ('partial_receiving_attention', 'Partial Receiving Attention', 'Raised when a product remains partially received beyond the configured attention threshold.', 'wso'),
    ('po_created', 'Purchase Order Created', 'Raised whenever a new Purchase Order is created.', 'po'),
    ('po_cancelled', 'Purchase Order Cancelled', 'Raised when a Purchase Order is cancelled.', 'po'),
    ('po_reactivated', 'Purchase Order Reactivated', 'Raised when a cancelled Purchase Order is restored.', 'po'),
    ('po_completed', 'Purchase Order Completed', 'Raised when every item on a Purchase Order has been fully received and accepted.', 'po'),
    ('po_defect_reported', 'Defect Reported', 'Raised when a defect is reported against a delivered line item.', 'po'),
    ('po_delivery_overdue', 'Delivery Overdue', 'Raised when a Purchase Order item passes its expected delivery date with quantity still outstanding.', 'po'),
    ('po_delivery_approaching', 'Delivery Approaching Deadline', 'Raised when a Purchase Order item is nearing its expected delivery date.', 'po'),
    ('po_defect_unresolved', 'Defect Unresolved', 'Raised when a reported defect remains open beyond the configured threshold.', 'po'),
    ('po_partial_stalled', 'Partial Delivery Stalled', 'Raised when a partially received Purchase Order item has had no new deliveries for a while.', 'po'),
    ('po_branding_goods_ready', 'Branding Goods Ready', 'Raised when a branded Purchase Order item has been fully received and accepted and is ready for branding.', 'po')
ON CONFLICT (code) DO NOTHING;

INSERT INTO notification_settings(notification_event_id)
SELECT id FROM notification_events ON CONFLICT (notification_event_id) DO NOTHING;
INSERT INTO partial_receiving_settings(id) VALUES (1) ON CONFLICT (id) DO NOTHING;
INSERT INTO po_settings(id) VALUES (1) ON CONFLICT (id) DO NOTHING;
