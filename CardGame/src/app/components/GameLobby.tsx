import React, { Component} from "react";
import { useRoute } from '@react-navigation/native';
import { View, Text, StyleProp, ViewStyle, Button } from "react-native";
import { FlatList } from "react-native-gesture-handler";

type Player = {username: string};
type ConnectedPlayers ={connected_players: Player[]};
type WebSocketResponse = {response_type: string, data: any};

class GameLobby extends Component {
  socket: WebSocket | undefined;
  host = '192.168.2.101';
  port = 8000;

  constructor(props: {route: any, navigation: any}) {
    super(props);
    this.state = {connected_players: []};
    this.socket;
    this.connectToGame(props.route.params.username);
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
          <Text style={{fontWeight: 'bold', padding: 10}}> Joined Players ({(this.state as ConnectedPlayers).connected_players.length}/4)</Text>
          <FlatList
            data={(this.state as ConnectedPlayers).connected_players}
            renderItem={({item}: {item: Player})=>(
              <View style={player_list_style}>
                <Text style={{textAlign: 'center', fontWeight: 'bold', fontSize: 20}}>{item.username}</Text>
              </View>
            )}
          />
          <Button 
            disabled={(this.state as ConnectedPlayers).connected_players.length < 4}
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
          // const {navigation} = (this.props as {navigation: any});
          // navigation.navigate('Game');
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

  connectToGame(username: string) {
    fetch('http://' + this.host + ':' + this.port + '/game-register', {
      method: "POST",
      headers: {
        Accept: 'application/json',
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(
        {'game_identifier': '1', 'player_username': username}
      )
    }).catch((e) => {alert("Could not join the game, room must be full."); throw e;} )
    .then((response) => response.json())
    .then((json: { game_id: string, players: [string], max_capacity: number, url: string} ) => {
      (this.state as ConnectedPlayers).connected_players = [];
      json.players.forEach(element => (this.state as ConnectedPlayers).connected_players.push({username: element}));
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
        {'game_id': '1'}
      )
    }).catch((e) => {alert("Could not start the game."); throw e;} );
  }

  updatePlayers(players: any) {
    let mapped_players = players.map((player: string) => { return {username: player} });
    (this.state as ConnectedPlayers).connected_players = [];
    mapped_players.forEach((element: Player) => (this.state as ConnectedPlayers).connected_players.push(element));
    this.forceUpdate();
  }
}

export default function(props: any) {
  const route = useRoute();

  return <GameLobby {... props} route={route} />
}