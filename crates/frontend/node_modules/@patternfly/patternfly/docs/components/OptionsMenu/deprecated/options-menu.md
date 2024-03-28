---
id: Options menu
section: components
subsection: menus
cssPrefix: pf-v5-c-options-menu
deprecated: true
---import './options-menu.css'

## Examples

### Single option

```html
<div class="pf-v5-c-options-menu">
  <button
    class="pf-v5-c-options-menu__toggle"
    type="button"
    id="options-menu-single-example-toggle"
    aria-haspopup="listbox"
    aria-expanded="false"
  >
    <span class="pf-v5-c-options-menu__toggle-text">Options menu</span>
    <div class="pf-v5-c-options-menu__toggle-icon">
      <i class="fas fa-caret-down" aria-hidden="true"></i>
    </div>
  </button>
  <ul
    class="pf-v5-c-options-menu__menu"
    role="menu"
    aria-labelledby="options-menu-single-example-toggle"
    hidden
  >
    <li role="none">
      <button
        class="pf-v5-c-options-menu__menu-item"
        type="button"
        role="menuitem"
      >Option 1</button>
    </li>
    <li role="none">
      <button
        class="pf-v5-c-options-menu__menu-item"
        type="button"
        role="menuitem"
      >
        Option 2
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
      >Option 3</button>
    </li>
  </ul>
</div>

<div class="pf-v5-c-options-menu pf-m-expanded">
  <button
    class="pf-v5-c-options-menu__toggle"
    type="button"
    id="options-menu-single-expanded-example-toggle"
    aria-haspopup="listbox"
    aria-expanded="true"
  >
    <span class="pf-v5-c-options-menu__toggle-text">Options menu</span>
    <div class="pf-v5-c-options-menu__toggle-icon">
      <i class="fas fa-caret-down" aria-hidden="true"></i>
    </div>
  </button>
  <ul
    class="pf-v5-c-options-menu__menu"
    role="menu"
    aria-labelledby="options-menu-single-expanded-example-toggle"
  >
    <li role="none">
      <button
        class="pf-v5-c-options-menu__menu-item"
        type="button"
        role="menuitem"
      >Option 1</button>
    </li>
    <li role="none">
      <button
        class="pf-v5-c-options-menu__menu-item"
        type="button"
        role="menuitem"
      >
        Option 2
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
      >Option 3</button>
    </li>
  </ul>
</div>

```

### Disabled

```html
<div class="pf-v5-c-options-menu">
  <button
    class="pf-v5-c-options-menu__toggle"
    type="button"
    id="options-menu-single-disabled-example-toggle"
    aria-haspopup="listbox"
    aria-expanded="false"
    disabled
  >
    <span class="pf-v5-c-options-menu__toggle-text">Disabled options menu</span>
    <div class="pf-v5-c-options-menu__toggle-icon">
      <i class="fas fa-caret-down" aria-hidden="true"></i>
    </div>
  </button>
</div>

```

### Multiple options

