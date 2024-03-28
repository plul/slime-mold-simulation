---
id: Navigation
section: components
cssPrefix: pf-v5-c-nav
---import './Navigation.css'

## Examples

### Default

```html
<nav class="pf-v5-c-nav" aria-label="Global">
  <ul class="pf-v5-c-nav__list" role="list">
    <li class="pf-v5-c-nav__item">
      <a href="#" class="pf-v5-c-nav__link">Link 1</a>
    </li>
    <li class="pf-v5-c-nav__item">
      <a
        href="#"
        class="pf-v5-c-nav__link pf-m-current"
        aria-current="page"
      >Current link</a>
    </li>
    <li class="pf-v5-c-nav__item">
      <a href="#" class="pf-v5-c-nav__link">Link 3</a>
    </li>
    <li class="pf-v5-c-nav__item">
      <a href="#" class="pf-v5-c-nav__link">Link 4</a>
    </li>
  </ul>
</nav>

```

### Grouped nav

```html
<nav class="pf-v5-c-nav" aria-label="Global">
  <section class="pf-v5-c-nav__section" aria-labelledby="grouped-title1">
    <h2 class="pf-v5-c-nav__section-title" id="grouped-title1">Section title 1</h2>
    <ul class="pf-v5-c-nav__list" role="list">
      <li class="pf-v5-c-nav__item">
        <a href="#" class="pf-v5-c-nav__link">Link 1</a>
      </li>
      <li class="pf-v5-c-nav__item">
        <a href="#" class="pf-v5-c-nav__link">Link 2</a>
      </li>
      <li class="pf-v5-c-nav__item">
        <a href="#" class="pf-v5-c-nav__link">Link 3</a>
      </li>
    </ul>
  </section>
  <section class="pf-v5-c-nav__section" aria-labelledby="grouped-title2">
    <h2 class="pf-v5-c-nav__section-title" id="grouped-title2">Section title 2</h2>
    <ul class="pf-v5-c-nav__list" role="list">
      <li class="pf-v5-c-nav__item">
        <a href="#" class="pf-v5-c-nav__link">Link 1</a>
      </li>
      <li class="pf-v5-c-nav__item">
        <a
          href="#"
          class="pf-v5-c-nav__link pf-m-current"
          aria-current="page"
        >Current link</a>
      </li>
      <li class="pf-v5-c-nav__item">
        <a href="#" class="pf-v5-c-nav__link">Link 3</a>
      </li>
    </ul>
  </section>
</nav>

```

### Grouped nav, no titles

```html
<nav class="pf-v5-c-nav" aria-label="Global">
  <section class="pf-v5-c-nav__section" aria-label="Section one">
    <ul class="pf-v5-c-nav__list" role="list">
      <li class="pf-v5-c-nav__item">
        <a href="#" class="pf-v5-c-nav__link">Link 1</a>
      </li>
      <li class="pf-v5-c-nav__item">
        <a href="#" class="pf-v5-c-nav__link">Link 2</a>
      </li>
      <li class="pf-v5-c-nav__item">
        <a href="#" class="pf-v5-c-nav__link">Link 3</a>
      </li>
    </ul>
  </section>
  <hr class="pf-v5-c-divider" />
  <section class="pf-v5-c-nav__section" aria-label="Section two">
    <ul class="pf-v5-c-nav__list" role="list">
      <li class="pf-v5-c-nav__item">
        <a href="#" class="pf-v5-c-nav__link">Section 2, link 1</a>
      </li>
      <li class="pf-v5-c-nav__item">
        <a
          href="#"
          class="pf-v5-c-nav__link pf-m-current"
          aria-current="page"
        >Current link</a>
      </li>
      <li class="pf-v5-c-nav__item">
        <a href="#" class="pf-v5-c-nav__link">Link 3</a>
      </li>
    </ul>
  </section>
</nav>

```

### Expanded

```html
<nav class="pf-v5-c-nav" aria-label="Global">
  <ul class="pf-v5-c-nav__list" role="list">
    <li class="pf-v5-c-nav__item pf-m-expandable pf-m-expanded pf-m-current">
      <button
        class="pf-v5-c-nav__link"
        id="expandable-example1"
        aria-expanded="true"
      >
        Link 1 (current and expanded example)
        <span class="pf-v5-c-nav__toggle">
          <span class="pf-v5-c-nav__toggle-icon">
            <i class="fas fa-angle-right" aria-hidden="true"></i>
          </span>
        </span>
      </button>
      <section
        class="pf-v5-c-nav__subnav"
        aria-labelledby="expandable-example1"
      >
        <ul class="pf-v5-c-nav__list" role="list">
          <li class="pf-v5-c-nav__item">
            <a href="#" class="pf-v5-c-nav__link">Current link</a>
          </li>
          <li class="pf-v5-c-divider" role="separator"></li>
          <li class="pf-v5-c-nav__item">
            <a href="#" class="pf-v5-c-nav__link">Subnav link 2</a>
          </li>
          <li class="pf-v5-c-nav__item">
            <a
              href="#"
              class="pf-v5-c-nav__link pf-m-current"
              aria-current="page"
            >Subnav link 3</a>
          </li>
        </ul>
      </section>
    </li>
    <li class="pf-v5-c-nav__item pf-m-expandable pf-m-expanded">
      <button
        class="pf-v5-c-nav__link"
        id="expandable-example2"
        aria-expanded="true"
      >
        Link 2 (expanded, but not current example)
        <span
          class="pf-v5-c-nav__toggle"
        >
          <span class="pf-v5-c-nav__toggle-icon">
            <i class="fas fa-angle-right" aria-hidden="true"></i>
          </span>
        </span>
      </button>
      <section
        class="pf-v5-c-nav__subnav"
        aria-labelledby="expandable-example2"
      >
        <ul class="pf-v5-c-nav__list" role="list">
          <li class="pf-v5-c-nav__item">
            <a href="#" class="pf-v5-c-nav__link">Subnav link 1</a>
          </li>
          <li class="pf-v5-c-nav__item">
            <a href="#" class="pf-v5-c-nav__link">Subnav link 2</a>
          </li>
        </ul>
      </section>
    </li>
    <li class="pf-v5-c-nav__item pf-m-expandable">
      <button
        class="pf-v5-c-nav__link"
        id="expandable-example3"
        aria-expanded="false"
      >
        Link 3
        <span class="pf-v5-c-nav__toggle">
          <span class="pf-v5-c-nav__toggle-icon">
            <i class="fas fa-angle-right" aria-hidden="true"></i>
          </span>
        </span>
      </button>
      <section
        class="pf-v5-c-nav__subnav"
        aria-labelledby="expandable-example3"
        hidden
      >
        <ul class="pf-v5-c-nav__list" role="list">
          <li class="pf-v5-c-nav__item">
            <a href="#" class="pf-v5-c-nav__link">Subnav link 1</a>
          </li>
          <li class="pf-v5-c-nav__item">
            <a href="#" class="pf-v5-c-nav__link">Subnav link 2</a>
          </li>
        </ul>
      </section>
    </li>
  </ul>
</nav>

```

### Expanded with subnav titles

```html
<nav class="pf-v5-c-nav" aria-label="Global">
  <ul class="pf-v5-c-nav__list" role="list">
    <li class="pf-v5-c-nav__item pf-m-expandable pf-m-expanded pf-m-current">
      <button class="pf-v5-c-nav__link" aria-expanded="true">
        Link 1
        <span class="pf-v5-c-nav__toggle">
          <span class="pf-v5-c-nav__toggle-icon">
            <i class="fas fa-angle-right" aria-hidden="true"></i>
          </span>
        </span>
      </button>
      <section class="pf-v5-c-nav__subnav" aria-labelledby="subnav-title1">
        <h2
          class="pf-v5-c-nav__subnav-title pf-v5-screen-reader"
          id="subnav-title1"
        >Current and expanded example sub-navigation</h2>
        <ul class="pf-v5-c-nav__list" role="list">
          <li class="pf-v5-c-nav__item">
            <a href="#" class="pf-v5-c-nav__link">Current link</a>
          </li>
          <li class="pf-v5-c-nav__item">
            <a
              href="#"
              class="pf-v5-c-nav__link pf-m-current"
              aria-current="page"
            >Subnav link 2</a>
          </li>
          <li class="pf-v5-c-nav__item">
            <a href="#" class="pf-v5-c-nav__link">Subnav link 3</a>
          </li>
        </ul>
      </section>
    </li>
    <li class="pf-v5-c-nav__item pf-m-expandable pf-m-expanded">
      <button class="pf-v5-c-nav__link" aria-expanded="true">
        Link 2
        <span class="pf-v5-c-nav__toggle">
          <span class="pf-v5-c-nav__toggle-icon">
            <i class="fas fa-angle-right" aria-hidden="true"></i>
          </span>
        </span>
      </button>
      <section class="pf-v5-c-nav__subnav" aria-labelledby="subnav-title2">
        <h2
          class="pf-v5-c-nav__subnav-title pf-v5-screen-reader"
          id="subnav-title2"
        >Expanded, but not current example sub-navigation</h2>
        <ul class="pf-v5-c-nav__list" role="list">
          <li class="pf-v5-c-nav__item">
            <a href="#" class="pf-v5-c-nav__link">Subnav link 1</a>
          </li>
          <li class="pf-v5-c-nav__item">
            <a href="#" class="pf-v5-c-nav__link">Subnav link 2</a>
          </li>
        </ul>
      </section>
    </li>
  </ul>
</nav>

```

### Mixed

```html
<nav class="pf-v5-c-nav" aria-label="Global">
  <ul class="pf-v5-c-nav__list" role="list">
    <li class="pf-v5-c-nav__item">
      <a href="#" class="pf-v5-c-nav__link">Link 1 (not expandable)</a>
    </li>
    <li class="pf-v5-c-nav__item pf-m-expandable pf-m-expanded">
      <button
        class="pf-v5-c-nav__link"
        id="nav-mixed-link2"
        aria-expanded="true"
      >
        Link 2 (expanded, but not current example)
        <span
          class="pf-v5-c-nav__toggle"
        >
          <span class="pf-v5-c-nav__toggle-icon">
            <i class="fas fa-angle-right" aria-hidden="true"></i>
          </span>
        </span>
      </button>
      <section class="pf-v5-c-nav__subnav" aria-labelledby="nav-mixed-link2">
        <ul class="pf-v5-c-nav__list" role="list">
          <li class="pf-v5-c-nav__item">
            <a href="#" class="pf-v5-c-nav__link">Subnav link 1</a>
          </li>
          <li class="pf-v5-c-nav__item">
            <a href="#" class="pf-v5-c-nav__link">Subnav link 2</a>
          </li>
        </ul>
      </section>
    </li>
    <li class="pf-v5-c-nav__item pf-m-expandable pf-m-current">
      <button
        class="pf-v5-c-nav__link"
        id="nav-mixed-link4"
        aria-expanded="false"
      >
        Link 3 (current, but not expanded example)
        <span
          class="pf-v5-c-nav__toggle"
        >
          <span class="pf-v5-c-nav__toggle-icon">
            <i class="fas fa-angle-right" aria-hidden="true"></i>
          </span>
        </span>
      </button>
      <section
        class="pf-v5-c-nav__subnav"
        aria-labelledby="nav-mixed-link4"
        hidden
      >
        <ul class="pf-v5-c-nav__list" role="list">
          <li class="pf-v5-c-nav__item">
            <a href="#" class="pf-v5-c-nav__link">Subnav link 1</a>
          </li>
          <li class="pf-v5-c-nav__item">
            <a
              href="#"
              class="pf-v5-c-nav__link pf-m-current"
              aria-current="page"
            >Subnav link 2</a>
          </li>
          <li class="pf-v5-c-nav__item">
            <a href="#" class="pf-v5-c-nav__link">Subnav link 3</a>
          </li>
        </ul>
      </section>
    </li>
  </ul>
</nav>

```

