<script lang="ts">
    import ImageSlider from "../../lib/ImageSlider.svelte";
    import type { SupportedLanguage } from "../../stores/translate.svelte";
    import Rendered from "./Attributes/Rendered.svelte";
    import type { Product } from "./Form.svelte";

    let {
        selectedLang,
        products,
        selectedProduct=$bindable()
    }: {
        products: Product[];
        selectedLang: SupportedLanguage;
        selectedProduct:number;
    } = $props();

    let product = $derived(products[selectedProduct]);
</script>

<div class="preview">
    {#if product}
        <ImageSlider
            images={product.gallery[selectedLang]}
            alt={product.title[selectedLang]}
        />
        <div class="picker">
            <h1>{product.title[selectedLang]}</h1>
            <p>{product.description[selectedLang]}</p>
            <Rendered {selectedLang} {products} bind:selectedProductId={selectedProduct}></Rendered>
        </div>
    {/if}
</div>

<style lang="scss">
    .preview {
        flex: 5;
        display: flex;
        align-items: flex-start;
        position: sticky;
        top: 1rem;
        align-self: flex-start;
        gap: 1rem;
        padding-right: 1rem;
        .picker {
            width: 40%;
            display: flex;
            flex-direction: column;
            gap: 0.5rem;
            h1{
                text-align: center;
                line-height: 1.1em;
            }
            p{
                text-align: center;
                line-height: 1.1em;
            }
        }
    }
</style>
