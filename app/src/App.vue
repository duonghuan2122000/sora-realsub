<script setup lang="ts">
import { ref } from "vue";
import DeviceSelector, { type AudioDeviceInfo } from "./components/DeviceSelector.vue";

const micDeviceId = ref<string | null>(null);
const loopbackDeviceId = ref<string | null>(null);
const confirmError = ref("");
const deviceLostWarning = ref("");

function onMicSelected(id: string | null) {
  micDeviceId.value = id;
}

function onLoopbackSelected(id: string | null) {
  loopbackDeviceId.value = id;
}

function checkStillAvailable(
  selectedId: string | null,
  devices: AudioDeviceInfo[],
): boolean {
  if (selectedId === null) return true;
  return devices.some((d) => d.id === selectedId);
}

function onMicDevicesLoaded(devices: AudioDeviceInfo[]) {
  if (!checkStillAvailable(micDeviceId.value, devices)) {
    micDeviceId.value = null;
    deviceLostWarning.value = "Thiết bị mic đã chọn không còn khả dụng, vui lòng chọn lại.";
  }
}

function onLoopbackDevicesLoaded(devices: AudioDeviceInfo[]) {
  if (!checkStillAvailable(loopbackDeviceId.value, devices)) {
    loopbackDeviceId.value = null;
    deviceLostWarning.value = "Thiết bị loopback đã chọn không còn khả dụng, vui lòng chọn lại.";
  }
}

function confirmSelection() {
  deviceLostWarning.value = "";
  const missing: string[] = [];
  if (!micDeviceId.value) missing.push("microphone");
  if (!loopbackDeviceId.value) missing.push("nguồn loopback");

  if (missing.length > 0) {
    confirmError.value = `Vui lòng chọn ${missing.join(" và ")} trước khi xác nhận.`;
    return;
  }
  confirmError.value = "";
}
</script>

<template>
  <main class="container">
    <h1>Sora RealSub</h1>
    <p class="subtitle">Chọn thiết bị để bắt đầu phiên phụ đề</p>

    <div class="device-panels">
      <DeviceSelector
        title="Chọn microphone"
        command="list_input_devices_cmd"
        empty-message="Không tìm thấy thiết bị microphone nào."
        :selected="micDeviceId"
        @update:selected="onMicSelected"
        @devices-loaded="onMicDevicesLoaded"
      />
      <DeviceSelector
        title="Chọn nguồn loopback"
        command="list_output_devices_cmd"
        empty-message="Không tìm thấy nguồn loopback nào."
        :selected="loopbackDeviceId"
        @update:selected="onLoopbackSelected"
        @devices-loaded="onLoopbackDevicesLoaded"
      />
    </div>

    <p v-if="deviceLostWarning" class="warning">{{ deviceLostWarning }}</p>
    <p v-if="confirmError" class="error">{{ confirmError }}</p>

    <button type="button" @click="confirmSelection">Xác nhận</button>
  </main>
</template>

<style scoped>
.device-panels {
  display: flex;
  gap: 2em;
  justify-content: center;
  flex-wrap: wrap;
}

.subtitle {
  opacity: 0.8;
}

.warning {
  color: #b8860b;
}

.error {
  color: #d32f2f;
}
</style>
<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

h1 {
  text-align: center;
}

button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
  cursor: pointer;
  outline: none;
}

button:hover {
  border-color: #396cd8;
}
button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }
}
</style>
