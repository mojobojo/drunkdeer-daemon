# Drunkdeer HID Protocol Reverse Engineering
 
 ---

## Notes
- The offsets in the packets are ignoring the first byte so the offsets are shifted by 1. This was due to how I was reverse engineering the web driver.

---

## Keyboard Types

These are all of the current keyboard models, their vendor ID, product ID, and 3 bytes used for some kind of version.

| Name      | Vendor ID | Product ID | Byte5 | Byte6 | Byte7 |
|-----------|-----------|------------|-------|-------|-------|
| G75       | 13613     | 9094       | 11    | 4     | 5     |
| G75JP     | 13613     | 9105       | 11    | 4     | 7     |
| A75       | 13613     | 9091       | 11    | 4     | 1     |
| A75Pro    | 13613     | 9091       | 11    | 4     | 3     |
| A75DE     | 13613     | 9091       | 11    | 4     | 2     |
| A75FR     | 13613     | 9091       | 11    | 4     | 2     |
| A75UK     | 13613     | 9091       | 11    | 4     | 2     |
| A75Ultra  | 13613     | 9091       | 11    | 4     | 4     |
| A75Master | 13613     | 9091       | 11    | 5     | 4     |
| G65       | 13613     | 9090       | 11    | 2     | 1     |
| G65Lite   | 13613     | 9090       | 11    | 2     | 5     |
| G60       | 13613     | 9092       | 11    | 3     | 1     |


## Command Data
The keyboard handles HID in 64 byte packets. When there is a succesful HID command sent it usually responds back with the exact data you sent unless in specific scenarios where you requested some kind of data. The first byte is 0x04 which is a report ID, as far as I can tell this is standard for HID protocols. The next byte is a command code, and the bytes after that are the data related to the command requested.

## Command List

Below are the main HID commands supported by the keyboard interface. Each command is identified by a command code (second byte in the packet) and serves a specific function.


| Command Code | Name/Function                | Description |
|--------------|-----------------------------|-------------|
| 160 (0xA0)   | Identity Data               | Returns device identity and status information. |
| 181 (0xB5)   | Common Settings             | Configure turbo, rapid trigger, dual trigger, last win, etc. |
| 170 (0xAA)   | Clear RTP / Reset Keyboard  | Clears RTP settings or resets the keyboard. |
| 167 (0xA7)   | RTP Authority               | Authority settings for RTP. |
| 168 (0xA8)   | RTP Authority Download      | Download authority data for RTP. |
| 174 (0xAE)   | LED Mode / Turbo LED Mode   | Set LED lighting mode, speed, brightness, color. |
| 253 (0xFD)   | Downstroke/Upstroke/Tracking| Configure or query downstroke, upstroke, tracking. |
| 252 (0xFC)   | Create LW/RDT Data          | Create Last Win or Rapid Double Tap data. |
| 182 (0xB6)   | Action Point                | Set or query action point data. |

---

### Common Data (Command 181 / 0xB5)

| Offset | Description |
|--------|-------------|
| 2      | 0x1E Unknown, seems to always be 0x1E |
| 3      | 0x01 Unknown, seems to always be 0x01 |
| 6      | 0x01 Unknown, seems to always be 0x01 |
| 7      | Turbo enabled (1 byte boolean) |
| 8      | Rapid trigger enabled (1 byte boolean) |
| 10     | Other Feature Flags (maybe bitflags): 0 = none, 1 = Last Win, 2 = Dual Trigger, 3 = Both |
| 11     | RTMatch value (unknown purpose) |

---

### Identity Data (Command 160 / 0xA0)

When the keyboard responds to the identity data command (code 160 / 0xA0), the returned packet contains device information at specific offsets:

| Offset | Description |
|--------|-------------|
| 1      | status byte 1? |
| 2      | status byte 2? |
| 3      | Unused |
| 4      | Version A |    
| 5      | Version B |
| 6      | Version C |
| 7      | Unknown |
| 8      | Unknown |
| 15     | Turbo Enabled (1 byte boolean) |
| 16     | Rapid Trigger Enabled (1 byte boolean) |
| 18     | Release Dual Trigger Enabled (1 byte boolean) |
| 19     | Last Win Enabled (1 byte boolean) |
| 30     | RT Match Enabled (1 byte boolean) |

# Notes
if status byte 1 is 2 and status byte 2 is 0 then keyboard link is not used. If status bytes 1 is not 2 and status byte 2 is 4 then keyboard link is used. No idea what keyboard link is referring to yet.

Keyboard link seems to be a variable that turns to true when there is data sent then set to false after a packet is returned and processed. So possibly something for internally checking if there are pending bytes to recieve. 

