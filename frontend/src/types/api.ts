export interface LoginResponse {
  uuid: string
  token: string
}

export interface Room {
  uuid: string
  owner_name: number
  name: string
  globa: boolean
}

export interface Message {
  sender: string
  message_type: 'text'
  content: string
  sent_at: string
}

