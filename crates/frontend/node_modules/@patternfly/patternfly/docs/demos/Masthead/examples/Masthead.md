---
id: Masthead
section: components
wrapperTag: div
---## Examples

### Basic

```html isFullscreen
<div class="pf-v5-c-page" id="masthead-basic-example">
  <div class="pf-v5-c-skip-to-content">
    <a
      class="pf-v5-c-button pf-m-primary"
      href="#main-content-masthead-basic-example"
    >Skip to content</a>
  </div>
  <header class="pf-v5-c-masthead" id="masthead-basic-example-masthead">
    <span class="pf-v5-c-masthead__toggle">
      <button
        class="pf-v5-c-button pf-m-plain"
        type="button"
        aria-label="Global navigation"
      >
        <i class="fas fa-bars" aria-hidden="true"></i>
      </button>
    </span>
    <div class="pf-v5-c-masthead__main">
      <a class="pf-v5-c-masthead__brand" href="#">
        <img
          class="pf-v5-c-brand"
          src="/assets/images/pf-logo.svg"
          alt="PatternFly logo"
          style="--pf-v5-c-brand--Height:36px"
        />
      </a>
    </div>
    <div class="pf-v5-c-masthead__content">
      <div
        class="pf-v5-c-toolbar pf-m-full-height pf-m-static"
        id="masthead-basic-example-masthead-toolbar"
      >
        <div class="pf-v5-c-toolbar__content">
          <div class="pf-v5-c-toolbar__content-section">
            <div class="pf-v5-c-toolbar__group pf-m-align-right">
              <div class="pf-v5-c-toolbar__item">
                <button
                  class="pf-v5-c-menu-toggle pf-m-plain"
                  type="button"
                  aria-expanded="false"
                  aria-label="Actions"
                >
                  <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </header>
  <div class="pf-v5-c-page__sidebar">
    <div class="pf-v5-c-page__sidebar-body">
      <nav
        class="pf-v5-c-nav"
        id="masthead-basic-example-primary-nav"
        aria-label="Global"
      >
        <ul class="pf-v5-c-nav__list" role="list">
          <li class="pf-v5-c-nav__item">
            <a href="#" class="pf-v5-c-nav__link">System panel</a>
          </li>
          <li class="pf-v5-c-nav__item">
            <a
              href="#"
              class="pf-v5-c-nav__link pf-m-current"
              aria-current="page"
            >Policy</a>
          </li>
          <li class="pf-v5-c-nav__item">
            <a href="#" class="pf-v5-c-nav__link">Authentication</a>
          </li>
          <li class="pf-v5-c-nav__item">
            <a href="#" class="pf-v5-c-nav__link">Network services</a>
          </li>
          <li class="pf-v5-c-nav__item">
            <a href="#" class="pf-v5-c-nav__link">Server</a>
          </li>
        </ul>
      </nav>
    </div>
  </div>
  <main
    class="pf-v5-c-page__main"
    tabindex="-1"
    id="main-content-masthead-basic-example"
  >
    <section class="pf-v5-c-page__main-breadcrumb pf-m-limit-width">
      <div class="pf-v5-c-page__main-body">
        <nav class="pf-v5-c-breadcrumb" aria-label="breadcrumb">
          <ol class="pf-v5-c-breadcrumb__list" role="list">
            <li class="pf-v5-c-breadcrumb__item">
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
      </div>
    </section>
    <section class="pf-v5-c-page__main-section pf-m-limit-width pf-m-light">
      <div class="pf-v5-c-page__main-body">
        <div class="pf-v5-c-content">
          <h1>Main title</h1>
          <p>This is a full page demo.</p>
        </div>
      </div>
    </section>
    <section class="pf-v5-c-page__main-section pf-m-limit-width">
      <div class="pf-v5-c-page__main-body">
        <div class="pf-v5-l-gallery pf-m-gutter">
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
        </div>
      </div>
    </section>
  </main>
</div>

```

### With context selector and dropdown

```html isFullscreen
<div class="pf-v5-c-page" id="masthead-context-selecton-drilldown-example">
  <div class="pf-v5-c-skip-to-content">
    <a
      class="pf-v5-c-button pf-m-primary"
      href="#main-content-masthead-context-selecton-drilldown-example"
    >Skip to content</a>
  </div>
  <header
    class="pf-v5-c-masthead"
    id="masthead-context-selecton-drilldown-example-masthead"
  >
    <span class="pf-v5-c-masthead__toggle">
      <button
        class="pf-v5-c-button pf-m-plain"
        type="button"
        aria-label="Global navigation"
      >
        <i class="fas fa-bars" aria-hidden="true"></i>
      </button>
    </span>
    <div class="pf-v5-c-masthead__main">
      <a class="pf-v5-c-masthead__brand" href="#">
        <img
          class="pf-v5-c-brand"
          src="/assets/images/pf-logo.svg"
          alt="PatternFly logo"
          style="--pf-v5-c-brand--Height:36px"
        />
      </a>
    </div>
    <div class="pf-v5-c-masthead__content">
      <div
        class="pf-v5-c-toolbar pf-m-full-height pf-m-static"
        id="masthead-context-selecton-drilldown-example-masthead-toolbar"
      >
        <div class="pf-v5-c-toolbar__content">
          <div class="pf-v5-c-toolbar__content-section">
            <div class="pf-v5-c-toolbar__group pf-m-filter-group">
              <div class="pf-v5-c-toolbar__item">
                <button
                  class="pf-v5-c-menu-toggle pf-m-full-height"
                  type="button"
                  aria-expanded="false"
                >
                  <span class="pf-v5-c-menu-toggle__text">Context selector</span>
                  <span class="pf-v5-c-menu-toggle__controls">
                    <span class="pf-v5-c-menu-toggle__toggle-icon">
                      <i class="fas fa-caret-down" aria-hidden="true"></i>
                    </span>
                  </span>
                </button>
              </div>
              <div class="pf-v5-c-toolbar__item pf-m-hidden pf-m-visible-on-lg">
                <button
                  class="pf-v5-c-menu-toggle pf-m-full-height"
                  type="button"
                  aria-expanded="false"
                >
                  <span class="pf-v5-c-menu-toggle__text">Dropdown</span>
                  <span class="pf-v5-c-menu-toggle__controls">
                    <span class="pf-v5-c-menu-toggle__toggle-icon">
                      <i class="fas fa-caret-down" aria-hidden="true"></i>
                    </span>
                  </span>
                </button>
              </div>
            </div>
            <div class="pf-v5-c-toolbar__item pf-m-align-right">
              <button
                class="pf-v5-c-menu-toggle pf-m-plain"
                type="button"
                aria-expanded="false"
                aria-label="Actions"
              >
                <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  </header>
  <div class="pf-v5-c-page__sidebar">
    <div class="pf-v5-c-page__sidebar-body">
      <nav
        class="pf-v5-c-nav"
        id="masthead-context-selecton-drilldown-example-primary-nav"
        aria-label="Global"
      >
        <ul class="pf-v5-c-nav__list" role="list">
          <li class="pf-v5-c-nav__item">
            <a href="#" class="pf-v5-c-nav__link">System panel</a>
          </li>
          <li class="pf-v5-c-nav__item">
            <a
              href="#"
              class="pf-v5-c-nav__link pf-m-current"
              aria-current="page"
            >Policy</a>
          </li>
          <li class="pf-v5-c-nav__item">
            <a href="#" class="pf-v5-c-nav__link">Authentication</a>
          </li>
          <li class="pf-v5-c-nav__item">
            <a href="#" class="pf-v5-c-nav__link">Network services</a>
          </li>
          <li class="pf-v5-c-nav__item">
            <a href="#" class="pf-v5-c-nav__link">Server</a>
          </li>
        </ul>
      </nav>
    </div>
  </div>
  <main
    class="pf-v5-c-page__main"
    tabindex="-1"
    id="main-content-masthead-context-selecton-drilldown-example"
  >
    <section class="pf-v5-c-page__main-breadcrumb pf-m-limit-width">
      <div class="pf-v5-c-page__main-body">
        <nav class="pf-v5-c-breadcrumb" aria-label="breadcrumb">
          <ol class="pf-v5-c-breadcrumb__list" role="list">
            <li class="pf-v5-c-breadcrumb__item">
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
      </div>
    </section>
    <section class="pf-v5-c-page__main-section pf-m-limit-width pf-m-light">
      <div class="pf-v5-c-page__main-body">
        <div class="pf-v5-c-content">
          <h1>Main title</h1>
          <p>This is a full page demo.</p>
        </div>
      </div>
    </section>
    <section class="pf-v5-c-page__main-section pf-m-limit-width">
      <div class="pf-v5-c-page__main-body">
        <div class="pf-v5-l-gallery pf-m-gutter">
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
        </div>
      </div>
    </section>
  </main>
</div>

```

