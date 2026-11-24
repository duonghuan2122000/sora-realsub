<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";

export interface AudioDeviceInfo {
  id: string;
  name: string;
  is_default: boolean;
}

const devices = ref<AudioDeviceInfo[]>([]);
const errorMessage = ref("");
const loading = ref(false);

async function loadDevices() {
  loading.value = true;
  errorMessage.value = "";
  try {
    devices.value = await invoke<AudioDeviceInfo[]>("list_input_devices_cmd");
  } catch (e) {
    devices.value = [];
    errorMessage.value = typeof e === "string" ? e : "Không thể lấy danh sách thiết bị.";
  } finally {
    loading.value = false;
  }
}

onMounted(loadDevices);

defineExpose({ loadDevices, devices, errorMessage });
</script>

<template>
  <div class="device-selector">
    <div class="device-selector__header">
      <h2>Chọn microphone</h2>
      <button type="button" @click="loadDevices" :disabled="loading">
        Làm mới
      </button>
    </div>

    <p v-if="loading">Đang tải danh sách thiết bị...</p>
    <p v-else-if="errorMessage" class="device-selector__error">{{ errorMessage }}</p>
    <p v-else-if="devices.length === 0">Không tìm thấy thiết bị microphone nào.</p>
    <ul v-else>
      <li v-for="device in devices" :key="device.id">
        {{ device.name }}
        <span v-if="device.is_default">(mặc định)</span>
      </li>
    </ul>
  </div>
</template>
