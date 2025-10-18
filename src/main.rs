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

#[allow(dead_code)]
fn key_test_1(tx: &mpsc::Sender<crate::jobs::DeerJob>) {
    loop {
        let _ = tx.send(crate::jobs::DeerJob::ColorOneKey{
            key_id: 0xCC,
            color_r: 255,
            color_g: 0,
            color_b: 0,
            brightness: 9,
        });
        let _ = tx.send(crate::jobs::DeerJob::ColorOneKey{
            key_id: 0xE1,
            color_r: 0,
            color_g: 0,
            color_b: 255,
            brightness: 9,
        });
        thread::sleep(Duration::from_secs(1));

        let _ = tx.send(crate::jobs::DeerJob::ColorOneKey{
            key_id: 0xCC,
            color_r: 0,
            color_g: 0,
            color_b: 255,
            brightness: 9,
        });
        let _ = tx.send(crate::jobs::DeerJob::ColorOneKey{
            key_id: 0xE1,
            color_r: 255,
            color_g: 0,
            color_b: 0,
            brightness: 9,
        });
        thread::sleep(Duration::from_secs(1));
    }
}

#[allow(dead_code)]
fn key_test_2(tx: &mpsc::Sender<crate::jobs::DeerJob>) {
    let key_id = 0xCB;
    loop {
        println!("Red");
        let _ = tx.send(crate::jobs::DeerJob::ColorOneKey{
            key_id: key_id,
            color_r: 255,
            color_g: 0,
            color_b: 0,
            brightness: 9,
        });
        thread::sleep(Duration::from_secs(1));

        println!("Green");
        let _ = tx.send(crate::jobs::DeerJob::ColorOneKey{
            key_id: key_id,
            color_r: 0,
            color_g: 255,
            color_b: 0,
            brightness: 9,
        });
        thread::sleep(Duration::from_secs(1));

        println!("Blue");
        let _ = tx.send(crate::jobs::DeerJob::ColorOneKey{
            key_id: key_id,
            color_r: 0,
            color_g: 0,
            color_b: 255,
            brightness: 9,
        });
        thread::sleep(Duration::from_secs(1));
    }
}

#[allow(dead_code)]
fn key_test_3(tx: &mpsc::Sender<crate::jobs::DeerJob>) {
    let key_ids: [u8; 12] = [ 0xC0, 0xC1, 0xC2, 0xC3, 0xC4, 0xC5, 0xC6, 0xC7, 0xC8, 0xC9, 0xCA, 0xCB ]; 

    for key_id in key_ids.iter() {
        println!("Key ID: {:02X}", key_id);
        println!("Red");
        let _ = tx.send(crate::jobs::DeerJob::ColorOneKey{
            key_id: *key_id,
            color_r: 255,
            color_g: 0,
            color_b: 0,
            brightness: 9,
        });
        thread::sleep(Duration::from_secs(1));

        println!("Green");
        let _ = tx.send(crate::jobs::DeerJob::ColorOneKey{
            key_id: *key_id,
            color_r: 0,
            color_g: 255,
            color_b: 0,
            brightness: 9,
        });
        thread::sleep(Duration::from_secs(1));

        println!("Blue");
        let _ = tx.send(crate::jobs::DeerJob::ColorOneKey{
            key_id: *key_id,
            color_r: 0,
            color_g: 0,
            color_b: 255,
            brightness: 9,
        });
        thread::sleep(Duration::from_secs(1));
        }
}

fn key_test_4(tx: &mpsc::Sender<crate::jobs::DeerJob>) {
    loop {
        for brightness in 0..=9 {
            println!("Brightness: {}", brightness);
            let _ = tx.send(crate::jobs::DeerJob::ColorAllKeys {
                color_r: 255,
                color_g: 0,
                color_b: 0,
                brightness: brightness,
            });
            thread::sleep(Duration::from_millis(500));
        }
    }
}

fn main() {
    let api = hidapi::HidApi::new().expect("unable to init HidApi");
    match crate::device::find_and_open_device(&api) {
        Some(device) => {
            let (tx, rx) = mpsc::channel::<crate::jobs::DeerJob>();

            crate::jobs::start_job_handler(device, rx);

            let _ = tx.send(crate::jobs::DeerJob::Common{
                turbo: false,
                rapid_trigger: true,
                dual_trigger: false,
                last_win: false,
            });
            let _ = tx.send(crate::jobs::DeerJob::ColorAllKeys{
                color_r: 0,
                color_g: 0,
                color_b: 0,
                brightness: 0,
            });
        
            key_test_4(&tx);

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
