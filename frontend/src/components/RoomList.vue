<template>
  <div class="room-list">
    <header class="rooms-header">
      <h2>Rooms</h2>
      <button class="create-btn" @click="showCreate = true" title="Create a room"><i
          class="fa-solid fa-plus"></i></button>
    </header>

    <Teleport to="body">
      <CreateRoomModal v-if="showCreate" @close="showCreate = false" @created="rooms.push($event)" />
    </Teleport>

    <div class="scroll-area">
      <router-link v-for="room in rooms" :key="room.uuid" :to="`/rooms/${room.uuid}`" class="btn room-item"
        :class="{ active: route.params.uuid === room.uuid }" @click="emit('select-room')">
        <div class="room-info">
          <span class="room-name">{{ room.name }}</span>
          <span class="room-owner">by {{ room.owner_name }}</span>
        </div>
      </router-link>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { useRoute } from 'vue-router';
import { fetchRooms } from '../api/rooms';
import type { Room } from '../types';
import CreateRoomModal from './CreateRoomModal.vue';

const route = useRoute();
const showCreate = ref(false);
const rooms = ref<Room[]>([]);

const emit = defineEmits(['select-room']);

onMounted(async () => {
  rooms.value = await fetchRooms();
});
</script>

<style scoped>
.room-list {
  display: flex;
  flex-direction: column;
  height: 100%;
  user-select: none;
  -webkit-user-select: none;
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
  margin-left: 38px;
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
  color: rgba(0, 0, 0, 0.8);
}

.room-item.active .room-owner {
  color: rgba(0, 0, 0, 1);
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
  margin: 0;
  padding: 18px;
  width: 28px;
  height: 28px;
  border-radius: var(--radius);
  border: none;
  background: transparent;
  color: white;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
}

.create-btn:hover {
  background: rgba(255, 255, 255, 0.05);
}
</style>
