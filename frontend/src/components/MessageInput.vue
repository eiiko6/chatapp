<script setup lang="ts">
import { ref, nextTick } from 'vue'

const content = ref('')
const emit = defineEmits<{ (e: 'send', content: string): void }>()

function submit() {
  if (!content.value) return
  emit('send', content.value)
  content.value = ''
  resize()
}

function resize() {
  nextTick(() => {
    const textarea = document.querySelector('textarea')
    if (textarea) {
      textarea.style.height = 'auto'
      textarea.style.height = textarea.scrollHeight + 'px'
    }
  })
}

function handleKeydown(e: KeyboardEvent) {
  if ((e.shiftKey || e.altKey) && e.key === 'Enter') {
    // Insert a line break at cursor
    const textarea = e.target as HTMLTextAreaElement
    const start = textarea.selectionStart
    const end = textarea.selectionEnd
    content.value = content.value.substring(0, start) + '\n' + content.value.substring(end)
    nextTick(() => {
      textarea.selectionStart = textarea.selectionEnd = start + 1
    })
    resize()
    e.preventDefault()
  } else if (!e.shiftKey && !e.altKey && e.key === 'Enter') {
    submit()
    e.preventDefault()
  }
}
</script>

<template>
  <textarea v-model="content" @input="resize" @keydown="handleKeydown" placeholder="type a message" rows="1"></textarea>
</template>


<style scoped>
textarea {
  width: 100%;
  resize: none;
  overflow: hidden;
  padding: 0.5rem;
  font-size: 1rem;
  line-height: 1.4;
  box-sizing: border-box;
}
</style>
