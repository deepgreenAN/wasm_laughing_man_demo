use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsValue, JsCast};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Document, HtmlElement, HtmlImageElement, HtmlVideoElement, MediaStream, MediaStreamConstraints, Node, Performance};
use js_sys::Error;
use serde::{Deserialize};
use rustface::{Detector, read_model, Rectangle};
use bytes::Buf;

use crate::dom_utils::{
    canvas, 
    context2d, 
    get_element_by_id, 
    navigator, 
    window,
    document
};
use crate::face_detection::{convert_rgba_to_luma_v2, detect_faces};
use crate::tracker::Tracker;


#[wasm_bindgen(typescript_custom_section)]
const APP_OPTION: &'static str = r#"
interface IAppOption {
    video_canvas_parent_id: string,
    canvas_app_id: string,
    make_image_canvas_id: string,
    video_id: string,
    model_url: string,
    laughing_man_svg_path: string,
    image_over_video_scale: number,
    min_face_size: number,
    score_thresh: number,
    pyramid_scale_factor: number,
    slide_window_step: number,
    is_active_laughing_man: boolean,
    allowable_not_detect_count: number,
    laughing_man_size_ratio: number,
    laughing_man_shift_ratio: number,
    laughing_man_z_index: number
}
"#;

#[wasm_bindgen(typescript_custom_section)]
const MODIFY_OPTION: &'static str = r#"
interface IModifyOption {
    image_over_video_scale: number,
    min_face_size: number,
    score_thresh: number,
    pyramid_scale_factor: number,
    slide_window_step: number,
    is_active_laughing_man: boolean,
    allowable_not_detect_count: number,
    laughing_man_size_ratio: number,
    laughing_man_shift_ratio: number
}
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "IAppOption")]
    pub type IAppOption;

    #[wasm_bindgen(typescript_type = "IModifyOption")]
    pub type IModifyOption;
}

#[wasm_bindgen]
#[derive(Deserialize)]
struct AppOption {  // 型をはっきりさせるため
    video_canvas_parent_id: String,
    canvas_app_id: String,
    make_image_canvas_id: String,
    video_id: String,
    model_url: String,
    laughing_man_svg_path: String,
    image_over_video_scale: f64,
    min_face_size: u32,
    score_thresh: f64,
    pyramid_scale_factor: f32,
    slide_window_step: u32,
    is_active_laughing_man: bool,
    allowable_not_detect_count: u32,
    laughing_man_size_ratio: f64,
    laughing_man_shift_ratio: f64,
    laughing_man_z_index: u32
}

impl AppOption {
    fn new(iapp_option: IAppOption) -> Result<Self, JsValue> {
        iapp_option.obj.into_serde::<AppOption>().map_err(|_|{
            Error::new("app option into_serde error").into()
        })
    }
}

#[wasm_bindgen]
#[derive(Deserialize)]
struct ModifyOption {  // 型をはっきりさせるため
    image_over_video_scale: f64,
    min_face_size: u32,
    score_thresh: f64,
    pyramid_scale_factor: f32,
    slide_window_step: u32,
    is_active_laughing_man: bool,
    allowable_not_detect_count: u32,
    laughing_man_size_ratio: f64,
    laughing_man_shift_ratio: f64
}

impl ModifyOption {
    fn new(imodify_option: IModifyOption) -> Result<Self, JsValue> {
        imodify_option.obj.into_serde::<ModifyOption>().map_err(|_|{
            Error::new("modify option into_serde eror").into()
        })
    }
}

#[wasm_bindgen]
pub async fn init_video(app_opt: IAppOption) -> Result<(), JsValue>{
    let app_opt = AppOption::new(app_opt)?;
    let video: HtmlVideoElement = get_element_by_id::<HtmlVideoElement>(&app_opt.video_id)?;

    let mut media_constraints = MediaStreamConstraints::new();
    media_constraints.audio(&JsValue::FALSE);
    media_constraints.video(&JsValue::TRUE);

    let stream_promise = navigator()?
        .media_devices()?
        .get_user_media_with_constraints(&media_constraints)?;
    
    let stream = JsFuture::from(stream_promise).await?
        .dyn_into::<MediaStream>()?;

    video.set_src_object(Some(&stream));
    JsFuture::from(video.play()?).await?;
    Ok(())
}