### With toolbar, filters

```html isFullscreen
<div class="pf-v5-c-page" id="masthead-toolbar-filters-example">
  <div class="pf-v5-c-skip-to-content">
    <a
      class="pf-v5-c-button pf-m-primary"
      href="#main-content-masthead-toolbar-filters-example"
    >Skip to content</a>
  </div>
  <header
    class="pf-v5-c-masthead"
    id="masthead-toolbar-filters-example-masthead"
  >
    <span class="pf-v5-c-masthead__toggle">
      <button
        class="pf-v5-c-button pf-m-plain"
        type="button"
        aria-label="Global navigation"
      >
        <i class="fas fa-bars" aria-hidden="true"></i>
      </button>
    </span>
    <div class="pf-v5-c-masthead__main">
      <a class="pf-v5-c-masthead__brand" href="#">
        <img
          class="pf-v5-c-brand"
          src="/assets/images/pf-logo.svg"
          alt="PatternFly logo"
          style="--pf-v5-c-brand--Height:36px"
        />
      </a>
    </div>
    <div class="pf-v5-c-masthead__content">
      <div
        class="pf-v5-c-toolbar pf-m-full-height pf-m-static"
        id="masthead-toolbar-filters-example-masthead-toolbar"
      >
        <div class="pf-v5-c-toolbar__content">
          <div class="pf-v5-c-toolbar__content-section">
            <div
              class="pf-v5-c-toolbar__group pf-m-toggle-group pf-m-show pf-m-align-right"
            >
              <div class="pf-v5-c-toolbar__toggle">
                <button
                  class="pf-v5-c-button pf-m-plain"
                  type="button"
                  aria-label="Show filters"
                  aria-expanded="true"
                  aria-controls="masthead-toolbar-filters-example-masthead-toolbar-expandable-content"
                >
                  <i class="fas fa-filter" aria-hidden="true"></i>
                </button>
              </div>
              <div class="pf-v5-c-toolbar__item pf-m-search-filter">
                <div
                  class="pf-v5-c-input-group"
                  aria-label="search filter"
                  role="group"
                >
                  <div class="pf-v5-c-input-group__item">
                    <div class="pf-v5-c-select" style="width: 124px">
                      <span
                        id="masthead-toolbar-filters-example-masthead-toolbar-select-name-label"
                        hidden
                      >Choose one</span>

                      <button
                        class="pf-v5-c-select__toggle"
                        type="button"
                        id="masthead-toolbar-filters-example-masthead-toolbar-select-name-toggle"
                        aria-haspopup="true"
                        aria-expanded="false"
                        aria-labelledby="masthead-toolbar-filters-example-masthead-toolbar-select-name-label masthead-toolbar-filters-example-masthead-toolbar-select-name-toggle"
                      >
                        <div class="pf-v5-c-select__toggle-wrapper">
                          <span class="pf-v5-c-select__toggle-icon">
                            <i class="fas fa-filter" aria-hidden="true"></i>
                          </span>
                          <span class="pf-v5-c-select__toggle-text">Name</span>
                        </div>
                        <span class="pf-v5-c-select__toggle-arrow">
                          <i class="fas fa-caret-down" aria-hidden="true"></i>
                        </span>
                      </button>

                      <ul
                        class="pf-v5-c-select__menu"
                        role="listbox"
                        aria-labelledby="masthead-toolbar-filters-example-masthead-toolbar-select-name-label"
                        hidden
                      >
                        <li role="presentation">
                          <button
                            class="pf-v5-c-select__menu-item"
                            role="option"
                          >Running</button>
                        </li>
                        <li role="presentation">
                          <button
                            class="pf-v5-c-select__menu-item pf-m-selected"
                            role="option"
                            aria-selected="true"
                          >
                            Stopped
                            <span class="pf-v5-c-select__menu-item-icon">
                              <i class="fas fa-check" aria-hidden="true"></i>
                            </span>
                          </button>
                        </li>
                        <li role="presentation">
                          <button
                            class="pf-v5-c-select__menu-item"
                            role="option"
                          >Down</button>
                        </li>
                        <li role="presentation">
                          <button
                            class="pf-v5-c-select__menu-item"
                            role="option"
                          >Degraded</button>
                        </li>
                        <li role="presentation">
                          <button
                            class="pf-v5-c-select__menu-item"
                            role="option"
                          >Needs maintenance</button>
                        </li>
                      </ul>
                    </div>
                  </div>
                  <div class="pf-v5-c-input-group__item pf-m-fill">
                    <div class="pf-v5-c-text-input-group">
                      <div class="pf-v5-c-text-input-group__main pf-m-icon">
                        <span class="pf-v5-c-text-input-group__text">
                          <span class="pf-v5-c-text-input-group__icon">
                            <i class="fas fa-fw fa-search"></i>
                          </span>
                          <input
                            class="pf-v5-c-text-input-group__text-input"
                            type="text"
                            placeholder="Filter by name"
                            value
                            aria-label="Search input"
                          />
                        </span>
                      </div>
                    </div>
                  </div>
                </div>
              </div>
            </div>
            <div class="pf-v5-c-toolbar__item">
              <button
                class="pf-v5-c-menu-toggle pf-m-plain"
                type="button"
                aria-expanded="false"
                aria-label="Actions"
              >
                <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
              </button>
            </div>
          </div>
          <div
            class="pf-v5-c-toolbar__expandable-content pf-m-hidden"
            id="masthead-toolbar-filters-example-masthead-toolbar-expandable-content"
            hidden
          ></div>
        </div>
      </div>
    </div>
  </header>
  <div class="pf-v5-c-page__sidebar">
    <div class="pf-v5-c-page__sidebar-body">
      <nav
        class="pf-v5-c-nav"
        id="masthead-toolbar-filters-example-primary-nav"
        aria-label="Global"
      >
        <ul class="pf-v5-c-nav__list" role="list">
          <li class="pf-v5-c-nav__item">
            <a href="#" class="pf-v5-c-nav__link">System panel</a>
          </li>
          <li class="pf-v5-c-nav__item">
            <a
              href="#"
              class="pf-v5-c-nav__link pf-m-current"
              aria-current="page"
            >Policy</a>
          </li>
          <li class="pf-v5-c-nav__item">
            <a href="#" class="pf-v5-c-nav__link">Authentication</a>
          </li>
          <li class="pf-v5-c-nav__item">
            <a href="#" class="pf-v5-c-nav__link">Network services</a>
          </li>
          <li class="pf-v5-c-nav__item">
            <a href="#" class="pf-v5-c-nav__link">Server</a>
          </li>
        </ul>
      </nav>
    </div>
  </div>
  <main
    class="pf-v5-c-page__main"
    tabindex="-1"
    id="main-content-masthead-toolbar-filters-example"
  >
    <section class="pf-v5-c-page__main-breadcrumb pf-m-limit-width">
      <div class="pf-v5-c-page__main-body">
        <nav class="pf-v5-c-breadcrumb" aria-label="breadcrumb">
          <ol class="pf-v5-c-breadcrumb__list" role="list">
            <li class="pf-v5-c-breadcrumb__item">
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
      </div>
    </section>
    <section class="pf-v5-c-page__main-section pf-m-limit-width pf-m-light">
      <div class="pf-v5-c-page__main-body">
        <div class="pf-v5-c-content">
          <h1>Main title</h1>
          <p>This is a full page demo.</p>
        </div>
      </div>
    </section>
    <section class="pf-v5-c-page__main-section pf-m-limit-width">
      <div class="pf-v5-c-page__main-body">
        <div class="pf-v5-l-gallery pf-m-gutter">
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
        </div>
      </div>
    </section>
  </main>
</div>

```

