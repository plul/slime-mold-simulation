---
id: Breadcrumb
section: components
cssPrefix: pf-v5-c-breadcrumb
---import './Breadcrumb.css'

## Examples

### Basic

```html
<nav class="pf-v5-c-breadcrumb" aria-label="breadcrumb">
  <ol class="pf-v5-c-breadcrumb__list" role="list">
    <li class="pf-v5-c-breadcrumb__item">
      <span class="pf-v5-c-breadcrumb__item-divider">
        <i class="fas fa-angle-right" aria-hidden="true"></i>
      </span>
      <a href="#" class="pf-v5-c-breadcrumb__link">Section home</a>
    </li>
    <li class="pf-v5-c-breadcrumb__item">
      <span class="pf-v5-c-breadcrumb__item-divider">
        <i class="fas fa-angle-right" aria-hidden="true"></i>
      </span>
      <a href="#" class="pf-v5-c-breadcrumb__link">Section title</a>
    </li>
    <li class="pf-v5-c-breadcrumb__item">
      <span class="pf-v5-c-breadcrumb__item-divider">
        <i class="fas fa-angle-right" aria-hidden="true"></i>
      </span>
      <a href="#" class="pf-v5-c-breadcrumb__link">Section title</a>
    </li>
    <li class="pf-v5-c-breadcrumb__item">
      <span class="pf-v5-c-breadcrumb__item-divider">
        <i class="fas fa-angle-right" aria-hidden="true"></i>
      </span>
      <a
        href="#"
        class="pf-v5-c-breadcrumb__link pf-m-current"
        aria-current="page"
      >Section landing</a>
    </li>
  </ol>
</nav>

```

### Without home link

```html
<nav class="pf-v5-c-breadcrumb" aria-label="breadcrumb">
  <ol class="pf-v5-c-breadcrumb__list" role="list">
    <li class="pf-v5-c-breadcrumb__item">
      <span class="pf-v5-c-breadcrumb__item-divider">
        <i class="fas fa-angle-right" aria-hidden="true"></i>
      </span>
      Section home
    </li>
    <li class="pf-v5-c-breadcrumb__item">
      <span class="pf-v5-c-breadcrumb__item-divider">
        <i class="fas fa-angle-right" aria-hidden="true"></i>
      </span>
      <a href="#" class="pf-v5-c-breadcrumb__link">Section title</a>
    </li>
    <li class="pf-v5-c-breadcrumb__item">
      <span class="pf-v5-c-breadcrumb__item-divider">
        <i class="fas fa-angle-right" aria-hidden="true"></i>
      </span>
      <a href="#" class="pf-v5-c-breadcrumb__link">Section title</a>
    </li>
    <li class="pf-v5-c-breadcrumb__item">
      <span class="pf-v5-c-breadcrumb__item-divider">
        <i class="fas fa-angle-right" aria-hidden="true"></i>
      </span>
      <a href="#" class="pf-v5-c-breadcrumb__link">Section title</a>
    </li>
    <li class="pf-v5-c-breadcrumb__item">
      <span class="pf-v5-c-breadcrumb__item-divider">
        <i class="fas fa-angle-right" aria-hidden="true"></i>
      </span>
      <a href="#" class="pf-v5-c-breadcrumb__link">Section title</a>
    </li>
    <li class="pf-v5-c-breadcrumb__item">
      <span class="pf-v5-c-breadcrumb__item-divider">
        <i class="fas fa-angle-right" aria-hidden="true"></i>
      </span>
      <a
        href="#"
        class="pf-v5-c-breadcrumb__link pf-m-current"
        aria-current="page"
      >Section landing</a>
    </li>
  </ol>
</nav>

```

### With heading

