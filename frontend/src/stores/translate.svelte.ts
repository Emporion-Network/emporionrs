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

export const getT = () => {
    return {
        get t() {
            return tst;
        },
        update,
    }

}