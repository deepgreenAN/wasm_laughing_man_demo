<script lang="ts">
    import {global_wasm_app, default_opt, default_interval_time} from './wasm_app';
    import type {IModifyOption} from './wasm_app';

    export let open = false

    let interval_time = global_wasm_app.interval_time;
    let last_is_active_laughing_man = true;

    let modify_option: IModifyOption = {  // デフォルトの値で初期化
        image_over_video_scale: default_opt.image_over_video_scale,
        min_face_size: default_opt.min_face_size,
        score_thresh: default_opt.score_thresh,
        pyramid_scale_factor: default_opt.pyramid_scale_factor,
        slide_window_step: default_opt.slide_window_step,
        is_active_laughing_man: default_opt.is_active_laughing_man,
        allowable_not_detect_count: default_opt.allowable_not_detect_count,
        laughing_man_size_ratio: default_opt.laughing_man_size_ratio,
        laughing_man_shift_ratio: default_opt.laughing_man_shift_ratio
    };

    let default_assign = () => {
        modify_option = {...default_opt};
        interval_time = default_interval_time;
    };

    let apply_to_wasm_app = () => {
        if (last_is_active_laughing_man && !modify_option.is_active_laughing_man){  // 笑い男モード->非笑い男モード
            global_wasm_app.canvas_app.remove_all_rois_element();
        } else if (!last_is_active_laughing_man && modify_option.is_active_laughing_man){  // 非笑い男モード->笑い男モード
            global_wasm_app.canvas_app.add_all_rois_element();
        };


        global_wasm_app.canvas_app.set_option_param(modify_option);
        last_is_active_laughing_man = modify_option.is_active_laughing_man;

        window.clearInterval(global_wasm_app.interval_id);
        global_wasm_app.interval_id = window.setInterval(()=>{
            global_wasm_app.canvas_app.step();
        }, interval_time);
        global_wasm_app.interval_time = interval_time;
    };
</script>

<aside class="expand-side-bar" class:open>
	<div class="side-bar-content">
        <label>
            ビデオサイズに対する画像比[0.1, 1]:
            <input type=number min=0.1 max=1 step=0.01 bind:value={modify_option.image_over_video_scale}>
        </label>
        <label>
            最小顔サイズ[20, ):
            <input type=number min=20 step=1 bind:value={modify_option.min_face_size}>
        </label>
        <label>
            スコア閾値(0, ):
            <input type=number min=0.01 bind:value={modify_option.score_thresh}>
        </label>
        <label>
            ピラミッドスケール[0.01, 0.99]:
            <input type=number min=0.01 max=0.99 step=0.01 bind:value={modify_option.pyramid_scale_factor}>
        </label>
        <label>
            スライドウインドウステップサイズ[1, ):
            <input type=number min=1 step=1 bind:value={modify_option.slide_window_step}>
        </label>
        <label>
            笑い男モード:
            <input type=checkbox bind:checked={modify_option.is_active_laughing_man}>
        </label>
        <label>
            トラッキング許容カウント[1, ):
            <input type=number min=1 bind:value={modify_option.allowable_not_detect_count}>
        </label>
        <label>笑い男拡大係数(0, ):
            <input type=number min=0 step=0.01 bind:value={modify_option.laughing_man_size_ratio}>
        </label>
        <label>笑い男シフト係数( , ):
            <input type=number step=0.01 bind:value={modify_option.laughing_man_shift_ratio}>
        </label>
        <label>
            描画インターバル時間(10, ):
            <input type=number step=10 bind:value={interval_time}>
        </label>
        <div id="default-apply-button">
            <button id="default-button" on:click={e => {default_assign()}}> デフォルト </button>
            <button id="apply-button" on:click={e => {apply_to_wasm_app()}}> 適用 </button>
        </div>
        <a href="https://gist.github.com/johan/1066590" target="_blank" rel="noopener noreferrer">画像引用元(Johan Sundströmさん)</a>
        <div id="library-link">
            顔認識には
            <a href="https://github.com/atomashpolskiy/rustface" target="_blank" rel="noopener noreferrer">rustface</a>
            を利用しています(元ライブラリ
            <a href="https://github.com/seetaface/SeetaFaceEngine/tree/master/FaceDetection" target="_blank" rel="noopener noreferrer">Seetaface</a>
            )
        </div>
    </div>
</aside>

<style>
    aside {
        background-color: #12b0c4;
    }

    .side-bar-content {
        display: flex;
        flex-direction: column;
        align-items: center;
    }

    @media (min-width:768px) {
        aside {
            position: absolute;
            width: 20rem;
            height: 100%;
            right: -20rem;
            top: 0px;
            transition-property: right;
            transition-duration: 0.3s;
            transition-timing-function: ease-in-out;
            z-index: 30;
        }
        .side-bar-content {
            padding-top: 4rem;
        }
        .open {
            right: 0;
        }
    }
	
    .side-bar-content input {
        width: 3rem;
    }

    .side-bar-content label {
        font-size: 0.8rem;
        margin-top: 0.2rem;
    }

    #default-apply-button {
        margin-top: 0.5rem;
        width: 10rem;
        display: flex;
        flex-direction: row;
        align-items: center;
        justify-content: space-around;
    }

    .side-bar-content a {
        margin-top: 1rem;
    }

    #library-link {
        margin-top: 1rem;
        font-size: 0.8rem;
    }
</style>