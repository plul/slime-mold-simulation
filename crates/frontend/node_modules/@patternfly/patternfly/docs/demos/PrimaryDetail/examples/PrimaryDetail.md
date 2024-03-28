---
id: Primary-detail
section: patterns
wrapperTag: div
---## Demos

### Primary-detail expanded

```html isFullscreen
<div class="pf-v5-c-page" id="primary-detail-expanded-example">
  <div class="pf-v5-c-skip-to-content">
    <a
      class="pf-v5-c-button pf-m-primary"
      href="#main-content-primary-detail-expanded-example"
    >Skip to content</a>
  </div>
  <header
    class="pf-v5-c-masthead pf-m-display-stack pf-m-display-inline-on-lg"
    id="primary-detail-expanded-example-masthead"
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
        id="primary-detail-expanded-example-masthead-toolbar"
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
        id="primary-detail-expanded-example-primary-nav"
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
    id="main-content-primary-detail-expanded-example"
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
    <div class="pf-v5-c-divider" role="separator"></div>
    <section class="pf-v5-c-page__main-section pf-m-no-padding">
      <!-- Drawer -->
      <div class="pf-v5-c-drawer pf-m-expanded pf-m-inline-on-2xl">
        <div class="pf-v5-c-drawer__main">
          <!-- Content -->
          <div class="pf-v5-c-drawer__content">
            <div class="pf-v5-c-drawer__body">
              <div
                class="pf-v5-c-toolbar pf-m-page-insets"
                id="primary-detail-expanded-example-drawer-toolbar"
              >
                <div class="pf-v5-c-toolbar__content">
                  <div class="pf-v5-c-toolbar__content-section pf-m-nowrap">
                    <div
                      class="pf-v5-c-toolbar__group pf-m-toggle-group pf-m-show-on-xl"
                    >
                      <div class="pf-v5-c-toolbar__toggle">
                        <button
                          class="pf-v5-c-button pf-m-plain"
                          type="button"
                          aria-label="Show filters"
                          aria-expanded="false"
                          aria-controls="primary-detail-expanded-example-drawer-toolbar-expandable-content"
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
                                id="primary-detail-expanded-example-drawer-toolbar-select-name-label"
                                hidden
                              >Choose one</span>

                              <button
                                class="pf-v5-c-select__toggle"
                                type="button"
                                id="primary-detail-expanded-example-drawer-toolbar-select-name-toggle"
                                aria-haspopup="true"
                                aria-expanded="false"
                                aria-labelledby="primary-detail-expanded-example-drawer-toolbar-select-name-label primary-detail-expanded-example-drawer-toolbar-select-name-toggle"
                              >
                                <div class="pf-v5-c-select__toggle-wrapper">
                                  <span class="pf-v5-c-select__toggle-icon">
                                    <i class="fas fa-filter" aria-hidden="true"></i>
                                  </span>
                                  <span class="pf-v5-c-select__toggle-text">Name</span>
                                </div>
                                <span class="pf-v5-c-select__toggle-arrow">
                                  <i
                                    class="fas fa-caret-down"
                                    aria-hidden="true"
                                  ></i>
                                </span>
                              </button>

                              <ul
                                class="pf-v5-c-select__menu"
                                role="listbox"
                                aria-labelledby="primary-detail-expanded-example-drawer-toolbar-select-name-label"
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
                                    <span
                                      class="pf-v5-c-select__menu-item-icon"
                                    >
                                      <i
                                        class="fas fa-check"
                                        aria-hidden="true"
                                      ></i>
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
                              <div
                                class="pf-v5-c-text-input-group__main pf-m-icon"
                              >
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

                      <div class="pf-v5-c-toolbar__group pf-m-filter-group">
                        <div class="pf-v5-c-toolbar__item">
                          <div class="pf-v5-c-select">
                            <span
                              id="primary-detail-expanded-example-drawer-toolbar-select-checkbox-status-label"
                              hidden
                            >Choose many</span>

                            <button
                              class="pf-v5-c-select__toggle"
                              type="button"
                              id="primary-detail-expanded-example-drawer-toolbar-select-checkbox-status-toggle"
                              aria-haspopup="true"
                              aria-expanded="false"
                              aria-labelledby="primary-detail-expanded-example-drawer-toolbar-select-checkbox-status-label primary-detail-expanded-example-drawer-toolbar-select-checkbox-status-toggle"
                            >
                              <div class="pf-v5-c-select__toggle-wrapper">
                                <span class="pf-v5-c-select__toggle-text">Status</span>
                              </div>
                              <span class="pf-v5-c-select__toggle-arrow">
                                <i class="fas fa-caret-down" aria-hidden="true"></i>
                              </span>
                            </button>

                            <div class="pf-v5-c-select__menu" hidden>
                              <fieldset
                                class="pf-v5-c-select__menu-fieldset"
                                aria-label="Select input"
                              >
                                <label
                                  class="pf-v5-c-check pf-v5-c-select__menu-item pf-m-description"
                                  for="primary-detail-expanded-example-drawer-toolbar-select-checkbox-status-active"
                                >
                                  <input
                                    class="pf-v5-c-check__input"
                                    type="checkbox"
                                    id="primary-detail-expanded-example-drawer-toolbar-select-checkbox-status-active"
                                    name="primary-detail-expanded-example-drawer-toolbar-select-checkbox-status-active"
                                  />

                                  <span class="pf-v5-c-check__label">Active</span>
                                  <span
                                    class="pf-v5-c-check__description"
                                  >This is a description</span>
                                </label>
                                <label
                                  class="pf-v5-c-check pf-v5-c-select__menu-item pf-m-description"
                                  for="primary-detail-expanded-example-drawer-toolbar-select-checkbox-status-canceled"
                                >
                                  <input
                                    class="pf-v5-c-check__input"
                                    type="checkbox"
                                    id="primary-detail-expanded-example-drawer-toolbar-select-checkbox-status-canceled"
                                    name="primary-detail-expanded-example-drawer-toolbar-select-checkbox-status-canceled"
                                  />

                                  <span class="pf-v5-c-check__label">Canceled</span>
                                  <span
                                    class="pf-v5-c-check__description"
                                  >This is a really long description that describes the menu item. This is a really long description that describes the menu item.</span>
                                </label>
                                <label
                                  class="pf-v5-c-check pf-v5-c-select__menu-item"
                                  for="primary-detail-expanded-example-drawer-toolbar-select-checkbox-status-paused"
                                >
                                  <input
                                    class="pf-v5-c-check__input"
                                    type="checkbox"
                                    id="primary-detail-expanded-example-drawer-toolbar-select-checkbox-status-paused"
                                    name="primary-detail-expanded-example-drawer-toolbar-select-checkbox-status-paused"
                                  />

                                  <span class="pf-v5-c-check__label">Paused</span>
                                </label>
                                <label
                                  class="pf-v5-c-check pf-v5-c-select__menu-item"
                                  for="primary-detail-expanded-example-drawer-toolbar-select-checkbox-status-warning"
                                >
                                  <input
                                    class="pf-v5-c-check__input"
                                    type="checkbox"
                                    id="primary-detail-expanded-example-drawer-toolbar-select-checkbox-status-warning"
                                    name="primary-detail-expanded-example-drawer-toolbar-select-checkbox-status-warning"
                                  />

                                  <span class="pf-v5-c-check__label">Warning</span>
                                </label>
                                <label
                                  class="pf-v5-c-check pf-v5-c-select__menu-item"
                                  for="primary-detail-expanded-example-drawer-toolbar-select-checkbox-status-restarted"
                                >
                                  <input
                                    class="pf-v5-c-check__input"
                                    type="checkbox"
                                    id="primary-detail-expanded-example-drawer-toolbar-select-checkbox-status-restarted"
                                    name="primary-detail-expanded-example-drawer-toolbar-select-checkbox-status-restarted"
                                  />

                                  <span class="pf-v5-c-check__label">Restarted</span>
                                </label>
                              </fieldset>
                            </div>
                          </div>
                        </div>
                        <div class="pf-v5-c-toolbar__item">
                          <div class="pf-v5-c-select">
                            <span
                              id="primary-detail-expanded-example-drawer-toolbar-select-checkbox-risk-label"
                              hidden
                            >Choose many</span>

                            <button
                              class="pf-v5-c-select__toggle"
                              type="button"
                              id="primary-detail-expanded-example-drawer-toolbar-select-checkbox-risk-toggle"
                              aria-haspopup="true"
                              aria-expanded="false"
                              aria-labelledby="primary-detail-expanded-example-drawer-toolbar-select-checkbox-risk-label primary-detail-expanded-example-drawer-toolbar-select-checkbox-risk-toggle"
                            >
                              <div class="pf-v5-c-select__toggle-wrapper">
                                <span class="pf-v5-c-select__toggle-text">Risk</span>
                              </div>
                              <span class="pf-v5-c-select__toggle-arrow">
                                <i class="fas fa-caret-down" aria-hidden="true"></i>
                              </span>
                            </button>

                            <div class="pf-v5-c-select__menu" hidden>
                              <fieldset
                                class="pf-v5-c-select__menu-fieldset"
                                aria-label="Select input"
                              >
                                <label
                                  class="pf-v5-c-check pf-v5-c-select__menu-item pf-m-description"
                                  for="primary-detail-expanded-example-drawer-toolbar-select-checkbox-risk-active"
                                >
                                  <input
                                    class="pf-v5-c-check__input"
                                    type="checkbox"
                                    id="primary-detail-expanded-example-drawer-toolbar-select-checkbox-risk-active"
                                    name="primary-detail-expanded-example-drawer-toolbar-select-checkbox-risk-active"
                                  />

                                  <span class="pf-v5-c-check__label">Active</span>
                                  <span
                                    class="pf-v5-c-check__description"
                                  >This is a description</span>
                                </label>
                                <label
                                  class="pf-v5-c-check pf-v5-c-select__menu-item pf-m-description"
                                  for="primary-detail-expanded-example-drawer-toolbar-select-checkbox-risk-canceled"
                                >
                                  <input
                                    class="pf-v5-c-check__input"
                                    type="checkbox"
                                    id="primary-detail-expanded-example-drawer-toolbar-select-checkbox-risk-canceled"
                                    name="primary-detail-expanded-example-drawer-toolbar-select-checkbox-risk-canceled"
                                  />

                                  <span class="pf-v5-c-check__label">Canceled</span>
                                  <span
                                    class="pf-v5-c-check__description"
                                  >This is a really long description that describes the menu item. This is a really long description that describes the menu item.</span>
                                </label>
                                <label
                                  class="pf-v5-c-check pf-v5-c-select__menu-item"
                                  for="primary-detail-expanded-example-drawer-toolbar-select-checkbox-risk-paused"
                                >
                                  <input
                                    class="pf-v5-c-check__input"
                                    type="checkbox"
                                    id="primary-detail-expanded-example-drawer-toolbar-select-checkbox-risk-paused"
                                    name="primary-detail-expanded-example-drawer-toolbar-select-checkbox-risk-paused"
                                  />

                                  <span class="pf-v5-c-check__label">Paused</span>
                                </label>
                                <label
                                  class="pf-v5-c-check pf-v5-c-select__menu-item"
                                  for="primary-detail-expanded-example-drawer-toolbar-select-checkbox-risk-warning"
                                >
                                  <input
                                    class="pf-v5-c-check__input"
                                    type="checkbox"
                                    id="primary-detail-expanded-example-drawer-toolbar-select-checkbox-risk-warning"
                                    name="primary-detail-expanded-example-drawer-toolbar-select-checkbox-risk-warning"
                                  />

                                  <span class="pf-v5-c-check__label">Warning</span>
                                </label>
                                <label
                                  class="pf-v5-c-check pf-v5-c-select__menu-item"
                                  for="primary-detail-expanded-example-drawer-toolbar-select-checkbox-risk-restarted"
                                >
                                  <input
                                    class="pf-v5-c-check__input"
                                    type="checkbox"
                                    id="primary-detail-expanded-example-drawer-toolbar-select-checkbox-risk-restarted"
                                    name="primary-detail-expanded-example-drawer-toolbar-select-checkbox-risk-restarted"
                                  />

                                  <span class="pf-v5-c-check__label">Restarted</span>
                                </label>
                              </fieldset>
                            </div>
                          </div>
                        </div>
                      </div>
                    </div>

                    <div class="pf-v5-c-toolbar__item">
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
                    </div>

                    <div
                      class="pf-v5-c-overflow-menu"
                      id="primary-detail-expanded-example-drawer-toolbar-overflow-menu"
                    >
                      <div
                        class="pf-v5-c-overflow-menu__content pf-v5-u-display-none pf-v5-u-display-flex-on-lg"
                      >
                        <div
                          class="pf-v5-c-overflow-menu__group pf-m-button-group"
                        >
                          <div class="pf-v5-c-overflow-menu__item">
                            <button
                              class="pf-v5-c-button pf-m-primary"
                              type="button"
                            >Create instance</button>
                          </div>
                        </div>
                      </div>
                      <div class="pf-v5-c-overflow-menu__control">
                        <div class="pf-v5-c-dropdown">
                          <button
                            class="pf-v5-c-button pf-v5-c-dropdown__toggle pf-m-plain"
                            type="button"
                            id="primary-detail-expanded-example-drawer-toolbar-overflow-menu-dropdown-toggle"
                            aria-label="Dropdown with additional options"
                            aria-expanded="false"
                          >
                            <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
                          </button>
                          <ul
                            class="pf-v5-c-dropdown__menu"
                            role="menu"
                            aria-labelledby="primary-detail-expanded-example-drawer-toolbar-overflow-menu-dropdown-toggle"
                            hidden
                          >
                            <li role="none">
                              <button
                                role="menuitem"
                                class="pf-v5-c-dropdown__menu-item"
                              >Action 7</button>
                            </li>
                          </ul>
                        </div>
                      </div>
                    </div>

                    <div class="pf-v5-c-toolbar__item pf-m-pagination">
                      <div class="pf-v5-c-pagination pf-m-compact">
                        <div class="pf-v5-c-options-menu">
                          <button
                            class="pf-v5-c-options-menu__toggle pf-m-text pf-m-plain"
                            type="button"
                            id="primary-detail-expanded-example-drawer-toolbar-top-pagination-toggle"
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
                            aria-labelledby="primary-detail-expanded-example-drawer-toolbar-top-pagination-toggle"
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
                                <div
                                  class="pf-v5-c-options-menu__menu-item-icon"
                                >
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
                          aria-label="Toolbar top pagination"
                        >
                          <div
                            class="pf-v5-c-pagination__nav-control pf-m-prev"
                          >
                            <button
                              class="pf-v5-c-button pf-m-plain"
                              type="button"
                              disabled
                              aria-label="Go to previous page"
                            >
                              <i class="fas fa-angle-left" aria-hidden="true"></i>
                            </button>
                          </div>
                          <div
                            class="pf-v5-c-pagination__nav-control pf-m-next"
                          >
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
                    </div>
                  </div>

                  <div
                    class="pf-v5-c-toolbar__expandable-content pf-m-hidden"
                    id="primary-detail-expanded-example-drawer-toolbar-expandable-content"
                    hidden
                  ></div>
                </div>
              </div>
              <ul
                class="pf-v5-c-data-list"
                role="list"
                aria-label="Simple data list example"
                id="primary-detail-expanded-example-data-list"
              >
                <li
                  class="pf-v5-c-data-list__item"
                  aria-labelledby="primary-detail-expanded-example-data-list-item-1"
                >
                  <div class="pf-v5-c-data-list__item-row">
                    <div class="pf-v5-c-data-list__item-content">
                      <div class="pf-v5-c-data-list__cell pf-m-align-left">
                        <div
                          class="pf-v5-l-flex pf-m-column pf-m-space-items-md"
                        >
                          <div
                            class="pf-v5-l-flex pf-m-column pf-m-space-items-none"
                          >
                            <div class="pf-v5-l-flex__item">
                              <p
                                id="primary-detail-expanded-example-data-list-item-1"
                              >patternfly</p>
                            </div>
                            <div class="pf-v5-l-flex__item">
                              <small>
                                Working repo for PatternFly 4
                                <a href>https://pf4.patternfly.org/</a>
                              </small>
                            </div>
                          </div>
                          <div class="pf-v5-l-flex pf-m-wrap">
                            <div class="pf-v5-l-flex pf-m-space-items-xs">
                              <div class="pf-v5-l-flex__item">
                                <i
                                  class="fas fa-code-branch"
                                  aria-hidden="true"
                                ></i>
                              </div>
                              <div class="pf-v5-l-flex__item">
                                <span>10</span>
                              </div>
                            </div>
                            <div class="pf-v5-l-flex pf-m-space-items-xs">
                              <div class="pf-v5-l-flex__item">
                                <i class="fas fa-code" aria-hidden="true"></i>
                              </div>
                              <div class="pf-v5-l-flex__item">
                                <span>4</span>
                              </div>
                            </div>
                            <div class="pf-v5-l-flex pf-m-space-items-xs">
                              <div class="pf-v5-l-flex__item">
                                <i class="fas fa-cube" aria-hidden="true"></i>
                              </div>
                              <div class="pf-v5-l-flex__item">
                                <span>5</span>
                              </div>
                            </div>
                            <div class="pf-v5-l-flex__item">Updated 2 days ago</div>
                          </div>
                        </div>
                      </div>
                      <div
                        class="pf-v5-c-data-list__cell pf-m-align-right pf-m-no-fill"
                      >
                        <button
                          class="pf-v5-c-button pf-m-secondary"
                          type="button"
                        >Action</button>
                        <button
                          class="pf-v5-c-button pf-m-link"
                          type="button"
                        >Link</button>
                      </div>
                    </div>
                  </div>
                </li>

                <li
                  class="pf-v5-c-data-list__item"
                  aria-labelledby="primary-detail-expanded-example-data-list-item-2"
                >
                  <div class="pf-v5-c-data-list__item-row">
                    <div class="pf-v5-c-data-list__item-content">
                      <div class="pf-v5-c-data-list__cell pf-m-align-left">
                        <div
                          class="pf-v5-l-flex pf-m-column pf-m-space-items-md"
                        >
                          <div
                            class="pf-v5-l-flex pf-m-column pf-m-space-items-none"
                          >
                            <div class="pf-v5-l-flex__item">
                              <p
                                id="primary-detail-expanded-example-data-list-item-2"
                              >patternfly-elements</p>
                            </div>
                            <div class="pf-v5-l-flex__item">
                              <small>PatternFly elements</small>
                            </div>
                          </div>
                          <div class="pf-v5-l-flex pf-m-wrap">
                            <div class="pf-v5-l-flex pf-m-space-items-xs">
                              <div class="pf-v5-l-flex__item">
                                <i
                                  class="fas fa-code-branch"
                                  aria-hidden="true"
                                ></i>
                              </div>
                              <div class="pf-v5-l-flex__item">
                                <span>5</span>
                              </div>
                            </div>
                            <div class="pf-v5-l-flex pf-m-space-items-xs">
                              <div class="pf-v5-l-flex__item">
                                <i class="fas fa-code" aria-hidden="true"></i>
                              </div>
                              <div class="pf-v5-l-flex__item">
                                <span>9</span>
                              </div>
                            </div>
                            <div class="pf-v5-l-flex pf-m-space-items-xs">
                              <div class="pf-v5-l-flex__item">
                                <i class="fas fa-cube" aria-hidden="true"></i>
                              </div>
                              <div class="pf-v5-l-flex__item">
                                <span>2</span>
                              </div>
                            </div>
                            <div class="pf-v5-l-flex pf-m-space-items-xs">
                              <div class="pf-v5-l-flex__item">
                                <i
                                  class="fas fa-check-circle"
                                  aria-hidden="true"
                                ></i>
                              </div>
                              <div class="pf-v5-l-flex__item">
                                <span>11</span>
                              </div>
                            </div>
                            <div class="pf-v5-l-flex pf-m-space-items-xs">
                              <div class="pf-v5-l-flex__item">
                                <i
                                  class="fas fa-exclamation-triangle"
                                  aria-hidden="true"
                                ></i>
                              </div>
                              <div class="pf-v5-l-flex__item">
                                <span>4</span>
                              </div>
                            </div>
                            <div class="pf-v5-l-flex pf-m-space-items-xs">
                              <div class="pf-v5-l-flex__item">
                                <i
                                  class="fas fa-times-circle"
                                  aria-hidden="true"
                                ></i>
                              </div>
                              <div class="pf-v5-l-flex__item">
                                <span>1</span>
                              </div>
                            </div>
                            <div class="pf-v5-l-flex__item">Updated 2 days ago</div>
                          </div>
                        </div>
                      </div>
                      <div
                        class="pf-v5-c-data-list__cell pf-m-align-right pf-m-no-fill"
                      >
                        <button
                          class="pf-v5-c-button pf-m-secondary"
                          type="button"
                        >Action</button>
                        <button
                          class="pf-v5-c-button pf-m-link"
                          type="button"
                        >Link</button>
                      </div>
                    </div>
                  </div>
                </li>

                <li
                  class="pf-v5-c-data-list__item"
                  aria-labelledby="primary-detail-expanded-example-data-list-item-3"
                >
                  <div class="pf-v5-c-data-list__item-row">
                    <div class="pf-v5-c-data-list__item-content">
                      <div class="pf-v5-c-data-list__cell pf-m-align-left">
                        <p
                          id="primary-detail-expanded-example-data-list-item-3"
                        >patternfly-unified-design-kit</p>
                      </div>
                      <div
                        class="pf-v5-c-data-list__cell pf-m-align-right pf-m-no-fill"
                      >
                        <button
                          class="pf-v5-c-button pf-m-secondary"
                          type="button"
                        >Action</button>
                        <button
                          class="pf-v5-c-button pf-m-link"
                          type="button"
                        >Link</button>
                      </div>
                    </div>
                  </div>
                </li>

                <li
                  class="pf-v5-c-data-list__item"
                  aria-labelledby="primary-detail-expanded-example-data-list-item-4"
                >
                  <div class="pf-v5-c-data-list__item-row">
                    <div class="pf-v5-c-data-list__item-content">
                      <div class="pf-v5-c-data-list__cell pf-m-align-left">
                        <div
                          class="pf-v5-l-flex pf-m-column pf-m-space-items-md"
                        >
                          <div
                            class="pf-v5-l-flex pf-m-column pf-m-space-items-none"
                          >
                            <div class="pf-v5-l-flex__item">
                              <p
                                id="primary-detail-expanded-example-data-list-item-4"
                              >patternfly</p>
                            </div>
                            <div class="pf-v5-l-flex__item">
                              <small>
                                Working repo for PatternFly 4
                                <a href>https://pf4.patternfly.org/</a>
                              </small>
                            </div>
                          </div>
                          <div class="pf-v5-l-flex pf-m-wrap">
                            <div class="pf-v5-l-flex pf-m-space-items-xs">
                              <div class="pf-v5-l-flex__item">
                                <i
                                  class="fas fa-code-branch"
                                  aria-hidden="true"
                                ></i>
                              </div>
                              <div class="pf-v5-l-flex__item">
                                <span>10</span>
                              </div>
                            </div>
                            <div class="pf-v5-l-flex pf-m-space-items-xs">
                              <div class="pf-v5-l-flex__item">
                                <i class="fas fa-code" aria-hidden="true"></i>
                              </div>
                              <div class="pf-v5-l-flex__item">
                                <span>4</span>
                              </div>
                            </div>
                            <div class="pf-v5-l-flex pf-m-space-items-xs">
                              <div class="pf-v5-l-flex__item">
                                <i class="fas fa-cube" aria-hidden="true"></i>
                              </div>
                              <div class="pf-v5-l-flex__item">
                                <span>5</span>
                              </div>
                            </div>
                            <div class="pf-v5-l-flex__item">Updated 2 days ago</div>
                          </div>
                        </div>
                      </div>
                      <div
                        class="pf-v5-c-data-list__cell pf-m-align-right pf-m-no-fill"
                      >
                        <button
                          class="pf-v5-c-button pf-m-secondary"
                          type="button"
                        >Action</button>
                        <button
                          class="pf-v5-c-button pf-m-link"
                          type="button"
                        >Link</button>
                      </div>
                    </div>
                  </div>
                </li>

                <li
                  class="pf-v5-c-data-list__item"
                  aria-labelledby="primary-detail-expanded-example-data-list-item-5"
                >
                  <div class="pf-v5-c-data-list__item-row">
                    <div class="pf-v5-c-data-list__item-content">
                      <div class="pf-v5-c-data-list__cell pf-m-align-left">
                        <div
                          class="pf-v5-l-flex pf-m-column pf-m-space-items-md"
                        >
                          <div
                            class="pf-v5-l-flex pf-m-column pf-m-space-items-none"
                          >
                            <div class="pf-v5-l-flex__item">
                              <p
                                id="primary-detail-expanded-example-data-list-item-5"
                              >patternfly-elements</p>
                            </div>
                            <div class="pf-v5-l-flex__item">
                              <small>PatternFly elements</small>
                            </div>
                          </div>
                          <div class="pf-v5-l-flex pf-m-wrap">
                            <div class="pf-v5-l-flex pf-m-space-items-xs">
                              <div class="pf-v5-l-flex__item">
                                <i
                                  class="fas fa-code-branch"
                                  aria-hidden="true"
                                ></i>
                              </div>
                              <div class="pf-v5-l-flex__item">
                                <span>5</span>
                              </div>
                            </div>
                            <div class="pf-v5-l-flex pf-m-space-items-xs">
                              <div class="pf-v5-l-flex__item">
                                <i class="fas fa-code" aria-hidden="true"></i>
                              </div>
                              <div class="pf-v5-l-flex__item">
                                <span>9</span>
                              </div>
                            </div>
                            <div class="pf-v5-l-flex pf-m-space-items-xs">
                              <div class="pf-v5-l-flex__item">
                                <i class="fas fa-cube" aria-hidden="true"></i>
                              </div>
                              <div class="pf-v5-l-flex__item">
                                <span>2</span>
                              </div>
                            </div>
                            <div class="pf-v5-l-flex pf-m-space-items-xs">
                              <div class="pf-v5-l-flex__item">
                                <i
                                  class="fas fa-check-circle"
                                  aria-hidden="true"
                                ></i>
                              </div>
                              <div class="pf-v5-l-flex__item">
                                <span>11</span>
                              </div>
                            </div>
                            <div class="pf-v5-l-flex pf-m-space-items-xs">
                              <div class="pf-v5-l-flex__item">
                                <i
                                  class="fas fa-exclamation-triangle"
                                  aria-hidden="true"
                                ></i>
                              </div>
                              <div class="pf-v5-l-flex__item">
                                <span>4</span>
                              </div>
                            </div>
                            <div class="pf-v5-l-flex pf-m-space-items-xs">
                              <div class="pf-v5-l-flex__item">
                                <i
                                  class="fas fa-times-circle"
                                  aria-hidden="true"
                                ></i>
                              </div>
                              <div class="pf-v5-l-flex__item">
                                <span>1</span>
                              </div>
                            </div>
                            <div class="pf-v5-l-flex__item">Updated 2 days ago</div>
                          </div>
                        </div>
                      </div>
                      <div
                        class="pf-v5-c-data-list__cell pf-m-align-right pf-m-no-fill"
                      >
                        <button
                          class="pf-v5-c-button pf-m-secondary"
                          type="button"
                        >Action</button>
                        <button
                          class="pf-v5-c-button pf-m-link"
                          type="button"
                        >Link</button>
                      </div>
                    </div>
                  </div>
                </li>
              </ul>
            </div>
          </div>

          <!-- Panel -->
          <div class="pf-v5-c-drawer__panel">
            <!-- Panel header -->
            <div class="pf-v5-c-drawer__body">
              <div class="pf-v5-l-flex pf-m-column">
                <div class="pf-v5-l-flex__item">
                  <div class="pf-v5-c-drawer__head">
                    <div class="pf-v5-c-drawer__actions">
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
                    <h2
                      class="pf-v5-c-title pf-m-lg"
                      id="primary-detail-expanded-example-drawer-drawer-label"
                    >Node 2</h2>
                  </div>
                </div>
                <div class="pf-v5-l-flex__item">
                  <a href="#">siemur/test-space</a>
                </div>
              </div>
            </div>

            <!-- Tabs -->
            <div class="pf-v5-c-drawer__body pf-m-no-padding">
              <div
                class="pf-v5-c-tabs pf-m-box pf-m-fill"
                role="region"
                id="primary-detail-expanded-example-drawer-tabs"
              >
                <button
                  class="pf-v5-c-tabs__scroll-button"
                  type="button"
                  aria-label="Scroll left"
                >
                  <i class="fas fa-angle-left" aria-hidden="true"></i>
                </button>
                <ul class="pf-v5-c-tabs__list" role="tablist">
                  <li
                    class="pf-v5-c-tabs__item pf-m-current"
                    role="presentation"
                  >
                    <button
                      type="button"
                      class="pf-v5-c-tabs__link"
                      role="tab"
                      aria-controls="primary-detail-expanded-example-drawer-tabs-tab1-panel"
                      id="primary-detail-expanded-example-drawer-tabs-tab1-link"
                    >
                      <span class="pf-v5-c-tabs__item-text">Overview</span>
                    </button>
                  </li>
                  <li class="pf-v5-c-tabs__item" role="presentation">
                    <button
                      type="button"
                      class="pf-v5-c-tabs__link"
                      role="tab"
                      aria-controls="primary-detail-expanded-example-drawer-tabs-tab2-panel"
                      id="primary-detail-expanded-example-drawer-tabs-tab2-link"
                    >
                      <span class="pf-v5-c-tabs__item-text">Activity</span>
                    </button>
                  </li>
                </ul>
                <button
                  class="pf-v5-c-tabs__scroll-button"
                  type="button"
                  aria-label="Scroll right"
                >
                  <i class="fas fa-angle-right" aria-hidden="true"></i>
                </button>
              </div>
            </div>

            <!-- Tab content -->
            <div class="pf-v5-c-drawer__body">
              <section
                class="pf-v5-c-tab-content"
                id="primary-detail-expanded-example-drawer-tabs-tab1-panel"
                aria-labelledby="primary-detail-expanded-example-drawer-tabs-tab1-link"
                role="tabpanel"
                tabindex="0"
              >
                <div class="pf-v5-c-tab-content__body">
                  <div class="pf-v5-l-flex pf-m-column pf-m-space-items-lg">
                    <div class="pf-v5-l-flex__item">
                      <p>The content of the drawer really is up to you. It could have form fields, definition lists, text lists, labels, charts, progress bars, etc. Spacing recommendation is 24px margins. You can put tabs in here, and can also make the drawer scrollable.</p>
                    </div>
                    <div class="pf-v5-l-flex__item">
                      <div
                        class="pf-v5-c-progress pf-m-sm"
                        id="primary-detail-expanded-example-drawer-progress-example1"
                      >
                        <div
                          class="pf-v5-c-progress__description"
                          id="primary-detail-expanded-example-drawer-progress-example1-description"
                        >Capacity</div>
                        <div
                          class="pf-v5-c-progress__status"
                          aria-hidden="true"
                        >
                          <span class="pf-v5-c-progress__measure">33%</span>
                        </div>
                        <div
                          class="pf-v5-c-progress__bar"
                          role="progressbar"
                          aria-valuemin="0"
                          aria-valuemax="100"
                          aria-valuenow="33"
                          aria-labelledby="primary-detail-expanded-example-drawer-progress-example1-description"
                          aria-label="Progress 1"
                        >
                          <div
                            class="pf-v5-c-progress__indicator"
                            style="width:33%;"
                          ></div>
                        </div>
                      </div>
                    </div>
                    <div class="pf-v5-l-flex__item">
                      <div
                        class="pf-v5-c-progress pf-m-sm"
                        id="primary-detail-expanded-example-drawer-progress-example2"
                      >
                        <div
                          class="pf-v5-c-progress__description"
                          id="primary-detail-expanded-example-drawer-progress-example2-description"
                        >Modules</div>
                        <div
                          class="pf-v5-c-progress__status"
                          aria-hidden="true"
                        >
                          <span class="pf-v5-c-progress__measure">66%</span>
                        </div>
                        <div
                          class="pf-v5-c-progress__bar"
                          role="progressbar"
                          aria-valuemin="0"
                          aria-valuemax="100"
                          aria-valuenow="66"
                          aria-labelledby="primary-detail-expanded-example-drawer-progress-example2-description"
                          aria-label="Progress 2"
                        >
                          <div
                            class="pf-v5-c-progress__indicator"
                            style="width:66%;"
                          ></div>
                        </div>
                      </div>
                    </div>
                  </div>
                </div>
              </section>
              <section
                class="pf-v5-c-tab-content"
                id="primary-detail-expanded-example-drawer-tabs-tab2-panel"
                aria-labelledby="primary-detail-expanded-example-drawer-tabs-tab2-link"
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
    </section>
  </main>
</div>

```

