---
id: 'Dual list selector'
section: components
cssPrefix: pf-v5-c-dual-list-selector
---## Examples

### Basic

```html
<div class="pf-v5-c-dual-list-selector">
  <div class="pf-v5-c-dual-list-selector__pane pf-m-available">
    <div class="pf-v5-c-dual-list-selector__header">
      <div class="pf-v5-c-dual-list-selector__title">
        <div class="pf-v5-c-dual-list-selector__title-text">Available options</div>
      </div>
    </div>
    <div class="pf-v5-c-dual-list-selector__tools">
      <div class="pf-v5-c-dual-list-selector__tools-filter">
        <div class="pf-v5-c-text-input-group">
          <div class="pf-v5-c-text-input-group__main pf-m-icon">
            <span class="pf-v5-c-text-input-group__text">
              <span class="pf-v5-c-text-input-group__icon">
                <i class="fas fa-fw fa-search"></i>
              </span>
              <input
                class="pf-v5-c-text-input-group__text-input"
                type="text"
                value
                aria-label="Search input"
              />
            </span>
          </div>
        </div>
      </div>
      <div class="pf-v5-c-dual-list-selector__tools-actions">
        <button
          class="pf-v5-c-button pf-m-plain"
          type="button"
          aria-label="Sort"
        >
          <i
            class="fas fa-sort-amount-down pf-v5-m-mirror-inline-rtl"
            aria-hidden="true"
          ></i>
        </button>
        <div class="pf-v5-c-dropdown">
          <button
            class="pf-v5-c-dropdown__toggle pf-m-plain"
            id="dropdown-kebab-basic-available-button"
            aria-expanded="false"
            type="button"
            aria-label="Actions"
          >
            <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
          </button>
          <ul
            class="pf-v5-c-dropdown__menu"
            aria-labelledby="dropdown-kebab-basic-available-button"
            hidden
            role="menu"
          >
            <li role="none">
              <a
                class="pf-v5-c-dropdown__menu-item"
                role="menuitem"
                href="#"
              >Link</a>
            </li>
            <li role="none">
              <button
                class="pf-v5-c-dropdown__menu-item"
                role="menuitem"
                type="button"
              >Action</button>
            </li>
            <li role="none">
              <a
                class="pf-v5-c-dropdown__menu-item pf-m-disabled"
                role="menuitem"
                href="#"
                aria-disabled="true"
                tabindex="-1"
              >Disabled link</a>
            </li>
            <li role="none">
              <button
                class="pf-v5-c-dropdown__menu-item"
                role="menuitem"
                type="button"
                disabled
              >Disabled action</button>
            </li>
            <li class="pf-v5-c-divider" role="separator"></li>
            <li role="none">
              <a
                class="pf-v5-c-dropdown__menu-item"
                role="menuitem"
                href="#"
              >Separated link</a>
            </li>
          </ul>
        </div>
      </div>
    </div>
    <div class="pf-v5-c-dual-list-selector__status">
      <span
        class="pf-v5-c-dual-list-selector__status-text"
        id="basic-available-status-text"
      >0 of 5 items selected</span>
    </div>
    <div class="pf-v5-c-dual-list-selector__menu">
      <ul
        class="pf-v5-c-dual-list-selector__list"
        role="listbox"
        aria-labelledby="basic-available-status-text"
        aria-multiselectable="true"
        aria-activedescendant
      >
        <li class="pf-v5-c-dual-list-selector__list-item" role="option">
          <div class="pf-v5-c-dual-list-selector__list-item-row">
            <span class="pf-v5-c-dual-list-selector__item">
              <span class="pf-v5-c-dual-list-selector__item-main">
                <span class="pf-v5-c-dual-list-selector__item-text">Item1</span>
              </span>
              <span class="pf-v5-c-dual-list-selector__item-count">
                <span class="pf-v5-c-badge pf-m-read"></span>
              </span>
            </span>
          </div>
        </li>
        <li class="pf-v5-c-dual-list-selector__list-item" role="option">
          <div class="pf-v5-c-dual-list-selector__list-item-row">
            <span class="pf-v5-c-dual-list-selector__item">
              <span class="pf-v5-c-dual-list-selector__item-main">
                <span class="pf-v5-c-dual-list-selector__item-text">Item2</span>
              </span>
              <span class="pf-v5-c-dual-list-selector__item-count">
                <span class="pf-v5-c-badge pf-m-read"></span>
              </span>
            </span>
          </div>
        </li>
        <li
          class="pf-v5-c-dual-list-selector__list-item pf-m-disabled"
          role="option"
        >
          <div class="pf-v5-c-dual-list-selector__list-item-row">
            <span class="pf-v5-c-dual-list-selector__item">
              <span class="pf-v5-c-dual-list-selector__item-main">
                <span
                  class="pf-v5-c-dual-list-selector__item-text"
                >Item3 (disabled)</span>
              </span>
              <span class="pf-v5-c-dual-list-selector__item-count">
                <span class="pf-v5-c-badge pf-m-read"></span>
              </span>
            </span>
          </div>
        </li>
        <li class="pf-v5-c-dual-list-selector__list-item" role="option">
          <div class="pf-v5-c-dual-list-selector__list-item-row">
            <span class="pf-v5-c-dual-list-selector__item">
              <span class="pf-v5-c-dual-list-selector__item-main">
                <span class="pf-v5-c-dual-list-selector__item-text">Item4</span>
              </span>
              <span class="pf-v5-c-dual-list-selector__item-count">
                <span class="pf-v5-c-badge pf-m-read"></span>
              </span>
            </span>
          </div>
        </li>
        <li class="pf-v5-c-dual-list-selector__list-item" role="option">
          <div class="pf-v5-c-dual-list-selector__list-item-row">
            <span class="pf-v5-c-dual-list-selector__item">
              <span class="pf-v5-c-dual-list-selector__item-main">
                <span class="pf-v5-c-dual-list-selector__item-text">Item5</span>
              </span>
              <span class="pf-v5-c-dual-list-selector__item-count">
                <span class="pf-v5-c-badge pf-m-read"></span>
              </span>
            </span>
          </div>
        </li>
      </ul>
    </div>
  </div>
  <div class="pf-v5-c-dual-list-selector__controls">
    <div class="pf-v5-c-dual-list-selector__controls-item">
      <button
        class="pf-v5-c-button pf-m-plain"
        type="button"
        disabled
        aria-label="Add selected"
      >
        <i class="fas fa-fw fa-angle-right"></i>
      </button>
    </div>
    <div class="pf-v5-c-dual-list-selector__controls-item">
      <button
        class="pf-v5-c-button pf-m-plain"
        type="button"
        aria-label="Add all"
      >
        <i class="fas fa-fw fa-angle-double-right"></i>
      </button>
    </div>
    <div class="pf-v5-c-dual-list-selector__controls-item">
      <button
        class="pf-v5-c-button pf-m-plain"
        type="button"
        disabled
        aria-label="Remove all"
      >
        <i class="fas fa-fw fa-angle-double-left"></i>
      </button>
    </div>
    <div class="pf-v5-c-dual-list-selector__controls-item">
      <button
        class="pf-v5-c-button pf-m-plain"
        type="button"
        disabled
        aria-label="Remove selected"
      >
        <i class="fas fa-fw fa-angle-left"></i>
      </button>
    </div>
  </div>
  <div class="pf-v5-c-dual-list-selector__pane pf-m-chosen">
    <div class="pf-v5-c-dual-list-selector__header">
      <div class="pf-v5-c-dual-list-selector__title">
        <div class="pf-v5-c-dual-list-selector__title-text">Chosen options</div>
      </div>
    </div>
    <div class="pf-v5-c-dual-list-selector__tools">
      <div class="pf-v5-c-dual-list-selector__tools-filter">
        <div class="pf-v5-c-text-input-group">
          <div class="pf-v5-c-text-input-group__main pf-m-icon">
            <span class="pf-v5-c-text-input-group__text">
              <span class="pf-v5-c-text-input-group__icon">
                <i class="fas fa-fw fa-search"></i>
              </span>
              <input
                class="pf-v5-c-text-input-group__text-input"
                type="text"
                value
                aria-label="Search input"
              />
            </span>
          </div>
        </div>
      </div>
      <div class="pf-v5-c-dual-list-selector__tools-actions">
        <button
          class="pf-v5-c-button pf-m-plain"
          type="button"
          aria-label="Sort"
        >
          <i
            class="fas fa-sort-amount-down pf-v5-m-mirror-inline-rtl"
            aria-hidden="true"
          ></i>
        </button>
        <div class="pf-v5-c-dropdown">
          <button
            class="pf-v5-c-dropdown__toggle pf-m-plain"
            id="dropdown-kebab-basic-chosen-button"
            aria-expanded="false"
            type="button"
            aria-label="Actions"
          >
            <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
          </button>
          <ul
            class="pf-v5-c-dropdown__menu"
            aria-labelledby="dropdown-kebab-basic-chosen-button"
            hidden
            role="menu"
          >
            <li role="none">
              <a
                class="pf-v5-c-dropdown__menu-item"
                role="menuitem"
                href="#"
              >Link</a>
            </li>
            <li role="none">
              <button
                class="pf-v5-c-dropdown__menu-item"
                role="menuitem"
                type="button"
              >Action</button>
            </li>
            <li role="none">
              <a
                class="pf-v5-c-dropdown__menu-item pf-m-disabled"
                role="menuitem"
                href="#"
                aria-disabled="true"
                tabindex="-1"
              >Disabled link</a>
            </li>
            <li role="none">
              <button
                class="pf-v5-c-dropdown__menu-item"
                role="menuitem"
                type="button"
                disabled
              >Disabled action</button>
            </li>
            <li class="pf-v5-c-divider" role="separator"></li>
            <li role="none">
              <a
                class="pf-v5-c-dropdown__menu-item"
                role="menuitem"
                href="#"
              >Separated link</a>
            </li>
          </ul>
        </div>
      </div>
    </div>
    <div class="pf-v5-c-dual-list-selector__status">
      <span
        class="pf-v5-c-dual-list-selector__status-text"
        id="basic-chosen-status-text"
      >0 of 0 items selected</span>
    </div>
    <div class="pf-v5-c-dual-list-selector__menu">
      <ul
        class="pf-v5-c-dual-list-selector__list"
        role="listbox"
        aria-labelledby="basic-chosen-status-text"
        aria-multiselectable="true"
        aria-activedescendant
      ></ul>
    </div>
  </div>
</div>

```

### Available item selected

