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


core_blocks: build/pre gen_blocks/blocks
	$(SETENV) && cd ../blocks && cargo build

_examples/%/gen: build/pre
	echo Generating target $*
	make gen_system/systems/examples/$*

_examples/%/build: build/pre
	echo Generating target $*
	make gen_system/systems/examples/$*
	echo Building target $*
	cd ../esp32 && . ~/export-esp.sh && make $*/build

_examples/%/flash: build/pre
	echo Generating target $*
	make gen_system/systems/examples/$*
	echo Building target $*
	cd ../esp32 && . ~/export-esp.sh && make $*/flash

SYSTEM_EXAMPLES = 54_gpio_joystick
define make-system-target	
examples/$1/gen: _examples/$1/gen
	echo Generated MCU files
examples/$1/build: _examples/$1/build
	echo Built MCU files
examples/$1/flash: _examples/$1/flash
	echo Flashed MCU firmware
endef

$(foreach element,$(SYSTEM_EXAMPLES),$(eval $(call make-system-target,$(element))))