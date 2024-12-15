import {fromBech32, toBech32} from "@cosmjs/encoding"

export function ngrams(str: string) {
    return str.split(/\B|\b/u).reduce((acc, c) => {
        let ret = "";
        if (acc[acc.length - 1]) {
            ret += acc[acc.length - 1];
        }
        ret += c.toLocaleLowerCase();
        acc.push(ret);
        return acc;
    }, [] as string[]);
};

export function findMap<T, R>(arr: T[], cb: (v: T) => R) {
    for (let i of arr) {
        let ret = cb(i);
        if (ret) return ret;
    }
}

export function bechToBech(address:string, prefix:string){
    return toBech32(prefix, fromBech32(address).data)
}