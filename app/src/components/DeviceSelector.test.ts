import { describe, it, expect, vi, beforeEach } from "vitest";
import { mount, flushPromises } from "@vue/test-utils";
import DeviceSelector from "./DeviceSelector.vue";

const invokeMock = vi.fn();
vi.mock("@tauri-apps/api/core", () => ({
  invoke: (...args: unknown[]) => invokeMock(...args),
}));

beforeEach(() => {
  invokeMock.mockReset();
});

describe("DeviceSelector", () => {
  it("hiển thị danh sách thiết bị và đánh dấu mặc định", async () => {
    invokeMock.mockResolvedValue([
      { id: "a", name: "Mic A", is_default: false },
      { id: "b", name: "Mic B", is_default: true },
    ]);
    const wrapper = mount(DeviceSelector);
    await flushPromises();

    expect(wrapper.text()).toContain("Mic A");
    expect(wrapper.text()).toContain("Mic B");
    expect(wrapper.text()).toContain("mặc định");
  });

  it("hiển thị thông báo khi không có thiết bị nào", async () => {
    invokeMock.mockResolvedValue([]);
    const wrapper = mount(DeviceSelector);
    await flushPromises();

    expect(wrapper.text()).toContain("Không tìm thấy thiết bị microphone nào");
  });

  it("hiển thị thông báo lỗi khi invoke thất bại", async () => {
    invokeMock.mockRejectedValue("lỗi provider: không truy vấn được thiết bị");
    const wrapper = mount(DeviceSelector);
    await flushPromises();

    expect(wrapper.text()).toContain("lỗi provider");
  });

  it("bấm làm mới gọi lại invoke", async () => {
    invokeMock.mockResolvedValue([{ id: "a", name: "Mic A", is_default: false }]);
    const wrapper = mount(DeviceSelector);
    await flushPromises();
    expect(invokeMock).toHaveBeenCalledTimes(1);

    await wrapper.find("button").trigger("click");
    await flushPromises();
    expect(invokeMock).toHaveBeenCalledTimes(2);
  });

  it("click vào thiết bị phát emit update:selected với id tương ứng", async () => {
    invokeMock.mockResolvedValue([
      { id: "a", name: "Mic A", is_default: false },
      { id: "b", name: "Mic B", is_default: true },
    ]);
    const wrapper = mount(DeviceSelector);
    await flushPromises();

    await wrapper.findAll("li")[1].trigger("click");

    expect(wrapper.emitted("update:selected")).toEqual([["b"]]);
  });

  it("dùng prop title/command/emptyMessage tuỳ biến cho nguồn loopback", async () => {
    invokeMock.mockResolvedValue([]);
    const wrapper = mount(DeviceSelector, {
      props: {
        title: "Chọn nguồn loopback",
        command: "list_output_devices_cmd",
        emptyMessage: "Không tìm thấy nguồn loopback nào.",
      },
    });
    await flushPromises();

    expect(invokeMock).toHaveBeenCalledWith("list_output_devices_cmd");
    expect(wrapper.text()).toContain("Chọn nguồn loopback");
    expect(wrapper.text()).toContain("Không tìm thấy nguồn loopback nào.");
  });
});
