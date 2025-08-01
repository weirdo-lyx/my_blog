# 前端项目文档 (Vue 3 + Vite)

本文档详细解释了博客前端项目的代码结构和每一部分的功能。

## 核心技术栈

*   **框架**: Vue 3 (使用 `<script setup>` 的组合式 API)
*   **构建工具**: Vite
*   **语言**: TypeScript
*   **路由**: Vue Router
*   **HTTP 请求**: Axios
*   **状态管理**: Pinia

---

## 文件结构解析

```
frontend/
├── public/             # 存放静态资源，如图片、favicon
│   └── background.jpg  # 背景图片
├── src/
│   ├── assets/         # 存放会被处理的静态资源，如 CSS
│   │   └── global.css  # 全局样式文件
│   ├── components/     # (可选) 可复用的小组件
│   ├── router/         # 路由配置
│   │   └── index.ts    # 路由定义
│   ├── stores/         # Pinia 状态管理
│   ├── views/          # 页面级组件 (路由渲染的目标)
│   │   ├── HomeView.vue
│   │   ├── PostView.vue
│   │   ├── CreatePostView.vue
│   │   └── EditPostView.vue
│   ├── App.vue         # 应用的根组件
│   └── main.ts         # 应用的入口文件
├── index.html          # 应用的 HTML 入口
├── package.json        # 项目依赖和脚本配置
└── vite.config.ts      # Vite 构建工具的配置文件
```

---

## 代码详解

### 1. `package.json` (项目依赖)

定义了项目所需的 npm 包和可以运行的脚本命令。

```json
"dependencies": {
  "axios": "^1.11.0",      // 用于发送 HTTP 请求到后端
  "pinia": "^2.2.0",       // Vue 的官方状态管理库
  "vue": "^3.5.18",        // Vue 框架核心
  "vue-router": "^4.5.1"  // Vue 的官方路由库
},
"scripts": {
  "dev": "vite",           // 启动开发服务器
  "build": "...",        // 构建生产版本
  "preview": "vite preview"// 预览生产版本
}
```

### 2. `index.html` (HTML 入口)

这是浏览器加载的第一个文件，也是整个 Vue 应用的挂载点。

```html
<body>
  <!-- Vue 应用将会被挂载到这个 div 中 -->
  <div id="app"></div>
  <!-- 引入应用的 JavaScript 入口文件 -->
  <script type="module" src="/src/main.ts"></script>
</body>
```

### 3. `src/main.ts` (应用入口)

这是应用的 JavaScript/TypeScript 入口。它负责创建和配置 Vue 实例。

```typescript
// 1. 导入全局 CSS，确保样式在应用加载前生效
import './assets/global.css'

// 2. 从 vue 导入 createApp 函数，用于创建应用实例
import { createApp } from 'vue'
// 3. 从 pinia 导入 createPinia 函数，用于创建状态管理实例
import { createPinia } from 'pinia'

// 4. 导入根组件 App 和路由配置 router
import App from './App.vue'
import router from './router'

// 5. 创建 Vue 应用实例
const app = createApp(App)

// 6. 使用插件。告诉 Vue 实例我们要使用 Pinia 和 Vue Router
app.use(createPinia())
app.use(router)

// 7. 将应用实例挂载到 index.html 中 id 为 'app' 的元素上
app.mount('#app')
```

### 4. `src/router/index.ts` (路由配置)

这个文件定义了应用的所有页面路径和它们对应的组件。

```typescript
import { createRouter, createWebHistory } from 'vue-router'
// 导入所有页面级组件
import HomeView from '../views/HomeView.vue'
// ... 其他组件

const router = createRouter({
  // 使用 HTML5 History 模式，URL 看起来更美观 (没有 #)
  history: createWebHistory(import.meta.env.BASE_URL),
  // 定义路由规则数组
  routes: [
    {
      path: '/', // URL 路径
      name: 'home', // 路由名字，方便在代码中引用
      component: HomeView // 当访问 / 时，渲染 HomeView 组件
    },
    {
      path: '/post/:id', // `:id` 是一个动态参数
      name: 'post',
      component: PostView,
      props: true // 将 URL 中的参数 (如 id) 作为 props 传递给组件
    },
    // ... 其他路由
  ]
})

export default router
```

