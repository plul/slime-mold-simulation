---
id: Application launcher
section: components
subsection: menus
cssPrefix: pf-v5-c-app-launcher
deprecated: true
---import './application-launcher.css'

## Examples

### Collapsed

```html
<nav
  class="pf-v5-c-app-launcher"
  aria-label="Application launcher"
  id="application-launcher-collapsed"
>
  <button
    class="pf-v5-c-app-launcher__toggle"
    type="button"
    id="application-launcher-collapsed-button"
    aria-expanded="false"
    aria-label="Application launcher"
  >
    <i class="fas fa-th" aria-hidden="true"></i>
  </button>
  <ul
    class="pf-v5-c-app-launcher__menu"
    aria-labelledby="application-launcher-collapsed-button"
    role="menu"
    hidden
  >
    <li role="none">
      <a class="pf-v5-c-app-launcher__menu-item" role="menuitem" href="#">Link</a>
    </li>
    <li role="none">
      <button
        class="pf-v5-c-app-launcher__menu-item"
        role="menuitem"
        type="button"
      >Action</button>
    </li>
    <li class="pf-v5-c-divider" role="separator"></li>
    <li role="none">
      <a
        class="pf-v5-c-app-launcher__menu-item pf-m-disabled"
        role="menuitem"
        href="#"
        aria-disabled="true"
        tabindex="-1"
      >Disabled link</a>
    </li>
  </ul>
</nav>

```

### Disabled

```html
<nav
  class="pf-v5-c-app-launcher"
  aria-label="Application launcher"
  id="application-launcher-disabled"
>
  <button
    class="pf-v5-c-app-launcher__toggle"
    type="button"
    id="application-launcher-disabled-button"
    aria-expanded="false"
    aria-label="Application launcher"
    disabled
  >
    <i class="fas fa-th" aria-hidden="true"></i>
  </button>
  <ul
    class="pf-v5-c-app-launcher__menu"
    aria-labelledby="application-launcher-disabled-button"
    role="menu"
    hidden
  >
    <li role="none">
      <a class="pf-v5-c-app-launcher__menu-item" role="menuitem" href="#">Link</a>
    </li>
    <li role="none">
      <button
        class="pf-v5-c-app-launcher__menu-item"
        role="menuitem"
        type="button"
      >Action</button>
    </li>
    <li class="pf-v5-c-divider" role="separator"></li>
    <li role="none">
      <a
        class="pf-v5-c-app-launcher__menu-item pf-m-disabled"
        role="menuitem"
        href="#"
        aria-disabled="true"
        tabindex="-1"
      >Disabled link</a>
    </li>
  </ul>
</nav>

```

### Expanded

```html
<nav
  class="pf-v5-c-app-launcher pf-m-expanded"
  aria-label="Application launcher"
  id="application-launcher-expanded"
>
  <button
    class="pf-v5-c-app-launcher__toggle"
    type="button"
    id="application-launcher-expanded-button"
    aria-expanded="true"
    aria-label="Application launcher"
  >
    <i class="fas fa-th" aria-hidden="true"></i>
  </button>
  <ul
    class="pf-v5-c-app-launcher__menu"
    aria-labelledby="application-launcher-expanded-button"
    role="menu"
  >
    <li role="none">
      <a class="pf-v5-c-app-launcher__menu-item" role="menuitem" href="#">Link</a>
    </li>
    <li role="none">
      <button
        class="pf-v5-c-app-launcher__menu-item"
        role="menuitem"
        type="button"
      >Action</button>
    </li>
    <li class="pf-v5-c-divider" role="separator"></li>
    <li role="none">
      <a
        class="pf-v5-c-app-launcher__menu-item pf-m-disabled"
        role="menuitem"
        href="#"
        aria-disabled="true"
        tabindex="-1"
      >Disabled link</a>
    </li>
  </ul>
</nav>

```

### Aligned right

