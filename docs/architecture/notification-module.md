Notification Module
Vision

The Notification Module exists to eliminate manual communication surrounding Workshop Orders (WSOs).

Today, once something important happens, someone usually has to:

send an email,
call another department,
send a WhatsApp message,
or update an Excel report.

This module allows the system to become the communicator.

Whenever an important event occurs, everyone who needs to know is informed immediately.

The objective is not to send more emails.

The objective is to ensure nobody has to ask:

"Has my order been completed yet?"

Philosophy

Not every action deserves a notification.

Routine actions create noise.

Important business milestones create communication.

Our rule is therefore simple:

Notify people only when someone would normally have informed them manually.

Types of Notifications

The system will eventually support two notification methods.

1. Email Notifications

These notify users even when they are away from the system.

Emails are intended for significant business events.

Examples:

Workshop Order Completed
Workshop Order Cancelled
Workshop Order Reactivated
Attachment Uploaded
Production Delayed
Order Ready for Collection
2. In-App Notifications

These appear inside the application itself.

They provide immediate visibility without filling people's inboxes.

Examples:

A line item was updated.
Somebody edited a description.
Fabric code changed.
Remarks updated.

These are useful while users are actively working inside the platform.

Guiding Principle

Not every database update deserves an email.

A line item quantity changing from

14

to

15

is not management information.

A Workshop Order becoming

Completed

is.

Events Worth Notifying

The following actions are considered business milestones.

Workshop Order Completed

Recipients:

Retail Department
Production Manager
Store Manager
Procurement (optional)

Purpose

Everyone immediately knows production has finished.

No phone calls.

No Excel reports.

Workshop Order Cancelled

Recipients

Retail
Production
Management

Purpose

Prevent unnecessary production work.

Everyone immediately knows this order no longer requires attention.

Workshop Order Reactivated

Recipients

Same people who received the cancellation.

Purpose

Production resumes.

Everyone knows the cancellation has been reversed.

Attachment Uploaded

Recipients

Production team only.

Purpose

A new design, artwork or supporting document is now available.

Workshop Order Created

Recipients

Production.

Purpose

A new production request has entered the system.

This effectively replaces someone emailing a PDF manually.

Workshop Order Edited

Normally:

No email.

Reason:

Minor edits happen frequently.

Instead, record these in an activity history.

Partial Receipt

No email.

Reason:

Receiving stock is a normal operational activity.

Instead:

Update dashboards.

Update progress.

Allow users to view production progress.

Fully Received

Recipients

Retail.

Purpose

Everything has now arrived.

The order can be closed.

Events That Should NOT Send Emails

The following should remain silent.

Editing remarks
Editing descriptions
Changing design codes
Updating fabric codes
Editing quantities before production
Uploading corrected spelling
Minor administrative corrections

These belong in an audit trail rather than an inbox.

Notification Audience

Instead of sending emails to individual people, notifications should eventually be sent to groups.

For example:

Retail Team

Store Managers
Procurement
Retail Operations

Production Team

Factory Manager
Cutting
Stitching
Printing

Management

Directors
Operations Manager

This allows staff to change without changing application logic.

Future Notification Preferences

Eventually, every user should decide how they wish to receive notifications.

Examples

Receive Emails

☑ Completed Orders

☑ Cancelled Orders

☐ Partial Receipts

Receive In-App Notifications

☑ Everything

This keeps inboxes useful rather than overwhelming.

Notification History

Every notification should remain visible inside the system.

Example

Time	Event	Sent To
08:43	WSO Created	Production
09:01	Attachment Uploaded	Production
12:12	WSO Completed	Retail, Management
12:13	Email Delivered	6 Recipients

This creates accountability.

Nobody can claim they were never informed.

Long-Term Vision

The Notification Module is the beginning of a much larger operational communication platform.

Future capabilities may include:

Microsoft Teams notifications
Slack integration
WhatsApp Business notifications
SMS alerts for urgent events
Daily management summaries
Weekly production reports generated automatically
Escalation reminders for overdue orders
Approval workflows
Read receipts for important notifications
Success Criteria

A successful Notification Module means:

Production never has to ask whether a WSO is active.
Retail never has to ask whether production is complete.
Management no longer waits for manually prepared Excel reports.
Important business events are communicated automatically.
Routine edits remain quiet.
The system becomes the single source of operational truth.

One addition I'd make, based on everything we've built so far, is to introduce a concept we'll use throughout the system:

An event is something that happened. A notification is a decision to tell someone about that event.

For example, "WSO Completed" is an event. Whether it generates an email, an in-app notification, both, or neither should be configurable. Designing the module around events rather than emails gives you flexibility later—for instance, adding Microsoft Teams or WhatsApp notifications without changing the business logic. I think this architecture will serve the project much better as it grows.