```html
<div class="pf-v5-c-options-menu">
  <button
    class="pf-v5-c-options-menu__toggle"
    type="button"
    id="options-menu-multiple-example-toggle"
    aria-haspopup="listbox"
    aria-expanded="false"
  >
    <span class="pf-v5-c-options-menu__toggle-text">Sort by</span>
    <div class="pf-v5-c-options-menu__toggle-icon">
      <i class="fas fa-caret-down" aria-hidden="true"></i>
    </div>
  </button>
  <div
    class="pf-v5-c-options-menu__menu"
    role="menu"
    aria-labelledby="options-menu-multiple-example-toggle"
    hidden
  >
    <section class="pf-v5-c-options-menu__group" role="none">
      <ul role="group" aria-label="Sort by">
        <li role="none">
          <button
            class="pf-v5-c-options-menu__menu-item"
            type="button"
            role="menuitem"
          >Name</button>
        </li>
        <li role="none">
          <button
            class="pf-v5-c-options-menu__menu-item"
            type="button"
            role="menuitem"
          >
            Date
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
            disabled
          >Disabled</button>
        </li>
        <li role="none">
          <button
            class="pf-v5-c-options-menu__menu-item"
            type="button"
            role="menuitem"
          >Size</button>
        </li>
      </ul>
    </section>
    <li class="pf-v5-c-divider" role="separator"></li>
    <section class="pf-v5-c-options-menu__group" role="none">
      <ul role="group" aria-label="Sort direction">
        <li role="none">
          <button
            class="pf-v5-c-options-menu__menu-item"
            type="button"
            role="menuitem"
          >
            Ascending
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
          >Descending</button>
        </li>
      </ul>
    </section>
  </div>
</div>

<div class="pf-v5-c-options-menu pf-m-expanded">
  <button
    class="pf-v5-c-options-menu__toggle"
    type="button"
    id="options-menu-multiple-expanded-example-toggle"
    aria-haspopup="listbox"
    aria-expanded="true"
  >
    <span class="pf-v5-c-options-menu__toggle-text">Sort by</span>
    <div class="pf-v5-c-options-menu__toggle-icon">
      <i class="fas fa-caret-down" aria-hidden="true"></i>
    </div>
  </button>
  <div
    class="pf-v5-c-options-menu__menu"
    role="menu"
    aria-labelledby="options-menu-multiple-expanded-example-toggle"
  >
    <section class="pf-v5-c-options-menu__group" role="none">
      <ul role="group" aria-label="Sort by">
        <li role="none">
          <button
            class="pf-v5-c-options-menu__menu-item"
            type="button"
            role="menuitem"
          >Name</button>
        </li>
        <li role="none">
          <button
            class="pf-v5-c-options-menu__menu-item"
            type="button"
            role="menuitem"
          >
            Date
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
            disabled
          >Disabled</button>
        </li>
        <li role="none">
          <button
            class="pf-v5-c-options-menu__menu-item"
            type="button"
            role="menuitem"
          >Size</button>
        </li>
      </ul>
    </section>
    <li class="pf-v5-c-divider" role="separator"></li>
    <section class="pf-v5-c-options-menu__group" role="none">
      <ul role="group" aria-label="Sort direction">
        <li role="none">
          <button
            class="pf-v5-c-options-menu__menu-item"
            type="button"
            role="menuitem"
          >
            Ascending
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
          >Descending</button>
        </li>
      </ul>
    </section>
  </div>
</div>

```

### Plain

```html
<div class="pf-v5-c-options-menu">
  <button
    class="pf-v5-c-options-menu__toggle pf-m-plain"
    type="button"
    id="options-menu-plain-disabled-example-toggle"
    aria-haspopup="listbox"
    aria-expanded="false"
    disabled
    aria-label="Sort by"
  >
    <i
      class="fas fa-sort-amount-down pf-v5-m-mirror-inline-rtl"
      aria-hidden="true"
    ></i>
  </button>
  <ul
    class="pf-v5-c-options-menu__menu"
    role="menu"
    aria-labelledby="options-menu-plain-disabled-example-toggle"
    hidden
  >
    <li role="none">
      <button
        class="pf-v5-c-options-menu__menu-item"
        type="button"
        role="menuitem"
      >Option 1</button>
    </li>
    <li role="none">
      <button
        class="pf-v5-c-options-menu__menu-item"
        type="button"
        role="menuitem"
      >
        Option 2
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
      >Option 3</button>
    </li>
  </ul>
</div>

<div class="pf-v5-c-options-menu">
  <button
    class="pf-v5-c-options-menu__toggle pf-m-plain"
    type="button"
    id="options-menu-plain-example-toggle"
    aria-haspopup="listbox"
    aria-expanded="false"
    aria-label="Sort by"
  >
    <i
      class="fas fa-sort-amount-down pf-v5-m-mirror-inline-rtl"
      aria-hidden="true"
    ></i>
  </button>
  <ul
    class="pf-v5-c-options-menu__menu"
    role="menu"
    aria-labelledby="options-menu-plain-example-toggle"
    hidden
  >
    <li role="none">
      <button
        class="pf-v5-c-options-menu__menu-item"
        type="button"
        role="menuitem"
      >Option 1</button>
    </li>
    <li role="none">
      <button
        class="pf-v5-c-options-menu__menu-item"
        type="button"
        role="menuitem"
      >
        Option 2
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
      >Option 3</button>
    </li>
  </ul>
</div>

<div class="pf-v5-c-options-menu pf-m-expanded">
  <button
    class="pf-v5-c-options-menu__toggle pf-m-plain"
    type="button"
    id="options-menu-plain-expanded-example-toggle"
    aria-haspopup="listbox"
    aria-expanded="true"
    aria-label="Sort by"
  >
    <i
      class="fas fa-sort-amount-down pf-v5-m-mirror-inline-rtl"
      aria-hidden="true"
    ></i>
  </button>
  <ul
    class="pf-v5-c-options-menu__menu"
    role="menu"
    aria-labelledby="options-menu-plain-expanded-example-toggle"
  >
    <li role="none">
      <button
        class="pf-v5-c-options-menu__menu-item"
        type="button"
        role="menuitem"
      >Option 1</button>
    </li>
    <li role="none">
      <button
        class="pf-v5-c-options-menu__menu-item"
        type="button"
        role="menuitem"
      >
        Option 2
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
      >Option 3</button>
    </li>
  </ul>
</div>

```

