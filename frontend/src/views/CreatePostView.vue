<template>
  <div class="create-post-container">
    <el-card class="create-post-card" shadow="hover">
      <template #header>
        <div class="create-post-header">
          <h1>写点随笔</h1>
          <p>记录你的想法和感受</p>
        </div>
      </template>
      
      <div class="create-post-body">
        <el-form 
          ref="formRef" 
          :model="form" 
          :rules="rules" 
          label-width="80px"
          @submit.prevent="createPost"
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
              @click="createPost"
              :loading="loading"
              :icon="DocumentAdd"
            >
              发布随笔
            </el-button>
          </el-form-item>
        </el-form>
      </div>
    </el-card>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'

onMounted(() => {
  console.log('Vite Env Variables:', import.meta.env);
});
import { useRouter } from 'vue-router'
import { ElMessage, type FormInstance, type FormRules } from 'element-plus'
import { DocumentAdd } from '@element-plus/icons-vue'
import axios from 'axios'

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

const createPost = async () => {
  if (!formRef.value) return
  
  try {
    await formRef.value.validate()
    loading.value = true
    
    await axios.post(`${import.meta.env.VITE_API_URL}/posts`, {
      title: form.title,
      content: form.content
    })
    
    ElMessage.success('发布成功')
    router.push('/')
  } catch (error) {
    if (error !== false) {
      console.error('创建文章失败:', error)
      ElMessage.error('发布失败，请重试')
    }
  } finally {
    loading.value = false
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
  max-width: 900px;
  width: 100%;
  margin: 0 auto;
  border-radius: 16px;
  overflow: hidden;
}

.create-post-card :deep(.el-card__header) {
  background: #87ceeb;
  border-bottom: none;
  padding: 2rem 3rem;
}

.create-post-card :deep(.el-card__body) {
  padding: 0;
}

.create-post-header {
  text-align: center;
  color: white;
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

.create-post-body :deep(.el-form-item__label) {
  font-weight: 600;
  color: #2d3748;
  font-size: 1.1rem;
}

.create-post-body :deep(.el-input__wrapper) {
  border-radius: 12px;
  box-shadow: 0 0 0 1px #e2e8f0;
}

.create-post-body :deep(.el-input__wrapper:hover) {
  box-shadow: 0 0 0 1px #667eea;
}

.create-post-body :deep(.el-input__wrapper.is-focus) {
  box-shadow: 0 0 0 1px #667eea;
}

.create-post-body :deep(.el-textarea__inner) {
  font-family: 'Georgia', 'Times New Roman', serif;
  line-height: 1.6;
  border-radius: 12px;
  box-shadow: 0 0 0 1px #e2e8f0;
}

.create-post-body :deep(.el-textarea__inner:hover) {
  box-shadow: 0 0 0 1px #667eea;
}

.create-post-body :deep(.el-textarea__inner:focus) {
  box-shadow: 0 0 0 1px #667eea;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .create-post-container {
    padding: 100px 1rem 2rem;
  }
  
  .create-post-card {
    max-width: 100%;
  }
  
  .create-post-card :deep(.el-card__header) {
    padding: 2rem 2rem 1.5rem;
  }
  
  .create-post-header h1 {
    font-size: 2rem;
  }
  
  .create-post-body {
    padding: 2rem;
  }
}

@media (max-width: 480px) {
  .create-post-card :deep(.el-card__header) {
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