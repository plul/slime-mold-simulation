---
id: Page
section: components
wrapperTag: div
deprecated: true
---import './PageHeader.css'

## Page header examples

### Vertical nav

```html
<div class="pf-v5-c-page">
  <header class="pf-v5-c-page__header">
    <div class="pf-v5-c-page__header-brand">
      <div class="pf-v5-c-page__header-brand-toggle">toggle</div>
      <a href="#" class="pf-v5-c-page__header-brand-link">Logo</a>
    </div>
    <div class="pf-v5-c-page__header-tools">header-tools</div>
  </header>
  <div class="pf-v5-c-page__sidebar">
    <div class="pf-v5-c-page__sidebar-body">Navigation</div>
  </div>
  <main class="pf-v5-c-page__main" tabindex="-1">
    <section class="pf-v5-c-page__main-section pf-m-dark-100">
      This
      <code>.pf-v5-c-page__main-section</code> uses
      <code>.pf-m-dark-100</code>.
    </section>
    <section class="pf-v5-c-page__main-section pf-m-dark-200">
      This
      <code>.pf-v5-c-page__main-section</code> uses
      <code>.pf-m-dark-200</code>.
    </section>
    <section class="pf-v5-c-page__main-section pf-m-light">
      This
      <code>.pf-v5-c-page__main-section</code> uses
      <code>.pf-m-light</code>.
    </section>
    <section class="pf-v5-c-page__main-section">
      This is a default
      <code>.pf-v5-c-page__main-section</code>.
    </section>
  </main>
</div>

```

### Horizontal nav

```html
<div class="pf-v5-c-page">
  <header class="pf-v5-c-page__header">
    <div class="pf-v5-c-page__header-brand">
      <a href="#" class="pf-v5-c-page__header-brand-link">Logo</a>
    </div>
    <div class="pf-v5-c-page__header-nav">Navigation</div>
    <div class="pf-v5-c-page__header-tools">header-tools</div>
  </header>
  <main class="pf-v5-c-page__main" tabindex="-1">
    <section class="pf-v5-c-page__main-section pf-m-dark-100"></section>
    <section class="pf-v5-c-page__main-section pf-m-dark-200"></section>
    <section class="pf-v5-c-page__main-section pf-m-light"></section>
    <section class="pf-v5-c-page__main-section"></section>
  </main>
</div>

```

### Multiple sidebar body elements, padding, and fill

```html
<div class="pf-v5-c-page">
  <header class="pf-v5-c-page__header">
    <div class="pf-v5-c-page__header-brand">
      <div class="pf-v5-c-page__header-brand-toggle">toggle</div>
      <a href="#" class="pf-v5-c-page__header-brand-link">Logo</a>
    </div>
    <div class="pf-v5-c-page__header-tools">header-tools</div>
  </header>
  <div class="pf-v5-c-page__sidebar">
    <div class="pf-v5-c-page__sidebar-body">Navigation</div>
    <div
      class="pf-v5-c-page__sidebar-body pf-m-fill pf-m-page-insets"
    >inset content</div>
    <div class="pf-v5-c-page__sidebar-body pf-m-page-insets">footer content</div>
  </div>
  <main class="pf-v5-c-page__main" tabindex="-1">
    <section class="pf-v5-c-page__main-section pf-m-light"></section>
  </main>
</div>

```

### With or without fill

```html
<div class="pf-v5-c-page">
  <header class="pf-v5-c-page__header">
    <div class="pf-v5-c-page__header-brand">
      <a href="#" class="pf-v5-c-page__header-brand-link">Logo</a>
    </div>
    <div class="pf-v5-c-page__header-nav">Navigation</div>
    <div class="pf-v5-c-page__header-tools">header-tools</div>
  </header>
  <main class="pf-v5-c-page__main" tabindex="-1">
    <section
      class="pf-v5-c-page__main-section pf-m-light"
    >A regular page section.</section>
    <section class="pf-v5-c-page__main-section pf-m-fill">
      This section uses
      <code>.pf-m-fill</code> to fill the available space.
    </section>
    <section class="pf-v5-c-page__main-section pf-m-light pf-m-no-fill">
      This section uses
      <code>.pf-m-no-fill</code> to not fill the available space.
    </section>
  </main>
</div>

```

### Main section padding

```html
<div class="pf-v5-c-page">
  <header class="pf-v5-c-page__header">
    <div class="pf-v5-c-page__header-brand">
      <div class="pf-v5-c-page__header-brand-toggle">toggle</div>
      <a href="#" class="pf-v5-c-page__header-brand-link">Logo</a>
    </div>
    <div class="pf-v5-c-page__header-tools">header-tools</div>
  </header>
  <div class="pf-v5-c-page__sidebar">
    <div class="pf-v5-c-page__sidebar-body">Navigation</div>
  </div>
  <main class="pf-v5-c-page__main" tabindex="-1">
    <section class="pf-v5-c-page__main-section">
      This
      <code>.pf-v5-c-page__main-section</code> has default padding.
    </section>
    <section class="pf-v5-c-page__main-section pf-m-no-padding pf-m-light">
      This
      <code>.pf-v5-c-page__main-section</code> uses
      <code>.pf-m-no-padding</code> to remove all padding.
    </section>
    <section
      class="pf-v5-c-page__main-section pf-m-no-padding pf-m-padding-on-md"
    >
      This
      <code>.pf-v5-c-page__main-section</code> uses
      <code>.pf-m-no-padding .pf-m-padding-on-md</code> to remove padding up to the
      <code>md</code> breakpoint.
    </section>
  </main>
</div>

```

