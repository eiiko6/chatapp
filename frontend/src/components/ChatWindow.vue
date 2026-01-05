<template>
  <InvitePeopleModal v-if="showInviteModal" :room_uuid=props.uuid @close="showInviteModal = false" />

  <div v-if="uuid === 'none'" class="no-room">
    <div class="empty-state">
      <i class="fa-solid fa-comments"></i>
      <p>{{ $t('chat-no-room') }}</p>
    </div>
  </div>

  <div v-else class="chat-container">
    <!-- <h2 class="room-name">{{ currentRoom?.name }}</h2> -->

    <div class="messages-container" ref="messageListRef" @scroll="handleScroll">
      <MessageList :messages="messages" />
    </div>

    <div class="input-container">
      <button v-if="isOwner && !currentRoom?.global" class="invite-btn" @click="showInviteModal = true"
        :title="$t('chat-invite-title')">
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

// Pagination State
const isLoadingMore = ref(false);
const hasMore = ref(true); // Assume there are more until API returns empty
const showInviteModal = ref(false);
let socket: WebSocket | null = null;

const isOwner = computed(() => {
  if (!currentUser.value || !currentRoom.value) return false;
  return currentUser.value.uuid === currentRoom.value.owner_uuid;
});

async function initializeRoom() {
  if (socket) { await socket.disconnect(); socket = null; }

  messages.value = [];
  hasMore.value = true;
  currentRoom.value = null;

  if (props.uuid === 'none') return;

  try {
    const [msgs, roomInfo, auth] = await Promise.all([
      fetchMessages(props.uuid, undefined, 40), // Load first 40
      fetchRoomInfo(props.uuid),
      getAuthData()
    ]);

    messages.value = msgs;
    currentRoom.value = roomInfo;
    currentUser.value = auth.user;
    if (msgs.length < 40) hasMore.value = false;

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
          nextTick().then(scrollToBottomIfAtEnd);
        }
      }
    });
  } catch (err) {
    console.error("Room initialization failed:", err);
  }
}

async function handleScroll() {
  const el = messageListRef.value;
  if (!el) return;

  // If user scrolls to the top, is not already loading, and there's more data
  if (el.scrollTop < 50 && !isLoadingMore.value && hasMore.value) {
    await loadMore();
  }
}

async function loadMore() {
  if (messages.value.length === 0) return;

  isLoadingMore.value = true;
  const oldestMsgUuid = messages.value[0].uuid;

  try {
    const olderMsgs = await fetchMessages(props.uuid, oldestMsgUuid, 30);

    if (olderMsgs.length === 0) {
      hasMore.value = false;
      return;
    }

    // Capture height before adding messages to maintain scroll position
    const el = messageListRef.value;
    const previousScrollHeight = el?.scrollHeight || 0;

    messages.value = [...olderMsgs, ...messages.value];

    await nextTick();

    // Restore scroll position so the view doesn't jump
    if (el) {
      el.scrollTop = el.scrollHeight - previousScrollHeight;
    }

    if (olderMsgs.length < 30) hasMore.value = false;
  } catch (err) {
    console.error("Failed to load more messages:", err);
  } finally {
    isLoadingMore.value = false;
  }
}

function scrollToBottom() {
  if (messageListRef.value) {
    messageListRef.value.scrollTop = messageListRef.value.scrollHeight;
  }
}

// Only scroll to bottom for new messages if the user is already near the bottom
function scrollToBottomIfAtEnd() {
  const el = messageListRef.value;
  if (!el) return;
  const threshold = 150;
  const isAtBottom = el.scrollHeight - el.scrollTop - el.clientHeight < threshold;
  if (isAtBottom) scrollToBottom();
}



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

async function onSend(content: string) {
  if (props.uuid === 'none') return;
  await sendMessage(props.uuid, content);
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
  scroll-behavior: auto;
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

/* .room-name { */
/*   margin: 15px 0; */
/*   text-align: center; */
/* } */

.loading-more {
  text-align: center;
  padding: 10px;
  font-size: 0.8rem;
  color: var(--muted);
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