### Expandable, third level

```html
<nav class="pf-v5-c-nav" aria-label="Global">
  <ul class="pf-v5-c-nav__list" role="list">
    <li class="pf-v5-c-nav__item">
      <a href="#" class="pf-v5-c-nav__link">Clusters</a>
    </li>
    <li class="pf-v5-c-nav__item pf-m-current">
      <a href="#" class="pf-v5-c-nav__link">Overview</a>
    </li>
    <li class="pf-v5-c-nav__item">
      <a href="#" class="pf-v5-c-nav__link">Releases</a>
    </li>
    <li class="pf-v5-c-nav__item pf-m-expandable">
      <button
        class="pf-v5-c-nav__link"
        id="expandable-third-level-example-example-1"
        aria-expanded="false"
      >
        Subscriptions
        <span class="pf-v5-c-nav__toggle">
          <span class="pf-v5-c-nav__toggle-icon">
            <i class="fas fa-angle-right" aria-hidden="true"></i>
          </span>
        </span>
      </button>
      <section
        class="pf-v5-c-nav__subnav"
        aria-labelledby="expandable-third-level-example-example-1"
        hidden
      >
        <ul class="pf-v5-c-nav__list" role="list">
          <li class="pf-v5-c-nav__item">
            <a href="#" class="pf-v5-c-nav__link">Subnav link 1</a>
          </li>
          <li class="pf-v5-c-nav__item">
            <a href="#" class="pf-v5-c-nav__link">Subnav link 2</a>
          </li>
        </ul>
      </section>
    </li>
    <li class="pf-v5-c-nav__item pf-m-expandable pf-m-expanded">
      <button
        class="pf-v5-c-nav__link"
        id="expandable-third-level-example-example-2"
        aria-expanded="true"
      >
        Cost management
        <span class="pf-v5-c-nav__toggle">
          <span class="pf-v5-c-nav__toggle-icon">
            <i class="fas fa-angle-right" aria-hidden="true"></i>
          </span>
        </span>
      </button>
      <section
        class="pf-v5-c-nav__subnav"
        aria-labelledby="expandable-third-level-example-example-2"
      >
        <ul class="pf-v5-c-nav__list" role="list">
          <li class="pf-v5-c-nav__item">
            <a href="#" class="pf-v5-c-nav__link">Overview</a>
          </li>
          <li class="pf-v5-c-nav__item">
            <a href="#" class="pf-v5-c-nav__link">Openshift</a>
          </li>
          <li class="pf-v5-c-nav__item pf-m-expandable pf-m-expanded">
            <button
              class="pf-v5-c-nav__link"
              id="expandable-third-level-example-sub-example-1"
              aria-expanded="true"
            >
              Public clouds
              <span class="pf-v5-c-nav__toggle">
                <span class="pf-v5-c-nav__toggle-icon">
                  <i class="fas fa-angle-right" aria-hidden="true"></i>
                </span>
              </span>
            </button>
            <section
              class="pf-v5-c-nav__subnav"
              aria-labelledby="expandable-third-level-example-sub-example-1"
            >
              <ul class="pf-v5-c-nav__list" role="list">
                <li class="pf-v5-c-nav__item">
                  <a href="#" class="pf-v5-c-nav__link">Amazon Web Services</a>
                </li>
                <li class="pf-v5-c-nav__item">
                  <a href="#" class="pf-v5-c-nav__link">Microsoft Azure</a>
                </li>
                <li class="pf-v5-c-nav__item">
                  <a href="#" class="pf-v5-c-nav__link">Google Cloud Services</a>
                </li>
              </ul>
            </section>
          </li>
          <li class="pf-v5-c-nav__item">
            <a href="#" class="pf-v5-c-nav__link">Cost Models</a>
          </li>
          <li class="pf-v5-c-nav__item">
            <a href="#" class="pf-v5-c-nav__link">Cost Explorer</a>
          </li>
        </ul>
      </section>
    </li>
    <li class="pf-v5-c-nav__item">
      <a href="#" class="pf-v5-c-nav__link">Support Cases</a>
    </li>
  </ul>
</nav>

```

### Horizontal

```html
<nav class="pf-v5-c-nav pf-m-horizontal" aria-label="Global">
  <button class="pf-v5-c-nav__scroll-button" disabled aria-label="Scroll left">
    <i class="fas fa-angle-left" aria-hidden="true"></i>
  </button>
  <ul class="pf-v5-c-nav__list" role="list">
    <li class="pf-v5-c-nav__item">
      <a
        href="#"
        class="pf-v5-c-nav__link pf-m-current"
        aria-current="page"
      >Item 1</a>
    </li>
    <li class="pf-v5-c-nav__item">
      <a href="#" class="pf-v5-c-nav__link">Item 2</a>
    </li>
    <li class="pf-v5-c-nav__item">
      <a href="#" class="pf-v5-c-nav__link">Item 3</a>
    </li>
  </ul>
  <button class="pf-v5-c-nav__scroll-button" disabled aria-label="Scroll right">
    <i class="fas fa-angle-right" aria-hidden="true"></i>
  </button>
</nav>

```

### Horizontal overflow

```html
<nav class="pf-v5-c-nav pf-m-horizontal pf-m-scrollable" aria-label="Global">
  <button class="pf-v5-c-nav__scroll-button" disabled aria-label="Scroll left">
    <i class="fas fa-angle-left" aria-hidden="true"></i>
  </button>
  <ul class="pf-v5-c-nav__list" role="list">
    <li class="pf-v5-c-nav__item">
      <a href="#" class="pf-v5-c-nav__link">Horizontal nav item 1</a>
    </li>
    <li class="pf-v5-c-nav__item">
      <a href="#" class="pf-v5-c-nav__link">Horizontal nav item 2</a>
    </li>
    <li class="pf-v5-c-nav__item">
      <a href="#" class="pf-v5-c-nav__link">Horizontal nav item 3</a>
    </li>
    <li class="pf-v5-c-nav__item">
      <a href="#" class="pf-v5-c-nav__link">Horizontal nav item 4</a>
    </li>
    <li class="pf-v5-c-nav__item">
      <a
        href="#"
        class="pf-v5-c-nav__link pf-m-current"
        aria-current="page"
      >Horizontal nav item 5</a>
    </li>
  </ul>
  <button class="pf-v5-c-nav__scroll-button" aria-label="Scroll right">
    <i class="fas fa-angle-right" aria-hidden="true"></i>
  </button>
</nav>

```

### Horizontal subnav

```html
<nav class="pf-v5-c-nav pf-m-horizontal-subnav" aria-label="Local">
  <ul class="pf-v5-c-nav__list" role="list">
    <li class="pf-v5-c-nav__item">
      <a
        href="#"
        class="pf-v5-c-nav__link pf-m-current"
        aria-current="page"
      >Item 1</a>
    </li>
    <li class="pf-v5-c-nav__item">
      <a href="#" class="pf-v5-c-nav__link">Item 2</a>
    </li>
    <li class="pf-v5-c-nav__item">
      <a href="#" class="pf-v5-c-nav__link">Item 3</a>
    </li>
  </ul>
</nav>

```

### Horizontal subnav overflow

```html
<nav
  class="pf-v5-c-nav pf-m-horizontal-subnav pf-m-scrollable"
  aria-label="Global"
>
  <button class="pf-v5-c-nav__scroll-button" disabled aria-label="Scroll left">
    <i class="fas fa-angle-left" aria-hidden="true"></i>
  </button>
  <ul class="pf-v5-c-nav__list" role="list">
    <li class="pf-v5-c-nav__item">
      <a href="#" class="pf-v5-c-nav__link">Horizontal nav item 1</a>
    </li>
    <li class="pf-v5-c-nav__item">
      <a href="#" class="pf-v5-c-nav__link">Horizontal nav item 2</a>
    </li>
    <li class="pf-v5-c-nav__item">
      <a href="#" class="pf-v5-c-nav__link">Horizontal nav item 3</a>
    </li>
    <li class="pf-v5-c-nav__item">
      <a href="#" class="pf-v5-c-nav__link">Horizontal nav item 4</a>
    </li>
    <li class="pf-v5-c-nav__item">
      <a
        href="#"
        class="pf-v5-c-nav__link pf-m-current"
        aria-current="page"
      >Horizontal nav item 5</a>
    </li>
  </ul>
  <button class="pf-v5-c-nav__scroll-button" aria-label="Scroll right">
    <i class="fas fa-angle-right" aria-hidden="true"></i>
  </button>
</nav>

```

### Tertiary

```html isDeprecated
<nav class="pf-v5-c-nav pf-m-tertiary" aria-label="Local">
  <button class="pf-v5-c-nav__scroll-button" disabled aria-label="Scroll left">
    <i class="fas fa-angle-left" aria-hidden="true"></i>
  </button>
  <ul class="pf-v5-c-nav__list" role="list">
    <li class="pf-v5-c-nav__item">
      <a
        href="#"
        class="pf-v5-c-nav__link pf-m-current"
        aria-current="page"
      >Item 1</a>
    </li>
    <li class="pf-v5-c-nav__item">
      <a href="#" class="pf-v5-c-nav__link">Item 2</a>
    </li>
    <li class="pf-v5-c-nav__item">
      <a href="#" class="pf-v5-c-nav__link">Item 3</a>
    </li>
  </ul>
  <button class="pf-v5-c-nav__scroll-button" disabled aria-label="Scroll right">
    <i class="fas fa-angle-right" aria-hidden="true"></i>
  </button>
</nav>

```

### Tertiary overflow

