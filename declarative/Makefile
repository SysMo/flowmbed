examples/run/blink: examples/gen/blink
	cargo run --example 01_blink_gen

examples/gen/blink: build/pre
	cargo +nightly run --example generate -- examples/01_blink.yaml examples/01_blink_gen.rs



terminal/clear:
	clear

build/pre: terminal/clear


# examples/blink_template:
# 	cargo run --example 01_blink_system_template