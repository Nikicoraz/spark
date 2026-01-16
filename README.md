# Spark
Spark is a simple command line utility to send Wake On Lan packets to wake WOL compatible devices.

### Dependencies
Since it's using rust the only dependency is `cargo`

### Usage
Run the following inside the project folder

```cargo run --release```

and when prompted insert the MAC address of the device to wake

Otherwise, you can pass the MAC address as the only argument to the executable

```./target/release/spark <mac_address>```

## Troubleshooting
### Device won't wake
Make sure the device from which you are running the executable is inside the LAN on which the device to wake is on.

>This is needed because WOL works by sending a frame in broadcast, and often broadcast frames are blocked by routers.

**This also applies if using a vpn.**

Finally, make sure that your device supports Wake On Lan and that it's enabled in the BIOS.