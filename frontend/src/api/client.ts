import { fetch } from '@tauri-apps/plugin-http';
import { getAuthData, clearAuthData } from '../authStore'
import { API } from '../main.ts'
import router from '../router'

export async function apiFetch<T>(
  path: string,
  options: RequestInit = {}
): Promise<T> {
  const auth = await getAuthData()

  const res = await fetch(`${API}${path}`, {
    ...options,
    method: options.method || 'GET',
    headers: {
      'Content-Type': 'application/json',
      ...(auth.token ? { Authorization: `Bearer ${auth.token}` } : {}),
      ...options.headers,
    },
  })

  if (res.status === 401 && auth.token) {
    await clearAuthData()
    router.push('/login')
    throw new Error("Session expired")
  }

  // Handle error responses
  if (!res.ok) {
    const text = await res.text()
    throw new Error(text || res.statusText)
  }

  // Get the response as text first
  const responseText = await res.text()

  if (!responseText) {
    return {} as T
  }

  try {
    return JSON.parse(responseText) as T
  } catch (e) {
    return responseText as unknown as T
  }
}
