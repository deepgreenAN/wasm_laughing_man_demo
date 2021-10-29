import laughing_man_svg_path from '../assets/laughing-man.svg';
import init, { init_video, CanvasApp } from "../../wasm/pkg";
import type { IAppOption, IModifyOption} from "../../wasm/pkg";

interface GlobalWasmApp {  // 各コンポーネントで利用するcanvas_appのインターフェース
    canvas_app: CanvasApp | null,
    interval_id: number | null,
    interval_time: number
}


const default_opt: IAppOption = {  // デフォルトのパラメーター
    video_canvas_parent_id: "canvas-video-parent",
    canvas_app_id: "canvas-app",
    make_image_canvas_id: "video-to-image",
    video_id: "stream-video",
    model_url: "https://dl.dropboxusercontent.com/s/ypb7jrufzgghp62/seeta_fd_frontal_v1.0.bin",
    laughing_man_svg_path: laughing_man_svg_path,
    image_over_video_scale: 0.2,
    min_face_size: 40,
    score_thresh: 2.0,
    pyramid_scale_factor: 0.5,
    slide_window_step: 4,
    is_active_laughing_man: true,
    allowable_not_detect_count: 4,
    laughing_man_size_ratio: 1.0,
    laughing_man_shift_ratio: 0,
    laughing_man_z_index: 20
};

const default_interval_time = 100;

let global_wasm_app: GlobalWasmApp = {  // このオブジェクトを各コンポーネントから参照する
    canvas_app: null,
    interval_id: null,
    interval_time: default_interval_time
}

const initialize_video = async () => {  // iport の初期化とビデオの初期化
    await init();
    await init_video(default_opt);
};

const initialize_canvas_app = async () => { // これが終った後，canvas_appを参照できる
    global_wasm_app.canvas_app = await new CanvasApp(default_opt);
}

export {default_opt, default_interval_time, global_wasm_app, initialize_video, initialize_canvas_app};
export type {IAppOption, IModifyOption, GlobalWasmApp};