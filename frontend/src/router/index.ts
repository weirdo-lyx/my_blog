import { createRouter, createWebHistory } from 'vue-router'
import HomeView from '../views/HomeView.vue'
import PostView from '../views/PostView.vue'
import CreatePostView from '../views/CreatePostView.vue'
import EditPostView from '../views/EditPostView.vue' // Import the new view

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: HomeView
    },
    {
      path: '/post/:id',
      name: 'post',
      component: PostView,
      props: true
    },
    {
      path: '/create',
      name: 'create',
      component: CreatePostView
    },
    {
      path: '/post/:id/edit', // Add the new route
      name: 'edit',
      component: EditPostView,
      props: true
    }
  ]
})

export default router