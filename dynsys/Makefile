SETENV:=export RUST_BACKTRACE=1

terminal/clear:
	clear

build/pre: terminal/clear


examples/messages:
	cargo run --example messages

examples/01_blink: build/pre	
	$(SETENV) && cargo run --example 01_blink

# examples/system_02: build/pre
# 	$(SETENV) && cargo run --example system_02