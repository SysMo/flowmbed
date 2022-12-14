
### Initial installation

```
cargo install espup
espup install
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