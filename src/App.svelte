<script lang="ts">
  import WasmApp from './lib/WasmApp.svelte'
  import Sidbar from './lib/Sidebar.svelte'
  import Header from './lib/Header.svelte'
  import Sidebar from './lib/Sidebar.svelte'
  import SidebarSync from './lib/SidebarSync.svelte'

  let param_open = false
  let header_height = 0;
</script>

<div class="container">
  <div bind:clientHeight={header_height}>
    <Header bind:param_open={param_open}/>
  </div>
  <Sidebar open={param_open}/>
  <div id=wasm-container>
    <WasmApp header_height={header_height}/>
    <SidebarSync open={param_open}/>
  </div>
</div>

<style>
  :global(body) {
    margin:0px;
    padding:0px;
    overflow-x: hidden;
  }
  .container {
    overflow-y: hidden; 
  }

  #wasm-container {
    display: flex;
    flex-direction: row;
    justify-content: space-between;
  }
</style>


