<script lang="ts">
    import { getTranslator } from "../../stores/translate.svelte";
    import MultiSelect from "../../lib/MultiSelect.svelte";
    let value = $state('')
    let t = getTranslator();
</script>
<div class="search-bar">
    <MultiSelect options={["all", "phones"]} value="all" multiple={false}>
        {#snippet valueRenderer(v)}
            {v}
        {/snippet}
        {#snippet optionRenderer(v)}
            {v}
        {/snippet}
    </MultiSelect>
    <input placeholder="{t.t("blue_alert_scallop_hope")}" type="text" bind:value="{value}">
    <button aria-label="{t.t("careful_awful_myna_scold")}">
        <i class="ri-search-line"></i>
    </button>
</div>

<style lang="scss">
    .search-bar{
        display: flex;
        border: 1px solid var(--neutral-6);
        --bg-color:var(--neutral-3);
        margin: 1rem;
        border-radius: 3px;
        &:focus-within{
            border: 1px solid var(--main-10);
            input{
                border-color: var(--main-10);
            }
        }
        input{
            flex:1;
            outline: none;
            border: none;
            color: var(--neutral-12);
            background-color: var(--neutral-2);
            border-left: 1px solid var(--neutral-6);
            padding-left: 0.5rem;
        }
        button{
            font-size: 1.2rem;
            width: 50px;
            outline: none;
            border: none;
            border-radius: 0 2px 2px 0;
            background-color: var(--neutral-3);
            color: var(--neutral-11);
        }
        :global(.multi-select .selected){
            border: none;
            &:hover{
                border: none;
            }
        }
        :global(.multi-select){
            border-radius: 2px 0 0 2px;
            margin-top: 0;
            min-width: 200px;
        }
    }
</style>