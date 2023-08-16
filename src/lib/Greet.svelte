<script lang="ts">
  import { Player } from "./player";
  import { Playlist } from "./playlist";
  import { invoke } from "@tauri-apps/api/tauri";
  import { open } from "@tauri-apps/api/dialog";
  import { audioDir } from "@tauri-apps/api/path";
  import { Song } from "./songs";

  let player = Player.getPlayer();
  let name = "";
  let time = 0;
  let songs: Map<number, Song> = new Map();

  invoke<Playlist[]>("get_all_playlists").then((playlists) => {
    let selectionPlaylist = document.getElementById(
      "selectionPlaylist"
    ) as HTMLSelectElement;
    playlists.forEach((playlist) => {
      selectionPlaylist.add(new Option(playlist.name));
    });
    let selectionSong = document.getElementById(
      "selectionSong"
    ) as HTMLSelectElement;
    let songAdder = document.getElementById("songAdder") as HTMLSelectElement;
    playlists[0].songs.forEach((song) => {
      let id = song.id;
      songs.set(id, song);
      selectionSong.add(new Option(song.title, id.toString()));
      songAdder.add(new Option(song.title, id.toString()));
    });
  });

  async function setPlaylist() {
    let selectionPlaylist = document.getElementById(
      "selectionPlaylist"
    ) as HTMLSelectElement;
    let name = selectionPlaylist.options[selectionPlaylist.selectedIndex].text;
    let songsArray = null;
    if (name === "All Songs") {
      songsArray = Array.from(songs.values());
    } else {
      let playlist = await invoke<Playlist>("get_playlist", {
        name: name,
      });
      songsArray = playlist.songs;
    }
    let selectionSong = document.getElementById(
      "selectionSong"
    ) as HTMLSelectElement;
    while (selectionSong.options.length > 0) {
      selectionSong.remove(0);
    }
    songsArray.forEach((song) => {
      selectionSong.add(new Option(song.title, song.id.toString()));
    });
  }

  async function createPlaylist() {
    await invoke<void>("create_playlist", { name: name });
    let selectionPlaylist = document.getElementById(
      "selectionPlaylist"
    ) as HTMLSelectElement;
    selectionPlaylist.add(new Option(name));
  }

  async function deletePlaylist() {
    let selectionPlaylist = document.getElementById(
      "selectionPlaylist"
    ) as HTMLSelectElement;
    let name = selectionPlaylist.options[selectionPlaylist.selectedIndex].text;
    if (name === "All Songs") return;
    await invoke<void>("delete_playlist", { name: name });
    selectionPlaylist.remove(selectionPlaylist.selectedIndex);
    let selectionSong = document.getElementById(
      "selectionSong"
    ) as HTMLSelectElement;
    while (selectionSong.options.length > 0) {
      selectionSong.remove(0);
    }
    Array.from(songs.values()).forEach((song) => {
      selectionSong.add(new Option(song.title, song.id.toString()));
    });
  }

  async function addSongToPlaylist() {
    let selectionPlaylist = document.getElementById(
      "selectionPlaylist"
    ) as HTMLSelectElement;
    let name = selectionPlaylist.options[selectionPlaylist.selectedIndex].text;
    if (name === "All Songs") return;
    let songAdder = document.getElementById("songAdder") as HTMLSelectElement;
    let song = songs.get(
      parseInt(songAdder.options[songAdder.selectedIndex].value)
    );
    await invoke<void>("add_playlist_song", { playlist: name, id: song.id });
    let selectionSong = document.getElementById(
      "selectionSong"
    ) as HTMLSelectElement;
    selectionSong.add(new Option(song.title, song.id.toString()));
  }

  async function removeSongFromPlaylist() {
    let selectionPlaylist = document.getElementById(
      "selectionPlaylist"
    ) as HTMLSelectElement;
    let name = selectionPlaylist.options[selectionPlaylist.selectedIndex].text;
    if (name === "All Songs") return;
    let selectionSong = document.getElementById(
      "selectionSong"
    ) as HTMLSelectElement;
    let song = songs.get(
      parseInt(selectionSong.options[selectionSong.selectedIndex].value)
    );
    await invoke<void>("remove_playlist_song", { playlist: name, id: song.id });
    selectionSong.remove(selectionSong.selectedIndex);
  }

  async function playSong() {
    let selection = document.getElementById(
      "selectionSong"
    ) as HTMLSelectElement;
    let song = songs.get(
      parseInt(selection.options[selection.selectedIndex].value)
    );
    await player.addSong(song);
  }

  async function pausePlay() {
    player.isPaused ? await player.play() : await player.pause();
  }

  async function currentTime() {
    let time = player.position();
    time == null
      ? console.log("Not Playing")
      : console.log(`Current time: ${time}/${player.length()}`);
  }

  async function seek() {
    player.seek(Number(time));
  }

  async function changeVolume() {
    let volume = document.getElementById("volume") as HTMLInputElement;
    player.setVolume(parseInt(volume.value));
  }

  async function addFolder() {
    const selected = await open({
      multiple: false,
      directory: true,
      defaultPath: await audioDir(),
    });
    if (selected === null) return;
    await invoke<void>("add_dir", {
      path: selected,
    });
    let playlist = await invoke<Playlist>("get_all_songs");
    let selectionPlaylist = document.getElementById(
      "selectionPlaylist"
    ) as HTMLSelectElement;
    if (
      selectionPlaylist.options[selectionPlaylist.selectedIndex].text ==
      "All Songs"
    ) {
      let selectionSong = document.getElementById(
        "selectionSong"
      ) as HTMLSelectElement;
      while (selectionSong.length > 0) {
        selectionSong.remove(0);
      }
      playlist.songs.forEach((song) => {
        selectionSong.add(new Option(song.title, song.toString()));
      });
    }
    let songAdder = document.getElementById("songAdder") as HTMLSelectElement;
    while (songAdder.length > 0) {
      songAdder.remove(0);
    }
    songs.clear();
    playlist.songs.forEach((song) => {
      let id = song.id;
      songs.set(id, song);
      songAdder.add(new Option(song.title, id.toString()));
    });
  }

  async function removeFolder() {
    const selected = await open({
      multiple: false,
      directory: true,
      defaultPath: await audioDir(),
    });
    if (selected === null) return;
    await invoke<void>("remove_dir", {
      path: selected,
    });
    let playlists = await invoke<Playlist[]>("get_all_playlists");
    let selectionPlaylist = document.getElementById(
      "selectionPlaylist"
    ) as HTMLSelectElement;
    while (selectionPlaylist.length > 0) {
      selectionPlaylist.remove(0);
    }
    playlists.forEach((playlist) => {
      selectionPlaylist.add(new Option(playlist.name));
    });
    let selectionSong = document.getElementById(
      "selectionSong"
    ) as HTMLSelectElement;
    let songAdder = document.getElementById("songAdder") as HTMLSelectElement;
    while (selectionSong.length > 0) {
      selectionSong.remove(0);
      songAdder.remove(0);
    }
    songs.clear();
    playlists[0].songs.forEach((song) => {
      let id = song.id;
      songs.set(id, song);
      selectionSong.add(new Option(song.title, id.toString()));
      songAdder.add(new Option(song.title, id.toString()));
    });
  }
