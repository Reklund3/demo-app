const invoke = window.__TAURI__.invoke

export async function invokeGreet(name) {
    return await invoke("greet", {name: name})
}

export async function invokeGetPost(id) {
    return await invoke("get_post", {id: id});
}