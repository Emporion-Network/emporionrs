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
    import Collapsable from "../../lib/Collapsable.svelte";
    import Categories from "./Categories.svelte";
    import ContextMenu from "../../lib/ContextMenu.svelte";

    let t = getTranslator();
    let registry = getTutoRegistry();
    let {
        selectedProduct = $bindable(),
        products = $bindable(),
        selectedLang = $bindable(),
    }: {
        products: Product[];
        selectedLang: SupportedLanguage;
        selectedProduct: number;
    } = $props();

    let attributes: Attribute[] = $state([]);
    let collectionName = $state("");
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
        products.forEach((p) => {
            p.attributes.push(bindClone(a as any));
        });
    };

    $effect.pre(() => {
        products;
        untrack(() => {
            attributes = $state.snapshot(products[0]?.attributes) || [];
            collectionName = products[0]?.collection || "";
        });
    });

    const toggleProductView = (productId?: number) => {
        if (productId !== undefined) {
            selectedProduct = productId;
            showProduct = true;
        } else {
            showProduct = false;
        }
    };

    const cloneProduct = (productId: number) => {
        products.push($state.snapshot(products[productId]));
    };

    const selectProduct = (productId: number) => {
        selectedProduct = productId;
    };
</script>

<div class="form">
    {#if !showProduct}
        <div class="collection">
            <div class="wpr">
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
            </div>
            <Collapsable opened>
                {#snippet head()}
                    <h3>General</h3>
                {/snippet}
                <div class="wpr">
                    <Input
                        type="text"
                        label={t.t("mealy_spare_thrush_gleam")}
                        placeholder={t.t("mealy_spare_thrush_gleam")}
                        bind:value={collectionName}
                        bind:this={registry["collection_name"]}
                    />
                    <Categories value="category_all"></Categories>
                </div>
            </Collapsable>
            <Collapsable opened>
                {#snippet head()}
                    <h3>Attributes</h3>
                {/snippet}

                <AttributesInCollection
                    {onremove}
                    {onswap}
                    {onpush}
                    {selectedLang}
                    bind:attributes
                />
            </Collapsable>
            <Collapsable opened>
                {#snippet head()}
                    <h3>Products</h3>
                {/snippet}
                <div class="products" bind:this={registry["products"]}>
                    {#each products as product, i}
                        <div class="preview">
                            {#if product.gallery[t.lang][0]}
                                <img src={product.gallery[t.lang][0]} alt="" />
                            {:else}
                                <i class="ri-image-line"></i>
                            {/if}
                            <h2>{product.title[t.lang]}</h2>
                            <ContextMenu>
                                {#snippet opener(props)}
                                    <button
                                        aria-label="More"
                                        bind:this={props.ref}
                                        {...props}
                                    >
                                        <i class="ri-more-line"></i>
                                    </button>
                                {/snippet}
                                {#snippet options(close)}
                                    <button
                                        onclick={() => toggleProductView(i)}
                                    >
                                        <i class="ri-pencil-line"></i>
                                        {t.t("sunny_lazy_puffin_devour")}
                                    </button>
                                    <button
                                        onclick={() => close() && cloneProduct(i)}>
                                        <i class="ri-file-copy-line"></i>
                                        Clone Product
                                    </button>
                                    <button
                                        onclick={() => close() && selectProduct(i)}>
                                        <i class="ri-file-copy-line"></i>
                                        Select Product
                                    </button>
                                    <!-- TODO: disable product -->
                                {/snippet}
                            </ContextMenu>
                        </div>
                    {/each}
                    <button
                        onclick={addProduct}
                        class="button-main"
                        bind:this={registry["add_product"]}
                    >
                        <i class="ri-add-line"></i>
                        {t.t("spry_cool_goose_sprout")}
                    </button>
                </div>
            </Collapsable>
        </div>
    {:else}
        <div class="product" transition:fly={{ x: -100 }}>
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
                class="button-main"
                onclick={() => toggleProductView()}
                bind:this={registry["close_product"]}
            >
                {t.t("actual_cool_racoon_laugh")}
            </button>
        </div>
    {/if}
</div>

<style lang="scss">
    .form {
        flex: 3;
        position: relative;
        display: flex;
        overflow: hidden;
        --parent-bg: var(--neutral-1);
        background-color: var(--parent-bg);
        border-right: 1px solid var(--neutral-6);

        .wpr {
            display: flex;
            flex-direction: column;
            padding: 1rem;
            gap: 1rem;
        }

        .collection {
            display: flex;
            flex-direction: column;
            min-width: 100%;
        }

        .product {
            background-color: var(--parent-bg);
            display: flex;
            flex-direction: column;
            gap: 1rem;
            z-index: 1;
            min-width: 100%;
            padding: 1rem;
        }

        .products {
            display: flex;
            flex-direction: column;
            gap: 1rem;
            padding: 1rem;

            .preview {
                display: flex;
                width: 100%;
                gap: 1rem;
                align-items: center;
                border: 1px solid var(--neutral-6);
                padding: 0.5rem;
                border-radius: 3px;
                color: var(--neutral-12);
                & > i,
                & > img {
                    width: 80px;
                    height: 80px;
                    object-fit: cover;
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
                    line-clamp: 2;
                    -webkit-box-orient: vertical;
                    overflow: hidden;
                }
                button {
                    background-color: transparent;
                    border: none;
                    color: var(--neutral-11);
                    outline: none;
                    display: flex;
                    gap: 0.5rem;
                    padding: 0.5rem;
                    &:hover {
                        cursor: pointer;
                        color: var(--neutral-12);
                    }
                }
            }
        }
    }
</style>
