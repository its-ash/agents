<script setup lang="ts">
import { computed, ref, watch } from 'vue'
import type { Agent, Run } from '../api'
import { runAgent, stopAgent } from '../api'

const props = defineProps<{
  agent: Agent
}>()

const emit = defineEmits<{
  ran: [agent: Agent]
  error: [message: string]
}>()

const placeholders = computed(() => extractPlaceholders(props.agent.prompt))
const values = ref<Record<string, string>>({})
const running = ref(false)
const stopped = ref(false)
const error = ref<string | null>(null)
const copiedRunId = ref<string | null>(null)

const searchQuery = ref('')

const filteredRuns = computed(() => {
  const q = searchQuery.value.trim().toLowerCase()
  if (!q) return props.agent.runs
  return props.agent.runs.filter((r) => r.output.toLowerCase().includes(q))
})

function highlightText(text: string): string {
  const q = searchQuery.value.trim()
  if (!q) return escapeHtml(text)
  const escaped = escapeRegex(q)
  const re = new RegExp(`(${escaped})`, 'gi')
  return escapeHtml(text).replace(re, '<mark class="search-mark">$1</mark>')
}

function escapeHtml(s: string): string {
  return s
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/"/g, '&quot;')
    .replace(/'/g, '&#39;')
}

function escapeRegex(s: string): string {
  return s.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')
}

async function copyOutput(run: Run) {
  try {
    await navigator.clipboard.writeText(run.output)
    copiedRunId.value = run.id
    setTimeout(() => (copiedRunId.value = null), 1500)
  } catch {
    const ta = document.createElement('textarea')
    ta.value = run.output
    document.body.appendChild(ta)
    ta.select()
    document.execCommand('copy')
    document.body.removeChild(ta)
    copiedRunId.value = run.id
    setTimeout(() => (copiedRunId.value = null), 1500)
  }
}

watch(
  () => props.agent.id,
  () => {
    values.value = {}
    error.value = null
    searchQuery.value = ''
  },
)

function extractPlaceholders(template: string): string[] {
  const re = /\{\{\s*([a-zA-Z0-9_]+)\s*\}\}/g
  const seen = new Set<string>()
  const out: string[] = []
  let m: RegExpExecArray | null
  while ((m = re.exec(template)) !== null) {
    if (!seen.has(m[1])) {
      seen.add(m[1])
      out.push(m[1])
    }
  }
  return out
}

function labelize(name: string) {
  return name.split('_').map((w) => w.charAt(0).toUpperCase() + w.slice(1)).join(' ')
}

function formatTime(ts: number) {
  return new Date(ts * 1000).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })
}

function inputEntries(run: Run): { name: string; value: string }[] {
  if (!run.inputs) return []
  return Object.entries(run.inputs)
    .filter(([, v]) => v && v.trim() !== '' && !v.startsWith('['))
    .map(([name, value]) => ({ name, value }))
}

async function handleRun() {
  if (running.value) return
  running.value = true
  stopped.value = false
  error.value = null
  try {
    const payload: Record<string, string> = {}
    for (const p of placeholders.value) {
      payload[p] = values.value[p]?.trim() || `[${p}]`
    }
    const run = await runAgent(props.agent.id, payload)
    props.agent.runs.unshift(run)
    emit('ran', props.agent)
  } catch (e: any) {
    error.value = String(e)
    emit('error', String(e))
  } finally {
    running.value = false
  }
}

async function handleStop() {
  if (!running.value) return
  stopped.value = true
  try {
    await stopAgent(props.agent.id)
  } catch {
    /* ignore */
  }
  running.value = false
}
</script>

