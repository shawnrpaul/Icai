import { Song } from "./songs";

export class Playlist {
    name: string;
    songs: Song[]

    constructor(name: string, songs: Song[] = null) {
        this.name = name;
        this.songs = songs != null ? songs : [];
    }
}


