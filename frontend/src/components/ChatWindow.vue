<template>
  <div v-if="uuid === 'none'" class="no-room">
    <div class="empty-state">
      <i class="fa-solid fa-comments"></i>
      <p>Select a room to start talking</p>
    </div>
  </div>

  <div v-else class="chat-container">
    <div class="messages-container" ref="messageListRef">
      <MessageList :messages="messages" />
    </div>

    <div class="input-container">
      <!-- Added ref="messageInputRef" -->
      <MessageInput ref="messageInputRef" @send="onSend" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch, nextTick } from "vue";
import { fetchMessages, sendMessage, getWsToken } from "../api/messages";
import type { Message } from "../types/api";
import { API_WS } from '../main.ts'
import MessageList from "./MessageList.vue";
import MessageInput from "./MessageInput.vue";

const props = defineProps<{ uuid: string }>();
const messages = ref<Message[]>([]);
const messageListRef = ref<HTMLElement | null>(null);
const messageInputRef = ref<InstanceType<typeof MessageInput> | null>(null); // Ref for focusing
let socket: WebSocket | null = null;

// Handle global Enter key to focus input
const handleGlobalKeyDown = (event: KeyboardEvent) => {
  if (event.key === 'Enter') {
    const active = document.activeElement?.tagName.toLowerCase();

    // Don't hijack focus if user is already typing in an input or textarea
    const isTyping = active === 'input' || active === 'textarea';

    if (!isTyping && messageInputRef.value) {
      // Prevent default to avoid side effects like clicking a focused button
      event.preventDefault();
      messageInputRef.value.focus();
    }
  }
};

async function initializeRoom() {
  if (socket) {
    socket.close();
    socket = null;
  }
  messages.value = [];

  if (props.uuid === 'none') return;

  messages.value = await fetchMessages(props.uuid);
  await nextTick();
  scrollToBottom();

  const wsToken = await getWsToken(props.uuid);
  socket = new WebSocket(`${API_WS}/rooms/${props.uuid}?token=${wsToken}`);

  socket.onmessage = (event) => {
    const msg: Message = JSON.parse(event.data);
    const exists = messages.value.some(m => m.sent_at === msg.sent_at && m.sender === msg.sender);
    if (!exists) {
      messages.value.push(msg);
      nextTick().then(scrollToBottom);
    }
  };
}

async function onSend(content: string) {
  if (props.uuid === 'none') return;
  await sendMessage(props.uuid, content);
}

function scrollToBottom() {
  if (messageListRef.value) {
    messageListRef.value.scrollTop = messageListRef.value.scrollHeight;
  }
}

watch(() => props.uuid, () => {
  initializeRoom();
});

onMounted(() => {
  initializeRoom();
  window.addEventListener('keydown', handleGlobalKeyDown);
});

onUnmounted(() => {
  socket?.close();
  window.removeEventListener('keydown', handleGlobalKeyDown);
});
</script>

<style scoped>
.chat-container {
  display: flex;
  flex-direction: column;
  height: 100%;
}

.messages-container {
  flex: 1;
  overflow-y: auto;
  padding: 1.5rem;
}

.input-container {
  padding: 1rem;
  border-top: 1px solid var(--border);
}

.no-room {
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--muted);
}

.empty-state {
  text-align: center;
  font-size: 1.2rem;
}

.empty-state i {
  font-size: 3rem;
  margin-bottom: 1rem;
  opacity: 0.3;
}
</style>
