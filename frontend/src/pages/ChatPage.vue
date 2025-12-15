<template>
  <div class="chat-page">
    <div class="messages-container" ref="messageListRef">
      <MessageList :messages="messages" />
    </div>

    <div class="input-container">
      <MessageInput @send="onSend" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, onUnmounted, ref, nextTick } from "vue";
import { fetchMessages, sendMessage, getWsToken } from "../api/messages";
import type { Message } from "../types/api";
import MessageList from "../components/MessageList.vue";
import MessageInput from "../components/MessageInput.vue";
import { API_WS } from '../main.ts'

let socket: WebSocket | null = null;

const props = defineProps<{ uuid: string }>();
const messages = ref<Message[]>([]);

const messageListRef = ref<HTMLElement | null>(null);

async function load() {
  messages.value = await fetchMessages(props.uuid);
}

async function onSend(content: string) {
  await sendMessage(props.uuid, content);
  // messages.value.push(msg);
  await nextTick();
  scrollToBottom();
}

function scrollToBottom() {
  if (messageListRef.value) {
    messageListRef.value.scrollTop = messageListRef.value.scrollHeight;
  }
}

onMounted(async () => {
  await load();
  await nextTick();
  scrollToBottom();

  // Fetch WebSocket token from the backend
  const wsToken = await getWsToken(props.uuid);

  // Connect to the WebSocket with the token
  socket = new WebSocket(`${API_WS}/rooms/${props.uuid}?token=${wsToken}`);

  socket.onmessage = (event) => {
    const msg: Message = JSON.parse(event.data);

    const exists = messages.value.some(
      (m) =>
        m.sent_at === msg.sent_at &&
        m.sender === msg.sender &&
        m.content === msg.content
    );

    if (!exists) {
      messages.value.push(msg);
      nextTick().then(scrollToBottom);
    }
  };

  socket.onclose = () => {
    console.warn("WebSocket closed");
  };
});

onUnmounted(() => {
  socket?.close();
});
</script>

<style scoped>
.chat-page {
  display: flex;
  flex-direction: column;
  height: 95%;
  width: 90%;
  padding: 15px;
  border: 1px solid var(--border);
  border-radius: var(--radius);
  background: var(--panel);
  overflow: hidden;
}

.messages-container {
  flex: 1;
  padding: 1rem;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  word-wrap: break-word;
}

.input-container {
  padding: 0.5rem 1rem;
  border-top: 1px solid var(--border);
  background: var(--panel);
}
</style>


<style scoped>
.chat-page {
  display: flex;
  flex-direction: column;
  height: 95%;
  width: 90%;
  padding: 15px;
  border: 1px solid var(--border);
  border-radius: var(--radius);
  background: var(--panel);
  overflow: hidden;
}

.messages-container {
  flex: 1;
  padding: 1rem;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  word-wrap: break-word;
}

.input-container {
  padding: 0.5rem 1rem;
  border-top: 1px solid var(--border);
  background: var(--panel);
}
</style>
