//mod data_packet;
mod key_id;
mod lightmode;

use hidapi::HidApi;
use hidapi::HidDevice;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use crate::lightmode::light_color;

struct KeyboardType {
    name: &'static str,
    vendor_id: u16,
    product_id: u16,
    byte5: u8,
    byte6: u8,
    byte7: u8,
}

const KEYBOARD_TYPES: &[KeyboardType] = &[
    KeyboardType {
        name: "G75",
        vendor_id: 13613,
        product_id: 9094,
        byte5: 11,
        byte6: 4,
        byte7: 5,
    },
    KeyboardType {
        name: "G75JP",
        vendor_id: 13613,
        product_id: 9105,
        byte5: 11,
        byte6: 4,
        byte7: 7,
    },
    KeyboardType {
        name: "A75",
        vendor_id: 13613,
        product_id: 9091,
        byte5: 11,
        byte6: 4,
        byte7: 1,
    },
    KeyboardType {
        name: "A75Pro",
        vendor_id: 13613,
        product_id: 9091,
        byte5: 11,
        byte6: 4,
        byte7: 3,
    },
    KeyboardType {
        name: "A75DE",
        vendor_id: 13613,
        product_id: 9091,
        byte5: 11,
        byte6: 4,
        byte7: 2,
    },
    KeyboardType {
        name: "A75FR",
        vendor_id: 13613,
        product_id: 9091,
        byte5: 11,
        byte6: 4,
        byte7: 2,
    },
    KeyboardType {
        name: "A75UK",
        vendor_id: 13613,
        product_id: 9091,
        byte5: 11,
        byte6: 4,
        byte7: 2,
    },
    KeyboardType {
        name: "A75Ultra",
        vendor_id: 13613,
        product_id: 9091,
        byte5: 11,
        byte6: 4,
        byte7: 4,
    },
    KeyboardType {
        name: "A75Master",
        vendor_id: 13613,
        product_id: 9091,
        byte5: 11,
        byte6: 5,
        byte7: 4,
    },
    KeyboardType {
        name: "G65",
        vendor_id: 13613,
        product_id: 9090,
        byte5: 11,
        byte6: 2,
        byte7: 1,
    },
    KeyboardType {
        name: "G65Lite",
        vendor_id: 13613,
        product_id: 9090,
        byte5: 11,
        byte6: 2,
        byte7: 5,
    },
    KeyboardType {
        name: "G60",
        vendor_id: 13613,
        product_id: 9092,
        byte5: 11,
        byte6: 3,
        byte7: 1,
    },
];

const VENDOR_ID: u16 = 13613;
// NOTE: Drunkeer has a few different HID interfaces, the config
// related one is located on this usage page.
const USAGE_PAGE: u16 = 0xFF00;
const _MANUFACTURER: &str = "Drunkdeer";
const REPORT_ID: u8 = 4;

fn find_and_open_device(api: &HidApi) -> Option<HidDevice> {
    for device in api.device_list() {
        if device.vendor_id() == VENDOR_ID && device.usage_page() == USAGE_PAGE {
            for keyboard in KEYBOARD_TYPES {
                if device.vendor_id() == keyboard.vendor_id
                    && device.product_id() == keyboard.product_id
                {
                    println!("Found Keyboard: {}", keyboard.name);
                    return device.open_device(&api).ok();
                }
            }
        }
    }

    return None;
}

#[derive(Debug)]
struct DataPacket {
    report_id: u8,
    command: u8,
    data: [u8; 62],
}

impl DataPacket {
    fn from_bytes(bytes: [u8; 64]) -> Self {
        let mut data = [0u8; 62];

        assert_eq!(bytes[0], REPORT_ID);

        data.copy_from_slice(&bytes[2..]);
        return Self {
            report_id: bytes[0],
            command: bytes[1],
            data,
        };
    }

    fn to_bytes(&self) -> [u8; 64] {
        let mut data = [0u8; 64];

        assert_eq!(self.report_id, REPORT_ID);

        data[0] = self.report_id;
        data[1] = self.command;
        data[2..].copy_from_slice(&self.data);

        return data;
    }

