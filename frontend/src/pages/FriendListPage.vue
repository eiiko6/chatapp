<template>
  <div class="friends-page">
    <header class="friends-header">
      <h1>Your friends</h1>

      <!-- Friend Request Form -->
      <div class="friend-request-form">
        <h3>Add Friend</h3>
        <div class="input-container">
          <input v-model="username" placeholder="Username" />
          <button @click="send">Send Request</button>
        </div>
        <p v-if="errorMessage" class="error-message">{{ errorMessage }}</p>
      </div>
    </header>

    <div class="friends-requests-container">
      <!-- Friends List -->
      <div class="friends-list">
        <h2>Your Friends</h2>
        <ul>
          <li v-for="friend in friends" :key="friend.uuid">
            {{ friend.username }}
          </li>
        </ul>
      </div>

      <!-- Friend Request List -->
      <div class="requests">
        <h2>Friend Requests</h2>
        <ul v-if="requests.length">
          <li v-for="req in requests" :key="req.sender_uuid">
            <span>{{ req.sender_username }}</span>
            <button @click="accept(req.sender_uuid)">Accept</button>
          </li>
        </ul>
        <p v-else>No pending requests</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { fetchFriends, fetchFriendRequests, acceptFriendRequest, sendFriendRequest } from '../api/friends'
import type { Friend, FriendRequest } from '../api'

const friends = ref<Friend[]>([])
const requests = ref<FriendRequest[]>([])
const username = ref('')
const errorMessage = ref('')

onMounted(async () => {
  friends.value = await fetchFriends()
  requests.value = await fetchFriendRequests()
})

async function accept(senderUuid: string) {
  try {
    await acceptFriendRequest(senderUuid)
    requests.value = requests.value.filter(r => r.sender_uuid !== senderUuid)
    fetchFriends().then(f => (friends.value = f))
  } catch (err) {
    errorMessage.value = 'An error occurred while accepting the request.' // TODO: handle this supposedly impossible case
  }
}

async function send() {
  if (!username.value) {
    // errorMessage.value = 'Username is required.'
    return
  }

  try {
    await sendFriendRequest(username.value)
    username.value = ''
    errorMessage.value = ''
  } catch (err: any) {
    errorMessage.value = err
  }
}
</script>

<style scoped>
.friends-page {
  max-width: 720px;
  margin: 0 auto;
  padding: 2rem 1.5rem;
  display: flex;
  flex-direction: column;
  gap: 2rem;
}

.friends-header {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.friend-request-form {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.input-container {
  display: flex;
  gap: 0.5rem;
}

.friend-request-form input {
  padding: 0.5rem;
  border-radius: var(--radius);
  border: 1px solid var(--border);
  flex-grow: 1;
}

.friend-request-form button {
  padding: 0.5rem 1rem;
  border-radius: var(--radius);
}

.error-message {
  color: red;
  font-size: 0.9rem;
}

.friends-requests-container {
  display: flex;
  gap: 2rem;
  flex-wrap: wrap;
}

.friends-list,
.requests {
  min-width: 250px;
  background: var(--panel);
  border: 1px solid var(--border);
  border-radius: var(--radius);
  padding: 1rem;
}

.friends-list ul,
.requests ul {
  list-style: none;
  padding: 0;
}

.friends-list li,
.requests li {
  background: var(--panel-light);
  border: 1px solid var(--border);
  border-radius: var(--radius);
  padding: 0.75rem 1rem;
  margin-bottom: 0.5rem;
}

.requests {
  max-height: 500px;
  overflow-y: auto;
}

.requests li {
  display: flex;
  justify-content: space-between;
  align-items: center;
  background: var(--panel-light);
  border: 1px solid var(--border);
  border-radius: var(--radius);
  padding: 0.75rem 1rem;
  margin-bottom: 0.5rem;
}

.requests li button {
  padding-top: 5px;
  padding-bottom: 5px;
}

.requests p {
  font-size: 0.9rem;
  color: gray;
}

.friends-list h2,
.requests h2 {
  font-size: 1.25rem;
  margin-bottom: 1rem;
}

@media (max-width: 720px) {
  .friends-requests-container {
    flex-direction: column;
  }
}
</style>
