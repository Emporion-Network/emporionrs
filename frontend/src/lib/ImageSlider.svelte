<script lang="ts">
    let {
        images,
        alt,
    }: {
        images: string[];
        alt: string;
    } = $props();

    let element: HTMLElement = $state()!;
    let i = $state(0);

    const slide = (n: number) => () => {
        i += n;
        element.querySelectorAll("img")[i].scrollIntoView({
            behavior: "smooth",
            inline: "center",
        });
    };
</script>

<div class="gellery" bind:this={element}>
    {#each images as img}
        <img src={img} {alt} />
    {/each}
    {#if images.length}
        <div class="buttons">
            <button
                aria-label="prev image"
                class:hide={i == 0}
                onclick={slide(-1)}
            >
                <i class="ri-arrow-left-wide-line"></i>
            </button>
            <button
                aria-label="next image"
                class:hide={i == images.length - 1}
                onclick={slide(+1)}
            >
                <i class="ri-arrow-right-wide-line"></i>
            </button>
        </div>
    {/if}
</div>

<style lang="scss">
    .gellery {
        flex: 1;
        display: flex;
        overflow: hidden;
        position: sticky;
        top: 2rem;
        border-radius: 4px;
        .buttons {
            right: 0%;
            position: sticky;
            min-width: 100%;
            margin-left: -100%;
            display: flex;
            justify-content: space-between;
            align-items: center;
            padding: 1rem;
            button {
                background-color: var(--neutral-4);
                border: none;
                outline: none;
                color: var(--neutral-12);
                aspect-ratio: 1;
                border-radius: 3px;
                cursor: pointer;
                &.hide{
                    visibility: hidden;
                }
            }
        }
        img {
            min-width: 100%;
            aspect-ratio: 1;
            object-fit: cover;
        }
    }
</style>
