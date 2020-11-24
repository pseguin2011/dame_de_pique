import network_config from '../config/Config';
export default class GameClient {
    game_id: string;
    player_id: number;
    host = network_config.host;
    port = network_config.port;

    constructor(game_id: string, player_id: number) {
        this.game_id = game_id;
        this.player_id = player_id;
    }
}