prefix ?= /usr/local

# Files to watch for modifications
SRC = Cargo.toml Cargo.lock $(shell find src -type f -wholename 'src/*.rs')

.PHONY: all clean distclean install uninstall

BIN=pop-shell-shortcuts

TARGET=debug
DEBUG ?= 0
ifeq ($(DEBUG),0)
	ARGS += "--release"
	TARGET = release
endif

BINARY_SRC=target/$(TARGET)/$(BIN)
BINARY_DST=$(DESTDIR)$(prefix)/bin/$(BIN)

VENDORED ?= 0
ifeq ($(VENDORED),1)
	ARGS += "--frozen"
endif


all: $(BINARY_SRC)

clean:
	cargo clean

distclean:
	rm -rf .cargo vendor vendor.tar.xz

run: $(BINARY_SRC)
	$(BINARY_SRC)

install: $(BINARY_SRC)
	install -Dm755 $(BINARY_SRC) $(BINARY_DST)

uninstall:
	rm -f $(BINARY_DST)

update:
	cargo update

vendor:
	mkdir -p .cargo
	cargo vendor | head -n -1 > .cargo/config
	echo 'directory = "vendor"' >> .cargo/config
	tar pcfJ vendor.tar.xz vendor
	rm -rf vendor

$(BINARY_SRC): $(SRC)
ifeq ($(VENDORED),1)
	tar pxf vendor.tar.xz
endif
	cargo build $(ARGS)
