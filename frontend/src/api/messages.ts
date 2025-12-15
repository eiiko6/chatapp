import { apiFetch } from './client'
import type { Message } from '../types/api'

export function fetchMessages(roomUuid: string) {
  return apiFetch<Message[]>(`/messages/${roomUuid}`)
}

export function sendMessage(roomUuid: string, content: string) {
  return apiFetch<Message>(`/messages/${roomUuid}`, {
    method: 'POST',
    body: JSON.stringify({
      message_type: 'text',
      content,
    }),
  })
}

