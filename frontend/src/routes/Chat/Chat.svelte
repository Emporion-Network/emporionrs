<script lang="ts">
    import Route from "../../lib/Route.svelte";
    import { api } from "../../stores/user.svelte";

    let text = $state("");
    let to = $state("")

    const send = (k:KeyboardEvent)=>{
        console.log(k);
        if(k.key === "Enter"){
            api.sendMessage(text, to);
            text = "";
        }
    }
</script>

<Route path="/chat">
    <div class="messages">
        {#each api.state?.notifications || [] as notification}
            {#if "message" in notification.data}
                {@const message = notification.data.message}
                {message.message}
            {/if}
        {/each}
        <input bind:value={to}>
        <textarea onkeydown={send} bind:value={text}></textarea>
    </div>
</Route>