### Main section variations

```html
<div class="pf-v5-c-page">
  <header class="pf-v5-c-page__header">
    <div class="pf-v5-c-page__header-brand">
      <div class="pf-v5-c-page__header-brand-toggle">toggle</div>
      <a href="#" class="pf-v5-c-page__header-brand-link">Logo</a>
    </div>
    <div class="pf-v5-c-page__header-tools">header-tools</div>
  </header>
  <div class="pf-v5-c-page__sidebar">
    <div class="pf-v5-c-page__sidebar-body">Navigation</div>
  </div>
  <main class="pf-v5-c-page__main" tabindex="-1">
    <section class="pf-v5-c-page__main-subnav">
      <code>.pf-v5-c-page__main-subnav</code> for horizontal subnav navigation
    </section>
    <section class="pf-v5-c-page__main-nav">
      <code>.pf-v5-c-page__main-nav</code> for tertiary navigation
    </section>
    <section class="pf-v5-c-page__main-tabs">
      <code>.pf-v5-c-page__main-tabs</code> for tabs
    </section>
    <div class="pf-v5-c-page__main-group">
      <code>.pf-v5-c-page__main-group</code> for a group of page sections
    </div>
    <section class="pf-v5-c-page__main-breadcrumb">
      <code>.pf-v5-c-page__main-breadcrumb</code> for breadcrumbs
    </section>
    <section class="pf-v5-c-page__main-section">
      <code>.pf-v5-c-page__main-section</code> for main sections
    </section>
    <section class="pf-v5-c-page__main-wizard">
      <code>.pf-v5-c-page__main-wizard</code> for wizards
    </section>
  </main>
</div>

```

### Centered section

```html
<div class="pf-v5-c-page">
  <header class="pf-v5-c-page__header">
    <div class="pf-v5-c-page__header-brand">
      <div class="pf-v5-c-page__header-brand-toggle">toggle</div>
      <a href="#" class="pf-v5-c-page__header-brand-link">Logo</a>
    </div>
    <div class="pf-v5-c-page__header-tools">header-tools</div>
  </header>
  <main class="pf-v5-c-page__main" tabindex="-1">
    <section
      class="pf-v5-c-page__main-section pf-m-limit-width pf-m-align-center"
    >
      <div class="pf-v5-c-page__main-body">
        <div class="pf-v5-c-card">
          <div class="pf-v5-c-card__body">
            When a width limited page section is wider than the value of
            <code>--pf-v5-c-page--section--m-limit-width--MaxWidth</code>, the section will be centered in the main section.
            <br />
            <br />The content in this example is placed in a card to better illustrate how the section behaves when it is centered. A card is not required to center a page section.
          </div>
        </div>
      </div>
    </section>
  </main>
</div>

```

## Documentation

### Overview

The page header component is a deprecated approach to building a header on the page component. The recommended approach uses the masthead component instead.

### Accessibility

| Attribute | Applied to | Outcome |
| -- | -- | -- |
| `role="banner"` | `.pf-v5-c-page__header` | Identifies the element that serves as the banner region. |
| `aria-expanded="true/false"` | `.pf-v5-c-page__header-brand-toggle > .pf-v5-c-button` | Indicates that the expandable content is visible and the current state of the contents. **Required** |
| `aria-controls="[id of nav]"` | `.pf-v5-c-page__header-brand-toggle > .pf-v5-c-button` | Identifies the element controlled by the toggle. **Required**

### Usage

| Class | Applied to | Outcome |
| -- | -- | -- |
| `.pf-v5-c-page__header` | `<header>` |   Declares the page header. |
| `.pf-v5-c-page__header-brand` | `<div>` |   Creates a header container to nest the brand component. |
| `.pf-v5-c-page__header-brand-toggle` | `<div>` |   Creates a container to nest the sidebar toggle. |
| `.pf-v5-c-page__header-brand-link` | `<a>`, `<span>` |   Creates a link for the brand logo. Use a `<span>` if there is no link. |
| `.pf-v5-c-page__header-selector` | `<div>` |   Creates a header container to nest the context selector component. |
| `.pf-v5-c-page__header-nav` | `<div>` |   Creates a container to nest the navigation component in the header. |
| `.pf-v5-c-page__header-tools` | `<div>` |   Creates a container to nest the icons and menus in header. |
| `.pf-v5-c-page__header-tools-group` | `<div>` |  Creates a container for grouping sets of icons and menus in header. |
| `.pf-v5-c-page__header-tools-item` | `<div>` |  Creates a container for an item in a header tools group. |
| `.pf-m-selected` | `.pf-v5-c-page__header-tools-item` | Modifies a header tools item to indicate that the button inside is in the selected state. |
| `.pf-m-hidden{-on-[breakpoint]}` | `.pf-v5-c-page__header-tools-group`, `.pf-v5-c-page__header-tools-item` | Hides a header tools group or item at an optional breakpoint, or hides it at all [breakpoints](/developer-resources/global-css-variables#breakpoint-variables-and-class-suffixes) with `.pf-m-hidden`. |
| `.pf-m-visible{-on-[breakpoint]}` | `.pf-v5-c-page__header-tools-group`, `.pf-v5-c-page__header-tools-item` | Shows a header tools group or item at an optional [breakpoint](/developer-resources/global-css-variables#breakpoint-variables-and-class-suffixes). |