### With toggle group and filters

```html isFullscreen
<div class="pf-v5-c-page" id="masthead-toggle-group-filters-example">
  <div class="pf-v5-c-skip-to-content">
    <a
      class="pf-v5-c-button pf-m-primary"
      href="#main-content-masthead-toggle-group-filters-example"
    >Skip to content</a>
  </div>
  <header
    class="pf-v5-c-masthead"
    id="masthead-toggle-group-filters-example-masthead"
  >
    <span class="pf-v5-c-masthead__toggle">
      <button
        class="pf-v5-c-button pf-m-plain"
        type="button"
        aria-label="Global navigation"
      >
        <i class="fas fa-bars" aria-hidden="true"></i>
      </button>
    </span>
    <div class="pf-v5-c-masthead__main">
      <a class="pf-v5-c-masthead__brand" href="#">
        <img
          class="pf-v5-c-brand"
          src="/assets/images/pf-logo.svg"
          alt="PatternFly logo"
          style="--pf-v5-c-brand--Height:36px"
        />
      </a>
    </div>
    <div class="pf-v5-c-masthead__content">
      <div
        class="pf-v5-c-toolbar pf-m-full-height pf-m-static"
        id="masthead-toggle-group-filters-example-masthead-toolbar"
      >
        <div class="pf-v5-c-toolbar__content">
          <div class="pf-v5-c-toolbar__content-section">
            <div
              class="pf-v5-c-toolbar__group pf-m-toggle-group pf-m-show-on-lg pf-m-align-right"
            >
              <div class="pf-v5-c-toolbar__toggle">
                <button
                  class="pf-v5-c-button pf-m-plain"
                  type="button"
                  aria-label="Show filters"
                  aria-expanded="true"
                  aria-controls="masthead-toggle-group-filters-example-masthead-toolbar-expandable-content"
                >
                  <i class="fas fa-filter" aria-hidden="true"></i>
                </button>
              </div>
              <div class="pf-v5-c-toolbar__item pf-m-search-filter">
                <div
                  class="pf-v5-c-input-group"
                  aria-label="search filter"
                  role="group"
                >
                  <div class="pf-v5-c-input-group__item">
                    <div class="pf-v5-c-select" style="width: 124px">
                      <span
                        id="masthead-toggle-group-filters-example-masthead-toolbar-select-name-label"
                        hidden
                      >Choose one</span>

                      <button
                        class="pf-v5-c-select__toggle"
                        type="button"
                        id="masthead-toggle-group-filters-example-masthead-toolbar-select-name-toggle"
                        aria-haspopup="true"
                        aria-expanded="false"
                        aria-labelledby="masthead-toggle-group-filters-example-masthead-toolbar-select-name-label masthead-toggle-group-filters-example-masthead-toolbar-select-name-toggle"
                      >
                        <div class="pf-v5-c-select__toggle-wrapper">
                          <span class="pf-v5-c-select__toggle-icon">
                            <i class="fas fa-filter" aria-hidden="true"></i>
                          </span>
                          <span class="pf-v5-c-select__toggle-text">Name</span>
                        </div>
                        <span class="pf-v5-c-select__toggle-arrow">
                          <i class="fas fa-caret-down" aria-hidden="true"></i>
                        </span>
                      </button>

                      <ul
                        class="pf-v5-c-select__menu"
                        role="listbox"
                        aria-labelledby="masthead-toggle-group-filters-example-masthead-toolbar-select-name-label"
                        hidden
                      >
                        <li role="presentation">
                          <button
                            class="pf-v5-c-select__menu-item"
                            role="option"
                          >Running</button>
                        </li>
                        <li role="presentation">
                          <button
                            class="pf-v5-c-select__menu-item pf-m-selected"
                            role="option"
                            aria-selected="true"
                          >
                            Stopped
                            <span class="pf-v5-c-select__menu-item-icon">
                              <i class="fas fa-check" aria-hidden="true"></i>
                            </span>
                          </button>
                        </li>
                        <li role="presentation">
                          <button
                            class="pf-v5-c-select__menu-item"
                            role="option"
                          >Down</button>
                        </li>
                        <li role="presentation">
                          <button
                            class="pf-v5-c-select__menu-item"
                            role="option"
                          >Degraded</button>
                        </li>
                        <li role="presentation">
                          <button
                            class="pf-v5-c-select__menu-item"
                            role="option"
                          >Needs maintenance</button>
                        </li>
                      </ul>
                    </div>
                  </div>
                  <div class="pf-v5-c-input-group__item pf-m-fill">
                    <div class="pf-v5-c-text-input-group">
                      <div class="pf-v5-c-text-input-group__main pf-m-icon">
                        <span class="pf-v5-c-text-input-group__text">
                          <span class="pf-v5-c-text-input-group__icon">
                            <i class="fas fa-fw fa-search"></i>
                          </span>
                          <input
                            class="pf-v5-c-text-input-group__text-input"
                            type="text"
                            placeholder="Filter by name"
                            value
                            aria-label="Search input"
                          />
                        </span>
                      </div>
                    </div>
                  </div>
                </div>
              </div>
            </div>
            <div class="pf-v5-c-toolbar__item">
              <button
                class="pf-v5-c-menu-toggle pf-m-plain"
                type="button"
                aria-expanded="false"
                aria-label="Actions"
              >
                <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
              </button>
            </div>
          </div>
          <div
            class="pf-v5-c-toolbar__expandable-content pf-m-hidden"
            id="masthead-toggle-group-filters-example-masthead-toolbar-expandable-content"
            hidden
          ></div>
        </div>
      </div>
    </div>
  </header>
  <div class="pf-v5-c-page__sidebar">
    <div class="pf-v5-c-page__sidebar-body">
      <nav
        class="pf-v5-c-nav"
        id="masthead-toggle-group-filters-example-primary-nav"
        aria-label="Global"
      >
        <ul class="pf-v5-c-nav__list" role="list">
          <li class="pf-v5-c-nav__item">
            <a href="#" class="pf-v5-c-nav__link">System panel</a>
          </li>
          <li class="pf-v5-c-nav__item">
            <a
              href="#"
              class="pf-v5-c-nav__link pf-m-current"
              aria-current="page"
            >Policy</a>
          </li>
          <li class="pf-v5-c-nav__item">
            <a href="#" class="pf-v5-c-nav__link">Authentication</a>
          </li>
          <li class="pf-v5-c-nav__item">
            <a href="#" class="pf-v5-c-nav__link">Network services</a>
          </li>
          <li class="pf-v5-c-nav__item">
            <a href="#" class="pf-v5-c-nav__link">Server</a>
          </li>
        </ul>
      </nav>
    </div>
  </div>
  <main
    class="pf-v5-c-page__main"
    tabindex="-1"
    id="main-content-masthead-toggle-group-filters-example"
  >
    <section class="pf-v5-c-page__main-breadcrumb pf-m-limit-width">
      <div class="pf-v5-c-page__main-body">
        <nav class="pf-v5-c-breadcrumb" aria-label="breadcrumb">
          <ol class="pf-v5-c-breadcrumb__list" role="list">
            <li class="pf-v5-c-breadcrumb__item">
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
      </div>
    </section>
    <section class="pf-v5-c-page__main-section pf-m-limit-width pf-m-light">
      <div class="pf-v5-c-page__main-body">
        <div class="pf-v5-c-content">
          <h1>Main title</h1>
          <p>This is a full page demo.</p>
        </div>
      </div>
    </section>
    <section class="pf-v5-c-page__main-section pf-m-limit-width">
      <div class="pf-v5-c-page__main-body">
        <div class="pf-v5-l-gallery pf-m-gutter">
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
        </div>
      </div>
    </section>
  </main>
</div>

```

