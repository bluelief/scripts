SHELL=/bin/bash
CARGO=cargo


.PHONY: all
all: prebuild build


.PHONY: build
build:
	@$(CARGO) build --target=x86_64-unknown-linux-musl --release


.PHONY: prebuild
prebuild:
	@$(CARGO) +stable fmt
	@$(CARGO) clippy


.PHONY: clean
clean:
	@$(CARGO) clean


.PHONY: test
test:
	@$(CARGO) test


.PHONY: version
version:
	@$(CARGO) --version


.PHONY: ldd
ldd:
	@basename `pwd` | xargs -I@ find ./target -type f -name @ -exec ldd {} \;


.PHONY: run
run:
	@basename `pwd` | xargs -I@ find ./target -type f -name @ -exec bash -c {} \;
