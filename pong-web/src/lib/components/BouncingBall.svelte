<script lang="ts">
  import { Canvas, Layer, type Render } from "svelte-canvas";
  import { tweened } from "svelte/motion";
  import { quadIn as easingDown, quadOut as easingUp } from "svelte/easing";

  export let active = true;
  export let height: number | undefined = undefined;
  const ballPosition = tweened([0.5, 0.5], {
    duration: 400,
    easing: easingDown,
  });

  let lastIsDown = false;

  setInterval(() => {
    if (!active) {
      ballPosition.set([0.5, 0.9], { easing: easingDown });
      return;
    }
    if (lastIsDown) {
      ballPosition.set([0.5, 0.1], { easing: easingUp });
    } else {
      ballPosition.set([0.5, 0.9], { easing: easingDown });
    }
    lastIsDown = !lastIsDown;
  }, 400);

  let render: Render;
  $: render = ({ context, width, height }) => {
    const radius = 15;
    const [x, y] = $ballPosition;
    context.fillStyle = "goldenrod";
    context.beginPath();
    context.arc(x * width, y * height, radius, 0, 2 * Math.PI);
    context.fill();

    const batLength = 0.7 * width;
    context.fillStyle = "green";
    context.fillRect(
      (width - batLength) / 2,
      height * 0.9 + radius,
      batLength,
      5,
    );
  };
</script>

<Canvas class="m-2" {height}>
  <Layer {render} />
</Canvas>
