<script lang="ts">
    import ButtonGroup from "../../../../lib/ButtonGroup.svelte";
    let {
        title = $bindable(),
        value = $bindable(),
        options = $bindable(),
    }:{
        title:string,
        value:string,
        options:string[],
    } = $props();
</script>

<h2>{title}</h2>
<ButtonGroup bind:options bind:value>
    {#snippet optionRenderer(v)}
        {v}
    {/snippet}
</ButtonGroup>