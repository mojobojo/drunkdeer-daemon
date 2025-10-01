//mod data_packet;
//mod key_id;
//mod lightmode;

use hidapi::HidApi;
use hidapi::HidDevice;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

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
                if device.product_id() == keyboard.product_id {
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

enum DeerJob {
    Heartbeat,
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
                        }
                    }
                })
                .expect("unable to start deerjob_handler");

            loop {
                match tx.send(DeerJob::Heartbeat) {
                    Ok(_) => {}
                    Err(error) => {
                        println!("send heartbeat failed: {:?}", error);
                        break;
                    }
                }
                thread::sleep(Duration::from_secs(10));
            }
        }
        None => eprintln!(""),
    }
}
