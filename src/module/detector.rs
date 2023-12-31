use super::knn_tracker::KNN;
use super::parameters_processing::Parameter;
use opencv::{core, imgproc as cv2};

pub struct Detector {
    pub open_flag: bool,
    det_flag: bool,
    center_list: Vec<(i32, i32)>,
    center: [i32; 2],
    center_x: i32,
    center_y: i32,
    frame_count: i32,
    parameter: Parameter,
    aim_x_list: Vec<i32>,
    aim_y_list: Vec<i32>,
    shoot_x_list: Vec<i32>,
    shoot_y_list: Vec<i32>,
}

// 储存射击参数
pub struct ShootingResult {
    pub aim_ring: u8,
    pub shoot_ring: u8,
    pub shake: u8,
    pub shake_v: u8,
    pub shoot_shake: u8,
    pub shoot_shake_v: u8,
    pub center_x: u8,
    pub center_y: u8,
}

impl Detector {
    pub fn new(open_flag: bool) -> Detector {
        Detector {
            open_flag,
            det_flag: false,
            center_list: vec![],
            center: [0, 0],
            center_x: 0,
            center_y: 0,
            frame_count: 0,
            parameter: Parameter::new(),
            aim_x_list: vec![],
            aim_y_list: vec![],
            shoot_x_list: vec![],
            shoot_y_list: vec![],
        }
    }

    //追踪激光点
    fn catch_center(&mut self, frame: &core::Mat) {
        match self.open_flag {
            true => {
                self.frame_count += 1;
                let mut knn = KNN::new().unwrap();
                let cents = knn.program(frame);
                self.det_flag = cents
                    .iter()
                    .any(|c| cv2::contour_area(c, false).unwrap() > 10.0);
                self.center_list = cents
                    .iter()
                    .filter(|c| cv2::contour_area(c, false).unwrap() > 10.0)
                    .filter_map(|c| {
                        let m = cv2::moments(c, false).unwrap();
                        match m.m00 {
                            x if x == 0.0 => {
                                self.det_flag = false;
                                None
                            }
                            _ => {
                                let x = (m.m10 / m.m00) as i32;
                                let y = (m.m01 / m.m00) as i32;
                                Some((x, y))
                            }
                        }
                    })
                    .collect();
                match self.center_list.last() {
                    Some(&(x, y)) => self.center = [x, y],
                    None => (),
                }
            }
            false => (),
        }
        match self.det_flag {
            true => {
                // 重置坐标
                self.det_flag = false;
                // 储存中心坐标
                self.aim_x_list.push(self.center[0]);
                self.aim_y_list.push(self.center[1]);
                self.shoot_x_list.push(self.center[0]);
                self.shoot_y_list.push(self.center[1]);
                self.center_x = self.center[0];
                self.center_y = self.center[1];
                // 清空列表
                self.center.fill(0);
            }
            false => (),
        }
    }

    // list_check
    fn list_check(&mut self) {
        match self.frame_count {
            0..=34 => (),
            _ => {
                self.aim_x_list.clear();
                self.aim_y_list.clear();
                self.frame_count = 0
            }
        };
        match self.shoot_x_list.len() {
            0..=100 => (),
            _ => {
                self.shoot_x_list.clear();
                self.shoot_y_list.clear();
            }
        };
    }

    // 更新相关参数
    pub fn update(&mut self, flag: bool, frame: &core::Mat) -> ShootingResult {
        self.catch_center(frame);
        match flag {
            // 更新射击参数
            true => {
                let message = ShootingResult {
                    aim_ring: self.parameter.aim_ring(&self.aim_x_list, &self.aim_y_list),
                    shoot_ring: self.parameter.shoot_ring(self.center_x, self.center_y),
                    shake: self.parameter.shake(&self.aim_x_list, &self.aim_y_list),
                    shake_v: self.parameter.shake_v(&self.aim_x_list, &self.aim_y_list),
                    shoot_shake: self.parameter.shoot_shake(
                        &self.shoot_x_list,
                        &self.shoot_y_list,
                        self.center_x,
                        self.center_y,
                    ),
                    shoot_shake_v: self.parameter.shoot_shake_v(
                        &self.shoot_x_list,
                        &self.shoot_y_list,
                        self.center_x,
                        self.center_y,
                    ),
                    center_x: self.center_x as u8,
                    center_y: self.center_y as u8,
                };
                self.shoot_x_list.clear();
                self.shoot_y_list.clear();
                self.list_check();
                return message;
            }
            false => {
                let message = ShootingResult {
                    aim_ring: self.parameter.aim_ring(&self.aim_x_list, &self.aim_y_list),
                    shoot_ring: 0,
                    shake: self.parameter.shake(&self.aim_x_list, &self.aim_y_list),
                    shake_v: self.parameter.shake_v(&self.aim_x_list, &self.aim_y_list),
                    shoot_shake: 0,
                    shoot_shake_v: 0,
                    center_x: self.center_x as u8,
                    center_y: self.center_y as u8,
                };
                self.list_check();
                return message;
            }
        };
    }
}
