---
id: Chip
section: components
cssPrefix: ['pf-v5-c-chip', 'pf-v5-c-chip-group']
---## Examples

### Chip variants

```html
<div class="pf-v5-c-chip">
  <span class="pf-v5-c-chip__content">
    <span class="pf-v5-c-chip__text" id="chip_one">Chip</span>
  </span>
  <span class="pf-v5-c-chip__actions">
    <button
      class="pf-v5-c-button pf-m-plain"
      type="button"
      aria-labelledby="remove_chip_one chip_one"
      aria-label="Remove"
      id="remove_chip_one"
    >
      <i class="fas fa-times" aria-hidden="true"></i>
    </button>
  </span>
</div>
<br />
<br />
<div class="pf-v5-c-chip">
  <span class="pf-v5-c-chip__content">
    <span
      class="pf-v5-c-chip__text"
      id="chip_two"
    >Really long chip that goes on and on</span>
  </span>
  <span class="pf-v5-c-chip__actions">
    <button
      class="pf-v5-c-button pf-m-plain"
      type="button"
      aria-labelledby="remove_chip_two chip_two"
      aria-label="Remove"
      id="remove_chip_two"
    >
      <i class="fas fa-times" aria-hidden="true"></i>
    </button>
  </span>
</div>
<br />
<br />
<div class="pf-v5-c-chip">
  <span class="pf-v5-c-chip__content">
    <span class="pf-v5-c-chip__text" id="chip_three">Chip</span>
    <span class="pf-v5-c-badge pf-m-read">00</span>
  </span>
  <span class="pf-v5-c-chip__actions">
    <button
      class="pf-v5-c-button pf-m-plain"
      type="button"
      aria-labelledby="remove_chip_three chip_three"
      aria-label="Remove"
      id="remove_chip_three"
    >
      <i class="fas fa-times" aria-hidden="true"></i>
    </button>
  </span>
</div>
<br />
<br />
<div class="pf-v5-c-chip">
  <span class="pf-v5-c-chip__content">
    <span class="pf-v5-c-chip__text">Read-only chip</span>
  </span>
</div>
<br />
<br />
<button class="pf-v5-c-chip pf-m-overflow">
  <span class="pf-v5-c-chip__content">
    <span class="pf-v5-c-chip__text">Overflow chip</span>
  </span>
</button>
<br />
<br />
<div class="pf-v5-c-chip pf-m-draggable">
  <span class="pf-v5-c-chip__content">
    <span class="pf-v5-c-chip__icon">
      <i class="fas fa-grip-vertical" role="img" aria-label="Drag"></i>
    </span>
    <span class="pf-v5-c-chip__text">Draggable chip</span>
  </span>
</div>

```

### Simple inline chip group with overflow

