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
    serverTimestamp: number;
    offsetFrames: number;
  }[] = [];

  // $: scoreView = score
  //   ? { player: score.leftPlayer.score, oponent: score.rightPlayer.score }
  //   : undefined;
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

  let frames = 0;
  let time = Date.now();

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
      reCalculateStateOnServerUpdate(oldTs, lastServerTimestamp, innerState);
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
    // const msPerFrame = 1000 / 60;
    // let totalStart = Date.now();
    // let frames = 0;
    // let start = Date.now();

    const newState = clientStateCalculation(
      innerState.ballPosition,
      innerState.ballMovement,
    );
    innerState.ballPosition = newState.ballPosititon;
    innerState.ballMovement = newState.ballMovement;

    const last = lastFrames.findLast(() => true);
    const offset =
      last && last.serverTimestamp == lastServerTimestamp
        ? last.offsetFrames + 1
        : 0;
    const frame = {
      state: { ...innerState },
      serverTimestamp: lastServerTimestamp,
      offsetFrames: offset,
    };
    lastFrames.push(frame);

    if (innerState.movement != "none") {
      let yDelta = 10;
      if (innerState.movement == "up") {
        yDelta *= -1;
      }
      SendUserMessageWithoutResponses(ws, {
        movePlayerRequest: { matchId, yDelta },
      });
    }

    // let end = Date.now();
    // const diff = end - start;
    // const spareFrameTime = msPerFrame - diff;
    // if (spareFrameTime > 0) {
    //   //await new Promise((resolve) => setTimeout(resolve, spareFrameTime));
    // } else {
    //   console.log("no spare time for frame. diff", spareFrameTime);
    // }
    // frames += 1;
    // if (Date.now() - time >= 1000) {
    //   console.log("fps:", frames);
    //   frames = 0;
    //   time = Date.now();
    // }
    requestAnimationFrame(handleFrame);
  }

  function clientStateCalculation(
    ballPosition: Position,
    ballMovement: MovementVector,
  ): { ballPosititon: Position; ballMovement: MovementVector } {
    const newBallPosition: Position = {
      x: ballPosition.x + ballMovement.horizontalVector,
      y: ballPosition.y + ballMovement.verticalVector,
    };

    let newBallMovement = { ...ballMovement };
    if (
      newBallPosition.y >= innerState.canvasDimension[1] ||
      newBallPosition.y <= 0
    ) {
      newBallMovement.verticalVector *= -1;
    }

    // if (
    //   newBallPosition.x >= innerState.canvasDimension[0] ||
    //   newBallPosition.x <= 0
    // ) {
    //   newBallMovement.horizontalVector *= -1;
    // }

    return { ballPosititon: newBallPosition, ballMovement: newBallMovement };
  }

  function reCalculateStateOnServerUpdate(
    lastServerTimestamp: number,
    newServerTimestamp: number,
    serverState: InnerState,
  ) {
    // handle first update
    if (lastFrames.every((frame) => frame.serverTimestamp == -1)) {
      console.log("FIRST UDPATE!!!!!");
      const missedFrames = lastFrames.length;
      // not good enough but just for now
      for (const frame of lastFrames) {
        frame.serverTimestamp = newServerTimestamp;
      }
      console.log("modified first frames count:", missedFrames);
      return;
    }

    // trying to find a calculated frame where (ts + (frames*16ms) ~= targetTimestamp). that is the frame I want to re-run from.
    const index = lastFrames.findIndex(
      (frame) =>
        frame.serverTimestamp != -1 &&
        Math.abs(
          frame.serverTimestamp + frame.offsetFrames * 16 - newServerTimestamp,
        ) <= 30,
    );
    console.log(
      "local frame that server is referring to stored at:",
      index,
      "delta from end:",
      lastFrames.length - 1 - index,
    );
    if (index == -1) return;
    for (let i = index; i < lastFrames.length; i++) {
      const frame = lastFrames[i];
      frame.serverTimestamp = newServerTimestamp;
      frame.offsetFrames = i - index;

      if (i == index) {
        frame.state = serverState;
        continue;
      }

      const prevFrame = lastFrames[i - 1];
      const newState = clientStateCalculation(
        prevFrame.state.ballPosition,
        prevFrame.state.ballMovement,
      );
      frame.state.ballPosition = newState.ballPosititon;
      frame.state.ballMovement = newState.ballMovement;
    }

    // TODO: something is NOT working
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

<!-- {#if countdown > 0}
  <p>{countdown}</p>
{:else}
  <p>player: {JSON.stringify(playerPosition)}</p>
  <p>oponentt: {JSON.stringify(oponentPosition)}</p>
  <p>ball: {JSON.stringify(ballPosition)}</p>
  <p>radius: {ballRadius}</p>
{/if} -->

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
