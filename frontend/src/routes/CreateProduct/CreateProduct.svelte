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

    const ondone = () => {
        didTutorial.set(true);
    };

    let selectedLang = $state(t.lang);
    onMount(() => {
        setTimeout(() => {
            selectedLang = t.lang;
        }, 10);
    });

    let products: Product[] = $state(DEV as any);
</script>

<Route path="/create-product">
    <div class="create-product">
        <Form bind:products bind:selectedLang></Form>
        <Preview {products} {selectedLang}></Preview>
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
