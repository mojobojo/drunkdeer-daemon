use hidapi::HidApi;
use hidapi::HidDevice;

use crate::data_packet::DataPacket;
use crate::keyboard::{KEYBOARD_TYPES, VENDOR_ID, USAGE_PAGE};
use crate::key_id;

pub fn find_and_open_device(api: &HidApi) -> Option<HidDevice> {
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

    None
}

pub fn write_data_and_wait_for_response(device: &HidDevice, data: [u8; 64]) -> [u8; 64] {
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

    buffer
}

pub fn write_packet_and_wait_for_response(device: &HidDevice, packet: DataPacket) -> DataPacket {
    let data = write_data_and_wait_for_response(device, packet.to_bytes());
    DataPacket::from_bytes(data)
}

pub fn color_all_keys(device: &HidDevice, color_r: u8, color_g: u8, color_b: u8, brightness: u8) {
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

    let _ = write_data_and_wait_for_response(device, packet);

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
    let _ = write_data_and_wait_for_response(device, packet);

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
    let _ = write_data_and_wait_for_response(device, packet);

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
    let _ = write_data_and_wait_for_response(device, packet);

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
    let _ = write_data_and_wait_for_response(device, packet);

    // final end packet
    let end_data: [u8; 10] = [
        0x04, 0xAE, 0x01, 0x00, 0x00, 0x13, 0x06, brightness, 0xFF, 0xFF,
    ];
    let mut end_packet = [0u8; 64];
    end_packet[..10].copy_from_slice(&end_data);

    let _ = write_data_and_wait_for_response(device, end_packet);
}