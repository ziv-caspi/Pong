import type { PageLoad } from './$types';

export const ssr = false;
export const prerender = true

export const load = (async () => {
    console.log('page loading')
    let websocketClient;
    websocketClient = new WebSocket('wss://' + location.hostname + ':5000');
    
    return {
        ws: websocketClient
    };
}) satisfies PageLoad;