```html
<div class="pf-v5-c-dual-list-selector">
  <div class="pf-v5-c-dual-list-selector__pane pf-m-available">
    <div class="pf-v5-c-dual-list-selector__header">
      <div class="pf-v5-c-dual-list-selector__title">
        <div class="pf-v5-c-dual-list-selector__title-text">Available options</div>
      </div>
    </div>
    <div class="pf-v5-c-dual-list-selector__tools">
      <div class="pf-v5-c-dual-list-selector__tools-filter">
        <div class="pf-v5-c-text-input-group">
          <div class="pf-v5-c-text-input-group__main pf-m-icon">
            <span class="pf-v5-c-text-input-group__text">
              <span class="pf-v5-c-text-input-group__icon">
                <i class="fas fa-fw fa-search"></i>
              </span>
              <input
                class="pf-v5-c-text-input-group__text-input"
                type="text"
                value
                aria-label="Search input"
              />
            </span>
          </div>
        </div>
      </div>
      <div class="pf-v5-c-dual-list-selector__tools-actions">
        <button
          class="pf-v5-c-button pf-m-plain"
          type="button"
          aria-label="Sort"
        >
          <i
            class="fas fa-sort-amount-down pf-v5-m-mirror-inline-rtl"
            aria-hidden="true"
          ></i>
        </button>
        <div class="pf-v5-c-dropdown">
          <button
            class="pf-v5-c-dropdown__toggle pf-m-plain"
            id="dropdown-kebab-available-item-selected-available-button"
            aria-expanded="false"
            type="button"
            aria-label="Actions"
          >
            <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
          </button>
          <ul
            class="pf-v5-c-dropdown__menu"
            aria-labelledby="dropdown-kebab-available-item-selected-available-button"
            hidden
            role="menu"
          >
            <li role="none">
              <a
                class="pf-v5-c-dropdown__menu-item"
                role="menuitem"
                href="#"
              >Link</a>
            </li>
            <li role="none">
              <button
                class="pf-v5-c-dropdown__menu-item"
                role="menuitem"
                type="button"
              >Action</button>
            </li>
            <li role="none">
              <a
                class="pf-v5-c-dropdown__menu-item pf-m-disabled"
                role="menuitem"
                href="#"
                aria-disabled="true"
                tabindex="-1"
              >Disabled link</a>
            </li>
            <li role="none">
              <button
                class="pf-v5-c-dropdown__menu-item"
                role="menuitem"
                type="button"
                disabled
              >Disabled action</button>
            </li>
            <li class="pf-v5-c-divider" role="separator"></li>
            <li role="none">
              <a
                class="pf-v5-c-dropdown__menu-item"
                role="menuitem"
                href="#"
              >Separated link</a>
            </li>
          </ul>
        </div>
      </div>
    </div>
    <div class="pf-v5-c-dual-list-selector__status">
      <span
        class="pf-v5-c-dual-list-selector__status-text"
        id="available-item-selected-available-status-text"
      >1 of 5 items selected</span>
    </div>
    <div class="pf-v5-c-dual-list-selector__menu">
      <ul
        class="pf-v5-c-dual-list-selector__list"
        role="listbox"
        aria-labelledby="available-item-selected-available-status-text"
        aria-multiselectable="true"
        aria-activedescendant
      >
        <li class="pf-v5-c-dual-list-selector__list-item" role="option">
          <div class="pf-v5-c-dual-list-selector__list-item-row">
            <span class="pf-v5-c-dual-list-selector__item">
              <span class="pf-v5-c-dual-list-selector__item-main">
                <span class="pf-v5-c-dual-list-selector__item-text">Item1</span>
              </span>
              <span class="pf-v5-c-dual-list-selector__item-count">
                <span class="pf-v5-c-badge pf-m-read"></span>
              </span>
            </span>
          </div>
        </li>
        <li class="pf-v5-c-dual-list-selector__list-item" role="option">
          <div class="pf-v5-c-dual-list-selector__list-item-row pf-m-selected">
            <span class="pf-v5-c-dual-list-selector__item">
              <span class="pf-v5-c-dual-list-selector__item-main">
                <span class="pf-v5-c-dual-list-selector__item-text">Item2</span>
              </span>
              <span class="pf-v5-c-dual-list-selector__item-count">
                <span class="pf-v5-c-badge pf-m-read"></span>
              </span>
            </span>
          </div>
        </li>
        <li class="pf-v5-c-dual-list-selector__list-item" role="option">
          <div class="pf-v5-c-dual-list-selector__list-item-row">
            <span class="pf-v5-c-dual-list-selector__item">
              <span class="pf-v5-c-dual-list-selector__item-main">
                <span class="pf-v5-c-dual-list-selector__item-text">Item3</span>
              </span>
              <span class="pf-v5-c-dual-list-selector__item-count">
                <span class="pf-v5-c-badge pf-m-read"></span>
              </span>
            </span>
          </div>
        </li>
        <li class="pf-v5-c-dual-list-selector__list-item" role="option">
          <div class="pf-v5-c-dual-list-selector__list-item-row">
            <span class="pf-v5-c-dual-list-selector__item">
              <span class="pf-v5-c-dual-list-selector__item-main">
                <span class="pf-v5-c-dual-list-selector__item-text">Item4</span>
              </span>
              <span class="pf-v5-c-dual-list-selector__item-count">
                <span class="pf-v5-c-badge pf-m-read"></span>
              </span>
            </span>
          </div>
        </li>
        <li class="pf-v5-c-dual-list-selector__list-item" role="option">
          <div class="pf-v5-c-dual-list-selector__list-item-row">
            <span class="pf-v5-c-dual-list-selector__item">
              <span class="pf-v5-c-dual-list-selector__item-main">
                <span class="pf-v5-c-dual-list-selector__item-text">Item5</span>
              </span>
              <span class="pf-v5-c-dual-list-selector__item-count">
                <span class="pf-v5-c-badge pf-m-read"></span>
              </span>
            </span>
          </div>
        </li>
      </ul>
    </div>
  </div>
  <div class="pf-v5-c-dual-list-selector__controls">
    <div class="pf-v5-c-dual-list-selector__controls-item">
      <button
        class="pf-v5-c-button pf-m-plain"
        type="button"
        aria-label="Add selected"
      >
        <i class="fas fa-fw fa-angle-right"></i>
      </button>
    </div>
    <div class="pf-v5-c-dual-list-selector__controls-item">
      <button
        class="pf-v5-c-button pf-m-plain"
        type="button"
        aria-label="Add all"
      >
        <i class="fas fa-fw fa-angle-double-right"></i>
      </button>
    </div>
    <div class="pf-v5-c-dual-list-selector__controls-item">
      <button
        class="pf-v5-c-button pf-m-plain"
        type="button"
        disabled
        aria-label="Remove all"
      >
        <i class="fas fa-fw fa-angle-double-left"></i>
      </button>
    </div>
    <div class="pf-v5-c-dual-list-selector__controls-item">
      <button
        class="pf-v5-c-button pf-m-plain"
        type="button"
        disabled
        aria-label="Remove selected"
      >
        <i class="fas fa-fw fa-angle-left"></i>
      </button>
    </div>
  </div>
  <div class="pf-v5-c-dual-list-selector__pane pf-m-chosen">
    <div class="pf-v5-c-dual-list-selector__header">
      <div class="pf-v5-c-dual-list-selector__title">
        <div class="pf-v5-c-dual-list-selector__title-text">Chosen options</div>
      </div>
    </div>
    <div class="pf-v5-c-dual-list-selector__tools">
      <div class="pf-v5-c-dual-list-selector__tools-filter">
        <div class="pf-v5-c-text-input-group">
          <div class="pf-v5-c-text-input-group__main pf-m-icon">
            <span class="pf-v5-c-text-input-group__text">
              <span class="pf-v5-c-text-input-group__icon">
                <i class="fas fa-fw fa-search"></i>
              </span>
              <input
                class="pf-v5-c-text-input-group__text-input"
                type="text"
                value
                aria-label="Search input"
              />
            </span>
          </div>
        </div>
      </div>
      <div class="pf-v5-c-dual-list-selector__tools-actions">
        <button
          class="pf-v5-c-button pf-m-plain"
          type="button"
          aria-label="Sort"
        >
          <i
            class="fas fa-sort-amount-down pf-v5-m-mirror-inline-rtl"
            aria-hidden="true"
          ></i>
        </button>
        <div class="pf-v5-c-dropdown">
          <button
            class="pf-v5-c-dropdown__toggle pf-m-plain"
            id="dropdown-kebab-available-item-selected-chosen-button"
            aria-expanded="false"
            type="button"
            aria-label="Actions"
          >
            <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
          </button>
          <ul
            class="pf-v5-c-dropdown__menu"
            aria-labelledby="dropdown-kebab-available-item-selected-chosen-button"
            hidden
            role="menu"
          >
            <li role="none">
              <a
                class="pf-v5-c-dropdown__menu-item"
                role="menuitem"
                href="#"
              >Link</a>
            </li>
            <li role="none">
              <button
                class="pf-v5-c-dropdown__menu-item"
                role="menuitem"
                type="button"
              >Action</button>
            </li>
            <li role="none">
              <a
                class="pf-v5-c-dropdown__menu-item pf-m-disabled"
                role="menuitem"
                href="#"
                aria-disabled="true"
                tabindex="-1"
              >Disabled link</a>
            </li>
            <li role="none">
              <button
                class="pf-v5-c-dropdown__menu-item"
                role="menuitem"
                type="button"
                disabled
              >Disabled action</button>
            </li>
            <li class="pf-v5-c-divider" role="separator"></li>
            <li role="none">
              <a
                class="pf-v5-c-dropdown__menu-item"
                role="menuitem"
                href="#"
              >Separated link</a>
            </li>
          </ul>
        </div>
      </div>
    </div>
    <div class="pf-v5-c-dual-list-selector__status">
      <span
        class="pf-v5-c-dual-list-selector__status-text"
        id="available-item-selected-chosen-status-text"
      >0 of 0 items selected</span>
    </div>
    <div class="pf-v5-c-dual-list-selector__menu">
      <ul
        class="pf-v5-c-dual-list-selector__list"
        role="listbox"
        aria-labelledby="available-item-selected-chosen-status-text"
        aria-multiselectable="true"
        aria-activedescendant
      ></ul>
    </div>
  </div>
</div>

```

### Multiple available items selected

```html
<div class="pf-v5-c-dual-list-selector">
  <div class="pf-v5-c-dual-list-selector__pane pf-m-available">
    <div class="pf-v5-c-dual-list-selector__header">
      <div class="pf-v5-c-dual-list-selector__title">
        <div class="pf-v5-c-dual-list-selector__title-text">Available options</div>
      </div>
    </div>
    <div class="pf-v5-c-dual-list-selector__tools">
      <div class="pf-v5-c-dual-list-selector__tools-filter">
        <div class="pf-v5-c-text-input-group">
          <div class="pf-v5-c-text-input-group__main pf-m-icon">
            <span class="pf-v5-c-text-input-group__text">
              <span class="pf-v5-c-text-input-group__icon">
                <i class="fas fa-fw fa-search"></i>
              </span>
              <input
                class="pf-v5-c-text-input-group__text-input"
                type="text"
                value
                aria-label="Search input"
              />
            </span>
          </div>
        </div>
      </div>
      <div class="pf-v5-c-dual-list-selector__tools-actions">
        <button
          class="pf-v5-c-button pf-m-plain"
          type="button"
          aria-label="Sort"
        >
          <i
            class="fas fa-sort-amount-down pf-v5-m-mirror-inline-rtl"
            aria-hidden="true"
          ></i>
        </button>
        <div class="pf-v5-c-dropdown">
          <button
            class="pf-v5-c-dropdown__toggle pf-m-plain"
            id="dropdown-kebab-multiple-available-items-selected-available-button"
            aria-expanded="false"
            type="button"
            aria-label="Actions"
          >
            <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
          </button>
          <ul
            class="pf-v5-c-dropdown__menu"
            aria-labelledby="dropdown-kebab-multiple-available-items-selected-available-button"
            hidden
            role="menu"
          >
            <li role="none">
              <a
                class="pf-v5-c-dropdown__menu-item"
                role="menuitem"
                href="#"
              >Link</a>
            </li>
            <li role="none">
              <button
                class="pf-v5-c-dropdown__menu-item"
                role="menuitem"
                type="button"
              >Action</button>
            </li>
            <li role="none">
              <a
                class="pf-v5-c-dropdown__menu-item pf-m-disabled"
                role="menuitem"
                href="#"
                aria-disabled="true"
                tabindex="-1"
              >Disabled link</a>
            </li>
            <li role="none">
              <button
                class="pf-v5-c-dropdown__menu-item"
                role="menuitem"
                type="button"
                disabled
              >Disabled action</button>
            </li>
            <li class="pf-v5-c-divider" role="separator"></li>
            <li role="none">
              <a
                class="pf-v5-c-dropdown__menu-item"
                role="menuitem"
                href="#"
              >Separated link</a>
            </li>
          </ul>
        </div>
      </div>
    </div>
    <div class="pf-v5-c-dual-list-selector__status">
      <span
        class="pf-v5-c-dual-list-selector__status-text"
        id="multiple-available-items-selected-available-status-text"
      >3 of 5 items selected</span>
    </div>
    <div class="pf-v5-c-dual-list-selector__menu">
      <ul
        class="pf-v5-c-dual-list-selector__list"
        role="listbox"
        aria-labelledby="multiple-available-items-selected-available-status-text"
        aria-multiselectable="true"
        aria-activedescendant
      >
        <li class="pf-v5-c-dual-list-selector__list-item" role="option">
          <div class="pf-v5-c-dual-list-selector__list-item-row">
            <span class="pf-v5-c-dual-list-selector__item">
              <span class="pf-v5-c-dual-list-selector__item-main">
                <span class="pf-v5-c-dual-list-selector__item-text">Item1</span>
              </span>
              <span class="pf-v5-c-dual-list-selector__item-count">
                <span class="pf-v5-c-badge pf-m-read"></span>
              </span>
            </span>
          </div>
        </li>
        <li class="pf-v5-c-dual-list-selector__list-item" role="option">
          <div class="pf-v5-c-dual-list-selector__list-item-row pf-m-selected">
            <span class="pf-v5-c-dual-list-selector__item">
              <span class="pf-v5-c-dual-list-selector__item-main">
                <span class="pf-v5-c-dual-list-selector__item-text">Item2</span>
              </span>
              <span class="pf-v5-c-dual-list-selector__item-count">
                <span class="pf-v5-c-badge pf-m-read"></span>
              </span>
            </span>
          </div>
        </li>
        <li class="pf-v5-c-dual-list-selector__list-item" role="option">
          <div class="pf-v5-c-dual-list-selector__list-item-row pf-m-selected">
            <span class="pf-v5-c-dual-list-selector__item">
              <span class="pf-v5-c-dual-list-selector__item-main">
                <span class="pf-v5-c-dual-list-selector__item-text">Item3</span>
              </span>
              <span class="pf-v5-c-dual-list-selector__item-count">
                <span class="pf-v5-c-badge pf-m-read"></span>
              </span>
            </span>
          </div>
        </li>
        <li class="pf-v5-c-dual-list-selector__list-item" role="option">
          <div class="pf-v5-c-dual-list-selector__list-item-row">
            <span class="pf-v5-c-dual-list-selector__item">
              <span class="pf-v5-c-dual-list-selector__item-main">
                <span class="pf-v5-c-dual-list-selector__item-text">Item4</span>
              </span>
              <span class="pf-v5-c-dual-list-selector__item-count">
                <span class="pf-v5-c-badge pf-m-read"></span>
              </span>
            </span>
          </div>
        </li>
        <li class="pf-v5-c-dual-list-selector__list-item" role="option">
          <div class="pf-v5-c-dual-list-selector__list-item-row pf-m-selected">
            <span class="pf-v5-c-dual-list-selector__item">
              <span class="pf-v5-c-dual-list-selector__item-main">
                <span class="pf-v5-c-dual-list-selector__item-text">Item5</span>
              </span>
              <span class="pf-v5-c-dual-list-selector__item-count">
                <span class="pf-v5-c-badge pf-m-read"></span>
              </span>
            </span>
          </div>
        </li>
      </ul>
    </div>
  </div>
  <div class="pf-v5-c-dual-list-selector__controls">
    <div class="pf-v5-c-dual-list-selector__controls-item">
      <button
        class="pf-v5-c-button pf-m-plain"
        type="button"
        aria-label="Add selected"
      >
        <i class="fas fa-fw fa-angle-right"></i>
      </button>
    </div>
    <div class="pf-v5-c-dual-list-selector__controls-item">
      <button
        class="pf-v5-c-button pf-m-plain"
        type="button"
        aria-label="Add all"
      >
        <i class="fas fa-fw fa-angle-double-right"></i>
      </button>
    </div>
    <div class="pf-v5-c-dual-list-selector__controls-item">
      <button
        class="pf-v5-c-button pf-m-plain"
        type="button"
        disabled
        aria-label="Remove all"
      >
        <i class="fas fa-fw fa-angle-double-left"></i>
      </button>
    </div>
    <div class="pf-v5-c-dual-list-selector__controls-item">
      <button
        class="pf-v5-c-button pf-m-plain"
        type="button"
        disabled
        aria-label="Remove selected"
      >
        <i class="fas fa-fw fa-angle-left"></i>
      </button>
    </div>
  </div>
  <div class="pf-v5-c-dual-list-selector__pane pf-m-chosen">
    <div class="pf-v5-c-dual-list-selector__header">
      <div class="pf-v5-c-dual-list-selector__title">
        <div class="pf-v5-c-dual-list-selector__title-text">Chosen options</div>
      </div>
    </div>
    <div class="pf-v5-c-dual-list-selector__tools">
      <div class="pf-v5-c-dual-list-selector__tools-filter">
        <div class="pf-v5-c-text-input-group">
          <div class="pf-v5-c-text-input-group__main pf-m-icon">
            <span class="pf-v5-c-text-input-group__text">
              <span class="pf-v5-c-text-input-group__icon">
                <i class="fas fa-fw fa-search"></i>
              </span>
              <input
                class="pf-v5-c-text-input-group__text-input"
                type="text"
                value
                aria-label="Search input"
              />
            </span>
          </div>
        </div>
      </div>
      <div class="pf-v5-c-dual-list-selector__tools-actions">
        <button
          class="pf-v5-c-button pf-m-plain"
          type="button"
          aria-label="Sort"
        >
          <i
            class="fas fa-sort-amount-down pf-v5-m-mirror-inline-rtl"
            aria-hidden="true"
          ></i>
        </button>
        <div class="pf-v5-c-dropdown">
          <button
            class="pf-v5-c-dropdown__toggle pf-m-plain"
            id="dropdown-kebab-multiple-available-items-selected-chosen-button"
            aria-expanded="false"
            type="button"
            aria-label="Actions"
          >
            <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
          </button>
          <ul
            class="pf-v5-c-dropdown__menu"
            aria-labelledby="dropdown-kebab-multiple-available-items-selected-chosen-button"
            hidden
            role="menu"
          >
            <li role="none">
              <a
                class="pf-v5-c-dropdown__menu-item"
                role="menuitem"
                href="#"
              >Link</a>
            </li>
            <li role="none">
              <button
                class="pf-v5-c-dropdown__menu-item"
                role="menuitem"
                type="button"
              >Action</button>
            </li>
            <li role="none">
              <a
                class="pf-v5-c-dropdown__menu-item pf-m-disabled"
                role="menuitem"
                href="#"
                aria-disabled="true"
                tabindex="-1"
              >Disabled link</a>
            </li>
            <li role="none">
              <button
                class="pf-v5-c-dropdown__menu-item"
                role="menuitem"
                type="button"
                disabled
              >Disabled action</button>
            </li>
            <li class="pf-v5-c-divider" role="separator"></li>
            <li role="none">
              <a
                class="pf-v5-c-dropdown__menu-item"
                role="menuitem"
                href="#"
              >Separated link</a>
            </li>
          </ul>
        </div>
      </div>
    </div>
    <div class="pf-v5-c-dual-list-selector__status">
      <span
        class="pf-v5-c-dual-list-selector__status-text"
        id="multiple-available-items-selected-chosen-status-text"
      >0 of 0 items selected</span>
    </div>
    <div class="pf-v5-c-dual-list-selector__menu">
      <ul
        class="pf-v5-c-dual-list-selector__list"
        role="listbox"
        aria-labelledby="multiple-available-items-selected-chosen-status-text"
        aria-multiselectable="true"
        aria-activedescendant
      ></ul>
    </div>
  </div>
</div>

```

