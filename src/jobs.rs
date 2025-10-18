use std::sync::mpsc;
use std::thread;
use hidapi::HidDevice;

use crate::data_packet::DataPacket;
use crate::device::*;

pub enum DeerJob {
    Heartbeat,
    Common {
        turbo: bool,
        rapid_trigger: bool,
        dual_trigger: bool,
        last_win: bool,
    },
    ColorAllKeys {
        color_r: u8,
        color_g: u8,
        color_b: u8,
        brightness: u8,
    },
    ColorOneKey {
        key_id: u8,
        color_r: u8,
        color_g: u8,
        color_b: u8,
        brightness: u8,
    },
}

pub fn start_job_handler(device: HidDevice, rx: mpsc::Receiver<DeerJob>) {
    thread::Builder::new()
        .name("deerjob_handler".to_string())
        .spawn(move || {
            while let Ok(next_job) = rx.recv() {
                match next_job {
                    DeerJob::Heartbeat => {
                        let packet = DataPacket::heartbeat();
                        let response = write_packet_and_wait_for_response(&device, packet);
                        println!("Heartbeat: {:?}", response);
                    }
                    DeerJob::Common {turbo, rapid_trigger, dual_trigger, last_win} => {
                        let packet = DataPacket::common(
                            turbo,
                            rapid_trigger,
                            dual_trigger,
                            last_win,
                        );
                        let _ = write_packet_and_wait_for_response(&device, packet);
                    }
                    DeerJob::ColorAllKeys {color_r, color_g, color_b, brightness} => {
                        // NOTE: if brightness is > 9 then the keyboard fails to respond
                        assert!(brightness <= 9);
                        color_all_keys(&device, color_r, color_g, color_b, brightness);
                    },
                    DeerJob::ColorOneKey {key_id, color_r, color_g, color_b, brightness} => {
                        assert!(brightness <= 9);
                        set_led_by_key_id(&device, key_id, color_r, color_g, color_b, brightness);
                    }
                }
            }
        })
        .expect("unable to start deerjob_handler");
}