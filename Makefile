%-rs:
	mkdir -p $@/src
	cp template-rs/Cargo.toml.template $@/Cargo.toml
	cp template-rs/Makefile $@/Makefile
	sed -e 's/%TITLE%/$*/' $@/Cargo.toml -i
	$(MAKE) -C $@ src/a.rs src/b.rs src/c.rs src/d.rs

%-cpp:
	cp -r template-cpp $@
