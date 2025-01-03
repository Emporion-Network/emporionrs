<script lang="ts">
    import type { SvelteComponent } from "svelte";
    import MultiSelect from "../../../lib/MultiSelect.svelte";
    import { getTranslator } from "../../../stores/translate.svelte";
    import { metas, type Attribute } from "./_metas";
    let t = getTranslator();
    let {
        attributeType = $bindable(),
        attributes = $bindable(),
        onpush,
    }:{
        attributeType:Attribute["display_type"],
        attributes:Attribute[],
        onpush?:() => void;
    } = $props();
    let el:HTMLElement = $state()!;
    let ms:SvelteComponent;

    const addAttribute = () => {
        attributes.push(metas[attributeType].defaultValue);
        onpush?.();
    };

    export const actions = {
        select(t:typeof attributeType){
            ms.actions.setValue(t)
        },
        add(){
            addAttribute()
        },
        open(){
            ms.actions.open();
        },
        clear(){
            ms.actions.setValue("buttons");
            attributes = [];
        }
    }

    export {
        el as element
    }
    
</script>
<div class="attribute-selector" bind:this={el}>
    <MultiSelect
        options={["buttons", "checkbox", "select"] as const}
        label={t.t("level_arable_robin_talk")}
        placeholder={t.t("level_arable_robin_talk")}
        bind:value={attributeType}
        multiple={false}
        bind:this={ms}
    >
        {#snippet optionRenderer(v)}
            {t.t(metas[v].label as any)}
        {/snippet}
        {#snippet valueRenderer(v)}
            {t.t(metas[v].label as any)}
        {/snippet}
    </MultiSelect>
    <button class="add_attribute" onclick={addAttribute}>
        <i class="ri-add-line"></i>
        {t.t("key_muddy_martin_flop")}
    </button>
</div>

<style lang="scss">
    .attribute-selector {
        display: flex;
        gap: 0.5rem;
        justify-content: center;
        button{
            outline: none;
            background-color: var(--main-a1);
            border: 1px solid var(--main-6);
            color: var(--main-11);
            border-radius: 3px;
            outline: none;
            padding-right: 0.5rem;
            &:hover{
                cursor: pointer;
                background-color: var(--main-a3);
                border: 1px solid var(--main-8);
                color: var(--main-12);
            }
        }
        :global(.multi-select){
           flex: 1;
        }
    }
</style>