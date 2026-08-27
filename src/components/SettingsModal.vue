<script setup lang="ts">
import { ref, watch } from 'vue'
import type { AppSettings, Provider } from '../api'
import { saveSettings, exportAgents, importAgentsFromFile, importAgentsFromRepo } from '../api'

const props = defineProps<{
  show: boolean
  settings: AppSettings
}>()

const emit = defineEmits<{
  close: []
  saved: [s: AppSettings]
  agentsImported: [count: number]
}>()

const local = ref<AppSettings>({ ...props.settings })
const saving = ref(false)
const savedFlash = ref(false)
const exporting = ref(false)
const importingFile = ref(false)
const importingRepo = ref(false)
const repoUrl = ref('')
const importMsg = ref<string | null>(null)
const importError = ref<string | null>(null)

watch(
  () => props.show,
  (v) => {
    if (v) {
      local.value = { ...props.settings }
      savedFlash.value = false
    }
  },
)

async function save() {
  saving.value = true
  try {
    await saveSettings(local.value)
    emit('saved', local.value)
    savedFlash.value = true
    setTimeout(() => {
      savedFlash.value = false
      emit('close')
    }, 600)
  } finally {
    saving.value = false
  }
}

function setProvider(p: Provider) {
  local.value.default_provider = p
}

async function handleExport() {
  exporting.value = true
  importMsg.value = null
  importError.value = null
  try {
    const path = await exportAgents()
    if (path) importMsg.value = `Exported to ${path}`
  } catch (e: any) {
    importError.value = String(e)
  } finally {
    exporting.value = false
  }
}

async function handleImportFile() {
  importingFile.value = true
  importMsg.value = null
  importError.value = null
  try {
    const count = await importAgentsFromFile()
    if (count > 0) {
      importMsg.value = `Imported ${count} agent${count !== 1 ? 's' : ''}`
      emit('agentsImported', count)
    }
  } catch (e: any) {
    importError.value = String(e)
  } finally {
    importingFile.value = false
  }
}

async function handleImportRepo() {
  if (!repoUrl.value.trim()) return
  importingRepo.value = true
  importMsg.value = null
  importError.value = null
  try {
    const count = await importAgentsFromRepo(repoUrl.value.trim())
    importMsg.value = `Imported ${count} agent${count !== 1 ? 's' : ''} from repo`
    emit('agentsImported', count)
  } catch (e: any) {
    importError.value = String(e)
  } finally {
    importingRepo.value = false
  }
}
</script>

