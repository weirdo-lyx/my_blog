<template>
  <div class="home-page">
    <section class="hero-section">
      <div class="hero-content">
        <h1>欢迎来到我的数字花园</h1>
        <p>在这里，思绪生根发芽，观点碰撞成长。</p>
      </div>
    </section>

    <section class="posts-section">
      <div class="posts-container">
        <h2>最新文章</h2>
        <div class="post-list">
          <div class="post-card" v-for="post in posts" :key="post.id">
            <h3>{{ post.title }}</h3>
            <p class="post-excerpt">{{ truncate(post.content, 100) }}</p>
            <router-link :to="{ name: 'post', params: { id: post.id } }" class="read-more">阅读全文 &rarr;</router-link>
          </div>
        </div>
      </div>
    </section>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import axios from 'axios'

interface Post {
  id: number
  title: string
  content: string
}

const posts = ref<Post[]>([])

const fetchPosts = async () => {
  try {
    const response = await axios.get('http://127.0.0.1:3000/posts')
    posts.value = response.data
  } catch (error) {
    console.error('获取文章列表失败:', error)
  }
}

const truncate = (text: string, length: number) => {
  if (text.length <= length) {
    return text
  }
  return text.substring(0, length) + '...'
}

onMounted(fetchPosts)
</script>

<style scoped>
.hero-section {
  height: 100vh;
  background-image: url('/hero-background.jpg');
  background-size: cover;
  background-position: center;
  background-attachment: fixed; /* This creates the parallax effect */
  display: flex;
  justify-content: center;
  align-items: center;
  text-align: center;
  color: white;
}

.hero-content {
  background-color: rgba(0, 0, 0, 0.5);
  padding: 2rem 3rem;
  border-radius: 8px;
  backdrop-filter: blur(5px);
}

.hero-content h1 {
  font-size: 3.5rem;
  margin: 0 0 0.5rem 0;
}

.hero-content p {
  font-size: 1.2rem;
  margin: 0;
}

.posts-section {
  background-color: #ffffff;
  padding: 4rem 2rem;
}

.posts-container {
  max-width: 960px;
  margin: 0 auto;
}

.posts-container h2 {
  font-size: 2.5rem;
  text-align: center;
  margin-bottom: 3rem;
  color: var(--text-dark);
}

.post-list {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: 2rem;
}

.post-card {
  border: 1px solid #e5e7eb;
  border-radius: 8px;
  padding: 1.5rem;
  transition: transform 0.2s ease-in-out, box-shadow 0.2s ease-in-out;
}

.post-card:hover {
  transform: translateY(-5px);
  box-shadow: 0 4px 12px rgba(0,0,0,0.1);
}

.post-card h3 {
  font-size: 1.5rem;
  margin: 0 0 1rem 0;
}

.post-excerpt {
  color: #4b5563;
  margin-bottom: 1.5rem;
}

.read-more {
  color: var(--primary-color);
  text-decoration: none;
  font-weight: 600;
}
</style>
