---
id: Context selector
section: components
subsection: menus
cssPrefix: pf-v5-c-context-selector
deprecated: true
---import './context-selector.css'

## Examples

### Basic

```html
<div class="pf-v5-c-context-selector">
  <span id="context-selector-collapsed-example-label" hidden>Selected project:</span>
  <button
    class="pf-v5-c-context-selector__toggle"
    aria-expanded="false"
    id="context-selector-collapsed-example-toggle"
    aria-labelledby="context-selector-collapsed-example-label context-selector-collapsed-example-toggle"
  >
    <span class="pf-v5-c-context-selector__toggle-text">My project</span>
    <span class="pf-v5-c-context-selector__toggle-icon">
      <i class="fas fa-caret-down" aria-hidden="true"></i>
    </span>
  </button>
  <div class="pf-v5-c-context-selector__menu" hidden>
    <div class="pf-v5-c-context-selector__menu-search">
      <div class="pf-v5-c-text-input-group">
        <div class="pf-v5-c-text-input-group__main pf-m-icon">
          <span class="pf-v5-c-text-input-group__text">
            <span class="pf-v5-c-text-input-group__icon">
              <i class="fas fa-fw fa-search"></i>
            </span>
            <input
              class="pf-v5-c-text-input-group__text-input"
              type="text"
              placeholder="Search"
              value
              aria-label="Search input"
            />
          </span>
        </div>
      </div>
    </div>
    <ul class="pf-v5-c-context-selector__menu-list" role="menu">
      <li role="none">
        <a
          class="pf-v5-c-context-selector__menu-list-item"
          href="#"
          role="menuitem"
        >Link</a>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-context-selector__menu-list-item"
          type="button"
          role="menuitem"
        >Action</button>
      </li>
      <li role="none">
        <a
          class="pf-v5-c-context-selector__menu-list-item pf-m-disabled"
          href="#"
          aria-disabled="true"
          tabindex="-1"
          role="menuitem"
        >Disabled link</a>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-context-selector__menu-list-item"
          type="button"
          disabled
          role="menuitem"
        >Disabled action</button>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-context-selector__menu-list-item"
          type="button"
          role="menuitem"
        >My project</button>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-context-selector__menu-list-item"
          type="button"
          role="menuitem"
        >OpenShift cluster</button>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-context-selector__menu-list-item"
          type="button"
          role="menuitem"
        >Production Ansible</button>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-context-selector__menu-list-item"
          type="button"
          role="menuitem"
        >AWS</button>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-context-selector__menu-list-item"
          type="button"
          role="menuitem"
        >Azure</button>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-context-selector__menu-list-item"
          type="button"
          role="menuitem"
        >My project</button>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-context-selector__menu-list-item"
          type="button"
          role="menuitem"
        >OpenShift cluster</button>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-context-selector__menu-list-item"
          type="button"
          role="menuitem"
        >Production Ansible</button>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-context-selector__menu-list-item"
          type="button"
          role="menuitem"
        >AWS</button>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-context-selector__menu-list-item"
          type="button"
          role="menuitem"
        >Azure</button>
      </li>
    </ul>
  </div>
</div>

<div class="pf-v5-c-context-selector pf-m-expanded">
  <span id="context-selector-expanded-example-label" hidden>Selected Project:</span>
  <button
    class="pf-v5-c-context-selector__toggle"
    aria-expanded="true"
    id="context-selector-expanded-example-toggle"
    aria-labelledby="context-selector-expanded-example-label context-selector-expanded-example-toggle"
  >
    <span class="pf-v5-c-context-selector__toggle-text">My project</span>
    <span class="pf-v5-c-context-selector__toggle-icon">
      <i class="fas fa-caret-down" aria-hidden="true"></i>
    </span>
  </button>
  <div class="pf-v5-c-context-selector__menu">
    <div class="pf-v5-c-context-selector__menu-search">
      <div class="pf-v5-c-text-input-group">
        <div class="pf-v5-c-text-input-group__main pf-m-icon">
          <span class="pf-v5-c-text-input-group__text">
            <span class="pf-v5-c-text-input-group__icon">
              <i class="fas fa-fw fa-search"></i>
            </span>
            <input
              class="pf-v5-c-text-input-group__text-input"
              type="text"
              placeholder="Search"
              value
              aria-label="Search input"
            />
          </span>
        </div>
      </div>
    </div>
    <ul class="pf-v5-c-context-selector__menu-list" role="menu">
      <li role="none">
        <a
          class="pf-v5-c-context-selector__menu-list-item"
          href="#"
          role="menuitem"
        >Link</a>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-context-selector__menu-list-item"
          type="button"
          role="menuitem"
        >Action</button>
      </li>
      <li role="none">
        <a
          class="pf-v5-c-context-selector__menu-list-item pf-m-disabled"
          href="#"
          aria-disabled="true"
          tabindex="-1"
          role="menuitem"
        >Disabled link</a>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-context-selector__menu-list-item"
          type="button"
          disabled
          role="menuitem"
        >Disabled action</button>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-context-selector__menu-list-item"
          type="button"
          role="menuitem"
        >My project</button>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-context-selector__menu-list-item"
          type="button"
          role="menuitem"
        >OpenShift cluster</button>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-context-selector__menu-list-item"
          type="button"
          role="menuitem"
        >Production Ansible</button>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-context-selector__menu-list-item"
          type="button"
          role="menuitem"
        >AWS</button>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-context-selector__menu-list-item"
          type="button"
          role="menuitem"
        >Azure</button>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-context-selector__menu-list-item"
          type="button"
          role="menuitem"
        >My project</button>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-context-selector__menu-list-item"
          type="button"
          role="menuitem"
        >OpenShift cluster</button>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-context-selector__menu-list-item"
          type="button"
          role="menuitem"
        >Production Ansible</button>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-context-selector__menu-list-item"
          type="button"
          role="menuitem"
        >AWS</button>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-context-selector__menu-list-item"
          type="button"
          role="menuitem"
        >Azure</button>
      </li>
    </ul>
  </div>
</div>

```