```html
<div
  class="pf-v5-c-chip-group"
  role="group"
  aria-label="simple inline chip group with overflow"
>
  <div class="pf-v5-c-chip-group__main">
    <ul
      class="pf-v5-c-chip-group__list"
      role="list"
      aria-label="Chip group list"
    >
      <li class="pf-v5-c-chip-group__list-item">
        <div class="pf-v5-c-chip">
          <span class="pf-v5-c-chip__content">
            <span
              class="pf-v5-c-chip__text"
              id="simple-inline-chip-group-overflowchip_one_select_collapsed"
            >Chip one</span>
          </span>
          <span class="pf-v5-c-chip__actions">
            <button
              class="pf-v5-c-button pf-m-plain"
              type="button"
              aria-labelledby="simple-inline-chip-group-overflowremove_chip_one_select_collapsed simple-inline-chip-group-overflowchip_one_select_collapsed"
              aria-label="Remove"
              id="simple-inline-chip-group-overflowremove_chip_one_select_collapsed"
            >
              <i class="fas fa-times" aria-hidden="true"></i>
            </button>
          </span>
        </div>
      </li>
      <li class="pf-v5-c-chip-group__list-item">
        <div class="pf-v5-c-chip">
          <span class="pf-v5-c-chip__content">
            <span
              class="pf-v5-c-chip__text"
              id="simple-inline-chip-group-overflowchip_two_select_collapsed"
            >Chip two</span>
          </span>
          <span class="pf-v5-c-chip__actions">
            <button
              class="pf-v5-c-button pf-m-plain"
              type="button"
              aria-labelledby="simple-inline-chip-group-overflowremove_chip_two_select_collapsed simple-inline-chip-group-overflowchip_two_select_collapsed"
              aria-label="Remove"
              id="simple-inline-chip-group-overflowremove_chip_two_select_collapsed"
            >
              <i class="fas fa-times" aria-hidden="true"></i>
            </button>
          </span>
        </div>
      </li>
      <li class="pf-v5-c-chip-group__list-item">
        <div class="pf-v5-c-chip">
          <span class="pf-v5-c-chip__content">
            <span
              class="pf-v5-c-chip__text"
              id="simple-inline-chip-group-overflowchip_three_select_collapsed"
            >Chip three</span>
          </span>
          <span class="pf-v5-c-chip__actions">
            <button
              class="pf-v5-c-button pf-m-plain"
              type="button"
              aria-labelledby="simple-inline-chip-group-overflowremove_chip_three_select_collapsed simple-inline-chip-group-overflowchip_three_select_collapsed"
              aria-label="Remove"
              id="simple-inline-chip-group-overflowremove_chip_three_select_collapsed"
            >
              <i class="fas fa-times" aria-hidden="true"></i>
            </button>
          </span>
        </div>
      </li>
      <li class="pf-v5-c-chip-group__list-item">
        <button class="pf-v5-c-chip pf-m-overflow">
          <span class="pf-v5-c-chip__content">
            <span class="pf-v5-c-chip__text">2 more</span>
          </span>
        </button>
      </li>
    </ul>
  </div>
</div>

```

### Simple inline chip group expanded

