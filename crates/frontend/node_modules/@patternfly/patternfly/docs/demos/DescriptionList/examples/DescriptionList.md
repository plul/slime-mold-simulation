---
id: 'Description list'
section: components
cssPrefix: pf-d-description-list
---## Examples

### Basic

```html isFullscreen
<div class="pf-v5-c-page" id="description-list-basic-example">
  <div class="pf-v5-c-skip-to-content">
    <a
      class="pf-v5-c-button pf-m-primary"
      href="#main-content-description-list-basic-example"
    >Skip to content</a>
  </div>
  <header class="pf-v5-c-masthead" id="description-list-basic-example-masthead">
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
        id="description-list-basic-example-masthead-toolbar"
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
        id="description-list-basic-example-primary-nav"
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
    id="main-content-description-list-basic-example"
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
        <div class="pf-v5-c-card">
          <div class="pf-v5-c-card__header">
            <h2 class="pf-v5-c-title pf-m-lg">Details</h2>
          </div>
          <hr class="pf-v5-c-divider" />
          <div class="pf-v5-c-card__body">
            <dl class="pf-v5-c-description-list pf-m-auto-fit">
              <div class="pf-v5-c-description-list__group">
                <dt class="pf-v5-c-description-list__term">
                  <span class="pf-v5-c-description-list__text">Name</span>
                </dt>
                <dd class="pf-v5-c-description-list__description">
                  <div class="pf-v5-c-description-list__text">mary-test</div>
                </dd>
              </div>
              <div class="pf-v5-c-description-list__group">
                <dt class="pf-v5-c-description-list__term">
                  <span class="pf-v5-c-description-list__text">Status</span>
                </dt>
                <dd class="pf-v5-c-description-list__description">
                  <div class="pf-v5-c-description-list__text">
                    <div class="pf-v5-l-flex pf-m-space-items-sm">
                      <div class="pf-v5-l-flex__item">
                        <i
                          class="fas fa-check-circle pf-v5-u-success-color-100"
                          aria-hidden="true"
                        ></i>
                      </div>
                      <div class="pf-v5-l-flex__item">
                        <span>Active</span>
                      </div>
                    </div>
                  </div>
                </dd>
              </div>
              <div class="pf-v5-c-description-list__group">
                <dt class="pf-v5-c-description-list__term">
                  <span
                    class="pf-v5-c-description-list__text"
                  >Default pull secret</span>
                </dt>
                <dd class="pf-v5-c-description-list__description">
                  <div class="pf-v5-c-description-list__text">
                    <span class="pf-v5-u-color-300">Not configured</span>
                  </div>
                </dd>
              </div>
              <div class="pf-v5-c-description-list__group">
                <dt class="pf-v5-c-description-list__term">
                  <span class="pf-v5-c-description-list__text">Tolerations</span>
                </dt>
                <dd class="pf-v5-c-description-list__description">
                  <div class="pf-v5-c-description-list__text">6 Tolerations</div>
                </dd>
              </div>
              <div class="pf-v5-c-description-list__group">
                <dt class="pf-v5-c-description-list__term">
                  <span class="pf-v5-c-description-list__text">Network Policies</span>
                </dt>
                <dd class="pf-v5-c-description-list__description">
                  <div class="pf-v5-c-description-list__text">
                    <a href="#">Network Policies</a>
                  </div>
                </dd>
              </div>
              <div class="pf-v5-c-description-list__group">
                <dt class="pf-v5-c-description-list__term">
                  <span class="pf-v5-c-description-list__text">Display name</span>
                </dt>
                <dd class="pf-v5-c-description-list__description">
                  <div class="pf-v5-c-description-list__text">mary</div>
                </dd>
              </div>
              <div class="pf-v5-c-description-list__group">
                <dt class="pf-v5-c-description-list__term">
                  <span class="pf-v5-c-description-list__text">Requester</span>
                </dt>
                <dd class="pf-v5-c-description-list__description">
                  <div class="pf-v5-c-description-list__text">kube:admin</div>
                </dd>
              </div>
              <div class="pf-v5-c-description-list__group">
                <dt class="pf-v5-c-description-list__term">
                  <span class="pf-v5-c-description-list__text">Created at:</span>
                </dt>
                <dd class="pf-v5-c-description-list__description">
                  <div class="pf-v5-c-description-list__text">3 minutes ago</div>
                </dd>
              </div>
            </dl>
          </div>
        </div>
      </div>
    </section>
  </main>
</div>

```

