# Compiler configuration
GENERAL_ARGS = --release

# Application configuration
define get_config_value
	$(shell sed -ne 's/^$(1).*"\(.*\)"/\1/p' Config.toml)
endef

all: build-linux build-windows

build-linux:
	cargo build $(GENERAL_ARGS)

build-windows:
	cargo build --target x86_64-pc-windows-gnu $(GENERAL_ARGS)