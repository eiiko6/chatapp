import { load, Store } from '@tauri-apps/plugin-store'
import type { LoginResponse } from '../types/api'
import { apiFetch } from '../api/client'

let store: Store | null = null

async function getStore() {
  if (!store) {
    store = await load('store.json', { autoSave: false })
  }
  return store
}

export async function initAuth() {
  const s = await getStore()
  const token = await s.get<string>('token')
  const uuid = await s.get<string>('uuid')
  return { token: token || null, uuid: uuid || null, isAuthenticated: !!token }
}

export async function login(email: string, username: string, password: string) {
  const res: LoginResponse = await apiFetch('/login', {
    method: 'POST',
    body: JSON.stringify({ email, username, password }),
  })

  const s = await getStore()
  await s.set('token', res.token)
  await s.set('uuid', res.uuid)
  await s.save()

  return { token: res.token, uuid: res.uuid, isAuthenticated: true }
}

export async function logout() {
  const s = await getStore()
  await s.delete('token')
  await s.delete('uuid')
  await s.save()
  return { token: null, uuid: null, isAuthenticated: false }
}
