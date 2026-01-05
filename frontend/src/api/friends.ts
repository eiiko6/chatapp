import { apiFetch } from './client'
import type { Friend, FriendRequest } from '../types'

export function fetchFriends() {
  return apiFetch<Friend[]>('/friends')
}

export function fetchFriendRequests() {
  return apiFetch<FriendRequest[]>('/friends/requests')
}

export function sendFriendRequest(receiverUsername: string) {
  return apiFetch<void>('/friends/request', {
    method: 'POST',
    body: JSON.stringify({ receiver_username: receiverUsername }),
  });
}

export function acceptFriendRequest(senderUuid: string) {
  return apiFetch<void>('/friends/accept', {
    method: 'POST',
    body: JSON.stringify({ sender_uuid: senderUuid }),
  })
}

export function declineFriendRequest(senderUuid: string) {
  return apiFetch<void>('/friends/decline', {
    method: 'POST',
    body: JSON.stringify({ sender_uuid: senderUuid }),
  })
}