### With expandable search, collapsed

```html isFullscreen
<div class="pf-v5-c-page" id="masthead-expandable-search-example">
  <div class="pf-v5-c-skip-to-content">
    <a
      class="pf-v5-c-button pf-m-primary"
      href="#main-content-masthead-expandable-search-example"
    >Skip to content</a>
  </div>
  <header
    class="pf-v5-c-masthead"
    id="masthead-expandable-search-example-masthead"
  >
    <span class="pf-v5-c-masthead__toggle">
      <button
        class="pf-v5-c-button pf-m-plain"
        type="button"
        aria-label="Global navigation"
      >
        <i class="fas fa-bars" aria-hidden="true"></i>
      </button>
    </span>
    <div class="pf-v5-c-masthead__main">
      <a class="pf-v5-c-masthead__brand" href="#">
        <img
          class="pf-v5-c-brand"
          src="/assets/images/pf-logo.svg"
          alt="PatternFly logo"
          style="--pf-v5-c-brand--Height:36px"
        />
      </a>
    </div>
    <div class="pf-v5-c-masthead__content">
      <div
        class="pf-v5-c-toolbar pf-m-full-height pf-m-static"
        id="masthead-expandable-search-example-masthead-toolbar"
      >
        <div class="pf-v5-c-toolbar__content">
          <div class="pf-v5-c-toolbar__content-section">
            <div class="pf-v5-c-toolbar__group pf-m-align-right">
              <div class="pf-v5-c-toolbar__item">
                <div class="pf-v5-c-input-group pf-m-plain">
                  <div class="pf-v5-c-input-group__item pf-m-plain">
                    <button
                      class="pf-v5-c-button pf-m-plain"
                      type="button"
                      aria-label="Open search"
                    >
                      <i class="fas fa-fw fa-search" aria-hidden="true"></i>
                    </button>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </header>
  <div class="pf-v5-c-page__sidebar">
    <div class="pf-v5-c-page__sidebar-body">
      <nav
        class="pf-v5-c-nav"
        id="masthead-expandable-search-example-primary-nav"
        aria-label="Global"
      >
        <ul class="pf-v5-c-nav__list" role="list">
          <li class="pf-v5-c-nav__item">
            <a href="#" class="pf-v5-c-nav__link">System panel</a>
          </li>
          <li class="pf-v5-c-nav__item">
            <a
              href="#"
              class="pf-v5-c-nav__link pf-m-current"
              aria-current="page"
            >Policy</a>
          </li>
          <li class="pf-v5-c-nav__item">
            <a href="#" class="pf-v5-c-nav__link">Authentication</a>
          </li>
          <li class="pf-v5-c-nav__item">
            <a href="#" class="pf-v5-c-nav__link">Network services</a>
          </li>
          <li class="pf-v5-c-nav__item">
            <a href="#" class="pf-v5-c-nav__link">Server</a>
          </li>
        </ul>
      </nav>
    </div>
  </div>
  <main
    class="pf-v5-c-page__main"
    tabindex="-1"
    id="main-content-masthead-expandable-search-example"
  >
    <section class="pf-v5-c-page__main-breadcrumb pf-m-limit-width">
      <div class="pf-v5-c-page__main-body">
        <nav class="pf-v5-c-breadcrumb" aria-label="breadcrumb">
          <ol class="pf-v5-c-breadcrumb__list" role="list">
            <li class="pf-v5-c-breadcrumb__item">
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
      </div>
    </section>
    <section class="pf-v5-c-page__main-section pf-m-limit-width pf-m-light">
      <div class="pf-v5-c-page__main-body">
        <div class="pf-v5-c-content">
          <h1>Main title</h1>
          <p>This is a full page demo.</p>
        </div>
      </div>
    </section>
    <section class="pf-v5-c-page__main-section pf-m-limit-width">
      <div class="pf-v5-c-page__main-body">
        <div class="pf-v5-l-gallery pf-m-gutter">
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
        </div>
      </div>
    </section>
  </main>
</div>

```

