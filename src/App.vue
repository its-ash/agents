<script setup lang="ts">
import { computed, onMounted, ref, watch } from 'vue'
import Sidebar from './components/Sidebar.vue'
import Workspace from './components/Workspace.vue'
import AgentDialog from './components/AgentDialog.vue'
import HelpModal from './components/HelpModal.vue'
import SettingsModal from './components/SettingsModal.vue'
import {
  listAgents,
  detectTools,
  getSettings,
  type Agent,
  type AppSettings,
  type DetectedTools,
} from './api.ts'

const agents = ref<Agent[]>([])
const selectedId = ref<string | null>(null)
const settings = ref<AppSettings>({
  openrouter_key: null,
  openai_key: null,
  default_provider: 'openrouter',
  default_model: null,
})
const tools = ref<DetectedTools | null>(null)

const mobileSidebarOpen = ref(false)
const configTargetId = ref<string | null>(null)
const dialogMode = ref<'create' | 'edit'>('create')
const showDialog = ref(false)
const showSettings = ref(false)
const showHelp = ref(false)

const isDark = ref(false)
const fontScale = ref(1)

function applyTheme() {
  document.documentElement.classList.toggle('dark', isDark.value)
}

function applyFontScale() {
  document.documentElement.style.fontSize = `${16 * fontScale.value}px`
}

function toggleDark() {
  isDark.value = !isDark.value
  localStorage.setItem('agent-studio-theme', isDark.value ? 'dark' : 'light')
  applyTheme()
}

function fontIncrease() {
  fontScale.value = Math.min(fontScale.value + 0.1, 1.6)
  localStorage.setItem('agent-studio-font-scale', String(fontScale.value))
  applyFontScale()
}

function fontDecrease() {
  fontScale.value = Math.max(fontScale.value - 0.1, 0.8)
  localStorage.setItem('agent-studio-font-scale', String(fontScale.value))
  applyFontScale()
}

onMounted(async () => {
  isDark.value = localStorage.getItem('agent-studio-theme') === 'dark'
  applyTheme()
  const savedScale = localStorage.getItem('agent-studio-font-scale')
  if (savedScale) {
    fontScale.value = parseFloat(savedScale)
    applyFontScale()
  }
  try {
    agents.value = await listAgents()
    settings.value = await getSettings()
    tools.value = await detectTools()
    if (agents.value.length && !selectedId.value) {
      selectedId.value = agents.value[2]?.id ?? agents.value[0].id
    }
  } catch (e) {
    console.error('init failed', e)
  }
})

const selectedAgent = computed(() =>
  agents.value.find((a) => a.id === selectedId.value) ?? null,
)

const configAgent = computed(() =>
  agents.value.find((a) => a.id === configTargetId.value) ?? null,
)

onMounted(async () => {
  try {
    agents.value = await listAgents()
    settings.value = await getSettings()
    tools.value = await detectTools()
    if (agents.value.length && !selectedId.value) {
      selectedId.value = agents.value[2]?.id ?? agents.value[0].id
    }
  } catch (e) {
    console.error('init failed', e)
  }
})

function selectAgent(id: string) {
  selectedId.value = id
  mobileSidebarOpen.value = false
}

function openNewAgent() {
  dialogMode.value = 'create'
  configTargetId.value = null
  showDialog.value = true
}

function openConfig(id: string) {
  dialogMode.value = 'edit'
  configTargetId.value = id
  showDialog.value = true
}

function closeDialog() {
  showDialog.value = false
  configTargetId.value = null
}

function onCreated(agent: Agent) {
  agents.value.unshift(agent)
  selectedId.value = agent.id
  closeDialog()
  mobileSidebarOpen.value = false
}

function onSaved(agent: Agent) {
  closeDialog()
}

function onDeleted(id: string) {
  agents.value = agents.value.filter((a) => a.id !== id)
  if (selectedId.value === id) {
    selectedId.value = agents.value[0]?.id ?? null
  }
  closeDialog()
}

async function reloadAgents() {
  agents.value = await listAgents()
  if (agents.value.length && !selectedId.value) {
    selectedId.value = agents.value[0].id
  }
}
</script>