```html
<nav class="pf-v5-c-breadcrumb" aria-label="breadcrumb">
  <ol class="pf-v5-c-breadcrumb__list" role="list">
    <li class="pf-v5-c-breadcrumb__item">
      <span class="pf-v5-c-breadcrumb__item-divider">
        <i class="fas fa-angle-right" aria-hidden="true"></i>
      </span>
      <a href="#" class="pf-v5-c-breadcrumb__link">Section home</a>
    </li>
    <li class="pf-v5-c-breadcrumb__item">
      <span class="pf-v5-c-breadcrumb__item-divider">
        <i class="fas fa-angle-right" aria-hidden="true"></i>
      </span>
      <a href="#" class="pf-v5-c-breadcrumb__link">Section title</a>
    </li>
    <li class="pf-v5-c-breadcrumb__item">
      <span class="pf-v5-c-breadcrumb__item-divider">
        <i class="fas fa-angle-right" aria-hidden="true"></i>
      </span>
      <a href="#" class="pf-v5-c-breadcrumb__link">Section title</a>
    </li>
    <li class="pf-v5-c-breadcrumb__item">
      <span class="pf-v5-c-breadcrumb__item-divider">
        <i class="fas fa-angle-right" aria-hidden="true"></i>
      </span>
      <a href="#" class="pf-v5-c-breadcrumb__link">Section title</a>
    </li>
    <li class="pf-v5-c-breadcrumb__item">
      <span class="pf-v5-c-breadcrumb__item-divider">
        <i class="fas fa-angle-right" aria-hidden="true"></i>
      </span>
      <h1 class="pf-v5-c-breadcrumb__heading">
        <a
          href="#"
          class="pf-v5-c-breadcrumb__link pf-m-current"
          aria-current="page"
        >Section title</a>
      </h1>
    </li>
  </ol>
</nav>

```

### With dropdown

```html
<nav class="pf-v5-c-breadcrumb" aria-label="breadcrumb">
  <ol class="pf-v5-c-breadcrumb__list" role="list">
    <li class="pf-v5-c-breadcrumb__item">
      <span class="pf-v5-c-breadcrumb__item-divider">
        <i class="fas fa-angle-right" aria-hidden="true"></i>
      </span>
      <a href="#" class="pf-v5-c-breadcrumb__link">Section home</a>
    </li>
    <li class="pf-v5-c-breadcrumb__item">
      <span class="pf-v5-c-breadcrumb__item-divider">
        <i class="fas fa-angle-right" aria-hidden="true"></i>
      </span>
      <a href="#" class="pf-v5-c-breadcrumb__link">Section title</a>
    </li>
    <li class="pf-v5-c-breadcrumb__item">
      <span class="pf-v5-c-breadcrumb__item-divider">
        <i class="fas fa-angle-right" aria-hidden="true"></i>
      </span>
      <span class="pf-v5-c-breadcrumb__dropdown">
        <div class="pf-v5-c-dropdown pf-m-expanded">
          <button
            class="pf-v5-c-dropdown__toggle pf-m-plain"
            id="dropdown-badge-toggle-button"
            aria-expanded="true"
            type="button"
          >
            <span class="pf-v5-c-badge pf-m-read">
              5
              <span class="pf-v5-c-dropdown__toggle-icon">
                <i class="fas fa-caret-down" aria-hidden="true"></i>
              </span>
            </span>
          </button>
          <ul
            class="pf-v5-c-dropdown__menu"
            aria-labelledby="dropdown-badge-toggle-button"
            role="menu"
          >
            <li role="none">
              <button
                class="pf-v5-c-dropdown__menu-item"
                role="menuitem"
                type="button"
              >Section title</button>
            </li>
            <li role="none">
              <button
                class="pf-v5-c-dropdown__menu-item"
                role="menuitem"
                type="button"
              >Section title</button>
            </li>
            <li role="none">
              <button
                class="pf-v5-c-dropdown__menu-item"
                role="menuitem"
                type="button"
              >Section title</button>
            </li>
            <li role="none">
              <button
                class="pf-v5-c-dropdown__menu-item"
                role="menuitem"
                type="button"
              >Section title</button>
            </li>
            <li role="none">
              <button
                class="pf-v5-c-dropdown__menu-item"
                role="menuitem"
                type="button"
              >Section title</button>
            </li>
          </ul>
        </div>
      </span>
    </li>
    <li class="pf-v5-c-breadcrumb__item">
      <span class="pf-v5-c-breadcrumb__item-divider">
        <i class="fas fa-angle-right" aria-hidden="true"></i>
      </span>
      <h1 class="pf-v5-c-breadcrumb__heading">
        <a
          href="#"
          class="pf-v5-c-breadcrumb__link pf-m-current"
          aria-current="page"
        >Section title</a>
      </h1>
    </li>
  </ol>
</nav>

```