### Primary-detail collapsed

```html isFullscreen
<div class="pf-v5-c-page" id="primary-detail-collapsed-example">
  <div class="pf-v5-c-skip-to-content">
    <a
      class="pf-v5-c-button pf-m-primary"
      href="#main-content-primary-detail-collapsed-example"
    >Skip to content</a>
  </div>
  <header
    class="pf-v5-c-masthead pf-m-display-stack pf-m-display-inline-on-lg"
    id="primary-detail-collapsed-example-masthead"
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
        id="primary-detail-collapsed-example-masthead-toolbar"
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
        id="primary-detail-collapsed-example-primary-nav"
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
    id="main-content-primary-detail-collapsed-example"
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
    <div class="pf-v5-c-divider" role="separator"></div>
    <section class="pf-v5-c-page__main-section pf-m-no-padding">
      <!-- Drawer -->
      <div class="pf-v5-c-drawer pf-m-inline-on-2xl">
        <div class="pf-v5-c-drawer__main">
          <!-- Content -->
          <div class="pf-v5-c-drawer__content">
            <div class="pf-v5-c-drawer__body">
              <div
                class="pf-v5-c-toolbar pf-m-page-insets"
                id="primary-detail-collapsed-example-drawer-toolbar"
              >
                <div class="pf-v5-c-toolbar__content">
                  <div class="pf-v5-c-toolbar__content-section pf-m-nowrap">
                    <div
                      class="pf-v5-c-toolbar__group pf-m-toggle-group pf-m-show-on-xl"
                    >
                      <div class="pf-v5-c-toolbar__toggle">
                        <button
                          class="pf-v5-c-button pf-m-plain"
                          type="button"
                          aria-label="Show filters"
                          aria-expanded="false"
                          aria-controls="primary-detail-collapsed-example-drawer-toolbar-expandable-content"
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
                                id="primary-detail-collapsed-example-drawer-toolbar-select-name-label"
                                hidden
                              >Choose one</span>

                              <button
                                class="pf-v5-c-select__toggle"
                                type="button"
                                id="primary-detail-collapsed-example-drawer-toolbar-select-name-toggle"
                                aria-haspopup="true"
                                aria-expanded="false"
                                aria-labelledby="primary-detail-collapsed-example-drawer-toolbar-select-name-label primary-detail-collapsed-example-drawer-toolbar-select-name-toggle"
                              >
                                <div class="pf-v5-c-select__toggle-wrapper">
                                  <span class="pf-v5-c-select__toggle-icon">
                                    <i class="fas fa-filter" aria-hidden="true"></i>
                                  </span>
                                  <span class="pf-v5-c-select__toggle-text">Name</span>
                                </div>
                                <span class="pf-v5-c-select__toggle-arrow">
                                  <i
                                    class="fas fa-caret-down"
                                    aria-hidden="true"
                                  ></i>
                                </span>
                              </button>

                              <ul
                                class="pf-v5-c-select__menu"
                                role="listbox"
                                aria-labelledby="primary-detail-collapsed-example-drawer-toolbar-select-name-label"
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
                                    <span
                                      class="pf-v5-c-select__menu-item-icon"
                                    >
                                      <i
                                        class="fas fa-check"
                                        aria-hidden="true"
                                      ></i>
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
                              <div
                                class="pf-v5-c-text-input-group__main pf-m-icon"
                              >
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

                      <div class="pf-v5-c-toolbar__group pf-m-filter-group">
                        <div class="pf-v5-c-toolbar__item">
                          <div class="pf-v5-c-select">
                            <span
                              id="primary-detail-collapsed-example-drawer-toolbar-select-checkbox-status-label"
                              hidden
                            >Choose many</span>

                            <button
                              class="pf-v5-c-select__toggle"
                              type="button"
                              id="primary-detail-collapsed-example-drawer-toolbar-select-checkbox-status-toggle"
                              aria-haspopup="true"
                              aria-expanded="false"
                              aria-labelledby="primary-detail-collapsed-example-drawer-toolbar-select-checkbox-status-label primary-detail-collapsed-example-drawer-toolbar-select-checkbox-status-toggle"
                            >
                              <div class="pf-v5-c-select__toggle-wrapper">
                                <span class="pf-v5-c-select__toggle-text">Status</span>
                              </div>
                              <span class="pf-v5-c-select__toggle-arrow">
                                <i class="fas fa-caret-down" aria-hidden="true"></i>
                              </span>
                            </button>

                            <div class="pf-v5-c-select__menu" hidden>
                              <fieldset
                                class="pf-v5-c-select__menu-fieldset"
                                aria-label="Select input"
                              >
                                <label
                                  class="pf-v5-c-check pf-v5-c-select__menu-item pf-m-description"
                                  for="primary-detail-collapsed-example-drawer-toolbar-select-checkbox-status-active"
                                >
                                  <input
                                    class="pf-v5-c-check__input"
                                    type="checkbox"
                                    id="primary-detail-collapsed-example-drawer-toolbar-select-checkbox-status-active"
                                    name="primary-detail-collapsed-example-drawer-toolbar-select-checkbox-status-active"
                                  />

                                  <span class="pf-v5-c-check__label">Active</span>
                                  <span
                                    class="pf-v5-c-check__description"
                                  >This is a description</span>
                                </label>
                                <label
                                  class="pf-v5-c-check pf-v5-c-select__menu-item pf-m-description"
                                  for="primary-detail-collapsed-example-drawer-toolbar-select-checkbox-status-canceled"
                                >
                                  <input
                                    class="pf-v5-c-check__input"
                                    type="checkbox"
                                    id="primary-detail-collapsed-example-drawer-toolbar-select-checkbox-status-canceled"
                                    name="primary-detail-collapsed-example-drawer-toolbar-select-checkbox-status-canceled"
                                  />

                                  <span class="pf-v5-c-check__label">Canceled</span>
                                  <span
                                    class="pf-v5-c-check__description"
                                  >This is a really long description that describes the menu item. This is a really long description that describes the menu item.</span>
                                </label>
                                <label
                                  class="pf-v5-c-check pf-v5-c-select__menu-item"
                                  for="primary-detail-collapsed-example-drawer-toolbar-select-checkbox-status-paused"
                                >
                                  <input
                                    class="pf-v5-c-check__input"
                                    type="checkbox"
                                    id="primary-detail-collapsed-example-drawer-toolbar-select-checkbox-status-paused"
                                    name="primary-detail-collapsed-example-drawer-toolbar-select-checkbox-status-paused"
                                  />

                                  <span class="pf-v5-c-check__label">Paused</span>
                                </label>
                                <label
                                  class="pf-v5-c-check pf-v5-c-select__menu-item"
                                  for="primary-detail-collapsed-example-drawer-toolbar-select-checkbox-status-warning"
                                >
                                  <input
                                    class="pf-v5-c-check__input"
                                    type="checkbox"
                                    id="primary-detail-collapsed-example-drawer-toolbar-select-checkbox-status-warning"
                                    name="primary-detail-collapsed-example-drawer-toolbar-select-checkbox-status-warning"
                                  />

                                  <span class="pf-v5-c-check__label">Warning</span>
                                </label>
                                <label
                                  class="pf-v5-c-check pf-v5-c-select__menu-item"
                                  for="primary-detail-collapsed-example-drawer-toolbar-select-checkbox-status-restarted"
                                >
                                  <input
                                    class="pf-v5-c-check__input"
                                    type="checkbox"
                                    id="primary-detail-collapsed-example-drawer-toolbar-select-checkbox-status-restarted"
                                    name="primary-detail-collapsed-example-drawer-toolbar-select-checkbox-status-restarted"
                                  />

                                  <span class="pf-v5-c-check__label">Restarted</span>
                                </label>
                              </fieldset>
                            </div>
                          </div>
                        </div>
                        <div class="pf-v5-c-toolbar__item">
                          <div class="pf-v5-c-select">
                            <span
                              id="primary-detail-collapsed-example-drawer-toolbar-select-checkbox-risk-label"
                              hidden
                            >Choose many</span>

                            <button
                              class="pf-v5-c-select__toggle"
                              type="button"
                              id="primary-detail-collapsed-example-drawer-toolbar-select-checkbox-risk-toggle"
                              aria-haspopup="true"
                              aria-expanded="false"
                              aria-labelledby="primary-detail-collapsed-example-drawer-toolbar-select-checkbox-risk-label primary-detail-collapsed-example-drawer-toolbar-select-checkbox-risk-toggle"
                            >
                              <div class="pf-v5-c-select__toggle-wrapper">
                                <span class="pf-v5-c-select__toggle-text">Risk</span>
                              </div>
                              <span class="pf-v5-c-select__toggle-arrow">
                                <i class="fas fa-caret-down" aria-hidden="true"></i>
                              </span>
                            </button>

                            <div class="pf-v5-c-select__menu" hidden>
                              <fieldset
                                class="pf-v5-c-select__menu-fieldset"
                                aria-label="Select input"
                              >
                                <label
                                  class="pf-v5-c-check pf-v5-c-select__menu-item pf-m-description"
                                  for="primary-detail-collapsed-example-drawer-toolbar-select-checkbox-risk-active"
                                >
                                  <input
                                    class="pf-v5-c-check__input"
                                    type="checkbox"
                                    id="primary-detail-collapsed-example-drawer-toolbar-select-checkbox-risk-active"
                                    name="primary-detail-collapsed-example-drawer-toolbar-select-checkbox-risk-active"
                                  />

                                  <span class="pf-v5-c-check__label">Active</span>
                                  <span
                                    class="pf-v5-c-check__description"
                                  >This is a description</span>
                                </label>
                                <label
                                  class="pf-v5-c-check pf-v5-c-select__menu-item pf-m-description"
                                  for="primary-detail-collapsed-example-drawer-toolbar-select-checkbox-risk-canceled"
                                >
                                  <input
                                    class="pf-v5-c-check__input"
                                    type="checkbox"
                                    id="primary-detail-collapsed-example-drawer-toolbar-select-checkbox-risk-canceled"
                                    name="primary-detail-collapsed-example-drawer-toolbar-select-checkbox-risk-canceled"
                                  />

                                  <span class="pf-v5-c-check__label">Canceled</span>
                                  <span
                                    class="pf-v5-c-check__description"
                                  >This is a really long description that describes the menu item. This is a really long description that describes the menu item.</span>
                                </label>
                                <label
                                  class="pf-v5-c-check pf-v5-c-select__menu-item"
                                  for="primary-detail-collapsed-example-drawer-toolbar-select-checkbox-risk-paused"
                                >
                                  <input
                                    class="pf-v5-c-check__input"
                                    type="checkbox"
                                    id="primary-detail-collapsed-example-drawer-toolbar-select-checkbox-risk-paused"
                                    name="primary-detail-collapsed-example-drawer-toolbar-select-checkbox-risk-paused"
                                  />

                                  <span class="pf-v5-c-check__label">Paused</span>
                                </label>
                                <label
                                  class="pf-v5-c-check pf-v5-c-select__menu-item"
                                  for="primary-detail-collapsed-example-drawer-toolbar-select-checkbox-risk-warning"
                                >
                                  <input
                                    class="pf-v5-c-check__input"
                                    type="checkbox"
                                    id="primary-detail-collapsed-example-drawer-toolbar-select-checkbox-risk-warning"
                                    name="primary-detail-collapsed-example-drawer-toolbar-select-checkbox-risk-warning"
                                  />

                                  <span class="pf-v5-c-check__label">Warning</span>
                                </label>
                                <label
                                  class="pf-v5-c-check pf-v5-c-select__menu-item"
                                  for="primary-detail-collapsed-example-drawer-toolbar-select-checkbox-risk-restarted"
                                >
                                  <input
                                    class="pf-v5-c-check__input"
                                    type="checkbox"
                                    id="primary-detail-collapsed-example-drawer-toolbar-select-checkbox-risk-restarted"
                                    name="primary-detail-collapsed-example-drawer-toolbar-select-checkbox-risk-restarted"
                                  />

                                  <span class="pf-v5-c-check__label">Restarted</span>
                                </label>
                              </fieldset>
                            </div>
                          </div>
                        </div>
                      </div>
                    </div>

                    <div class="pf-v5-c-toolbar__item">
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
                    </div>

                    <div
                      class="pf-v5-c-overflow-menu"
                      id="primary-detail-collapsed-example-drawer-toolbar-overflow-menu"
                    >
                      <div
                        class="pf-v5-c-overflow-menu__content pf-v5-u-display-none pf-v5-u-display-flex-on-lg"
                      >
                        <div
                          class="pf-v5-c-overflow-menu__group pf-m-button-group"
                        >
                          <div class="pf-v5-c-overflow-menu__item">
                            <button
                              class="pf-v5-c-button pf-m-primary"
                              type="button"
                            >Create instance</button>
                          </div>
                        </div>
                      </div>
                      <div class="pf-v5-c-overflow-menu__control">
                        <div class="pf-v5-c-dropdown">
                          <button
                            class="pf-v5-c-button pf-v5-c-dropdown__toggle pf-m-plain"
                            type="button"
                            id="primary-detail-collapsed-example-drawer-toolbar-overflow-menu-dropdown-toggle"
                            aria-label="Dropdown with additional options"
                            aria-expanded="false"
                          >
                            <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
                          </button>
                          <ul
                            class="pf-v5-c-dropdown__menu"
                            role="menu"
                            aria-labelledby="primary-detail-collapsed-example-drawer-toolbar-overflow-menu-dropdown-toggle"
                            hidden
                          >
                            <li role="none">
                              <button
                                role="menuitem"
                                class="pf-v5-c-dropdown__menu-item"
                              >Action 7</button>
                            </li>
                          </ul>
                        </div>
                      </div>
                    </div>

                    <div class="pf-v5-c-toolbar__item pf-m-pagination">
                      <div class="pf-v5-c-pagination pf-m-compact">
                        <div class="pf-v5-c-options-menu">
                          <button
                            class="pf-v5-c-options-menu__toggle pf-m-text pf-m-plain"
                            type="button"
                            id="primary-detail-collapsed-example-drawer-toolbar-top-pagination-toggle"
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
                            aria-labelledby="primary-detail-collapsed-example-drawer-toolbar-top-pagination-toggle"
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
                                <div
                                  class="pf-v5-c-options-menu__menu-item-icon"
                                >
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
                          aria-label="Toolbar top pagination"
                        >
                          <div
                            class="pf-v5-c-pagination__nav-control pf-m-prev"
                          >
                            <button
                              class="pf-v5-c-button pf-m-plain"
                              type="button"
                              disabled
                              aria-label="Go to previous page"
                            >
                              <i class="fas fa-angle-left" aria-hidden="true"></i>
                            </button>
                          </div>
                          <div
                            class="pf-v5-c-pagination__nav-control pf-m-next"
                          >
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
                    </div>
                  </div>

                  <div
                    class="pf-v5-c-toolbar__expandable-content pf-m-hidden"
                    id="primary-detail-collapsed-example-drawer-toolbar-expandable-content"
                    hidden
                  ></div>
                </div>
              </div>
              <ul
                class="pf-v5-c-data-list"
                role="list"
                aria-label="Simple data list example"
                id="primary-detail-collapsed-example-data-list"
              >
                <li
                  class="pf-v5-c-data-list__item"
                  aria-labelledby="primary-detail-collapsed-example-data-list-item-1"
                >
                  <div class="pf-v5-c-data-list__item-row">
                    <div class="pf-v5-c-data-list__item-content">
                      <div class="pf-v5-c-data-list__cell pf-m-align-left">
                        <div
                          class="pf-v5-l-flex pf-m-column pf-m-space-items-md"
                        >
                          <div
                            class="pf-v5-l-flex pf-m-column pf-m-space-items-none"
                          >
                            <div class="pf-v5-l-flex__item">
                              <p
                                id="primary-detail-collapsed-example-data-list-item-1"
                              >patternfly</p>
                            </div>
                            <div class="pf-v5-l-flex__item">
                              <small>
                                Working repo for PatternFly 4
                                <a href>https://pf4.patternfly.org/</a>
                              </small>
                            </div>
                          </div>
                          <div class="pf-v5-l-flex pf-m-wrap">
                            <div class="pf-v5-l-flex pf-m-space-items-xs">
                              <div class="pf-v5-l-flex__item">
                                <i
                                  class="fas fa-code-branch"
                                  aria-hidden="true"
                                ></i>
                              </div>
                              <div class="pf-v5-l-flex__item">
                                <span>10</span>
                              </div>
                            </div>
                            <div class="pf-v5-l-flex pf-m-space-items-xs">
                              <div class="pf-v5-l-flex__item">
                                <i class="fas fa-code" aria-hidden="true"></i>
                              </div>
                              <div class="pf-v5-l-flex__item">
                                <span>4</span>
                              </div>
                            </div>
                            <div class="pf-v5-l-flex pf-m-space-items-xs">
                              <div class="pf-v5-l-flex__item">
                                <i class="fas fa-cube" aria-hidden="true"></i>
                              </div>
                              <div class="pf-v5-l-flex__item">
                                <span>5</span>
                              </div>
                            </div>
                            <div class="pf-v5-l-flex__item">Updated 2 days ago</div>
                          </div>
                        </div>
                      </div>
                      <div
                        class="pf-v5-c-data-list__cell pf-m-align-right pf-m-no-fill"
                      >
                        <button
                          class="pf-v5-c-button pf-m-secondary"
                          type="button"
                        >Action</button>
                        <button
                          class="pf-v5-c-button pf-m-link"
                          type="button"
                        >Link</button>
                      </div>
                    </div>
                  </div>
                </li>

                <li
                  class="pf-v5-c-data-list__item"
                  aria-labelledby="primary-detail-collapsed-example-data-list-item-2"
                >
                  <div class="pf-v5-c-data-list__item-row">
                    <div class="pf-v5-c-data-list__item-content">
                      <div class="pf-v5-c-data-list__cell pf-m-align-left">
                        <div
                          class="pf-v5-l-flex pf-m-column pf-m-space-items-md"
                        >
                          <div
                            class="pf-v5-l-flex pf-m-column pf-m-space-items-none"
                          >
                            <div class="pf-v5-l-flex__item">
                              <p
                                id="primary-detail-collapsed-example-data-list-item-2"
                              >patternfly-elements</p>
                            </div>
                            <div class="pf-v5-l-flex__item">
                              <small>PatternFly elements</small>
                            </div>
                          </div>
                          <div class="pf-v5-l-flex pf-m-wrap">
                            <div class="pf-v5-l-flex pf-m-space-items-xs">
                              <div class="pf-v5-l-flex__item">
                                <i
                                  class="fas fa-code-branch"
                                  aria-hidden="true"
                                ></i>
                              </div>
                              <div class="pf-v5-l-flex__item">
                                <span>5</span>
                              </div>
                            </div>
                            <div class="pf-v5-l-flex pf-m-space-items-xs">
                              <div class="pf-v5-l-flex__item">
                                <i class="fas fa-code" aria-hidden="true"></i>
                              </div>
                              <div class="pf-v5-l-flex__item">
                                <span>9</span>
                              </div>
                            </div>
                            <div class="pf-v5-l-flex pf-m-space-items-xs">
                              <div class="pf-v5-l-flex__item">
                                <i class="fas fa-cube" aria-hidden="true"></i>
                              </div>
                              <div class="pf-v5-l-flex__item">
                                <span>2</span>
                              </div>
                            </div>
                            <div class="pf-v5-l-flex pf-m-space-items-xs">
                              <div class="pf-v5-l-flex__item">
                                <i
                                  class="fas fa-check-circle"
                                  aria-hidden="true"
                                ></i>
                              </div>
                              <div class="pf-v5-l-flex__item">
                                <span>11</span>
                              </div>
                            </div>
                            <div class="pf-v5-l-flex pf-m-space-items-xs">
                              <div class="pf-v5-l-flex__item">
                                <i
                                  class="fas fa-exclamation-triangle"
                                  aria-hidden="true"
                                ></i>
                              </div>
                              <div class="pf-v5-l-flex__item">
                                <span>4</span>
                              </div>
                            </div>
                            <div class="pf-v5-l-flex pf-m-space-items-xs">
                              <div class="pf-v5-l-flex__item">
                                <i
                                  class="fas fa-times-circle"
                                  aria-hidden="true"
                                ></i>
                              </div>
                              <div class="pf-v5-l-flex__item">
                                <span>1</span>
                              </div>
                            </div>
                            <div class="pf-v5-l-flex__item">Updated 2 days ago</div>
                          </div>
                        </div>
                      </div>
                      <div
                        class="pf-v5-c-data-list__cell pf-m-align-right pf-m-no-fill"
                      >
                        <button
                          class="pf-v5-c-button pf-m-secondary"
                          type="button"
                        >Action</button>
                        <button
                          class="pf-v5-c-button pf-m-link"
                          type="button"
                        >Link</button>
                      </div>
                    </div>
                  </div>
                </li>

                <li
                  class="pf-v5-c-data-list__item"
                  aria-labelledby="primary-detail-collapsed-example-data-list-item-3"
                >
                  <div class="pf-v5-c-data-list__item-row">
                    <div class="pf-v5-c-data-list__item-content">
                      <div class="pf-v5-c-data-list__cell pf-m-align-left">
                        <p
                          id="primary-detail-collapsed-example-data-list-item-3"
                        >patternfly-unified-design-kit</p>
                      </div>
                      <div
                        class="pf-v5-c-data-list__cell pf-m-align-right pf-m-no-fill"
                      >
                        <button
                          class="pf-v5-c-button pf-m-secondary"
                          type="button"
                        >Action</button>
                        <button
                          class="pf-v5-c-button pf-m-link"
                          type="button"
                        >Link</button>
                      </div>
                    </div>
                  </div>
                </li>

                <li
                  class="pf-v5-c-data-list__item"
                  aria-labelledby="primary-detail-collapsed-example-data-list-item-4"
                >
                  <div class="pf-v5-c-data-list__item-row">
                    <div class="pf-v5-c-data-list__item-content">
                      <div class="pf-v5-c-data-list__cell pf-m-align-left">
                        <div
                          class="pf-v5-l-flex pf-m-column pf-m-space-items-md"
                        >
                          <div
                            class="pf-v5-l-flex pf-m-column pf-m-space-items-none"
                          >
                            <div class="pf-v5-l-flex__item">
                              <p
                                id="primary-detail-collapsed-example-data-list-item-4"
                              >patternfly</p>
                            </div>
                            <div class="pf-v5-l-flex__item">
                              <small>
                                Working repo for PatternFly 4
                                <a href>https://pf4.patternfly.org/</a>
                              </small>
                            </div>
                          </div>
                          <div class="pf-v5-l-flex pf-m-wrap">
                            <div class="pf-v5-l-flex pf-m-space-items-xs">
                              <div class="pf-v5-l-flex__item">
                                <i
                                  class="fas fa-code-branch"
                                  aria-hidden="true"
                                ></i>
                              </div>
                              <div class="pf-v5-l-flex__item">
                                <span>10</span>
                              </div>
                            </div>
                            <div class="pf-v5-l-flex pf-m-space-items-xs">
                              <div class="pf-v5-l-flex__item">
                                <i class="fas fa-code" aria-hidden="true"></i>
                              </div>
                              <div class="pf-v5-l-flex__item">
                                <span>4</span>
                              </div>
                            </div>
                            <div class="pf-v5-l-flex pf-m-space-items-xs">
                              <div class="pf-v5-l-flex__item">
                                <i class="fas fa-cube" aria-hidden="true"></i>
                              </div>
                              <div class="pf-v5-l-flex__item">
                                <span>5</span>
                              </div>
                            </div>
                            <div class="pf-v5-l-flex__item">Updated 2 days ago</div>
                          </div>
                        </div>
                      </div>
                      <div
                        class="pf-v5-c-data-list__cell pf-m-align-right pf-m-no-fill"
                      >
                        <button
                          class="pf-v5-c-button pf-m-secondary"
                          type="button"
                        >Action</button>
                        <button
                          class="pf-v5-c-button pf-m-link"
                          type="button"
                        >Link</button>
                      </div>
                    </div>
                  </div>
                </li>

                <li
                  class="pf-v5-c-data-list__item"
                  aria-labelledby="primary-detail-collapsed-example-data-list-item-5"
                >
                  <div class="pf-v5-c-data-list__item-row">
                    <div class="pf-v5-c-data-list__item-content">
                      <div class="pf-v5-c-data-list__cell pf-m-align-left">
                        <div
                          class="pf-v5-l-flex pf-m-column pf-m-space-items-md"
                        >
                          <div
                            class="pf-v5-l-flex pf-m-column pf-m-space-items-none"
                          >
                            <div class="pf-v5-l-flex__item">
                              <p
                                id="primary-detail-collapsed-example-data-list-item-5"
                              >patternfly-elements</p>
                            </div>
                            <div class="pf-v5-l-flex__item">
                              <small>PatternFly elements</small>
                            </div>
                          </div>
                          <div class="pf-v5-l-flex pf-m-wrap">
                            <div class="pf-v5-l-flex pf-m-space-items-xs">
                              <div class="pf-v5-l-flex__item">
                                <i
                                  class="fas fa-code-branch"
                                  aria-hidden="true"
                                ></i>
                              </div>
                              <div class="pf-v5-l-flex__item">
                                <span>5</span>
                              </div>
                            </div>
                            <div class="pf-v5-l-flex pf-m-space-items-xs">
                              <div class="pf-v5-l-flex__item">
                                <i class="fas fa-code" aria-hidden="true"></i>
                              </div>
                              <div class="pf-v5-l-flex__item">
                                <span>9</span>
                              </div>
                            </div>
                            <div class="pf-v5-l-flex pf-m-space-items-xs">
                              <div class="pf-v5-l-flex__item">
                                <i class="fas fa-cube" aria-hidden="true"></i>
                              </div>
                              <div class="pf-v5-l-flex__item">
                                <span>2</span>
                              </div>
                            </div>
                            <div class="pf-v5-l-flex pf-m-space-items-xs">
                              <div class="pf-v5-l-flex__item">
                                <i
                                  class="fas fa-check-circle"
                                  aria-hidden="true"
                                ></i>
                              </div>
                              <div class="pf-v5-l-flex__item">
                                <span>11</span>
                              </div>
                            </div>
                            <div class="pf-v5-l-flex pf-m-space-items-xs">
                              <div class="pf-v5-l-flex__item">
                                <i
                                  class="fas fa-exclamation-triangle"
                                  aria-hidden="true"
                                ></i>
                              </div>
                              <div class="pf-v5-l-flex__item">
                                <span>4</span>
                              </div>
                            </div>
                            <div class="pf-v5-l-flex pf-m-space-items-xs">
                              <div class="pf-v5-l-flex__item">
                                <i
                                  class="fas fa-times-circle"
                                  aria-hidden="true"
                                ></i>
                              </div>
                              <div class="pf-v5-l-flex__item">
                                <span>1</span>
                              </div>
                            </div>
                            <div class="pf-v5-l-flex__item">Updated 2 days ago</div>
                          </div>
                        </div>
                      </div>
                      <div
                        class="pf-v5-c-data-list__cell pf-m-align-right pf-m-no-fill"
                      >
                        <button
                          class="pf-v5-c-button pf-m-secondary"
                          type="button"
                        >Action</button>
                        <button
                          class="pf-v5-c-button pf-m-link"
                          type="button"
                        >Link</button>
                      </div>
                    </div>
                  </div>
                </li>
              </ul>
            </div>
          </div>

          <!-- Panel -->
          <div class="pf-v5-c-drawer__panel" hidden>
            <!-- Panel header -->
            <div class="pf-v5-c-drawer__body">
              <div class="pf-v5-l-flex pf-m-column">
                <div class="pf-v5-l-flex__item">
                  <div class="pf-v5-c-drawer__head">
                    <div class="pf-v5-c-drawer__actions">
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
                    <h2
                      class="pf-v5-c-title pf-m-lg"
                      id="primary-detail-collapsed-example-drawer-drawer-label"
                    >Patternfly-elements</h2>
                  </div>
                </div>
                <div class="pf-v5-l-flex__item">PatternFly elements</div>
              </div>
            </div>

            <!-- Tab content -->
            <div class="pf-v5-c-drawer__body">
              <div class="pf-v5-l-flex pf-m-column pf-m-space-items-lg">
                <div class="pf-v5-l-flex__item">
                  <p>The content of the drawer really is up to you. It could have form fields, definition lists, text lists, labels, charts, progress bars, etc. Spacing recommendation is 24px margins. You can put tabs in here, and can also make the drawer scrollable.</p>
                </div>
                <div class="pf-v5-l-flex__item">
                  <div
                    class="pf-v5-c-progress pf-m-sm"
                    id="primary-detail-collapsed-example-drawer-progress-example1"
                  >
                    <div
                      class="pf-v5-c-progress__description"
                      id="primary-detail-collapsed-example-drawer-progress-example1-description"
                    >Capacity</div>
                    <div class="pf-v5-c-progress__status" aria-hidden="true">
                      <span class="pf-v5-c-progress__measure">33%</span>
                    </div>
                    <div
                      class="pf-v5-c-progress__bar"
                      role="progressbar"
                      aria-valuemin="0"
                      aria-valuemax="100"
                      aria-valuenow="33"
                      aria-labelledby="primary-detail-collapsed-example-drawer-progress-example1-description"
                      aria-label="Progress 1"
                    >
                      <div
                        class="pf-v5-c-progress__indicator"
                        style="width:33%;"
                      ></div>
                    </div>
                  </div>
                </div>
                <div class="pf-v5-l-flex__item">
                  <div
                    class="pf-v5-c-progress pf-m-sm"
                    id="primary-detail-collapsed-example-drawer-progress-example2"
                  >
                    <div
                      class="pf-v5-c-progress__description"
                      id="primary-detail-collapsed-example-drawer-progress-example2-description"
                    >Modules</div>
                    <div class="pf-v5-c-progress__status" aria-hidden="true">
                      <span class="pf-v5-c-progress__measure">66%</span>
                    </div>
                    <div
                      class="pf-v5-c-progress__bar"
                      role="progressbar"
                      aria-valuemin="0"
                      aria-valuemax="100"
                      aria-valuenow="66"
                      aria-labelledby="primary-detail-collapsed-example-drawer-progress-example2-description"
                      aria-label="Progress 2"
                    >
                      <div
                        class="pf-v5-c-progress__indicator"
                        style="width:66%;"
                      ></div>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>
  </main>
</div>

```

