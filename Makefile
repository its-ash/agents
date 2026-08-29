.PHONY: deploy build move-to-release commit release push run

VERSION := $(shell grep '"version"' src-tauri/tauri.conf.json | head -1 | sed 's/.*"version"[[:space:]]*:[[:space:]]*"\([^"]*\)".*/\1/')
RELEASE_DIR := release
BUNDLE_DIR := src-tauri/target/release/bundle

# NOTE: push runs before release because `gh release create --target main`
# tags the remote main HEAD, so the commit must be pushed first.
deploy: build move-to-release commit push release

build:
	pnpm tauri build

move-to-release:
	rm -rf $(RELEASE_DIR)
	mkdir -p $(RELEASE_DIR)
	find $(BUNDLE_DIR) -type f \( \
		-name '*.dmg' -o -name '*.app' -o -name '*.deb' -o -name '*.rpm' \
		-o -name '*.msi' -o -name '*.exe' -o -name '*.AppImage' \
		-o -name '*.snap' -o -name '*.apk' -o -name '*.aab' \
	\) -exec cp {} $(RELEASE_DIR)/ \;

commit:
	git add -A
	git commit -m "$$(gh copilot -p 'Write a concise one-line git commit message (conventional commits style) summarizing this staged diff. Output ONLY the commit message text, nothing else.' --allow-tool 'shell(git diff --staged)' --silent)"

release:
	gh release create v$(VERSION) $(RELEASE_DIR)/* --title "v$(VERSION)" --generate-notes --target main

push:
	git push origin main

run:
	pnpm tauri dev