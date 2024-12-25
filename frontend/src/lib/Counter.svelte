<script lang="ts">
    import { getLocation } from "../stores/location.svelte";
    let {
        path,
    }: {
        path: string;
    } = $props();

    let location = getLocation();
    const increment = () => {
        location.goTo(path);
    };
</script>

<button onclick={increment}>
    {path}
</button>

<style lang="scss"></style>
