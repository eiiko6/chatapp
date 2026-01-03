import { load, Store } from '@tauri-apps/plugin-store'
import { UpdateUserResponse, User } from './types'

let store: Store | null = null

async function getStore() {
  if (!store) store = await load('store.json')
  return store
}

export async function getAuthData() {
  const s = await getStore()
  const token = await s.get<string>('token')
  const user = await s.get<User>('user')
  return { token: token || null, user: user || null, isAuthenticated: !!token }
}

export async function saveAuthData(token: string, user: User) {
  const s = await getStore()
  await s.set('token', token)
  await s.set('user', user)
  await s.save()
}

export async function clearAuthData() {
  const s = await getStore()
  await s.delete('token')
  await s.delete('user')
  await s.delete('last_room_uuid')
  await s.save()
}

export async function updateLocalUser(newData: UpdateUserResponse, uuid?: string) {
  const updatedUser = {
    username: newData.username,
    email: newData.email,
    uuid
  }

  const s = await getStore()
  await s.set('user', updatedUser)
  await s.save()
}

export async function setLastRoom(uuid: string) {
  if (!uuid || uuid === 'none') return
  const s = await getStore()
  await s.set('last_room_uuid', uuid)
  await s.save()
}

export async function getLastRoom(): Promise<string | null> {
  const s = await getStore()
  return (await s.get<string>('last_room_uuid')) ?? null
}
