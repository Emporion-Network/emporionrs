<script lang="ts">
    import { onMount } from "svelte";
    import Route from "../../lib/Route.svelte";
    import { storage } from "../../stores/localStorage";
    import {
        getTranslator,
        supportedLangs,
    } from "../../stores/translate.svelte";
    import Form from "./Form.svelte";
    import type { Product } from "./Form.svelte";
    import Tuto from "./Tuto.svelte";
    import { TUTORIAL_PRODUCT } from "./tutorialProduct";
    let t = getTranslator();

    let didTutorial = storage<boolean>("product-tutorial");

    const ondone = () => {
        // didTutorial.set(true);
        // products = [];
    };

    let selectedLang = $state(t.lang);
    onMount(() => {
        setTimeout(() => {
            selectedLang = t.lang;
        }, 10);
    });

    let products: Product[] = $state([]);
</script>

<Route path="/create-product">
    <div class="create-product">
        <Form bind:products bind:selectedLang></Form>
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
        min-height: 100vh;
        .preview {
            flex: 5;
        }
    }
</style>
