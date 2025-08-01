<template>
  <div class="content-wrapper">
    <h1>创建新文章</h1>
    <div class="card">
      <form @submit.prevent="createPost">
        <div class="form-group">
          <label for="title">标题</label>
          <input type="text" id="title" v-model="title" required>
        </div>
        <div class="form-group">
          <label for="content">内容</label>
          <textarea id="content" v-model="content" required rows="12"></textarea>
        </div>
        <div class="controls">
          <button type="submit" class="button-primary">发布文章</button>
        </div>
      </form>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import axios from 'axios'
import '../assets/content.css' // Import shared styles

const title = ref('')
const content = ref('')
const router = useRouter()

const createPost = async () => {
  try {
    await axios.post('http://127.0.0.1:3000/posts', {
      title: title.value,
      content: content.value
    })
    router.push('/')
  } catch (error) {
    console.error('创建文章失败:', error)
  }
}
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
}

.button-primary {
  background-color: var(--primary-color);
  color: white;
  padding: 0.75rem 1.5rem;
  border: none;
  border-radius: 6px;
  cursor: pointer;
}
</style>