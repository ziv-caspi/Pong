import type { PotentialMatchUpdate, ServerMessage, UserMessage } from "./messages"

// const websocketClient = new WebSocket('ws://localhost:5000');

export const RecvResponse = (ws: WebSocket) => {
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

export const SendUserMessageWithoutResponses = (ws: WebSocket, message: UserMessage): void => {
    const asStr = JSON.stringify(message);
    ws.send(asStr);
}

export const SendNoUpdates = async (ws: WebSocket) => {
    ws.send(JSON.stringify('noUpdates'));
    const resp = await RecvResponse(ws)
    return resp;
}

export const WaitForMatchUpdate = async (ws: WebSocket): Promise<PotentialMatchUpdate> => {
    let response = await RecvResponse(ws);
    if (response.serverPushUpdate?.potentialMatchUpdate) {
        return response.serverPushUpdate.potentialMatchUpdate
    }
    throw new Error();
}

export const SubscribeToServerMessages = (ws: WebSocket, callback: (message: ServerMessage) => void) => {
    ws.onmessage = (message) => {
        callback(JSON.parse(message.data));
    }
}