<template>
  <div class="edit-post-container">
    <div class="edit-post-card">
      <div class="edit-post-header">
        <h1>编辑随笔</h1>
        <p>修改你的想法和感受</p>
      </div>
      <div class="edit-post-body">
        <form @submit.prevent="updatePost" v-if="post">
          <div class="form-group">
            <label for="title">标题</label>
            <input type="text" id="title" v-model="post.title" required placeholder="给你的随笔起个标题...">
          </div>
          <div class="form-group">
            <label for="content">内容</label>
            <textarea id="content" v-model="post.content" required rows="16" placeholder="在这里写下你的想法..."></textarea>
          </div>
          <div class="form-controls">
            <button type="submit" class="button-save">保存更改</button>
            <router-link :to="{ name: 'post', params: { id: post.id } }" class="button-cancel">取消</router-link>
          </div>
        </form>
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
.edit-post-container {
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

.edit-post-card {
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

.edit-post-header {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  padding: 3rem 4rem 2rem;
  color: white;
  text-align: center;
}

.edit-post-header h1 {
  font-size: 2.5rem;
  font-weight: 700;
  margin: 0 0 0.5rem 0;
  text-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.edit-post-header p {
  font-size: 1.1rem;
  margin: 0;
  opacity: 0.9;
}

.edit-post-body {
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
  gap: 1rem;
  justify-content: center;
}

.button-save, .button-cancel {
  padding: 1rem 2.5rem;
  border: none;
  border-radius: 12px;
  font-size: 1.1rem;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.3s ease;
  text-decoration: none;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  min-width: 150px;
}

.button-save {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  box-shadow: 0 4px 12px rgba(102, 126, 234, 0.4);
}

.button-save:hover {
  transform: translateY(-2px);
  box-shadow: 0 6px 20px rgba(102, 126, 234, 0.6);
}

.button-cancel {
  background: linear-gradient(135deg, #f093fb 0%, #f5576c 100%);
  color: white;
  box-shadow: 0 4px 12px rgba(240, 147, 251, 0.4);
}

.button-cancel:hover {
  transform: translateY(-2px);
  box-shadow: 0 6px 20px rgba(240, 147, 251, 0.6);
}

.button-save:active, .button-cancel:active {
  transform: translateY(0);
}

/* 响应式设计 */
@media (max-width: 768px) {
  .edit-post-container {
    padding: 100px 1rem 2rem;
  }
  
  .edit-post-card {
    max-width: 100%;
  }
  
  .edit-post-header {
    padding: 2rem 2rem 1.5rem;
  }
  
  .edit-post-header h1 {
    font-size: 2rem;
  }
  
  .edit-post-body {
    padding: 2rem;
  }
  
  .form-group input,
  .form-group textarea {
    padding: 0.875rem 1rem;
  }
  
  .form-controls {
    flex-direction: column;
  }
  
  .button-save, .button-cancel {
    width: 100%;
    padding: 1rem 2rem;
  }
}

@media (max-width: 480px) {
  .edit-post-header {
    padding: 1.5rem 1.5rem 1rem;
  }
  
  .edit-post-header h1 {
    font-size: 1.75rem;
  }
  
  .edit-post-body {
    padding: 1.5rem;
  }
}
</style>