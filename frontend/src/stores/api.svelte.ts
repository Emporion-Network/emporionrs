import type {Notification} from "../../../backend/bindings/Notification";
import type {NonceReq} from "../../../backend/bindings/NonceReq";
import type {NonceRes} from "../../../backend/bindings/NonceRes";
import type {Prices} from "../../../backend/bindings/Prices";
import type {TokenRes} from "../../../backend/bindings/TokenRes";
import type {TokenReq} from "../../../backend/bindings/TokenReq";
import type {Error as Err} from "../../../backend/bindings/Error";

import { storage } from "./localStorage";



class Res<T> {
    #v:[null,Err] | [T, null];

    private constructor(v:[null,Err] | [T, null]){
        this.#v = v;
    }
    static Err<T>(e:Err){
        return new Res<T>([null, e]) as unknown as Res<T>;
    }
    static Ok<T>(v:T){
        return new Res<T>([v, null]) as unknown as Res<T>;
    }
    static Wrp<T>(v:T):Res<NonNullable<T>>{
        if(v === undefined || v == null) {
            return Err<NonNullable<T>>({message:"Undefined"})
        }
        return Ok<NonNullable<T>>(v);
    }

    unwrap(){
        if(this.#v[0]){
            return this.#v[0]
        }
        throw this.#v[1];
    }
    is_ok(){
        return this.#v[1] == null;
    }
    expect(err:string){
        if(this.#v[0]){
            return this.#v[0]
        }
        throw {
            message:err
        };
    }

    map<R>(f:(v:T)=>R):Res<R>{
        if(this.#v[1] !== null){
            return this as unknown as Res<R>;
        }
        return Res.Ok(f(this.#v[0]))
    }

    map_err(f:(v:string)=>string):Res<T>{
        if(this.#v[1] !== null){
            return Res.Err({message:f(this.#v[1].message)})
        }
        return this as unknown as Res<T>;
    }
    static try<T>(fn:()=>Promise<Res<T>>):Promise<Res<T>>;
    static try<T>(fn:()=>Res<T>):Res<T>;

    static try<T>(fn:Function){
        try {
            const res = fn();
            if(res instanceof Promise) {
                return res.catch(e => {
                    return Err<T>({
                        message:"message" in e ? String(e.message) : "Unknown error"
                    });
                })
            }
            return res;
        } catch(e:any){
            return Err<T>({
                message:"message" in e ? String(e.message) : "Unknown error"
            });
        }
    }
};

const {
    Ok,
    Err,
    Wrp,
} = Res;

const DEFAULT_CHAIN = "cosmoshub-4";

export class API {
    #root:string;
    #jwt:ReturnType<typeof storage<string>>
    state:{
        address:string;
        notifications:Notification[];
    }|null = $state(null);
    #ws:WebSocket|null;


    constructor(root:string){
        this.#jwt = storage("token");
        this.#root = root;
        this.#ws = null;
    }
    async get<R>(url:string){
        return Res.try(async ()=>{
            const req =  await fetch(new URL(url, this.#root), {
                method:"GET",
                headers: {
                    'Authorization': `Bearer ${this.#jwt.get()}`,
                },
            });
            const res = await req.json();
            if(req.ok){
                return Ok<R>(res);
            };
            return Err<R>(res);
        });
    }

    async post<T, R>(url:string, body:T):Promise<Res<R>>{
        return Res.try(async ()=>{
            const req = await fetch(new URL(url, this.#root), {
                method:"POST",
                body: body instanceof FormData ? body : JSON.stringify(body),
                headers: {
                    'Authorization': `Bearer ${this.#jwt.get()}`,
                    ...(body instanceof FormData ?  {} : {"Content-Type": "application/json"}),
                },
            });
            const res = await req.json();
            if(req.ok){
                return Ok(res);
            };
            return Err(res);
        })
    }

    async getNonce(address:string){
        return(await this.post<NonceReq, NonceRes>('/nonce', {
            address
        })).map(v => v.nonce);
    }

    async translate(v:{translations:Record<string, string>}){
        return await this.post<
        {translations:Record<string, string>},
        Record<string, string>
        >("/translate", v);
    }

    async check(){
        return await this.get("/check");
    }

    async auth(){
        await Res.try(async ()=>{
            const {bech32Address} = Wrp(await window.keplr?.getKey(DEFAULT_CHAIN)).expect("need key");
            if(!(await this.check()).is_ok()){
                let nonce = (await this.getNonce(bech32Address)).unwrap();
                let signature = Wrp(await window.keplr?.signArbitrary(DEFAULT_CHAIN, bech32Address, nonce)).unwrap();
                let token = (await this.post<TokenReq, TokenRes>('/check-nonce', {
                        ...signature,
                        nonce,
                })).unwrap();
                this.#jwt.set(token.token);
            }
            this.state = {
                address:bech32Address,
                notifications:[],
            }
            return Ok(true)
        });
        const ws = new WebSocket(`${this.#root}/ws`);
        ws.addEventListener('open', ()=>{
            ws.send(JSON.stringify({
                token:this.#jwt.get()!
            } satisfies TokenRes))
        });
        ws.addEventListener('message', (m)=>{
            const notification = JSON.parse(m.data) as Notification;
            this.state?.notifications.push(notification);
        });
        this.#ws = ws;
        window.addEventListener('keplr_keystorechange', () => {
            this.state = null;
            this.#jwt.clear();
            this.auth()
            ws.close();
        }, {
            once:true
        });
        
    }

    sendMessage(text:string, to:string){
        this.#ws?.send(JSON.stringify({
            for:{message:{}},
            from:{user:this.state?.address!},
            data:{message:{
                chat_id:"1",
                message:text,
                from:this.state?.address!,
                to:to,
                date:Date.now() as unknown as bigint,
            }}
        } satisfies Notification))
    }

    logout(){
        storage<string>('token').clear()
        window.keplr?.disable(DEFAULT_CHAIN);
        this.state = null;
    }
}