```html isDeprecated
<nav class="pf-v5-c-nav pf-m-tertiary pf-m-scrollable" aria-label="Local">
  <button class="pf-v5-c-nav__scroll-button" disabled aria-label="Scroll left">
    <i class="fas fa-angle-left" aria-hidden="true"></i>
  </button>
  <ul class="pf-v5-c-nav__list" role="list">
    <li class="pf-v5-c-nav__item">
      <a
        href="#"
        class="pf-v5-c-nav__link pf-m-current"
        aria-current="page"
      >Tertiary nav item 1</a>
    </li>
    <li class="pf-v5-c-nav__item">
      <a href="#" class="pf-v5-c-nav__link">Tertiary nav item 2</a>
    </li>
    <li class="pf-v5-c-nav__item">
      <a href="#" class="pf-v5-c-nav__link">Tertiary nav item 3</a>
    </li>
    <li class="pf-v5-c-nav__item">
      <a href="#" class="pf-v5-c-nav__link">Tertiary nav item 4</a>
    </li>
    <li class="pf-v5-c-nav__item">
      <a href="#" class="pf-v5-c-nav__link">Tertiary nav item 5</a>
    </li>
  </ul>
  <button class="pf-v5-c-nav__scroll-button" aria-label="Scroll right">
    <i class="fas fa-angle-right" aria-hidden="true"></i>
  </button>
</nav>

```

### Default in light mode

```html isDeprecated
<nav class="pf-v5-c-nav pf-m-light" aria-label="Global">
  <ul class="pf-v5-c-nav__list" role="list">
    <li class="pf-v5-c-nav__item">
      <a href="#" class="pf-v5-c-nav__link">Current link</a>
    </li>
    <li class="pf-v5-c-nav__item">
      <a
        href="#"
        class="pf-v5-c-nav__link pf-m-current"
        aria-current="page"
      >Link 2</a>
    </li>
    <li class="pf-v5-c-nav__item">
      <a href="#" class="pf-v5-c-nav__link">Link 3</a>
    </li>
    <li class="pf-v5-c-nav__item">
      <a href="#" class="pf-v5-c-nav__link">Link 4</a>
    </li>
  </ul>
</nav>

```

### Expanded in light mode

```html isDeprecated
<nav class="pf-v5-c-nav pf-m-light" aria-label="Global">
  <ul class="pf-v5-c-nav__list" role="list">
    <li class="pf-v5-c-nav__item pf-m-expandable pf-m-expanded pf-m-current">
      <button
        class="pf-v5-c-nav__link"
        id="expandable-light-example1"
        aria-expanded="true"
      >
        Link 1 (current and expanded example)
        <span class="pf-v5-c-nav__toggle">
          <span class="pf-v5-c-nav__toggle-icon">
            <i class="fas fa-angle-right" aria-hidden="true"></i>
          </span>
        </span>
      </button>
      <section
        class="pf-v5-c-nav__subnav"
        aria-labelledby="expandable-light-example1"
      >
        <ul class="pf-v5-c-nav__list" role="list">
          <li class="pf-v5-c-nav__item">
            <a href="#" class="pf-v5-c-nav__link">Current link</a>
          </li>
          <li class="pf-v5-c-divider" role="separator"></li>
          <li class="pf-v5-c-nav__item">
            <a href="#" class="pf-v5-c-nav__link">Subnav link 2</a>
          </li>
          <li class="pf-v5-c-nav__item">
            <a
              href="#"
              class="pf-v5-c-nav__link pf-m-current"
              aria-current="page"
            >Subnav link 3</a>
          </li>
        </ul>
      </section>
    </li>
    <li class="pf-v5-c-nav__item pf-m-expandable pf-m-expanded">
      <button
        class="pf-v5-c-nav__link"
        id="expandable-light-example2"
        aria-expanded="true"
      >
        Link 2 (expanded, but not current example)
        <span
          class="pf-v5-c-nav__toggle"
        >
          <span class="pf-v5-c-nav__toggle-icon">
            <i class="fas fa-angle-right" aria-hidden="true"></i>
          </span>
        </span>
      </button>
      <section
        class="pf-v5-c-nav__subnav"
        aria-labelledby="expandable-light-example2"
      >
        <ul class="pf-v5-c-nav__list" role="list">
          <li class="pf-v5-c-nav__item">
            <a href="#" class="pf-v5-c-nav__link">Subnav link 1</a>
          </li>
          <li class="pf-v5-c-nav__item">
            <a href="#" class="pf-v5-c-nav__link">Subnav link 2</a>
          </li>
        </ul>
      </section>
    </li>
    <li class="pf-v5-c-nav__item pf-m-expandable">
      <button
        class="pf-v5-c-nav__link"
        id="expandable-light-example3"
        aria-expanded="false"
      >
        Link 3
        <span class="pf-v5-c-nav__toggle">
          <span class="pf-v5-c-nav__toggle-icon">
            <i class="fas fa-angle-right" aria-hidden="true"></i>
          </span>
        </span>
      </button>
      <section
        class="pf-v5-c-nav__subnav"
        aria-labelledby="expandable-light-example3"
        hidden
      >
        <ul class="pf-v5-c-nav__list" role="list">
          <li class="pf-v5-c-nav__item">
            <a href="#" class="pf-v5-c-nav__link">Subnav link 1</a>
          </li>
          <li class="pf-v5-c-nav__item">
            <a href="#" class="pf-v5-c-nav__link">Subnav link 2</a>
          </li>
        </ul>
      </section>
    </li>
  </ul>
</nav>

```

### Nav with flyout

```html isBeta
<nav class="pf-v5-c-nav" aria-label="Global">
  <ul class="pf-v5-c-nav__list" role="list">
    <li class="pf-v5-c-nav__item">
      <a href="#" class="pf-v5-c-nav__link">Clusters</a>
    </li>
    <li class="pf-v5-c-nav__item">
      <a href="#" class="pf-v5-c-nav__link">Overview</a>
    </li>
    <li class="pf-v5-c-nav__item">
      <a href="#" class="pf-v5-c-nav__link">Releases</a>
    </li>
    <li class="pf-v5-c-nav__item pf-m-flyout">
      <a
        href="#"
        class="pf-v5-c-nav__link pf-m-hover"
        aria-haspopup="true"
        aria-expanded="true"
      >
        Subscriptions
        <span class="pf-v5-c-nav__toggle">
          <span class="pf-v5-c-nav__toggle-icon">
            <i class="fas fa-angle-right" aria-hidden="true"></i>
          </span>
        </span>
      </a>
    </li>
    <li class="pf-v5-c-nav__item">
      <a href="#" class="pf-v5-c-nav__link">Support cases</a>
    </li>
    <li class="pf-v5-c-nav__item">
      <a href="#" class="pf-v5-c-nav__link">Cluster manager feedback</a>
    </li>
    <li class="pf-v5-c-nav__item">
      <a href="#" class="pf-v5-c-nav__link">Red Hat Marketplace</a>
    </li>
    <li class="pf-v5-c-nav__item">
      <a href="#" class="pf-v5-c-nav__link">Documentation</a>
    </li>
  </ul>
</nav>
<div class="pf-v5-c-menu pf-m-flyout pf-m-nav">
  <div class="pf-v5-c-menu__content">
    <ul class="pf-v5-c-menu__list" role="menu">
      <li class="pf-v5-c-menu__list-item" role="none">
        <a class="pf-v5-c-menu__item" href="#" role="menuitem">
          <span class="pf-v5-c-menu__item-main">
            <span class="pf-v5-c-menu__item-text">Container platform</span>
          </span>
        </a>
      </li>
      <li class="pf-v5-c-menu__list-item" role="none">
        <button
          class="pf-v5-c-menu__item"
          type="button"
          role="menuitem"
          aria-expanded="true"
        >
          <span class="pf-v5-c-menu__item-main">
            <span class="pf-v5-c-menu__item-text">Dedicated</span>
            <span class="pf-v5-c-menu__item-toggle-icon">
              <i class="fas fa-angle-right"></i>
            </span>
          </span>
        </button>
        <div class="pf-v5-c-menu">
          <div class="pf-v5-c-menu__content">
            <ul class="pf-v5-c-menu__list" role="menu">
              <li class="pf-v5-c-menu__list-item" role="none">
                <a class="pf-v5-c-menu__item" href="#" role="menuitem">
                  <span class="pf-v5-c-menu__item-main">
                    <span class="pf-v5-c-menu__item-text">Dedicated (Annual)</span>
                  </span>
                </a>
              </li>
              <li class="pf-v5-c-menu__list-item" role="none">
                <a class="pf-v5-c-menu__item" href="#" role="menuitem">
                  <span class="pf-v5-c-menu__item-main">
                    <span class="pf-v5-c-menu__item-text">Dedicated (On-Demand)</span>
                  </span>
                </a>
              </li>
              <li class="pf-v5-c-menu__list-item" role="none">
                <a class="pf-v5-c-menu__item" href="#" role="menuitem">
                  <span class="pf-v5-c-menu__item-main">
                    <span
                      class="pf-v5-c-menu__item-text"
                    >Dedicated (On-Demand limits)</span>
                  </span>
                </a>
              </li>
            </ul>
          </div>
        </div>
      </li>
    </ul>
  </div>
</div>

```

### Nav with drilldown menu

