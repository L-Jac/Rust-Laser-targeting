use super::detector::Detector;
use super::webcamstream::WebcamStream;

pub struct StateMachine {
    pub detect: Detector,
    pub webcamstream: WebcamStream,
}

impl StateMachine {
    pub fn new() -> StateMachine {
        StateMachine {
            detect: Detector::new(false),
            webcamstream: WebcamStream::new(1).unwrap(),
        }
    }
}
