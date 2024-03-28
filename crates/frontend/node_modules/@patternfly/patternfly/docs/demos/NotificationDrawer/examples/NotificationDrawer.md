---
id: Notification drawer
section: components
---## Demos

### Collapsed

```html isFullscreen
<div class="pf-v5-c-page" id="drawer-collapsed-example-page">
  <div class="pf-v5-c-skip-to-content">
    <a
      class="pf-v5-c-button pf-m-primary"
      href="#main-content-drawer-collapsed-example-page"
    >Skip to content</a>
  </div>
  <header class="pf-v5-c-masthead" id="drawer-collapsed-example-page-masthead">
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
        id="drawer-collapsed-example-page-masthead-toolbar"
      >
        <div class="pf-v5-c-toolbar__content">
          <div class="pf-v5-c-toolbar__content-section">
            <div
              class="pf-v5-c-toolbar__group pf-m-icon-button-group pf-m-align-right pf-m-spacer-none pf-m-spacer-md-on-md"
            >
              <div class="pf-v5-c-toolbar__item">
                <button
                  class="pf-v5-c-button pf-m-plain"
                  type="button"
                  aria-label="Notifications"
                >
                  <span class="pf-v5-c-notification-badge pf-m-read">
                    <i class="pf-v5-pficon-bell" aria-hidden="true"></i>
                  </span>
                </button>
              </div>
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
        id="drawer-collapsed-example-page-primary-nav"
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
    <div class="pf-v5-c-drawer">
      <div class="pf-v5-c-drawer__main">
        <div class="pf-v5-c-drawer__content">
          <div class="pf-v5-c-drawer__body">
            <main class="pf-v5-c-page__main" tabindex="-1">
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
        <div class="pf-v5-c-drawer__panel pf-m-width-33" hidden>
          <div class="pf-v5-c-drawer__body pf-m-no-padding">
            <div class="pf-v5-c-notification-drawer">
              <div class="pf-v5-c-notification-drawer__header">
                <h1
                  class="pf-v5-c-notification-drawer__header-title"
                >Notifications</h1>
                <span
                  class="pf-v5-c-notification-drawer__header-status"
                >0 unread</span>
                <div class="pf-v5-c-notification-drawer__header-action">
                  <div class="pf-v5-c-dropdown">
                    <button
                      class="pf-v5-c-dropdown__toggle pf-m-plain"
                      id="drawer-demo-notification-drawer-basic-header-action-button"
                      aria-expanded="false"
                      type="button"
                      aria-label="Actions"
                    >
                      <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
                    </button>
                    <ul
                      class="pf-v5-c-dropdown__menu pf-m-align-right"
                      aria-labelledby="drawer-demo-notification-drawer-basic-header-action-button"
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
              <div class="pf-v5-c-notification-drawer__body">
                <ul class="pf-v5-c-notification-drawer__list" role="list">
                  <li
                    class="pf-v5-c-notification-drawer__list-item pf-m-read pf-m-info"
                  >
                    <div class="pf-v5-c-notification-drawer__list-item-header">
                      <span
                        class="pf-v5-c-notification-drawer__list-item-header-icon"
                      >
                        <i class="fas fa-info-circle" aria-hidden="true"></i>
                      </span>
                      <h2
                        class="pf-v5-c-notification-drawer__list-item-header-title"
                      >
                        <span class="pf-v5-screen-reader">Info notification:</span>
                        Read
                        info notification title
                      </h2>
                    </div>
                    <div class="pf-v5-c-notification-drawer__list-item-action">
                      <div class="pf-v5-c-dropdown">
                        <button
                          class="pf-v5-c-dropdown__toggle pf-m-plain"
                          id="drawer-demo-notification-drawer-basicdropdown-kebab-1-button"
                          aria-expanded="false"
                          type="button"
                          aria-label="Actions"
                        >
                          <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
                        </button>
                        <ul
                          class="pf-v5-c-dropdown__menu pf-m-align-right"
                          aria-labelledby="drawer-demo-notification-drawer-basicdropdown-kebab-1-button"
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
                    <div
                      class="pf-v5-c-notification-drawer__list-item-description"
                    >This is an info notification description.</div>
                    <div
                      class="pf-v5-c-notification-drawer__list-item-timestamp"
                    >5 minutes ago</div>
                  </li>

                  <li
                    class="pf-v5-c-notification-drawer__list-item pf-m-read pf-m-custom"
                  >
                    <div class="pf-v5-c-notification-drawer__list-item-header">
                      <span
                        class="pf-v5-c-notification-drawer__list-item-header-icon"
                      >
                        <i class="fas fa-arrow-circle-up" aria-hidden="true"></i>
                      </span>
                      <h2
                        class="pf-v5-c-notification-drawer__list-item-header-title"
                      >
                        <span class="pf-v5-screen-reader">Custom notification:</span>
                        Read
                        recommendation notification title. This is a long title to show how the title will wrap if it is long and wraps to multiple lines.
                      </h2>
                    </div>
                    <div class="pf-v5-c-notification-drawer__list-item-action">
                      <div class="pf-v5-c-dropdown">
                        <button
                          class="pf-v5-c-dropdown__toggle pf-m-plain"
                          id="drawer-demo-notification-drawer-basicdropdown-kebab-2-button"
                          aria-expanded="false"
                          type="button"
                          aria-label="Actions"
                        >
                          <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
                        </button>
                        <ul
                          class="pf-v5-c-dropdown__menu pf-m-align-right"
                          aria-labelledby="drawer-demo-notification-drawer-basicdropdown-kebab-2-button"
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
                    <div
                      class="pf-v5-c-notification-drawer__list-item-description"
                    >This is a recommendation notification description. This is a long description to show how the title will wrap if it is long and wraps to multiple lines.</div>
                    <div
                      class="pf-v5-c-notification-drawer__list-item-timestamp"
                    >10 minutes ago</div>
                  </li>

                  <li
                    class="pf-v5-c-notification-drawer__list-item pf-m-read pf-m-custom"
                  >
                    <div class="pf-v5-c-notification-drawer__list-item-header">
                      <span
                        class="pf-v5-c-notification-drawer__list-item-header-icon"
                      >
                        <i class="fas fa-arrow-circle-up" aria-hidden="true"></i>
                      </span>
                      <h2
                        class="pf-v5-c-notification-drawer__list-item-header-title"
                      >
                        <span class="pf-v5-screen-reader">Custom notification:</span>
                        Read
                        recommendation notification title. This is a long title to show how the title will wrap if it is long and wraps to multiple lines.
                      </h2>
                    </div>
                    <div class="pf-v5-c-notification-drawer__list-item-action">
                      <div class="pf-v5-c-dropdown">
                        <button
                          class="pf-v5-c-dropdown__toggle pf-m-plain"
                          id="drawer-demo-notification-drawer-basicdropdown-kebab-3-button"
                          aria-expanded="false"
                          type="button"
                          aria-label="Actions"
                        >
                          <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
                        </button>
                        <ul
                          class="pf-v5-c-dropdown__menu pf-m-align-right"
                          aria-labelledby="drawer-demo-notification-drawer-basicdropdown-kebab-3-button"
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
                    <div
                      class="pf-v5-c-notification-drawer__list-item-description"
                    >This is a recommendation notification description. This is a long description to show how the title will wrap if it is long and wraps to multiple lines.</div>
                    <div
                      class="pf-v5-c-notification-drawer__list-item-timestamp"
                    >20 minutes ago</div>
                  </li>
                  <li
                    class="pf-v5-c-notification-drawer__list-item pf-m-read pf-m-warning pf-m-hoverable"
                  >
                    <div class="pf-v5-c-notification-drawer__list-item-header">
                      <span
                        class="pf-v5-c-notification-drawer__list-item-header-icon"
                      >
                        <i
                          class="fas fa-exclamation-triangle"
                          aria-hidden="true"
                        ></i>
                      </span>
                      <h2
                        class="pf-v5-c-notification-drawer__list-item-header-title"
                      >
                        <span class="pf-v5-screen-reader">Warning notification:</span>
                        Read warning notification title
                      </h2>
                    </div>
                    <div class="pf-v5-c-notification-drawer__list-item-action">
                      <div class="pf-v5-c-dropdown pf-m-top">
                        <button
                          class="pf-v5-c-dropdown__toggle pf-m-plain"
                          id="drawer-demo-notification-drawer-basicdropdown-kebab-4-button"
                          aria-expanded="false"
                          type="button"
                          aria-label="Actions"
                        >
                          <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
                        </button>
                        <ul
                          class="pf-v5-c-dropdown__menu pf-m-align-right"
                          aria-labelledby="drawer-demo-notification-drawer-basicdropdown-kebab-4-button"
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
                    <div
                      class="pf-v5-c-notification-drawer__list-item-description"
                    >This is a warning notification description.</div>
                    <div
                      class="pf-v5-c-notification-drawer__list-item-timestamp"
                    >20 minutes ago</div>
                  </li>
                  <li
                    class="pf-v5-c-notification-drawer__list-item pf-m-read pf-m-success pf-m-hoverable"
                  >
                    <div class="pf-v5-c-notification-drawer__list-item-header">
                      <span
                        class="pf-v5-c-notification-drawer__list-item-header-icon"
                      >
                        <i class="fas fa-check-circle" aria-hidden="true"></i>
                      </span>
                      <h2
                        class="pf-v5-c-notification-drawer__list-item-header-title"
                      >
                        <span class="pf-v5-screen-reader">Success notification:</span>
                        Read success notification title
                      </h2>
                    </div>
                    <div class="pf-v5-c-notification-drawer__list-item-action">
                      <div class="pf-v5-c-dropdown pf-m-top">
                        <button
                          class="pf-v5-c-dropdown__toggle pf-m-plain"
                          id="drawer-demo-notification-drawer-basicdropdown-kebab-5-button"
                          aria-expanded="false"
                          type="button"
                          aria-label="Actions"
                        >
                          <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
                        </button>
                        <ul
                          class="pf-v5-c-dropdown__menu pf-m-align-right"
                          aria-labelledby="drawer-demo-notification-drawer-basicdropdown-kebab-5-button"
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
                    <div
                      class="pf-v5-c-notification-drawer__list-item-description"
                    >This is a success notification description.</div>
                    <div
                      class="pf-v5-c-notification-drawer__list-item-timestamp"
                    >30 minutes ago</div>
                  </li>
                  <li
                    class="pf-v5-c-notification-drawer__list-item pf-m-read pf-m-success pf-m-hoverable"
                  >
                    <div class="pf-v5-c-notification-drawer__list-item-header">
                      <span
                        class="pf-v5-c-notification-drawer__list-item-header-icon"
                      >
                        <i class="fas fa-check-circle" aria-hidden="true"></i>
                      </span>
                      <h2
                        class="pf-v5-c-notification-drawer__list-item-header-title pf-m-truncate"
                      >
                        <span class="pf-v5-screen-reader">Success notification:</span>
                        Lorem ipsum dolor sit amet, consectetur adipiscing elit. Praesent quis odio risus. Ut dictum vitae sapien at posuere. Nullam suscipit massa quis lacus pellentesque scelerisque. Donec non maximus neque, quis ornare nunc. Vivamus in nibh sed libero feugiat feugiat. Nulla lacinia rutrum est, a commodo odio vestibulum suscipit. Nullam id quam et quam porttitor interdum quis nec tellus. Vestibulum arcu dui, pulvinar eu tellus in, semper mattis diam. Sed commodo tincidunt lacus non pulvinar. Curabitur tempor molestie vestibulum. Vivamus vel mi dignissim, efficitur neque eget, efficitur massa. Mauris vitae nunc augue. Donec augue lorem, malesuada et quam vitae, volutpat mattis nisi. Nullam nec venenatis ex, quis lobortis purus. Sed nisl dolor, mattis sit amet tincidunt quis, mollis sed massa.
                      </h2>
                    </div>
                    <div class="pf-v5-c-notification-drawer__list-item-action">
                      <div class="pf-v5-c-dropdown pf-m-top">
                        <button
                          class="pf-v5-c-dropdown__toggle pf-m-plain"
                          id="drawer-demo-notification-drawer-basicdropdown-kebab-6-button"
                          aria-expanded="false"
                          type="button"
                          aria-label="Actions"
                        >
                          <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
                        </button>
                        <ul
                          class="pf-v5-c-dropdown__menu pf-m-align-right"
                          aria-labelledby="drawer-demo-notification-drawer-basicdropdown-kebab-6-button"
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
                    <div
                      class="pf-v5-c-notification-drawer__list-item-description"
                    >This example uses ".pf-m-truncate" to limit the title to a single line and truncate any overflow text with ellipses.</div>
                    <div
                      class="pf-v5-c-notification-drawer__list-item-timestamp"
                    >40 minutes ago</div>
                  </li>
                  <li
                    class="pf-v5-c-notification-drawer__list-item pf-m-read pf-m-success pf-m-hoverable"
                  >
                    <div class="pf-v5-c-notification-drawer__list-item-header">
                      <span
                        class="pf-v5-c-notification-drawer__list-item-header-icon"
                      >
                        <i class="fas fa-check-circle" aria-hidden="true"></i>
                      </span>
                      <h2
                        class="pf-v5-c-notification-drawer__list-item-header-title pf-m-truncate"
                        style="--pf-v5-c-notification-drawer__list-item-header-title--max-lines: 2"
                      >
                        <span class="pf-v5-screen-reader">Success notification:</span>
                        Lorem ipsum dolor sit amet, consectetur adipiscing elit. Praesent quis odio risus. Ut dictum vitae sapien at posuere. Nullam suscipit massa quis lacus pellentesque scelerisque. Donec non maximus neque, quis ornare nunc. Vivamus in nibh sed libero feugiat feugiat. Nulla lacinia rutrum est, a commodo odio vestibulum suscipit. Nullam id quam et quam porttitor interdum quis nec tellus. Vestibulum arcu dui, pulvinar eu tellus in, semper mattis diam. Sed commodo tincidunt lacus non pulvinar. Curabitur tempor molestie vestibulum. Vivamus vel mi dignissim, efficitur neque eget, efficitur massa. Mauris vitae nunc augue. Donec augue lorem, malesuada et quam vitae, volutpat mattis nisi. Nullam nec venenatis ex, quis lobortis purus. Sed nisl dolor, mattis sit amet tincidunt quis, mollis sed massa.
                      </h2>
                    </div>
                    <div class="pf-v5-c-notification-drawer__list-item-action">
                      <div class="pf-v5-c-dropdown pf-m-top">
                        <button
                          class="pf-v5-c-dropdown__toggle pf-m-plain"
                          id="drawer-demo-notification-drawer-basicdropdown-kebab-7-button"
                          aria-expanded="false"
                          type="button"
                          aria-label="Actions"
                        >
                          <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
                        </button>
                        <ul
                          class="pf-v5-c-dropdown__menu pf-m-align-right"
                          aria-labelledby="drawer-demo-notification-drawer-basicdropdown-kebab-7-button"
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
                    <div
                      class="pf-v5-c-notification-drawer__list-item-description"
                    >This example uses ".pf-m-truncate" and sets "--pf-v5-c-notification-drawer__list-item-header-title--max-lines: 2" to limit title to two lines and truncate any overflow text with ellipses.</div>
                    <div
                      class="pf-v5-c-notification-drawer__list-item-timestamp"
                    >50 minutes ago</div>
                  </li>
                </ul>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</div>

```

