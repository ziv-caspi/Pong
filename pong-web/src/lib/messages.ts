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
    opoonentsIds: PotentialPlayer[]
}

export type PotentialPlayer = {
    id: string,
    nickname: string
}

export type JoinLobbyRequest = {
    matchId: string
}

export type JoinLobbyResponse = {
    Ok: {matchId: string}
}

export type MatchStatusChange = {
    start?: string[]
    stop?: string
    playerReady?: string
}

export type GameStateChange = {
    id: string,
    state: GameState,
    timestampMs: number,
}

export type GameState = {
    player1Pos: Player,
    player2Pos: Player,
    ballPos: BallInfo,
    countdown: number,
    score: Score,
    recentHandledActions: string[],
}

export type Player = {
    id: String,
    position: Position,
    dimensions: Dimensions,
}

export type MovePlayerRequest = {
    matchId: string,
    yDelta: number,
    actionId: string,
}

export type Position = { x: number; y: number };
export type  Dimensions = {0: number, 1: number};
export type MovementVector = {
    horizontalVector: number,
    verticalVector: number,
};

export type BallInfo = {
    position: Position,
    radius: number,
    movement: MovementVector
}

export type Movement = 'up' | 'down' | 'none'

export type Score = {
    leftPlayer: PlayerWithScore,
    rightPlayer: PlayerWithScore,
    winner: {some?: string},
}


export type PlayerWithScore = {
    player: string,
    score: number,
}