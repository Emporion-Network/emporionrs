<script lang="ts">
    import type { SvelteComponent } from "svelte";
    import Route from "../../lib/Route.svelte";
    import Inp from "./Comp.svelte";
    let c:SvelteComponent;
</script>


<Route path="/test">
    <Inp bind:this={c} value="FFFFFF"></Inp>
</Route>