```html
<nav
  class="pf-v5-c-app-launcher pf-m-expanded"
  aria-label="Application launcher"
  id="application-launcher-aligned-right"
>
  <button
    class="pf-v5-c-app-launcher__toggle"
    type="button"
    id="application-launcher-aligned-right-button"
    aria-expanded="true"
    aria-label="Application launcher"
  >
    <i class="fas fa-th" aria-hidden="true"></i>
  </button>
  <ul
    class="pf-v5-c-app-launcher__menu pf-m-align-right"
    aria-labelledby="application-launcher-aligned-right-button"
    role="menu"
  >
    <li role="none">
      <a class="pf-v5-c-app-launcher__menu-item" role="menuitem" href="#">Link</a>
    </li>
    <li role="none">
      <button
        class="pf-v5-c-app-launcher__menu-item"
        role="menuitem"
        type="button"
      >Action</button>
    </li>
    <li class="pf-v5-c-divider" role="separator"></li>
    <li role="none">
      <a
        class="pf-v5-c-app-launcher__menu-item pf-m-disabled"
        role="menuitem"
        href="#"
        aria-disabled="true"
        tabindex="-1"
      >Disabled link</a>
    </li>
  </ul>
</nav>

```

### Aligned top

```html
<nav
  class="pf-v5-c-app-launcher pf-m-expanded pf-m-top"
  aria-label="Application launcher"
  id="application-launcher-aligned-top"
>
  <button
    class="pf-v5-c-app-launcher__toggle"
    type="button"
    id="application-launcher-aligned-top-button"
    aria-expanded="true"
    aria-label="Application launcher"
  >
    <i class="fas fa-th" aria-hidden="true"></i>
  </button>
  <ul
    class="pf-v5-c-app-launcher__menu"
    aria-labelledby="application-launcher-aligned-top-button"
    role="menu"
  >
    <li role="none">
      <a class="pf-v5-c-app-launcher__menu-item" role="menuitem" href="#">Link</a>
    </li>
    <li role="none">
      <button
        class="pf-v5-c-app-launcher__menu-item"
        role="menuitem"
        type="button"
      >Action</button>
    </li>
    <li class="pf-v5-c-divider" role="separator"></li>
    <li role="none">
      <a
        class="pf-v5-c-app-launcher__menu-item pf-m-disabled"
        role="menuitem"
        href="#"
        aria-disabled="true"
        tabindex="-1"
      >Disabled link</a>
    </li>
  </ul>
</nav>

```

### With sections and dividers between sections

```html
<nav
  class="pf-v5-c-app-launcher pf-m-expanded"
  aria-label="Application launcher"
  id="application-launcher-divided-sections"
>
  <button
    class="pf-v5-c-app-launcher__toggle"
    type="button"
    id="application-launcher-divided-sections-button"
    aria-expanded="true"
    aria-label="Application launcher"
  >
    <i class="fas fa-th" aria-hidden="true"></i>
  </button>
  <div class="pf-v5-c-app-launcher__menu">
    <section class="pf-v5-c-app-launcher__group">
      <ul role="menu">
        <li role="none">
          <a
            class="pf-v5-c-app-launcher__menu-item"
            role="menuitem"
            href="#"
          >Link not in group</a>
        </li>
      </ul>
    </section>
    <hr class="pf-v5-c-divider" />
    <section class="pf-v5-c-app-launcher__group">
      <h1 class="pf-v5-c-app-launcher__group-title">Group 1</h1>
      <ul role="menu">
        <li role="none">
          <a
            class="pf-v5-c-app-launcher__menu-item"
            role="menuitem"
            href="#"
          >Group 1 link</a>
        </li>
        <li role="none">
          <a
            class="pf-v5-c-app-launcher__menu-item"
            role="menuitem"
            href="#"
          >Group 1 link</a>
        </li>
      </ul>
    </section>
    <hr class="pf-v5-c-divider" />
    <section class="pf-v5-c-app-launcher__group">
      <h1 class="pf-v5-c-app-launcher__group-title">Group 2</h1>
      <ul role="menu">
        <li role="none">
          <a
            class="pf-v5-c-app-launcher__menu-item"
            role="menuitem"
            href="#"
          >Group 2 link</a>
        </li>
        <li role="none">
          <a
            class="pf-v5-c-app-launcher__menu-item"
            role="menuitem"
            href="#"
          >Group 2 link</a>
        </li>
      </ul>
    </section>
  </div>
</nav>

```

### With sections and dividers between items

