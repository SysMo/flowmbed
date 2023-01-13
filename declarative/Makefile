SETENV:=export RUST_BACKTRACE=1


terminal/clear:
	clear

build/pre: terminal/clear

gen_blocks/%:
	$(eval BLOCK_FOLDER:=../$*/src)
	$(SETENV) && cargo +nightly run --bin generate_blocks -- $(BLOCK_FOLDER)

gen_system/%:
	$(eval FILE_PATH:=../$*)
	$(SETENV) && cargo +nightly run --bin generate_system -- $(FILE_PATH)


examples/core_blocks: build/pre gen_blocks/blocks
	$(SETENV) && cd ../blocks && cargo run --example 01_test_gen_block


examples/01_blink: build/pre gen_system/systems/examples/01_blink
	$(SETENV) && cd ../systems && cargo run --example 01_blink_gen






# examples/%/run_system:
# 	cargo run --example $*_gen

# examples/blink: build/pre examples/51_blink/gen_system examples/51_blink/run_system




# examples/blink_template:
# 	cargo run --example 01_blink_system_template