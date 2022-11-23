#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{time::Duration, collections::HashMap};

use lidar::ydlidarx2;
use tauri::Manager;

mod lidar;
mod mytools;

use mytools::Xtools;
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
                let mut port = match serialport::new("COM5", 115200)
                    .stop_bits(serialport::StopBits::One)
                    .data_bits(serialport::DataBits::Eight)
                    .timeout(Duration::from_millis(10))
                    .open()
                {
                    Ok(p) => p,
                    Err(_) => panic!(),
                };
                let mut serial_buf: Vec<u8> = vec![0; 3500];

                let roun = 1;

                let mut map: HashMap<String, f64> = HashMap::new();

                loop {
                    match port.read(serial_buf.as_mut_slice()) {
                        Ok(t) => {
                            let mut map_vec= Vec::new();

                            let mut data = serial_buf[..t].to_vec();
                            let points = ydlidarx2(&mut data);

                            for i in points.iter() {
                                //println!("{}",i.0.roundf(10));
                                map.insert(i.0.roundf(roun).to_string(), i.1);
                            }

                            //println!("{}",map.len());

                            for (a, d) in map.iter() {
                                let a = a.parse::<f64>().unwrap();
                                let y = a.sin() * d;
                                let x = a.cos() * d;
                                map_vec.push((x,y))
                            }

                            /*
                            for i in points.iter() {
                                map_vec.push(((i.0.cos() * i.1),(i.0.sin() * i.1)))
                            }
                            */
                           
                            

                            

                            app_handle.emit_all("back-to-front", map_vec.clone()).unwrap();

                        }

                        Err(_) => {}
                    }
                }

                /*
                let mut t = Vec::new();
                let mut a = (0.0, 0.0);

                loop {
                    t.push(a);
                    app_handle.emit_all("back-to-front", t.clone()).unwrap();

                    a = (a.0 + 1.0, a.1 + 1.0);
                    std::thread::sleep(std::time::Duration::from_secs(1))
                }
                
                */
                
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
