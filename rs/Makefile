# read -e に必要
SHELL=/bin/bash

define CARGOBIN
[[bin]]
name = "$(CONTEST)-$(TARGET_BIN)"
path = "src/$(CONTEST)/$(TARGET_BIN).rs"
endef

export CARGOBIN

# 再生成を防ぐため、.template.rsへの依存は記載しない
src/%.rs:
	$(eval CONTEST := $(patsubst %/,%,$(dir $*)))
	$(eval TARGET_BIN := $(notdir $(basename $*)))
	grep -q "$(CONTEST)-$(TARGET_BIN)" ./Cargo.toml || echo $$'\n'"$$CARGOBIN" >> ./Cargo.toml
	mkdir -p $(dir $@)
	cp src/.template.rs $@

.PHONY: gen-impl
gen-impl:
	@for target in $(TARGET); do \
		if [[ $$target == *..* ]]; then \
			for t in $$(eval echo {$${target}}); do \
				$(MAKE) src/$(CONTEST)/$$t.rs; \
			done \
		else \
			$(MAKE) src/$(CONTEST)/$${target}.rs; \
		fi \
	done

.PHONY: gen
gen:
	$(eval CONTEST := $(shell read -e -p "Contest name: " -i "$(shell cat .last-contest 2> /dev/null)" CONTEST && echo "$${CONTEST}" | tee .last-contest))
	$(eval TARGET := $(shell read -e -p "Target name (space separated): " TARGET && echo "$${TARGET}"))
	$(MAKE) gen-impl CONTEST="$(CONTEST)" TARGET="$(TARGET)"

.PHONY: snippet
snippet:
	cargo snippet -t vscode > ../.vscode/atcoder-snippets.code-snippets