```html
<div
  class="pf-v5-c-chip-group"
  role="group"
  aria-label="simple inline chip group expanded"
>
  <div class="pf-v5-c-chip-group__main">
    <ul
      class="pf-v5-c-chip-group__list"
      role="list"
      aria-label="Chip group list"
    >
      <li class="pf-v5-c-chip-group__list-item">
        <div class="pf-v5-c-chip">
          <span class="pf-v5-c-chip__content">
            <span
              class="pf-v5-c-chip__text"
              id="simple-inline-chip-group-expandedchip_one_select"
            >Chip one</span>
          </span>
          <span class="pf-v5-c-chip__actions">
            <button
              class="pf-v5-c-button pf-m-plain"
              type="button"
              aria-labelledby="simple-inline-chip-group-expandedremove_chip_one_select simple-inline-chip-group-expandedchip_one_select"
              aria-label="Remove"
              id="simple-inline-chip-group-expandedremove_chip_one_select"
            >
              <i class="fas fa-times" aria-hidden="true"></i>
            </button>
          </span>
        </div>
      </li>
      <li class="pf-v5-c-chip-group__list-item">
        <div class="pf-v5-c-chip">
          <span class="pf-v5-c-chip__content">
            <span
              class="pf-v5-c-chip__text"
              id="simple-inline-chip-group-expandedchip_two_select"
            >Chip two</span>
          </span>
          <span class="pf-v5-c-chip__actions">
            <button
              class="pf-v5-c-button pf-m-plain"
              type="button"
              aria-labelledby="simple-inline-chip-group-expandedremove_chip_two_select simple-inline-chip-group-expandedchip_two_select"
              aria-label="Remove"
              id="simple-inline-chip-group-expandedremove_chip_two_select"
            >
              <i class="fas fa-times" aria-hidden="true"></i>
            </button>
          </span>
        </div>
      </li>
      <li class="pf-v5-c-chip-group__list-item">
        <div class="pf-v5-c-chip">
          <span class="pf-v5-c-chip__content">
            <span
              class="pf-v5-c-chip__text"
              id="simple-inline-chip-group-expandedchip_three_select"
            >Chip three</span>
          </span>
          <span class="pf-v5-c-chip__actions">
            <button
              class="pf-v5-c-button pf-m-plain"
              type="button"
              aria-labelledby="simple-inline-chip-group-expandedremove_chip_three_select simple-inline-chip-group-expandedchip_three_select"
              aria-label="Remove"
              id="simple-inline-chip-group-expandedremove_chip_three_select"
            >
              <i class="fas fa-times" aria-hidden="true"></i>
            </button>
          </span>
        </div>
      </li>
      <li class="pf-v5-c-chip-group__list-item">
        <div class="pf-v5-c-chip">
          <span class="pf-v5-c-chip__content">
            <span
              class="pf-v5-c-chip__text"
              id="simple-inline-chip-group-expandedchip_four_select"
            >Chip four</span>
          </span>
          <span class="pf-v5-c-chip__actions">
            <button
              class="pf-v5-c-button pf-m-plain"
              type="button"
              aria-labelledby="simple-inline-chip-group-expandedremove_chip_four_select simple-inline-chip-group-expandedchip_four_select"
              aria-label="Remove"
              id="simple-inline-chip-group-expandedremove_chip_four_select"
            >
              <i class="fas fa-times" aria-hidden="true"></i>
            </button>
          </span>
        </div>
      </li>
      <li class="pf-v5-c-chip-group__list-item">
        <div class="pf-v5-c-chip">
          <span class="pf-v5-c-chip__content">
            <span
              class="pf-v5-c-chip__text"
              id="simple-inline-chip-group-expandedchip_five_select"
            >Chip five</span>
          </span>
          <span class="pf-v5-c-chip__actions">
            <button
              class="pf-v5-c-button pf-m-plain"
              type="button"
              aria-labelledby="simple-inline-chip-group-expandedremove_chip_five_select simple-inline-chip-group-expandedchip_five_select"
              aria-label="Remove"
              id="simple-inline-chip-group-expandedremove_chip_five_select"
            >
              <i class="fas fa-times" aria-hidden="true"></i>
            </button>
          </span>
        </div>
      </li>
      <li class="pf-v5-c-chip-group__list-item">
        <button class="pf-v5-c-chip pf-m-overflow">
          <span class="pf-v5-c-chip__content">
            <span class="pf-v5-c-chip__text">Show less</span>
          </span>
        </button>
      </li>
    </ul>
  </div>
</div>

```

### Chip group with categories

