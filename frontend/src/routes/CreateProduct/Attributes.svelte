<script lang="ts">
    import MultiSelect from "../../lib/MultiSelect.svelte";
    import {
        type T,
        type SupportedLanguage,
        Languages,
    } from "../../stores/translate.svelte";
    import Buttons from "./Attributes/Buttons.svelte";
    import Checkbox from "./Attributes/Checkbox.svelte";
    import Select from "./Attributes/Select.svelte";
    import {type Attribute, metas} from "./Attributes/_metas";
    import Draggable from "./Draggable.svelte";
    let {
        attributes = $bindable(),
        selectedLang
    }: {
        selectedLang: SupportedLanguage;
        attributes: Attribute[];
    } = $props();

    let attributeType: Attribute[SupportedLanguage]["type"] = $state("buttons");

    const addAttribute = () => {
        let attribute = Object.keys(Languages).reduce((acc, lang) => {
            acc[lang as SupportedLanguage] = metas[attributeType].defaultAttribute;
            return acc;
        }, {} as Attribute);
        attributes.push(attribute);
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
    {#each attributes as attr (attr)}
        {@const attribute = attr[selectedLang]}
        <Draggable onswap={swap(attr)}>
            {#snippet content()}
                {#if attribute.type === metas.buttons.type}
                    <Buttons
                        bind:title={attribute.title}
                        bind:value={attribute.value}
                    ></Buttons>
                {:else if attribute.type === metas.checkbox.type}
                    <Checkbox
                        bind:title={attribute.title}
                        bind:description={attribute.description}
                    ></Checkbox>
                {:else if attribute.type === metas.select.type}
                    <Select
                        bind:title={attribute.title}
                        bind:value={attribute.value}
                    ></Select>
                {/if}
            {/snippet}
            {#snippet menu()}
                <button
                    onclick={removeAttribute(attr)}
                    aria-label="Delete"
                >
                    <i class="ri-delete-bin-line"></i>
                </button>
            {/snippet}
        </Draggable>
    {/each}
</div>

<MultiSelect
    options={["buttons", "checkbox", "select"]}
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

<style lang="scss">
    .attributes {
        display: flex;
        flex-direction: column;
        padding: 1rem;
    }
</style>
