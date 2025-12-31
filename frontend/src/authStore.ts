import { load, Store } from '@tauri-apps/plugin-store'

let store: Store | null = null

async function getStore() {
  if (!store) store = await load('store.json')
  return store
}

export async function getAuthData() {
  const s = await getStore()
  const token = await s.get<string>('token')
  const uuid = await s.get<string>('uuid')
  return { token: token || null, uuid: uuid || null, isAuthenticated: !!token }
}

export async function saveAuthData(token: string, uuid: string) {
  const s = await getStore()
  await s.set('token', token)
  await s.set('uuid', uuid)
  await s.save()
}

export async function clearAuthData() {
  const s = await getStore()
  await s.delete('token')
  await s.delete('uuid')
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
