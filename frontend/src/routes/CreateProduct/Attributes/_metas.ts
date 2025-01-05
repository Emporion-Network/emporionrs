import { meta as buttonMeta, type Attribute as ButtonAttribute  } from './Buttons/_meta';
import { meta as checkboxMeta, type Attribute as CheckboxAttribute } from './Checkbox/_meta';
import { meta as selectMeta, type Attribute as SelectAttribute } from './Select/_meta';
import { meta as colorMeta, type Attribute as ColorAttribute } from './Color/_meta';


export const metas = {
    [buttonMeta.type]: buttonMeta,
    [checkboxMeta.type]: checkboxMeta,
    [selectMeta.type]: selectMeta,
    [colorMeta.type]:colorMeta
};

export type Attribute = ButtonAttribute | CheckboxAttribute | SelectAttribute | ColorAttribute;