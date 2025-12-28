<template>
  <div class="chat-layout">
    <aside class="sidebar">
      <RoomList />
    </aside>

    <main class="chat-window-container">
      <ChatWindow :uuid="uuid" />
    </main>
  </div>
</template>

<script setup lang="ts">
defineProps<{ uuid: string }>();
import RoomList from "../components/RoomList.vue";
import ChatWindow from "../components/ChatWindow.vue";
</script>

<style scoped>
.chat-layout {
  display: flex;
  height: 80vh;
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
}

.chat-window-container {
  flex: 1;
  display: flex;
  flex-direction: column;
  min-width: 0;
}

@media (max-width: 720px) {
  .sidebar {
    display: none;
  }
}
</style>
