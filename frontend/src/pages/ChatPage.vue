<template>
  <div class="chat-layout">
    <button class="menu-toggle" :class="{ 'sidebar-closed': !isSidebarOpen }" @click="isSidebarOpen = !isSidebarOpen">
      <i class="fa-solid fa-bars"></i>
    </button>

    <aside class="sidebar" :class="{ 'is-open': isSidebarOpen }">
      <div class="sidebar-content">
        <RoomList @select-room="handleRoomSelection" />
      </div>
    </aside>

    <div v-if="isSidebarOpen" class="sidebar-overlay" @click="isSidebarOpen = false"></div>

    <main class="chat-window-container" :class="{ 'sidebar-is-open': isSidebarOpen }">
      <ChatWindow :uuid="uuid" />
    </main>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import RoomList from "../components/RoomList.vue";
import ChatWindow from "../components/ChatWindow.vue";

defineProps<{ uuid: string }>();
const isSidebarOpen = ref(true);

const handleRoomSelection = () => {
  if (window.innerWidth <= 720) {
    isSidebarOpen.value = false;
  }
};
</script>

<style scoped>
.chat-layout {
  position: relative;
  display: flex;
  height: 100%;
  width: 100%;
  max-width: 1200px;
  background: var(--panel);
  border: 1px solid var(--border);
  border-radius: var(--radius);
  overflow: hidden;
}

.sidebar {
  width: 300px;
  border-right: 1px solid var(--border);
  display: flex;
  flex-direction: column;
  background: var(--panel);
  transition: width 0.3s ease, transform 0.3s ease;
  z-index: 20;
  overflow: hidden;
}

.sidebar:not(.is-open) {
  width: 0;
  border-right: none;
}

.sidebar-content {
  width: 300px;
  height: 100%;
}

.chat-window-container {
  flex: 1;
  padding-left: 38px;
  display: flex;
  flex-direction: column;
  min-width: 0;
  transition: all 0.3s ease;
}

.menu-toggle {
  position: absolute;
  top: 24px;
  left: 15px;
  z-index: 30;
  background: var(--panel);
  border-radius: 4px;
  cursor: pointer;
  padding: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: left 0.3s ease;
}

.menu-toggle.sidebar-closed {
  left: 15px;
}

.menu-toggle i {
  font-size: 1.6rem;
}

@media (min-width: 721px) {
  .chat-window-container.sidebar-is-open {
    padding-left: 0;
  }
}

@media (max-width: 720px) {
  .sidebar {
    position: absolute;
    left: 0;
    top: 0;
    bottom: 0;
    width: 280px;
    transform: translateX(-100%);
  }

  .sidebar.is-open {
    transform: translateX(0);
    width: 280px;
  }

  .sidebar:not(.is-open) {
    width: 280px;
    transform: translateX(-100%);
  }

  .sidebar-content {
    width: 280px;
  }

  .sidebar-overlay {
    position: absolute;
    inset: 0;
    background: rgba(0, 0, 0, 0.3);
    z-index: 15;
  }

  .menu-toggle:hover i {
    color: var(--muted);
  }
}
</style>
