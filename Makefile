prefix ?= /usr/local

export DESKTOP_BIN=pop-shell-shortcuts
BINARY_SRC=target/$(TARGET)/$(DESKTOP_BIN)
BINARY_DST=$(DESTDIR)$(prefix)/bin/$(DESKTOP_BIN)

TARGET=debug
DEBUG ?= 0
ifeq ($(DEBUG),0)
	TARGET = release
	ARGS = --release
endif

VENDORED ?= 0
ifeq ($(VENDORED),1)
	ARGS += "--frozen"
endif

# Files to watch for modifications
SRC=Cargo.toml Cargo.lock $(shell find src -type f -wholename 'src/*.rs')

.PHONY: all clean distclean install uninstall

all: $(BINARY_SRC) $(DESKTOP)

clean:
	cargo clean

distclean:
	rm -rf .cargo vendor vendor.tar

vendor:
	mkdir -p .cargo
	cargo vendor | head -n -1 > .cargo/config
	echo 'directory = "vendor"' >> .cargo/config
	tar pcf vendor.tar vendor
	rm -rf vendor

run: all
	$(BINARY_SRC)

install:
	install -Dm755 $(BINARY_SRC) $(BINARY_DST)

uninstall:
	rm $(BINARY_DST)

$(BINARY_SRC): $(SRC)
ifeq ($(VENDORED),1)
	tar pxf vendor.tar.xz
endif
	cargo build $(ARGS)
