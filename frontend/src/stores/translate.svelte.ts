import { type msg, langs } from "../../messages.d.js";
import { I18n } from "i18n-js";
import en from "../../../messages/en.json";

const ini = $state(new I18n({
    en:en
}, {
    locale:"en"
}));

let tst = $state((text: msg) => {
    return ini.t(text)
});


const update = (newLang: string) => {
    import(`../../../messages/${newLang}.json`).then(e => {
        ini.locale = newLang;
        ini.store({
            [newLang]: e,
        })
        tst = (text: msg) => {
            return ini.t(text)
        };
    });
}

export const getTranslator = () => {
    return {
        get t() {
            return tst;
        },
        update,
    }

}


export const Languages = {
    "en":"English", 
    "es":"Español", 
    "fr":"Français", 
    "ko":"한국어", 
    "pl":"Polski", 
    "pt-br":"Portuguese", 
    "ro":"Romana", 
    "tr":"Türkçe", 
    "zh-cn":"简体中文", 
    "zh-tw":"正體中文", 
    "zh-hk":"香港語", 
    "fa":"فارسی", 
    "ja":"日本語", 
    "de":"Deutsch", 
    "hi":"हिन्दी", 
    "ru":"Русский", 
    "gu":"ગુજરાતી", 
} as const satisfies {
    [k in typeof langs[number]]: string;
};

export type SupportedLanguage = typeof langs[number];

export type T<K> = {
    -readonly [k in keyof typeof Languages]: K;
}

export const translatedString = () => {
    return langs.reduce((acc, lang) => {
        acc[lang] = "";
        return acc;
    }, {} as T<string>)
}