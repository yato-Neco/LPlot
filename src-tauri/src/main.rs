#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::time::Duration;

use lidar::ydlidarx2;
use tauri::Manager;

mod lidar;
mod mytools;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {
            let app_handle = app.app_handle();
            std::thread::spawn(move || {
                /*


                let mut port = match serialport::new("COM3", 115200)
                    .stop_bits(serialport::StopBits::One)
                    .data_bits(serialport::DataBits::Eight)
                    .timeout(Duration::from_millis(10))
                    .open()
                {
                    Ok(p) => p,
                    Err(_) => panic!(),
                };
                let mut serial_buf: Vec<u8> = vec![0; 1500];

                loop {
                    match port.read(serial_buf.as_mut_slice()) {
                        Ok(t) => {
                            let mut data = serial_buf[..t].to_vec();
                            let points = ydlidarx2(&mut data);

                            //slam.push(((0.0_f64,0.0_f64),points));


                            /*
                            for i in points {
                                if i.0 >= 165.0 && i.0 <= 195.0 && i.1 < 4.5 {
                                    println!("{}度 {}cm", i.0, i.1);
                                }
                            }
                            */
                        }

                        Err(_) => {}
                    }

                }
                */
                let mut t = Vec::new();
                let mut a = (0.0,0.0);

                loop {

                    t.push(a);
                    app_handle
                        .emit_all("back-to-front", t.clone())
                        .unwrap();

                    a = (a.0 + 1.0, a.1 + 1.0);
                    std::thread::sleep(std::time::Duration::from_secs(1))
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