```html
<div
  class="pf-v5-c-chip-group pf-m-category"
  role="group"
  aria-labelledby="chip-group-with-categories-label"
>
  <div class="pf-v5-c-chip-group__main">
    <span
      class="pf-v5-c-chip-group__label"
      id="chip-group-with-categories-label"
    >Category one</span>
    <ul
      class="pf-v5-c-chip-group__list"
      role="list"
      aria-labelledby="chip-group-with-categories-label"
    >
      <li class="pf-v5-c-chip-group__list-item">
        <div class="pf-v5-c-chip">
          <span class="pf-v5-c-chip__content">
            <span
              class="pf-v5-c-chip__text"
              id="chip-group-with-categorieschip_one_toolbar_collapsed"
            >Chip one</span>
          </span>
          <span class="pf-v5-c-chip__actions">
            <button
              class="pf-v5-c-button pf-m-plain"
              type="button"
              aria-labelledby="chip-group-with-categoriesremove_chip_one_toolbar_collapsed chip-group-with-categorieschip_one_toolbar_collapsed"
              aria-label="Remove"
              id="chip-group-with-categoriesremove_chip_one_toolbar_collapsed"
            >
              <i class="fas fa-times" aria-hidden="true"></i>
            </button>
          </span>
        </div>
      </li>
      <li class="pf-v5-c-chip-group__list-item">
        <div class="pf-v5-c-chip">
          <span class="pf-v5-c-chip__content">
            <span
              class="pf-v5-c-chip__text"
              id="chip-group-with-categorieschip_two_toolbar_collapsed"
            >Chip two</span>
          </span>
          <span class="pf-v5-c-chip__actions">
            <button
              class="pf-v5-c-button pf-m-plain"
              type="button"
              aria-labelledby="chip-group-with-categoriesremove_chip_two_toolbar_collapsed chip-group-with-categorieschip_two_toolbar_collapsed"
              aria-label="Remove"
              id="chip-group-with-categoriesremove_chip_two_toolbar_collapsed"
            >
              <i class="fas fa-times" aria-hidden="true"></i>
            </button>
          </span>
        </div>
      </li>
      <li class="pf-v5-c-chip-group__list-item">
        <div class="pf-v5-c-chip">
          <span class="pf-v5-c-chip__content">
            <span
              class="pf-v5-c-chip__text"
              id="chip-group-with-categorieschip_three_toolbar_collapsed"
            >Chip three</span>
          </span>
          <span class="pf-v5-c-chip__actions">
            <button
              class="pf-v5-c-button pf-m-plain"
              type="button"
              aria-labelledby="chip-group-with-categoriesremove_chip_three_toolbar_collapsed chip-group-with-categorieschip_three_toolbar_collapsed"
              aria-label="Remove"
              id="chip-group-with-categoriesremove_chip_three_toolbar_collapsed"
            >
              <i class="fas fa-times" aria-hidden="true"></i>
            </button>
          </span>
        </div>
      </li>
    </ul>
  </div>
</div>

```

### Chip group with categories and overflow

```html
<div
  class="pf-v5-c-chip-group pf-m-category"
  role="group"
  aria-labelledby="chip-group-with-categories-overflow-label"
>
  <div class="pf-v5-c-chip-group__main">
    <span
      class="pf-v5-c-chip-group__label"
      id="chip-group-with-categories-overflow-label"
    >Category one</span>
    <ul
      class="pf-v5-c-chip-group__list"
      role="list"
      aria-labelledby="chip-group-with-categories-overflow-label"
    >
      <li class="pf-v5-c-chip-group__list-item">
        <div class="pf-v5-c-chip">
          <span class="pf-v5-c-chip__content">
            <span
              class="pf-v5-c-chip__text"
              id="chip-group-with-categories-overflowchip_one_toolbar_collapsed"
            >Chip one</span>
          </span>
          <span class="pf-v5-c-chip__actions">
            <button
              class="pf-v5-c-button pf-m-plain"
              type="button"
              aria-labelledby="chip-group-with-categories-overflowremove_chip_one_toolbar_collapsed chip-group-with-categories-overflowchip_one_toolbar_collapsed"
              aria-label="Remove"
              id="chip-group-with-categories-overflowremove_chip_one_toolbar_collapsed"
            >
              <i class="fas fa-times" aria-hidden="true"></i>
            </button>
          </span>
        </div>
      </li>
      <li class="pf-v5-c-chip-group__list-item">
        <div class="pf-v5-c-chip">
          <span class="pf-v5-c-chip__content">
            <span
              class="pf-v5-c-chip__text"
              id="chip-group-with-categories-overflowchip_two_toolbar_collapsed"
            >Chip two</span>
          </span>
          <span class="pf-v5-c-chip__actions">
            <button
              class="pf-v5-c-button pf-m-plain"
              type="button"
              aria-labelledby="chip-group-with-categories-overflowremove_chip_two_toolbar_collapsed chip-group-with-categories-overflowchip_two_toolbar_collapsed"
              aria-label="Remove"
              id="chip-group-with-categories-overflowremove_chip_two_toolbar_collapsed"
            >
              <i class="fas fa-times" aria-hidden="true"></i>
            </button>
          </span>
        </div>
      </li>
      <li class="pf-v5-c-chip-group__list-item">
        <div class="pf-v5-c-chip">
          <span class="pf-v5-c-chip__content">
            <span
              class="pf-v5-c-chip__text"
              id="chip-group-with-categories-overflowchip_three_toolbar_collapsed"
            >Chip three</span>
          </span>
          <span class="pf-v5-c-chip__actions">
            <button
              class="pf-v5-c-button pf-m-plain"
              type="button"
              aria-labelledby="chip-group-with-categories-overflowremove_chip_three_toolbar_collapsed chip-group-with-categories-overflowchip_three_toolbar_collapsed"
              aria-label="Remove"
              id="chip-group-with-categories-overflowremove_chip_three_toolbar_collapsed"
            >
              <i class="fas fa-times" aria-hidden="true"></i>
            </button>
          </span>
        </div>
      </li>
      <li class="pf-v5-c-chip-group__list-item">
        <button class="pf-v5-c-chip pf-m-overflow">
          <span class="pf-v5-c-chip__content">
            <span class="pf-v5-c-chip__text">2 more</span>
          </span>
        </button>
      </li>
    </ul>
  </div>
</div>

```

