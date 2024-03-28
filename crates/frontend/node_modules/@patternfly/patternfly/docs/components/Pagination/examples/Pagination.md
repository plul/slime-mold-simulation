---
id: Pagination
section: components
cssPrefix: pf-v5-c-pagination
---import './Pagination.css'

## Examples

### Top

```html
<div class="pf-v5-c-pagination">
  <div class="pf-v5-c-pagination__total-items">
    <b>1 - 10</b>&nbsp;of&nbsp;
    <b>36</b>
  </div>
  <div class="pf-v5-c-options-menu">
    <button
      class="pf-v5-c-options-menu__toggle pf-m-text pf-m-plain"
      type="button"
      id="pagination-options-menu-top-example-toggle"
      aria-haspopup="listbox"
      aria-expanded="false"
    >
      <span class="pf-v5-c-options-menu__toggle-text">
        <b>1 - 10</b>&nbsp;of&nbsp;
        <b>36</b>
      </span>
      <div class="pf-v5-c-options-menu__toggle-icon">
        <i class="fas fa-caret-down" aria-hidden="true"></i>
      </div>
    </button>
    <ul
      class="pf-v5-c-options-menu__menu"
      role="menu"
      aria-labelledby="pagination-options-menu-top-example-toggle"
      hidden
    >
      <li role="none">
        <button
          class="pf-v5-c-options-menu__menu-item"
          type="button"
          role="menuitem"
        >5 per page</button>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-options-menu__menu-item"
          type="button"
          role="menuitem"
        >
          10 per page
          <div class="pf-v5-c-options-menu__menu-item-icon">
            <i class="fas fa-check" aria-hidden="true"></i>
          </div>
        </button>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-options-menu__menu-item"
          type="button"
          role="menuitem"
        >20 per page</button>
      </li>
    </ul>
  </div>
  <nav
    class="pf-v5-c-pagination__nav"
    aria-label="Pagination nav - top example"
  >
    <div class="pf-v5-c-pagination__nav-control pf-m-first">
      <button
        class="pf-v5-c-button pf-m-plain"
        type="button"
        disabled
        aria-label="Go to first page"
      >
        <i class="fas fa-angle-double-left" aria-hidden="true"></i>
      </button>
    </div>
    <div class="pf-v5-c-pagination__nav-control pf-m-prev">
      <button
        class="pf-v5-c-button pf-m-plain"
        type="button"
        disabled
        aria-label="Go to previous page"
      >
        <i class="fas fa-angle-left" aria-hidden="true"></i>
      </button>
    </div>
    <div class="pf-v5-c-pagination__nav-page-select">
      <span class="pf-v5-c-form-control">
        <input
          aria-label="Current page"
          type="number"
          min="1"
          max="4"
          value="1"
        />
      </span>
      <span aria-hidden="true">of 4</span>
    </div>
    <div class="pf-v5-c-pagination__nav-control pf-m-next">
      <button
        class="pf-v5-c-button pf-m-plain"
        type="button"
        aria-label="Go to next page"
      >
        <i class="fas fa-angle-right" aria-hidden="true"></i>
      </button>
    </div>
    <div class="pf-v5-c-pagination__nav-control pf-m-last">
      <button
        class="pf-v5-c-button pf-m-plain"
        type="button"
        aria-label="Go to last page"
      >
        <i class="fas fa-angle-double-right" aria-hidden="true"></i>
      </button>
    </div>
  </nav>
</div>

```

### Top expanded

```html
<div class="pf-v5-c-pagination">
  <div class="pf-v5-c-pagination__total-items">
    <b>1 - 10</b>&nbsp;of&nbsp;
    <b>36</b>
  </div>
  <div class="pf-v5-c-options-menu pf-m-expanded">
    <button
      class="pf-v5-c-options-menu__toggle pf-m-text pf-m-plain"
      type="button"
      id="pagination-options-menu-top-expanded-example-toggle"
      aria-haspopup="listbox"
      aria-expanded="true"
    >
      <span class="pf-v5-c-options-menu__toggle-text">
        <b>1 - 10</b>&nbsp;of&nbsp;
        <b>36</b>
      </span>
      <div class="pf-v5-c-options-menu__toggle-icon">
        <i class="fas fa-caret-down" aria-hidden="true"></i>
      </div>
    </button>
    <ul
      class="pf-v5-c-options-menu__menu"
      role="menu"
      aria-labelledby="pagination-options-menu-top-expanded-example-toggle"
    >
      <li role="none">
        <button
          class="pf-v5-c-options-menu__menu-item"
          type="button"
          role="menuitem"
        >5 per page</button>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-options-menu__menu-item"
          type="button"
          role="menuitem"
        >
          10 per page
          <div class="pf-v5-c-options-menu__menu-item-icon">
            <i class="fas fa-check" aria-hidden="true"></i>
          </div>
        </button>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-options-menu__menu-item"
          type="button"
          role="menuitem"
        >20 per page</button>
      </li>
    </ul>
  </div>
  <nav
    class="pf-v5-c-pagination__nav"
    aria-label="Pagination nav - top expanded example"
  >
    <div class="pf-v5-c-pagination__nav-control pf-m-first">
      <button
        class="pf-v5-c-button pf-m-plain"
        type="button"
        disabled
        aria-label="Go to first page"
      >
        <i class="fas fa-angle-double-left" aria-hidden="true"></i>
      </button>
    </div>
    <div class="pf-v5-c-pagination__nav-control pf-m-prev">
      <button
        class="pf-v5-c-button pf-m-plain"
        type="button"
        disabled
        aria-label="Go to previous page"
      >
        <i class="fas fa-angle-left" aria-hidden="true"></i>
      </button>
    </div>
    <div class="pf-v5-c-pagination__nav-page-select">
      <span class="pf-v5-c-form-control">
        <input
          aria-label="Current page"
          type="number"
          min="1"
          max="4"
          value="1"
        />
      </span>
      <span aria-hidden="true">of 4</span>
    </div>
    <div class="pf-v5-c-pagination__nav-control pf-m-next">
      <button
        class="pf-v5-c-button pf-m-plain"
        type="button"
        aria-label="Go to next page"
      >
        <i class="fas fa-angle-right" aria-hidden="true"></i>
      </button>
    </div>
    <div class="pf-v5-c-pagination__nav-control pf-m-last">
      <button
        class="pf-v5-c-button pf-m-plain"
        type="button"
        aria-label="Go to last page"
      >
        <i class="fas fa-angle-double-right" aria-hidden="true"></i>
      </button>
    </div>
  </nav>
</div>

```

