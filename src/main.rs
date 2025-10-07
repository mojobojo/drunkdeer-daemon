//mod data_packet;
mod data_packet;
mod key_id;
mod lightmode;
mod keyboard;
mod device;
mod jobs;

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let api = hidapi::HidApi::new().expect("unable to init HidApi");
    match crate::device::find_and_open_device(&api) {
        Some(device) => {
            let (tx, rx) = mpsc::channel::<crate::jobs::DeerJob>();

            crate::jobs::start_job_handler(device, rx);

            let _ = tx.send(crate::jobs::DeerJob::Common(false, true, false, false));
            let _ = tx.send(crate::jobs::DeerJob::ColorAllKeys(0x00, 0x00, 0xFF, 9));
            /*
            let mut swtich = true;
            loop {
                match tx.send(crate::jobs::DeerJob::Common(swtich, true, false, false)) {
                    Ok(_) => {}
                    Err(error) => {
                        eprintln!("{:?}", error)
                    }
                }

                swtich = !swtich;
                thread::sleep(Duration::from_secs(1));
            }

            match tx.send(crate::jobs::DeerJob::LedMode(
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
                match tx.send(crate::jobs::DeerJob::Heartbeat) {
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
