export interface User {
  uuid: string
  username: string
}

export interface LoginResponse {
  uuid: string
  token: string
}

export interface Room {
  uuid: string
  owner_name: string
  owner_uuid: string
  name: string
  global: boolean
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

export interface RoomInvite {
  room_uuid: string
  room_name: string
  sender_uuid: string
  sender_username: string
}
