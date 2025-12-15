import { defineStore } from 'pinia'
import type { LoginResponse } from '../types/api'
import { apiFetch } from '../api/client'

export const useAuthStore = defineStore('auth', {
  state: () => ({
    token: null as string | null,
    uuid: null as string | null,
  }),

  getters: {
    isAuthenticated: (s) => !!s.token,
  },

  actions: {
    async login(email: string, username: string, password: string) {
      const res = await apiFetch<LoginResponse>('/login', {
        method: 'POST',
        body: JSON.stringify({ email, username, password }),
      })

      this.token = res.token
      this.uuid = res.uuid
    },

    logout() {
      this.token = null
      this.uuid = null
    },
  },
})