### Plain with text

```html
<div class="pf-v5-c-context-selector">
  <span
    id="context-selector-plain-with-text-collapsed-example-label"
    hidden
  >Selected project:</span>
  <button
    class="pf-v5-c-context-selector__toggle pf-m-text pf-m-plain"
    aria-expanded="false"
    id="context-selector-plain-with-text-collapsed-example-toggle"
    aria-labelledby="context-selector-plain-with-text-collapsed-example-label context-selector-plain-with-text-collapsed-example-toggle"
  >
    <span class="pf-v5-c-context-selector__toggle-text">My project</span>
    <span class="pf-v5-c-context-selector__toggle-icon">
      <i class="fas fa-caret-down" aria-hidden="true"></i>
    </span>
  </button>
  <div class="pf-v5-c-context-selector__menu" hidden>
    <div class="pf-v5-c-context-selector__menu-search">
      <div class="pf-v5-c-text-input-group">
        <div class="pf-v5-c-text-input-group__main pf-m-icon">
          <span class="pf-v5-c-text-input-group__text">
            <span class="pf-v5-c-text-input-group__icon">
              <i class="fas fa-fw fa-search"></i>
            </span>
            <input
              class="pf-v5-c-text-input-group__text-input"
              type="text"
              placeholder="Search"
              value
              aria-label="Search input"
            />
          </span>
        </div>
      </div>
    </div>
    <ul class="pf-v5-c-context-selector__menu-list" role="menu">
      <li role="none">
        <a
          class="pf-v5-c-context-selector__menu-list-item"
          href="#"
          role="menuitem"
        >Link</a>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-context-selector__menu-list-item"
          type="button"
          role="menuitem"
        >Action</button>
      </li>
      <li role="none">
        <a
          class="pf-v5-c-context-selector__menu-list-item pf-m-disabled"
          href="#"
          aria-disabled="true"
          tabindex="-1"
          role="menuitem"
        >Disabled link</a>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-context-selector__menu-list-item"
          type="button"
          disabled
          role="menuitem"
        >Disabled action</button>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-context-selector__menu-list-item"
          type="button"
          role="menuitem"
        >My project</button>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-context-selector__menu-list-item"
          type="button"
          role="menuitem"
        >OpenShift cluster</button>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-context-selector__menu-list-item"
          type="button"
          role="menuitem"
        >Production Ansible</button>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-context-selector__menu-list-item"
          type="button"
          role="menuitem"
        >AWS</button>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-context-selector__menu-list-item"
          type="button"
          role="menuitem"
        >Azure</button>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-context-selector__menu-list-item"
          type="button"
          role="menuitem"
        >My project</button>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-context-selector__menu-list-item"
          type="button"
          role="menuitem"
        >OpenShift cluster</button>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-context-selector__menu-list-item"
          type="button"
          role="menuitem"
        >Production Ansible</button>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-context-selector__menu-list-item"
          type="button"
          role="menuitem"
        >AWS</button>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-context-selector__menu-list-item"
          type="button"
          role="menuitem"
        >Azure</button>
      </li>
    </ul>
  </div>
</div>

<div class="pf-v5-c-context-selector pf-m-expanded">
  <span
    id="context-selector-plain-with-text-expanded-example-label"
    hidden
  >Selected Project:</span>
  <button
    class="pf-v5-c-context-selector__toggle pf-m-text pf-m-plain"
    aria-expanded="true"
    id="context-selector-plain-with-text-expanded-example-toggle"
    aria-labelledby="context-selector-plain-with-text-expanded-example-label context-selector-plain-with-text-expanded-example-toggle"
  >
    <span class="pf-v5-c-context-selector__toggle-text">My project</span>
    <span class="pf-v5-c-context-selector__toggle-icon">
      <i class="fas fa-caret-down" aria-hidden="true"></i>
    </span>
  </button>
  <div class="pf-v5-c-context-selector__menu">
    <div class="pf-v5-c-context-selector__menu-search">
      <div class="pf-v5-c-text-input-group">
        <div class="pf-v5-c-text-input-group__main pf-m-icon">
          <span class="pf-v5-c-text-input-group__text">
            <span class="pf-v5-c-text-input-group__icon">
              <i class="fas fa-fw fa-search"></i>
            </span>
            <input
              class="pf-v5-c-text-input-group__text-input"
              type="text"
              placeholder="Search"
              value
              aria-label="Search input"
            />
          </span>
        </div>
      </div>
    </div>
    <ul class="pf-v5-c-context-selector__menu-list" role="menu">
      <li role="none">
        <a
          class="pf-v5-c-context-selector__menu-list-item"
          href="#"
          role="menuitem"
        >Link</a>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-context-selector__menu-list-item"
          type="button"
          role="menuitem"
        >Action</button>
      </li>
      <li role="none">
        <a
          class="pf-v5-c-context-selector__menu-list-item pf-m-disabled"
          href="#"
          aria-disabled="true"
          tabindex="-1"
          role="menuitem"
        >Disabled link</a>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-context-selector__menu-list-item"
          type="button"
          disabled
          role="menuitem"
        >Disabled action</button>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-context-selector__menu-list-item"
          type="button"
          role="menuitem"
        >My project</button>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-context-selector__menu-list-item"
          type="button"
          role="menuitem"
        >OpenShift cluster</button>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-context-selector__menu-list-item"
          type="button"
          role="menuitem"
        >Production Ansible</button>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-context-selector__menu-list-item"
          type="button"
          role="menuitem"
        >AWS</button>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-context-selector__menu-list-item"
          type="button"
          role="menuitem"
        >Azure</button>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-context-selector__menu-list-item"
          type="button"
          role="menuitem"
        >My project</button>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-context-selector__menu-list-item"
          type="button"
          role="menuitem"
        >OpenShift cluster</button>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-context-selector__menu-list-item"
          type="button"
          role="menuitem"
        >Production Ansible</button>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-context-selector__menu-list-item"
          type="button"
          role="menuitem"
        >AWS</button>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-context-selector__menu-list-item"
          type="button"
          role="menuitem"
        >Azure</button>
      </li>
    </ul>
  </div>
</div>

```

