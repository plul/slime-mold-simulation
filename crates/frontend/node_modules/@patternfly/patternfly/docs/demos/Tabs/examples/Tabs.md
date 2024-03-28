---
id: 'Tabs'
section: components
---## Examples

### Open tabs

```html isFullscreen
<div class="pf-v5-c-page" id="tabs-tables-and-tabs-example">
  <div class="pf-v5-c-skip-to-content">
    <a
      class="pf-v5-c-button pf-m-primary"
      href="#main-content-tabs-tables-and-tabs-example"
    >Skip to content</a>
  </div>
  <header class="pf-v5-c-masthead" id="tabs-tables-and-tabs-example-masthead">
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
        id="tabs-tables-and-tabs-example-masthead-toolbar"
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
        id="tabs-tables-and-tabs-example-primary-nav"
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
    id="main-content-tabs-tables-and-tabs-example"
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

    <section class="pf-v5-c-page__main-tabs pf-m-limit-width">
      <div class="pf-v5-c-page__main-body">
        <div
          class="pf-v5-c-tabs pf-m-page-insets"
          role="region"
          id="tabs-tables-and-tabs-example-tabs"
        >
          <ul class="pf-v5-c-tabs__list" role="tablist">
            <li class="pf-v5-c-tabs__item pf-m-current" role="presentation">
              <button
                type="button"
                class="pf-v5-c-tabs__link"
                role="tab"
                aria-controls="tabs-tables-and-tabs-example-tabs-details-panel"
                id="tabs-tables-and-tabs-example-tabs-details-link"
              >
                <span class="pf-v5-c-tabs__item-text">Details</span>
              </button>
            </li>
            <li class="pf-v5-c-tabs__item" role="presentation">
              <button
                type="button"
                class="pf-v5-c-tabs__link"
                role="tab"
                aria-controls="tabs-tables-and-tabs-example-tabs-yaml-panel"
                id="tabs-tables-and-tabs-example-tabs-yaml-link"
              >
                <span class="pf-v5-c-tabs__item-text">YAML</span>
              </button>
            </li>
            <li class="pf-v5-c-tabs__item" role="presentation">
              <button
                type="button"
                class="pf-v5-c-tabs__link"
                role="tab"
                aria-controls="tabs-tables-and-tabs-example-tabs-environment-panel"
                id="tabs-tables-and-tabs-example-tabs-environment-link"
              >
                <span class="pf-v5-c-tabs__item-text">Environment</span>
              </button>
            </li>
            <li class="pf-v5-c-tabs__item" role="presentation">
              <button
                type="button"
                class="pf-v5-c-tabs__link"
                role="tab"
                aria-controls="tabs-tables-and-tabs-example-tabs-logs-panel"
                id="tabs-tables-and-tabs-example-tabs-logs-link"
              >
                <span class="pf-v5-c-tabs__item-text">Logs</span>
              </button>
            </li>
            <li class="pf-v5-c-tabs__item" role="presentation">
              <button
                type="button"
                class="pf-v5-c-tabs__link"
                role="tab"
                aria-controls="tabs-tables-and-tabs-example-tabs-events-panel"
                id="tabs-tables-and-tabs-example-tabs-events-link"
              >
                <span class="pf-v5-c-tabs__item-text">Events</span>
              </button>
            </li>
            <li class="pf-v5-c-tabs__item" role="presentation">
              <button
                type="button"
                class="pf-v5-c-tabs__link"
                role="tab"
                aria-controls="tabs-tables-and-tabs-example-tabs-terminal-panel"
                id="tabs-tables-and-tabs-example-tabs-terminal-link"
              >
                <span class="pf-v5-c-tabs__item-text">Terminal</span>
              </button>
            </li>
          </ul>
        </div>
      </div>
    </section>

    <section class="pf-v5-c-page__main-section pf-m-limit-width pf-m-light">
      <div class="pf-v5-c-page__main-body">
        <section
          class="pf-v5-c-tab-content"
          aria-labelledby="tabs-tables-and-tabs-example-tabs-details-link"
          id="tabs-tables-and-tabs-example-tabs-details-panel"
          role="tabpanel"
          tabindex="0"
        >
          <div class="pf-v5-c-tab-content__body">
            <div class="pf-v5-l-flex pf-m-column">
              <div class="pf-v5-l-flex__item pf-m-spacer-lg">
                <h2
                  class="pf-v5-c-title pf-m-lg pf-v5-u-mt-sm"
                  id="-details-title"
                >Pod details</h2>
              </div>
              <div class="pf-v5-l-flex__item">
                <dl
                  class="pf-v5-c-description-list pf-m-2-col-on-lg"
                  aria-labelledby="-details-title"
                >
                  <div class="pf-v5-c-description-list__group">
                    <dt class="pf-v5-c-description-list__term">Name</dt>
                    <dd class="pf-v5-c-description-list__description">
                      <div
                        class="pf-v5-c-description-list__text"
                      >3scale-control-fccb6ddb9-phyqv9</div>
                    </dd>
                  </div>
                  <div class="pf-v5-c-description-list__group">
                    <dt class="pf-v5-c-description-list__term">Status</dt>
                    <dd class="pf-v5-c-description-list__description">
                      <div class="pf-v5-c-description-list__text">
                        <div class="pf-v5-l-flex pf-m-space-items-sm">
                          <div class="pf-v5-l-flex__item">
                            <i
                              class="fas fa-fw fa-check-circle"
                              aria-hidden="true"
                            ></i>
                          </div>
                          <div class="pf-v5-l-flex__item">Running</div>
                        </div>
                      </div>
                    </dd>
                  </div>
                  <div class="pf-v5-c-description-list__group">
                    <dt class="pf-v5-c-description-list__term">Namespace</dt>
                    <dd class="pf-v5-c-description-list__description">
                      <div class="pf-v5-c-description-list__text">
                        <div class="pf-v5-l-flex pf-m-space-items-sm">
                          <div class="pf-v5-l-flex__item">
                            <span class="pf-v5-c-label pf-m-cyan">
                              <span class="pf-v5-c-label__content">
                                <span class="pf-v5-c-label__text">NS</span>
                              </span>
                            </span>
                          </div>
                          <div class="pf-v5-l-flex__item">
                            <a href="#">knative-serving-ingress</a>
                          </div>
                        </div>
                      </div>
                    </dd>
                  </div>
                  <div class="pf-v5-c-description-list__group">
                    <dt class="pf-v5-c-description-list__term">Restart policy</dt>
                    <dd class="pf-v5-c-description-list__description">
                      <div class="pf-v5-c-description-list__text">Always restart</div>
                    </dd>
                  </div>
                  <div class="pf-v5-c-description-list__group">
                    <dt class="pf-v5-c-description-list__term">Labels</dt>
                    <dd class="pf-v5-c-description-list__description">
                      <div class="pf-v5-c-description-list__text">
                        <div class="pf-v5-c-label-group">
                          <div class="pf-v5-c-label-group__main">
                            <ul
                              class="pf-v5-c-label-group__list"
                              role="list"
                              aria-label="Group of labels"
                            >
                              <li class="pf-v5-c-label-group__list-item">
                                <span class="pf-v5-c-label pf-m-outline">
                                  <span class="pf-v5-c-label__content">
                                    <span
                                      class="pf-v5-c-label__text"
                                    >app=3scale-gateway</span>
                                  </span>
                                </span>
                              </li>
                              <li class="pf-v5-c-label-group__list-item">
                                <span class="pf-v5-c-label pf-m-outline">
                                  <span class="pf-v5-c-label__content">
                                    <span
                                      class="pf-v5-c-label__text"
                                    >pod-template-has=6747686899</span>
                                  </span>
                                </span>
                              </li>
                            </ul>
                          </div>
                        </div>
                      </div>
                    </dd>
                  </div>
                  <div class="pf-v5-c-description-list__group">
                    <dt
                      class="pf-v5-c-description-list__term"
                    >Active deadline seconds</dt>
                    <dd class="pf-v5-c-description-list__description">
                      <div class="pf-v5-c-description-list__text">Not configured</div>
                    </dd>
                  </div>
                  <div class="pf-v5-c-description-list__group">
                    <dt class="pf-v5-c-description-list__term">Tolerations</dt>
                    <dd class="pf-v5-c-description-list__description">
                      <div class="pf-v5-c-description-list__text">stuff</div>
                    </dd>
                  </div>
                  <div class="pf-v5-c-description-list__group">
                    <dt class="pf-v5-c-description-list__term">Pod IP</dt>
                    <dd class="pf-v5-c-description-list__description">
                      <div class="pf-v5-c-description-list__text">10.345.2.197</div>
                    </dd>
                  </div>
                  <div class="pf-v5-c-description-list__group">
                    <dt class="pf-v5-c-description-list__term">Annotations</dt>
                    <dd class="pf-v5-c-description-list__description">
                      <div class="pf-v5-c-description-list__text">stuff</div>
                    </dd>
                  </div>
                  <div class="pf-v5-c-description-list__group">
                    <dt class="pf-v5-c-description-list__term">Node</dt>
                    <dd class="pf-v5-c-description-list__description">
                      <div class="pf-v5-c-description-list__text">
                        <div class="pf-v5-l-flex pf-m-space-items-sm">
                          <div class="pf-v5-l-flex__item">
                            <span class="pf-v5-c-label pf-m-purple">
                              <span class="pf-v5-c-label__content">
                                <span class="pf-v5-c-label__text">N</span>
                              </span>
                            </span>
                          </div>
                          <div
                            class="pf-v5-l-flex__item"
                          >ip-10-0-233-118.us-east-2.computer.external</div>
                        </div>
                      </div>
                    </dd>
                  </div>
                  <div class="pf-v5-c-description-list__group">
                    <dt class="pf-v5-c-description-list__term">Created at</dt>
                    <dd class="pf-v5-c-description-list__description">
                      <div class="pf-v5-c-description-list__text">
                        <time>Oct 15, 1:51 pm</time>
                      </div>
                    </dd>
                  </div>
                </dl>
              </div>
            </div>
          </div>
        </section>
        <section
          class="pf-v5-c-tab-content"
          aria-labelledby="tabs-tables-and-tabs-example-tabs-yaml-link"
          id="tabs-tables-and-tabs-example-tabs-yaml-panel"
          role="tabpanel"
          tabindex="0"
          hidden
        >
          <div class="pf-v5-c-tab-content__body">YAML panel</div>
        </section>
        <section
          class="pf-v5-c-tab-content"
          aria-labelledby="tabs-tables-and-tabs-example-tabs-environment-link"
          id="tabs-tables-and-tabs-example-tabs-environment-panel"
          role="tabpanel"
          tabindex="0"
          hidden
        >
          <div class="pf-v5-c-tab-content__body">Environment panel</div>
        </section>
        <section
          class="pf-v5-c-tab-content"
          aria-labelledby="tabs-tables-and-tabs-example-tabs-logs-link"
          id="tabs-tables-and-tabs-example-tabs-logs-panel"
          role="tabpanel"
          tabindex="0"
          hidden
        >
          <div class="pf-v5-c-tab-content__body">Logs panel</div>
        </section>
        <section
          class="pf-v5-c-tab-content"
          aria-labelledby="tabs-tables-and-tabs-example-tabs-events-link"
          id="tabs-tables-and-tabs-example-tabs-events-panel"
          role="tabpanel"
          tabindex="0"
          hidden
        >
          <div class="pf-v5-c-tab-content__body">Events panel</div>
        </section>
        <section
          class="pf-v5-c-tab-content"
          aria-labelledby="tabs-tables-and-tabs-example-tabs-terminal-link"
          id="tabs-tables-and-tabs-example-tabs-terminal-panel"
          role="tabpanel"
          tabindex="0"
          hidden
        >
          <div class="pf-v5-c-tab-content__body">Terminal panel</div>
        </section>
      </div>
    </section>
  </main>
</div>

```

