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

  async discardAction(cardIndex: number, post_event: () => {}) {

    await fetch('http://' + this.host + ':' + this.port + '/discard-card', {
      method: "POST",
      headers: {
        Accept: 'application/json',
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(
        {'game_id': this.game_id, 'card_index': cardIndex}
      )
    }).catch((e) => {alert("Could not discard."); throw e;} )
    .then((_)=>{
      post_event();
    });
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

  async addPointsAction(cards: number[]) {
    await fetch('http://' + this.host + ':' + this.port + '/player-add-points', {
      method: "POST",
      headers: {
        Accept: 'application/json',
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(
        {'game_id': this.game_id, 'card_indices': cards}
      )
    }).catch((e) => {alert("Could add points. Either a set of 3, cards already in the points deck, Joker, or 2 is required for this action."); throw e;} )
    .then((_) => {

    });
  }

  async pickupDiscardAction(cards: number[], post_event: () => {}) {
    await fetch('http://' + this.host + ':' + this.port + '/player-pickup-discard', {
      method: "POST",
      headers: {
        Accept: 'application/json',
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(
        {'game_id': this.game_id, 'card_indices': cards}
      )
    }).catch((e) => { alert("Could not pickup the discard pile, an error occured."); throw e;} )
    .then((e) => {
      if (e.status == 200) {
        post_event();
      } else {
        alert("You can't pickup the discard pile.")
      }
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