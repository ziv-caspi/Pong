export type UserMessage = {
    noUpdates?: NoUpdates
    queueUpRequest?: QueueUpRequest
    joinLobbyRequest?: JoinLobbyRequest;
    movePlayerRequest?: MovePlayerRequest;
}

export type ServerMessage = {
    queueUpResponse?:  QueueUpResponse
    joinLobbyResponse?: JoinLobbyResponse
    serverPushUpdate?: ServerPushUpdate
}

export type QueueUpRequest = {
    nickname: string
}

export type QueueUpResponse = {
    Ok: {id: string},
}

export type NoUpdates = 'noUpdates'

export type ServerPushUpdate = {
    potentialMatchUpdate?: PotentialMatchUpdate,
    matchStatusChange?: MatchStatusChange
    gameStateChange?: GameStateChange
}

export type PotentialMatchUpdate = {
    matchId: string,
    opoonentsIds: string[]
}

export type JoinLobbyRequest = {
    matchId: string
}

export type JoinLobbyResponse = {
    Ok: {matchId: string}
}

export type MatchStatusChange = {
    start: string[]
    stop: string
}

export type GameStateChange = {
    id: string,
    state: GameState
}

export type GameState = {
    player1Pos: Player,
    player2Pos: Player,
    ballPos: BallInfo,
    countdown: number,
}

export type Player = {
    id: String,
    position: Position,
    dimensions: Dimensions,
}

export type MovePlayerRequest = {
    matchId: string,
    yDelta: number,
}

export type Position = { x: number; y: number };
export type  Dimensions = {0: number, 1: number};
export type BallInfo = {
    position: Position,
    radius: number,
}

export type Movement = 'up' | 'down' | 'none'