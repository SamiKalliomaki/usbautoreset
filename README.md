# usbautoreset

If you are like me, your USB devices sometimes stop working and even unplugging
and plugging them back in doesn't help. This is a simple script that resets
the USB devices by unbinding and binding them.

It looks for errors like this:
```
xhci_hcd 0000:00:14.0: WARN: buffer overrun event for slot 10 ep 1 on endpoint
xhci_hcd 0000:00:14.0: WARN: buffer overrun event for slot 10 ep 1 on endpoint
```

When it sees a message like this, it does the equivalent of:
```bash
echo -n "0000:00:14.0" > /sys/bus/pci/drivers/xhci_hcd/unbind && echo -n "0000:00:14.0" > /sys/bus/pci/drivers/xhci_hcd/bind
```

## Installation
```bash
cargo install usbautoreset
```

## Usage
```bash
sudo ./usbautoreset
```
