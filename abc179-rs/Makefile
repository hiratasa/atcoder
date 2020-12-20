TITLE=$(shell basename $${PWD%-rs})

TARGET_BIN=a
TARGET_SRC=src/a.rs

define CARGOBIN
[[bin]]
name = "$(TITLE)-$(TARGET_BIN)"
path = "$(TARGET_SRC)"
endef

export CARGOBIN

.PHONY: add-to-cargo
add-to-cargo:
	# blank line
	echo "" >> ./Cargo.toml
	echo "$$CARGOBIN" >> ./Cargo.toml

# 再生成を防ぐため、.template.rsへの依存は記載しない
src/%.rs:
	cp ../template-rs/src/.template.rs $@
	$(MAKE) add-to-cargo TARGET_BIN=$* TARGET_SRC=$@

%: src/%.rs
	cargo build --bin $(TITLE)-$@
	ln -sf ../target/debug/$(TITLE)-$@ ./$@

.PHONY: update-cargo
update-cargo:
	cp -i ../template-rs/Cargo.toml.template ./Cargo.toml
	sed -e "s/%TITLE%/$(shell basename $(shell pwd) | sed -e 's/-rs$$//')/" ./Cargo.toml -i
	for f in $$(ls src); do $(MAKE) add-to-cargo TARGET_BIN=$${f%.rs} TARGET_SRC=src/$$f; done

.PHONY: update
update: update-cargo
	cp -i ../template-rs/Makefile ./
	grep "fn main() {}" -lr src/ | xargs -p -I{} cp ../template-rs/src/.template.rs ./{}

.PHONY: update-all
update-all: update-cargo
	cp -i ../template-rs/Makefile ./
	ls src/*.rs | xargs -p -I{} cp ../template-rs/src/.template.rs ./{}
