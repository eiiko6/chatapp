import { load, Store } from '@tauri-apps/plugin-store'
import type { LoginResponse } from './types'
import { apiFetch } from './api/client'

let store: Store | null = null

async function getStore() {
  if (!store) {
    store = await load('store.json')
  }
  return store
}

export async function setLastRoom(uuid: string) {
  if (!uuid || uuid === 'none') return

  const s = await getStore()
  await s.set('last_room_uuid', uuid)
  await s.save()
}

export async function getLastRoom(): Promise<string | null> {
  const s = await getStore()
  const lastRoom = await s.get<string>('last_room_uuid')
  return lastRoom ?? null
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

export async function validateToken(): Promise<boolean> {
  const auth = await initAuth()
  if (!auth.token) return false

  try {
    await apiFetch('/validate-token')
    return true
  } catch (e: any) {
    // Only logout if token is bad
    if (e.message.includes('401')) {
      await logout()
      return false
    }
    return true
  }
}
