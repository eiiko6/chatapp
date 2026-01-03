import { UpdateUserResponse } from '../types';
import { apiFetch } from './client'
// import type { User } from '../types'

export function updateSettings(username: string, email: string, password: string) {
  return apiFetch<UpdateUserResponse>('/settings', {
    method: 'PUT',
    body: JSON.stringify({ username, email, password }),
  });
}