    fn heartbeat() -> Self {
        let mut packet = Self {
            report_id: REPORT_ID,
            command: 160,
            data: [0u8; 62],
        };
        packet.data[0] = 2;

        return packet;
    }

    fn common(turbo: bool, rapid_trigger: bool, dual_trigger: bool, last_win: bool) -> Self {
        let mut packet = Self {
            report_id: REPORT_ID,
            command: 181,
            data: [0u8; 62],
        };

        packet.data[1] = 0x1E;
        packet.data[2] = 0x01;
        packet.data[5] = 0x01;

        if turbo {
            packet.data[6] = 1;
        }

        if rapid_trigger {
            packet.data[7] = 1;
        }

        if dual_trigger && last_win {
            packet.data[9] = 3;
        } else if dual_trigger {
            packet.data[9] = 2;
        } else if last_win {
            packet.data[9] = 1;
        }

        packet.data[10] = 0; // NOTE: this is whats known as the RTMatch value. I am not sure where it comes from yet.

        return packet;
    }

    fn led_mode(b: u8, mode: u8, speed: u8, brightness: u8, color: u8) -> Self {
        let mut packet = Self {
            report_id: REPORT_ID,
            command: 174,
            data: [0u8; 62],
        };

        packet.data[0] = 1;
        packet.data[1] = 0;
        packet.data[2] = b;
        packet.data[3] = mode;
        packet.data[4] = speed;
        packet.data[5] = brightness;
        packet.data[6] = color;

        return packet;
    }
}

fn write_data_and_wait_for_response(device: &HidDevice, data: [u8; 64]) -> [u8; 64] {
    match device.write(&data) {
        Ok(amount_written) => {
            assert_eq!(amount_written, 64);
        }
        Err(error) => {
            panic!("write to keyboard failed: {:?}", error);
        }
    }

    let mut buffer = [0u8; 64];

    match device.read_timeout(&mut buffer, 1000) {
        Ok(amount_read) => {
            assert_eq!(amount_read, 64);
        }
        Err(error) => {
            panic!("read from device failed: {:?}", error);
        }
    }

    return buffer;
}

fn write_packet_and_wait_for_response(device: &HidDevice, packet: DataPacket) -> DataPacket {
    let data = write_data_and_wait_for_response(&device, packet.to_bytes());
    return DataPacket::from_bytes(data);
}

fn heartbeat(device: &HidDevice) {
    let packet = write_packet_and_wait_for_response(&device, DataPacket::heartbeat());
    println!("Heartbeat: {:?}", packet);
}

