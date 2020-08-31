# `esc-rs`

## target hardware

`stm32f302r8`

### debug

```
# openocd -f board/st_nucleo_f3.cfg -f interface/stlink-v2-1.cfg
openocd -f openocd.cfg
arm-none-eabi-gdb -x openocd.gdb target/thumbv7em-none-eabihf/debug/esc-rs
```


### ports

User LD2: (Arduino D13) PB13 ( PIN 34 )
User B1: PC13 ( PIN 2 )
USART: USART2 PA2 PA3