### With expandable search, expanded

```html isFullscreen
<div class="pf-v5-c-page" id="masthead-expandable-search-expanded-example">
  <div class="pf-v5-c-skip-to-content">
    <a
      class="pf-v5-c-button pf-m-primary"
      href="#main-content-masthead-expandable-search-expanded-example"
    >Skip to content</a>
  </div>
  <header
    class="pf-v5-c-masthead"
    id="masthead-expandable-search-expanded-example-masthead"
  >
    <span class="pf-v5-c-masthead__toggle">
      <button
        class="pf-v5-c-button pf-m-plain"
        type="button"
        aria-label="Global navigation"
      >
        <i class="fas fa-bars" aria-hidden="true"></i>
      </button>
    </span>
    <div class="pf-v5-c-masthead__main">
      <a class="pf-v5-c-masthead__brand" href="#">
        <img
          class="pf-v5-c-brand"
          src="/assets/images/pf-logo.svg"
          alt="PatternFly logo"
          style="--pf-v5-c-brand--Height:36px"
        />
      </a>
    </div>
    <div class="pf-v5-c-masthead__content">
      <div
        class="pf-v5-c-toolbar pf-m-full-height pf-m-static"
        id="masthead-expandable-search-expanded-example-masthead-toolbar"
      >
        <div class="pf-v5-c-toolbar__content">
          <div class="pf-v5-c-toolbar__content-section">
            <div class="pf-v5-c-toolbar__group pf-m-align-right">
              <div class="pf-v5-c-toolbar__item">
                <div class="pf-v5-c-input-group pf-m-plain">
                  <div class="pf-v5-c-input-group__item pf-m-fill">
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
                            aria-label="Type to filter"
                          />
                        </span>
                      </div>
                    </div>
                  </div>
                  <div class="pf-v5-c-input-group__item pf-m-plain">
                    <button
                      class="pf-v5-c-button pf-m-plain"
                      type="button"
                      aria-label="Close"
                    >
                      <i class="fas fa-times" aria-hidden="true"></i>
                    </button>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </header>
  <div class="pf-v5-c-page__sidebar">
    <div class="pf-v5-c-page__sidebar-body">
      <nav
        class="pf-v5-c-nav"
        id="masthead-expandable-search-expanded-example-primary-nav"
        aria-label="Global"
      >
        <ul class="pf-v5-c-nav__list" role="list">
          <li class="pf-v5-c-nav__item">
            <a href="#" class="pf-v5-c-nav__link">System panel</a>
          </li>
          <li class="pf-v5-c-nav__item">
            <a
              href="#"
              class="pf-v5-c-nav__link pf-m-current"
              aria-current="page"
            >Policy</a>
          </li>
          <li class="pf-v5-c-nav__item">
            <a href="#" class="pf-v5-c-nav__link">Authentication</a>
          </li>
          <li class="pf-v5-c-nav__item">
            <a href="#" class="pf-v5-c-nav__link">Network services</a>
          </li>
          <li class="pf-v5-c-nav__item">
            <a href="#" class="pf-v5-c-nav__link">Server</a>
          </li>
        </ul>
      </nav>
    </div>
  </div>
  <main
    class="pf-v5-c-page__main"
    tabindex="-1"
    id="main-content-masthead-expandable-search-expanded-example"
  >
    <section class="pf-v5-c-page__main-breadcrumb pf-m-limit-width">
      <div class="pf-v5-c-page__main-body">
        <nav class="pf-v5-c-breadcrumb" aria-label="breadcrumb">
          <ol class="pf-v5-c-breadcrumb__list" role="list">
            <li class="pf-v5-c-breadcrumb__item">
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
      </div>
    </section>
    <section class="pf-v5-c-page__main-section pf-m-limit-width pf-m-light">
      <div class="pf-v5-c-page__main-body">
        <div class="pf-v5-c-content">
          <h1>Main title</h1>
          <p>This is a full page demo.</p>
        </div>
      </div>
    </section>
    <section class="pf-v5-c-page__main-section pf-m-limit-width">
      <div class="pf-v5-c-page__main-body">
        <div class="pf-v5-l-gallery pf-m-gutter">
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
        </div>
      </div>
    </section>
  </main>
</div>

```

