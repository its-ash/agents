<script setup lang="ts">
import { computed, ref, watch } from 'vue'
import type { Agent, CliTestResult, DetectedTools, Provider } from '../api'
import { createAgent, updateAgent, deleteAgent, testTool } from '../api'

const props = defineProps<{
  show: boolean
  agent: Agent | null
  mode: 'create' | 'edit'
  tools: DetectedTools | null
}>()

const emit = defineEmits<{
  close: []
  saved: [agent: Agent]
  created: [agent: Agent]
  deleted: [id: string]
}>()

const name = ref('')
const prompt = ref('')
const provider = ref<Provider>('openrouter')
const model = ref<string>('')
const customModel = ref<string>('')
const systemInstruction = ref<string>('')
const saving = ref(false)
const testing = ref(false)
const testResult = ref<CliTestResult | null>(null)

const isEdit = computed(() => props.mode === 'edit')
const isOllama = computed(() => provider.value === 'ollama')
const ollamaModels = computed(() => props.tools?.ollama_models ?? [])
const showModelField = computed(() =>
  provider.value === 'ollama' || provider.value === 'openrouter' || provider.value === 'openai',
)

const availableProviders = computed(() => {
  const t = props.tools
  return [
    { key: 'openrouter' as Provider, label: 'OpenRouter', available: true },
    { key: 'openai' as Provider, label: 'OpenAI', available: true },
    { key: 'copilot' as Provider, label: 'Copilot CLI', available: !!t?.copilot },
    { key: 'claude' as Provider, label: 'Claude Code', available: !!t?.claude },
    { key: 'ollama' as Provider, label: 'Ollama', available: !!t?.ollama },
  ]
})

watch(
  () => [props.show, props.agent],
  () => {
    if (!props.show) return
    if (props.agent && isEdit.value) {
      name.value = props.agent.name
      prompt.value = props.agent.prompt
      provider.value = props.agent.provider
      model.value = props.agent.model ?? ''
      customModel.value = props.agent.model ?? ''
      systemInstruction.value = ''
    } else {
      name.value = ''
      prompt.value = ''
      model.value = ''
      customModel.value = ''
      systemInstruction.value = ''
    }
    testResult.value = null
  },
)

function selectOllamaModel(m: string) {
  model.value = m
  customModel.value = m
}

