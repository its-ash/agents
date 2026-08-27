<script setup lang="ts">
import { computed } from 'vue'
import type { Agent } from '../api'

const props = defineProps<{
  agents: Agent[]
  selectedId: string | null
  mobileOpen: boolean
}>()

const emit = defineEmits<{
  select: [id: string]
  configure: [id: string]
  newAgent: []
  closeMobile: []
}>()

const list = computed(() => props.agents)

function initials(name: string) {
  return name.trim().split(/\s+/).slice(0, 2).map((w) => w[0]?.toUpperCase()).join('')
}
</script>

<template>
  <aside
    class="fixed lg:static inset-y-0 left-0 z-40 w-[260px] max-w-[85vw] bg-sidebar flex flex-col
           transform transition-transform duration-300 ease-out
           lg:translate-x-0 lg:rounded-none rounded-r-2xl lg:shadow-none shadow-2xl"
    :class="mobileOpen ? 'translate-x-0' : '-translate-x-full'"
  >
    <div class="shrink-0 p-4 pb-3">
      <div class="flex items-center justify-between mb-4">
        <div class="flex items-center gap-2">
          <div class="w-6 h-6 rounded-lg bg-black flex items-center justify-center">
            <svg xmlns="http://www.w3.org/2000/svg" class="w-3.5 h-3.5" viewBox="0 0 24 24" fill="none" stroke="white" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><path d="M12 2L2 7l10 5 10-5-10-5z"/><path d="M2 17l10 5 10-5"/><path d="M2 12l10 5 10-5"/></svg>
          </div>
          <span class="text-sm font-semibold tracking-tight">Agent Studio</span>
        </div>
        <button class="lg:hidden w-8 h-8 flex items-center justify-center rounded-full hover:bg-ink/5" @click="$emit('closeMobile')">
          <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round"><line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/></svg>
        </button>
      </div>

      <button
        class="w-full flex items-center justify-center gap-1.5 bg-ink text-canvas text-sm font-medium py-2 px-3 rounded-xl
               hover:bg-ink/85 active:scale-[0.98] transition shadow-sm"
        @click="$emit('newAgent')"
      >
        <svg xmlns="http://www.w3.org/2000/svg" class="w-3.5 h-3.5" viewBox="0 0 24 24" fill="none" stroke="white" stroke-width="2.5" stroke-linecap="round"><line x1="12" y1="5" x2="12" y2="19"/><line x1="5" y1="12" x2="19" y2="12"/></svg>
        <span>New Agent</span>
      </button>
    </div>

    <div class="flex-1 overflow-y-auto scroll-thin px-3 pb-4 space-y-1.5">
      <div
        v-for="agent in list"
        :key="agent.id"
        class="agent-card relative rounded-xl p-2.5 pr-9 cursor-pointer transition"
        :class="agent.id === selectedId ? 'bg-surface shadow-[0_8px_24px_-8px_rgba(0,0,0,0.12),0_2px_8px_-2px_rgba(0,0,0,0.06)]' : 'hover:bg-sidebar'"
        @click="$emit('select', agent.id)"
      >
        <div class="min-w-0">
          <p class="text-[13px] font-semibold truncate leading-tight">{{ agent.name }}</p>
        </div>
        <button
          class="absolute bottom-1.5 right-1.5 w-6 h-6 flex items-center justify-center rounded-full bg-surface/80 hover:bg-surface shadow-sm transition"
          aria-label="Configure agent"
          @click.stop="$emit('configure', agent.id)"
        >
          <svg xmlns="http://www.w3.org/2000/svg" class="w-3 h-3" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="3"/><path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 1 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-4 0v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 1 1-2.83-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1 0-4h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 1 1 2.83-2.83l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 1 1 2.83 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z"/></svg>
        </button>
      </div>
    </div>
  </aside>
</template>
