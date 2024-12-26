<script lang="ts">
    import ToolTip from "../../lib/ToolTip.svelte";
    let {
        missingLanguages,
        onclick
    }:{
        onclick:()=>void;
        missingLanguages: string[];
    } = $props();
</script>
<div class="auto-translate">
    <ToolTip openTimout={200}>
        <i class="ri-error-warning-line" aria-label="Warning some missing translations"></i>
        {#snippet content()}
            <p>Missing translations for:</p>
            <ul>
                {#each missingLanguages as lang}
                    <li>{lang}</li>
                {/each}
            </ul>
        {/snippet}
    </ToolTip>

    <ToolTip openTimout={1000}>
        <button aria-label="Auto translate" {onclick}>
            <i class="ri-ai-generate-2"></i>
        </button>
        {#snippet content()}
            <p>Click to auto translate</p>
        {/snippet}
    </ToolTip>
</div>

<style lang="scss">
    .auto-translate {
        display: flex;
        justify-content: center;
        align-items: center;
    }
</style>