### Advanced integration with menu options

```html isFullscreen
<div class="pf-v5-c-page" id="masthead-advanced-with-menu-example">
  <div class="pf-v5-c-skip-to-content">
    <a
      class="pf-v5-c-button pf-m-primary"
      href="#main-content-masthead-advanced-with-menu-example"
    >Skip to content</a>
  </div>
  <header
    class="pf-v5-c-masthead"
    id="masthead-advanced-with-menu-example-masthead"
  >
    <span class="pf-v5-c-masthead__toggle">
      <button
        class="pf-v5-c-button pf-m-plain"
        type="button"
        aria-label="Global navigation"
      >
        <i class="fas fa-bars" aria-hidden="true"></i>
      </button>
    </span>
    <div class="pf-v5-c-masthead__main">
      <a class="pf-v5-c-masthead__brand" href="#">
        <img
          class="pf-v5-c-brand"
          src="/assets/images/pf-logo.svg"
          alt="PatternFly logo"
          style="--pf-v5-c-brand--Height:36px"
        />
      </a>
    </div>
    <div class="pf-v5-c-masthead__content">
      <div
        class="pf-v5-c-toolbar pf-m-full-height pf-m-static"
        id="masthead-advanced-with-menu-example-masthead-toolbar"
      >
        <div class="pf-v5-c-toolbar__content">
          <div class="pf-v5-c-toolbar__content-section">
            <div
              class="pf-v5-c-toolbar__group pf-m-icon-button-group pf-m-align-right pf-m-spacer-none pf-m-spacer-md-on-md"
            >
              <div
                class="pf-v5-c-toolbar__group pf-m-icon-button-group pf-m-hidden pf-m-visible-on-lg"
              >
                <div class="pf-v5-c-toolbar__item">
                  <button
                    class="pf-v5-c-menu-toggle pf-m-plain"
                    type="button"
                    aria-expanded="false"
                    aria-label="Application launcher"
                  >
                    <i class="fas fa-th" aria-hidden="true"></i>
                  </button>
                </div>
                <div class="pf-v5-c-toolbar__item">
                  <button
                    class="pf-v5-c-menu-toggle pf-m-plain"
                    type="button"
                    aria-expanded="false"
                    aria-label="Settings"
                  >
                    <i class="fas fa-cog" aria-hidden="true"></i>
                  </button>
                </div>
                <div class="pf-v5-c-toolbar__item">
                  <button
                    class="pf-v5-c-menu-toggle pf-m-plain"
                    type="button"
                    aria-expanded="false"
                    aria-label="Help"
                  >
                    <i class="fas fa-question-circle" aria-hidden="true"></i>
                  </button>
                </div>
              </div>
              <div class="pf-v5-c-toolbar__item pf-m-hidden-on-lg">
                <button
                  class="pf-v5-c-menu-toggle pf-m-plain"
                  type="button"
                  aria-expanded="false"
                  aria-label="Actions"
                >
                  <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
                </button>
              </div>
            </div>
            <div class="pf-v5-c-toolbar__item pf-m-hidden pf-m-visible-on-sm">
              <button
                class="pf-v5-c-menu-toggle pf-m-full-height"
                type="button"
                aria-expanded="false"
              >
                <span class="pf-v5-c-menu-toggle__icon">
                  <img
                    class="pf-v5-c-avatar"
                    alt="Avatar image"
                    src="/assets/images/img_avatar-light.svg"
                  />
                </span>
                <span class="pf-v5-c-menu-toggle__text">Ned Username</span>
                <span class="pf-v5-c-menu-toggle__controls">
                  <span class="pf-v5-c-menu-toggle__toggle-icon">
                    <i class="fas fa-caret-down" aria-hidden="true"></i>
                  </span>
                </span>
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  </header>
  <div class="pf-v5-c-page__sidebar">
    <div class="pf-v5-c-page__sidebar-body">
      <nav
        class="pf-v5-c-nav"
        id="masthead-advanced-with-menu-example-primary-nav"
        aria-label="Global"
      >
        <ul class="pf-v5-c-nav__list" role="list">
          <li class="pf-v5-c-nav__item">
            <a href="#" class="pf-v5-c-nav__link">System panel</a>
          </li>
          <li class="pf-v5-c-nav__item">
            <a
              href="#"
              class="pf-v5-c-nav__link pf-m-current"
              aria-current="page"
            >Policy</a>
          </li>
          <li class="pf-v5-c-nav__item">
            <a href="#" class="pf-v5-c-nav__link">Authentication</a>
          </li>
          <li class="pf-v5-c-nav__item">
            <a href="#" class="pf-v5-c-nav__link">Network services</a>
          </li>
          <li class="pf-v5-c-nav__item">
            <a href="#" class="pf-v5-c-nav__link">Server</a>
          </li>
        </ul>
      </nav>
    </div>
  </div>
  <main
    class="pf-v5-c-page__main"
    tabindex="-1"
    id="main-content-masthead-advanced-with-menu-example"
  >
    <section class="pf-v5-c-page__main-breadcrumb pf-m-limit-width">
      <div class="pf-v5-c-page__main-body">
        <nav class="pf-v5-c-breadcrumb" aria-label="breadcrumb">
          <ol class="pf-v5-c-breadcrumb__list" role="list">
            <li class="pf-v5-c-breadcrumb__item">
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
      </div>
    </section>
    <section class="pf-v5-c-page__main-section pf-m-limit-width pf-m-light">
      <div class="pf-v5-c-page__main-body">
        <div class="pf-v5-c-content">
          <h1>Main title</h1>
          <p>This is a full page demo.</p>
        </div>
      </div>
    </section>
    <section class="pf-v5-c-page__main-section pf-m-limit-width">
      <div class="pf-v5-c-page__main-body">
        <div class="pf-v5-l-gallery pf-m-gutter">
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
        </div>
      </div>
    </section>
  </main>
</div>

```

### Horizontal nav

