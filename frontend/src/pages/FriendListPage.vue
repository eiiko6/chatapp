<template>
  <div class="friends-page">
    <header class="friends-header">
      <h1>Your friends</h1>

      <form class="friend-request-form" @submit.prevent="send">
        <h3>Add Friend</h3>
        <div class="input-container">
          <input v-model="username" placeholder="Username" autofocus />
          <button type="submit">Send Request</button>
        </div>
        <p v-if="errorMessage" class="error-message">{{ errorMessage }}</p>
      </form>
    </header>

    <!-- Friends List -->
    <div class="friends-list">
      <h2>Your Friends</h2>
      <ul>
        <li v-for="friend in friends" :key="friend.uuid">
          {{ friend.username }}
        </li>
      </ul>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { fetchFriends, sendFriendRequest } from '../api/friends'
import type { Friend } from '../types'

const friends = ref<Friend[]>([])
const username = ref('')
const errorMessage = ref('')

onMounted(async () => {
  friends.value = await fetchFriends()
})

async function send() {
  if (!username.value) {
    // errorMessage.value = 'Username is required.'
    return
  }

  try {
    await sendFriendRequest(username.value)
    username.value = ''
    errorMessage.value = ''
    friends.value = await fetchFriends()
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

.friends-list {
  min-width: 250px;
  background: var(--panel);
  border: 1px solid var(--border);
  border-radius: var(--radius);
  padding: 1rem;
}

.friends-list ul {
  list-style: none;
  padding: 0;
}

.friends-list li {
  background: var(--panel-light);
  border: 1px solid var(--border);
  border-radius: var(--radius);
  padding: 0.75rem 1rem;
  margin-bottom: 0.5rem;
}

.friends-list h2 {
  font-size: 1.25rem;
  margin-bottom: 1rem;
}
</style>
