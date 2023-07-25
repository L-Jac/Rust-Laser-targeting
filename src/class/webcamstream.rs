use opencv::{core, prelude::*, videoio as cv2, Error};

pub struct WebcamStream {
    videocapture: cv2::VideoCapture,
    pub workstatue: bool,
    frame: core::Mat,
}

impl WebcamStream {
    pub fn new(stream_id: i32) -> Result<WebcamStream, Error> {
        let mut videocapture = cv2::VideoCapture::new(stream_id, cv2::CAP_ANY)?;
        // ...
        let mut frame = Mat::default();
        match videocapture.read(&mut frame)? {
            true => Ok(WebcamStream {
                videocapture,
                workstatue: true,
                frame,
            }),
            _ => {
                println!("[Exiting]: 读取帧时出错");
                std::process::exit(0);
            }
        }
    }

    // 读取一帧
    pub fn update_frame(&mut self) -> Result<Mat, Error> {
        match self.videocapture.read(&mut self.frame)? {
            true => Ok(self.frame.clone()),
            _ => {
                self.workstatue = false;
                println!("[Exiting]: 读取帧时出错");
                std::process::exit(0);
            }
        }
    }

    // 停止
    pub fn stop(&mut self) {
        self.workstatue = false;
    }
}
