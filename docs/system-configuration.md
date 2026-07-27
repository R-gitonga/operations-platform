System Configuration Module
Purpose

The System Configuration Module provides a single location where administrators control how the WSO Tracker behaves without requiring software changes.

Instead of hardcoding business rules into the application, configurable features are stored in the database and managed through the Settings interface.

The goal is that future operational changes should be made by an administrator rather than a developer.

Design Philosophy

The application should separate

Business Logic
Business Configuration

Business logic answers:

"What can happen?"

Business configuration answers:

"How should our company handle it?"

Example

Business Logic

A Workshop Order can be cancelled.

Configuration

Who should be informed when it is cancelled?

Module Structure

Initially, the Settings module will contain only one section.

Settings

Notifications

As the system grows it will eventually contain

Settings

Notifications
Email Templates
Categories
Departments
Roles
Users
Permissions
Production Statuses
Company Information
ERP Integration
System Preferences
Phase 1

Notification Settings

The first release focuses entirely on notification management.

The system should already know how to send notifications.

Administrators decide

whether notifications are enabled
who receives them
which events generate them
Notification Architecture

The notification system consists of five parts.

1.

Business Event

Something important happens.

Examples

WSO Created
WSO Cancelled
WSO Reactivated
WSO Completed
Attachment Uploaded

This is simply an event.

Nothing has been sent yet.

2.

Notification Rules

The system checks

Should this event generate a notification?

Example

WSO Completed

Email Enabled?

YES

Continue.

Attachment Uploaded

Email Enabled?

NO

Stop.

3.

Recipients

The system determines

Who should receive this notification?

Recipients should never be hardcoded.

Instead

Completed Orders

↓

Production Manager

↓

Retail Manager

↓

Managing Director

Later

Groups

Production Team

Retail Team

Management Team

4.

Notification Builder

The notification engine prepares

Subject

Body

Links

Attachments

This will eventually support templates.

5.

Delivery

Finally the notification is sent through

Email

Later

Teams

Slack

SMS

WhatsApp

Push Notifications

Configurable Events

Every business event should be configurable.

Event	Email	In-App	Enabled
WSO Created	Yes	Yes	Yes
WSO Cancelled	Yes	Yes	Yes
WSO Reactivated	Yes	Yes	Yes
WSO Completed	Yes	Yes	Yes
Fully Received	Yes	Yes	Yes
Attachment Uploaded	Optional	Yes	Yes
Notification Channels

Every event can support multiple delivery methods.

Example

Completed

☑ Email

☑ In-App

☐ SMS

☐ Teams

Email Recipients

Each event has its own recipient list.

Example

Completed

Retail Manager

Factory Manager

Managing Director

Cancelled

Retail

Production

Operations

This makes notifications independent.

Notification Templates

Emails should not be written inside Rust code.

Instead

Each event references a template.

Example

Template

Workshop Order Completed

Subject

Workshop Order {{wso_number}} Completed

Body

Hello,

Workshop Order {{wso_number}} has now been completed.

Outstanding Quantity

{{balance}}

Completed Date

{{completed_date}}

Regards

WSO Tracker

Settings Screen

Notifications

Workshop Order Created

☑ Enabled

☑ Email

☑ In-App

Recipients

Production Manager

Factory Manager

Workshop Order Completed

☑ Enabled

☑ Email

☑ In-App

Recipients

Retail Manager

Managing Director

Workshop Order Cancelled

☑ Enabled

☑ Email

☑ In-App

Recipients

Retail

Production

Future Recipient Management

Recipients should eventually come from Roles instead of individual users.

Instead of

john@company.com

mary@company.com

We assign

Production Manager

Retail Manager

Operations

The role determines who currently occupies that position.

When staff change, no notification settings need updating.

Database Direction

Although we won't build the schema immediately, the design naturally points toward a few core tables:

Table	Purpose
notification_events	Defines each event the system understands (e.g. WSO Completed, WSO Cancelled).
notification_settings	Stores whether an event is enabled and which delivery channels are active.
notification_recipients	Associates events with recipients, roles, or groups.
notification_templates	Stores email subjects and message bodies with placeholders.
notification_log	Records every notification attempt, delivery status, timestamps, and any failures.

By keeping these concerns separate, changing recipients or email content won't require code changes.

Suggested Technology Stack
Backend
Axum — API endpoints for settings and notifications.
PostgreSQL — Stores notification configuration, templates, recipients, and logs.
Tokio — Executes email sending asynchronously so users don't wait for emails to be delivered.
lettre — Rust email library for SMTP integration (Gmail, Office 365, Exchange, SendGrid SMTP, etc.).
Frontend
React
TanStack Query
shadcn/ui
Existing Settings page with tabs (Notifications first, more sections later).
Email Providers

The application should communicate through SMTP, allowing organizations to use their own provider:

Microsoft 365
Gmail Workspace
Exchange
SendGrid
Mailgun
Amazon SES

This keeps the WSO Tracker independent of any single email service.

Notification Lifecycle
User completes WSO
        │
        ▼
Business Event Raised
        │
        ▼
Load Notification Settings
        │
        ▼
Is event enabled?
        │
    No ─────────► Stop
        │
       Yes
        │
        ▼
Resolve recipients
        │
        ▼
Build email from template
        │
        ▼
Send notification
        │
        ▼
Save notification log
        │
        ▼
Show in-app notification
Guiding Principles
Events drive notifications. Business actions generate events; events may generate one or more notifications.
Configuration over code. Administrators should control behavior through Settings rather than code changes.
Roles over people. Notifications should target business roles or groups whenever possible.
Templates over hardcoded text. Email content should be editable without recompiling the application.
Every notification is auditable. Every send attempt—successful or failed—should be recorded.
Extensible by design. Adding Microsoft Teams, Slack, SMS, or other channels should require implementing a new delivery channel, not rewriting business logic.

I believe this is the point where the project starts transitioning from a WSO Tracker into an operations platform. Once the Settings module exists as the central place for configurable behavior, adding future features—approvals, departments, user roles, ERP synchronization, scheduled reports, or integrations—becomes a matter of plugging them into the same configuration framework rather than scattering settings across the codebase. That foundation will pay dividends as the system grows.