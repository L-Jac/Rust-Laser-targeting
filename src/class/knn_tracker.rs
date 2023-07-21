use opencv::{core, imgproc as cv2, prelude::*, types, video, Error};

pub struct KNN {
    gray: core::Mat,
    blur: core::Mat,
    fg_mask: core::Mat,
    thresh1: core::Mat,
    thresh2: core::Mat,
    erode_kernel: core::Mat,
    dilate_kernel: core::Mat,
    k_size: core::Size_<i32>,
    cents: types::VectorOfVectorOfPoint,
    knn: types::PtrOfBackgroundSubtractorKNN,
}

impl KNN {
    pub fn new() -> Result<KNN, Error> {
        let k_size = core::Size_::new(7, 7);
        let mut cents = types::VectorOfVectorOfPoint::new();
        let mut knn = video::create_background_subtractor_knn(500, 200.0, false)?;
        let erode_kernel = cv2::get_structuring_element(
            cv2::MORPH_ELLIPSE,
            core::Size::new(7, 5),
            core::Point::new(-1, -1),
        )?;
        let dilate_kernel = cv2::get_structuring_element(
            cv2::MORPH_ELLIPSE,
            core::Size::new(17, 11),
            core::Point::new(-1, -1),
        )?;

        Ok(KNN {
            gray: core::Mat::default(),
            blur: core::Mat::default(),
            fg_mask: core::Mat::default(),
            thresh1: core::Mat::default(),
            thresh2: core::Mat::default(),
            erode_kernel,
            dilate_kernel,
            k_size,
            cents,
            knn,
        })
    }

    pub fn program(&mut self, frame: &core::Mat) -> Result<types::VectorOfVectorOfPoint, Error> {
        cv2::cvt_color(&frame, &mut self.gray, cv2::COLOR_BGR2GRAY, 0)?;
        cv2::gaussian_blur(
            &self.gray,
            &mut self.blur,
            self.k_size,
            0.0,
            0.0,
            core::BORDER_DEFAULT,
        )?;
        self.knn.apply(&self.blur, &mut self.fg_mask, -1.0)?;
        cv2::threshold(
            &self.fg_mask,
            &mut self.thresh1,
            244.0,
            255.0,
            cv2::THRESH_BINARY,
        )?;
        cv2::erode(
            &self.thresh1,
            &mut self.thresh2,
            &self.erode_kernel,
            core::Point::new(-1, -1),
            1,
            core::BORDER_CONSTANT,
            cv2::morphology_default_border_value()?,
        )?;
        cv2::dilate(
            &self.thresh2,
            &mut self.thresh1,
            &self.erode_kernel,
            core::Point::new(-1, -1),
            1,
            core::BORDER_CONSTANT,
            cv2::morphology_default_border_value()?,
        )?;
        cv2::find_contours(
            &self.thresh1,
            &mut self.cents,
            cv2::RETR_EXTERNAL,
            cv2::CHAIN_APPROX_SIMPLE,
            core::Point::new(0, 0),
        )?;
        Ok(self.cents.clone())
    }
}
