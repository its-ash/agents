export const siteCategories = [
  {
    name: 'Development',
    slug: 'development',
    icon: '01',
    description: 'Code generation, debugging, review, architecture, and delivery systems.',
  },
  {
    name: 'Writing',
    slug: 'writing',
    icon: '02',
    description: 'Editorial workflows, structured copywriting, and long-form publishing prompts.',
  },
  {
    name: 'Research',
    slug: 'research',
    icon: '03',
    description: 'Evidence-led synthesis, source evaluation, and rigorous analytical work.',
  },
  {
    name: 'Design',
    slug: 'design',
    icon: '04',
    description: 'UI critique, systems thinking, accessibility, and interface specification.',
  },
  {
    name: 'Data',
    slug: 'data',
    icon: '05',
    description: 'Data wrangling, SQL workflows, dashboards, and operational analysis.',
  },
  {
    name: 'Support',
    slug: 'support',
    icon: '06',
    description: 'Triage, support operations, escalation flows, and service communication.',
  },
] as const

export function useSite() {
  const runtimeConfig = useRuntimeConfig()

  const siteUrl = String(runtimeConfig.public.siteUrl || '').replace(/\/$/, '')
  const repoUrl = String(runtimeConfig.public.repoUrl || '').trim()

  return {
    name: 'The Agent Dispatch',
    shortName: 'Agent Dispatch',
    strapline: 'Systems for working with AI agents',
    description: 'A curated library of production-ready prompts for AI agents. Structured, reusable, and built for real workflows.',
    siteUrl,
    repoUrl,
    categories: siteCategories,
  }
}

export function useAbsoluteUrl(path = '/') {
  const { siteUrl } = useSite()

  if (!siteUrl) {
    return undefined
  }

  return new URL(path, `${siteUrl}/`).toString()
}