```html
<nav
  class="pf-v5-c-app-launcher pf-m-expanded"
  aria-label="Application launcher"
  id="application-launcher-divided-items"
>
  <button
    class="pf-v5-c-app-launcher__toggle"
    type="button"
    id="application-launcher-divided-items-button"
    aria-expanded="true"
    aria-label="Application launcher"
  >
    <i class="fas fa-th" aria-hidden="true"></i>
  </button>
  <div class="pf-v5-c-app-launcher__menu">
    <section class="pf-v5-c-app-launcher__group">
      <ul role="menu">
        <li role="none">
          <a
            class="pf-v5-c-app-launcher__menu-item"
            role="menuitem"
            href="#"
          >Link not in group</a>
        </li>
        <li class="pf-v5-c-divider" role="separator"></li>
      </ul>
    </section>
    <section class="pf-v5-c-app-launcher__group">
      <h1 class="pf-v5-c-app-launcher__group-title">Group 1</h1>
      <ul role="menu">
        <li role="none">
          <a
            class="pf-v5-c-app-launcher__menu-item"
            role="menuitem"
            href="#"
          >Group 1 link</a>
        </li>
        <li role="none">
          <a
            class="pf-v5-c-app-launcher__menu-item"
            role="menuitem"
            href="#"
          >Group 1 link</a>
        </li>
        <li class="pf-v5-c-divider" role="separator"></li>
      </ul>
    </section>
    <section class="pf-v5-c-app-launcher__group">
      <h1 class="pf-v5-c-app-launcher__group-title">Group 2</h1>
      <ul role="menu">
        <li role="none">
          <a
            class="pf-v5-c-app-launcher__menu-item"
            role="menuitem"
            href="#"
          >Group 2 link</a>
        </li>
        <li role="none">
          <a
            class="pf-v5-c-app-launcher__menu-item"
            role="menuitem"
            href="#"
          >Group 2 link</a>
        </li>
      </ul>
    </section>
  </div>
</nav>

```

### With sections, dividers, icons, and external links

```html
<nav
  class="pf-v5-c-app-launcher pf-m-expanded"
  aria-label="Application launcher"
  id="application-launcher-sections-dividers-icons-links"
>
  <button
    class="pf-v5-c-app-launcher__toggle"
    type="button"
    id="application-launcher-sections-dividers-icons-links-button"
    aria-expanded="true"
    aria-label="Application launcher"
  >
    <i class="fas fa-th" aria-hidden="true"></i>
  </button>
  <div class="pf-v5-c-app-launcher__menu">
    <section class="pf-v5-c-app-launcher__group">
      <ul role="menu">
        <li role="none">
          <a class="pf-v5-c-app-launcher__menu-item" role="menuitem" href="#">
            <span class="pf-v5-c-app-launcher__menu-item-icon">
              <img src="/assets/images/pf-logo-small.svg" alt="PatternFly logo" />
            </span>
            Link not in group
          </a>
        </li>
      </ul>
    </section>
    <li class="pf-v5-c-divider" role="separator"></li>
    <section class="pf-v5-c-app-launcher__group">
      <h1 class="pf-v5-c-app-launcher__group-title">Group 1</h1>
      <ul role="menu">
        <li role="none">
          <a
            class="pf-v5-c-app-launcher__menu-item pf-m-external"
            role="menuitem"
            href="#"
            target="_blank"
          >
            <span class="pf-v5-c-app-launcher__menu-item-icon">
              <img src="/assets/images/pf-logo-small.svg" alt="PatternFly logo" />
            </span>
            Group 1 link
            <span
              class="pf-v5-c-app-launcher__menu-item-external-icon"
            >
              <i class="fas fa-external-link-alt" aria-hidden="true"></i>
            </span>
            <span class="pf-v5-screen-reader">(opens new window)</span>
          </a>
        </li>
        <li role="none">
          <a
            class="pf-v5-c-app-launcher__menu-item pf-m-external"
            role="menuitem"
            href="#"
            target="_blank"
          >
            <span class="pf-v5-c-app-launcher__menu-item-icon">
              <img src="/assets/images/pf-logo-small.svg" alt="PatternFly logo" />
            </span>
            Group 1 link
            <span
              class="pf-v5-c-app-launcher__menu-item-external-icon"
            >
              <i class="fas fa-external-link-alt" aria-hidden="true"></i>
            </span>
            <span class="pf-v5-screen-reader">(opens new window)</span>
          </a>
        </li>
        <li class="pf-v5-c-divider" role="separator"></li>
      </ul>
    </section>
    <section class="pf-v5-c-app-launcher__group">
      <h1 class="pf-v5-c-app-launcher__group-title">Group 2</h1>
      <ul role="menu">
        <li role="none">
          <a
            class="pf-v5-c-app-launcher__menu-item pf-m-external"
            role="menuitem"
            href="#"
            target="_blank"
          >
            <span class="pf-v5-c-app-launcher__menu-item-icon">
              <img src="/assets/images/pf-logo-small.svg" alt="PatternFly logo" />
            </span>
            Group 2 link
            <span
              class="pf-v5-c-app-launcher__menu-item-external-icon"
            >
              <i class="fas fa-external-link-alt" aria-hidden="true"></i>
            </span>
            <span class="pf-v5-screen-reader">(opens new window)</span>
          </a>
        </li>
        <li role="none">
          <a
            class="pf-v5-c-app-launcher__menu-item pf-m-external"
            role="menuitem"
            href="#"
            target="_blank"
          >
            <span class="pf-v5-c-app-launcher__menu-item-icon">
              <img src="/assets/images/pf-logo-small.svg" alt="PatternFly logo" />
            </span>
            Group 2 link
            <span
              class="pf-v5-c-app-launcher__menu-item-external-icon"
            >
              <i class="fas fa-external-link-alt" aria-hidden="true"></i>
            </span>
            <span class="pf-v5-screen-reader">(opens new window)</span>
          </a>
        </li>
      </ul>
    </section>
  </div>
</nav>

```

