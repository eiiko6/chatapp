import { createApp } from 'vue'
import router from './router.ts'
import App from './App.vue'
import { validateToken } from './store.ts'
import { fluent, setLanguage } from './i18n'

import './base.css'
import { getLocalePreference } from './authStore.ts'

async function init() {
  await validateToken()

  const app = createApp(App)
  app.use(router)
  app.use(fluent)

  const savedLocale = await getLocalePreference();
  const osLocale = navigator.language;

  if (savedLocale) {
    setLanguage(savedLocale);
  } else {
    setLanguage(osLocale);
  }

  app.mount('#app')
}

init()

export const API = 'http://127.0.0.1:8080'
// export const API = 'https://alatreon.org/chatapp'
export const API_WS = 'ws://127.0.0.1:8080/ws'
// export const API_WS = 'wss://alatreon.org/chatapp/ws'