### Chosen item

```html
<div class="pf-v5-c-dual-list-selector">
  <div class="pf-v5-c-dual-list-selector__pane pf-m-available">
    <div class="pf-v5-c-dual-list-selector__header">
      <div class="pf-v5-c-dual-list-selector__title">
        <div class="pf-v5-c-dual-list-selector__title-text">Available options</div>
      </div>
    </div>
    <div class="pf-v5-c-dual-list-selector__tools">
      <div class="pf-v5-c-dual-list-selector__tools-filter">
        <div class="pf-v5-c-text-input-group">
          <div class="pf-v5-c-text-input-group__main pf-m-icon">
            <span class="pf-v5-c-text-input-group__text">
              <span class="pf-v5-c-text-input-group__icon">
                <i class="fas fa-fw fa-search"></i>
              </span>
              <input
                class="pf-v5-c-text-input-group__text-input"
                type="text"
                value
                aria-label="Search input"
              />
            </span>
          </div>
        </div>
      </div>
      <div class="pf-v5-c-dual-list-selector__tools-actions">
        <button
          class="pf-v5-c-button pf-m-plain"
          type="button"
          aria-label="Sort"
        >
          <i
            class="fas fa-sort-amount-down pf-v5-m-mirror-inline-rtl"
            aria-hidden="true"
          ></i>
        </button>
        <div class="pf-v5-c-dropdown">
          <button
            class="pf-v5-c-dropdown__toggle pf-m-plain"
            id="dropdown-kebab-chosen-item-available-button"
            aria-expanded="false"
            type="button"
            aria-label="Actions"
          >
            <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
          </button>
          <ul
            class="pf-v5-c-dropdown__menu"
            aria-labelledby="dropdown-kebab-chosen-item-available-button"
            hidden
            role="menu"
          >
            <li role="none">
              <a
                class="pf-v5-c-dropdown__menu-item"
                role="menuitem"
                href="#"
              >Link</a>
            </li>
            <li role="none">
              <button
                class="pf-v5-c-dropdown__menu-item"
                role="menuitem"
                type="button"
              >Action</button>
            </li>
            <li role="none">
              <a
                class="pf-v5-c-dropdown__menu-item pf-m-disabled"
                role="menuitem"
                href="#"
                aria-disabled="true"
                tabindex="-1"
              >Disabled link</a>
            </li>
            <li role="none">
              <button
                class="pf-v5-c-dropdown__menu-item"
                role="menuitem"
                type="button"
                disabled
              >Disabled action</button>
            </li>
            <li class="pf-v5-c-divider" role="separator"></li>
            <li role="none">
              <a
                class="pf-v5-c-dropdown__menu-item"
                role="menuitem"
                href="#"
              >Separated link</a>
            </li>
          </ul>
        </div>
      </div>
    </div>
    <div class="pf-v5-c-dual-list-selector__status">
      <span
        class="pf-v5-c-dual-list-selector__status-text"
        id="chosen-item-available-status-text"
      >0 of 4 items selected</span>
    </div>
    <div class="pf-v5-c-dual-list-selector__menu">
      <ul
        class="pf-v5-c-dual-list-selector__list"
        role="listbox"
        aria-labelledby="chosen-item-available-status-text"
        aria-multiselectable="true"
        aria-activedescendant
      >
        <li class="pf-v5-c-dual-list-selector__list-item" role="option">
          <div class="pf-v5-c-dual-list-selector__list-item-row">
            <span class="pf-v5-c-dual-list-selector__item">
              <span class="pf-v5-c-dual-list-selector__item-main">
                <span class="pf-v5-c-dual-list-selector__item-text">Item1</span>
              </span>
              <span class="pf-v5-c-dual-list-selector__item-count">
                <span class="pf-v5-c-badge pf-m-read"></span>
              </span>
            </span>
          </div>
        </li>
        <li class="pf-v5-c-dual-list-selector__list-item" role="option">
          <div class="pf-v5-c-dual-list-selector__list-item-row">
            <span class="pf-v5-c-dual-list-selector__item">
              <span class="pf-v5-c-dual-list-selector__item-main">
                <span class="pf-v5-c-dual-list-selector__item-text">Item2</span>
              </span>
              <span class="pf-v5-c-dual-list-selector__item-count">
                <span class="pf-v5-c-badge pf-m-read"></span>
              </span>
            </span>
          </div>
        </li>
        <li class="pf-v5-c-dual-list-selector__list-item" role="option">
          <div class="pf-v5-c-dual-list-selector__list-item-row">
            <span class="pf-v5-c-dual-list-selector__item">
              <span class="pf-v5-c-dual-list-selector__item-main">
                <span class="pf-v5-c-dual-list-selector__item-text">Item3</span>
              </span>
              <span class="pf-v5-c-dual-list-selector__item-count">
                <span class="pf-v5-c-badge pf-m-read"></span>
              </span>
            </span>
          </div>
        </li>
        <li class="pf-v5-c-dual-list-selector__list-item" role="option">
          <div class="pf-v5-c-dual-list-selector__list-item-row">
            <span class="pf-v5-c-dual-list-selector__item">
              <span class="pf-v5-c-dual-list-selector__item-main">
                <span class="pf-v5-c-dual-list-selector__item-text">Item4</span>
              </span>
              <span class="pf-v5-c-dual-list-selector__item-count">
                <span class="pf-v5-c-badge pf-m-read"></span>
              </span>
            </span>
          </div>
        </li>
        <li class="pf-v5-c-dual-list-selector__list-item" role="option">
          <div class="pf-v5-c-dual-list-selector__list-item-row">
            <span class="pf-v5-c-dual-list-selector__item">
              <span class="pf-v5-c-dual-list-selector__item-main">
                <span class="pf-v5-c-dual-list-selector__item-text">Item5</span>
              </span>
              <span class="pf-v5-c-dual-list-selector__item-count">
                <span class="pf-v5-c-badge pf-m-read"></span>
              </span>
            </span>
          </div>
        </li>
      </ul>
    </div>
  </div>
  <div class="pf-v5-c-dual-list-selector__controls">
    <div class="pf-v5-c-dual-list-selector__controls-item">
      <button
        class="pf-v5-c-button pf-m-plain"
        type="button"
        disabled
        aria-label="Add selected"
      >
        <i class="fas fa-fw fa-angle-right"></i>
      </button>
    </div>
    <div class="pf-v5-c-dual-list-selector__controls-item">
      <button
        class="pf-v5-c-button pf-m-plain"
        type="button"
        aria-label="Add all"
      >
        <i class="fas fa-fw fa-angle-double-right"></i>
      </button>
    </div>
    <div class="pf-v5-c-dual-list-selector__controls-item">
      <button
        class="pf-v5-c-button pf-m-plain"
        type="button"
        aria-label="Remove all"
      >
        <i class="fas fa-fw fa-angle-double-left"></i>
      </button>
    </div>
    <div class="pf-v5-c-dual-list-selector__controls-item">
      <button
        class="pf-v5-c-button pf-m-plain"
        type="button"
        disabled
        aria-label="Remove selected"
      >
        <i class="fas fa-fw fa-angle-left"></i>
      </button>
    </div>
  </div>
  <div class="pf-v5-c-dual-list-selector__pane pf-m-chosen">
    <div class="pf-v5-c-dual-list-selector__header">
      <div class="pf-v5-c-dual-list-selector__title">
        <div class="pf-v5-c-dual-list-selector__title-text">Chosen options</div>
      </div>
    </div>
    <div class="pf-v5-c-dual-list-selector__tools">
      <div class="pf-v5-c-dual-list-selector__tools-filter">
        <div class="pf-v5-c-text-input-group">
          <div class="pf-v5-c-text-input-group__main pf-m-icon">
            <span class="pf-v5-c-text-input-group__text">
              <span class="pf-v5-c-text-input-group__icon">
                <i class="fas fa-fw fa-search"></i>
              </span>
              <input
                class="pf-v5-c-text-input-group__text-input"
                type="text"
                value
                aria-label="Search input"
              />
            </span>
          </div>
        </div>
      </div>
      <div class="pf-v5-c-dual-list-selector__tools-actions">
        <button
          class="pf-v5-c-button pf-m-plain"
          type="button"
          aria-label="Sort"
        >
          <i
            class="fas fa-sort-amount-down pf-v5-m-mirror-inline-rtl"
            aria-hidden="true"
          ></i>
        </button>
        <div class="pf-v5-c-dropdown">
          <button
            class="pf-v5-c-dropdown__toggle pf-m-plain"
            id="dropdown-kebab-chosen-item-chosen-button"
            aria-expanded="false"
            type="button"
            aria-label="Actions"
          >
            <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
          </button>
          <ul
            class="pf-v5-c-dropdown__menu"
            aria-labelledby="dropdown-kebab-chosen-item-chosen-button"
            hidden
            role="menu"
          >
            <li role="none">
              <a
                class="pf-v5-c-dropdown__menu-item"
                role="menuitem"
                href="#"
              >Link</a>
            </li>
            <li role="none">
              <button
                class="pf-v5-c-dropdown__menu-item"
                role="menuitem"
                type="button"
              >Action</button>
            </li>
            <li role="none">
              <a
                class="pf-v5-c-dropdown__menu-item pf-m-disabled"
                role="menuitem"
                href="#"
                aria-disabled="true"
                tabindex="-1"
              >Disabled link</a>
            </li>
            <li role="none">
              <button
                class="pf-v5-c-dropdown__menu-item"
                role="menuitem"
                type="button"
                disabled
              >Disabled action</button>
            </li>
            <li class="pf-v5-c-divider" role="separator"></li>
            <li role="none">
              <a
                class="pf-v5-c-dropdown__menu-item"
                role="menuitem"
                href="#"
              >Separated link</a>
            </li>
          </ul>
        </div>
      </div>
    </div>
    <div class="pf-v5-c-dual-list-selector__status">
      <span
        class="pf-v5-c-dual-list-selector__status-text"
        id="chosen-item-chosen-status-text"
      >0 of 1 items selected</span>
    </div>
    <div class="pf-v5-c-dual-list-selector__menu">
      <ul
        class="pf-v5-c-dual-list-selector__list"
        role="listbox"
        aria-labelledby="chosen-item-chosen-status-text"
        aria-multiselectable="true"
        aria-activedescendant
      >
        <li class="pf-v5-c-dual-list-selector__list-item" role="option">
          <div class="pf-v5-c-dual-list-selector__list-item-row">
            <span class="pf-v5-c-dual-list-selector__item">
              <span class="pf-v5-c-dual-list-selector__item-main">
                <span class="pf-v5-c-dual-list-selector__item-text">Item1</span>
              </span>
              <span class="pf-v5-c-dual-list-selector__item-count">
                <span class="pf-v5-c-badge pf-m-read"></span>
              </span>
            </span>
          </div>
        </li>
      </ul>
    </div>
  </div>
</div>

```

