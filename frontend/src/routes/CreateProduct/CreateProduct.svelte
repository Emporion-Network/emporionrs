<script lang="ts">
    import Input from "../../lib/Input.svelte";
    import MultiSelect from "../../lib/MultiSelect.svelte";
    import Route from "../../lib/Route.svelte";
    import TranslatableInput from "../../lib/TranslatableInput.svelte";
    import { getKeys } from "../../lib/utils";
    import {
        getTranslator,
        Languages,
        translatedString,
        type SupportedLanguage,
        type T,
    } from "../../stores/translate.svelte";
    import Attributes from "./Attributes.svelte";
    import { type Attribute } from "./Attributes/_metas";
    let t = getTranslator();

    let langs = getKeys(Languages);

    type Product = {
        title: T<string>;
        description: T<string>;
        collection: string;
        attributes: Attribute[];
    };

    let selectedLang = $state(langs[0]);
    let product: Product = $state({
        title: translatedString(),
        description: translatedString(),
        collection: "",
        attributes: [],
    });
</script>

<Route path="/create-product">
    <div class="create-product">
        <div class="preview"></div>
        <div class="form">
            <MultiSelect
                options={langs}
                bind:value={selectedLang}
                multiple={false}
                placeholder="Language"
                label="Language"
            >
                {#snippet valueRenderer(v)}
                    {t.t(Languages[v])}
                {/snippet}
                {#snippet optionRenderer(v)}
                    {t.t(Languages[v])}
                {/snippet}
            </MultiSelect>

            <TranslatableInput
                type="textarea"
                label="Product title"
                bind:selectedLang={selectedLang}
                bind:value={product.title}
            />
            <TranslatableInput
                type="textarea"
                label="Product description"
                bind:selectedLang={selectedLang}
                bind:value={product.description}
            />

            <Input
                type="text"
                label="Collection name"
                placeholder="Collection name"
                bind:value={product.collection}
            />
            <Attributes
                selectedLang={selectedLang}
                bind:attributes={product.attributes}
            ></Attributes>
        </div>
    </div>
</Route>

<style lang="scss"></style>
