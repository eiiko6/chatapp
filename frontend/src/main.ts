import { createApp } from 'vue'
import router from './router'
import App from './App.vue'
import { validateToken } from './stores/auth.ts'

import './base.css'

async function init() {
  await validateToken()

  const app = createApp(App)
  app.use(router)
  app.mount('#app')
}

init()

export const API = 'http://127.0.0.1:8080'
export const API_WS = 'ws://127.0.0.1:8080/ws'