```html isBeta
<nav class="pf-v5-c-nav" aria-label="Drilldown menu example">
  <div class="pf-v5-c-menu pf-m-drilldown">
    <div class="pf-v5-c-menu__content">
      <ul class="pf-v5-c-menu__list" role="menu">
        <li class="pf-v5-c-menu__list-item" role="none">
          <button class="pf-v5-c-menu__item" type="button" role="menuitem">
            <span class="pf-v5-c-menu__item-main">
              <span class="pf-v5-c-menu__item-text">Start rollout</span>
            </span>
          </button>
        </li>
        <li class="pf-v5-c-menu__list-item" role="none">
          <button class="pf-v5-c-menu__item" type="button" role="menuitem">
            <span class="pf-v5-c-menu__item-main">
              <span class="pf-v5-c-menu__item-text">Pause rollout</span>
            </span>
          </button>
        </li>
        <li class="pf-v5-c-menu__list-item" role="none">
          <button
            class="pf-v5-c-menu__item pf-m-current"
            type="button"
            role="menuitem"
          >
            <span class="pf-v5-c-menu__item-main">
              <span class="pf-v5-c-menu__item-text">Current link</span>
            </span>
          </button>
        </li>
        <li class="pf-v5-c-menu__list-item" role="none">
          <button class="pf-v5-c-menu__item" type="button" role="menuitem">
            <span class="pf-v5-c-menu__item-main">
              <span class="pf-v5-c-menu__item-text">Add storage</span>
            </span>
          </button>
        </li>
        <li class="pf-v5-c-menu__list-item" role="none">
          <button
            class="pf-v5-c-menu__item"
            type="button"
            role="menuitem"
            aria-expanded="false"
          >
            <span class="pf-v5-c-menu__item-main">
              <span class="pf-v5-c-menu__item-text">Edit</span>
              <span class="pf-v5-c-menu__item-toggle-icon">
                <i class="fas fa-angle-right"></i>
              </span>
            </span>
          </button>
          <div class="pf-v5-c-menu">
            <div class="pf-v5-c-menu__content">
              <ul class="pf-v5-c-menu__list" role="menu">
                <li class="pf-v5-c-menu__list-item pf-m-drill-up" role="none">
                  <button
                    class="pf-v5-c-menu__item"
                    type="button"
                    role="menuitem"
                    tabindex="0"
                  >
                    <span class="pf-v5-c-menu__item-main">
                      <span class="pf-v5-c-menu__item-toggle-icon">
                        <i class="fas fa-angle-left"></i>
                      </span>
                      <span class="pf-v5-c-menu__item-text">Edit</span>
                    </span>
                  </button>
                </li>
                <li class="pf-v5-c-menu__list-item" role="none">
                  <button
                    class="pf-v5-c-menu__item"
                    type="button"
                    role="menuitem"
                    aria-expanded="false"
                  >
                    <span class="pf-v5-c-menu__item-main">
                      <span class="pf-v5-c-menu__item-text">Deployment</span>
                      <span class="pf-v5-c-menu__item-toggle-icon">
                        <i class="fas fa-angle-right"></i>
                      </span>
                    </span>
                  </button>
                  <div class="pf-v5-c-menu">
                    <div class="pf-v5-c-menu__content">
                      <ul class="pf-v5-c-menu__list" role="menu">
                        <li
                          class="pf-v5-c-menu__list-item pf-m-drill-up"
                          role="none"
                        >
                          <button
                            class="pf-v5-c-menu__item"
                            type="button"
                            role="menuitem"
                            tabindex="0"
                          >
                            <span class="pf-v5-c-menu__item-main">
                              <span class="pf-v5-c-menu__item-toggle-icon">
                                <i class="fas fa-angle-left"></i>
                              </span>
                              <span class="pf-v5-c-menu__item-text">Deployment</span>
                            </span>
                          </button>
                        </li>
                        <li class="pf-v5-c-menu__list-item" role="none">
                          <button
                            class="pf-v5-c-menu__item"
                            type="button"
                            role="menuitem"
                          >
                            <span class="pf-v5-c-menu__item-main">
                              <span class="pf-v5-c-menu__item-text">Routes</span>
                            </span>
                          </button>
                        </li>
                        <li class="pf-v5-c-menu__list-item" role="none">
                          <button
                            class="pf-v5-c-menu__item"
                            type="button"
                            role="menuitem"
                          >
                            <span class="pf-v5-c-menu__item-main">
                              <span class="pf-v5-c-menu__item-text">Nodes</span>
                            </span>
                          </button>
                        </li>
                        <li class="pf-v5-c-menu__list-item" role="none">
                          <button
                            class="pf-v5-c-menu__item"
                            type="button"
                            role="menuitem"
                          >
                            <span class="pf-v5-c-menu__item-main">
                              <span class="pf-v5-c-menu__item-text">URLs</span>
                            </span>
                          </button>
                        </li>
                        <li class="pf-v5-c-menu__list-item" role="none">
                          <button
                            class="pf-v5-c-menu__item"
                            type="button"
                            role="menuitem"
                            aria-expanded="false"
                          >
                            <span class="pf-v5-c-menu__item-main">
                              <span
                                class="pf-v5-c-menu__item-text"
                              >Advanced settings</span>
                              <span class="pf-v5-c-menu__item-toggle-icon">
                                <i class="fas fa-angle-right"></i>
                              </span>
                            </span>
                          </button>
                          <div class="pf-v5-c-menu">
                            <div class="pf-v5-c-menu__content">
                              <ul class="pf-v5-c-menu__list" role="menu">
                                <li
                                  class="pf-v5-c-menu__list-item pf-m-drill-up"
                                  role="none"
                                >
                                  <button
                                    class="pf-v5-c-menu__item"
                                    type="button"
                                    role="menuitem"
                                    tabindex="0"
                                  >
                                    <span class="pf-v5-c-menu__item-main">
                                      <span
                                        class="pf-v5-c-menu__item-toggle-icon"
                                      >
                                        <i class="fas fa-angle-left"></i>
                                      </span>
                                      <span
                                        class="pf-v5-c-menu__item-text"
                                      >Advanced settings</span>
                                    </span>
                                  </button>
                                </li>
                                <li class="pf-v5-c-menu__list-item" role="none">
                                  <button
                                    class="pf-v5-c-menu__item"
                                    type="button"
                                    role="menuitem"
                                  >
                                    <span class="pf-v5-c-menu__item-main">
                                      <span
                                        class="pf-v5-c-menu__item-text"
                                      >Reports</span>
                                    </span>
                                  </button>
                                </li>
                                <li class="pf-v5-c-menu__list-item" role="none">
                                  <button
                                    class="pf-v5-c-menu__item"
                                    type="button"
                                    role="menuitem"
                                  >
                                    <span class="pf-v5-c-menu__item-main">
                                      <span
                                        class="pf-v5-c-menu__item-text"
                                      >Policies</span>
                                    </span>
                                  </button>
                                </li>
                                <li class="pf-v5-c-menu__list-item" role="none">
                                  <button
                                    class="pf-v5-c-menu__item"
                                    type="button"
                                    role="menuitem"
                                  >
                                    <span class="pf-v5-c-menu__item-main">
                                      <span
                                        class="pf-v5-c-menu__item-text"
                                      >Systems</span>
                                    </span>
                                  </button>
                                </li>
                              </ul>
                            </div>
                          </div>
                        </li>
                      </ul>
                    </div>
                  </div>
                </li>
                <li class="pf-v5-c-menu__list-item" role="none">
                  <button
                    class="pf-v5-c-menu__item"
                    type="button"
                    role="menuitem"
                    aria-expanded="false"
                  >
                    <span class="pf-v5-c-menu__item-main">
                      <span class="pf-v5-c-menu__item-text">RBAC</span>
                      <span class="pf-v5-c-menu__item-toggle-icon">
                        <i class="fas fa-angle-right"></i>
                      </span>
                    </span>
                  </button>
                  <div class="pf-v5-c-menu">
                    <div class="pf-v5-c-menu__content">
                      <ul class="pf-v5-c-menu__list" role="menu">
                        <li
                          class="pf-v5-c-menu__list-item pf-m-drill-up"
                          role="none"
                        >
                          <button
                            class="pf-v5-c-menu__item"
                            type="button"
                            role="menuitem"
                          >
                            <span class="pf-v5-c-menu__item-main">
                              <span class="pf-v5-c-menu__item-toggle-icon">
                                <i class="fas fa-angle-left"></i>
                              </span>
                              <span class="pf-v5-c-menu__item-text">RBAC</span>
                            </span>
                          </button>
                        </li>
                        <li class="pf-v5-c-menu__list-item" role="none">
                          <button
                            class="pf-v5-c-menu__item"
                            type="button"
                            role="menuitem"
                          >
                            <span class="pf-v5-c-menu__item-main">
                              <span class="pf-v5-c-menu__item-text">Reports</span>
                            </span>
                          </button>
                        </li>
                        <li class="pf-v5-c-menu__list-item" role="none">
                          <button
                            class="pf-v5-c-menu__item"
                            type="button"
                            role="menuitem"
                          >
                            <span class="pf-v5-c-menu__item-main">
                              <span class="pf-v5-c-menu__item-text">Policies</span>
                            </span>
                          </button>
                        </li>
                        <li class="pf-v5-c-menu__list-item" role="none">
                          <button
                            class="pf-v5-c-menu__item"
                            type="button"
                            role="menuitem"
                          >
                            <span class="pf-v5-c-menu__item-main">
                              <span class="pf-v5-c-menu__item-text">Systems</span>
                            </span>
                          </button>
                        </li>
                      </ul>
                    </div>
                  </div>
                </li>
                <li class="pf-v5-c-menu__list-item" role="none">
                  <button
                    class="pf-v5-c-menu__item"
                    type="button"
                    role="menuitem"
                  >
                    <span class="pf-v5-c-menu__item-main">
                      <span
                        class="pf-v5-c-menu__item-text"
                      >Thing with a longer label</span>
                    </span>
                  </button>
                </li>
              </ul>
            </div>
          </div>
        </li>
        <li class="pf-v5-c-menu__list-item" role="none">
          <button
            class="pf-v5-c-menu__item"
            type="button"
            role="menuitem"
            aria-expanded="false"
          >
            <span class="pf-v5-c-menu__item-main">
              <span class="pf-v5-c-menu__item-text">Configuration</span>
              <span class="pf-v5-c-menu__item-toggle-icon">
                <i class="fas fa-angle-right"></i>
              </span>
            </span>
          </button>
          <div class="pf-v5-c-menu">
            <div class="pf-v5-c-menu__content">
              <ul class="pf-v5-c-menu__list" role="menu">
                <li class="pf-v5-c-menu__list-item pf-m-drill-up" role="none">
                  <button
                    class="pf-v5-c-menu__item"
                    type="button"
                    role="menuitem"
                  >
                    <span class="pf-v5-c-menu__item-main">
                      <span class="pf-v5-c-menu__item-toggle-icon">
                        <i class="fas fa-angle-left"></i>
                      </span>
                      <span class="pf-v5-c-menu__item-text">Configuration</span>
                    </span>
                  </button>
                </li>
                <li class="pf-v5-c-menu__list-item" role="none">
                  <button
                    class="pf-v5-c-menu__item"
                    type="button"
                    role="menuitem"
                    aria-expanded="false"
                  >
                    <span class="pf-v5-c-menu__item-main">
                      <span class="pf-v5-c-menu__item-text">Profile</span>
                      <span class="pf-v5-c-menu__item-toggle-icon">
                        <i class="fas fa-angle-right"></i>
                      </span>
                    </span>
                  </button>
                  <div class="pf-v5-c-menu">
                    <div class="pf-v5-c-menu__content">
                      <ul class="pf-v5-c-menu__list" role="menu">
                        <li
                          class="pf-v5-c-menu__list-item pf-m-drill-up"
                          role="none"
                        >
                          <button
                            class="pf-v5-c-menu__item"
                            type="button"
                            role="menuitem"
                          >
                            <span class="pf-v5-c-menu__item-main">
                              <span class="pf-v5-c-menu__item-toggle-icon">
                                <i class="fas fa-angle-left"></i>
                              </span>
                              <span class="pf-v5-c-menu__item-text">Profile</span>
                            </span>
                          </button>
                        </li>
                        <li class="pf-v5-c-menu__list-item" role="none">
                          <button
                            class="pf-v5-c-menu__item"
                            type="button"
                            role="menuitem"
                          >
                            <span class="pf-v5-c-menu__item-main">
                              <span class="pf-v5-c-menu__item-text">Avatar</span>
                            </span>
                          </button>
                        </li>
                        <li class="pf-v5-c-menu__list-item" role="none">
                          <button
                            class="pf-v5-c-menu__item"
                            type="button"
                            role="menuitem"
                          >
                            <span class="pf-v5-c-menu__item-main">
                              <span class="pf-v5-c-menu__item-text">Name</span>
                            </span>
                          </button>
                        </li>
                        <li class="pf-v5-c-menu__list-item" role="none">
                          <button
                            class="pf-v5-c-menu__item"
                            type="button"
                            role="menuitem"
                          >
                            <span class="pf-v5-c-menu__item-main">
                              <span class="pf-v5-c-menu__item-text">Title</span>
                            </span>
                          </button>
                        </li>
                      </ul>
                    </div>
                  </div>
                </li>
                <li class="pf-v5-c-menu__list-item" role="none">
                  <button
                    class="pf-v5-c-menu__item"
                    type="button"
                    role="menuitem"
                  >
                    <span class="pf-v5-c-menu__item-main">
                      <span class="pf-v5-c-menu__item-text">Time zone</span>
                    </span>
                  </button>
                </li>
                <li class="pf-v5-c-menu__list-item" role="none">
                  <button
                    class="pf-v5-c-menu__item"
                    type="button"
                    role="menuitem"
                  >
                    <span class="pf-v5-c-menu__item-main">
                      <span class="pf-v5-c-menu__item-text">Account settings</span>
                    </span>
                  </button>
                </li>
                <li class="pf-v5-c-menu__list-item" role="none">
                  <button
                    class="pf-v5-c-menu__item"
                    type="button"
                    role="menuitem"
                  >
                    <span class="pf-v5-c-menu__item-main">
                      <span
                        class="pf-v5-c-menu__item-text"
                      >Thing with a longer label</span>
                    </span>
                  </button>
                </li>
                <li class="pf-v5-c-menu__list-item" role="none">
                  <button
                    class="pf-v5-c-menu__item"
                    type="button"
                    role="menuitem"
                    aria-expanded="false"
                  >
                    <span class="pf-v5-c-menu__item-main">
                      <span class="pf-v5-c-menu__item-text">Edit access settings</span>
                      <span class="pf-v5-c-menu__item-toggle-icon">
                        <i class="fas fa-angle-right"></i>
                      </span>
                    </span>
                  </button>
                  <div class="pf-v5-c-menu">
                    <div class="pf-v5-c-menu__content">
                      <ul class="pf-v5-c-menu__list" role="menu">
                        <li
                          class="pf-v5-c-menu__list-item pf-m-drill-up"
                          role="none"
                        >
                          <button
                            class="pf-v5-c-menu__item"
                            type="button"
                            role="menuitem"
                          >
                            <span class="pf-v5-c-menu__item-main">
                              <span class="pf-v5-c-menu__item-toggle-icon">
                                <i class="fas fa-angle-left"></i>
                              </span>
                              <span
                                class="pf-v5-c-menu__item-text"
                              >Edit access settings</span>
                            </span>
                          </button>
                        </li>
                        <li class="pf-v5-c-menu__list-item" role="none">
                          <button
                            class="pf-v5-c-menu__item"
                            type="button"
                            role="menuitem"
                          >
                            <span class="pf-v5-c-menu__item-main">
                              <span
                                class="pf-v5-c-menu__item-text"
                              >Global access</span>
                            </span>
                          </button>
                        </li>
                        <li class="pf-v5-c-menu__list-item" role="none">
                          <button
                            class="pf-v5-c-menu__item"
                            type="button"
                            role="menuitem"
                          >
                            <span class="pf-v5-c-menu__item-main">
                              <span
                                class="pf-v5-c-menu__item-text"
                              >Account access</span>
                            </span>
                          </button>
                        </li>
                      </ul>
                    </div>
                  </div>
                </li>
              </ul>
            </div>
          </div>
        </li>
      </ul>
    </div>
  </div>
</nav>

```