fn color_all_keys(device: &HidDevice, color_r: u8, color_g: u8, color_b: u8, brightness: u8) {
    let mut packet = [0u8; 64];

    let data: [u8; 62] = [
        0x04,
        0xAE,
        0x01,
        0x00,
        0x00,
        0x13,
        0x06,
        brightness,
        0xFF,
        key_id::key_id::ESCAPE,
        color_r,
        color_g,
        color_b,
        key_id::key_id::KEY_1,
        color_r,
        color_g,
        color_b,
        key_id::key_id::KEY_2,
        color_r,
        color_g,
        color_b,
        key_id::key_id::KEY_3,
        color_r,
        color_g,
        color_b,
        key_id::key_id::KEY_4,
        color_r,
        color_g,
        color_b,
        key_id::key_id::KEY_5,
        color_r,
        color_g,
        color_b,
        key_id::key_id::KEY_6,
        color_r,
        color_g,
        color_b,
        key_id::key_id::KEY_7,
        color_r,
        color_g,
        color_b,
        key_id::key_id::KEY_8,
        color_r,
        color_g,
        color_b,
        key_id::key_id::KEY_9,
        color_r,
        color_g,
        color_b,
        key_id::key_id::KEY_0,
        color_r,
        color_g,
        color_b,
        key_id::key_id::MINUS,
        color_r,
        color_g,
        color_b,
        key_id::key_id::EQUALS,
        color_r,
        color_g,
        color_b,
        0xFF,
    ];

    packet[..62].copy_from_slice(&data);

    let _ = write_data_and_wait_for_response(&device, packet);

    let data: [u8; 62] = [
        0x04,
        0xAE,
        0x01,
        0x00,
        0x00,
        0x13,
        0x06,
        brightness,
        0xFF,
        key_id::key_id::BACKSPACE,
        color_r,
        color_g,
        color_b,
        key_id::key_id::TAB,
        color_r,
        color_g,
        color_b,
        key_id::key_id::Q,
        color_r,
        color_g,
        color_b,
        key_id::key_id::W,
        color_r,
        color_g,
        color_b,
        key_id::key_id::E,
        color_r,
        color_g,
        color_b,
        key_id::key_id::R,
        color_r,
        color_g,
        color_b,
        key_id::key_id::T,
        color_r,
        color_g,
        color_b,
        key_id::key_id::Y,
        color_r,
        color_g,
        color_b,
        key_id::key_id::U,
        color_r,
        color_g,
        color_b,
        key_id::key_id::I,
        color_r,
        color_g,
        color_b,
        key_id::key_id::O,
        color_r,
        color_g,
        color_b,
        key_id::key_id::P,
        color_r,
        color_g,
        color_b,
        key_id::key_id::LBRACKET,
        color_r,
        color_g,
        color_b,
        0xFF,
    ];

    packet[..62].copy_from_slice(&data);
    let _ = write_data_and_wait_for_response(&device, packet);

    let data: [u8; 62] = [
        0x04,
        0xAE,
        0x01,
        0x00,
        0x00,
        0x13,
        0x06,
        brightness,
        0xFF,
        key_id::key_id::RBRACKET,
        color_r,
        color_g,
        color_b,
        key_id::key_id::BACKSLASH,
        color_r,
        color_g,
        color_b,
        key_id::key_id::CAPSLOCK,
        color_r,
        color_g,
        color_b,
        key_id::key_id::A,
        color_r,
        color_g,
        color_b,
        key_id::key_id::S,
        color_r,
        color_g,
        color_b,
        key_id::key_id::D,
        color_r,
        color_g,
        color_b,
        key_id::key_id::F,
        color_r,
        color_g,
        color_b,
        key_id::key_id::G,
        color_r,
        color_g,
        color_b,
        key_id::key_id::H,
        color_r,
        color_g,
        color_b,
        key_id::key_id::J,
        color_r,
        color_g,
        color_b,
        key_id::key_id::K,
        color_r,
        color_g,
        color_b,
        key_id::key_id::L,
        color_r,
        color_g,
        color_b,
        key_id::key_id::SEMICOLON,
        color_r,
        color_g,
        color_b,
        0xFF,
    ];

    packet[..62].copy_from_slice(&data);
    let _ = write_data_and_wait_for_response(&device, packet);

    let data: [u8; 62] = [
        0x04,
        0xAE,
        0x01,
        0x00,
        0x00,
        0x13,
        0x06,
        brightness,
        0xFF,
        key_id::key_id::APOSTROPHE,
        color_r,
        color_g,
        color_b,
        key_id::key_id::LSHIFT,
        color_r,
        color_g,
        color_b,
        key_id::key_id::Z,
        color_r,
        color_g,
        color_b,
        key_id::key_id::X,
        color_r,
        color_g,
        color_b,
        key_id::key_id::C,
        color_r,
        color_g,
        color_b,
        key_id::key_id::V,
        color_r,
        color_g,
        color_b,
        key_id::key_id::B,
        color_r,
        color_g,
        color_b,
        key_id::key_id::N,
        color_r,
        color_g,
        color_b,
        key_id::key_id::M,
        color_r,
        color_g,
        color_b,
        key_id::key_id::COMMA,
        color_r,
        color_g,
        color_b,
        key_id::key_id::PERIOD,
        color_r,
        color_g,
        color_b,
        key_id::key_id::SLASH,
        color_r,
        color_g,
        color_b,
        key_id::key_id::RSHIFT,
        color_r,
        color_g,
        color_b,
        0xFF,
    ];

    packet[..62].copy_from_slice(&data);
    let _ = write_data_and_wait_for_response(&device, packet);

    let data: [u8; 38] = [
        0x04,
        0xAE,
        0x01,
        0x00,
        0x00,
        0x13,
        0x06,
        brightness,
        0xFF,
        key_id::key_id::ENTER,
        color_r,
        color_g,
        color_b,
        key_id::key_id::LCTRL,
        color_r,
        color_g,
        color_b,
        key_id::key_id::LWIN,
        color_r,
        color_g,
        color_b,
        key_id::key_id::LALT,
        color_r,
        color_g,
        color_b,
        key_id::key_id::SPACE,
        color_r,
        color_g,
        color_b,
        key_id::key_id::MENU,
        color_r,
        color_g,
        color_b,
        key_id::key_id::RCTRL,
        color_r,
        color_g,
        color_b,
        0xFF,
    ];

    packet[..38].copy_from_slice(&data);
    let _ = write_data_and_wait_for_response(&device, packet);

    // final end packet
    let end_data: [u8; 10] = [
        0x04, 0xAE, 0x01, 0x00, 0x00, 0x13, 0x06, brightness, 0xFF, 0xFF,
    ];
    let mut end_packet = [0u8; 64];
    end_packet[..10].copy_from_slice(&end_data);

    let _ = write_data_and_wait_for_response(&device, end_packet);
}

