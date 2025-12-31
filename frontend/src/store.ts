import { apiFetch } from './api/client'
import type { LoginResponse } from './types'
import * as authStore from './authStore'

export const initAuth = authStore.getAuthData
export const getLastRoom = authStore.getLastRoom
export const setLastRoom = authStore.setLastRoom

export async function login(email: string, username: string, password: string) {
  const res: LoginResponse = await apiFetch('/login', {
    method: 'POST',
    body: JSON.stringify({ email, username, password }),
  })

  await authStore.saveAuthData(res.token, res.uuid)
  return { token: res.token, uuid: res.uuid, isAuthenticated: true }
}

export async function logout() {
  await authStore.clearAuthData()
  return { token: null, uuid: null, isAuthenticated: false }
}

export async function validateToken(): Promise<boolean> {
  const auth = await authStore.getAuthData()
  if (!auth.token) return false

  try {
    await apiFetch('/validate-token')
    return true
  } catch (e) {
    return false
  }
}
