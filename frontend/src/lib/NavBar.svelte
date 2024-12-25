<script lang="ts">
    import { getTheme } from "../stores/theme.svelte";
    import {api} from "../stores/user.svelte";
    let theme = getTheme()
</script>
<nav>
    {api.state?.address}
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