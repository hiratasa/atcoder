%-rs:
	cp -r template-rs $@
	mv $@/Cargo.toml.template $@/Cargo.toml
	sed -e 's/%TITLE%/$*/' $@/Cargo.toml -i
	$(MAKE) -C $@ src/a.rs src/b.rs src/c.rs src/d.rs src/e.rs src/f.rs

%:
	cp -r template $@
