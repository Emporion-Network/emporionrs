import { fromBech32, toBech32 } from "@cosmjs/encoding"

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

export function bechToBech(address: string, prefix: string) {
    return toBech32(prefix, fromBech32(address).data)
}

export function getImageData(src: string): Promise<ImageData> {
    return new Promise(resolve => {
        let c = document.createElement('canvas');
        c.width = 200;
        c.height = 200;
        let ctx = c.getContext('2d')!;
        let i = new Image();
        i.crossOrigin = "";
        i.onload = () => {
            ctx.drawImage(i, 0, 0, 200, 200);
            resolve(ctx.getImageData(0, 0, 200, 200));
        }
        i.src = src;
    })
}

export async function getAvgColor(src: string) {
    let { data } = await getImageData(src);
    let color = [
        data[0],
        data[1],
        data[2],
    ];
    for (let i = 0; i < data.length - 4; i += 4) {
        color = [
            color[0] + data[i + 0],
            color[1] + data[i + 1],
            color[2] + data[i + 2],
        ]
    }
    return [
        Math.floor(color[0] / (data.length / 4)),
        Math.floor(color[1] / (data.length / 4)),
        Math.floor(color[2] / (data.length / 4)),
    ]
}

export type WithSkeleton<T> = { skeleton: true } | (T & { skeleton?: false });

export const getKeys = Object.keys as <T extends object>(obj: T) => Array<keyof T>;

export const wait = async (ms: number) => new Promise(resolve => setTimeout(resolve, ms))

export const typeStr = async (v:string, cb:(v:string)=>void, speed=50)=>{
    let acc = ""
    for(let i of v){
        acc +=i;
        cb(acc);
        await wait(10 + Math.floor(Math.random() * speed));
    }
}

export const pickImages = (multiple:boolean = true) => {
    return new Promise<File[]>((resolve) => {
        let el = document.createElement('input');
        el.type = "file"
        el.multiple = multiple;
        el.accept = "image/*";
        el.onchange = (evt: any) => {
            const newfiles = Array.from(evt.currentTarget?.files || []) as File[];
            resolve(newfiles);
        }
        el.click();
    })
}