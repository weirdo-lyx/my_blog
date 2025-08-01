<template>
  <div class="content-wrapper">
    <h1>编辑文章</h1>
    <div class="card">
      <form @submit.prevent="updatePost" v-if="post">
        <div class="form-group">
          <label for="title">标题</label>
          <input type="text" id="title" v-model="post.title" required>
        </div>
        <div class="form-group">
          <label for="content">内容</label>
          <textarea id="content" v-model="post.content" required rows="12"></textarea>
        </div>
        <div class="controls">
          <button type="submit" class="button-primary">保存更改</button>
          <router-link :to="{ name: 'post', params: { id: post.id } }" class="button-secondary">取消</router-link>
        </div>
      </form>
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
.form-group {
  margin-bottom: 1.5rem;
}

.form-group label {
  display: block;
  margin-bottom: 0.5rem;
  font-weight: 600;
}

.form-group input,
.form-group textarea {
  width: 100%;
  padding: 0.75rem 1rem;
  border: 1px solid #d1d5db;
  border-radius: 6px;
  font-size: 1rem;
  box-sizing: border-box;
}

.controls {
  margin-top: 2rem;
  display: flex;
  justify-content: flex-end;
  gap: 1rem;
}

.button-primary, .button-secondary {
  padding: 0.75rem 1.5rem;
  border: 1px solid transparent;
  border-radius: 6px;
  cursor: pointer;
  text-decoration: none;
}

.button-primary {
  background-color: var(--primary-color);
  color: white;
}

.button-secondary {
  background-color: #e5e7eb;
  color: var(--text-dark);
}
</style>