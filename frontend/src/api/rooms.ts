import { apiFetch } from './client'
import type { Room, RoomInvite } from '../types'

export function fetchRooms() {
  return apiFetch<Room[]>(`/rooms`)
}

export function createRoom(name: string, global: boolean) {
  return apiFetch<Room>('/rooms', {
    method: 'POST',
    body: JSON.stringify({ name, global }),
  })
}

export function fetchRoomInvites() {
  return apiFetch<RoomInvite[]>('/rooms/invites')
}

export function sendRoomInvite(receiverUsername: string, roomUuid: string) {
  return apiFetch<void>('/rooms/invite', {
    method: 'POST',
    body: JSON.stringify({ receiver_username: receiverUsername, room_uuid: roomUuid }),
  });
}

export function acceptRoomInvite(senderUuid: string, roomUuid: string) {
  return apiFetch<void>('/rooms/join', {
    method: 'POST',
    body: JSON.stringify({ sender_uuid: senderUuid, room_uuid: roomUuid }),
  })
}
