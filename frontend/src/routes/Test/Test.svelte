<script lang="ts">
    import type { SvelteComponent } from "svelte";
    import Route from "../../lib/Route.svelte";
    import Inp from "./Comp.svelte";
    import Wpr from "./Wpr.svelte";
    let c:SvelteComponent;
</script>


<Route path="/test">
    <Wpr bind:c={c}>
        <Inp bind:this={c}></Inp>
    </Wpr>
</Route>
