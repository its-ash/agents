<script setup lang="ts">
const site = useSite()
const listingUrl = useAbsoluteUrl('/prompts')

useSeoMeta({
  title: 'All Agent Prompts',
  description: 'Browse the complete archive of AI agent prompts. Filter by category, search by keyword, and inspect featured systems or latest entries.',
  ogTitle: `Browse All Agent Prompts | ${site.name}`,
  ogDescription: 'Production-ready AI agent prompts organized by category, publication date, and featured status.',
  twitterCard: 'summary_large_image',
})

useHead(() => ({
  script: [
    {
      type: 'application/ld+json',
      children: JSON.stringify({
        '@context': 'https://schema.org',
        '@type': 'CollectionPage',
        name: 'All Agent Prompts',
        description: 'A curated library of AI agent prompts.',
        ...(listingUrl ? { url: listingUrl } : {}),
      }),
    },
  ],
}))

const route = useRoute()
const router = useRouter()

const categories = [
  'All',
  ...site.categories.map(category => category.name),
]

function resolveCategory(value: string | string[] | undefined) {
  if (typeof value !== 'string' || !value) {
    return 'All'
  }

  const match = site.categories.find(category => category.slug === value || category.name.toLowerCase() === value.toLowerCase())
  return match?.name || 'All'
}

const selectedCategory = ref(resolveCategory(route.query.category))
const searchQuery = ref((route.query.q as string) || '')
const sortBy = ref((route.query.sort as string) || 'featured')
const featuredOnly = ref(route.query.featured === 'true')

const { data: allPrompts } = await useAsyncData('prompts-list', () =>
  queryCollection('prompts').all()
)

watch(
  () => route.query,
  (query) => {
    selectedCategory.value = resolveCategory(query.category)
    searchQuery.value = typeof query.q === 'string' ? query.q : ''
    sortBy.value = typeof query.sort === 'string' ? query.sort : 'featured'
    featuredOnly.value = query.featured === 'true'
  }
)

watch([selectedCategory, searchQuery, sortBy, featuredOnly], () => {
  const matchedCategory = site.categories.find(category => category.name === selectedCategory.value)

  router.replace({
    query: {
      ...(matchedCategory ? { category: matchedCategory.slug } : {}),
      ...(searchQuery.value.trim() ? { q: searchQuery.value.trim() } : {}),
      ...(sortBy.value !== 'featured' ? { sort: sortBy.value } : {}),
      ...(featuredOnly.value ? { featured: 'true' } : {}),
    },
  })
})

const filteredPrompts = computed(() => {
  let list = allPrompts.value ?? []

  if (featuredOnly.value) {
    list = list.filter(prompt => Boolean(prompt.featured))
  }

  if (selectedCategory.value && selectedCategory.value !== 'All') {
    list = list.filter(p => p.category?.toLowerCase() === selectedCategory.value.toLowerCase())
  }

  if (searchQuery.value.trim()) {
    const q = searchQuery.value.toLowerCase()
    list = list.filter(
      p =>
        p.title?.toLowerCase().includes(q) ||
        p.description?.toLowerCase().includes(q) ||
        p.tags?.some((t: string) => t.toLowerCase().includes(q))
    )
  }

  if (sortBy.value === 'date') {
    list = [...list].sort((a, b) => new Date(b.date).getTime() - new Date(a.date).getTime())
  } else if (sortBy.value === 'featured') {
    list = [...list].sort((a, b) => (b.featured ? 1 : 0) - (a.featured ? 1 : 0))
  }

  return list
})
</script>

