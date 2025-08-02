<template>
  <div class="edit-post-container">
    <el-card class="edit-post-card" shadow="hover">
      <template #header>
        <div class="edit-post-header">
          <h1>编辑随笔</h1>
          <p>修改你的想法和感受</p>
        </div>
      </template>
      
      <div class="edit-post-body">
        <el-form 
          v-if="post"
          ref="formRef" 
          :model="form" 
          :rules="rules" 
          label-width="80px"
          @submit.prevent="updatePost"
        >
          <el-form-item label="标题" prop="title">
            <el-input 
              v-model="form.title" 
              placeholder="给你的随笔起个标题..."
              size="large"
              clearable
            />
          </el-form-item>
          
          <el-form-item label="内容" prop="content">
            <el-input 
              v-model="form.content" 
              type="textarea" 
              :rows="16"
              placeholder="在这里写下你的想法..."
              size="large"
              resize="vertical"
            />
          </el-form-item>
          
          <el-form-item>
            <el-button 
              type="primary" 
              size="large"
              @click="updatePost"
              :loading="loading"
              :icon="Check"
            >
              保存更改
            </el-button>
            <el-button 
              type="info" 
              size="large"
              @click="cancelEdit"
              :icon="Close"
            >
              取消
            </el-button>
          </el-form-item>
        </el-form>
      </div>
    </el-card>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { ElMessage, type FormInstance, type FormRules } from 'element-plus'
import { Check, Close } from '@element-plus/icons-vue'
import axios from 'axios'

interface Post {
  id: number
  title: string
  content: string
}

const post = ref<Post | null>(null)
const route = useRoute()
const router = useRouter()
const formRef = ref<FormInstance>()
const loading = ref(false)

const form = reactive({
  title: '',
  content: ''
})

const rules: FormRules = {
  title: [
    { required: true, message: '请输入标题', trigger: 'blur' },
    { min: 1, max: 100, message: '标题长度在 1 到 100 个字符', trigger: 'blur' }
  ],
  content: [
    { required: true, message: '请输入内容', trigger: 'blur' },
    { min: 1, max: 10000, message: '内容长度在 1 到 10000 个字符', trigger: 'blur' }
  ]
}

const fetchPost = async () => {
  const postId = route.params.id
  try {
    const response = await axios.get(`${import.meta.env.VITE_API_URL}/posts/${postId}`)
    post.value = response.data
    form.title = response.data.title
    form.content = response.data.content
  } catch (error) {
    console.error(`获取文章 (id: ${postId}) 失败:`, error)
    ElMessage.error('获取文章失败')
  }
}

const updatePost = async () => {
  if (!formRef.value || !post.value) return
  
  try {
    await formRef.value.validate()
    loading.value = true
    
    await axios.put(`${import.meta.env.VITE_API_URL}/posts/${post.value.id}`, {
      title: form.title,
      content: form.content
    })
    
    ElMessage.success('保存成功')
    router.push({ name: 'post', params: { id: post.value.id } })
  } catch (error) {
    if (error !== false) {
      console.error(`更新文章 (id: ${post.value.id}) 失败:`, error)
      ElMessage.error('保存失败，请重试')
    }
  } finally {
    loading.value = false
  }
}

const cancelEdit = () => {
  router.push({ name: 'post', params: { id: post.value?.id } })
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
  max-width: 900px;
  width: 100%;
  margin: 0 auto;
  border-radius: 16px;
  overflow: hidden;
}

.edit-post-card :deep(.el-card__header) {
  background: #87ceeb;
  border-bottom: none;
  padding: 2rem 3rem;
}

.edit-post-card :deep(.el-card__body) {
  padding: 0;
}

.edit-post-header {
  text-align: center;
  color: white;
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

.edit-post-body :deep(.el-form-item__label) {
  font-weight: 600;
  color: #2d3748;
  font-size: 1.1rem;
}

.edit-post-body :deep(.el-input__wrapper) {
  border-radius: 12px;
  box-shadow: 0 0 0 1px #e2e8f0;
}

.edit-post-body :deep(.el-input__wrapper:hover) {
  box-shadow: 0 0 0 1px #667eea;
}

.edit-post-body :deep(.el-input__wrapper.is-focus) {
  box-shadow: 0 0 0 1px #667eea;
}

.edit-post-body :deep(.el-textarea__inner) {
  font-family: 'Georgia', 'Times New Roman', serif;
  line-height: 1.6;
  border-radius: 12px;
  box-shadow: 0 0 0 1px #e2e8f0;
}

.edit-post-body :deep(.el-textarea__inner:hover) {
  box-shadow: 0 0 0 1px #667eea;
}

.edit-post-body :deep(.el-textarea__inner:focus) {
  box-shadow: 0 0 0 1px #667eea;
}

.edit-post-body :deep(.el-form-item:last-child .el-form-item__content) {
  display: flex;
  gap: 1rem;
  justify-content: center;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .edit-post-container {
    padding: 100px 1rem 2rem;
  }
  
  .edit-post-card {
    max-width: 100%;
  }
  
  .edit-post-card :deep(.el-card__header) {
    padding: 2rem 2rem 1.5rem;
  }
  
  .edit-post-header h1 {
    font-size: 2rem;
  }
  
  .edit-post-body {
    padding: 2rem;
  }
  
  .edit-post-body :deep(.el-form-item:last-child .el-form-item__content) {
    flex-direction: column;
  }
}

@media (max-width: 480px) {
  .edit-post-card :deep(.el-card__header) {
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