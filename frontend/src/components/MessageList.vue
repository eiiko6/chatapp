<script setup lang="ts">
import type { Message } from '../types/api'
defineProps<{ messages: Message[] }>()
</script>

<template>
  <ul>
    <li v-for="(m, i) in messages" :key="i" class="message">
      <div class="sender">{{ m.sender }}</div>
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
  margin-bottom: 1rem;
}

.sender {
  font-weight: bold;
  margin-bottom: 0.25rem;
}

.message-content {
  padding-left: 1rem;
  white-space: pre-wrap;
  word-wrap: break-word;
  max-width: 100%;
  display: block;
}
</style>
