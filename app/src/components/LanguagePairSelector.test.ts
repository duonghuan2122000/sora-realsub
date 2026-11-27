import { describe, it, expect, vi, beforeEach } from "vitest";
import { mount, flushPromises } from "@vue/test-utils";
import LanguagePairSelector from "./LanguagePairSelector.vue";

const invokeMock = vi.fn();
vi.mock("@tauri-apps/api/core", () => ({
  invoke: (...args: unknown[]) => invokeMock(...args),
}));

beforeEach(() => {
  invokeMock.mockReset();
});

describe("LanguagePairSelector", () => {
  it("hiển thị danh sách cặp ngôn ngữ lấy từ list_language_pairs_cmd", async () => {
    invokeMock.mockResolvedValue([
      { id: "en-vi", sourceLang: "en", targetLang: "vi", label: "Anh → Việt" },
    ]);
    const wrapper = mount(LanguagePairSelector);
    await flushPromises();

    expect(invokeMock).toHaveBeenCalledWith("list_language_pairs_cmd");
    expect(wrapper.text()).toContain("Anh → Việt");
  });

  it("hiển thị thông báo khi không có cặp ngôn ngữ nào", async () => {
    invokeMock.mockResolvedValue([]);
    const wrapper = mount(LanguagePairSelector);
    await flushPromises();

    expect(wrapper.text()).toContain("Không có cặp ngôn ngữ nào được hỗ trợ.");
  });

  it("hiển thị thông báo lỗi khi invoke thất bại", async () => {
    invokeMock.mockRejectedValue("lỗi provider: không lấy được danh sách ngôn ngữ");
    const wrapper = mount(LanguagePairSelector);
    await flushPromises();

    expect(wrapper.text()).toContain("lỗi provider");
  });

  it("chọn cặp ngôn ngữ khác phát emit update:selected với id tương ứng", async () => {
    invokeMock.mockResolvedValue([
      { id: "en-vi", sourceLang: "en", targetLang: "vi", label: "Anh → Việt" },
      { id: "vi-en", sourceLang: "vi", targetLang: "en", label: "Việt → Anh" },
    ]);
    const wrapper = mount(LanguagePairSelector);
    await flushPromises();

    await wrapper.find("select").setValue("vi-en");

    expect(wrapper.emitted("update:selected")).toEqual([["vi-en"]]);
  });

  it("khoá ô chọn khi prop disabled = true", async () => {
    invokeMock.mockResolvedValue([
      { id: "en-vi", sourceLang: "en", targetLang: "vi", label: "Anh → Việt" },
    ]);
    const wrapper = mount(LanguagePairSelector, { props: { disabled: true } });
    await flushPromises();

    expect(wrapper.find("select").attributes("disabled")).toBeDefined();
  });
});
