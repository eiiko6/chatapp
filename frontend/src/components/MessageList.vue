<script setup lang="ts">
import type { Message } from '../types/api'
defineProps<{ messages: Message[] }>()
</script>

<template>
  <ul>
    <li v-for="(m, i) in messages" :key="i">
      <strong>{{ m.sender }}:</strong> {{ m.content }}
    </li>
  </ul>
</template>

<style scoped>
ul {
  padding: 0;
  margin: 0;
  list-style: none;
  word-wrap: break-word;
}

li {
  white-space: pre-wrap;
}

.message-content {
  display: inline-block;
  max-width: 100%;
  word-break: break-word;
}
</style>