<template>
  <div class="max-w-2xl mx-auto px-4 sm:px-6 py-4 sm:py-6 space-y-4 fade-in">
    <div class="bg-surface rounded-2xl shadow-[0_4px_24px_-6px_rgba(0,0,0,0.15),0_8px_12px_-4px_rgba(0,0,0,0.08)] p-4 sm:p-5">
      <p class="text-[11px] font-semibold text-ink/40 uppercase tracking-wide mb-1">Agent</p>
      <h1 class="text-xl sm:text-2xl font-semibold tracking-tight mb-4">{{ agent.name }}</h1>

      <p class="text-[11px] font-semibold text-ink/40 uppercase tracking-wide mb-2">Inputs</p>
      <div class="space-y-2.5">
        <p v-if="placeholders.length === 0" class="text-[13px] text-ink/40">
          This prompt has no placeholders — it's ready to run as-is.
        </p>
        <div v-for="name in placeholders" :key="name">
          <label class="block text-[13px] font-medium mb-1">{{ labelize(name) }}</label>
          <input
            v-model="values[name]"
            type="text"
            class="w-full bg-canvas rounded-xl px-3 py-2 text-sm outline-none focus:ring-2 focus:ring-ink/10 placeholder:text-ink/40"
            :placeholder="`Enter ${labelize(name).toLowerCase()}...`"
          />
        </div>
      </div>

      <div class="border-t border-ink/5 mt-4 pt-4 flex items-center gap-3">
        <button
          id="runBtn"
          v-if="!running"
          class="w-full sm:w-auto sm:px-6 flex items-center justify-center gap-1.5 bg-ink text-canvas text-sm font-medium py-2.5 px-4 rounded-xl hover:bg-ink/85 active:scale-[0.98] transition shadow-sm"
          @click="handleRun"
        >
          <svg xmlns="http://www.w3.org/2000/svg" class="w-3.5 h-3.5" viewBox="0 0 24 24" fill="currentColor"><path d="M8 5v14l11-7z"/></svg>
          <span>Run</span>
        </button>
        <button
          id="stopBtn"
          v-else
          class="w-full sm:w-auto sm:px-6 flex items-center justify-center gap-1.5 bg-red-600 text-white text-sm font-medium py-2.5 px-4 rounded-xl hover:bg-red-500 active:scale-[0.98] transition shadow-sm"
          @click="handleStop"
        >
          <svg xmlns="http://www.w3.org/2000/svg" class="w-3.5 h-3.5" viewBox="0 0 24 24" fill="currentColor"><path d="M6 6h12v12H6z"/></svg>
          <span>Stop</span>
        </button>
        <span v-if="running" class="flex items-center gap-1.5 text-[11px] text-ink/40">
          <span class="dot-pulse"><span></span><span></span><span></span></span>
          <span>Running…</span>
        </span>
        <span v-if="agent.provider" class="text-[11px] text-ink/40 uppercase tracking-wide font-semibold ml-auto">{{ agent.provider }}</span>
      </div>

      <p v-if="stopped" class="mt-3 text-[12px] text-amber-600 bg-amber-50 rounded-lg px-3 py-2">Agent stopped by user.</p>
      <p v-if="error && !stopped" class="mt-3 text-[12px] text-red-600 bg-red-50 rounded-lg px-3 py-2">{{ error }}</p>
    </div>

    <div>
      <div class="flex items-center justify-between gap-3 mb-2">
        <p class="text-[11px] font-semibold text-ink/40 uppercase tracking-wide shrink-0">Response</p>
        <div v-if="agent.runs.length > 0" class="relative flex-1 max-w-[220px]">
          <svg xmlns="http://www.w3.org/2000/svg" class="w-3.5 h-3.5 absolute left-2.5 top-1/2 -translate-y-1/2 text-ink/30 pointer-events-none" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="11" cy="11" r="8"/><line x1="21" y1="21" x2="16.65" y2="16.65"/></svg>
          <input
            v-model="searchQuery"
            type="text"
            class="w-full bg-surface rounded-lg pl-8 pr-3 py-1.5 text-[13px] outline-none focus:ring-2 focus:ring-ink/10 placeholder:text-ink/40 shadow-sm"
            placeholder="Search responses…"
          />
        </div>
      </div>
      <div class="space-y-3">
        <div
          v-for="(run, idx) in filteredRuns"
          :key="run.id"
          class="fade-in bg-gradient-to-br from-[#2A2622] to-[#181614] text-white rounded-2xl shadow-[0_8px_24px_-6px_rgba(0,0,0,0.4),0_4px_8px_-2px_rgba(0,0,0,0.25)] p-4 sm:p-5"
        >
            <div class="flex items-center justify-between mb-1.5">
              <span class="inline-flex items-center gap-1 text-[10px] font-semibold uppercase tracking-wide text-white/40">
                <span class="w-1.5 h-1.5 rounded-full bg-emerald-400"></span>
                Run {{ agent.runs.length - agent.runs.indexOf(run) }}
              </span>
              <div class="flex items-center gap-2">
                <button
                  class="flex items-center gap-1 text-[10px] font-semibold text-white/50 hover:text-white transition"
                  @click="copyOutput(run)"
                >
                  <svg v-if="copiedRunId === run.id" xmlns="http://www.w3.org/2000/svg" class="w-3 h-3" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><polyline points="20 6 9 17 4 12"/></svg>
                  <svg v-else xmlns="http://www.w3.org/2000/svg" class="w-3 h-3" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="9" y="9" width="13" height="13" rx="2" ry="2"/><path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"/></svg>
                  <span>{{ copiedRunId === run.id ? 'Copied' : 'Copy' }}</span>
                </button>
                <span class="text-[10px] text-white/30">{{ formatTime(run.created_at) }}</span>
              </div>
            </div>

            <div v-if="inputEntries(run).length > 0" class="flex flex-wrap gap-1.5 mb-2">
              <span
                v-for="entry in inputEntries(run)"
                :key="entry.name"
                class="inline-flex items-center gap-1 bg-indigo-500/15 text-indigo-300 rounded-full px-2 py-0.5 text-[10px] font-semibold"
              >
                <span class="text-indigo-400/70">{{ labelize(entry.name) }}:</span>
                <span>{{ entry.value }}</span>
              </span>
            </div>

            <p class="text-[13px] leading-snug whitespace-pre-wrap text-white/95" v-html="highlightText(run.output)"></p>
            <div v-if="run.model || run.tokens" class="mt-1.5 flex gap-2 flex-wrap">
              <span v-if="run.model" class="inline-flex items-center bg-surface/10 text-white/70 rounded-full px-2 py-0.5 text-[10px] font-semibold">{{ run.model }}</span>
              <span v-if="run.tokens" class="inline-flex items-center bg-surface/10 text-white/70 rounded-full px-2 py-0.5 text-[10px] font-semibold">{{ run.tokens }} tokens</span>
            </div>
        </div>
        <div v-if="agent.runs.length === 0" class="fade-in bg-gradient-to-br from-[#2A2622] to-[#181614] rounded-2xl shadow-[0_8px_24px_-6px_rgba(0,0,0,0.4),0_4px_8px_-2px_rgba(0,0,0,0.25)] p-4 sm:p-5 min-h-[100px] flex flex-col items-center justify-center text-center text-white/40">
          <svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5 mb-2" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round"><path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z"/></svg>
          <p class="text-[13px]">Run the agent to see its response here.</p>
        </div>
        <div v-else-if="filteredRuns.length === 0" class="fade-in bg-gradient-to-br from-[#2A2622] to-[#181614] rounded-2xl shadow-[0_8px_24px_-6px_rgba(0,0,0,0.4),0_4px_8px_-2px_rgba(0,0,0,0.25)] p-4 sm:p-5 min-h-[80px] flex flex-col items-center justify-center text-center text-white/40">
          <svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5 mb-2" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round"><circle cx="11" cy="11" r="8"/><line x1="21" y1="21" x2="16.65" y2="16.65"/></svg>
          <p class="text-[13px]">No responses match "{{ searchQuery }}".</p>
        </div>
      </div>
    </div>
  </div>
</template>