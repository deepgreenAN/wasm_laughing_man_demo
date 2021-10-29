<script lang="ts">
    export let open = false
</script>

<div class="expand-sync-bar" class:open></div>

<style>
    .expand-sync-bar {
        background-color: rgba(0, 0, 0, 0);
    }

    @media (min-width: 768px) {
        .expand-sync-bar {
            width: 0rem;
            height: 100%;
            transition-property: width;
            transition-duration: 0.3s;
            transition-timing-function: ease-in-out;
        }
        .open {
            width: 20rem;
        } 
    }


</style>