import { readBinaryFile } from "@tauri-apps/api/fs"
import { createEventDispatcher } from "svelte";

export class Song {
    file: string;

    constructor(file: string) {
        this.file = file;
    }
}

class QueueSong {
    song: Song;
    private player: Player;
    audio: AudioBufferSourceNode;
    audioContext: AudioContext;

    constructor(song: Song, player: Player) {
        this.song = song;
        this.player = player;
        this.audioContext = new AudioContext();
        this.audioContext.suspend().then();
        this.createAudioElement().then();
    }

    private async getFileBuffer(filePath) {
        const arrayBuffer = (await readBinaryFile(filePath)).buffer;
        return await this.audioContext.decodeAudioData(arrayBuffer);
    }

    private async createAudioElement() {
        this.audio = this.audioContext.createBufferSource();
        this.audio.buffer = await this.getFileBuffer(this.song.file);
        this.audio.connect(this.audioContext.destination);
        this.audio.onended = () => this.player.playNextSong().then();
        this.audio.start(0);
    }

    length() {
        return this.audio.buffer.duration;
    }

    async start() {
        await this.audioContext.resume();
    }

    async stop() {
        await this.audioContext.suspend();
    }
}

export class Player {
    static _instance: Player = null;
    isPaused: boolean;
    private queue: QueueSong[];
    private currentSong: QueueSong;

    static getPlayer() {
        if (this._instance == null) {
            this._instance = new Player();
        }
        return this._instance;
    }

    constructor() {
        this.currentSong = null;
        this.queue = [];
        this.isPaused = true;
    }

    async addSong(song: Song, index: number = -1) {
        if (this.currentSong == null) {
            this.currentSong = new QueueSong(song, this);
            this.isPaused = false;
            return await this.currentSong.start();
        }
        if (index < 0) index = this.queue.length;
        this.queue.splice(index, 0, new QueueSong(song, this));
        this.queue.forEach(song => {
            console.log(song.song.file);
        });
    }

    removeSong(index: number) {
        this.queue.splice(index, 1);
    }

    nowPlaying() {
        return this.currentSong != null ? this.currentSong.song : null;
    }

    async play() {
        if (!this.isPaused || this.currentSong == null) return;
        await this.currentSong.start();
        this.isPaused = false;
    }

    async pause() {
        if (!this.isPaused || this.currentSong == null) return;

        await this.currentSong.stop();
        this.isPaused = true;
    }

    position() {
        if (this.currentSong === null) {
            return null;
        }

        return this.currentSong.audioContext.currentTime;
    }

    length() {
        if (this.currentSong == null)
            return 0;
        return this.currentSong.length();
    }

    async playNextSong() {
        this.currentSong = this.queue.shift();
        if (this.currentSong == null) {
            this.isPaused = true;
            console.log("Playing: none");
            return;
        }
        console.log(`Playing: ${this.currentSong.song.file}`);
        await this.currentSong.start();
    }
}
