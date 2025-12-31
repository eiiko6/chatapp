<template>
  <InvitePeopleModal v-if="showInviteModal" :room_uuid=props.uuid @close="showInviteModal = false" />

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
      <button v-if="isOwner && !currentRoom?.global" class="invite-btn" @click="showInviteModal = true"
        title="Invite people">
        <i class="fa-solid fa-users"></i>
      </button>

      <MessageInput ref="messageInputRef" @send="onSend" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch, nextTick, computed } from "vue";
import { fetchMessages, sendMessage, getWsToken } from "../api/messages";
import type { Message, Room, User } from "../types";
import { API_WS } from '../main.ts';
import MessageList from "./MessageList.vue";
import MessageInput from "./MessageInput.vue";
import InvitePeopleModal from './InvitePeopleModal.vue';

import WebSocket from '@tauri-apps/plugin-websocket';
import { getAuthData } from "../authStore.ts";
import { fetchRoomInfo } from "../api/rooms.ts";

const props = defineProps<{ uuid: string }>();
const messages = ref<Message[]>([]);
const messageListRef = ref<HTMLElement | null>(null);
const messageInputRef = ref<InstanceType<typeof MessageInput> | null>(null);
const currentUser = ref<User | null>(null);
const currentRoom = ref<Room | null>(null);

const showInviteModal = ref(false);

let socket: WebSocket | null = null;

const isOwner = computed(() => {
  if (!currentUser.value || !currentRoom.value) return false;
  return currentUser.value.uuid === currentRoom.value.owner_uuid;
});

const handleGlobalKeyDown = (event: KeyboardEvent) => {
  if (event.key === 'Enter') {
    const active = document.activeElement?.tagName.toLowerCase();
    const isTyping = active === 'input' || active === 'textarea';
    if (!isTyping && messageInputRef.value) {
      event.preventDefault();
      messageInputRef.value.focus();
    }
  }
};

async function initializeRoom() {
  if (socket) {
    await socket.disconnect();
    socket = null;
  }

  // messages.value = [];
  currentRoom.value = null;

  if (props.uuid === 'none') return;

  try {
    // 5. Fetch Room Details and Messages in parallel
    const [msgs, roomInfo, auth] = await Promise.all([
      fetchMessages(props.uuid),
      fetchRoomInfo(props.uuid),
      getAuthData()
    ]);

    messages.value = msgs;
    currentRoom.value = roomInfo;
    currentUser.value = auth.user;

    await nextTick();
    scrollToBottom();

    const wsToken = await getWsToken(props.uuid);
    const url = `${API_WS}/rooms/${props.uuid}?token=${wsToken}`;
    socket = await WebSocket.connect(url);

    socket.addListener((msg) => {
      if (msg.type === 'Text') {
        const data: Message = JSON.parse(msg.data);
        if (!messages.value.some(m => m.uuid === data.uuid)) {
          messages.value.push(data);
          nextTick().then(scrollToBottom);
        }
      }
    });

  } catch (err) {
    console.error("Room initialization failed:", err);
  }
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

onUnmounted(async () => {
  if (socket) {
    await socket.disconnect();
  }
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
  display: flex;
  gap: 10px;
  align-items: flex-end;
  padding: 10px;
  border-top: 1px solid var(--border);
}

/* Ensure the MessageInput component expands to fill the width */
:deep(.input-container > *:last-child) {
  flex: 1;
}

.invite-btn {
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
  user-select: none;
  -webkit-user-select: none;
}

.empty-state i {
  font-size: 3rem;
  margin-bottom: 1rem;
  opacity: 0.3;
}
</style>
