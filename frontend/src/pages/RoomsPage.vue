<template>
  <div class="rooms-page">
    <header class="rooms-header">
      <h1>Your rooms</h1>
      <CreateRoomForm @created="rooms.push($event)" />
    </header>

    <ul class="rooms-list">
      <li v-for="room in rooms" :key="room.uuid" class="room-item">
        <router-link class="room-link" :to="`/rooms/${room.uuid}`">
          {{ room.name }}
        </router-link>
      </li>
    </ul>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { useAuthStore } from '../stores/auth'
import { fetchRooms } from '../api/rooms'
import type { Room } from '../types/api'

const auth = useAuthStore()
const rooms = ref<Room[]>([])

onMounted(async () => {
  rooms.value = await fetchRooms(auth.uuid!)
})
</script>

<script lang="ts">
import CreateRoomForm from '../components/CreateRoomForm.vue'
export default { components: { CreateRoomForm } }
</script>

<style scoped>
.rooms-page {
  max-width: 720px;
  margin: 0 auto;
  padding: 2rem 1.5rem;

  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.rooms-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 1rem;
}

.rooms-header h1 {
  font-size: 1.5rem;
  font-weight: 600;
}

.rooms-list {
  list-style: none;
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.room-item {
  background: var(--panel);
  border: 1px solid var(--border);
  border-radius: var(--radius);
}

.room-link {
  display: block;
  padding: 0.75rem 1rem;
  color: var(--text);
  text-decoration: none;
}

.room-link:hover {
  background: rgba(255, 255, 255, 0.03);
}
</style>
