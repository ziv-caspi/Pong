<script lang="ts">
  import { SendNoUpdates, SendUserMessage } from "$lib/api";
  import type {
    Dimensions,
    GameState,
    GameStateChange,
    Movement,
    Position,
  } from "$lib/messages";
  import { onMount } from "svelte";
  import GameCanvas from "./GameCanvas.svelte";

  export let matchId: string;
  export let playerId: string;
  export let ws: WebSocket;

  const canvasDimennsion: Dimensions = { "0": 667, "1": 300 };
  const playerDimensions: Dimensions = { "0": 5, "1": 75 };

  let playerPosition: Position = { x: canvasDimennsion[0] / 2 - 200, y: 100 };
  let oponentPosition: Position = { x: canvasDimennsion[0] / 2 + 200, y: 100 };
  let ballPosition: Position = { x: 50, y: 50 };
  let ballRadius = 8;
  let countdown = 3;
  let movement: Movement = "none";
  let ballXDirection = 4;
  let ballYDirection = 4;

  onMount(async () => {
    callbackGameLoop((state) => {
      const positions = getPositions(state.state);
      playerPosition = positions.player;
      oponentPosition = positions.oponent;
      ballPosition = state.state.ballPos.position;
      ballRadius = state.state.ballPos.radius;
      countdown = state.state.countdown;
    });
  });

  function callbackGameLoop(onStateChange: (state: GameStateChange) => void) {
    new Promise(async (resolve, reject) => {
      await handleFrame(onStateChange);
    });
  }

  async function handleFrame(onStateChange: (state: GameStateChange) => void) {
    // const msPerFrame = 1000 / 60;
    // let totalStart = Date.now();
    // let frames = 0;
    // let start = Date.now();
    const state: GameStateChange = getCurrentState();
    if (
      (ballXDirection > 0 && ballPosition.x > canvasDimennsion[0]) ||
      (ballXDirection < 0 && ballPosition.x < 0)
    ) {
      ballXDirection *= -1;
    }

    if (
      (ballYDirection > 0 && ballPosition.y > canvasDimennsion[1]) ||
      (ballYDirection < 0 && ballPosition.y < 0)
    ) {
      ballYDirection *= -1;
    }

    state.state.ballPos.position.x = ballPosition.x + ballXDirection;
    state.state.ballPos.position.y = ballPosition.y + ballYDirection;
    if (state) {
      onStateChange(state);
    }
    if (movement != "none") {
      let yDelta = 6;
      if (movement == "up") {
        yDelta *= -1;
      }

      const state: GameStateChange = getCurrentState();
      state.state.player1Pos.position.y = playerPosition.y + yDelta;
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
    // if (Date.now() - totalStart >= 1000) {
    //   console.log("fps:", frames);
    //   frames = 0;
    //   totalStart = Date.now();
    // }
    requestAnimationFrame(async () => await handleFrame(onStateChange));
  }

  function getCurrentState(): GameStateChange {
    return {
      id: matchId,
      state: {
        ballPos: {
          radius: ballRadius,
          position: { x: ballPosition.x, y: ballPosition.y },
        },
        countdown: countdown,
        player1Pos: {
          id: playerId,
          dimensions: playerDimensions,
          position: playerPosition,
        },
        player2Pos: {
          id: "",
          dimensions: playerDimensions,
          position: oponentPosition,
        },
      },
    };
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
  {playerPosition}
  {oponentPosition}
  {ballPosition}
  {ballRadius}
  {countdown}
  onPlayerMovementChange={(pmovement) => {
    if (movement != pmovement) {
      console.log(pmovement);
      movement = pmovement;
    }
  }}
  {canvasDimennsion}
  {playerDimensions}
/>