### Expanded read

```html isFullscreen
<div class="pf-v5-c-page" id="drawer-expanded-read-example-page">
  <div class="pf-v5-c-skip-to-content">
    <a
      class="pf-v5-c-button pf-m-primary"
      href="#main-content-drawer-expanded-read-example-page"
    >Skip to content</a>
  </div>
  <header
    class="pf-v5-c-masthead"
    id="drawer-expanded-read-example-page-masthead"
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
        id="drawer-expanded-read-example-page-masthead-toolbar"
      >
        <div class="pf-v5-c-toolbar__content">
          <div class="pf-v5-c-toolbar__content-section">
            <div
              class="pf-v5-c-toolbar__group pf-m-icon-button-group pf-m-align-right pf-m-spacer-none pf-m-spacer-md-on-md"
            >
              <div class="pf-v5-c-toolbar__item">
                <button
                  class="pf-v5-c-button pf-m-plain"
                  type="button"
                  aria-expanded="true"
                  aria-label="Notifications"
                >
                  <span
                    class="pf-v5-c-notification-badge pf-m-read pf-m-expanded"
                  >
                    <i class="pf-v5-pficon-bell" aria-hidden="true"></i>
                  </span>
                </button>
              </div>
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
        id="drawer-expanded-read-example-page-primary-nav"
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
            <main class="pf-v5-c-page__main" tabindex="-1">
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
        <div class="pf-v5-c-drawer__panel pf-m-width-33">
          <div class="pf-v5-c-drawer__body pf-m-no-padding">
            <div class="pf-v5-c-notification-drawer">
              <div class="pf-v5-c-notification-drawer__header">
                <h1
                  class="pf-v5-c-notification-drawer__header-title"
                >Notifications</h1>
                <span
                  class="pf-v5-c-notification-drawer__header-status"
                >0 unread</span>
                <div class="pf-v5-c-notification-drawer__header-action">
                  <div class="pf-v5-c-dropdown">
                    <button
                      class="pf-v5-c-dropdown__toggle pf-m-plain"
                      id="drawer-demo-notification-drawer-basic-header-action-button"
                      aria-expanded="false"
                      type="button"
                      aria-label="Actions"
                    >
                      <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
                    </button>
                    <ul
                      class="pf-v5-c-dropdown__menu pf-m-align-right"
                      aria-labelledby="drawer-demo-notification-drawer-basic-header-action-button"
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
              <div class="pf-v5-c-notification-drawer__body">
                <ul class="pf-v5-c-notification-drawer__list" role="list">
                  <li
                    class="pf-v5-c-notification-drawer__list-item pf-m-read pf-m-info"
                  >
                    <div class="pf-v5-c-notification-drawer__list-item-header">
                      <span
                        class="pf-v5-c-notification-drawer__list-item-header-icon"
                      >
                        <i class="fas fa-info-circle" aria-hidden="true"></i>
                      </span>
                      <h2
                        class="pf-v5-c-notification-drawer__list-item-header-title"
                      >
                        <span class="pf-v5-screen-reader">Info notification:</span>
                        Read
                        info notification title
                      </h2>
                    </div>
                    <div class="pf-v5-c-notification-drawer__list-item-action">
                      <div class="pf-v5-c-dropdown">
                        <button
                          class="pf-v5-c-dropdown__toggle pf-m-plain"
                          id="drawer-demo-notification-drawer-basicdropdown-kebab-1-button"
                          aria-expanded="false"
                          type="button"
                          aria-label="Actions"
                        >
                          <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
                        </button>
                        <ul
                          class="pf-v5-c-dropdown__menu pf-m-align-right"
                          aria-labelledby="drawer-demo-notification-drawer-basicdropdown-kebab-1-button"
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
                    <div
                      class="pf-v5-c-notification-drawer__list-item-description"
                    >This is an info notification description.</div>
                    <div
                      class="pf-v5-c-notification-drawer__list-item-timestamp"
                    >5 minutes ago</div>
                  </li>

                  <li
                    class="pf-v5-c-notification-drawer__list-item pf-m-read pf-m-custom"
                  >
                    <div class="pf-v5-c-notification-drawer__list-item-header">
                      <span
                        class="pf-v5-c-notification-drawer__list-item-header-icon"
                      >
                        <i class="fas fa-arrow-circle-up" aria-hidden="true"></i>
                      </span>
                      <h2
                        class="pf-v5-c-notification-drawer__list-item-header-title"
                      >
                        <span class="pf-v5-screen-reader">Custom notification:</span>
                        Read
                        recommendation notification title. This is a long title to show how the title will wrap if it is long and wraps to multiple lines.
                      </h2>
                    </div>
                    <div class="pf-v5-c-notification-drawer__list-item-action">
                      <div class="pf-v5-c-dropdown">
                        <button
                          class="pf-v5-c-dropdown__toggle pf-m-plain"
                          id="drawer-demo-notification-drawer-basicdropdown-kebab-2-button"
                          aria-expanded="false"
                          type="button"
                          aria-label="Actions"
                        >
                          <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
                        </button>
                        <ul
                          class="pf-v5-c-dropdown__menu pf-m-align-right"
                          aria-labelledby="drawer-demo-notification-drawer-basicdropdown-kebab-2-button"
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
                    <div
                      class="pf-v5-c-notification-drawer__list-item-description"
                    >This is a recommendation notification description. This is a long description to show how the title will wrap if it is long and wraps to multiple lines.</div>
                    <div
                      class="pf-v5-c-notification-drawer__list-item-timestamp"
                    >10 minutes ago</div>
                  </li>

                  <li
                    class="pf-v5-c-notification-drawer__list-item pf-m-read pf-m-custom"
                  >
                    <div class="pf-v5-c-notification-drawer__list-item-header">
                      <span
                        class="pf-v5-c-notification-drawer__list-item-header-icon"
                      >
                        <i class="fas fa-arrow-circle-up" aria-hidden="true"></i>
                      </span>
                      <h2
                        class="pf-v5-c-notification-drawer__list-item-header-title"
                      >
                        <span class="pf-v5-screen-reader">Custom notification:</span>
                        Read
                        recommendation notification title. This is a long title to show how the title will wrap if it is long and wraps to multiple lines.
                      </h2>
                    </div>
                    <div class="pf-v5-c-notification-drawer__list-item-action">
                      <div class="pf-v5-c-dropdown">
                        <button
                          class="pf-v5-c-dropdown__toggle pf-m-plain"
                          id="drawer-demo-notification-drawer-basicdropdown-kebab-3-button"
                          aria-expanded="false"
                          type="button"
                          aria-label="Actions"
                        >
                          <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
                        </button>
                        <ul
                          class="pf-v5-c-dropdown__menu pf-m-align-right"
                          aria-labelledby="drawer-demo-notification-drawer-basicdropdown-kebab-3-button"
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
                    <div
                      class="pf-v5-c-notification-drawer__list-item-description"
                    >This is a recommendation notification description. This is a long description to show how the title will wrap if it is long and wraps to multiple lines.</div>
                    <div
                      class="pf-v5-c-notification-drawer__list-item-timestamp"
                    >20 minutes ago</div>
                  </li>
                  <li
                    class="pf-v5-c-notification-drawer__list-item pf-m-read pf-m-warning pf-m-hoverable"
                  >
                    <div class="pf-v5-c-notification-drawer__list-item-header">
                      <span
                        class="pf-v5-c-notification-drawer__list-item-header-icon"
                      >
                        <i
                          class="fas fa-exclamation-triangle"
                          aria-hidden="true"
                        ></i>
                      </span>
                      <h2
                        class="pf-v5-c-notification-drawer__list-item-header-title"
                      >
                        <span class="pf-v5-screen-reader">Warning notification:</span>
                        Read warning notification title
                      </h2>
                    </div>
                    <div class="pf-v5-c-notification-drawer__list-item-action">
                      <div class="pf-v5-c-dropdown pf-m-top">
                        <button
                          class="pf-v5-c-dropdown__toggle pf-m-plain"
                          id="drawer-demo-notification-drawer-basicdropdown-kebab-4-button"
                          aria-expanded="false"
                          type="button"
                          aria-label="Actions"
                        >
                          <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
                        </button>
                        <ul
                          class="pf-v5-c-dropdown__menu pf-m-align-right"
                          aria-labelledby="drawer-demo-notification-drawer-basicdropdown-kebab-4-button"
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
                    <div
                      class="pf-v5-c-notification-drawer__list-item-description"
                    >This is a warning notification description.</div>
                    <div
                      class="pf-v5-c-notification-drawer__list-item-timestamp"
                    >20 minutes ago</div>
                  </li>
                  <li
                    class="pf-v5-c-notification-drawer__list-item pf-m-read pf-m-success pf-m-hoverable"
                  >
                    <div class="pf-v5-c-notification-drawer__list-item-header">
                      <span
                        class="pf-v5-c-notification-drawer__list-item-header-icon"
                      >
                        <i class="fas fa-check-circle" aria-hidden="true"></i>
                      </span>
                      <h2
                        class="pf-v5-c-notification-drawer__list-item-header-title"
                      >
                        <span class="pf-v5-screen-reader">Success notification:</span>
                        Read success notification title
                      </h2>
                    </div>
                    <div class="pf-v5-c-notification-drawer__list-item-action">
                      <div class="pf-v5-c-dropdown pf-m-top">
                        <button
                          class="pf-v5-c-dropdown__toggle pf-m-plain"
                          id="drawer-demo-notification-drawer-basicdropdown-kebab-5-button"
                          aria-expanded="false"
                          type="button"
                          aria-label="Actions"
                        >
                          <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
                        </button>
                        <ul
                          class="pf-v5-c-dropdown__menu pf-m-align-right"
                          aria-labelledby="drawer-demo-notification-drawer-basicdropdown-kebab-5-button"
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
                    <div
                      class="pf-v5-c-notification-drawer__list-item-description"
                    >This is a success notification description.</div>
                    <div
                      class="pf-v5-c-notification-drawer__list-item-timestamp"
                    >30 minutes ago</div>
                  </li>
                  <li
                    class="pf-v5-c-notification-drawer__list-item pf-m-read pf-m-success pf-m-hoverable"
                  >
                    <div class="pf-v5-c-notification-drawer__list-item-header">
                      <span
                        class="pf-v5-c-notification-drawer__list-item-header-icon"
                      >
                        <i class="fas fa-check-circle" aria-hidden="true"></i>
                      </span>
                      <h2
                        class="pf-v5-c-notification-drawer__list-item-header-title pf-m-truncate"
                      >
                        <span class="pf-v5-screen-reader">Success notification:</span>
                        Lorem ipsum dolor sit amet, consectetur adipiscing elit. Praesent quis odio risus. Ut dictum vitae sapien at posuere. Nullam suscipit massa quis lacus pellentesque scelerisque. Donec non maximus neque, quis ornare nunc. Vivamus in nibh sed libero feugiat feugiat. Nulla lacinia rutrum est, a commodo odio vestibulum suscipit. Nullam id quam et quam porttitor interdum quis nec tellus. Vestibulum arcu dui, pulvinar eu tellus in, semper mattis diam. Sed commodo tincidunt lacus non pulvinar. Curabitur tempor molestie vestibulum. Vivamus vel mi dignissim, efficitur neque eget, efficitur massa. Mauris vitae nunc augue. Donec augue lorem, malesuada et quam vitae, volutpat mattis nisi. Nullam nec venenatis ex, quis lobortis purus. Sed nisl dolor, mattis sit amet tincidunt quis, mollis sed massa.
                      </h2>
                    </div>
                    <div class="pf-v5-c-notification-drawer__list-item-action">
                      <div class="pf-v5-c-dropdown pf-m-top">
                        <button
                          class="pf-v5-c-dropdown__toggle pf-m-plain"
                          id="drawer-demo-notification-drawer-basicdropdown-kebab-6-button"
                          aria-expanded="false"
                          type="button"
                          aria-label="Actions"
                        >
                          <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
                        </button>
                        <ul
                          class="pf-v5-c-dropdown__menu pf-m-align-right"
                          aria-labelledby="drawer-demo-notification-drawer-basicdropdown-kebab-6-button"
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
                    <div
                      class="pf-v5-c-notification-drawer__list-item-description"
                    >This example uses ".pf-m-truncate" to limit the title to a single line and truncate any overflow text with ellipses.</div>
                    <div
                      class="pf-v5-c-notification-drawer__list-item-timestamp"
                    >40 minutes ago</div>
                  </li>
                  <li
                    class="pf-v5-c-notification-drawer__list-item pf-m-read pf-m-success pf-m-hoverable"
                  >
                    <div class="pf-v5-c-notification-drawer__list-item-header">
                      <span
                        class="pf-v5-c-notification-drawer__list-item-header-icon"
                      >
                        <i class="fas fa-check-circle" aria-hidden="true"></i>
                      </span>
                      <h2
                        class="pf-v5-c-notification-drawer__list-item-header-title pf-m-truncate"
                        style="--pf-v5-c-notification-drawer__list-item-header-title--max-lines: 2"
                      >
                        <span class="pf-v5-screen-reader">Success notification:</span>
                        Lorem ipsum dolor sit amet, consectetur adipiscing elit. Praesent quis odio risus. Ut dictum vitae sapien at posuere. Nullam suscipit massa quis lacus pellentesque scelerisque. Donec non maximus neque, quis ornare nunc. Vivamus in nibh sed libero feugiat feugiat. Nulla lacinia rutrum est, a commodo odio vestibulum suscipit. Nullam id quam et quam porttitor interdum quis nec tellus. Vestibulum arcu dui, pulvinar eu tellus in, semper mattis diam. Sed commodo tincidunt lacus non pulvinar. Curabitur tempor molestie vestibulum. Vivamus vel mi dignissim, efficitur neque eget, efficitur massa. Mauris vitae nunc augue. Donec augue lorem, malesuada et quam vitae, volutpat mattis nisi. Nullam nec venenatis ex, quis lobortis purus. Sed nisl dolor, mattis sit amet tincidunt quis, mollis sed massa.
                      </h2>
                    </div>
                    <div class="pf-v5-c-notification-drawer__list-item-action">
                      <div class="pf-v5-c-dropdown pf-m-top">
                        <button
                          class="pf-v5-c-dropdown__toggle pf-m-plain"
                          id="drawer-demo-notification-drawer-basicdropdown-kebab-7-button"
                          aria-expanded="false"
                          type="button"
                          aria-label="Actions"
                        >
                          <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
                        </button>
                        <ul
                          class="pf-v5-c-dropdown__menu pf-m-align-right"
                          aria-labelledby="drawer-demo-notification-drawer-basicdropdown-kebab-7-button"
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
                    <div
                      class="pf-v5-c-notification-drawer__list-item-description"
                    >This example uses ".pf-m-truncate" and sets "--pf-v5-c-notification-drawer__list-item-header-title--max-lines: 2" to limit title to two lines and truncate any overflow text with ellipses.</div>
                    <div
                      class="pf-v5-c-notification-drawer__list-item-timestamp"
                    >50 minutes ago</div>
                  </li>
                </ul>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</div>

```

