pub struct KeyboardType {
    pub name: &'static str,
    pub vendor_id: u16,
    pub product_id: u16,
    pub byte5: u8,
    pub byte6: u8,
    pub byte7: u8,
}

pub const KEYBOARD_TYPES: &[KeyboardType] = &[
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

pub const VENDOR_ID: u16 = 13613;
// NOTE: Drunkeer has a few different HID interfaces, the config
// related one is located on this usage page.
pub const USAGE_PAGE: u16 = 0xFF00;
pub const _MANUFACTURER: &str = "Drunkdeer";
//pub const REPORT_ID: u8 = 4;