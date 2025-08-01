<template>
  <div class="create-post-container">
    <div class="create-post-card">
      <div class="create-post-header">
        <h1>写点随笔</h1>
        <p>记录你的想法和感受</p>
      </div>
      <div class="create-post-body">
        <form @submit.prevent="createPost">
          <div class="form-group">
            <label for="title">标题</label>
            <input type="text" id="title" v-model="title" required placeholder="给你的随笔起个标题...">
          </div>
          <div class="form-group">
            <label for="content">内容</label>
            <textarea id="content" v-model="content" required rows="16" placeholder="在这里写下你的想法..."></textarea>
          </div>
          <div class="form-controls">
            <button type="submit" class="button-publish">发布随笔</button>
          </div>
        </form>
      </div>
    </div>
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
.create-post-container {
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

.create-post-card {
  background-color: rgba(255, 255, 255, 0.95);
  border-radius: 16px;
  box-shadow: 0 20px 40px rgba(0, 0, 0, 0.1);
  backdrop-filter: blur(20px);
  max-width: 900px;
  width: 100%;
  margin: 0 auto;
  overflow: hidden;
  border: 1px solid rgba(255, 255, 255, 0.2);
}

.create-post-header {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  padding: 3rem 4rem 2rem;
  color: white;
  text-align: center;
}

.create-post-header h1 {
  font-size: 2.5rem;
  font-weight: 700;
  margin: 0 0 0.5rem 0;
  text-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.create-post-header p {
  font-size: 1.1rem;
  margin: 0;
  opacity: 0.9;
}

.create-post-body {
  padding: 3rem 4rem;
}

.form-group {
  margin-bottom: 2rem;
}

.form-group label {
  display: block;
  margin-bottom: 0.75rem;
  font-weight: 600;
  color: #2d3748;
  font-size: 1.1rem;
}

.form-group input,
.form-group textarea {
  width: 100%;
  padding: 1rem 1.25rem;
  border: 2px solid #e2e8f0;
  border-radius: 12px;
  font-size: 1rem;
  box-sizing: border-box;
  transition: all 0.3s ease;
  background-color: #f8fafc;
  font-family: 'Georgia', 'Times New Roman', serif;
}

.form-group input:focus,
.form-group textarea:focus {
  outline: none;
  border-color: #667eea;
  background-color: white;
  box-shadow: 0 0 0 3px rgba(102, 126, 234, 0.1);
}

.form-group textarea {
  resize: vertical;
  min-height: 300px;
  line-height: 1.6;
}

.form-controls {
  margin-top: 3rem;
  display: flex;
  justify-content: center;
}

.button-publish {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  padding: 1rem 2.5rem;
  border: none;
  border-radius: 12px;
  font-size: 1.1rem;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.3s ease;
  box-shadow: 0 4px 12px rgba(102, 126, 234, 0.4);
  min-width: 150px;
}

.button-publish:hover {
  transform: translateY(-2px);
  box-shadow: 0 6px 20px rgba(102, 126, 234, 0.6);
}

.button-publish:active {
  transform: translateY(0);
}

/* 响应式设计 */
@media (max-width: 768px) {
  .create-post-container {
    padding: 100px 1rem 2rem;
  }
  
  .create-post-card {
    max-width: 100%;
  }
  
  .create-post-header {
    padding: 2rem 2rem 1.5rem;
  }
  
  .create-post-header h1 {
    font-size: 2rem;
  }
  
  .create-post-body {
    padding: 2rem;
  }
  
  .form-group input,
  .form-group textarea {
    padding: 0.875rem 1rem;
  }
  
  .button-publish {
    width: 100%;
    padding: 1rem 2rem;
  }
}

@media (max-width: 480px) {
  .create-post-header {
    padding: 1.5rem 1.5rem 1rem;
  }
  
  .create-post-header h1 {
    font-size: 1.75rem;
  }
  
  .create-post-body {
    padding: 1.5rem;
  }
}
</style>