<script lang="ts">
    import {global_wasm_app, initialize_video, initialize_canvas_app} from './wasm_app'

    export let header_height: number;

    const svelte_initialize_wasm = async () => {
        await initialize_video();

        // videoの元サイズを取得
        const stream_video_element = document.getElementById("stream-video") as HTMLVideoElement;
        const video_width = stream_video_element.videoWidth;
        const video_height = stream_video_element.videoHeight;
        const video_width_over_height = video_width / video_height;

        // ビデオ・キャンバスのサイズを計算
        //const screen_height = screen.height;
        //const screen_height = screen.availHeight;
        const screen_height = window.innerHeight;
        const screen_width = window.innerWidth;

        let video_show_height = 0;
        let video_show_width = 0;

        if (video_width > video_height) {
            video_show_height = screen_height - header_height;
            video_show_width = video_show_height * video_width_over_height;
        } else {
            video_show_width = screen_width;
            video_show_height = video_show_width * (1 / video_width_over_height)
        }


        stream_video_element.width = video_show_width;
        stream_video_element.height = video_show_height;

        const app_canvas_element = document.getElementById("canvas-app") as HTMLCanvasElement;
        app_canvas_element.width = video_show_width;
        app_canvas_element.height = video_show_height;

        const canvas_video_parent_element = document.getElementById("canvas-video-parent");
        canvas_video_parent_element.style.width = `${video_show_width}px`;
        canvas_video_parent_element.style.height = `${video_show_height}px`;

        await initialize_canvas_app();

        global_wasm_app.interval_id = window.setInterval(()=>{
            global_wasm_app.canvas_app.step()
        }, global_wasm_app.interval_time);
    };
 
</script>

<div id="canvas-app-container">
    <div id="canvas-video-parent">
        <video id="stream-video" width="100px" height="100px"><track kind="captions"></video>
        <canvas id="canvas-app" width="100px" height="100px"></canvas>
        <canvas id="video-to-image" style="display:none"></canvas>
        <!-- <canvas id="video-to-image"></canvas> -->
    </div>
    {#await svelte_initialize_wasm()}
    <div id="waiting-text">...メディアの利用許可とモデルの読み込みを待っています</div>
    {:then x} 
    <!-- <div></div> -->
    {/await}
</div>


<style>
    #canvas-app-container {
        display:flex;
        flex-direction: row-reverse;
        margin-left: auto;
        margin-right: auto;
    }

    #canvas-video-parent {
        position:relative;
    }
    #stream-video {
        position:absolute;
        top:0px;
        left:0px;
        z-index: 10;
    }
    #canvas-app {
        position:absolute;
        top:0px;
        left:0px;
        z-index: 20;
    }
</style>

