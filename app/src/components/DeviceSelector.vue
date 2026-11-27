<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";

export interface AudioDeviceInfo {
  id: string;
  name: string;
  is_default: boolean;
}

const props = withDefaults(
  defineProps<{
    title?: string;
    command?: string;
    emptyMessage?: string;
    selected?: string | null;
  }>(),
  {
    title: "Chọn microphone",
    command: "list_input_devices_cmd",
    emptyMessage: "Không tìm thấy thiết bị microphone nào.",
    selected: null,
  },
);

const emit = defineEmits<{
  "update:selected": [id: string | null];
  "devices-loaded": [devices: AudioDeviceInfo[]];
}>();

const devices = ref<AudioDeviceInfo[]>([]);
const errorMessage = ref("");
const loading = ref(false);

async function loadDevices() {
  loading.value = true;
  errorMessage.value = "";
  try {
    devices.value = await invoke<AudioDeviceInfo[]>(props.command);
  } catch (e) {
    devices.value = [];
    errorMessage.value = typeof e === "string" ? e : "Không thể lấy danh sách thiết bị.";
  } finally {
    loading.value = false;
    emit("devices-loaded", devices.value);
  }
}

function selectDevice(id: string) {
  emit("update:selected", id);
}

onMounted(loadDevices);

defineExpose({ loadDevices, devices, errorMessage });
</script>

<template>
  <div class="device-selector">
    <div class="device-selector__header">
      <h2>{{ title }}</h2>
      <button type="button" @click="loadDevices" :disabled="loading">
        Làm mới
      </button>
    </div>

    <p v-if="loading">Đang tải danh sách thiết bị...</p>
    <p v-else-if="errorMessage" class="device-selector__error">{{ errorMessage }}</p>
    <p v-else-if="devices.length === 0">{{ emptyMessage }}</p>
    <ul v-else>
      <li
        v-for="device in devices"
        :key="device.id"
        :class="{ 'device-selector__item--selected': device.id === selected }"
        @click="selectDevice(device.id)"
      >
        {{ device.name }}
        <span v-if="device.is_default">(mặc định)</span>
      </li>
    </ul>
  </div>
</template>

<style scoped>
.device-selector ul {
  list-style: none;
  padding: 0;
}

.device-selector li {
  cursor: pointer;
  padding: 0.4em 0.6em;
  border-radius: 6px;
}

.device-selector li:hover {
  background-color: rgba(100, 108, 255, 0.1);
}

.device-selector__item--selected {
  background-color: rgba(100, 108, 255, 0.25);
  font-weight: 600;
}
</style>
