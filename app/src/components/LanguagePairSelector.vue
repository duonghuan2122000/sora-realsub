<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";

export interface LanguagePairInfo {
  id: string;
  sourceLang: string;
  targetLang: string;
  label: string;
}

const props = withDefaults(
  defineProps<{
    selected?: string | null;
    disabled?: boolean;
  }>(),
  {
    selected: null,
    disabled: false,
  },
);

const emit = defineEmits<{
  "update:selected": [id: string | null];
  "pairs-loaded": [pairs: LanguagePairInfo[]];
}>();

const pairs = ref<LanguagePairInfo[]>([]);
const errorMessage = ref("");
const loading = ref(false);

async function loadPairs() {
  loading.value = true;
  errorMessage.value = "";
  try {
    pairs.value = await invoke<LanguagePairInfo[]>("list_language_pairs_cmd");
  } catch (e) {
    pairs.value = [];
    errorMessage.value =
      typeof e === "string" ? e : "Không thể lấy danh sách cặp ngôn ngữ.";
  } finally {
    loading.value = false;
    emit("pairs-loaded", pairs.value);
  }
}

function onChange(event: Event) {
  const id = (event.target as HTMLSelectElement).value;
  emit("update:selected", id || null);
}

onMounted(loadPairs);

defineExpose({ loadPairs, pairs, errorMessage });
</script>

<template>
  <div class="language-pair-selector">
    <label for="language-pair-select">Ngôn ngữ nguồn-đích</label>
    <p v-if="loading">Đang tải danh sách ngôn ngữ...</p>
    <p v-else-if="errorMessage" class="language-pair-selector__error">{{ errorMessage }}</p>
    <p v-else-if="pairs.length === 0">Không có cặp ngôn ngữ nào được hỗ trợ.</p>
    <select
      v-else
      id="language-pair-select"
      :value="selected"
      :disabled="disabled"
      @change="onChange"
    >
      <option v-for="pair in pairs" :key="pair.id" :value="pair.id">
        {{ pair.label }}
      </option>
    </select>
  </div>
</template>

<style scoped>
.language-pair-selector {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 0.4em;
}

.language-pair-selector__error {
  color: #d32f2f;
}
</style>
