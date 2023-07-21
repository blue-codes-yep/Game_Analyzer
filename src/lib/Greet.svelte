<script lang="ts">
  import { dialog, invoke, event } from "@tauri-apps/api";

  let gameState = null;
  event.listen("game_state_update", (update) => {
    gameState = update;
  });

  async function selectDirectory() {
    const path = await dialog.open({ directory: true });
    console.log(path);

    // Call the create_config command with the selected path
    if (typeof path === 'string') {
      invoke('create_config', { args: [JSON.stringify({ path: path })] });
    }
  }
</script>
<div>
  {#if gameState}
    <pre>{JSON.stringify(gameState, null, 2)}</pre>
  {/if}
</div>
<div>
  <button on:click={selectDirectory}>Select CS:GO Directory</button>
</div>
