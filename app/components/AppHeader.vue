<script setup lang="ts">
const site = useSite()
</script>

<template>
  <header class="site-header" role="banner">
    <div class="header-top newsprint-texture">
      <div class="container">
        <div class="header-top__inner">
          <NuxtLink to="/" class="masthead__logo" :aria-label="`${site.name} — Home`">
            <span class="masthead__title">{{ site.name }}</span>
          </NuxtLink>

          <div class="nav-bar__actions">
            <NuxtLink to="/prompts" class="btn btn--primary">
              Browse
            </NuxtLink>
          </div>
        </div>
      </div>
    </div>
  </header>
</template>

<style lang="scss">
@use '~/assets/scss/tokens' as *;

// ── Site header ───────────────────────────────────────────────
.site-header {
  position: sticky;
  top: 0;
  z-index: $z-header;
  background-color: $c-bg;
  border-bottom: $border-heavy;
}

.header-top {
  border-top: $border;
}

.header-top__inner {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: $sp-4;
  min-height: 100px;
}

.masthead__logo {
  text-decoration: none;
  display: inline-flex;
  align-items: center;
  min-height: 48px;
  padding-right: $sp-4;
}

.masthead__title {
  font-family: $f-serif;
  font-size: clamp(1.5rem, 2.4vw, 2.2rem);
  font-weight: 900;
  letter-spacing: -0.04em;
  line-height: 0.95;
  text-transform: uppercase;
  color: $c-fg;
  transition: color 0.15s ease-out;

  .masthead__logo:hover & { color: $c-accent; }
}

// ── Nav bar ───────────────────────────────────────────────────
.nav-bar {
  margin-left: auto;
}

.nav-bar__links {
  display: flex;
  align-items: center;
  list-style: none;
  border-left: $border;
}

.nav-bar__actions {
  display: flex;
  align-items: center;
  gap: $sp-3;
}

// ── Hamburger ─────────────────────────────────────────────────
.nav-bar__hamburger {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 44px;
  height: 44px;
  background: none;
  border: $border;
  cursor: pointer;
  padding: 0;
  transition: background 0.15s;

  &:hover { background-color: $c-fg; }
}

.nav-bar__hamburger-icon {
  display: flex;
  flex-direction: column;
  gap: 4px;

  span {
    display: block;
    width: 18px;
    height: 2px;
    background-color: $c-fg;
    transition: background 0.15s;
  }

  .nav-bar__hamburger:hover & span { background-color: $c-bg; }
}

// ── Mobile nav ────────────────────────────────────────────────
.nav-mobile {
  border-top: $border-heavy;
  background: $c-bg;
  padding-block: $sp-4;

  ul { list-style: none; padding: 0; }
}

.nav-mobile__link {
  display: block;
  padding: $sp-3 0;
  font-family: $f-sans;
  font-size: 0.75rem;
  font-weight: 600;
  letter-spacing: 0.1em;
  text-transform: uppercase;
  color: $c-fg;
  border-bottom: 1px solid $c-muted;
  text-decoration: none;
  transition: color 0.15s, padding-left 0.15s;

  &:hover {
    color: $c-accent;
    padding-left: $sp-2;
  }

  &--cta {
    color: $c-accent;
    font-weight: 700;
    border-bottom: none;
    margin-top: $sp-2;
  }
}
</style>