### Open tabs with secondary tabs

```html isFullscreen
<div class="pf-v5-c-page" id="tabs-tables-and-tabs-example">
  <div class="pf-v5-c-skip-to-content">
    <a
      class="pf-v5-c-button pf-m-primary"
      href="#main-content-tabs-tables-and-tabs-example"
    >Skip to content</a>
  </div>
  <header class="pf-v5-c-masthead" id="tabs-tables-and-tabs-example-masthead">
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
        id="tabs-tables-and-tabs-example-masthead-toolbar"
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
        id="tabs-tables-and-tabs-example-primary-nav"
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
    id="main-content-tabs-tables-and-tabs-example"
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
    <section class="pf-v5-c-page__main-tabs pf-m-limit-width">
      <div class="pf-v5-c-page__main-body">
        <div
          class="pf-v5-c-tabs pf-m-page-insets"
          role="region"
          id="tabs-tables-and-tabs-example-tabs"
        >
          <ul class="pf-v5-c-tabs__list" role="tablist">
            <li class="pf-v5-c-tabs__item pf-m-current" role="presentation">
              <button
                type="button"
                class="pf-v5-c-tabs__link"
                role="tab"
                aria-controls="tabs-tables-and-tabs-example-tabs-details-panel"
                id="tabs-tables-and-tabs-example-tabs-details-link"
              >
                <span class="pf-v5-c-tabs__item-text">Details</span>
              </button>
            </li>
            <li class="pf-v5-c-tabs__item" role="presentation">
              <button
                type="button"
                class="pf-v5-c-tabs__link"
                role="tab"
                aria-controls="tabs-tables-and-tabs-example-tabs-yaml-panel"
                id="tabs-tables-and-tabs-example-tabs-yaml-link"
              >
                <span class="pf-v5-c-tabs__item-text">YAML</span>
              </button>
            </li>
            <li class="pf-v5-c-tabs__item" role="presentation">
              <button
                type="button"
                class="pf-v5-c-tabs__link"
                role="tab"
                aria-controls="tabs-tables-and-tabs-example-tabs-environment-panel"
                id="tabs-tables-and-tabs-example-tabs-environment-link"
              >
                <span class="pf-v5-c-tabs__item-text">Environment</span>
              </button>
            </li>
            <li class="pf-v5-c-tabs__item" role="presentation">
              <button
                type="button"
                class="pf-v5-c-tabs__link"
                role="tab"
                aria-controls="tabs-tables-and-tabs-example-tabs-logs-panel"
                id="tabs-tables-and-tabs-example-tabs-logs-link"
              >
                <span class="pf-v5-c-tabs__item-text">Logs</span>
              </button>
            </li>
            <li class="pf-v5-c-tabs__item" role="presentation">
              <button
                type="button"
                class="pf-v5-c-tabs__link"
                role="tab"
                aria-controls="tabs-tables-and-tabs-example-tabs-events-panel"
                id="tabs-tables-and-tabs-example-tabs-events-link"
              >
                <span class="pf-v5-c-tabs__item-text">Events</span>
              </button>
            </li>
            <li class="pf-v5-c-tabs__item" role="presentation">
              <button
                type="button"
                class="pf-v5-c-tabs__link"
                role="tab"
                aria-controls="tabs-tables-and-tabs-example-tabs-terminal-panel"
                id="tabs-tables-and-tabs-example-tabs-terminal-link"
              >
                <span class="pf-v5-c-tabs__item-text">Terminal</span>
              </button>
            </li>
          </ul>
        </div>
      </div>
    </section>
    <section
      class="pf-v5-c-page__main-section pf-m-limit-width pf-m-light pf-m-no-padding"
    >
      <div class="pf-v5-c-page__main-body">
        <div
          class="pf-v5-c-tabs pf-m-secondary pf-m-page-insets"
          role="region"
          id="tabs-tables-and-tabs-example-tabs-secondary"
        >
          <ul class="pf-v5-c-tabs__list" role="tablist">
            <li class="pf-v5-c-tabs__item pf-m-current" role="presentation">
              <button
                type="button"
                class="pf-v5-c-tabs__link"
                role="tab"
                aria-controls="tabs-tables-and-tabs-example-tabs-secondary-pod-info-panel"
                id="tabs-tables-and-tabs-example-tabs-secondary-pod-info-link"
              >
                <span class="pf-v5-c-tabs__item-text">Pod information</span>
              </button>
            </li>
            <li class="pf-v5-c-tabs__item" role="presentation">
              <button
                type="button"
                class="pf-v5-c-tabs__link"
                role="tab"
                aria-controls="tabs-tables-and-tabs-example-tabs-secondary-editable-aspects-panel"
                id="tabs-tables-and-tabs-example-tabs-secondary-editable-aspects-link"
              >
                <span class="pf-v5-c-tabs__item-text">Editable Aspects</span>
              </button>
            </li>
          </ul>
        </div>
        <section
          class="pf-v5-c-tab-content"
          aria-labelledby="tabs-tables-and-tabs-example-tabs-details-link"
          id="tabs-tables-and-tabs-example-tabs-details-panel"
          role="tabpanel"
          tabindex="0"
        >
          <div class="pf-v5-c-tab-content__body pf-m-padding">
            <section
              class="pf-v5-c-tab-content"
              aria-labelledby="tabs-tables-and-tabs-example-tabs-secondary-pod-info-link"
              id="tabs-tables-and-tabs-example-tabs-secondary-pod-info-panel"
              role="tabpanel"
              tabindex="0"
            >
              <div class="pf-v5-c-tab-content__body">
                <div class="pf-v5-l-flex pf-m-column">
                  <div class="pf-v5-l-flex__item">
                    <dl
                      class="pf-v5-c-description-list pf-m-2-col-on-lg"
                      aria-label="Pod information list"
                    >
                      <div class="pf-v5-c-description-list__group">
                        <dt class="pf-v5-c-description-list__term">Name</dt>
                        <dd class="pf-v5-c-description-list__description">
                          <div
                            class="pf-v5-c-description-list__text"
                          >3scale-control-fccb6ddb9-phyqv9</div>
                        </dd>
                      </div>
                      <div class="pf-v5-c-description-list__group">
                        <dt class="pf-v5-c-description-list__term">Status</dt>
                        <dd class="pf-v5-c-description-list__description">
                          <div class="pf-v5-c-description-list__text">
                            <div class="pf-v5-l-flex pf-m-space-items-sm">
                              <div class="pf-v5-l-flex__item">
                                <i
                                  class="fas fa-fw fa-check-circle"
                                  aria-hidden="true"
                                ></i>
                              </div>
                              <div class="pf-v5-l-flex__item">Running</div>
                            </div>
                          </div>
                        </dd>
                      </div>
                      <div class="pf-v5-c-description-list__group">
                        <dt class="pf-v5-c-description-list__term">Namespace</dt>
                        <dd class="pf-v5-c-description-list__description">
                          <div class="pf-v5-c-description-list__text">
                            <div class="pf-v5-l-flex pf-m-space-items-sm">
                              <div class="pf-v5-l-flex__item">
                                <span class="pf-v5-c-label pf-m-cyan">
                                  <span class="pf-v5-c-label__content">
                                    <span class="pf-v5-c-label__text">NS</span>
                                  </span>
                                </span>
                              </div>
                              <div class="pf-v5-l-flex__item">
                                <a href="#">knative-serving-ingress</a>
                              </div>
                            </div>
                          </div>
                        </dd>
                      </div>
                      <div class="pf-v5-c-description-list__group">
                        <dt
                          class="pf-v5-c-description-list__term"
                        >Restart policy</dt>
                        <dd class="pf-v5-c-description-list__description">
                          <div
                            class="pf-v5-c-description-list__text"
                          >Always restart</div>
                        </dd>
                      </div>
                      <div class="pf-v5-c-description-list__group">
                        <dt class="pf-v5-c-description-list__term">Pod IP</dt>
                        <dd class="pf-v5-c-description-list__description">
                          <div
                            class="pf-v5-c-description-list__text"
                          >10.345.2.197</div>
                        </dd>
                      </div>
                      <div class="pf-v5-c-description-list__group">
                        <dt
                          class="pf-v5-c-description-list__term"
                        >Active deadline seconds</dt>
                        <dd class="pf-v5-c-description-list__description">
                          <div
                            class="pf-v5-c-description-list__text"
                          >Not configured</div>
                        </dd>
                      </div>
                      <div class="pf-v5-c-description-list__group">
                        <dt class="pf-v5-c-description-list__term">Created at</dt>
                        <dd class="pf-v5-c-description-list__description">
                          <div class="pf-v5-c-description-list__text">
                            <time>Oct 15, 1:51 pm</time>
                          </div>
                        </dd>
                      </div>
                      <div class="pf-v5-c-description-list__group">
                        <dt class="pf-v5-c-description-list__term">Node</dt>
                        <dd class="pf-v5-c-description-list__description">
                          <div class="pf-v5-c-description-list__text">
                            <div class="pf-v5-l-flex pf-m-space-items-sm">
                              <div class="pf-v5-l-flex__item">
                                <span class="pf-v5-c-label pf-m-purple">
                                  <span class="pf-v5-c-label__content">
                                    <span class="pf-v5-c-label__text">N</span>
                                  </span>
                                </span>
                              </div>
                              <div
                                class="pf-v5-l-flex__item"
                              >ip-10-0-233-118.us-east-2.computer.external</div>
                            </div>
                          </div>
                        </dd>
                      </div>
                    </dl>
                  </div>
                </div>
              </div>
            </section>
            <section
              class="pf-v5-c-tab-content"
              aria-labelledby="tabs-tables-and-tabs-example-tabs-secondary-editable-aspects-link"
              id="tabs-tables-and-tabs-example-tabs-secondary-editable-aspects-panel"
              role="tabpanel"
              tabindex="0"
              hidden
            >
              <div class="pf-v5-c-tab-content__body">Editable aspects panel</div>
            </section>
          </div>
        </section>
        <section
          class="pf-v5-c-tab-content"
          aria-labelledby="tabs-tables-and-tabs-example-tabs-yaml-link"
          id="tabs-tables-and-tabs-example-tabs-yaml-panel"
          role="tabpanel"
          tabindex="0"
          hidden
        >
          <div class="pf-v5-c-tab-content__body pf-m-padding">YAML panel</div>
        </section>
        <section
          class="pf-v5-c-tab-content"
          aria-labelledby="tabs-tables-and-tabs-example-tabs-environment-link"
          id="tabs-tables-and-tabs-example-tabs-environment-panel"
          role="tabpanel"
          tabindex="0"
          hidden
        >
          <div class="pf-v5-c-tab-content__body pf-m-padding">Environment panel</div>
        </section>
        <section
          class="pf-v5-c-tab-content"
          aria-labelledby="tabs-tables-and-tabs-example-tabs-logs-link"
          id="tabs-tables-and-tabs-example-tabs-logs-panel"
          role="tabpanel"
          tabindex="0"
          hidden
        >
          <div class="pf-v5-c-tab-content__body pf-m-padding">Logs panel</div>
        </section>
        <section
          class="pf-v5-c-tab-content"
          aria-labelledby="tabs-tables-and-tabs-example-tabs-events-link"
          id="tabs-tables-and-tabs-example-tabs-events-panel"
          role="tabpanel"
          tabindex="0"
          hidden
        >
          <div class="pf-v5-c-tab-content__body pf-m-padding">Events panel</div>
        </section>
        <section
          class="pf-v5-c-tab-content"
          aria-labelledby="tabs-tables-and-tabs-example-tabs-terminal-link"
          id="tabs-tables-and-tabs-example-tabs-terminal-panel"
          role="tabpanel"
          tabindex="0"
          hidden
        >
          <div class="pf-v5-c-tab-content__body pf-m-padding">Terminal panel</div>
        </section>
      </div>
    </section>
  </main>
</div>

```

