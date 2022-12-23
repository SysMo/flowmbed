
### Initial installation

```
cargo install ldproxy
cargo install espup
espup install
cargo install cargo-espflash
```

### Serial port troubleshooting

Ensure that you are part of the `dialout` group:
```
sudo gpasswd --add ${USER} dialout
```


If no /dev/ttyUSB0 or /dev/ttyACM0 device is present when the ESP is connected, then drivers for the serial 
commmunication chip must be installed.



#### CH340
```
git clone https://github.com/juliagoda/CH341SER.git
cd CH341SER
make
sudo make load
```

Also in Ubuntu 22.04 another service prevents serial connection. If `sudo dmesg` shows:
```
input: BRLTTY 6.4 Linux Screen Driver Keyboard as /devices/virtual/input/input39
usb 1-5: usbfs: interface 0 claimed by ch341 while 'brltty' sets config #1
ch341-uart ttyUSB0: ch341-uart converter now disconnected from ttyUSB0
ch341 1-5:1.0: device disconnected
```


This should fix it:
```
for f in /usr/lib/udev/rules.d/*brltty*.rules; do
    sudo ln -s /dev/null "/etc/udev/rules.d/$(basename "$f")"
done
sudo udevadm control --reload-rules
```


### Building

```
. ~/export-esp.sh
make flash
```

- the baudrate is increased to 921600
- the stack sizes are increased:
  ```
  CONFIG_ESP_MAIN_TASK_STACK_SIZE=20000
  CONFIG_ESP_SYSTEM_EVENT_TASK_STACK_SIZE=4096
  ```
- the size of the flash partition is increased to accomodate the program 