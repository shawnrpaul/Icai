import { readBinaryFile } from "@tauri-apps/api/fs"
import { Song } from "./songs";

class QueueSong {
    song: Song;
    private audio: HTMLAudioElement;

    private constructor(song: Song, audio: HTMLAudioElement) {
        this.song = song;
        this.audio = audio;
    }

    static async createSong(song: Song, player: Player) {
        let audio = await QueueSong.createAudioElement(song.path);
        audio.onended = () => player.playNextSong().then();
        return new QueueSong(song, audio);
    }

    private static async createAudioElement(filePath) {
        let blob = new Blob([(await readBinaryFile(filePath)).buffer]);
        const url = URL.createObjectURL(blob);
        return new Audio(url);
    }

    position() {
        return Math.round(this.audio.currentTime);
    }

    length() {
        return Math.round(this.audio.duration);
    }

    async play() {
        await this.audio.play();
    }

    async pause() {
        await this.audio.pause();
    }

    seek(time: number) {
        this.audio.currentTime = time;
    }

    setVolume(volume: number) {
        this.audio.volume = volume / 100.0;
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
        let queueSong = await QueueSong.createSong(song, this);
        if (this.currentSong == null) {
            this.currentSong = queueSong;
            this.isPaused = false;
            return await this.currentSong.play();
        }
        if (index < 0) index = this.queue.length;
        this.queue.splice(index, 0, queueSong);
    }

    removeSong(index: number) {
        this.queue.splice(index, 1);
    }

    nowPlaying() {
        return this.currentSong != null ? this.currentSong.song : null;
    }

    async play() {
        if (!this.isPaused || this.currentSong == null) return;
        await this.currentSong.play();
        this.isPaused = false;
    }

    async pause() {
        if (this.isPaused || this.currentSong == null) return;

        await this.currentSong.pause();
        this.isPaused = true;
    }

    seek(time: number) {
        this.currentSong.seek(time);
    }

    position() {
        return this.currentSong.position() != null ? this.currentSong.length() : 0;
    }

    length() {
        return this.currentSong != null ? this.currentSong.length() : 0;
    }

    setVolume(volume: number) {
        this.currentSong != null ? this.currentSong.setVolume(volume) : null;
    }

    async playNextSong() {
        this.currentSong = this.queue.shift();
        if (this.currentSong == null) {
            this.isPaused = true;
            return;
        }
        await this.currentSong.play();
    }
}
