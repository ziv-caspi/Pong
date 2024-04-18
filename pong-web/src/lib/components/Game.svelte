<script lang="ts">
  import {
    SendNoUpdates,
    SendUserMessage,
    SendUserMessageWithoutResponses,
    SubscribeToServerMessages,
  } from "$lib/api";
  import type {
    Dimensions,
    GameState,
    GameStateChange,
    Movement,
    MovementVector,
    Position,
    Score,
  } from "$lib/messages";
  import { onMount } from "svelte";
  import GameCanvas from "./GameCanvas.svelte";

  type InnerState = {
    playerPosition: Position;
    oponentPosition: Position;
    ballPosition: Position;
    ballMovement: MovementVector;
    ballRadius: number;
    countdown: number;
    playerDimensions: Dimensions;
    movement: Movement;
    score: Score | undefined;
    canvasDimension: Dimensions;
  };

  export let matchId: string;
  export let playerId: string;
  export let ws: WebSocket;

  let innerState: InnerState = {
    playerPosition: { x: -100, y: -100 },
    oponentPosition: { x: -100, y: -100 },
    ballPosition: { x: -100, y: -100 },
    ballMovement: {
      horizontalVector: 0,
      verticalVector: 0,
    },
    ballRadius: 0,
    countdown: -1,
    playerDimensions: { "0": 1, "1": 1 },
    movement: "none",
    score: undefined,
    canvasDimension: { "0": 667, "1": 300 },
  };
  let lastServerTimestamp = -1;
  let lastFrames: {
    state: InnerState;
    clientTimestamp: number;
  }[] = [];

  $: scoreView = () => {
    if (!innerState.score) return undefined;
    if (innerState.score.leftPlayer.player == playerId) {
      return {
        player: innerState.score.leftPlayer.score,
        oponent: innerState.score.rightPlayer.score,
        playerIsRight: false,
      };
    }

    return {
      player: innerState.score.rightPlayer.score,
      oponent: innerState.score.leftPlayer.score,
      playerIsRight: true,
    };
  };

  onMount(async () => {
    SubscribeToServerMessages(ws, (message) => {
      const state = message.serverPushUpdate?.gameStateChange;
      if (!state) return;
      console.log("lag:", Date.now() - state.timestampMs);
      const oldTs = lastServerTimestamp;
      const positions = getPositions(state.state);
      innerState.playerPosition = positions.player;
      innerState.oponentPosition = positions.oponent;
      innerState.ballPosition = state.state.ballPos.position;
      innerState.ballRadius = state.state.ballPos.radius;
      innerState.countdown = state.state.countdown;
      innerState.playerDimensions = state.state.player1Pos.dimensions;
      innerState.ballMovement = state.state.ballPos.movement;
      innerState.score = state.state.score;
      lastServerTimestamp = state.timestampMs;
      reCalculateStateOnServerUpdate(lastServerTimestamp, innerState);
    });

    callbackGameLoop((state) => {
      // not needed anymore
    });
  });

  function callbackGameLoop(onStateChange: (state: GameStateChange) => void) {
    new Promise(async (resolve, reject) => {
      handleFrame();
    });
  }

  function handleFrame() {
    const newState = clientStateCalculation(innerState);
    innerState.ballPosition = newState.ballPosititon;
    innerState.ballMovement = newState.ballMovement;

    lastFrames.push({
      state: { ...innerState },
      clientTimestamp: Date.now(),
    });

    if (innerState.movement != "none") {
      let yDelta = 10;
      if (innerState.movement == "up") {
        yDelta *= -1;
      }
      SendUserMessageWithoutResponses(ws, {
        movePlayerRequest: { matchId, yDelta },
      });
    }

    requestAnimationFrame(handleFrame);
  }

  function clientStateCalculation(state: InnerState): {
    ballPosititon: Position;
    ballMovement: MovementVector;
  } {
    let newBallMovement = { ...state.ballMovement };
    if (
      state.ballPosition.y + innerState.ballRadius >=
        innerState.canvasDimension[1] ||
      state.ballPosition.y - innerState.ballRadius <= 0
    ) {
      newBallMovement.verticalVector *= -1;
    }

    const [left, right] = [state.playerPosition, state.oponentPosition].sort(
      (a, b) => a.x - b.x,
    );

    const rightPlayerCollision =
      newBallMovement.horizontalVector > 0 &&
      state.ballPosition.x + innerState.ballRadius >= right.x &&
      state.ballPosition.x <= right.x + innerState.playerDimensions[0] &&
      state.ballPosition.y + innerState.ballRadius >= right.y &&
      state.ballPosition.y <= right.y + innerState.playerDimensions[1];

    const leftPlayerCOllision =
      newBallMovement.horizontalVector < 0 &&
      state.ballPosition.x - innerState.ballRadius <= left.x &&
      state.ballPosition.x >= left.x + innerState.playerDimensions[0] &&
      state.ballPosition.y + innerState.ballRadius >= left.y &&
      state.ballPosition.y <= left.y + innerState.playerDimensions[1];

    if (rightPlayerCollision || leftPlayerCOllision) {
      newBallMovement.horizontalVector *= -1;
    }

    const x = state.ballPosition.x + newBallMovement.horizontalVector;
    const y = state.ballPosition.y + newBallMovement.verticalVector;
    const newBallPosition: Position = {
      x: x,
      y: y >= 0 ? y : 0,
    };

    return { ballPosititon: newBallPosition, ballMovement: newBallMovement };
  }

  function reCalculateStateOnServerUpdate(
    serverTimestamp: number,
    serverState: InnerState,
  ) {
    // const index = lastFramesV2.findIndex(
    //   (frame) => Math.abs(frame.clientTimestamp - serverTimestamp) <= 10,
    // );

    const filtered = lastFrames.filter(
      (frame) => Math.abs(frame.clientTimestamp - serverTimestamp) <= 10,
    );
    if (filtered.length == 0) return;

    const closest = filtered.sort(
      (a, b) => a.clientTimestamp - b.clientTimestamp,
    )[0];
    const index = lastFrames.indexOf(closest);
    console.log(
      "matching frames:",
      filtered.length,
      "closest:",
      closest,
      "is exact time:",
      closest.clientTimestamp == serverTimestamp,
      "is exact state:",
      closest.state == serverState,
    );

    for (let i = index; i < lastFrames.length; i++) {
      const frame = lastFrames[i];
      if (i == index) {
        frame.state = serverState;
        continue;
      }

      const prevFrame = lastFrames[i - 1];
      const newState = clientStateCalculation(prevFrame.state);
      frame.state.ballPosition = newState.ballPosititon;
      frame.state.ballMovement = newState.ballMovement;
    }
    lastFrames = [];
  }

  function getPositions(state: GameState): {
    player: Position;
    oponent: Position;
  } {
    if (playerId === state.player1Pos.id) {
      return {
        player: state.player1Pos.position,
        oponent: state.player2Pos.position,
      };
    } else {
      return {
        player: state.player2Pos.position,
        oponent: state.player1Pos.position,
      };
    }
  }
</script>

<GameCanvas
  playerPosition={innerState.playerPosition}
  oponentPosition={innerState.oponentPosition}
  ballPosition={innerState.ballPosition}
  ballRadius={innerState.ballRadius}
  countdown={innerState.countdown}
  score={scoreView()}
  onPlayerMovementChange={(pmovement) => {
    if (innerState.movement != pmovement) {
      console.log(pmovement);
      innerState.movement = pmovement;
    }
  }}
  canvasDimennsion={innerState.canvasDimension}
  playerDimensions={innerState.playerDimensions}
/>
