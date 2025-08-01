<template>
  <div id="app-container">
    <header :class="{ 'scrolled': isScrolled }">
      <div class="header-content">
        <h1 class="blog-title">我的博客</h1>
        <nav>
          <RouterLink to="/">主页</RouterLink>
          <RouterLink to="/create">创建新文章</RouterLink>
        </nav>
      </div>
    </header>

    <!-- The RouterView will now be the main content area -->
    <RouterView />

    <footer class="app-footer">
      <p>&copy; 2025 我的博客. All Rights Reserved.</p>
    </footer>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { RouterLink, RouterView } from 'vue-router'

const isScrolled = ref(false)

const handleScroll = () => {
  isScrolled.value = window.scrollY > 50
}

onMounted(() => {
  window.addEventListener('scroll', handleScroll)
})

onUnmounted(() => {
  window.removeEventListener('scroll', handleScroll)
})
</script>

<style>
/* Global Styles */
:root {
  --primary-color: #3b82f6; /* Blue 500 */
  --text-light: #f9fafb; /* Gray 50 */
  --text-dark: #1f2937; /* Gray 800 */
  --header-bg-scrolled: rgba(255, 255, 255, 0.9);
  --footer-bg: #111827; /* Gray 900 */
}

body, html, #app {
  margin: 0;
  padding: 0;
  width: 100%;
  font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
  background-color: #ffffff;
}

/* Header Styles */
header {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  padding: 1.5rem 2rem;
  z-index: 1000;
  transition: background-color 0.3s ease-in-out, box-shadow 0.3s ease-in-out, padding 0.3s ease-in-out;
  box-sizing: border-box;
}

header.scrolled {
  background-color: var(--header-bg-scrolled);
  box-shadow: 0 2px 4px rgba(0,0,0,0.1);
  padding: 1rem 2rem;
}

.header-content {
  max-width: 1200px;
  margin: 0 auto;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.blog-title {
  font-size: 1.5rem;
  font-weight: 600;
  color: var(--text-light);
  margin: 0;
  transition: color 0.3s ease-in-out;
}

header.scrolled .blog-title {
  color: var(--text-dark);
}

nav a {
  color: var(--text-light);
  text-decoration: none;
  margin-left: 1.5rem;
  font-weight: 500;
  transition: color 0.3s ease-in-out;
}

header.scrolled nav a {
  color: var(--text-dark);
}

nav a:hover {
  color: var(--primary-color);
}

/* Footer Styles */
.app-footer {
  background-color: var(--footer-bg);
  color: var(--text-light);
  text-align: center;
  padding: 2rem;
  margin-top: auto; /* Push footer to the bottom */
}
</style>