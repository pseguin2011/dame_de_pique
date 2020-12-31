import React, { Component } from 'react';
import { View, Text, Button, Pressable} from 'react-native';
import { FlatList } from "react-native-gesture-handler";

import { useRoute } from '@react-navigation/native';

import GameClient from "../clients/game_client";

import network_config from '../config/Config';
import {Card, CARD_SUIT, CARD_VALUE} from '../models/Card';
import Deck from '../models/Deck';

import {DECK_CONTAINER_VIEW_STYLE, PLAYER_CONTAINER_VIEW_STYLE, GAME_ACTIONS_STYLE} from '../styles/game_styles';
import {TITLE_STYLES, GAME_ACTION_STYLE, TEAM_CONTAINER_VIEW_STYLE} from '../styles/game_styles';
import TeamPoints from '../models/TeamPoints';

type CardType = {value: CARD_VALUE, suit: CARD_SUIT};
type TeamPointsType = {
  'A'?: CardType[], '2'?: CardType[], '3'?: CardType[], '4'?: CardType[],
  '5'?: CardType[], '6'?: CardType[], '7'?: CardType[], '8'?: CardType[],
  '9'?: CardType[], '10'?: CardType[], 'J'?: CardType[], 'Q'?: CardType[],
  'K'?: CardType[], 'Joker'?:CardType[], ''?: CardType[]
};
type GameState = {
  selected: boolean[],
  did_draw: boolean,
  round_over: boolean,
  game_state: {
    player_hand: {card: CardType, index: number}[],
    team1_points: TeamPointsType,
    team2_points: TeamPointsType,
    team_1_total_points: number,
    team_2_total_points: number,
    top_discard?: CardType,
    turn: number,
  },
};

type WebSocketResponse = {response_type: string, data: any};

export class Game extends Component {
  state: GameState;
  host = network_config.host;
  port = network_config.port;
  client: GameClient;
  game_id: string;
  socket: WebSocket;
  player_id: number;
  player_names: string[];

  constructor(props: any) {
    super(props);
    this.game_id = props.route.params.game_id;
    this.socket = props.route.params.websocket;
    this.player_id = props.route.params.player_id;
    this.player_names = props.route.params.player_names;
    this.client = new GameClient(this.game_id, this.player_id);
    this.state = { 
      selected: [],
      did_draw: false,
      game_state: {
        player_hand: [],
        team1_points: {'': []},
        team2_points: {'': []},
        team_1_total_points: 0,
        team_2_total_points: 0,
        top_discard: undefined,
        turn: 0,
      },
      round_over: false,
    };

    // Updates the websocket messages received to handle game state messages while in this component.
    this.socket.onmessage = (e) => {
      let json: WebSocketResponse = JSON.parse(e.data);
      switch (json.response_type) {
        case "GameState":
          this.updateGameState();
          break;
        case "EndRound":
          this.endRound();
          break;
        case "EndGame":
          this.endGame();
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

  pickupDiscardAction() {
    this.client.pickupDiscardAction(this.getSelectedCards(), async() => {
      this.state.did_draw = true;
      this.forceUpdate();
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

  startNewRound() {
    this.state.round_over = false;
    fetch('http://' + this.host + ':' + this.port + '/game-start', {
      method: "POST",
      headers: {
        Accept: 'application/json',
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(
        {'game_id': '1'}
      )
    }).catch((e) => {alert("Could not start the game."); throw e;} );
  }

  endRound() {
    this.client.updateGameState(async (json: any) => {
      this.state.round_over = true;
      alert("Round Has Ended \nTeam 1 has: " + json.team_1_total_points + " points\nTeam 2 has: " + json.team_2_total_points + " points");
    });
  }

  endGame() {
    alert("Game Has Ended");
  }

  render(): JSX.Element {
      return <View style={{backgroundColor: 'darkgreen', height: '100%', overflow: 'scroll'}}>
        <View style={{flexDirection: 'row'}}>
          <View style={{flexDirection: 'column', flex: 4}}>
            <TeamPoints team_points={this.state.game_state.team1_points} team_name="Team 1" />
            <TeamPoints team_points={this.state.game_state.team2_points} team_name="Team 2" />
          </View>
          <View style={DECK_CONTAINER_VIEW_STYLE}>
              <View style={{flexDirection: 'column'}}>
              <Deck/>
              <Card value={this.state.game_state.top_discard?.value as CARD_VALUE} suit={this.state.game_state.top_discard?.suit as CARD_SUIT} selected={false}/>
              <View>
                <Text style={TITLE_STYLES}>Player</Text>
                <Text style={{fontSize: 15, fontWeight: 'bold'}}>{this.player_names[this.player_id]}</Text>
                <Text style={{fontSize: 12, fontWeight: 'bold'}}>Team {((this.player_id % 2) == 0) ? 1 : 2} / Player {this.player_id + 1} </Text>
                <Text style={TITLE_STYLES}>Team Points</Text>
                <Text style={{fontSize: 15}}>Team 1: {this.state.game_state.team_1_total_points}</Text>
                <Text style={{fontSize: 15}}>Team 2: {this.state.game_state.team_2_total_points}</Text>
                <Text style={TITLE_STYLES}>Turn</Text>
                <Text style={{fontSize: 12}}>{this.player_names[this.state.game_state.turn]}'s Turn</Text>
              </View>
              </View>
          </View>
        </View>
        <View style={{...PLAYER_CONTAINER_VIEW_STYLE}}>
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
              onPress={async()=>{ await this.pickupDiscardAction();}}/>
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
              onPress={async()=>{
                this.client.addPointsAction(this.getSelectedCards());
              }}/>
          </View>
          <View style={GAME_ACTION_STYLE}>
            <Button
              disabled={this.player_id != this.state.game_state.turn || !this.state.did_draw}
              title="Discard"
              color="#678547"
              onPress={async()=> await this.discardAction()}/>
          </View>
          {this.player_id == 0 &&
          <View style={GAME_ACTION_STYLE}>
            <Button
              disabled={!this.state.round_over}
              title="Start new round"
              color="#678547"
              onPress={async()=> await this.startNewRound()}/>
          </View>
          }
        </View>
      </View>;
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

