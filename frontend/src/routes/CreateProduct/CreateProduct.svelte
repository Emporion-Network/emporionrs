<script lang="ts">
    import Input from "../../lib/Input.svelte";
    import MultiSelect from "../../lib/MultiSelect.svelte";
    import Route from "../../lib/Route.svelte";
    import TranslatableInput from "../../lib/TranslatableInput.svelte";
    import { getKeys } from "../../lib/utils";
    import { storage } from "../../stores/localStorage";
    import {
        getTranslator,
        translatedArray,
        TranslatedLanguages,
        translatedString,
        type SupportedLanguage,
        type T,
    } from "../../stores/translate.svelte";
    import Attributes from "./Attributes.svelte";
    import { type Attribute } from "./Attributes/_metas";
    import Gallery from "./Gallery.svelte";
    import Tuto from "./Tuto.svelte";
    import {TUTORIAL_PRODUCT} from "./tutorialProduct";
    let t = getTranslator();
    let langs = getKeys(TranslatedLanguages);
    let didTutorial = storage<boolean>("product-tutorial");

    const ondone = ()=>{
        didTutorial.set(true);
    }

    type Product = {
        title: T<string>;
        description: T<string>;
        collection: string;
        gallery:Record<SupportedLanguage, string[]>,
        attributes: Attribute[];
    };

    let selectedLang = $state(langs[0]);
    let product: Product = $state({
        title: translatedString(),
        description: translatedString(),
        collection: "",
        gallery: translatedArray(),
        attributes: [],
    });
    const create = ()=>{
        console.log($state.snapshot(product))
    }
    if(!didTutorial.get()){
        product = TUTORIAL_PRODUCT as Product;
    }
</script>

<Route path="/create-product">
    <div class="create-product">
        <div class="form">
            <MultiSelect
                options={langs}
                bind:value={selectedLang}
                multiple={false}
                placeholder={t.t("weird_fuzzy_warbler_edit")}
                label={t.t("weird_fuzzy_warbler_edit")}
                selector="tuto-selector=language"
            >
                {#snippet valueRenderer(v)}
                    {t.t(TranslatedLanguages[v])}
                {/snippet}
                {#snippet optionRenderer(v)}
                    {t.t(TranslatedLanguages[v])}
                {/snippet}
            </MultiSelect>

            <Gallery {selectedLang} bind:images={product.gallery}></Gallery>

            <TranslatableInput
                type="textarea"
                label={t.t("vivid_great_platypus_clasp")}
                bind:selectedLang
                bind:value={product.title}
                selector={"tuto-selector=product_title"}
            />
            <TranslatableInput
                type="textarea"
                label={t.t("alive_sad_grebe_beam")}
                bind:selectedLang
                bind:value={product.description}
                selector={"tuto-selector=product_description"}
                buttonSelector={"tuto-selector=translate"}
            />
            <Input
                type="text"
                label={t.t("mealy_spare_thrush_gleam")}
                placeholder={t.t("mealy_spare_thrush_gleam")}
                bind:value={product.collection}
                selector={"tuto-selector=product_collection"}
            />
            
            <Attributes
                {selectedLang}
                bind:attributes={product.attributes}
            ></Attributes>
            <button onclick={create}>create</button>
        </div>
        <div class="preview"></div>
        {#if !didTutorial.get()}
            <Tuto {ondone}></Tuto>
        {/if}
    </div>
</Route>

<style lang="scss">
    .create-product {
        display: flex;
        padding: 1rem;
        position: relative;
        .preview {
            flex: 5;
        }
        .form {
            flex: 3;
            display: flex;
            flex-direction: column;
            gap: 1rem;
        }
    }
</style>
