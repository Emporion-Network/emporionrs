<script lang="ts">
    import { getTheme } from "../stores/theme.svelte";
    import { getTranslator } from "../stores/translate.svelte";
    import {api} from "../stores/user.svelte";
    import MultiSelect from "./MultiSelect.svelte";
    let theme = getTheme()
    let t = getTranslator();
    const changeLang = ()=>{
        console.log('gee');
        t.update("fr");
    }
</script>
<nav>
    {api.state?.address}
    <button onclick={changeLang}>Fr</button>
    <button onclick={()=>api.state?.address ? api.logout() : api.auth()}>
        {#if api.state?.address}
              Log out
              {:else}
              Log in
        {/if}
    </button>
    <button onclick={theme.toggle}>
        {theme.theme}
    </button>
</nav>