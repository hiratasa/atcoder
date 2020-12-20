TITLE=$(shell basename $${PWD%-rs})

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
	grep "fn main() {}" -lr src/ | xargs -p -I{} cp ../template-rs/{} ./{}

.PHONY: update-all
update-all: update-cargo
	cp -i ../template-rs/Makefile ./
	cp -i ../template-rs/src/*.rs ./src/

.PHONY: snippet
snippet:
	cargo snippet -t vscode > ../.vscode/atcoder-snippets.code-snippets
