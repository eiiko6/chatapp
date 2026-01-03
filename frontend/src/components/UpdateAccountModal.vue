<template>
  <div class="backdrop" @click.self="emit('close')">
    <form class="modal" @submit.prevent="submit">
      <h2>Update your Account</h2>
      <p class="subtitle">Only fill in the fields you wish to change.</p>

      <p v-if="errorMessage" class="error-message">{{ errorMessage }}</p>

      <div class="input-group">
        <label>Username</label>
        <input v-model="username" placeholder="New username" autofocus />
      </div>

      <div class="input-group">
        <label>Email</label>
        <input v-model="email" type="email" placeholder="New email" />
      </div>

      <div class="input-group">
        <label>New Password</label>
        <input v-model="password" type="password" placeholder="Leave blank to keep current" />
        <input v-model="confirmPassword" type="password" placeholder="Confirm new password" />
      </div>

      <div class="actions">
        <button type="button" @click="emit('close')" class="secondary" :disabled="isSubmitting">
          Cancel
        </button>

        <button type="submit" :disabled="isSubmitting">
          {{ isSubmitting ? 'Updating...' : 'Save Changes' }}
        </button>
      </div>
    </form>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { updateAccount } from '../api/account'
import { updateLocalUser } from '../authStore'
import type { User } from '../types'

const props = defineProps<{ user: User | null }>()
const emit = defineEmits(['close', 'updated'])

const errorMessage = ref('')
const isSubmitting = ref(false)

const username = ref(props.user?.username || '')
const email = ref(props.user?.email || '')
const password = ref('')
const confirmPassword = ref('')

async function submit() {
  errorMessage.value = ''

  if (password.value !== confirmPassword.value) {
    errorMessage.value = "Passwords do not match."
    return
  }

  // Prevent empty submission
  if (!username.value || !email.value) {
    errorMessage.value = "Username and Email are required."
    return
  }

  isSubmitting.value = true
  errorMessage.value = ''

  try {
    const updatedData = await updateAccount(
      username.value.trim(),
      email.value.trim(),
      password.value
    )

    await updateLocalUser(updatedData, props.user?.uuid)

    emit('updated')
    emit('close')
  } catch (err: any) {
    errorMessage.value = err?.message || 'Update failed'
  } finally {
    isSubmitting.value = false
  }
}
</script>

<style scoped>
.backdrop {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.7);
  backdrop-filter: blur(4px);
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
  display: flex;
  flex-direction: column;
  gap: 1.2rem;
  box-shadow: 0 10px 25px rgba(0, 0, 0, 0.3);
}

.subtitle {
  font-size: 0.85rem;
  color: var(--muted);
  margin-top: -0.5rem;
  margin-bottom: 0.5rem;
}

.input-group {
  display: flex;
  flex-direction: column;
  gap: 0.3rem;
}

.input-group label {
  font-size: 0.8rem;
  font-weight: 600;
  color: var(--muted);
}

.error-message {
  /* background: rgba(255, 0, 0, 0.1); */
  /* border: 1px solid red; */
  /* padding: 0.5rem; */
  /* border-radius: 4px; */
  color: var(--error);
  font-size: 0.9rem;
}

.actions {
  display: flex;
  justify-content: flex-end;
  gap: 0.5rem;
  margin-top: 1rem;
}

button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.secondary {
  background: transparent;
  color: var(--text);
  border: 1px solid var(--border);
}
</style>