### With footer

```html
<div class="pf-v5-c-context-selector pf-m-expanded">
  <span id="context-selector-with-footer-example-label" hidden>Selected Project:</span>
  <button
    class="pf-v5-c-context-selector__toggle"
    aria-expanded="true"
    id="context-selector-with-footer-example-toggle"
    aria-labelledby="context-selector-with-footer-example-label context-selector-with-footer-example-toggle"
  >
    <span class="pf-v5-c-context-selector__toggle-text">My project</span>
    <span class="pf-v5-c-context-selector__toggle-icon">
      <i class="fas fa-caret-down" aria-hidden="true"></i>
    </span>
  </button>
  <div class="pf-v5-c-context-selector__menu">
    <div class="pf-v5-c-context-selector__menu-search">
      <div class="pf-v5-c-text-input-group">
        <div class="pf-v5-c-text-input-group__main pf-m-icon">
          <span class="pf-v5-c-text-input-group__text">
            <span class="pf-v5-c-text-input-group__icon">
              <i class="fas fa-fw fa-search"></i>
            </span>
            <input
              class="pf-v5-c-text-input-group__text-input"
              type="text"
              placeholder="Search"
              value
              aria-label="Search input"
            />
          </span>
        </div>
      </div>
    </div>
    <ul class="pf-v5-c-context-selector__menu-list" role="menu">
      <li role="none">
        <a
          class="pf-v5-c-context-selector__menu-list-item"
          href="#"
          role="menuitem"
        >Link</a>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-context-selector__menu-list-item"
          type="button"
          role="menuitem"
        >Action</button>
      </li>
      <li role="none">
        <a
          class="pf-v5-c-context-selector__menu-list-item pf-m-disabled"
          href="#"
          aria-disabled="true"
          tabindex="-1"
          role="menuitem"
        >Disabled link</a>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-context-selector__menu-list-item"
          type="button"
          disabled
          role="menuitem"
        >Disabled action</button>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-context-selector__menu-list-item"
          type="button"
          role="menuitem"
        >My project</button>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-context-selector__menu-list-item"
          type="button"
          role="menuitem"
        >OpenShift cluster</button>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-context-selector__menu-list-item"
          type="button"
          role="menuitem"
        >Production Ansible</button>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-context-selector__menu-list-item"
          type="button"
          role="menuitem"
        >AWS</button>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-context-selector__menu-list-item"
          type="button"
          role="menuitem"
        >Azure</button>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-context-selector__menu-list-item"
          type="button"
          role="menuitem"
        >My project</button>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-context-selector__menu-list-item"
          type="button"
          role="menuitem"
        >OpenShift cluster</button>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-context-selector__menu-list-item"
          type="button"
          role="menuitem"
        >Production Ansible</button>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-context-selector__menu-list-item"
          type="button"
          role="menuitem"
        >AWS</button>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-context-selector__menu-list-item"
          type="button"
          role="menuitem"
        >Azure</button>
      </li>
    </ul>

    <div class="pf-v5-c-context-selector__menu-footer">
      <button
        class="pf-v5-c-button pf-m-secondary"
        type="button"
      >Manage projects</button>
    </div>
  </div>
</div>

<div class="pf-v5-c-context-selector pf-m-expanded">
  <span
    id="context-selector-with-footer-example-two-label"
    hidden
  >Selected Project:</span>
  <button
    class="pf-v5-c-context-selector__toggle"
    aria-expanded="true"
    id="context-selector-with-footer-example-two-toggle"
    aria-labelledby="context-selector-with-footer-example-two-label context-selector-with-footer-example-two-toggle"
  >
    <span class="pf-v5-c-context-selector__toggle-text">My project</span>
    <span class="pf-v5-c-context-selector__toggle-icon">
      <i class="fas fa-caret-down" aria-hidden="true"></i>
    </span>
  </button>
  <div class="pf-v5-c-context-selector__menu">
    <div class="pf-v5-c-context-selector__menu-search">
      <div class="pf-v5-c-text-input-group">
        <div class="pf-v5-c-text-input-group__main pf-m-icon">
          <span class="pf-v5-c-text-input-group__text">
            <span class="pf-v5-c-text-input-group__icon">
              <i class="fas fa-fw fa-search"></i>
            </span>
            <input
              class="pf-v5-c-text-input-group__text-input"
              type="text"
              placeholder="Search"
              value
              aria-label="Search input"
            />
          </span>
        </div>
      </div>
    </div>
    <ul class="pf-v5-c-context-selector__menu-list" role="menu">
      <li role="none">
        <a
          class="pf-v5-c-context-selector__menu-list-item"
          href="#"
          role="menuitem"
        >Link</a>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-context-selector__menu-list-item"
          type="button"
          role="menuitem"
        >Action</button>
      </li>
      <li role="none">
        <a
          class="pf-v5-c-context-selector__menu-list-item pf-m-disabled"
          href="#"
          aria-disabled="true"
          tabindex="-1"
          role="menuitem"
        >Disabled link</a>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-context-selector__menu-list-item"
          type="button"
          disabled
          role="menuitem"
        >Disabled action</button>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-context-selector__menu-list-item"
          type="button"
          role="menuitem"
        >My project</button>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-context-selector__menu-list-item"
          type="button"
          role="menuitem"
        >OpenShift cluster</button>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-context-selector__menu-list-item"
          type="button"
          role="menuitem"
        >Production Ansible</button>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-context-selector__menu-list-item"
          type="button"
          role="menuitem"
        >AWS</button>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-context-selector__menu-list-item"
          type="button"
          role="menuitem"
        >Azure</button>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-context-selector__menu-list-item"
          type="button"
          role="menuitem"
        >My project</button>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-context-selector__menu-list-item"
          type="button"
          role="menuitem"
        >OpenShift cluster</button>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-context-selector__menu-list-item"
          type="button"
          role="menuitem"
        >Production Ansible</button>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-context-selector__menu-list-item"
          type="button"
          role="menuitem"
        >AWS</button>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-context-selector__menu-list-item"
          type="button"
          role="menuitem"
        >Azure</button>
      </li>
    </ul>

    <div class="pf-v5-c-context-selector__menu-footer">
      <button
        class="pf-v5-c-button pf-m-link pf-m-inline"
        type="button"
      >Manage projects</button>
    </div>
  </div>
</div>

```