### 5. `src/App.vue` (根组件)

这是所有页面内容的“外壳”或“布局”。它通常包含不变的元素，如导航栏、页脚等。

```vue
<template>
  <!-- 这是整个应用的容器，应用了背景图 -->
  <div id="app-container">
    <!-- 导航栏 -->
    <header>
      <div class="wrapper">
        <h1>博客</h1>
        <nav>
          <!-- `<RouterLink>` 是 Vue Router 提供的组件，用于生成导航链接 -->
          <RouterLink to="/">主页</RouterLink>
          <RouterLink to="/create">创建新文章</RouterLink>
        </nav>
      </div>
    </header>

    <!-- 主要内容区域 -->
    <main>
      <!-- `<RouterView>` 是一个占位符，Vue Router 会在这里渲染当前 URL 匹配到的组件 -->
      <RouterView />
    </main>
  </div>
</template>

<script setup lang="ts">
// 从 vue-router 导入导航和视图组件
import { RouterLink, RouterView } from 'vue-router'
</script>

<style>
/* 这里是定义应用布局和全局视觉效果的 CSS */
/* 例如，#app-container 的背景图样式，header 和 main 的半透明背景等 */
</style>
```

### 6. `src/views/HomeView.vue` (主页视图)

这是一个典型的页面组件，展示了如何从后端获取数据并渲染列表。

```vue
<template>
  <div class="home">
    <!-- 使用 v-for 指令来遍历 posts 数组，为每篇文章创建一个列表项 -->
    <ul class="post-list">
      <li v-for="post in posts" :key="post.id">
        <!-- 使用 RouterLink 跳转到文章详情页 -->
        <router-link :to="{ name: 'post', params: { id: post.id } }">{{ post.title }}</router-link>
        <!-- 点击按钮时，调用 deletePost 方法 -->
        <button @click="deletePost(post.id)" class="delete-button">删除</button>
      </li>
    </ul>
  </div>
</template>

<script setup lang="ts">
// 从 vue 导入 ref (用于创建响应式变量) 和 onMounted (生命周期钩子)
import { ref, onMounted } from 'vue'
// 导入 axios 用于发送 HTTP 请求
import axios from 'axios'

// 定义 Post 接口，用于 TypeScript 类型检查
interface Post {
  id: number
  title: string
  content: string
}

// 使用 ref 创建一个响应式的 posts 数组。当它改变时，模板会自动更新
const posts = ref<Post[]>([])

// 定义一个异步函数来获取文章列表
const fetchPosts = async () => {
  try {
    // 使用 axios 发送 GET 请求到后端 API
    const response = await axios.get('http://127.0.0.1:3000/posts')
    // 将获取到的数据赋值给 posts.value
    posts.value = response.data
  } catch (error) {
    console.error('获取文章列表失败:', error)
  }
}

// 定义删除文章的函数
const deletePost = async (id: number) => {
  // 弹出确认框，防止误删
  if (confirm('确定要删除这篇文章吗？')) {
    try {
      // 发送 DELETE 请求到后端
      await axios.delete(`http://127.0.0.1:3000/posts/${id}`)
      // 删除成功后，重新获取文章列表以刷新页面
      await fetchPosts()
    } catch (error) {
      console.error(`删除文章 (id: ${id}) 失败:`, error)
    }
  }
}

// `onMounted` 是一个生命周期钩子，它会在组件被挂载到 DOM 上之后运行
// 我们在这里调用 fetchPosts()，以确保页面一加载就去获取数据
onMounted(fetchPosts)
</script>

<style scoped>
/* `scoped` 属性意味着这里的样式只对当前组件生效，不会影响到其他组件 */
</style>
```

其他视图组件 (`PostView.vue`, `CreatePostView.vue`, `EditPostView.vue`) 的结构与 `HomeView.vue` 类似，都遵循着“定义模板 -> 编写脚本逻辑 -> 添加局部样式”的模式，只是具体的业务逻辑（获取单篇文章、创建、更新）有所不同。
