import { apiFetch } from './client'
import type { Room } from '../types'

export function fetchRooms(userUuid: string) {
  return apiFetch<Room[]>(`/rooms/${userUuid}`)
}

export function createRoom(name: string, global: boolean) {
  return apiFetch<Room>('/rooms', {
    method: 'POST',
    body: JSON.stringify({ name, global }),
  })
}

