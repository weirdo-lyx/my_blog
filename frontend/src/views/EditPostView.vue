<template>
  <div class="form-container">
    <h1>编辑文章</h1>
    <form @submit.prevent="updatePost" v-if="post">
      <div class="form-group">
        <label for="title">标题</label>
        <input type="text" id="title" v-model="post.title" required>
      </div>
      <div class="form-group">
        <label for="content">内容</label>
        <textarea id="content" v-model="post.content" required rows="15"></textarea>
      </div>
      <div class="controls">
        <button type="submit" class="button-primary">保存更改</button>
        <router-link :to="{ name: 'post', params: { id: post.id } }" class="button-secondary">取消</router-link>
      </div>
    </form>
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

const updatePost = async () => {
  if (!post.value) return
  try {
    await axios.put(`http://127.0.0.1:3000/posts/${post.value.id}`, {
      title: post.value.title,
      content: post.value.content
    })
    router.push({ name: 'post', params: { id: post.value.id } })
  } catch (error) {
    console.error(`更新文章 (id: ${post.value.id}) 失败:`, error)
  }
}

onMounted(fetchPost)
</script>

<style scoped>
.form-container h1 {
  margin-bottom: 1.5rem;
}

.form-group {
  margin-bottom: 1.5rem;
}

.form-group label {
  display: block;
  margin-bottom: 0.5rem;
  font-weight: 600;
  font-size: 1.1rem;
}

.form-group input,
.form-group textarea {
  width: 100%;
  padding: 0.8rem 1rem;
  border: 1px solid #ccc;
  border-radius: 6px;
  font-size: 1rem;
  box-sizing: border-box; /* Important for width */
  transition: border-color 0.2s ease-in-out, box-shadow 0.2s ease-in-out;
}

.form-group input:focus,
.form-group textarea:focus {
  outline: none;
  border-color: var(--primary-color);
  box-shadow: 0 0 0 3px rgba(0, 123, 255, 0.25);
}

.controls {
  margin-top: 2rem;
  display: flex;
  gap: 1rem;
  align-items: center;
}

.button-primary, .button-secondary {
  padding: 0.8rem 2rem;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-size: 1.1rem;
  text-decoration: none;
  text-align: center;
  transition: background-color 0.2s ease-in-out, color 0.2s ease-in-out;
}

.button-primary {
  background-color: var(--primary-color);
  color: white;
}

.button-primary:hover {
  background-color: var(--primary-hover-color);
}

.button-secondary {
  background-color: transparent;
  color: var(--text-color);
  border: 1px solid #ccc;
}

.button-secondary:hover {
  background-color: #f8f9fa;
  border-color: #bbb;
}
</style>