### Chip group with categories and overflow expanded

```html
<div
  class="pf-v5-c-chip-group pf-m-category"
  role="group"
  aria-labelledby="chip-group-with-categories-overflow-expanded-label"
>
  <div class="pf-v5-c-chip-group__main">
    <span
      class="pf-v5-c-chip-group__label"
      id="chip-group-with-categories-overflow-expanded-label"
    >Category one</span>
    <ul
      class="pf-v5-c-chip-group__list"
      role="list"
      aria-labelledby="chip-group-with-categories-overflow-expanded-label"
    >
      <li class="pf-v5-c-chip-group__list-item">
        <div class="pf-v5-c-chip">
          <span class="pf-v5-c-chip__content">
            <span
              class="pf-v5-c-chip__text"
              id="chip-group-with-categories-overflow-expandedchip_one_toolbar"
            >Chip one</span>
          </span>
          <span class="pf-v5-c-chip__actions">
            <button
              class="pf-v5-c-button pf-m-plain"
              type="button"
              aria-labelledby="chip-group-with-categories-overflow-expandedremove_chip_one_toolbar chip-group-with-categories-overflow-expandedchip_one_toolbar"
              aria-label="Remove"
              id="chip-group-with-categories-overflow-expandedremove_chip_one_toolbar"
            >
              <i class="fas fa-times" aria-hidden="true"></i>
            </button>
          </span>
        </div>
      </li>
      <li class="pf-v5-c-chip-group__list-item">
        <div class="pf-v5-c-chip">
          <span class="pf-v5-c-chip__content">
            <span
              class="pf-v5-c-chip__text"
              id="chip-group-with-categories-overflow-expandedchip_two_toolbar"
            >Chip two</span>
          </span>
          <span class="pf-v5-c-chip__actions">
            <button
              class="pf-v5-c-button pf-m-plain"
              type="button"
              aria-labelledby="chip-group-with-categories-overflow-expandedremove_chip_two_toolbar chip-group-with-categories-overflow-expandedchip_two_toolbar"
              aria-label="Remove"
              id="chip-group-with-categories-overflow-expandedremove_chip_two_toolbar"
            >
              <i class="fas fa-times" aria-hidden="true"></i>
            </button>
          </span>
        </div>
      </li>
      <li class="pf-v5-c-chip-group__list-item">
        <div class="pf-v5-c-chip">
          <span class="pf-v5-c-chip__content">
            <span
              class="pf-v5-c-chip__text"
              id="chip-group-with-categories-overflow-expandedchip_three_toolbar"
            >Chip three</span>
          </span>
          <span class="pf-v5-c-chip__actions">
            <button
              class="pf-v5-c-button pf-m-plain"
              type="button"
              aria-labelledby="chip-group-with-categories-overflow-expandedremove_chip_three_toolbar chip-group-with-categories-overflow-expandedchip_three_toolbar"
              aria-label="Remove"
              id="chip-group-with-categories-overflow-expandedremove_chip_three_toolbar"
            >
              <i class="fas fa-times" aria-hidden="true"></i>
            </button>
          </span>
        </div>
      </li>
      <li class="pf-v5-c-chip-group__list-item">
        <div class="pf-v5-c-chip">
          <span class="pf-v5-c-chip__content">
            <span
              class="pf-v5-c-chip__text"
              id="chip-group-with-categories-overflow-expandedchip_four_toolbar"
            >Chip four</span>
          </span>
          <span class="pf-v5-c-chip__actions">
            <button
              class="pf-v5-c-button pf-m-plain"
              type="button"
              aria-labelledby="chip-group-with-categories-overflow-expandedremove_chip_four_toolbar chip-group-with-categories-overflow-expandedchip_four_toolbar"
              aria-label="Remove"
              id="chip-group-with-categories-overflow-expandedremove_chip_four_toolbar"
            >
              <i class="fas fa-times" aria-hidden="true"></i>
            </button>
          </span>
        </div>
      </li>
      <li class="pf-v5-c-chip-group__list-item">
        <div class="pf-v5-c-chip">
          <span class="pf-v5-c-chip__content">
            <span
              class="pf-v5-c-chip__text"
              id="chip-group-with-categories-overflow-expandedchip_five_select"
            >Chip five</span>
          </span>
          <span class="pf-v5-c-chip__actions">
            <button
              class="pf-v5-c-button pf-m-plain"
              type="button"
              aria-labelledby="chip-group-with-categories-overflow-expandedremove_chip_five_select chip-group-with-categories-overflow-expandedchip_five_select"
              aria-label="Remove"
              id="chip-group-with-categories-overflow-expandedremove_chip_five_select"
            >
              <i class="fas fa-times" aria-hidden="true"></i>
            </button>
          </span>
        </div>
      </li>
      <li class="pf-v5-c-chip-group__list-item">
        <button class="pf-v5-c-chip pf-m-overflow">
          <span class="pf-v5-c-chip__content">
            <span class="pf-v5-c-chip__text">Show less</span>
          </span>
        </button>
      </li>
    </ul>
  </div>
</div>

```

