<template>
  <div class="post-view-container">
    <el-card v-if="post" class="post-content-card" shadow="hover">
      <template #header>
        <div class="post-header">
          <h1 class="post-title">{{ post?.title }}</h1>
        </div>
      </template>
      
      <div class="post-body">
        <p class="post-content">{{ post?.content }}</p>
      </div>
      
      <template #footer>
        <div class="post-controls">
          <el-button 
            type="primary" 
            size="large"
            @click="editPost"
            :icon="Edit"
          >
            编辑
          </el-button>
          <el-button 
            type="danger" 
            size="large"
            @click="deletePost"
            :icon="Delete"
          >
            删除
          </el-button>
        </div>
      </template>
    </el-card>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { ElMessage, ElMessageBox } from 'element-plus'
import { Edit, Delete } from '@element-plus/icons-vue'
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
    const response = await axios.get(`${import.meta.env.VITE_API_URL}/posts/${postId}`)
    post.value = response.data
  } catch (error) {
    console.error(`获取文章 (id: ${postId}) 失败:`, error)
    ElMessage.error('获取文章失败')
  }
}

const editPost = () => {
  router.push({ name: 'edit', params: { id: post.value?.id } })
}

const deletePost = async () => {
  try {
    await ElMessageBox.confirm(
      '确定要删除这篇随笔吗？此操作不可恢复。',
      '确认删除',
      {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        type: 'warning',
      }
    )
    
    await axios.delete(`${import.meta.env.VITE_API_URL}/posts/${post.value?.id}`)
    ElMessage.success('删除成功')
    router.push('/')
  } catch (error) {
    if (error !== 'cancel') {
      console.error(`删除随笔 (id: ${post.value?.id}) 失败:`, error)
      ElMessage.error('删除失败')
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
  max-width: 1000px;
  width: 100%;
  margin: 0 auto;
  border-radius: 16px;
  overflow: hidden;
}

.post-content-card :deep(.el-card__header) {
  background: #87ceeb;
  border-bottom: none;
  padding: 2rem 3rem;
}

.post-content-card :deep(.el-card__body) {
  padding: 0;
}

.post-content-card :deep(.el-card__footer) {
  background-color: #f8fafc;
  border-top: 1px solid #e2e8f0;
  padding: 2rem 3rem;
}

.post-header {
  text-align: center;
}

.post-title {
  font-size: 2.5rem;
  font-weight: 700;
  margin: 0;
  line-height: 1.2;
  color: white;
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
  display: flex;
  gap: 1rem;
  justify-content: center;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .post-view-container {
    padding: 100px 1rem 2rem;
  }
  
  .post-content-card {
    max-width: 100%;
  }
  
  .post-content-card :deep(.el-card__header) {
    padding: 1.5rem 2rem;
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
  
  .post-content-card :deep(.el-card__footer) {
    padding: 1.5rem 2rem;
  }
  
  .post-controls {
    flex-direction: column;
  }
}

@media (max-width: 480px) {
  .post-content-card :deep(.el-card__header) {
    padding: 1rem 1.5rem;
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
  
  .post-content-card :deep(.el-card__footer) {
    padding: 1rem 1.5rem;
  }
}
</style>
