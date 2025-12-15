<script setup lang="ts">
import { ref } from 'vue'
import { createRoom } from '../api/rooms'
import type { Room } from '../types/api'

const name = ref('')
const emit = defineEmits<{ (e: 'created', room: Room): void }>()

async function submit() {
  const room = await createRoom(name.value)
  emit('created', room)
  name.value = ''
}
</script>

<template>
  <div>
    <input v-model="name" placeholder="room name" />
    <button @click="submit">Create</button>
  </div>
</template>
