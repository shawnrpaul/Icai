<script lang="ts">
  import { Player, Song } from "../player";

  let name = "";
  let player = Player.getPlayer();

  async function addSong() {
    await player.addSong(new Song(name));
  }

  async function pausePlay() {
    player.isPaused ? await player.play() : await player.pause();
  }

  async function currentTime() {
    let time = player.position();
    if (time == null) {
      console.log("Not Playing");
    } else {
      console.log(`Current time: ${time}/${player.length()}`);
    }
  }
</script>

<div>
  <form class="row" on:submit|preventDefault={addSong}>
    <input id="greet-input" placeholder="Enter a name..." bind:value={name} />
    <button type="submit">Add Song</button>
  </form>
  <form class="row" on:submit|preventDefault={pausePlay}>
    <button type="submit">Play\Pause</button>
  </form>
  <form class="row" on:submit|preventDefault={currentTime}>
    <button type="submit">Time</button>
  </form>
</div>
