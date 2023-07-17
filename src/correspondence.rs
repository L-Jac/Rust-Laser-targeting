use serialport;
use std::{thread, time};
use chrono::{DateTime, Utc};

struct DeviceCommunicator {
    net_idl: i32,
    start: Option<DateTime<Utc>>,
    data_length: i32,
    init_order: [i32; 18],
    machine_id:u16,
    core_gun: [i32; 12],
    core_web: [i32; 15],
    dadat_gun: [i32; 12],
    datat_web: [i32; 15],
    count: i32,
    receive_flag_g: bool,
    receive_flag_w: bool,
    target_enroll: bool,
    core_gun_flag: bool,
    core_web_flag: bool,
    receive_enroll_flag: bool,
    receive_data_flag: bool,
    receive_quit_flag: bool,
    confirm_set_flag: bool,
    send_interval: bool,
    open_flag: bool,
}

struct DeviceCommunicatorOptions {
    uart_baud: i32,
    machine_id: u16,
    net_idh: i32,
    net_idl: i32,
    rf_power: i32,
    rf_channel: i32,
    data_length: i32,
}

impl DeviceCommunicator {
    fn new(options: &DeviceCommunicatorOptions) -> DeviceCommunicator {
        DeviceCommunicator {
            net_idl: options.net_idl,
            start: None,
            data_length: options.data_length,
            init_order: [0; 18],
            machine_id: options.machine_id,
            core_gun: [0; 12],
            core_web: [0; 15],
            dadat_gun: [0; 12],
            datat_web: [0; 15],
            count: 0,
            receive_flag_g: false,
            receive_flag_w: false,
            target_enroll: false,
            core_gun_flag: false,
            core_web_flag: false,
            receive_enroll_flag: false,
            receive_data_flag: false,
            receive_quit_flag: false,
            confirm_set_flag: false,
            send_interval: false,
            open_flag: false,
        }
    }
}
