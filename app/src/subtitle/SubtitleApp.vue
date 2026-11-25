<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'

interface SubtitlePayload {
  startFrameIndex: number
  endFrameIndex: number
  sourceText: string
  translatedText: string
}

const currentSubtitle = ref('')
let unlisten: UnlistenFn | undefined

onMounted(async () => {
  unlisten = await listen<SubtitlePayload>('subtitle-ready', (event) => {
    currentSubtitle.value = event.payload.translatedText
  })
})

onUnmounted(() => {
  unlisten?.()
})
</script>

<template>
  <div class="subtitle-overlay">
    <p v-if="currentSubtitle" class="subtitle-text">{{ currentSubtitle }}</p>
  </div>
</template>

<style scoped>
.subtitle-overlay {
  width: 100vw;
  height: 100vh;
  background: transparent;
  display: flex;
  align-items: flex-end;
  justify-content: center;
}

.subtitle-text {
  max-width: 90vw;
  margin: 0 0 2vh 0;
  padding: 0.4em 0.8em;
  color: #fff;
  background: rgba(0, 0, 0, 0.6);
  border-radius: 6px;
  font-size: 1.5rem;
  text-align: center;
  word-wrap: break-word;
  overflow-wrap: break-word;
  overflow: hidden;
}
</style>
