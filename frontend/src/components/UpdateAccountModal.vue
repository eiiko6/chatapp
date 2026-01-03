<template>
  <div class="backdrop" @click.self="emit('close')">
    <form class="modal" @submit.prevent="submit">
      <h2>{{ $t('settings-account-update-modal-title') }}</h2>
      <p class="subtitle">{{ $t('settings-account-update-subtitle') }}</p>

      <p v-if="errorMessage" class="error-message">{{ errorMessage }}</p>

      <div class="input-group">
        <label>{{ $t('auth-username') }}</label>
        <input v-model="username" :placeholder="$t('auth-username')" autofocus />
      </div>

      <div class="input-group">
        <label>{{ $t('auth-email') }}</label>
        <input v-model="email" type="email" :placeholder="$t('auth-email')" />
      </div>

      <div class="input-group">
        <label>{{ $t('settings-new-password') }}</label>
        <input v-model="password" type="password" :placeholder="$t('settings-new-password')" />
        <input v-model="confirmPassword" type="password" :placeholder="$t('settings-new-password-confirm')" />
      </div>

      <div class="actions">
        <button type="button" @click="emit('close')" class="secondary" :disabled="isSubmitting">
          {{ $t('shared-cancel') }}
        </button>

        <button type="submit" :disabled="isSubmitting">
          {{ isSubmitting ? $t('settings-updating') : $t('settings-update-save') }}
        </button>
      </div>
    </form>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { updateSettings } from '../api/account'
import { updateLocalUser } from '../authStore'
import type { User } from '../types'
import { useFluent } from 'fluent-vue';

const { $t } = useFluent();

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
    errorMessage.value = $t('auth-error-password-match')
    return
  }

  // Prevent empty submission
  if (!username.value || !email.value) {
    errorMessage.value = $t('settings-error-required')
    return
  }

  isSubmitting.value = true
  errorMessage.value = ''

  try {
    const updatedData = await updateSettings(
      username.value.trim(),
      email.value.trim(),
      password.value
    )

    await updateLocalUser(updatedData, props.user?.uuid)

    emit('updated')
    emit('close')
  } catch (err: any) {
    errorMessage.value = err?.message || $t('settings-error-failed')
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
