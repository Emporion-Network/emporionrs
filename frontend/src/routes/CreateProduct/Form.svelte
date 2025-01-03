<script lang="ts" module>
    export type Product = {
        title: T<string>;
        description: T<string>;
        collection: string;
        gallery: Record<SupportedLanguage, string[]>;
        attributes: Attribute[];
    };
</script>

<script lang="ts">
    import Input from "../../lib/Input.svelte";
    import MultiSelect from "../../lib/MultiSelect.svelte";
    import TranslatableInput from "../../lib/TranslatableInput.svelte";
    import {
        getTranslator,
        TranslatedLanguages,
        supportedLangs,
        type SupportedLanguage,
        type T,
        translatedString,
        translatedArray,
    } from "../../stores/translate.svelte";
    import AttributesInCollection from "./Attributes/InCollection.svelte";
    import { metas, type Attribute } from "./Attributes/_metas";
    import Gallery from "./Gallery.svelte";
    import AttributesInProduct from "./Attributes/InProduct.svelte";
    import { untrack } from "svelte";
    import { fly } from "svelte/transition";
    import { getTutoRegistry } from "./tutoStore.svelte";

    let t = getTranslator();
    let registry = getTutoRegistry();
    let {
        products = $bindable(),
        selectedLang = $bindable(),
    }: {
        products: Product[];
        selectedLang: SupportedLanguage;
    } = $props();

    let attributes: Attribute[] = $state([]);
    let collectionName = $state("");
    let selectedProduct = $state(-1);
    let showProduct = $state(false);

    const addProduct = () => {
        products.push({
            title: translatedString(),
            description: translatedString(),
            collection: collectionName,
            gallery: translatedArray(),
            attributes: attributes.map((e) =>
                metas[e.display_type].bindClone(e as any),
            ),
        });
        toggleProductView(products.length - 1);
    };

    const onswap = (i: number, j: number) => {
        products.forEach((p) => {
            let temp = p.attributes[i];
            p.attributes[i] = p.attributes[j];
            p.attributes[j] = temp;
        });
    };

    const onremove = (idx: number) => {
        products.forEach((p) => {
            p.attributes = p.attributes.filter((_, i) => idx !== i);
        });
    };

    const onpush = () => {
        const a = attributes[attributes.length - 1];
        const bindClone = metas[a.display_type].bindClone;
        console.log("hhehhe");
        products.forEach((p) => {
            p.attributes.push(bindClone(a as any));
        });
    };

    $effect.pre(() => {
        products;
        untrack(() => {
            attributes = $state.snapshot(products[0]?.attributes) || [];
        });
    });

    const toggleProductView = (productId?: number) => {
        if (productId !== undefined) {
            selectedProduct = productId;
            showProduct = true;
        } else {
            showProduct = false;
            productId = -1;
        }
    };
</script>

<div class="form">
    {#if !showProduct}
        <div class="colloection">
            <MultiSelect
                options={supportedLangs}
                bind:value={selectedLang}
                multiple={false}
                placeholder={t.t("weird_fuzzy_warbler_edit")}
                label={t.t("weird_fuzzy_warbler_edit")}
                bind:this={registry["lang_selector"]}
            >
                {#snippet valueRenderer(v)}
                    {t.t(TranslatedLanguages[v])}
                {/snippet}
                {#snippet optionRenderer(v)}
                    {t.t(TranslatedLanguages[v])}
                {/snippet}
            </MultiSelect>

            <Input
                type="text"
                label={t.t("mealy_spare_thrush_gleam")}
                placeholder={t.t("mealy_spare_thrush_gleam")}
                bind:value={collectionName}
                bind:this={registry["collection_name"]}
            />

            <AttributesInCollection
                {onremove}
                {onswap}
                {onpush}
                {selectedLang}
                bind:attributes
            />
            <div class="products" bind:this={registry["products"]}>
                {#each products as product, i}
                    <div class="preview">
                        {#if product.gallery[selectedLang][0]}
                            <img src={product.gallery[selectedLang][0]} alt="" />
                            {:else}
                            <i class="ri-image-line"></i>
                        {/if}
                        <h2>{product.title[selectedLang]}</h2>
                        <button
                            onclick={() => toggleProductView(i)}
                            aria-label="Edit product"
                        >
                            <i class="ri-pencil-line"></i>
                        </button>
                    </div>
                {/each}
                <button
                    onclick={addProduct}
                    bind:this={registry["add_product"]}
                >
                    <i class="ri-add-line"></i>
                    Add product
                </button>
            </div>
        </div>
    {:else}
        <div class="product" transition:fly={{ x: -100 }}>
            <Gallery
                {selectedLang}
                bind:images={products[selectedProduct].gallery}
                bind:this={registry["gallery"]}
            ></Gallery>

            <TranslatableInput
                type="textarea"
                label={t.t("vivid_great_platypus_clasp")}
                bind:selectedLang
                bind:value={products[selectedProduct].title}
                bind:this={registry["product_name"]}
            />
            <TranslatableInput
                type="textarea"
                label={t.t("alive_sad_grebe_beam")}
                bind:selectedLang
                bind:value={products[selectedProduct].description}
                bind:this={registry["product_description"]}
            />
            <AttributesInProduct
                bind:attributes={products[selectedProduct].attributes}
                {selectedLang}
            />
            <button
                onclick={() => toggleProductView()}
                bind:this={registry["close_product"]}>close</button
            >
        </div>
    {/if}
</div>

<style lang="scss">
    .form {
        flex: 3;
        position: relative;
        display: flex;
        flex-direction: column;
        gap: 1rem;
        padding-top: 1rem;

        .colloection{
            display: flex;
            flex-direction: column;
            gap: 1rem;
        }

        .product {
            background-color: var(--neutral-1);
            display: flex;
            flex-direction: column;
            gap: 1rem;
            opacity: 1;
            z-index: 1;
            gap: 1rem;
        }

        .products {
            display: flex;
            flex-direction: column;
            gap: 1rem;

            .preview {
                display: flex;
                width: 100%;
                gap: 1rem;
                align-items: center;
                border: 1px solid var(--neutral-6);
                padding: 0.5rem;
                border-radius: 3px;
                color: var(--neutral-12);
                &>i,
                &>img {
                    width: 80px;
                    height: 80px;
                    border-radius: 2px;
                    aspect-ratio: 1;
                    display: flex;
                    justify-content: center;
                    align-items: center;
                    background-color: var(--neutral-2);
                }
                h2 {
                    flex: 1;
                    height: 100%;
                    font-size: 1.1rem;
                    display: -webkit-box;
                    -webkit-line-clamp: 3;
                    line-clamp: 3;
                    -webkit-box-orient: vertical;  
                    overflow: hidden;
                }
                button {
                    background-color: transparent;
                    border: none;
                    color: var(--neutral-11);
                    outline: none;
                    &:hover {
                        cursor: pointer;
                        color: var(--neutral-12);
                    }
                }
            }

            & > button {
                outline: none;
                background-color: var(--main-a1);
                border: 1px solid var(--main-6);
                color: var(--main-11);
                border-radius: 3px;
                outline: none;
                height: var(--height-2);
                &:hover {
                    cursor: pointer;
                    background-color: var(--main-a3);
                    border: 1px solid var(--main-8);
                    color: var(--main-12);
                }
            }
        }
    }
</style>
