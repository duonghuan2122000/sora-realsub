import { describe, it, expect, vi, beforeEach } from "vitest";
import { mount, flushPromises } from "@vue/test-utils";
import App from "./App.vue";

const invokeMock = vi.fn();
vi.mock("@tauri-apps/api/core", () => ({
  invoke: (...args: unknown[]) => invokeMock(...args),
}));

const micDevices = [{ id: "mic-a", name: "Mic A", is_default: true }];
const loopbackDevices = [{ id: "spk-a", name: "Speaker A", is_default: true }];

beforeEach(() => {
  invokeMock.mockReset();
  invokeMock.mockImplementation((cmd: string) => {
    if (cmd === "list_input_devices_cmd") return Promise.resolve(micDevices);
    if (cmd === "list_output_devices_cmd") return Promise.resolve(loopbackDevices);
    return Promise.reject(`lệnh không xác định: ${cmd}`);
  });
});

describe("App - màn hình chính chọn thiết bị", () => {
  it("xác nhận thành công khi đã chọn đủ mic + loopback", async () => {
    const wrapper = mount(App);
    await flushPromises();

    const items = wrapper.findAll("li");
    await items[0].trigger("click"); // mic
    await items[1].trigger("click"); // loopback
    await wrapper.find("button:not(.device-selector button)").trigger("click");

    expect(wrapper.text()).not.toContain("Vui lòng chọn");
  });

  it("báo lỗi rõ ràng khi xác nhận lúc thiếu cả mic và loopback", async () => {
    const wrapper = mount(App);
    await flushPromises();

    await wrapper.find("button:not(.device-selector button)").trigger("click");

    expect(wrapper.text()).toContain("Vui lòng chọn microphone và nguồn loopback");
  });

  it("báo lỗi rõ ràng khi chỉ thiếu loopback", async () => {
    const wrapper = mount(App);
    await flushPromises();

    const items = wrapper.findAll("li");
    await items[0].trigger("click"); // chỉ chọn mic

    await wrapper.find("button:not(.device-selector button)").trigger("click");

    expect(wrapper.text()).toContain("Vui lòng chọn nguồn loopback");
  });

  it("xoá lựa chọn và báo cảnh báo khi thiết bị đã chọn không còn trong danh sách tải lại", async () => {
    const wrapper = mount(App);
    await flushPromises();

    await wrapper.findAll("li")[0].trigger("click"); // chọn mic-a

    // Lần tải lại tiếp theo không còn mic-a
    invokeMock.mockImplementation((cmd: string) => {
      if (cmd === "list_input_devices_cmd") return Promise.resolve([]);
      if (cmd === "list_output_devices_cmd") return Promise.resolve(loopbackDevices);
      return Promise.reject(`lệnh không xác định: ${cmd}`);
    });
    const refreshButtons = wrapper.findAll(".device-selector button");
    await refreshButtons[0].trigger("click"); // làm mới panel mic
    await flushPromises();

    expect(wrapper.text()).toContain("không còn khả dụng");
  });
});
