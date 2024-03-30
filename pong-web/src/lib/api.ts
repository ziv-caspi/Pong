import type { PotentialMatchUpdate, ServerMessage, UserMessage } from "./messages"

// const websocketClient = new WebSocket('ws://localhost:5000');

const RecvResponse = (ws: WebSocket) => {
    return new Promise<ServerMessage>(function(resolve, reject) {
        ws.onmessage = (message) => {
            return resolve(JSON.parse(message.data));
        }

        ws.onerror = (error) => {
            return reject(error);
        }
   });
}

export const SendUserMessage = async (ws: WebSocket, message: UserMessage): Promise<ServerMessage> => {
    const asStr = JSON.stringify(message);
    ws.send(asStr)
    const resp = await RecvResponse(ws)
    return resp;
}

export const SendNoUpdates = async (ws: WebSocket) => {
    ws.send(JSON.stringify('noUpdates'));
    const resp = await RecvResponse(ws)
    return resp;
}

export const WaitForMatchUpdate = async (ws: WebSocket): Promise<PotentialMatchUpdate> => {
    while (true) {
        let response = await SendNoUpdates(ws);
        if (response.serverPushUpdate?.potentialMatchUpdate) {
            return response.serverPushUpdate.potentialMatchUpdate
        }
    }
}