### Top sticky

```html
<div class="pf-v5-c-pagination pf-m-sticky">
  <div class="pf-v5-c-pagination__total-items">
    <b>1 - 10</b>&nbsp;of&nbsp;
    <b>36</b>
  </div>
  <div class="pf-v5-c-options-menu">
    <button
      class="pf-v5-c-options-menu__toggle pf-m-text pf-m-plain"
      type="button"
      id="pagination-options-menu-top-sticky-example-toggle"
      aria-haspopup="listbox"
      aria-expanded="false"
    >
      <span class="pf-v5-c-options-menu__toggle-text">
        <b>1 - 10</b>&nbsp;of&nbsp;
        <b>36</b>
      </span>
      <div class="pf-v5-c-options-menu__toggle-icon">
        <i class="fas fa-caret-down" aria-hidden="true"></i>
      </div>
    </button>
    <ul
      class="pf-v5-c-options-menu__menu"
      role="menu"
      aria-labelledby="pagination-options-menu-top-sticky-example-toggle"
      hidden
    >
      <li role="none">
        <button
          class="pf-v5-c-options-menu__menu-item"
          type="button"
          role="menuitem"
        >5 per page</button>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-options-menu__menu-item"
          type="button"
          role="menuitem"
        >
          10 per page
          <div class="pf-v5-c-options-menu__menu-item-icon">
            <i class="fas fa-check" aria-hidden="true"></i>
          </div>
        </button>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-options-menu__menu-item"
          type="button"
          role="menuitem"
        >20 per page</button>
      </li>
    </ul>
  </div>
  <nav
    class="pf-v5-c-pagination__nav"
    aria-label="Pagination nav - top sticky example"
  >
    <div class="pf-v5-c-pagination__nav-control pf-m-first">
      <button
        class="pf-v5-c-button pf-m-plain"
        type="button"
        disabled
        aria-label="Go to first page"
      >
        <i class="fas fa-angle-double-left" aria-hidden="true"></i>
      </button>
    </div>
    <div class="pf-v5-c-pagination__nav-control pf-m-prev">
      <button
        class="pf-v5-c-button pf-m-plain"
        type="button"
        disabled
        aria-label="Go to previous page"
      >
        <i class="fas fa-angle-left" aria-hidden="true"></i>
      </button>
    </div>
    <div class="pf-v5-c-pagination__nav-page-select">
      <span class="pf-v5-c-form-control">
        <input
          aria-label="Current page"
          type="number"
          min="1"
          max="4"
          value="1"
        />
      </span>
      <span aria-hidden="true">of 4</span>
    </div>
    <div class="pf-v5-c-pagination__nav-control pf-m-next">
      <button
        class="pf-v5-c-button pf-m-plain"
        type="button"
        aria-label="Go to next page"
      >
        <i class="fas fa-angle-right" aria-hidden="true"></i>
      </button>
    </div>
    <div class="pf-v5-c-pagination__nav-control pf-m-last">
      <button
        class="pf-v5-c-button pf-m-plain"
        type="button"
        aria-label="Go to last page"
      >
        <i class="fas fa-angle-double-right" aria-hidden="true"></i>
      </button>
    </div>
  </nav>
</div>
<div>Lorem ipsum dolor sit amet, consectetur adipiscing elit. Phasellus pretium est a porttitor vehicula. Quisque vel commodo urna. Morbi mattis rutrum ante, id vehicula ex accumsan ut. Morbi viverra, eros vel porttitor facilisis, eros purus aliquet erat, nec lobortis felis elit pulvinar sem. Vivamus vulputate, risus eget commodo eleifend, eros nibh porta quam, vitae lacinia leo libero at magna. Maecenas aliquam sagittis orci, et posuere nisi ultrices sit amet. Aliquam ex odio, malesuada sed posuere quis, pellentesque at mauris. Phasellus venenatis massa ex, eget pulvinar libero auctor pretium. Aliquam erat volutpat. Duis euismod justo in quam ullamcorper, in commodo massa vulputate.</div>
<br />
<br />
<div>Lorem ipsum dolor sit amet, consectetur adipiscing elit. Phasellus pretium est a porttitor vehicula. Quisque vel commodo urna. Morbi mattis rutrum ante, id vehicula ex accumsan ut. Morbi viverra, eros vel porttitor facilisis, eros purus aliquet erat, nec lobortis felis elit pulvinar sem. Vivamus vulputate, risus eget commodo eleifend, eros nibh porta quam, vitae lacinia leo libero at magna. Maecenas aliquam sagittis orci, et posuere nisi ultrices sit amet. Aliquam ex odio, malesuada sed posuere quis, pellentesque at mauris. Phasellus venenatis massa ex, eget pulvinar libero auctor pretium. Aliquam erat volutpat. Duis euismod justo in quam ullamcorper, in commodo massa vulputate.</div>
<br />
<br />
<div>Lorem ipsum dolor sit amet, consectetur adipiscing elit. Phasellus pretium est a porttitor vehicula. Quisque vel commodo urna. Morbi mattis rutrum ante, id vehicula ex accumsan ut. Morbi viverra, eros vel porttitor facilisis, eros purus aliquet erat, nec lobortis felis elit pulvinar sem. Vivamus vulputate, risus eget commodo eleifend, eros nibh porta quam, vitae lacinia leo libero at magna. Maecenas aliquam sagittis orci, et posuere nisi ultrices sit amet. Aliquam ex odio, malesuada sed posuere quis, pellentesque at mauris. Phasellus venenatis massa ex, eget pulvinar libero auctor pretium. Aliquam erat volutpat. Duis euismod justo in quam ullamcorper, in commodo massa vulputate.</div>
<br />
<br />
<div>Lorem ipsum dolor sit amet, consectetur adipiscing elit. Phasellus pretium est a porttitor vehicula. Quisque vel commodo urna. Morbi mattis rutrum ante, id vehicula ex accumsan ut. Morbi viverra, eros vel porttitor facilisis, eros purus aliquet erat, nec lobortis felis elit pulvinar sem. Vivamus vulputate, risus eget commodo eleifend, eros nibh porta quam, vitae lacinia leo libero at magna. Maecenas aliquam sagittis orci, et posuere nisi ultrices sit amet. Aliquam ex odio, malesuada sed posuere quis, pellentesque at mauris. Phasellus venenatis massa ex, eget pulvinar libero auctor pretium. Aliquam erat volutpat. Duis euismod justo in quam ullamcorper, in commodo massa vulputate.</div>

```