### Align top

```html
<div class="pf-v5-c-options-menu pf-m-expanded pf-m-top">
  <button
    class="pf-v5-c-options-menu__toggle"
    type="button"
    id="options-menu-top-example-toggle"
    aria-haspopup="listbox"
    aria-expanded="true"
  >
    <span class="pf-v5-c-options-menu__toggle-text">Options menu</span>
    <div class="pf-v5-c-options-menu__toggle-icon">
      <i class="fas fa-caret-down" aria-hidden="true"></i>
    </div>
  </button>
  <ul
    class="pf-v5-c-options-menu__menu pf-m-top"
    role="menu"
    aria-labelledby="options-menu-top-example-toggle"
  >
    <li role="none">
      <button
        class="pf-v5-c-options-menu__menu-item"
        type="button"
        role="menuitem"
      >Option 1</button>
    </li>
    <li role="none">
      <button
        class="pf-v5-c-options-menu__menu-item"
        type="button"
        role="menuitem"
      >
        Option 2
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
      >Option 3</button>
    </li>
  </ul>
</div>

```

### Align right

```html
<div class="pf-v5-c-options-menu pf-m-expanded pf-m-align-right">
  <button
    class="pf-v5-c-options-menu__toggle"
    type="button"
    id="options-menu-align-right-example-toggle"
    aria-haspopup="listbox"
    aria-expanded="true"
  >
    <span class="pf-v5-c-options-menu__toggle-text">Options menu</span>
    <div class="pf-v5-c-options-menu__toggle-icon">
      <i class="fas fa-caret-down" aria-hidden="true"></i>
    </div>
  </button>
  <ul
    class="pf-v5-c-options-menu__menu pf-m-align-right"
    role="menu"
    aria-labelledby="options-menu-align-right-example-toggle"
  >
    <li role="none">
      <button
        class="pf-v5-c-options-menu__menu-item"
        type="button"
        role="menuitem"
      >Option 1</button>
    </li>
    <li role="none">
      <button
        class="pf-v5-c-options-menu__menu-item"
        type="button"
        role="menuitem"
      >
        Option 2
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
      >Option 3</button>
    </li>
  </ul>
</div>

```

### Plain with text

