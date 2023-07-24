use super::detector::ShootingResult;
use serialport::SerialPort;

pub struct DeviceCommunicator {
    serial: Box<dyn SerialPort>,
    core_gun: [u8; 12],
    core_web: [u8; 15],
    data_gun: [u8; 12],
    data_web: [u8; 15],
    count: i32,
    receive_flag: i32,
    pub target_enroll: bool,
    pub core_gun_flag: bool,
    pub core_web_flag: bool,
    pub receive_enroll_flag: bool,
    pub receive_data_flag: bool,
    pub receive_quit_flag: bool,
    confirm_set_flag: bool,
    send_interval: bool,
    pub open_flag: bool,
}

impl DeviceCommunicator {
    pub fn new(baud_rate: u32) -> DeviceCommunicator {
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

            receive_flag: 0,
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
    pub fn receive(&mut self) {
        let mut temp: [u8; 17] = [0; 17];
        let _ = self.serial.read(&mut temp);
        match temp[0] {
            0xaa => (),
            0x0b => {
                self.data_gun[0] = 0x0b;
                self.data_gun[1..]
                    .iter_mut()
                    .zip(temp.iter())
                    .for_each(|(a, b)| *a = *b);
                self.receive_flag = 1;
            }
            0x16 => {
                self.data_web[0] = 0x16;
                self.data_web[1..]
                    .iter_mut()
                    .zip(temp.iter())
                    .for_each(|(a, b)| *a = *b);
                self.receive_flag = 2;
            }
            _ => (),
        }
    }

    // 信息处理
    pub fn message(&mut self) {
        match self.receive_flag {
            1 => {
                self.receive_flag = 0;
                if self.data_gun[0] == 11 && self.data_gun[2] == 16 {
                    let mut temp = self.data_gun.clone();
                    temp[8] = 153;
                    temp[2] = 0x01;
                    self.send(&temp);
                    match self.data_gun[1] {
                        0x00 => {
                            self.receive_enroll_flag = true;
                            self.receive_quit_flag = false;
                            self.core_gun[3] = self.data_gun[3];
                            self.core_gun[4] = self.data_gun[4];
                        }
                        0x01 => {
                            self.receive_enroll_flag = false;
                            self.receive_quit_flag = true;
                            self.core_gun[3] = 0xff;
                            self.core_gun[4] = 0xff;
                        }
                        0x10 => {
                            self.receive_data_flag = true;
                        }
                        0x11 => {
                            self.confirm_set_flag = true;
                        }
                        _ => (),
                    }
                }
            }
            2 => {
                self.receive_flag = 0;
                if self.data_web[0] == 0x16 && self.data_web[2] == 0x01 {
                    match self.data_web[1] {
                        0x00 => match self.data_web[8] {
                            0x99 => {
                                print!("确认注册");
                                self.target_enroll = true;
                            }
                            _ => (),
                        },
                        0x01 => match self.data_web[8] {
                            0x99 => {
                                print!("退出");
                            }
                            _ => (),
                        },
                        0x10 => match self.data_web[8] {
                            0x99 => {
                                print!("数据确认");
                            }
                            _ => (),
                        },
                        0x11 => {
                            print!("收到设置");
                            self.core_gun_flag = true;
                        }
                        _ => (),
                    }
                }
            }
            _ => {
                self.data_gun.fill(0);
                self.data_web.fill(0);
            }
        }
    }

    // 注册
    pub fn enroll(&mut self) {
        self.core_web[0] = 0x16;
        self.core_web[1] = 0x00;
        self.core_web[2] = 0x10;
        self.core_web[8] = 0x00;
        let temp = self.core_web.clone();
        self.send(&temp);
    }

    // 传射击数据
    fn hit(&mut self, flag: bool, message: ShootingResult) {
        match flag {
            true => self.core_web[3] = 0x01,
            false => self.core_web[3] = 0x00,
        };
        self.core_web[0..3].copy_from_slice(&[0x16, 0x10, 0x10]);
        self.core_web[4..10].copy_from_slice(&[
            message.aim_ring,
            message.shoot_ring,
            message.shake,
            message.shake_v,
            message.shoot_shake,
            message.shoot_shake_v,
        ]);
        let center_x_h = message.center_x as f32 / 480.0 * 100.0;
        let center_x_l = ((message.center_x as f32 / 480.0 * 100.0 - center_x_h) * 100.0) as u8;
        let center_y_h = message.center_y as f32 / 480.0 * 100.0;
        let center_y_l = ((message.center_y as f32 / 480.0 * 100.0 - center_y_h) * 100.0) as u8;
        self.core_web[10..].copy_from_slice(&[
            center_x_h as u8,
            center_x_l,
            center_y_h as u8,
            center_y_l,
        ]);
        format!("坐标为{},{}", message.center_x, message.center_y);
        let temp = self.core_web.clone();
        self.send(&temp);
    }
}