### Indeterminate (item count is not known)

```html
<div class="pf-v5-c-pagination">
  <div class="pf-v5-c-pagination__total-items">
    <b>1 - 10</b>&nbsp;of&nbsp;
    <b>many</b>
  </div>
  <div class="pf-v5-c-options-menu">
    <button
      class="pf-v5-c-options-menu__toggle pf-m-text pf-m-plain"
      type="button"
      id="pagination-options-menu-top-indeterminate-example-toggle"
      aria-haspopup="listbox"
      aria-expanded="false"
    >
      <span class="pf-v5-c-options-menu__toggle-text">
        <b>1 - 10</b>&nbsp;of&nbsp;
        <b>many</b>
      </span>
      <div class="pf-v5-c-options-menu__toggle-icon">
        <i class="fas fa-caret-down" aria-hidden="true"></i>
      </div>
    </button>
    <ul
      class="pf-v5-c-options-menu__menu"
      role="menu"
      aria-labelledby="pagination-options-menu-top-indeterminate-example-toggle"
      hidden
    >
      <li role="none">
        <button
          class="pf-v5-c-options-menu__menu-item"
          type="button"
          role="menuitem"
        >5 per page</button>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-options-menu__menu-item"
          type="button"
          role="menuitem"
        >
          10 per page
          <div class="pf-v5-c-options-menu__menu-item-icon">
            <i class="fas fa-check" aria-hidden="true"></i>
          </div>
        </button>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-options-menu__menu-item"
          type="button"
          role="menuitem"
        >20 per page</button>
      </li>
    </ul>
  </div>
  <nav
    class="pf-v5-c-pagination__nav"
    aria-label="Pagination nav - indeterminate item count example"
  >
    <div class="pf-v5-c-pagination__nav-control pf-m-first">
      <button
        class="pf-v5-c-button pf-m-plain"
        type="button"
        disabled
        aria-label="Go to first page"
      >
        <i class="fas fa-angle-double-left" aria-hidden="true"></i>
      </button>
    </div>
    <div class="pf-v5-c-pagination__nav-control pf-m-prev">
      <button
        class="pf-v5-c-button pf-m-plain"
        type="button"
        disabled
        aria-label="Go to previous page"
      >
        <i class="fas fa-angle-left" aria-hidden="true"></i>
      </button>
    </div>
    <div class="pf-v5-c-pagination__nav-page-select">
      <span class="pf-v5-c-form-control">
        <input
          aria-label="Current page"
          type="number"
          min="1"
          max="4"
          value="1"
        />
      </span>
    </div>
    <div class="pf-v5-c-pagination__nav-control pf-m-next">
      <button
        class="pf-v5-c-button pf-m-plain"
        type="button"
        aria-label="Go to next page"
      >
        <i class="fas fa-angle-right" aria-hidden="true"></i>
      </button>
    </div>
    <div class="pf-v5-c-pagination__nav-control pf-m-last">
      <button
        class="pf-v5-c-button pf-m-plain"
        type="button"
        aria-label="Go to last page"
      >
        <i class="fas fa-angle-double-right" aria-hidden="true"></i>
      </button>
    </div>
  </nav>
</div>

```

### Bottom

```html
<div class="pf-v5-c-pagination pf-m-bottom">
  <div class="pf-v5-c-options-menu pf-m-top">
    <button
      class="pf-v5-c-options-menu__toggle pf-m-text pf-m-plain"
      type="button"
      id="pagination-options-menu-bottom-example-toggle"
      aria-haspopup="listbox"
      aria-expanded="false"
    >
      <span class="pf-v5-c-options-menu__toggle-text">
        <b>1 - 10</b>&nbsp;of&nbsp;
        <b>36</b>
      </span>
      <div class="pf-v5-c-options-menu__toggle-icon">
        <i class="fas fa-caret-down" aria-hidden="true"></i>
      </div>
    </button>
    <ul
      class="pf-v5-c-options-menu__menu pf-m-top"
      role="menu"
      aria-labelledby="pagination-options-menu-bottom-example-toggle"
      hidden
    >
      <li role="none">
        <button
          class="pf-v5-c-options-menu__menu-item"
          type="button"
          role="menuitem"
        >5 per page</button>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-options-menu__menu-item"
          type="button"
          role="menuitem"
        >
          10 per page
          <div class="pf-v5-c-options-menu__menu-item-icon">
            <i class="fas fa-check" aria-hidden="true"></i>
          </div>
        </button>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-options-menu__menu-item"
          type="button"
          role="menuitem"
        >20 per page</button>
      </li>
    </ul>
  </div>
  <nav
    class="pf-v5-c-pagination__nav"
    aria-label="Pagination nav - bottom example"
  >
    <div class="pf-v5-c-pagination__nav-control pf-m-first">
      <button
        class="pf-v5-c-button pf-m-plain"
        type="button"
        disabled
        aria-label="Go to first page"
      >
        <i class="fas fa-angle-double-left" aria-hidden="true"></i>
      </button>
    </div>
    <div class="pf-v5-c-pagination__nav-control pf-m-prev">
      <button
        class="pf-v5-c-button pf-m-plain"
        type="button"
        disabled
        aria-label="Go to previous page"
      >
        <i class="fas fa-angle-left" aria-hidden="true"></i>
      </button>
    </div>
    <div class="pf-v5-c-pagination__nav-page-select">
      <span class="pf-v5-c-form-control">
        <input
          aria-label="Current page"
          type="number"
          min="1"
          max="4"
          value="1"
        />
      </span>
      <span aria-hidden="true">of 4</span>
    </div>
    <div class="pf-v5-c-pagination__nav-control pf-m-next">
      <button
        class="pf-v5-c-button pf-m-plain"
        type="button"
        aria-label="Go to next page"
      >
        <i class="fas fa-angle-right" aria-hidden="true"></i>
      </button>
    </div>
    <div class="pf-v5-c-pagination__nav-control pf-m-last">
      <button
        class="pf-v5-c-button pf-m-plain"
        type="button"
        aria-label="Go to last page"
      >
        <i class="fas fa-angle-double-right" aria-hidden="true"></i>
      </button>
    </div>
  </nav>
</div>

```