```html isFullscreen
<div class="pf-v5-c-page" id="masthead-horizontal-nav">
  <div class="pf-v5-c-skip-to-content">
    <a
      class="pf-v5-c-button pf-m-primary"
      href="#main-content-masthead-horizontal-nav"
    >Skip to content</a>
  </div>
  <header class="pf-v5-c-masthead" id="masthead-horizontal-nav-masthead">
    <div class="pf-v5-c-masthead__main">
      <a class="pf-v5-c-masthead__brand" href="#">
        <img
          class="pf-v5-c-brand"
          src="/assets/images/pf-logo.svg"
          alt="PatternFly logo"
          style="--pf-v5-c-brand--Height:36px"
        />
      </a>
    </div>
    <div class="pf-v5-c-masthead__content">
      <div
        class="pf-v5-c-toolbar pf-m-full-height pf-m-static"
        id="masthead-horizontal-nav-masthead-toolbar"
      >
        <div class="pf-v5-c-toolbar__content">
          <div class="pf-v5-c-toolbar__content-section">
            <div
              class="pf-v5-c-toolbar__item pf-m-overflow-container"
              style="--pf-v5-c-toolbar__item--MinWidth: 18ch;"
            >
              <nav
                class="pf-v5-c-nav pf-m-horizontal pf-m-scrollable"
                id="-horizontal-nav"
                aria-label="Global"
              >
                <button
                  class="pf-v5-c-nav__scroll-button"
                  disabled
                  aria-label="Scroll left"
                >
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
                <button
                  class="pf-v5-c-nav__scroll-button"
                  aria-label="Scroll right"
                >
                  <i class="fas fa-angle-right" aria-hidden="true"></i>
                </button>
              </nav>
            </div>
            <div
              class="pf-v5-c-toolbar__group pf-m-icon-button-group pf-m-align-right pf-m-spacer-none pf-m-spacer-md-on-md"
            >
              <div
                class="pf-v5-c-toolbar__group pf-m-icon-button-group pf-m-hidden pf-m-visible-on-lg"
              >
                <div class="pf-v5-c-toolbar__item">
                  <button
                    class="pf-v5-c-menu-toggle pf-m-plain"
                    type="button"
                    aria-expanded="false"
                    aria-label="Application launcher"
                  >
                    <i class="fas fa-th" aria-hidden="true"></i>
                  </button>
                </div>
                <div class="pf-v5-c-toolbar__item">
                  <button
                    class="pf-v5-c-menu-toggle pf-m-plain"
                    type="button"
                    aria-expanded="false"
                    aria-label="Settings"
                  >
                    <i class="fas fa-cog" aria-hidden="true"></i>
                  </button>
                </div>
                <div class="pf-v5-c-toolbar__item">
                  <button
                    class="pf-v5-c-menu-toggle pf-m-plain"
                    type="button"
                    aria-expanded="false"
                    aria-label="Help"
                  >
                    <i class="fas fa-question-circle" aria-hidden="true"></i>
                  </button>
                </div>
              </div>
              <div class="pf-v5-c-toolbar__item pf-m-hidden-on-lg">
                <button
                  class="pf-v5-c-menu-toggle pf-m-plain"
                  type="button"
                  aria-expanded="false"
                  aria-label="Actions"
                >
                  <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
                </button>
              </div>
            </div>
            <div class="pf-v5-c-toolbar__item pf-m-hidden pf-m-visible-on-sm">
              <button
                class="pf-v5-c-menu-toggle pf-m-full-height"
                type="button"
                aria-expanded="false"
              >
                <span class="pf-v5-c-menu-toggle__icon">
                  <img
                    class="pf-v5-c-avatar"
                    alt="Avatar image"
                    src="/assets/images/img_avatar-light.svg"
                  />
                </span>
                <span class="pf-v5-c-menu-toggle__text">Ned Username</span>
                <span class="pf-v5-c-menu-toggle__controls">
                  <span class="pf-v5-c-menu-toggle__toggle-icon">
                    <i class="fas fa-caret-down" aria-hidden="true"></i>
                  </span>
                </span>
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  </header>
  <main
    class="pf-v5-c-page__main"
    tabindex="-1"
    id="main-content-masthead-horizontal-nav"
  >
    <section class="pf-v5-c-page__main-breadcrumb pf-m-limit-width">
      <div class="pf-v5-c-page__main-body">
        <nav class="pf-v5-c-breadcrumb" aria-label="breadcrumb">
          <ol class="pf-v5-c-breadcrumb__list" role="list">
            <li class="pf-v5-c-breadcrumb__item">
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
      </div>
    </section>
    <section class="pf-v5-c-page__main-section pf-m-limit-width pf-m-light">
      <div class="pf-v5-c-page__main-body">
        <div class="pf-v5-c-content">
          <h1>Main title</h1>
          <p>This is a full page demo.</p>
        </div>
      </div>
    </section>
    <section class="pf-v5-c-page__main-section pf-m-limit-width">
      <div class="pf-v5-c-page__main-body">
        <div class="pf-v5-l-gallery pf-m-gutter">
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
        </div>
      </div>
    </section>
  </main>
</div>

```

## Mobile examples

### With toggle group, filters, expandable content expanded (mobile)