### Favorites

```html
<nav
  class="pf-v5-c-app-launcher pf-m-expanded"
  aria-label="Application launcher"
  id="app-launcher-favorites"
>
  <button
    class="pf-v5-c-app-launcher__toggle"
    type="button"
    id="app-launcher-favorites-button"
    aria-expanded="true"
    aria-label="Application launcher"
  >
    <i class="fas fa-th" aria-hidden="true"></i>
  </button>
  <div class="pf-v5-c-app-launcher__menu">
    <div class="pf-v5-c-app-launcher__menu-search">
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
    <section class="pf-v5-c-app-launcher__group">
      <h1 class="pf-v5-c-app-launcher__group-title">Favorites</h1>
      <ul role="menu">
        <li
          class="pf-v5-c-app-launcher__menu-wrapper pf-m-external pf-m-favorite"
          role="none"
        >
          <a
            class="pf-v5-c-app-launcher__menu-item pf-m-link"
            role="menuitem"
            href="#"
            target="_blank"
          >
            <span class="pf-v5-c-app-launcher__menu-item-icon">
              <img src="/assets/images/pf-logo-small.svg" alt="PatternFly logo" />
            </span>
            Link 2
            <span
              class="pf-v5-c-app-launcher__menu-item-external-icon"
            >
              <i class="fas fa-external-link-alt" aria-hidden="true"></i>
            </span>
            <span class="pf-v5-screen-reader">(opens new window)</span>
          </a>
          <button
            class="pf-v5-c-app-launcher__menu-item pf-m-action"
            type="button"
            aria-label="Favorite"
          >
            <i class="fas fa-star" aria-hidden="true"></i>
          </button>
        </li>
        <li
          class="pf-v5-c-app-launcher__menu-wrapper pf-m-external pf-m-favorite"
          role="none"
        >
          <a
            class="pf-v5-c-app-launcher__menu-item pf-m-link"
            role="menuitem"
            href="#"
            target="_blank"
          >
            <span class="pf-v5-c-app-launcher__menu-item-icon">
              <img src="/assets/images/pf-logo-small.svg" alt="PatternFly logo" />
            </span>
            Link 3
            <span
              class="pf-v5-c-app-launcher__menu-item-external-icon"
            >
              <i class="fas fa-external-link-alt" aria-hidden="true"></i>
            </span>
            <span class="pf-v5-screen-reader">(opens new window)</span>
          </a>
          <button
            class="pf-v5-c-app-launcher__menu-item pf-m-action"
            type="button"
            aria-label="Favorite"
          >
            <i class="fas fa-star" aria-hidden="true"></i>
          </button>
        </li>
      </ul>
    </section>
    <hr class="pf-v5-c-divider" />
    <section class="pf-v5-c-app-launcher__group">
      <h1 class="pf-v5-c-app-launcher__group-title">Group 1</h1>
      <ul role="menu">
        <li
          class="pf-v5-c-app-launcher__menu-wrapper pf-m-external"
          role="none"
        >
          <a
            class="pf-v5-c-app-launcher__menu-item pf-m-link"
            role="menuitem"
            href="#"
            target="_blank"
          >
            <span class="pf-v5-c-app-launcher__menu-item-icon">
              <img src="/assets/images/pf-logo-small.svg" alt="PatternFly logo" />
            </span>
            Link 1
            <span
              class="pf-v5-c-app-launcher__menu-item-external-icon"
            >
              <i class="fas fa-external-link-alt" aria-hidden="true"></i>
            </span>
            <span class="pf-v5-screen-reader">(opens new window)</span>
          </a>
          <button
            class="pf-v5-c-app-launcher__menu-item pf-m-action"
            type="button"
            aria-label="Favorite"
          >
            <i class="fas fa-star" aria-hidden="true"></i>
          </button>
        </li>
        <li
          class="pf-v5-c-app-launcher__menu-wrapper pf-m-external pf-m-favorite"
          role="none"
        >
          <a
            class="pf-v5-c-app-launcher__menu-item pf-m-link"
            role="menuitem"
            href="#"
            target="_blank"
          >
            <span class="pf-v5-c-app-launcher__menu-item-icon">
              <img src="/assets/images/pf-logo-small.svg" alt="PatternFly logo" />
            </span>
            Link 2
            <span
              class="pf-v5-c-app-launcher__menu-item-external-icon"
            >
              <i class="fas fa-external-link-alt" aria-hidden="true"></i>
            </span>
            <span class="pf-v5-screen-reader">(opens new window)</span>
          </a>
          <button
            class="pf-v5-c-app-launcher__menu-item pf-m-action"
            type="button"
            aria-label="Favorite"
          >
            <i class="fas fa-star" aria-hidden="true"></i>
          </button>
        </li>
      </ul>
    </section>
    <hr class="pf-v5-c-divider" />
    <section class="pf-v5-c-app-launcher__group">
      <h1 class="pf-v5-c-app-launcher__group-title">Group 2</h1>
      <ul role="menu">
        <li
          class="pf-v5-c-app-launcher__menu-wrapper pf-m-external pf-m-favorite"
          role="none"
        >
          <a
            class="pf-v5-c-app-launcher__menu-item pf-m-link"
            role="menuitem"
            href="#"
            target="_blank"
          >
            <span class="pf-v5-c-app-launcher__menu-item-icon">
              <img src="/assets/images/pf-logo-small.svg" alt="PatternFly logo" />
            </span>
            Link 3
            <span
              class="pf-v5-c-app-launcher__menu-item-external-icon"
            >
              <i class="fas fa-external-link-alt" aria-hidden="true"></i>
            </span>
            <span class="pf-v5-screen-reader">(opens new window)</span>
          </a>
          <button
            class="pf-v5-c-app-launcher__menu-item pf-m-action"
            type="button"
            aria-label="Favorite"
          >
            <i class="fas fa-star" aria-hidden="true"></i>
          </button>
        </li>
        <li
          class="pf-v5-c-app-launcher__menu-wrapper pf-m-external"
          role="none"
        >
          <a
            class="pf-v5-c-app-launcher__menu-item pf-m-link"
            role="menuitem"
            href="#"
            target="_blank"
          >
            <span class="pf-v5-c-app-launcher__menu-item-icon">
              <img src="/assets/images/pf-logo-small.svg" alt="PatternFly logo" />
            </span>
            Link 4
            <span
              class="pf-v5-c-app-launcher__menu-item-external-icon"
            >
              <i class="fas fa-external-link-alt" aria-hidden="true"></i>
            </span>
            <span class="pf-v5-screen-reader">(opens new window)</span>
          </a>
          <button
            class="pf-v5-c-app-launcher__menu-item pf-m-action"
            type="button"
            aria-label="Favorite"
          >
            <i class="fas fa-star" aria-hidden="true"></i>
          </button>
        </li>
      </ul>
    </section>
  </div>
</nav>

```

