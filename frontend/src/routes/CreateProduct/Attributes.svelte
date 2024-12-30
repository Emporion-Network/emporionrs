<script lang="ts">
    import MultiSelect from "../../lib/MultiSelect.svelte";
    import { data, type DataAttribute } from "../../lib/actions.svelte";
    import {
        type SupportedLanguage,
        getTranslator,
    } from "../../stores/translate.svelte";
    import Buttons from "./Attributes/Buttons.svelte";
    import Checkbox from "./Attributes/Checkbox.svelte";
    import Select from "./Attributes/Select.svelte";
    import { metas, type Attribute } from "./Attributes/_metas";
    import Draggable from "../../lib/Draggable.svelte";
    let {
        attributes = $bindable(),
        selectedLang,
    }: {
        selectedLang: SupportedLanguage;
        attributes: Attribute[];
    } = $props();

    let t = getTranslator();

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

<div class="attribute-selector" use:data={"tuto-selector=add_attribute"}>
    <MultiSelect
        options={["buttons", "checkbox", "select"] as const}
        label={t.t("level_arable_robin_talk")}
        placeholder={t.t("level_arable_robin_talk")}
        bind:value={attributeType}
        multiple={false}
    >
        {#snippet optionRenderer(v)}
            {t.t(metas[v].label as any)}
        {/snippet}
        {#snippet valueRenderer(v)}
            {t.t(metas[v].label as any)}
        {/snippet}
    </MultiSelect>
    <button onclick={addAttribute}>
        <i class="ri-add-line"></i>
        {t.t("key_muddy_martin_flop")}
    </button>
</div>

<div class="attributes">
    {#each attributes as attr, i (attr)}
        {@const Component = map[attr.type] as any}
        <Draggable onswap={swap(attr)}>
            {#snippet content()}
                <h2>{attr.type}</h2>
                <Component bind:attribute={attributes[i]} lang={selectedLang}
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
    :global(.input-attribute){
        display: flex;
        flex-direction: column;
        gap: 1rem;
    }
</style>
