<script lang="ts">
    import { onMount } from "svelte";
    import Route from "../../lib/Route.svelte";
    import { storage } from "../../stores/localStorage";
    import {
        getTranslator,
    } from "../../stores/translate.svelte";
    import Form from "./Form.svelte";
    import type { Product } from "./Form.svelte";
    import Tuto from "./Tuto.svelte";
    import Preview from "./Preview.svelte";
    import {DEV} from "./tutorialProduct";
    let t = getTranslator();

    let didTutorial = storage<boolean>("product-tutorial");
    let selectedLang = $state(t.lang);
    let selectedProduct = $state(0);
    let products: Product[] = $state([]/*DEV as any*/);

    const ondone = () => {
        didTutorial.set(true);
    };

    onMount(() => {
        setTimeout(() => {
            selectedLang = t.lang;
        }, 10);
    });

    
</script>

<Route path="/create-product">
    <div class="create-product">
        <Form bind:products bind:selectedLang bind:selectedProduct></Form>
        <Preview {products} {selectedLang} bind:selectedProduct></Preview>
        {#if !didTutorial.get()}
            <Tuto {ondone} bind:products></Tuto>
        {/if}
    </div>
</Route>

<style lang="scss">
    .create-product {
        display: flex;
        position: relative;
        min-height: 100vh;
        gap: 1rem;
    }
</style>
