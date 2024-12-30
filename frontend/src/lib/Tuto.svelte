<script lang="ts">
    import { type Snippet } from "svelte";

    type T = $$Generic<string[]>;
    let {
        elementSelectors,
        help,
        step = $bindable(),
        ondone,
    }: {
        elementSelectors: T;
        step: number;
        help: Snippet<[T[number], DOMRect]>;
        ondone: () => void;
    } = $props();

    let currStep = $derived(elementSelectors[step]);
    let currEl = $state(new DOMRect());
    let size = $state(new DOMRect());
    let visible = $state(true);
    let elmt:HTMLElement|undefined = $state();

    $effect(() => {
        size;
        let el = document.querySelector(
            `[tuto-selector="${currStep}"]`,
        ) as HTMLElement;
        let rect = el?.getBoundingClientRect();
        let other = elmt?.getBoundingClientRect();
        el.scrollIntoView({
            behavior:"smooth",
            block:"center"
        })
        if (el&&other) currEl = new DOMRect(
            rect.x - other.x,
            rect.y - other.y,
            rect.width,
            rect.height,
        );
    });

    const close = () => {
        visible = false;
        ondone();
    };
    const next = () => {
        step++;
    };
    const prev = () => {
        step--;
    };
</script>

{#if visible}
    <div
        class="tuto"
        style:--x="{currEl.x - 10}px"
        style:--y="{currEl.y - 10}px"
        style:--w="{currEl.width + 20}px"
        style:--h="{currEl.height + 20}px"
        bind:contentRect={size}
        bind:this={elmt}
    >
        <div class="wpr">
            <div class="help">
                <div class="content">
                    {@render help(currStep, currEl)}
                </div>
                <div class="buttons">
                    <button onclick={prev} disabled={step == 0}>
                        <i class="ri-arrow-left-line"></i> Prev
                    </button>
                    {#if step < elementSelectors.length - 1}
                        <button onclick={next}>
                            Next <i class="ri-arrow-right-line"></i>
                        </button>
                    {:else}
                        <button class="done" onclick={close}> Finish </button>
                    {/if}
                </div>
            </div>
        </div>
    </div>
{/if}

<style lang="scss">
    .tuto {
        background-color: var(--black-a9);
        position: absolute;
        top: 0;
        left: 0;
        right: 0;
        bottom: 0;
        z-index: 2;
        transition: all 200ms ease-in-out;
        clip-path: polygon(
            0% 0%,
            0% 100%,
            var(--x) 100%,
            var(--x) var(--y),
            calc(var(--x) + var(--w)) var(--y),
            calc(var(--x) + var(--w)) calc(var(--y) + var(--h)),
            var(--x) calc(var(--y) + var(--h)),
            var(--x) 100%,
            100% 100%,
            100% 0%
        );
        .wpr {
            position: absolute;
            left: calc(var(--x) - 2px);
            top: calc(var(--y) - 2px);
            width: calc(var(--w) + 4px);
            height: calc(var(--h) + 4px);
            border: 1px solid var(--main-11);
            border-radius: 3px;
            transition: all 200ms ease-in-out;
            .help {
                position: absolute;
                top: 100%;
                width: 400px;
                margin-top: 1rem;
                left: 0;
                padding: 1rem;
                background-color: var(--neutral-2);
                border-radius: 3px;
                border: 1px solid var(--neutral-6);
                display: flex;
                flex-direction: column;
                gap: 1rem;
                .buttons {
                    display: flex;
                    gap: 1rem;
                    button {
                        background-color: var(--neutral-3);
                        border: 1px solid var(--neutral-6);
                        color: var(--neutral-12);
                        border-radius: 3px;
                        cursor: pointer;
                        outline: none;
                        &:hover{
                            background-color: var(--neutral-4);
                            border: 1px solid var(--neutral-7);
                        }
                        &.done{
                            background-color: var(--main-3);
                            border: 1px solid var(--main-6);
                            &:hover{
                                background-color: var(--main-4);
                                border: 1px solid var(--main-7);
                            }
                        }
                    }
                }
            }
        }
    }
</style>
