import { defineContentConfig, defineCollection, z } from '@nuxt/content'

export default defineContentConfig({
  collections: {
    prompts: defineCollection({
      type: 'page',
      source: 'prompts/**/*.md',
      schema: z.object({
        title: z.string(),
        description: z.string(),
        category: z.string(),
        tags: z.array(z.string()).default([]),
        author: z.string().default('Community'),
        date: z.string(),
        featured: z.boolean().optional().default(false),
        icon: z.string().optional().default('terminal'),
        model: z.string().optional(),
        rawbody: z.string() 
      }),
    }),
  },
})