## Documentation

### Accessibility

| Attribute | Applied | Outcome |
| -- | -- | -- |
| `aria-label="Application launcher"` | `.pf-v5-c-app-launcher` |  Gives the app launcher element an accessible name. **Required** |
| `aria-expanded="false"` | `.pf-v5-c-button` |  Indicates that the menu is hidden. |
| `aria-expanded="true"` | `.pf-v5-c-button` |  Indicates that the menu is visible. |
| `aria-label="Actions"` | `.pf-v5-c-button` | Provides an accessible name for the app launcher when an icon is used. **Required** |
| `hidden` | `.pf-v5-c-app-launcher__menu` | Indicates that the menu is hidden so that it isn't visible in the UI and isn't accessed by assistive technologies. |
| `disabled` | `.pf-v5-c-app-launcher__toggle` | Disables the app launcher toggle and removes it from keyboard focus. |
| `disabled` | `button.pf-v5-c-app-launcher__menu-item` | When the menu item uses a button element, indicates that it is unavailable and removes it from keyboard focus. |
| `aria-disabled="true"` | `a.pf-v5-c-app-launcher__menu-item` | When the menu item uses a link element, indicates that it is unavailable. |
| `tabindex="-1"` | `a.pf-v5-c-app-launcher__menu-item` | When the menu item uses a link element, removes it from keyboard focus. |
| `aria-hidden="true"` | `.pf-v5-c-app-launcher__menu-item-external-icon > *` | Hides the icon from assistive technologies. |

