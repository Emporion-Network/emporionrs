<script lang="ts">
    import { onMount, type SvelteComponent, type Snippet } from "svelte";

    let {
        c=$bindable(),
        children,
    }:{
        children:Snippet<[]>
        c:SvelteComponent,
    } = $props();

    onMount(()=>{
        c.v = "hooo";
    })
</script>

<div class="ex">
    {@render children()}
</div>