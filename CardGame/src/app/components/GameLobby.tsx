import React, { Component} from "react";
import { useRoute } from '@react-navigation/native';
import { View, Text, StyleProp, ViewStyle, Button } from "react-native";
import { FlatList } from "react-native-gesture-handler";
import network_config from '../config/Config';

type GameSession = {game_id: string, players: string[]}
type WebSocketResponse = {response_type: string, data: any};

class GameLobby extends Component {
  state: GameSession;
  socket: WebSocket | undefined;
  name: string;
  host = network_config.host;
  port = network_config.port;

  constructor(props: {route: any, navigation: any}) {
    super(props);
    this.state = {game_id: props.route.params.game_session_id, players: []};
    this.socket;
    this.name = props.route.params.username;
    this.connectToGame(props.route.params.username, props.route.params.game_session_id);
    this.connectWebSocket(props.route.params.websocket_url);
  }
  
  render() {
    let player_list_style: StyleProp<ViewStyle> = {
      backgroundColor: 'white',
      marginBottom: 4,
      padding: 10,
    };
    return <View style={{flex: 1, height: 2, alignContent: 'flex-start', flexDirection:'row'}}>
          <View style={{ backgroundColor: '#DAD7D7', width: '50%', height: '100%', padding: 10}}>
          <Text style={{fontWeight: 'bold', padding: 10}}> Joined Players ({this.state.players.length}/4)</Text>
          <FlatList
            data={this.state.players}
            renderItem={({item}: {item: string})=>(
              <View style={player_list_style}>
                <Text style={{textAlign: 'center', fontWeight: 'bold', fontSize: 20}}>{item}</Text>
              </View>
            )}
          />
          <Button 
            disabled={this.state.players.length < 4}
            onPress={
              async () => {
                this.startGame()
              }
            }
            title="Start Game"
            color="#678547"
            accessibilityLabel="This button creates a new game"

          />
      </View>
    </View>;
  }

  connectWebSocket(url: string) {
    this.socket = new WebSocket(url);
    
    this.socket.onmessage = (e) => {
      let json: WebSocketResponse = JSON.parse(e.data);
      switch (json.response_type) {
        case "GameSession":
          this.updatePlayers(json.data['players']);
          break;
        case "StartGameResponse":
          const {navigation} = (this.props as {navigation: any});
          navigation.push('Game', {
            game_id:    this.state.game_id,
            websocket:  this.socket,
            websocket_url: url,
            player_id:  this.state.players.indexOf(this.name),
            player_names: this.state.players,
          });
          navigation.navigate('Game');
          break;
      }
    };
    
    this.socket.onerror = (e) => {
      // an error occurred
      console.log((e as any).message);
    };
    
    this.socket.onclose = (e) => {
      // connection closed
      console.log(e.code, e.reason);
    };
  }

  connectToGame(username: string, game_id: string) {
    fetch('http://' + this.host + ':' + this.port + '/game-register', {
      method: "POST",
      headers: {
        Accept: 'application/json',
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(
        {'game_identifier': game_id, 'player_username': username}
      )
    }).catch((e) => {alert("Could not join the game, room must be full."); throw e;} )
    .then((response) => response.json())
    .then((json: { game_id: string, players: [string], max_capacity: number, url: string} ) => {
      this.state.players = [];
      json.players.forEach(element => this.state.players.push(element));
      this.forceUpdate();
    });
  }

  startGame() {
    fetch('http://' + this.host + ':' + this.port + '/game-start', {
      method: "POST",
      headers: {
        Accept: 'application/json',
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(
        {'game_id': this.state.game_id}
      )
    }).catch((e) => {alert("Could not start the game."); throw e;} );
  }

  updatePlayers(players: string[]) {
    this.state.players = players;
    this.forceUpdate();
  }
}

export default function(props: any) {
  const route = useRoute();

  return <GameLobby {... props} route={route} />
}