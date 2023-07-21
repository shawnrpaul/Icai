<script lang="ts">
  import { Player, Song } from "../player";

  let name = "";
  let greetMsg = "";

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    let player = Player.getPlayer();
    player.addSong(new Song(name));
    if (player.currentSong != null) {
      return;
    }
    await player.play();
  }
</script>

<div>
  <form class="row" on:submit|preventDefault={greet}>
    <input id="greet-input" placeholder="Enter a name..." bind:value={name} />
    <button type="submit">Greet</button>
  </form>
  <p>{greetMsg}</p>
</div>