---

### Clear RTP / Reset Keyboard (Command 170 / 0xAA)

Currently not reverse engineered.

---

### RTP Authority (Command 167 / 0xA7)

Currently not reverse engineered.

---

### RTP Authority Download (Command 168 / 0xA8school)

Currently not reverse engineered.

---

### LED Mode / Turbo LED Mode (Command 174 / 0xAE)

Sets LED lighting mode, speed, brightness, and color.

| Offset | Description |
|--------|-------------|
| 1      | LED mode flag (1) |
| 2      | Reserved (0 or turbo value) |
| 3      | Device value (varies) |
| 4      | Lighting mode (see Lighting Modes table) |
| 5      | Speed |
| 6      | Brightness |
| 7      | Color (see Color Modes table) |

### Lighting Modes
Lighting modes are set using the LED Mode command (code 174). Each mode has a numeric value:

| Value | Mode Name           |
|-------|---------------------|
| 0     | Off                 |
| 1     | Rainbow Marquee     |
| 2     | Wave TT Spectrum    |
| 3     | Surf Right          |
| 4     | Breath              |
| 5     | Center Surfing      |
| 6     | Spectrum            |
| 7     | Ripple              |
| 8     | Always Light        |
| 9     | Light By Press      |
| 10    | Serpent             |
| 11    | Colorful Fountain   |
| 12    | Laser Key           |
| 13    | Glowing Fan         |
| 14    | Surfing Cross       |
| 15    | Heart               |
| 16    | Traffic             |
| 17    | GStrike             |
| 18    | Raindrops           |
| 19    | Static              |
| 20    | Demo/Test Mode?     |

### Hardcoded Color Modes

There are hardcoded color modes that are set using the LED Mode command (code 174).

| Value | Color Name |
|-------|------------|
| 0     | Rainbow    |
| 1     | Red        |
| 2     | Green      |
| 3     | Blue       |
| 4     | Yellow     |
| 5     | Magenta    |
| 6     | Cyan       |
| 7     | White      |

---

### Downstroke/Upstroke/Tracking (Command 253 / 0xFD)

Currently not reverse engineered.

---

### Create LW/RDT Data (Command 252 / 0xFC)

Currently not reverse engineered.

---

### Action Point (Command 182 / 0xB6)

Currently not reverse engineered.

---

### Keyboard Access Permissions (Linux)

Originally I started this project because the web driver was not working. Support was less than helpful and the Drunkdeer discord was dead and filled with barely moderated spam bots.

At the time I had no idea about how raw HID access on linux needed special permissions set up so this section is here to help anyone that needs it.

Find the USB Device
```
mojobojo@mojobojo-pc:~$ lsusb
Bus 001 Device 001: ID 1d6b:0002 Linux Foundation 2.0 root hub
Bus 001 Device 002: ID 0b05:19af ASUSTek Computer, Inc. AURA LED Controller
Bus 001 Device 003: ID b58e:9e84 Blue Microphones Yeti Stereo Microphone
Bus 001 Device 004: ID 28bd:090d XP-Pen 15.6 inch PenDisplay
Bus 001 Device 005: ID 05e3:0608 Genesys Logic, Inc. Hub
Bus 001 Device 006: ID 05e3:0608 Genesys Logic, Inc. Hub
Bus 001 Device 007: ID 09da:fa10 A4Tech Co., Ltd. USB Device 
Bus 001 Device 008: ID 13d3:3548 IMC Networks Bluetooth Radio
Bus 001 Device 010: ID 046d:c539 Logitech, Inc. Lightspeed Receiver
Bus 001 Device 012: ID 056a:037a Wacom Co., Ltd CTL-472 [One by Wacom (S)]
Bus 001 Device 021: ID 046d:081b Logitech, Inc. Webcam C310
Bus 001 Device 022: ID 352d:2384 Drunkdeer Drunkdeer G60 ANSI
Bus 002 Device 001: ID 1d6b:0003 Linux Foundation 3.0 root hub
```
The vendor id and product id are the two hexadecimal numbers separated by the colon.

Make a new file in your device rules directory.
```
mojobojo@mojobojo-pc:~$ sudo nano /etc/udev/rules.d/50-drunkdeer.rules
```

The vendor id is on the left of the colon and the product id is on the right of the colon.
```
KERNEL=="hidraw*", ATTRS{idVendor}=="352d", ATTRS{idProduct}=="2384", MODE="0666"
```

Reload the rules
```
mojobojo@mojobojo-pc:~$ sudo udevadm control --reload-rules
mojobojo@mojobojo-pc:~$ sudo udevadm trigger
```

And it should be working. This also lets you run the daemon without sudo.

