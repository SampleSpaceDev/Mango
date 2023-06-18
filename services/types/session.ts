export interface Session {
    name?: string;
    id: string;
    started: number;
    ownerId: string;

    start: {
        bedwars: {
            wins: number;
            losses: number;
            kills: number;
            deaths: number;
            finalKills: number;
            finalDeaths: number;
            bedsBroken: number;
            bedsLost: number;
            gamesPlayed: number;
            level: number;
            coins: number;
            experience: number;
        }
    }
}

export interface LinkedPlayer {
    id: string;
    uuid: string;
}