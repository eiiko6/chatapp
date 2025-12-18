<template>
  <div class="rooms-page">
    <header class="rooms-header">
      <h1>Your rooms</h1>
      <button @click="showCreate = true">Create room</button>
    </header>

    <CreateRoomModal v-if="showCreate" @close="showCreate = false" @created="rooms.push($event)" />


    <ul class="rooms-list">
      <li v-for="room in rooms" :key="room.uuid" class="room-item">
        <router-link class="room-link" :to="`/rooms/${room.uuid}`">
          {{ room.name }} <span class="owner">{{ room.owner_name }}</span>
        </router-link>
      </li>
    </ul>

    <button class="logout" @click="logout()">Logout</button>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { useRouter } from 'vue-router'
import { initAuth, logout as authLogout } from '../stores/auth.ts'
import { fetchRooms } from '../api/rooms'
import type { Room } from '../types/api'
import CreateRoomModal from '../components/CreateRoomModal.vue'

const showCreate = ref(false)


const router = useRouter()

const rooms = ref<Room[]>([])

onMounted(async () => {
  const auth = await initAuth()
  rooms.value = await fetchRooms(auth.uuid!)
})

function logout() {
  authLogout()
  router.push('/login')
}
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

.logout {
  position: absolute;
  left: 0;
  top: 0;
  margin: 30px;
}

.owner {
  font-weight: normal;
  opacity: 0.7;
  font-size: 0.85rem;
  margin-left: 0.5rem;
}
</style>
