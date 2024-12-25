<script lang="ts">
    import Input from "../../lib/Input.svelte";
    import MultiSelect from "../../lib/MultiSelect.svelte";
    import Route from "../../lib/Route.svelte";
    import {
        Languages,
        translatedString,
        type SupportedLanguage,
        type T,
    } from "../../stores/translate.svelte";
    import { api } from "../../stores/user.svelte";
    import Attributes from "./Attributes.svelte";
    import { type Attribute, metas } from "./Attributes/_metas";
    const langs = Object.keys(Languages).map((key) => ({
        value: key as SupportedLanguage,
        label: Languages[key as SupportedLanguage],
    }));

    type Product = {
        title: T<string>;
        description: T<string>;
        collection: T<string>;
        attributes: Attribute[];
    };

    let selectedLang = $state(langs[0]);
    let product: Product = $state({
        title: translatedString(),
        description: translatedString(),
        collection: translatedString(),
        attributes: [],
    });
    const keys = ["title", "description", "collection"] as const;
    let missingTranslations = $derived.by(() => {
        let missing: Partial<
            Record<(typeof keys)[number], SupportedLanguage[]>
        > = {};
        keys.forEach((key) => {
            let allEmpty = langs.every((lang) => !product[key][lang.value]);
            Object.keys(product[key]).forEach((lang) => {
                if (!product[key][lang as SupportedLanguage] && !allEmpty) {
                    if (!missing[key]) {
                        missing[key] = [];
                    }
                    missing[key]!.push(lang as SupportedLanguage);
                }
            });
        });
        return missing;
    });

    const translate = (key:typeof keys[number], missingKeys:SupportedLanguage[]) => ()=> {
        const providedKey = Object.keys(product[key]).find((k) => !missingKeys.includes(k as SupportedLanguage))! as SupportedLanguage;
        missingKeys.forEach(async (lang) => {
            let res = (await api.translate({
                translations:{
                    [providedKey]: product[key][providedKey],
                    [lang]: "",
                }
            })).unwrap();
            product[key][lang] = res[lang];
        });
        
    }
</script>

<Route path="/create-product">
    <div class="create-product">
        <div class="preview"></div>
        <div class="form">
            {JSON.stringify(missingTranslations)}
            <MultiSelect
                options={langs}
                bind:value={selectedLang}
                multiple={false}
            >
                {#snippet valueRenderer(v)}
                    {v.label}
                {/snippet}
                {#snippet optionRenderer(v)}
                    {v.label}
                {/snippet}
            </MultiSelect>
            <div class="wpr">
                <Input
                    type="textarea"
                    label="Product title"
                    placeholder="Product title"
                    bind:value={product.title[selectedLang.value]}
                />
                {#if missingTranslations.title}
                    <button onclick={translate('title', missingTranslations.title)}>Translate {missingTranslations.title.length} missing translations</button>
                {/if}
            </div>
            <div class="wpr">
                <Input
                    type="textarea"
                    label="Product description"
                    placeholder="Product description"
                    bind:value={product.description[selectedLang.value]}
                />
                {#if missingTranslations.description}
                    <button onclick={translate("description", missingTranslations.description)}>Translate {missingTranslations.description.length} missing translations</button
                    >
                {/if}
            </div>
            <div class="wpr">
                <Input
                    type="text"
                    label="Collection name"
                    placeholder="Collection name"
                    bind:value={product.collection[selectedLang.value]}
                />
                {#if missingTranslations.collection}
                    <button>Translate {missingTranslations.collection.length} missing translations</button>
                {/if}
            </div>
            <Attributes
                selectedLang={selectedLang.value}
                bind:attributes={product.attributes}
            ></Attributes>
        </div>
    </div>
</Route>

<style lang="scss">
    .wpr{
        display: flex;
        :global(*){
            flex: 1;
        }
        button{
            max-width: max-content;
            margin-top: 0.5rem;
        }
    }
</style>