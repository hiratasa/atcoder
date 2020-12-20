TITLE=$(shell basename $${PWD%-rs})

define CARGOBIN
[[bin]]
name = "$(TITLE)-$*"
path = "$@"
endef

export CARGOBIN

# 再生成を防ぐため、.template.rsへの依存は記載しない
src/%.rs:
	cp ../template-rs/src/.template.rs $@
	# blank line
	echo "" >> ./Cargo.toml
	echo "$$CARGOBIN" >> ./Cargo.toml

%: src/%.rs
	cargo build --bin $(TITLE)-$@
	ln -sf ../target/debug/$(TITLE)-$@ ./$@

.PHONY: update-cargo
update-cargo:
	cp -i ../template-rs/Cargo.toml.template ./Cargo.toml
	sed -e "s/%TITLE%/$(shell basename $(shell pwd) | sed -e 's/-rs$$//')/" ./Cargo.toml -i

.PHONY: update
update: update-cargo
	cp -i ../template-rs/Makefile ./
	grep "fn main() {}" -lr src/ | xargs -p -I{} cp ../template-rs/src/.template.rs ./{}

.PHONY: update-all
update-all: update-cargo
	cp -i ../template-rs/Makefile ./
	ls src/*.rs | xargs -p -I{} cp ../template-rs/src/.template.rs ./{}
