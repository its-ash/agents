<script setup lang="ts">
import { ref } from 'vue'

defineProps<{ show: boolean }>()
const emit = defineEmits<{ close: [] }>()

const copiedId = ref<string | null>(null)

const templates = [
  {
    id: 'email',
    name: 'Email Writer',
    description: 'Tone-controlled email with placeholders',
    prompt: 'Write a {{tone}} email to {{recipient}} about {{topic}}. Keep it under {{length}} words.',
  },
  {
    id: 'product',
    name: 'Product Description',
    description: 'Targeted marketing copy',
    prompt: 'Write a compelling product description for {{product_name}}, targeted at {{audience}}. Highlight the key benefit: {{key_benefit}}.',
  },
  {
    id: 'code',
    name: 'Code Explainer',
    description: 'Explains code to a given skill level',
    prompt: 'Explain the following {{language}} code to a {{audience_level}} developer in plain language:\n\n{{code_snippet}}',
  },
  {
    id: 'summary',
    name: 'Meeting Summarizer',
    description: 'Extracts decisions and action items',
    prompt: 'Summarize the meeting notes below into {{format}}, focused on decisions and action items. Attendees: {{attendees}}.\n\nNotes:\n{{meeting_notes}}',
  },
  {
    id: 'social',
    name: 'Social Caption',
    description: 'Platform-aware social media caption',
    prompt: 'Write a {{platform}} caption for a post about {{subject}} in a {{tone}} voice.',
  },
  {
    id: 'review',
    name: 'Code Reviewer',
    description: 'Reviews code for issues and improvements',
    prompt: 'Review the following {{language}} code for bugs, security issues, and improvements. Rate severity 1-5.\n\n{{code_snippet}}',
  },
  {
    id: 'translate',
    name: 'Translator',
    description: 'Translates text between languages',
    prompt: 'Translate the following text from {{source_language}} to {{target_language}}. Preserve tone and meaning.\n\n{{text}}',
  },
]

async function copy(template: typeof templates[0]) {
  try {
    await navigator.clipboard.writeText(template.prompt)
    copiedId.value = template.id
    setTimeout(() => (copiedId.value = null), 1500)
  } catch {
    const ta = document.createElement('textarea')
    ta.value = template.prompt
    document.body.appendChild(ta)
    ta.select()
    document.execCommand('copy')
    document.body.removeChild(ta)
    copiedId.value = template.id
    setTimeout(() => (copiedId.value = null), 1500)
  }
}
</script>

<template>
  <div
    v-if="show"
    class="fixed inset-0 bg-ink/60 backdrop-blur-sm z-50 flex items-center justify-center p-4"
    @click.self="$emit('close')"
  >
    <div class="bg-canvas w-full max-w-lg rounded-2xl shadow-2xl fade-in flex flex-col max-h-[88vh]">
      <div class="shrink-0 flex items-start justify-between p-5 pb-4 border-b border-ink/5">
        <div>
          <h3 class="text-base font-semibold tracking-tight">Template Help</h3>
          <p class="text-xs text-ink/45 mt-0.5">Supported prompt templates with placeholders. Copy any to use as a starting point.</p>
        </div>
        <button class="w-8 h-8 shrink-0 flex items-center justify-center rounded-full bg-surface hover:bg-ink/5 transition shadow-sm" @click="$emit('close')">
          <svg xmlns="http://www.w3.org/2000/svg" class="w-3.5 h-3.5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round"><line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/></svg>
        </button>
      </div>

      <div class="flex-1 overflow-y-auto scroll-thin p-5 space-y-2.5">
        <div v-for="t in templates" :key="t.id" class="bg-surface rounded-xl p-3.5 shadow-sm">
          <div class="flex items-center justify-between mb-1.5">
            <div class="flex items-center gap-2">
              <span class="text-[13px] font-semibold">{{ t.name }}</span>
              <span class="text-[10px] text-ink/40 uppercase tracking-wide font-medium">{{ t.description }}</span>
            </div>
            <button
              class="shrink-0 flex items-center gap-1 text-[11px] font-medium px-2.5 py-1.5 rounded-lg transition"
              :class="copiedId === t.id ? 'bg-emerald-100 text-emerald-700' : 'bg-sidebar text-ink hover:bg-ink/10'"
              @click="copy(t)"
            >
              <svg v-if="copiedId === t.id" xmlns="http://www.w3.org/2000/svg" class="w-3 h-3" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><polyline points="20 6 9 17 4 12"/></svg>
              <svg v-else xmlns="http://www.w3.org/2000/svg" class="w-3 h-3" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="9" y="9" width="13" height="13" rx="2" ry="2"/><path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"/></svg>
              <span>{{ copiedId === t.id ? 'Copied' : 'Copy' }}</span>
            </button>
          </div>
          <pre class="text-[11px] text-ink/70 bg-canvas rounded-lg p-2.5 whitespace-pre-wrap font-mono leading-snug">{{ t.prompt }}</pre>
        </div>
      </div>

      <div class="shrink-0 p-5 pt-4 border-t border-ink/5">
        <div class="bg-sidebar rounded-xl p-3">
          <p class="text-[11px] text-ink/55 font-semibold mb-1 uppercase tracking-wide">How placeholders work</p>
          <p class="text-[12px] text-ink/60 leading-relaxed">
            Use <span class="font-mono font-semibold" v-pre>{{placeholder_name}}</span> tokens in your prompt.
            One input field is generated per unique placeholder when running the agent.
            Underscores become spaces in the label (e.g. <span class="font-mono" v-pre>{{product_name}}</span> → "Product Name").
          </p>
        </div>
      </div>
    </div>
  </div>
</template>