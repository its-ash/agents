<script setup lang="ts">
const site = useSite()
const homeUrl = useAbsoluteUrl('/')
const promptsUrl = useAbsoluteUrl('/prompts')

useSeoMeta({
  title: site.name,
  titleTemplate: '%s',
  description: site.description,
  ogTitle: `${site.name} — Structured Systems for AI Workflows`,
  ogDescription: site.description,
  ogType: 'website',
  twitterCard: 'summary_large_image',
  twitterTitle: site.name,
  twitterDescription: site.description,
})

useHead(() => ({
  script: [
    {
      type: 'application/ld+json',
      children: JSON.stringify({
        '@context': 'https://schema.org',
        '@type': 'WebSite',
        name: site.name,
        description: site.description,
        ...(homeUrl ? { url: homeUrl } : {}),
        ...(promptsUrl
          ? {
              potentialAction: {
                '@type': 'SearchAction',
                target: `${promptsUrl}?q={search_term_string}`,
                'query-input': 'required name=search_term_string',
              },
            }
          : {}),
      }),
    },
  ],
}))

const { data: allPrompts } = await useAsyncData('all-prompts', () =>
  queryCollection('prompts').order('date', 'DESC').all()
)

const searchQuery = ref('')
const searchFocused = ref(false)
const searchRef = ref<HTMLElement | null>(null)

const suggestions = computed(() => {
  const q = searchQuery.value.trim().toLowerCase()
  if (!q || !allPrompts.value) return []
  return allPrompts.value
    .filter(p =>
      p.title.toLowerCase().includes(q) ||
      p.description?.toLowerCase().includes(q) ||
      p.category?.toLowerCase().includes(q) ||
      p.tags?.some(t => t.toLowerCase().includes(q))
    )
    .slice(0, 6)
})

const showDropdown = computed(() => searchFocused.value && suggestions.value.length > 0)

function handleSearch() {
  if (searchQuery.value.trim()) {
    searchFocused.value = false
    navigateTo(`/prompts?q=${encodeURIComponent(searchQuery.value.trim())}`)
  }
}

function selectSuggestion(prompt: { path?: string }) {
  searchFocused.value = false
  navigateTo(prompt.path ?? '/prompts')
}

function handleBlur() {
  // Delay so click on suggestion fires first
  setTimeout(() => { searchFocused.value = false }, 150)
}
</script>

<template>
  <div>
    <!-- Hero 1: Search -->
    <section class="home-hero" aria-labelledby="home-heading">
      <div class="container">
        <div class="home-hero__inner">
          <p class="home-hero__label label">AI Agent Library</p>
          <h1 id="home-heading" class="home-hero__title">
            Find the right<br>agent for the job.
          </h1>
          <p class="home-hero__sub">
            {{ site.description }}
          </p>
          <form class="home-search" role="search" @submit.prevent="handleSearch" ref="searchRef">
            <label for="home-search-input" class="sr-only">Search agents</label>
            <div class="home-search__wrap">
              <input
                id="home-search-input"
                v-model="searchQuery"
                type="search"
                class="home-search__input"
                placeholder="Search agents — e.g. coding, research, design…"
                autocomplete="off"
                spellcheck="false"
                @focus="searchFocused = true"
                @blur="handleBlur"
              >
              <ul v-if="showDropdown" class="home-search__dropdown" role="listbox" aria-label="Suggestions">
                <li
                  v-for="prompt in suggestions"
                  :key="prompt.path"
                  class="home-search__suggestion"
                  role="option"
                  @mousedown.prevent="selectSuggestion(prompt)"
                >
                  <span class="home-search__suggestion-title">{{ prompt.title }}</span>
                  <span class="home-search__suggestion-cat">{{ prompt.category }}</span>
                </li>
              </ul>
            </div>
            <button type="submit" class="home-search__btn btn btn--primary">
              Search
            </button>
          </form>
          <p class="home-hero__count label" aria-live="polite">
            {{ allPrompts?.length ?? 0 }} agents available
          </p>
        </div>
      </div>
    </section>

    <div class="section-divider"></div>

    <!-- Hero 2: All Agents -->
    <section class="section" aria-labelledby="agents-heading">
      <div class="container">
        <div class="home-agents-header">
          <h2 id="agents-heading" class="headline-section">All Agents</h2>
          <NuxtLink to="/prompts" class="btn btn--ghost">Browse Archive →</NuxtLink>
        </div>
        <div class="home-agents-grid mt-8">
          <PromptCard
            v-for="prompt in allPrompts"
            :key="prompt.path"
            :prompt="prompt"
          />
        </div>
      </div>
    </section>
  </div>