### Nav with drilldown menu level two

```html isBeta
<nav class="pf-v5-c-nav" aria-label="Drilldown menu example">
  <div class="pf-v5-c-menu pf-m-drilldown pf-m-drilled-in">
    <div
      class="pf-v5-c-menu__content"
      style="--pf-v5-c-menu__content--Height: 228px;"
    >
      <ul class="pf-v5-c-menu__list" role="menu">
        <li class="pf-v5-c-menu__list-item" role="none">
          <button class="pf-v5-c-menu__item" type="button" role="menuitem">
            <span class="pf-v5-c-menu__item-main">
              <span class="pf-v5-c-menu__item-text">Start rollout</span>
            </span>
          </button>
        </li>
        <li class="pf-v5-c-menu__list-item" role="none">
          <button class="pf-v5-c-menu__item" type="button" role="menuitem">
            <span class="pf-v5-c-menu__item-main">
              <span class="pf-v5-c-menu__item-text">Pause rollout</span>
            </span>
          </button>
        </li>
        <li class="pf-v5-c-menu__list-item" role="none">
          <button
            class="pf-v5-c-menu__item pf-m-current"
            type="button"
            role="menuitem"
          >
            <span class="pf-v5-c-menu__item-main">
              <span class="pf-v5-c-menu__item-text">Current link</span>
            </span>
          </button>
        </li>
        <li class="pf-v5-c-menu__list-item" role="none">
          <button class="pf-v5-c-menu__item" type="button" role="menuitem">
            <span class="pf-v5-c-menu__item-main">
              <span class="pf-v5-c-menu__item-text">Add storage</span>
            </span>
          </button>
        </li>
        <li class="pf-v5-c-menu__list-item pf-m-current-path" role="none">
          <button
            class="pf-v5-c-menu__item"
            type="button"
            role="menuitem"
            aria-expanded="true"
          >
            <span class="pf-v5-c-menu__item-main">
              <span class="pf-v5-c-menu__item-text">Edit</span>
              <span class="pf-v5-c-menu__item-toggle-icon">
                <i class="fas fa-angle-right"></i>
              </span>
            </span>
          </button>
          <div class="pf-v5-c-menu">
            <div class="pf-v5-c-menu__content">
              <ul class="pf-v5-c-menu__list" role="menu">
                <li class="pf-v5-c-menu__list-item pf-m-drill-up" role="none">
                  <button
                    class="pf-v5-c-menu__item"
                    type="button"
                    role="menuitem"
                    tabindex="0"
                  >
                    <span class="pf-v5-c-menu__item-main">
                      <span class="pf-v5-c-menu__item-toggle-icon">
                        <i class="fas fa-angle-left"></i>
                      </span>
                      <span class="pf-v5-c-menu__item-text">Edit</span>
                    </span>
                  </button>
                </li>
                <li class="pf-v5-c-menu__list-item" role="none">
                  <button
                    class="pf-v5-c-menu__item"
                    type="button"
                    role="menuitem"
                    aria-expanded="false"
                  >
                    <span class="pf-v5-c-menu__item-main">
                      <span class="pf-v5-c-menu__item-text">Deployment</span>
                      <span class="pf-v5-c-menu__item-toggle-icon">
                        <i class="fas fa-angle-right"></i>
                      </span>
                    </span>
                  </button>
                  <div class="pf-v5-c-menu">
                    <div class="pf-v5-c-menu__content">
                      <ul class="pf-v5-c-menu__list" role="menu">
                        <li
                          class="pf-v5-c-menu__list-item pf-m-drill-up"
                          role="none"
                        >
                          <button
                            class="pf-v5-c-menu__item"
                            type="button"
                            role="menuitem"
                            tabindex="0"
                          >
                            <span class="pf-v5-c-menu__item-main">
                              <span class="pf-v5-c-menu__item-toggle-icon">
                                <i class="fas fa-angle-left"></i>
                              </span>
                              <span class="pf-v5-c-menu__item-text">Deployment</span>
                            </span>
                          </button>
                        </li>
                        <li class="pf-v5-c-menu__list-item" role="none">
                          <button
                            class="pf-v5-c-menu__item"
                            type="button"
                            role="menuitem"
                          >
                            <span class="pf-v5-c-menu__item-main">
                              <span class="pf-v5-c-menu__item-text">Routes</span>
                            </span>
                          </button>
                        </li>
                        <li class="pf-v5-c-menu__list-item" role="none">
                          <button
                            class="pf-v5-c-menu__item"
                            type="button"
                            role="menuitem"
                          >
                            <span class="pf-v5-c-menu__item-main">
                              <span class="pf-v5-c-menu__item-text">Nodes</span>
                            </span>
                          </button>
                        </li>
                        <li class="pf-v5-c-menu__list-item" role="none">
                          <button
                            class="pf-v5-c-menu__item"
                            type="button"
                            role="menuitem"
                          >
                            <span class="pf-v5-c-menu__item-main">
                              <span class="pf-v5-c-menu__item-text">URLs</span>
                            </span>
                          </button>
                        </li>
                        <li class="pf-v5-c-menu__list-item" role="none">
                          <button
                            class="pf-v5-c-menu__item"
                            type="button"
                            role="menuitem"
                            aria-expanded="false"
                          >
                            <span class="pf-v5-c-menu__item-main">
                              <span
                                class="pf-v5-c-menu__item-text"
                              >Advanced settings</span>
                              <span class="pf-v5-c-menu__item-toggle-icon">
                                <i class="fas fa-angle-right"></i>
                              </span>
                            </span>
                          </button>
                          <div class="pf-v5-c-menu">
                            <div class="pf-v5-c-menu__content">
                              <ul class="pf-v5-c-menu__list" role="menu">
                                <li
                                  class="pf-v5-c-menu__list-item pf-m-drill-up"
                                  role="none"
                                >
                                  <button
                                    class="pf-v5-c-menu__item"
                                    type="button"
                                    role="menuitem"
                                    tabindex="0"
                                  >
                                    <span class="pf-v5-c-menu__item-main">
                                      <span
                                        class="pf-v5-c-menu__item-toggle-icon"
                                      >
                                        <i class="fas fa-angle-left"></i>
                                      </span>
                                      <span
                                        class="pf-v5-c-menu__item-text"
                                      >Advanced settings</span>
                                    </span>
                                  </button>
                                </li>
                                <li class="pf-v5-c-menu__list-item" role="none">
                                  <button
                                    class="pf-v5-c-menu__item"
                                    type="button"
                                    role="menuitem"
                                  >
                                    <span class="pf-v5-c-menu__item-main">
                                      <span
                                        class="pf-v5-c-menu__item-text"
                                      >Reports</span>
                                    </span>
                                  </button>
                                </li>
                                <li class="pf-v5-c-menu__list-item" role="none">
                                  <button
                                    class="pf-v5-c-menu__item"
                                    type="button"
                                    role="menuitem"
                                  >
                                    <span class="pf-v5-c-menu__item-main">
                                      <span
                                        class="pf-v5-c-menu__item-text"
                                      >Policies</span>
                                    </span>
                                  </button>
                                </li>
                                <li class="pf-v5-c-menu__list-item" role="none">
                                  <button
                                    class="pf-v5-c-menu__item"
                                    type="button"
                                    role="menuitem"
                                  >
                                    <span class="pf-v5-c-menu__item-main">
                                      <span
                                        class="pf-v5-c-menu__item-text"
                                      >Systems</span>
                                    </span>
                                  </button>
                                </li>
                              </ul>
                            </div>
                          </div>
                        </li>
                      </ul>
                    </div>
                  </div>
                </li>
                <li class="pf-v5-c-menu__list-item" role="none">
                  <button
                    class="pf-v5-c-menu__item"
                    type="button"
                    role="menuitem"
                    aria-expanded="false"
                  >
                    <span class="pf-v5-c-menu__item-main">
                      <span class="pf-v5-c-menu__item-text">RBAC</span>
                      <span class="pf-v5-c-menu__item-toggle-icon">
                        <i class="fas fa-angle-right"></i>
                      </span>
                    </span>
                  </button>
                  <div class="pf-v5-c-menu">
                    <div class="pf-v5-c-menu__content">
                      <ul class="pf-v5-c-menu__list" role="menu">
                        <li
                          class="pf-v5-c-menu__list-item pf-m-drill-up"
                          role="none"
                        >
                          <button
                            class="pf-v5-c-menu__item"
                            type="button"
                            role="menuitem"
                          >
                            <span class="pf-v5-c-menu__item-main">
                              <span class="pf-v5-c-menu__item-toggle-icon">
                                <i class="fas fa-angle-left"></i>
                              </span>
                              <span class="pf-v5-c-menu__item-text">RBAC</span>
                            </span>
                          </button>
                        </li>
                        <li class="pf-v5-c-menu__list-item" role="none">
                          <button
                            class="pf-v5-c-menu__item"
                            type="button"
                            role="menuitem"
                          >
                            <span class="pf-v5-c-menu__item-main">
                              <span class="pf-v5-c-menu__item-text">Reports</span>
                            </span>
                          </button>
                        </li>
                        <li class="pf-v5-c-menu__list-item" role="none">
                          <button
                            class="pf-v5-c-menu__item"
                            type="button"
                            role="menuitem"
                          >
                            <span class="pf-v5-c-menu__item-main">
                              <span class="pf-v5-c-menu__item-text">Policies</span>
                            </span>
                          </button>
                        </li>
                        <li class="pf-v5-c-menu__list-item" role="none">
                          <button
                            class="pf-v5-c-menu__item"
                            type="button"
                            role="menuitem"
                          >
                            <span class="pf-v5-c-menu__item-main">
                              <span class="pf-v5-c-menu__item-text">Systems</span>
                            </span>
                          </button>
                        </li>
                      </ul>
                    </div>
                  </div>
                </li>
                <li class="pf-v5-c-menu__list-item" role="none">
                  <button
                    class="pf-v5-c-menu__item"
                    type="button"
                    role="menuitem"
                  >
                    <span class="pf-v5-c-menu__item-main">
                      <span
                        class="pf-v5-c-menu__item-text"
                      >Thing with a longer label</span>
                    </span>
                  </button>
                </li>
              </ul>
            </div>
          </div>
        </li>
        <li class="pf-v5-c-menu__list-item" role="none">
          <button
            class="pf-v5-c-menu__item"
            type="button"
            role="menuitem"
            aria-expanded="false"
          >
            <span class="pf-v5-c-menu__item-main">
              <span class="pf-v5-c-menu__item-text">Configuration</span>
              <span class="pf-v5-c-menu__item-toggle-icon">
                <i class="fas fa-angle-right"></i>
              </span>
            </span>
          </button>
          <div class="pf-v5-c-menu">
            <div class="pf-v5-c-menu__content">
              <ul class="pf-v5-c-menu__list" role="menu">
                <li class="pf-v5-c-menu__list-item pf-m-drill-up" role="none">
                  <button
                    class="pf-v5-c-menu__item"
                    type="button"
                    role="menuitem"
                  >
                    <span class="pf-v5-c-menu__item-main">
                      <span class="pf-v5-c-menu__item-toggle-icon">
                        <i class="fas fa-angle-left"></i>
                      </span>
                      <span class="pf-v5-c-menu__item-text">Configuration</span>
                    </span>
                  </button>
                </li>
                <li class="pf-v5-c-menu__list-item" role="none">
                  <button
                    class="pf-v5-c-menu__item"
                    type="button"
                    role="menuitem"
                    aria-expanded="false"
                  >
                    <span class="pf-v5-c-menu__item-main">
                      <span class="pf-v5-c-menu__item-text">Profile</span>
                      <span class="pf-v5-c-menu__item-toggle-icon">
                        <i class="fas fa-angle-right"></i>
                      </span>
                    </span>
                  </button>
                  <div class="pf-v5-c-menu">
                    <div class="pf-v5-c-menu__content">
                      <ul class="pf-v5-c-menu__list" role="menu">
                        <li
                          class="pf-v5-c-menu__list-item pf-m-drill-up"
                          role="none"
                        >
                          <button
                            class="pf-v5-c-menu__item"
                            type="button"
                            role="menuitem"
                          >
                            <span class="pf-v5-c-menu__item-main">
                              <span class="pf-v5-c-menu__item-toggle-icon">
                                <i class="fas fa-angle-left"></i>
                              </span>
                              <span class="pf-v5-c-menu__item-text">Profile</span>
                            </span>
                          </button>
                        </li>
                        <li class="pf-v5-c-menu__list-item" role="none">
                          <button
                            class="pf-v5-c-menu__item"
                            type="button"
                            role="menuitem"
                          >
                            <span class="pf-v5-c-menu__item-main">
                              <span class="pf-v5-c-menu__item-text">Avatar</span>
                            </span>
                          </button>
                        </li>
                        <li class="pf-v5-c-menu__list-item" role="none">
                          <button
                            class="pf-v5-c-menu__item"
                            type="button"
                            role="menuitem"
                          >
                            <span class="pf-v5-c-menu__item-main">
                              <span class="pf-v5-c-menu__item-text">Name</span>
                            </span>
                          </button>
                        </li>
                        <li class="pf-v5-c-menu__list-item" role="none">
                          <button
                            class="pf-v5-c-menu__item"
                            type="button"
                            role="menuitem"
                          >
                            <span class="pf-v5-c-menu__item-main">
                              <span class="pf-v5-c-menu__item-text">Title</span>
                            </span>
                          </button>
                        </li>
                      </ul>
                    </div>
                  </div>
                </li>
                <li class="pf-v5-c-menu__list-item" role="none">
                  <button
                    class="pf-v5-c-menu__item"
                    type="button"
                    role="menuitem"
                  >
                    <span class="pf-v5-c-menu__item-main">
                      <span class="pf-v5-c-menu__item-text">Time zone</span>
                    </span>
                  </button>
                </li>
                <li class="pf-v5-c-menu__list-item" role="none">
                  <button
                    class="pf-v5-c-menu__item"
                    type="button"
                    role="menuitem"
                  >
                    <span class="pf-v5-c-menu__item-main">
                      <span class="pf-v5-c-menu__item-text">Account settings</span>
                    </span>
                  </button>
                </li>
                <li class="pf-v5-c-menu__list-item" role="none">
                  <button
                    class="pf-v5-c-menu__item"
                    type="button"
                    role="menuitem"
                  >
                    <span class="pf-v5-c-menu__item-main">
                      <span
                        class="pf-v5-c-menu__item-text"
                      >Thing with a longer label</span>
                    </span>
                  </button>
                </li>
                <li class="pf-v5-c-menu__list-item" role="none">
                  <button
                    class="pf-v5-c-menu__item"
                    type="button"
                    role="menuitem"
                    aria-expanded="false"
                  >
                    <span class="pf-v5-c-menu__item-main">
                      <span class="pf-v5-c-menu__item-text">Edit access settings</span>
                      <span class="pf-v5-c-menu__item-toggle-icon">
                        <i class="fas fa-angle-right"></i>
                      </span>
                    </span>
                  </button>
                  <div class="pf-v5-c-menu">
                    <div class="pf-v5-c-menu__content">
                      <ul class="pf-v5-c-menu__list" role="menu">
                        <li
                          class="pf-v5-c-menu__list-item pf-m-drill-up"
                          role="none"
                        >
                          <button
                            class="pf-v5-c-menu__item"
                            type="button"
                            role="menuitem"
                          >
                            <span class="pf-v5-c-menu__item-main">
                              <span class="pf-v5-c-menu__item-toggle-icon">
                                <i class="fas fa-angle-left"></i>
                              </span>
                              <span
                                class="pf-v5-c-menu__item-text"
                              >Edit access settings</span>
                            </span>
                          </button>
                        </li>
                        <li class="pf-v5-c-menu__list-item" role="none">
                          <button
                            class="pf-v5-c-menu__item"
                            type="button"
                            role="menuitem"
                          >
                            <span class="pf-v5-c-menu__item-main">
                              <span
                                class="pf-v5-c-menu__item-text"
                              >Global access</span>
                            </span>
                          </button>
                        </li>
                        <li class="pf-v5-c-menu__list-item" role="none">
                          <button
                            class="pf-v5-c-menu__item"
                            type="button"
                            role="menuitem"
                          >
                            <span class="pf-v5-c-menu__item-main">
                              <span
                                class="pf-v5-c-menu__item-text"
                              >Account access</span>
                            </span>
                          </button>
                        </li>
                      </ul>
                    </div>
                  </div>
                </li>
              </ul>
            </div>
          </div>
        </li>
      </ul>
    </div>
  </div>
</nav>

```

