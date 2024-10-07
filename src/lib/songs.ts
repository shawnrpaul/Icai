export class Song {
    id: number;
    title: string;
    path: string;
    count: number

    constructor(id: number, title: string, path: string, count: number) {
        this.id = id;
        this.title = title;
        this.path = path;
        this.count = count;
    }
}