### Expanded unread

```html isFullscreen
<div class="pf-v5-c-page" id="drawer-expanded-unread-example-page">
  <div class="pf-v5-c-skip-to-content">
    <a
      class="pf-v5-c-button pf-m-primary"
      href="#main-content-drawer-expanded-unread-example-page"
    >Skip to content</a>
  </div>
  <header
    class="pf-v5-c-masthead"
    id="drawer-expanded-unread-example-page-masthead"
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
        id="drawer-expanded-unread-example-page-masthead-toolbar"
      >
        <div class="pf-v5-c-toolbar__content">
          <div class="pf-v5-c-toolbar__content-section">
            <div
              class="pf-v5-c-toolbar__group pf-m-icon-button-group pf-m-align-right pf-m-spacer-none pf-m-spacer-md-on-md"
            >
              <div class="pf-v5-c-toolbar__item">
                <button
                  class="pf-v5-c-button pf-m-plain"
                  type="button"
                  aria-expanded="true"
                  aria-label="Unread notifications"
                >
                  <span
                    class="pf-v5-c-notification-badge pf-m-unread pf-m-expanded"
                  >
                    <i class="pf-v5-pficon-bell" aria-hidden="true"></i>
                  </span>
                </button>
              </div>
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
        id="drawer-expanded-unread-example-page-primary-nav"
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
            <main class="pf-v5-c-page__main" tabindex="-1">
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
        <div class="pf-v5-c-drawer__panel pf-m-width-33">
          <div class="pf-v5-c-drawer__body pf-m-no-padding">
            <div class="pf-v5-c-notification-drawer">
              <div class="pf-v5-c-notification-drawer__header">
                <h1
                  class="pf-v5-c-notification-drawer__header-title"
                >Notifications</h1>
                <span
                  class="pf-v5-c-notification-drawer__header-status"
                >3 unread</span>
                <div class="pf-v5-c-notification-drawer__header-action">
                  <div class="pf-v5-c-dropdown">
                    <button
                      class="pf-v5-c-dropdown__toggle pf-m-plain"
                      id="drawer-demo-notification-drawer-basic-header-action-button"
                      aria-expanded="false"
                      type="button"
                      aria-label="Actions"
                    >
                      <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
                    </button>
                    <ul
                      class="pf-v5-c-dropdown__menu pf-m-align-right"
                      aria-labelledby="drawer-demo-notification-drawer-basic-header-action-button"
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
              <div class="pf-v5-c-notification-drawer__body">
                <ul class="pf-v5-c-notification-drawer__list" role="list">
                  <li
                    class="pf-v5-c-notification-drawer__list-item pf-m-hoverable pf-m-info"
                    tabindex="0"
                  >
                    <div class="pf-v5-c-notification-drawer__list-item-header">
                      <span
                        class="pf-v5-c-notification-drawer__list-item-header-icon"
                      >
                        <i class="fas fa-info-circle" aria-hidden="true"></i>
                      </span>
                      <h2
                        class="pf-v5-c-notification-drawer__list-item-header-title"
                      >
                        <span class="pf-v5-screen-reader">Info notification:</span>
                        Unread
                        info notification title
                      </h2>
                    </div>
                    <div class="pf-v5-c-notification-drawer__list-item-action">
                      <div class="pf-v5-c-dropdown">
                        <button
                          class="pf-v5-c-dropdown__toggle pf-m-plain"
                          id="drawer-demo-notification-drawer-basicdropdown-kebab-1-button"
                          aria-expanded="false"
                          type="button"
                          aria-label="Actions"
                        >
                          <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
                        </button>
                        <ul
                          class="pf-v5-c-dropdown__menu pf-m-align-right"
                          aria-labelledby="drawer-demo-notification-drawer-basicdropdown-kebab-1-button"
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
                    <div
                      class="pf-v5-c-notification-drawer__list-item-description"
                    >This is an info notification description.</div>
                    <div
                      class="pf-v5-c-notification-drawer__list-item-timestamp"
                    >5 minutes ago</div>
                  </li>

                  <li
                    class="pf-v5-c-notification-drawer__list-item pf-m-hoverable pf-m-custom"
                    tabindex="0"
                  >
                    <div class="pf-v5-c-notification-drawer__list-item-header">
                      <span
                        class="pf-v5-c-notification-drawer__list-item-header-icon"
                      >
                        <i class="fas fa-arrow-circle-up" aria-hidden="true"></i>
                      </span>
                      <h2
                        class="pf-v5-c-notification-drawer__list-item-header-title"
                      >
                        <span class="pf-v5-screen-reader">Custom notification:</span>
                        Unread
                        recommendation notification title. This is a long title to show how the title will wrap if it is long and wraps to multiple lines.
                      </h2>
                    </div>
                    <div class="pf-v5-c-notification-drawer__list-item-action">
                      <div class="pf-v5-c-dropdown">
                        <button
                          class="pf-v5-c-dropdown__toggle pf-m-plain"
                          id="drawer-demo-notification-drawer-basicdropdown-kebab-2-button"
                          aria-expanded="false"
                          type="button"
                          aria-label="Actions"
                        >
                          <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
                        </button>
                        <ul
                          class="pf-v5-c-dropdown__menu pf-m-align-right"
                          aria-labelledby="drawer-demo-notification-drawer-basicdropdown-kebab-2-button"
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
                    <div
                      class="pf-v5-c-notification-drawer__list-item-description"
                    >This is a recommendation notification description. This is a long description to show how the title will wrap if it is long and wraps to multiple lines.</div>
                    <div
                      class="pf-v5-c-notification-drawer__list-item-timestamp"
                    >10 minutes ago</div>
                  </li>

                  <li
                    class="pf-v5-c-notification-drawer__list-item pf-m-hoverable pf-m-custom"
                    tabindex="0"
                  >
                    <div class="pf-v5-c-notification-drawer__list-item-header">
                      <span
                        class="pf-v5-c-notification-drawer__list-item-header-icon"
                      >
                        <i class="fas fa-arrow-circle-up" aria-hidden="true"></i>
                      </span>
                      <h2
                        class="pf-v5-c-notification-drawer__list-item-header-title"
                      >
                        <span class="pf-v5-screen-reader">Custom notification:</span>
                        Unread
                        recommendation notification title. This is a long title to show how the title will wrap if it is long and wraps to multiple lines.
                      </h2>
                    </div>
                    <div class="pf-v5-c-notification-drawer__list-item-action">
                      <div class="pf-v5-c-dropdown">
                        <button
                          class="pf-v5-c-dropdown__toggle pf-m-plain"
                          id="drawer-demo-notification-drawer-basicdropdown-kebab-3-button"
                          aria-expanded="false"
                          type="button"
                          aria-label="Actions"
                        >
                          <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
                        </button>
                        <ul
                          class="pf-v5-c-dropdown__menu pf-m-align-right"
                          aria-labelledby="drawer-demo-notification-drawer-basicdropdown-kebab-3-button"
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
                    <div
                      class="pf-v5-c-notification-drawer__list-item-description"
                    >This is a recommendation notification description. This is a long description to show how the title will wrap if it is long and wraps to multiple lines.</div>
                    <div
                      class="pf-v5-c-notification-drawer__list-item-timestamp"
                    >20 minutes ago</div>
                  </li>
                  <li
                    class="pf-v5-c-notification-drawer__list-item pf-m-read pf-m-warning pf-m-hoverable"
                  >
                    <div class="pf-v5-c-notification-drawer__list-item-header">
                      <span
                        class="pf-v5-c-notification-drawer__list-item-header-icon"
                      >
                        <i
                          class="fas fa-exclamation-triangle"
                          aria-hidden="true"
                        ></i>
                      </span>
                      <h2
                        class="pf-v5-c-notification-drawer__list-item-header-title"
                      >
                        <span class="pf-v5-screen-reader">Warning notification:</span>
                        Read warning notification title
                      </h2>
                    </div>
                    <div class="pf-v5-c-notification-drawer__list-item-action">
                      <div class="pf-v5-c-dropdown pf-m-top">
                        <button
                          class="pf-v5-c-dropdown__toggle pf-m-plain"
                          id="drawer-demo-notification-drawer-basicdropdown-kebab-4-button"
                          aria-expanded="false"
                          type="button"
                          aria-label="Actions"
                        >
                          <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
                        </button>
                        <ul
                          class="pf-v5-c-dropdown__menu pf-m-align-right"
                          aria-labelledby="drawer-demo-notification-drawer-basicdropdown-kebab-4-button"
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
                    <div
                      class="pf-v5-c-notification-drawer__list-item-description"
                    >This is a warning notification description.</div>
                    <div
                      class="pf-v5-c-notification-drawer__list-item-timestamp"
                    >20 minutes ago</div>
                  </li>
                  <li
                    class="pf-v5-c-notification-drawer__list-item pf-m-read pf-m-success pf-m-hoverable"
                  >
                    <div class="pf-v5-c-notification-drawer__list-item-header">
                      <span
                        class="pf-v5-c-notification-drawer__list-item-header-icon"
                      >
                        <i class="fas fa-check-circle" aria-hidden="true"></i>
                      </span>
                      <h2
                        class="pf-v5-c-notification-drawer__list-item-header-title"
                      >
                        <span class="pf-v5-screen-reader">Success notification:</span>
                        Read success notification title
                      </h2>
                    </div>
                    <div class="pf-v5-c-notification-drawer__list-item-action">
                      <div class="pf-v5-c-dropdown pf-m-top">
                        <button
                          class="pf-v5-c-dropdown__toggle pf-m-plain"
                          id="drawer-demo-notification-drawer-basicdropdown-kebab-5-button"
                          aria-expanded="false"
                          type="button"
                          aria-label="Actions"
                        >
                          <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
                        </button>
                        <ul
                          class="pf-v5-c-dropdown__menu pf-m-align-right"
                          aria-labelledby="drawer-demo-notification-drawer-basicdropdown-kebab-5-button"
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
                    <div
                      class="pf-v5-c-notification-drawer__list-item-description"
                    >This is a success notification description.</div>
                    <div
                      class="pf-v5-c-notification-drawer__list-item-timestamp"
                    >30 minutes ago</div>
                  </li>
                  <li
                    class="pf-v5-c-notification-drawer__list-item pf-m-read pf-m-success pf-m-hoverable"
                  >
                    <div class="pf-v5-c-notification-drawer__list-item-header">
                      <span
                        class="pf-v5-c-notification-drawer__list-item-header-icon"
                      >
                        <i class="fas fa-check-circle" aria-hidden="true"></i>
                      </span>
                      <h2
                        class="pf-v5-c-notification-drawer__list-item-header-title pf-m-truncate"
                      >
                        <span class="pf-v5-screen-reader">Success notification:</span>
                        Lorem ipsum dolor sit amet, consectetur adipiscing elit. Praesent quis odio risus. Ut dictum vitae sapien at posuere. Nullam suscipit massa quis lacus pellentesque scelerisque. Donec non maximus neque, quis ornare nunc. Vivamus in nibh sed libero feugiat feugiat. Nulla lacinia rutrum est, a commodo odio vestibulum suscipit. Nullam id quam et quam porttitor interdum quis nec tellus. Vestibulum arcu dui, pulvinar eu tellus in, semper mattis diam. Sed commodo tincidunt lacus non pulvinar. Curabitur tempor molestie vestibulum. Vivamus vel mi dignissim, efficitur neque eget, efficitur massa. Mauris vitae nunc augue. Donec augue lorem, malesuada et quam vitae, volutpat mattis nisi. Nullam nec venenatis ex, quis lobortis purus. Sed nisl dolor, mattis sit amet tincidunt quis, mollis sed massa.
                      </h2>
                    </div>
                    <div class="pf-v5-c-notification-drawer__list-item-action">
                      <div class="pf-v5-c-dropdown pf-m-top">
                        <button
                          class="pf-v5-c-dropdown__toggle pf-m-plain"
                          id="drawer-demo-notification-drawer-basicdropdown-kebab-6-button"
                          aria-expanded="false"
                          type="button"
                          aria-label="Actions"
                        >
                          <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
                        </button>
                        <ul
                          class="pf-v5-c-dropdown__menu pf-m-align-right"
                          aria-labelledby="drawer-demo-notification-drawer-basicdropdown-kebab-6-button"
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
                    <div
                      class="pf-v5-c-notification-drawer__list-item-description"
                    >This example uses ".pf-m-truncate" to limit the title to a single line and truncate any overflow text with ellipses.</div>
                    <div
                      class="pf-v5-c-notification-drawer__list-item-timestamp"
                    >40 minutes ago</div>
                  </li>
                  <li
                    class="pf-v5-c-notification-drawer__list-item pf-m-read pf-m-success pf-m-hoverable"
                  >
                    <div class="pf-v5-c-notification-drawer__list-item-header">
                      <span
                        class="pf-v5-c-notification-drawer__list-item-header-icon"
                      >
                        <i class="fas fa-check-circle" aria-hidden="true"></i>
                      </span>
                      <h2
                        class="pf-v5-c-notification-drawer__list-item-header-title pf-m-truncate"
                        style="--pf-v5-c-notification-drawer__list-item-header-title--max-lines: 2"
                      >
                        <span class="pf-v5-screen-reader">Success notification:</span>
                        Lorem ipsum dolor sit amet, consectetur adipiscing elit. Praesent quis odio risus. Ut dictum vitae sapien at posuere. Nullam suscipit massa quis lacus pellentesque scelerisque. Donec non maximus neque, quis ornare nunc. Vivamus in nibh sed libero feugiat feugiat. Nulla lacinia rutrum est, a commodo odio vestibulum suscipit. Nullam id quam et quam porttitor interdum quis nec tellus. Vestibulum arcu dui, pulvinar eu tellus in, semper mattis diam. Sed commodo tincidunt lacus non pulvinar. Curabitur tempor molestie vestibulum. Vivamus vel mi dignissim, efficitur neque eget, efficitur massa. Mauris vitae nunc augue. Donec augue lorem, malesuada et quam vitae, volutpat mattis nisi. Nullam nec venenatis ex, quis lobortis purus. Sed nisl dolor, mattis sit amet tincidunt quis, mollis sed massa.
                      </h2>
                    </div>
                    <div class="pf-v5-c-notification-drawer__list-item-action">
                      <div class="pf-v5-c-dropdown pf-m-top">
                        <button
                          class="pf-v5-c-dropdown__toggle pf-m-plain"
                          id="drawer-demo-notification-drawer-basicdropdown-kebab-7-button"
                          aria-expanded="false"
                          type="button"
                          aria-label="Actions"
                        >
                          <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
                        </button>
                        <ul
                          class="pf-v5-c-dropdown__menu pf-m-align-right"
                          aria-labelledby="drawer-demo-notification-drawer-basicdropdown-kebab-7-button"
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
                    <div
                      class="pf-v5-c-notification-drawer__list-item-description"
                    >This example uses ".pf-m-truncate" and sets "--pf-v5-c-notification-drawer__list-item-header-title--max-lines: 2" to limit title to two lines and truncate any overflow text with ellipses.</div>
                    <div
                      class="pf-v5-c-notification-drawer__list-item-timestamp"
                    >50 minutes ago</div>
                  </li>
                </ul>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</div>

```

