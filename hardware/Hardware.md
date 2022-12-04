# Hardware

This document is for the hardware portion of the project.

## Hardware Design Log
1. Import ESP32-C3-Mini-1 symbol and footprint from [Espressif KiCad Library](https://github.com/espressif/kicad-libraries)
2. Place `ESP32-CE-Mini-1` module with decoupling/bulking caps
3. Create `enable` circuit for ESP32C3
4. Create programming circuit for ESP32C3
    - Only need `USB D+/D-` pins `18/19` to program the ESP32-C3 because it has a `integrated USB Serial/JTAG Controller`, see [doc](https://docs.espressif.com/projects/esp-idf/en/latest/esp32c3/api-guides/usb-serial-jtag-console.html)
    - [USB C plug vs recepticle](https://www.arrow.com/en/research-and-events/articles/usb-technology-c-plug-and-receptacle-pinouts)
5. Add power circuits
    - `USB`




## TODO
- Page Documentation
- Circuit documenations
- spec specific passives


## Circuits
- ESP32-C3
    - Power bulk cap
    - Power decoupling cap
    - `Enable` circuit
        - `RC` circuit
    - Programming circuit


