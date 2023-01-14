# SERIAL=/dev/ttyACM0
# SERIAL=/dev/ttyUSB0
# include credentials.vars

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

# build:
# 	cargo espflash save-image --target xtensa-esp32-espidf ESP32 program.bin

# flash:
# 	cargo espflash --speed=921600 --partition-table partitions.csv --monitor $(SERIAL)

example/build/%: build/pre
	cargo espflash save-image --release --example $* ESP32 program.bin

example/flash/%: build/pre
	cargo espflash --release --example $* --speed=921600 --partition-table partitions.csv --monitor $(SERIAL)

monitor:
	cargo espflash serial-monitor ${SERIAL}

01_blink_gen/build:  example/build/01_blink_gen
01_blink_gen/flash:  example/flash/01_blink_gen

02_blink_pwm_gen/build:  example/build/02_blink_pwm_gen
02_blink_pwm_gen/flash:  example/flash/02_blink_pwm_gen