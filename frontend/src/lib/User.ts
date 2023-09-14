
export class OsuUser {
    userId: number
    username: string
    country: string
    coverUrl: string

    constructor(userId: number, username: string, country: string, coverUrl: string) {
        this.userId = userId;
        this.username = username;
        this.country = country;
        this.coverUrl = coverUrl;
    }
}