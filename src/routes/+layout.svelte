<script lang="ts">
  import 'material-symbols/outlined.css';
  import '@fontsource/inter/400.css';
  import '@fontsource/inter/500.css';
  import '@fontsource/inter/600.css';
  import '@fontsource/inter/700.css';
  import { onNavigate } from '$app/navigation';

  onNavigate((navigation) => {
    if (!document.startViewTransition) return;

    return new Promise((resolve) => {
      document.startViewTransition(async () => {
        resolve();
        await navigation.complete;
      });
    });
  });
</script>

<div class="app-shell">

  <div class="content">
    <slot />
  </div>
</div>

<style>
  :global(body) {
    margin: 0;
    padding: 0;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    text-rendering: optimizeLegibility;
    font-feature-settings: "kern" 1, "liga" 1;
  }
  :global(*) {
    font-family: 'Inter', system-ui, sans-serif;
    letter-spacing: 0;
  }

  .app-shell {
    background: #131313;
    color: #e5e5e5;
    min-height: 100vh;
  }

  .content {
    padding: 24px;
    max-width: 900px;
    margin: 0 auto;
  }

  :global(::view-transition-old(root)) {
    animation: 100ms ease-in both fade-to-black;
  }
  :global(::view-transition-new(root)) {
    animation: 150ms ease-out 100ms both fade-from-black;
  }

  @keyframes fade-to-black {
    from { opacity: 1; }
    to   { opacity: 0; }
  }

  @keyframes fade-from-black {
    from { opacity: 0; transform: translateY(16px); }
    to   { opacity: 1; transform: translateY(0); }
  }
</style>