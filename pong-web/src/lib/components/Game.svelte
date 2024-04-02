<script lang="ts">
  import {
    SendNoUpdates,
    SendUserMessage,
    SubscribeToServerMessages,
  } from "$lib/api";
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

  let playerPosition: Position = { x: -100, y: -100 };
  let oponentPosition: Position = { x: -100, y: -100 };
  let ballPosition: Position = { x: -100, y: -100 };
  let ballRadius = 0;
  let countdown = -1;
  let playerDimensions: Dimensions = { "0": 1, "1": 1 };
  let movement: Movement = "none";

  onMount(async () => {
    // callbackGameLoop((state) => {
    //   const positions = getPositions(state.state);
    //   playerPosition = positions.player;
    //   oponentPosition = positions.oponent;
    //   ballPosition = state.state.ballPos.position;
    //   ballRadius = state.state.ballPos.radius;
    //   countdown = state.state.countdown;
    //   playerDimensions = state.state.player1Pos.dimensions;
    // });
    SubscribeToServerMessages(ws, (message) => {
      const state = message.serverPushUpdate?.gameStateChange;
      if (!state) return;
      const positions = getPositions(state.state);
      playerPosition = positions.player;
      oponentPosition = positions.oponent;
      ballPosition = state.state.ballPos.position;
      ballRadius = state.state.ballPos.radius;
      countdown = state.state.countdown;
      playerDimensions = state.state.player1Pos.dimensions;
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
    const message = await SendNoUpdates(ws);
    const state = message.serverPushUpdate?.gameStateChange;
    if (state) {
      onStateChange(state);
    }
    if (movement != "none") {
      let yDelta = 10;
      if (movement == "up") {
        yDelta *= -1;
      }
      await SendUserMessage(ws, {
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
    // if (Date.now() - totalStart >= 1000) {
    //   console.log("fps:", frames);
    //   frames = 0;
    //   totalStart = Date.now();
    // }
    requestAnimationFrame(async () => await handleFrame(onStateChange));
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
  canvasDimennsion={{ "0": 667, "1": 300 }}
  {playerDimensions}
/>
