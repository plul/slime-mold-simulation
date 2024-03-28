---
id: 'Context selector'
section: components
subsection: menus
deprecated: true
---## Examples

### Context selector in masthead

```html isFullscreen
<div class="pf-v5-c-page" id="context-selector-in-masthead">
  <div class="pf-v5-c-skip-to-content">
    <a
      class="pf-v5-c-button pf-m-primary"
      href="#main-content-context-selector-in-masthead"
    >Skip to content</a>
  </div>
  <header class="pf-v5-c-masthead" id="context-selector-in-masthead-masthead">
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
        id="context-selector-in-masthead-masthead-toolbar"
      >
        <div class="pf-v5-c-toolbar__content">
          <div class="pf-v5-c-toolbar__content-section">
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
          </div>
        </div>
      </div>
    </div>
  </header>
  <div class="pf-v5-c-page__sidebar">
    <div class="pf-v5-c-page__sidebar-body">
      <nav
        class="pf-v5-c-nav"
        id="context-selector-in-masthead-primary-nav"
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
    id="main-content-context-selector-in-masthead"
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

### Context selector in sidebar

```html isFullscreen
<div class="pf-v5-c-page" id="context-selector-in-sidebar-example">
  <div class="pf-v5-c-skip-to-content">
    <a
      class="pf-v5-c-button pf-m-primary"
      href="#main-content-context-selector-in-sidebar-example"
    >Skip to content</a>
  </div>
  <header
    class="pf-v5-c-masthead"
    id="context-selector-in-sidebar-example-masthead"
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
        id="context-selector-in-sidebar-example-masthead-toolbar"
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
    <div class="pf-v5-c-page__sidebar-body pf-m-menu">
      <div class="pf-v5-c-context-selector pf-m-page-insets pf-m-large">
        <span
          id="context-selector-collapsed-example-label"
          hidden
        >Selected project:</span>
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
    </div>
    <div class="pf-v5-c-page__sidebar-body">
      <nav
        class="pf-v5-c-nav"
        id="context-selector-in-sidebar-example-primary-nav"
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
    id="main-content-context-selector-in-sidebar-example"
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

### Context selector in sidebar expanded

```html isFullscreen
<div class="pf-v5-c-page" id="context-selector-in-sidebar-expanded-example">
  <div class="pf-v5-c-skip-to-content">
    <a
      class="pf-v5-c-button pf-m-primary"
      href="#main-content-context-selector-in-sidebar-expanded-example"
    >Skip to content</a>
  </div>
  <header
    class="pf-v5-c-masthead"
    id="context-selector-in-sidebar-expanded-example-masthead"
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
        id="context-selector-in-sidebar-expanded-example-masthead-toolbar"
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
    <div class="pf-v5-c-page__sidebar-body pf-m-menu">
      <div
        class="pf-v5-c-context-selector pf-m-expanded pf-m-page-insets pf-m-large"
      >
        <span
          id="context-selector-collapsed-example-label"
          hidden
        >Selected project:</span>
        <button
          class="pf-v5-c-context-selector__toggle"
          aria-expanded="true"
          id="context-selector-collapsed-example-toggle"
          aria-labelledby="context-selector-collapsed-example-label context-selector-collapsed-example-toggle"
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
    </div>
    <div class="pf-v5-c-page__sidebar-body">
      <nav
        class="pf-v5-c-nav"
        id="context-selector-in-sidebar-expanded-example-primary-nav"
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
    id="main-content-context-selector-in-sidebar-expanded-example"
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

### Context selector in page content

```html isFullscreen
<div class="pf-v5-c-page" id="context-selector-in-page-content-example">
  <div class="pf-v5-c-skip-to-content">
    <a
      class="pf-v5-c-button pf-m-primary"
      href="#main-content-context-selector-in-page-content-example"
    >Skip to content</a>
  </div>
  <header
    class="pf-v5-c-masthead"
    id="context-selector-in-page-content-example-masthead"
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
        id="context-selector-in-page-content-example-masthead-toolbar"
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
        id="context-selector-in-page-content-example-primary-nav"
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
    id="main-content-context-selector-in-page-content-example"
  >
    <section
      class="pf-v5-c-page__main-section pf-m-limit-width pf-m-light pf-m-no-padding"
    >
      <div class="pf-v5-c-page__main-body">
        <div
          class="pf-v5-c-toolbar pf-m-inset-none"
          id="toolbar-simple-example"
        >
          <div class="pf-v5-c-toolbar__content">
            <div class="pf-v5-c-toolbar__content-section">
              <div class="pf-v5-c-toolbar__item">
                <div
                  class="pf-v5-c-context-selector pf-m-page-insets pf-m-width-auto"
                  style="--pf-v5-c-context-selector--Width: 270px;"
                >
                  <span
                    id="context-selector-in-page-content-example-context-selector-label"
                    hidden
                  >Selected project:</span>
                  <button
                    class="pf-v5-c-context-selector__toggle pf-m-text pf-m-plain"
                    aria-expanded="false"
                    id="context-selector-in-page-content-example-context-selector-toggle"
                    aria-labelledby="context-selector-in-page-content-example-context-selector-label context-selector-in-page-content-example-context-selector-toggle"
                  >
                    <span
                      class="pf-v5-c-context-selector__toggle-text"
                    >Project: openshift-apple1</span>
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
              </div>
              <div class="pf-v5-c-toolbar__item">
                <div class="pf-v5-c-select">
                  <span
                    id="context-selector-in-page-content-example-select-label"
                    hidden
                  >Choose one</span>

                  <button
                    class="pf-v5-c-select__toggle pf-m-plain"
                    type="button"
                    id="context-selector-in-page-content-example-select-toggle"
                    aria-haspopup="true"
                    aria-expanded="false"
                    aria-labelledby="context-selector-in-page-content-example-select-label context-selector-in-page-content-example-select-toggle"
                  >
                    <div class="pf-v5-c-select__toggle-wrapper">
                      <span class="pf-v5-c-select__toggle-text">All applications</span>
                    </div>
                    <span class="pf-v5-c-select__toggle-arrow">
                      <i class="fas fa-caret-down" aria-hidden="true"></i>
                    </span>
                  </button>

                  <ul
                    class="pf-v5-c-select__menu"
                    role="listbox"
                    aria-labelledby="context-selector-in-page-content-example-select-label"
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
            </div>
          </div>
        </div>
      </div>
    </section>
    <hr class="pf-v5-c-divider" />
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
