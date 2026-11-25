import { describe, it, expect, vi, beforeEach } from 'vitest'
import { mount } from '@vue/test-utils'

type EventHandler = (event: { payload: { translatedText: string } }) => void

let registeredHandler: EventHandler | undefined

vi.mock('@tauri-apps/api/event', () => ({
  listen: vi.fn((_eventName: string, handler: EventHandler) => {
    registeredHandler = handler
    return Promise.resolve(() => {
      registeredHandler = undefined
    })
  }),
}))

import SubtitleApp from './SubtitleApp.vue'

function emitSubtitleReady(translatedText: string) {
  registeredHandler?.({ payload: { startFrameIndex: 0, endFrameIndex: 0, sourceText: '', translatedText } } as never)
}

describe('SubtitleApp', () => {
  beforeEach(() => {
    registeredHandler = undefined
  })

  it('hiển thị text dịch khi nhận 1 event subtitle-ready', async () => {
    const wrapper = mount(SubtitleApp)
    await vi.waitFor(() => expect(registeredHandler).toBeDefined())

    emitSubtitleReady('xin chào')
    await wrapper.vm.$nextTick()

    expect(wrapper.text()).toContain('xin chào')
  })

  it('cập nhật đúng thứ tự khi nhận nhiều event liên tiếp', async () => {
    const wrapper = mount(SubtitleApp)
    await vi.waitFor(() => expect(registeredHandler).toBeDefined())

    for (let i = 0; i < 20; i++) {
      emitSubtitleReady(`câu ${i}`)
      await wrapper.vm.$nextTick()
      expect(wrapper.text()).toContain(`câu ${i}`)
    }

    expect(wrapper.text()).toContain('câu 19')
    expect(wrapper.text()).not.toContain('câu 18')
  })

  it('không hiển thị dòng trống khi translatedText rỗng', async () => {
    const wrapper = mount(SubtitleApp)
    await vi.waitFor(() => expect(registeredHandler).toBeDefined())

    emitSubtitleReady('')
    await wrapper.vm.$nextTick()

    expect(wrapper.find('.subtitle-text').exists()).toBe(false)
  })
})
