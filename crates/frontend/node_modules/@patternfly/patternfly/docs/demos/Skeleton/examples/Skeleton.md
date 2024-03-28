---
id: Skeleton
section: components
---## Demos

### Skeleton card

```html isFullscreen
<div class="pf-v5-c-page" id="skeleton-basic-example">
  <div class="pf-v5-c-skip-to-content">
    <a
      class="pf-v5-c-button pf-m-primary"
      href="#main-content-skeleton-basic-example"
    >Skip to content</a>
  </div>
  <header class="pf-v5-c-masthead" id="skeleton-basic-example-masthead">
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
        id="skeleton-basic-example-masthead-toolbar"
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
        id="skeleton-basic-example-primary-nav"
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
    id="main-content-skeleton-basic-example"
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
        <div class="pf-v5-c-card pf-m-hoverable pf-m-compact">
          <div class="pf-v5-c-card__body">
            <div class="pf-v5-l-flex pf-m-column pf-m-spacer-md">
              <div class="pf-v5-c-skeleton"></div>
              <div class="pf-v5-c-skeleton pf-m-width-66"></div>
              <div class="pf-v5-c-skeleton pf-m-width-25"></div>
              <div class="pf-v5-c-skeleton pf-m-width-50"></div>
            </div>
          </div>
          <div class="pf-v5-c-card__body">
            <div class="pf-v5-c-skeleton pf-m-square pf-m-width-md"></div>
          </div>
          <div class="pf-v5-c-card__body">
            <div class="pf-v5-l-flex pf-m-column pf-m-spacer-md">
              <div class="pf-v5-c-skeleton"></div>
              <div class="pf-v5-c-skeleton pf-m-width-25"></div>
              <div class="pf-v5-c-skeleton pf-m-width-75"></div>
              <div class="pf-v5-c-skeleton pf-m-width-50"></div>
            </div>
          </div>
        </div>
        <div class="pf-v5-c-card pf-m-hoverable pf-m-compact">
          <div class="pf-v5-c-card__body">
            <div class="pf-v5-l-flex pf-m-column pf-m-spacer-md">
              <div class="pf-v5-c-skeleton"></div>
              <div class="pf-v5-c-skeleton pf-m-width-66"></div>
              <div class="pf-v5-c-skeleton pf-m-width-25"></div>
              <div class="pf-v5-c-skeleton pf-m-width-50"></div>
            </div>
          </div>
          <div class="pf-v5-c-card__body">
            <div class="pf-v5-c-skeleton pf-m-square pf-m-width-md"></div>
          </div>
          <div class="pf-v5-c-card__body">
            <div class="pf-v5-l-flex pf-m-column pf-m-spacer-md">
              <div class="pf-v5-c-skeleton"></div>
              <div class="pf-v5-c-skeleton pf-m-width-25"></div>
              <div class="pf-v5-c-skeleton pf-m-width-75"></div>
              <div class="pf-v5-c-skeleton pf-m-width-50"></div>
            </div>
          </div>
        </div>
        <div class="pf-v5-c-card pf-m-hoverable pf-m-compact">
          <div class="pf-v5-c-card__body">
            <div class="pf-v5-l-flex pf-m-column pf-m-spacer-md">
              <div class="pf-v5-c-skeleton"></div>
              <div class="pf-v5-c-skeleton pf-m-width-66"></div>
              <div class="pf-v5-c-skeleton pf-m-width-25"></div>
              <div class="pf-v5-c-skeleton pf-m-width-50"></div>
            </div>
          </div>
          <div class="pf-v5-c-card__body">
            <div class="pf-v5-c-skeleton pf-m-square pf-m-width-md"></div>
          </div>
          <div class="pf-v5-c-card__body">
            <div class="pf-v5-l-flex pf-m-column pf-m-spacer-md">
              <div class="pf-v5-c-skeleton"></div>
              <div class="pf-v5-c-skeleton pf-m-width-25"></div>
              <div class="pf-v5-c-skeleton pf-m-width-75"></div>
              <div class="pf-v5-c-skeleton pf-m-width-50"></div>
            </div>
          </div>
        </div>
        <div class="pf-v5-c-card pf-m-hoverable pf-m-compact">
          <div class="pf-v5-c-card__body">
            <div class="pf-v5-l-flex pf-m-column pf-m-spacer-md">
              <div class="pf-v5-c-skeleton"></div>
              <div class="pf-v5-c-skeleton pf-m-width-66"></div>
              <div class="pf-v5-c-skeleton pf-m-width-25"></div>
              <div class="pf-v5-c-skeleton pf-m-width-50"></div>
            </div>
          </div>
          <div class="pf-v5-c-card__body">
            <div class="pf-v5-c-skeleton pf-m-square pf-m-width-md"></div>
          </div>
          <div class="pf-v5-c-card__body">
            <div class="pf-v5-l-flex pf-m-column pf-m-spacer-md">
              <div class="pf-v5-c-skeleton"></div>
              <div class="pf-v5-c-skeleton pf-m-width-25"></div>
              <div class="pf-v5-c-skeleton pf-m-width-75"></div>
              <div class="pf-v5-c-skeleton pf-m-width-50"></div>
            </div>
          </div>
        </div>
        <div class="pf-v5-c-card pf-m-hoverable pf-m-compact">
          <div class="pf-v5-c-card__body">
            <div class="pf-v5-l-flex pf-m-column pf-m-spacer-md">
              <div class="pf-v5-c-skeleton"></div>
              <div class="pf-v5-c-skeleton pf-m-width-66"></div>
              <div class="pf-v5-c-skeleton pf-m-width-25"></div>
              <div class="pf-v5-c-skeleton pf-m-width-50"></div>
            </div>
          </div>
          <div class="pf-v5-c-card__body">
            <div class="pf-v5-c-skeleton pf-m-square pf-m-width-md"></div>
          </div>
          <div class="pf-v5-c-card__body">
            <div class="pf-v5-l-flex pf-m-column pf-m-spacer-md">
              <div class="pf-v5-c-skeleton"></div>
              <div class="pf-v5-c-skeleton pf-m-width-25"></div>
              <div class="pf-v5-c-skeleton pf-m-width-75"></div>
              <div class="pf-v5-c-skeleton pf-m-width-50"></div>
            </div>
          </div>
        </div>
        <div class="pf-v5-c-card pf-m-hoverable pf-m-compact">
          <div class="pf-v5-c-card__body">
            <div class="pf-v5-l-flex pf-m-column pf-m-spacer-md">
              <div class="pf-v5-c-skeleton"></div>
              <div class="pf-v5-c-skeleton pf-m-width-66"></div>
              <div class="pf-v5-c-skeleton pf-m-width-25"></div>
              <div class="pf-v5-c-skeleton pf-m-width-50"></div>
            </div>
          </div>
          <div class="pf-v5-c-card__body">
            <div class="pf-v5-c-skeleton pf-m-square pf-m-width-md"></div>
          </div>
          <div class="pf-v5-c-card__body">
            <div class="pf-v5-l-flex pf-m-column pf-m-spacer-md">
              <div class="pf-v5-c-skeleton"></div>
              <div class="pf-v5-c-skeleton pf-m-width-25"></div>
              <div class="pf-v5-c-skeleton pf-m-width-75"></div>
              <div class="pf-v5-c-skeleton pf-m-width-50"></div>
            </div>
          </div>
        </div>
        <div class="pf-v5-c-card pf-m-hoverable pf-m-compact">
          <div class="pf-v5-c-card__body">
            <div class="pf-v5-l-flex pf-m-column pf-m-spacer-md">
              <div class="pf-v5-c-skeleton"></div>
              <div class="pf-v5-c-skeleton pf-m-width-66"></div>
              <div class="pf-v5-c-skeleton pf-m-width-25"></div>
              <div class="pf-v5-c-skeleton pf-m-width-50"></div>
            </div>
          </div>
          <div class="pf-v5-c-card__body">
            <div class="pf-v5-c-skeleton pf-m-square pf-m-width-md"></div>
          </div>
          <div class="pf-v5-c-card__body">
            <div class="pf-v5-l-flex pf-m-column pf-m-spacer-md">
              <div class="pf-v5-c-skeleton"></div>
              <div class="pf-v5-c-skeleton pf-m-width-25"></div>
              <div class="pf-v5-c-skeleton pf-m-width-75"></div>
              <div class="pf-v5-c-skeleton pf-m-width-50"></div>
            </div>
          </div>
        </div>
      </div>
    </section>
  </main>
</div>

```