### Bottom sticky

```html
<div>Lorem ipsum dolor sit amet, consectetur adipiscing elit. Phasellus pretium est a porttitor vehicula. Quisque vel commodo urna. Morbi mattis rutrum ante, id vehicula ex accumsan ut. Morbi viverra, eros vel porttitor facilisis, eros purus aliquet erat, nec lobortis felis elit pulvinar sem. Vivamus vulputate, risus eget commodo eleifend, eros nibh porta quam, vitae lacinia leo libero at magna. Maecenas aliquam sagittis orci, et posuere nisi ultrices sit amet. Aliquam ex odio, malesuada sed posuere quis, pellentesque at mauris. Phasellus venenatis massa ex, eget pulvinar libero auctor pretium. Aliquam erat volutpat. Duis euismod justo in quam ullamcorper, in commodo massa vulputate.</div>
<br />
<br />
<div>Lorem ipsum dolor sit amet, consectetur adipiscing elit. Phasellus pretium est a porttitor vehicula. Quisque vel commodo urna. Morbi mattis rutrum ante, id vehicula ex accumsan ut. Morbi viverra, eros vel porttitor facilisis, eros purus aliquet erat, nec lobortis felis elit pulvinar sem. Vivamus vulputate, risus eget commodo eleifend, eros nibh porta quam, vitae lacinia leo libero at magna. Maecenas aliquam sagittis orci, et posuere nisi ultrices sit amet. Aliquam ex odio, malesuada sed posuere quis, pellentesque at mauris. Phasellus venenatis massa ex, eget pulvinar libero auctor pretium. Aliquam erat volutpat. Duis euismod justo in quam ullamcorper, in commodo massa vulputate.</div>
<br />
<br />
<div>Lorem ipsum dolor sit amet, consectetur adipiscing elit. Phasellus pretium est a porttitor vehicula. Quisque vel commodo urna. Morbi mattis rutrum ante, id vehicula ex accumsan ut. Morbi viverra, eros vel porttitor facilisis, eros purus aliquet erat, nec lobortis felis elit pulvinar sem. Vivamus vulputate, risus eget commodo eleifend, eros nibh porta quam, vitae lacinia leo libero at magna. Maecenas aliquam sagittis orci, et posuere nisi ultrices sit amet. Aliquam ex odio, malesuada sed posuere quis, pellentesque at mauris. Phasellus venenatis massa ex, eget pulvinar libero auctor pretium. Aliquam erat volutpat. Duis euismod justo in quam ullamcorper, in commodo massa vulputate.</div>
<br />
<br />
<div>Lorem ipsum dolor sit amet, consectetur adipiscing elit. Phasellus pretium est a porttitor vehicula. Quisque vel commodo urna. Morbi mattis rutrum ante, id vehicula ex accumsan ut. Morbi viverra, eros vel porttitor facilisis, eros purus aliquet erat, nec lobortis felis elit pulvinar sem. Vivamus vulputate, risus eget commodo eleifend, eros nibh porta quam, vitae lacinia leo libero at magna. Maecenas aliquam sagittis orci, et posuere nisi ultrices sit amet. Aliquam ex odio, malesuada sed posuere quis, pellentesque at mauris. Phasellus venenatis massa ex, eget pulvinar libero auctor pretium. Aliquam erat volutpat. Duis euismod justo in quam ullamcorper, in commodo massa vulputate.</div>
<div class="pf-v5-c-pagination pf-m-bottom pf-m-sticky">
  <div class="pf-v5-c-options-menu pf-m-top">
    <button
      class="pf-v5-c-options-menu__toggle pf-m-text pf-m-plain"
      type="button"
      id="pagination-options-menu-bottom-sticky-example-toggle"
      aria-haspopup="listbox"
      aria-expanded="false"
    >
      <span class="pf-v5-c-options-menu__toggle-text">
        <b>1 - 10</b>&nbsp;of&nbsp;
        <b>36</b>
      </span>
      <div class="pf-v5-c-options-menu__toggle-icon">
        <i class="fas fa-caret-down" aria-hidden="true"></i>
      </div>
    </button>
    <ul
      class="pf-v5-c-options-menu__menu pf-m-top"
      role="menu"
      aria-labelledby="pagination-options-menu-bottom-sticky-example-toggle"
      hidden
    >
      <li role="none">
        <button
          class="pf-v5-c-options-menu__menu-item"
          type="button"
          role="menuitem"
        >5 per page</button>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-options-menu__menu-item"
          type="button"
          role="menuitem"
        >
          10 per page
          <div class="pf-v5-c-options-menu__menu-item-icon">
            <i class="fas fa-check" aria-hidden="true"></i>
          </div>
        </button>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-options-menu__menu-item"
          type="button"
          role="menuitem"
        >20 per page</button>
      </li>
    </ul>
  </div>
  <nav
    class="pf-v5-c-pagination__nav"
    aria-label="Pagination nav - bottom sticky example"
  >
    <div class="pf-v5-c-pagination__nav-control pf-m-first">
      <button
        class="pf-v5-c-button pf-m-plain"
        type="button"
        disabled
        aria-label="Go to first page"
      >
        <i class="fas fa-angle-double-left" aria-hidden="true"></i>
      </button>
    </div>
    <div class="pf-v5-c-pagination__nav-control pf-m-prev">
      <button
        class="pf-v5-c-button pf-m-plain"
        type="button"
        disabled
        aria-label="Go to previous page"
      >
        <i class="fas fa-angle-left" aria-hidden="true"></i>
      </button>
    </div>
    <div class="pf-v5-c-pagination__nav-page-select">
      <span class="pf-v5-c-form-control">
        <input
          aria-label="Current page"
          type="number"
          min="1"
          max="4"
          value="1"
        />
      </span>
      <span aria-hidden="true">of 4</span>
    </div>
    <div class="pf-v5-c-pagination__nav-control pf-m-next">
      <button
        class="pf-v5-c-button pf-m-plain"
        type="button"
        aria-label="Go to next page"
      >
        <i class="fas fa-angle-right" aria-hidden="true"></i>
      </button>
    </div>
    <div class="pf-v5-c-pagination__nav-control pf-m-last">
      <button
        class="pf-v5-c-button pf-m-plain"
        type="button"
        aria-label="Go to last page"
      >
        <i class="fas fa-angle-double-right" aria-hidden="true"></i>
      </button>
    </div>
  </nav>
</div>

```

