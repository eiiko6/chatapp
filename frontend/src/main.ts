import { createApp } from 'vue'
import router from './router'
import App from './App.vue'

import './base.css'

createApp(App)
  .use(router)
  .mount('#app')

export const API = 'http://127.0.0.1:8080'
export const API_WS = 'ws://127.0.0.1:8080/ws'
