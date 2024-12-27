<script lang="ts">
    import MultiSelect from "../../lib/MultiSelect.svelte";
    import {
        type SupportedLanguage,
        Languages,
    } from "../../stores/translate.svelte";
    import Buttons from "./Attributes/Buttons.svelte";
    import Checkbox from "./Attributes/Checkbox.svelte";
    import Select from "./Attributes/Select.svelte";
    import { metas, type Attribute } from "./Attributes/_metas";
    import Draggable from "./Draggable.svelte";
    let {
        attributes = $bindable(),
        selectedLang,
    }: {
        selectedLang: SupportedLanguage;
        attributes: Attribute[];
    } = $props();

    let attributeType: Attribute["type"] = $state("buttons");

    const map = {
        [metas.buttons.type]: Buttons,
        [metas.checkbox.type]: Checkbox,
        [metas.select.type]: Select,
    };

    const addAttribute = () => {
        attributes.push(metas[attributeType].defaultAttribute);
    };

    const removeAttribute = (attribute: Attribute) => () => {
        attributes = attributes.filter((a) => a !== attribute);
    };

    const swap = (attribute: Attribute) => (dir: number) => {
        const index = attributes.indexOf(attribute);
        const newIndex = index + dir;
        if (newIndex < 0 || newIndex >= attributes.length) return;
        attributes[index] = attributes[newIndex];
        attributes[newIndex] = attribute;
    };
</script>

<div class="attributes">
    {#each attributes as attr, i (attr)}
        {@const Component = map[attr.type] as any}
        <Draggable onswap={swap(attr)}>
            {#snippet content()}
                <h1>{attr.type}</h1>
                <Component bind:attribute={attributes[i]} lang={selectedLang}
                ></Component>
            {/snippet}
            {#snippet menu()}
                <button onclick={removeAttribute(attr)} aria-label="Delete">
                    <i class="ri-delete-bin-line"></i>
                </button>
            {/snippet}
        </Draggable>
    {/each}
</div>

<div class="attribute-selector">
    <MultiSelect
        options={["buttons", "checkbox", "select"]}
        label="Attribute type"
        placeholder="Attribute type"
        bind:value={attributeType}
        multiple={false}
    >
        {#snippet optionRenderer(v)}
            {v}
        {/snippet}
        {#snippet valueRenderer(v)}
            {v}
        {/snippet}
    </MultiSelect>
    <button onclick={addAttribute}>Add attribute</button>
</div>

<style lang="scss">
    .attributes {
        display: flex;
        flex-direction: column;
        padding: 1rem;
    }
    .attribute-selector {
        display: flex;
        gap: 0.5rem;
        justify-content: center;
        button{
            margin-top: 0.5rem;
        }
        :global(.multi-select){
            min-width: 200px;
        }
    }
</style>