```html
<div class="pf-v5-c-options-menu">
  <button
    class="pf-v5-c-options-menu__toggle pf-m-text pf-m-plain"
    type="button"
    id="options-menu-disabled-text-example-toggle"
    aria-haspopup="listbox"
    aria-expanded="false"
    disabled
  >
    <span class="pf-v5-c-options-menu__toggle-text">Custom text</span>
    <div class="pf-v5-c-options-menu__toggle-icon">
      <i class="fas fa-caret-down" aria-hidden="true"></i>
    </div>
  </button>
  <ul
    class="pf-v5-c-options-menu__menu"
    role="menu"
    aria-labelledby="options-menu-disabled-text-example-toggle"
    hidden
  >
    <li role="none">
      <button
        class="pf-v5-c-options-menu__menu-item"
        type="button"
        role="menuitem"
      >Option 1</button>
    </li>
    <li role="none">
      <button
        class="pf-v5-c-options-menu__menu-item"
        type="button"
        role="menuitem"
      >
        Option 2
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
      >Option 3</button>
    </li>
  </ul>
</div>

<div class="pf-v5-c-options-menu">
  <button
    class="pf-v5-c-options-menu__toggle pf-m-text pf-m-plain"
    type="button"
    id="options-menu-plain-text-example-toggle"
    aria-haspopup="listbox"
    aria-expanded="false"
  >
    <span class="pf-v5-c-options-menu__toggle-text">Custom text</span>
    <div class="pf-v5-c-options-menu__toggle-icon">
      <i class="fas fa-caret-down" aria-hidden="true"></i>
    </div>
  </button>
  <ul
    class="pf-v5-c-options-menu__menu"
    role="menu"
    aria-labelledby="options-menu-plain-text-example-toggle"
    hidden
  >
    <li role="none">
      <button
        class="pf-v5-c-options-menu__menu-item"
        type="button"
        role="menuitem"
      >Option 1</button>
    </li>
    <li role="none">
      <button
        class="pf-v5-c-options-menu__menu-item"
        type="button"
        role="menuitem"
      >
        Option 2
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
      >Option 3</button>
    </li>
  </ul>
</div>

<div class="pf-v5-c-options-menu pf-m-expanded">
  <button
    class="pf-v5-c-options-menu__toggle pf-m-text pf-m-plain"
    type="button"
    id="options-menu-plain-text-expanded-example-toggle"
    aria-haspopup="listbox"
    aria-expanded="true"
  >
    <span class="pf-v5-c-options-menu__toggle-text">Custom text</span>
    <div class="pf-v5-c-options-menu__toggle-icon">
      <i class="fas fa-caret-down" aria-hidden="true"></i>
    </div>
  </button>
  <ul
    class="pf-v5-c-options-menu__menu"
    role="menu"
    aria-labelledby="options-menu-plain-text-expanded-example-toggle"
  >
    <li role="none">
      <button
        class="pf-v5-c-options-menu__menu-item"
        type="button"
        role="menuitem"
      >Option 1</button>
    </li>
    <li role="none">
      <button
        class="pf-v5-c-options-menu__menu-item"
        type="button"
        role="menuitem"
      >
        Option 2
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
      >Option 3</button>
    </li>
  </ul>
</div>

```

### With groups

```html
<div class="pf-v5-c-options-menu pf-m-expanded">
  <button
    class="pf-v5-c-options-menu__toggle"
    type="button"
    id="options-menu-groups-toggle"
    aria-haspopup="listbox"
    aria-expanded="true"
  >
    <span class="pf-v5-c-options-menu__toggle-text">Options menu</span>
    <div class="pf-v5-c-options-menu__toggle-icon">
      <i class="fas fa-caret-down" aria-hidden="true"></i>
    </div>
  </button>
  <div
    class="pf-v5-c-options-menu__menu"
    role="menu"
    aria-labelledby="options-menu-groups-toggle"
  >
    <section class="pf-v5-c-options-menu__group" role="none">
      <ul role="group">
        <li role="none">
          <button
            class="pf-v5-c-options-menu__menu-item"
            type="button"
            role="menuitem"
          >Option 1</button>
        </li>
        <li role="none">
          <button
            class="pf-v5-c-options-menu__menu-item"
            type="button"
            role="menuitem"
          >
            Option 2
            <div class="pf-v5-c-options-menu__menu-item-icon">
              <i class="fas fa-check" aria-hidden="true"></i>
            </div>
          </button>
        </li>
      </ul>
    </section>
    <section class="pf-v5-c-options-menu__group" role="none">
      <h1 class="pf-v5-c-options-menu__group-title">Group 1</h1>

      <ul role="group">
        <li role="none">
          <button
            class="pf-v5-c-options-menu__menu-item"
            type="button"
            role="menuitem"
          >Option 1</button>
        </li>
        <li role="none">
          <button
            class="pf-v5-c-options-menu__menu-item"
            type="button"
            role="menuitem"
          >Option 2</button>
        </li>
      </ul>
    </section>
    <section class="pf-v5-c-options-menu__group" role="none">
      <h1 class="pf-v5-c-options-menu__group-title">Group 2</h1>

      <ul role="group">
        <li role="none">
          <button
            class="pf-v5-c-options-menu__menu-item"
            type="button"
            role="menuitem"
          >Option 1</button>
        </li>
        <li role="none">
          <button
            class="pf-v5-c-options-menu__menu-item"
            type="button"
            role="menuitem"
          >Option 2</button>
        </li>
      </ul>
    </section>
  </div>
</div>

```