### Primary-detail content body padding

```html isFullscreen
<div class="pf-v5-c-page" id="primary-detail-content-body-padding-example">
  <div class="pf-v5-c-skip-to-content">
    <a
      class="pf-v5-c-button pf-m-primary"
      href="#main-content-primary-detail-content-body-padding-example"
    >Skip to content</a>
  </div>
  <header
    class="pf-v5-c-masthead pf-m-display-stack pf-m-display-inline-on-lg"
    id="primary-detail-content-body-padding-example-masthead"
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
        id="primary-detail-content-body-padding-example-masthead-toolbar"
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
        id="primary-detail-content-body-padding-example-primary-nav"
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
    id="main-content-primary-detail-content-body-padding-example"
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
    <div class="pf-v5-c-divider" role="separator"></div>
    <section class="pf-v5-c-page__main-section pf-m-no-padding">
      <!-- Drawer -->
      <div class="pf-v5-c-drawer pf-m-expanded pf-m-inline-on-2xl">
        <div class="pf-v5-c-drawer__main">
          <!-- Content -->
          <div class="pf-v5-c-drawer__content pf-m-no-background">
            <div class="pf-v5-c-drawer__body pf-m-padding">
              <div
                class="pf-v5-c-toolbar pf-m-page-insets"
                id="primary-detail-content-body-padding-example-drawer-toolbar"
              >
                <div class="pf-v5-c-toolbar__content">
                  <div class="pf-v5-c-toolbar__content-section pf-m-nowrap">
                    <div
                      class="pf-v5-c-toolbar__group pf-m-toggle-group pf-m-show-on-xl"
                    >
                      <div class="pf-v5-c-toolbar__toggle">
                        <button
                          class="pf-v5-c-button pf-m-plain"
                          type="button"
                          aria-label="Show filters"
                          aria-expanded="false"
                          aria-controls="primary-detail-content-body-padding-example-drawer-toolbar-expandable-content"
                        >
                          <i class="fas fa-filter" aria-hidden="true"></i>
                        </button>
                      </div>

                      <div class="pf-v5-c-toolbar__group pf-m-filter-group">
                        <div class="pf-v5-c-toolbar__item">
                          <div class="pf-v5-c-select">
                            <span
                              id="primary-detail-content-body-padding-example-drawer-toolbar-select-checkbox-status-label"
                              hidden
                            >Choose many</span>

                            <button
                              class="pf-v5-c-select__toggle"
                              type="button"
                              id="primary-detail-content-body-padding-example-drawer-toolbar-select-checkbox-status-toggle"
                              aria-haspopup="true"
                              aria-expanded="false"
                              aria-labelledby="primary-detail-content-body-padding-example-drawer-toolbar-select-checkbox-status-label primary-detail-content-body-padding-example-drawer-toolbar-select-checkbox-status-toggle"
                            >
                              <div class="pf-v5-c-select__toggle-wrapper">
                                <span class="pf-v5-c-select__toggle-text">Status</span>
                              </div>
                              <span class="pf-v5-c-select__toggle-arrow">
                                <i class="fas fa-caret-down" aria-hidden="true"></i>
                              </span>
                            </button>

                            <div class="pf-v5-c-select__menu" hidden>
                              <fieldset
                                class="pf-v5-c-select__menu-fieldset"
                                aria-label="Select input"
                              >
                                <label
                                  class="pf-v5-c-check pf-v5-c-select__menu-item pf-m-description"
                                  for="primary-detail-content-body-padding-example-drawer-toolbar-select-checkbox-status-active"
                                >
                                  <input
                                    class="pf-v5-c-check__input"
                                    type="checkbox"
                                    id="primary-detail-content-body-padding-example-drawer-toolbar-select-checkbox-status-active"
                                    name="primary-detail-content-body-padding-example-drawer-toolbar-select-checkbox-status-active"
                                  />

                                  <span class="pf-v5-c-check__label">Active</span>
                                  <span
                                    class="pf-v5-c-check__description"
                                  >This is a description</span>
                                </label>
                                <label
                                  class="pf-v5-c-check pf-v5-c-select__menu-item pf-m-description"
                                  for="primary-detail-content-body-padding-example-drawer-toolbar-select-checkbox-status-canceled"
                                >
                                  <input
                                    class="pf-v5-c-check__input"
                                    type="checkbox"
                                    id="primary-detail-content-body-padding-example-drawer-toolbar-select-checkbox-status-canceled"
                                    name="primary-detail-content-body-padding-example-drawer-toolbar-select-checkbox-status-canceled"
                                  />

                                  <span class="pf-v5-c-check__label">Canceled</span>
                                  <span
                                    class="pf-v5-c-check__description"
                                  >This is a really long description that describes the menu item. This is a really long description that describes the menu item.</span>
                                </label>
                                <label
                                  class="pf-v5-c-check pf-v5-c-select__menu-item"
                                  for="primary-detail-content-body-padding-example-drawer-toolbar-select-checkbox-status-paused"
                                >
                                  <input
                                    class="pf-v5-c-check__input"
                                    type="checkbox"
                                    id="primary-detail-content-body-padding-example-drawer-toolbar-select-checkbox-status-paused"
                                    name="primary-detail-content-body-padding-example-drawer-toolbar-select-checkbox-status-paused"
                                  />

                                  <span class="pf-v5-c-check__label">Paused</span>
                                </label>
                                <label
                                  class="pf-v5-c-check pf-v5-c-select__menu-item"
                                  for="primary-detail-content-body-padding-example-drawer-toolbar-select-checkbox-status-warning"
                                >
                                  <input
                                    class="pf-v5-c-check__input"
                                    type="checkbox"
                                    id="primary-detail-content-body-padding-example-drawer-toolbar-select-checkbox-status-warning"
                                    name="primary-detail-content-body-padding-example-drawer-toolbar-select-checkbox-status-warning"
                                  />

                                  <span class="pf-v5-c-check__label">Warning</span>
                                </label>
                                <label
                                  class="pf-v5-c-check pf-v5-c-select__menu-item"
                                  for="primary-detail-content-body-padding-example-drawer-toolbar-select-checkbox-status-restarted"
                                >
                                  <input
                                    class="pf-v5-c-check__input"
                                    type="checkbox"
                                    id="primary-detail-content-body-padding-example-drawer-toolbar-select-checkbox-status-restarted"
                                    name="primary-detail-content-body-padding-example-drawer-toolbar-select-checkbox-status-restarted"
                                  />

                                  <span class="pf-v5-c-check__label">Restarted</span>
                                </label>
                              </fieldset>
                            </div>
                          </div>
                        </div>
                        <div class="pf-v5-c-toolbar__item">
                          <div class="pf-v5-c-select">
                            <span
                              id="primary-detail-content-body-padding-example-drawer-toolbar-select-checkbox-risk-label"
                              hidden
                            >Choose many</span>

                            <button
                              class="pf-v5-c-select__toggle"
                              type="button"
                              id="primary-detail-content-body-padding-example-drawer-toolbar-select-checkbox-risk-toggle"
                              aria-haspopup="true"
                              aria-expanded="false"
                              aria-labelledby="primary-detail-content-body-padding-example-drawer-toolbar-select-checkbox-risk-label primary-detail-content-body-padding-example-drawer-toolbar-select-checkbox-risk-toggle"
                            >
                              <div class="pf-v5-c-select__toggle-wrapper">
                                <span class="pf-v5-c-select__toggle-text">Risk</span>
                              </div>
                              <span class="pf-v5-c-select__toggle-arrow">
                                <i class="fas fa-caret-down" aria-hidden="true"></i>
                              </span>
                            </button>

                            <div class="pf-v5-c-select__menu" hidden>
                              <fieldset
                                class="pf-v5-c-select__menu-fieldset"
                                aria-label="Select input"
                              >
                                <label
                                  class="pf-v5-c-check pf-v5-c-select__menu-item pf-m-description"
                                  for="primary-detail-content-body-padding-example-drawer-toolbar-select-checkbox-risk-active"
                                >
                                  <input
                                    class="pf-v5-c-check__input"
                                    type="checkbox"
                                    id="primary-detail-content-body-padding-example-drawer-toolbar-select-checkbox-risk-active"
                                    name="primary-detail-content-body-padding-example-drawer-toolbar-select-checkbox-risk-active"
                                  />

                                  <span class="pf-v5-c-check__label">Active</span>
                                  <span
                                    class="pf-v5-c-check__description"
                                  >This is a description</span>
                                </label>
                                <label
                                  class="pf-v5-c-check pf-v5-c-select__menu-item pf-m-description"
                                  for="primary-detail-content-body-padding-example-drawer-toolbar-select-checkbox-risk-canceled"
                                >
                                  <input
                                    class="pf-v5-c-check__input"
                                    type="checkbox"
                                    id="primary-detail-content-body-padding-example-drawer-toolbar-select-checkbox-risk-canceled"
                                    name="primary-detail-content-body-padding-example-drawer-toolbar-select-checkbox-risk-canceled"
                                  />

                                  <span class="pf-v5-c-check__label">Canceled</span>
                                  <span
                                    class="pf-v5-c-check__description"
                                  >This is a really long description that describes the menu item. This is a really long description that describes the menu item.</span>
                                </label>
                                <label
                                  class="pf-v5-c-check pf-v5-c-select__menu-item"
                                  for="primary-detail-content-body-padding-example-drawer-toolbar-select-checkbox-risk-paused"
                                >
                                  <input
                                    class="pf-v5-c-check__input"
                                    type="checkbox"
                                    id="primary-detail-content-body-padding-example-drawer-toolbar-select-checkbox-risk-paused"
                                    name="primary-detail-content-body-padding-example-drawer-toolbar-select-checkbox-risk-paused"
                                  />

                                  <span class="pf-v5-c-check__label">Paused</span>
                                </label>
                                <label
                                  class="pf-v5-c-check pf-v5-c-select__menu-item"
                                  for="primary-detail-content-body-padding-example-drawer-toolbar-select-checkbox-risk-warning"
                                >
                                  <input
                                    class="pf-v5-c-check__input"
                                    type="checkbox"
                                    id="primary-detail-content-body-padding-example-drawer-toolbar-select-checkbox-risk-warning"
                                    name="primary-detail-content-body-padding-example-drawer-toolbar-select-checkbox-risk-warning"
                                  />

                                  <span class="pf-v5-c-check__label">Warning</span>
                                </label>
                                <label
                                  class="pf-v5-c-check pf-v5-c-select__menu-item"
                                  for="primary-detail-content-body-padding-example-drawer-toolbar-select-checkbox-risk-restarted"
                                >
                                  <input
                                    class="pf-v5-c-check__input"
                                    type="checkbox"
                                    id="primary-detail-content-body-padding-example-drawer-toolbar-select-checkbox-risk-restarted"
                                    name="primary-detail-content-body-padding-example-drawer-toolbar-select-checkbox-risk-restarted"
                                  />

                                  <span class="pf-v5-c-check__label">Restarted</span>
                                </label>
                              </fieldset>
                            </div>
                          </div>
                        </div>
                      </div>
                    </div>

                    <div class="pf-v5-c-toolbar__item">
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
                    </div>

                    <div
                      class="pf-v5-c-overflow-menu"
                      id="primary-detail-content-body-padding-example-drawer-toolbar-overflow-menu"
                    >
                      <div
                        class="pf-v5-c-overflow-menu__content pf-v5-u-display-none pf-v5-u-display-flex-on-lg"
                      >
                        <div
                          class="pf-v5-c-overflow-menu__group pf-m-button-group"
                        >
                          <div class="pf-v5-c-overflow-menu__item">
                            <button
                              class="pf-v5-c-button pf-m-primary"
                              type="button"
                            >Create instance</button>
                          </div>
                        </div>
                      </div>
                      <div class="pf-v5-c-overflow-menu__control">
                        <div class="pf-v5-c-dropdown">
                          <button
                            class="pf-v5-c-button pf-v5-c-dropdown__toggle pf-m-plain"
                            type="button"
                            id="primary-detail-content-body-padding-example-drawer-toolbar-overflow-menu-dropdown-toggle"
                            aria-label="Dropdown with additional options"
                            aria-expanded="false"
                          >
                            <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
                          </button>
                          <ul
                            class="pf-v5-c-dropdown__menu"
                            role="menu"
                            aria-labelledby="primary-detail-content-body-padding-example-drawer-toolbar-overflow-menu-dropdown-toggle"
                            hidden
                          >
                            <li role="none">
                              <button
                                role="menuitem"
                                class="pf-v5-c-dropdown__menu-item"
                              >Action 7</button>
                            </li>
                          </ul>
                        </div>
                      </div>
                    </div>

                    <div class="pf-v5-c-toolbar__item pf-m-pagination">
                      <div class="pf-v5-c-pagination pf-m-compact">
                        <div class="pf-v5-c-options-menu">
                          <button
                            class="pf-v5-c-options-menu__toggle pf-m-text pf-m-plain"
                            type="button"
                            id="primary-detail-content-body-padding-example-drawer-toolbar-top-pagination-toggle"
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
                            aria-labelledby="primary-detail-content-body-padding-example-drawer-toolbar-top-pagination-toggle"
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
                                <div
                                  class="pf-v5-c-options-menu__menu-item-icon"
                                >
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
                          aria-label="Toolbar top pagination"
                        >
                          <div
                            class="pf-v5-c-pagination__nav-control pf-m-prev"
                          >
                            <button
                              class="pf-v5-c-button pf-m-plain"
                              type="button"
                              disabled
                              aria-label="Go to previous page"
                            >
                              <i class="fas fa-angle-left" aria-hidden="true"></i>
                            </button>
                          </div>
                          <div
                            class="pf-v5-c-pagination__nav-control pf-m-next"
                          >
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
                    </div>
                  </div>

                  <div
                    class="pf-v5-c-toolbar__expandable-content pf-m-hidden"
                    id="primary-detail-content-body-padding-example-drawer-toolbar-expandable-content"
                    hidden
                  ></div>
                </div>
              </div>
              <ul
                class="pf-v5-c-data-list"
                role="list"
                aria-label="Simple data list example"
                id="primary-detail-content-body-padding-example-data-list"
              >
                <li
                  class="pf-v5-c-data-list__item"
                  aria-labelledby="primary-detail-content-body-padding-example-data-list-item-1"
                >
                  <div class="pf-v5-c-data-list__item-row">
                    <div class="pf-v5-c-data-list__item-content">
                      <div class="pf-v5-c-data-list__cell pf-m-align-left">
                        <div
                          class="pf-v5-l-flex pf-m-column pf-m-space-items-md"
                        >
                          <div
                            class="pf-v5-l-flex pf-m-column pf-m-space-items-none"
                          >
                            <div class="pf-v5-l-flex__item">
                              <p
                                id="primary-detail-content-body-padding-example-data-list-item-1"
                              >patternfly</p>
                            </div>
                            <div class="pf-v5-l-flex__item">
                              <small>
                                Working repo for PatternFly 4
                                <a href>https://pf4.patternfly.org/</a>
                              </small>
                            </div>
                          </div>
                          <div class="pf-v5-l-flex pf-m-wrap">
                            <div class="pf-v5-l-flex pf-m-space-items-xs">
                              <div class="pf-v5-l-flex__item">
                                <i
                                  class="fas fa-code-branch"
                                  aria-hidden="true"
                                ></i>
                              </div>
                              <div class="pf-v5-l-flex__item">
                                <span>10</span>
                              </div>
                            </div>
                            <div class="pf-v5-l-flex pf-m-space-items-xs">
                              <div class="pf-v5-l-flex__item">
                                <i class="fas fa-code" aria-hidden="true"></i>
                              </div>
                              <div class="pf-v5-l-flex__item">
                                <span>4</span>
                              </div>
                            </div>
                            <div class="pf-v5-l-flex pf-m-space-items-xs">
                              <div class="pf-v5-l-flex__item">
                                <i class="fas fa-cube" aria-hidden="true"></i>
                              </div>
                              <div class="pf-v5-l-flex__item">
                                <span>5</span>
                              </div>
                            </div>
                            <div class="pf-v5-l-flex__item">Updated 2 days ago</div>
                          </div>
                        </div>
                      </div>
                      <div
                        class="pf-v5-c-data-list__cell pf-m-align-right pf-m-no-fill"
                      >
                        <button
                          class="pf-v5-c-button pf-m-secondary"
                          type="button"
                        >Action</button>
                        <button
                          class="pf-v5-c-button pf-m-link"
                          type="button"
                        >Link</button>
                      </div>
                    </div>
                  </div>
                </li>

                <li
                  class="pf-v5-c-data-list__item"
                  aria-labelledby="primary-detail-content-body-padding-example-data-list-item-2"
                >
                  <div class="pf-v5-c-data-list__item-row">
                    <div class="pf-v5-c-data-list__item-content">
                      <div class="pf-v5-c-data-list__cell pf-m-align-left">
                        <div
                          class="pf-v5-l-flex pf-m-column pf-m-space-items-md"
                        >
                          <div
                            class="pf-v5-l-flex pf-m-column pf-m-space-items-none"
                          >
                            <div class="pf-v5-l-flex__item">
                              <p
                                id="primary-detail-content-body-padding-example-data-list-item-2"
                              >patternfly-elements</p>
                            </div>
                            <div class="pf-v5-l-flex__item">
                              <small>PatternFly elements</small>
                            </div>
                          </div>
                          <div class="pf-v5-l-flex pf-m-wrap">
                            <div class="pf-v5-l-flex pf-m-space-items-xs">
                              <div class="pf-v5-l-flex__item">
                                <i
                                  class="fas fa-code-branch"
                                  aria-hidden="true"
                                ></i>
                              </div>
                              <div class="pf-v5-l-flex__item">
                                <span>5</span>
                              </div>
                            </div>
                            <div class="pf-v5-l-flex pf-m-space-items-xs">
                              <div class="pf-v5-l-flex__item">
                                <i class="fas fa-code" aria-hidden="true"></i>
                              </div>
                              <div class="pf-v5-l-flex__item">
                                <span>9</span>
                              </div>
                            </div>
                            <div class="pf-v5-l-flex pf-m-space-items-xs">
                              <div class="pf-v5-l-flex__item">
                                <i class="fas fa-cube" aria-hidden="true"></i>
                              </div>
                              <div class="pf-v5-l-flex__item">
                                <span>2</span>
                              </div>
                            </div>
                            <div class="pf-v5-l-flex pf-m-space-items-xs">
                              <div class="pf-v5-l-flex__item">
                                <i
                                  class="fas fa-check-circle"
                                  aria-hidden="true"
                                ></i>
                              </div>
                              <div class="pf-v5-l-flex__item">
                                <span>11</span>
                              </div>
                            </div>
                            <div class="pf-v5-l-flex pf-m-space-items-xs">
                              <div class="pf-v5-l-flex__item">
                                <i
                                  class="fas fa-exclamation-triangle"
                                  aria-hidden="true"
                                ></i>
                              </div>
                              <div class="pf-v5-l-flex__item">
                                <span>4</span>
                              </div>
                            </div>
                            <div class="pf-v5-l-flex pf-m-space-items-xs">
                              <div class="pf-v5-l-flex__item">
                                <i
                                  class="fas fa-times-circle"
                                  aria-hidden="true"
                                ></i>
                              </div>
                              <div class="pf-v5-l-flex__item">
                                <span>1</span>
                              </div>
                            </div>
                            <div class="pf-v5-l-flex__item">Updated 2 days ago</div>
                          </div>
                        </div>
                      </div>
                      <div
                        class="pf-v5-c-data-list__cell pf-m-align-right pf-m-no-fill"
                      >
                        <button
                          class="pf-v5-c-button pf-m-secondary"
                          type="button"
                        >Action</button>
                        <button
                          class="pf-v5-c-button pf-m-link"
                          type="button"
                        >Link</button>
                      </div>
                    </div>
                  </div>
                </li>

                <li
                  class="pf-v5-c-data-list__item"
                  aria-labelledby="primary-detail-content-body-padding-example-data-list-item-3"
                >
                  <div class="pf-v5-c-data-list__item-row">
                    <div class="pf-v5-c-data-list__item-content">
                      <div class="pf-v5-c-data-list__cell pf-m-align-left">
                        <p
                          id="primary-detail-content-body-padding-example-data-list-item-3"
                        >patternfly-unified-design-kit</p>
                      </div>
                      <div
                        class="pf-v5-c-data-list__cell pf-m-align-right pf-m-no-fill"
                      >
                        <button
                          class="pf-v5-c-button pf-m-secondary"
                          type="button"
                        >Action</button>
                        <button
                          class="pf-v5-c-button pf-m-link"
                          type="button"
                        >Link</button>
                      </div>
                    </div>
                  </div>
                </li>

                <li
                  class="pf-v5-c-data-list__item"
                  aria-labelledby="primary-detail-content-body-padding-example-data-list-item-4"
                >
                  <div class="pf-v5-c-data-list__item-row">
                    <div class="pf-v5-c-data-list__item-content">
                      <div class="pf-v5-c-data-list__cell pf-m-align-left">
                        <div
                          class="pf-v5-l-flex pf-m-column pf-m-space-items-md"
                        >
                          <div
                            class="pf-v5-l-flex pf-m-column pf-m-space-items-none"
                          >
                            <div class="pf-v5-l-flex__item">
                              <p
                                id="primary-detail-content-body-padding-example-data-list-item-4"
                              >patternfly</p>
                            </div>
                            <div class="pf-v5-l-flex__item">
                              <small>
                                Working repo for PatternFly 4
                                <a href>https://pf4.patternfly.org/</a>
                              </small>
                            </div>
                          </div>
                          <div class="pf-v5-l-flex pf-m-wrap">
                            <div class="pf-v5-l-flex pf-m-space-items-xs">
                              <div class="pf-v5-l-flex__item">
                                <i
                                  class="fas fa-code-branch"
                                  aria-hidden="true"
                                ></i>
                              </div>
                              <div class="pf-v5-l-flex__item">
                                <span>10</span>
                              </div>
                            </div>
                            <div class="pf-v5-l-flex pf-m-space-items-xs">
                              <div class="pf-v5-l-flex__item">
                                <i class="fas fa-code" aria-hidden="true"></i>
                              </div>
                              <div class="pf-v5-l-flex__item">
                                <span>4</span>
                              </div>
                            </div>
                            <div class="pf-v5-l-flex pf-m-space-items-xs">
                              <div class="pf-v5-l-flex__item">
                                <i class="fas fa-cube" aria-hidden="true"></i>
                              </div>
                              <div class="pf-v5-l-flex__item">
                                <span>5</span>
                              </div>
                            </div>
                            <div class="pf-v5-l-flex__item">Updated 2 days ago</div>
                          </div>
                        </div>
                      </div>
                      <div
                        class="pf-v5-c-data-list__cell pf-m-align-right pf-m-no-fill"
                      >
                        <button
                          class="pf-v5-c-button pf-m-secondary"
                          type="button"
                        >Action</button>
                        <button
                          class="pf-v5-c-button pf-m-link"
                          type="button"
                        >Link</button>
                      </div>
                    </div>
                  </div>
                </li>

                <li
                  class="pf-v5-c-data-list__item"
                  aria-labelledby="primary-detail-content-body-padding-example-data-list-item-5"
                >
                  <div class="pf-v5-c-data-list__item-row">
                    <div class="pf-v5-c-data-list__item-content">
                      <div class="pf-v5-c-data-list__cell pf-m-align-left">
                        <div
                          class="pf-v5-l-flex pf-m-column pf-m-space-items-md"
                        >
                          <div
                            class="pf-v5-l-flex pf-m-column pf-m-space-items-none"
                          >
                            <div class="pf-v5-l-flex__item">
                              <p
                                id="primary-detail-content-body-padding-example-data-list-item-5"
                              >patternfly-elements</p>
                            </div>
                            <div class="pf-v5-l-flex__item">
                              <small>PatternFly elements</small>
                            </div>
                          </div>
                          <div class="pf-v5-l-flex pf-m-wrap">
                            <div class="pf-v5-l-flex pf-m-space-items-xs">
                              <div class="pf-v5-l-flex__item">
                                <i
                                  class="fas fa-code-branch"
                                  aria-hidden="true"
                                ></i>
                              </div>
                              <div class="pf-v5-l-flex__item">
                                <span>5</span>
                              </div>
                            </div>
                            <div class="pf-v5-l-flex pf-m-space-items-xs">
                              <div class="pf-v5-l-flex__item">
                                <i class="fas fa-code" aria-hidden="true"></i>
                              </div>
                              <div class="pf-v5-l-flex__item">
                                <span>9</span>
                              </div>
                            </div>
                            <div class="pf-v5-l-flex pf-m-space-items-xs">
                              <div class="pf-v5-l-flex__item">
                                <i class="fas fa-cube" aria-hidden="true"></i>
                              </div>
                              <div class="pf-v5-l-flex__item">
                                <span>2</span>
                              </div>
                            </div>
                            <div class="pf-v5-l-flex pf-m-space-items-xs">
                              <div class="pf-v5-l-flex__item">
                                <i
                                  class="fas fa-check-circle"
                                  aria-hidden="true"
                                ></i>
                              </div>
                              <div class="pf-v5-l-flex__item">
                                <span>11</span>
                              </div>
                            </div>
                            <div class="pf-v5-l-flex pf-m-space-items-xs">
                              <div class="pf-v5-l-flex__item">
                                <i
                                  class="fas fa-exclamation-triangle"
                                  aria-hidden="true"
                                ></i>
                              </div>
                              <div class="pf-v5-l-flex__item">
                                <span>4</span>
                              </div>
                            </div>
                            <div class="pf-v5-l-flex pf-m-space-items-xs">
                              <div class="pf-v5-l-flex__item">
                                <i
                                  class="fas fa-times-circle"
                                  aria-hidden="true"
                                ></i>
                              </div>
                              <div class="pf-v5-l-flex__item">
                                <span>1</span>
                              </div>
                            </div>
                            <div class="pf-v5-l-flex__item">Updated 2 days ago</div>
                          </div>
                        </div>
                      </div>
                      <div
                        class="pf-v5-c-data-list__cell pf-m-align-right pf-m-no-fill"
                      >
                        <button
                          class="pf-v5-c-button pf-m-secondary"
                          type="button"
                        >Action</button>
                        <button
                          class="pf-v5-c-button pf-m-link"
                          type="button"
                        >Link</button>
                      </div>
                    </div>
                  </div>
                </li>
              </ul>
            </div>
          </div>

          <!-- Panel -->
          <div class="pf-v5-c-drawer__panel">
            <!-- Panel header -->
            <div class="pf-v5-c-drawer__body">
              <div class="pf-v5-l-flex pf-m-column">
                <div class="pf-v5-l-flex__item">
                  <div class="pf-v5-c-drawer__head">
                    <div class="pf-v5-c-drawer__actions">
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
                    <h2
                      class="pf-v5-c-title pf-m-lg"
                      id="primary-detail-content-body-padding-example-drawer-drawer-label"
                    >Patternfly-elements</h2>
                  </div>
                </div>
                <div class="pf-v5-l-flex__item">PatternFly elements</div>
              </div>
            </div>

            <!-- Tab content -->
            <div class="pf-v5-c-drawer__body">
              <div class="pf-v5-l-flex pf-m-column pf-m-space-items-lg">
                <div class="pf-v5-l-flex__item">
                  <p>The content of the drawer really is up to you. It could have form fields, definition lists, text lists, labels, charts, progress bars, etc. Spacing recommendation is 24px margins. You can put tabs in here, and can also make the drawer scrollable.</p>
                </div>
                <div class="pf-v5-l-flex__item">
                  <div
                    class="pf-v5-c-progress pf-m-sm"
                    id="primary-detail-content-body-padding-example-drawer-progress-example1"
                  >
                    <div
                      class="pf-v5-c-progress__description"
                      id="primary-detail-content-body-padding-example-drawer-progress-example1-description"
                    >Capacity</div>
                    <div class="pf-v5-c-progress__status" aria-hidden="true">
                      <span class="pf-v5-c-progress__measure">33%</span>
                    </div>
                    <div
                      class="pf-v5-c-progress__bar"
                      role="progressbar"
                      aria-valuemin="0"
                      aria-valuemax="100"
                      aria-valuenow="33"
                      aria-labelledby="primary-detail-content-body-padding-example-drawer-progress-example1-description"
                      aria-label="Progress 1"
                    >
                      <div
                        class="pf-v5-c-progress__indicator"
                        style="width:33%;"
                      ></div>
                    </div>
                  </div>
                </div>
                <div class="pf-v5-l-flex__item">
                  <div
                    class="pf-v5-c-progress pf-m-sm"
                    id="primary-detail-content-body-padding-example-drawer-progress-example2"
                  >
                    <div
                      class="pf-v5-c-progress__description"
                      id="primary-detail-content-body-padding-example-drawer-progress-example2-description"
                    >Modules</div>
                    <div class="pf-v5-c-progress__status" aria-hidden="true">
                      <span class="pf-v5-c-progress__measure">66%</span>
                    </div>
                    <div
                      class="pf-v5-c-progress__bar"
                      role="progressbar"
                      aria-valuemin="0"
                      aria-valuemax="100"
                      aria-valuenow="66"
                      aria-labelledby="primary-detail-content-body-padding-example-drawer-progress-example2-description"
                      aria-label="Progress 2"
                    >
                      <div
                        class="pf-v5-c-progress__indicator"
                        style="width:66%;"
                      ></div>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>
  </main>
</div>

```

