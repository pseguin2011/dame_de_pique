import React, { Component } from 'react';
import { View, Text, Button, Pressable} from 'react-native';
import { FlatList } from "react-native-gesture-handler";

import { useRoute } from '@react-navigation/native';

import GameClient from "../clients/game_client";

import {Card, CARD_SUIT, CARD_VALUE} from '../models/Card';
import Deck from '../models/Deck';

import {DECK_CONTAINER_VIEW_STYLE, PLAYER_CONTAINER_VIEW_STYLE, GAME_ACTIONS_STYLE} from '../styles/game_styles';
import {TITLE_STYLES, GAME_ACTION_STYLE, TEAM_CONTAINER_VIEW_STYLE} from '../styles/game_styles';

type CardType = {value: CARD_VALUE, suit: CARD_SUIT};
type GameState = {
  selected: boolean[],
  did_draw: boolean,
  game_state: {
    player_hand: {card: CardType, index: number}[],
    team1_points: CardType[],
    team2_points: CardType[],
    top_discard?: CardType,
    turn: number,
  },
};

type WebSocketResponse = {response_type: string, data: any};

export class Game extends Component {
  state: GameState;
  host = '10.0.0.153';
  port = 8000;
  client: GameClient;
  game_id: string;
  socket: WebSocket;
  player_id: number;

  constructor(props: any) {
    super(props);
    this.game_id = props.route.params.game_id;
    this.socket = props.route.params.websocket;
    this.player_id = props.route.params.player_id;
    this.client = new GameClient(this.game_id, this.player_id);
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

    // Updates the websocket messages received to handle game state messages while in this component.
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

  /// Sends the draw card action for the current game to the server.
  /// Force updates the game state and sets the did_draw to true upon receiving a successful response.
  drawCardAction() {
    this.client.drawCardAction(async () => {
      this.updateGameState();
      this.state.did_draw = true;
    });
  }

  /// Discards a single selected card
  /// Resets the did_draw value so the correct buttons are disabled
  discardAction() {
    let selected_indices = this.getSelectedCards();
    if (selected_indices.length != 1) {
      alert("You must select ONE card to discard");
      return;
    }
    this.client.discardAction(selected_indices[0]);
    this.state.did_draw = false;
  }

  /// Fetches the current game state and updates the all components and unselects all cards. 
  updateGameState() {
    this.client.updateGameState(async (json: any) => {
      this.state.game_state = json;
      this.state.game_state.player_hand = json.player_hand.map((card: any, i: number) => {return {card: card, index: i}});
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
                renderItem={({item}: {item: any}) => <Card value={item} suit='Black' selected={false}/>}
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
                renderItem={({item}: {item: any}) => <Card value={item} suit='Black' selected={false}/>}
              />
            </View>
          </View>
          <View style={DECK_CONTAINER_VIEW_STYLE}>
              <Deck/>
              <Card value={this.state.game_state.top_discard?.value} suit={this.state.game_state.top_discard?.suit} selected={false}/>
          </View>
        </View>

        <View style={PLAYER_CONTAINER_VIEW_STYLE}>
          <Text style={TITLE_STYLES}>Player</Text>
          <FlatList
            horizontal
            style={{flexDirection:'row'}}
            data={this.state.game_state?.player_hand}
            renderItem={({item}: {item: any})=> <View style={{marginRight: -50}}>
              <Pressable onPress={() => {
                  this.state.selected[item.index] = !this.state.selected[item.index];
                  this.forceUpdate();
                }}
              >
                <Card selected={this.state.selected[item.index]} id={item.index} value={item.card.value} suit={item.card.suit}/>
              </Pressable>
            </View>}
          />
        </View>
        <View style={GAME_ACTIONS_STYLE}>
          <View style={GAME_ACTION_STYLE}>
            <Button
              disabled={this.player_id != this.state.game_state.turn || this.state.did_draw}
              title="Draw Card"
              color="#678547"
              onPress={async()=>{ await this.drawCardAction(); }}/>
          </View>
          <View style={GAME_ACTION_STYLE}>
            <Button
              disabled={this.player_id != this.state.game_state.turn || this.state.did_draw}
              title="Pickup Deck"
              color="#678547"
              onPress={async()=>{
                if (this.getSelectedCards().length != 8) {
                  alert("EXACTLY 8 cards must be selected to pickup the deck and at least one must have the same value as the discarded card.");
                  return;
                }
                this.state.did_draw = true;
                this.forceUpdate();
              }}/>
          </View>
          <View style={GAME_ACTION_STYLE}>
            <Button
              disabled={this.player_id != this.state.game_state.turn || !this.state.did_draw}
              title="Open"
              color="#678547"
              onPress={async()=>{
                this.client.openAction(this.getSelectedCards());
              }}/>
          </View>
          <View style={GAME_ACTION_STYLE}>
            <Button
              disabled={this.player_id != this.state.game_state.turn || !this.state.did_draw}
              title="Add Points"
              color="#678547"
              onPress={async()=>{}}/>
          </View>
          <View style={GAME_ACTION_STYLE}>
            <Button
              disabled={this.player_id != this.state.game_state.turn || !this.state.did_draw}
              title="Discard"
              color="#678547"
              onPress={async()=> await this.discardAction()}/>
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

  /// ## Purpose
  /// Searches through the player's hand to find cards with a selected state of true
  ///
  /// ## Returns
  /// All indices of selected cards
  getSelectedCards(): number[] {
    var indices = [];
    for (var i = 0; i<this.state.selected.length; i++){
      if (this.state.selected[i])
        indices.push(i);
    }
    return indices;
  }
}

export default function(props: any) {
  const route = useRoute();
  return <Game {... props} route={route} />
}