</script>

<div>
  <form class="row">
    <select id="selectionPlaylist" class="row" />
    <button type="button" on:click={setPlaylist}>Set Playlist</button>
    <button type="button" on:click={deletePlaylist}>Delete Playlist</button>
  </form>
  <form class="row">
    <select id="selectionSong" class="row" />
    <button type="button" on:click={playSong}>Play Song</button>
    <button type="button" on:click={removeSongFromPlaylist}>Remove Song</button>
  </form>
  <form class="row">
    <input placeholder="Enter a name..." bind:value={name} />
    <button type="button" on:click={createPlaylist}>Create Playlist</button>
  </form>
  <form class="row">
    <select id="songAdder" class="row" />
    <button type="button" on:click={addSongToPlaylist}>Add Song</button>
  </form>
  <form class="row">
    <button type="button" on:click={addFolder}>Add Folder</button>
    <button type="button" on:click={removeFolder}>Remove Folder</button>
  </form>
  <form class="row">
    <input id="greet-input" placeholder="Enter a time..." bind:value={time} />
    <button type="button" on:click={seek}>Seek</button>
  </form>
  <form class="row">
    <button type="button" on:click={pausePlay}>Play\Pause</button>
  </form>
  <form class="row">
    <button type="button" on:click={currentTime}>Time</button>
  </form>
  <div>
    <input
      id="volume"
      type="range"
      min="1"
      max="100"
      value="100"
      on:change={changeVolume}
      style="padding:0;"
    />
  </div>
</div>
