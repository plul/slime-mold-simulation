---
id: Notification badge
section: components
cssPrefix: pf-v5-c-notification-badge
---import './NotificationBadge.css'

## Examples

### Basic

```html
<div class="pf-v5-t-dark">
  <button
    class="pf-v5-c-button pf-m-plain"
    type="button"
    aria-label="Notifications"
  >
    <span class="pf-v5-c-notification-badge pf-m-read">
      <i class="pf-v5-pficon-bell" aria-hidden="true"></i>
    </span>
  </button>
  <button
    class="pf-v5-c-button pf-m-plain"
    type="button"
    aria-label="Unread notifications"
  >
    <span class="pf-v5-c-notification-badge pf-m-unread">
      <i class="pf-v5-pficon-bell" aria-hidden="true"></i>
    </span>
  </button>
  <button
    class="pf-v5-c-button pf-m-plain"
    type="button"
    aria-label="Attention notifications"
  >
    <span class="pf-v5-c-notification-badge pf-m-attention">
      <i class="pf-v5-pficon-attention-bell" aria-hidden="true"></i>
    </span>
  </button>
  <br />
  <br />
  <button class="pf-v5-c-button pf-m-plain" type="button" aria-label="Tasks">
    <span class="pf-v5-c-notification-badge pf-m-read">
      <i class="pf-v5-pficon-task" aria-hidden="true"></i>
    </span>
  </button>
  <button
    class="pf-v5-c-button pf-m-plain"
    type="button"
    aria-label="Unread tasks"
  >
    <span class="pf-v5-c-notification-badge pf-m-unread">
      <i class="pf-v5-pficon-task" aria-hidden="true"></i>
    </span>
  </button>
  <button
    class="pf-v5-c-button pf-m-plain"
    type="button"
    aria-label="Attention tasks"
  >
    <span class="pf-v5-c-notification-badge pf-m-attention">
      <i class="pf-v5-pficon-task" aria-hidden="true"></i>
    </span>
  </button>
</div>

```

### With count

```html
<div class="pf-v5-t-dark">
  <button
    class="pf-v5-c-button pf-m-plain"
    type="button"
    aria-label="24 notifications"
  >
    <span class="pf-v5-c-notification-badge pf-m-read">
      <i class="pf-v5-pficon-bell" aria-hidden="true"></i>
      <span class="pf-v5-c-notification-badge__count">24</span>
    </span>
  </button>
  <button
    class="pf-v5-c-button pf-m-plain"
    type="button"
    aria-label="24 unread notifications"
  >
    <span class="pf-v5-c-notification-badge pf-m-unread">
      <i class="pf-v5-pficon-bell" aria-hidden="true"></i>
      <span class="pf-v5-c-notification-badge__count">24</span>
    </span>
  </button>
  <button
    class="pf-v5-c-button pf-m-plain"
    type="button"
    aria-label="24 attention notifications"
  >
    <span class="pf-v5-c-notification-badge pf-m-attention">
      <i class="pf-v5-pficon-attention-bell" aria-hidden="true"></i>
      <span class="pf-v5-c-notification-badge__count">24</span>
    </span>
  </button>
  <br />
  <br />
  <button class="pf-v5-c-button pf-m-plain" type="button" aria-label="24 tasks">
    <span class="pf-v5-c-notification-badge pf-m-read">
      <i class="pf-v5-pficon-task" aria-hidden="true"></i>
      <span class="pf-v5-c-notification-badge__count">24</span>
    </span>
  </button>
  <button
    class="pf-v5-c-button pf-m-plain"
    type="button"
    aria-label="24 unread tasks"
  >
    <span class="pf-v5-c-notification-badge pf-m-unread">
      <i class="pf-v5-pficon-task" aria-hidden="true"></i>
      <span class="pf-v5-c-notification-badge__count">24</span>
    </span>
  </button>
  <button
    class="pf-v5-c-button pf-m-plain"
    type="button"
    aria-label="24 attention tasks"
  >
    <span class="pf-v5-c-notification-badge pf-m-attention">
      <i class="pf-v5-pficon-task" aria-hidden="true"></i>
      <span class="pf-v5-c-notification-badge__count">24</span>
    </span>
  </button>
</div>

```

### Expanded

```html
<div class="pf-v5-t-dark">
  <button
    class="pf-v5-c-button pf-m-plain"
    type="button"
    aria-expanded="true"
    aria-label="Notifications"
  >
    <span class="pf-v5-c-notification-badge pf-m-read pf-m-expanded">
      <i class="pf-v5-pficon-bell" aria-hidden="true"></i>
    </span>
  </button>
  <button
    class="pf-v5-c-button pf-m-plain"
    type="button"
    aria-expanded="true"
    aria-label="Unread notifications"
  >
    <span class="pf-v5-c-notification-badge pf-m-unread pf-m-expanded">
      <i class="pf-v5-pficon-bell" aria-hidden="true"></i>
    </span>
  </button>
  <button
    class="pf-v5-c-button pf-m-plain"
    type="button"
    aria-expanded="true"
    aria-label="Attention notifications"
  >
    <span class="pf-v5-c-notification-badge pf-m-attention pf-m-expanded">
      <i class="pf-v5-pficon-attention-bell" aria-hidden="true"></i>
    </span>
  </button>
  <br />
  <br />
  <button
    class="pf-v5-c-button pf-m-plain"
    type="button"
    aria-expanded="true"
    aria-label="Tasks"
  >
    <span class="pf-v5-c-notification-badge pf-m-read pf-m-expanded">
      <i class="pf-v5-pficon-task" aria-hidden="true"></i>
    </span>
  </button>
  <button
    class="pf-v5-c-button pf-m-plain"
    type="button"
    aria-expanded="true"
    aria-label="Unread tasks"
  >
    <span class="pf-v5-c-notification-badge pf-m-unread pf-m-expanded">
      <i class="pf-v5-pficon-task" aria-hidden="true"></i>
    </span>
  </button>
  <button
    class="pf-v5-c-button pf-m-plain"
    type="button"
    aria-expanded="true"
    aria-label="Attention tasks"
  >
    <span class="pf-v5-c-notification-badge pf-m-attention pf-m-expanded">
      <i class="pf-v5-pficon-task" aria-hidden="true"></i>
    </span>
  </button>
</div>

```

## Documentation

### Overview

Always add a modifier class to indicate read, unread, or attention state. Never use the class `.pf-v5-c-notification-badge` on its own. This component is designed to be used within a dark component such as the [masthead](/components/masthead).

### Accessibility

Be sure that the component associated with this indicator handles screen reader text indicating read or unread notifications.

### Usage

| Class | Applied to | Outcome |
| -- | -- | -- |
| `.pf-v5-c-notification-badge` | `<div>` |  Initiates a notification badge. **Always use it with a modifier class.** |
| `.pf-v5-c-notification-badge__count` | `<span>` |  Initiates a notification badge count. |
| `.pf-m-read` | `.pf-v5-c-notification-badge` |  Applies read notification badge styling. |
| `.pf-m-unread` | `.pf-v5-c-notification-badge` |  Applies unread notification badge styling. |
| `.pf-m-attention` | `.pf-v5-c-notification-badge` |  Applies attention notification badge styling. |
| `.pf-m-expanded` | `.pf-v5-c-notification-badge` |  Applies expanded notification badge styling. |
