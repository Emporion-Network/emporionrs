//@ts-check

const signMessage = async (nonce) => {
    // @ts-ignore
    if (!window.keplr) return;
    // @ts-ignore
    const key = await window.keplr.getKey('cosmoshub-4')
    // @ts-ignore
    const signature = await window.keplr.signArbitrary('cosmoshub-4', key.bech32Address, nonce);
    return {
        ...signature,
        nonce,
    };
}

const post = async (url, body)=> {
    const req = await fetch(url, {
        method: "POST",
        headers: {
            "Content-Type": "application/json"
        },
        body: body instanceof FormData ? body : JSON.stringify(body),
    });
    return req.json()
}

const get = async (url, body)=> {
    const req = await fetch(url, {
        method: "GET",
        headers: {
            "Content-Type": "application/json"
        },
        body: body instanceof FormData ? body : JSON.stringify(body),
    });
    return req.json()
}



let b = document.createElement('button');
b.innerText = "sign"

let send = document.createElement('button');
send.innerText = "send"

let txt = document.createElement('textarea');

let to = document.createElement('input');

let messages = document.createElement('div');




document.body.append(messages);
document.body.append(b);
document.body.append(to);
document.body.append(txt);
document.body.append(send);



const createmessage = (message)=>{
    let me = document.createElement('div');
    me.innerText = message.data.message.message;
    messages.append(me)
}


/**
 * @type {WebSocket}
 */
let ws;
/**
 * @type {string}
 */
let address;
b.onclick = async ()=>{
    // @ts-ignore
    const key = await window.keplr.getKey('cosmoshub-4');
    address = key.bech32Address;
    let {nonce}  = await post("http://localhost:3000/nonce", {
        address,
    })
    let signature = await signMessage(nonce);
    let token = await post('http://localhost:3000/check-nonce', signature)
    console.log(token);
    ws = new WebSocket("http://localhost:3000/ws");
    ws.addEventListener('open', ()=>{
        ws.send(JSON.stringify(token))
    })
    ws.addEventListener("message", (data)=>{
        let message = JSON.parse(data.data);
        createmessage(message);
    })
}


send.onclick = ()=>{
    let v = txt.value;
    const message = {
        from:{user:address},
        for:{message:{}},
        data:{
            message:{
                from:address,
                to:to.value,
                message:v,
                chat_id:'hello',
                date:Date.now(),
            }
        }
    }
    createmessage(message);
    ws.send(JSON.stringify(message))
}



// let ws = new WebSocket("http://localhost:3000/ws");
