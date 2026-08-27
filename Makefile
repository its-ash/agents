.PHONY: deploy run commit

deploy:
	pnpm tauri build

run:
	pnpm tauri dev

commit:
	git add -A
	git commit -m "$$(gh copilot suggest -t commit)"