<script lang="ts">
  import {
    RecvResponse,
    SendNoUpdates,
    SendUserMessage,
    SendUserMessageWithoutResponses,
    SubscribeToServerMessages,
  } from "$lib/api";
  import type { MatchStatusChange } from "$lib/messages";
  import { onMount } from "svelte";
  import BouncingBall from "./BouncingBall.svelte";

  export let ws: WebSocket;
  export let matchId: string;
  export let onMatchStart: (playerIds: string[]) => void;
  export let players: {
    player: { id: string; nickname: string; ready: boolean };
    oponent: { id: string; nickname: string; ready: boolean };
  };

  onMount(() => {
    SubscribeToServerMessages(ws, (message) => {
      if (message.serverPushUpdate?.matchStatusChange) {
        onUpdate(message.serverPushUpdate.matchStatusChange);
      }
    });
  });

  function ready() {
    SendUserMessageWithoutResponses(ws, {
      joinLobbyRequest: { matchId },
    });
    players.player.ready = true;
    console.log("player ready");
  }

  function onUpdate(change: MatchStatusChange) {
    console.log(change);
    if (change.start) {
      onMatchStart(change.start);
      return;
    }

    if (change.stop) {
      console.error("game died!!", change.stop);
      return;
    }

    if (change.playerReady && change.playerReady !== players.player.id) {
      players.oponent.ready = true;
    }
  }
</script>

<div class="m-10"></div>
<!-- <p>match id: {matchId}. playerId: {playerId}</p> -->

<div class="grid grid-cols-3 m-10 text-center" style="max-height: 500px;">
  <div class="m-2">
    <p class="text-black">{players.oponent.nickname}</p>
    <BouncingBall height={200} active={!players.oponent.ready} />
    {#if players.oponent.ready}
      <p class="text-green-500 font-bold">READY</p>
    {:else}
      <p class="text-red-500 font-bold">NOT READY</p>
    {/if}
  </div>
  <div class="m-2">
    {#if !players.player.ready}
      <button
        class="col-span-2 px-3 py-2 rounded-md text-white bg-blue-500 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
        on:click={ready}
      >
        I'm Ready!
      </button>
    {/if}
  </div>
  <div class="m-2">
    <p class="text-green-400">{players.player.nickname}</p>
    <BouncingBall height={200} active={!players.player.ready} />
    {#if players.player.ready}
      <p class="text-green-500 font-bold">READY</p>
    {:else}
      <p class="text-red-500 font-bold">NOT READY</p>
    {/if}
  </div>
</div>