### Chip group with removable categories

```html
<div
  class="pf-v5-c-chip-group pf-m-category"
  role="group"
  aria-labelledby="chip-group-with-categories-removable-label"
>
  <div class="pf-v5-c-chip-group__main">
    <span
      class="pf-v5-c-chip-group__label"
      id="chip-group-with-categories-removable-label"
    >Category one</span>
    <ul
      class="pf-v5-c-chip-group__list"
      role="list"
      aria-labelledby="chip-group-with-categories-removable-label"
    >
      <li class="pf-v5-c-chip-group__list-item">
        <div class="pf-v5-c-chip">
          <span class="pf-v5-c-chip__content">
            <span
              class="pf-v5-c-chip__text"
              id="chip-group-with-categories-removablechip_one_toolbar"
            >Chip one</span>
          </span>
          <span class="pf-v5-c-chip__actions">
            <button
              class="pf-v5-c-button pf-m-plain"
              type="button"
              aria-labelledby="chip-group-with-categories-removableremove_chip_one_toolbar chip-group-with-categories-removablechip_one_toolbar"
              aria-label="Remove"
              id="chip-group-with-categories-removableremove_chip_one_toolbar"
            >
              <i class="fas fa-times" aria-hidden="true"></i>
            </button>
          </span>
        </div>
      </li>
      <li class="pf-v5-c-chip-group__list-item">
        <div class="pf-v5-c-chip">
          <span class="pf-v5-c-chip__content">
            <span
              class="pf-v5-c-chip__text"
              id="chip-group-with-categories-removablechip_two_toolbar"
            >Chip two</span>
          </span>
          <span class="pf-v5-c-chip__actions">
            <button
              class="pf-v5-c-button pf-m-plain"
              type="button"
              aria-labelledby="chip-group-with-categories-removableremove_chip_two_toolbar chip-group-with-categories-removablechip_two_toolbar"
              aria-label="Remove"
              id="chip-group-with-categories-removableremove_chip_two_toolbar"
            >
              <i class="fas fa-times" aria-hidden="true"></i>
            </button>
          </span>
        </div>
      </li>
      <li class="pf-v5-c-chip-group__list-item">
        <div class="pf-v5-c-chip">
          <span class="pf-v5-c-chip__content">
            <span
              class="pf-v5-c-chip__text"
              id="chip-group-with-categories-removablechip_three_toolbar"
            >Chip three</span>
          </span>
          <span class="pf-v5-c-chip__actions">
            <button
              class="pf-v5-c-button pf-m-plain"
              type="button"
              aria-labelledby="chip-group-with-categories-removableremove_chip_three_toolbar chip-group-with-categories-removablechip_three_toolbar"
              aria-label="Remove"
              id="chip-group-with-categories-removableremove_chip_three_toolbar"
            >
              <i class="fas fa-times" aria-hidden="true"></i>
            </button>
          </span>
        </div>
      </li>
      <li class="pf-v5-c-chip-group__list-item">
        <div class="pf-v5-c-chip">
          <span class="pf-v5-c-chip__content">
            <span
              class="pf-v5-c-chip__text"
              id="chip-group-with-categories-removablechip_four_toolbar"
            >Chip four</span>
          </span>
          <span class="pf-v5-c-chip__actions">
            <button
              class="pf-v5-c-button pf-m-plain"
              type="button"
              aria-labelledby="chip-group-with-categories-removableremove_chip_four_toolbar chip-group-with-categories-removablechip_four_toolbar"
              aria-label="Remove"
              id="chip-group-with-categories-removableremove_chip_four_toolbar"
            >
              <i class="fas fa-times" aria-hidden="true"></i>
            </button>
          </span>
        </div>
      </li>
      <li class="pf-v5-c-chip-group__list-item">
        <div class="pf-v5-c-chip">
          <span class="pf-v5-c-chip__content">
            <span
              class="pf-v5-c-chip__text"
              id="chip-group-with-categories-removablechip_five_toolbar"
            >Chip five</span>
          </span>
          <span class="pf-v5-c-chip__actions">
            <button
              class="pf-v5-c-button pf-m-plain"
              type="button"
              aria-labelledby="chip-group-with-categories-removableremove_chip_five_toolbar chip-group-with-categories-removablechip_five_toolbar"
              aria-label="Remove"
              id="chip-group-with-categories-removableremove_chip_five_toolbar"
            >
              <i class="fas fa-times" aria-hidden="true"></i>
            </button>
          </span>
        </div>
      </li>
      <li class="pf-v5-c-chip-group__list-item">
        <div class="pf-v5-c-chip">
          <span class="pf-v5-c-chip__content">
            <span
              class="pf-v5-c-chip__text"
              id="chip-group-with-categories-removablechip_six_toolbar"
            >Chip six</span>
          </span>
          <span class="pf-v5-c-chip__actions">
            <button
              class="pf-v5-c-button pf-m-plain"
              type="button"
              aria-labelledby="chip-group-with-categories-removableremove_chip_six_toolbar chip-group-with-categories-removablechip_six_toolbar"
              aria-label="Remove"
              id="chip-group-with-categories-removableremove_chip_six_toolbar"
            >
              <i class="fas fa-times" aria-hidden="true"></i>
            </button>
          </span>
        </div>
      </li>
    </ul>
  </div>
  <div class="pf-v5-c-chip-group__close">
    <button
      class="pf-v5-c-button pf-m-plain"
      type="button"
      aria-labelledby="chip-group-with-categories-removable-button chip-group-with-categories-removable-label"
      aria-label="Close chip group"
      id="chip-group-with-categories-removable-button"
    >
      <i class="fas fa-times-circle" aria-hidden="true"></i>
    </button>
  </div>
</div>

```

