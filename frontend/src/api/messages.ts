import { apiFetch } from './client'
import type { Message } from '../types'

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

export async function getWsToken(roomUuid: string): Promise<string> {
  const data = await apiFetch<{ token: string }>(`/ws/issue-token/rooms/${roomUuid}`);
  return data.token;
}
