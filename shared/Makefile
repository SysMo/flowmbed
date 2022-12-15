SETENV:=export RUST_BACKTRACE=1

terminal/clear:
	clear

build/pre: terminal/clear


examples/messages:
	cargo run --example messages

examples/system_01: build/pre	
	cargo run --example system_01

examples/system_02: build/pre
	$(SETENV) && cargo run --example system_02