import { invoke } from '@tauri-apps/api/core'

export interface Run {
  id: string
  output: string
  created_at: number
  model?: string | null
  tokens?: number | null
}

export type Provider = 'openrouter' | 'openai' | 'copilot' | 'claude' | 'ollama'

export interface Agent {
  id: string
  name: string
  prompt: string
  model?: string | null
  provider: Provider
  runs: Run[]
}

export interface AppSettings {
  openrouter_key?: string | null
  openai_key?: string | null
  default_provider: Provider
  default_model?: string | null
}

export interface DetectedTools {
  copilot: boolean
  copilot_path?: string | null
  claude: boolean
  claude_path?: string | null
  ollama: boolean
  ollama_path?: string | null
  ollama_models: string[]
}

export interface CliTestResult {
  ok: boolean
  message: string
  command: string
  found: boolean
}

export const listAgents = () => invoke<Agent[]>('list_agents')
export const createAgent = (
  name: string,
  prompt: string,
  provider?: Provider,
  model?: string,
) => invoke<Agent>('create_agent', { name, prompt, provider, model })
export const updateAgent = (
  id: string,
  name: string,
  prompt: string,
  provider?: Provider,
  model?: string,
) => invoke<void>('update_agent', { id, name, prompt, provider, model })
export const deleteAgent = (id: string) => invoke<void>('delete_agent', { id })
export const getPlaceholders = (template: string) =>
  invoke<string[]>('get_placeholders', { template })
export const runAgent = (id: string, values: Record<string, string>) =>
  invoke<Run>('run_agent', { id, values })
export const getSettings = () => invoke<AppSettings>('get_settings')
export const saveSettings = (settings: AppSettings) =>
  invoke<void>('save_settings', { settings })
export const detectTools = () => invoke<DetectedTools>('detect_tools')
export const testTool = (provider: Provider) =>
  invoke<CliTestResult>('test_tool', { provider })
export const exportAgents = () => invoke<string | null>('export_agents')
export const importAgentsFromFile = () => invoke<number>('import_agents_from_file')
export const importAgentsFromRepo = (repoUrl: string) =>
  invoke<number>('import_agents_from_repo', { repoUrl })