### Nested tabs

```html isFullscreen
<div class="pf-v5-c-page" id="nested-tabs-example">
  <div class="pf-v5-c-skip-to-content">
    <a
      class="pf-v5-c-button pf-m-primary"
      href="#main-content-nested-tabs-example"
    >Skip to content</a>
  </div>
  <header class="pf-v5-c-masthead" id="nested-tabs-example-masthead">
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
        id="nested-tabs-example-masthead-toolbar"
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
        id="nested-tabs-example-primary-nav"
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
    id="main-content-nested-tabs-example"
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
    <section class="pf-v5-c-page__main-tabs pf-m-limit-width">
      <div class="pf-v5-c-page__main-body">
        <div
          class="pf-v5-c-tabs pf-m-page-insets"
          role="region"
          id="nested-tabs-example-tabs-tabs"
        >
          <ul class="pf-v5-c-tabs__list" role="tablist">
            <li class="pf-v5-c-tabs__item pf-m-current" role="presentation">
              <button
                type="button"
                class="pf-v5-c-tabs__link"
                role="tab"
                aria-controls="nested-tabs-example-tabs-tabs-cluster-1-panel"
                id="nested-tabs-example-tabs-tabs-cluster-1-link"
              >
                <span class="pf-v5-c-tabs__item-text">Cluster 1</span>
              </button>
            </li>
            <li class="pf-v5-c-tabs__item" role="presentation">
              <button
                type="button"
                class="pf-v5-c-tabs__link"
                role="tab"
                aria-controls="nested-tabs-example-tabs-tabs-cluster-2-panel"
                id="nested-tabs-example-tabs-tabs-cluster-2-link"
              >
                <span class="pf-v5-c-tabs__item-text">Cluster 2</span>
              </button>
            </li>
          </ul>
        </div>
      </div>
    </section>
    <section class="pf-v5-c-page__main-section pf-m-limit-width">
      <div class="pf-v5-c-page__main-body">
        <section
          class="pf-v5-c-tab-content"
          aria-labelledby="nested-tabs-example-tabs-tabs-cluster-1-link"
          id="nested-tabs-example-tabs-tabs-cluster-1-panel"
          role="tabpanel"
          tabindex="0"
        >
          <div class="pf-v5-c-tab-content__body">
            <div class="pf-v5-l-grid pf-m-gutter">
              <div class="pf-v5-l-grid__item pf-m-6-col-on-md pf-m-8-col-on-xl">
                <div class="pf-v5-c-card pf-m-full-height">
                  <div class="pf-v5-c-card__header">
                    <h2 class="pf-v5-c-title pf-m-lg">Status</h2>
                  </div>
                  <div class="pf-v5-c-card__body">
                    <div class="pf-v5-l-flex pf-m-column">
                      <div class="pf-v5-l-flex__item">
                        <div
                          class="pf-v5-c-tabs pf-m-secondary"
                          role="region"
                          id="nested-tabs-example-tabs-tabs-subtabs"
                        >
                          <ul class="pf-v5-c-tabs__list" role="tablist">
                            <li
                              class="pf-v5-c-tabs__item pf-m-current"
                              role="presentation"
                            >
                              <button
                                type="button"
                                class="pf-v5-c-tabs__link"
                                role="tab"
                                aria-controls="nested-tabs-example-tabs-tabs-subtabs-cluster-panel"
                                id="nested-tabs-example-tabs-tabs-subtabs-cluster-link"
                              >
                                <span class="pf-v5-c-tabs__item-text">Cluster</span>
                              </button>
                            </li>
                            <li class="pf-v5-c-tabs__item" role="presentation">
                              <button
                                type="button"
                                class="pf-v5-c-tabs__link"
                                role="tab"
                                aria-controls="nested-tabs-example-tabs-tabs-subtabs-control-plane-panel"
                                id="nested-tabs-example-tabs-tabs-subtabs-control-plane-link"
                              >
                                <span
                                  class="pf-v5-c-tabs__item-text"
                                >Control plane</span>
                              </button>
                            </li>
                            <li class="pf-v5-c-tabs__item" role="presentation">
                              <button
                                type="button"
                                class="pf-v5-c-tabs__link"
                                role="tab"
                                aria-controls="nested-tabs-example-tabs-tabs-subtabs-operators-panel"
                                id="nested-tabs-example-tabs-tabs-subtabs-operators-link"
                              >
                                <span class="pf-v5-c-tabs__item-text">Operators</span>
                              </button>
                            </li>
                            <li class="pf-v5-c-tabs__item" role="presentation">
                              <button
                                type="button"
                                class="pf-v5-c-tabs__link"
                                role="tab"
                                aria-controls="nested-tabs-example-tabs-tabs-subtabs-virtualization-panel"
                                id="nested-tabs-example-tabs-tabs-subtabs-virtualization-link"
                              >
                                <span
                                  class="pf-v5-c-tabs__item-text"
                                >Virtualization</span>
                              </button>
                            </li>
                          </ul>
                        </div>
                      </div>
                      <div class="pf-v5-l-flex__item">
                        <section
                          class="pf-v5-c-tab-content"
                          aria-labelledby="nested-tabs-example-tabs-tabs-subtabs-cluster-link"
                          id="nested-tabs-example-tabs-tabs-subtabs-cluster-panel"
                          role="tabpanel"
                          tabindex="0"
                        >
                          <div class="pf-v5-c-tab-content__body">
                            <div class="pf-v5-c-content">
                              <p>Lorem ipsum dolor sit amet, consectetur adipiscing elit. Fusce in odio porttitor, feugiat risus in, feugiat arcu. Nullam euismod enim eget fringilla condimentum. Maecenas tincidunt et metus id aliquet. Integer et fermentum purus. Nulla tempor velit arcu, vitae semper purus iaculis at. Sed malesuada auctor luctus. Pellentesque et leo urna. Aliquam vitae felis congue lacus mattis fringilla. Nullam et ultricies erat, sed dignissim elit. Cras mattis pulvinar aliquam. In ac est nulla. Pellentesque fermentum nibh ac sapien porta, ut congue orci aliquam. Sed nisl est, tempor eu pharetra eget, ullamcorper ut augue. Vestibulum eleifend libero eu nulla cursus lacinia.</p>
                            </div>
                          </div>
                        </section>
                        <section
                          class="pf-v5-c-tab-content"
                          aria-labelledby="nested-tabs-example-tabs-tabs-subtabs-control-plane-link"
                          id="nested-tabs-example-tabs-tabs-subtabs-control-plane-panel"
                          role="tabpanel"
                          tabindex="0"
                          hidden
                        >
                          <div
                            class="pf-v5-c-tab-content__body"
                          >Control plane content</div>
                        </section>
                        <section
                          class="pf-v5-c-tab-content"
                          aria-labelledby="nested-tabs-example-tabs-tabs-subtabs-operators-link"
                          id="nested-tabs-example-tabs-tabs-subtabs-operators-panel"
                          role="tabpanel"
                          tabindex="0"
                          hidden
                        >
                          <div
                            class="pf-v5-c-tab-content__body"
                          >Operators content</div>
                        </section>
                        <section
                          class="pf-v5-c-tab-content"
                          aria-labelledby="nested-tabs-example-tabs-tabs-subtabs-virtualization-link"
                          id="nested-tabs-example-tabs-tabs-subtabs-virtualization-panel"
                          role="tabpanel"
                          tabindex="0"
                          hidden
                        >
                          <div
                            class="pf-v5-c-tab-content__body"
                          >Virtualization content</div>
                        </section>
                      </div>
                    </div>
                  </div>
                </div>
              </div>
              <div class="pf-v5-l-grid__item pf-m-6-col-on-md pf-m-4-col-on-xl">
                <div class="pf-v5-l-flex pf-m-column pf-v5-u-h-100">
                  <div class="pf-v5-l-flex__item pf-m-flex-1">
                    <div class="pf-v5-c-card pf-m-full-height">
                      <div class="pf-v5-c-card__header">
                        <h2 class="pf-v5-c-title pf-m-lg">Title of card</h2>
                      </div>
                    </div>
                  </div>
                  <div class="pf-v5-l-flex__item pf-m-flex-1">
                    <div class="pf-v5-c-card pf-m-full-height">
                      <div class="pf-v5-c-card__header">
                        <h2 class="pf-v5-c-title pf-m-lg">Title of card</h2>
                      </div>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </section>
        <section
          class="pf-v5-c-tab-content"
          aria-labelledby="nested-tabs-example-tabs-tabs-cluster-2-link"
          id="nested-tabs-example-tabs-tabs-cluster-2-panel"
          role="tabpanel"
          tabindex="0"
          hidden
        >
          <div class="pf-v5-c-tab-content__body">
            <div class="pf-v5-c-content">
              <p>Cluster 2 content</p>
            </div>
          </div>
        </section>
      </div>
    </section>
  </main>
</div>

```

