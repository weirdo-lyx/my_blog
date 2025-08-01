<template>
  <div class="content-wrapper">
    <div class="card">
      <h1>{{ post?.title }}</h1>
      <p class="post-content">{{ post?.content }}</p>
      <div class="controls">
        <router-link :to="{ name: 'edit', params: { id: post.id } }" class="button-edit">编辑</router-link>
        <button @click="deletePost" class="button-delete">删除</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import axios from 'axios'
import '../assets/content.css' // Import shared styles

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
      router.push('/')
    } catch (error) {
      console.error(`删除文章 (id: ${post.value.id}) 失败:`, error)
    }
  }
}

onMounted(fetchPost)
</script>

<style scoped>
.post-content {
  font-family: var(--font-serif);
  white-space: pre-wrap;
  font-size: 1.1rem;
  line-height: 1.7;
  color: #374151;
}

.controls {
  margin-top: 3rem;
  display: flex;
  gap: 1rem;
  border-top: 1px solid #e5e7eb;
  padding-top: 2rem;
}

.button-edit, .button-delete {
  padding: 0.5rem 1rem;
  border: 1px solid transparent;
  border-radius: 6px;
  font-size: 0.9rem;
  text-decoration: none;
  cursor: pointer;
  transition: all 0.2s ease-in-out;
}

.button-edit {
  background-color: var(--primary-color);
  color: white;
}

.button-delete {
  background-color: transparent;
  color: #ef4444;
  border-color: #fca5a5;
}
</style>
