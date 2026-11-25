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

  it('cập nhật đúng thứ tự khi nhận nhiều event liên tiếp, giữ tối đa 3 dòng gần nhất', async () => {
    const wrapper = mount(SubtitleApp)
    await vi.waitFor(() => expect(registeredHandler).toBeDefined())

    for (let i = 0; i < 20; i++) {
      emitSubtitleReady(`câu ${i}`)
      await wrapper.vm.$nextTick()
    }

    const texts = wrapper.findAll('.subtitle-text').map((el) => el.text())
    expect(texts).toEqual(['câu 17', 'câu 18', 'câu 19'])
  })

  it('loại dòng cũ nhất khỏi DOM khi vượt giới hạn N=3', async () => {
    const wrapper = mount(SubtitleApp)
    await vi.waitFor(() => expect(registeredHandler).toBeDefined())

    emitSubtitleReady('dòng 1')
    emitSubtitleReady('dòng 2')
    emitSubtitleReady('dòng 3')
    await wrapper.vm.$nextTick()
    expect(wrapper.text()).toContain('dòng 1')

    emitSubtitleReady('dòng 4')
    await wrapper.vm.$nextTick()

    expect(wrapper.text()).not.toContain('dòng 1')
    expect(wrapper.text()).toContain('dòng 2')
    expect(wrapper.text()).toContain('dòng 3')
    expect(wrapper.text()).toContain('dòng 4')
  })

  it('dòng mới nhất mờ ít nhất, dòng cũ hơn mờ nhiều hơn', async () => {
    const wrapper = mount(SubtitleApp)
    await vi.waitFor(() => expect(registeredHandler).toBeDefined())

    emitSubtitleReady('dòng A')
    emitSubtitleReady('dòng B')
    emitSubtitleReady('dòng C')
    await wrapper.vm.$nextTick()

    const lineEls = wrapper.findAll('.subtitle-text')
    expect(lineEls).toHaveLength(3)
    expect(lineEls[0].classes()).toContain('subtitle-text--age-2')
    expect(lineEls[1].classes()).toContain('subtitle-text--age-1')
    expect(lineEls[2].classes()).toContain('subtitle-text--age-0')
  })

  it('không hiển thị dòng trống khi translatedText rỗng', async () => {
    const wrapper = mount(SubtitleApp)
    await vi.waitFor(() => expect(registeredHandler).toBeDefined())

    emitSubtitleReady('')
    await wrapper.vm.$nextTick()

    expect(wrapper.find('.subtitle-text').exists()).toBe(false)
  })

  it('vùng gốc cửa sổ có data-tauri-drag-region để kéo-thả được (PBI 23)', () => {
    const wrapper = mount(SubtitleApp)
    expect(wrapper.find('.subtitle-overlay').attributes('data-tauri-drag-region')).toBeDefined()
  })
})
