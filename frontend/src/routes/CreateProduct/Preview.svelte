<script lang="ts">
    import ImageSlider from "../../lib/ImageSlider.svelte";
    import type { SupportedLanguage } from "../../stores/translate.svelte";
    import Rendered from "./Attributes/Rendered.svelte";
    import type { Product } from "./Form.svelte";

    let {
        selectedLang,
        products,
    }: {
        products: Product[];
        selectedLang: SupportedLanguage;
    } = $props();

    let selectedProductId = $state(0);
    let selectedProduct = $derived(products[selectedProductId]);
</script>

<div class="preview">
    {#if selectedProduct}
        <ImageSlider
            images={selectedProduct.gallery[selectedLang]}
            alt={selectedProduct.title[selectedLang]}
        />
        <div class="picker">
            <h1>{selectedProduct.title[selectedLang]}</h1>
            <p>{selectedProduct.description[selectedLang]}</p>
            <Rendered {selectedLang} {products} bind:selectedProductId></Rendered>
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
            }
            p{
                text-align: center;
            }
        }
    }
</style>
