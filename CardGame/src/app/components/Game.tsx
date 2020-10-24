import React, { Component } from 'react';
import { View, Text, ViewStyle, Button, TextStyle} from 'react-native';
import { FlatList } from "react-native-gesture-handler";
import {Card} from '../models/Card';
import { useRoute } from '@react-navigation/native';

const GAME_ACTIONS_STYLE: ViewStyle = {
  height: 5, 
  alignContent: 'flex-start', 
  flexDirection:'row',
  flexWrap: 'wrap',
  flex: 1,
  width: '100%',
};

const GAME_ACTION_STYLE: ViewStyle = {
  margin: 20,
  width: 160,
};
const CARD_CONTAINER_VIEW_STYLE: ViewStyle = {
  margin: 7,
  padding: 10,
  backgroundColor: 'white',
  borderRadius: 5,
  left: '0%',
};

const TEAM_CONTAINER_VIEW_STYLE: ViewStyle = {
  flex:1,
  height: 180,
  ... CARD_CONTAINER_VIEW_STYLE
};

const PLAYER_CONTAINER_VIEW_STYLE: ViewStyle = {
  height: 180,
  ... CARD_CONTAINER_VIEW_STYLE
};

const DECK_CONTAINER_VIEW_STYLE: ViewStyle = {
  flex: 1,
  flexDirection: "row",
  flexWrap: 'wrap',
  height: 240,
  minWidth: 30,
  ... CARD_CONTAINER_VIEW_STYLE
};



type GameState = {
  selected: boolean[],
  did_draw: boolean,
  game_state: {
    player_hand: Card[],
    team1_points: Card[],
    team2_points: Card[],
    top_discard?: JSX.Element,
    turn: number,
  },
};

const TITLE_STYLES: TextStyle = {
  color: 'Black',
  fontWeight: 'bold',
  margin: 5,
  fontSize: 20
};

const card_style: ViewStyle = {
  borderStyle: 'solid',
  borderWidth: 1,
  borderRadius: 3,
  width: 80, 
  height: 100,
  margin: 5,
  backgroundColor: 'red',
};


type WebSocketResponse = {response_type: string, data: any};

export class Game extends Component {
  state: GameState;
  host = '10.0.0.153';
  port = 8000;
  game_id: String;
  socket: WebSocket;
  player_id: number;
  constructor(props: any) {
    super(props);
    this.updateSelections = this.updateSelections.bind(this);
    this.game_id = props.route.params.game_id;
    this.socket = props.route.params.websocket;
    this.player_id = props.route.params.player_id;
    this.state = { 
      selected: [],
      did_draw: false,
      game_state: {
        player_hand: [],
        team1_points: [],
        team2_points: [],
        top_discard: undefined,
        turn: 0,
      },
    };
    this.socket.onmessage = (e) => {
      let json: WebSocketResponse = JSON.parse(e.data);
      switch (json.response_type) {
        case "GameState":
          this.updateGameState();
          break;
      }
    };
    this.updateGameState();
  }
  setDefaultGameState() {
    this.state.game_state = {
      player_hand: [],
      team1_points: [],
      team2_points: [],
      top_discard: undefined,
      turn: 0,
    };
    this.forceUpdate();
  }

  async updateGameState() {
    fetch('http://' + this.host + ':' + this.port + '/game-state/?game-id=' + this.game_id + '&player=' + this.player_id, {
      method: "GET",
      headers: {
        Accept: 'application/json',
        'Content-Type': 'application/json',
      },
    }).catch((e) => {alert("Could not join the game, room must be full."); throw e;} )
    .then((response) => response.json())
    .then((json: any ) => {
      this.setDefaultGameState();
      let top_card = <Card {... this.props} value={json.top_discard.value} suit={json.top_discard.suit} id={-1}/>;
      this.state.game_state = {
        player_hand: json.player_hand.map((card: any, index: number) => <Card onChange={this.updateSelections} {... this.props} id={index} value={card.value} suit={card.suit}/>),
        team1_points: json.team1_points,
        team2_points: json.team2_points,
        turn: json.turn,
        top_discard: top_card,
      };
      this.state.selected = this.state.game_state.player_hand.map(()=> false);
      this.forceUpdate();
    });
  }

