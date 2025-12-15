export interface LoginResponse {
  uuid: string
  token: string
}

export interface Room {
  uuid: string
  owner: number
  name: string
}

export interface Message {
  sender: string
  message_type: 'text'
  content: string
}

