<script lang="ts">
  import { RecvResponse, SendNoUpdates, SendUserMessage } from "$lib/api";

  export let ws: WebSocket;
  export let matchId: string;
  export let playerId: string;
  export let onMatchStart: (playerIds: string[]) => void;

  async function ready() {
    const response = await SendUserMessage(ws, {
      joinLobbyRequest: { matchId },
    });
    if (response.serverPushUpdate?.matchStatusChange) {
      console.log(response);
      onMatchStart(response.serverPushUpdate?.matchStatusChange?.start);
      return;
    }

    let updates = await RecvResponse(ws);
    if (updates.serverPushUpdate?.matchStatusChange) {
      console.log(updates);
      onMatchStart(updates.serverPushUpdate?.matchStatusChange?.start);
      return;
    }
  }
</script>

<p>match id: {matchId}. playerId: {playerId}</p>
<button
  class="col-span-2 px-3 py-2 rounded-md text-white bg-blue-500 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
  on:click={ready}
>
  I'm Ready!
</button>