## Documentation

### Chip overview

A Chip is used to display items that have been filtered or selected from a larger group. They comprise of a text element and a button component that is used to remove the chip from selection. When the text overflows it is truncated using ellipses. A chip can be grouped by using the "chip-group" component.

### Chip accessibility

| Attribute | Applied to | Outcome |
| -- | -- | -- |
| `aria-label="[button label text]"` | `.pf-v5-c-button` |  Provides an accessible name for the button when an icon is used instead of text. Required when an icon is used with no supporting text. |
| `aria-labelledby="[id value of .pf-v5-c-button]"` | `.pf-v5-c-button` | Gives the button an accessible name by referring to the element that provides the position of the button within a list. Required when the button is being removed. |
| `aria-hidden="true"` | `<i>` |  Hides the icon from assistive technologies. |

### Chip usage

| Class | Applied to | Outcome |
| -- | -- | -- |
| `.pf-v5-c-chip` | `<div>`, `<button>`, | Initiates the chip component. Use a `<button>` with overflow chips **Required** |
| `.pf-v5-c-chip__content` | `<span>` | Creates a content wrapper for the chip. **Required** |
| `.pf-v5-c-chip__text` | `<span>` | Initiates the text inside the chip. **Required** |
| `.pf-v5-c-chip__icon` | `<span>` | Initiates the icon inside the chip. |
| `.pf-v5-c-chip__actions` | `<span>` | Creates a wrapper for chip actions. **Required for removable chips** |
| `.pf-m-overflow` | `button.pf-v5-c-chip` | Applies styling of the overflow chip. |
| `.pf-m-draggable` | `.pf-v5-c-chip` | Modifies the chip to be in the draggable state. |