### Usage

| Class | Applied | Outcome |
| -- | -- | -- |
| `.pf-v5-c-app-launcher` | `<nav>` | Defines the parent wrapper of the app launcher. |
| `.pf-v5-c-app-launcher__toggle` | `<button>` | Defines the app launcher toggle. |
| `.pf-v5-c-app-launcher__menu` | `<ul>`, `<div>` | Defines the parent wrapper of the menu items. Use a `<div>` if your app launcher has groups. |
| `.pf-v5-c-app-launcher__menu-search` | `<div>` | Defines the wrapper for the search input. |
| `.pf-v5-c-app-launcher__group` | `<section>` | Defines a group of items. Required when there is more than one group. |
| `.pf-v5-c-app-launcher__group-title` | `<h1>` | Defines a title for a group of items. |
| `.pf-v5-c-app-launcher__menu-wrapper` | `<li>` | Defines a menu wrapper for use with multiple actionable items in a single item row. |
| `.pf-v5-c-app-launcher__menu-item` | `<a>`, `<button>` | Defines a menu item. |
| `.pf-v5-c-app-launcher__menu-item-icon` | `<span>` | Defines the wrapper for the menu item icon. |
| `.pf-v5-c-app-launcher__menu-item-external-icon` | `<span>` | Defines the wrapper for the external link icon that appears on hover/focus. Use with `.pf-m-external`. |
| `.pf-m-expanded` | `.pf-v5-c-app-launcher` | Modifies for the expanded state. |
| `.pf-m-top` | `.pf-v5-c-app-launcher` | Modifies to display the menu above the toggle. |
| `.pf-m-align-right` | `.pf-v5-c-app-launcher__menu` | Modifies to display the menu aligned to the right edge of the toggle. |
| `.pf-m-static` | `.pf-v5-c-app-launcher__menu` | Modifies to position the menu statically to support custom positioning. |
| `.pf-m-disabled` | `a.pf-v5-c-app-launcher__menu-item` | Modifies to display the menu item as disabled. |
| `.pf-m-external` | `.pf-v5-c-app-launcher__menu-item` | Modifies to display the menu item as having an external link icon on hover/focus. |
| `.pf-m-favorite` | `.pf-v5-c-app-launcher__menu-wrapper` | Modifies wrapper to indicate that the item row has been favorited. |
| `.pf-m-link` | `.pf-v5-c-app-launcher__menu-item.pf-m-wrapper > .pf-v5-c-app-launcher__menu-item` | Modifies item for link styles. |
| `.pf-m-action` | `.pf-v5-c-app-launcher__menu-item.pf-m-wrapper > .pf-v5-c-app-launcher__menu-item` | Modifies item to for action styles. |
| `.pf-m-active` | `.pf-v5-c-app-launcher__toggle` | Forces display of the active state of the toggle. |
