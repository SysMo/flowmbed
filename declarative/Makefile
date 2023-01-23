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


# examples/01_blink: build/pre gen_system/systems/examples/01_blink
# 	$(SETENV) && cd ../systems && cargo run --example 01_blink_gen

# examples/02_blink_pwm: build/pre gen_system/systems/examples/02_blink_pwm
# 	$(SETENV) && cd ../systems && cargo run --example 02_blink_pwm_gen

# examples/03_blink_pwm: build/pre gen_system/systems/examples/03_blink_pwm
# 	$(SETENV) && cd ../systems && cargo run --example 03_blink_pwm_gen

# examples/03_blink_pwm: build/pre gen_system/systems/examples/03_blink_pwm
# 	$(SETENV) && cd ../systems && cargo run --example 03_blink_pwm_gen

examples/build/%: build/pre
	echo Generating target $*
	make gen_system/systems/examples/$*
	echo Building target $*
	cd ../esp32 && . ~/export-esp.sh && make $*_gen/build

# gen_system/systems/examples/$*
examples/flash/%: build/pre
	echo Generating target $*
	make gen_system/systems/examples/$*
	echo Building target $*
	cd ../esp32 && . ~/export-esp.sh && make $*_gen/flash


examples/54_temperature/build: examples/build/54_temperature

examples/54_temperature/flash: examples/flash/54_temperature


# examples/%/run_system:
# 	cargo run --example $*_gen

# examples/blink: build/pre examples/51_blink/gen_system examples/51_blink/run_system




# examples/blink_template:
# 	cargo run --example 01_blink_system_template