### Chip group overview

A chip group is constrained to the width of its container and will wrap when it exceeds that width. An overflow value can be set and when the number of chips exceeds that value, additional chips will be hidden by default. The default overflow value will be set to 3 chips but this can be adjusted per application needs. The toggle button after the last chip allows the group to be expanded (or collapsed).

If you want to create sub-groupings of chips to represent multiple values applied against the same category, chips can be grouped by category. This can be useful in filtering use cases, for example, where you want all items that match more than one value of the same attribute, e.g., ‘status = down OR needs maintenance’.

The chip group requires the [chip component](#chip-overview). **All single chip accessibility and usage requirements apply.**

### Chip group accessibility

| Attributes for closable chip group button | Applied to | Outcome |
| -- | -- | -- |
| `role="list"` | `.pf-v5-c-chip-group__list` | Indicates that the chip group list is a list element. This role is redundant since `.pf-v5-c-chip-group__list` is a `<ul>` but is required for screen readers to announce the list properly. **Required** |
| `aria-label="[button label text]"` | `.pf-v5-c-chip-group__close > button` |  Provides an accessible name for a chip group close when an icon is used instead of text. Required when an icon is used with no supporting text. **Required** |
| `aria-labelledby="[id value of .pf-v5-c-chip-group__close > button] [id value of .pf-v5-c-chip-group__label]"` | `.pf-v5-c-chip-group__close > button` | Provides an accessible name for the button. **Required** |

### Chip group usage

| Class | Applied to | Outcome |
| -- | -- | -- |
| `.pf-v5-c-chip-group` | `<div>` | Initiates the chip group component. **Required.** |
| `.pf-v5-c-chip-group__list` | `<ul>` | Initiates the container for a list of chips. **Required.** |
| `.pf-v5-c-chip-group__list-item` | `<li>` | Initiates the list item inside of the chip group. **Required.** |
| `.pf-v5-c-chip-group__label` | `<span>` | Initiates the label to be used in the chip group. |
| `.pf-v5-c-chip-group__close` | `<div>` | Initiates the container used for the button to remove the chip group. |
| `.pf-v5-c-chip-group__main` | `<div>` | Initiates the container for the label and list elements so that they wrap together. **Required** |
| `.pf-v5-c-button` | `.pf-v5-c-chip-group__close <button>` | Initiates the button used to remove the chip group. |
| `.pf-m-category` | `.pf-v5-c-chip-group` | Modifies the chip group to support category styling. |