pub async fn get_detecor(url: String) -> Result<Box<dyn rustface::Detector>, JsValue>{
    let res = reqwest_wasm::get(&url).await
        .map_err(|_|{JsValue::from(Error::new("detector model request error"))})?;
    let res_bytes = res.bytes().await
        .map_err(|_|{JsValue::from(Error::new("model requests cannot get bytes error"))})?;

    let model = read_model(res_bytes.reader())
        .map_err(|_|{JsValue::from(Error::new("cannot read model from bytes from request"))})?;
    Ok(rustface::create_detector_with_model(model))
}

#[wasm_bindgen]
pub struct CanvasApp {
    document: Document,
    app_context: web_sys::CanvasRenderingContext2d,
    make_image_context: web_sys::CanvasRenderingContext2d,
    stream_video: HtmlVideoElement,
    video_canvas_parent_node: Node,
    laughing_man_svg_path: String,
    video_show_width: u32,
    video_show_height: u32,
    image_over_video_show_scale: f64,
    image_width: u32,
    image_height: u32,
    detector: Box<dyn Detector>,
    performance: Performance,
    tracker: Tracker,
    is_active_laughing_man: bool,
    laughing_man_size_ratio: f64,
    laughing_man_shift_ratio: f64,
    laughing_man_z_index: u32

}

#[wasm_bindgen]
impl CanvasApp {
    #[wasm_bindgen(constructor)]
    pub async fn new(app_opt: IAppOption) -> Result<CanvasApp, JsValue> {
        let app_opt = AppOption::new(app_opt)?;
        // document
        let _document = document()?;

        // video
        let stream_video = get_element_by_id::<HtmlVideoElement>(&app_opt.video_id)?;

        let video_width = stream_video.video_width();  // 動画の元サイズ
        let video_height = stream_video.video_height();  // 動画の元サイズ

        let video_show_width = stream_video.width();  // 動画の表示サイズ
        let video_show_height = stream_video.height();  // 動画の表示サイズ

        let image_width = (video_width as f64 * app_opt.image_over_video_scale) as u32;
        let image_height = (video_height as f64 * app_opt.image_over_video_scale) as u32;

        // image_over_video_show_scale
        let image_over_video_show_scale = image_width as f64 / video_show_width as f64;

        // make_image_canvas
        let make_image_canvas = canvas(&app_opt.make_image_canvas_id)?;
        make_image_canvas.set_width(image_width);
        make_image_canvas.set_height(image_height);
        
        let make_image_context = context2d(&app_opt.make_image_canvas_id)?;
        
        // app_context
        let app_context = context2d(&app_opt.canvas_app_id)?;
        app_context.set_font("20px serif");

        // video_canvas_parent
        let video_canvas_parent = get_element_by_id::<HtmlElement>(&app_opt.video_canvas_parent_id)?;
        let video_canvas_parent_node = video_canvas_parent.dyn_into::<Node>()?;

        // detector
        let mut detector = get_detecor(app_opt.model_url).await?;

        detector.set_min_face_size(app_opt.min_face_size);
        detector.set_score_thresh(app_opt.score_thresh);
        detector.set_pyramid_scale_factor(app_opt.pyramid_scale_factor);
        detector.set_slide_window_step(app_opt.slide_window_step, app_opt.slide_window_step);

        // performance
        let performance = window()?
            .performance()
            .ok_or(JsValue::from(Error::new("cannot get performance from window")))?;
            

        // tracker
        let tracker = Tracker::new(app_opt.allowable_not_detect_count);

        Ok(Self {
            document:_document,
            app_context, 
            make_image_context, 
            stream_video,
            video_canvas_parent_node,
            laughing_man_svg_path: app_opt.laughing_man_svg_path,
            video_show_width,
            video_show_height,
            image_over_video_show_scale,
            image_width,
            image_height, 
            detector,
            performance,
            tracker,
            is_active_laughing_man: app_opt.is_active_laughing_man,
            laughing_man_size_ratio: app_opt.laughing_man_size_ratio,
            laughing_man_shift_ratio: app_opt.laughing_man_shift_ratio,
            laughing_man_z_index: app_opt.laughing_man_z_index
        })
    }

