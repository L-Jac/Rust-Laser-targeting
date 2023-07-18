use ndarray::Array;

struct Parameter {
    r: [i32; 11],
    k: f64,
    center_x: i32,
    center_y: i32,
}

impl Parameter {
    fn new() -> Parameter {
        Parameter {
            r: [0, 0, 0, 0, 0, 216, 168, 120, 72, 24, 0],
            k: 500 / 480,
            center_x: 296,
            center_y: 264,
        }
    }

    // 根据坐标算出环数
    fn ring(&self, hit_x: i32, hit_y: i32) -> i32 {
        let n = (((hit_x as f64 - self.center_x as f64).powi(2))
            + ((hit_y as f64 - self.center_y as f64).powi(2)))
        .sqrt();
        let r_i = match n {
            n if n >= 0 && n < 24 => 10,
            n if n < 72 => 9,
            n if n < 120 => 8,
            n if n < 168 => 7,
            n if n <= 216 => 6,
            _ => return 0,
        };
        let r_d = ((n - self.r[r_i] as f64) / 48.0);
        return (10 * (r_i as f64 + (1.0 - r_d))) as i32;
    }

    //计算瞄准平均坐标

    fn aim_axis(&self, aim_x_list: &Vec<i32>, aim_y_list: &Vec<i32>) -> (f64, f64) {
        if !aim_x_list.is_empty() && !aim_y_list.is_empty() {
            let x_mean = aim_x_list.iter().sum::<i32>() as f64 / aim_x_list.len() as f64;
            let y_mean = aim_y_list.iter().sum::<i32>() as f64 / aim_y_list.len() as f64;
            (x_mean, y_mean)
        } else {
            (0.0, 0.0)
        }
    }

    //计算瞄准环值
    fn aim_ring(&self, aim_x_list: &Vec<i32>, aim_y_list: &Vec<i32>) -> i32 {
        let (aim_x, aim_y) = self.aim_axis(aim_x_list, aim_y_list);
        let ring_aim = self.ring(aim_x, aim_y);
        ring_aim
    }

    //计算击中环值
    fn shoot_ring(&self, shoot_x: i32, shoot_y: i32) -> i32 {
        let ring_shoot = self.ring(shoot_x, shoot_y);
        ring_shoot
    }

    // 计算持枪晃动
    fn shake(&self, aim_x_list: &Vec<i32>, aim_y_list: &Vec<i32>) -> i32 {
        let (aim_x, aim_y) = self.aim_axis(aim_x_list, aim_y_list);
        let shake = (aim_x_list
            .iter()
            .zip(aim_y_list.iter())
            .map(|(x, y)| (((*x as f64 - aim_x).powi(2)) + ((*y as f64 - aim_y).powi(2))).sqrt())
            .sum::<f64>()
            / aim_x_list.len() as f64
            * self.k) as i32;
        match shake {
            0..=254 => shake,
            _ => 255,
        }
    }

    // 计算晃动速率
    fn shake_v(&self, aim_x_list: &Vec<i32>, aim_y_list: &Vec<i32>) -> i32 {
        let shake_v = (aim_x_list
            .iter()
            .zip(aim_y_list.iter())
            .map(|(x, y)| {
                (((*x as f64 - self.center_x as f64).powi(2))
                    + ((*y as f64 - self.center_y as f64).powi(2)))
                .sqrt()
            })
            .sum::<f64>()
            * self.k) as i32;
        match shake_v {
            0..=254 => shake_v,
            _ => 255,
        }
    }

    // 计算击发晃动量
    fn shoot_shake(
        &self,
        aim_x_list: &Vec<i32>,
        aim_y_list: &Vec<i32>,
        shoot_x: i32,
        shoot_y: i32,
    ) -> i32 {
        let shoot_shake = (aim_x_list
            .iter()
            .zip(aim_y_list.iter())
            .map(|(x, y)| {
                (((*x as f64 - shoot_x as f64).powi(2)) + ((*y as f64 - shoot_y as f64).powi(2)))
                    .sqrt()
            })
            .fold(f64::NEG_INFINITY, f64::max)
            * self.k) as i32;
        match shoot_shake {
            0..=254 => shoot_shake,
            _ => 255,
        }
    }

    // 计算击发速率
    fn shoot_shake_v(
        &self,
        aim_x_list: &Vec<i32>,
        aim_y_list: &Vec<i32>,
        shoot_x: i32,
        shoot_y: i32,
    ) -> i32 {
        shoot_shake(aim_x_list, aim_y_list, shoot_x, shoot_y)
    }
}
