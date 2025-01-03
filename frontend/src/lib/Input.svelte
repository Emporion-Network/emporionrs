<script lang="ts">
    import type { Snippet } from "svelte";
    import { type FormEventHandler } from "svelte/elements";
    import Autocompleter from "./Autocompleter.svelte";

    let {
        value = $bindable(),
        label,
        placeholder,
        type,
        error,
        children,
        //@ts-ignore
        completions = $bindable(),
    }: {
        label: string;
        placeholder: string;
        error?: boolean;
        children?: Snippet<[]>;
    } & (
        | {
              value?: string | number;
              type: "text" | "textarea" | "search";
          }
        | {
              value?: number;
              type: "number";
          }
        | {
              value?: string;
              type: "autocomplete";
              completions: string[];
          }
    ) = $props();

    let lastValid: any = "";
    let ipt = $state(value?.toString());
    let el:HTMLElement = $state()!;

    const validate: FormEventHandler<HTMLInputElement> = (e) => {
        const regex =
            /(^[0-9]\.[0-9]*$)|(^[0-9]$)|(^[1-9][0-9]*$)|(^[1-9][0-9]*\.[0-9]*$)|(^$)/;
        let t = e.currentTarget;
        if (!regex.test(t.value)) {
            ipt = lastValid;
        }
        lastValid = ipt;
        value = Number(lastValid);
    };

    const clear = () => {
        value = "";
    };
    export const actions = {
        setValue(nv:typeof value){
            value = nv;
        }
    }
    export {
        el as element
    }
</script>

<label class="input {type}" class:error bind:this={el}>
    <div>{label}</div>
    {#if type == "text"}
        <input class="native" type="text" {placeholder} bind:value />
        {@render children?.()}
    {:else if type == "search"}
        {@render children?.()}
        <input class="native" type="text" {placeholder} bind:value />
        <button aria-label="clear" onclick={clear}>
            {#if typeof value == "string" && value.length > 0}
                <i class="ri-close-line"></i>
            {:else}
                <i class="ri-search-line"></i>
            {/if}
        </button>
    {:else if type === "number"}
        <input
            type="text"
            class="native"
            inputmode="numeric"
            oninput={validate}
            {placeholder}
            bind:value={ipt}
        />
        {@render children?.()}
    {:else if type == "autocomplete" && typeof value !== "number"}
        <Autocompleter bind:value bind:completions {placeholder} />
    {:else}
        <textarea class="native" bind:value {placeholder}></textarea>
        {@render children?.()}
    {/if}
</label>

<style lang="scss">
    .input {
        display: flex;
        align-items: center;
        position: relative;
        border: 1px solid var(--neutral-6);
        transition: all 200ms ease-in-out;
        border-radius: 2px;
        padding: 0 0.5rem;
        padding-top: 0.5rem;
        button {
            background-color: transparent;
            color: var(--neutral-10);
            border: none;
            outline: none;
            font-size: 1rem;
            cursor: pointer;
        }
        &:hover {
            border: 1px solid var(--neutral-8);
            div {
                color: var(--neutral-12);
            }
        }
        &:focus-within {
            border: 1px solid var(--main-10);
            :global(.native::placeholder) {
                color: var(--neutral-10);
            }
            div {
                top: 0%;
                transform: translateY(-50%) scale(0.8);
                color: var(--main-10);
            }
        }

        &:has(:global(.native:placeholder-shown)) {
            div {
                top: 1.75rem;
                transform: translateY(-50%);
            }
            &:focus-within {
                div {
                    top: 0%;
                    transform: translateY(-50%) scale(0.8);
                }
            }
        }

        div {
            position: absolute;
            left: 0.5rem;
            transition:
                color 100ms ease-in-out,
                transform 100ms ease-in-out,
                top 100ms ease-in-out;
            background-color: var(--parent-bg, var(--neutral-1));
            padding: 0 0.2rem;
            top: 0%;
            transform: translateY(-50%) scale(0.8);
            color: var(--neutral-10);
            pointer-events: none;
            user-select: none;
        }

        :global(.native) {
            outline: none;
            background-color: transparent;
            border: none;
            height: var(--height-2);
            width: 100%;
            color: var(--neutral-12);
            &::selection {
                background: var(--main-6);
            }
            &::placeholder {
                color: transparent;
                transition: all 200ms ease-in-out;
                pointer-events: none;
                user-select: none;
            }
        }

        :global(.autocompleter) {
            height: 7rem;
            margin-top: 0.5rem;
        }

        textarea {
            height: 7rem !important;
            resize: none;
        }

        &.error {
            border-color: var(--red-11);
            div {
                color: var(--red-11);
            }
        }
    }
</style>