<template>
  <div class="h-screen w-full flex overflow-hidden text-ink">
    <Sidebar
      :agents="agents"
      :selected-id="selectedId"
      :mobile-open="mobileSidebarOpen"
      @select="selectAgent"
      @configure="openConfig"
      @new-agent="openNewAgent"
      @close-mobile="mobileSidebarOpen = false"
    />

    <div class="flex-1 flex flex-col min-w-0 h-screen overflow-hidden">
      <div
        v-if="mobileSidebarOpen"
        class="fixed inset-0 bg-ink/40 z-30 lg:hidden backdrop-blur-[2px]"
        @click="mobileSidebarOpen = false"
      ></div>

      <div class="shrink-0 flex items-center justify-between px-4 py-2 bg-sidebar border-b border-ink/5">
        <button class="lg:hidden w-8 h-8 flex items-center justify-center rounded-xl bg-surface shadow-sm" @click="mobileSidebarOpen = true">
          <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round"><line x1="3" y1="6" x2="21" y2="6"/><line x1="3" y1="12" x2="21" y2="12"/><line x1="3" y1="18" x2="21" y2="18"/></svg>
        </button>
        <span class="lg:hidden text-sm font-semibold flex-1 ml-2">{{ selectedAgent?.name ?? 'Agent Studio' }}</span>

        <div class="flex items-center gap-1 ml-auto">
          <div class="flex items-center gap-0.5 mr-1 rounded-lg bg-surface/60 p-0.5">
            <button class="w-7 h-7 flex items-center justify-center rounded-md hover:bg-ink/10 transition text-[13px] font-semibold" @click="fontDecrease" aria-label="Decrease font size" :disabled="fontScale <= 0.8" :class="{ 'opacity-40 cursor-not-allowed': fontScale <= 0.8 }">A-</button>
            <button class="w-7 h-7 flex items-center justify-center rounded-md hover:bg-ink/10 transition text-[15px] font-semibold" @click="fontIncrease" aria-label="Increase font size" :disabled="fontScale >= 1.6" :class="{ 'opacity-40 cursor-not-allowed': fontScale >= 1.6 }">A+</button>
          </div>
          <button class="w-8 h-8 flex items-center justify-center rounded-lg hover:bg-ink/5 transition" @click="toggleDark" aria-label="Toggle dark mode">
            <svg v-if="isDark" xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="5"/><line x1="12" y1="1" x2="12" y2="3"/><line x1="12" y1="21" x2="12" y2="23"/><line x1="4.22" y1="4.22" x2="5.64" y2="5.64"/><line x1="18.36" y1="18.36" x2="19.78" y2="19.78"/><line x1="1" y1="12" x2="3" y2="12"/><line x1="21" y1="12" x2="23" y2="12"/><line x1="4.22" y1="19.78" x2="5.64" y2="18.36"/><line x1="18.36" y1="5.64" x2="19.78" y2="4.22"/></svg>
            <svg v-else xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"/></svg>
          </button>
          <button class="w-8 h-8 flex items-center justify-center rounded-lg hover:bg-ink/5 transition" @click="showHelp = true" aria-label="Help">
            <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><path d="M9.09 9a3 3 0 0 1 5.83 1c0 2-3 3-3 3"/><line x1="12" y1="17" x2="12.01" y2="17"/></svg>
          </button>
          <button class="w-8 h-8 flex items-center justify-center rounded-lg hover:bg-ink/5 transition" @click="showSettings = true" aria-label="Settings">
            <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="3"/><path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 1 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-4 0v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 1 1-2.83-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1 0-4h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 1 1 2.83-2.83l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 1 1 2.83 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z"/></svg>
          </button>
        </div>
      </div>

      <main class="flex-1 overflow-y-auto scroll-thin min-w-0">
        <Workspace v-if="selectedAgent" :key="selectedAgent.id" :agent="selectedAgent" />
        <div v-else class="h-full flex items-center justify-center p-6">
          <div class="text-center max-w-sm fade-in">
            <div class="w-12 h-12 mx-auto rounded-2xl bg-surface shadow-sm flex items-center justify-center mb-4">
              <svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round"><path d="M12 2L2 7l10 5 10-5-10-5z"/><path d="M2 17l10 5 10-5"/><path d="M2 12l10 5 10-5"/></svg>
            </div>
            <h2 class="text-lg font-semibold tracking-tight mb-1.5">Select an agent to get started</h2>
            <p class="text-[13px] text-ink/45 leading-relaxed">Choose an agent from the sidebar, or create a new one to define a prompt template and run it.</p>
          </div>
        </div>
      </main>
    </div>

    <AgentDialog
      :show="showDialog"
      :agent="configAgent"
      :mode="dialogMode"
      :tools="tools"
      @close="closeDialog"
      @created="onCreated"
      @saved="onSaved"
      @deleted="onDeleted"
    />

    <HelpModal
      :show="showHelp"
      @close="showHelp = false"
    />

    <SettingsModal
      :show="showSettings"
      :settings="settings"
      @close="showSettings = false"
      @saved="(s) => (settings = s)"
      @agents-imported="reloadAgents"
    />
  </div>
</template>