## Documentation

### Accessibility

| Class | Applied to | Outcome |
| -- | -- | -- |
| `aria-expanded="false"` | `.pf-v5-c-context-selector__toggle` |  Indicates that the menu is hidden. |
| `aria-expanded="true"` | `.pf-v5-c-context-selector__toggle` |  Indicates that the menu is visible. |
| `aria-hidden="true"` | `.pf-v5-c-context-selector__toggle-icon > *` | Hides the icon from assistive technologies. |
| `disabled` | `button.pf-v5-c-context-selector__menu-item` | When the menu item uses a button element, indicates that it is unavailable and removes it from keyboard focus. |
| `aria-disabled="true"` | `a.pf-v5-c-context-selector__menu-item` | When the menu item uses a link element, indicates that it is unavailable. |
| `tabindex="-1"` | `a.pf-v5-c-context-selector__menu-item` | When the menu item uses a link element, removes it from keyboard focus. |

### Usage

| Class | Applied to | Outcome |
| -- | -- | -- |
| `.pf-v5-c-context-selector` | `<div>` | Initiates a context selector.|
| `.pf-v5-c-context-selector__toggle` | `<button>` | Initiates a toggle. |
| `.pf-v5-c-context-selector__toggle-text` | `<span>` | Initiates text inside the toggle. |
| `.pf-v5-c-context-selector__toggle-icon` | `<span>` | Inititiates the toggle icon wrapper. |
| `.pf-v5-c-context-selector__menu` | `<div>` | Initiaties a menu. |
| `.pf-v5-c-context-selector__menu-search` | `<div>` | Initiates a container for the search input group. |
| `.pf-v5-c-context-selector__menu-list` | `<ul>` | Initiaties an unordered list of menu items that sits under the input container. |
| `.pf-v5-c-context-selector__menu-footer` | `<div>` | Initiaties a menu footer. |
| `.pf-v5-c-context-selector__menu-list-item` | `<li>` | Initiaties a menu item. |
| `.pf-m-expanded` | `.pf-v5-c-context-selector` | Modifies for the expanded state. |
| `.pf-m-active` | `.pf-v5-c-context-selector__toggle` | Forces display of the active state of the toggle. |
| `.pf-m-plain.pf-m-text` | `.pf-v5-c-context-selector__toggle` | Modifies the context selector toggle for plain text styles. |
| `.pf-m-disabled` | `a.pf-v5-c-context-selector__menu-list-item` | Modifies an item for the disabled state.|
| `.pf-m-full-height` | `.pf-v5-c-context-selector` | Modifies a context selector to full height of parent. See masthead for use. |
| `.pf-m-large` | `.pf-v5-c-context-selector` | Modifies toggle height to be large. |
| `.pf-m-page-insets` | `.pf-v5-c-context-selector` | Modifies toggle and menu insets to match the responsive page chrome insets. |