### Nav with drilldown menu level three

```html isBeta
<nav class="pf-v5-c-nav" aria-label="Drilldown menu example">
  <div class="pf-v5-c-menu pf-m-drilldown pf-m-drilled-in">
    <div
      class="pf-v5-c-menu__content"
      style="--pf-v5-c-menu__content--Height: 284px;"
    >
      <ul class="pf-v5-c-menu__list" role="menu">
        <li class="pf-v5-c-menu__list-item" role="none">
          <button class="pf-v5-c-menu__item" type="button" role="menuitem">
            <span class="pf-v5-c-menu__item-main">
              <span class="pf-v5-c-menu__item-text">Start rollout</span>
            </span>
          </button>
        </li>
        <li class="pf-v5-c-menu__list-item" role="none">
          <button class="pf-v5-c-menu__item" type="button" role="menuitem">
            <span class="pf-v5-c-menu__item-main">
              <span class="pf-v5-c-menu__item-text">Pause rollout</span>
            </span>
          </button>
        </li>
        <li class="pf-v5-c-menu__list-item" role="none">
          <button
            class="pf-v5-c-menu__item pf-m-current"
            type="button"
            role="menuitem"
          >
            <span class="pf-v5-c-menu__item-main">
              <span class="pf-v5-c-menu__item-text">Current link</span>
            </span>
          </button>
        </li>
        <li class="pf-v5-c-menu__list-item" role="none">
          <button class="pf-v5-c-menu__item" type="button" role="menuitem">
            <span class="pf-v5-c-menu__item-main">
              <span class="pf-v5-c-menu__item-text">Add storage</span>
            </span>
          </button>
        </li>
        <li class="pf-v5-c-menu__list-item pf-m-current-path" role="none">
          <button
            class="pf-v5-c-menu__item"
            type="button"
            role="menuitem"
            aria-expanded="true"
          >
            <span class="pf-v5-c-menu__item-main">
              <span class="pf-v5-c-menu__item-text">Edit</span>
              <span class="pf-v5-c-menu__item-toggle-icon">
                <i class="fas fa-angle-right"></i>
              </span>
            </span>
          </button>
          <div class="pf-v5-c-menu pf-m-drilled-in">
            <div class="pf-v5-c-menu__content">
              <ul class="pf-v5-c-menu__list" role="menu">
                <li class="pf-v5-c-menu__list-item pf-m-drill-up" role="none">
                  <button
                    class="pf-v5-c-menu__item"
                    type="button"
                    role="menuitem"
                    tabindex="0"
                  >
                    <span class="pf-v5-c-menu__item-main">
                      <span class="pf-v5-c-menu__item-toggle-icon">
                        <i class="fas fa-angle-left"></i>
                      </span>
                      <span class="pf-v5-c-menu__item-text">Edit</span>
                    </span>
                  </button>
                </li>
                <li
                  class="pf-v5-c-menu__list-item pf-m-current-path"
                  role="none"
                >
                  <button
                    class="pf-v5-c-menu__item"
                    type="button"
                    role="menuitem"
                    aria-expanded="true"
                  >
                    <span class="pf-v5-c-menu__item-main">
                      <span class="pf-v5-c-menu__item-text">Deployment</span>
                      <span class="pf-v5-c-menu__item-toggle-icon">
                        <i class="fas fa-angle-right"></i>
                      </span>
                    </span>
                  </button>
                  <div class="pf-v5-c-menu">
                    <div class="pf-v5-c-menu__content">
                      <ul class="pf-v5-c-menu__list" role="menu">
                        <li
                          class="pf-v5-c-menu__list-item pf-m-drill-up"
                          role="none"
                        >
                          <button
                            class="pf-v5-c-menu__item"
                            type="button"
                            role="menuitem"
                            tabindex="0"
                          >
                            <span class="pf-v5-c-menu__item-main">
                              <span class="pf-v5-c-menu__item-toggle-icon">
                                <i class="fas fa-angle-left"></i>
                              </span>
                              <span class="pf-v5-c-menu__item-text">Deployment</span>
                            </span>
                          </button>
                        </li>
                        <li class="pf-v5-c-menu__list-item" role="none">
                          <button
                            class="pf-v5-c-menu__item"
                            type="button"
                            role="menuitem"
                          >
                            <span class="pf-v5-c-menu__item-main">
                              <span class="pf-v5-c-menu__item-text">Routes</span>
                            </span>
                          </button>
                        </li>
                        <li class="pf-v5-c-menu__list-item" role="none">
                          <button
                            class="pf-v5-c-menu__item"
                            type="button"
                            role="menuitem"
                          >
                            <span class="pf-v5-c-menu__item-main">
                              <span class="pf-v5-c-menu__item-text">Nodes</span>
                            </span>
                          </button>
                        </li>
                        <li class="pf-v5-c-menu__list-item" role="none">
                          <button
                            class="pf-v5-c-menu__item"
                            type="button"
                            role="menuitem"
                          >
                            <span class="pf-v5-c-menu__item-main">
                              <span class="pf-v5-c-menu__item-text">URLs</span>
                            </span>
                          </button>
                        </li>
                        <li class="pf-v5-c-menu__list-item" role="none">
                          <button
                            class="pf-v5-c-menu__item"
                            type="button"
                            role="menuitem"
                            aria-expanded="false"
                          >
                            <span class="pf-v5-c-menu__item-main">
                              <span
                                class="pf-v5-c-menu__item-text"
                              >Advanced settings</span>
                              <span class="pf-v5-c-menu__item-toggle-icon">
                                <i class="fas fa-angle-right"></i>
                              </span>
                            </span>
                          </button>
                          <div class="pf-v5-c-menu">
                            <div class="pf-v5-c-menu__content">
                              <ul class="pf-v5-c-menu__list" role="menu">
                                <li
                                  class="pf-v5-c-menu__list-item pf-m-drill-up"
                                  role="none"
                                >
                                  <button
                                    class="pf-v5-c-menu__item"
                                    type="button"
                                    role="menuitem"
                                    tabindex="0"
                                  >
                                    <span class="pf-v5-c-menu__item-main">
                                      <span
                                        class="pf-v5-c-menu__item-toggle-icon"
                                      >
                                        <i class="fas fa-angle-left"></i>
                                      </span>
                                      <span
                                        class="pf-v5-c-menu__item-text"
                                      >Advanced settings</span>
                                    </span>
                                  </button>
                                </li>
                                <li class="pf-v5-c-menu__list-item" role="none">
                                  <button
                                    class="pf-v5-c-menu__item"
                                    type="button"
                                    role="menuitem"
                                  >
                                    <span class="pf-v5-c-menu__item-main">
                                      <span
                                        class="pf-v5-c-menu__item-text"
                                      >Reports</span>
                                    </span>
                                  </button>
                                </li>
                                <li class="pf-v5-c-menu__list-item" role="none">
                                  <button
                                    class="pf-v5-c-menu__item"
                                    type="button"
                                    role="menuitem"
                                  >
                                    <span class="pf-v5-c-menu__item-main">
                                      <span
                                        class="pf-v5-c-menu__item-text"
                                      >Policies</span>
                                    </span>
                                  </button>
                                </li>
                                <li class="pf-v5-c-menu__list-item" role="none">
                                  <button
                                    class="pf-v5-c-menu__item"
                                    type="button"
                                    role="menuitem"
                                  >
                                    <span class="pf-v5-c-menu__item-main">
                                      <span
                                        class="pf-v5-c-menu__item-text"
                                      >Systems</span>
                                    </span>
                                  </button>
                                </li>
                              </ul>
                            </div>
                          </div>
                        </li>
                      </ul>
                    </div>
                  </div>
                </li>
                <li class="pf-v5-c-menu__list-item" role="none">
                  <button
                    class="pf-v5-c-menu__item"
                    type="button"
                    role="menuitem"
                    aria-expanded="false"
                  >
                    <span class="pf-v5-c-menu__item-main">
                      <span class="pf-v5-c-menu__item-text">RBAC</span>
                      <span class="pf-v5-c-menu__item-toggle-icon">
                        <i class="fas fa-angle-right"></i>
                      </span>
                    </span>
                  </button>
                  <div class="pf-v5-c-menu">
                    <div class="pf-v5-c-menu__content">
                      <ul class="pf-v5-c-menu__list" role="menu">
                        <li
                          class="pf-v5-c-menu__list-item pf-m-drill-up"
                          role="none"
                        >
                          <button
                            class="pf-v5-c-menu__item"
                            type="button"
                            role="menuitem"
                          >
                            <span class="pf-v5-c-menu__item-main">
                              <span class="pf-v5-c-menu__item-toggle-icon">
                                <i class="fas fa-angle-left"></i>
                              </span>
                              <span class="pf-v5-c-menu__item-text">RBAC</span>
                            </span>
                          </button>
                        </li>
                        <li class="pf-v5-c-menu__list-item" role="none">
                          <button
                            class="pf-v5-c-menu__item"
                            type="button"
                            role="menuitem"
                          >
                            <span class="pf-v5-c-menu__item-main">
                              <span class="pf-v5-c-menu__item-text">Reports</span>
                            </span>
                          </button>
                        </li>
                        <li class="pf-v5-c-menu__list-item" role="none">
                          <button
                            class="pf-v5-c-menu__item"
                            type="button"
                            role="menuitem"
                          >
                            <span class="pf-v5-c-menu__item-main">
                              <span class="pf-v5-c-menu__item-text">Policies</span>
                            </span>
                          </button>
                        </li>
                        <li class="pf-v5-c-menu__list-item" role="none">
                          <button
                            class="pf-v5-c-menu__item"
                            type="button"
                            role="menuitem"
                          >
                            <span class="pf-v5-c-menu__item-main">
                              <span class="pf-v5-c-menu__item-text">Systems</span>
                            </span>
                          </button>
                        </li>
                      </ul>
                    </div>
                  </div>
                </li>
                <li class="pf-v5-c-menu__list-item" role="none">
                  <button
                    class="pf-v5-c-menu__item"
                    type="button"
                    role="menuitem"
                  >
                    <span class="pf-v5-c-menu__item-main">
                      <span
                        class="pf-v5-c-menu__item-text"
                      >Thing with a longer label</span>
                    </span>
                  </button>
                </li>
              </ul>
            </div>
          </div>
        </li>
        <li class="pf-v5-c-menu__list-item" role="none">
          <button
            class="pf-v5-c-menu__item"
            type="button"
            role="menuitem"
            aria-expanded="false"
          >
            <span class="pf-v5-c-menu__item-main">
              <span class="pf-v5-c-menu__item-text">Configuration</span>
              <span class="pf-v5-c-menu__item-toggle-icon">
                <i class="fas fa-angle-right"></i>
              </span>
            </span>
          </button>
          <div class="pf-v5-c-menu">
            <div class="pf-v5-c-menu__content">
              <ul class="pf-v5-c-menu__list" role="menu">
                <li class="pf-v5-c-menu__list-item pf-m-drill-up" role="none">
                  <button
                    class="pf-v5-c-menu__item"
                    type="button"
                    role="menuitem"
                  >
                    <span class="pf-v5-c-menu__item-main">
                      <span class="pf-v5-c-menu__item-toggle-icon">
                        <i class="fas fa-angle-left"></i>
                      </span>
                      <span class="pf-v5-c-menu__item-text">Configuration</span>
                    </span>
                  </button>
                </li>
                <li class="pf-v5-c-menu__list-item" role="none">
                  <button
                    class="pf-v5-c-menu__item"
                    type="button"
                    role="menuitem"
                    aria-expanded="false"
                  >
                    <span class="pf-v5-c-menu__item-main">
                      <span class="pf-v5-c-menu__item-text">Profile</span>
                      <span class="pf-v5-c-menu__item-toggle-icon">
                        <i class="fas fa-angle-right"></i>
                      </span>
                    </span>
                  </button>
                  <div class="pf-v5-c-menu">
                    <div class="pf-v5-c-menu__content">
                      <ul class="pf-v5-c-menu__list" role="menu">
                        <li
                          class="pf-v5-c-menu__list-item pf-m-drill-up"
                          role="none"
                        >
                          <button
                            class="pf-v5-c-menu__item"
                            type="button"
                            role="menuitem"
                          >
                            <span class="pf-v5-c-menu__item-main">
                              <span class="pf-v5-c-menu__item-toggle-icon">
                                <i class="fas fa-angle-left"></i>
                              </span>
                              <span class="pf-v5-c-menu__item-text">Profile</span>
                            </span>
                          </button>
                        </li>
                        <li class="pf-v5-c-menu__list-item" role="none">
                          <button
                            class="pf-v5-c-menu__item"
                            type="button"
                            role="menuitem"
                          >
                            <span class="pf-v5-c-menu__item-main">
                              <span class="pf-v5-c-menu__item-text">Avatar</span>
                            </span>
                          </button>
                        </li>
                        <li class="pf-v5-c-menu__list-item" role="none">
                          <button
                            class="pf-v5-c-menu__item"
                            type="button"
                            role="menuitem"
                          >
                            <span class="pf-v5-c-menu__item-main">
                              <span class="pf-v5-c-menu__item-text">Name</span>
                            </span>
                          </button>
                        </li>
                        <li class="pf-v5-c-menu__list-item" role="none">
                          <button
                            class="pf-v5-c-menu__item"
                            type="button"
                            role="menuitem"
                          >
                            <span class="pf-v5-c-menu__item-main">
                              <span class="pf-v5-c-menu__item-text">Title</span>
                            </span>
                          </button>
                        </li>
                      </ul>
                    </div>
                  </div>
                </li>
                <li class="pf-v5-c-menu__list-item" role="none">
                  <button
                    class="pf-v5-c-menu__item"
                    type="button"
                    role="menuitem"
                  >
                    <span class="pf-v5-c-menu__item-main">
                      <span class="pf-v5-c-menu__item-text">Time zone</span>
                    </span>
                  </button>
                </li>
                <li class="pf-v5-c-menu__list-item" role="none">
                  <button
                    class="pf-v5-c-menu__item"
                    type="button"
                    role="menuitem"
                  >
                    <span class="pf-v5-c-menu__item-main">
                      <span class="pf-v5-c-menu__item-text">Account settings</span>
                    </span>
                  </button>
                </li>
                <li class="pf-v5-c-menu__list-item" role="none">
                  <button
                    class="pf-v5-c-menu__item"
                    type="button"
                    role="menuitem"
                  >
                    <span class="pf-v5-c-menu__item-main">
                      <span
                        class="pf-v5-c-menu__item-text"
                      >Thing with a longer label</span>
                    </span>
                  </button>
                </li>
                <li class="pf-v5-c-menu__list-item" role="none">
                  <button
                    class="pf-v5-c-menu__item"
                    type="button"
                    role="menuitem"
                    aria-expanded="false"
                  >
                    <span class="pf-v5-c-menu__item-main">
                      <span class="pf-v5-c-menu__item-text">Edit access settings</span>
                      <span class="pf-v5-c-menu__item-toggle-icon">
                        <i class="fas fa-angle-right"></i>
                      </span>
                    </span>
                  </button>
                  <div class="pf-v5-c-menu">
                    <div class="pf-v5-c-menu__content">
                      <ul class="pf-v5-c-menu__list" role="menu">
                        <li
                          class="pf-v5-c-menu__list-item pf-m-drill-up"
                          role="none"
                        >
                          <button
                            class="pf-v5-c-menu__item"
                            type="button"
                            role="menuitem"
                          >
                            <span class="pf-v5-c-menu__item-main">
                              <span class="pf-v5-c-menu__item-toggle-icon">
                                <i class="fas fa-angle-left"></i>
                              </span>
                              <span
                                class="pf-v5-c-menu__item-text"
                              >Edit access settings</span>
                            </span>
                          </button>
                        </li>
                        <li class="pf-v5-c-menu__list-item" role="none">
                          <button
                            class="pf-v5-c-menu__item"
                            type="button"
                            role="menuitem"
                          >
                            <span class="pf-v5-c-menu__item-main">
                              <span
                                class="pf-v5-c-menu__item-text"
                              >Global access</span>
                            </span>
                          </button>
                        </li>
                        <li class="pf-v5-c-menu__list-item" role="none">
                          <button
                            class="pf-v5-c-menu__item"
                            type="button"
                            role="menuitem"
                          >
                            <span class="pf-v5-c-menu__item-main">
                              <span
                                class="pf-v5-c-menu__item-text"
                              >Account access</span>
                            </span>
                          </button>
                        </li>
                      </ul>
                    </div>
                  </div>
                </li>
              </ul>
            </div>
          </div>
        </li>
      </ul>
    </div>
  </div>
</nav>

```

