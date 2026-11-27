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
    await wrapper.find(".confirm-btn").trigger("click");

    expect(wrapper.text()).not.toContain("Vui lòng chọn");
  });

  it("báo lỗi rõ ràng khi xác nhận lúc thiếu cả mic và loopback", async () => {
    const wrapper = mount(App);
    await flushPromises();

    await wrapper.find(".confirm-btn").trigger("click");

    expect(wrapper.text()).toContain("Vui lòng chọn microphone và nguồn loopback");
  });

  it("báo lỗi rõ ràng khi chỉ thiếu loopback", async () => {
    const wrapper = mount(App);
    await flushPromises();

    const items = wrapper.findAll("li");
    await items[0].trigger("click"); // chỉ chọn mic

    await wrapper.find(".confirm-btn").trigger("click");

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

describe("App - nút Start/Stop phiên dịch phụ đề", () => {
  async function mountWithDevicesSelected() {
    const wrapper = mount(App);
    await flushPromises();
    const items = wrapper.findAll("li");
    await items[0].trigger("click"); // mic
    await items[1].trigger("click"); // loopback
    return wrapper;
  }

  it("nút Start bị vô hiệu hoá khi chưa chọn đủ thiết bị", async () => {
    const wrapper = mount(App);
    await flushPromises();

    const sessionBtn = wrapper.find(".session-btn");
    expect(sessionBtn.text()).toBe("Start");
    expect(sessionBtn.attributes("disabled")).toBeDefined();
  });

  it("bấm Start gọi start_capture, chuyển Đang chạy... rồi Stop khi resolve", async () => {
    let resolveStart: () => void = () => {};
    invokeMock.mockImplementation((cmd: string) => {
      if (cmd === "list_input_devices_cmd") return Promise.resolve(micDevices);
      if (cmd === "list_output_devices_cmd") return Promise.resolve(loopbackDevices);
      if (cmd === "start_capture") return new Promise<void>((resolve) => (resolveStart = resolve));
      return Promise.reject(`lệnh không xác định: ${cmd}`);
    });
    const wrapper = await mountWithDevicesSelected();

    const sessionBtn = wrapper.find(".session-btn");
    await sessionBtn.trigger("click");
    expect(wrapper.find(".session-btn").text()).toBe("Đang chạy...");
    expect(wrapper.find(".session-btn").attributes("disabled")).toBeDefined();

    resolveStart();
    await flushPromises();

    expect(wrapper.find(".session-btn").text()).toBe("Stop");
    expect(invokeMock).toHaveBeenCalledWith("start_capture");
  });

  it("bấm Start nhiều lần lúc đang chờ chỉ gọi start_capture 1 lần", async () => {
    let resolveStart: () => void = () => {};
    invokeMock.mockImplementation((cmd: string) => {
      if (cmd === "list_input_devices_cmd") return Promise.resolve(micDevices);
      if (cmd === "list_output_devices_cmd") return Promise.resolve(loopbackDevices);
      if (cmd === "start_capture") return new Promise<void>((resolve) => (resolveStart = resolve));
      return Promise.reject(`lệnh không xác định: ${cmd}`);
    });
    const wrapper = await mountWithDevicesSelected();

    const sessionBtn = wrapper.find(".session-btn");
    await sessionBtn.trigger("click");
    await sessionBtn.trigger("click");
    await sessionBtn.trigger("click");
    resolveStart();
    await flushPromises();

    expect(invokeMock.mock.calls.filter((c) => c[0] === "start_capture")).toHaveLength(1);
  });

  it("Start lỗi → quay lại Start và hiển thị thông báo lỗi", async () => {
    invokeMock.mockImplementation((cmd: string) => {
      if (cmd === "list_input_devices_cmd") return Promise.resolve(micDevices);
      if (cmd === "list_output_devices_cmd") return Promise.resolve(loopbackDevices);
      if (cmd === "start_capture") return Promise.reject("đã có phiên đang chạy");
      return Promise.reject(`lệnh không xác định: ${cmd}`);
    });
    const wrapper = await mountWithDevicesSelected();

    await wrapper.find(".session-btn").trigger("click");
    await flushPromises();

    expect(wrapper.find(".session-btn").text()).toBe("Start");
    expect(wrapper.text()).toContain("đã có phiên đang chạy");
  });

  it("bấm Stop khi đang chạy gọi stop_capture rồi quay về Start", async () => {
    invokeMock.mockImplementation((cmd: string) => {
      if (cmd === "list_input_devices_cmd") return Promise.resolve(micDevices);
      if (cmd === "list_output_devices_cmd") return Promise.resolve(loopbackDevices);
      if (cmd === "start_capture") return Promise.resolve();
      if (cmd === "stop_capture") return Promise.resolve();
      return Promise.reject(`lệnh không xác định: ${cmd}`);
    });
    const wrapper = await mountWithDevicesSelected();
    await wrapper.find(".session-btn").trigger("click");
    await flushPromises();
    expect(wrapper.find(".session-btn").text()).toBe("Stop");

    await wrapper.find(".session-btn").trigger("click");
    await flushPromises();

    expect(wrapper.find(".session-btn").text()).toBe("Start");
    expect(invokeMock).toHaveBeenCalledWith("stop_capture");
  });

  it("khoá chọn thiết bị (inert) khi phiên không ở trạng thái idle", async () => {
    invokeMock.mockImplementation((cmd: string) => {
      if (cmd === "list_input_devices_cmd") return Promise.resolve(micDevices);
      if (cmd === "list_output_devices_cmd") return Promise.resolve(loopbackDevices);
      if (cmd === "start_capture") return new Promise(() => {});
      return Promise.reject(`lệnh không xác định: ${cmd}`);
    });
    const wrapper = await mountWithDevicesSelected();

    await wrapper.find(".session-btn").trigger("click");

    expect(wrapper.find(".device-panels").attributes("inert")).toBeDefined();
  });
});
