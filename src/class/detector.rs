mod knn_tracker;
mod parameters_processing;

use knn_tracker::KNN;
use opencv::{core, imgproc as cv2, prelude::*};
use parameters_processing::Parameter;

struct Detector {
    open_flag: bool,
    update_flag: bool,
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
struct Result {
    aim_ring: i32,
    shoot_ring: i32,
    shake: i32,
    shake_v: i32,
    shoot_shake: i32,
    shoot_shake_v: i32,
    center_x: i32,
    center_y: i32,
}

impl Detector {
    fn new(open_flag: bool) -> Detector {
        Detector {
            open_flag,
            update_flag: false,
            det_flag: false,
            center_list: Vec![],
            center: [],
            center_x: 0,
            center_y: 0,
            frame_count: 0,
            parameter: Parameter::new(),
            aim_x_list: Vec![],
            aim_y_list: Vec![],
            shoot_x_list: Vec![],
            shoot_y_list: Vec![],
        }
    }

    //追踪激光点
    fn catch_center(&mut self, frame: &core::Mat) {
        match self.open_flag {
            true => {
                self.frame_count += 1;
                let knn = KNN::new();
                let cents = knn.apply(frame);
                self.det_flag = cents.iter().any(|c| cv2::contour_area(c) > 10.0);
                self.center_list = cents
                    .iter()
                    .filter(|c| cv2::contour_area(c) > 10.0)
                    .filter_map(|c| {
                        let m = cv2::moments(c, false);
                        match m.m00 {
                            0.0 => {
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
                self.center.clear();
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
    fn update(&mut self, flag: bool, frame: &core::Mat) -> Result {
        self.catch_center(frame);
        match flag {
            // 更新射击参数
            ture => {
                let message = Result {
                    aim_ring: self.score.aim_ring(self.aim_x_list, self.aim_y_list),
                    shoot_ring: self.score.shoot_ring(self.center_x, self.center_y),
                    shake: self.score.shake(self.aim_x_list, self.aim_y_list),
                    shake_v: self.score.shake_v(self.aim_x_list, self.aim_y_list),
                    shoot_shake: self.score.shoot_shake(
                        self.shoot_x_list,
                        self.shoot_y_list,
                        self.center_x,
                        self.center_y,
                    ),
                    shoot_shake_v: self.score.shoot_shake_v(
                        self.shoot_x_list,
                        self.shoot_y_list,
                        self.center_x,
                        self.center_y,
                    ),
                    center_x: self.center_x,
                    center_y: self.center_y,
                };
                self.shoot_x_list.clear();
                self.shoot_y_list.clear();
                self.list_check();
                message
            }
            false => {
                let message = Result {
                    aim_ring: self.score.aim_ring(self.aim_x_list, self.aim_y_list),
                    shoot_ring: 0,
                    shake: self.score.shake(self.aim_x_list, self.aim_y_list),
                    shake_v: self.score.shake_v(self.aim_x_list, self.aim_y_list),
                    shoot_shake: 0,
                    shoot_shake_v: 0,
                    center_x: self.center_x,
                    center_y: self.center_y,
                };
                self.list_check();
                message
            }
        };
    }
}
