<script lang="ts">
    import MultiSelect from "../../../lib/MultiSelect.svelte";
    import {
        type SupportedLanguage,
        getTranslator,
    } from "../../../stores/translate.svelte";
    import Buttons from "./Buttons/InAttributes.svelte";
    import Checkbox from "./Checkbox/InAttributes.svelte";
    import Select from "./Select/InAttributes.svelte";
    import { metas, type Attribute } from "./_metas";
    import Draggable from "../../../lib/Draggable.svelte";
    import { getTutoRegistry } from "../tutoStore.svelte";
    import AttributeSelector from "./AttributeSelector.svelte";
    let {
        attributes = $bindable(),
        selectedLang,
        onswap,
        onremove,
        onpush,
    }: {
        selectedLang: SupportedLanguage;
        attributes: Attribute[];
        onswap?:(i:number, j:number)=>void;
        onremove?:(i:number)=>void;
        onpush?:() => void;
    } = $props();

    let t = getTranslator();
    let registry = getTutoRegistry();

    let attributeType: Attribute["display_type"] = $state("buttons");

    const map = {
        [metas.buttons.type]: Buttons,
        [metas.checkbox.type]: Checkbox,
        [metas.select.type]: Select,
    };

    const removeAttribute = (attribute: Attribute) => () => {
        let idx = attributes.indexOf(attribute);
        attributes = attributes.filter((_, i) => idx !== i);
        onremove?.(idx);
    };

    const swap = (attribute: Attribute) => (dir: number) => {
        const index = attributes.indexOf(attribute);
        const newIndex = index + dir;
        if (newIndex < 0 || newIndex >= attributes.length) return;
        attributes[index] = attributes[newIndex];
        attributes[newIndex] = attribute;
        onswap?.(index, newIndex);
    };
</script>

<AttributeSelector {onpush} bind:attributes bind:attributeType bind:this={registry["attribute_selector"]}/>

<div class="attributes">
    {#each attributes as attr, i (attr)}
        {@const Component = map[attr.display_type] as any}
        <Draggable 
        onswap={swap(attr)}
        bind:this={registry[`attribute_${i}`]}
        >
            {#snippet content()}
                <h2>{t.t(metas[attr.display_type].label as any)}</h2>
                <Component bind:attribute={attributes[i]} lang={selectedLang}
                    bind:this={registry[`attribute_${i}_elem`]}
                ></Component>
            {/snippet}
            {#snippet menu()}
                <button onclick={removeAttribute(attr)} aria-label={t.t("large_crisp_dove_spur")}>
                    <i class="ri-delete-bin-line"></i>
                </button>
            {/snippet}
        </Draggable>
    {/each}
</div>



<style lang="scss">
    .attributes {
        display: flex;
        flex-direction: column;
        padding: 1rem 0;
        h2{
            font-family: var(--font-2);
            margin-bottom: 1rem;
            text-transform: capitalize;
        }
    }
    
    :global(.input-attribute){
        display: flex;
        flex-direction: column;
        gap: 1rem;
    }
</style>
