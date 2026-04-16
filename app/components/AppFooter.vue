<script setup lang="ts">
const year = new Date().getFullYear()

const site = useSite()

const footerLinks = [
  {
    heading: 'Archive',
    links: [
      { label: 'All Prompts', to: '/prompts' },
      { label: 'Featured', to: '/prompts?featured=true' },
      { label: 'Latest', to: '/prompts?sort=date' },
    ],
  },
  {
    heading: 'Categories',
    links: site.categories.map(category => ({
      label: category.name,
      to: `/prompts?category=${category.slug}`,
    })),
  },
  {
    heading: 'Site',
    links: [
      { label: 'Home', to: '/' },
      { label: 'Prompt Index', to: '/prompts' },
      { label: 'Latest Entries', to: '/prompts?sort=date' },
    ],
  },
]
</script>

<template>
  <footer class="site-footer" role="contentinfo">
    <div class="footer-grid newsprint-texture">
      <div class="container">
        <div class="footer-grid__inner">
          <div class="footer-grid__brand">
            <NuxtLink to="/" class="footer-brand" :aria-label="`${site.name} — Home`">
              {{ site.name }}
            </NuxtLink>
            <p class="footer-brand__desc">
              {{ site.description }}
            </p>
            <p class="footer-brand__edition label">
              Edition: Vol I.0 &nbsp;·&nbsp; Structured on the Web
            </p>
          </div>

          <div
            v-for="col in footerLinks"
            :key="col.heading"
            class="footer-grid__col"
          >
            <h3 class="footer-col__heading">{{ col.heading }}</h3>
            <ul role="list">
              <li v-for="link in col.links" :key="link.to">
                <NuxtLink :to="link.to" class="footer-col__link">
                  {{ link.label }}
                </NuxtLink>
              </li>
            </ul>
          </div>
        </div>
      </div>
    </div>

    <div class="footer-bottom">
      <div class="container">
        <div class="footer-bottom__inner">
          <span class="label">
            © {{ year }} {{ site.name }} &nbsp;·&nbsp; Systematic Prompt Archive
          </span>
          <span class="label hide-mobile">
            Built with Nuxt &amp; Nuxt Content
          </span>
        </div>
      </div>
    </div>
  </footer>
</template>

<style lang="scss">
@use '~/assets/scss/tokens' as *;

.site-footer {
  border-top: $border-heavy;
  margin-top: auto;
}

// ── Footer grid ───────────────────────────────────────────────
.footer-grid {
  padding-block: $sp-12;
  border-bottom: $border;
}

.footer-grid__inner {
  display: grid;
  grid-template-columns: 1fr;
  gap: 0;
  border-top: $border;
  border-left: $border;

  > * {
    border-right: $border;
    border-bottom: $border;
    padding: $sp-6;
  }

  @media (min-width: $bp-md) {
    grid-template-columns: 2fr 1fr 1fr 1fr;
  }
}

// ── Brand column ──────────────────────────────────────────────
.footer-brand {
  display: block;
  font-family: $f-serif;
  font-size: 1.5rem;
  font-weight: 900;
  letter-spacing: -0.02em;
  color: $c-fg;
  text-decoration: none;
  margin-bottom: $sp-4;
  transition: color 0.15s;

  &:hover { color: $c-accent; }
}

.footer-brand__desc {
  font-family: $f-body;
  font-size: 0.875rem;
  line-height: 1.6;
  color: $c-n600;
  margin-bottom: $sp-4;
}

.footer-brand__edition {
  color: $c-n500;
}

// ── Link column ───────────────────────────────────────────────
.footer-col__heading {
  font-family: $f-mono;
  font-size: 0.6875rem;
  font-weight: 600;
  letter-spacing: 0.15em;
  text-transform: uppercase;
  color: $c-fg;
  margin-bottom: $sp-4;
}

.footer-col__link {
  display: block;
  font-family: $f-sans;
  font-size: 0.8125rem;
  color: $c-n600;
  text-decoration: none;
  padding-block: $sp-1;
  transition: color 0.15s, padding-left 0.15s;

  &:hover {
    color: $c-accent;
    padding-left: $sp-2;
  }
}

// ── Bottom bar ────────────────────────────────────────────────
.footer-bottom {
  padding-block: $sp-4;
}

.footer-bottom__inner {
  display: flex;
  align-items: center;
  justify-content: space-between;
}
</style>
