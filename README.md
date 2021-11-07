# sensor

## Hardware

This assumes you're using a DHT11 temperature and humidity sensor.

Refer to [this page](https://pinout.xyz/) for Raspberry Pi pinouts. Make sure you connect the data line to a pin labeled "GPIO" on that page, and pass that pin to the binary.

## Building

(Arch Linux x86_64 cross compilation instructions)

Required packages:
- `arm-linux-gnueabihf-gcc` (AUR) (need to edit PKGBUILD files for `-stage1`, `-stage2`, and the final package

```
cross build --target armv7-unknown-linux-gnueabihf --release
```

## Deployment

Ensure the running user is in the `gpio` group, otherwise you'll see errors like this: `Error: Permission denied: /dev/gpiomem`. Then,

## License

[MIT](LICENSE.md)