### With buttons

```html
<nav class="pf-v5-c-breadcrumb" aria-label="breadcrumb">
  <ol class="pf-v5-c-breadcrumb__list" role="list">
    <li class="pf-v5-c-breadcrumb__item">
      <span class="pf-v5-c-breadcrumb__item-divider">
        <i class="fas fa-angle-right" aria-hidden="true"></i>
      </span>
      <button class="pf-v5-c-breadcrumb__link" type="button">Section home</button>
    </li>
    <li class="pf-v5-c-breadcrumb__item">
      <span class="pf-v5-c-breadcrumb__item-divider">
        <i class="fas fa-angle-right" aria-hidden="true"></i>
      </span>
      <button class="pf-v5-c-breadcrumb__link" type="button">Section title</button>
    </li>
    <li class="pf-v5-c-breadcrumb__item">
      <span class="pf-v5-c-breadcrumb__item-divider">
        <i class="fas fa-angle-right" aria-hidden="true"></i>
      </span>
      <button class="pf-v5-c-breadcrumb__link" type="button">Section title</button>
    </li>
    <li class="pf-v5-c-breadcrumb__item">
      <span class="pf-v5-c-breadcrumb__item-divider">
        <i class="fas fa-angle-right" aria-hidden="true"></i>
      </span>
      <button
        class="pf-v5-c-breadcrumb__link pf-m-current"
        type="button"
        aria-current="page"
      >Section landing</button>
    </li>
  </ol>
</nav>

```

## Documentation

### Overview

A breadcrumb is a list of links to display a user's navigational hierarchy. The last item of the breadcrumb list indicates the current page's location.

*   `.pf-v5-c-breadcrumb__list` is the default breadcrumb navigation. It provides links to previous navigation pages and also shows the current page's location.

In the event that a page does not have a traditional `<h1>` page title, a heading can be included in the breadcrumbs and an optional link within.

### Accessibility

| Attribute | Applied to | Outcome |
| -- | -- | -- |
| `aria-label="[landmark description]"` | `.pf-v5-c-breadcrumb` |  Describes `<nav>` landmark. |
| `aria-label="[link name]"` | `.pf-v5-c-breadcrumb__link` | If link has no text (icon), add an aria-label. |
| `aria-current="page"` | `.pf-v5-c-breadcrumb__item`, `.pf-v5-c-breadcrumb__link` |  Indicates the current page within a set of pages. |

### Usage

| Class | Applied to | Outcome |
| -- | -- | -- |
| `.pf-v5-c-breadcrumb` | `<nav>` | Initiates a primary breadcrumb element. **Required** |
| `.pf-v5-c-breadcrumb__list` | `<ol>` | Initiates default breadcrumb ordered list. |
| `.pf-v5-c-breadcrumb__item` | `<li>` | Initiates default breadcrumb list item. |
| `.pf-v5-c-breadcrumb__item-divider` | `<span>` | Initiates default breadcrumb list item divider. |
| `.pf-v5-c-breadcrumb__link` | `<a>`, `<button>` | Initiates default breadcrumb list link. |
| `.pf-v5-c-breadcrumb__title` | `<h1>` | Initiates breadcrumb header. |
| `.pf-m-current` | `.pf-v5-c-breadcrumb__link` | Modifies to display the list item as the current item. |
