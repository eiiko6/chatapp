<template>
  <div class="notifications-page">
    <h1>{{ $t('notifications-title') }}</h1>
    <p v-if="errorMessage" class="error-message">{{ errorMessage }}</p>

    <!-- Friend Request List -->
    <div class="list">
      <h2>{{ $t('notifications-friend-requests') }}</h2>
      <ul v-if="requests.length">
        <li v-for="req in requests" :key="req.sender_uuid">
          <span>{{ req.sender_username }}</span>
          <div class="actions">
            <button @click="acceptFriend(req.sender_uuid)">{{ $t('notifications-accept') }}</button>
            <button class="decline-btn" @click="declineFriend(req.sender_uuid)">{{ $t('notifications-decline')
            }}</button>
          </div>
        </li>
      </ul>
      <p v-else>{{ $t('notifications-no-requests') }}</p>
    </div>

    <!-- Room Invites List -->
    <div class="list">
      <h2>{{ $t('notifications-room-invites') }}</h2>
      <ul v-if="invites.length">
        <li v-for="inv in invites" :key="inv.sender_uuid">
          <span>{{ inv.room_name }}</span>
          <span>{{ $t('notifications-invite-from', { user: inv.sender_username }) }}</span>
          <div class="actions">
            <button @click="acceptRoom(inv.sender_uuid, inv.room_uuid)">{{ $t('notifications-join') }}</button>
            <button class="decline-btn" @click="declineRoom(inv.sender_uuid, inv.room_uuid)">{{
              $t('notifications-decline') }}</button>
          </div>
        </li>
      </ul>
      <p v-else>{{ $t('notifications-no-invites') }}</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { fetchFriendRequests, acceptFriendRequest, declineFriendRequest } from '../api/friends'
import { fetchRoomInvites, acceptRoomInvite, declineRoomInvite } from '../api/rooms.ts'
import { useNotifications } from '../store'
import { useFluent } from 'fluent-vue';

const { $t } = useFluent();

const errorMessage = ref('')
const { requests, invites, refreshNotifications } = useNotifications()

onMounted(async () => {
  await refreshNotifications()
  requests.value = await fetchFriendRequests()
  invites.value = await fetchRoomInvites()
})

async function acceptFriend(senderUuid: string) {
  try {
    await acceptFriendRequest(senderUuid)
    requests.value = requests.value.filter(r => r.sender_uuid !== senderUuid)
    // fetchFriends().then(f => (friends.value = f))
  } catch (err) {
    errorMessage.value = $t('notifications-error-friend-accept')
  }
}

async function declineFriend(senderUuid: string) {
  try {
    await declineFriendRequest(senderUuid)
    requests.value = requests.value.filter(r => r.sender_uuid !== senderUuid)
    // fetchFriends().then(f => (friends.value = f))
  } catch (err) {
    errorMessage.value = $t('notifications-error-friend-decline')
  }
}

async function acceptRoom(senderUuid: string, roomUuid: string) {
  try {
    await acceptRoomInvite(senderUuid, roomUuid)
    invites.value = invites.value.filter(r => r.room_uuid !== roomUuid)
  } catch (err) {
    errorMessage.value = $t('notifications-error-room-accept')
    throw err
  }
}

async function declineRoom(senderUuid: string, roomUuid: string) {
  try {
    await declineRoomInvite(senderUuid, roomUuid)
    invites.value = invites.value.filter(r => r.room_uuid !== roomUuid)
  } catch (err) {
    errorMessage.value = $t('notifications-error-room-decline')
    throw err
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
  color: var(--error);
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

.decline-btn {
  color: var(--text);
  background-color: transparent;
  border: 1px solid var(--accent);
  border-radius: var(--radius);
}

.decline-btn:hover {
  background-color: rgba(255, 255, 255, 0.05);
}

@media (max-width: 720px) {
  .friend-requests-container {
    flex-direction: column;
  }
}
</style>
