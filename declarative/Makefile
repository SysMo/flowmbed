examples/blink/gen:
	cargo +nightly run --example generate -- examples/01_blink.yaml examples/01_blink_gen.rs

examples/blink/run:
	cargo run --example 01_blink_gen

examples/blink: build/pre examples/blink/gen examples/blink/run


terminal/clear:
	clear

build/pre: terminal/clear


# examples/blink_template:
# 	cargo run --example 01_blink_system_template