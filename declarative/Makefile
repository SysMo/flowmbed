examples/%/gen_system:
	cargo +nightly run --example generate_system -- examples/$*.yaml examples/$*_gen.rs

examples/%/run_system:
	cargo run --example $*_gen

examples/%/gen_block:
	cargo +nightly run --example generate_block -- examples/$*.yaml examples/$*


examples/blink: build/pre examples/51_blink/gen_system examples/51_blink/run_system

examples/counting_trigger: build/pre examples/11_counting_trigger/gen_block


terminal/clear:
	clear

build/pre: terminal/clear


# examples/blink_template:
# 	cargo run --example 01_blink_system_template