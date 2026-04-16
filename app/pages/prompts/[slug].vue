<script setup lang="ts">
const route = useRoute()
const slug = route.params.slug as string
const site = useSite()
const promptUrl = useAbsoluteUrl(`/prompts/${slug}`)

const { data: prompt } = await useAsyncData(`prompt-${slug}`, () =>
  queryCollection('prompts')
    .path(`/prompts/${slug}`)
    .first()
)

// 404 if not found
if (!prompt.value) {
  throw createError({ statusCode: 404, message: 'Prompt not found' })
}

useSeoMeta({
  title: prompt.value.title,
  description: prompt.value.description,
  ogTitle: `${prompt.value.title} | ${site.name}`,
  ogDescription: prompt.value.description,
  ogType: 'article',
  twitterCard: 'summary_large_image',
  twitterTitle: prompt.value.title,
  twitterDescription: prompt.value.description,
  articlePublishedTime: prompt.value.date,
  articleSection: prompt.value.category,
  articleTag: prompt.value.tags?.join(', '),
})

useHead(() => ({
  script: [
    {
      type: 'application/ld+json',
      children: JSON.stringify({
        '@context': 'https://schema.org',
        '@type': 'TechArticle',
        headline: prompt.value?.title,
        description: prompt.value?.description,
        datePublished: prompt.value?.date,
        author: { '@type': 'Person', name: prompt.value?.author ?? 'Community' },
        keywords: prompt.value?.tags?.join(', '),
        articleSection: prompt.value?.category,
        ...(promptUrl ? { url: promptUrl } : {}),
      }),
    },
  ],
}))

const breadcrumbs = computed(() => [
  { label: 'Home', to: '/' },
  { label: 'Prompts', to: '/prompts' },
  { label: prompt.value!.title, to: route.fullPath },
])

const formattedDate = computed(() => {
  if (!prompt.value?.date) return ''
  return new Date(prompt.value.date).toLocaleDateString('en-US', {
    weekday: 'long',
    year: 'numeric',
    month: 'long',
    day: 'numeric',
  })
})

const copied = ref(false)

async function copyPrompt() {  
  try {
    await navigator.clipboard.writeText(String(prompt.value?.rawbody.split("---")[2]).trim() ?? '')
    copied.value = true
    setTimeout(() => { copied.value = false }, 2000)
  } catch {
    copied.value = false
  }
}
</script>

<template>
  <div v-if="prompt">
    <nav class="breadcrumb-nav" aria-label="Breadcrumb">
      <div class="container">
        <ol class="breadcrumb" role="list">
          <li v-for="(crumb, i) in breadcrumbs" :key="crumb.to" class="breadcrumb__item">
            <NuxtLink
              v-if="i < breadcrumbs.length - 1"
              :to="crumb.to"
              class="breadcrumb__link"
            >{{ crumb.label }}</NuxtLink>
            <span v-else class="breadcrumb__current" aria-current="page">{{ crumb.label }}</span>
            <span v-if="i < breadcrumbs.length - 1" class="breadcrumb__sep" aria-hidden="true"> / </span>
          </li>
        </ol>
      </div>
    </nav>

    <div class="section-divider--muted"></div>

    <div class="prompt-detail section">
      <div class="container">
        <div class="prompt-detail__layout">
          <article class="prompt-detail__main" aria-labelledby="prompt-title">
            <div class="flex items-center gap-4 mb-4">
              <span class="badge badge--accent">{{ prompt.category }}</span>
              <span v-if="prompt.featured" class="badge badge--breaking">Featured</span>
            </div>

            <h1 id="prompt-title" class="prompt-detail__title">{{ prompt.title }}</h1>

            <div class="prompt-detail__meta">
              <span class="byline">By {{ prompt.author ?? 'Community' }}</span>
              <span class="byline" aria-hidden="true"> · </span>
              <time :datetime="prompt.date" class="byline">{{ formattedDate }}</time>
              <span v-if="prompt.model" class="byline"> · {{ prompt.model }}</span>
            </div>

            <hr class="mt-6 mb-6" />

            <p class="prompt-detail__description drop-cap">{{ prompt.description }}</p>

            <div v-if="prompt.tags?.length" class="flex flex-wrap gap-2 mt-6">
              <span v-for="tag in prompt.tags" :key="tag" class="badge">{{ tag }}</span>
            </div>

            <hr class="mt-8 mb-6" />

            <div class="prompt-block">
              <div class="prompt-block__copy">
                <button
                  class="btn btn--ghost"
                  style="color: var(--c-bg); font-size: 0.625rem;"
                  :aria-label="copied ? 'Copied!' : 'Copy prompt to clipboard'"
                  @click="copyPrompt"
                >
                  {{ copied ? '✓ Copied' : 'Copy' }}
                </button>
              </div>
              <div>
                <ContentRenderer v-if="prompt.body" :value="prompt" class="prose" />
                <p v-else style="color: var(--c-n400); font-style: italic;">
                  No prompt body found.
                </p>
              </div>
            </div>

          </article>
        </div>
      </div>
    </div>
  </div>
</template>

<style lang="scss">
@use '~/assets/scss/tokens' as *;

// ── Breadcrumb ────────────────────────────────────────────────
.breadcrumb-nav {
  padding-block: $sp-4;
}

.breadcrumb {
  display: flex;
  align-items: center;
  flex-wrap: wrap;
  gap: $sp-1;
  list-style: none;
  padding: 0;
}

.breadcrumb__item {
  display: flex;
  align-items: center;
  gap: $sp-1;
}

.breadcrumb__link {
  font-family: $f-mono;
  font-size: 0.6875rem;
  letter-spacing: 0.1em;
  text-transform: uppercase;
  color: $c-n500;
  text-decoration: none;
  transition: color 0.15s;

  &:hover { color: $c-accent; }
}

.breadcrumb__current {
  font-family: $f-mono;
  font-size: 0.6875rem;
  letter-spacing: 0.1em;
  text-transform: uppercase;
  color: $c-fg;
  font-weight: 600;
}

.breadcrumb__sep {
  font-family: $f-mono;
  font-size: 0.6875rem;
  color: $c-n400;
}

// ── Prompt detail layout ──────────────────────────────────────
.prompt-detail__layout {
  display: block;
}

.prompt-detail__main {
  max-width: 860px;
  margin: 0 auto;
  padding: $sp-8 0;
}

// ── Prompt title ──────────────────────────────────────────────
.prompt-detail__title {
  font-family: $f-serif;
  font-size: clamp(2rem, 4vw, 3.5rem);
  font-weight: 900;
  letter-spacing: -0.03em;
  line-height: 1.05;
  color: $c-fg;
  margin: $sp-4 0 $sp-4;
}

.prompt-detail__meta {
  display: flex;
  align-items: center;
  flex-wrap: wrap;
  gap: $sp-2;
}

.prompt-detail__description {
  font-family: $f-body;
  font-size: 1.125rem;
  line-height: 1.75;
  color: $c-n600;
  text-align: justify;
}


</style>
