<script lang="ts">
    import Input from "../../../../lib/Input.svelte";
    import type { SupportedLanguage } from "../../../../stores/translate.svelte";
    import {type Attribute} from "./_meta";
    let { attribute = $bindable() } : {
        attribute:Attribute,
    } = $props();

    let element:HTMLElement = $state()!;

    export const actions = {
        setValue(v:string){
            attribute.trait_type = v;
        }
    }
    export {
        element
    }
</script>
<div class="attribute" bind:this={element}>
    <Input type="text" label="Trait type" placeholder="Trait type" bind:value={attribute.trait_type}/>
</div>