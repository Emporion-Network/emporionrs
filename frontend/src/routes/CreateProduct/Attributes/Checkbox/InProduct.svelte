<script lang="ts">
    import Checkbox from "../../../../lib/Checkbox.svelte";
import TranslatableInput from "../../../../lib/TranslatableInput.svelte";
    import { getTranslator, type SupportedLanguage } from "../../../../stores/translate.svelte";
    import {type Attribute} from "./_meta";
    let { attribute = $bindable(), lang = $bindable() }: {
        attribute:Attribute,
        lang:SupportedLanguage,
    } = $props();

    let t = getTranslator();
    
</script>
<div class="input-attribute">
    <Checkbox bind:value={attribute.value}></Checkbox>
    <div>{attribute.description[lang]}</div>
</div>