### With groups and dividers between groups

```html
<div class="pf-v5-c-options-menu pf-m-expanded">
  <button
    class="pf-v5-c-options-menu__toggle"
    type="button"
    id="options-menu-groups-and-dividers-between-groups-toggle"
    aria-haspopup="listbox"
    aria-expanded="true"
  >
    <span class="pf-v5-c-options-menu__toggle-text">Options menu</span>
    <div class="pf-v5-c-options-menu__toggle-icon">
      <i class="fas fa-caret-down" aria-hidden="true"></i>
    </div>
  </button>
  <div
    class="pf-v5-c-options-menu__menu"
    role="menu"
    aria-labelledby="options-menu-groups-and-dividers-between-groups-toggle"
  >
    <section class="pf-v5-c-options-menu__group" role="none">
      <ul role="group">
        <li role="none">
          <button
            class="pf-v5-c-options-menu__menu-item"
            type="button"
            role="menuitem"
          >Option 1</button>
        </li>
        <li role="none">
          <button
            class="pf-v5-c-options-menu__menu-item"
            type="button"
            role="menuitem"
          >
            Option 2
            <div class="pf-v5-c-options-menu__menu-item-icon">
              <i class="fas fa-check" aria-hidden="true"></i>
            </div>
          </button>
        </li>
      </ul>
    </section>
    <hr class="pf-v5-c-divider" />
    <section class="pf-v5-c-options-menu__group" role="none">
      <h1 class="pf-v5-c-options-menu__group-title">Group 1</h1>

      <ul role="group">
        <li role="none">
          <button
            class="pf-v5-c-options-menu__menu-item"
            type="button"
            role="menuitem"
          >Option 1</button>
        </li>
        <li role="none">
          <button
            class="pf-v5-c-options-menu__menu-item"
            type="button"
            role="menuitem"
          >Option 2</button>
        </li>
      </ul>
    </section>
    <hr class="pf-v5-c-divider" />
    <section class="pf-v5-c-options-menu__group" role="none">
      <h1 class="pf-v5-c-options-menu__group-title">Group 2</h1>

      <ul role="group">
        <li role="none">
          <button
            class="pf-v5-c-options-menu__menu-item"
            type="button"
            role="menuitem"
          >Option 1</button>
        </li>
        <li role="none">
          <button
            class="pf-v5-c-options-menu__menu-item"
            type="button"
            role="menuitem"
          >Option 2</button>
        </li>
      </ul>
    </section>
  </div>
</div>

```

### With groups and dividers between items

```html
<div class="pf-v5-c-options-menu pf-m-expanded">
  <button
    class="pf-v5-c-options-menu__toggle"
    type="button"
    id="options-menu-groups-and-dividers-between-items-toggle"
    aria-haspopup="listbox"
    aria-expanded="true"
  >
    <span class="pf-v5-c-options-menu__toggle-text">Options menu</span>
    <div class="pf-v5-c-options-menu__toggle-icon">
      <i class="fas fa-caret-down" aria-hidden="true"></i>
    </div>
  </button>
  <div
    class="pf-v5-c-options-menu__menu"
    role="menu"
    aria-labelledby="options-menu-groups-and-dividers-between-items-toggle"
  >
    <section class="pf-v5-c-options-menu__group" role="none">
      <ul role="group">
        <li role="none">
          <button
            class="pf-v5-c-options-menu__menu-item"
            type="button"
            role="menuitem"
          >Option 1</button>
        </li>
        <li role="none">
          <button
            class="pf-v5-c-options-menu__menu-item"
            type="button"
            role="menuitem"
          >
            Option 2
            <div class="pf-v5-c-options-menu__menu-item-icon">
              <i class="fas fa-check" aria-hidden="true"></i>
            </div>
          </button>
        </li>
        <li class="pf-v5-c-divider" role="separator"></li>
      </ul>
    </section>
    <section class="pf-v5-c-options-menu__group" role="none">
      <h1 class="pf-v5-c-options-menu__group-title">Group 1</h1>

      <ul role="group">
        <li role="none">
          <button
            class="pf-v5-c-options-menu__menu-item"
            type="button"
            role="menuitem"
          >Option 1</button>
        </li>
        <li role="none">
          <button
            class="pf-v5-c-options-menu__menu-item"
            type="button"
            role="menuitem"
          >Option 2</button>
        </li>
        <li class="pf-v5-c-divider" role="separator"></li>
      </ul>
    </section>
    <section class="pf-v5-c-options-menu__group" role="none">
      <h1 class="pf-v5-c-options-menu__group-title">Group 2</h1>

      <ul role="group">
        <li role="none">
          <button
            class="pf-v5-c-options-menu__menu-item"
            type="button"
            role="menuitem"
          >Option 1</button>
        </li>
        <li role="none">
          <button
            class="pf-v5-c-options-menu__menu-item"
            type="button"
            role="menuitem"
          >Option 2</button>
        </li>
      </ul>
    </section>
  </div>
</div>

```

