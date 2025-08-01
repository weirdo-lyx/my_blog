<template>
  <div class="post-view-container">
    <div v-if="post" class="post-content-card">
      <div class="post-header">
        <h1 class="post-title">{{ post?.title }}</h1>
      </div>
      <div class="post-body">
        <p class="post-content">{{ post?.content }}</p>
      </div>
      <div class="post-controls">
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
  if (confirm('确定要删除这篇随笔吗？')) {
    try {
      await axios.delete(`http://127.0.0.1:3000/posts/${post.value.id}`)
      router.push('/')
    } catch (error) {
      console.error(`删除随笔 (id: ${post.value.id}) 失败:`, error)
    }
  }
}

onMounted(fetchPost)
</script>

<style scoped>
.post-view-container {
  min-height: 100vh;
  background-image: url('/hero-background.jpg');
  background-size: cover;
  background-position: center;
  background-attachment: fixed;
  padding: 120px 2rem 4rem;
  display: flex;
  justify-content: center;
  align-items: flex-start;
}

.post-content-card {
  background-color: rgba(255, 255, 255, 0.95);
  border-radius: 16px;
  box-shadow: 0 20px 40px rgba(0, 0, 0, 0.1);
  backdrop-filter: blur(20px);
  max-width: 1000px;
  width: 100%;
  margin: 0 auto;
  overflow: hidden;
  border: 1px solid rgba(255, 255, 255, 0.2);
}

.post-header {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  padding: 3rem 4rem 2rem;
  color: white;
}

.post-title {
  font-size: 2.5rem;
  font-weight: 700;
  margin: 0;
  line-height: 1.2;
  text-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.post-body {
  padding: 3rem 4rem;
  min-height: 60vh;
}

.post-content {
  font-family: 'Georgia', 'Times New Roman', serif;
  white-space: pre-wrap;
  font-size: 1.25rem;
  line-height: 1.8;
  color: #2d3748;
  margin: 0;
  text-align: justify;
  letter-spacing: 0.01em;
}

.post-controls {
  background-color: #f8fafc;
  padding: 2rem 4rem;
  display: flex;
  gap: 1rem;
  border-top: 1px solid #e2e8f0;
  justify-content: flex-end;
}

.button-edit, .button-delete {
  padding: 0.75rem 1.5rem;
  border: none;
  border-radius: 8px;
  font-size: 1rem;
  font-weight: 600;
  text-decoration: none;
  cursor: pointer;
  transition: all 0.3s ease;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  min-width: 100px;
}

.button-edit {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  box-shadow: 0 4px 12px rgba(102, 126, 234, 0.4);
}

.button-edit:hover {
  transform: translateY(-2px);
  box-shadow: 0 6px 20px rgba(102, 126, 234, 0.6);
}

.button-delete {
  background: linear-gradient(135deg, #f093fb 0%, #f5576c 100%);
  color: white;
  box-shadow: 0 4px 12px rgba(240, 147, 251, 0.4);
}

.button-delete:hover {
  transform: translateY(-2px);
  box-shadow: 0 6px 20px rgba(240, 147, 251, 0.6);
}

/* 响应式设计 */
@media (max-width: 768px) {
  .post-view-container {
    padding: 100px 1rem 2rem;
  }
  
  .post-content-card {
    max-width: 100%;
  }
  
  .post-header {
    padding: 2rem 2rem 1.5rem;
  }
  
  .post-title {
    font-size: 2rem;
  }
  
  .post-body {
    padding: 2rem;
    min-height: 50vh;
  }
  
  .post-content {
    font-size: 1.1rem;
    line-height: 1.7;
  }
  
  .post-controls {
    padding: 1.5rem 2rem;
    flex-direction: column;
  }
  
  .button-edit, .button-delete {
    width: 100%;
  }
}

@media (max-width: 480px) {
  .post-header {
    padding: 1.5rem 1.5rem 1rem;
  }
  
  .post-title {
    font-size: 1.75rem;
  }
  
  .post-body {
    padding: 1.5rem;
  }
  
  .post-content {
    font-size: 1rem;
  }
}
</style>