### Nav link text

When using anything other than a text node for the link text, wrap the link text in an element with `.pf-v5-c-nav__link-text`.

```html isBeta
<nav class="pf-v5-c-nav" aria-label="Global">
  <ul class="pf-v5-c-nav__list" role="list">
    <li class="pf-v5-c-nav__item">
      <a href="#" class="pf-v5-c-nav__link">
        <span class="pf-v5-c-nav__link-text">
          Link 1
          <i class="fas fa-arrow-right" aria-hidden="true"></i>
        </span>
      </a>
    </li>
    <li class="pf-v5-c-nav__item pf-m-expandable pf-m-expanded">
      <button
        class="pf-v5-c-nav__link"
        id="nav-link-text-link2"
        aria-expanded="true"
      >
        <span class="pf-v5-c-nav__link-text">
          Link 2
          <small>(small text)</small>
        </span>
        <span class="pf-v5-c-nav__toggle">
          <span class="pf-v5-c-nav__toggle-icon">
            <i class="fas fa-angle-right" aria-hidden="true"></i>
          </span>
        </span>
      </button>
      <section
        class="pf-v5-c-nav__subnav"
        aria-labelledby="nav-link-text-link2"
      >
        <ul class="pf-v5-c-nav__list" role="list">
          <li class="pf-v5-c-nav__item">
            <a href="#" class="pf-v5-c-nav__link">
              <span class="pf-v5-c-nav__link-text">
                <i class="fas fa-user" aria-hidden="true"></i>
                Subnav link 1
              </span>
            </a>
          </li>
          <li class="pf-v5-c-nav__item">
            <a href="#" class="pf-v5-c-nav__link">
              <span class="pf-v5-c-nav__link-text">
                <i class="fas fa-user" aria-hidden="true"></i>
                Subnav link 2
              </span>
            </a>
          </li>
        </ul>
      </section>
    </li>
    <li class="pf-v5-c-nav__item pf-m-expandable pf-m-current">
      <button
        class="pf-v5-c-nav__link"
        id="nav-link-text-link4"
        aria-expanded="false"
      >
        <span class="pf-v5-c-nav__link-text">
          Link 3
          <strong>(strong text)</strong>
        </span>
        <span class="pf-v5-c-nav__toggle">
          <span class="pf-v5-c-nav__toggle-icon">
            <i class="fas fa-angle-right" aria-hidden="true"></i>
          </span>
        </span>
      </button>
      <section
        class="pf-v5-c-nav__subnav"
        aria-labelledby="nav-link-text-link4"
        hidden
      >
        <ul class="pf-v5-c-nav__list" role="list">
          <li class="pf-v5-c-nav__item">
            <a href="#" class="pf-v5-c-nav__link">Subnav link 1</a>
          </li>
          <li class="pf-v5-c-nav__item">
            <a
              href="#"
              class="pf-v5-c-nav__link pf-m-current"
              aria-current="page"
            >Subnav link 2</a>
          </li>
          <li class="pf-v5-c-nav__item">
            <a href="#" class="pf-v5-c-nav__link">Subnav link 3</a>
          </li>
        </ul>
      </section>
    </li>
  </ul>
</nav>

```

