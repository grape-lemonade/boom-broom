# Compiler configuration

# Application configuration
define get_config_value
	$(shell sed -ne 's/^$(1).*"\(.*\)"/\1/p' Config.toml)
endef

all: linux windows

debug:
	make linux
	./target/x86_64-unknown-linux-gnu/debug/boom-broom

release-linux:
	cargo build --target x86_64-unknown-linux-gnu --release

linux:
	cargo build --target x86_64-unknown-linux-gnu

release-windows:
	cargo build --target x86_64-pc-windows-gnu --release

windows:
	cross build --target x86_64-pc-windows-gnu
	