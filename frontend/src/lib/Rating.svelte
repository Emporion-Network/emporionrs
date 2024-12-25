<script lang="ts">
    import { getLocation } from "../stores/location.svelte";
    import { getTranslator } from "../stores/translate.svelte";
    import type { WithSkeleton } from "./utils";
    type Rating = (
        | {
              type: "short";
              avg_rating: number;
              url: string
          }
        | {
              type: "long";
              avg_rating: number;
              nb_ratings: number;
              url: string
          } | {
            type:"editable",
            value?: number
          }
    );

    let { goTo } = getLocation();
    let t = getTranslator();
    let {
        //@ts-expect-error
        value=$bindable(),
        ...props
    }: WithSkeleton<Rating> = $props();

    const setValue = (v:number)=>()=>{
        value = v;
    }

    const handleRedirect = (url: string) => () => {
        goTo(url);
    };
</script>

{#if props.skeleton}
    <div class="rating skeleton">
        {#each { length: 5 } as _}
            <i class="ri-star-fill"></i>
        {/each}
    </div>
{:else}
    <div class="rating">
        {#if props.type == "editable"}
            {#each { length: 5 } as _, i}
                <button aria-label={`${i} ${t.t("less_sleek_nils_read")}`} onclick={setValue(i)}>
                    <i class="ri-star-fill" class:selected={i <= value}></i>
                </button>
            {/each}
        {:else}
            <button
                aria-label={`${props.avg_rating}/5 ${t.t("less_sleek_nils_read")}`}
                onclick={handleRedirect(props.url)}
            >
                {#each { length: 5 } as _, i}
                    <i class="ri-star-fill"></i>
                {/each}
                {#if props.type == "long"}
                    ({props.nb_ratings} {t.t("fancy_known_peacock_pray")})
                {/if}
            </button>
        {/if}
    </div>
{/if}

<style lang="scss">
    .rating {
        display: flex;
        justify-content: flex-start;
        align-items: center;
        width: max-content;
        gap: 0.5rem;
        font-size: 1.3rem;
        &.skeleton {
            background-clip: text;
            color: transparent;
        }
        i.selected{
        }
    }
</style>