### Primary-detail card view expanded

```html isFullscreen
<div class="pf-v5-c-page" id="primary-detail-card-view-expanded-example">
  <div class="pf-v5-c-skip-to-content">
    <a
      class="pf-v5-c-button pf-m-primary"
      href="#main-content-primary-detail-card-view-expanded-example"
    >Skip to content</a>
  </div>
  <header
    class="pf-v5-c-masthead pf-m-display-stack pf-m-display-inline-on-lg"
    id="primary-detail-card-view-expanded-example-masthead"
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
        id="primary-detail-card-view-expanded-example-masthead-toolbar"
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
        id="primary-detail-card-view-expanded-example-primary-nav"
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
    id="main-content-primary-detail-card-view-expanded-example"
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
    <section class="pf-v5-c-page__main-section pf-m-no-padding">
      <!-- Drawer -->
      <div class="pf-v5-c-drawer pf-m-expanded pf-m-inline-on-2xl">
        <div class="pf-v5-c-drawer__section">
          <div
            class="pf-v5-c-toolbar pf-m-page-insets"
            id="primary-detail-card-view-expanded-example-drawer-toolbar"
          >
            <div class="pf-v5-c-toolbar__content">
              <div class="pf-v5-c-toolbar__content-section pf-m-nowrap">
                <div
                  class="pf-v5-c-toolbar__group pf-m-toggle-group pf-m-show-on-xl"
                >
                  <div class="pf-v5-c-toolbar__toggle">
                    <button
                      class="pf-v5-c-button pf-m-plain"
                      type="button"
                      aria-label="Show filters"
                      aria-expanded="false"
                      aria-controls="primary-detail-card-view-expanded-example-drawer-toolbar-expandable-content"
                    >
                      <i class="fas fa-filter" aria-hidden="true"></i>
                    </button>
                  </div>
                  <div class="pf-v5-c-toolbar__item pf-m-bulk-select">
                    <div class="pf-v5-c-dropdown">
                      <div class="pf-v5-c-dropdown__toggle pf-m-split-button">
                        <label
                          class="pf-v5-c-dropdown__toggle-check"
                          for="primary-detail-card-view-expanded-example-drawer-toolbar-bulk-select-toggle-check"
                        >
                          <div class="pf-v5-c-check pf-m-standalone">
                            <input
                              class="pf-v5-c-check__input"
                              type="checkbox"
                              id="primary-detail-card-view-expanded-example-drawer-toolbar-bulk-select-toggle-check"
                              aria-label="Select all"
                            />
                          </div>
                        </label>

                        <button
                          class="pf-v5-c-dropdown__toggle-button"
                          type="button"
                          aria-expanded="false"
                          id="primary-detail-card-view-expanded-example-drawer-toolbar-bulk-select-toggle-button"
                          aria-label="Dropdown toggle"
                        >
                          <i class="fas fa-caret-down" aria-hidden="true"></i>
                        </button>
                      </div>
                      <ul class="pf-v5-c-dropdown__menu" hidden role="menu">
                        <li role="none">
                          <button
                            class="pf-v5-c-dropdown__menu-item"
                            role="menuitem"
                            type="button"
                          >Select all</button>
                        </li>
                        <li role="none">
                          <button
                            class="pf-v5-c-dropdown__menu-item"
                            role="menuitem"
                            type="button"
                          >Select none</button>
                        </li>
                        <li role="none">
                          <button
                            class="pf-v5-c-dropdown__menu-item"
                            role="menuitem"
                            type="button"
                          >Other action</button>
                        </li>
                      </ul>
                    </div>
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
                            id="primary-detail-card-view-expanded-example-drawer-toolbar-select-name-label"
                            hidden
                          >Choose one</span>

                          <button
                            class="pf-v5-c-select__toggle"
                            type="button"
                            id="primary-detail-card-view-expanded-example-drawer-toolbar-select-name-toggle"
                            aria-haspopup="true"
                            aria-expanded="false"
                            aria-labelledby="primary-detail-card-view-expanded-example-drawer-toolbar-select-name-label primary-detail-card-view-expanded-example-drawer-toolbar-select-name-toggle"
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
                            aria-labelledby="primary-detail-card-view-expanded-example-drawer-toolbar-select-name-label"
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
                    class="pf-v5-c-button pf-m-plain"
                    type="button"
                    aria-label="Sort"
                  >
                    <i
                      class="fas fa-sort-amount-down pf-v5-m-mirror-inline-rtl"
                      aria-hidden="true"
                    ></i>
                  </button>
                </div>

                <div
                  class="pf-v5-c-overflow-menu"
                  id="primary-detail-card-view-expanded-example-drawer-toolbar-overflow-menu"
                >
                  <div
                    class="pf-v5-c-overflow-menu__content pf-v5-u-display-none pf-v5-u-display-flex-on-lg"
                  >
                    <div class="pf-v5-c-overflow-menu__group pf-m-button-group">
                      <div class="pf-v5-c-overflow-menu__item">
                        <button
                          class="pf-v5-c-button pf-m-primary"
                          type="button"
                        >Create instance</button>
                      </div>

                      <div class="pf-v5-c-overflow-menu__item">
                        <button
                          class="pf-v5-c-button pf-m-secondary"
                          type="button"
                        >Action</button>
                      </div>
                    </div>
                  </div>
                  <div class="pf-v5-c-overflow-menu__control">
                    <div class="pf-v5-c-dropdown">
                      <button
                        class="pf-v5-c-button pf-v5-c-dropdown__toggle pf-m-plain"
                        type="button"
                        id="primary-detail-card-view-expanded-example-drawer-toolbar-overflow-menu-dropdown-toggle"
                        aria-label="Dropdown with additional options"
                        aria-expanded="false"
                      >
                        <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
                      </button>
                      <ul
                        class="pf-v5-c-dropdown__menu"
                        role="menu"
                        aria-labelledby="primary-detail-card-view-expanded-example-drawer-toolbar-overflow-menu-dropdown-toggle"
                        hidden
                      >
                        <li role="none">
                          <button
                            role="menuitem"
                            class="pf-v5-c-dropdown__menu-item"
                          >Action 7</button>
                        </li>
                      </ul>
                    </div>
                  </div>
                </div>

                <div
                  class="pf-v5-c-toolbar__group pf-m-icon-button-group pf-m-align-right"
                >
                  <div class="pf-v5-c-toolbar__item">
                    <button
                      class="pf-v5-c-button pf-m-plain pf-m-active"
                      type="button"
                      aria-label="Grid view"
                    >
                      <i class="fas fa-th" aria-hidden="true"></i>
                    </button>
                  </div>
                  <div class="pf-v5-c-toolbar__item">
                    <button
                      class="pf-v5-c-button pf-m-plain"
                      type="button"
                      aria-label="List view"
                    >
                      <i class="fas fa-list-ul" aria-hidden="true"></i>
                    </button>
                  </div>
                </div>

                <div class="pf-v5-c-toolbar__item pf-m-pagination">
                  <div class="pf-v5-c-pagination pf-m-compact">
                    <div class="pf-v5-c-options-menu">
                      <button
                        class="pf-v5-c-options-menu__toggle pf-m-text pf-m-plain"
                        type="button"
                        id="primary-detail-card-view-expanded-example-drawer-toolbar-top-pagination-toggle"
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
                        aria-labelledby="primary-detail-card-view-expanded-example-drawer-toolbar-top-pagination-toggle"
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
                      aria-label="Toolbar top pagination"
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
                </div>
              </div>

              <div
                class="pf-v5-c-toolbar__expandable-content pf-m-hidden"
                id="primary-detail-card-view-expanded-example-drawer-toolbar-expandable-content"
                hidden
              ></div>
            </div>
          </div>
          <div class="pf-v5-c-divider" role="separator"></div>
        </div>

        <div class="pf-v5-c-drawer__main">
          <!-- Content -->
          <div class="pf-v5-c-drawer__content pf-m-no-background">
            <div class="pf-v5-c-drawer__body pf-m-padding">
              <div class="pf-v5-l-gallery pf-m-gutter">
                <div
                  class="pf-v5-c-card pf-m-selectable-raised pf-m-selected-raised"
                  id="primary-detail-card-view-expanded-example-drawer-card-1"
                >
                  <div class="pf-v5-c-card__header">
                    <img
                      src="/assets/images/pf-logo-small.svg"
                      alt="PatternFly logo"
                    />
                    <div class="pf-v5-c-card__actions">
                      <div class="pf-v5-c-dropdown">
                        <button
                          class="pf-v5-c-dropdown__toggle pf-m-plain"
                          id="primary-detail-card-view-expanded-example-drawer-card-1-dropdown-kebab-button"
                          aria-expanded="false"
                          type="button"
                          aria-label="Actions"
                        >
                          <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
                        </button>
                        <ul
                          class="pf-v5-c-dropdown__menu pf-m-align-right"
                          aria-labelledby="primary-detail-card-view-expanded-example-drawer-card-1-dropdown-kebab-button"
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
                      <div class="pf-v5-c-check pf-m-standalone">
                        <input
                          class="pf-v5-c-check__input"
                          type="checkbox"
                          id="primary-detail-card-view-expanded-example-drawer-card-1-check"
                          name="primary-detail-card-view-expanded-example-drawer-card-1-check"
                          aria-labelledby="primary-detail-card-view-expanded-example-drawer-card-1-check-label"
                        />
                      </div>
                    </div>
                  </div>
                  <div class="pf-v5-c-card__title">
                    <h2
                      class="pf-v5-c-card__title-text"
                      id="primary-detail-card-view-expanded-example-drawer-card-1-check-label"
                    >Patternfly</h2>
                    <div class="pf-v5-c-content">
                      <small>Provided by Red Hat</small>
                    </div>
                  </div>
                  <div
                    class="pf-v5-c-card__body"
                  >PatternFly is a community project that promotes design commonality and improves user experience.</div>
                </div>
                <div
                  class="pf-v5-c-card pf-m-selectable-raised"
                  id="primary-detail-card-view-expanded-example-drawer-card-2"
                >
                  <div class="pf-v5-c-card__header">
                    <img
                      src="/assets/images/activemq-core_200x150.png"
                      width="60px"
                      alt="Logo"
                    />
                    <div class="pf-v5-c-card__actions">
                      <div class="pf-v5-c-dropdown">
                        <button
                          class="pf-v5-c-dropdown__toggle pf-m-plain"
                          id="primary-detail-card-view-expanded-example-drawer-card-2-dropdown-kebab-button"
                          aria-expanded="false"
                          type="button"
                          aria-label="Actions"
                        >
                          <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
                        </button>
                        <ul
                          class="pf-v5-c-dropdown__menu pf-m-align-right"
                          aria-labelledby="primary-detail-card-view-expanded-example-drawer-card-2-dropdown-kebab-button"
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
                      <div class="pf-v5-c-check pf-m-standalone">
                        <input
                          class="pf-v5-c-check__input"
                          type="checkbox"
                          id="primary-detail-card-view-expanded-example-drawer-card-2-check"
                          name="primary-detail-card-view-expanded-example-drawer-card-2-check"
                          aria-labelledby="primary-detail-card-view-expanded-example-drawer-card-2-check-label"
                        />
                      </div>
                    </div>
                  </div>
                  <div class="pf-v5-c-card__title">
                    <h2
                      class="pf-v5-c-card__title-text"
                      id="primary-detail-card-view-expanded-example-drawer-card-2-check-label"
                    >ActiveQ</h2>
                    <div class="pf-v5-c-content">
                      <small>Provided by Red Hat</small>
                    </div>
                  </div>
                  <div
                    class="pf-v5-c-card__body"
                  >The ActiveMQ component allows messages to be sent to a JMS Queue or Topic; or messages to be consumed from a JMS Queue or Topic using Apache ActiveMQ.</div>
                </div>
                <div
                  class="pf-v5-c-card pf-m-selectable-raised"
                  id="primary-detail-card-view-expanded-example-drawer-card-3"
                >
                  <div class="pf-v5-c-card__header">
                    <img
                      src="/assets/images/camel-spark_200x150.png"
                      width="60px"
                      alt="Logo"
                    />
                    <div class="pf-v5-c-card__actions">
                      <div class="pf-v5-c-dropdown">
                        <button
                          class="pf-v5-c-dropdown__toggle pf-m-plain"
                          id="primary-detail-card-view-expanded-example-drawer-card-3-dropdown-kebab-button"
                          aria-expanded="false"
                          type="button"
                          aria-label="Actions"
                        >
                          <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
                        </button>
                        <ul
                          class="pf-v5-c-dropdown__menu pf-m-align-right"
                          aria-labelledby="primary-detail-card-view-expanded-example-drawer-card-3-dropdown-kebab-button"
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
                      <div class="pf-v5-c-check pf-m-standalone">
                        <input
                          class="pf-v5-c-check__input"
                          type="checkbox"
                          id="primary-detail-card-view-expanded-example-drawer-card-3-check"
                          name="primary-detail-card-view-expanded-example-drawer-card-3-check"
                          aria-labelledby="primary-detail-card-view-expanded-example-drawer-card-3-check-label"
                        />
                      </div>
                    </div>
                  </div>
                  <div class="pf-v5-c-card__title">
                    <h2
                      class="pf-v5-c-card__title-text"
                      id="primary-detail-card-view-expanded-example-drawer-card-3-check-label"
                    >Apache Spark</h2>
                    <div class="pf-v5-c-content">
                      <small>Provided by Red Hat</small>
                    </div>
                  </div>
                  <div
                    class="pf-v5-c-card__body"
                  >This documentation page covers the Apache Spark component for the Apache Camel.</div>
                </div>
                <div
                  class="pf-v5-c-card pf-m-selectable-raised"
                  id="primary-detail-card-view-expanded-example-drawer-card-4"
                >
                  <div class="pf-v5-c-card__header">
                    <img
                      src="/assets/images/camel-avro_200x150.png"
                      width="60px"
                      alt="Logo"
                    />
                    <div class="pf-v5-c-card__actions">
                      <div class="pf-v5-c-dropdown">
                        <button
                          class="pf-v5-c-dropdown__toggle pf-m-plain"
                          id="primary-detail-card-view-expanded-example-drawer-card-4-dropdown-kebab-button"
                          aria-expanded="false"
                          type="button"
                          aria-label="Actions"
                        >
                          <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
                        </button>
                        <ul
                          class="pf-v5-c-dropdown__menu pf-m-align-right"
                          aria-labelledby="primary-detail-card-view-expanded-example-drawer-card-4-dropdown-kebab-button"
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
                      <div class="pf-v5-c-check pf-m-standalone">
                        <input
                          class="pf-v5-c-check__input"
                          type="checkbox"
                          id="primary-detail-card-view-expanded-example-drawer-card-4-check"
                          name="primary-detail-card-view-expanded-example-drawer-card-4-check"
                          aria-labelledby="primary-detail-card-view-expanded-example-drawer-card-4-check-label"
                        />
                      </div>
                    </div>
                  </div>
                  <div class="pf-v5-c-card__title">
                    <h2
                      class="pf-v5-c-card__title-text"
                      id="primary-detail-card-view-expanded-example-drawer-card-4-check-label"
                    >Avro</h2>
                    <div class="pf-v5-c-content">
                      <small>Provided by Red Hat</small>
                    </div>
                  </div>
                  <div
                    class="pf-v5-c-card__body"
                  >This component provides a dataformat for avro, which allows serialization and deserialization of messages using Apache Avro’s binary dataformat. Moreover, it provides support for Apache Avro’s rpc, by providing producers and consumers endpoint for using avro over netty or http.</div>
                </div>
                <div
                  class="pf-v5-c-card pf-m-selectable-raised"
                  id="primary-detail-card-view-expanded-example-drawer-card-5"
                >
                  <div class="pf-v5-c-card__header">
                    <img
                      src="/assets/images/FuseConnector_Icons_AzureServices.png"
                      width="60px"
                      alt="Logo"
                    />
                    <div class="pf-v5-c-card__actions">
                      <div class="pf-v5-c-dropdown">
                        <button
                          class="pf-v5-c-dropdown__toggle pf-m-plain"
                          id="primary-detail-card-view-expanded-example-drawer-card-5-dropdown-kebab-button"
                          aria-expanded="false"
                          type="button"
                          aria-label="Actions"
                        >
                          <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
                        </button>
                        <ul
                          class="pf-v5-c-dropdown__menu pf-m-align-right"
                          aria-labelledby="primary-detail-card-view-expanded-example-drawer-card-5-dropdown-kebab-button"
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
                      <div class="pf-v5-c-check pf-m-standalone">
                        <input
                          class="pf-v5-c-check__input"
                          type="checkbox"
                          id="primary-detail-card-view-expanded-example-drawer-card-5-check"
                          name="primary-detail-card-view-expanded-example-drawer-card-5-check"
                          aria-labelledby="primary-detail-card-view-expanded-example-drawer-card-5-check-label"
                        />
                      </div>
                    </div>
                  </div>
                  <div class="pf-v5-c-card__title">
                    <h2
                      class="pf-v5-c-card__title-text"
                      id="primary-detail-card-view-expanded-example-drawer-card-5-check-label"
                    >Azure Services</h2>
                    <div class="pf-v5-c-content">
                      <small>Provided by Red Hat</small>
                    </div>
                  </div>
                  <div
                    class="pf-v5-c-card__body"
                  >The Camel Components for Windows Azure Services provide connectivity to Azure services from Camel.</div>
                </div>
                <div
                  class="pf-v5-c-card pf-m-selectable-raised"
                  id="primary-detail-card-view-expanded-example-drawer-card-6"
                >
                  <div class="pf-v5-c-card__header">
                    <img
                      src="/assets/images/camel-saxon_200x150.png"
                      width="60px"
                      alt="Logo"
                    />
                    <div class="pf-v5-c-card__actions">
                      <div class="pf-v5-c-dropdown">
                        <button
                          class="pf-v5-c-dropdown__toggle pf-m-plain"
                          id="primary-detail-card-view-expanded-example-drawer-card-6-dropdown-kebab-button"
                          aria-expanded="false"
                          type="button"
                          aria-label="Actions"
                        >
                          <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
                        </button>
                        <ul
                          class="pf-v5-c-dropdown__menu pf-m-align-right"
                          aria-labelledby="primary-detail-card-view-expanded-example-drawer-card-6-dropdown-kebab-button"
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
                      <div class="pf-v5-c-check pf-m-standalone">
                        <input
                          class="pf-v5-c-check__input"
                          type="checkbox"
                          id="primary-detail-card-view-expanded-example-drawer-card-6-check"
                          name="primary-detail-card-view-expanded-example-drawer-card-6-check"
                          aria-labelledby="primary-detail-card-view-expanded-example-drawer-card-6-check-label"
                        />
                      </div>
                    </div>
                  </div>
                  <div class="pf-v5-c-card__title">
                    <h2
                      class="pf-v5-c-card__title-text"
                      id="primary-detail-card-view-expanded-example-drawer-card-6-check-label"
                    >Crypto</h2>
                    <div class="pf-v5-c-content">
                      <small>Provided by Red Hat</small>
                    </div>
                  </div>
                  <div
                    class="pf-v5-c-card__body"
                  >For providing flexible endpoints to sign and verify exchanges using the Signature Service of the Java Cryptographic Extension.</div>
                </div>
                <div
                  class="pf-v5-c-card pf-m-selectable-raised"
                  id="primary-detail-card-view-expanded-example-drawer-card-7"
                >
                  <div class="pf-v5-c-card__header">
                    <img
                      src="/assets/images/pf-logo-small.svg"
                      alt="PatternFly logo"
                    />
                    <div class="pf-v5-c-card__actions">
                      <div class="pf-v5-c-dropdown">
                        <button
                          class="pf-v5-c-dropdown__toggle pf-m-plain"
                          id="primary-detail-card-view-expanded-example-drawer-card-7-dropdown-kebab-button"
                          aria-expanded="false"
                          type="button"
                          aria-label="Actions"
                        >
                          <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
                        </button>
                        <ul
                          class="pf-v5-c-dropdown__menu pf-m-align-right"
                          aria-labelledby="primary-detail-card-view-expanded-example-drawer-card-7-dropdown-kebab-button"
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
                      <div class="pf-v5-c-check pf-m-standalone">
                        <input
                          class="pf-v5-c-check__input"
                          type="checkbox"
                          id="primary-detail-card-view-expanded-example-drawer-card-7-check"
                          name="primary-detail-card-view-expanded-example-drawer-card-7-check"
                          aria-labelledby="primary-detail-card-view-expanded-example-drawer-card-7-check-label"
                        />
                      </div>
                    </div>
                  </div>
                  <div class="pf-v5-c-card__title">
                    <h2
                      class="pf-v5-c-card__title-text"
                      id="primary-detail-card-view-expanded-example-drawer-card-7-check-label"
                    >Patternfly</h2>
                    <div class="pf-v5-c-content">
                      <small>Provided by Red Hat</small>
                    </div>
                  </div>
                  <div
                    class="pf-v5-c-card__body"
                  >PatternFly is a community project that promotes design commonality and improves user experience.</div>
                </div>
                <div
                  class="pf-v5-c-card pf-m-selectable-raised"
                  id="primary-detail-card-view-expanded-example-drawer-card-8"
                >
                  <div class="pf-v5-c-card__header">
                    <img
                      src="/assets/images/pf-logo-small.svg"
                      alt="PatternFly logo"
                    />
                    <div class="pf-v5-c-card__actions">
                      <div class="pf-v5-c-dropdown">
                        <button
                          class="pf-v5-c-dropdown__toggle pf-m-plain"
                          id="primary-detail-card-view-expanded-example-drawer-card-8-dropdown-kebab-button"
                          aria-expanded="false"
                          type="button"
                          aria-label="Actions"
                        >
                          <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
                        </button>
                        <ul
                          class="pf-v5-c-dropdown__menu pf-m-align-right"
                          aria-labelledby="primary-detail-card-view-expanded-example-drawer-card-8-dropdown-kebab-button"
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
                      <div class="pf-v5-c-check pf-m-standalone">
                        <input
                          class="pf-v5-c-check__input"
                          type="checkbox"
                          id="primary-detail-card-view-expanded-example-drawer-card-8-check"
                          name="primary-detail-card-view-expanded-example-drawer-card-8-check"
                          aria-labelledby="primary-detail-card-view-expanded-example-drawer-card-8-check-label"
                        />
                      </div>
                    </div>
                  </div>
                  <div class="pf-v5-c-card__title">
                    <h2
                      class="pf-v5-c-card__title-text"
                      id="primary-detail-card-view-expanded-example-drawer-card-8-check-label"
                    >Patternfly</h2>
                    <div class="pf-v5-c-content">
                      <small>Provided by Red Hat</small>
                    </div>
                  </div>
                  <div
                    class="pf-v5-c-card__body"
                  >PatternFly is a community project that promotes design commonality and improves user experience.</div>
                </div>
                <div
                  class="pf-v5-c-card pf-m-selectable-raised"
                  id="primary-detail-card-view-expanded-example-drawer-card-9"
                >
                  <div class="pf-v5-c-card__header">
                    <img
                      src="/assets/images/pf-logo-small.svg"
                      alt="PatternFly logo"
                    />
                    <div class="pf-v5-c-card__actions">
                      <div class="pf-v5-c-dropdown">
                        <button
                          class="pf-v5-c-dropdown__toggle pf-m-plain"
                          id="primary-detail-card-view-expanded-example-drawer-card-9-dropdown-kebab-button"
                          aria-expanded="false"
                          type="button"
                          aria-label="Actions"
                        >
                          <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
                        </button>
                        <ul
                          class="pf-v5-c-dropdown__menu pf-m-align-right"
                          aria-labelledby="primary-detail-card-view-expanded-example-drawer-card-9-dropdown-kebab-button"
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
                      <div class="pf-v5-c-check pf-m-standalone">
                        <input
                          class="pf-v5-c-check__input"
                          type="checkbox"
                          id="primary-detail-card-view-expanded-example-drawer-card-9-check"
                          name="primary-detail-card-view-expanded-example-drawer-card-9-check"
                          aria-labelledby="primary-detail-card-view-expanded-example-drawer-card-9-check-label"
                        />
                      </div>
                    </div>
                  </div>
                  <div class="pf-v5-c-card__title">
                    <h2
                      class="pf-v5-c-card__title-text"
                      id="primary-detail-card-view-expanded-example-drawer-card-9-check-label"
                    >Patternfly</h2>
                    <div class="pf-v5-c-content">
                      <small>Provided by Red Hat</small>
                    </div>
                  </div>
                  <div
                    class="pf-v5-c-card__body"
                  >PatternFly is a community project that promotes design commonality and improves user experience.</div>
                </div>
                <div
                  class="pf-v5-c-card pf-m-selectable-raised"
                  id="primary-detail-card-view-expanded-example-drawer-card-10"
                >
                  <div class="pf-v5-c-card__header">
                    <img
                      src="/assets/images/pf-logo-small.svg"
                      alt="PatternFly logo"
                    />
                    <div class="pf-v5-c-card__actions">
                      <div class="pf-v5-c-dropdown">
                        <button
                          class="pf-v5-c-dropdown__toggle pf-m-plain"
                          id="primary-detail-card-view-expanded-example-drawer-card-10-dropdown-kebab-button"
                          aria-expanded="false"
                          type="button"
                          aria-label="Actions"
                        >
                          <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
                        </button>
                        <ul
                          class="pf-v5-c-dropdown__menu pf-m-align-right"
                          aria-labelledby="primary-detail-card-view-expanded-example-drawer-card-10-dropdown-kebab-button"
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
                      <div class="pf-v5-c-check pf-m-standalone">
                        <input
                          class="pf-v5-c-check__input"
                          type="checkbox"
                          id="primary-detail-card-view-expanded-example-drawer-card-10-check"
                          name="primary-detail-card-view-expanded-example-drawer-card-10-check"
                          aria-labelledby="primary-detail-card-view-expanded-example-drawer-card-10-check-label"
                        />
                      </div>
                    </div>
                  </div>
                  <div class="pf-v5-c-card__title">
                    <h2
                      class="pf-v5-c-card__title-text"
                      id="primary-detail-card-view-expanded-example-drawer-card-10-check-label"
                    >Patternfly</h2>
                    <div class="pf-v5-c-content">
                      <small>Provided by Red Hat</small>
                    </div>
                  </div>
                  <div
                    class="pf-v5-c-card__body"
                  >PatternFly is a community project that promotes design commonality and improves user experience.</div>
                </div>
              </div>
            </div>
          </div>

          <!-- Panel -->
          <div class="pf-v5-c-drawer__panel">
            <!-- Panel header -->
            <div class="pf-v5-c-drawer__body">
              <div class="pf-v5-l-flex pf-m-column">
                <div class="pf-v5-l-flex__item">
                  <div class="pf-v5-c-drawer__head">
                    <div class="pf-v5-c-drawer__actions">
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
                    <h2
                      class="pf-v5-c-title pf-m-lg"
                      id="primary-detail-card-view-expanded-example-drawer-drawer-label"
                    >Patternfly</h2>
                  </div>
                </div>
                <div class="pf-v5-l-flex__item">PatternFly elements</div>
              </div>
            </div>

            <div class="pf-v5-c-drawer__body">
              <div class="pf-v5-l-flex pf-m-column pf-m-space-items-lg">
                <div class="pf-v5-l-flex__item">
                  <p>The content of the drawer really is up to you. It could have form fields, definition lists, text lists, labels, charts, progress bars, etc. Spacing recommendation is 24px margins. You can put tabs in here, and can also make the drawer scrollable.</p>
                </div>
                <div class="pf-v5-l-flex__item">
                  <div
                    class="pf-v5-c-progress"
                    id="primary-detail-card-view-expanded-example-drawer-progress-example1"
                  >
                    <div
                      class="pf-v5-c-progress__description"
                      id="primary-detail-card-view-expanded-example-drawer-progress-example1-description"
                    >Capacity</div>
                    <div class="pf-v5-c-progress__status" aria-hidden="true">
                      <span class="pf-v5-c-progress__measure">33%</span>
                    </div>
                    <div
                      class="pf-v5-c-progress__bar"
                      role="progressbar"
                      aria-valuemin="0"
                      aria-valuemax="100"
                      aria-valuenow="33"
                      aria-labelledby="primary-detail-card-view-expanded-example-drawer-progress-example1-description"
                      aria-label="Progress 1"
                    >
                      <div
                        class="pf-v5-c-progress__indicator"
                        style="width:33%;"
                      ></div>
                    </div>
                  </div>
                </div>
                <div class="pf-v5-l-flex__item">
                  <div
                    class="pf-v5-c-progress"
                    id="primary-detail-card-view-expanded-example-drawer-progress-example2"
                  >
                    <div
                      class="pf-v5-c-progress__description"
                      id="primary-detail-card-view-expanded-example-drawer-progress-example2-description"
                    >Modules</div>
                    <div class="pf-v5-c-progress__status" aria-hidden="true">
                      <span class="pf-v5-c-progress__measure">66%</span>
                    </div>
                    <div
                      class="pf-v5-c-progress__bar"
                      role="progressbar"
                      aria-valuemin="0"
                      aria-valuemax="100"
                      aria-valuenow="66"
                      aria-labelledby="primary-detail-card-view-expanded-example-drawer-progress-example2-description"
                      aria-label="Progress 2"
                    >
                      <div
                        class="pf-v5-c-progress__indicator"
                        style="width:66%;"
                      ></div>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>
  </main>
</div>

```