<template>
  <div>
    <!-- Page header -->
    <section class="prompts-header newsprint-texture section--sm">
      <div class="container">
        <span class="label label--accent">Complete Archive</span>
        <h1 class="headline-section mt-4">All Prompts</h1>
        <p class="prompts-header__lead mt-4">
          {{ filteredPrompts.length }} prompt{{ filteredPrompts.length !== 1 ? 's' : '' }}
          in the dispatch.
        </p>
      </div>
    </section>

    <div class="section-divider"></div>

    <!-- Filters -->
    <div class="prompts-filters section--sm">
      <div class="container">
        <div class="prompts-filters__inner">
          <div class="prompts-filters__search">
            <label for="search-prompts" class="sr-only">Search prompts</label>
            <input
              id="search-prompts"
              v-model="searchQuery"
              type="search"
              placeholder="Search prompts…"
              class="input"
              aria-controls="prompts-grid"
            />
          </div>

          <div class="prompts-filters__cats" role="tablist" aria-label="Filter by category">
            <button
              v-for="cat in categories"
              :key="cat"
              role="tab"
              class="filter-tab"
              :class="{ 'filter-tab--active': selectedCategory === cat }"
              :aria-selected="selectedCategory === cat"
              @click="selectedCategory = cat"
            >
              {{ cat }}
            </button>
          </div>

          <div class="prompts-filters__sort">
            <label for="sort-prompts" class="sr-only">Sort by</label>
            <select id="sort-prompts" v-model="sortBy" class="input prompts-filters__select">
              <option value="featured">Featured First</option>
              <option value="date">Newest First</option>
            </select>
          </div>

          <button
            class="filter-toggle"
            :class="{ 'filter-toggle--active': featuredOnly }"
            @click="featuredOnly = !featuredOnly"
          >
            Featured Only
          </button>
        </div>
      </div>
    </div>

    <div class="section-divider--muted"></div>

    <!-- Grid -->
    <section class="section" id="categories">
      <div class="container">
        <!-- No results -->
        <div v-if="filteredPrompts.length === 0" class="prompts-empty">
          <p class="label">No prompts found for "{{ searchQuery }}"</p>
          <button class="btn btn--secondary mt-6" @click="searchQuery = ''; selectedCategory = 'All'; featuredOnly = false; sortBy = 'featured'">
            Clear filters
          </button>
        </div>

        <!-- Prompt grid -->
        <div
          v-else
          id="prompts-grid"
          class="grid-newspaper"
          :aria-live="'polite'"
          :aria-label="`${filteredPrompts.length} prompts`"
        >
          <PromptCard
            v-for="prompt in filteredPrompts"
            :key="prompt.path"
            :prompt="prompt"
            :variant="prompt.featured ? 'featured' : 'default'"
          />
        </div>
      </div>
    </section>
  </div>
</template>

<style lang="scss">
@use '~/assets/scss/tokens' as *;

.prompts-header {
  border-bottom: $border;
}

.prompts-header__lead {
  font-family: $f-mono;
  font-size: 0.875rem;
  color: $c-n500;
  letter-spacing: 0.04em;
}

// ── Filters ───────────────────────────────────────────────────
.prompts-filters {
  border-bottom: $border;
}

.prompts-filters__inner {
  display: flex;
  align-items: flex-start;
  gap: $sp-6;
  flex-wrap: wrap;
}

.prompts-filters__search {
  flex: 1;
  min-width: 200px;
}

.prompts-filters__cats {
  display: flex;
  flex-wrap: wrap;
  gap: $sp-2;
}

.prompts-filters__sort {
  flex-shrink: 0;
}

.prompts-filters__select {
  width: auto;
}

// ── Filter tab ────────────────────────────────────────────────
.filter-tab {
  font-family: $f-mono;
  font-size: 0.6875rem;
  font-weight: 500;
  letter-spacing: 0.1em;
  text-transform: uppercase;
  padding: $sp-2 $sp-4;
  min-height: 36px;
  background: transparent;
  border: 1px solid $c-muted;
  color: $c-n600;
  cursor: pointer;
  transition: all 0.15s ease-out;

  &:hover {
    border-color: $c-fg;
    color: $c-fg;
  }

  &--active {
    background-color: $c-fg;
    color: $c-bg;
    border-color: $c-fg;
  }
}

.filter-toggle {
  min-height: 52px;
  padding: $sp-3 $sp-4;
  border: $border;
  background: $c-bg;
  color: $c-fg;
  font-family: $f-sans;
  font-size: 0.75rem;
  font-weight: 800;
  letter-spacing: 0.16em;
  text-transform: uppercase;
  cursor: pointer;
  transition: background-color 0.15s linear, color 0.15s linear, border-color 0.15s linear;

  &:hover,
  &--active {
    background-color: $c-accent;
    border-color: $c-accent;
    color: $c-bg;
  }
}

// ── Empty state ───────────────────────────────────────────────
.prompts-empty {
  padding: $sp-16;
  text-align: center;
  border: $border;
}
</style>
