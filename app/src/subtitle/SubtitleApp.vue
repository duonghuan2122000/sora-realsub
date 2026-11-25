<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'

interface SubtitlePayload {
  startFrameIndex: number
  endFrameIndex: number
  sourceText: string
  translatedText: string
}

interface SubtitleLine {
  id: number
  text: string
}

const MAX_LINES = 3

const lines = ref<SubtitleLine[]>([])
let nextId = 0
let unlisten: UnlistenFn | undefined

onMounted(async () => {
  unlisten = await listen<SubtitlePayload>('subtitle-ready', (event) => {
    const text = event.payload.translatedText
    if (!text) return
    lines.value = [...lines.value, { id: nextId++, text }].slice(-MAX_LINES)
  })
})

onUnmounted(() => {
  unlisten?.()
})
</script>

<template>
  <div class="subtitle-overlay" data-tauri-drag-region>
    <p
      v-for="(line, idx) in lines"
      :key="line.id"
      class="subtitle-text"
      :class="`subtitle-text--age-${lines.length - 1 - idx}`"
    >
      {{ line.text }}
    </p>
  </div>
</template>

<style scoped>
.subtitle-overlay {
  width: 100vw;
  height: 100vh;
  background: transparent;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: flex-end;
}

.subtitle-text {
  max-width: 90vw;
  margin: 0.2vh 0;
  padding: 0.4em 0.8em;
  color: #fff;
  background: rgba(0, 0, 0, 0.6);
  border-radius: 6px;
  font-size: 1.5rem;
  text-align: center;
  word-wrap: break-word;
  overflow-wrap: break-word;
  overflow: hidden;
  transition: opacity 0.3s ease, transform 0.3s ease;
}

.subtitle-text:last-child {
  margin-bottom: 2vh;
}

.subtitle-text--age-0 {
  opacity: 1;
}

.subtitle-text--age-1 {
  opacity: 0.6;
}

.subtitle-text--age-2 {
  opacity: 0.35;
}
</style>