### Top disabled

```html
<div class="pf-v5-c-pagination">
  <div class="pf-v5-c-pagination__total-items">
    <b>1 - 10</b>&nbsp;of&nbsp;
    <b>36</b>
  </div>
  <div class="pf-v5-c-options-menu">
    <button
      class="pf-v5-c-options-menu__toggle pf-m-text pf-m-plain"
      type="button"
      id="pagination-options-menu-top-disabled-example-toggle"
      aria-haspopup="listbox"
      aria-expanded="false"
      disabled
    >
      <span class="pf-v5-c-options-menu__toggle-text">
        <b>1 - 10</b>&nbsp;of&nbsp;
        <b>36</b>
      </span>
      <div class="pf-v5-c-options-menu__toggle-icon">
        <i class="fas fa-caret-down" aria-hidden="true"></i>
      </div>
    </button>
    <ul
      class="pf-v5-c-options-menu__menu"
      role="menu"
      aria-labelledby="pagination-options-menu-top-disabled-example-toggle"
      hidden
    >
      <li role="none">
        <button
          class="pf-v5-c-options-menu__menu-item"
          type="button"
          role="menuitem"
        >5 per page</button>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-options-menu__menu-item"
          type="button"
          role="menuitem"
        >
          10 per page
          <div class="pf-v5-c-options-menu__menu-item-icon">
            <i class="fas fa-check" aria-hidden="true"></i>
          </div>
        </button>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-options-menu__menu-item"
          type="button"
          role="menuitem"
        >20 per page</button>
      </li>
    </ul>
  </div>
  <nav
    class="pf-v5-c-pagination__nav"
    aria-label="Pagination nav - top disabled example"
  >
    <div class="pf-v5-c-pagination__nav-control pf-m-first">
      <button
        class="pf-v5-c-button pf-m-plain"
        type="button"
        disabled
        aria-label="Go to first page"
      >
        <i class="fas fa-angle-double-left" aria-hidden="true"></i>
      </button>
    </div>
    <div class="pf-v5-c-pagination__nav-control pf-m-prev">
      <button
        class="pf-v5-c-button pf-m-plain"
        type="button"
        disabled
        aria-label="Go to previous page"
      >
        <i class="fas fa-angle-left" aria-hidden="true"></i>
      </button>
    </div>
    <div class="pf-v5-c-pagination__nav-page-select">
      <span class="pf-v5-c-form-control pf-m-disabled">
        <input
          disabled
          aria-label="Current page"
          type="number"
          min="1"
          max="4"
          value="1"
        />
      </span>
      <span aria-hidden="true">of 4</span>
    </div>
    <div class="pf-v5-c-pagination__nav-control pf-m-next">
      <button
        class="pf-v5-c-button pf-m-plain"
        type="button"
        disabled
        aria-label="Go to next page"
      >
        <i class="fas fa-angle-right" aria-hidden="true"></i>
      </button>
    </div>
    <div class="pf-v5-c-pagination__nav-control pf-m-last">
      <button
        class="pf-v5-c-button pf-m-plain"
        type="button"
        disabled
        aria-label="Go to last page"
      >
        <i class="fas fa-angle-double-right" aria-hidden="true"></i>
      </button>
    </div>
  </nav>
</div>

```

### Compact

```html
<div class="pf-v5-c-pagination pf-m-compact">
  <div class="pf-v5-c-pagination__total-items">
    <b>1 - 10</b>&nbsp;of&nbsp;
    <b>36</b>
  </div>
  <div class="pf-v5-c-options-menu">
    <button
      class="pf-v5-c-options-menu__toggle pf-m-text pf-m-plain"
      type="button"
      id="pagination-options-menu-compact-example-toggle"
      aria-haspopup="listbox"
      aria-expanded="false"
    >
      <span class="pf-v5-c-options-menu__toggle-text">
        <b>1 - 10</b>&nbsp;of&nbsp;
        <b>36</b>
      </span>
      <div class="pf-v5-c-options-menu__toggle-icon">
        <i class="fas fa-caret-down" aria-hidden="true"></i>
      </div>
    </button>
    <ul
      class="pf-v5-c-options-menu__menu"
      role="menu"
      aria-labelledby="pagination-options-menu-compact-example-toggle"
      hidden
    >
      <li role="none">
        <button
          class="pf-v5-c-options-menu__menu-item"
          type="button"
          role="menuitem"
        >5 per page</button>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-options-menu__menu-item"
          type="button"
          role="menuitem"
        >
          10 per page
          <div class="pf-v5-c-options-menu__menu-item-icon">
            <i class="fas fa-check" aria-hidden="true"></i>
          </div>
        </button>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-options-menu__menu-item"
          type="button"
          role="menuitem"
        >20 per page</button>
      </li>
    </ul>
  </div>
  <nav
    class="pf-v5-c-pagination__nav"
    aria-label="Pagination nav - compact example"
  >
    <div class="pf-v5-c-pagination__nav-control pf-m-prev">
      <button
        class="pf-v5-c-button pf-m-plain"
        type="button"
        disabled
        aria-label="Go to previous page"
      >
        <i class="fas fa-angle-left" aria-hidden="true"></i>
      </button>
    </div>
    <div class="pf-v5-c-pagination__nav-control pf-m-next">
      <button
        class="pf-v5-c-button pf-m-plain"
        type="button"
        aria-label="Go to next page"
      >
        <i class="fas fa-angle-right" aria-hidden="true"></i>
      </button>
    </div>
  </nav>
</div>

```

### Top with display summary modifier

