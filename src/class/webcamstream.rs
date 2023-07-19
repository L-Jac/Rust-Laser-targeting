use opencv::{
    core,
    prelude::*,
    videoio as cv2,
    // videoio::{
    //     VideoCapture, VideoCaptureTrait, CAP_PROP_FPS, CAP_PROP_FRAME_HEIGHT, CAP_PROP_FRAME_WIDTH,
    // },
};

struct WebcamStream {
    stream_id: i32,
    videocapture: cv2::VideoCapture,
    workstatue: bool,
    frame: core::Mat,
}

impl WebcamStream {
    fn new(stream_id: i32) -> WebcamStream {
        let mut videocapture = cv2::VideoCapture::new(stream_id, cv2::CAP_ANY)?;
        // 宽度设置为 640 像素，高度设置为 480 像素，帧率设置为 30 帧/秒
        videocapture.set(cv2::CAP_PROP_FRAME_WIDTH, 640.0)?;
        videocapture.set(cv2::CAP_PROP_FRAME_HEIGHT, 480.0)?;
        videocapture.set(cv2::CAP_PROP_FPS, 30.0)?;
        // 检查视频流是否成功打开，并将其打印实际帧率
        match videocapture.is_opened()? {
            true => (),
            _ => {
                println!("[Exiting]: 访问网络摄像头流时出错");
                std::process::exit(0);
            }
        };
        let fps_input_stream = videocapture.get(cv2::CAP_PROP_FPS)? as i32;
        println!("摄像头帧率: {}", fps_input_stream);
        // 检测能否正常读取
        let frame = Mat::default()?;
        match videocapture.read(&mut frame)? {
            true => WebcamStream {
                stream_id,
                videocapture,
                workstatue: true,
                frame,
            },
            _ => {
                println!("[Exiting]: 读取帧时出错");
                std::process::exit(0);
            }
        };
    }

    // 读取一帧
    fn update_frame(&mut self) -> None {
        match videocapture.read(&mut self.frame)? {
            true => (),
            _ => {
                self.workstatue = false;
                println!("[Exiting]: 读取帧时出错");
                break;
            }
        };
    }

    // 停止
    fn stop(&mut self) -> None {
        self.workstatue = false;
    }
}
