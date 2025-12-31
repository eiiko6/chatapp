<template>
  <div class="notifications-page">
    <h1>Notifications</h1>

    <p v-if="errorMessage" class="error-message">{{ errorMessage }}</p>

    <!-- Friend Request List -->
    <div class="list">
      <h2>Friend Requests</h2>
      <ul v-if="requests.length">
        <li v-for="req in requests" :key="req.sender_uuid">
          <span>{{ req.sender_username }}</span>
          <button @click="acceptFriend(req.sender_uuid)">Accept</button>
        </li>
      </ul>
      <p v-else>No pending requests</p>
    </div>

    <!-- Room Invites List -->
    <div class="list">
      <h2>Room Invites</h2>
      <ul v-if="invites.length">
        <li v-for="inv in invites" :key="inv.sender_uuid">
          <span>{{ inv.room_name }}</span>
          <span>from: {{ inv.sender_username }}</span>
          <button @click="acceptRoom(inv.sender_uuid, inv.room_uuid)">Join</button>
        </li>
      </ul>
      <p v-else>No pending invites</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { fetchFriendRequests, acceptFriendRequest } from '../api/friends'
import { fetchRoomInvites, acceptRoomInvite } from '../api/rooms.ts'
import type { FriendRequest, RoomInvite } from '../types'

const requests = ref<FriendRequest[]>([])
const invites = ref<RoomInvite[]>([])
const errorMessage = ref('')

onMounted(async () => {
  requests.value = await fetchFriendRequests()
  invites.value = await fetchRoomInvites()
})

async function acceptFriend(senderUuid: string) {
  try {
    await acceptFriendRequest(senderUuid)
    requests.value = requests.value.filter(r => r.sender_uuid !== senderUuid)
    // fetchFriends().then(f => (friends.value = f))
  } catch (err) {
    errorMessage.value = 'An error occurred while accepting the request.' // TODO: handle this case
  }
}

async function acceptRoom(senderUuid: string, roomUuid: string) {
  try {
    await acceptRoomInvite(senderUuid, roomUuid)
    invites.value = invites.value.filter(r => r.room_uuid !== roomUuid)
    // fetchFriends().then(f => (friends.value = f))
  } catch (err) {
    errorMessage.value = 'An error occurred while accepting the invite.' // TODO: handle this case
  }
}
</script>

<style scoped>
.notifications-page {
  max-width: 720px;
  margin: 0 auto;
  padding: 2rem 1.5rem;
  display: flex;
  flex-direction: column;
  gap: 2rem;
}

.error-message {
  color: red;
  font-size: 0.9rem;
}

.friend-requests-container {
  display: flex;
  justify-content: center;
  gap: 2rem;
  flex-wrap: wrap;
}

.list {
  min-width: 250px;
  background: var(--panel);
  border: 1px solid var(--border);
  border-radius: var(--radius);
  padding: 1rem;
}

.list ul {
  list-style: none;
  padding: 0;
}

.list li {
  background: var(--panel-light);
  border: 1px solid var(--border);
  border-radius: var(--radius);
  padding: 0.75rem 1rem;
  margin-bottom: 0.5rem;
}

.list {
  max-height: 500px;
  overflow-y: auto;
}

.list li {
  display: flex;
  justify-content: space-between;
  align-items: center;
  background: var(--panel-light);
  border: 1px solid var(--border);
  border-radius: var(--radius);
  padding: 0.75rem 1rem;
  margin-bottom: 0.5rem;
}

.list li button {
  padding-top: 5px;
  padding-bottom: 5px;
}

.list p {
  font-size: 0.9rem;
  color: gray;
}

.list h2 {
  font-size: 1.25rem;
  margin-bottom: 1rem;
}

@media (max-width: 720px) {
  .friend-requests-container {
    flex-direction: column;
  }
}
</style>
