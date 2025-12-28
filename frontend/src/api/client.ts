import { initAuth, logout } from '../stores/auth.ts'
import { API } from '../main.ts'
import router from '../router'

export async function apiFetch<T>(
  path: string,
  options: RequestInit = {}
): Promise<T> {
  const auth = await initAuth()

  const res = await fetch(`${API}${path}`, {
    ...options,
    headers: {
      'Content-Type': 'application/json',
      ...(auth.token ? { Authorization: `Bearer ${auth.token}` } : {}),
      ...options.headers,
    },
  })

  if (res.status === 401) {
    await logout()
    router.push('/login')
    throw new Error("Session expired")
  }

  if (!res.ok) {
    const text = await res.text()
    throw new Error(text || res.statusText)
  }

  return res.json()
}