### In drawer

```html isFullscreen
<div class="pf-v5-c-page" id="description-list-in-drawer-example">
  <div class="pf-v5-c-skip-to-content">
    <a
      class="pf-v5-c-button pf-m-primary"
      href="#main-content-description-list-in-drawer-example"
    >Skip to content</a>
  </div>
  <header
    class="pf-v5-c-masthead"
    id="description-list-in-drawer-example-masthead"
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
        id="description-list-in-drawer-example-masthead-toolbar"
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
        id="description-list-in-drawer-example-primary-nav"
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
  <div class="pf-v5-c-page__drawer">
    <div class="pf-v5-c-drawer pf-m-expanded">
      <div class="pf-v5-c-drawer__main">
        <div class="pf-v5-c-drawer__content">
          <div class="pf-v5-c-drawer__body">
            <main
              class="pf-v5-c-page__main"
              tabindex="-1"
              id="main-content-description-list-in-drawer-example"
            >
              <section class="pf-v5-c-page__main-breadcrumb pf-m-limit-width">
                <div class="pf-v5-c-page__main-body">
                  <nav class="pf-v5-c-breadcrumb" aria-label="breadcrumb">
                    <ol class="pf-v5-c-breadcrumb__list" role="list">
                      <li class="pf-v5-c-breadcrumb__item">
                        <a
                          href="#"
                          class="pf-v5-c-breadcrumb__link"
                        >Section home</a>
                      </li>
                      <li class="pf-v5-c-breadcrumb__item">
                        <span class="pf-v5-c-breadcrumb__item-divider">
                          <i class="fas fa-angle-right" aria-hidden="true"></i>
                        </span>

                        <a
                          href="#"
                          class="pf-v5-c-breadcrumb__link"
                        >Section title</a>
                      </li>
                      <li class="pf-v5-c-breadcrumb__item">
                        <span class="pf-v5-c-breadcrumb__item-divider">
                          <i class="fas fa-angle-right" aria-hidden="true"></i>
                        </span>

                        <a
                          href="#"
                          class="pf-v5-c-breadcrumb__link"
                        >Section title</a>
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
              <section
                class="pf-v5-c-page__main-section pf-m-limit-width pf-m-light"
              >
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
        </div>
        <div class="pf-v5-c-drawer__panel pf-m-width-33-on-lg">
          <div class="pf-v5-c-drawer__body">
            <div class="pf-v5-c-drawer__head">
              <div class="pf-v5-c-drawer__actions">
                <div class="pf-v5-c-dropdown">
                  <button
                    class="pf-v5-c-dropdown__toggle pf-m-plain"
                    id="-button"
                    aria-expanded="false"
                    type="button"
                    aria-label="Actions"
                  >
                    <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
                  </button>
                  <ul
                    class="pf-v5-c-dropdown__menu"
                    aria-labelledby="-button"
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
                <div class="pf-v5-c-drawer__close">
                  <button
                    class="pf-v5-c-button pf-m-plain"
                    type="button"
                    aria-label="Close drawer panel"
                  >
                    <i class="fas fa-times" aria-hidden="true"></i>
                  </button>
                </div>
              </div>
              <div class="pf-v5-l-flex pf-m-space-items-sm">
                <div class="pf-v5-l-flex__item">
                  <span class="pf-v5-c-label pf-m-compact pf-m-blue">
                    <span class="pf-v5-c-label__content">
                      <span class="pf-v5-c-label__text">DC</span>
                    </span>
                  </span>
                </div>
                <div class="pf-v5-l-flex__item">
                  <h2
                    class="pf-v5-c-title pf-m-xl"
                    id="description-list-in-drawer-example-drawer-label"
                  >mary-test</h2>
                </div>
              </div>
            </div>
          </div>
          <div class="pf-v5-c-drawer__body pf-m-no-padding">
            <div class="pf-v5-c-tabs pf-m-box pf-m-fill">
              <ul class="pf-v5-c-tabs__list" role="tablist">
                <li class="pf-v5-c-tabs__item pf-m-current" role="presentation">
                  <button
                    type="button"
                    class="pf-v5-c-tabs__link"
                    role="tab"
                    id="description-list-in-drawer-example-panel-tabs-tab1-link"
                  >
                    <span class="pf-v5-c-tabs__item-text">Overview</span>
                  </button>
                </li>
                <li class="pf-v5-c-tabs__item" role="presentation">
                  <button
                    type="button"
                    class="pf-v5-c-tabs__link"
                    role="tab"
                    id="description-list-in-drawer-example-panel-tabs-tab2-link"
                  >
                    <span class="pf-v5-c-tabs__item-text">Activity</span>
                  </button>
                </li>
              </ul>
            </div>
          </div>
          <div class="pf-v5-c-drawer__body">
            <section
              class="pf-v5-c-tab-content"
              id="description-list-in-drawer-example-panel-tabs-tab1-panel"
              aria-labelledby="description-list-in-drawer-example-panel-tabs-tab1-link"
              role="tabpanel"
              tabindex="0"
            >
              <div class="pf-v5-c-tab-content__body">
                <dl
                  class="pf-v5-c-description-list pf-m-fill-columns pf-m-2-col pf-m-compact"
                >
                  <div class="pf-v5-c-description-list__group">
                    <dt class="pf-v5-c-description-list__term">
                      <span class="pf-v5-c-description-list__text">Name</span>
                    </dt>
                    <dd class="pf-v5-c-description-list__description">
                      <div class="pf-v5-c-description-list__text">mary-test</div>
                    </dd>
                  </div>
                  <div class="pf-v5-c-description-list__group">
                    <dt class="pf-v5-c-description-list__term">
                      <span class="pf-v5-c-description-list__text">Namespace</span>
                    </dt>
                    <dd class="pf-v5-c-description-list__description">
                      <div class="pf-v5-c-description-list__text">
                        <div class="pf-v5-l-flex pf-m-space-items-sm">
                          <div class="pf-v5-l-flex__item">
                            <span class="pf-v5-c-label pf-m-green">
                              <span class="pf-v5-c-label__content">
                                <span class="pf-v5-c-label__text">NS</span>
                              </span>
                            </span>
                          </div>
                          <div class="pf-v5-l-flex__item">
                            <a href="#">mary-test</a>
                          </div>
                        </div>
                      </div>
                    </dd>
                  </div>
                  <div class="pf-v5-c-description-list__group">
                    <dt class="pf-v5-c-description-list__term">
                      <span class="pf-v5-c-description-list__text">Labels</span>
                    </dt>
                    <dd class="pf-v5-c-description-list__description">
                      <div class="pf-v5-c-description-list__text">
                        <div class="pf-v5-c-chip">
                          <span class="pf-v5-c-chip__content">
                            <span class="pf-v5-c-chip__text">app=mary-test</span>
                          </span>
                        </div>
                      </div>
                    </dd>
                  </div>
                  <div class="pf-v5-c-description-list__group">
                    <dt class="pf-v5-c-description-list__term">
                      <span class="pf-v5-c-description-list__text">Node selector</span>
                    </dt>
                    <dd class="pf-v5-c-description-list__description">
                      <p
                        class="pf-v5-c-description-list__text pf-v5-u-color-200"
                      >Nod selector is not available at this time</p>
                    </dd>
                  </div>
                  <div class="pf-v5-c-description-list__group">
                    <dt class="pf-v5-c-description-list__term">
                      <span class="pf-v5-c-description-list__text">Tolerations</span>
                    </dt>
                    <dd class="pf-v5-c-description-list__description">
                      <div
                        class="pf-v5-c-description-list__text pf-v5-u-color-200"
                      >No tolerations</div>
                    </dd>
                  </div>
                  <div class="pf-v5-c-description-list__group">
                    <dt class="pf-v5-c-description-list__term">
                      <span class="pf-v5-c-description-list__text">Annotations</span>
                    </dt>
                    <dd class="pf-v5-c-description-list__description">
                      <div
                        class="pf-v5-c-description-list__text pf-v5-u-color-200"
                      >No annotations</div>
                    </dd>
                  </div>
                  <div class="pf-v5-c-description-list__group">
                    <dt class="pf-v5-c-description-list__term">
                      <span class="pf-v5-c-description-list__text">Status</span>
                    </dt>
                    <dd class="pf-v5-c-description-list__description">
                      <div class="pf-v5-c-description-list__text">Active</div>
                    </dd>
                  </div>
                  <div class="pf-v5-c-description-list__group">
                    <dt class="pf-v5-c-description-list__term">
                      <span class="pf-v5-c-description-list__text">Created at:</span>
                    </dt>
                    <dd class="pf-v5-c-description-list__description">
                      <div class="pf-v5-c-description-list__text">3 minutes ago</div>
                    </dd>
                  </div>
                  <div class="pf-v5-c-description-list__group">
                    <dt class="pf-v5-c-description-list__term">
                      <span class="pf-v5-c-description-list__text">Pod selector</span>
                    </dt>
                    <dd class="pf-v5-c-description-list__description">
                      <div class="pf-v5-c-description-list__text">
                        <a href="#">
                          <div class="pf-v5-l-flex pf-m-space-items-sm">
                            <div class="pf-v5-l-flex__item">
                              <i class="fas fa-search" aria-hidden="true"></i>
                            </div>
                            <div class="pf-v5-l-flex__item">
                              <span>app=MyApp</span>
                            </div>
                          </div>
                        </a>
                      </div>
                    </dd>
                  </div>
                  <div class="pf-v5-c-description-list__group">
                    <dt class="pf-v5-c-description-list__term">
                      <span class="pf-v5-c-description-list__text">Annotations</span>
                    </dt>
                    <dd class="pf-v5-c-description-list__description">
                      <div class="pf-v5-c-description-list__text">2 Annotations</div>
                    </dd>
                  </div>
                  <div class="pf-v5-c-description-list__group">
                    <dt class="pf-v5-c-description-list__term">
                      <span
                        class="pf-v5-c-description-list__text"
                      >Session affinity</span>
                    </dt>
                    <dd class="pf-v5-c-description-list__description">
                      <div
                        class="pf-v5-c-description-list__text pf-v5-u-color-200"
                      >None</div>
                    </dd>
                  </div>
                  <div class="pf-v5-c-description-list__group">
                    <dt class="pf-v5-c-description-list__term">
                      <span
                        class="pf-v5-c-description-list__text"
                      >Latest version</span>
                    </dt>
                    <dd class="pf-v5-c-description-list__description">
                      <div class="pf-v5-c-description-list__text">1.0</div>
                    </dd>
                  </div>
                  <div class="pf-v5-c-description-list__group">
                    <dt class="pf-v5-c-description-list__term">
                      <span
                        class="pf-v5-c-description-list__text"
                      >Update strategy</span>
                    </dt>
                    <dd class="pf-v5-c-description-list__description">
                      <div class="pf-v5-c-description-list__text">Rolling</div>
                    </dd>
                  </div>
                  <div class="pf-v5-c-description-list__group">
                    <dt class="pf-v5-c-description-list__term">
                      <span class="pf-v5-c-description-list__text">Timeout</span>
                    </dt>
                    <dd class="pf-v5-c-description-list__description">
                      <div class="pf-v5-c-description-list__text">600 seconds</div>
                    </dd>
                  </div>
                  <div class="pf-v5-c-description-list__group">
                    <dt class="pf-v5-c-description-list__term">
                      <span class="pf-v5-c-description-list__text">Update period</span>
                    </dt>
                    <dd class="pf-v5-c-description-list__description">
                      <div class="pf-v5-c-description-list__text">1 second</div>
                    </dd>
                  </div>
                  <div class="pf-v5-c-description-list__group">
                    <dt class="pf-v5-c-description-list__term">
                      <span class="pf-v5-c-description-list__text">Interval</span>
                    </dt>
                    <dd class="pf-v5-c-description-list__description">
                      <div class="pf-v5-c-description-list__text">1 second</div>
                    </dd>
                  </div>
                  <div class="pf-v5-c-description-list__group">
                    <dt class="pf-v5-c-description-list__term">
                      <span class="pf-v5-c-description-list__text">Max available</span>
                    </dt>
                    <dd class="pf-v5-c-description-list__description">
                      <div class="pf-v5-c-description-list__text">25% of 1 pod</div>
                    </dd>
                  </div>
                  <div class="pf-v5-c-description-list__group">
                    <dt class="pf-v5-c-description-list__term">
                      <span class="pf-v5-c-description-list__text">Max surge</span>
                    </dt>
                    <dd class="pf-v5-c-description-list__description">
                      <div
                        class="pf-v5-c-description-list__text"
                      >25% greater than 1 pod</div>
                    </dd>
                  </div>
                  <div class="pf-v5-c-description-list__group">
                    <dt class="pf-v5-c-description-list__term">
                      <span
                        class="pf-v5-c-description-list__text"
                      >Min ready seconds</span>
                    </dt>
                    <dd class="pf-v5-c-description-list__description">
                      <div
                        class="pf-v5-c-description-list__text pf-v5-u-color-200"
                      >Not configured</div>
                    </dd>
                  </div>
                  <div class="pf-v5-c-description-list__group">
                    <dt class="pf-v5-c-description-list__term">
                      <span class="pf-v5-c-description-list__text">Triggers</span>
                    </dt>
                    <dd class="pf-v5-c-description-list__description">
                      <div class="pf-v5-c-description-list__text">
                        ImageChange,
                        ConfigChange
                      </div>
                    </dd>
                  </div>
                </dl>
              </div>
            </section>
            <section
              class="pf-v5-c-tab-content"
              id="description-list-in-drawer-example-panel-tabs-tab2-panel"
              aria-labelledby="description-list-in-drawer-example-panel-tabs-tab2-link"
              role="tabpanel"
              tabindex="0"
              hidden
            >
              <div class="pf-v5-c-tab-content__body">Panel 2</div>
            </section>
          </div>
        </div>
      </div>
    </div>
  </div>
</div>

```

