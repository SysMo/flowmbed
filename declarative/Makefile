SETENV:=export RUST_BACKTRACE=1

examples/%/gen_system:
	cargo +nightly run --example generate_system -- examples/$*.yaml examples/$*_gen.rs

examples/%/run_system:
	cargo run --example $*_gen

examples/%/gen_block:
	$(eval BLOCK_FOLDER:=../$*/src)
	$(SETENV) && cargo +nightly run --example generate_block -- $(BLOCK_FOLDER)

examples/compile_blocks: build/pre
	cargo build --example 11_blocks

examples/blink: build/pre examples/51_blink/gen_system examples/51_blink/run_system

examples/core_blocks: build/pre examples/blocks/gen_block
	$(SETENV) && cd ../blocks && cargo run --example 01_test_gen_block


terminal/clear:
	clear

build/pre: terminal/clear


# examples/blink_template:
# 	cargo run --example 01_blink_system_template