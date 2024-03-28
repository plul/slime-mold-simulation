---
id: 'Code block'
section: components
cssPrefix: pf-v5-c-code-block
---## Examples

### Basic

```html
<div class="pf-v5-c-code-block">
  <div class="pf-v5-c-code-block__header">
    <div class="pf-v5-c-code-block__actions">
      <div class="pf-v5-c-code-block__actions-item">
        <button
          class="pf-v5-c-button pf-m-plain"
          type="button"
          aria-label="Copy to clipboard"
        >
          <i class="fas fa-copy" aria-hidden="true"></i>
        </button>
      </div>
      <div class="pf-v5-c-code-block__actions-item">
        <button
          class="pf-v5-c-button pf-m-plain"
          type="button"
          aria-label="Run in Web Terminal"
        >
          <i class="fas fa-play" aria-hidden="true"></i>
        </button>
      </div>
    </div>
  </div>
  <div class="pf-v5-c-code-block__content">
    <pre class="pf-v5-c-code-block__pre"><code class="pf-v5-c-code-block__code">apiVersion: helm.openshift.io/v1beta1/
kind: HelmChartRepository
metadata:
name: azure-sample-repo
spec:
connectionConfig:
url: https://raw.githubusercontent.com/Azure-Samples/helm-charts/master/docs</code>
</pre>
  </div>
</div>

```

### Expandable

```html
<div class="pf-v5-c-code-block">
  <div class="pf-v5-c-code-block__header">
    <div class="pf-v5-c-code-block__actions">
      <div class="pf-v5-c-code-block__actions-item">
        <button
          class="pf-v5-c-button pf-m-plain"
          type="button"
          aria-label="Copy to clipboard"
        >
          <i class="fas fa-copy" aria-hidden="true"></i>
        </button>
      </div>
      <div class="pf-v5-c-code-block__actions-item">
        <button
          class="pf-v5-c-button pf-m-plain"
          type="button"
          aria-label="Run in Web Terminal"
        >
          <i class="fas fa-play" aria-hidden="true"></i>
        </button>
      </div>
    </div>
  </div>
  <div class="pf-v5-c-code-block__content">
    <pre class="pf-v5-c-code-block__pre"><code class="pf-v5-c-code-block__code">apiVersion: helm.openshift.io/v1beta1/
kind: HelmChartRepository
metadata:
name: azure-sample-repo<div
  class="pf-v5-c-expandable-section pf-m-detached"
><div
  class="pf-v5-c-expandable-section__content"
  hidden
  id="code-block-expandable-content"
>
  spec:
  connectionConfig:
  url: https://raw.githubusercontent.com/Azure-Samples/helm-charts/master/docs</div>
</div>
</code>
</pre>
    <div class="pf-v5-c-expandable-section pf-m-detached">
      <button
        type="button"
        class="pf-v5-c-expandable-section__toggle"
        aria-expanded="false"
        aria-controls="code-block-expandable-content"
      >
        <span class="pf-v5-c-expandable-section__toggle-icon pf-m-expand-top">
          <i class="fas fa-angle-right" aria-hidden="true"></i>
        </span>
        <span class="pf-v5-c-expandable-section__toggle-text">Show more</span>
      </button>
    </div>
  </div>
</div>

```

### Expandable expanded

```html
<div class="pf-v5-c-code-block">
  <div class="pf-v5-c-code-block__header">
    <div class="pf-v5-c-code-block__actions">
      <div class="pf-v5-c-code-block__actions-item">
        <button
          class="pf-v5-c-button pf-m-plain"
          type="button"
          aria-label="Copy to clipboard"
        >
          <i class="fas fa-copy" aria-hidden="true"></i>
        </button>
      </div>
      <div class="pf-v5-c-code-block__actions-item">
        <button
          class="pf-v5-c-button pf-m-plain"
          type="button"
          aria-label="Run in Web Terminal"
        >
          <i class="fas fa-play" aria-hidden="true"></i>
        </button>
      </div>
    </div>
  </div>
  <div class="pf-v5-c-code-block__content">
    <pre class="pf-v5-c-code-block__pre"><code class="pf-v5-c-code-block__code">apiVersion: helm.openshift.io/v1beta1/
kind: HelmChartRepository
metadata:
name: azure-sample-repo<div
  class="pf-v5-c-expandable-section pf-m-expanded pf-m-detached"
><div
  class="pf-v5-c-expandable-section__content"
  id="code-block-expandable-expanded-content"
>
  spec:
  connectionConfig:
  url: https://raw.githubusercontent.com/Azure-Samples/helm-charts/master/docs</div>
</div>
</code>
</pre>
    <div class="pf-v5-c-expandable-section pf-m-expanded pf-m-detached">
      <button
        type="button"
        class="pf-v5-c-expandable-section__toggle"
        aria-expanded="true"
        aria-controls="code-block-expandable-expanded-content"
      >
        <span class="pf-v5-c-expandable-section__toggle-icon pf-m-expand-top">
          <i class="fas fa-angle-right" aria-hidden="true"></i>
        </span>
        <span class="pf-v5-c-expandable-section__toggle-text">Show less</span>
      </button>
    </div>
  </div>
</div>

```

## Documentation

### Usage

| Class | Applied to | Outcome |
| -- | -- | -- |
| `.pf-v5-c-code-block` | `<div>` | Initiates the code block component. **Required** |
| `.pf-v5-c-code-block__header` | `<div>` | Initiates the code block header. |
| `.pf-v5-c-code-block__actions` | `<div>` | Initiates the code block actions. |
| `.pf-v5-c-code-block__actions-item` | `<div>` | Initiates a code block action item. |
| `.pf-v5-c-code-block__content` | `<div>` | Initiates the code block content. **Required** |
| `.pf-v5-c-code-block__pre` | `<pre>` | Initiates the code block `<pre>` element. **Required** |
| `.pf-v5-c-code-block__code` | `<code>` | Initiates the code block `<code>` element. **Required** |
