import { createRouter, createWebHistory } from 'vue-router'
import { useAuthStore } from '../stores/auth'

import LoginPage from '../pages/LoginPage.vue'
import RoomsPage from '../pages/RoomsPage.vue'
import ChatPage from '../pages/ChatPage.vue'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    { path: '/login', component: LoginPage },
    { path: '/', component: RoomsPage },
    { path: '/rooms/:uuid', component: ChatPage, props: true },
  ],
})

router.beforeEach((to) => {
  const auth = useAuthStore()
  if (!auth.isAuthenticated && to.path !== '/login') {
    return '/login'
  }
})

export default router

