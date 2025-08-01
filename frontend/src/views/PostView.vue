<template>
  <div class="post-view" v-if="post">
    <h1>{{ post.title }}</h1>
    <p class="post-content">{{ post.content }}</p>
    <div class="controls">
      <router-link :to="{ name: 'edit', params: { id: post.id } }" class="button edit-button">编辑</router-link>
      <button @click="deletePost" class="button delete-button">删除</button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import axios from 'axios'

interface Post {
  id: number
  title: string
  content: string
}

const post = ref<Post | null>(null)
const route = useRoute()
const router = useRouter()

const fetchPost = async () => {
  const postId = route.params.id
  try {
    const response = await axios.get(`http://127.0.0.1:3000/posts/${postId}`)
    post.value = response.data
  } catch (error) {
    console.error(`获取文章 (id: ${postId}) 失败:`, error)
  }
}

const deletePost = async () => {
  if (!post.value) return
  if (confirm('确定要删除这篇文章吗？')) {
    try {
      await axios.delete(`http://127.0.0.1:3000/posts/${post.value.id}`)
      router.push('/') // Redirect to home page after deletion
    } catch (error) {
      console.error(`删除文章 (id: ${post.value.id}) 失败:`, error)
    }
  }
}

onMounted(fetchPost)
</script>

<style scoped>
.post-view h1 {
  font-size: 2.5rem;
  margin-bottom: 1.5rem;
}

.post-content {
  font-family: var(--font-serif);
  white-space: pre-wrap; /* Preserve whitespace and newlines */
  font-size: 1.2rem;
  line-height: 1.8;
  color: #444;
}

.controls {
  margin-top: 3rem;
  display: flex;
  gap: 1rem;
  border-top: 1px solid #eee;
  padding-top: 2rem;
}

.button {
  display: inline-block;
  padding: 0.7rem 1.5rem;
  border: none;
  border-radius: 5px;
  text-decoration: none;
  color: white;
  cursor: pointer;
  font-size: 1rem;
  text-align: center;
  transition: background-color 0.2s ease-in-out, transform 0.1s ease-in-out;
}

.button:hover {
  transform: translateY(-2px);
}

.edit-button {
  background-color: var(--primary-color);
}

.edit-button:hover {
  background-color: var(--primary-hover-color);
}

.delete-button {
  background-color: #dc3545;
}

.delete-button:hover {
  background-color: #c82333;
}
</style>
