<template>
  <div class="room-list">
    <header class="rooms-header">
      <h2>Rooms</h2>
      <button class="create-btn" @click="showCreate = true">+</button>
    </header>

    <CreateRoomModal v-if="showCreate" @close="showCreate = false" @created="rooms.push($event)" />

    <div class="scroll-area">
      <router-link v-for="room in rooms" :key="room.uuid" :to="`/rooms/${room.uuid}`" class="room-item"
        :class="{ active: route.params.uuid === room.uuid }">
        <div class="room-info">
          <span class="room-name">{{ room.name }}</span>
          <span class="room-owner">{{ room.owner_name }}</span>
        </div>
      </router-link>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { useRoute } from 'vue-router';
import { initAuth } from '../stores/auth.ts';
import { fetchRooms } from '../api/rooms';
import type { Room } from '../types/api';
import CreateRoomModal from './CreateRoomModal.vue';

const route = useRoute();
const showCreate = ref(false);
const rooms = ref<Room[]>([]);

onMounted(async () => {
  const auth = await initAuth();
  rooms.value = await fetchRooms(auth.uuid!);
});
</script>

<style scoped>
.room-list {
  display: flex;
  flex-direction: column;
  height: 100%;
}

.rooms-header {
  padding: 1.5rem;
  display: flex;
  align-items: center;
  justify-content: space-between;
  border-bottom: 1px solid var(--border);
}

.rooms-header h2 {
  font-size: 1.1rem;
  margin: 0;
}

.scroll-area {
  flex: 1;
  overflow-y: auto;
  padding: 0.5rem;
}

.room-item {
  display: block;
  padding: 0.75rem 1rem;
  margin-bottom: 0.25rem;
  border-radius: 8px;
  text-decoration: none;
  color: var(--muted);
  transition: all 0.2s;
}

.room-item:hover {
  background: rgba(255, 255, 255, 0.05);
  color: var(--text);
}

.room-item.active {
  background: var(--accent);
  color: white;
}

.room-item.active .room-owner {
  color: rgba(255, 255, 255, 0.7);
}

.room-info {
  display: flex;
  flex-direction: column;
}

.room-name {
  font-weight: 500;
}

.room-owner {
  font-size: 0.75rem;
  opacity: 0.6;
}

.create-btn {
  width: 28px;
  height: 28px;
  border-radius: 50%;
  border: none;
  background: var(--accent);
  color: white;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
}
</style>
