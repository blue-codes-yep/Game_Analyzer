<script lang="ts">
  import { dialog, invoke } from "@tauri-apps/api";

  async function selectDirectory() {
    const path = await dialog.open({ directory: true });
    console.log(path);

    // Call the create_config command with the selected path
    if (typeof path === 'string') {
      invoke('create_config', { args: [path] });
    }
  }
</script>

<div>
  <button on:click={selectDirectory}>Select CS:GO Directory</button>
</div>
