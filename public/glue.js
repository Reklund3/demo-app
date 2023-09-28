const invoke = window.__TAURI__.invoke

export async function invokeGreet(name) {
    return await invoke("greet", {name: name})
}

export async function invokeCreatePost(user_id, body) {
    return await invoke("create_post", {user_id: user_id, body: body});
}

export async function invokeGetPost(id) {
    return await invoke("get_post", {id: id});
}