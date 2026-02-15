<script lang="ts">
  import StatusDisplay from "$lib/components/StatusDisplay.svelte";
</script>

<main class="container">
  <h1>Second Page</h1>
  
  <p>This is the second page of the application.</p>
  
  <StatusDisplay />

  <nav class="navigation">
    <a href="/" class="nav-button">← Back to Home</a>
  </nav>
</main>

<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
  gap: 2rem;
}

.navigation {
  margin-top: 2rem;
}

.nav-button {
  display: inline-block;
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #ffffff;
  background-color: #646cff;
  text-decoration: none;
  transition: background-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

.nav-button:hover {
  background-color: #535bf2;
}

h1 {
  text-align: center;
  color: #646cff;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  h1 {
    color: #24c8db;
  }

  .nav-button {
    background-color: #24c8db;
  }

  .nav-button:hover {
    background-color: #1db3c8;
  }
}
</style>