  render(): JSX.Element {
      return <View style={{backgroundColor: 'darkgreen', height: '100%', overflow: 'scroll'}}>
        <View style={{flexDirection: 'row'}}>
          <View style={{flexDirection: 'column', flex: 4}}>
            <View style={TEAM_CONTAINER_VIEW_STYLE}>
              <View style={{flexDirection: 'column'}}>
                <Text style={TITLE_STYLES}>Team 1</Text>
                <Text>Total Points {this.calculateTeam2TotalPoints()}</Text>
              </View>
              <FlatList
                style={{flexDirection:'row'}}
                data={Object.keys(this.state.game_state.team1_points)}
                renderItem={({item}: {item: any}) => <Card value={item} suit='Black'/>}
              />
            </View>
            <View style={TEAM_CONTAINER_VIEW_STYLE}>
              <View style={{flexDirection: 'column'}}>
                <Text style={TITLE_STYLES}>Team 2</Text>
                <Text>Total Points {this.calculateTeam1TotalPoints()}</Text>
              </View>
              <FlatList
                horizontal
                style={{flexDirection:'row'}}
                data={Object.keys(this.state.game_state.team2_points)}
                renderItem={({item}: {item: any}) => <Card value={item} suit='Black'/>}
              />
            </View>
          </View>
          <View style={DECK_CONTAINER_VIEW_STYLE}>

              <View style={card_style}>
                <View style={{paddingRight: 1, width: '100%', height: '100%', borderStyle: 'solid', borderWidth: 1, left: 1}}>
                  <View style={{paddingRight: 1, width: '100%', height: '100%',borderStyle: 'solid', borderWidth: 1, left: 2}}>
                    <View style={{paddingRight: 1, width: '100%', height: '100%', borderStyle: 'solid', borderWidth: 1, left: 3}}>
                      <View style={{position: 'absolute', padding: 10, width: '40%', left: '5%', top: '10%', height: '40%', borderStyle: 'solid', borderWidth: 2,}}/>
                      <View style={{position: 'absolute', padding: 10, width: '40%', left: '5%', top: '55%', height: '40%', borderStyle: 'solid', borderWidth: 2,}}/>
                      <View style={{position: 'absolute', padding: 10, width: '40%', left: '30%', top: '30%', height: '40%', borderStyle: 'solid', borderWidth: 2,}}/>
                      <View style={{position: 'absolute', padding: 10, width: '40%', right: '5%', top: '55%', height: '40%', borderStyle: 'solid', borderWidth: 2,}}/>
                      <View style={{position: 'absolute', padding: 10, width: '40%', right: '5%', top: '10%', height: '40%', borderStyle: 'solid', borderWidth: 2,}}/>
                    </View>
                  </View>
                </View>
              </View>
            {this.state.game_state.top_discard}
          </View>
        </View>

        <View style={PLAYER_CONTAINER_VIEW_STYLE}>
          <Text style={TITLE_STYLES}>Player</Text>
          <FlatList
            horizontal
            style={{flexDirection:'row'}}
            data={this.state.game_state?.player_hand}
            renderItem={({item}: {item: any})=> <View style={{marginRight: -50,}}>{item}</View>}
          />
        </View>
        <View style={GAME_ACTIONS_STYLE}>
          <View style={GAME_ACTION_STYLE}>
            <Button
              disabled={this.player_id != this.state.game_state.turn || this.state.did_draw}
              title="Draw Card"
              color="#678547"
              onPress={async()=>{ await this.drawCardAction(); this.state.did_draw = true;}}/>
          </View>
          <View style={GAME_ACTION_STYLE}>
            <Button
              disabled={
                this.player_id != this.state.game_state.turn ||
                this.state.did_draw
              }
              title="Pickup Deck"
              color="#678547"
              onPress={async()=>{
                if (this.getSelectedCards().length != 8) {
                  alert("EXACTLY 8 cards must be selected to pickup the deck");
                  return;
                }
                this.state.did_draw = true;
                this.forceUpdate();
              }}/>
          </View>
          <View style={GAME_ACTION_STYLE}>
            <Button
              disabled={
                this.player_id != this.state.game_state.turn ||
                !this.state.did_draw
              }
              title="Open"
              color="#678547"
              onPress={async()=>{
                this.forceUpdate();
              }}/>
          </View>
          <View style={GAME_ACTION_STYLE}>
            <Button
              disabled={
                this.player_id != this.state.game_state.turn ||
                !this.state.did_draw
              }
              title="Add Points"
              color="#678547"
              onPress={async()=>{}}/>
          </View>
          <View style={GAME_ACTION_STYLE}>
            <Button
              disabled={
                this.player_id != this.state.game_state.turn ||
                !this.state.did_draw
              }
              title="Discard"
              color="#678547"
              onPress={async()=>{await this.discardAction(); this.state.did_draw = false;}}/>
          </View>
        </View>
      </View>;
  }
  calculateTeam1TotalPoints(): number {
    return 0;
  }

  calculateTeam2TotalPoints(): number {
    return 0;
  }

  async drawCardAction() {
    await fetch('http://' + this.host + ':' + this.port + '/draw-card/?game-id=' + this.game_id + '&player=' + this.player_id, {
      method: "GET",
    }).catch((e) => {alert("Could not draw a card, an error occured."); throw e;} )
    .then((_) => {
      this.updateGameState();
      this.forceUpdate();
    });
  }

  async discardAction() {
    let selected_indices = this.getSelectedCards();
    if (selected_indices.length != 1) {
      alert("You must select ONE card to discard");
      return;
    }
    await fetch('http://' + this.host + ':' + this.port + '/discard-card', {
      method: "POST",
      headers: {
        Accept: 'application/json',
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(
        {'game_id': this.game_id, 'card_index': selected_indices[0]}
      )
    }).catch((e) => {alert("Could not discard "); throw e;} )
    .then((_) => {
      this.state.game_state.player_hand = [];
      this.state.selected = [];
      this.forceUpdate();
    });
  }

  async openAction() {
    let selected_indices = this.getSelectedCards();
    if (selected_indices.length != 1) {
      alert("You must select ONE card to discard");
      return;
    }
    await fetch('http://' + this.host + ':' + this.port + '/discard-card', {
      method: "POST",
      headers: {
        Accept: 'application/json',
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(
        {'game_id': this.game_id, 'card_index': selected_indices[0]}
      )
    }).catch((e) => {alert("Could not discard "); throw e;} )
    .then((_) => {
      this.state.game_state.player_hand = [];
      this.state.selected = [];
      this.forceUpdate();
    });
  }

  getSelectedCards(): number[] {
    var indices = [];
    for (var i = 0; i<this.state.selected.length; i++){
      if (this.state.selected[i])
        indices.push(i);
    }
    return indices;
  }

  updateSelections(index: number) {
    this.state.selected[index] = !this.state.selected[index];
  }

}

export default function(props: any) {
  const route = useRoute();

  return <Game {... props} route={route} />
}