### Primary-detail card simple list expanded on mobile

```html isFullscreen
<div
  class="pf-v5-c-page"
  id="primary-detail-card-simple-list-on-mobile-example"
>
  <div class="pf-v5-c-skip-to-content">
    <a
      class="pf-v5-c-button pf-m-primary"
      href="#main-content-primary-detail-card-simple-list-on-mobile-example"
    >Skip to content</a>
  </div>
  <header
    class="pf-v5-c-masthead"
    id="primary-detail-card-simple-list-on-mobile-example-masthead"
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
        id="primary-detail-card-simple-list-on-mobile-example-masthead-toolbar"
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
        id="primary-detail-card-simple-list-on-mobile-example-primary-nav"
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
    id="main-content-primary-detail-card-simple-list-on-mobile-example"
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
    <div class="pf-v5-c-divider" role="separator"></div>
    <section class="pf-v5-c-page__main-section">
      <div class="pf-v5-c-card">
        <!-- Drawer -->
        <div class="pf-v5-c-drawer pf-m-expanded pf-m-static">
          <div class="pf-v5-c-drawer__main">
            <!-- Content -->
            <div class="pf-v5-c-drawer__content">
              <div class="pf-v5-c-simple-list">
                <section class="pf-v5-c-simple-list__section">
                  <h2 class="pf-v5-c-simple-list__title">Section title</h2>
                  <ul class="pf-v5-c-simple-list__list" role="list">
                    <li class="pf-v5-c-simple-list__item">
                      <button
                        class="pf-v5-c-simple-list__item-link pf-m-current"
                        type="button"
                      >List item 1</button>
                    </li>
                    <li class="pf-v5-c-simple-list__item">
                      <button
                        class="pf-v5-c-simple-list__item-link"
                        type="button"
                      >List item 2</button>
                    </li>
                    <li class="pf-v5-c-simple-list__item">
                      <button
                        class="pf-v5-c-simple-list__item-link"
                        type="button"
                      >List item 3</button>
                    </li>
                    <li class="pf-v5-c-simple-list__item">
                      <button
                        class="pf-v5-c-simple-list__item-link"
                        type="button"
                      >List item 4</button>
                    </li>
                  </ul>
                </section>
                <section class="pf-v5-c-simple-list__section">
                  <h2 class="pf-v5-c-simple-list__title">Section title</h2>
                  <ul class="pf-v5-c-simple-list__list" role="list">
                    <li class="pf-v5-c-simple-list__item">
                      <button
                        class="pf-v5-c-simple-list__item-link"
                        type="button"
                      >List item 5</button>
                    </li>
                    <li class="pf-v5-c-simple-list__item">
                      <button
                        class="pf-v5-c-simple-list__item-link"
                        type="button"
                      >List item 6</button>
                    </li>
                    <li class="pf-v5-c-simple-list__item">
                      <button
                        class="pf-v5-c-simple-list__item-link"
                        type="button"
                      >List item 7</button>
                    </li>
                    <li class="pf-v5-c-simple-list__item">
                      <button
                        class="pf-v5-c-simple-list__item-link"
                        type="button"
                      >List item 8</button>
                    </li>
                    <li class="pf-v5-c-simple-list__item">
                      <button
                        class="pf-v5-c-simple-list__item-link"
                        type="button"
                      >List item 9</button>
                    </li>
                    <li class="pf-v5-c-simple-list__item">
                      <button
                        class="pf-v5-c-simple-list__item-link"
                        type="button"
                      >List item 10</button>
                    </li>
                  </ul>
                </section>
              </div>
            </div>

            <!-- Panel -->
            <div
              class="pf-v5-c-drawer__panel pf-m-width-75-on-xl"
              id="primary-detail-card-simple-list-on-mobile-example-drawer-panel"
              aria-label="Panel"
            >
              <!-- Panel header -->
              <div class="pf-v5-c-drawer__body">
                <div class="pf-v5-l-flex pf-m-column">
                  <div class="pf-v5-l-flex__item">
                    <div class="pf-v5-c-drawer__head">
                      <div class="pf-v5-c-drawer__actions">
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
                      <h2
                        class="pf-v5-c-title pf-m-lg"
                        id="primary-detail-card-simple-list-on-mobile-example-drawer-drawer-label"
                      >Patternfly-elements</h2>
                    </div>
                  </div>
                </div>
              </div>

              <div class="pf-v5-c-drawer__body">
                <div class="pf-v5-l-flex pf-m-column pf-m-space-items-lg">
                  <div class="pf-v5-l-flex__item">
                    <p>The content of the drawer really is up to you. It could have form fields, definition lists, text lists, labels, charts, progress bars, etc. Spacing recommendation is 24px margins. You can put tabs in here, and can also make the drawer scrollable.</p>
                  </div>
                  <div class="pf-v5-l-flex__item">
                    <div
                      class="pf-v5-c-progress"
                      id="primary-detail-card-simple-list-on-mobile-example-drawer-progress-example1"
                    >
                      <div
                        class="pf-v5-c-progress__description"
                        id="primary-detail-card-simple-list-on-mobile-example-drawer-progress-example1-description"
                      >Capacity</div>
                      <div class="pf-v5-c-progress__status" aria-hidden="true">
                        <span class="pf-v5-c-progress__measure">33%</span>
                      </div>
                      <div
                        class="pf-v5-c-progress__bar"
                        role="progressbar"
                        aria-valuemin="0"
                        aria-valuemax="100"
                        aria-valuenow="33"
                        aria-labelledby="primary-detail-card-simple-list-on-mobile-example-drawer-progress-example1-description"
                        aria-label="Progress 1"
                      >
                        <div
                          class="pf-v5-c-progress__indicator"
                          style="width:33%;"
                        ></div>
                      </div>
                    </div>
                  </div>
                  <div class="pf-v5-l-flex__item">
                    <div
                      class="pf-v5-c-progress"
                      id="primary-detail-card-simple-list-on-mobile-example-drawer-progress-example2"
                    >
                      <div
                        class="pf-v5-c-progress__description"
                        id="primary-detail-card-simple-list-on-mobile-example-drawer-progress-example2-description"
                      >Modules</div>
                      <div class="pf-v5-c-progress__status" aria-hidden="true">
                        <span class="pf-v5-c-progress__measure">66%</span>
                      </div>
                      <div
                        class="pf-v5-c-progress__bar"
                        role="progressbar"
                        aria-valuemin="0"
                        aria-valuemax="100"
                        aria-valuenow="66"
                        aria-labelledby="primary-detail-card-simple-list-on-mobile-example-drawer-progress-example2-description"
                        aria-label="Progress 2"
                      >
                        <div
                          class="pf-v5-c-progress__indicator"
                          style="width:66%;"
                        ></div>
                      </div>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>
  </main>
</div>

```