```html
<div class="pf-v5-c-pagination pf-m-display-summary">
  <div class="pf-v5-c-pagination__total-items">
    <b>1 - 10</b>&nbsp;of&nbsp;
    <b>36</b>
  </div>
  <div class="pf-v5-c-options-menu">
    <button
      class="pf-v5-c-options-menu__toggle pf-m-text pf-m-plain"
      type="button"
      id="pagination-top-with-summary-modifier-options-menu-toggle"
      aria-haspopup="listbox"
      aria-expanded="false"
    >
      <span class="pf-v5-c-options-menu__toggle-text">
        <b>1 - 10</b>&nbsp;of&nbsp;
        <b>36</b>
      </span>
      <div class="pf-v5-c-options-menu__toggle-icon">
        <i class="fas fa-caret-down" aria-hidden="true"></i>
      </div>
    </button>
    <ul
      class="pf-v5-c-options-menu__menu"
      role="menu"
      aria-labelledby="pagination-top-with-summary-modifier-options-menu-toggle"
      hidden
    >
      <li role="none">
        <button
          class="pf-v5-c-options-menu__menu-item"
          type="button"
          role="menuitem"
        >5 per page</button>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-options-menu__menu-item"
          type="button"
          role="menuitem"
        >
          10 per page
          <div class="pf-v5-c-options-menu__menu-item-icon">
            <i class="fas fa-check" aria-hidden="true"></i>
          </div>
        </button>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-options-menu__menu-item"
          type="button"
          role="menuitem"
        >20 per page</button>
      </li>
    </ul>
  </div>
  <nav
    class="pf-v5-c-pagination__nav"
    aria-label="Pagination nav - top with display summary modifier example"
  >
    <div class="pf-v5-c-pagination__nav-control pf-m-first">
      <button
        class="pf-v5-c-button pf-m-plain"
        type="button"
        disabled
        aria-label="Go to first page"
      >
        <i class="fas fa-angle-double-left" aria-hidden="true"></i>
      </button>
    </div>
    <div class="pf-v5-c-pagination__nav-control pf-m-prev">
      <button
        class="pf-v5-c-button pf-m-plain"
        type="button"
        disabled
        aria-label="Go to previous page"
      >
        <i class="fas fa-angle-left" aria-hidden="true"></i>
      </button>
    </div>
    <div class="pf-v5-c-pagination__nav-page-select">
      <span class="pf-v5-c-form-control">
        <input
          aria-label="Current page"
          type="number"
          min="1"
          max="4"
          value="1"
        />
      </span>
      <span aria-hidden="true">of 4</span>
    </div>
    <div class="pf-v5-c-pagination__nav-control pf-m-next">
      <button
        class="pf-v5-c-button pf-m-plain"
        type="button"
        aria-label="Go to next page"
      >
        <i class="fas fa-angle-right" aria-hidden="true"></i>
      </button>
    </div>
    <div class="pf-v5-c-pagination__nav-control pf-m-last">
      <button
        class="pf-v5-c-button pf-m-plain"
        type="button"
        aria-label="Go to last page"
      >
        <i class="fas fa-angle-double-right" aria-hidden="true"></i>
      </button>
    </div>
  </nav>
</div>

```

### Top with display full modifier

```html
<div class="pf-v5-c-pagination pf-m-display-full">
  <div class="pf-v5-c-pagination__total-items">
    <b>1 - 10</b>&nbsp;of&nbsp;
    <b>36</b>
  </div>
  <div class="pf-v5-c-options-menu">
    <button
      class="pf-v5-c-options-menu__toggle pf-m-text pf-m-plain"
      type="button"
      id="pagination-top-with-full-modifier-options-menu-toggle"
      aria-haspopup="listbox"
      aria-expanded="false"
    >
      <span class="pf-v5-c-options-menu__toggle-text">
        <b>1 - 10</b>&nbsp;of&nbsp;
        <b>36</b>
      </span>
      <div class="pf-v5-c-options-menu__toggle-icon">
        <i class="fas fa-caret-down" aria-hidden="true"></i>
      </div>
    </button>
    <ul
      class="pf-v5-c-options-menu__menu"
      role="menu"
      aria-labelledby="pagination-top-with-full-modifier-options-menu-toggle"
      hidden
    >
      <li role="none">
        <button
          class="pf-v5-c-options-menu__menu-item"
          type="button"
          role="menuitem"
        >5 per page</button>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-options-menu__menu-item"
          type="button"
          role="menuitem"
        >
          10 per page
          <div class="pf-v5-c-options-menu__menu-item-icon">
            <i class="fas fa-check" aria-hidden="true"></i>
          </div>
        </button>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-options-menu__menu-item"
          type="button"
          role="menuitem"
        >20 per page</button>
      </li>
    </ul>
  </div>
  <nav
    class="pf-v5-c-pagination__nav"
    aria-label="Pagination nav - top with display full modifier example"
  >
    <div class="pf-v5-c-pagination__nav-control pf-m-first">
      <button
        class="pf-v5-c-button pf-m-plain"
        type="button"
        disabled
        aria-label="Go to first page"
      >
        <i class="fas fa-angle-double-left" aria-hidden="true"></i>
      </button>
    </div>
    <div class="pf-v5-c-pagination__nav-control pf-m-prev">
      <button
        class="pf-v5-c-button pf-m-plain"
        type="button"
        disabled
        aria-label="Go to previous page"
      >
        <i class="fas fa-angle-left" aria-hidden="true"></i>
      </button>
    </div>
    <div class="pf-v5-c-pagination__nav-page-select">
      <span class="pf-v5-c-form-control">
        <input
          aria-label="Current page"
          type="number"
          min="1"
          max="4"
          value="1"
        />
      </span>
      <span aria-hidden="true">of 4</span>
    </div>
    <div class="pf-v5-c-pagination__nav-control pf-m-next">
      <button
        class="pf-v5-c-button pf-m-plain"
        type="button"
        aria-label="Go to next page"
      >
        <i class="fas fa-angle-right" aria-hidden="true"></i>
      </button>
    </div>
    <div class="pf-v5-c-pagination__nav-control pf-m-last">
      <button
        class="pf-v5-c-button pf-m-plain"
        type="button"
        aria-label="Go to last page"
      >
        <i class="fas fa-angle-double-right" aria-hidden="true"></i>
      </button>
    </div>
  </nav>
</div>

```

### Top with responsive display summary and display full modifiers