async function save() {
  if (!name.value.trim() || !prompt.value.trim()) return
  saving.value = true
  try {
    let fullPrompt = prompt.value.trim()
    if (systemInstruction.value.trim()) {
      fullPrompt = `System: ${systemInstruction.value.trim()}\n\n${fullPrompt}`
    }
    const modelVal = isOllama.value
      ? (model.value || undefined)
      : showModelField.value
        ? (customModel.value.trim() || undefined)
        : undefined

    if (isEdit.value && props.agent) {
      await updateAgent(props.agent.id, name.value.trim(), fullPrompt, provider.value, modelVal)
      props.agent.name = name.value.trim()
      props.agent.prompt = fullPrompt
      props.agent.provider = provider.value
      props.agent.model = modelVal ?? null
      emit('saved', props.agent)
    } else {
      const agent = await createAgent(name.value.trim(), fullPrompt, provider.value, modelVal)
      emit('created', agent)
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
    <div class="bg-canvas rounded-2xl shadow-2xl fade-in flex flex-col" style="width: 72vw; max-height: 90vh">
      <div class="shrink-0 flex items-start justify-between p-5 pb-4 border-b border-ink/5">
        <div>
          <h3 class="text-base font-semibold tracking-tight">{{ isEdit ? 'Edit agent' : 'New agent' }}</h3>
          <p class="text-xs text-ink/45 mt-0.5">{{ isEdit ? 'Modify the name, prompt, provider, and model.' : 'Define a name, prompt template, and pick a provider.' }}</p>
        </div>
        <button class="w-8 h-8 shrink-0 flex items-center justify-center rounded-full bg-surface hover:bg-ink/5 transition shadow-sm" @click="$emit('close')">
          <svg xmlns="http://www.w3.org/2000/svg" class="w-3.5 h-3.5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round"><line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/></svg>
        </button>
      </div>

      <div class="flex-1 overflow-y-auto scroll-thin p-5 space-y-3.5">
        <div>
          <label class="block text-[11px] font-semibold text-ink/50 mb-1.5 uppercase tracking-wide">Agent name</label>
          <input v-model="name" type="text" placeholder="e.g. Blog Outline Builder"
            class="w-full bg-surface rounded-xl px-3 py-2 text-sm outline-none focus:ring-2 focus:ring-ink/10 shadow-sm placeholder:text-ink/40" />
        </div>

        <div>
          <label class="block text-[11px] font-semibold text-ink/50 mb-1.5 uppercase tracking-wide">Prompt template</label>
          <textarea v-model="prompt" rows="6" placeholder="Write a {{tone}} note about {{topic}}..."
            class="w-full bg-surface rounded-xl px-3 py-2 text-sm outline-none focus:ring-2 focus:ring-ink/10 shadow-sm resize-none placeholder:text-ink/40"></textarea>
          <p class="text-[11px] text-ink/40 mt-1.5">Use <span class="font-semibold" v-pre>{{placeholder_name}}</span> tokens — one input field will be generated per unique placeholder.</p>
        </div>

        <div>
          <label class="block text-[11px] font-semibold text-ink/50 mb-1.5 uppercase tracking-wide">System instruction (optional)</label>
          <textarea v-model="systemInstruction" rows="3" placeholder="You are a helpful assistant that writes concise, professional content."
            class="w-full bg-surface rounded-xl px-3 py-2 text-sm outline-none focus:ring-2 focus:ring-ink/10 shadow-sm resize-none placeholder:text-ink/40"></textarea>
          <p class="text-[11px] text-ink/40 mt-1.5">Prepended to the prompt on every run. Use it for persona, tone, or constraints.</p>
        </div>

        <div>
          <label class="block text-[11px] font-semibold text-ink/50 mb-1.5 uppercase tracking-wide">Provider</label>
          <div class="flex flex-wrap gap-1.5">
            <button
              v-for="p in availableProviders"
              :key="p.key"
              :disabled="!p.available"
              type="button"
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
          <label class="block text-[11px] font-semibold text-ink/50 mb-1.5 uppercase tracking-wide">Ollama model ({{ ollamaModels.length }} detected)</label>
          <select :value="model" @change="selectOllamaModel(($event.target as HTMLSelectElement).value)"
            class="w-full bg-surface rounded-xl px-3 py-2 text-sm outline-none focus:ring-2 focus:ring-ink/10 shadow-sm">
            <option v-for="m in ollamaModels" :key="m" :value="m">{{ m }}</option>
          </select>
        </div>
        <div v-if="isOllama && ollamaModels.length === 0" class="text-[11px] text-amber-700 bg-amber-50 border border-amber-200 rounded-lg px-3 py-2">
          Ollama is installed but has no models. Run <span class="font-mono">ollama pull llama3.2</span> to get one.
        </div>

        <div v-if="showModelField && !isOllama">
          <label class="block text-[11px] font-semibold text-ink/50 mb-1.5 uppercase tracking-wide">Model override (optional)</label>
          <input v-model="customModel" type="text" placeholder="e.g. anthropic/claude-3.5-sonnet"
            class="w-full bg-surface rounded-xl px-3 py-2 text-sm outline-none focus:ring-2 focus:ring-ink/10 shadow-sm placeholder:text-ink/40" />
        </div>

        <div v-if="provider === 'copilot' || provider === 'claude' || provider === 'ollama'" class="flex items-center gap-2 pt-1">
          <button
            :disabled="testing"
            type="button"
            class="flex items-center gap-1.5 bg-surface border border-ink/10 text-ink text-sm font-medium py-2 px-4 rounded-xl hover:bg-ink/5 transition disabled:opacity-60"
            @click="test"
          >
            <svg v-if="testing" class="animate-spin w-3.5 h-3.5" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24"><circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"/><path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 0 1 8-8V0C5.37 0 0 5.37 0 12h4z"/></svg>
            <svg v-else xmlns="http://www.w3.org/2000/svg" class="w-3.5 h-3.5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/><polyline points="22 4 12 14.01 9 11.01"/></svg>
            <span>{{ testing ? 'Testing…' : 'Test tool' }}</span>
          </button>
          <span v-if="testResult" class="text-[11px] font-medium" :class="testResult.ok ? 'text-emerald-700' : 'text-red-600'">
            {{ testResult.ok ? '✓ ' : '✗ ' }}{{ testResult.message }}
          </span>
        </div>
      </div>

      <div class="shrink-0 flex gap-2 p-5 pt-4 border-t border-ink/5">
        <button :disabled="saving" type="button"
          class="flex-1 bg-ink text-canvas text-sm font-medium py-2.5 rounded-xl hover:bg-ink/85 transition disabled:opacity-60"
          @click="save">{{ isEdit ? 'Save changes' : 'Create agent' }}</button>
        <button v-if="isEdit" type="button"
          class="px-4 py-2.5 rounded-xl bg-chip text-ink text-sm font-medium hover:bg-chip-hover transition"
          @click="remove">Delete</button>
        <button v-else type="button"
          class="px-4 py-2.5 rounded-xl bg-sidebar text-ink text-sm font-medium hover:bg-ink/10 transition"
          @click="$emit('close')">Cancel</button>
      </div>
    </div>
  </div>
</template>
