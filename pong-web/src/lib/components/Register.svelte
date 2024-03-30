<script lang="ts">
  import { SendUserMessage, WaitForMatchUpdate } from "$lib/api";
  export let ws: WebSocket;
  export let onMatchEnter: (matchId: string, playerId: string) => void;

  let nickname: string;
  let id: string | undefined = undefined;
  let waiting = false;

  const sendQueueUp = async () => {
    let response = await SendUserMessage(ws, {
      queueUpRequest: { nickname },
    });
    id = response.queueUpResponse!.Ok.id;
    console.log(id);
    waiting = true;
    console.log("waiting for message");
    const match = await WaitForMatchUpdate(ws);
    waiting = false;
    onMatchEnter(match.matchId, id);
  };
</script>

{#if !waiting}
  <div>
    <h1 class="text-3xl m-10">Welcome to Pong!</h1>
  </div>

  <div class="grid grid-cols-8">
    <input
      type="text"
      class="col-span-6 rounded-md px-3 py-2 text-gray-700 bg-white border border-gray-300 focus:outline-none focus:ring-indigo-500 focus:ring-1"
      bind:value={nickname}
      placeholder="enter your name"
    />
    <button
      type="button"
      class="col-span-2 px-3 py-2 rounded-md text-white bg-blue-500 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
      on:click={sendQueueUp}
    >
      Click Me
    </button>
  </div>
{:else}
  <div>
    <h1 class="text-3xl m-10">
      {nickname}, please wait for a match
    </h1>
  </div>
  <div class="flex justify-center items-center h-10 w-10 animate-spin">
    <svg
      fill="#000000"
      width="800px"
      height="800px"
      viewBox="0 0 50 50"
      xmlns="http://www.w3.org/2000/svg"
    >
      <path
        d="M41.9 23.9c-.3-6.1-4-11.8-9.5-14.4-6-2.7-13.3-1.6-18.3 2.6-4.8 4-7 10.5-5.6 16.6 1.3 6 6 10.9 11.9 12.5 7.1 2 13.6-1.4 17.6-7.2-3.6 4.8-9.1 8-15.2 6.9-6.1-1.1-11.1-5.7-12.5-11.7-1.5-6.4 1.5-13.1 7.2-16.4 5.9-3.4 14.2-2.1 18.1 3.7 1 1.4 1.7 3.1 2 4.8.3 1.4.2 2.9.4 4.3.2 1.3 1.3 3 2.8 2.1 1.3-.8 1.2-2.5 1.1-3.8 0-.4.1.7 0 0z"
      />
    </svg>
  </div>
{/if}