```html
<div
  class="pf-v5-c-pagination pf-m-display-summary pf-m-display-full-on-lg pf-m-display-summary-on-xl pf-m-display-full-on-2xl"
>
  <div class="pf-v5-c-pagination__total-items">
    <b>1 - 10</b>&nbsp;of&nbsp;
    <b>36</b>
  </div>
  <div class="pf-v5-c-options-menu">
    <button
      class="pf-v5-c-options-menu__toggle pf-m-text pf-m-plain"
      type="button"
      id="pagination-top-with-responsive-summary-navigation-modifiers-options-menu-toggle"
      aria-haspopup="listbox"
      aria-expanded="false"
    >
      <span class="pf-v5-c-options-menu__toggle-text">
        <b>1 - 10</b>&nbsp;of&nbsp;
        <b>36</b>
      </span>
      <div class="pf-v5-c-options-menu__toggle-icon">
        <i class="fas fa-caret-down" aria-hidden="true"></i>
      </div>
    </button>
    <ul
      class="pf-v5-c-options-menu__menu"
      role="menu"
      aria-labelledby="pagination-top-with-responsive-summary-navigation-modifiers-options-menu-toggle"
      hidden
    >
      <li role="none">
        <button
          class="pf-v5-c-options-menu__menu-item"
          type="button"
          role="menuitem"
        >5 per page</button>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-options-menu__menu-item"
          type="button"
          role="menuitem"
        >
          10 per page
          <div class="pf-v5-c-options-menu__menu-item-icon">
            <i class="fas fa-check" aria-hidden="true"></i>
          </div>
        </button>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-options-menu__menu-item"
          type="button"
          role="menuitem"
        >20 per page</button>
      </li>
    </ul>
  </div>
  <nav
    class="pf-v5-c-pagination__nav"
    aria-label="Pagination nav - top with responsive display summary and display full modifiers example"
  >
    <div class="pf-v5-c-pagination__nav-control pf-m-first">
      <button
        class="pf-v5-c-button pf-m-plain"
        type="button"
        disabled
        aria-label="Go to first page"
      >
        <i class="fas fa-angle-double-left" aria-hidden="true"></i>
      </button>
    </div>
    <div class="pf-v5-c-pagination__nav-control pf-m-prev">
      <button
        class="pf-v5-c-button pf-m-plain"
        type="button"
        disabled
        aria-label="Go to previous page"
      >
        <i class="fas fa-angle-left" aria-hidden="true"></i>
      </button>
    </div>
    <div class="pf-v5-c-pagination__nav-page-select">
      <span class="pf-v5-c-form-control">
        <input
          aria-label="Current page"
          type="number"
          min="1"
          max="4"
          value="1"
        />
      </span>
      <span aria-hidden="true">of 4</span>
    </div>
    <div class="pf-v5-c-pagination__nav-control pf-m-next">
      <button
        class="pf-v5-c-button pf-m-plain"
        type="button"
        aria-label="Go to next page"
      >
        <i class="fas fa-angle-right" aria-hidden="true"></i>
      </button>
    </div>
    <div class="pf-v5-c-pagination__nav-control pf-m-last">
      <button
        class="pf-v5-c-button pf-m-plain"
        type="button"
        aria-label="Go to last page"
      >
        <i class="fas fa-angle-double-right" aria-hidden="true"></i>
      </button>
    </div>
  </nav>
</div>

```

### Compact display full modifier

```html
<div class="pf-v5-c-pagination pf-m-compact pf-m-display-full">
  <div class="pf-v5-c-pagination__total-items">
    <b>1 - 10</b>&nbsp;of&nbsp;
    <b>36</b>
  </div>
  <div class="pf-v5-c-options-menu">
    <button
      class="pf-v5-c-options-menu__toggle pf-m-text pf-m-plain"
      type="button"
      id="pagination-compact-with-full-modifier-options-menu-toggle"
      aria-haspopup="listbox"
      aria-expanded="false"
    >
      <span class="pf-v5-c-options-menu__toggle-text">
        <b>1 - 10</b>&nbsp;of&nbsp;
        <b>36</b>
      </span>
      <div class="pf-v5-c-options-menu__toggle-icon">
        <i class="fas fa-caret-down" aria-hidden="true"></i>
      </div>
    </button>
    <ul
      class="pf-v5-c-options-menu__menu"
      role="menu"
      aria-labelledby="pagination-compact-with-full-modifier-options-menu-toggle"
      hidden
    >
      <li role="none">
        <button
          class="pf-v5-c-options-menu__menu-item"
          type="button"
          role="menuitem"
        >5 per page</button>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-options-menu__menu-item"
          type="button"
          role="menuitem"
        >
          10 per page
          <div class="pf-v5-c-options-menu__menu-item-icon">
            <i class="fas fa-check" aria-hidden="true"></i>
          </div>
        </button>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-options-menu__menu-item"
          type="button"
          role="menuitem"
        >20 per page</button>
      </li>
    </ul>
  </div>
  <nav
    class="pf-v5-c-pagination__nav"
    aria-label="Pagination nav - compact display full modifier example"
  >
    <div class="pf-v5-c-pagination__nav-control pf-m-prev">
      <button
        class="pf-v5-c-button pf-m-plain"
        type="button"
        disabled
        aria-label="Go to previous page"
      >
        <i class="fas fa-angle-left" aria-hidden="true"></i>
      </button>
    </div>
    <div class="pf-v5-c-pagination__nav-control pf-m-next">
      <button
        class="pf-v5-c-button pf-m-plain"
        type="button"
        aria-label="Go to next page"
      >
        <i class="fas fa-angle-right" aria-hidden="true"></i>
      </button>
    </div>
  </nav>
</div>

```

### Inset