### Complex content

```html isFullscreen
<div class="pf-v5-c-page" id="description-list-complex-content-example">
  <div class="pf-v5-c-skip-to-content">
    <a
      class="pf-v5-c-button pf-m-primary"
      href="#main-content-description-list-complex-content-example"
    >Skip to content</a>
  </div>
  <header
    class="pf-v5-c-masthead"
    id="description-list-complex-content-example-masthead"
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
        id="description-list-complex-content-example-masthead-toolbar"
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
        id="description-list-complex-content-example-primary-nav"
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
    id="main-content-description-list-complex-content-example"
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
    <hr class="pf-v5-c-divider" />
    <section class="pf-v5-c-page__main-section pf-m-limit-width pf-m-light">
      <div class="pf-v5-c-page__main-body">
        <div class="pf-v5-l-grid pf-m-gutter">
          <div class="pf-v5-l-grid__item pf-m-5-col-on-lg pf-m-4-col-on-xl">
            <div class="pf-v5-l-grid pf-m-gutter">
              <div class="pf-v5-l-grid__item">
                <h2 class="pf-v5-c-title pf-m-lg">Service overview</h2>
              </div>
              <div class="pf-v5-l-grid__item">
                <dl class="pf-v5-c-description-list pf-m-2-col-on-xl">
                  <div class="pf-v5-c-description-list__group">
                    <dt class="pf-v5-c-description-list__term">
                      <span class="pf-v5-c-description-list__text">Name</span>
                    </dt>
                    <dd class="pf-v5-c-description-list__description">
                      <div class="pf-v5-c-description-list__text">mary-test</div>
                    </dd>
                  </div>
                  <div class="pf-v5-c-description-list__group">
                    <dt class="pf-v5-c-description-list__term">
                      <span class="pf-v5-c-description-list__text">Namespace</span>
                    </dt>
                    <dd class="pf-v5-c-description-list__description">
                      <div class="pf-v5-c-description-list__text">
                        <div class="pf-v5-l-flex pf-m-space-items-sm">
                          <div class="pf-v5-l-flex__item">
                            <span class="pf-v5-c-label pf-m-green">
                              <span class="pf-v5-c-label__content">
                                <span class="pf-v5-c-label__text">NS</span>
                              </span>
                            </span>
                          </div>
                          <div class="pf-v5-l-flex__item">
                            <a href="#">mary-test</a>
                          </div>
                        </div>
                      </div>
                    </dd>
                  </div>
                  <div class="pf-v5-c-description-list__group">
                    <dt class="pf-v5-c-description-list__term">
                      <span class="pf-v5-c-description-list__text">Labels</span>
                    </dt>
                    <dd class="pf-v5-c-description-list__description">
                      <div class="pf-v5-c-description-list__text">No labels</div>
                    </dd>
                  </div>
                  <div class="pf-v5-c-description-list__group">
                    <dt class="pf-v5-c-description-list__term">
                      <span class="pf-v5-c-description-list__text">Pod selector</span>
                    </dt>
                    <dd class="pf-v5-c-description-list__description">
                      <div class="pf-v5-c-description-list__text">
                        <a href="#">
                          <div class="pf-v5-l-flex pf-m-space-items-sm">
                            <div class="pf-v5-l-flex__item">
                              <i class="fas fa-search" aria-hidden="true"></i>
                            </div>
                            <div class="pf-v5-l-flex__item">
                              <span>app=MyApp</span>
                            </div>
                          </div>
                        </a>
                      </div>
                    </dd>
                  </div>
                  <div class="pf-v5-c-description-list__group">
                    <dt class="pf-v5-c-description-list__term">
                      <span class="pf-v5-c-description-list__text">Annotations</span>
                    </dt>
                    <dd class="pf-v5-c-description-list__description">
                      <div class="pf-v5-c-description-list__text">2 Annotations</div>
                    </dd>
                  </div>
                  <div class="pf-v5-c-description-list__group">
                    <dt class="pf-v5-c-description-list__term">
                      <span
                        class="pf-v5-c-description-list__text"
                      >Session affinity</span>
                    </dt>
                    <dd class="pf-v5-c-description-list__description">
                      <div class="pf-v5-c-description-list__text">None</div>
                    </dd>
                  </div>
                  <div class="pf-v5-c-description-list__group">
                    <dt class="pf-v5-c-description-list__term">
                      <span class="pf-v5-c-description-list__text">Created at:</span>
                    </dt>
                    <dd class="pf-v5-c-description-list__description">
                      <div class="pf-v5-c-description-list__text">3 minutes ago</div>
                    </dd>
                  </div>
                </dl>
              </div>
            </div>
          </div>
          <div class="pf-v5-l-grid__item pf-m-6-col-on-lg pf-m-4-col-on-xl">
            <div class="pf-v5-l-grid pf-m-gutter">
              <div class="pf-v5-l-grid__item">
                <h2 class="pf-v5-c-title pf-m-lg">Service routing</h2>
              </div>
              <div class="pf-v5-l-grid__item">
                <dl class="pf-v5-c-description-list">
                  <div class="pf-v5-c-description-list__group">
                    <dt class="pf-v5-c-description-list__term">
                      <span
                        class="pf-v5-c-description-list__text"
                      >Service address</span>
                    </dt>
                    <dd class="pf-v5-c-description-list__description">
                      <div class="pf-v5-c-description-list__text">
                        <table
                          class="pf-v5-c-table pf-m-grid-md pf-m-compact"
                          role="grid"
                          aria-label="Service address"
                          id="service-address"
                        >
                          <thead class="pf-v5-c-table__thead">
                            <tr class="pf-v5-c-table__tr" role="row">
                              <th
                                class="pf-v5-c-table__th"
                                role="columnheader"
                                scope="col"
                              >Type</th>
                              <th
                                class="pf-v5-c-table__th"
                                role="columnheader"
                                scope="col"
                              >Location</th>
                            </tr>
                          </thead>
                          <tbody class="pf-v5-c-table__tbody" role="rowgroup">
                            <tr class="pf-v5-c-table__tr" role="row">
                              <td
                                class="pf-v5-c-table__td"
                                role="cell"
                                data-label="Type"
                              >Cluster IP</td>
                              <td
                                class="pf-v5-c-table__td"
                                role="cell"
                                data-label="Location"
                              >172.30.126.106</td>
                            </tr>
                            <tr class="pf-v5-c-table__tr" role="row">
                              <td
                                class="pf-v5-c-table__td"
                                role="cell"
                                data-label="Type"
                              >Accessible within the cluster only</td>
                              <td
                                class="pf-v5-c-table__td"
                                role="cell"
                                data-label="Location"
                              >n/a</td>
                            </tr>
                          </tbody>
                        </table>
                      </div>
                    </dd>
                  </div>
                  <div class="pf-v5-c-description-list__group">
                    <dt class="pf-v5-c-description-list__term">
                      <span
                        class="pf-v5-c-description-list__text"
                      >Service port mapping</span>
                    </dt>
                    <dd class="pf-v5-c-description-list__description">
                      <div class="pf-v5-c-description-list__text">
                        <table
                          class="pf-v5-c-table pf-m-grid-md pf-m-compact"
                          role="grid"
                          aria-label="Service address"
                          id="service-port"
                        >
                          <thead class="pf-v5-c-table__thead">
                            <tr class="pf-v5-c-table__tr" role="row">
                              <th
                                class="pf-v5-c-table__th"
                                role="columnheader"
                                scope="col"
                              >Name</th>
                              <th
                                class="pf-v5-c-table__th"
                                role="columnheader"
                                scope="col"
                              >Port</th>
                              <th
                                class="pf-v5-c-table__th"
                                role="columnheader"
                                scope="col"
                              >Protocol</th>
                              <th
                                class="pf-v5-c-table__th"
                                role="columnheader"
                                scope="col"
                              >Pod port or name</th>
                            </tr>
                          </thead>
                          <tbody class="pf-v5-c-table__tbody" role="rowgroup">
                            <tr class="pf-v5-c-table__tr" role="row">
                              <td
                                class="pf-v5-c-table__td"
                                role="cell"
                                data-label="Name"
                              >--</td>
                              <td
                                class="pf-v5-c-table__td"
                                role="cell"
                                data-label="Port"
                              >
                                <div class="pf-v5-l-flex pf-m-space-items-sm">
                                  <div class="pf-v5-l-flex__item">
                                    <span
                                      class="pf-v5-c-label pf-m-compact pf-m-green"
                                    >
                                      <span class="pf-v5-c-label__content">
                                        <span class="pf-v5-c-label__text">S</span>
                                      </span>
                                    </span>
                                  </div>
                                  <div class="pf-v5-l-flex__item">80</div>
                                </div>
                              </td>
                              <td
                                class="pf-v5-c-table__td"
                                role="cell"
                                data-label="Protocol"
                              >TCP</td>
                              <td
                                class="pf-v5-c-table__td"
                                role="cell"
                                data-label="Pod port or name"
                              >
                                <div class="pf-v5-l-flex pf-m-space-items-sm">
                                  <div class="pf-v5-l-flex__item">
                                    <span
                                      class="pf-v5-c-label pf-m-compact pf-m-cyan"
                                    >
                                      <span class="pf-v5-c-label__content">
                                        <span class="pf-v5-c-label__text">P</span>
                                      </span>
                                    </span>
                                  </div>
                                  <div class="pf-v5-l-flex__item">80</div>
                                </div>
                              </td>
                            </tr>
                          </tbody>
                        </table>
                      </div>
                    </dd>
                  </div>
                </dl>
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>
  </main>
</div>

```
