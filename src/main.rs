use hidapi::HidApi;
use hidapi::HidDevice;

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
const USAGE_PAGE: u16 = 0xFF00;
const MANUFACTURER: &str = "Drunkdeer";
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

fn main() {
    let api = HidApi::new().expect("unable to init HidApi");
    match find_and_open_device(&api) {
        Some(device) => {
            let dev_info: hidapi::DeviceInfo = device.get_device_info().unwrap();
        }
        None => eprintln!(""),
    }
}
