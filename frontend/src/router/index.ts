import { createRouter, createWebHistory } from 'vue-router'
import { initAuth } from '../stores/auth.ts'

import LoginPage from '../pages/LoginPage.vue'
import ChatPage from '../pages/ChatPage.vue'
import FriendListPage from '../pages/FriendListPage.vue'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    { path: '/', redirect: '/rooms/none' },
    {
      path: '/login',
      name: 'login',
      component: LoginPage,
      meta: { hideNavbar: true }
    },
    {
      path: '/rooms/:uuid',
      name: 'chat',
      component: ChatPage,
      props: true
    },
    { path: '/friendlist', component: FriendListPage }
  ],
})

router.beforeEach(async (to) => {
  if (to.path === '/login') return true

  const auth = await initAuth()

  if (!auth.isAuthenticated) {
    return '/login'
  }

  return true
})

export default router
