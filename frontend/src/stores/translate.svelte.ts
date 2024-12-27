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
    "en":"caring_last_jellyfish_tap", 
    "es":"antsy_pretty_jannes_scoop", 
    "fr":"round_many_guppy_comfort", 
    "ko":"round_noble_swan_slurp", 
    "pl":"fit_cozy_platypus_hope", 
    "pt-br":"just_sweet_swallow_fry", 
    "ro":"born_house_bulldog_sprout", 
    "tr":"raw_zany_gorilla_quiz", 
    "zh-cn":"game_moving_salmon_nail", 
    "zh-tw":"sleek_full_felix_build", 
    "zh-hk":"spare_wise_goat_dial", 
    "fa":"close_busy_boar_dig", 
    "ja":"caring_inclusive_tortoise_link", 
    "de":"next_patient_pony_hint", 
    "hi":"weak_hour_dragonfly_dial", 
    "ru":"zesty_lower_bumblebee_pinch", 
    "gu":"bright_real_lemur_emerge", 
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