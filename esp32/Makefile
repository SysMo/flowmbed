# include credentials.vars
BAUD_RATE = 460800

terminal/clear:
	clear

build/pre: terminal/clear

check_credentials:
	echo ${RUST_ESP32_STD_DEMO_WIFI_PASS}

clean:
	cargo clean

deepclean:
	rm -rf target
	rm -rf .embuild

examples/%/build: build/pre
	. ~/export-esp.sh && cargo espflash save-image --release --example $*_gen ESP32 program.bin

examples/%/flash: build/pre
	. ~/export-esp.sh && cargo espflash --speed=460080 --release --example $*_gen --partition-table partitions.csv --monitor $(SERIAL)

bin/%/build: build/pre
	. ~/export-esp.sh && cargo espflash save-image --release --example $* ESP32 program.bin

bin/%/flash: build/pre
	. ~/export-esp.sh && cargo espflash --release --example $* --partition-table partitions.csv --monitor $(SERIAL)


monitor:
	cargo espflash serial-monitor ${SERIAL}

SYSTEM_EXAMPLES = 54_gpio_joystick
define make-system-target	
$1/build: examples/$1/build
$1/flash: examples/$1/flash
endef

$(foreach element,$(SYSTEM_EXAMPLES),$(eval $(call make-system-target,$(element))))