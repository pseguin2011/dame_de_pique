import React, { Component} from "react";
import {Card} from "../models/Card";
import { View, Button, TextInput } from "react-native";
import { uniqueNamesGenerator, Config, adjectives, names } from 'unique-names-generator';

type Player = {username: string};

const config: Config = {
  dictionaries: [adjectives, names],
  separator: '-',
  style: 'capital'
}

export default class Login extends Component {
  state: Player;
  host = '10.0.0.153';
  port = 8000;
  constructor(props: any) {
    super(props);
    this.state = {username: uniqueNamesGenerator(config)};
  }

  render() {
    const {navigation} = (this.props as {navigation: any});
    return <View style={{ backgroundColor: '#DAD7D7', width: '50%', height: '100%', padding: 10}}>
        <TextInput         
            style={{height: 60}}
            placeholder="Player Name"
            onChangeText={player_name => this.state = {username: player_name}}
            defaultValue={''}
        />
        <Button
            onPress={
                async () => {
                    try {
                        await this.registerPlayer(this.state.username);
                        navigation.push('Game Lobby', this.state)
                        navigation.navigate('Game Lobby');
                    } catch (e) {
                        alert("Username could not be registered");
                    }
                }
            }
            title="Connect to a game"
            color="#678547"
            accessibilityLabel="This button logs in a user and connects them to a game"
        />
    </View>;
  }
  
  async registerPlayer(username: string) {
    await fetch('http://' + this.host + ':' + this.port + '/player-register', {
      method: "POST",
      headers: {
        Accept: 'application/json',
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(
        { username: username }
      )
    }).then((response) => response.json())
    .then((json: {username: string, game_session_id: string, websocket_url: string}) => {
      this.state = json;
    })
    .catch((e) => { throw e; });
  }
}
