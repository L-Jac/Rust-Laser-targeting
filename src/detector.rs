mod shooting_parameters;

use opencv::{
    prelude::*,
    videoio::{VideoWriter, VideoWriter_fourcc},
};
use shooting_parameters::Parameter;

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
    aim_ring: i32,
    shoot_ring: i32,
    shake: i32,
    shake_v: i32,
    shoot_shake: i32,
    shoot_shake_v: i32,
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
            aim_ring: 0,
            shoot_ring: 0,
            shake: 0,
            shake_v: 0,
            shoot_shake: 0,
            shoot_shake_v: 0,
        }
    }

    //追踪激光点
    
}
