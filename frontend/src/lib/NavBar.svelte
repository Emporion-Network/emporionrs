<script lang="ts">
    import { getTheme } from "../stores/theme.svelte";
    import { getTranslator, Languages } from "../stores/translate.svelte";
    import { api } from "../stores/user.svelte";
    import MultiSelect from "./MultiSelect.svelte";
    import { getKeys } from "./utils";
    import { getLocation } from "../stores/location.svelte";
    let theme = getTheme();
    let t = getTranslator();
    let location = getLocation();

</script>

<nav>
    {api.state?.address}
    <MultiSelect
        options={getKeys(Languages)}
        bind:value={t.lang}
        multiple={false}
    >
        {#snippet valueRenderer(v)}
            {Languages[v]}
        {/snippet}
        {#snippet optionRenderer(v)}
            {Languages[v]}
        {/snippet}
    </MultiSelect>
    <button onclick={() => (api.state?.address ? api.logout() : api.auth())}>
        {#if api.state?.address}
            Log out
        {:else}
            Log in
        {/if}
    </button>
    <button onclick={theme.toggle}>
        {theme.theme}
    </button>
    <button onclick={()=> location.goTo('/transfers')}>/transfers</button>
</nav>

<style lang="scss">
    nav {
        display: flex;
        justify-content: space-between;
        align-items: center;
    }
</style>
