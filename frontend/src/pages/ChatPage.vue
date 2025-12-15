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
import { onMounted, ref, nextTick } from "vue";
import { fetchMessages, sendMessage } from "../api/messages";
import type { Message } from "../types/api";
import MessageList from "../components/MessageList.vue";
import MessageInput from "../components/MessageInput.vue";

const props = defineProps<{ uuid: string }>();
const messages = ref<Message[]>([]);

async function load() {
  messages.value = await fetchMessages(props.uuid);
}

async function onSend(content: string) {
  const msg = await sendMessage(props.uuid, content);
  messages.value.push(msg);
  await nextTick();
  scrollToBottom();
}

onMounted(async () => {
  await load();
  await nextTick();
  scrollToBottom();
});

const messageListRef = ref<HTMLElement | null>(null);

function scrollToBottom() {
  if (messageListRef.value) {
    messageListRef.value.scrollTop = messageListRef.value.scrollHeight;
  }
}
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
