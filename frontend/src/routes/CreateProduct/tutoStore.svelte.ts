import type {SvelteComponent } from "svelte";


let registry:Record<string, SvelteComponent|HTMLElement> = $state({});

export const getTutoRegistry = ()=>{
    return registry;
}