    #[wasm_bindgen]
    pub fn step(&mut self) -> Result<(), JsValue> {
        let start_time = self.performance.now();
        self.make_image_context.draw_image_with_html_video_element_and_dw_and_dh(
            &self.stream_video,
            0.0,
            0.0,
            self.image_width as f64,
            self.image_height as f64
        )?;

        let image_vec = self.make_image_context.get_image_data(
            0.0,
            0.0,
            self.image_width as f64,
            self.image_height as f64
        )?
        .data()
        .0;

        let gray_vec = convert_rgba_to_luma_v2(image_vec, self.image_width, self.image_height);

        let faces = detect_faces(&mut *self.detector, &gray_vec, self.image_width, self.image_height);
        let faces: Vec<Rectangle> = faces.iter().map(|face|{*face.bbox()}).collect();
        let face_number = faces.len();

        // tracking
        let (added_rois, removed_rois) = self.tracker.track(&faces);

        self.app_context.clear_rect(
            0.0,
            0.0,
            self.video_show_width as f64,
            self.video_show_height as f64
        );

        // domの追加・削除
        if self.is_active_laughing_man {
            for roi in added_rois {
                let img_element = self.document.create_element("img")?.dyn_into::<HtmlImageElement>()?;
                img_element.set_id(&format!("laughing-man-img-{}", roi.id));
                //img_element.set_width((roi.width * (1.0 / self.image_over_video_scale)) as u32);
                img_element.set_height((self.laughing_man_size_ratio * roi.height * (1.0 / self.image_over_video_show_scale)) as u32);
                let img_style = img_element.style();
                img_style.set_css_text(
                    &format!(
                        "position:absolute;top:{}px;left:{}px;z-index:{}",
                        ((self.laughing_man_shift_ratio * roi.height + roi.tl_y) * (1.0 / self.image_over_video_show_scale)) as u32,
                        ((self.laughing_man_shift_ratio * roi.width + roi.tl_x) * (1.0 / self.image_over_video_show_scale)) as u32,
                        self.laughing_man_z_index
                    )
                );
                img_element.set_src(&self.laughing_man_svg_path);
                let img_node = img_element.dyn_into::<Node>()?;
                self.video_canvas_parent_node.append_child(&img_node)?;
            }

            for roi in removed_rois {
                let img_element = get_element_by_id::<HtmlImageElement>(&format!("laughing-man-img-{}", roi.id))?;
                img_element.remove();
            }
        }

        // 描画
        for tracker_roi in self.tracker.rois.iter() {
            if !self.is_active_laughing_man {
                self.app_context.stroke_rect(
                    tracker_roi.tl_x * (1.0 / self.image_over_video_show_scale),
                    tracker_roi.tl_y * (1.0 / self.image_over_video_show_scale),
                    tracker_roi.width * (1.0 / self.image_over_video_show_scale),
                    tracker_roi.height * (1.0 / self.image_over_video_show_scale)
                );
                self.app_context.fill_text(
                    &format!("id:{}", tracker_roi.id),
                    tracker_roi.tl_x * (1.0 / self.image_over_video_show_scale),
                    tracker_roi.tl_y * (1.0 / self.image_over_video_show_scale),     
                ).map_err(|_|{JsValue::from(Error::new("fill text to canvas error"))})?;
            } else {
                let img_element = get_element_by_id::<HtmlImageElement>(&format!("laughing-man-img-{}", tracker_roi.id))?;
                //img_element.set_width((tracker_roi.width * (1.0 / self.image_over_video_scale)) as u32);
                img_element.set_height((self.laughing_man_size_ratio * tracker_roi.height * (1.0 / self.image_over_video_show_scale)) as u32);
                let img_style = img_element.style();
                img_style.set_property(
                    "top", 
                    &format!(
                        "{}px", 
                        ((self.laughing_man_shift_ratio * tracker_roi.height + tracker_roi.tl_y) * (1.0 / self.image_over_video_show_scale)) as u32)
                )?;
                img_style.set_property(
                    "left", 
                    &format!(
                        "{}px", 
                        ((self.laughing_man_shift_ratio * tracker_roi.width + tracker_roi.tl_x) * (1.0 / self.image_over_video_show_scale)) as u32
                    )
                )?;
            }
        }

        let text_info = format!("{:<3} faces detected in {:>5}[ms]", face_number, (self.performance.now() - start_time) as i32);
        let text_x = self.video_show_width as f64 * 0.02;
        let text_y = self.video_show_height as f64 * 0.05;

        self.app_context.fill_text(
            &text_info,
            text_x,
            text_y
        )?;

        Ok(())
        
    }

