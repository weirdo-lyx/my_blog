<template>
  <div class="home">
    <ul class="post-list">
      <li v-for="post in posts" :key="post.id">
        <router-link :to="{ name: 'post', params: { id: post.id } }">{{ post.title }}</router-link>
        <button @click="deletePost(post.id)" class="delete-button">删除</button>
      </li>
    </ul>
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

const deletePost = async (id: number) => {
  if (confirm('确定要删除这篇文章吗？')) {
    try {
      await axios.delete(`http://127.0.0.1:3000/posts/${id}`)
      await fetchPosts()
    } catch (error) {
      console.error(`删除文章 (id: ${id}) 失败:`, error)
    }
  }
}

onMounted(fetchPosts)
</script>

<style scoped>
.post-list {
  list-style: none;
  padding: 0;
}

.post-list li {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem 1.5rem;
  border-radius: 6px;
  transition: background-color 0.2s ease-in-out, box-shadow 0.2s ease-in-out;
  margin-bottom: 1rem;
  border: 1px solid #eee;
}

.post-list li:hover {
  background-color: #fcfcfc;
  box-shadow: 0 4px 8px rgba(0,0,0,0.05);
}

.post-list li a {
  font-size: 1.3rem;
  color: var(--primary-color);
  text-decoration: none;
  font-weight: 500;
}

.post-list li a:hover {
  text-decoration: underline;
}

.delete-button {
  background-color: #dc3545;
  color: white;
  border: none;
  padding: 0.5rem 1rem;
  border-radius: 4px;
  cursor: pointer;
  font-size: 0.9rem;
  opacity: 0.8;
  transition: opacity 0.2s ease-in-out, background-color 0.2s ease-in-out;
}

.delete-button:hover {
  opacity: 1;
  background-color: #c82333;
}
</style>
