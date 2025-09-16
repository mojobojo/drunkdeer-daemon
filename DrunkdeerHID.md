# Drunkdeer HID Protocol Reverse Engineering

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
| 2      | 0x1E |
| 3      | 0x01 |
| 6      | 0x01 |
| 7      | Turbo enabled (1 = on) |
| 8      | Rapid trigger enabled (1 = on) |
| 10     | Dual/Last Win: 0 = none, 1 = last win, 2 = dual trigger, 3 = both |
| 11     | RTMatch value (unknown purpose) |

---

### Identity Data (Command 160 / 0xA0)

When the keyboard responds to the identity data command (code 160 / 0xA0), the returned packet contains device information at specific offsets:

| Offset | Description |
|--------|-------------|
| 2      | Device link status (used in logic) |
| 3      | Unused in this context |
| 4      | Device type or model (used in callback) |
| 5      | Device type or model (used in callback) |
| 6      | Device type or model (used in callback) |
| 7      | Firmware version (used in callback and string) |
| 8      | Firmware version (used in callback and string) |
| 15     | Turbo setting value |
| 16     | Rapid trigger value |
| 18     | Rapid double-tap value |
| 19     | Last win value |
| 30     | RT match value |

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