## Documentation

### Overview

The navigation system relies on several different sub-components:

*   `.pf-v5-c-nav__list` - default navigation list. It is the basis for both default and expandable, vertical navigation.

### Accessibility

| Attribute | Applied to | Outcome |
| -- | -- | -- |
| `aria-label="[landmark description]"` | `.pf-v5-c-nav` |  Describes `<nav>` landmark. |
| `aria-label="[section title]"` | `.pf-v5-c-nav__section` |  Describes a nav `<section>`, where a `.pf-v5-c-nav__section-title` is not present. |
| `aria-labelledby="[id value of link describing subnav]"` | `.pf-v5-c-nav__subnav` |  Gives the subnav `<section>` landmark an accessible name by referring to the element that provides the subnav `<section>` landmark title. |
| `aria-expanded="false"` | `.pf-v5-c-nav__link` |  Indicates that subnav section is hidden. |
| `aria-expanded="true"` | `.pf-v5-c-nav__link` |  Indicates that subnav section is visible. |
| `hidden` | `.pf-v5-c-nav__subnav` |  Indicates that the subnav section is hidden so that it isn't visible in the UI and isn't accessed by assistive technologies. |
| `disabled` | `.pf-v5-c-nav__scroll-button` | Indicates that a scroll button is disabled, when at the first or last item of a list. **Required when disabled** |
| `aria-current="page"` | `.pf-v5-c-nav__link` |  Indicates the current page link. Can only occur once on page. |
| `aria-haspopup="true"` | `.pf-v5-c-nav__link` | Declares that a nav item has a submenu. |

### Usage

| Class | Applied to | Outcome |
| -- | -- | -- |
| `.pf-v5-c-nav` | `<nav>` | Initiates a primary nav element. |
| `.pf-v5-c-nav__subnav` | `<section>` | Initiates a subnav section. |
| `.pf-v5-c-nav__list` | `<ul>` | Initiates nav list. |
| `.pf-v5-c-nav__item` | `<li>` | Initiates nav list item. |
| `.pf-v5-c-nav__link` | `<a>` | Initiates nav list link. |
| `.pf-v5-c-nav__link-text` | `<span>` | Initiates nav list link text. |
| `.pf-v5-c-nav__section` | `<section>` | Initiates a nav section element. |
| `.pf-v5-c-nav__section-title` | `<h1>`, `<h2>`, `<h3>`, `<h4>`, `<h5>`, `<h6>` | Initiates a nav section title. |
| `.pf-v5-c-nav__toggle` | `<span>` | Initiates the nav toggle wrapper. |
| `.pf-v5-c-nav__toggle-icon` | `<span>` | Initiates a nav toggle icon wrapper. |
| `.pf-v5-c-nav__scroll-button` | `<button>` | Initiates a nav scroll button. **Required for horizontal navs** |
| `.pf-m-horizontal` | `.pf-v5-c-nav` | Modifies nav for the horizontal variation. |
| `.pf-m-horizontal-subnav` | `.pf-v5-c-nav` | Modifies nav for the horizontal subnav variation. |
| `.pf-m-tertiary` | `.pf-v5-c-nav` | Modifies nav for the tertiary variation. |
| `.pf-m-light` | `.pf-v5-c-nav` | Modifies nav for the light variation. **Note: only for use with vertical navs, and requires `.pf-m-light` on the page component's sidebar element (`.pf-v5-c-page__sidebar`)**. |
| `.pf-m-flyout` | `.pf-v5-c-nav__item` | Modifies nav item for the flyout variation. |
| `.pf-m-scrollable` | `.pf-v5-c-nav` | Modifies nav for the scrollable state. |
| `.pf-m-expandable` | `.pf-v5-c-nav__item` | Modifies for the expandable state. |
| `.pf-m-expanded` | `.pf-v5-c-nav__item` | Modifies for the expanded state. |
| `.pf-m-current` | `.pf-v5-c-nav__link` | Modifies for the current state. |
| `.pf-m-hover` | `.pf-v5-c-nav__link` | Modifies for the hover state. |
| `.pf-m-start` | `.pf-v5-c-nav__toggle` | Modifies nav toggle to align left. |
