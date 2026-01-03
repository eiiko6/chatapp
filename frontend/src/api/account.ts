import { UpdateUserResponse } from '../types';
import { apiFetch } from './client'
// import type { User } from '../types'

export function updateAccount(username: string, email: string, password: string) {
  return apiFetch<UpdateUserResponse>('/account', {
    method: 'PUT',
    body: JSON.stringify({ username, email, password }),
  });
}
