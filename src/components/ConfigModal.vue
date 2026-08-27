<script setup lang="ts">
import { computed, ref, watch } from 'vue'
import type { Agent, DetectedTools, Provider } from '../api'
import { updateAgent, deleteAgent, testTool } from '../api'
import type { CliTestResult } from '../api'

const props = defineProps<{
  show: boolean
  agent: Agent | null
  tools: DetectedTools | null
}>()

const emit = defineEmits<{
  close: []
  saved: [agent: Agent]
  deleted: [id: string]
}>()

const name = ref('')
const prompt = ref('')
const provider = ref<Provider>('openrouter')
const model = ref<string>('')
const saving = ref(false)
const testing = ref(false)
const testResult = ref<CliTestResult | null>(null)

const availableProviders = computed(() => {
  const t = props.tools
  const list: { key: Provider; label: string; available: boolean }[] = [
    { key: 'openrouter', label: 'OpenRouter', available: true },
    { key: 'openai', label: 'OpenAI', available: true },
    { key: 'copilot', label: 'Copilot CLI', available: !!t?.copilot },
    { key: 'claude', label: 'Claude Code', available: !!t?.claude },
    { key: 'ollama', label: 'Ollama', available: !!t?.ollama },
  ]
  return list
})

const isOllama = computed(() => provider.value === 'ollama')
const ollamaModels = computed(() => props.tools?.ollama_models ?? [])

watch(
  () => props.agent,
  (a) => {
    if (a) {
      name.value = a.name
      prompt.value = a.prompt
      provider.value = a.provider
      model.value = a.model ?? ''
      testResult.value = null
    }
  },
)

async function save() {
  if (!props.agent || !name.value.trim() || !prompt.value.trim()) return
  saving.value = true
  try {
    await updateAgent(
      props.agent.id,
      name.value.trim(),
      prompt.value.trim(),
      provider.value,
      isOllama.value ? (model.value || undefined) : undefined,
    )
    if (props.agent) {
      props.agent.name = name.value.trim()
      props.agent.prompt = prompt.value.trim()
      props.agent.provider = provider.value
      props.agent.model = isOllama.value ? model.value : null
      emit('saved', props.agent)
    }
  } finally {
    saving.value = false
  }
}

async function test() {
  testing.value = true
  testResult.value = null
  try {
    testResult.value = await testTool(provider.value)
  } catch (e: any) {
    testResult.value = { ok: false, message: String(e), command: '', found: false }
  } finally {
    testing.value = false
  }
}

async function remove() {
  if (!props.agent) return
  await deleteAgent(props.agent.id)
  emit('deleted', props.agent.id)
}
</script>

<template>
  <div
    v-if="show"
    class="fixed inset-0 bg-ink/60 backdrop-blur-sm z-50 flex items-center justify-center p-4"
    @click.self="$emit('close')"
  >
    <div class="bg-canvas w-full max-w-md rounded-2xl shadow-2xl p-5 fade-in max-h-[88vh] overflow-y-auto">
      <div class="flex items-start justify-between mb-4">
        <div>
          <h3 class="text-base font-semibold tracking-tight">Configure agent</h3>
          <p class="text-xs text-ink/45 mt-0.5">Edit the name, prompt template, and provider.</p>
        </div>
        <button class="w-8 h-8 shrink-0 flex items-center justify-center rounded-full bg-surface hover:bg-ink/5 transition shadow-sm" @click="$emit('close')">
          <svg xmlns="http://www.w3.org/2000/svg" class="w-3.5 h-3.5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round"><line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/></svg>
        </button>
      </div>

      <div class="space-y-3">
        <div>
          <label class="block text-[11px] font-semibold text-ink/50 mb-1.5 uppercase tracking-wide">Agent name</label>
          <input v-model="name" type="text" class="w-full bg-surface rounded-xl px-3 py-2 text-sm outline-none focus:ring-2 focus:ring-ink/10 shadow-sm" />
        </div>
        <div>
          <label class="block text-[11px] font-semibold text-ink/50 mb-1.5 uppercase tracking-wide">Prompt template</label>
          <textarea v-model="prompt" rows="5" class="w-full bg-surface rounded-xl px-3 py-2 text-sm outline-none focus:ring-2 focus:ring-ink/10 shadow-sm resize-none"></textarea>
          <p class="text-[11px] text-ink/40 mt-1.5">Use <span class="font-semibold" v-pre>{{placeholder_name}}</span> tokens — one input field will be generated per unique placeholder.</p>
        </div>

        <div>
          <label class="block text-[11px] font-semibold text-ink/50 mb-1.5 uppercase tracking-wide">Provider</label>
          <div class="flex flex-wrap gap-1.5">
            <button
              v-for="p in availableProviders"
              :key="p.key"
              :disabled="!p.available"
              class="py-1.5 px-3 rounded-xl text-[12px] font-medium transition border"
              :class="provider === p.key ? 'bg-ink text-canvas border-ink' : p.available ? 'bg-surface text-ink border-ink/10 hover:bg-ink/5' : 'bg-sidebar text-ink/30 border-ink/5 cursor-not-allowed'"
              @click="provider = p.key"
            >
              {{ p.label }}
              <span v-if="!p.available" class="ml-1 text-[9px]">— not installed</span>
            </button>
          </div>
        </div>

        <div v-if="isOllama && ollamaModels.length > 0">
          <label class="block text-[11px] font-semibold text-ink/50 mb-1.5 uppercase tracking-wide">Ollama model ({{ ollamaModels.length }} available)</label>
          <select v-model="model" class="w-full bg-surface rounded-xl px-3 py-2 text-sm outline-none focus:ring-2 focus:ring-ink/10 shadow-sm">
            <option v-for="m in ollamaModels" :key="m" :value="m">{{ m }}</option>
          </select>
        </div>
        <div v-if="isOllama && ollamaModels.length === 0" class="text-[11px] text-amber-700 bg-amber-50 border border-amber-200 rounded-lg px-3 py-2">
          Ollama is installed but has no models. Run <span class="font-mono">ollama pull llama3.2</span> to get one.
        </div>

        <div v-if="provider === 'copilot' || provider === 'claude' || provider === 'ollama'" class="flex items-center gap-2">
          <button
            :disabled="testing"
            class="flex items-center gap-1.5 bg-surface border border-ink/10 text-ink text-sm font-medium py-2 px-4 rounded-xl hover:bg-ink/5 transition disabled:opacity-60"
            @click="test"
          >
            <svg v-if="testing" class="animate-spin w-3.5 h-3.5" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24"><circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"/><path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 0 1 8-8V0C5.37 0 0 5.37 0 12h4z"/></svg>
            <svg v-else xmlns="http://www.w3.org/2000/svg" class="w-3.5 h-3.5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/><polyline points="22 4 12 14.01 9 11.01"/></svg>
            <span>{{ testing ? 'Testing…' : 'Test' }}</span>
          </button>
          <span v-if="testResult" class="text-[11px] font-medium" :class="testResult.ok ? 'text-emerald-700' : 'text-red-600'">
            {{ testResult.ok ? '✓ ' : '✗ ' }}{{ testResult.message }}
          </span>
        </div>
      </div>

      <div class="flex gap-2 mt-5">
        <button :disabled="saving" class="flex-1 bg-ink text-canvas text-sm font-medium py-2.5 rounded-xl hover:bg-ink/85 transition disabled:opacity-60" @click="save">Save changes</button>
        <button class="px-4 py-2.5 rounded-xl bg-chip text-ink text-sm font-medium hover:bg-chip-hover transition" @click="remove">Delete</button>
      </div>
    </div>
  </div>
</template>
