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
    MovePlayerRequest,
    Movement,
    MovementVector,
    Position,
    Score,
  } from "$lib/messages";
  import { onMount } from "svelte";
  import GameCanvas from "./GameCanvas.svelte";
  import BouncingBall from "./BouncingBall.svelte";
  import { v4 as uuidv4 } from "uuid";

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
  export let playerNickname: string;
  export let oponentNicknae: string;
  export let onGameFinished: () => void;

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
  let unackedActions: MovePlayerRequest[] = [];

  let framesInSec = 0;
  let startMs = Date.now();
  let gotFirstUpdate = false;

  let entityInterpolation: {state: InnerState, ts: number}[] = [];

  $: winner = (): "player" | "opponent" | undefined => {
    const calc = () => {
      if (!innerState.score?.winner) return undefined;
      if (innerState.score.winner == playerId) return "player";
      else return "opponent";
    };
    return calc();
  };
  $: playerIsRight = () => innerState.score?.rightPlayer.player == playerId;
  $: scoreView = () => {
    if (!innerState.score) return undefined;
    if (!playerIsRight()) {
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
  $: nicknameBySide = (
    side: "left" | "right",
  ): { nickname: string; color: string } => {
    const player = { nickname: playerNickname, color: "text-green-400" };
    const opponent = { nickname: oponentNicknae, color: "text-black" };

    if (side == "right") {
      if (playerIsRight()) return player;
      else return opponent;
    } else {
      if (!playerIsRight()) return player;
      else return opponent;
    }
  };

  onMount(async () => {
    SubscribeToServerMessages(ws, (message) => {
      if (winner()) return;
      const state = message.serverPushUpdate?.gameStateChange;
      if (!state) return;

      const now = Date.now();
      framesInSec += 1;
      if (now - startMs >= 1000) {
        console.log("server fps:", framesInSec);
        framesInSec = 0;
        startMs = now;
      }

      console.log("lag:", Date.now() - state.timestampMs);
      console.log(
        "since last update:",
        state.timestampMs - lastServerTimestamp,
      );

      let serverInnerState: InnerState = {...innerState}
      const positions = getPositions(state.state);
      
      // player prediction + reconcilliation
      if (!gotFirstUpdate) {
        serverInnerState.playerPosition = positions.player;
        gotFirstUpdate = true;
      } else {
        unackedActions = unackedActions.filter(action => !state.state.recentHandledActions.includes(action.actionId));
        console.log('unacked:', unackedActions.length);
        for (const action of unackedActions) {
          serverInnerState.playerPosition.y += action.yDelta;
        }
      }

      // here entity interpolation is needed
      //serverInnerState.oponentPosition = positions.oponent;
      //serverInnerState.ballPosition = state.state.ballPos.position;
      serverInnerState.ballMovement = state.state.ballPos.movement;

      serverInnerState.ballRadius = state.state.ballPos.radius;
      serverInnerState.countdown = state.state.countdown;
      serverInnerState.playerDimensions = state.state.player1Pos.dimensions;
      serverInnerState.score = state.state.score;
      lastServerTimestamp = state.timestampMs;
      
      entityInterpolation.push({state: {...serverInnerState, oponentPosition: positions.oponent, ballPosition: state.state.ballPos.position}, ts: Date.now()})
      // apply after entityInterpolation
      innerState = serverInnerState;

      //reCalculateStateOnServerUpdate(lastServerTimestamp, serverInnerState);
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
    if (winner()) return;
    // const newState = clientStateCalculation(innerState);
    // innerState.ballPosition = newState.ballPosititon;
    // innerState.ballMovement = newState.ballMovement;

    const render_timestamp = Date.now() - 1000/20;
    // Drop older positions.
    console.log('interpolation length', entityInterpolation.length)
    while (entityInterpolation.length >= 2 && entityInterpolation[1].ts <= render_timestamp) {
      entityInterpolation.shift();
    }
    // Interpolate between the two surrounding authoritative positions.
    if (entityInterpolation.length >= 2 && entityInterpolation[0].ts <= render_timestamp && render_timestamp <= entityInterpolation[1].ts) {
      const a = entityInterpolation[0];
      const b = entityInterpolation[1];
      // entityInterpolation.shift();
      // entityInterpolation.shift();
      function interpolation(x0: number, x1: number, t0: number, t1: number, now: number) {
        return x0 + (x1 - x0) * (now - t0) / (t1 - t0);
      }
      const y = interpolation(a.state.oponentPosition.y, b.state.oponentPosition.y, a.ts, b.ts, render_timestamp);
      innerState.oponentPosition.y = y;
      const x = interpolation(a.state.oponentPosition.x, b.state.oponentPosition.x, a.ts, b.ts, render_timestamp);
      innerState.oponentPosition.x = x;
      console.log('interpolation oponent', x, y)

      const bx = interpolation(a.state.ballPosition.x, b.state.ballPosition.x, a.ts, b.ts, render_timestamp)
      const by = interpolation(a.state.ballPosition.y, b.state.ballPosition.y, a.ts, b.ts, render_timestamp)
      innerState.ballPosition.x = bx;
      innerState.ballPosition.y = by;
    } else {
      //console.log('interpolation skipped'); 
      // innerState.ballPosition.x += innerState.ballMovement.horizontalVector;
      // innerState.ballPosition.y += innerState.ballMovement.verticalVector;
    }

    // if (entityInterpolation.length >= 2) {
    //   const a = entityInterpolation[0];
    //   const b = entityInterpolation[1];
    //   entityInterpolation.shift();
    //   entityInterpolation.shift();
    //   function interpolation(x0: number, x1: number, t0: number, t1: number, now: number) {
    //     return x0 + (x1 - x0) * (now - t0) / (t1 - t0);
    //   }
    //   const y = interpolation(a.state.oponentPosition.y, b.state.oponentPosition.y, a.ts, b.ts, Date.now() - (1000.0 / 60));
    //   innerState.oponentPosition.y = y;
    //   const x = interpolation(a.state.oponentPosition.x, b.state.oponentPosition.x, a.ts, b.ts, Date.now() - (1000.0 / 60));
    //   innerState.oponentPosition.x = x;
    //   console.log('interpolation oponent', x, y)
    // } else {
    //   console.log('interpolation skipped'); 
    // }

    if (innerState.movement != "none") {
      let yDelta = 10;
      if (innerState.movement == "up") {
        yDelta *= -1;
      }
      const actionId = uuidv4();
      const request = { matchId, yDelta, actionId };
      
      const newPlayer = applyLocalAction(innerState, request);
      if (newPlayer) innerState.playerPosition = newPlayer;
    }

    lastFrames.push({
      state: { ...innerState },
      clientTimestamp: Date.now(),
    });

    

    requestAnimationFrame(handleFrame);
  }

  function applyLocalAction(state: InnerState, action: MovePlayerRequest) : Position | undefined {
    let position = {...state.playerPosition};
    let targetY = position.y + action.yDelta;
    if (targetY < 0) return undefined;
    if (targetY + state.playerDimensions[1] > state.canvasDimension[1]) return undefined;

    position.y = targetY;
    
    unackedActions.push(action);
    SendUserMessageWithoutResponses(ws, {
        movePlayerRequest: action,
      });
    return position;
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

    const last = {...lastFrames[lastFrames.length-1]};

    const closest = filtered.sort(
      (a, b) => a.clientTimestamp - b.clientTimestamp,
    )[0];
    console.log(`closest and server: timestamp diff ${closest.clientTimestamp-serverTimestamp}, x diff ${closest.state.ballPosition.x-serverState.ballPosition.x}, y diff: ${closest.state.ballPosition.x-serverState.ballPosition.y}`)
    const index = lastFrames.indexOf(closest);

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
    const newLast = lastFrames[lastFrames.length-1];
    
    //const newPlayer = applyLocalActions(last.state, 'all');
    //innerState.playerPosition = newPlayer;

    
    innerState = JSON.parse(JSON.stringify(newLast.state));
    lastFrames = []; // could be causingg the bug
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

<div class="flex flex-row gap-x-80 font-bold text-lg">
  <div>
    <p class={nicknameBySide("left").color}>
      {nicknameBySide("left").nickname}
    </p>
  </div>
  <div>
    <p class={nicknameBySide("right").color}>
      {nicknameBySide("right").nickname}
    </p>
  </div>
</div>
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
  winner={winner()}
/>
{#if winner()}
  <button on:click={onGameFinished}>Home</button>
{/if}
