
import { translatedString } from '../../../../stores/translate.svelte';


export const defaultValue = {
    display_type: "paragraph" as const,
    trait_type: "",
    value: translatedString(),
};
export type Attribute = typeof defaultValue;

const bindClone = (a: Attribute) => {
    return {
        get display_type() { return a.display_type },
        get trait_type() { return a.trait_type },
        get value() { return a.value },
    }
}

export const meta = {
    type: defaultValue.display_type,
    defaultValue,
    label: "short_super_warthog_pick",
    bindClone
};