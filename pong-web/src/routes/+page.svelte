<script lang="ts">
  import Game from "$lib/components/Game.svelte";
  import GameCanvas from "$lib/components/GameCanvas.svelte";
  import Lobby from "$lib/components/Lobby.svelte";
  import OfflineGame from "$lib/components/OfflineGame.svelte";
  import Register from "$lib/components/Register.svelte";
  import type { Position } from "$lib/messages";
  import "../app.css";

  export let data;
  let state: "register" | "in lobby" | "in match" = "register";
  let matchId: string;
  let playerId: string;

  let playerPosition: Position = { x: 25, y: 50 };
  let oponentPosition: Position = { x: 667 - 25, y: 50 };
  let ballPosition: Position = { x: 150, y: 150 };
  let ballRadius = 12;
  let countdown = -1;

  function onMatchEnter(match: string, player: string): void {
    matchId = match;
    playerId = player;
    state = "in lobby";
  }

  function onMatchStart(playerIds: string[]): void {
    console.log(playerIds);
    state = "in match";
  }
</script>

<div class="parent flex flex-col h-full justify-center items-center">
  <!-- <OfflineGame ws={data.ws} {matchId} {playerId} /> -->
  {#if state == "register"}
    <Register ws={data.ws} {onMatchEnter} />
  {/if}
  {#if state == "in lobby"}
    <Lobby ws={data.ws} {matchId} {playerId} {onMatchStart} />
  {/if}
  {#if state == "in match"}
    <Game {matchId} {playerId} ws={data.ws} />
  {/if}
  <!-- <button on:click={() => (countdown += 1)}>count</button> -->
  <!-- <GameCanvas
    {playerPosition}
    {oponentPosition}
    {ballPosition}
    {ballRadius}
    {countdown}
  /> -->
</div>
