all: build

.PHONY: build
build:
	$(shell [[ $EUID -eq 0 ]] && echo "build can not be run as root" && exit 1)
	@echo ":: Rebuilding in release mode..."
	@cargo build --release


.PHONY: build-debug
build-debug:
	$(shell [[ $EUID -eq 0 ]] && echo "build can not be run as root" && exit 1)
	@echo ":: Rebuilding in debug mode..."
	@cargo build

.PHONY: install-helpers
install-helpers:
	@echo ":: Installing ./bin..."
	@mkdir -p /usr/local/bin
	@cp -R bin/. /usr/local/bin
	@ls bin | xargs -I {} chmod 755 /usr/local/bin/{}
	@echo ":: Copying over xsession file..."
	@cp illef-wm.desktop /usr/share/xsessions/

.PHONY: install-illef-wm-release
install-illef-wm-release:
	@echo ":: Installing release build of penrose-from-scratch..."
	@mkdir -p /usr/local/bin
	@cp -f target/release/illef-wm /usr/local/bin
	@chmod 755 /usr/local/bin/illef-wm

.PHONY: install-illef-wm-debug
install-illef-wm-debug:
	@echo ":: Installing debug build of penrose-from-scratch..."
	@strip target/debug/illef-wm
	@mkdir -p /usr/local/bin
	@cp -f target/debug/illef-wm /usr/local/bin
	@chmod 755 /usr/local/bin/illef-wm

.PHONY: install
install: install-illef-wm-release install-helpers
	@echo ":: Done"
	
.PHONY: install-debug
install-debug: install-illef-wm-debug install-helpers
	@echo ":: Done"

.PHONY: uninstall
uninstall:
	@echo ":: Removing binaries..."
	@ls bin | xargs -I {} rm -f /usr/local/bin/{}
	@rm -f /usr/local/bin/penrose-from-scratch
	@echo ":: Done"

.PHONY: update-penrose
update-penrose:
	@echo "Updating to latest version of penrose from GitHub..."
	cargo update -p penrose