</template>

<style lang="scss">
@use '~/assets/scss/tokens' as *;

// ── Hero 1: Search ─────────────────────────────────────────────
.home-hero {
  padding: $sp-24 0 $sp-20;
  border-bottom: $border;
}

.home-hero__inner {
  max-width: 720px;
  margin: 0 auto;
  text-align: center;
  display: flex;
  flex-direction: column;
  align-items: center;
}

.home-hero__label {
  color: $c-accent;
  margin-bottom: $sp-4;
}

.home-hero__title {
  font-family: $f-serif;
  font-size: clamp(2.5rem, 6vw, 4.5rem);
  font-weight: 900;
  line-height: 1.05;
  letter-spacing: -0.02em;
  color: $c-fg;
  margin: 0 0 $sp-6;
}

.home-hero__sub {
  font-family: $f-body;
  font-size: 1.0625rem;
  line-height: 1.7;
  color: $c-n500;
  margin: 0 0 $sp-8;
  max-width: 52ch;
}

.home-hero__count {
  color: $c-n400;
  margin-top: $sp-4;
}

// ── Search form ────────────────────────────────────────────────
.home-search {
  display: flex;
  gap: 0;
  border: $border;
  width: 100%;
  max-width: 620px;
  position: relative;
}

.home-search__wrap {
  flex: 1;
  position: relative;
  min-width: 0;
}

.home-search__input {
  width: 100%;
  padding: $sp-4 $sp-6;
  font-family: $f-body;
  font-size: 1rem;
  color: $c-fg;
  background: $c-bg;
  border: none;
  outline: none;
  display: block;

  &::placeholder { color: $c-n400; }
  &:focus { background: $c-n100; }
}

.home-search__dropdown {
  position: absolute;
  top: 100%;
  left: -2px;
  right: 0;
  background: $c-bg;
  border: $border;
  border-top: none;
  list-style: none;
  margin: 0;
  padding: 0;
  z-index: 100;
  box-shadow: 4px 4px 0 $c-fg;
}

.home-search__suggestion {
  display: flex;
  align-items: baseline;
  justify-content: space-between;
  gap: $sp-4;
  padding: $sp-3 $sp-6;
  cursor: pointer;
  border-bottom: 1px solid $c-n200;
  transition: background 0.1s;

  &:last-child { border-bottom: none; }

  &:hover {
    background: $c-n100;
  }
}

.home-search__suggestion-title {
  font-family: $f-body;
  font-size: 0.9375rem;
  font-weight: 600;
  color: $c-fg;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.home-search__suggestion-cat {
  font-family: $f-mono;
  font-size: 0.6875rem;
  text-transform: uppercase;
  letter-spacing: 0.08em;
  color: $c-accent;
  flex-shrink: 0;
}

.home-search__btn {
  border-radius: 0;
  border: none;
  border-left: $border;
  flex-shrink: 0;
  white-space: nowrap;
}

// ── Hero 2: Agents grid ────────────────────────────────────────
.home-agents-header {
  display: flex;
  align-items: baseline;
  justify-content: space-between;
  gap: $sp-4;
  flex-wrap: wrap;
}

.home-agents-grid {
  display: grid;
  grid-template-columns: 1fr;
  border-top: $border;
  border-left: $border;

  @media (min-width: $bp-md) { grid-template-columns: repeat(2, 1fr); }
  @media (min-width: $bp-lg) { grid-template-columns: repeat(3, 1fr); }

  > * {
    border-right: $border;
    border-bottom: $border;
  }
}

.sr-only {
  position: absolute;
  width: 1px;
  height: 1px;
  padding: 0;
  margin: -1px;
  overflow: hidden;
  clip: rect(0,0,0,0);
  white-space: nowrap;
  border: 0;
}
</style>
