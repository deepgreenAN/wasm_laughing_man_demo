/* tslint:disable */
/* eslint-disable */
/**
*/
export function set_panic_hook(): void;
/**
* @param {IAppOption} app_opt
* @returns {Promise<void>}
*/
export function init_video(app_opt: IAppOption): Promise<void>;

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


/**
*/
export class AppOption {
  free(): void;
}
/**
*/
export class CanvasApp {
  free(): void;
/**
* @param {IAppOption} app_opt
*/
  constructor(app_opt: IAppOption);
/**
*/
  step(): void;
/**
*/
  remove_all_rois_element(): void;
/**
*/
  add_all_rois_element(): void;
/**
* @param {IModifyOption} modify_opt
*/
  set_option_param(modify_opt: IModifyOption): void;
/**
*/
  video_show_size_change_hook(): void;
}
/**
*/
export class ModifyOption {
  free(): void;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly set_panic_hook: () => void;
  readonly __wbg_appoption_free: (a: number) => void;
  readonly __wbg_modifyoption_free: (a: number) => void;
  readonly init_video: (a: number) => number;
  readonly __wbg_canvasapp_free: (a: number) => void;
  readonly canvasapp_new: (a: number) => number;
  readonly canvasapp_step: (a: number) => void;
  readonly canvasapp_remove_all_rois_element: (a: number) => void;
  readonly canvasapp_add_all_rois_element: (a: number) => void;
  readonly canvasapp_set_option_param: (a: number, b: number) => void;
  readonly canvasapp_video_show_size_change_hook: (a: number) => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly _dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__ha8df210d52fe841e: (a: number, b: number, c: number) => void;
  readonly __wbindgen_free: (a: number, b: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly wasm_bindgen__convert__closures__invoke2_mut__h00aa7fb92f4cd1ba: (a: number, b: number, c: number, d: number) => void;
  readonly __wbindgen_start: () => void;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
