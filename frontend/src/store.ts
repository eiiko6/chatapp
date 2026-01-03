import { apiFetch } from './api/client'
import type { LoginResponse, User } from './types'
import * as authStore from './authStore'
import { ref, computed } from 'vue'
import { fetchFriendRequests } from './api/friends'
import { fetchRoomInvites } from './api/rooms'
import type { FriendRequest, RoomInvite } from './types'

export const initAuth = authStore.getAuthData
export const getLastRoom = authStore.getLastRoom
export const setLastRoom = authStore.setLastRoom

export async function login(email: string, username: string, password: string) {
  const res: LoginResponse = await apiFetch('/login', {
    method: 'POST',
    body: JSON.stringify({ email, username, password }),
  })

  let user: User = {
    uuid: res.uuid,
    username: res.username,
    email: res.email
  };
  await authStore.saveAuthData(res.token, user)
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

export async function register(email: string, username: string, password: string) {
  const response: LoginResponse = await apiFetch('/register', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ email, username, password })
  });

  await login(email, username, password)
  return response;
}

const requests = ref<FriendRequest[]>([])
const invites = ref<RoomInvite[]>([])

export function useNotifications() {
  const totalCount = computed(() => requests.value.length + invites.value.length)

  async function refreshNotifications() {
    const auth = await authStore.getAuthData()
    if (!auth.token) {
      return
    }

    try {
      const [fReqs, rInvs] = await Promise.all([
        fetchFriendRequests(),
        fetchRoomInvites()
      ])
      requests.value = fReqs
      invites.value = rInvs
    } catch (err) {
      console.error("Failed to fetch notifications", err)
    }
  }

  return {
    requests,
    invites,
    totalCount,
    refreshNotifications
  }
}
