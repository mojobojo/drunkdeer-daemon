use std::sync::mpsc;
use std::thread;
use hidapi::HidDevice;

use crate::data_packet::DataPacket;
use crate::device::{write_packet_and_wait_for_response, color_all_keys};

pub enum DeerJob {
    Heartbeat,
    Common(bool, bool, bool, bool),
    ColorAllKeys(u8, u8, u8, u8),
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
                    DeerJob::Common(turbo, rapid_trigger, dual_trigger, last_win) => {
                        let packet = DataPacket::common(
                            turbo,
                            rapid_trigger,
                            dual_trigger,
                            last_win,
                        );
                        let _ = write_packet_and_wait_for_response(&device, packet);
                    }
                    DeerJob::ColorAllKeys(color_r, color_g, color_b, brightness) => {
                        color_all_keys(&device, color_r, color_g, color_b, brightness);
                    }
                }
            }
        })
        .expect("unable to start deerjob_handler");
}