### Expanded attention

```html isFullscreen
<div class="pf-v5-c-page" id="drawer-expanded-attention-example-page">
  <div class="pf-v5-c-skip-to-content">
    <a
      class="pf-v5-c-button pf-m-primary"
      href="#main-content-drawer-expanded-attention-example-page"
    >Skip to content</a>
  </div>
  <header
    class="pf-v5-c-masthead"
    id="drawer-expanded-attention-example-page-masthead"
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
        id="drawer-expanded-attention-example-page-masthead-toolbar"
      >
        <div class="pf-v5-c-toolbar__content">
          <div class="pf-v5-c-toolbar__content-section">
            <div
              class="pf-v5-c-toolbar__group pf-m-icon-button-group pf-m-align-right pf-m-spacer-none pf-m-spacer-md-on-md"
            >
              <div class="pf-v5-c-toolbar__item">
                <button
                  class="pf-v5-c-button pf-m-plain"
                  type="button"
                  aria-expanded="true"
                  aria-label="Attention notifications"
                >
                  <span
                    class="pf-v5-c-notification-badge pf-m-attention pf-m-expanded"
                  >
                    <i class="pf-v5-pficon-attention-bell" aria-hidden="true"></i>
                  </span>
                </button>
              </div>
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
        id="drawer-expanded-attention-example-page-primary-nav"
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
            <main class="pf-v5-c-page__main" tabindex="-1">
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
        <div class="pf-v5-c-drawer__panel pf-m-width-33">
          <div class="pf-v5-c-drawer__body pf-m-no-padding">
            <div class="pf-v5-c-notification-drawer">
              <div class="pf-v5-c-notification-drawer__header">
                <h1
                  class="pf-v5-c-notification-drawer__header-title"
                >Notifications</h1>
                <span
                  class="pf-v5-c-notification-drawer__header-status"
                >3 unread</span>
                <div class="pf-v5-c-notification-drawer__header-action">
                  <div class="pf-v5-c-dropdown">
                    <button
                      class="pf-v5-c-dropdown__toggle pf-m-plain"
                      id="drawer-demo-notification-drawer-basic-header-action-button"
                      aria-expanded="false"
                      type="button"
                      aria-label="Actions"
                    >
                      <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
                    </button>
                    <ul
                      class="pf-v5-c-dropdown__menu pf-m-align-right"
                      aria-labelledby="drawer-demo-notification-drawer-basic-header-action-button"
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
              <div class="pf-v5-c-notification-drawer__body">
                <ul class="pf-v5-c-notification-drawer__list" role="list">
                  <li
                    class="pf-v5-c-notification-drawer__list-item pf-m-hoverable pf-m-info"
                    tabindex="0"
                  >
                    <div class="pf-v5-c-notification-drawer__list-item-header">
                      <span
                        class="pf-v5-c-notification-drawer__list-item-header-icon"
                      >
                        <i class="fas fa-info-circle" aria-hidden="true"></i>
                      </span>
                      <h2
                        class="pf-v5-c-notification-drawer__list-item-header-title"
                      >
                        <span class="pf-v5-screen-reader">Info notification:</span>
                        Unread
                        info notification title
                      </h2>
                    </div>
                    <div class="pf-v5-c-notification-drawer__list-item-action">
                      <div class="pf-v5-c-dropdown">
                        <button
                          class="pf-v5-c-dropdown__toggle pf-m-plain"
                          id="drawer-demo-notification-drawer-basicdropdown-kebab-1-button"
                          aria-expanded="false"
                          type="button"
                          aria-label="Actions"
                        >
                          <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
                        </button>
                        <ul
                          class="pf-v5-c-dropdown__menu pf-m-align-right"
                          aria-labelledby="drawer-demo-notification-drawer-basicdropdown-kebab-1-button"
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
                    <div
                      class="pf-v5-c-notification-drawer__list-item-description"
                    >This is an info notification description.</div>
                    <div
                      class="pf-v5-c-notification-drawer__list-item-timestamp"
                    >5 minutes ago</div>
                  </li>

                  <li
                    class="pf-v5-c-notification-drawer__list-item pf-m-hoverable pf-m-danger"
                    tabindex="0"
                  >
                    <div class="pf-v5-c-notification-drawer__list-item-header">
                      <span
                        class="pf-v5-c-notification-drawer__list-item-header-icon"
                      >
                        <i class="fas fa-exclamation-circle" aria-hidden="true"></i>
                      </span>
                      <h2
                        class="pf-v5-c-notification-drawer__list-item-header-title"
                      >
                        <span class="pf-v5-screen-reader">Danger notification:</span>
                        Unread danger notification title. This is a long title to show how the title will wrap if it is long and wraps to multiple lines.
                      </h2>
                    </div>
                    <div class="pf-v5-c-notification-drawer__list-item-action">
                      <div class="pf-v5-c-dropdown">
                        <button
                          class="pf-v5-c-dropdown__toggle pf-m-plain"
                          id="drawer-demo-notification-drawer-basicdropdown-kebab-2-button"
                          aria-expanded="false"
                          type="button"
                          aria-label="Actions"
                        >
                          <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
                        </button>
                        <ul
                          class="pf-v5-c-dropdown__menu pf-m-align-right"
                          aria-labelledby="drawer-demo-notification-drawer-basicdropdown-kebab-2-button"
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
                    <div
                      class="pf-v5-c-notification-drawer__list-item-description"
                    >This is a danger notification description. This is a long description to show how the title will wrap if it is long and wraps to multiple lines.</div>
                    <div
                      class="pf-v5-c-notification-drawer__list-item-timestamp"
                    >10 minutes ago</div>
                  </li>

                  <li
                    class="pf-v5-c-notification-drawer__list-item pf-m-hoverable pf-m-custom"
                    tabindex="0"
                  >
                    <div class="pf-v5-c-notification-drawer__list-item-header">
                      <span
                        class="pf-v5-c-notification-drawer__list-item-header-icon"
                      >
                        <i class="fas fa-arrow-circle-up" aria-hidden="true"></i>
                      </span>
                      <h2
                        class="pf-v5-c-notification-drawer__list-item-header-title"
                      >
                        <span class="pf-v5-screen-reader">Custom notification:</span>
                        Unread
                        recommendation notification title. This is a long title to show how the title will wrap if it is long and wraps to multiple lines.
                      </h2>
                    </div>
                    <div class="pf-v5-c-notification-drawer__list-item-action">
                      <div class="pf-v5-c-dropdown">
                        <button
                          class="pf-v5-c-dropdown__toggle pf-m-plain"
                          id="drawer-demo-notification-drawer-basicdropdown-kebab-3-button"
                          aria-expanded="false"
                          type="button"
                          aria-label="Actions"
                        >
                          <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
                        </button>
                        <ul
                          class="pf-v5-c-dropdown__menu pf-m-align-right"
                          aria-labelledby="drawer-demo-notification-drawer-basicdropdown-kebab-3-button"
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
                    <div
                      class="pf-v5-c-notification-drawer__list-item-description"
                    >This is a recommendation notification description. This is a long description to show how the title will wrap if it is long and wraps to multiple lines.</div>
                    <div
                      class="pf-v5-c-notification-drawer__list-item-timestamp"
                    >20 minutes ago</div>
                  </li>
                  <li
                    class="pf-v5-c-notification-drawer__list-item pf-m-read pf-m-warning pf-m-hoverable"
                  >
                    <div class="pf-v5-c-notification-drawer__list-item-header">
                      <span
                        class="pf-v5-c-notification-drawer__list-item-header-icon"
                      >
                        <i
                          class="fas fa-exclamation-triangle"
                          aria-hidden="true"
                        ></i>
                      </span>
                      <h2
                        class="pf-v5-c-notification-drawer__list-item-header-title"
                      >
                        <span class="pf-v5-screen-reader">Warning notification:</span>
                        Read warning notification title
                      </h2>
                    </div>
                    <div class="pf-v5-c-notification-drawer__list-item-action">
                      <div class="pf-v5-c-dropdown pf-m-top">
                        <button
                          class="pf-v5-c-dropdown__toggle pf-m-plain"
                          id="drawer-demo-notification-drawer-basicdropdown-kebab-4-button"
                          aria-expanded="false"
                          type="button"
                          aria-label="Actions"
                        >
                          <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
                        </button>
                        <ul
                          class="pf-v5-c-dropdown__menu pf-m-align-right"
                          aria-labelledby="drawer-demo-notification-drawer-basicdropdown-kebab-4-button"
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
                    <div
                      class="pf-v5-c-notification-drawer__list-item-description"
                    >This is a warning notification description.</div>
                    <div
                      class="pf-v5-c-notification-drawer__list-item-timestamp"
                    >20 minutes ago</div>
                  </li>
                  <li
                    class="pf-v5-c-notification-drawer__list-item pf-m-read pf-m-success pf-m-hoverable"
                  >
                    <div class="pf-v5-c-notification-drawer__list-item-header">
                      <span
                        class="pf-v5-c-notification-drawer__list-item-header-icon"
                      >
                        <i class="fas fa-check-circle" aria-hidden="true"></i>
                      </span>
                      <h2
                        class="pf-v5-c-notification-drawer__list-item-header-title"
                      >
                        <span class="pf-v5-screen-reader">Success notification:</span>
                        Read success notification title
                      </h2>
                    </div>
                    <div class="pf-v5-c-notification-drawer__list-item-action">
                      <div class="pf-v5-c-dropdown pf-m-top">
                        <button
                          class="pf-v5-c-dropdown__toggle pf-m-plain"
                          id="drawer-demo-notification-drawer-basicdropdown-kebab-5-button"
                          aria-expanded="false"
                          type="button"
                          aria-label="Actions"
                        >
                          <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
                        </button>
                        <ul
                          class="pf-v5-c-dropdown__menu pf-m-align-right"
                          aria-labelledby="drawer-demo-notification-drawer-basicdropdown-kebab-5-button"
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
                    <div
                      class="pf-v5-c-notification-drawer__list-item-description"
                    >This is a success notification description.</div>
                    <div
                      class="pf-v5-c-notification-drawer__list-item-timestamp"
                    >30 minutes ago</div>
                  </li>
                  <li
                    class="pf-v5-c-notification-drawer__list-item pf-m-read pf-m-success pf-m-hoverable"
                  >
                    <div class="pf-v5-c-notification-drawer__list-item-header">
                      <span
                        class="pf-v5-c-notification-drawer__list-item-header-icon"
                      >
                        <i class="fas fa-check-circle" aria-hidden="true"></i>
                      </span>
                      <h2
                        class="pf-v5-c-notification-drawer__list-item-header-title pf-m-truncate"
                      >
                        <span class="pf-v5-screen-reader">Success notification:</span>
                        Lorem ipsum dolor sit amet, consectetur adipiscing elit. Praesent quis odio risus. Ut dictum vitae sapien at posuere. Nullam suscipit massa quis lacus pellentesque scelerisque. Donec non maximus neque, quis ornare nunc. Vivamus in nibh sed libero feugiat feugiat. Nulla lacinia rutrum est, a commodo odio vestibulum suscipit. Nullam id quam et quam porttitor interdum quis nec tellus. Vestibulum arcu dui, pulvinar eu tellus in, semper mattis diam. Sed commodo tincidunt lacus non pulvinar. Curabitur tempor molestie vestibulum. Vivamus vel mi dignissim, efficitur neque eget, efficitur massa. Mauris vitae nunc augue. Donec augue lorem, malesuada et quam vitae, volutpat mattis nisi. Nullam nec venenatis ex, quis lobortis purus. Sed nisl dolor, mattis sit amet tincidunt quis, mollis sed massa.
                      </h2>
                    </div>
                    <div class="pf-v5-c-notification-drawer__list-item-action">
                      <div class="pf-v5-c-dropdown pf-m-top">
                        <button
                          class="pf-v5-c-dropdown__toggle pf-m-plain"
                          id="drawer-demo-notification-drawer-basicdropdown-kebab-6-button"
                          aria-expanded="false"
                          type="button"
                          aria-label="Actions"
                        >
                          <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
                        </button>
                        <ul
                          class="pf-v5-c-dropdown__menu pf-m-align-right"
                          aria-labelledby="drawer-demo-notification-drawer-basicdropdown-kebab-6-button"
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
                    <div
                      class="pf-v5-c-notification-drawer__list-item-description"
                    >This example uses ".pf-m-truncate" to limit the title to a single line and truncate any overflow text with ellipses.</div>
                    <div
                      class="pf-v5-c-notification-drawer__list-item-timestamp"
                    >40 minutes ago</div>
                  </li>
                  <li
                    class="pf-v5-c-notification-drawer__list-item pf-m-read pf-m-success pf-m-hoverable"
                  >
                    <div class="pf-v5-c-notification-drawer__list-item-header">
                      <span
                        class="pf-v5-c-notification-drawer__list-item-header-icon"
                      >
                        <i class="fas fa-check-circle" aria-hidden="true"></i>
                      </span>
                      <h2
                        class="pf-v5-c-notification-drawer__list-item-header-title pf-m-truncate"
                        style="--pf-v5-c-notification-drawer__list-item-header-title--max-lines: 2"
                      >
                        <span class="pf-v5-screen-reader">Success notification:</span>
                        Lorem ipsum dolor sit amet, consectetur adipiscing elit. Praesent quis odio risus. Ut dictum vitae sapien at posuere. Nullam suscipit massa quis lacus pellentesque scelerisque. Donec non maximus neque, quis ornare nunc. Vivamus in nibh sed libero feugiat feugiat. Nulla lacinia rutrum est, a commodo odio vestibulum suscipit. Nullam id quam et quam porttitor interdum quis nec tellus. Vestibulum arcu dui, pulvinar eu tellus in, semper mattis diam. Sed commodo tincidunt lacus non pulvinar. Curabitur tempor molestie vestibulum. Vivamus vel mi dignissim, efficitur neque eget, efficitur massa. Mauris vitae nunc augue. Donec augue lorem, malesuada et quam vitae, volutpat mattis nisi. Nullam nec venenatis ex, quis lobortis purus. Sed nisl dolor, mattis sit amet tincidunt quis, mollis sed massa.
                      </h2>
                    </div>
                    <div class="pf-v5-c-notification-drawer__list-item-action">
                      <div class="pf-v5-c-dropdown pf-m-top">
                        <button
                          class="pf-v5-c-dropdown__toggle pf-m-plain"
                          id="drawer-demo-notification-drawer-basicdropdown-kebab-7-button"
                          aria-expanded="false"
                          type="button"
                          aria-label="Actions"
                        >
                          <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
                        </button>
                        <ul
                          class="pf-v5-c-dropdown__menu pf-m-align-right"
                          aria-labelledby="drawer-demo-notification-drawer-basicdropdown-kebab-7-button"
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
                    <div
                      class="pf-v5-c-notification-drawer__list-item-description"
                    >This example uses ".pf-m-truncate" and sets "--pf-v5-c-notification-drawer__list-item-header-title--max-lines: 2" to limit title to two lines and truncate any overflow text with ellipses.</div>
                    <div
                      class="pf-v5-c-notification-drawer__list-item-timestamp"
                    >50 minutes ago</div>
                  </li>
                </ul>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</div>

```