## Documentation

### Accessibility

*This section to be updated once the React implementation is complete.*

| Attribute | Applied to | Outcome |
| -- | -- | -- |
| `role` or `aria` | `pf-v5-c-options-menu` |  accessibility notes. |
| `disabled` | `.pf-v5-c-options-menu__toggle`, `.pf-v5-c-options-menu__toggle-button` | Disables the options menu toggle and toggle button and removes it from keyboard focus. |
*Note:* The attribute `aria-selected="true"` should be set programmatically to the selected item(s).

### Usage

| Class | Applied to | Outcome |
| -- | -- | -- |
| `.pf-v5-c-options-menu` | `<div>` |  Initiates a custom options menu. |
| `.pf-v5-c-options-menu__toggle` | `<button>` |  Initiates a custom toggle. |
| `.pf-v5-c-options-menu__toggle-text` | `<span>` | Initiates a wrapper for toggle text.
| `.pf-v5-c-options-menu__toggle-icon` | `<i>` | Initiates the up/down arrow beside toggle text. |
| `.pf-v5-c-options-menu__toggle-button` | `<button>` | Initiates a custom toggle button for use when `.pf-v5-c-options-menu__toggle` is a `<div>` or non-interactive element. |
| `.pf-v5-c-options-menu__menu` | `<ul>` |  Initiates the custom options-menu menu. |
| `.pf-v5-c-options-menu__menu-item` | `<li>` |  Initiates the items in the custom options-menu menu. |
| `.pf-v5-c-options-menu__menu-item-icon` | `<i>` |  Initiates the icon to indicate selected menu items. |
| `.pf-v5-c-options-menu__group` | `<section>` | Defines a group of items in an options menu. **Required when there is more than one group in an options menu**. |
| `.pf-v5-c-options-menu__group-title` | `<h1>` | Defines the title for a group of items in an options menu. |
| `.pf-m-top` | `.pf-v5-c-options-menu` | Modifies to display the menu above the toggle. |
| `.pf-m-align-right` | `.pf-v5-c-options-menu__menu` | Modifies to display the menu aligned to the right edge of the toggle |
| `.pf-m-expanded` | `.pf-v5-c-options-menu` |  Modifies for the expanded state. |
| `.pf-m-static` | `.pf-v5-c-options-menu__menu` | Modifies the menu to be statically positioned to support custom positioning. |
| `.pf-m-plain` | `.pf-v5-c-options-menu__toggle` |  Modifies to display the toggle with no border. **Note:** Can be combined with `.pf-m-text` to create a normal text toggle with no border. |
| `.pf-m-disabled` | `.pf-v5-c-options-menu__toggle` | Modifies to display the options menu toggle as disabled. This applies to `pf-v5-c-options-menu__toggle` and should not be used in lieu of the `disabled` attribute on `pf-v5-c-options-menu__toggle`. When this is used, `disabled` should also be added to any form elements in `.pf-v5-c-options-menu__toggle` |
| `.pf-m-text` | `.pf-v5-c-options-menu__toggle` |  For use when the `.pf-v5-c-options-menu__toggle` is a `<div>` or some non-interactive elment, and you're using a custom `.pf-v5-c-options-menu__toggle-button` to toggle the options menu. |
| `.pf-m-active` | `.pf-v5-c-options-menu__toggle` | Forces display of the active state of the toggle. |
| `.pf-m-selected` | `.pf-v5-c-options-menu__menu-item` | Modifies the menu item for the selected state. |
