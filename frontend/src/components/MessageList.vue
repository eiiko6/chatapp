<script setup lang="ts">
import type { Message } from '../types'
defineProps<{ messages: Message[] }>()
</script>

<template>
  <ul>
    <li v-for="(m, i) in messages" :key="i" class="message">
      <div class="sender">{{ m.sender }} <span class="timestamp">{{ m.sent_at }}</span></div>
      <div class="message-content">{{ m.content }}</div>
    </li>
  </ul>
</template>

<style scoped>
ul {
  padding: 0;
  margin: 0;
  list-style: none;
}

.message {
  margin-bottom: 0.5rem;
  background-color: rgba(0, 0, 0, 0.2);
  padding: 10px;
  border-radius: var(--radius);
}

.sender {
  font-weight: bold;
  font-size: 1.1rem;
  margin-bottom: 0.25rem;
}

.sender .timestamp {
  font-weight: normal;
  opacity: 0.7;
  font-size: 0.85rem;
  margin-left: 0.5rem;
}

.message-content {
  padding-left: 1rem;
  white-space: pre-wrap;
  word-wrap: break-word;
  max-width: 100%;
  display: block;
}
</style>
