use serialport::SerialPort;

struct DeviceCommunicator {
    serial: Box<dyn SerialPort>,
    core_gun: [u8; 12],
    core_web: [u8; 15],
    data_gun: [u8; 12],
    data_web: [u8; 15],
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
    fn new(baud_rate: u32) -> DeviceCommunicator {
        DeviceCommunicator {
            serial: serialport::new("/dev/ttyUSB0", baud_rate)
                .open()
                .expect("串口启动失败"),
            core_gun: [
                0x0b, 0x10, 0x10, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x1E, 0x67, 0x68,
            ],
            core_web: [
                0x16, 0x10, 0x10, 0x00, 0x0c, 0x00, 0x00, 0x01, 0x00, 0x1E, 0x07, 0x08, 0x69, 0x70,
                0x71,
            ],
            data_gun: [0; 12],
            data_web: [0; 15],
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
    fn send(&mut self, data_buffer: &[u8]) {
        self.serial.write_all(data_buffer);
    }

    // 收信息
    fn receive(&mut self) {
        let mut temp: [u8; 17] = [0; 17];
        self.serial.read(&mut temp);
        match temp[0] {
            0xaa => (),
            0x0b => {
                self.data_gun[0] = 0x0b;
                self.data_gun[1..]
                    .iter_mut()
                    .zip(temp.iter())
                    .for_each(|(a, b)| *a = *b);
                self.receive_flag_g = true;
            }
            0x16 => {
                self.data_web[0] = 0x16;
                self.data_web[1..]
                    .iter_mut()
                    .zip(temp.iter())
                    .for_each(|(a, b)| *a = *b);
                self.receive_flag_w = true
            }
            _ => (),
        }
    }
}