<template>
  <div
    v-if="show"
    class="fixed inset-0 bg-ink/60 backdrop-blur-sm z-50 flex items-center justify-center p-4"
    @click.self="$emit('close')"
  >
    <div class="bg-canvas w-full max-w-md rounded-2xl shadow-2xl fade-in flex flex-col max-h-[88vh]">
      <div class="shrink-0 flex items-start justify-between p-5 pb-4 border-b border-ink/5">
        <div>
          <h3 class="text-base font-semibold tracking-tight">Settings</h3>
          <p class="text-xs text-ink/45 mt-0.5">API keys are stored locally on this device.</p>
        </div>
        <button class="w-8 h-8 shrink-0 flex items-center justify-center rounded-full bg-surface hover:bg-ink/5 transition shadow-sm" @click="$emit('close')">
          <svg xmlns="http://www.w3.org/2000/svg" class="w-3.5 h-3.5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round"><line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/></svg>
        </button>
      </div>

      <div class="flex-1 overflow-y-auto scroll-thin p-5 space-y-4">
        <div>
          <label class="block text-[11px] font-semibold text-ink/50 mb-1.5 uppercase tracking-wide">Default provider</label>
          <div class="flex flex-wrap gap-2">
            <button class="py-2 px-3 rounded-xl text-sm font-medium transition border"
              :class="local.default_provider === 'openrouter' ? 'bg-ink text-canvas border-ink' : 'bg-surface text-ink border-ink/10 hover:bg-ink/5'"
              @click="setProvider('openrouter')">OpenRouter</button>
            <button class="py-2 px-3 rounded-xl text-sm font-medium transition border"
              :class="local.default_provider === 'openai' ? 'bg-ink text-canvas border-ink' : 'bg-surface text-ink border-ink/10 hover:bg-ink/5'"
              @click="setProvider('openai')">OpenAI</button>
            <button class="py-2 px-3 rounded-xl text-sm font-medium transition border"
              :class="local.default_provider === 'copilot' ? 'bg-ink text-canvas border-ink' : 'bg-surface text-ink border-ink/10 hover:bg-ink/5'"
              @click="setProvider('copilot')">Copilot</button>
            <button class="py-2 px-3 rounded-xl text-sm font-medium transition border"
              :class="local.default_provider === 'claude' ? 'bg-ink text-canvas border-ink' : 'bg-surface text-ink border-ink/10 hover:bg-ink/5'"
              @click="setProvider('claude')">Claude</button>
            <button class="py-2 px-3 rounded-xl text-sm font-medium transition border"
              :class="local.default_provider === 'ollama' ? 'bg-ink text-canvas border-ink' : 'bg-surface text-ink border-ink/10 hover:bg-ink/5'"
              @click="setProvider('ollama')">Ollama</button>
          </div>
        </div>

        <div v-if="local.default_provider === 'openrouter'">
          <label class="block text-[11px] font-semibold text-ink/50 mb-1.5 uppercase tracking-wide">OpenRouter API Key</label>
          <input v-model="local.openrouter_key" type="password" autocomplete="off" placeholder="sk-or-..."
            class="w-full bg-surface rounded-xl px-3 py-2 text-sm outline-none focus:ring-2 focus:ring-ink/10 shadow-sm" />
          <p class="text-[11px] text-ink/40 mt-1.5">Get one at <span class="font-semibold">openrouter.ai/keys</span>.</p>
        </div>

        <div v-if="local.default_provider === 'openai'">
          <label class="block text-[11px] font-semibold text-ink/50 mb-1.5 uppercase tracking-wide">OpenAI API Key</label>
          <input v-model="local.openai_key" type="password" autocomplete="off" placeholder="sk-..."
            class="w-full bg-surface rounded-xl px-3 py-2 text-sm outline-none focus:ring-2 focus:ring-ink/10 shadow-sm" />
          <p class="text-[11px] text-ink/40 mt-1.5">Get one at <span class="font-semibold">platform.openai.com/api-keys</span>.</p>
        </div>

        <div v-if="local.default_provider === 'copilot' || local.default_provider === 'claude' || local.default_provider === 'ollama'" class="text-[11px] text-ink/45 bg-sidebar rounded-lg px-3 py-2.5">
          No API key needed — agents run the CLI tool directly. Make sure it's installed and on your <span class="font-semibold">$PATH</span>.
        </div>

        <div>
          <label class="block text-[11px] font-semibold text-ink/50 mb-1.5 uppercase tracking-wide">Default model for imports (optional)</label>
          <input v-model="local.default_model" type="text" placeholder="e.g. anthropic/claude-3.5-sonnet"
            class="w-full bg-surface rounded-xl px-3 py-2 text-sm outline-none focus:ring-2 focus:ring-ink/10 shadow-sm" />
          <p class="text-[11px] text-ink/40 mt-1.5">When importing agents, this model is applied if the file doesn't specify one.</p>
        </div>

        <div class="pt-2 border-t border-ink/5">
          <label class="block text-[11px] font-semibold text-ink/50 mb-1.5 uppercase tracking-wide">Export / Import agents</label>
          <div class="flex gap-2 mb-2">
            <button :disabled="exporting" type="button" class="flex-1 flex items-center justify-center gap-1.5 bg-surface text-ink text-sm font-medium py-2 rounded-xl border border-ink/10 hover:bg-ink/5 transition disabled:opacity-60" @click="handleExport">
              <svg xmlns="http://www.w3.org/2000/svg" class="w-3.5 h-3.5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><polyline points="7 10 12 15 17 10"/><line x1="12" y1="15" x2="12" y2="3"/></svg>
              <span>{{ exporting ? 'Exporting…' : 'Export' }}</span>
            </button>
            <button :disabled="importingFile" type="button" class="flex-1 flex items-center justify-center gap-1.5 bg-surface text-ink text-sm font-medium py-2 rounded-xl border border-ink/10 hover:bg-ink/5 transition disabled:opacity-60" @click="handleImportFile">
              <svg xmlns="http://www.w3.org/2000/svg" class="w-3.5 h-3.5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><polyline points="17 8 12 3 7 8"/><line x1="12" y1="3" x2="12" y2="15"/></svg>
              <span>{{ importingFile ? 'Importing…' : 'Import folder' }}</span>
            </button>
          </div>
          <div class="flex gap-2">
            <input v-model="repoUrl" type="text" placeholder="https://github.com/user/repo"
              class="flex-1 bg-surface rounded-xl px-3 py-2 text-sm outline-none focus:ring-2 focus:ring-ink/10 shadow-sm placeholder:text-ink/40" />
            <button :disabled="importingRepo || !repoUrl.trim()" type="button" class="px-4 py-2 rounded-xl bg-surface text-ink text-sm font-medium border border-ink/10 hover:bg-ink/5 transition disabled:opacity-60" @click="handleImportRepo">
              {{ importingRepo ? 'Cloning…' : 'Import repo' }}
            </button>
          </div>
          <p class="text-[11px] text-ink/40 mt-1.5">Each agent is a <span class="font-mono font-semibold">.md</span> file in the repo root. Filename becomes the agent name.</p>
          <p v-if="importMsg" class="mt-2 text-[12px] text-emerald-600 font-medium">{{ importMsg }}</p>
          <p v-if="importError" class="mt-2 text-[12px] text-red-600 font-medium">{{ importError }}</p>
        </div>
      </div>

      <div class="shrink-0 flex gap-2 p-5 pt-4 border-t border-ink/5 items-center">
        <button :disabled="saving" class="flex-1 bg-ink text-canvas text-sm font-medium py-2.5 rounded-xl hover:bg-ink/85 transition disabled:opacity-60" @click="save">
          {{ savedFlash ? 'Saved ✓' : 'Save' }}
        </button>
      </div>
    </div>
  </div>
</template>