### Chosen item selected

```html
<div class="pf-v5-c-dual-list-selector">
  <div class="pf-v5-c-dual-list-selector__pane pf-m-available">
    <div class="pf-v5-c-dual-list-selector__header">
      <div class="pf-v5-c-dual-list-selector__title">
        <div class="pf-v5-c-dual-list-selector__title-text">Available options</div>
      </div>
    </div>
    <div class="pf-v5-c-dual-list-selector__tools">
      <div class="pf-v5-c-dual-list-selector__tools-filter">
        <div class="pf-v5-c-text-input-group">
          <div class="pf-v5-c-text-input-group__main pf-m-icon">
            <span class="pf-v5-c-text-input-group__text">
              <span class="pf-v5-c-text-input-group__icon">
                <i class="fas fa-fw fa-search"></i>
              </span>
              <input
                class="pf-v5-c-text-input-group__text-input"
                type="text"
                value
                aria-label="Search input"
              />
            </span>
          </div>
        </div>
      </div>
      <div class="pf-v5-c-dual-list-selector__tools-actions">
        <button
          class="pf-v5-c-button pf-m-plain"
          type="button"
          aria-label="Sort"
        >
          <i
            class="fas fa-sort-amount-down pf-v5-m-mirror-inline-rtl"
            aria-hidden="true"
          ></i>
        </button>
        <div class="pf-v5-c-dropdown">
          <button
            class="pf-v5-c-dropdown__toggle pf-m-plain"
            id="dropdown-kebab-chosen-item-selected-available-button"
            aria-expanded="false"
            type="button"
            aria-label="Actions"
          >
            <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
          </button>
          <ul
            class="pf-v5-c-dropdown__menu"
            aria-labelledby="dropdown-kebab-chosen-item-selected-available-button"
            hidden
            role="menu"
          >
            <li role="none">
              <a
                class="pf-v5-c-dropdown__menu-item"
                role="menuitem"
                href="#"
              >Link</a>
            </li>
            <li role="none">
              <button
                class="pf-v5-c-dropdown__menu-item"
                role="menuitem"
                type="button"
              >Action</button>
            </li>
            <li role="none">
              <a
                class="pf-v5-c-dropdown__menu-item pf-m-disabled"
                role="menuitem"
                href="#"
                aria-disabled="true"
                tabindex="-1"
              >Disabled link</a>
            </li>
            <li role="none">
              <button
                class="pf-v5-c-dropdown__menu-item"
                role="menuitem"
                type="button"
                disabled
              >Disabled action</button>
            </li>
            <li class="pf-v5-c-divider" role="separator"></li>
            <li role="none">
              <a
                class="pf-v5-c-dropdown__menu-item"
                role="menuitem"
                href="#"
              >Separated link</a>
            </li>
          </ul>
        </div>
      </div>
    </div>
    <div class="pf-v5-c-dual-list-selector__status">
      <span
        class="pf-v5-c-dual-list-selector__status-text"
        id="chosen-item-selected-available-status-text"
      >0 of 4 items selected</span>
    </div>
    <div class="pf-v5-c-dual-list-selector__menu">
      <ul
        class="pf-v5-c-dual-list-selector__list"
        role="listbox"
        aria-labelledby="chosen-item-selected-available-status-text"
        aria-multiselectable="true"
        aria-activedescendant
      >
        <li class="pf-v5-c-dual-list-selector__list-item" role="option">
          <div class="pf-v5-c-dual-list-selector__list-item-row">
            <span class="pf-v5-c-dual-list-selector__item">
              <span class="pf-v5-c-dual-list-selector__item-main">
                <span class="pf-v5-c-dual-list-selector__item-text">Item1</span>
              </span>
              <span class="pf-v5-c-dual-list-selector__item-count">
                <span class="pf-v5-c-badge pf-m-read"></span>
              </span>
            </span>
          </div>
        </li>
        <li class="pf-v5-c-dual-list-selector__list-item" role="option">
          <div class="pf-v5-c-dual-list-selector__list-item-row">
            <span class="pf-v5-c-dual-list-selector__item">
              <span class="pf-v5-c-dual-list-selector__item-main">
                <span class="pf-v5-c-dual-list-selector__item-text">Item2</span>
              </span>
              <span class="pf-v5-c-dual-list-selector__item-count">
                <span class="pf-v5-c-badge pf-m-read"></span>
              </span>
            </span>
          </div>
        </li>
        <li class="pf-v5-c-dual-list-selector__list-item" role="option">
          <div class="pf-v5-c-dual-list-selector__list-item-row">
            <span class="pf-v5-c-dual-list-selector__item">
              <span class="pf-v5-c-dual-list-selector__item-main">
                <span class="pf-v5-c-dual-list-selector__item-text">Item3</span>
              </span>
              <span class="pf-v5-c-dual-list-selector__item-count">
                <span class="pf-v5-c-badge pf-m-read"></span>
              </span>
            </span>
          </div>
        </li>
        <li class="pf-v5-c-dual-list-selector__list-item" role="option">
          <div class="pf-v5-c-dual-list-selector__list-item-row">
            <span class="pf-v5-c-dual-list-selector__item">
              <span class="pf-v5-c-dual-list-selector__item-main">
                <span class="pf-v5-c-dual-list-selector__item-text">Item4</span>
              </span>
              <span class="pf-v5-c-dual-list-selector__item-count">
                <span class="pf-v5-c-badge pf-m-read"></span>
              </span>
            </span>
          </div>
        </li>
        <li class="pf-v5-c-dual-list-selector__list-item" role="option">
          <div class="pf-v5-c-dual-list-selector__list-item-row">
            <span class="pf-v5-c-dual-list-selector__item">
              <span class="pf-v5-c-dual-list-selector__item-main">
                <span class="pf-v5-c-dual-list-selector__item-text">Item5</span>
              </span>
              <span class="pf-v5-c-dual-list-selector__item-count">
                <span class="pf-v5-c-badge pf-m-read"></span>
              </span>
            </span>
          </div>
        </li>
      </ul>
    </div>
  </div>
  <div class="pf-v5-c-dual-list-selector__controls">
    <div class="pf-v5-c-dual-list-selector__controls-item">
      <button
        class="pf-v5-c-button pf-m-plain"
        type="button"
        disabled
        aria-label="Add selected"
      >
        <i class="fas fa-fw fa-angle-right"></i>
      </button>
    </div>
    <div class="pf-v5-c-dual-list-selector__controls-item">
      <button
        class="pf-v5-c-button pf-m-plain"
        type="button"
        aria-label="Add all"
      >
        <i class="fas fa-fw fa-angle-double-right"></i>
      </button>
    </div>
    <div class="pf-v5-c-dual-list-selector__controls-item">
      <button
        class="pf-v5-c-button pf-m-plain"
        type="button"
        aria-label="Remove all"
      >
        <i class="fas fa-fw fa-angle-double-left"></i>
      </button>
    </div>
    <div class="pf-v5-c-dual-list-selector__controls-item">
      <button
        class="pf-v5-c-button pf-m-plain"
        type="button"
        aria-label="Remove selected"
      >
        <i class="fas fa-fw fa-angle-left"></i>
      </button>
    </div>
  </div>
  <div class="pf-v5-c-dual-list-selector__pane pf-m-chosen">
    <div class="pf-v5-c-dual-list-selector__header">
      <div class="pf-v5-c-dual-list-selector__title">
        <div class="pf-v5-c-dual-list-selector__title-text">Chosen options</div>
      </div>
    </div>
    <div class="pf-v5-c-dual-list-selector__tools">
      <div class="pf-v5-c-dual-list-selector__tools-filter">
        <div class="pf-v5-c-text-input-group">
          <div class="pf-v5-c-text-input-group__main pf-m-icon">
            <span class="pf-v5-c-text-input-group__text">
              <span class="pf-v5-c-text-input-group__icon">
                <i class="fas fa-fw fa-search"></i>
              </span>
              <input
                class="pf-v5-c-text-input-group__text-input"
                type="text"
                value
                aria-label="Search input"
              />
            </span>
          </div>
        </div>
      </div>
      <div class="pf-v5-c-dual-list-selector__tools-actions">
        <button
          class="pf-v5-c-button pf-m-plain"
          type="button"
          aria-label="Sort"
        >
          <i
            class="fas fa-sort-amount-down pf-v5-m-mirror-inline-rtl"
            aria-hidden="true"
          ></i>
        </button>
        <div class="pf-v5-c-dropdown">
          <button
            class="pf-v5-c-dropdown__toggle pf-m-plain"
            id="dropdown-kebab-chosen-item-selected-chosen-button"
            aria-expanded="false"
            type="button"
            aria-label="Actions"
          >
            <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
          </button>
          <ul
            class="pf-v5-c-dropdown__menu"
            aria-labelledby="dropdown-kebab-chosen-item-selected-chosen-button"
            hidden
            role="menu"
          >
            <li role="none">
              <a
                class="pf-v5-c-dropdown__menu-item"
                role="menuitem"
                href="#"
              >Link</a>
            </li>
            <li role="none">
              <button
                class="pf-v5-c-dropdown__menu-item"
                role="menuitem"
                type="button"
              >Action</button>
            </li>
            <li role="none">
              <a
                class="pf-v5-c-dropdown__menu-item pf-m-disabled"
                role="menuitem"
                href="#"
                aria-disabled="true"
                tabindex="-1"
              >Disabled link</a>
            </li>
            <li role="none">
              <button
                class="pf-v5-c-dropdown__menu-item"
                role="menuitem"
                type="button"
                disabled
              >Disabled action</button>
            </li>
            <li class="pf-v5-c-divider" role="separator"></li>
            <li role="none">
              <a
                class="pf-v5-c-dropdown__menu-item"
                role="menuitem"
                href="#"
              >Separated link</a>
            </li>
          </ul>
        </div>
      </div>
    </div>
    <div class="pf-v5-c-dual-list-selector__status">
      <span
        class="pf-v5-c-dual-list-selector__status-text"
        id="chosen-item-selected-chosen-status-text"
      >1 of 1 items selected</span>
    </div>
    <div class="pf-v5-c-dual-list-selector__menu">
      <ul
        class="pf-v5-c-dual-list-selector__list"
        role="listbox"
        aria-labelledby="chosen-item-selected-chosen-status-text"
        aria-multiselectable="true"
        aria-activedescendant
      >
        <li class="pf-v5-c-dual-list-selector__list-item" role="option">
          <div class="pf-v5-c-dual-list-selector__list-item-row pf-m-selected">
            <span class="pf-v5-c-dual-list-selector__item">
              <span class="pf-v5-c-dual-list-selector__item-main">
                <span class="pf-v5-c-dual-list-selector__item-text">Item5</span>
              </span>
              <span class="pf-v5-c-dual-list-selector__item-count">
                <span class="pf-v5-c-badge pf-m-read"></span>
              </span>
            </span>
          </div>
        </li>
      </ul>
    </div>
  </div>
</div>

```

### Tree view

