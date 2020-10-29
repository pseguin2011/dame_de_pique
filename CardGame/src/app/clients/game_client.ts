export default class GameClient {
    game_id: string;
    player_id: number;
    host = "10.0.0.153";
    port = 8000;

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
    }).catch((e) => {alert("Could not discard "); throw e;} );
  }

  async openAction(cards: [number]) {
    // if (selected_indices.length != 1) {
    //   alert("You must select ONE card to discard");
    //   return;
    // }
    // await fetch('http://' + this.host + ':' + this.port + '/discard-card', {
    //   method: "POST",
    //   headers: {
    //     Accept: 'application/json',
    //     'Content-Type': 'application/json',
    //   },
    //   body: JSON.stringify(
    //     {'game_id': this.game_id, 'card_index': selected_indices[0]}
    //   )
    // }).catch((e) => {alert("Could not discard "); throw e;} )
    // .then((_) => {

    // });
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