### Primary-detail card data list expanded on mobile

```html isFullscreen
<div class="pf-v5-c-page" id="primary-detail-card-data-list-example">
  <div class="pf-v5-c-skip-to-content">
    <a
      class="pf-v5-c-button pf-m-primary"
      href="#main-content-primary-detail-card-data-list-example"
    >Skip to content</a>
  </div>
  <header
    class="pf-v5-c-masthead"
    id="primary-detail-card-data-list-example-masthead"
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
        id="primary-detail-card-data-list-example-masthead-toolbar"
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
        id="primary-detail-card-data-list-example-primary-nav"
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
    id="main-content-primary-detail-card-data-list-example"
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
    <div class="pf-v5-c-divider" role="separator"></div>
    <section class="pf-v5-c-page__main-section">
      <div class="pf-v5-c-card">
        <!-- Drawer -->
        <div class="pf-v5-c-drawer pf-m-expanded pf-m-static">
          <div class="pf-v5-c-drawer__main">
            <!-- Content -->
            <div class="pf-v5-c-drawer__content">
              <div class="pf-v5-c-drawer__body">
                <div
                  class="pf-v5-c-toolbar"
                  id="primary-detail-card-data-list-example-drawer-toolbar-card-toolbar"
                >
                  <div class="pf-v5-c-toolbar__content">
                    <div class="pf-v5-c-toolbar__content-section">
                      <div class="pf-v5-c-toolbar__item">
                        <div class="pf-v5-c-select" style="width: 150px">
                          <span
                            id="primary-detail-card-data-list-example-drawer-toolbar-card-toolbar-select-dropdown-label"
                            hidden
                          >Choose one</span>

                          <button
                            class="pf-v5-c-select__toggle"
                            type="button"
                            id="primary-detail-card-data-list-example-drawer-toolbar-card-toolbar-select-dropdown-toggle"
                            aria-haspopup="true"
                            aria-expanded="false"
                            aria-labelledby="primary-detail-card-data-list-example-drawer-toolbar-card-toolbar-select-dropdown-label primary-detail-card-data-list-example-drawer-toolbar-card-toolbar-select-dropdown-toggle"
                          >
                            <div class="pf-v5-c-select__toggle-wrapper">
                              <span class="pf-v5-c-select__toggle-text">Dropdown</span>
                            </div>
                            <span class="pf-v5-c-select__toggle-arrow">
                              <i class="fas fa-caret-down" aria-hidden="true"></i>
                            </span>
                          </button>

                          <ul
                            class="pf-v5-c-select__menu"
                            role="listbox"
                            aria-labelledby="primary-detail-card-data-list-example-drawer-toolbar-card-toolbar-select-dropdown-label"
                            hidden
                            style="width: 150px"
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
                      <div class="pf-v5-c-toolbar__item pf-m-pagination">
                        <div class="pf-v5-c-pagination">
                          <div class="pf-v5-c-pagination__total-items">
                            <b>1 - 10</b> of
                            <b>37</b>
                          </div>
                          <div class="pf-v5-c-options-menu">
                            <button
                              class="pf-v5-c-options-menu__toggle pf-m-text pf-m-plain"
                              type="button"
                              id="primary-detail-card-data-list-example-drawer-toolbar-card-toolbar-pagination-options-menu-toggle"
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
                              aria-labelledby="primary-detail-card-data-list-example-drawer-toolbar-card-toolbar-pagination-options-menu-toggle"
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
                                  <div
                                    class="pf-v5-c-options-menu__menu-item-icon"
                                  >
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
                        </div>
                      </div>
                    </div>
                    <div
                      class="pf-v5-c-toolbar__expandable-content pf-m-hidden"
                      id="primary-detail-card-data-list-example-drawer-toolbar-card-toolbar-expandable-content"
                      hidden
                    ></div>
                  </div>
                </div>
                <ul
                  class="pf-v5-c-data-list"
                  role="list"
                  aria-label="Selectable rows data list example"
                  id="primary-detail-card-data-list-example-drawer-card-data-list"
                >
                  <li
                    class="pf-v5-c-data-list__item"
                    aria-labelledby="primary-detail-card-data-list-example-drawer-card-data-list-item-1"
                  >
                    <div class="pf-v5-c-data-list__item-row">
                      <div class="pf-v5-c-data-list__item-content">
                        <div class="pf-v5-c-data-list__cell">
                          <div
                            id="primary-detail-card-data-list-example-drawer-card-data-list-item-1"
                          >Node 1</div>
                          <a href="#">siemur/test-space</a>
                        </div>
                      </div>
                    </div>
                  </li>

                  <li
                    class="pf-v5-c-data-list__item pf-m-selected"
                    aria-labelledby="primary-detail-card-data-list-example-drawer-card-data-list-item-2"
                  >
                    <div class="pf-v5-c-data-list__item-row">
                      <div class="pf-v5-c-data-list__item-content">
                        <div class="pf-v5-c-data-list__cell">
                          <div
                            id="primary-detail-card-data-list-example-drawer-card-data-list-item-2"
                          >Node 2</div>
                          <a href="#">siemur/test-space</a>
                        </div>
                      </div>
                    </div>
                  </li>

                  <li
                    class="pf-v5-c-data-list__item"
                    aria-labelledby="primary-detail-card-data-list-example-drawer-card-data-list-item-3"
                  >
                    <div class="pf-v5-c-data-list__item-row">
                      <div class="pf-v5-c-data-list__item-content">
                        <div class="pf-v5-c-data-list__cell">
                          <div
                            id="primary-detail-card-data-list-example-drawer-card-data-list-item-3"
                          >Node 3</div>
                          <a href="#">siemur/test-space</a>
                        </div>
                      </div>
                    </div>
                  </li>

                  <li
                    class="pf-v5-c-data-list__item"
                    aria-labelledby="primary-detail-card-data-list-example-drawer-card-data-list-item-4"
                  >
                    <div class="pf-v5-c-data-list__item-row">
                      <div class="pf-v5-c-data-list__item-content">
                        <div class="pf-v5-c-data-list__cell">
                          <div
                            id="primary-detail-card-data-list-example-drawer-card-data-list-item-4"
                          >Node 4</div>
                          <a href="#">siemur/test-space</a>
                        </div>
                      </div>
                    </div>
                  </li>

                  <li
                    class="pf-v5-c-data-list__item"
                    aria-labelledby="primary-detail-card-data-list-example-drawer-card-data-list-item-5"
                  >
                    <div class="pf-v5-c-data-list__item-row">
                      <div class="pf-v5-c-data-list__item-content">
                        <div class="pf-v5-c-data-list__cell">
                          <div
                            id="primary-detail-card-data-list-example-drawer-card-data-list-item-5"
                          >Node 5</div>
                          <a href="#">siemur/test-space</a>
                        </div>
                      </div>
                    </div>
                  </li>

                  <li
                    class="pf-v5-c-data-list__item"
                    aria-labelledby="primary-detail-card-data-list-example-drawer-card-data-list-item-6"
                  >
                    <div class="pf-v5-c-data-list__item-row">
                      <div class="pf-v5-c-data-list__item-content">
                        <div class="pf-v5-c-data-list__cell">
                          <div
                            id="primary-detail-card-data-list-example-drawer-card-data-list-item-6"
                          >Node 6</div>
                          <a href="#">siemur/test-space</a>
                        </div>
                      </div>
                    </div>
                  </li>

                  <li
                    class="pf-v5-c-data-list__item"
                    aria-labelledby="primary-detail-card-data-list-example-drawer-card-data-list-item-7"
                  >
                    <div class="pf-v5-c-data-list__item-row">
                      <div class="pf-v5-c-data-list__item-content">
                        <div class="pf-v5-c-data-list__cell">
                          <div
                            id="primary-detail-card-data-list-example-drawer-card-data-list-item-7"
                          >Node 7</div>
                          <a href="#">siemur/test-space</a>
                        </div>
                      </div>
                    </div>
                  </li>

                  <li
                    class="pf-v5-c-data-list__item"
                    aria-labelledby="primary-detail-card-data-list-example-drawer-card-data-list-item-8"
                  >
                    <div class="pf-v5-c-data-list__item-row">
                      <div class="pf-v5-c-data-list__item-content">
                        <div class="pf-v5-c-data-list__cell">
                          <div
                            id="primary-detail-card-data-list-example-drawer-card-data-list-item-8"
                          >Node 8</div>
                          <a href="#">siemur/test-space</a>
                        </div>
                      </div>
                    </div>
                  </li>

                  <li
                    class="pf-v5-c-data-list__item"
                    aria-labelledby="primary-detail-card-data-list-example-drawer-card-data-list-item-9"
                  >
                    <div class="pf-v5-c-data-list__item-row">
                      <div class="pf-v5-c-data-list__item-content">
                        <div class="pf-v5-c-data-list__cell">
                          <div
                            id="primary-detail-card-data-list-example-drawer-card-data-list-item-9"
                          >Node 9</div>
                          <a href="#">siemur/test-space</a>
                        </div>
                      </div>
                    </div>
                  </li>

                  <li
                    class="pf-v5-c-data-list__item"
                    aria-labelledby="primary-detail-card-data-list-example-drawer-card-data-list-item-10"
                  >
                    <div class="pf-v5-c-data-list__item-row">
                      <div class="pf-v5-c-data-list__item-content">
                        <div class="pf-v5-c-data-list__cell">
                          <div
                            id="primary-detail-card-data-list-example-drawer-card-data-list-item-10"
                          >Node 10</div>
                          <a href="#">siemur/test-space</a>
                        </div>
                      </div>
                    </div>
                  </li>
                </ul>
              </div>
            </div>

            <!-- Panel -->
            <div
              class="pf-v5-c-drawer__panel pf-m-width-75-on-2xl"
              id="primary-detail-card-data-list-example-drawer-panel"
              aria-label="Panel"
            >
              <!-- Panel header -->
              <div class="pf-v5-c-drawer__body">
                <div class="pf-v5-l-flex pf-m-column">
                  <div class="pf-v5-l-flex__item">
                    <div class="pf-v5-c-drawer__head">
                      <div class="pf-v5-c-drawer__actions">
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
                      <h2
                        class="pf-v5-c-title pf-m-lg"
                        id="primary-detail-card-data-list-example-drawer-drawer-label"
                      >Patternfly-elements</h2>
                    </div>
                  </div>
                </div>
              </div>
              <div class="pf-v5-c-drawer__body">
                <div class="pf-v5-l-flex pf-m-column pf-m-space-items-lg">
                  <div class="pf-v5-l-flex__item">
                    <p>The content of the drawer really is up to you. It could have form fields, definition lists, text lists, labels, charts, progress bars, etc. Spacing recommendation is 24px margins. You can put tabs in here, and can also make the drawer scrollable.</p>
                  </div>
                  <div class="pf-v5-l-flex__item">
                    <div
                      class="pf-v5-c-progress"
                      id="primary-detail-card-data-list-example-drawer-progress-example1"
                    >
                      <div
                        class="pf-v5-c-progress__description"
                        id="primary-detail-card-data-list-example-drawer-progress-example1-description"
                      >Capacity</div>
                      <div class="pf-v5-c-progress__status" aria-hidden="true">
                        <span class="pf-v5-c-progress__measure">33%</span>
                      </div>
                      <div
                        class="pf-v5-c-progress__bar"
                        role="progressbar"
                        aria-valuemin="0"
                        aria-valuemax="100"
                        aria-valuenow="33"
                        aria-labelledby="primary-detail-card-data-list-example-drawer-progress-example1-description"
                        aria-label="Progress 1"
                      >
                        <div
                          class="pf-v5-c-progress__indicator"
                          style="width:33%;"
                        ></div>
                      </div>
                    </div>
                  </div>
                  <div class="pf-v5-l-flex__item">
                    <div
                      class="pf-v5-c-progress"
                      id="primary-detail-card-data-list-example-drawer-progress-example2"
                    >
                      <div
                        class="pf-v5-c-progress__description"
                        id="primary-detail-card-data-list-example-drawer-progress-example2-description"
                      >Modules</div>
                      <div class="pf-v5-c-progress__status" aria-hidden="true">
                        <span class="pf-v5-c-progress__measure">66%</span>
                      </div>
                      <div
                        class="pf-v5-c-progress__bar"
                        role="progressbar"
                        aria-valuemin="0"
                        aria-valuemax="100"
                        aria-valuenow="66"
                        aria-labelledby="primary-detail-card-data-list-example-drawer-progress-example2-description"
                        aria-label="Progress 2"
                      >
                        <div
                          class="pf-v5-c-progress__indicator"
                          style="width:66%;"
                        ></div>
                      </div>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>
  </main>
</div>

```

