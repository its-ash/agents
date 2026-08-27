.PHONY: deploy run commit

deploy: commit
	git push origin main
	pnpm tauri build

run:
	pnpm tauri dev

commit:
	git add -A
	git commit -m "$$(gh copilot -p 'Write a concise one-line git commit message (conventional commits style) summarizing this staged diff. Output ONLY the commit message text, nothing else.' --allow-tool 'shell(git diff --staged)' --silent)"