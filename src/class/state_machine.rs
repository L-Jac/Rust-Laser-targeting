use super::correspondence::DeviceCommunicator;
use super::detector::Detector;
use super::webcamstream::WebcamStream;
use opencv::Error;

enum Status {
    Idle,      // 空闲态
    EnrollWeb, // 注册网关
    EnrollGun, // 注册枪
    Quit,      // 退出
}

enum Events {
    Null,       // 无事件
    OpenDetect, //
    Quit,       // 关摄像头
    SendCurve,  // 发轨迹
    SendPoint,  // 发击中坐标
}

struct StateMachine {
    connect: DeviceCommunicator,
    detect: Detector,
    status: Status,
    event: Events,
    webcamstream: WebcamStream,
    target_start: f64,
    target_end: f64,
    gun_start: f64,
    gun_end: f64,
    count: i32,
}

impl StateMachine {
    fn new() -> StateMachine {
        StateMachine {
            connect: DeviceCommunicator::new(9600),
            detect: Detector::new(false),
            status: Status::Idle,
            event: Events::Null,
            webcamstream: WebcamStream::new(10).unwrap(),
            target_start: 0.0,
            target_end: 0.0,
            gun_start: 0.0,
            gun_end: 0.0,
            count: 0,
        }
    }

    fn get_action(&mut self) {
        self.connect.receive();
        self.connect.message();
    }

    fn fsm_status(&mut self) {
        loop {
            match self.status {
                Status::Idle => {
                    match self.connect.target_enroll {
                        true => {
                            self.connect.target_enroll = false;
                            self.status = Status::EnrollWeb;
                            self.event = Events::Null;
                            self.count = 0;
                            print!("靶箱确认注册");
                        }
                        false => {
                            self.status = Status::Idle;
                            self.event = Events::Null;
                            self.count += 1;
                            if self.count % 5 == 0 {
                                print!("靶箱注册中");
                                self.status = Status::Idle;
                                self.event = Events::Null;
                                self.count = 0;
                                self.connect.enroll();
                            }
                        }
                    };
                    break;
                }
                Status::EnrollWeb => {
                    match self.connect.receive_enroll_flag {
                        true => {
                            self.status = Status::EnrollGun;
                            self.event = Events::OpenDetect;
                            self.count = 0;
                        }
                        false => {
                            self.status = Status::EnrollWeb;
                            self.event = Events::Null;
                            self.count += 1;
                            if self.count > 600 {
                                print!("连接超时");
                                self.count = 0;
                            };
                        }
                    };
                    break;
                }
                Status::EnrollGun => {
                    match self.connect.receive_data_flag {
                        true => {
                            self.connect.core_web_flag = true;
                            self.connect.receive_data_flag = false;
                        }
                        false => (),
                    };
                    match (self.connect.core_web_flag, self.detect.open_flag) {
                        (true, true) => {
                            self.status = Status::EnrollGun;
                            self.event = Events::SendPoint;
                            self.connect.core_web_flag = false;
                        }
                        _ => (),
                    };
                    match self.connect.receive_quit_flag {
                        true => {
                            print!("枪退出");
                            self.connect.receive_quit_flag = false;
                            self.status = Status::EnrollWeb;
                            self.event = Events::Quit;
                            self.count = 0;
                            break;
                        }
                        false => (),
                    };
                    match self.connect.core_gun_flag {
                        true => match self.detect.open_flag {
                            true => {
                                self.status = Status::EnrollGun;
                                self.event = Events::SendCurve;
                                self.count = 0;
                            }
                            false => (),
                        },
                        false => (),
                    }
                    break;
                }
                Status::Quit => break,
            }
        }
    }

    fn fsm_events(&mut self) {
        loop {
            match self.event {
                Events::OpenDetect => {
                    self.detect.open_flag = true;
                    self.event = Events::Null;
                    print!("相机启动");
                    break;
                }
                Events::Quit => {
                    self.connect.open_flag = false;
                    self.connect.receive_quit_flag = false;
                    self.connect.receive_enroll_flag = false;
                    self.webcamstream.stop();
                    self.event = Events::Null;
                    print!("相机关闭");
                    break;
                }
                Events::SendCurve => todo!(),
                Events::SendPoint => todo!(),
                Events::Null => break,
            }
        }
    }
}
