<template>
  <div id="app-container">
    <header>
      <div class="wrapper">
        <h1>博客</h1>
        <nav>
          <RouterLink to="/">主页</RouterLink>
          <RouterLink to="/create">创建新文章</RouterLink>
        </nav>
      </div>
    </header>

    <main>
      <RouterView />
    </main>
  </div>
</template>

<script setup lang="ts">
import { RouterLink, RouterView } from 'vue-router'
</script>

<style>
:root {
  --font-sans: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
  --font-serif: Georgia, Cambria, "Times New Roman", Times, serif;
  --primary-color: #007bff;
  --primary-hover-color: #0056b3;
  --text-color: #333;
  --background-main: rgba(255, 255, 255, 0.9);
  --background-header: rgba(255, 255, 255, 0.85);
}

/* Global styles */
html, body, #app {
  margin: 0;
  padding: 0;
  height: 100%;
  width: 100%;
  font-family: var(--font-sans);
}

/* App container with background */
#app-container {
  min-height: 100vh; /* Ensure it covers the full screen height */
  width: 100%;
  background-image: url('/background.jpg');
  background-size: cover;
  background-position: center;
  background-attachment: fixed;
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 0 1rem 2rem 1rem; /* Add some padding */
  box-sizing: border-box;
}

/* Header styles */
header {
  width: 100%;
  background-color: var(--background-header);
  padding: 1rem 2rem;
  box-shadow: 0 2px 4px rgba(0,0,0,0.1);
  box-sizing: border-box;
  backdrop-filter: blur(8px); /* Frosted glass effect */
  margin-bottom: 2rem;
}

header .wrapper {
  display: flex;
  justify-content: space-between;
  align-items: center;
  max-width: 960px;
  margin: 0 auto;
}

header h1 {
  margin: 0;
  font-size: 1.8rem;
  color: var(--text-color);
}

nav a {
  color: var(--text-color);
  text-decoration: none;
  margin-left: 1.5rem;
  font-size: 1.1rem;
  font-weight: 500;
  transition: color 0.2s ease-in-out;
}

nav a:hover {
  color: var(--primary-hover-color);
}

nav a.router-link-exact-active {
  color: var(--primary-color);
  font-weight: bold;
}

/* Main content area styles */
main {
  flex-grow: 1;
  background-color: var(--background-main);
  padding: 2rem 3rem; /* Increased padding */
  border-radius: 8px;
  width: 100%;
  max-width: 960px;
  box-sizing: border-box;
}
</style>