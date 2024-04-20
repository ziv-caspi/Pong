<script lang="ts">
  import BouncingBall from "$lib/components/BouncingBall.svelte";
  import Game from "$lib/components/Game.svelte";
  import GameCanvas from "$lib/components/GameCanvas.svelte";
  import Lobby from "$lib/components/Lobby.svelte";
  import OfflineGame from "$lib/components/OfflineGame.svelte";
  import Register from "$lib/components/Register.svelte";
  import type { Position, PotentialPlayer } from "$lib/messages";
  import "../app.css";

  export let data;
  let state: "register" | "in lobby" | "in match" = "register";
  let matchId: string;
  let player: PotentialPlayer;
  let oponent: PotentialPlayer;

  function onMatchEnter(
    match: string,
    playerP: PotentialPlayer,
    oponentP: PotentialPlayer,
  ): void {
    matchId = match;
    player = playerP;
    oponent = oponentP;
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
    <Lobby
      ws={data.ws}
      {matchId}
      {onMatchStart}
      onMatchDeath={() => {
        location.reload();
      }}
      players={{
        player: { id: player.id, nickname: player.nickname, ready: false },
        oponent: { id: oponent.id, nickname: oponent.nickname, ready: false },
      }}
    />
  {/if}
  {#if state == "in match"}
    <Game
      {matchId}
      playerId={player.id}
      ws={data.ws}
      playerNickname={player.nickname}
      oponentNicknae={oponent.nickname}
      onGameFinished={() => {
        location.reload();
      }}
    />
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
