use serial2::SerialPort;

struct DeviceCommunicator {
    serial: SerialPort,
    core_gun: [u8; 12],
    core_web: [u8; 15],
    dadat_gun: [u8; 12],
    datat_web: [u8; 15],
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

impl DeviceCommunicator {
    fn new(baud_rate: i32) -> DeviceCommunicator {
        DeviceCommunicator {
            serial: SerialPort::open("/dev/ttyUSB0", baud_rate)?,
            core_gun: [
                0x0b, 0x10, 0x10, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x1E, 0x67, 0x68,
            ],
            core_web: [
                0x16, 0x10, 0x10, 0x00, 0x0c, 0x00, 0x00, 0x01, 0x00, 0x1E, 0x07, 0x08, 0x69, 0x70,
                0x71,
            ],
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

    // 发信息
    fn send(&self, data_buffer: &[u8]) {
        let temp = data_buffer.to_vec();
        self.serial.write(temp);
    }

    // 收信息
    fn receive(&self) {
        let temp = " ";
    }
}
