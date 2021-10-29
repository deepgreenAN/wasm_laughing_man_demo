<script lang="ts">
    import Hanburger from './Hamburger.svelte'
    import logo_path from '../assets/laughing-man.svg'
    import logo_text_path from '../assets/logo_text.svg'

    export let param_open = false;
</script>

<div id="header">
    <div id="logo-container">
        <img id="logo-svg" src={logo_path} alt="laghing man logo">
        <img id="logo-text-svg" src={logo_text_path} alt="logo text">
    </div>
    <div id="hanburger-container">
        <div id="hanburger-text"> パラメーター </div>
        <Hanburger bind:open={param_open}/>
    </div>
</div>


<style>
    #header {
        background: linear-gradient(to top, cyan, 30%, black);
        height: 5rem;
        width: 100%;
        display: flex;
        flex-direction: row;
        justify-content: space-between;
        align-items: center;
    }

    #logo-container {
        display: flex;
        justify-content: start;
        align-items: center;
    }

    #logo-svg {
        height: 4.5rem;
    }

    #logo-text-svg {
        height: 3.0rem;
        margin-left: 0.5rem;
    }

    #hanburger-container {
        display: flex;
        justify-content: end;
        align-items: center;
    }

    #hanburger-text {
        color: #ededed;
        z-index: 40;
    }

    @media (min-width: 320px) and (max-width: 768px) {
        #hanburger-text {
            visibility: hidden;
        } 
    }
    @media (min-width: 768px) {
        #logo-svg {
            margin-left: 4rem;
        }
    }
</style>