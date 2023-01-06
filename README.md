# Remote Data Logger
Remote data logger project for use in engineering development and debugging of electronics in the field

## Feature List

### Hardware
- ESP32-C3 circuit
- programming circuit
- battery circuit
- uart connection to target
- ADC ic chip circuit
- uSD card

### Firmware
- ADC reading
- UART read/write
- Store values to uSD card
- Send values via WiFi

### Software
- Graph readings from ESP32-C3
- Talk with ESP32-C3 through Wi-Fi
- Talk with ESP32-C3 through UART


## Git
Update submodules manually:
```
git submodule update  --init --recursive
```

Configure repo to use custom `.githooks` path and make files executable:
```
git config --local core.hooksPath .githooks/
chmod -R +x .githooks
```