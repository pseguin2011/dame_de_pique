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

  async drawCardAction(post_event: () => {}) {
    this.request('draw-card/?game-id=' + this.game_id + '&player=' + this.player_id, "GET")
      .catch((e) => { alert("Could not draw a card, an error occured."); throw e;} )
      .then((_) => {
          post_event();
    });
  }

  async discardAction(cardIndex: number) {

    await fetch('http://' + this.host + ':' + this.port + '/discard-card', {
      method: "POST",
      headers: {
        Accept: 'application/json',
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(
        {'game_id': this.game_id, 'card_index': cardIndex}
      )
    }).catch((e) => {alert("Could not discard."); throw e;} );
  }

  async openAction(cards: number[]) {
    await fetch('http://' + this.host + ':' + this.port + '/player-open', {
      method: "POST",
      headers: {
        Accept: 'application/json',
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(
        {'game_id': this.game_id, 'card_indices': cards}
      )
    }).catch((e) => {alert("Could not Open. 3 sets of 3 are required if your opponent has not opened. 1 set of 3 is required if they have opened."); throw e;} )
    .then((_) => {

    });
  }

  async pickupDiscardAction(cards: number[]) {
    await fetch('http://' + this.host + ':' + this.port + '/player-pickup-discard', {
      method: "POST",
      headers: {
        Accept: 'application/json',
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(
        {'game_id': this.game_id, 'card_indices': cards}
      )
    }).catch((e) => {alert("Could not Open. 3 sets of 3 are required if your opponent has not opened. 1 set of 3 is required if they have opened."); throw e;} )
    .then((_) => {

    });
  }

  async updateGameState(response_handler: (json: any) => {}) {
    this.request('game-state/?game-id=' + this.game_id + '&player=' + this.player_id, "GET")
      .catch((e) => {alert("Could not update the game state, refresh your browser."); throw e;} )
      .then((response) => response.json())
      .then((json: any ) => {
        response_handler(json);
    });
  }


  async request(path: string, method: string): Promise<Response> {
    return fetch('http://' + this.host + ':' + this.port + "/" + path, {
      method: method,
      headers: {
        Accept: 'application/json',
        'Content-Type': 'application/json',
      },
    });
  }
}