### Tables and tabs

```html isFullscreen
<div class="pf-v5-c-page" id="table-tabs-example">
  <div class="pf-v5-c-skip-to-content">
    <a
      class="pf-v5-c-button pf-m-primary"
      href="#main-content-table-tabs-example"
    >Skip to content</a>
  </div>
  <header class="pf-v5-c-masthead" id="table-tabs-example-masthead">
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
        id="table-tabs-example-masthead-toolbar"
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
        id="table-tabs-example-primary-nav"
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
    id="main-content-table-tabs-example"
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
    <section class="pf-v5-c-page__main-section pf-m-no-padding pf-m-light">
      <div
        class="pf-v5-c-toolbar pf-m-page-insets"
        id="table-tabs-example-tabs-toolbar"
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
                  aria-controls="table-tabs-example-tabs-toolbar-expandable-content"
                >
                  <i class="fas fa-filter" aria-hidden="true"></i>
                </button>
              </div>

              <div class="pf-v5-c-toolbar__item">
                <div class="pf-v5-c-select">
                  <span
                    id="table-tabs-example-tabs-toolbar-select-checkbox-status-label"
                    hidden
                  >Choose many</span>

                  <button
                    class="pf-v5-c-select__toggle"
                    type="button"
                    id="table-tabs-example-tabs-toolbar-select-checkbox-status-toggle"
                    aria-haspopup="true"
                    aria-expanded="false"
                    aria-labelledby="table-tabs-example-tabs-toolbar-select-checkbox-status-label table-tabs-example-tabs-toolbar-select-checkbox-status-toggle"
                  >
                    <div class="pf-v5-c-select__toggle-wrapper">
                      <span class="pf-v5-c-select__toggle-text">Name</span>
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
                        for="table-tabs-example-tabs-toolbar-select-checkbox-status-active"
                      >
                        <input
                          class="pf-v5-c-check__input"
                          type="checkbox"
                          id="table-tabs-example-tabs-toolbar-select-checkbox-status-active"
                          name="table-tabs-example-tabs-toolbar-select-checkbox-status-active"
                        />

                        <span class="pf-v5-c-check__label">Active</span>
                        <span
                          class="pf-v5-c-check__description"
                        >This is a description</span>
                      </label>
                      <label
                        class="pf-v5-c-check pf-v5-c-select__menu-item pf-m-description"
                        for="table-tabs-example-tabs-toolbar-select-checkbox-status-canceled"
                      >
                        <input
                          class="pf-v5-c-check__input"
                          type="checkbox"
                          id="table-tabs-example-tabs-toolbar-select-checkbox-status-canceled"
                          name="table-tabs-example-tabs-toolbar-select-checkbox-status-canceled"
                        />

                        <span class="pf-v5-c-check__label">Canceled</span>
                        <span
                          class="pf-v5-c-check__description"
                        >This is a really long description that describes the menu item. This is a really long description that describes the menu item.</span>
                      </label>
                      <label
                        class="pf-v5-c-check pf-v5-c-select__menu-item"
                        for="table-tabs-example-tabs-toolbar-select-checkbox-status-paused"
                      >
                        <input
                          class="pf-v5-c-check__input"
                          type="checkbox"
                          id="table-tabs-example-tabs-toolbar-select-checkbox-status-paused"
                          name="table-tabs-example-tabs-toolbar-select-checkbox-status-paused"
                        />

                        <span class="pf-v5-c-check__label">Paused</span>
                      </label>
                      <label
                        class="pf-v5-c-check pf-v5-c-select__menu-item"
                        for="table-tabs-example-tabs-toolbar-select-checkbox-status-warning"
                      >
                        <input
                          class="pf-v5-c-check__input"
                          type="checkbox"
                          id="table-tabs-example-tabs-toolbar-select-checkbox-status-warning"
                          name="table-tabs-example-tabs-toolbar-select-checkbox-status-warning"
                        />

                        <span class="pf-v5-c-check__label">Warning</span>
                      </label>
                      <label
                        class="pf-v5-c-check pf-v5-c-select__menu-item"
                        for="table-tabs-example-tabs-toolbar-select-checkbox-status-restarted"
                      >
                        <input
                          class="pf-v5-c-check__input"
                          type="checkbox"
                          id="table-tabs-example-tabs-toolbar-select-checkbox-status-restarted"
                          name="table-tabs-example-tabs-toolbar-select-checkbox-status-restarted"
                        />

                        <span class="pf-v5-c-check__label">Restarted</span>
                      </label>
                    </fieldset>
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
              id="table-tabs-example-tabs-toolbar-overflow-menu"
            >
              <div
                class="pf-v5-c-overflow-menu__content pf-v5-u-display-none pf-v5-u-display-flex-on-lg"
              >
                <div class="pf-v5-c-overflow-menu__group pf-m-button-group">
                  <div class="pf-v5-c-overflow-menu__item">
                    <button
                      class="pf-v5-c-button pf-m-primary"
                      type="button"
                    >Generate</button>
                  </div>

                  <div class="pf-v5-c-overflow-menu__item">
                    <button
                      class="pf-v5-c-button pf-m-secondary"
                      type="button"
                    >Deploy</button>
                  </div>
                </div>
              </div>
              <div class="pf-v5-c-overflow-menu__control">
                <div class="pf-v5-c-dropdown">
                  <button
                    class="pf-v5-c-button pf-v5-c-dropdown__toggle pf-m-plain"
                    type="button"
                    id="table-tabs-example-tabs-toolbar-overflow-menu-dropdown-toggle"
                    aria-label="Dropdown with additional options"
                    aria-expanded="false"
                  >
                    <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
                  </button>
                  <ul
                    class="pf-v5-c-dropdown__menu"
                    role="menu"
                    aria-labelledby="table-tabs-example-tabs-toolbar-overflow-menu-dropdown-toggle"
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
          </div>

          <div
            class="pf-v5-c-toolbar__expandable-content pf-m-hidden"
            id="table-tabs-example-tabs-toolbar-expandable-content"
            hidden
          ></div>
        </div>
      </div>
      <hr class="pf-v5-c-divider" />
      <div class="pf-v5-c-drawer pf-m-expanded pf-m-inline">
        <div class="pf-v5-c-drawer__main">
          <!-- Content -->
          <div class="pf-v5-c-drawer__content">
            <div class="pf-v5-c-drawer__body">
              <table
                class="pf-v5-c-table pf-m-grid-md"
                role="grid"
                aria-label="This is a table with checkboxes"
                id="table-tabs-example-table"
              >
                <thead class="pf-v5-c-table__thead">
                  <tr class="pf-v5-c-table__tr" role="row">
                    <td
                      class="pf-v5-c-table__td pf-v5-c-table__check"
                      role="cell"
                    >
                      <div class="pf-v5-c-check pf-m-standalone">
                        <input
                          class="pf-v5-c-check__input"
                          type="checkbox"
                          name="check-all"
                          aria-label="Select all rows"
                        />
                      </div>
                    </td>
                    <th
                      class="pf-v5-c-table__th"
                      role="columnheader"
                      scope="col"
                    >Repositories</th>
                    <th
                      class="pf-v5-c-table__th"
                      role="columnheader"
                      scope="col"
                    >Branches</th>
                    <th
                      class="pf-v5-c-table__th"
                      role="columnheader"
                      scope="col"
                    >Pull requests</th>
                    <th
                      class="pf-v5-c-table__th"
                      role="columnheader"
                      scope="col"
                    >Workspaces</th>
                    <th
                      class="pf-v5-c-table__th"
                      role="columnheader"
                      scope="col"
                    >Last commit</th>
                    <td class="pf-v5-c-table__td"></td>
                  </tr>
                </thead>

                <tbody class="pf-v5-c-table__tbody" role="rowgroup">
                  <tr class="pf-v5-c-table__tr" role="row">
                    <td
                      class="pf-v5-c-table__td pf-v5-c-table__check"
                      role="cell"
                    >
                      <div class="pf-v5-c-check pf-m-standalone">
                        <input
                          class="pf-v5-c-check__input"
                          type="checkbox"
                          name="checkrow1"
                          aria-labelledby="table-tabs-example-table-node1"
                        />
                      </div>
                    </td>
                    <th
                      class="pf-v5-c-table__th"
                      role="columnheader"
                      data-label="Repository name"
                    >
                      <div>
                        <div id="table-tabs-example-table-node1">Node 1</div>
                        <a href="#">siemur/test-space</a>
                      </div>
                    </th>
                    <td
                      class="pf-v5-c-table__td"
                      role="cell"
                      data-label="Branches"
                    >
                      <div class="pf-v5-l-flex pf-m-space-items-sm pf-m-nowrap">
                        <div class="pf-v5-l-flex__item">10</div>
                        <div class="pf-v5-l-flex__item">
                          <i class="fas fa-code-branch"></i>
                        </div>
                      </div>
                    </td>
                    <td
                      class="pf-v5-c-table__td"
                      role="cell"
                      data-label="Pull requests"
                    >
                      <div class="pf-v5-l-flex pf-m-space-items-sm pf-m-nowrap">
                        <div class="pf-v5-l-flex__item">25</div>
                        <div class="pf-v5-l-flex__item">
                          <i class="fas fa-code"></i>
                        </div>
                      </div>
                    </td>
                    <td
                      class="pf-v5-c-table__td"
                      role="cell"
                      data-label="Workspaces"
                    >
                      <div class="pf-v5-l-flex pf-m-space-items-sm pf-m-nowrap">
                        <div class="pf-v5-l-flex__item">5</div>
                        <div class="pf-v5-l-flex__item">
                          <i class="fas fa-cube"></i>
                        </div>
                      </div>
                    </td>
                    <td
                      class="pf-v5-c-table__td"
                      role="cell"
                      data-label="Last commit"
                    >2 days ago</td>
                    <td
                      class="pf-v5-c-table__td pf-v5-c-table__action"
                      role="cell"
                    >
                      <div
                        class="pf-v5-c-overflow-menu"
                        id="table-tabs-example-table-dropdown-kebab-1"
                      >
                        <div class="pf-v5-c-overflow-menu__control">
                          <div class="pf-v5-c-dropdown">
                            <button
                              class="pf-v5-c-button pf-v5-c-dropdown__toggle pf-m-plain"
                              type="button"
                              id="table-tabs-example-table-dropdown-kebab-1-dropdown-toggle"
                              aria-label="Dropdown for tabs table"
                              aria-expanded="false"
                            >
                              <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
                            </button>
                            <ul
                              class="pf-v5-c-dropdown__menu pf-m-align-right"
                              role="menu"
                              aria-labelledby="table-tabs-example-table-dropdown-kebab-1-dropdown-toggle"
                              hidden
                            >
                              <li role="none">
                                <button
                                  role="menuitem"
                                  class="pf-v5-c-dropdown__menu-item"
                                >Action Link</button>
                              </li>
                            </ul>
                          </div>
                        </div>
                      </div>
                    </td>
                  </tr>

                  <tr class="pf-v5-c-table__tr pf-m-selected" role="row">
                    <td
                      class="pf-v5-c-table__td pf-v5-c-table__check"
                      role="cell"
                    >
                      <div class="pf-v5-c-check pf-m-standalone">
                        <input
                          class="pf-v5-c-check__input"
                          type="checkbox"
                          name="checkrow2"
                          aria-labelledby="table-tabs-example-table-node2"
                        />
                      </div>
                    </td>
                    <th
                      class="pf-v5-c-table__th"
                      role="columnheader"
                      data-label="Repository name"
                    >
                      <div>
                        <div id="table-tabs-example-table-node2">Node 2</div>
                        <a href="#">siemur/test-space</a>
                      </div>
                    </th>
                    <td
                      class="pf-v5-c-table__td"
                      role="cell"
                      data-label="Branches"
                    >
                      <div class="pf-v5-l-flex pf-m-space-items-sm pf-m-nowrap">
                        <div class="pf-v5-l-flex__item">8</div>
                        <div class="pf-v5-l-flex__item">
                          <i class="fas fa-code-branch"></i>
                        </div>
                      </div>
                    </td>
                    <td
                      class="pf-v5-c-table__td"
                      role="cell"
                      data-label="Pull requests"
                    >
                      <div class="pf-v5-l-flex pf-m-space-items-sm pf-m-nowrap">
                        <div class="pf-v5-l-flex__item">30</div>
                        <div class="pf-v5-l-flex__item">
                          <i class="fas fa-code"></i>
                        </div>
                      </div>
                    </td>
                    <td
                      class="pf-v5-c-table__td"
                      role="cell"
                      data-label="Workspaces"
                    >
                      <div class="pf-v5-l-flex pf-m-space-items-sm pf-m-nowrap">
                        <div class="pf-v5-l-flex__item">2</div>
                        <div class="pf-v5-l-flex__item">
                          <i class="fas fa-cube"></i>
                        </div>
                      </div>
                    </td>
                    <td
                      class="pf-v5-c-table__td"
                      role="cell"
                      data-label="Last commit"
                    >2 days ago</td>
                    <td
                      class="pf-v5-c-table__td pf-v5-c-table__action"
                      role="cell"
                    >
                      <div
                        class="pf-v5-c-overflow-menu"
                        id="table-tabs-example-table-dropdown-kebab-2"
                      >
                        <div class="pf-v5-c-overflow-menu__control">
                          <div class="pf-v5-c-dropdown">
                            <button
                              class="pf-v5-c-button pf-v5-c-dropdown__toggle pf-m-plain"
                              type="button"
                              id="table-tabs-example-table-dropdown-kebab-2-dropdown-toggle"
                              aria-label="Dropdown for tabs table"
                              aria-expanded="false"
                            >
                              <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
                            </button>
                            <ul
                              class="pf-v5-c-dropdown__menu pf-m-align-right"
                              role="menu"
                              aria-labelledby="table-tabs-example-table-dropdown-kebab-2-dropdown-toggle"
                              hidden
                            >
                              <li role="none">
                                <button
                                  role="menuitem"
                                  class="pf-v5-c-dropdown__menu-item"
                                >Action Link</button>
                              </li>
                            </ul>
                          </div>
                        </div>
                      </div>
                    </td>
                  </tr>

                  <tr class="pf-v5-c-table__tr" role="row">
                    <td
                      class="pf-v5-c-table__td pf-v5-c-table__check"
                      role="cell"
                    >
                      <div class="pf-v5-c-check pf-m-standalone">
                        <input
                          class="pf-v5-c-check__input"
                          type="checkbox"
                          name="checkrow3"
                          aria-labelledby="table-tabs-example-table-node3"
                        />
                      </div>
                    </td>
                    <th
                      class="pf-v5-c-table__th"
                      role="columnheader"
                      data-label="Repository name"
                    >
                      <div>
                        <div id="table-tabs-example-table-node3">Node 3</div>
                        <a href="#">siemur/test-space</a>
                      </div>
                    </th>
                    <td
                      class="pf-v5-c-table__td"
                      role="cell"
                      data-label="Branches"
                    >
                      <div class="pf-v5-l-flex pf-m-space-items-sm pf-m-nowrap">
                        <div class="pf-v5-l-flex__item">12</div>
                        <div class="pf-v5-l-flex__item">
                          <i class="fas fa-code-branch"></i>
                        </div>
                      </div>
                    </td>
                    <td
                      class="pf-v5-c-table__td"
                      role="cell"
                      data-label="Pull requests"
                    >
                      <div class="pf-v5-l-flex pf-m-space-items-sm pf-m-nowrap">
                        <div class="pf-v5-l-flex__item">48</div>
                        <div class="pf-v5-l-flex__item">
                          <i class="fas fa-code"></i>
                        </div>
                      </div>
                    </td>
                    <td
                      class="pf-v5-c-table__td"
                      role="cell"
                      data-label="Workspaces"
                    >
                      <div class="pf-v5-l-flex pf-m-space-items-sm pf-m-nowrap">
                        <div class="pf-v5-l-flex__item">13</div>
                        <div class="pf-v5-l-flex__item">
                          <i class="fas fa-cube"></i>
                        </div>
                      </div>
                    </td>
                    <td
                      class="pf-v5-c-table__td"
                      role="cell"
                      data-label="Last commit"
                    >30 days ago</td>
                    <td
                      class="pf-v5-c-table__td pf-v5-c-table__action"
                      role="cell"
                    >
                      <div
                        class="pf-v5-c-overflow-menu"
                        id="table-tabs-example-table-dropdown-kebab-3"
                      >
                        <div class="pf-v5-c-overflow-menu__control">
                          <div class="pf-v5-c-dropdown">
                            <button
                              class="pf-v5-c-button pf-v5-c-dropdown__toggle pf-m-plain"
                              type="button"
                              id="table-tabs-example-table-dropdown-kebab-3-dropdown-toggle"
                              aria-label="Dropdown for tabs table"
                              aria-expanded="false"
                            >
                              <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
                            </button>
                            <ul
                              class="pf-v5-c-dropdown__menu pf-m-align-right"
                              role="menu"
                              aria-labelledby="table-tabs-example-table-dropdown-kebab-3-dropdown-toggle"
                              hidden
                            >
                              <li role="none">
                                <button
                                  role="menuitem"
                                  class="pf-v5-c-dropdown__menu-item"
                                >Action Link</button>
                              </li>
                            </ul>
                          </div>
                        </div>
                      </div>
                    </td>
                  </tr>

                  <tr class="pf-v5-c-table__tr" role="row">
                    <td
                      class="pf-v5-c-table__td pf-v5-c-table__check"
                      role="cell"
                    >
                      <div class="pf-v5-c-check pf-m-standalone">
                        <input
                          class="pf-v5-c-check__input"
                          type="checkbox"
                          name="checkrow4"
                          aria-labelledby="table-tabs-example-table-node4"
                        />
                      </div>
                    </td>
                    <th
                      class="pf-v5-c-table__th"
                      role="columnheader"
                      data-label="Repository name"
                    >
                      <div>
                        <div id="table-tabs-example-table-node4">Node 4</div>
                        <a href="#">siemur/test-space</a>
                      </div>
                    </th>
                    <td
                      class="pf-v5-c-table__td"
                      role="cell"
                      data-label="Branches"
                    >
                      <div class="pf-v5-l-flex pf-m-space-items-sm pf-m-nowrap">
                        <div class="pf-v5-l-flex__item">3</div>
                        <div class="pf-v5-l-flex__item">
                          <i class="fas fa-code-branch"></i>
                        </div>
                      </div>
                    </td>
                    <td
                      class="pf-v5-c-table__td"
                      role="cell"
                      data-label="Pull requests"
                    >
                      <div class="pf-v5-l-flex pf-m-space-items-sm pf-m-nowrap">
                        <div class="pf-v5-l-flex__item">8</div>
                        <div class="pf-v5-l-flex__item">
                          <i class="fas fa-code"></i>
                        </div>
                      </div>
                    </td>
                    <td
                      class="pf-v5-c-table__td"
                      role="cell"
                      data-label="Workspaces"
                    >
                      <div class="pf-v5-l-flex pf-m-space-items-sm pf-m-nowrap">
                        <div class="pf-v5-l-flex__item">20</div>
                        <div class="pf-v5-l-flex__item">
                          <i class="fas fa-cube"></i>
                        </div>
                      </div>
                    </td>
                    <td
                      class="pf-v5-c-table__td"
                      role="cell"
                      data-label="Last commit"
                    >8 days ago</td>
                    <td
                      class="pf-v5-c-table__td pf-v5-c-table__action"
                      role="cell"
                    >
                      <div
                        class="pf-v5-c-overflow-menu"
                        id="table-tabs-example-table-dropdown-kebab-4"
                      >
                        <div class="pf-v5-c-overflow-menu__control">
                          <div class="pf-v5-c-dropdown">
                            <button
                              class="pf-v5-c-button pf-v5-c-dropdown__toggle pf-m-plain"
                              type="button"
                              id="table-tabs-example-table-dropdown-kebab-4-dropdown-toggle"
                              aria-label="Dropdown for tabs table"
                              aria-expanded="false"
                            >
                              <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
                            </button>
                            <ul
                              class="pf-v5-c-dropdown__menu pf-m-align-right"
                              role="menu"
                              aria-labelledby="table-tabs-example-table-dropdown-kebab-4-dropdown-toggle"
                              hidden
                            >
                              <li role="none">
                                <button
                                  role="menuitem"
                                  class="pf-v5-c-dropdown__menu-item"
                                >Action Link</button>
                              </li>
                            </ul>
                          </div>
                        </div>
                      </div>
                    </td>
                  </tr>

                  <tr class="pf-v5-c-table__tr" role="row">
                    <td
                      class="pf-v5-c-table__td pf-v5-c-table__check"
                      role="cell"
                    >
                      <div class="pf-v5-c-check pf-m-standalone">
                        <input
                          class="pf-v5-c-check__input"
                          type="checkbox"
                          name="checkrow5"
                          aria-labelledby="table-tabs-example-table-node5"
                        />
                      </div>
                    </td>
                    <td
                      class="pf-v5-c-table__td"
                      role="cell"
                      data-label="Repository name"
                    >
                      <div>
                        <div id="table-tabs-example-table-node5">Node 5</div>
                        <a href="#">siemur/test-space</a>
                      </div>
                    </td>
                    <td
                      class="pf-v5-c-table__td"
                      role="cell"
                      data-label="Branches"
                    >
                      <div class="pf-v5-l-flex pf-m-space-items-sm pf-m-nowrap">
                        <div class="pf-v5-l-flex__item">34</div>
                        <div class="pf-v5-l-flex__item">
                          <i class="fas fa-code-branch"></i>
                        </div>
                      </div>
                    </td>
                    <td
                      class="pf-v5-c-table__td"
                      role="cell"
                      data-label="Pull requests"
                    >
                      <div class="pf-v5-l-flex pf-m-space-items-sm pf-m-nowrap">
                        <div class="pf-v5-l-flex__item">21</div>
                        <div class="pf-v5-l-flex__item">
                          <i class="fas fa-code"></i>
                        </div>
                      </div>
                    </td>
                    <td
                      class="pf-v5-c-table__td"
                      role="cell"
                      data-label="Workspaces"
                    >
                      <div class="pf-v5-l-flex pf-m-space-items-sm pf-m-nowrap">
                        <div class="pf-v5-l-flex__item">26</div>
                        <div class="pf-v5-l-flex__item">
                          <i class="fas fa-cube"></i>
                        </div>
                      </div>
                    </td>
                    <td
                      class="pf-v5-c-table__td"
                      role="cell"
                      data-label="Last commit"
                    >2 days ago</td>
                    <td
                      class="pf-v5-c-table__td pf-v5-c-table__action"
                      role="cell"
                    >
                      <div
                        class="pf-v5-c-overflow-menu"
                        id="table-tabs-example-table-dropdown-kebab-5"
                      >
                        <div class="pf-v5-c-overflow-menu__control">
                          <div class="pf-v5-c-dropdown">
                            <button
                              class="pf-v5-c-button pf-v5-c-dropdown__toggle pf-m-plain"
                              type="button"
                              id="table-tabs-example-table-dropdown-kebab-5-dropdown-toggle"
                              aria-label="Dropdown for tabs table"
                              aria-expanded="false"
                            >
                              <i class="fas fa-ellipsis-v" aria-hidden="true"></i>
                            </button>
                            <ul
                              class="pf-v5-c-dropdown__menu pf-m-align-right"
                              role="menu"
                              aria-labelledby="table-tabs-example-table-dropdown-kebab-5-dropdown-toggle"
                              hidden
                            >
                              <li role="none">
                                <button
                                  role="menuitem"
                                  class="pf-v5-c-dropdown__menu-item"
                                >Action Link</button>
                              </li>
                            </ul>
                          </div>
                        </div>
                      </div>
                    </td>
                  </tr>
                </tbody>
              </table>
            </div>
          </div>

          <!-- Panel -->
          <div class="pf-v5-c-drawer__panel pf-m-width-33 pf-m-width-33-on-xl">
            <div class="pf-v5-c-drawer__body">
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
                <div class="pf-v5-l-flex pf-m-column pf-m-space-items-sm">
                  <div class="pf-v5-l-flex__item">
                    <h2 class="pf-v5-c-title pf-m-lg" id="-drawer-label">Node 2</h2>
                  </div>
                  <div class="pf-v5-l-flex__item">
                    <a href="#">siemur/test-space</a>
                  </div>
                </div>
              </div>
            </div>
            <div class="pf-v5-c-drawer__body pf-m-no-padding">
              <div
                class="pf-v5-c-tabs pf-m-box pf-m-fill"
                role="region"
                id="-tabs"
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
                      aria-controls="-tabs-tab1-panel"
                      id="-tabs-tab1-link"
                    >
                      <span class="pf-v5-c-tabs__item-text">Overview</span>
                    </button>
                  </li>
                  <li class="pf-v5-c-tabs__item" role="presentation">
                    <button
                      type="button"
                      class="pf-v5-c-tabs__link"
                      role="tab"
                      aria-controls="-tabs-tab2-panel"
                      id="-tabs-tab2-link"
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
            <div class="pf-v5-c-drawer__body">
              <section
                class="pf-v5-c-tab-content"
                id="-tabs-tab1-panel"
                aria-labelledby="-tabs-tab1-link"
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
                        id="-progress-example1"
                      >
                        <div
                          class="pf-v5-c-progress__description"
                          id="-progress-example1-description"
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
                          aria-labelledby="-progress-example1-description"
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
                        id="-progress-example2"
                      >
                        <div
                          class="pf-v5-c-progress__description"
                          id="-progress-example2-description"
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
                          aria-labelledby="-progress-example2-description"
                          aria-label="Progress 2"
                        >
                          <div
                            class="pf-v5-c-progress__indicator"
                            style="width:66%;"
                          ></div>
                        </div>
                      </div>
                    </div>
                    <div class="pf-v5-l-flex pf-m-column">
                      <div class="pf-v5-l-flex__item">
                        <h3 class="pf-v5-c-title" id="-title">Tags</h3>
                      </div>
                      <div class="pf-v5-l-flex__item">
                        <div class="pf-v5-c-label-group">
                          <div class="pf-v5-c-label-group__main">
                            <ul
                              class="pf-v5-c-label-group__list"
                              role="list"
                              aria-label="Group of labels"
                            >
                              <li class="pf-v5-c-label-group__list-item">
                                <span class="pf-v5-c-label pf-m-outline">
                                  <span class="pf-v5-c-label__content">
                                    <span class="pf-v5-c-label__text">Tag 1</span>
                                  </span>
                                </span>
                              </li>
                              <li class="pf-v5-c-label-group__list-item">
                                <span class="pf-v5-c-label pf-m-outline">
                                  <span class="pf-v5-c-label__content">
                                    <span class="pf-v5-c-label__text">Tag 2</span>
                                  </span>
                                </span>
                              </li>
                              <li class="pf-v5-c-label-group__list-item">
                                <span class="pf-v5-c-label pf-m-outline">
                                  <span class="pf-v5-c-label__content">
                                    <span class="pf-v5-c-label__text">Tag 3</span>
                                  </span>
                                </span>
                              </li>
                              <li class="pf-v5-c-label-group__list-item">
                                <button
                                  class="pf-v5-c-label pf-m-overflow"
                                  type="button"
                                >
                                  <span class="pf-v5-c-label__content">
                                    <span class="pf-v5-c-label__text">2 more</span>
                                  </span>
                                </button>
                              </li>
                            </ul>
                          </div>
                        </div>
                      </div>
                    </div>
                  </div>
                </div>
              </section>
              <section
                class="pf-v5-c-tab-content"
                id="-tabs-tab2-panel"
                aria-labelledby="-tabs-tab2-link"
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

### Modal tabs

```html isFullscreen
<div class="pf-v5-c-backdrop">
  <div class="pf-v5-l-bullseye">
    <div
      class="pf-v5-c-modal-box pf-m-sm"
      role="dialog"
      aria-modal="true"
      aria-labelledby="modal-tabs-example-modal-title"
      aria-describedby="modal-tabs-example-modal-description"
    >
      <div class="pf-v5-c-modal-box__close">
        <button
          class="pf-v5-c-button pf-m-plain"
          type="button"
          aria-label="Close"
        >
          <i class="fas fa-times" aria-hidden="true"></i>
        </button>
      </div>
      <header class="pf-v5-c-modal-box__header">
        <h1
          class="pf-v5-c-modal-box__title"
          id="modal-tabs-example-modal-title"
        >PatternFly</h1>
      </header>
      <div
        class="pf-v5-c-modal-box__body"
        id="modal-tabs-example-modal-description"
      >
        <div class="pf-v5-l-grid pf-m-gutter">
          <div class="pf-v5-l-grid__item">
            <div
              class="pf-v5-c-tabs pf-m-secondary pf-m-inset-none"
              role="region"
              id="modal-tabs-example-tabs"
            >
              <ul class="pf-v5-c-tabs__list" role="tablist">
                <li class="pf-v5-c-tabs__item pf-m-current" role="presentation">
                  <button
                    type="button"
                    class="pf-v5-c-tabs__link"
                    role="tab"
                    aria-controls="modal-tabs-example-tabs-details-panel"
                    id="modal-tabs-example-tabs-details-link"
                  >
                    <span class="pf-v5-c-tabs__item-text">Details</span>
                  </button>
                </li>
                <li class="pf-v5-c-tabs__item" role="presentation">
                  <button
                    type="button"
                    class="pf-v5-c-tabs__link"
                    role="tab"
                    aria-controls="modal-tabs-example-tabs-documentation-panel"
                    id="modal-tabs-example-tabs-documentation-link"
                  >
                    <span class="pf-v5-c-tabs__item-text">Documentation</span>
                  </button>
                </li>
              </ul>
            </div>
          </div>
          <div class="pf-v5-l-grid__item">
            <section
              class="pf-v5-c-tab-content"
              aria-labelledby="modal-tabs-example-tabs-details-link"
              id="modal-tabs-example-tabs-details-panel"
              role="tabpanel"
              tabindex="0"
            >
              <div class="pf-v5-c-tab-content__body">
                <p>PatternFly is a community project that promotes design commonality and improves user experience.</p>
              </div>
            </section>
            <section
              class="pf-v5-c-tab-content"
              aria-labelledby="modal-tabs-example-tabs-documentation-link"
              id="modal-tabs-example-tabs-documentation-panel"
              role="tabpanel"
              tabindex="0"
              hidden
            >
              <div class="pf-v5-c-tab-content__body">
                <ul class="pf-v5-c-list" role="list">
                  <li>
                    <a>Doc link 1</a>
                  </li>
                  <li>
                    <a>Doc link 2</a>
                  </li>
                  <li>
                    <a>Doc link 3</a>
                  </li>
                </ul>
              </div>
            </section>
          </div>
        </div>
      </div>
    </div>
  </div>
