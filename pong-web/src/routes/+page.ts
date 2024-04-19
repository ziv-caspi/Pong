import type { PageLoad } from './$types';

export const ssr = false;

export const load = (async () => {
    console.log('page loading')
    let websocketClient;
    websocketClient = new WebSocket('ws://' + location.hostname + ':5000');
    
    return {
        ws: websocketClient
    };
}) satisfies PageLoad;