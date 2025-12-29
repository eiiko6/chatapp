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
  uuid: string
  sender: string
  message_type: 'text'
  content: string
  sent_at: string
}

export interface Friend {
  uuid: string
  username: string
}

export interface FriendRequest {
  sender_uuid: string
  sender_username: string
}
