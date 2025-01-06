<script lang="ts">
    import type { SvelteComponent } from "svelte";
    import Route from "../../lib/Route.svelte";
    import Inp from "./Comp.svelte";
    import ContextMenu from "../../lib/ContextMenu.svelte";
    let c: SvelteComponent;
</script>

<Route path="/test">
    <div>
        <ContextMenu>
            {#snippet opener(p)}
                <button bind:this={p.ref} {...p}>
                    <i class="ri-more-line"></i>
                </button>
            {/snippet}
            {#snippet options(close)}
                <button onclick={()=>{console.log('heee');close()}}>Clone</button>
                <button>Edit</button>
                <button>Disable</button>
                <button>Show</button>
            {/snippet}
        </ContextMenu>
    </div>
</Route>

<style lang="scss">
    div {
        min-height: 100vh;
        display: flex;
        justify-content: center;
        align-items: center;
        i {
            color: wheat;
        }
    }
</style>
