import { createRouter, createWebHistory } from 'vue-router'
import { initAuth, getLastRoom, setLastRoom } from './store'

import LoginPage from './pages/LoginPage.vue'
import ChatPage from './pages/ChatPage.vue'
import FriendListPage from './pages/FriendListPage.vue'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    { path: '/', name: 'root', component: { render: () => null } },
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

  // Handle the redirect from "/" to the last room
  if (to.path === '/') {
    const lastRoom = await getLastRoom()
    return `/rooms/${lastRoom || 'none'}`
  }

  return true
})

// Save the room UUID to storage after every successful navigation to a chat room
router.afterEach((to) => {
  if (to.name === 'chat' && to.params.uuid) {
    setLastRoom(to.params.uuid as string)
  }
})

export default router
