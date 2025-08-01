<template>
  <div class="form-container">
    <h1>创建新文章</h1>
    <form @submit.prevent="createPost">
      <div class="form-group">
        <label for="title">标题</label>
        <input type="text" id="title" v-model="title" required>
      </div>
      <div class="form-group">
        <label for="content">内容</label>
        <textarea id="content" v-model="content" required rows="15"></textarea>
      </div>
      <div class="controls">
        <button type="submit" class="button-primary">创建</button>
      </div>
    </form>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import axios from 'axios'

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
}

.button-primary {
  background-color: var(--primary-color);
  color: white;
  padding: 0.8rem 2rem;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-size: 1.1rem;
  transition: background-color 0.2s ease-in-out;
}

.button-primary:hover {
  background-color: var(--primary-hover-color);
}
</style>