### Inline modifier

```html isFullscreen
<div class="pf-v5-c-page" id="primary-detail-inline-modifier-example">
  <div class="pf-v5-c-skip-to-content">
    <a
      class="pf-v5-c-button pf-m-primary"
      href="#main-content-primary-detail-inline-modifier-example"
    >Skip to content</a>
  </div>
  <header
    class="pf-v5-c-masthead"
    id="primary-detail-inline-modifier-example-masthead"
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
        id="primary-detail-inline-modifier-example-masthead-toolbar"
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
        id="primary-detail-inline-modifier-example-primary-nav"
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
    id="main-content-primary-detail-inline-modifier-example"
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
    <div class="pf-v5-c-divider" role="separator"></div>
    <div class="pf-v5-c-drawer pf-m-expanded pf-m-inline-on-2xl">
      <div class="pf-v5-c-drawer__main">
        <!-- Content -->
        <div class="pf-v5-c-drawer__content">
          <div class="pf-v5-c-drawer__body">
            <div
              class="pf-v5-c-toolbar pf-m-page-insets"
              id="primary-detail-inline-modifier-example-drawer-toolbar"
            >
              <div class="pf-v5-c-toolbar__content">
                <div class="pf-v5-c-toolbar__content-section pf-m-nowrap">
                  <div class="pf-v5-c-toolbar__item">
                    <div class="pf-v5-c-select" style="width: 150px">
                      <span
                        id="primary-detail-inline-modifier-example-drawer-toolbar-select-status-label"
                        hidden
                      >Choose one</span>

                      <button
                        class="pf-v5-c-select__toggle"
                        type="button"
                        id="primary-detail-inline-modifier-example-drawer-toolbar-select-status-toggle"
                        aria-haspopup="true"
                        aria-expanded="false"
                        aria-labelledby="primary-detail-inline-modifier-example-drawer-toolbar-select-status-label primary-detail-inline-modifier-example-drawer-toolbar-select-status-toggle"
                      >
                        <div class="pf-v5-c-select__toggle-wrapper">
                          <span class="pf-v5-c-select__toggle-icon">
                            <i class="fas fa-bookmark" aria-hidden="true"></i>
                          </span>
                          <span class="pf-v5-c-select__toggle-text">Status</span>
                        </div>
                        <span class="pf-v5-c-select__toggle-arrow">
                          <i class="fas fa-caret-down" aria-hidden="true"></i>
                        </span>
                      </button>

                      <ul
                        class="pf-v5-c-select__menu"
                        role="listbox"
                        aria-labelledby="primary-detail-inline-modifier-example-drawer-toolbar-select-status-label"
                        hidden
                        style="width: 150px"
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

                  <div
                    class="pf-v5-c-overflow-menu"
                    id="primary-detail-inline-modifier-example-drawer-toolbar-overflow-menu"
                  >
                    <div
                      class="pf-v5-c-overflow-menu__content pf-v5-u-display-none pf-v5-u-display-flex-on-lg"
                    >
                      <div
                        class="pf-v5-c-overflow-menu__group pf-m-button-group"
                      >
                        <div class="pf-v5-c-overflow-menu__item">
                          <button
                            class="pf-v5-c-button pf-m-primary"
                            type="button"
                          >Create instance</button>
                        </div>

                        <div class="pf-v5-c-overflow-menu__item">
                          <button
                            class="pf-v5-c-button pf-m-secondary"
                            type="button"
                          >Action</button>
                        </div>
                      </div>
                    </div>
                    <div class="pf-v5-c-overflow-menu__control">
                      <div class="pf-v5-c-dropdown">
                        <button
                          class="pf-v5-c-button pf-v5-c-dropdown__toggle pf-m-plain"
                          type="button"
                          id="primary-detail-inline-modifier-example-drawer-toolbar-overflow-menu-dropdown-toggle"
                          aria-label="Dropdown with additional options"
                          aria-expanded="false"
                        >
                          <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
                        </button>
                        <ul
                          class="pf-v5-c-dropdown__menu"
                          role="menu"
                          aria-labelledby="primary-detail-inline-modifier-example-drawer-toolbar-overflow-menu-dropdown-toggle"
                          hidden
                        >
                          <li role="none">
                            <button
                              role="menuitem"
                              class="pf-v5-c-dropdown__menu-item"
                            >Action 7</button>
                          </li>
                        </ul>
                      </div>
                    </div>
                  </div>

                  <div class="pf-v5-c-toolbar__item pf-m-pagination">
                    <div class="pf-v5-c-pagination pf-m-compact">
                      <div class="pf-v5-c-options-menu">
                        <button
                          class="pf-v5-c-options-menu__toggle pf-m-text pf-m-plain"
                          type="button"
                          id="primary-detail-inline-modifier-example-drawer-toolbar-top-pagination-toggle"
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
                          aria-labelledby="primary-detail-inline-modifier-example-drawer-toolbar-top-pagination-toggle"
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
                        aria-label="Toolbar top pagination"
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
                  </div>
                </div>

                <div
                  class="pf-v5-c-toolbar__expandable-content pf-m-hidden"
                  id="primary-detail-inline-modifier-example-drawer-toolbar-expandable-content"
                  hidden
                ></div>
              </div>
            </div>
            <ul
              class="pf-v5-c-data-list"
              role="list"
              aria-label="Simple data list example"
              id="primary-detail-inline-modifier-example-data-list"
            >
              <li
                class="pf-v5-c-data-list__item"
                aria-labelledby="primary-detail-inline-modifier-example-data-list-item-1"
              >
                <div class="pf-v5-c-data-list__item-row">
                  <div class="pf-v5-c-data-list__item-content">
                    <div class="pf-v5-c-data-list__cell pf-m-align-left">
                      <div class="pf-v5-l-flex pf-m-column pf-m-space-items-md">
                        <div
                          class="pf-v5-l-flex pf-m-column pf-m-space-items-none"
                        >
                          <div class="pf-v5-l-flex__item">
                            <p
                              id="primary-detail-inline-modifier-example-data-list-item-1"
                            >patternfly</p>
                          </div>
                          <div class="pf-v5-l-flex__item">
                            <small>
                              Working repo for PatternFly 4
                              <a href>https://pf4.patternfly.org/</a>
                            </small>
                          </div>
                        </div>
                        <div class="pf-v5-l-flex pf-m-wrap">
                          <div class="pf-v5-l-flex pf-m-space-items-xs">
                            <div class="pf-v5-l-flex__item">
                              <i class="fas fa-code-branch" aria-hidden="true"></i>
                            </div>
                            <div class="pf-v5-l-flex__item">
                              <span>10</span>
                            </div>
                          </div>
                          <div class="pf-v5-l-flex pf-m-space-items-xs">
                            <div class="pf-v5-l-flex__item">
                              <i class="fas fa-code" aria-hidden="true"></i>
                            </div>
                            <div class="pf-v5-l-flex__item">
                              <span>4</span>
                            </div>
                          </div>
                          <div class="pf-v5-l-flex pf-m-space-items-xs">
                            <div class="pf-v5-l-flex__item">
                              <i class="fas fa-cube" aria-hidden="true"></i>
                            </div>
                            <div class="pf-v5-l-flex__item">
                              <span>5</span>
                            </div>
                          </div>
                          <div class="pf-v5-l-flex__item">Updated 2 days ago</div>
                        </div>
                      </div>
                    </div>
                    <div
                      class="pf-v5-c-data-list__cell pf-m-align-right pf-m-no-fill"
                    >
                      <button
                        class="pf-v5-c-button pf-m-secondary"
                        type="button"
                      >Action</button>
                      <button
                        class="pf-v5-c-button pf-m-link"
                        type="button"
                      >Link</button>
                    </div>
                  </div>
                </div>
              </li>

              <li
                class="pf-v5-c-data-list__item"
                aria-labelledby="primary-detail-inline-modifier-example-data-list-item-2"
              >
                <div class="pf-v5-c-data-list__item-row">
                  <div class="pf-v5-c-data-list__item-content">
                    <div class="pf-v5-c-data-list__cell pf-m-align-left">
                      <div class="pf-v5-l-flex pf-m-column pf-m-space-items-md">
                        <div
                          class="pf-v5-l-flex pf-m-column pf-m-space-items-none"
                        >
                          <div class="pf-v5-l-flex__item">
                            <p
                              id="primary-detail-inline-modifier-example-data-list-item-2"
                            >patternfly-elements</p>
                          </div>
                          <div class="pf-v5-l-flex__item">
                            <small>PatternFly elements</small>
                          </div>
                        </div>
                        <div class="pf-v5-l-flex pf-m-wrap">
                          <div class="pf-v5-l-flex pf-m-space-items-xs">
                            <div class="pf-v5-l-flex__item">
                              <i class="fas fa-code-branch" aria-hidden="true"></i>
                            </div>
                            <div class="pf-v5-l-flex__item">
                              <span>5</span>
                            </div>
                          </div>
                          <div class="pf-v5-l-flex pf-m-space-items-xs">
                            <div class="pf-v5-l-flex__item">
                              <i class="fas fa-code" aria-hidden="true"></i>
                            </div>
                            <div class="pf-v5-l-flex__item">
                              <span>9</span>
                            </div>
                          </div>
                          <div class="pf-v5-l-flex pf-m-space-items-xs">
                            <div class="pf-v5-l-flex__item">
                              <i class="fas fa-cube" aria-hidden="true"></i>
                            </div>
                            <div class="pf-v5-l-flex__item">
                              <span>2</span>
                            </div>
                          </div>
                          <div class="pf-v5-l-flex pf-m-space-items-xs">
                            <div class="pf-v5-l-flex__item">
                              <i class="fas fa-check-circle" aria-hidden="true"></i>
                            </div>
                            <div class="pf-v5-l-flex__item">
                              <span>11</span>
                            </div>
                          </div>
                          <div class="pf-v5-l-flex pf-m-space-items-xs">
                            <div class="pf-v5-l-flex__item">
                              <i
                                class="fas fa-exclamation-triangle"
                                aria-hidden="true"
                              ></i>
                            </div>
                            <div class="pf-v5-l-flex__item">
                              <span>4</span>
                            </div>
                          </div>
                          <div class="pf-v5-l-flex pf-m-space-items-xs">
                            <div class="pf-v5-l-flex__item">
                              <i class="fas fa-times-circle" aria-hidden="true"></i>
                            </div>
                            <div class="pf-v5-l-flex__item">
                              <span>1</span>
                            </div>
                          </div>
                          <div class="pf-v5-l-flex__item">Updated 2 days ago</div>
                        </div>
                      </div>
                    </div>
                    <div
                      class="pf-v5-c-data-list__cell pf-m-align-right pf-m-no-fill"
                    >
                      <button
                        class="pf-v5-c-button pf-m-secondary"
                        type="button"
                      >Action</button>
                      <button
                        class="pf-v5-c-button pf-m-link"
                        type="button"
                      >Link</button>
                    </div>
                  </div>
                </div>
              </li>

              <li
                class="pf-v5-c-data-list__item"
                aria-labelledby="primary-detail-inline-modifier-example-data-list-item-3"
              >
                <div class="pf-v5-c-data-list__item-row">
                  <div class="pf-v5-c-data-list__item-content">
                    <div class="pf-v5-c-data-list__cell pf-m-align-left">
                      <p
                        id="primary-detail-inline-modifier-example-data-list-item-3"
                      >patternfly-unified-design-kit</p>
                    </div>
                    <div
                      class="pf-v5-c-data-list__cell pf-m-align-right pf-m-no-fill"
                    >
                      <button
                        class="pf-v5-c-button pf-m-secondary"
                        type="button"
                      >Action</button>
                      <button
                        class="pf-v5-c-button pf-m-link"
                        type="button"
                      >Link</button>
                    </div>
                  </div>
                </div>
              </li>

              <li
                class="pf-v5-c-data-list__item"
                aria-labelledby="primary-detail-inline-modifier-example-data-list-item-4"
              >
                <div class="pf-v5-c-data-list__item-row">
                  <div class="pf-v5-c-data-list__item-content">
                    <div class="pf-v5-c-data-list__cell pf-m-align-left">
                      <div class="pf-v5-l-flex pf-m-column pf-m-space-items-md">
                        <div
                          class="pf-v5-l-flex pf-m-column pf-m-space-items-none"
                        >
                          <div class="pf-v5-l-flex__item">
                            <p
                              id="primary-detail-inline-modifier-example-data-list-item-4"
                            >patternfly</p>
                          </div>
                          <div class="pf-v5-l-flex__item">
                            <small>
                              Working repo for PatternFly 4
                              <a href>https://pf4.patternfly.org/</a>
                            </small>
                          </div>
                        </div>
                        <div class="pf-v5-l-flex pf-m-wrap">
                          <div class="pf-v5-l-flex pf-m-space-items-xs">
                            <div class="pf-v5-l-flex__item">
                              <i class="fas fa-code-branch" aria-hidden="true"></i>
                            </div>
                            <div class="pf-v5-l-flex__item">
                              <span>10</span>
                            </div>
                          </div>
                          <div class="pf-v5-l-flex pf-m-space-items-xs">
                            <div class="pf-v5-l-flex__item">
                              <i class="fas fa-code" aria-hidden="true"></i>
                            </div>
                            <div class="pf-v5-l-flex__item">
                              <span>4</span>
                            </div>
                          </div>
                          <div class="pf-v5-l-flex pf-m-space-items-xs">
                            <div class="pf-v5-l-flex__item">
                              <i class="fas fa-cube" aria-hidden="true"></i>
                            </div>
                            <div class="pf-v5-l-flex__item">
                              <span>5</span>
                            </div>
                          </div>
                          <div class="pf-v5-l-flex__item">Updated 2 days ago</div>
                        </div>
                      </div>
                    </div>
                    <div
                      class="pf-v5-c-data-list__cell pf-m-align-right pf-m-no-fill"
                    >
                      <button
                        class="pf-v5-c-button pf-m-secondary"
                        type="button"
                      >Action</button>
                      <button
                        class="pf-v5-c-button pf-m-link"
                        type="button"
                      >Link</button>
                    </div>
                  </div>
                </div>
              </li>

              <li
                class="pf-v5-c-data-list__item"
                aria-labelledby="primary-detail-inline-modifier-example-data-list-item-5"
              >
                <div class="pf-v5-c-data-list__item-row">
                  <div class="pf-v5-c-data-list__item-content">
                    <div class="pf-v5-c-data-list__cell pf-m-align-left">
                      <div class="pf-v5-l-flex pf-m-column pf-m-space-items-md">
                        <div
                          class="pf-v5-l-flex pf-m-column pf-m-space-items-none"
                        >
                          <div class="pf-v5-l-flex__item">
                            <p
                              id="primary-detail-inline-modifier-example-data-list-item-5"
                            >patternfly-elements</p>
                          </div>
                          <div class="pf-v5-l-flex__item">
                            <small>PatternFly elements</small>
                          </div>
                        </div>
                        <div class="pf-v5-l-flex pf-m-wrap">
                          <div class="pf-v5-l-flex pf-m-space-items-xs">
                            <div class="pf-v5-l-flex__item">
                              <i class="fas fa-code-branch" aria-hidden="true"></i>
                            </div>
                            <div class="pf-v5-l-flex__item">
                              <span>5</span>
                            </div>
                          </div>
                          <div class="pf-v5-l-flex pf-m-space-items-xs">
                            <div class="pf-v5-l-flex__item">
                              <i class="fas fa-code" aria-hidden="true"></i>
                            </div>
                            <div class="pf-v5-l-flex__item">
                              <span>9</span>
                            </div>
                          </div>
                          <div class="pf-v5-l-flex pf-m-space-items-xs">
                            <div class="pf-v5-l-flex__item">
                              <i class="fas fa-cube" aria-hidden="true"></i>
                            </div>
                            <div class="pf-v5-l-flex__item">
                              <span>2</span>
                            </div>
                          </div>
                          <div class="pf-v5-l-flex pf-m-space-items-xs">
                            <div class="pf-v5-l-flex__item">
                              <i class="fas fa-check-circle" aria-hidden="true"></i>
                            </div>
                            <div class="pf-v5-l-flex__item">
                              <span>11</span>
                            </div>
                          </div>
                          <div class="pf-v5-l-flex pf-m-space-items-xs">
                            <div class="pf-v5-l-flex__item">
                              <i
                                class="fas fa-exclamation-triangle"
                                aria-hidden="true"
                              ></i>
                            </div>
                            <div class="pf-v5-l-flex__item">
                              <span>4</span>
                            </div>
                          </div>
                          <div class="pf-v5-l-flex pf-m-space-items-xs">
                            <div class="pf-v5-l-flex__item">
                              <i class="fas fa-times-circle" aria-hidden="true"></i>
                            </div>
                            <div class="pf-v5-l-flex__item">
                              <span>1</span>
                            </div>
                          </div>
                          <div class="pf-v5-l-flex__item">Updated 2 days ago</div>
                        </div>
                      </div>
                    </div>
                    <div
                      class="pf-v5-c-data-list__cell pf-m-align-right pf-m-no-fill"
                    >
                      <button
                        class="pf-v5-c-button pf-m-secondary"
                        type="button"
                      >Action</button>
                      <button
                        class="pf-v5-c-button pf-m-link"
                        type="button"
                      >Link</button>
                    </div>
                  </div>
                </div>
              </li>
            </ul>
          </div>
        </div>

        <!-- Panel -->
        <div class="pf-v5-c-drawer__panel">
          <!-- Panel header -->
          <div class="pf-v5-c-drawer__body">
            <div class="pf-v5-l-flex pf-m-column">
              <div class="pf-v5-l-flex__item">
                <div class="pf-v5-c-drawer__head">
                  <div class="pf-v5-c-drawer__actions">
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
                  <h2
                    class="pf-v5-c-title pf-m-lg"
                    id="primary-detail-inline-modifier-example-drawer-drawer-label"
                  >Node 2</h2>
                </div>
              </div>
              <div class="pf-v5-l-flex__item">
                <a href="#">siemur/test-space</a>
              </div>
            </div>
          </div>

          <!-- Tabs -->
          <div class="pf-v5-c-drawer__body pf-m-no-padding">
            <div
              class="pf-v5-c-tabs pf-m-box pf-m-fill"
              role="region"
              id="primary-detail-inline-modifier-example-drawer-tabs"
            >
              <button
                class="pf-v5-c-tabs__scroll-button"
                type="button"
                aria-label="Scroll left"
              >
                <i class="fas fa-angle-left" aria-hidden="true"></i>
              </button>
              <ul class="pf-v5-c-tabs__list" role="tablist">
                <li class="pf-v5-c-tabs__item pf-m-current" role="presentation">
                  <button
                    type="button"
                    class="pf-v5-c-tabs__link"
                    role="tab"
                    aria-controls="primary-detail-inline-modifier-example-drawer-tabs-tab1-panel"
                    id="primary-detail-inline-modifier-example-drawer-tabs-tab1-link"
                  >
                    <span class="pf-v5-c-tabs__item-text">Overview</span>
                  </button>
                </li>
                <li class="pf-v5-c-tabs__item" role="presentation">
                  <button
                    type="button"
                    class="pf-v5-c-tabs__link"
                    role="tab"
                    aria-controls="primary-detail-inline-modifier-example-drawer-tabs-tab2-panel"
                    id="primary-detail-inline-modifier-example-drawer-tabs-tab2-link"
                  >
                    <span class="pf-v5-c-tabs__item-text">Activity</span>
                  </button>
                </li>
              </ul>
              <button
                class="pf-v5-c-tabs__scroll-button"
                type="button"
                aria-label="Scroll right"
              >
                <i class="fas fa-angle-right" aria-hidden="true"></i>
              </button>
            </div>
          </div>

          <!-- Tab content -->
          <div class="pf-v5-c-drawer__body">
            <section
              class="pf-v5-c-tab-content"
              id="primary-detail-inline-modifier-example-drawer-tabs-tab1-panel"
              aria-labelledby="primary-detail-inline-modifier-example-drawer-tabs-tab1-link"
              role="tabpanel"
              tabindex="0"
            >
              <div class="pf-v5-c-tab-content__body">
                <div class="pf-v5-l-flex pf-m-column pf-m-space-items-lg">
                  <div class="pf-v5-l-flex__item">
                    <p>The content of the drawer really is up to you. It could have form fields, definition lists, text lists, labels, charts, progress bars, etc. Spacing recommendation is 24px margins. You can put tabs in here, and can also make the drawer scrollable.</p>
                  </div>
                  <div class="pf-v5-l-flex__item">
                    <div
                      class="pf-v5-c-progress pf-m-sm"
                      id="primary-detail-inline-modifier-example-drawer-progress-example1"
                    >
                      <div
                        class="pf-v5-c-progress__description"
                        id="primary-detail-inline-modifier-example-drawer-progress-example1-description"
                      >Capacity</div>
                      <div class="pf-v5-c-progress__status" aria-hidden="true">
                        <span class="pf-v5-c-progress__measure">33%</span>
                      </div>
                      <div
                        class="pf-v5-c-progress__bar"
                        role="progressbar"
                        aria-valuemin="0"
                        aria-valuemax="100"
                        aria-valuenow="33"
                        aria-labelledby="primary-detail-inline-modifier-example-drawer-progress-example1-description"
                        aria-label="Progress 1"
                      >
                        <div
                          class="pf-v5-c-progress__indicator"
                          style="width:33%;"
                        ></div>
                      </div>
                    </div>
                  </div>
                  <div class="pf-v5-l-flex__item">
                    <div
                      class="pf-v5-c-progress pf-m-sm"
                      id="primary-detail-inline-modifier-example-drawer-progress-example2"
                    >
                      <div
                        class="pf-v5-c-progress__description"
                        id="primary-detail-inline-modifier-example-drawer-progress-example2-description"
                      >Modules</div>
                      <div class="pf-v5-c-progress__status" aria-hidden="true">
                        <span class="pf-v5-c-progress__measure">66%</span>
                      </div>
                      <div
                        class="pf-v5-c-progress__bar"
                        role="progressbar"
                        aria-valuemin="0"
                        aria-valuemax="100"
                        aria-valuenow="66"
                        aria-labelledby="primary-detail-inline-modifier-example-drawer-progress-example2-description"
                        aria-label="Progress 2"
                      >
                        <div
                          class="pf-v5-c-progress__indicator"
                          style="width:66%;"
                        ></div>
                      </div>
                    </div>
                  </div>
                </div>
              </div>
            </section>
            <section
              class="pf-v5-c-tab-content"
              id="primary-detail-inline-modifier-example-drawer-tabs-tab2-panel"
              aria-labelledby="primary-detail-inline-modifier-example-drawer-tabs-tab2-link"
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
  </main>
</div>

```

## Documentation

This demo implements the drawer in context of the page component.