### Expanded with groups

```html isFullscreen
<div class="pf-v5-c-page" id="drawer-expanded-with-groups-example-page">
  <div class="pf-v5-c-skip-to-content">
    <a
      class="pf-v5-c-button pf-m-primary"
      href="#main-content-drawer-expanded-with-groups-example-page"
    >Skip to content</a>
  </div>
  <header
    class="pf-v5-c-masthead"
    id="drawer-expanded-with-groups-example-page-masthead"
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
        id="drawer-expanded-with-groups-example-page-masthead-toolbar"
      >
        <div class="pf-v5-c-toolbar__content">
          <div class="pf-v5-c-toolbar__content-section">
            <div
              class="pf-v5-c-toolbar__group pf-m-icon-button-group pf-m-align-right pf-m-spacer-none pf-m-spacer-md-on-md"
            >
              <div class="pf-v5-c-toolbar__item">
                <button
                  class="pf-v5-c-button pf-m-plain"
                  type="button"
                  aria-expanded="true"
                  aria-label="Unread notifications"
                >
                  <span
                    class="pf-v5-c-notification-badge pf-m-unread pf-m-expanded"
                  >
                    <i class="pf-v5-pficon-bell" aria-hidden="true"></i>
                  </span>
                </button>
              </div>
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
        id="drawer-expanded-with-groups-example-page-primary-nav"
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
            <main class="pf-v5-c-page__main" tabindex="-1">
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
        <div class="pf-v5-c-drawer__panel pf-m-width-33">
          <div class="pf-v5-c-drawer__body pf-m-no-padding">
            <div class="pf-v5-c-notification-drawer">
              <div class="pf-v5-c-notification-drawer__header">
                <h1
                  class="pf-v5-c-notification-drawer__header-title"
                >Notifications</h1>
                <span
                  class="pf-v5-c-notification-drawer__header-status"
                >9 unread</span>
                <div class="pf-v5-c-notification-drawer__header-action">
                  <div class="pf-v5-c-dropdown">
                    <button
                      class="pf-v5-c-dropdown__toggle pf-m-plain"
                      id="drawer-demo-notification-drawer-groups-header-action-button"
                      aria-expanded="false"
                      type="button"
                      aria-label="Actions"
                    >
                      <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
                    </button>
                    <ul
                      class="pf-v5-c-dropdown__menu pf-m-align-right"
                      aria-labelledby="drawer-demo-notification-drawer-groups-header-action-button"
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
              <div class="pf-v5-c-notification-drawer__body">
                <div class="pf-v5-c-notification-drawer__group-list">
                  <section class="pf-v5-c-notification-drawer__group">
                    <h1>
                      <button
                        class="pf-v5-c-notification-drawer__group-toggle"
                        aria-expanded="false"
                      >
                        <div
                          class="pf-v5-c-notification-drawer__group-toggle-title"
                        >First notification group</div>
                        <div
                          class="pf-v5-c-notification-drawer__group-toggle-count"
                        >
                          <span class="pf-v5-c-badge pf-m-unread">2</span>
                        </div>
                        <span
                          class="pf-v5-c-notification-drawer__group-toggle-icon"
                        >
                          <i class="fas fa-angle-right" aria-hidden="true"></i>
                        </span>
                      </button>
                    </h1>
                    <ul
                      class="pf-v5-c-notification-drawer__list"
                      role="list"
                      hidden
                    >
                      <li
                        class="pf-v5-c-notification-drawer__list-item pf-m-hoverable pf-m-info"
                        tabindex="0"
                      >
                        <div
                          class="pf-v5-c-notification-drawer__list-item-header"
                        >
                          <span
                            class="pf-v5-c-notification-drawer__list-item-header-icon"
                          >
                            <i class="fas fa-info-circle" aria-hidden="true"></i>
                          </span>
                          <h2
                            class="pf-v5-c-notification-drawer__list-item-header-title"
                          >
                            <span class="pf-v5-screen-reader">Info notification:</span>
                            Unread
                            info notification title
                          </h2>
                        </div>
                        <div
                          class="pf-v5-c-notification-drawer__list-item-action"
                        >
                          <div class="pf-v5-c-dropdown">
                            <button
                              class="pf-v5-c-dropdown__toggle pf-m-plain"
                              id="drawer-demo-notification-drawer-groups-group1dropdown-kebab-1-button"
                              aria-expanded="false"
                              type="button"
                              aria-label="Actions"
                            >
                              <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
                            </button>
                            <ul
                              class="pf-v5-c-dropdown__menu pf-m-align-right"
                              aria-labelledby="drawer-demo-notification-drawer-groups-group1dropdown-kebab-1-button"
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
                        <div
                          class="pf-v5-c-notification-drawer__list-item-description"
                        >This is an info notification description.</div>
                        <div
                          class="pf-v5-c-notification-drawer__list-item-timestamp"
                        >5 minutes ago</div>
                      </li>

                      <li
                        class="pf-v5-c-notification-drawer__list-item pf-m-hoverable pf-m-custom"
                        tabindex="0"
                      >
                        <div
                          class="pf-v5-c-notification-drawer__list-item-header"
                        >
                          <span
                            class="pf-v5-c-notification-drawer__list-item-header-icon"
                          >
                            <i
                              class="fas fa-arrow-circle-up"
                              aria-hidden="true"
                            ></i>
                          </span>
                          <h2
                            class="pf-v5-c-notification-drawer__list-item-header-title"
                          >
                            <span
                              class="pf-v5-screen-reader"
                            >Custom notification:</span>
                            Unread
                            recommendation notification title. This is a long title to show how the title will wrap if it is long and wraps to multiple lines.
                          </h2>
                        </div>
                        <div
                          class="pf-v5-c-notification-drawer__list-item-action"
                        >
                          <div class="pf-v5-c-dropdown">
                            <button
                              class="pf-v5-c-dropdown__toggle pf-m-plain"
                              id="drawer-demo-notification-drawer-groups-group1dropdown-kebab-2-button"
                              aria-expanded="false"
                              type="button"
                              aria-label="Actions"
                            >
                              <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
                            </button>
                            <ul
                              class="pf-v5-c-dropdown__menu pf-m-align-right"
                              aria-labelledby="drawer-demo-notification-drawer-groups-group1dropdown-kebab-2-button"
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
                        <div
                          class="pf-v5-c-notification-drawer__list-item-description"
                        >This is a recommendation notification description. This is a long description to show how the title will wrap if it is long and wraps to multiple lines.</div>
                        <div
                          class="pf-v5-c-notification-drawer__list-item-timestamp"
                        >10 minutes ago</div>
                      </li>

                      <li
                        class="pf-v5-c-notification-drawer__list-item pf-m-hoverable pf-m-custom"
                        tabindex="0"
                      >
                        <div
                          class="pf-v5-c-notification-drawer__list-item-header"
                        >
                          <span
                            class="pf-v5-c-notification-drawer__list-item-header-icon"
                          >
                            <i
                              class="fas fa-arrow-circle-up"
                              aria-hidden="true"
                            ></i>
                          </span>
                          <h2
                            class="pf-v5-c-notification-drawer__list-item-header-title"
                          >
                            <span
                              class="pf-v5-screen-reader"
                            >Custom notification:</span>
                            Unread
                            recommendation notification title. This is a long title to show how the title will wrap if it is long and wraps to multiple lines.
                          </h2>
                        </div>
                        <div
                          class="pf-v5-c-notification-drawer__list-item-action"
                        >
                          <div class="pf-v5-c-dropdown">
                            <button
                              class="pf-v5-c-dropdown__toggle pf-m-plain"
                              id="drawer-demo-notification-drawer-groups-group1dropdown-kebab-3-button"
                              aria-expanded="false"
                              type="button"
                              aria-label="Actions"
                            >
                              <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
                            </button>
                            <ul
                              class="pf-v5-c-dropdown__menu pf-m-align-right"
                              aria-labelledby="drawer-demo-notification-drawer-groups-group1dropdown-kebab-3-button"
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
                        <div
                          class="pf-v5-c-notification-drawer__list-item-description"
                        >This is a recommendation notification description. This is a long description to show how the title will wrap if it is long and wraps to multiple lines.</div>
                        <div
                          class="pf-v5-c-notification-drawer__list-item-timestamp"
                        >20 minutes ago</div>
                      </li>
                      <li
                        class="pf-v5-c-notification-drawer__list-item pf-m-read pf-m-warning pf-m-hoverable"
                      >
                        <div
                          class="pf-v5-c-notification-drawer__list-item-header"
                        >
                          <span
                            class="pf-v5-c-notification-drawer__list-item-header-icon"
                          >
                            <i
                              class="fas fa-exclamation-triangle"
                              aria-hidden="true"
                            ></i>
                          </span>
                          <h2
                            class="pf-v5-c-notification-drawer__list-item-header-title"
                          >
                            <span
                              class="pf-v5-screen-reader"
                            >Warning notification:</span>
                            Read warning notification title
                          </h2>
                        </div>
                        <div
                          class="pf-v5-c-notification-drawer__list-item-action"
                        >
                          <div class="pf-v5-c-dropdown pf-m-top">
                            <button
                              class="pf-v5-c-dropdown__toggle pf-m-plain"
                              id="drawer-demo-notification-drawer-groups-group1dropdown-kebab-4-button"
                              aria-expanded="false"
                              type="button"
                              aria-label="Actions"
                            >
                              <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
                            </button>
                            <ul
                              class="pf-v5-c-dropdown__menu pf-m-align-right"
                              aria-labelledby="drawer-demo-notification-drawer-groups-group1dropdown-kebab-4-button"
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
                        <div
                          class="pf-v5-c-notification-drawer__list-item-description"
                        >This is a warning notification description.</div>
                        <div
                          class="pf-v5-c-notification-drawer__list-item-timestamp"
                        >20 minutes ago</div>
                      </li>
                      <li
                        class="pf-v5-c-notification-drawer__list-item pf-m-read pf-m-success pf-m-hoverable"
                      >
                        <div
                          class="pf-v5-c-notification-drawer__list-item-header"
                        >
                          <span
                            class="pf-v5-c-notification-drawer__list-item-header-icon"
                          >
                            <i class="fas fa-check-circle" aria-hidden="true"></i>
                          </span>
                          <h2
                            class="pf-v5-c-notification-drawer__list-item-header-title"
                          >
                            <span
                              class="pf-v5-screen-reader"
                            >Success notification:</span>
                            Read success notification title
                          </h2>
                        </div>
                        <div
                          class="pf-v5-c-notification-drawer__list-item-action"
                        >
                          <div class="pf-v5-c-dropdown pf-m-top">
                            <button
                              class="pf-v5-c-dropdown__toggle pf-m-plain"
                              id="drawer-demo-notification-drawer-groups-group1dropdown-kebab-5-button"
                              aria-expanded="false"
                              type="button"
                              aria-label="Actions"
                            >
                              <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
                            </button>
                            <ul
                              class="pf-v5-c-dropdown__menu pf-m-align-right"
                              aria-labelledby="drawer-demo-notification-drawer-groups-group1dropdown-kebab-5-button"
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
                        <div
                          class="pf-v5-c-notification-drawer__list-item-description"
                        >This is a success notification description.</div>
                        <div
                          class="pf-v5-c-notification-drawer__list-item-timestamp"
                        >30 minutes ago</div>
                      </li>
                      <li
                        class="pf-v5-c-notification-drawer__list-item pf-m-read pf-m-success pf-m-hoverable"
                      >
                        <div
                          class="pf-v5-c-notification-drawer__list-item-header"
                        >
                          <span
                            class="pf-v5-c-notification-drawer__list-item-header-icon"
                          >
                            <i class="fas fa-check-circle" aria-hidden="true"></i>
                          </span>
                          <h2
                            class="pf-v5-c-notification-drawer__list-item-header-title pf-m-truncate"
                          >
                            <span
                              class="pf-v5-screen-reader"
                            >Success notification:</span>
                            Lorem ipsum dolor sit amet, consectetur adipiscing elit. Praesent quis odio risus. Ut dictum vitae sapien at posuere. Nullam suscipit massa quis lacus pellentesque scelerisque. Donec non maximus neque, quis ornare nunc. Vivamus in nibh sed libero feugiat feugiat. Nulla lacinia rutrum est, a commodo odio vestibulum suscipit. Nullam id quam et quam porttitor interdum quis nec tellus. Vestibulum arcu dui, pulvinar eu tellus in, semper mattis diam. Sed commodo tincidunt lacus non pulvinar. Curabitur tempor molestie vestibulum. Vivamus vel mi dignissim, efficitur neque eget, efficitur massa. Mauris vitae nunc augue. Donec augue lorem, malesuada et quam vitae, volutpat mattis nisi. Nullam nec venenatis ex, quis lobortis purus. Sed nisl dolor, mattis sit amet tincidunt quis, mollis sed massa.
                          </h2>
                        </div>
                        <div
                          class="pf-v5-c-notification-drawer__list-item-action"
                        >
                          <div class="pf-v5-c-dropdown pf-m-top">
                            <button
                              class="pf-v5-c-dropdown__toggle pf-m-plain"
                              id="drawer-demo-notification-drawer-groups-group1dropdown-kebab-6-button"
                              aria-expanded="false"
                              type="button"
                              aria-label="Actions"
                            >
                              <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
                            </button>
                            <ul
                              class="pf-v5-c-dropdown__menu pf-m-align-right"
                              aria-labelledby="drawer-demo-notification-drawer-groups-group1dropdown-kebab-6-button"
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
                        <div
                          class="pf-v5-c-notification-drawer__list-item-description"
                        >This example uses ".pf-m-truncate" to limit the title to a single line and truncate any overflow text with ellipses.</div>
                        <div
                          class="pf-v5-c-notification-drawer__list-item-timestamp"
                        >40 minutes ago</div>
                      </li>
                      <li
                        class="pf-v5-c-notification-drawer__list-item pf-m-read pf-m-success pf-m-hoverable"
                      >
                        <div
                          class="pf-v5-c-notification-drawer__list-item-header"
                        >
                          <span
                            class="pf-v5-c-notification-drawer__list-item-header-icon"
                          >
                            <i class="fas fa-check-circle" aria-hidden="true"></i>
                          </span>
                          <h2
                            class="pf-v5-c-notification-drawer__list-item-header-title pf-m-truncate"
                            style="--pf-v5-c-notification-drawer__list-item-header-title--max-lines: 2"
                          >
                            <span
                              class="pf-v5-screen-reader"
                            >Success notification:</span>
                            Lorem ipsum dolor sit amet, consectetur adipiscing elit. Praesent quis odio risus. Ut dictum vitae sapien at posuere. Nullam suscipit massa quis lacus pellentesque scelerisque. Donec non maximus neque, quis ornare nunc. Vivamus in nibh sed libero feugiat feugiat. Nulla lacinia rutrum est, a commodo odio vestibulum suscipit. Nullam id quam et quam porttitor interdum quis nec tellus. Vestibulum arcu dui, pulvinar eu tellus in, semper mattis diam. Sed commodo tincidunt lacus non pulvinar. Curabitur tempor molestie vestibulum. Vivamus vel mi dignissim, efficitur neque eget, efficitur massa. Mauris vitae nunc augue. Donec augue lorem, malesuada et quam vitae, volutpat mattis nisi. Nullam nec venenatis ex, quis lobortis purus. Sed nisl dolor, mattis sit amet tincidunt quis, mollis sed massa.
                          </h2>
                        </div>
                        <div
                          class="pf-v5-c-notification-drawer__list-item-action"
                        >
                          <div class="pf-v5-c-dropdown pf-m-top">
                            <button
                              class="pf-v5-c-dropdown__toggle pf-m-plain"
                              id="drawer-demo-notification-drawer-groups-group1dropdown-kebab-7-button"
                              aria-expanded="false"
                              type="button"
                              aria-label="Actions"
                            >
                              <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
                            </button>
                            <ul
                              class="pf-v5-c-dropdown__menu pf-m-align-right"
                              aria-labelledby="drawer-demo-notification-drawer-groups-group1dropdown-kebab-7-button"
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
                        <div
                          class="pf-v5-c-notification-drawer__list-item-description"
                        >This example uses ".pf-m-truncate" and sets "--pf-v5-c-notification-drawer__list-item-header-title--max-lines: 2" to limit title to two lines and truncate any overflow text with ellipses.</div>
                        <div
                          class="pf-v5-c-notification-drawer__list-item-timestamp"
                        >50 minutes ago</div>
                      </li>
                    </ul>
                  </section>
                  <section
                    class="pf-v5-c-notification-drawer__group pf-m-expanded"
                  >
                    <h1>
                      <button
                        class="pf-v5-c-notification-drawer__group-toggle"
                        aria-expanded="true"
                      >
                        <div
                          class="pf-v5-c-notification-drawer__group-toggle-title"
                        >Second notification group</div>
                        <div
                          class="pf-v5-c-notification-drawer__group-toggle-count"
                        >
                          <span class="pf-v5-c-badge pf-m-unread">3</span>
                        </div>
                        <span
                          class="pf-v5-c-notification-drawer__group-toggle-icon"
                        >
                          <i class="fas fa-angle-right" aria-hidden="true"></i>
                        </span>
                      </button>
                    </h1>
                    <ul class="pf-v5-c-notification-drawer__list" role="list">
                      <li
                        class="pf-v5-c-notification-drawer__list-item pf-m-hoverable pf-m-info"
                        tabindex="0"
                      >
                        <div
                          class="pf-v5-c-notification-drawer__list-item-header"
                        >
                          <span
                            class="pf-v5-c-notification-drawer__list-item-header-icon"
                          >
                            <i class="fas fa-info-circle" aria-hidden="true"></i>
                          </span>
                          <h2
                            class="pf-v5-c-notification-drawer__list-item-header-title"
                          >
                            <span class="pf-v5-screen-reader">Info notification:</span>
                            Unread
                            info notification title
                          </h2>
                        </div>
                        <div
                          class="pf-v5-c-notification-drawer__list-item-action"
                        >
                          <div class="pf-v5-c-dropdown">
                            <button
                              class="pf-v5-c-dropdown__toggle pf-m-plain"
                              id="drawer-demo-notification-drawer-groups-group2dropdown-kebab-1-button"
                              aria-expanded="false"
                              type="button"
                              aria-label="Actions"
                            >
                              <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
                            </button>
                            <ul
                              class="pf-v5-c-dropdown__menu pf-m-align-right"
                              aria-labelledby="drawer-demo-notification-drawer-groups-group2dropdown-kebab-1-button"
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
                        <div
                          class="pf-v5-c-notification-drawer__list-item-description"
                        >This is an info notification description.</div>
                        <div
                          class="pf-v5-c-notification-drawer__list-item-timestamp"
                        >5 minutes ago</div>
                      </li>

                      <li
                        class="pf-v5-c-notification-drawer__list-item pf-m-hoverable pf-m-custom"
                        tabindex="0"
                      >
                        <div
                          class="pf-v5-c-notification-drawer__list-item-header"
                        >
                          <span
                            class="pf-v5-c-notification-drawer__list-item-header-icon"
                          >
                            <i
                              class="fas fa-arrow-circle-up"
                              aria-hidden="true"
                            ></i>
                          </span>
                          <h2
                            class="pf-v5-c-notification-drawer__list-item-header-title"
                          >
                            <span
                              class="pf-v5-screen-reader"
                            >Custom notification:</span>
                            Unread
                            recommendation notification title. This is a long title to show how the title will wrap if it is long and wraps to multiple lines.
                          </h2>
                        </div>
                        <div
                          class="pf-v5-c-notification-drawer__list-item-action"
                        >
                          <div class="pf-v5-c-dropdown">
                            <button
                              class="pf-v5-c-dropdown__toggle pf-m-plain"
                              id="drawer-demo-notification-drawer-groups-group2dropdown-kebab-2-button"
                              aria-expanded="false"
                              type="button"
                              aria-label="Actions"
                            >
                              <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
                            </button>
                            <ul
                              class="pf-v5-c-dropdown__menu pf-m-align-right"
                              aria-labelledby="drawer-demo-notification-drawer-groups-group2dropdown-kebab-2-button"
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
                        <div
                          class="pf-v5-c-notification-drawer__list-item-description"
                        >This is a recommendation notification description. This is a long description to show how the title will wrap if it is long and wraps to multiple lines.</div>
                        <div
                          class="pf-v5-c-notification-drawer__list-item-timestamp"
                        >10 minutes ago</div>
                      </li>

                      <li
                        class="pf-v5-c-notification-drawer__list-item pf-m-hoverable pf-m-custom"
                        tabindex="0"
                      >
                        <div
                          class="pf-v5-c-notification-drawer__list-item-header"
                        >
                          <span
                            class="pf-v5-c-notification-drawer__list-item-header-icon"
                          >
                            <i
                              class="fas fa-arrow-circle-up"
                              aria-hidden="true"
                            ></i>
                          </span>
                          <h2
                            class="pf-v5-c-notification-drawer__list-item-header-title"
                          >
                            <span
                              class="pf-v5-screen-reader"
                            >Custom notification:</span>
                            Unread
                            recommendation notification title. This is a long title to show how the title will wrap if it is long and wraps to multiple lines.
                          </h2>
                        </div>
                        <div
                          class="pf-v5-c-notification-drawer__list-item-action"
                        >
                          <div class="pf-v5-c-dropdown">
                            <button
                              class="pf-v5-c-dropdown__toggle pf-m-plain"
                              id="drawer-demo-notification-drawer-groups-group2dropdown-kebab-3-button"
                              aria-expanded="false"
                              type="button"
                              aria-label="Actions"
                            >
                              <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
                            </button>
                            <ul
                              class="pf-v5-c-dropdown__menu pf-m-align-right"
                              aria-labelledby="drawer-demo-notification-drawer-groups-group2dropdown-kebab-3-button"
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
                        <div
                          class="pf-v5-c-notification-drawer__list-item-description"
                        >This is a recommendation notification description. This is a long description to show how the title will wrap if it is long and wraps to multiple lines.</div>
                        <div
                          class="pf-v5-c-notification-drawer__list-item-timestamp"
                        >20 minutes ago</div>
                      </li>
                      <li
                        class="pf-v5-c-notification-drawer__list-item pf-m-read pf-m-warning pf-m-hoverable"
                      >
                        <div
                          class="pf-v5-c-notification-drawer__list-item-header"
                        >
                          <span
                            class="pf-v5-c-notification-drawer__list-item-header-icon"
                          >
                            <i
                              class="fas fa-exclamation-triangle"
                              aria-hidden="true"
                            ></i>
                          </span>
                          <h2
                            class="pf-v5-c-notification-drawer__list-item-header-title"
                          >
                            <span
                              class="pf-v5-screen-reader"
                            >Warning notification:</span>
                            Read warning notification title
                          </h2>
                        </div>
                        <div
                          class="pf-v5-c-notification-drawer__list-item-action"
                        >
                          <div class="pf-v5-c-dropdown pf-m-top">
                            <button
                              class="pf-v5-c-dropdown__toggle pf-m-plain"
                              id="drawer-demo-notification-drawer-groups-group2dropdown-kebab-4-button"
                              aria-expanded="false"
                              type="button"
                              aria-label="Actions"
                            >
                              <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
                            </button>
                            <ul
                              class="pf-v5-c-dropdown__menu pf-m-align-right"
                              aria-labelledby="drawer-demo-notification-drawer-groups-group2dropdown-kebab-4-button"
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
                        <div
                          class="pf-v5-c-notification-drawer__list-item-description"
                        >This is a warning notification description.</div>
                        <div
                          class="pf-v5-c-notification-drawer__list-item-timestamp"
                        >20 minutes ago</div>
                      </li>
                      <li
                        class="pf-v5-c-notification-drawer__list-item pf-m-read pf-m-success pf-m-hoverable"
                      >
                        <div
                          class="pf-v5-c-notification-drawer__list-item-header"
                        >
                          <span
                            class="pf-v5-c-notification-drawer__list-item-header-icon"
                          >
                            <i class="fas fa-check-circle" aria-hidden="true"></i>
                          </span>
                          <h2
                            class="pf-v5-c-notification-drawer__list-item-header-title"
                          >
                            <span
                              class="pf-v5-screen-reader"
                            >Success notification:</span>
                            Read success notification title
                          </h2>
                        </div>
                        <div
                          class="pf-v5-c-notification-drawer__list-item-action"
                        >
                          <div class="pf-v5-c-dropdown pf-m-top">
                            <button
                              class="pf-v5-c-dropdown__toggle pf-m-plain"
                              id="drawer-demo-notification-drawer-groups-group2dropdown-kebab-5-button"
                              aria-expanded="false"
                              type="button"
                              aria-label="Actions"
                            >
                              <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
                            </button>
                            <ul
                              class="pf-v5-c-dropdown__menu pf-m-align-right"
                              aria-labelledby="drawer-demo-notification-drawer-groups-group2dropdown-kebab-5-button"
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
                        <div
                          class="pf-v5-c-notification-drawer__list-item-description"
                        >This is a success notification description.</div>
                        <div
                          class="pf-v5-c-notification-drawer__list-item-timestamp"
                        >30 minutes ago</div>
                      </li>
                      <li
                        class="pf-v5-c-notification-drawer__list-item pf-m-read pf-m-success pf-m-hoverable"
                      >
                        <div
                          class="pf-v5-c-notification-drawer__list-item-header"
                        >
                          <span
                            class="pf-v5-c-notification-drawer__list-item-header-icon"
                          >
                            <i class="fas fa-check-circle" aria-hidden="true"></i>
                          </span>
                          <h2
                            class="pf-v5-c-notification-drawer__list-item-header-title pf-m-truncate"
                          >
                            <span
                              class="pf-v5-screen-reader"
                            >Success notification:</span>
                            Lorem ipsum dolor sit amet, consectetur adipiscing elit. Praesent quis odio risus. Ut dictum vitae sapien at posuere. Nullam suscipit massa quis lacus pellentesque scelerisque. Donec non maximus neque, quis ornare nunc. Vivamus in nibh sed libero feugiat feugiat. Nulla lacinia rutrum est, a commodo odio vestibulum suscipit. Nullam id quam et quam porttitor interdum quis nec tellus. Vestibulum arcu dui, pulvinar eu tellus in, semper mattis diam. Sed commodo tincidunt lacus non pulvinar. Curabitur tempor molestie vestibulum. Vivamus vel mi dignissim, efficitur neque eget, efficitur massa. Mauris vitae nunc augue. Donec augue lorem, malesuada et quam vitae, volutpat mattis nisi. Nullam nec venenatis ex, quis lobortis purus. Sed nisl dolor, mattis sit amet tincidunt quis, mollis sed massa.
                          </h2>
                        </div>
                        <div
                          class="pf-v5-c-notification-drawer__list-item-action"
                        >
                          <div class="pf-v5-c-dropdown pf-m-top">
                            <button
                              class="pf-v5-c-dropdown__toggle pf-m-plain"
                              id="drawer-demo-notification-drawer-groups-group2dropdown-kebab-6-button"
                              aria-expanded="false"
                              type="button"
                              aria-label="Actions"
                            >
                              <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
                            </button>
                            <ul
                              class="pf-v5-c-dropdown__menu pf-m-align-right"
                              aria-labelledby="drawer-demo-notification-drawer-groups-group2dropdown-kebab-6-button"
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
                        <div
                          class="pf-v5-c-notification-drawer__list-item-description"
                        >This example uses ".pf-m-truncate" to limit the title to a single line and truncate any overflow text with ellipses.</div>
                        <div
                          class="pf-v5-c-notification-drawer__list-item-timestamp"
                        >40 minutes ago</div>
                      </li>
                      <li
                        class="pf-v5-c-notification-drawer__list-item pf-m-read pf-m-success pf-m-hoverable"
                      >
                        <div
                          class="pf-v5-c-notification-drawer__list-item-header"
                        >
                          <span
                            class="pf-v5-c-notification-drawer__list-item-header-icon"
                          >
                            <i class="fas fa-check-circle" aria-hidden="true"></i>
                          </span>
                          <h2
                            class="pf-v5-c-notification-drawer__list-item-header-title pf-m-truncate"
                            style="--pf-v5-c-notification-drawer__list-item-header-title--max-lines: 2"
                          >
                            <span
                              class="pf-v5-screen-reader"
                            >Success notification:</span>
                            Lorem ipsum dolor sit amet, consectetur adipiscing elit. Praesent quis odio risus. Ut dictum vitae sapien at posuere. Nullam suscipit massa quis lacus pellentesque scelerisque. Donec non maximus neque, quis ornare nunc. Vivamus in nibh sed libero feugiat feugiat. Nulla lacinia rutrum est, a commodo odio vestibulum suscipit. Nullam id quam et quam porttitor interdum quis nec tellus. Vestibulum arcu dui, pulvinar eu tellus in, semper mattis diam. Sed commodo tincidunt lacus non pulvinar. Curabitur tempor molestie vestibulum. Vivamus vel mi dignissim, efficitur neque eget, efficitur massa. Mauris vitae nunc augue. Donec augue lorem, malesuada et quam vitae, volutpat mattis nisi. Nullam nec venenatis ex, quis lobortis purus. Sed nisl dolor, mattis sit amet tincidunt quis, mollis sed massa.
                          </h2>
                        </div>
                        <div
                          class="pf-v5-c-notification-drawer__list-item-action"
                        >
                          <div class="pf-v5-c-dropdown pf-m-top">
                            <button
                              class="pf-v5-c-dropdown__toggle pf-m-plain"
                              id="drawer-demo-notification-drawer-groups-group2dropdown-kebab-7-button"
                              aria-expanded="false"
                              type="button"
                              aria-label="Actions"
                            >
                              <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
                            </button>
                            <ul
                              class="pf-v5-c-dropdown__menu pf-m-align-right"
                              aria-labelledby="drawer-demo-notification-drawer-groups-group2dropdown-kebab-7-button"
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
                        <div
                          class="pf-v5-c-notification-drawer__list-item-description"
                        >This example uses ".pf-m-truncate" and sets "--pf-v5-c-notification-drawer__list-item-header-title--max-lines: 2" to limit title to two lines and truncate any overflow text with ellipses.</div>
                        <div
                          class="pf-v5-c-notification-drawer__list-item-timestamp"
                        >50 minutes ago</div>
                      </li>
                    </ul>
                  </section>
                  <section class="pf-v5-c-notification-drawer__group">
                    <h1>
                      <button
                        class="pf-v5-c-notification-drawer__group-toggle"
                        aria-expanded="false"
                      >
                        <div
                          class="pf-v5-c-notification-drawer__group-toggle-title"
                        >Third notification group</div>
                        <div
                          class="pf-v5-c-notification-drawer__group-toggle-count"
                        >
                          <span class="pf-v5-c-badge pf-m-unread">2</span>
                        </div>
                        <span
                          class="pf-v5-c-notification-drawer__group-toggle-icon"
                        >
                          <i class="fas fa-angle-right" aria-hidden="true"></i>
                        </span>
                      </button>
                    </h1>
                    <ul
                      class="pf-v5-c-notification-drawer__list"
                      role="list"
                      hidden
                    >
                      <li
                        class="pf-v5-c-notification-drawer__list-item pf-m-hoverable pf-m-info"
                        tabindex="0"
                      >
                        <div
                          class="pf-v5-c-notification-drawer__list-item-header"
                        >
                          <span
                            class="pf-v5-c-notification-drawer__list-item-header-icon"
                          >
                            <i class="fas fa-info-circle" aria-hidden="true"></i>
                          </span>
                          <h2
                            class="pf-v5-c-notification-drawer__list-item-header-title"
                          >
                            <span class="pf-v5-screen-reader">Info notification:</span>
                            Unread
                            info notification title
                          </h2>
                        </div>
                        <div
                          class="pf-v5-c-notification-drawer__list-item-action"
                        >
                          <div class="pf-v5-c-dropdown">
                            <button
                              class="pf-v5-c-dropdown__toggle pf-m-plain"
                              id="drawer-demo-notification-drawer-groups-group3dropdown-kebab-1-button"
                              aria-expanded="false"
                              type="button"
                              aria-label="Actions"
                            >
                              <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
                            </button>
                            <ul
                              class="pf-v5-c-dropdown__menu pf-m-align-right"
                              aria-labelledby="drawer-demo-notification-drawer-groups-group3dropdown-kebab-1-button"
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
                        <div
                          class="pf-v5-c-notification-drawer__list-item-description"
                        >This is an info notification description.</div>
                        <div
                          class="pf-v5-c-notification-drawer__list-item-timestamp"
                        >5 minutes ago</div>
                      </li>

                      <li
                        class="pf-v5-c-notification-drawer__list-item pf-m-hoverable pf-m-custom"
                        tabindex="0"
                      >
                        <div
                          class="pf-v5-c-notification-drawer__list-item-header"
                        >
                          <span
                            class="pf-v5-c-notification-drawer__list-item-header-icon"
                          >
                            <i
                              class="fas fa-arrow-circle-up"
                              aria-hidden="true"
                            ></i>
                          </span>
                          <h2
                            class="pf-v5-c-notification-drawer__list-item-header-title"
                          >
                            <span
                              class="pf-v5-screen-reader"
                            >Custom notification:</span>
                            Unread
                            recommendation notification title. This is a long title to show how the title will wrap if it is long and wraps to multiple lines.
                          </h2>
                        </div>
                        <div
                          class="pf-v5-c-notification-drawer__list-item-action"
                        >
                          <div class="pf-v5-c-dropdown">
                            <button
                              class="pf-v5-c-dropdown__toggle pf-m-plain"
                              id="drawer-demo-notification-drawer-groups-group3dropdown-kebab-2-button"
                              aria-expanded="false"
                              type="button"
                              aria-label="Actions"
                            >
                              <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
                            </button>
                            <ul
                              class="pf-v5-c-dropdown__menu pf-m-align-right"
                              aria-labelledby="drawer-demo-notification-drawer-groups-group3dropdown-kebab-2-button"
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
                        <div
                          class="pf-v5-c-notification-drawer__list-item-description"
                        >This is a recommendation notification description. This is a long description to show how the title will wrap if it is long and wraps to multiple lines.</div>
                        <div
                          class="pf-v5-c-notification-drawer__list-item-timestamp"
                        >10 minutes ago</div>
                      </li>

                      <li
                        class="pf-v5-c-notification-drawer__list-item pf-m-hoverable pf-m-custom"
                        tabindex="0"
                      >
                        <div
                          class="pf-v5-c-notification-drawer__list-item-header"
                        >
                          <span
                            class="pf-v5-c-notification-drawer__list-item-header-icon"
                          >
                            <i
                              class="fas fa-arrow-circle-up"
                              aria-hidden="true"
                            ></i>
                          </span>
                          <h2
                            class="pf-v5-c-notification-drawer__list-item-header-title"
                          >
                            <span
                              class="pf-v5-screen-reader"
                            >Custom notification:</span>
                            Unread
                            recommendation notification title. This is a long title to show how the title will wrap if it is long and wraps to multiple lines.
                          </h2>
                        </div>
                        <div
                          class="pf-v5-c-notification-drawer__list-item-action"
                        >
                          <div class="pf-v5-c-dropdown">
                            <button
                              class="pf-v5-c-dropdown__toggle pf-m-plain"
                              id="drawer-demo-notification-drawer-groups-group3dropdown-kebab-3-button"
                              aria-expanded="false"
                              type="button"
                              aria-label="Actions"
                            >
                              <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
                            </button>
                            <ul
                              class="pf-v5-c-dropdown__menu pf-m-align-right"
                              aria-labelledby="drawer-demo-notification-drawer-groups-group3dropdown-kebab-3-button"
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
                        <div
                          class="pf-v5-c-notification-drawer__list-item-description"
                        >This is a recommendation notification description. This is a long description to show how the title will wrap if it is long and wraps to multiple lines.</div>
                        <div
                          class="pf-v5-c-notification-drawer__list-item-timestamp"
                        >20 minutes ago</div>
                      </li>
                      <li
                        class="pf-v5-c-notification-drawer__list-item pf-m-read pf-m-warning pf-m-hoverable"
                      >
                        <div
                          class="pf-v5-c-notification-drawer__list-item-header"
                        >
                          <span
                            class="pf-v5-c-notification-drawer__list-item-header-icon"
                          >
                            <i
                              class="fas fa-exclamation-triangle"
                              aria-hidden="true"
                            ></i>
                          </span>
                          <h2
                            class="pf-v5-c-notification-drawer__list-item-header-title"
                          >
                            <span
                              class="pf-v5-screen-reader"
                            >Warning notification:</span>
                            Read warning notification title
                          </h2>
                        </div>
                        <div
                          class="pf-v5-c-notification-drawer__list-item-action"
                        >
                          <div class="pf-v5-c-dropdown pf-m-top">
                            <button
                              class="pf-v5-c-dropdown__toggle pf-m-plain"
                              id="drawer-demo-notification-drawer-groups-group3dropdown-kebab-4-button"
                              aria-expanded="false"
                              type="button"
                              aria-label="Actions"
                            >
                              <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
                            </button>
                            <ul
                              class="pf-v5-c-dropdown__menu pf-m-align-right"
                              aria-labelledby="drawer-demo-notification-drawer-groups-group3dropdown-kebab-4-button"
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
                        <div
                          class="pf-v5-c-notification-drawer__list-item-description"
                        >This is a warning notification description.</div>
                        <div
                          class="pf-v5-c-notification-drawer__list-item-timestamp"
                        >20 minutes ago</div>
                      </li>
                      <li
                        class="pf-v5-c-notification-drawer__list-item pf-m-read pf-m-success pf-m-hoverable"
                      >
                        <div
                          class="pf-v5-c-notification-drawer__list-item-header"
                        >
                          <span
                            class="pf-v5-c-notification-drawer__list-item-header-icon"
                          >
                            <i class="fas fa-check-circle" aria-hidden="true"></i>
                          </span>
                          <h2
                            class="pf-v5-c-notification-drawer__list-item-header-title"
                          >
                            <span
                              class="pf-v5-screen-reader"
                            >Success notification:</span>
                            Read success notification title
                          </h2>
                        </div>
                        <div
                          class="pf-v5-c-notification-drawer__list-item-action"
                        >
                          <div class="pf-v5-c-dropdown pf-m-top">
                            <button
                              class="pf-v5-c-dropdown__toggle pf-m-plain"
                              id="drawer-demo-notification-drawer-groups-group3dropdown-kebab-5-button"
                              aria-expanded="false"
                              type="button"
                              aria-label="Actions"
                            >
                              <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
                            </button>
                            <ul
                              class="pf-v5-c-dropdown__menu pf-m-align-right"
                              aria-labelledby="drawer-demo-notification-drawer-groups-group3dropdown-kebab-5-button"
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
                        <div
                          class="pf-v5-c-notification-drawer__list-item-description"
                        >This is a success notification description.</div>
                        <div
                          class="pf-v5-c-notification-drawer__list-item-timestamp"
                        >30 minutes ago</div>
                      </li>
                      <li
                        class="pf-v5-c-notification-drawer__list-item pf-m-read pf-m-success pf-m-hoverable"
                      >
                        <div
                          class="pf-v5-c-notification-drawer__list-item-header"
                        >
                          <span
                            class="pf-v5-c-notification-drawer__list-item-header-icon"
                          >
                            <i class="fas fa-check-circle" aria-hidden="true"></i>
                          </span>
                          <h2
                            class="pf-v5-c-notification-drawer__list-item-header-title pf-m-truncate"
                          >
                            <span
                              class="pf-v5-screen-reader"
                            >Success notification:</span>
                            Lorem ipsum dolor sit amet, consectetur adipiscing elit. Praesent quis odio risus. Ut dictum vitae sapien at posuere. Nullam suscipit massa quis lacus pellentesque scelerisque. Donec non maximus neque, quis ornare nunc. Vivamus in nibh sed libero feugiat feugiat. Nulla lacinia rutrum est, a commodo odio vestibulum suscipit. Nullam id quam et quam porttitor interdum quis nec tellus. Vestibulum arcu dui, pulvinar eu tellus in, semper mattis diam. Sed commodo tincidunt lacus non pulvinar. Curabitur tempor molestie vestibulum. Vivamus vel mi dignissim, efficitur neque eget, efficitur massa. Mauris vitae nunc augue. Donec augue lorem, malesuada et quam vitae, volutpat mattis nisi. Nullam nec venenatis ex, quis lobortis purus. Sed nisl dolor, mattis sit amet tincidunt quis, mollis sed massa.
                          </h2>
                        </div>
                        <div
                          class="pf-v5-c-notification-drawer__list-item-action"
                        >
                          <div class="pf-v5-c-dropdown pf-m-top">
                            <button
                              class="pf-v5-c-dropdown__toggle pf-m-plain"
                              id="drawer-demo-notification-drawer-groups-group3dropdown-kebab-6-button"
                              aria-expanded="false"
                              type="button"
                              aria-label="Actions"
                            >
                              <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
                            </button>
                            <ul
                              class="pf-v5-c-dropdown__menu pf-m-align-right"
                              aria-labelledby="drawer-demo-notification-drawer-groups-group3dropdown-kebab-6-button"
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
                        <div
                          class="pf-v5-c-notification-drawer__list-item-description"
                        >This example uses ".pf-m-truncate" to limit the title to a single line and truncate any overflow text with ellipses.</div>
                        <div
                          class="pf-v5-c-notification-drawer__list-item-timestamp"
                        >40 minutes ago</div>
                      </li>
                      <li
                        class="pf-v5-c-notification-drawer__list-item pf-m-read pf-m-success pf-m-hoverable"
                      >
                        <div
                          class="pf-v5-c-notification-drawer__list-item-header"
                        >
                          <span
                            class="pf-v5-c-notification-drawer__list-item-header-icon"
                          >
                            <i class="fas fa-check-circle" aria-hidden="true"></i>
                          </span>
                          <h2
                            class="pf-v5-c-notification-drawer__list-item-header-title pf-m-truncate"
                            style="--pf-v5-c-notification-drawer__list-item-header-title--max-lines: 2"
                          >
                            <span
                              class="pf-v5-screen-reader"
                            >Success notification:</span>
                            Lorem ipsum dolor sit amet, consectetur adipiscing elit. Praesent quis odio risus. Ut dictum vitae sapien at posuere. Nullam suscipit massa quis lacus pellentesque scelerisque. Donec non maximus neque, quis ornare nunc. Vivamus in nibh sed libero feugiat feugiat. Nulla lacinia rutrum est, a commodo odio vestibulum suscipit. Nullam id quam et quam porttitor interdum quis nec tellus. Vestibulum arcu dui, pulvinar eu tellus in, semper mattis diam. Sed commodo tincidunt lacus non pulvinar. Curabitur tempor molestie vestibulum. Vivamus vel mi dignissim, efficitur neque eget, efficitur massa. Mauris vitae nunc augue. Donec augue lorem, malesuada et quam vitae, volutpat mattis nisi. Nullam nec venenatis ex, quis lobortis purus. Sed nisl dolor, mattis sit amet tincidunt quis, mollis sed massa.
                          </h2>
                        </div>
                        <div
                          class="pf-v5-c-notification-drawer__list-item-action"
                        >
                          <div class="pf-v5-c-dropdown pf-m-top">
                            <button
                              class="pf-v5-c-dropdown__toggle pf-m-plain"
                              id="drawer-demo-notification-drawer-groups-group3dropdown-kebab-7-button"
                              aria-expanded="false"
                              type="button"
                              aria-label="Actions"
                            >
                              <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
                            </button>
                            <ul
                              class="pf-v5-c-dropdown__menu pf-m-align-right"
                              aria-labelledby="drawer-demo-notification-drawer-groups-group3dropdown-kebab-7-button"
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
                        <div
                          class="pf-v5-c-notification-drawer__list-item-description"
                        >This example uses ".pf-m-truncate" and sets "--pf-v5-c-notification-drawer__list-item-header-title--max-lines: 2" to limit title to two lines and truncate any overflow text with ellipses.</div>
                        <div
                          class="pf-v5-c-notification-drawer__list-item-timestamp"
                        >50 minutes ago</div>
                      </li>
                    </ul>
                  </section>
                  <section class="pf-v5-c-notification-drawer__group">
                    <h1>
                      <button
                        class="pf-v5-c-notification-drawer__group-toggle"
                        aria-expanded="false"
                      >
                        <div
                          class="pf-v5-c-notification-drawer__group-toggle-title"
                        >Fourth notification group</div>
                        <div
                          class="pf-v5-c-notification-drawer__group-toggle-count"
                        >
                          <span class="pf-v5-c-badge pf-m-unread">2</span>
                        </div>
                        <span
                          class="pf-v5-c-notification-drawer__group-toggle-icon"
                        >
                          <i class="fas fa-angle-right" aria-hidden="true"></i>
                        </span>
                      </button>
                    </h1>
                    <ul
                      class="pf-v5-c-notification-drawer__list"
                      role="list"
                      hidden
                    >
                      <li
                        class="pf-v5-c-notification-drawer__list-item pf-m-hoverable pf-m-info"
                        tabindex="0"
                      >
                        <div
                          class="pf-v5-c-notification-drawer__list-item-header"
                        >
                          <span
                            class="pf-v5-c-notification-drawer__list-item-header-icon"
                          >
                            <i class="fas fa-info-circle" aria-hidden="true"></i>
                          </span>
                          <h2
                            class="pf-v5-c-notification-drawer__list-item-header-title"
                          >
                            <span class="pf-v5-screen-reader">Info notification:</span>
                            Unread
                            info notification title
                          </h2>
                        </div>
                        <div
                          class="pf-v5-c-notification-drawer__list-item-action"
                        >
                          <div class="pf-v5-c-dropdown">
                            <button
                              class="pf-v5-c-dropdown__toggle pf-m-plain"
                              id="drawer-demo-notification-drawer-groups-group4dropdown-kebab-1-button"
                              aria-expanded="false"
                              type="button"
                              aria-label="Actions"
                            >
                              <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
                            </button>
                            <ul
                              class="pf-v5-c-dropdown__menu pf-m-align-right"
                              aria-labelledby="drawer-demo-notification-drawer-groups-group4dropdown-kebab-1-button"
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
                        <div
                          class="pf-v5-c-notification-drawer__list-item-description"
                        >This is an info notification description.</div>
                        <div
                          class="pf-v5-c-notification-drawer__list-item-timestamp"
                        >5 minutes ago</div>
                      </li>

                      <li
                        class="pf-v5-c-notification-drawer__list-item pf-m-hoverable pf-m-custom"
                        tabindex="0"
                      >
                        <div
                          class="pf-v5-c-notification-drawer__list-item-header"
                        >
                          <span
                            class="pf-v5-c-notification-drawer__list-item-header-icon"
                          >
                            <i
                              class="fas fa-arrow-circle-up"
                              aria-hidden="true"
                            ></i>
                          </span>
                          <h2
                            class="pf-v5-c-notification-drawer__list-item-header-title"
                          >
                            <span
                              class="pf-v5-screen-reader"
                            >Custom notification:</span>
                            Unread
                            recommendation notification title. This is a long title to show how the title will wrap if it is long and wraps to multiple lines.
                          </h2>
                        </div>
                        <div
                          class="pf-v5-c-notification-drawer__list-item-action"
                        >
                          <div class="pf-v5-c-dropdown">
                            <button
                              class="pf-v5-c-dropdown__toggle pf-m-plain"
                              id="drawer-demo-notification-drawer-groups-group4dropdown-kebab-2-button"
                              aria-expanded="false"
                              type="button"
                              aria-label="Actions"
                            >
                              <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
                            </button>
                            <ul
                              class="pf-v5-c-dropdown__menu pf-m-align-right"
                              aria-labelledby="drawer-demo-notification-drawer-groups-group4dropdown-kebab-2-button"
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
                        <div
                          class="pf-v5-c-notification-drawer__list-item-description"
                        >This is a recommendation notification description. This is a long description to show how the title will wrap if it is long and wraps to multiple lines.</div>
                        <div
                          class="pf-v5-c-notification-drawer__list-item-timestamp"
                        >10 minutes ago</div>
                      </li>

                      <li
                        class="pf-v5-c-notification-drawer__list-item pf-m-hoverable pf-m-custom"
                        tabindex="0"
                      >
                        <div
                          class="pf-v5-c-notification-drawer__list-item-header"
                        >
                          <span
                            class="pf-v5-c-notification-drawer__list-item-header-icon"
                          >
                            <i
                              class="fas fa-arrow-circle-up"
                              aria-hidden="true"
                            ></i>
                          </span>
                          <h2
                            class="pf-v5-c-notification-drawer__list-item-header-title"
                          >
                            <span
                              class="pf-v5-screen-reader"
                            >Custom notification:</span>
                            Unread
                            recommendation notification title. This is a long title to show how the title will wrap if it is long and wraps to multiple lines.
                          </h2>
                        </div>
                        <div
                          class="pf-v5-c-notification-drawer__list-item-action"
                        >
                          <div class="pf-v5-c-dropdown">
                            <button
                              class="pf-v5-c-dropdown__toggle pf-m-plain"
                              id="drawer-demo-notification-drawer-groups-group4dropdown-kebab-3-button"
                              aria-expanded="false"
                              type="button"
                              aria-label="Actions"
                            >
                              <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
                            </button>
                            <ul
                              class="pf-v5-c-dropdown__menu pf-m-align-right"
                              aria-labelledby="drawer-demo-notification-drawer-groups-group4dropdown-kebab-3-button"
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
                        <div
                          class="pf-v5-c-notification-drawer__list-item-description"
                        >This is a recommendation notification description. This is a long description to show how the title will wrap if it is long and wraps to multiple lines.</div>
                        <div
                          class="pf-v5-c-notification-drawer__list-item-timestamp"
                        >20 minutes ago</div>
                      </li>
                      <li
                        class="pf-v5-c-notification-drawer__list-item pf-m-read pf-m-warning pf-m-hoverable"
                      >
                        <div
                          class="pf-v5-c-notification-drawer__list-item-header"
                        >
                          <span
                            class="pf-v5-c-notification-drawer__list-item-header-icon"
                          >
                            <i
                              class="fas fa-exclamation-triangle"
                              aria-hidden="true"
                            ></i>
                          </span>
                          <h2
                            class="pf-v5-c-notification-drawer__list-item-header-title"
                          >
                            <span
                              class="pf-v5-screen-reader"
                            >Warning notification:</span>
                            Read warning notification title
                          </h2>
                        </div>
                        <div
                          class="pf-v5-c-notification-drawer__list-item-action"
                        >
                          <div class="pf-v5-c-dropdown pf-m-top">
                            <button
                              class="pf-v5-c-dropdown__toggle pf-m-plain"
                              id="drawer-demo-notification-drawer-groups-group4dropdown-kebab-4-button"
                              aria-expanded="false"
                              type="button"
                              aria-label="Actions"
                            >
                              <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
                            </button>
                            <ul
                              class="pf-v5-c-dropdown__menu pf-m-align-right"
                              aria-labelledby="drawer-demo-notification-drawer-groups-group4dropdown-kebab-4-button"
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
                        <div
                          class="pf-v5-c-notification-drawer__list-item-description"
                        >This is a warning notification description.</div>
                        <div
                          class="pf-v5-c-notification-drawer__list-item-timestamp"
                        >20 minutes ago</div>
                      </li>
                      <li
                        class="pf-v5-c-notification-drawer__list-item pf-m-read pf-m-success pf-m-hoverable"
                      >
                        <div
                          class="pf-v5-c-notification-drawer__list-item-header"
                        >
                          <span
                            class="pf-v5-c-notification-drawer__list-item-header-icon"
                          >
                            <i class="fas fa-check-circle" aria-hidden="true"></i>
                          </span>
                          <h2
                            class="pf-v5-c-notification-drawer__list-item-header-title"
                          >
                            <span
                              class="pf-v5-screen-reader"
                            >Success notification:</span>
                            Read success notification title
                          </h2>
                        </div>
                        <div
                          class="pf-v5-c-notification-drawer__list-item-action"
                        >
                          <div class="pf-v5-c-dropdown pf-m-top">
                            <button
                              class="pf-v5-c-dropdown__toggle pf-m-plain"
                              id="drawer-demo-notification-drawer-groups-group4dropdown-kebab-5-button"
                              aria-expanded="false"
                              type="button"
                              aria-label="Actions"
                            >
                              <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
                            </button>
                            <ul
                              class="pf-v5-c-dropdown__menu pf-m-align-right"
                              aria-labelledby="drawer-demo-notification-drawer-groups-group4dropdown-kebab-5-button"
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
                        <div
                          class="pf-v5-c-notification-drawer__list-item-description"
                        >This is a success notification description.</div>
                        <div
                          class="pf-v5-c-notification-drawer__list-item-timestamp"
                        >30 minutes ago</div>
                      </li>
                      <li
                        class="pf-v5-c-notification-drawer__list-item pf-m-read pf-m-success pf-m-hoverable"
                      >
                        <div
                          class="pf-v5-c-notification-drawer__list-item-header"
                        >
                          <span
                            class="pf-v5-c-notification-drawer__list-item-header-icon"
                          >
                            <i class="fas fa-check-circle" aria-hidden="true"></i>
                          </span>
                          <h2
                            class="pf-v5-c-notification-drawer__list-item-header-title pf-m-truncate"
                          >
                            <span
                              class="pf-v5-screen-reader"
                            >Success notification:</span>
                            Lorem ipsum dolor sit amet, consectetur adipiscing elit. Praesent quis odio risus. Ut dictum vitae sapien at posuere. Nullam suscipit massa quis lacus pellentesque scelerisque. Donec non maximus neque, quis ornare nunc. Vivamus in nibh sed libero feugiat feugiat. Nulla lacinia rutrum est, a commodo odio vestibulum suscipit. Nullam id quam et quam porttitor interdum quis nec tellus. Vestibulum arcu dui, pulvinar eu tellus in, semper mattis diam. Sed commodo tincidunt lacus non pulvinar. Curabitur tempor molestie vestibulum. Vivamus vel mi dignissim, efficitur neque eget, efficitur massa. Mauris vitae nunc augue. Donec augue lorem, malesuada et quam vitae, volutpat mattis nisi. Nullam nec venenatis ex, quis lobortis purus. Sed nisl dolor, mattis sit amet tincidunt quis, mollis sed massa.
                          </h2>
                        </div>
                        <div
                          class="pf-v5-c-notification-drawer__list-item-action"
                        >
                          <div class="pf-v5-c-dropdown pf-m-top">
                            <button
                              class="pf-v5-c-dropdown__toggle pf-m-plain"
                              id="drawer-demo-notification-drawer-groups-group4dropdown-kebab-6-button"
                              aria-expanded="false"
                              type="button"
                              aria-label="Actions"
                            >
                              <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
                            </button>
                            <ul
                              class="pf-v5-c-dropdown__menu pf-m-align-right"
                              aria-labelledby="drawer-demo-notification-drawer-groups-group4dropdown-kebab-6-button"
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
                        <div
                          class="pf-v5-c-notification-drawer__list-item-description"
                        >This example uses ".pf-m-truncate" to limit the title to a single line and truncate any overflow text with ellipses.</div>
                        <div
                          class="pf-v5-c-notification-drawer__list-item-timestamp"
                        >40 minutes ago</div>
                      </li>
                      <li
                        class="pf-v5-c-notification-drawer__list-item pf-m-read pf-m-success pf-m-hoverable"
                      >
                        <div
                          class="pf-v5-c-notification-drawer__list-item-header"
                        >
                          <span
                            class="pf-v5-c-notification-drawer__list-item-header-icon"
                          >
                            <i class="fas fa-check-circle" aria-hidden="true"></i>
                          </span>
                          <h2
                            class="pf-v5-c-notification-drawer__list-item-header-title pf-m-truncate"
                            style="--pf-v5-c-notification-drawer__list-item-header-title--max-lines: 2"
                          >
                            <span
                              class="pf-v5-screen-reader"
                            >Success notification:</span>
                            Lorem ipsum dolor sit amet, consectetur adipiscing elit. Praesent quis odio risus. Ut dictum vitae sapien at posuere. Nullam suscipit massa quis lacus pellentesque scelerisque. Donec non maximus neque, quis ornare nunc. Vivamus in nibh sed libero feugiat feugiat. Nulla lacinia rutrum est, a commodo odio vestibulum suscipit. Nullam id quam et quam porttitor interdum quis nec tellus. Vestibulum arcu dui, pulvinar eu tellus in, semper mattis diam. Sed commodo tincidunt lacus non pulvinar. Curabitur tempor molestie vestibulum. Vivamus vel mi dignissim, efficitur neque eget, efficitur massa. Mauris vitae nunc augue. Donec augue lorem, malesuada et quam vitae, volutpat mattis nisi. Nullam nec venenatis ex, quis lobortis purus. Sed nisl dolor, mattis sit amet tincidunt quis, mollis sed massa.
                          </h2>
                        </div>
                        <div
                          class="pf-v5-c-notification-drawer__list-item-action"
                        >
                          <div class="pf-v5-c-dropdown pf-m-top">
                            <button
                              class="pf-v5-c-dropdown__toggle pf-m-plain"
                              id="drawer-demo-notification-drawer-groups-group4dropdown-kebab-7-button"
                              aria-expanded="false"
                              type="button"
                              aria-label="Actions"
                            >
                              <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
                            </button>
                            <ul
                              class="pf-v5-c-dropdown__menu pf-m-align-right"
                              aria-labelledby="drawer-demo-notification-drawer-groups-group4dropdown-kebab-7-button"
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
                        <div
                          class="pf-v5-c-notification-drawer__list-item-description"
                        >This example uses ".pf-m-truncate" and sets "--pf-v5-c-notification-drawer__list-item-header-title--max-lines: 2" to limit title to two lines and truncate any overflow text with ellipses.</div>
                        <div
                          class="pf-v5-c-notification-drawer__list-item-timestamp"
                        >50 minutes ago</div>
                      </li>
                    </ul>
                  </section>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</div>

```

## Documentation

This demo implements the notification drawer in context of the page component.