</div>

<div class="pf-v5-c-page" id="modal-tabs-example">
  <div class="pf-v5-c-skip-to-content">
    <a
      class="pf-v5-c-button pf-m-primary"
      href="#main-content-modal-tabs-example"
    >Skip to content</a>
  </div>
  <header class="pf-v5-c-masthead" id="modal-tabs-example-masthead">
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
        id="modal-tabs-example-masthead-toolbar"
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
        id="modal-tabs-example-primary-nav"
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
    id="main-content-modal-tabs-example"
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
    <section class="pf-v5-c-page__main-section">
      <div class="pf-v5-l-gallery pf-m-gutter">
        <div
          class="pf-v5-c-card pf-m-selectable-raised pf-m-compact"
          id="modal-tabs-example-card-1"
        >
          <div class="pf-v5-c-card__title">
            <h2 class="pf-v5-c-card__title-text">PatternFly</h2>
          </div>
          <div
            class="pf-v5-c-card__body"
          >PatternFly is a community project that promotes design commonality and improves user experience.</div>
        </div>
        <div
          class="pf-v5-c-card pf-m-selectable-raised pf-m-compact"
          id="modal-tabs-example-card-2"
        >
          <div class="pf-v5-c-card__title">
            <h2 class="pf-v5-c-card__title-text">ActiveMQ</h2>
          </div>
          <div
            class="pf-v5-c-card__body"
          >The ActiveMQ component allows messages to be sent to a JMS Queue or Topic; or messages to be consumed from a JMS Queue or Topic using Apache ActiveMQ.</div>
        </div>
        <div
          class="pf-v5-c-card pf-m-selectable-raised pf-m-compact"
          id="modal-tabs-example-card-3"
        >
          <div class="pf-v5-c-card__title">
            <h2 class="pf-v5-c-card__title-text">Apache Spark</h2>
          </div>
          <div
            class="pf-v5-c-card__body"
          >This documentation page covers the Apache Spark component for the Apache Camel.</div>
        </div>
      </div>
    </section>
  </main>
