<template>
  <textarea ref="textareaRef" v-model="content" @input="resize" @keydown="handleKeydown" rows="1"
    :placeholder="$t('chat-input-placeholder')"></textarea>
</template>

<script setup lang="ts">
import { ref, nextTick } from 'vue'

const content = ref('')
const textareaRef = ref<HTMLTextAreaElement | null>(null)

const emit = defineEmits<{ (e: 'send', content: string): void }>()

defineExpose({
  focus: () => {
    textareaRef.value?.focus()
  }
})

function submit() {
  if (!content.value.trim()) return
  emit('send', content.value)
  content.value = ''
  resize()
}

function resize() {
  nextTick(() => {
    if (textareaRef.value) {
      textareaRef.value.style.height = 'auto'
      textareaRef.value.style.height = textareaRef.value.scrollHeight + 'px'
    }
  })
}

function handleKeydown(e: KeyboardEvent) {
  if ((e.shiftKey || e.altKey) && e.key === 'Enter') {
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

<style scoped>
textarea {
  width: 100%;
  resize: none;
  overflow: hidden;
  padding: 0.5rem;
  font-size: 1rem;
  line-height: 1.4;
  box-sizing: border-box;
  background: transparent;
  border: none;
  color: inherit;
  outline: none;
}
</style>