```html
<div class="pf-v5-c-dual-list-selector">
  <div class="pf-v5-c-dual-list-selector__pane pf-m-available">
    <div class="pf-v5-c-dual-list-selector__header">
      <div class="pf-v5-c-dual-list-selector__title">
        <div class="pf-v5-c-dual-list-selector__title-text">Available options</div>
      </div>
    </div>
    <div class="pf-v5-c-dual-list-selector__tools">
      <div class="pf-v5-c-dual-list-selector__tools-filter">
        <div class="pf-v5-c-text-input-group">
          <div class="pf-v5-c-text-input-group__main pf-m-icon">
            <span class="pf-v5-c-text-input-group__text">
              <span class="pf-v5-c-text-input-group__icon">
                <i class="fas fa-fw fa-search"></i>
              </span>
              <input
                class="pf-v5-c-text-input-group__text-input"
                type="text"
                value
                aria-label="Search input"
              />
            </span>
          </div>
        </div>
      </div>
      <div class="pf-v5-c-dual-list-selector__tools-actions">
        <button
          class="pf-v5-c-button pf-m-plain"
          type="button"
          aria-label="Sort"
        >
          <i
            class="fas fa-sort-amount-down pf-v5-m-mirror-inline-rtl"
            aria-hidden="true"
          ></i>
        </button>
        <div class="pf-v5-c-dropdown">
          <button
            class="pf-v5-c-dropdown__toggle pf-m-plain"
            id="dropdown-kebab-tree-available-button"
            aria-expanded="false"
            type="button"
            aria-label="Actions"
          >
            <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
          </button>
          <ul
            class="pf-v5-c-dropdown__menu"
            aria-labelledby="dropdown-kebab-tree-available-button"
            hidden
            role="menu"
          >
            <li role="none">
              <a
                class="pf-v5-c-dropdown__menu-item"
                role="menuitem"
                href="#"
              >Link</a>
            </li>
            <li role="none">
              <button
                class="pf-v5-c-dropdown__menu-item"
                role="menuitem"
                type="button"
              >Action</button>
            </li>
            <li role="none">
              <a
                class="pf-v5-c-dropdown__menu-item pf-m-disabled"
                role="menuitem"
                href="#"
                aria-disabled="true"
                tabindex="-1"
              >Disabled link</a>
            </li>
            <li role="none">
              <button
                class="pf-v5-c-dropdown__menu-item"
                role="menuitem"
                type="button"
                disabled
              >Disabled action</button>
            </li>
            <li class="pf-v5-c-divider" role="separator"></li>
            <li role="none">
              <a
                class="pf-v5-c-dropdown__menu-item"
                role="menuitem"
                href="#"
              >Separated link</a>
            </li>
          </ul>
        </div>
      </div>
    </div>
    <div class="pf-v5-c-dual-list-selector__status">
      <span
        class="pf-v5-c-dual-list-selector__status-text"
        id="tree-available-status-text"
      >1 of 11 items selected</span>
    </div>
    <div class="pf-v5-c-dual-list-selector__menu">
      <ul
        class="pf-v5-c-dual-list-selector__list"
        role="tree"
        aria-labelledby="tree-available-status-text"
        aria-multiselectable="true"
        aria-activedescendant
      >
        <li
          class="pf-v5-c-dual-list-selector__list-item pf-m-expandable pf-m-expanded"
          aria-expanded="true"
          role="treeitem"
        >
          <div class="pf-v5-c-dual-list-selector__list-item-row pf-m-check">
            <div class="pf-v5-c-dual-list-selector__item" tabindex="0">
              <span class="pf-v5-c-dual-list-selector__item-main">
                <div class="pf-v5-c-dual-list-selector__item-toggle">
                  <span class="pf-v5-c-dual-list-selector__item-toggle-icon">
                    <i class="fas fa-angle-right" aria-hidden="true"></i>
                  </span>
                </div>
                <span class="pf-v5-c-dual-list-selector__item-check">
                  <div class="pf-v5-c-check pf-m-standalone">
                    <input
                      class="pf-v5-c-check__input"
                      type="checkbox"
                      id="check-0"
                      aria-label="Dual list selector item check"
                    />
                  </div>
                </span>
                <span class="pf-v5-c-dual-list-selector__item-text">Colors</span>
              </span>
              <span class="pf-v5-c-dual-list-selector__item-count">
                <span class="pf-v5-c-badge pf-m-read">6</span>
              </span>
            </div>
          </div>

          <ul
            class="pf-v5-c-dual-list-selector__list"
            role="group"
            aria-labelledby="-status-text"
          >
            <li class="pf-v5-c-dual-list-selector__list-item" role="treeitem">
              <div class="pf-v5-c-dual-list-selector__list-item-row pf-m-check">
                <div class="pf-v5-c-dual-list-selector__item" tabindex="0">
                  <span class="pf-v5-c-dual-list-selector__item-main">
                    <span class="pf-v5-c-dual-list-selector__item-check">
                      <div class="pf-v5-c-check pf-m-standalone">
                        <input
                          class="pf-v5-c-check__input"
                          type="checkbox"
                          id="check-1"
                          aria-label="Dual list selector item check"
                        />
                      </div>
                    </span>
                    <span class="pf-v5-c-dual-list-selector__item-text">Red</span>
                  </span>
                  <span class="pf-v5-c-dual-list-selector__item-count">
                    <span class="pf-v5-c-badge pf-m-read"></span>
                  </span>
                </div>
              </div>
            </li>
            <li class="pf-v5-c-dual-list-selector__list-item" role="treeitem">
              <div
                class="pf-v5-c-dual-list-selector__list-item-row pf-m-check pf-m-selected"
              >
                <div class="pf-v5-c-dual-list-selector__item" tabindex="0">
                  <span class="pf-v5-c-dual-list-selector__item-main">
                    <span class="pf-v5-c-dual-list-selector__item-check">
                      <div class="pf-v5-c-check pf-m-standalone">
                        <input
                          class="pf-v5-c-check__input"
                          type="checkbox"
                          id="check-2"
                          checked
                          aria-label="Dual list selector item check checked"
                        />
                      </div>
                    </span>
                    <span class="pf-v5-c-dual-list-selector__item-text">Orange</span>
                  </span>
                  <span class="pf-v5-c-dual-list-selector__item-count">
                    <span class="pf-v5-c-badge pf-m-read"></span>
                  </span>
                </div>
              </div>
            </li>
            <li class="pf-v5-c-dual-list-selector__list-item" role="treeitem">
              <div class="pf-v5-c-dual-list-selector__list-item-row pf-m-check">
                <div class="pf-v5-c-dual-list-selector__item" tabindex="0">
                  <span class="pf-v5-c-dual-list-selector__item-main">
                    <span class="pf-v5-c-dual-list-selector__item-check">
                      <div class="pf-v5-c-check pf-m-standalone">
                        <input
                          class="pf-v5-c-check__input"
                          type="checkbox"
                          id="check-3"
                          aria-label="Dual list selector item check"
                        />
                      </div>
                    </span>
                    <span class="pf-v5-c-dual-list-selector__item-text">Yellow</span>
                  </span>
                  <span class="pf-v5-c-dual-list-selector__item-count">
                    <span class="pf-v5-c-badge pf-m-read"></span>
                  </span>
                </div>
              </div>
            </li>
            <li
              class="pf-v5-c-dual-list-selector__list-item pf-m-expandable pf-m-expanded"
              aria-expanded="true"
              role="treeitem"
            >
              <div class="pf-v5-c-dual-list-selector__list-item-row pf-m-check">
                <div class="pf-v5-c-dual-list-selector__item" tabindex="0">
                  <span class="pf-v5-c-dual-list-selector__item-main">
                    <div class="pf-v5-c-dual-list-selector__item-toggle">
                      <span
                        class="pf-v5-c-dual-list-selector__item-toggle-icon"
                      >
                        <i class="fas fa-angle-right" aria-hidden="true"></i>
                      </span>
                    </div>
                    <span class="pf-v5-c-dual-list-selector__item-check">
                      <div class="pf-v5-c-check pf-m-standalone">
                        <input
                          class="pf-v5-c-check__input"
                          type="checkbox"
                          id="check-4"
                          aria-label="Dual list selector item check"
                        />
                      </div>
                    </span>
                    <span class="pf-v5-c-dual-list-selector__item-text">Green</span>
                  </span>
                  <span class="pf-v5-c-dual-list-selector__item-count">
                    <span class="pf-v5-c-badge pf-m-read"></span>
                  </span>
                </div>
              </div>

              <ul
                class="pf-v5-c-dual-list-selector__list"
                role="group"
                aria-labelledby="-status-text"
              >
                <li
                  class="pf-v5-c-dual-list-selector__list-item"
                  role="treeitem"
                >
                  <div
                    class="pf-v5-c-dual-list-selector__list-item-row pf-m-check"
                  >
                    <div class="pf-v5-c-dual-list-selector__item" tabindex="0">
                      <span class="pf-v5-c-dual-list-selector__item-main">
                        <span class="pf-v5-c-dual-list-selector__item-check">
                          <div class="pf-v5-c-check pf-m-standalone">
                            <input
                              class="pf-v5-c-check__input"
                              type="checkbox"
                              id="check-5"
                              aria-label="Dual list selector item check"
                            />
                          </div>
                        </span>
                        <span
                          class="pf-v5-c-dual-list-selector__item-text"
                        >Light green</span>
                      </span>
                      <span class="pf-v5-c-dual-list-selector__item-count">
                        <span class="pf-v5-c-badge pf-m-read"></span>
                      </span>
                    </div>
                  </div>
                </li>
                <li
                  class="pf-v5-c-dual-list-selector__list-item"
                  role="treeitem"
                >
                  <div
                    class="pf-v5-c-dual-list-selector__list-item-row pf-m-check"
                  >
                    <div class="pf-v5-c-dual-list-selector__item" tabindex="0">
                      <span class="pf-v5-c-dual-list-selector__item-main">
                        <span class="pf-v5-c-dual-list-selector__item-check">
                          <div class="pf-v5-c-check pf-m-standalone">
                            <input
                              class="pf-v5-c-check__input"
                              type="checkbox"
                              id="check-6"
                              aria-label="Dual list selector item check"
                            />
                          </div>
                        </span>
                        <span
                          class="pf-v5-c-dual-list-selector__item-text"
                        >Medium green</span>
                      </span>
                      <span class="pf-v5-c-dual-list-selector__item-count">
                        <span class="pf-v5-c-badge pf-m-read"></span>
                      </span>
                    </div>
                  </div>
                </li>
                <li
                  class="pf-v5-c-dual-list-selector__list-item"
                  role="treeitem"
                >
                  <div
                    class="pf-v5-c-dual-list-selector__list-item-row pf-m-check"
                  >
                    <div class="pf-v5-c-dual-list-selector__item" tabindex="0">
                      <span class="pf-v5-c-dual-list-selector__item-main">
                        <span class="pf-v5-c-dual-list-selector__item-check">
                          <div class="pf-v5-c-check pf-m-standalone">
                            <input
                              class="pf-v5-c-check__input"
                              type="checkbox"
                              id="check-7"
                              aria-label="Dual list selector item check"
                            />
                          </div>
                        </span>
                        <span
                          class="pf-v5-c-dual-list-selector__item-text"
                        >Dark green</span>
                      </span>
                      <span class="pf-v5-c-dual-list-selector__item-count">
                        <span class="pf-v5-c-badge pf-m-read"></span>
                      </span>
                    </div>
                  </div>
                </li>
              </ul>
            </li>
          </ul>
        </li>

        <li
          class="pf-v5-c-dual-list-selector__list-item pf-m-expandable"
          aria-expanded="true"
          role="treeitem"
        >
          <div class="pf-v5-c-dual-list-selector__list-item-row pf-m-check">
            <div class="pf-v5-c-dual-list-selector__item" tabindex="0">
              <span class="pf-v5-c-dual-list-selector__item-main">
                <div class="pf-v5-c-dual-list-selector__item-toggle">
                  <span class="pf-v5-c-dual-list-selector__item-toggle-icon">
                    <i class="fas fa-angle-right" aria-hidden="true"></i>
                  </span>
                </div>
                <span class="pf-v5-c-dual-list-selector__item-check">
                  <div class="pf-v5-c-check pf-m-standalone">
                    <input
                      class="pf-v5-c-check__input"
                      type="checkbox"
                      id="check-8"
                      aria-label="Dual list selector item check"
                    />
                  </div>
                </span>
                <span
                  class="pf-v5-c-dual-list-selector__item-text"
                >Type something</span>
              </span>
              <span class="pf-v5-c-dual-list-selector__item-count">
                <span class="pf-v5-c-badge pf-m-read"></span>
              </span>
            </div>
          </div>
        </li>

        <li class="pf-v5-c-dual-list-selector__list-item" role="treeitem">
          <div class="pf-v5-c-dual-list-selector__list-item-row pf-m-check">
            <div class="pf-v5-c-dual-list-selector__item" tabindex="0">
              <span class="pf-v5-c-dual-list-selector__item-main">
                <span class="pf-v5-c-dual-list-selector__item-check">
                  <div class="pf-v5-c-check pf-m-standalone">
                    <input
                      class="pf-v5-c-check__input"
                      type="checkbox"
                      id="check-9"
                      aria-label="Dual list selector item check"
                    />
                  </div>
                </span>
                <span
                  class="pf-v5-c-dual-list-selector__item-text"
                >Type something</span>
              </span>
              <span class="pf-v5-c-dual-list-selector__item-count">
                <span class="pf-v5-c-badge pf-m-read"></span>
              </span>
            </div>
          </div>
        </li>

        <li
          class="pf-v5-c-dual-list-selector__list-item pf-m-expandable"
          aria-expanded="true"
          role="treeitem"
        >
          <div class="pf-v5-c-dual-list-selector__list-item-row pf-m-check">
            <div class="pf-v5-c-dual-list-selector__item" tabindex="0">
              <span class="pf-v5-c-dual-list-selector__item-main">
                <div class="pf-v5-c-dual-list-selector__item-toggle">
                  <span class="pf-v5-c-dual-list-selector__item-toggle-icon">
                    <i class="fas fa-angle-right" aria-hidden="true"></i>
                  </span>
                </div>
                <span class="pf-v5-c-dual-list-selector__item-check">
                  <div class="pf-v5-c-check pf-m-standalone">
                    <input
                      class="pf-v5-c-check__input"
                      type="checkbox"
                      id="check-10"
                      aria-label="Dual list selector item check"
                    />
                  </div>
                </span>
                <span
                  class="pf-v5-c-dual-list-selector__item-text"
                >Type something</span>
              </span>
              <span class="pf-v5-c-dual-list-selector__item-count">
                <span class="pf-v5-c-badge pf-m-read"></span>
              </span>
            </div>
          </div>
        </li>
      </ul>
    </div>
  </div>
  <div class="pf-v5-c-dual-list-selector__controls">
    <div class="pf-v5-c-dual-list-selector__controls-item">
      <button
        class="pf-v5-c-button pf-m-plain"
        type="button"
        disabled
        aria-label="Add selected"
      >
        <i class="fas fa-fw fa-angle-right"></i>
      </button>
    </div>
    <div class="pf-v5-c-dual-list-selector__controls-item">
      <button
        class="pf-v5-c-button pf-m-plain"
        type="button"
        aria-label="Add all"
      >
        <i class="fas fa-fw fa-angle-double-right"></i>
      </button>
    </div>
    <div class="pf-v5-c-dual-list-selector__controls-item">
      <button
        class="pf-v5-c-button pf-m-plain"
        type="button"
        disabled
        aria-label="Remove all"
      >
        <i class="fas fa-fw fa-angle-double-left"></i>
      </button>
    </div>
    <div class="pf-v5-c-dual-list-selector__controls-item">
      <button
        class="pf-v5-c-button pf-m-plain"
        type="button"
        disabled
        aria-label="Remove selected"
      >
        <i class="fas fa-fw fa-angle-left"></i>
      </button>
    </div>
  </div>
  <div class="pf-v5-c-dual-list-selector__pane pf-m-chosen">
    <div class="pf-v5-c-dual-list-selector__header">
      <div class="pf-v5-c-dual-list-selector__title">
        <div class="pf-v5-c-dual-list-selector__title-text">Chosen options</div>
      </div>
    </div>
    <div class="pf-v5-c-dual-list-selector__tools">
      <div class="pf-v5-c-dual-list-selector__tools-filter">
        <div class="pf-v5-c-text-input-group">
          <div class="pf-v5-c-text-input-group__main pf-m-icon">
            <span class="pf-v5-c-text-input-group__text">
              <span class="pf-v5-c-text-input-group__icon">
                <i class="fas fa-fw fa-search"></i>
              </span>
              <input
                class="pf-v5-c-text-input-group__text-input"
                type="text"
                value
                aria-label="Search input"
              />
            </span>
          </div>
        </div>
      </div>
      <div class="pf-v5-c-dual-list-selector__tools-actions">
        <button
          class="pf-v5-c-button pf-m-plain"
          type="button"
          aria-label="Sort"
        >
          <i
            class="fas fa-sort-amount-down pf-v5-m-mirror-inline-rtl"
            aria-hidden="true"
          ></i>
        </button>
        <div class="pf-v5-c-dropdown">
          <button
            class="pf-v5-c-dropdown__toggle pf-m-plain"
            id="dropdown-kebab-tree-chosen-button"
            aria-expanded="false"
            type="button"
            aria-label="Actions"
          >
            <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
          </button>
          <ul
            class="pf-v5-c-dropdown__menu"
            aria-labelledby="dropdown-kebab-tree-chosen-button"
            hidden
            role="menu"
          >
            <li role="none">
              <a
                class="pf-v5-c-dropdown__menu-item"
                role="menuitem"
                href="#"
              >Link</a>
            </li>
            <li role="none">
              <button
                class="pf-v5-c-dropdown__menu-item"
                role="menuitem"
                type="button"
              >Action</button>
            </li>
            <li role="none">
              <a
                class="pf-v5-c-dropdown__menu-item pf-m-disabled"
                role="menuitem"
                href="#"
                aria-disabled="true"
                tabindex="-1"
              >Disabled link</a>
            </li>
            <li role="none">
              <button
                class="pf-v5-c-dropdown__menu-item"
                role="menuitem"
                type="button"
                disabled
              >Disabled action</button>
            </li>
            <li class="pf-v5-c-divider" role="separator"></li>
            <li role="none">
              <a
                class="pf-v5-c-dropdown__menu-item"
                role="menuitem"
                href="#"
              >Separated link</a>
            </li>
          </ul>
        </div>
      </div>
    </div>
    <div class="pf-v5-c-dual-list-selector__status">
      <span
        class="pf-v5-c-dual-list-selector__status-text"
        id="tree-chosen-status-text"
      >0 of 0 items selected</span>
    </div>
    <div class="pf-v5-c-dual-list-selector__menu">
      <ul
        class="pf-v5-c-dual-list-selector__list"
        role="listbox"
        aria-labelledby="tree-chosen-status-text"
        aria-multiselectable="true"
        aria-activedescendant
      ></ul>
    </div>
  </div>
</div>

```

