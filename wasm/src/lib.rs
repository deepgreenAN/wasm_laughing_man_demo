pub mod utils;
pub mod dom_utils;
pub mod canvas_app;
pub mod face_detection;
pub mod tracker_roi;
pub mod tracker;


// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
