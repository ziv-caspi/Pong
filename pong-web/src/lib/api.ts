import type { ServerMessage, UserMessage } from "./messages"

// const websocketClient = new WebSocket('ws://localhost:5000');

const GetResponse = (ws: WebSocket) => {
    return new Promise<string>(function(resolve, reject) {
        ws.onmessage = (message) => {
            return resolve(message.data);
        }

        ws.onerror = (error) => {
            return reject(error);
        }
   });
}

export const SendUserMessage = async (ws: WebSocket, message: UserMessage): Promise<ServerMessage> => {
    const asStr = JSON.stringify(message);
    ws.send(asStr)
    const resp = await GetResponse(ws)
    let serverMessage: ServerMessage = JSON.parse(resp)
    return serverMessage;
}