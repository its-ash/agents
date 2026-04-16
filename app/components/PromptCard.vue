<script setup lang="ts">
interface Prompt {
  title: string
  description: string
  category: string
  tags?: string[]
  author?: string
  date?: string
  featured?: boolean
  icon?: string
  path?: string
  stem?: string
}

const props = defineProps<{
  prompt: Prompt
  variant?: 'default' | 'featured' | 'compact'
}>()

const formattedDate = computed(() => {
  if (!props.prompt.date) return ''
  return new Date(props.prompt.date).toLocaleDateString('en-US', {
    year: 'numeric',
    month: 'short',
    day: 'numeric',
  })
})

const routePath = computed(() => {
  if (props.prompt.path) {
    return props.prompt.path
  }
  return '/prompts'
})
</script>

<template>
  <NuxtLink
    :to="routePath"
    class="card"
    :class="{
      'card--featured': variant === 'featured',
      'card--compact': variant === 'compact',
    }"
    :aria-label="`Read prompt: ${prompt.title}`"
  >
    <div class="card__eyebrow">
      <span class="card__category">{{ prompt.category }}</span>
      <span v-if="prompt.featured" class="badge badge--breaking">Featured</span>
    </div>

    <h3 class="card__title">{{ prompt.title }}</h3>

    <p class="card__description">{{ prompt.description }}</p>

    <div v-if="prompt.tags?.length" class="card__tags">
      <span
        v-for="tag in prompt.tags.slice(0, 4)"
        :key="tag"
        class="badge"
      >{{ tag }}</span>
    </div>

    <div class="card__footer">
      <div class="card__meta">
        <span v-if="formattedDate">{{ formattedDate }}</span>
        <span v-if="formattedDate && prompt.author"> · </span>
        <span v-if="prompt.author">{{ prompt.author }}</span>
      </div>
      <span class="card__arrow" aria-hidden="true">↗</span>
    </div>
  </NuxtLink>
</template>

<style lang="scss">
@use '~/assets/scss/tokens' as *;

.card--compact {
  padding: $sp-4;
  gap: $sp-2;

  .card__title { font-size: 1.125rem; }
  .card__description { font-size: 0.8125rem; }
  .card__tags { display: none; }
}
</style>
