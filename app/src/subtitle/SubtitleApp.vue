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
const FONT_SIZE_KEY = 'subtitle-font-size'
const FONT_SIZE_DEFAULT = 1.5
const FONT_SIZE_MIN = 1.0
const FONT_SIZE_MAX = 3.0
const FONT_SIZE_STEP = 0.25

function readStoredFontSize(): number {
  const raw = localStorage.getItem(FONT_SIZE_KEY)
  const parsed = raw === null ? NaN : parseFloat(raw)
  if (Number.isNaN(parsed)) return FONT_SIZE_DEFAULT
  return Math.min(FONT_SIZE_MAX, Math.max(FONT_SIZE_MIN, parsed))
}

const lines = ref<SubtitleLine[]>([])
const fontSize = ref(readStoredFontSize())
let nextId = 0
let unlisten: UnlistenFn | undefined

function setFontSize(next: number) {
  fontSize.value = Math.min(FONT_SIZE_MAX, Math.max(FONT_SIZE_MIN, next))
  localStorage.setItem(FONT_SIZE_KEY, String(fontSize.value))
}

function increaseFontSize() {
  setFontSize(fontSize.value + FONT_SIZE_STEP)
}

function decreaseFontSize() {
  setFontSize(fontSize.value - FONT_SIZE_STEP)
}

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
  <div
    class="subtitle-overlay"
    data-tauri-drag-region
    :style="{ '--subtitle-font-size': `${fontSize}rem` }"
  >
    <div class="font-size-controls" data-tauri-drag-region="false">
      <button type="button" class="font-size-btn font-size-btn--decrease" @click="decreaseFontSize">-</button>
      <button type="button" class="font-size-btn font-size-btn--increase" @click="increaseFontSize">+</button>
    </div>
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
  font-size: var(--subtitle-font-size, 1.5rem);
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

.font-size-controls {
  position: absolute;
  top: 0.5vh;
  right: 0.5vw;
  display: flex;
  gap: 0.3em;
}

.font-size-btn {
  width: 1.8em;
  height: 1.8em;
  border: none;
  border-radius: 4px;
  background: rgba(0, 0, 0, 0.6);
  color: #fff;
  font-size: 1rem;
  line-height: 1;
  cursor: pointer;
}

.font-size-btn:hover {
  background: rgba(0, 0, 0, 0.8);
}
</style>