```html
<div
  class="pf-v5-c-pagination pf-m-inset-none pf-m-inset-md-on-md pf-m-inset-2xl-on-lg"
>
  <div class="pf-v5-c-pagination__total-items">
    <b>1 - 10</b>&nbsp;of&nbsp;
    <b>36</b>
  </div>
  <div class="pf-v5-c-options-menu">
    <button
      class="pf-v5-c-options-menu__toggle pf-m-text pf-m-plain"
      type="button"
      id="pagination-inset-options-menu-toggle"
      aria-haspopup="listbox"
      aria-expanded="false"
    >
      <span class="pf-v5-c-options-menu__toggle-text">
        <b>1 - 10</b>&nbsp;of&nbsp;
        <b>36</b>
      </span>
      <div class="pf-v5-c-options-menu__toggle-icon">
        <i class="fas fa-caret-down" aria-hidden="true"></i>
      </div>
    </button>
    <ul
      class="pf-v5-c-options-menu__menu"
      role="menu"
      aria-labelledby="pagination-inset-options-menu-toggle"
      hidden
    >
      <li role="none">
        <button
          class="pf-v5-c-options-menu__menu-item"
          type="button"
          role="menuitem"
        >5 per page</button>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-options-menu__menu-item"
          type="button"
          role="menuitem"
        >
          10 per page
          <div class="pf-v5-c-options-menu__menu-item-icon">
            <i class="fas fa-check" aria-hidden="true"></i>
          </div>
        </button>
      </li>
      <li role="none">
        <button
          class="pf-v5-c-options-menu__menu-item"
          type="button"
          role="menuitem"
        >20 per page</button>
      </li>
    </ul>
  </div>
  <nav
    class="pf-v5-c-pagination__nav"
    aria-label="Pagination nav - inset example"
  >
    <div class="pf-v5-c-pagination__nav-control pf-m-first">
      <button
        class="pf-v5-c-button pf-m-plain"
        type="button"
        disabled
        aria-label="Go to first page"
      >
        <i class="fas fa-angle-double-left" aria-hidden="true"></i>
      </button>
    </div>
    <div class="pf-v5-c-pagination__nav-control pf-m-prev">
      <button
        class="pf-v5-c-button pf-m-plain"
        type="button"
        disabled
        aria-label="Go to previous page"
      >
        <i class="fas fa-angle-left" aria-hidden="true"></i>
      </button>
    </div>
    <div class="pf-v5-c-pagination__nav-page-select">
      <span class="pf-v5-c-form-control">
        <input
          aria-label="Current page"
          type="number"
          min="1"
          max="4"
          value="1"
        />
      </span>
      <span aria-hidden="true">of 4</span>
    </div>
    <div class="pf-v5-c-pagination__nav-control pf-m-next">
      <button
        class="pf-v5-c-button pf-m-plain"
        type="button"
        aria-label="Go to next page"
      >
        <i class="fas fa-angle-right" aria-hidden="true"></i>
      </button>
    </div>
    <div class="pf-v5-c-pagination__nav-control pf-m-last">
      <button
        class="pf-v5-c-button pf-m-plain"
        type="button"
        aria-label="Go to last page"
      >
        <i class="fas fa-angle-double-right" aria-hidden="true"></i>
      </button>
    </div>
  </nav>
</div>

```

## Documentation

Note: `<button>` or `<a>` elements can be used in `.pf-v5-c-pagination__nav-page-select`.

### Accessibility

| Attribute | Applied to | Outcome |
| -- | -- | -- |
| `aria-label`  | `.pf-v5-c-pagination__nav` |  Provides an accessible name for pagination navigation element. **Required** |
| `type="number"` | `.pf-v5-c-pagination__nav-page-select` > `.pf-v5-c-form-control` | Defines a field as a number. **Required** |
| `value` | `.pf-v5-c-pagination__nav-page-select` > `.pf-v5-c-form-control` | Provides initial integer value. **Required** |
| `min` | `.pf-v5-c-pagination__nav-page-select` > `.pf-v5-c-form-control` | Provides minimum integer value. **Required** |
| `max` | `.pf-v5-c-pagination__nav-page-select` > `.pf-v5-c-form-control` | Provides max integer value. **Required** |

### Usage

| Class | Applied to | Outcome |
| -- | -- | -- |
| `.pf-v5-c-pagination` | `<div>` |  Initiates pagination. |
| `.pf-v5-c-pagination__current` | `<div>` |  Initiates element to display currently displayed items for use in responsive view. Only needed for default pagination, not `.pf-m-bottom`. |
| `.pf-v5-c-pagination__total-items` | `<div>` | Initiates element to replace the options menu on summary. |
| `.pf-v5-c-pagination__nav` | `<nav>` |  Initiates pagination nav. |
| `.pf-v5-c-pagination__nav-control` | `<div>` |  Initiates pagination nav control. |
| `.pf-v5-c-pagination__nav-page-select` | `<div>` |  Initiates pagination nav page select. |
| `.pf-m-display-summary{-on-[breakpoint]}` | `.pf-v5-c-pagination` | Modifies for summary display pagination component styles at optional [breakpoint](/developer-resources/global-css-variables#breakpoint-variables-and-class-suffixes). |
| `.pf-m-display-full{-on-[breakpoint]}` | `.pf-v5-c-pagination` | Modifies for full display pagination component styles at optional [breakpoint](/developer-resources/global-css-variables#breakpoint-variables-and-class-suffixes). |
| `.pf-m-bottom` | `.pf-v5-c-pagination` | Modifies for bottom pagination component styles. |
| `.pf-m-compact` | `.pf-v5-c-pagination` | Modifies for compact pagination component styles. |
| `.pf-m-static` | `.pf-v5-c-pagination.pf-m-bottom` | Modifies bottom pagination to not be positioned sticky on summary. |
| `.pf-m-sticky` | `.pf-v5-c-pagination` | Modifies the pagination to be sticky to its container. It will be sticky to the top of the container by default, and sticky to the bottom of the container when applied to `.pf-v5-c-pagination.pf-m-bottom`. |
| `.pf-m-inset-{none, sm, md, lg, xl, 2xl}{-on-[breakpoint]}` | `.pf-v5-c-pagination` | Modifies pagination horizontal padding at optional [breakpoint](/developer-resources/global-css-variables#breakpoint-variables-and-class-suffixes). |
| `.pf-m-page-insets` | `.pf-v5-c-pagination` |  Modifies the pagination component padding/inset to visually match padding of page elements. |
| `.pf-m-first` | `.pf-v5-c-pagination__nav-control` | Indicates the control is for the first page button. |
| `.pf-m-prev` | `.pf-v5-c-pagination__nav-control` | Indicates the control is for the previous page button. |
| `.pf-m-next` | `.pf-v5-c-pagination__nav-control` | Indicates the control is for the next page button. |
| `.pf-m-last` | `.pf-v5-c-pagination__nav-control` | Indicates the control is for the last page button. |
