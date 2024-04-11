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

  export let matchId: string;
  export let playerId: string;
  export let ws: WebSocket;

  let playerPosition: Position = { x: -100, y: -100 };
  let oponentPosition: Position = { x: -100, y: -100 };
  let ballPosition: Position = { x: -100, y: -100 };
  let ballMovement: MovementVector = {
    horizontalVector: 0,
    verticalVector: 0,
  };
  let ballRadius = 0;
  let countdown = -1;
  let playerDimensions: Dimensions = { "0": 1, "1": 1 };
  let movement: Movement = "none";
  let score: Score | undefined = undefined;

  // $: scoreView = score
  //   ? { player: score.leftPlayer.score, oponent: score.rightPlayer.score }
  //   : undefined;
  $: scoreView = () => {
    if (!score) return undefined;
    if (score.leftPlayer.player.id == playerId) {
      return {
        player: score.leftPlayer.score,
        oponent: score.rightPlayer.score,
        playerIsRight: false,
      };
    }

    return {
      player: score.rightPlayer.score,
      oponent: score.leftPlayer.score,
      playerIsRight: true,
    };
  };

  let frames = 0;
  let time = Date.now();

  onMount(async () => {
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
      ballMovement = state.state.ballPos.movement;
      score = state.state.score;
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

    ballPosition.x += ballMovement.horizontalVector;
    ballPosition.y += ballMovement.verticalVector;

    if (movement != "none") {
      let yDelta = 10;
      if (movement == "up") {
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
  score={scoreView()}
  onPlayerMovementChange={(pmovement) => {
    if (movement != pmovement) {
      console.log(pmovement);
      movement = pmovement;
    }
  }}
  canvasDimennsion={{ "0": 667, "1": 300 }}
  {playerDimensions}
/>
