import type { T } from '../../../stores/translate.svelte';
import { meta as buttonMeta, type Attribute as ButtonAttribute  } from './Buttons.svelte';
import { meta as checkboxMeta, type Attribute as CheckboxAttribute } from './Checkbox.svelte';
import { meta as selectMeta, type Attribute as SelectAttribute } from './Select.svelte';

export const metas = {
    [buttonMeta.type]: buttonMeta,
    [checkboxMeta.type]: checkboxMeta,
    [selectMeta.type]: selectMeta,
};

export type Attribute = ButtonAttribute | CheckboxAttribute | SelectAttribute;