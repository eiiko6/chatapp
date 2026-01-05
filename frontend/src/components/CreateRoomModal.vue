<template>
  <div class="backdrop" @click.self="emit('close')">
    <form class="modal" @submit.prevent="submit">
      <h2>{{ $t('chat-create-title') }}</h2>

      <div class="input-group">
        <label>{{ $t('chat-create-name') }}</label>
        <input v-model="name" :placeholder="$t('chat-create-name-placeholder')" autofocus />
      </div>

      <label class="checkbox">
        <input type="checkbox" v-model="global" />
        <span>{{ $t('chat-create-global') }}</span>
      </label>

      <div class="actions">
        <button type="button" @click="emit('close')" class="secondary">
          {{ $t('shared-cancel') }}
        </button>

        <button type="submit">
          {{ $t('chat-create-submit') }}
        </button>
      </div>
    </form>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { createRoom } from '../api/rooms'
import type { Room } from '../types'

const name = ref('')
const global = ref(false)

const emit = defineEmits<{
  (e: 'created', room: Room): void
  (e: 'close'): void
}>()

async function submit() {
  const room = await createRoom(name.value, global.value)
  emit('created', room)
  emit('close')
  name.value = ''
  global.value = false
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

  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.modal h2 {
  font-size: 1.25rem;
  font-weight: 600;
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
