SETENV:=export RUST_BACKTRACE=1

examples/%/gen_system:
	cargo +nightly run --example generate_system -- examples/$*.yaml examples/$*_gen.rs

examples/%/run_system:
	cargo run --example $*_gen

BLOCK_FOLDER:=../blocks/src

examples/%/gen_block:
	$(SETENV) && cargo +nightly run --example generate_block -- $(BLOCK_FOLDER)

examples/compile_blocks: build/pre
	cargo build --example 11_blocks

examples/blink: build/pre examples/51_blink/gen_system examples/51_blink/run_system

examples/discrete_blocks: build/pre examples/discrete/gen_block
	cd ../blocks && cargo build


terminal/clear:
	clear

build/pre: terminal/clear


# examples/blink_template:
# 	cargo run --example 01_blink_system_template