### Tree view with chosen and disabled options

```html
<div class="pf-v5-c-dual-list-selector">
  <div class="pf-v5-c-dual-list-selector__pane pf-m-available">
    <div class="pf-v5-c-dual-list-selector__header">
      <div class="pf-v5-c-dual-list-selector__title">
        <div class="pf-v5-c-dual-list-selector__title-text">Available options</div>
      </div>
    </div>
    <div class="pf-v5-c-dual-list-selector__tools">
      <div class="pf-v5-c-dual-list-selector__tools-filter">
        <div class="pf-v5-c-text-input-group">
          <div class="pf-v5-c-text-input-group__main pf-m-icon">
            <span class="pf-v5-c-text-input-group__text">
              <span class="pf-v5-c-text-input-group__icon">
                <i class="fas fa-fw fa-search"></i>
              </span>
              <input
                class="pf-v5-c-text-input-group__text-input"
                type="text"
                value
                aria-label="Search input"
              />
            </span>
          </div>
        </div>
      </div>
      <div class="pf-v5-c-dual-list-selector__tools-actions">
        <button
          class="pf-v5-c-button pf-m-plain"
          type="button"
          aria-label="Sort"
        >
          <i
            class="fas fa-sort-amount-down pf-v5-m-mirror-inline-rtl"
            aria-hidden="true"
          ></i>
        </button>
        <div class="pf-v5-c-dropdown">
          <button
            class="pf-v5-c-dropdown__toggle pf-m-plain"
            id="dropdown-kebab-tree-options-available-button"
            aria-expanded="false"
            type="button"
            aria-label="Actions"
          >
            <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
          </button>
          <ul
            class="pf-v5-c-dropdown__menu"
            aria-labelledby="dropdown-kebab-tree-options-available-button"
            hidden
            role="menu"
          >
            <li role="none">
              <a
                class="pf-v5-c-dropdown__menu-item"
                role="menuitem"
                href="#"
              >Link</a>
            </li>
            <li role="none">
              <button
                class="pf-v5-c-dropdown__menu-item"
                role="menuitem"
                type="button"
              >Action</button>
            </li>
            <li role="none">
              <a
                class="pf-v5-c-dropdown__menu-item pf-m-disabled"
                role="menuitem"
                href="#"
                aria-disabled="true"
                tabindex="-1"
              >Disabled link</a>
            </li>
            <li role="none">
              <button
                class="pf-v5-c-dropdown__menu-item"
                role="menuitem"
                type="button"
                disabled
              >Disabled action</button>
            </li>
            <li class="pf-v5-c-divider" role="separator"></li>
            <li role="none">
              <a
                class="pf-v5-c-dropdown__menu-item"
                role="menuitem"
                href="#"
              >Separated link</a>
            </li>
          </ul>
        </div>
      </div>
    </div>
    <div class="pf-v5-c-dual-list-selector__status">
      <span
        class="pf-v5-c-dual-list-selector__status-text"
        id="tree-options-available-status-text"
      >0 of 10 items selected</span>
    </div>
    <div class="pf-v5-c-dual-list-selector__menu">
      <ul
        class="pf-v5-c-dual-list-selector__list"
        role="tree"
        aria-labelledby="tree-options-available-status-text"
        aria-multiselectable="true"
        aria-activedescendant
      >
        <li
          class="pf-v5-c-dual-list-selector__list-item pf-m-expandable pf-m-expanded"
          aria-expanded="true"
          role="treeitem"
        >
          <div class="pf-v5-c-dual-list-selector__list-item-row pf-m-check">
            <div class="pf-v5-c-dual-list-selector__item" tabindex="0">
              <span class="pf-v5-c-dual-list-selector__item-main">
                <div class="pf-v5-c-dual-list-selector__item-toggle">
                  <span class="pf-v5-c-dual-list-selector__item-toggle-icon">
                    <i class="fas fa-angle-right" aria-hidden="true"></i>
                  </span>
                </div>
                <span class="pf-v5-c-dual-list-selector__item-check">
                  <div class="pf-v5-c-check pf-m-standalone">
                    <input
                      class="pf-v5-c-check__input"
                      type="checkbox"
                      id="check-11"
                      aria-label="Dual list selector item check"
                    />
                  </div>
                </span>
                <span class="pf-v5-c-dual-list-selector__item-text">Colors</span>
              </span>
              <span class="pf-v5-c-dual-list-selector__item-count">
                <span class="pf-v5-c-badge pf-m-read">6</span>
              </span>
            </div>
          </div>

          <ul
            class="pf-v5-c-dual-list-selector__list"
            role="group"
            aria-labelledby="-status-text"
          >
            <li class="pf-v5-c-dual-list-selector__list-item" role="treeitem">
              <div
                class="pf-v5-c-dual-list-selector__list-item-row pf-m-check pf-m-selected"
              >
                <div class="pf-v5-c-dual-list-selector__item" tabindex="0">
                  <span class="pf-v5-c-dual-list-selector__item-main">
                    <span class="pf-v5-c-dual-list-selector__item-check">
                      <div class="pf-v5-c-check pf-m-standalone">
                        <input
                          class="pf-v5-c-check__input"
                          type="checkbox"
                          id="check-12"
                          checked
                          aria-label="Dual list selector item check checked"
                        />
                      </div>
                    </span>
                    <span class="pf-v5-c-dual-list-selector__item-text">Orange</span>
                  </span>
                  <span class="pf-v5-c-dual-list-selector__item-count">
                    <span class="pf-v5-c-badge pf-m-read"></span>
                  </span>
                </div>
              </div>
            </li>
            <li class="pf-v5-c-dual-list-selector__list-item" role="treeitem">
              <div class="pf-v5-c-dual-list-selector__list-item-row pf-m-check">
                <div class="pf-v5-c-dual-list-selector__item" tabindex="0">
                  <span class="pf-v5-c-dual-list-selector__item-main">
                    <span class="pf-v5-c-dual-list-selector__item-check">
                      <div class="pf-v5-c-check pf-m-standalone">
                        <input
                          class="pf-v5-c-check__input"
                          type="checkbox"
                          id="check-13"
                          aria-label="Dual list selector item check"
                        />
                      </div>
                    </span>
                    <span class="pf-v5-c-dual-list-selector__item-text">Yellow</span>
                  </span>
                  <span class="pf-v5-c-dual-list-selector__item-count">
                    <span class="pf-v5-c-badge pf-m-read"></span>
                  </span>
                </div>
              </div>
            </li>
            <li
              class="pf-v5-c-dual-list-selector__list-item pf-m-expandable pf-m-expanded pf-m-disabled"
              aria-expanded="true"
              role="treeitem"
            >
              <div class="pf-v5-c-dual-list-selector__list-item-row pf-m-check">
                <div class="pf-v5-c-dual-list-selector__item">
                  <span class="pf-v5-c-dual-list-selector__item-main">
                    <div class="pf-v5-c-dual-list-selector__item-toggle">
                      <span
                        class="pf-v5-c-dual-list-selector__item-toggle-icon"
                      >
                        <i class="fas fa-angle-right" aria-hidden="true"></i>
                      </span>
                    </div>
                    <span class="pf-v5-c-dual-list-selector__item-check">
                      <div class="pf-v5-c-check pf-m-standalone">
                        <input
                          class="pf-v5-c-check__input"
                          type="checkbox"
                          disabled
                          id="check-14"
                          aria-label="Dual list selector item check"
                        />
                      </div>
                    </span>
                    <span
                      class="pf-v5-c-dual-list-selector__item-text"
                    >Green (disabled)</span>
                  </span>
                  <span class="pf-v5-c-dual-list-selector__item-count">
                    <span class="pf-v5-c-badge pf-m-read"></span>
                  </span>
                </div>
              </div>

              <ul
                class="pf-v5-c-dual-list-selector__list"
                role="group"
                aria-labelledby="-status-text"
              >
                <li
                  class="pf-v5-c-dual-list-selector__list-item"
                  role="treeitem"
                >
                  <div
                    class="pf-v5-c-dual-list-selector__list-item-row pf-m-check"
                  >
                    <div class="pf-v5-c-dual-list-selector__item">
                      <span class="pf-v5-c-dual-list-selector__item-main">
                        <span class="pf-v5-c-dual-list-selector__item-check">
                          <div class="pf-v5-c-check pf-m-standalone">
                            <input
                              class="pf-v5-c-check__input"
                              type="checkbox"
                              disabled
                              id="check-15"
                              aria-label="Dual list selector item check"
                            />
                          </div>
                        </span>
                        <span
                          class="pf-v5-c-dual-list-selector__item-text"
                        >Light green</span>
                      </span>
                      <span class="pf-v5-c-dual-list-selector__item-count">
                        <span class="pf-v5-c-badge pf-m-read"></span>
                      </span>
                    </div>
                  </div>
                </li>
                <li
                  class="pf-v5-c-dual-list-selector__list-item"
                  role="treeitem"
                >
                  <div
                    class="pf-v5-c-dual-list-selector__list-item-row pf-m-check"
                  >
                    <div class="pf-v5-c-dual-list-selector__item">
                      <span class="pf-v5-c-dual-list-selector__item-main">
                        <span class="pf-v5-c-dual-list-selector__item-check">
                          <div class="pf-v5-c-check pf-m-standalone">
                            <input
                              class="pf-v5-c-check__input"
                              type="checkbox"
                              disabled
                              id="check-16"
                              aria-label="Dual list selector item check"
                            />
                          </div>
                        </span>
                        <span
                          class="pf-v5-c-dual-list-selector__item-text"
                        >Medium green</span>
                      </span>
                      <span class="pf-v5-c-dual-list-selector__item-count">
                        <span class="pf-v5-c-badge pf-m-read"></span>
                      </span>
                    </div>
                  </div>
                </li>
                <li
                  class="pf-v5-c-dual-list-selector__list-item"
                  role="treeitem"
                >
                  <div
                    class="pf-v5-c-dual-list-selector__list-item-row pf-m-check"
                  >
                    <div class="pf-v5-c-dual-list-selector__item">
                      <span class="pf-v5-c-dual-list-selector__item-main">
                        <span class="pf-v5-c-dual-list-selector__item-check">
                          <div class="pf-v5-c-check pf-m-standalone">
                            <input
                              class="pf-v5-c-check__input"
                              type="checkbox"
                              disabled
                              id="check-17"
                              aria-label="Dual list selector item check"
                            />
                          </div>
                        </span>
                        <span
                          class="pf-v5-c-dual-list-selector__item-text"
                        >Dark green</span>
                      </span>
                      <span class="pf-v5-c-dual-list-selector__item-count">
                        <span class="pf-v5-c-badge pf-m-read"></span>
                      </span>
                    </div>
                  </div>
                </li>
              </ul>
            </li>
          </ul>
        </li>
        <li
          class="pf-v5-c-dual-list-selector__list-item pf-m-expandable"
          aria-expanded="true"
          role="treeitem"
        >
          <div class="pf-v5-c-dual-list-selector__list-item-row pf-m-check">
            <div class="pf-v5-c-dual-list-selector__item" tabindex="0">
              <span class="pf-v5-c-dual-list-selector__item-main">
                <div class="pf-v5-c-dual-list-selector__item-toggle">
                  <span class="pf-v5-c-dual-list-selector__item-toggle-icon">
                    <i class="fas fa-angle-right" aria-hidden="true"></i>
                  </span>
                </div>
                <span class="pf-v5-c-dual-list-selector__item-check">
                  <div class="pf-v5-c-check pf-m-standalone">
                    <input
                      class="pf-v5-c-check__input"
                      type="checkbox"
                      id="check-18"
                      aria-label="Dual list selector item check"
                    />
                  </div>
                </span>
                <span
                  class="pf-v5-c-dual-list-selector__item-text"
                >Type something</span>
              </span>
              <span class="pf-v5-c-dual-list-selector__item-count">
                <span class="pf-v5-c-badge pf-m-read"></span>
              </span>
            </div>
          </div>
        </li>
        <li class="pf-v5-c-dual-list-selector__list-item" role="treeitem">
          <div class="pf-v5-c-dual-list-selector__list-item-row pf-m-check">
            <div class="pf-v5-c-dual-list-selector__item" tabindex="0">
              <span class="pf-v5-c-dual-list-selector__item-main">
                <span class="pf-v5-c-dual-list-selector__item-check">
                  <div class="pf-v5-c-check pf-m-standalone">
                    <input
                      class="pf-v5-c-check__input"
                      type="checkbox"
                      id="check-19"
                      aria-label="Dual list selector item check"
                    />
                  </div>
                </span>
                <span
                  class="pf-v5-c-dual-list-selector__item-text"
                >Type something</span>
              </span>
              <span class="pf-v5-c-dual-list-selector__item-count">
                <span class="pf-v5-c-badge pf-m-read"></span>
              </span>
            </div>
          </div>
        </li>
        <li
          class="pf-v5-c-dual-list-selector__list-item pf-m-expandable"
          aria-expanded="true"
          role="treeitem"
        >
          <div class="pf-v5-c-dual-list-selector__list-item-row pf-m-check">
            <div class="pf-v5-c-dual-list-selector__item" tabindex="0">
              <span class="pf-v5-c-dual-list-selector__item-main">
                <div class="pf-v5-c-dual-list-selector__item-toggle">
                  <span class="pf-v5-c-dual-list-selector__item-toggle-icon">
                    <i class="fas fa-angle-right" aria-hidden="true"></i>
                  </span>
                </div>
                <span class="pf-v5-c-dual-list-selector__item-check">
                  <div class="pf-v5-c-check pf-m-standalone">
                    <input
                      class="pf-v5-c-check__input"
                      type="checkbox"
                      id="check-20"
                      aria-label="Dual list selector item check"
                    />
                  </div>
                </span>
                <span
                  class="pf-v5-c-dual-list-selector__item-text"
                >Type something</span>
              </span>
              <span class="pf-v5-c-dual-list-selector__item-count">
                <span class="pf-v5-c-badge pf-m-read"></span>
              </span>
            </div>
          </div>
        </li>
      </ul>
    </div>
  </div>
  <div class="pf-v5-c-dual-list-selector__controls">
    <div class="pf-v5-c-dual-list-selector__controls-item">
      <button
        class="pf-v5-c-button pf-m-plain"
        type="button"
        disabled
        aria-label="Add selected"
      >
        <i class="fas fa-fw fa-angle-right"></i>
      </button>
    </div>
    <div class="pf-v5-c-dual-list-selector__controls-item">
      <button
        class="pf-v5-c-button pf-m-plain"
        type="button"
        aria-label="Add all"
      >
        <i class="fas fa-fw fa-angle-double-right"></i>
      </button>
    </div>
    <div class="pf-v5-c-dual-list-selector__controls-item">
      <button
        class="pf-v5-c-button pf-m-plain"
        type="button"
        disabled
        aria-label="Remove all"
      >
        <i class="fas fa-fw fa-angle-double-left"></i>
      </button>
    </div>
    <div class="pf-v5-c-dual-list-selector__controls-item">
      <button
        class="pf-v5-c-button pf-m-plain"
        type="button"
        disabled
        aria-label="Remove selected"
      >
        <i class="fas fa-fw fa-angle-left"></i>
      </button>
    </div>
  </div>
  <div class="pf-v5-c-dual-list-selector__pane pf-m-chosen">
    <div class="pf-v5-c-dual-list-selector__header">
      <div class="pf-v5-c-dual-list-selector__title">
        <div class="pf-v5-c-dual-list-selector__title-text">Chosen options</div>
      </div>
    </div>
    <div class="pf-v5-c-dual-list-selector__tools">
      <div class="pf-v5-c-dual-list-selector__tools-filter">
        <div class="pf-v5-c-text-input-group">
          <div class="pf-v5-c-text-input-group__main pf-m-icon">
            <span class="pf-v5-c-text-input-group__text">
              <span class="pf-v5-c-text-input-group__icon">
                <i class="fas fa-fw fa-search"></i>
              </span>
              <input
                class="pf-v5-c-text-input-group__text-input"
                type="text"
                value
                aria-label="Search input"
              />
            </span>
          </div>
        </div>
      </div>
      <div class="pf-v5-c-dual-list-selector__tools-actions">
        <button
          class="pf-v5-c-button pf-m-plain"
          type="button"
          aria-label="Sort"
        >
          <i
            class="fas fa-sort-amount-down pf-v5-m-mirror-inline-rtl"
            aria-hidden="true"
          ></i>
        </button>
        <div class="pf-v5-c-dropdown">
          <button
            class="pf-v5-c-dropdown__toggle pf-m-plain"
            id="dropdown-kebab-tree-options-chosen-button"
            aria-expanded="false"
            type="button"
            aria-label="Actions"
          >
            <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
          </button>
          <ul
            class="pf-v5-c-dropdown__menu"
            aria-labelledby="dropdown-kebab-tree-options-chosen-button"
            hidden
            role="menu"
          >
            <li role="none">
              <a
                class="pf-v5-c-dropdown__menu-item"
                role="menuitem"
                href="#"
              >Link</a>
            </li>
            <li role="none">
              <button
                class="pf-v5-c-dropdown__menu-item"
                role="menuitem"
                type="button"
              >Action</button>
            </li>
            <li role="none">
              <a
                class="pf-v5-c-dropdown__menu-item pf-m-disabled"
                role="menuitem"
                href="#"
                aria-disabled="true"
                tabindex="-1"
              >Disabled link</a>
            </li>
            <li role="none">
              <button
                class="pf-v5-c-dropdown__menu-item"
                role="menuitem"
                type="button"
                disabled
              >Disabled action</button>
            </li>
            <li class="pf-v5-c-divider" role="separator"></li>
            <li role="none">
              <a
                class="pf-v5-c-dropdown__menu-item"
                role="menuitem"
                href="#"
              >Separated link</a>
            </li>
          </ul>
        </div>
      </div>
    </div>
    <div class="pf-v5-c-dual-list-selector__status">
      <span
        class="pf-v5-c-dual-list-selector__status-text"
        id="tree-options-chosen-status-text"
      >0 of 0 items selected</span>
    </div>
    <div class="pf-v5-c-dual-list-selector__menu">
      <ul
        class="pf-v5-c-dual-list-selector__list"
        role="tree"
        aria-labelledby="tree-options-chosen-status-text"
        aria-multiselectable="true"
        aria-activedescendant
      >
        <li
          class="pf-v5-c-dual-list-selector__list-item pf-m-expandable pf-m-expanded"
          aria-expanded="true"
          role="treeitem"
        >
          <div class="pf-v5-c-dual-list-selector__list-item-row pf-m-check">
            <div class="pf-v5-c-dual-list-selector__item" tabindex="0">
              <span class="pf-v5-c-dual-list-selector__item-main">
                <div class="pf-v5-c-dual-list-selector__item-toggle">
                  <span class="pf-v5-c-dual-list-selector__item-toggle-icon">
                    <i class="fas fa-angle-right" aria-hidden="true"></i>
                  </span>
                </div>
                <span class="pf-v5-c-dual-list-selector__item-check">
                  <div class="pf-v5-c-check pf-m-standalone">
                    <input
                      class="pf-v5-c-check__input"
                      type="checkbox"
                      id="check-21"
                      aria-label="Dual list selector item check"
                    />
                  </div>
                </span>
                <span class="pf-v5-c-dual-list-selector__item-text">Colors</span>
              </span>
              <span class="pf-v5-c-dual-list-selector__item-count">
                <span class="pf-v5-c-badge pf-m-read"></span>
              </span>
            </div>
          </div>

          <ul
            class="pf-v5-c-dual-list-selector__list"
            role="group"
            aria-labelledby="-status-text"
          >
            <li class="pf-v5-c-dual-list-selector__list-item" role="treeitem">
              <div
                class="pf-v5-c-dual-list-selector__list-item-row pf-m-check pf-m-selected"
              >
                <div class="pf-v5-c-dual-list-selector__item" tabindex="0">
                  <span class="pf-v5-c-dual-list-selector__item-main">
                    <span class="pf-v5-c-dual-list-selector__item-check">
                      <div class="pf-v5-c-check pf-m-standalone">
                        <input
                          class="pf-v5-c-check__input"
                          type="checkbox"
                          id="check-22"
                          checked
                          aria-label="Dual list selector item check checked"
                        />
                      </div>
                    </span>
                    <span class="pf-v5-c-dual-list-selector__item-text">Orange</span>
                  </span>
                  <span class="pf-v5-c-dual-list-selector__item-count">
                    <span class="pf-v5-c-badge pf-m-read"></span>
                  </span>
                </div>
              </div>
            </li>
          </ul>
        </li>
      </ul>
    </div>
  </div>
</div>

```

