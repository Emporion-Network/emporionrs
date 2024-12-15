<script lang="ts">
    import {api} from "../stores/user.svelte";
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
</nav>