enum DeerJob {
    Heartbeat,
    Common(bool, bool, bool, bool),
    LedMode(u8, u8, u8, u8, u8),
    ColorAllKeys(u8, u8, u8, u8),
}

fn main() {
    let api = HidApi::new().expect("unable to init HidApi");
    match find_and_open_device(&api) {
        Some(device) => {
            let (tx, rx) = mpsc::channel::<DeerJob>();

            thread::Builder::new()
                .name("deerjob_handler".to_string())
                .spawn(move || {
                    while let Ok(next_job) = rx.recv() {
                        match next_job {
                            DeerJob::Heartbeat => {
                                heartbeat(&device);
                            }
                            DeerJob::Common(turbo, rapid_trigger, dual_trigger, last_win) => {
                                let packet = DataPacket::common(
                                    turbo,
                                    rapid_trigger,
                                    dual_trigger,
                                    last_win,
                                );
                                let _ = write_packet_and_wait_for_response(&device, packet);
                            }
                            DeerJob::LedMode(b, mode, speed, brightness, color) => {
                                let packet =
                                    DataPacket::led_mode(b, mode, speed, brightness, color);
                                let _ = write_packet_and_wait_for_response(&device, packet);
                            }
                            DeerJob::ColorAllKeys(color_r, color_g, color_b, brightness) => {
                                color_all_keys(&device, color_r, color_g, color_b, brightness);
                            }
                        }
                    }
                })
                .expect("unable to start deerjob_handler");

            let _ = tx.send(DeerJob::Common(false, true, false, false));
            let _ = tx.send(DeerJob::ColorAllKeys(0x00, 0x00, 0xFF, 9));
            /*
            let mut swtich = true;
            loop {
                match tx.send(DeerJob::Common(swtich, true, false, false)) {
                    Ok(_) => {}
                    Err(error) => {
                        eprintln!("{:?}", error)
                    }
                }

                swtich = !swtich;
                thread::sleep(Duration::from_secs(1));
            }

            match tx.send(DeerJob::LedMode(
                0,
                lightmode::light_mode::ALWAYSLIGHT,
                6,
                9,
                lightmode::light_color::color_b,
            )) {
                Ok(_) => {}
                Err(error) => {
                    eprintln!("fuck {:?}", error);
                }
            }*/

            loop {
                match tx.send(DeerJob::Heartbeat) {
                    Ok(_) => {}
                    Err(error) => {
                        eprintln!("send heartbeat failed: {:?}", error);
                        break;
                    }
                }
                thread::sleep(Duration::from_secs(10));
            }
        }
        None => eprintln!(""),
    }
}
