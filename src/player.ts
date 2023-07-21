export class Song {
    file: string;

    constructor(file: string) {
        this.file = file;
    }
}

export class Player {
    static _instance: Player = null;

    currentSong: Song;
    isPaused: boolean;
    queue: Song[];
    #audio: HTMLAudioElement;

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
        this.#audio = null;
    }

    addSong(song: Song, index: number = -1) {
        if (index < 0) index = this.queue.length;
        this.queue.splice(index, 0, song);
    }

    removeSong(index: number) {
        this.queue.splice(index, 1);
    }

    createAudioElement() {
        if (this.queue.length == 0) {
            this.#audio = null;
            return;
        }
        this.currentSong = this.queue.shift();
        this.#audio = new Audio(this.currentSong.file);
        this.#audio.onended = () => this.playNextSong().then();
    }

    async play() {
        if (this.currentSong == null)
            this.createAudioElement();

        await this.#audio.play();
        this.isPaused = false;
    }

    pause() {
        if (this.currentSong == null)
            return;

        this.#audio.pause();
        this.isPaused = true;
    }

    position() {
        if (this.currentSong == null)
            return 0;

        return this.#audio.currentTime;
    }

    length() {
        if (this.currentSong == null)
            return 0;
        return this.#audio.duration;
    }

    seek(position: number) {
        if (this.currentSong == null)
            return;

        this.#audio.currentTime = position;
    }

    private async playNextSong() {
        this.createAudioElement();
        if (this.#audio != null)
            await this.#audio.play();
    }
}
