<script context="module">
</script>

<script lang="ts">
  import type { Dimensions, Movement, Position } from "$lib/messages";
  import { Canvas, Layer, type Render } from "svelte-canvas";

  export let canvasDimennsion: Dimensions;
  export let playerDimensions: Dimensions;
  export let playerPosition: Position;
  export let oponentPosition: Position;
  export let ballPosition: Position;
  export let ballRadius: number;
  export let countdown: number;
  export let score:
    | undefined
    | {
        player: number;
        oponent: number;
        playerIsRight: boolean;
      };
  export let onPlayerMovementChange: (movement: Movement) => void;
  export let winner: "player" | "opponent" | undefined;

  let movement: Movement = "none";

  let canvas: Canvas;
  $: rect = canvas?.getCanvas()?.getBoundingClientRect();

  let count: Render;
  $: count = ({ context, width, height }) => {
    if (countdown <= 0) return;
    context.font = `${width / 20}px sans-serif`;
    context.textAlign = "center";
    context.textBaseline = "middle";
    context.fillStyle = "tomato";
    context.fillText(`${countdown}`, width / 2, height / 10);
  };

  let player: Render;
  $: player = ({ context, width, height }) => {
    context.fillStyle = "green";
    context.fillRect(
      playerPosition.x,
      playerPosition.y,
      playerDimensions[0],
      playerDimensions[1],
    );
  };

  let oponent: Render;
  $: oponent = ({ context, width, height }) => {
    context.fillStyle = "black";
    context.fillRect(
      oponentPosition.x,
      oponentPosition.y,
      playerDimensions[0],
      playerDimensions[1],
    );
  };

  let ball: Render;
  $: ball = ({ context, width, height }) => {
    context.fillStyle = "goldenrod";
    context.beginPath();
    context.arc(ballPosition.x, ballPosition.y, ballRadius, 0, 2 * Math.PI);
    context.fill();
  };

  let scoreView: Render;
  $: scoreView = ({ context, width, height }) => {
    if (!score || countdown > 0) return;

    context.font = `${width / 15}px sans-serif`;
    context.textAlign = "center";
    context.textBaseline = "middle";
    if (score.playerIsRight) {
      context.fillStyle = "black";
      context.fillText(`${score.oponent} :`, width / 2, height / 2);
      context.fillStyle = "green";
      context.fillText(`${score.player}`, width / 2 + 50, height / 2);
    } else {
      context.fillStyle = "green";
      context.fillText(`${score.player} :`, width / 2, height / 2);
      context.fillStyle = "black";
      context.fillText(`${score.oponent}`, width / 2 + 50, height / 2);
    }
  };

  let winnerView: Render;
  $: winnerView = ({ context, width, height }) => {
    if (!winner) return;
    context.font = `${width / 15}px sans-serif`;
    context.textAlign = "center";
    context.textBaseline = "middle";
    context.fillStyle = "tomato";
    const text = winner == "player" ? "YOU WIN!" : "YOU LOSE!";
    context.fillText(text, width / 1.85, height - height / 6);
  };

  function getTouchRelativePosition(event: TouchEvent): Position {
    return {
      x: event.targetTouches[0].clientX - rect.left,
      y: event.targetTouches[0].clientY - rect.top,
    };
  }

  function onTouch(event: TouchEvent) {
    event.preventDefault();
    const position = getTouchRelativePosition(event);
    let newMovement: Movement;
    if (position.y >= canvasDimennsion[1] / 2) {
      newMovement = "down";
    } else {
      newMovement = "up";
    }

    // if (newMovement != movement) {
    //   movement = newMovement;
    // }
    onPlayerMovementChange(newMovement);
  }

  function onTouchEnd(event: TouchEvent) {
    event.preventDefault();
    onPlayerMovementChange("none");
  }
</script>

<Canvas
  class="border border-cyan-950 m-2"
  width={canvasDimennsion[0]}
  height={canvasDimennsion[1]}
  autoclear
  bind:this={canvas}
  on:touchstart={onTouch}
  on:touchend={onTouchEnd}
  on:touchmove={onTouch}
>
  <Layer render={count} />
  <Layer render={scoreView} />
  <Layer render={player} />
  <Layer render={oponent} />
  <Layer render={ball} />
  <Layer render={winnerView} />
</Canvas>
