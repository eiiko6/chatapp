<template>
  <div class="backdrop" @click.self="emit('close')">
    <form class="modal" @submit.prevent="submit">
      <h2>Invite People</h2>

      <p v-if="errorMessage" class="error-message">{{ errorMessage }}</p>

      <input v-model="receiverUsername" placeholder="username" autofocus />

      <label class="checkbox">
        <input type="checkbox" v-model="requestFriend" />
        <span>Also send a friend request</span>
      </label>

      <div class="actions">
        <button type="button" @click="emit('close')" class="secondary">
          Cancel
        </button>

        <button type="submit">
          Send
        </button>
      </div>
    </form>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { sendRoomInvite } from '../api/rooms'
import { sendFriendRequest } from '../api/friends';

const props = defineProps<{ room_uuid: string }>();

const errorMessage = ref('')
const receiverUsername = ref('')
const requestFriend = ref(false)

const emit = defineEmits<{
  (e: 'close'): void
}>()

async function submit() {
  errorMessage.value = ''
  const username = receiverUsername.value.trim()

  if (!username) {
    // errorMessage.value = 'Username is required.'
    return
  }

  try {
    await sendRoomInvite(username, props.room_uuid)

    if (requestFriend.value) {
      await sendFriendRequest(username)
    }

    receiverUsername.value = ''
    requestFriend.value = false
    emit('close')

  } catch (err: any) {
    console.error(err)
    errorMessage.value = err?.message || err || 'An error occurred'
  }
}
</script>

<style scoped>
.backdrop {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.6);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.modal {
  background: var(--panel);
  border: 1px solid var(--border);
  border-radius: var(--radius);
  padding: 1.5rem;
  width: 100%;
  max-width: 420px;
  max-height: 500px;

  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.modal h2 {
  font-size: 1.25rem;
  font-weight: 600;
}

.error-message {
  color: red;
  font-size: 0.9rem;
}

.checkbox {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  color: var(--muted);
  font-size: 0.9rem;
}

.actions {
  display: flex;
  justify-content: flex-end;
  gap: 0.5rem;
}

.secondary {
  background: transparent;
  color: var(--text);
  border: 1px solid var(--border);
}

.secondary:hover {
  background: rgba(255, 255, 255, 0.05);
}
</style>