### Draggable

```html
<div
  id="draggable-help"
>Activate the reorder button and use the arrow keys to reorder the list or use your mouse to drag/reorder. Press escape to cancel the reordering.</div>
<div class="pf-v5-c-dual-list-selector">
  <div class="pf-v5-c-dual-list-selector__pane pf-m-available">
    <div class="pf-v5-c-dual-list-selector__header">
      <div class="pf-v5-c-dual-list-selector__title">
        <div class="pf-v5-c-dual-list-selector__title-text">Available options</div>
      </div>
    </div>
    <div class="pf-v5-c-dual-list-selector__tools">
      <div class="pf-v5-c-dual-list-selector__tools-filter">
        <div class="pf-v5-c-text-input-group">
          <div class="pf-v5-c-text-input-group__main pf-m-icon">
            <span class="pf-v5-c-text-input-group__text">
              <span class="pf-v5-c-text-input-group__icon">
                <i class="fas fa-fw fa-search"></i>
              </span>
              <input
                class="pf-v5-c-text-input-group__text-input"
                type="text"
                value
                aria-label="Search input"
              />
            </span>
          </div>
        </div>
      </div>
      <div class="pf-v5-c-dual-list-selector__tools-actions">
        <button
          class="pf-v5-c-button pf-m-plain"
          type="button"
          aria-label="Sort"
        >
          <i
            class="fas fa-sort-amount-down pf-v5-m-mirror-inline-rtl"
            aria-hidden="true"
          ></i>
        </button>
        <div class="pf-v5-c-dropdown">
          <button
            class="pf-v5-c-dropdown__toggle pf-m-plain"
            id="dropdown-kebab-draggable-available-button"
            aria-expanded="false"
            type="button"
            aria-label="Actions"
          >
            <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
          </button>
          <ul
            class="pf-v5-c-dropdown__menu"
            aria-labelledby="dropdown-kebab-draggable-available-button"
            hidden
            role="menu"
          >
            <li role="none">
              <a
                class="pf-v5-c-dropdown__menu-item"
                role="menuitem"
                href="#"
              >Link</a>
            </li>
            <li role="none">
              <button
                class="pf-v5-c-dropdown__menu-item"
                role="menuitem"
                type="button"
              >Action</button>
            </li>
            <li role="none">
              <a
                class="pf-v5-c-dropdown__menu-item pf-m-disabled"
                role="menuitem"
                href="#"
                aria-disabled="true"
                tabindex="-1"
              >Disabled link</a>
            </li>
            <li role="none">
              <button
                class="pf-v5-c-dropdown__menu-item"
                role="menuitem"
                type="button"
                disabled
              >Disabled action</button>
            </li>
            <li class="pf-v5-c-divider" role="separator"></li>
            <li role="none">
              <a
                class="pf-v5-c-dropdown__menu-item"
                role="menuitem"
                href="#"
              >Separated link</a>
            </li>
          </ul>
        </div>
      </div>
    </div>
    <div class="pf-v5-c-dual-list-selector__status">
      <span
        class="pf-v5-c-dual-list-selector__status-text"
        id="draggable-available-status-text"
      >0 of 5 items selected</span>
    </div>
    <div class="pf-v5-c-dual-list-selector__menu">
      <ul
        class="pf-v5-c-dual-list-selector__list"
        role="listbox"
        aria-labelledby="draggable-available-status-text"
        aria-multiselectable="true"
        aria-activedescendant
      >
        <li class="pf-v5-c-dual-list-selector__list-item" role="option">
          <div class="pf-v5-c-dual-list-selector__list-item-row">
            <span class="pf-v5-c-dual-list-selector__item">
              <span class="pf-v5-c-dual-list-selector__item-main">
                <span class="pf-v5-c-dual-list-selector__item-text">Item1</span>
              </span>
              <span class="pf-v5-c-dual-list-selector__item-count">
                <span class="pf-v5-c-badge pf-m-read"></span>
              </span>
            </span>
          </div>
        </li>
        <li class="pf-v5-c-dual-list-selector__list-item" role="option">
          <div class="pf-v5-c-dual-list-selector__list-item-row">
            <span class="pf-v5-c-dual-list-selector__item">
              <span class="pf-v5-c-dual-list-selector__item-main">
                <span class="pf-v5-c-dual-list-selector__item-text">Item4</span>
              </span>
              <span class="pf-v5-c-dual-list-selector__item-count">
                <span class="pf-v5-c-badge pf-m-read"></span>
              </span>
            </span>
          </div>
        </li>
        <li class="pf-v5-c-dual-list-selector__list-item" role="option">
          <div class="pf-v5-c-dual-list-selector__list-item-row">
            <span class="pf-v5-c-dual-list-selector__item">
              <span class="pf-v5-c-dual-list-selector__item-main">
                <span class="pf-v5-c-dual-list-selector__item-text">Item6</span>
              </span>
              <span class="pf-v5-c-dual-list-selector__item-count">
                <span class="pf-v5-c-badge pf-m-read"></span>
              </span>
            </span>
          </div>
        </li>
      </ul>
    </div>
  </div>
  <div class="pf-v5-c-dual-list-selector__controls">
    <div class="pf-v5-c-dual-list-selector__controls-item">
      <button
        class="pf-v5-c-button pf-m-plain"
        type="button"
        disabled
        aria-label="Add selected"
      >
        <i class="fas fa-fw fa-angle-right"></i>
      </button>
    </div>
    <div class="pf-v5-c-dual-list-selector__controls-item">
      <button
        class="pf-v5-c-button pf-m-plain"
        type="button"
        aria-label="Add all"
      >
        <i class="fas fa-fw fa-angle-double-right"></i>
      </button>
    </div>
    <div class="pf-v5-c-dual-list-selector__controls-item">
      <button
        class="pf-v5-c-button pf-m-plain"
        type="button"
        disabled
        aria-label="Remove all"
      >
        <i class="fas fa-fw fa-angle-double-left"></i>
      </button>
    </div>
    <div class="pf-v5-c-dual-list-selector__controls-item">
      <button
        class="pf-v5-c-button pf-m-plain"
        type="button"
        disabled
        aria-label="Remove selected"
      >
        <i class="fas fa-fw fa-angle-left"></i>
      </button>
    </div>
  </div>
  <div class="pf-v5-c-dual-list-selector__pane pf-m-chosen">
    <div class="pf-v5-c-dual-list-selector__header">
      <div class="pf-v5-c-dual-list-selector__title">
        <div class="pf-v5-c-dual-list-selector__title-text">Chosen options</div>
      </div>
    </div>
    <div class="pf-v5-c-dual-list-selector__tools">
      <div class="pf-v5-c-dual-list-selector__tools-filter">
        <div class="pf-v5-c-text-input-group">
          <div class="pf-v5-c-text-input-group__main pf-m-icon">
            <span class="pf-v5-c-text-input-group__text">
              <span class="pf-v5-c-text-input-group__icon">
                <i class="fas fa-fw fa-search"></i>
              </span>
              <input
                class="pf-v5-c-text-input-group__text-input"
                type="text"
                value
                aria-label="Search input"
              />
            </span>
          </div>
        </div>
      </div>
      <div class="pf-v5-c-dual-list-selector__tools-actions">
        <button
          class="pf-v5-c-button pf-m-plain"
          type="button"
          aria-label="Sort"
        >
          <i
            class="fas fa-sort-amount-down pf-v5-m-mirror-inline-rtl"
            aria-hidden="true"
          ></i>
        </button>
        <div class="pf-v5-c-dropdown">
          <button
            class="pf-v5-c-dropdown__toggle pf-m-plain"
            id="dropdown-kebab-draggable-chosen-button"
            aria-expanded="false"
            type="button"
            aria-label="Actions"
          >
            <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
          </button>
          <ul
            class="pf-v5-c-dropdown__menu"
            aria-labelledby="dropdown-kebab-draggable-chosen-button"
            hidden
            role="menu"
          >
            <li role="none">
              <a
                class="pf-v5-c-dropdown__menu-item"
                role="menuitem"
                href="#"
              >Link</a>
            </li>
            <li role="none">
              <button
                class="pf-v5-c-dropdown__menu-item"
                role="menuitem"
                type="button"
              >Action</button>
            </li>
            <li role="none">
              <a
                class="pf-v5-c-dropdown__menu-item pf-m-disabled"
                role="menuitem"
                href="#"
                aria-disabled="true"
                tabindex="-1"
              >Disabled link</a>
            </li>
            <li role="none">
              <button
                class="pf-v5-c-dropdown__menu-item"
                role="menuitem"
                type="button"
                disabled
              >Disabled action</button>
            </li>
            <li class="pf-v5-c-divider" role="separator"></li>
            <li role="none">
              <a
                class="pf-v5-c-dropdown__menu-item"
                role="menuitem"
                href="#"
              >Separated link</a>
            </li>
          </ul>
        </div>
      </div>
    </div>
    <div class="pf-v5-c-dual-list-selector__status">
      <span
        class="pf-v5-c-dual-list-selector__status-text"
        id="draggable-chosen-status-text"
      >0 of 0 items selected</span>
    </div>
    <div class="pf-v5-c-dual-list-selector__menu">
      <ul
        class="pf-v5-c-dual-list-selector__list"
        role="listbox"
        aria-labelledby="draggable-chosen-status-text"
        aria-multiselectable="true"
        aria-activedescendant
      >
        <li class="pf-v5-c-dual-list-selector__list-item" role="option">
          <div class="pf-v5-c-dual-list-selector__list-item-row">
            <div class="pf-v5-c-dual-list-selector__draggable">
              <span class="pf-v5-c-button pf-m-disabled pf-m-plain">
                <i class="fas fa-grip-vertical" aria-hidden="true"></i>
              </span>
            </div>
            <span class="pf-v5-c-dual-list-selector__item">
              <span class="pf-v5-c-dual-list-selector__item-main">
                <span
                  class="pf-v5-c-dual-list-selector__item-text"
                  id="draggable-list-item-2-item-text"
                >Item2 - draggable icon disabled</span>
              </span>
              <span class="pf-v5-c-dual-list-selector__item-count">
                <span class="pf-v5-c-badge pf-m-read"></span>
              </span>
            </span>
          </div>
        </li>
        <li class="pf-v5-c-dual-list-selector__list-item" role="option">
          <div class="pf-v5-c-dual-list-selector__list-item-row">
            <div class="pf-v5-c-dual-list-selector__draggable">
              <span class="pf-v5-c-button pf-m-plain">
                <i class="fas fa-grip-vertical" aria-hidden="true"></i>
              </span>
            </div>
            <span class="pf-v5-c-dual-list-selector__item">
              <span class="pf-v5-c-dual-list-selector__item-main">
                <span
                  class="pf-v5-c-dual-list-selector__item-text"
                  id="draggable-list-item-3-item-text"
                >Item3</span>
              </span>
              <span class="pf-v5-c-dual-list-selector__item-count">
                <span class="pf-v5-c-badge pf-m-read"></span>
              </span>
            </span>
          </div>
        </li>
        <li class="pf-v5-c-dual-list-selector__list-item" role="option">
          <div class="pf-v5-c-dual-list-selector__list-item-row pf-m-ghost-row">
            <div class="pf-v5-c-dual-list-selector__draggable">
              <span class="pf-v5-c-button pf-m-disabled pf-m-plain">
                <i class="fas fa-grip-vertical" aria-hidden="true"></i>
              </span>
            </div>
            <span class="pf-v5-c-dual-list-selector__item">
              <span class="pf-v5-c-dual-list-selector__item-main">
                <span
                  class="pf-v5-c-dual-list-selector__item-text"
                  id="draggable-list-item-5-item-text"
                >Item5 - ghost row</span>
              </span>
              <span class="pf-v5-c-dual-list-selector__item-count">
                <span class="pf-v5-c-badge pf-m-read"></span>
              </span>
            </span>
          </div>
        </li>
        <li class="pf-v5-c-dual-list-selector__list-item" role="option">
          <div class="pf-v5-c-dual-list-selector__list-item-row pf-m-selected">
            <div class="pf-v5-c-dual-list-selector__draggable">
              <span class="pf-v5-c-button pf-m-plain">
                <i class="fas fa-grip-vertical" aria-hidden="true"></i>
              </span>
            </div>
            <span class="pf-v5-c-dual-list-selector__item">
              <span class="pf-v5-c-dual-list-selector__item-main">
                <span
                  class="pf-v5-c-dual-list-selector__item-text"
                  id="draggable-list-item-7-item-text"
                >Item7 - selected</span>
              </span>
              <span class="pf-v5-c-dual-list-selector__item-count">
                <span class="pf-v5-c-badge pf-m-read"></span>
              </span>
            </span>
          </div>
        </li>
      </ul>
    </div>
  </div>
</div>
<div
  class="pf-v5-screen-reader"
  aria-live="assertive"
>This is the aria-live section that provides real-time feedback to the user.</div>

```