    #[wasm_bindgen]
    pub fn remove_all_rois_element(&mut self) -> Result<(), JsValue> {
        for roi in self.tracker.rois.iter() {
            let img_element = get_element_by_id::<HtmlImageElement>(&format!("laughing-man-img-{}", roi.id))?;
            img_element.remove();
        }
        Ok(())
    }

    #[wasm_bindgen]
    pub fn add_all_rois_element(&mut self) -> Result<(), JsValue> {
        for roi in self.tracker.rois.iter() {
            let img_element = self.document.create_element("img")?.dyn_into::<HtmlImageElement>()?;
            img_element.set_id(&format!("laughing-man-img-{}", roi.id));
            //img_element.set_width((roi.width * (1.0 / self.image_over_video_scale)) as u32);
            img_element.set_height((self.laughing_man_size_ratio * roi.height * (1.0 / self.image_over_video_show_scale)) as u32);
            let img_style = img_element.style();
            img_style.set_css_text(
                &format!(
                    "position:absolute;top:{}px;left:{}px;z-index:20",
                    ((self.laughing_man_shift_ratio * roi.height + roi.tl_y) * (1.0 / self.image_over_video_show_scale)) as u32,
                    ((self.laughing_man_shift_ratio * roi.width + roi.tl_x) * (1.0 / self.image_over_video_show_scale)) as u32
                )
            );
            img_element.set_src(&self.laughing_man_svg_path);
            let img_node = img_element.dyn_into::<Node>()?;
            self.video_canvas_parent_node.append_child(&img_node)?;
        }
        Ok(())
    }

    #[wasm_bindgen]
    pub fn set_option_param(&mut self, modify_opt: IModifyOption) -> Result<(), JsValue>{
        let modify_opt = ModifyOption::new(modify_opt)?;
        // make_image_canvas関連
        let video_width = self.stream_video.video_width();
        let video_height = self.stream_video.video_height();

        self.image_width = (video_width as f64 * modify_opt.image_over_video_scale) as u32;
        self.image_height = (video_height as f64 * modify_opt.image_over_video_scale) as u32;

        let make_image_canvas = self.make_image_context.canvas()
            .ok_or(JsValue::from(Error::new("not found")))?;
        make_image_canvas.set_width(self.image_width);
        make_image_canvas.set_height(self.image_height);

        self.detector.set_min_face_size(modify_opt.min_face_size);
        self.detector.set_score_thresh(modify_opt.score_thresh);
        self.detector.set_pyramid_scale_factor(modify_opt.pyramid_scale_factor);
        self.detector.set_slide_window_step(modify_opt.slide_window_step, modify_opt.slide_window_step);

        self.is_active_laughing_man = modify_opt.is_active_laughing_man;
        self.tracker.allowable_not_detect_count = modify_opt.allowable_not_detect_count;
        self.laughing_man_size_ratio = modify_opt.laughing_man_size_ratio;
        self.laughing_man_shift_ratio = modify_opt.laughing_man_shift_ratio;
        
        Ok(())
    }

    #[wasm_bindgen]
    pub fn video_show_size_change_hook(&mut self) -> Result<(), JsValue> {
        let video_show_width = self.stream_video.width();
        self.image_over_video_show_scale = self.image_width as f64 / video_show_width as f64;

        Ok(())
    }
}



