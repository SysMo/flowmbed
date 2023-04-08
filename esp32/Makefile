include credentials.vars
# FLASH_SPEED= 460080
# FLASH_SPEED = 115200
FLASH_SPEED = 920160
FLASH_FLAGS= --partition-table partitions.csv --baud ${FLASH_SPEED} --monitor

terminal/clear:
	clear

build/pre: terminal/clear

code:
	. ~/export-esp.sh && code .

check_credentials:
	echo ${RUST_ESP32_STD_DEMO_WIFI_PASS}

clean:
	cargo clean

dep-tree:
	. ~/export-esp.sh && cargo tree

deepclean:
	rm -rf target
	rm -rf .embuild

examples/%/build: build/pre
	. ~/export-esp.sh && cargo espflash save-image --release --example $*_gen --chip esp32 program.bin

examples/%/flash: build/pre
	. ~/export-esp.sh && cargo espflash flash --speed=460080 --release --example $*_gen --partition-table partitions.csv --monitor $(SERIAL)

bin/%/build: build/pre
	. ~/export-esp.sh && cargo espflash save-image --bin $* --release --chip esp32 program.bin

bin/%/flash: build/pre
	. ~/export-esp.sh && cargo espflash flash --bin $* --release $(FLASH_FLAGS)


monitor:
	cargo espflash monitor

STANDALONE_EXAMPLES= 10_gpio 11_i2c 12_one_wire 13_pwm_motor_control 14_mqtt 15_mqtt_eth 21_bioreactor_model_esp
define make-standalone-target
$1/build: bin/$1/build
$1/flash: bin/$1/flash
endef

$(foreach element,$(STANDALONE_EXAMPLES),$(eval $(call make-standalone-target,$(element))))

SYSTEM_EXAMPLES= 54_gpio_joystick
define make-system-target	
$1/build: examples/$1/build
$1/flash: examples/$1/flash
endef

$(foreach element,$(SYSTEM_EXAMPLES),$(eval $(call make-system-target,$(element))))