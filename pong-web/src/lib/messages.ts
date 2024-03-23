export type UserMessage = {
    noUpdates?: NoUpdates
    queueUpRequest?: QueueUpRequest
}

export type ServerMessage = {
    queueUpResponse:  QueueUpResponse
}

export type QueueUpRequest = {
    nickname: string
}

export type QueueUpResponse = {
    id: string
}

export type NoUpdates = 'noUpdates'