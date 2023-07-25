mod module {
    mod detector;
    mod knn_tracker;
    pub mod state_machine;
    mod webcamstream;
}

use module::state_machine::StateMachine;
use opencv::{core, highgui, imgproc};

fn main() {
    let mut project = StateMachine::new();
    loop {
        let mut newframe = project.webcamstream.update_frame().unwrap();
        project.detect.catch_center(&newframe);
        let points: Vec<core::Point> = project
            .detect
            .center_list
            .clone()
            .into_iter()
            .map(|(x, y)| core::Point::new(x, y))
            .collect();
        let color = opencv::core::Scalar::new(255.0, 0.0, 0.0, 0.0);
        for window in points.windows(2) {
            let _ = imgproc::line(
                &mut newframe,
                window[1],
                window[0],
                color,
                1,
                imgproc::LINE_8,
                0,
            );
        }

        // for i in 1..points.len() {
        //     let color = opencv::core::Scalar::new(255.0, 0.0, 0.0, 0.0);
        //     let _ = imgproc::line(
        //         &mut newframe,
        //         points[i],
        //         points[i - 1],
        //         color,
        //         1,
        //         imgproc::LINE_8,
        //         0,
        //     );
        // }

        let _ = highgui::imshow("TestShow", &newframe);
        let key = highgui::wait_key(10).unwrap();
        if key == 27 {
            break;
        }
    }

    let _ = highgui::destroy_all_windows();
}
