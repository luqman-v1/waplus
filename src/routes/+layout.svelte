<script lang="ts">
  import '../app.css';

  let { children } = $props();
</script>

<div class="app-layout">
  {@render children()}
</div>

<style>
  .app-layout {
    width: 100vw;
    height: 100vh;
    overflow: hidden;
    background-color: var(--bg-surface);
  }
</style>
