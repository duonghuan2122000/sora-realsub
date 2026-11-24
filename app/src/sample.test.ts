import { describe, it, expect } from "vitest";
import { mount } from "@vue/test-utils";
import { defineComponent } from "vue";

const Greeting = defineComponent({
  props: { name: { type: String, default: "" } },
  template: "<p>Hello {{ name }}</p>",
});

describe("Greeting", () => {
  it("renders the given name", () => {
    const wrapper = mount(Greeting, { props: { name: "Sora" } });
    expect(wrapper.text()).toBe("Hello Sora");
  });
});
