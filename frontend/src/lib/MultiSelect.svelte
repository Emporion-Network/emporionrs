<script lang="ts">
    import type { Snippet } from "svelte";
    import type { SvelteSet } from "svelte/reactivity";

    type T = $$Generic;
    type K = $$Generic<boolean>;

    let {
        value = $bindable(),
        options,
        optionRenderer,
        valueRenderer,
        filterRenderer,
        multiple,
        filter = $bindable(),
        label = "",
        placeholder = "",
    }: {
        value: typeof multiple extends true ? NoInfer<SvelteSet<T>> : T;
        optionRenderer: Snippet<[T, boolean]>;
        valueRenderer: Snippet<
            [typeof multiple extends true ? NoInfer<SvelteSet<T>> : T]
        >;
        filterRenderer?: Snippet;
        filter?: (v: T) => boolean;
        options: T[];
        multiple: K;
        label?: string;
        placeholder?: string;
    } = $props();

    const isMultiple = (b: SvelteSet<T> | T): b is SvelteSet<T> => {
        return multiple === true;
    };

    let el: HTMLElement = $state()!;
    let pos = $state(new DOMRect());

    const slectOption = (o: T) => () => {
        if (!isMultiple(value)) {
            (value as T) = o;
        } else {
            if (value.has(o)) {
                value.delete(o);
            } else {
                value.add(o);
            }
        }
        //@ts-ignore
        document.activeElement.blur();
    };

    const setPos = () => {
        pos = el.getBoundingClientRect();
    };

    export const actions = {
        setValue: (v: T) => slectOption(v)(),
        open: () => el.focus(),
    };
    export { el as element };
</script>

<svelte:window onscroll={setPos} onresize={setPos} />

<div
    bind:this={el}
    onfocus={setPos}
    class="multi-select"
    class:hasValue={(isMultiple(value) && value.size > 0) ||
        (!isMultiple(value) && value)}
    tabindex="0"
    role="listbox"
>
    <div class="selected">
        {#if (isMultiple(value) && value.size > 0) || (!isMultiple(value) && value !== undefined)}
            {@render valueRenderer(value)}
        {:else}
            <div class="placeholder">{placeholder}</div>
        {/if}
        <i class="ri-arrow-down-s-line"></i>
    </div>

    <div class="label">{label}</div>

    <div
        class="options"
        style="--l{pos.left}px; --t:{pos.top + pos.height}px; --w:{pos.width}px"
    >
        {#if filterRenderer}
            <div class="filter">
                {@render filterRenderer?.()}
            </div>
        {/if}
        {#each options as option (option)}
            {#if !filter || filter(option)}
                <button onclick={slectOption(option)}>
                    {@render optionRenderer(
                        option,
                        isMultiple(value) ? value.has(option) : value == option,
                    )}
                </button>
            {/if}
        {/each}
    </div>
</div>

<style lang="scss">
    @use "../mixins" as m;

    .multi-select {
        display: flex;
        flex-direction: column;
        position: relative;
        outline: none;
        background-color: var(--parent-bg, transparent);

        &:hover {
            .label {
                color: var(--neutral-12);
            }
            .selected {
                border: 1px solid var(--neutral-8);
                i {
                    color: var(--neutral-12);
                }
            }
        }

        &:focus-within {
            z-index: 2;
            .selected,
            .selected:hover {
                border: 1px solid var(--main-10);
                i {
                    transform: rotate(180deg);
                }
            }
            .placeholder {
                color: var(--neutral-10);
            }
            .label {
                color: var(--main-10);
                top: 0%;
                transform: translateY(-50%) scale(0.8);
            }
            .options {
                display: flex;
            }
        }
        &.hasValue {
            .label {
                top: 0%;
                transform: translateY(-50%) scale(0.8);
            }
        }
        .label {
            position: absolute;
            transition:
                color 100ms ease-in-out,
                transform 100ms ease-in-out,
                top 100ms ease-in-out;
            background-color: var(--parent-bg, var(--neutral-1));
            padding: 0 0.2rem;
            left: 0.5rem;
            top: 1.25rem;
            transform: translateY(-50%);

            color: var(--neutral-10);
            pointer-events: none;
        }
        .placeholder {
            color: transparent;
            pointer-events: none;
            transition: color 100ms ease-in-out;
            user-select: none;
            pointer-events: none;
        }
        .selected {
            display: flex;
            gap: 0.5rem;
            height: var(--height-2);
            justify-content: start;
            align-items: center;
            overflow-x: scroll;
            border: 1px solid var(--neutral-6);
            border-radius: 2px;
            padding-left: 0.5rem;
            min-height: var(--height-2);
            i {
                margin-left: auto;
                margin-right: 0.5rem;
                transition: transform 100ms ease-in-out;
                color: var(--neutral-8);
            }
        }
        .options {
            position: fixed;
            max-height: 300px;
            top: calc(var(--t) + 0.5rem);
            left: var(--l);
            width: var(--w);

            overflow-y: auto;
            display: none;
            flex-direction: column;
            border: 1px solid var(--neutral-6);
            background-color: var(--parent-bg, var(--neutral-1));

            .filter {
                position: sticky;
                top: 0;
                background-color: var(--parent-bg, var(--neutral-1));
                width: 100%;
            }

            @include m.media("<=phone") {
                width: 100vw;
                min-height: 70vh;
                position: fixed;
                top: 30vh;
                left: 0;
                padding: 0 0.5rem;
            }

            button {
                min-height: var(--height-2);
                background-color: transparent;
                border: none;
                outline: none;
                display: flex;
                gap: 0.5rem;
                justify-content: start;
                align-items: center;
                cursor: pointer;
                color: var(--neutral-12);
                &:hover,
                &:focus {
                    background-color: var(--neutral-a3);
                }
            }
        }
    }
</style>
