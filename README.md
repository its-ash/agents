# Agent Studio

A Tauri desktop app for running prompt-template agents via OpenRouter and OpenAI APIs.

- **Backend:** Rust (Tauri v2) — agent CRUD, prompt templating, LLM calls
- **Frontend:** Vue 3 + TypeScript + Tailwind CSS v4
- **LLM providers:** OpenRouter (Claude, Copilot, GPT, etc.) and OpenAI

## Prerequisites

- [Node.js](https://nodejs.org/) (see [.nvmrc](.nvmrc)) and [pnpm](https://pnpm.io/)
- [Rust](https://www.rust-lang.org/tools/install) toolchain
- [Tauri prerequisites](https://tauri.app/start/prerequisites/) for your OS

## Getting started

```bash
pnpm install
make run
```

This starts the Vite dev server (port 1420) and launches the native window.

## Build a production bundle

```bash
make deploy
```

## Releasing

Pushing a tag matching `v*` (e.g. `v0.1.0`) triggers [.github/workflows/release.yml](.github/workflows/release.yml), which builds signed bundles for macOS (Apple Silicon + Intel), Linux, and Windows, and attaches them to a draft GitHub Release.

```bash
git tag v0.1.0
git push origin v0.1.0
```

Publish the draft release once the workflow finishes.

## Setup

1. Open the app — sample agents are seeded automatically.
2. Click **API Keys** (top-right gear on mobile, top-right link on desktop).
3. Paste your OpenRouter key (`sk-or-...`) and/or OpenAI key (`sk-...`).
4. Pick a default provider and optional default model.
5. Select an agent, fill the placeholder inputs, hit **Run**.

Keys and agents are stored locally at:
- macOS: `~/Library/Application Support/agent-studio/`

## How agents work

Each agent has a prompt template with `{{placeholder}}` tokens. The UI auto-generates one input field per unique placeholder. On **Run**, the Rust backend:

1. Renders the template by substituting placeholder values.
2. Sends a chat completion request to the configured provider.
3. Stores the run (output, model, token usage) and persists it to `agents.json`.

## Project layout

```
.
├── src/                       # Vue frontend
│   ├── main.ts
│   ├── api.ts                 # Tauri command bindings
│   ├── App.vue                # Root layout
│   └── components/
│       ├── Sidebar.vue        # Agent list + new-agent form
│       ├── Workspace.vue      # Inputs + run + response history
│       ├── AgentDialog.vue    # Create / edit agent
│       ├── ConfigModal.vue    # Edit / delete agent
│       ├── SettingsModal.vue  # API keys + provider config
│       └── HelpModal.vue
└── src-tauri/                 # Rust backend
    ├── Cargo.toml
    ├── tauri.conf.json
    └── src/
        ├── lib.rs             # Tauri builder + handler registration
        ├── commands.rs        # Tauri commands (list/create/run/...)
        ├── llm.rs             # OpenRouter / OpenAI HTTP client
        ├── models.rs          # Agent / Run / Settings + template helpers
        ├── models_seed.rs     # Default sample agents
        └── storage.rs         # JSON persistence to data dir
```

## Default models

- OpenRouter: `anthropic/claude-3.5-sonnet`
- OpenAI: `gpt-4o-mini`

Override via the default model field in Settings.

## Available commands

| Command | Description |
|---|---|
| `make run` | Start the dev server and native window |
| `make deploy` | Build a production bundle |
| `make commit` | Stage changes and commit with an AI-generated message |

## License

[MIT](LICENSE)