</div>

```

### Nested, unindented tabs

```html isFullscreen
<div class="pf-v5-c-page" id="gray-tabs-example">
  <div class="pf-v5-c-skip-to-content">
    <a
      class="pf-v5-c-button pf-m-primary"
      href="#main-content-gray-tabs-example"
    >Skip to content</a>
  </div>
  <header class="pf-v5-c-masthead" id="gray-tabs-example-masthead">
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
        id="gray-tabs-example-masthead-toolbar"
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
        id="gray-tabs-example-primary-nav"
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
    id="main-content-gray-tabs-example"
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
    <section class="pf-v5-c-page__main-tabs pf-m-limit-width">
      <div class="pf-v5-c-page__main-body">
        <div
          class="pf-v5-c-tabs pf-m-page-insets"
          role="region"
          id="gray-tabs-example-tabs-tabs"
        >
          <ul class="pf-v5-c-tabs__list" role="tablist">
            <li class="pf-v5-c-tabs__item" role="presentation">
              <button
                type="button"
                class="pf-v5-c-tabs__link"
                role="tab"
                aria-controls="gray-tabs-example-tabs-tabs-new-panel"
                id="gray-tabs-example-tabs-tabs-new-link"
              >
                <span class="pf-v5-c-tabs__item-text">What's new</span>
              </button>
            </li>
            <li class="pf-v5-c-tabs__item pf-m-current" role="presentation">
              <button
                type="button"
                class="pf-v5-c-tabs__link"
                role="tab"
                aria-controls="gray-tabs-example-tabs-tabs-get-started-panel"
                id="gray-tabs-example-tabs-tabs-get-started-link"
              >
                <span class="pf-v5-c-tabs__item-text">Get started</span>
              </button>
            </li>
            <li class="pf-v5-c-tabs__item" role="presentation">
              <button
                type="button"
                class="pf-v5-c-tabs__link"
                role="tab"
                aria-controls="gray-tabs-example-tabs-tabs-knowledge-panel"
                id="gray-tabs-example-tabs-tabs-knowledge-link"
              >
                <span class="pf-v5-c-tabs__item-text">Knowledge</span>
              </button>
            </li>
            <li class="pf-v5-c-tabs__item" role="presentation">
              <button
                type="button"
                class="pf-v5-c-tabs__link"
                role="tab"
                aria-controls="gray-tabs-example-tabs-tabs-support-panel"
                id="gray-tabs-example-tabs-tabs-support-link"
              >
                <span class="pf-v5-c-tabs__item-text">Support</span>
              </button>
            </li>
          </ul>
        </div>
      </div>
    </section>
    <section class="pf-v5-c-page__main-section pf-m-limit-width pf-m-light">
      <div class="pf-v5-c-page__main-body">
        <section
          class="pf-v5-c-tab-content"
          aria-labelledby="gray-tabs-example-tabs-tabs-new-link"
          id="gray-tabs-example-tabs-tabs-new-panel"
          role="tabpanel"
          tabindex="0"
          hidden
        >
          <div class="pf-v5-c-tab-content__body">What's new content</div>
        </section>
        <section
          class="pf-v5-c-tab-content"
          aria-labelledby="gray-tabs-example-tabs-tabs-get-started-link"
          id="gray-tabs-example-tabs-tabs-get-started-panel"
          role="tabpanel"
          tabindex="0"
        >
          <div class="pf-v5-c-tab-content__body">
            <div class="pf-v5-l-grid pf-m-gutter">
              <div class="pf-v5-l-grid__item">
                <h1
                  class="pf-v5-c-title pf-m-lg"
                >Get started with Red Hat Enterprise Linux</h1>
              </div>
              <div class="pf-v5-l-grid__item">
                <div
                  class="pf-v5-c-tabs pf-m-secondary pf-m-inset-none"
                  role="region"
                  id="gray-tabs-example-tabs-subtabs"
                >
                  <ul class="pf-v5-c-tabs__list" role="tablist">
                    <li
                      class="pf-v5-c-tabs__item pf-m-current"
                      role="presentation"
                    >
                      <button
                        type="button"
                        class="pf-v5-c-tabs__link"
                        role="tab"
                        aria-controls="gray-tabs-example-tabs-subtabs-x86-panel"
                        id="gray-tabs-example-tabs-subtabs-x86-link"
                      >
                        <span class="pf-v5-c-tabs__item-text">x86 architecture</span>
                      </button>
                    </li>
                    <li class="pf-v5-c-tabs__item" role="presentation">
                      <button
                        type="button"
                        class="pf-v5-c-tabs__link"
                        role="tab"
                        aria-controls="gray-tabs-example-tabs-subtabs-additional-architectures-panel"
                        id="gray-tabs-example-tabs-subtabs-additional-architectures-link"
                      >
                        <span
                          class="pf-v5-c-tabs__item-text"
                        >Additional Architectures</span>
                      </button>
                    </li>
                  </ul>
                </div>
              </div>
              <div class="pf-v5-l-grid__item">
                <section
                  class="pf-v5-c-tab-content"
                  aria-labelledby="gray-tabs-example-tabs-subtabs-x86-link"
                  id="gray-tabs-example-tabs-subtabs-x86-panel"
                  role="tabpanel"
                  tabindex="0"
                >
                  <div class="pf-v5-c-tab-content__body">
                    <div class="pf-v5-l-grid pf-m-gutter">
                      <div class="pf-v5-l-grid__item">
                        <div class="pf-v5-c-content">
                          <p>To perform a standard x86_64 installation using the GUI, you'll need to:</p>
                        </div>
                      </div>
                      <div
                        class="pf-v5-l-grid pf-m-all-6-col-on-md pf-m-all-3-col-on-2xl pf-m-gutter"
                      >
                        <div class="pf-v5-c-card pf-m-flat">
                          <div class="pf-v5-c-card__title">
                            <h2
                              class="pf-v5-c-card__title-text"
                            >Check system requirements</h2>
                          </div>
                          <div class="pf-v5-c-card__body">
                            <p>
                              Your physical or virtual machine should meet the
                              <a href="#">system requirement</a>.
                            </p>
                          </div>
                        </div>
                        <div class="pf-v5-c-card pf-m-flat">
                          <div class="pf-v5-c-card__title">
                            <h2
                              class="pf-v5-c-card__title-text"
                            >Download an installation ISO image</h2>
                          </div>
                          <div class="pf-v5-c-card__body">
                            <p>
                              <a href="#">Download</a>&nbsp;the binary DVD ISO.
                            </p>
                          </div>
                        </div>
                        <div class="pf-v5-c-card pf-m-flat">
                          <div class="pf-v5-c-card__title">
                            <h2
                              class="pf-v5-c-card__title-text"
                            >Create a bootable installation media</h2>
                          </div>
                          <div class="pf-v5-c-card__body">
                            <p>
                              <a href="#">Create</a>&nbsp;a bootable installation media, for example a USB flash drive.
                            </p>
                          </div>
                        </div>
                      </div>
                    </div>
                  </div>
                </section>
                <section
                  class="pf-v5-c-tab-content"
                  aria-labelledby="gray-tabs-example-tabs-subtabs-additional-architectures-link"
                  id="gray-tabs-example-tabs-subtabs-additional-architectures-panel"
                  role="tabpanel"
                  tabindex="0"
                  hidden
                >
                  <div class="pf-v5-c-tab-content__body">
                    <p>Additional architectural content</p>
                  </div>
                </section>
              </div>
            </div>
          </div>
        </section>
        <section
          class="pf-v5-c-tab-content"
          aria-labelledby="gray-tabs-example-tabs-tabs-knowledge-link"
          id="gray-tabs-example-tabs-tabs-knowledge-panel"
          role="tabpanel"
          tabindex="0"
          hidden
        >
          <div class="pf-v5-c-tab-content__body">
            <div class="pf-v5-c-content">
              <p>Knowledge content</p>
            </div>
          </div>
        </section>
        <section
          class="pf-v5-c-tab-content"
          aria-labelledby="gray-tabs-example-tabs-tabs-support-link"
          id="gray-tabs-example-tabs-tabs-support-panel"
          role="tabpanel"
          tabindex="0"
          hidden
        >
          <div class="pf-v5-c-tab-content__body">
            <div class="pf-v5-c-content">
              <p>Support content</p>
            </div>
          </div>
        </section>
      </div>
    </section>
  </main>
</div>

```
