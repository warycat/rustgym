export function constraints() {
    return {
        video: {
            width: {ideal: 320},
            height: {ideal: 240}
        },
        audio: true,
    };
}

export function ice_servers() {
    let servers = [{urls:["stun:stun.l.google.com:19302"]}];
    return servers;
}