## Documentation

### Accessibility

| Attribute | Applied to | Outcome |
| -- | -- | -- |
| `aria-live` | `[element with live text]` | To give screen reader users live feedback about what's happening during interaction with the dual list selector, both during drag and drop interactions and keyboard interactions. **Highly Recommended** |
| `aria-labelledby="[id of .pf-v5-c-dual-list-selector__status-text]` | `.pf-v5-c-dual-list-selector__list [ul]` | Gives the list an accessible name. |
| `role="listbox or tree or group"` | `.pf-v5-c-dual-list-selector__list [ul]` | Indicates the list is single, a tree, or a subgroup within the tree. |
| `aria-multiselectable="true"` | `.pf-v5-c-dual-list-selector__list [ul]` | Indicates the list is multiselectable. |
| `aria-activedescendant=""` | `.pf-v5-c-dual-list-selector__list [ul]` | Indicates the list has clickable children. |
| `role="option or treeitem"` | `.pf-v5-c-dual-list-selector__list-item [li]` | Indicates whether the item is part of a tree. |
| `aria-expanded="true"`      | `.pf-v5-c-dual-list-selector__list-item [li]` | Indicates a treeitem is expanded. |

### Usage

| Class | Applied | Outcome |
| -- | -- | -- |
| `.pf-v5-c-dual-list-selector` | `<div>` | Initiates the dual list selector component. **Required** |
| `.pf-v5-c-dual-list-selector__pane` | `<div>` | Initiates a dual list selector pane. **Required** |
| `.pf-v5-c-dual-list-selector__header` | `<div>` | Initiates a dual list selector pane header. **Required** |
| `.pf-v5-c-dual-list-selector__title` | `<div>` | Initiates a dual list selector pane title. **Required** |
| `.pf-v5-c-dual-list-selector__title-text` | `<div>` | Initiates a dual list selector pane title text. **Required** |
| `.pf-v5-c-dual-list-selector__tools` | `<div>` | Initiates a dual list selector tools. **Required** |
| `.pf-v5-c-dual-list-selector__filter` | `<div>` | Initiates a dual list selector pane filter. **Required** |
| `.pf-v5-c-dual-list-selector__actions` | `<div>` | Initiates a dual list selector pane actions. |
| `.pf-v5-c-dual-list-selector__actions-item` | `<div>` | Initiates a dual list selector pane actions item. |
| `.pf-v5-c-dual-list-selector__status` | `<div>` | Initiates a dual list selector pane selected status. **Required** |
| `.pf-v5-c-dual-list-selector__status-text` | `<span>` | Initiates a dual list selector pane selected status text. **Required** |
| `.pf-v5-c-dual-list-selector__menu` | `<div>` | Initiates a dual list selector pane menu container. **Required** |
| `.pf-v5-c-dual-list-selector__list` | `<ul>` | Initiates a dual list selector pane menu list. **Required** |
| `.pf-v5-c-dual-list-selector__list-item` | `<li>` | Initiates a dual list selector pane menu list item. **Required** |
| `.pf-v5-c-dual-list-selector__list-item-row` | `<div>` | Initiates a dual list selector pane menu list item row. **Required** |
| `.pf-v5-c-dual-list-selector__draggable` | `<div>` | Initiates a dual list selector pane draggable element. |
| `.pf-v5-c-dual-list-selector__item` | `<span>`, `<div>` | Initiates a dual list selector pane menu item. **Required** |
| `.pf-v5-c-dual-list-selector__item-main` | `<span>` | Initiates a dual list selector pane menu item main container. **Required** |
| `.pf-v5-c-dual-list-selector__item-check` | `<span>` | Initiates the dual list selector item check. |
| `.pf-v5-c-dual-list-selector__item-count` | `<span>` | Initiates the dual list selector item count. |
| `.pf-v5-c-dual-list-selector__item-toggle-icon` | `<span>` | Initiates the dual list selector item toggle icon. |
| `.pf-v5-c-dual-list-selector__item-toggle` | `<button>` | Initiates the dual list selector item toggle. |
| `.pf-v5-c-dual-list-selector__item-text` | `<span>` | Initiates a dual list selector pane menu item text. **Required** |
| `.pf-v5-c-dual-list-selector__controls` | `<div>` | Initiates the dual list selector controls. **Required** |
| `.pf-v5-c-dual-list-selector__controls-item` | `<div>` | Initiates the dual list selector controls item. **Required** |
| `.pf-m-available` | `.pf-v5-c-dual-list-selector__pane` | Defines a pane as the available list. |
| `.pf-m-chosen` | `.pf-v5-c-dual-list-selector__pane` | Defines a pane as the chosen list. |
| `.pf-m-drag-over` | `.pf-v5-c-dual-list-selector__list` | Modifies the dual list selector list to indicate that a draggable item is being dragged over the list. |
| `.pf-m-ghost-row` | `.pf-v5-c-dual-list-selector__list-item-row` | Modifies the list item main to be a ghost row. |
| `.pf-m-selected` | `.pf-v5-c-dual-list-selector__list-item-row` | Modifies the menu item for the selected state. |
| `.pf-m-check` | `.pf-v5-c-dual-list-selector__list-item-row` | Indicates that an item is selectable with a checkbox. |
| `.pf-m-expandable` | `.pf-v5-c-dual-list-selector__list-item` | Indicates that an item is expandable. |
| `.pf-m-expanded` | `.pf-v5-c-dual-list-selector__list-item` | Indicates that an item is expanded. |
| `.pf-m-disabled` | `.pf-v5-c-dual-list-selector__list-item` | Indicates that an item is disabled. **Note:** If an item is expandable, only the top level item needs the disabled class. |