```html isFullscreen
<div
  class="pf-v5-c-page"
  id="masthead-toggle-group-filters-expanded-mobile-example"
>
  <div class="pf-v5-c-skip-to-content">
    <a
      class="pf-v5-c-button pf-m-primary"
      href="#main-content-masthead-toggle-group-filters-expanded-mobile-example"
    >Skip to content</a>
  </div>
  <header
    class="pf-v5-c-masthead"
    id="masthead-toggle-group-filters-expanded-mobile-example-masthead"
  >
    <span class="pf-v5-c-masthead__toggle">
      <button
        class="pf-v5-c-button pf-m-plain"
        type="button"
        aria-label="Global navigation"
      >
        <i class="fas fa-bars" aria-hidden="true"></i>
      </button>
    </span>
    <div class="pf-v5-c-masthead__main">
      <a class="pf-v5-c-masthead__brand" href="#">
        <img
          class="pf-v5-c-brand"
          src="/assets/images/pf-logo.svg"
          alt="PatternFly logo"
          style="--pf-v5-c-brand--Height:36px"
        />
      </a>
    </div>
    <div class="pf-v5-c-masthead__content">
      <div
        class="pf-v5-c-toolbar pf-m-full-height pf-m-static"
        id="masthead-toggle-group-filters-expanded-mobile-example-masthead-toolbar"
      >
        <div class="pf-v5-c-toolbar__content">
          <div class="pf-v5-c-toolbar__content-section">
            <div
              class="pf-v5-c-toolbar__group pf-m-toggle-group pf-m-show pf-m-align-right"
            >
              <div class="pf-v5-c-toolbar__toggle">
                <button
                  class="pf-v5-c-button pf-m-plain"
                  type="button"
                  aria-label="Show filters"
                  aria-expanded="true"
                  aria-controls="masthead-toggle-group-filters-expanded-mobile-example-masthead-toolbar-expandable-content"
                >
                  <i class="fas fa-filter" aria-hidden="true"></i>
                </button>
              </div>
            </div>
            <div class="pf-v5-c-toolbar__item">
              <button
                class="pf-v5-c-menu-toggle pf-m-plain"
                type="button"
                aria-expanded="false"
                aria-label="Actions"
              >
                <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
              </button>
            </div>
          </div>
          <div
            class="pf-v5-c-toolbar__expandable-content pf-m-expanded"
            id="masthead-toggle-group-filters-expanded-mobile-example-masthead-toolbar-expandable-content"
          >
            <div class="pf-v5-c-toolbar__item pf-m-search-filter">
              <div
                class="pf-v5-c-input-group"
                aria-label="search filter"
                role="group"
              >
                <div class="pf-v5-c-input-group__item">
                  <div class="pf-v5-c-select" style="width: 124px">
                    <span
                      id="masthead-toggle-group-filters-expanded-mobile-example-masthead-toolbar-select-name-label"
                      hidden
                    >Choose one</span>

                    <button
                      class="pf-v5-c-select__toggle"
                      type="button"
                      id="masthead-toggle-group-filters-expanded-mobile-example-masthead-toolbar-select-name-toggle"
                      aria-haspopup="true"
                      aria-expanded="false"
                      aria-labelledby="masthead-toggle-group-filters-expanded-mobile-example-masthead-toolbar-select-name-label masthead-toggle-group-filters-expanded-mobile-example-masthead-toolbar-select-name-toggle"
                    >
                      <div class="pf-v5-c-select__toggle-wrapper">
                        <span class="pf-v5-c-select__toggle-icon">
                          <i class="fas fa-filter" aria-hidden="true"></i>
                        </span>
                        <span class="pf-v5-c-select__toggle-text">Name</span>
                      </div>
                      <span class="pf-v5-c-select__toggle-arrow">
                        <i class="fas fa-caret-down" aria-hidden="true"></i>
                      </span>
                    </button>

                    <ul
                      class="pf-v5-c-select__menu"
                      role="listbox"
                      aria-labelledby="masthead-toggle-group-filters-expanded-mobile-example-masthead-toolbar-select-name-label"
                      hidden
                    >
                      <li role="presentation">
                        <button
                          class="pf-v5-c-select__menu-item"
                          role="option"
                        >Running</button>
                      </li>
                      <li role="presentation">
                        <button
                          class="pf-v5-c-select__menu-item pf-m-selected"
                          role="option"
                          aria-selected="true"
                        >
                          Stopped
                          <span class="pf-v5-c-select__menu-item-icon">
                            <i class="fas fa-check" aria-hidden="true"></i>
                          </span>
                        </button>
                      </li>
                      <li role="presentation">
                        <button
                          class="pf-v5-c-select__menu-item"
                          role="option"
                        >Down</button>
                      </li>
                      <li role="presentation">
                        <button
                          class="pf-v5-c-select__menu-item"
                          role="option"
                        >Degraded</button>
                      </li>
                      <li role="presentation">
                        <button
                          class="pf-v5-c-select__menu-item"
                          role="option"
                        >Needs maintenance</button>
                      </li>
                    </ul>
                  </div>
                </div>
                <div class="pf-v5-c-input-group__item pf-m-fill">
                  <div class="pf-v5-c-text-input-group">
                    <div class="pf-v5-c-text-input-group__main pf-m-icon">
                      <span class="pf-v5-c-text-input-group__text">
                        <span class="pf-v5-c-text-input-group__icon">
                          <i class="fas fa-fw fa-search"></i>
                        </span>
                        <input
                          class="pf-v5-c-text-input-group__text-input"
                          type="text"
                          placeholder="Filter by name"
                          value
                          aria-label="Search input"
                        />
                      </span>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </header>
  <div class="pf-v5-c-page__sidebar">
    <div class="pf-v5-c-page__sidebar-body">
      <nav
        class="pf-v5-c-nav"
        id="masthead-toggle-group-filters-expanded-mobile-example-primary-nav"
        aria-label="Global"
      >
        <ul class="pf-v5-c-nav__list" role="list">
          <li class="pf-v5-c-nav__item">
            <a href="#" class="pf-v5-c-nav__link">System panel</a>
          </li>
          <li class="pf-v5-c-nav__item">
            <a
              href="#"
              class="pf-v5-c-nav__link pf-m-current"
              aria-current="page"
            >Policy</a>
          </li>
          <li class="pf-v5-c-nav__item">
            <a href="#" class="pf-v5-c-nav__link">Authentication</a>
          </li>
          <li class="pf-v5-c-nav__item">
            <a href="#" class="pf-v5-c-nav__link">Network services</a>
          </li>
          <li class="pf-v5-c-nav__item">
            <a href="#" class="pf-v5-c-nav__link">Server</a>
          </li>
        </ul>
      </nav>
    </div>
  </div>
  <main
    class="pf-v5-c-page__main"
    tabindex="-1"
    id="main-content-masthead-toggle-group-filters-expanded-mobile-example"
  >
    <section class="pf-v5-c-page__main-breadcrumb pf-m-limit-width">
      <div class="pf-v5-c-page__main-body">
        <nav class="pf-v5-c-breadcrumb" aria-label="breadcrumb">
          <ol class="pf-v5-c-breadcrumb__list" role="list">
            <li class="pf-v5-c-breadcrumb__item">
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
      </div>
    </section>
    <section class="pf-v5-c-page__main-section pf-m-limit-width pf-m-light">
      <div class="pf-v5-c-page__main-body">
        <div class="pf-v5-c-content">
          <h1>Main title</h1>
          <p>This is a full page demo.</p>
        </div>
      </div>
    </section>
    <section class="pf-v5-c-page__main-section pf-m-limit-width">
      <div class="pf-v5-c-page__main-body">
        <div class="pf-v5-l-gallery pf-m-gutter">
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
          <div class="pf-v5-c-card">
            <div class="pf-v5-c-card__body">This is a card</div>
          </div>
        </div>
      </div>
    </section>
  </main>
</div>

```
