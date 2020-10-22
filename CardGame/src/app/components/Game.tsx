import React, { Component } from 'react';
import { View, Text, ViewStyle, Button, TextStyle} from 'react-native';
import { FlatList } from "react-native-gesture-handler";
import {Card} from '../models/Card';
import { useRoute } from '@react-navigation/native';

const GAME_ACTIONS_STYLE: ViewStyle = {
  height: 5, 
  alignContent: 'flex-start', 
  flexDirection:'row',
};

const GAME_ACTION_STYLE: ViewStyle = {
  margin: 10,
};
const CARD_CONTAINER_VIEW_STYLE: ViewStyle = {
  margin: 20,
  padding: 20,
  backgroundColor: 'white',
  borderRadius: 5,
  left: '0%',
};

const TEAM_CONTAINER_VIEW_STYLE: ViewStyle = {
  flex:1,
  height: 150,
  ... CARD_CONTAINER_VIEW_STYLE
};

const PLAYER_CONTAINER_VIEW_STYLE: ViewStyle = {
  height: 150,
  ... CARD_CONTAINER_VIEW_STYLE
};

const DECK_CONTAINER_VIEW_STYLE: ViewStyle = {
  flex: 1.5,
  height: 300,
  maxWidth: 300,
  ... CARD_CONTAINER_VIEW_STYLE
};



type GameState = {
  game_id: String,
  socket: WebSocket,
  player_id: number,
  game_state: {
    player_hand: Card[],
    team1_points: Card[],
    team2_points: Card[],
    top_discard?: Card,
    turn: number,
  },
};

const TITLE_STYLES: TextStyle = {
  color: 'Black',
  fontWeight: 'bold',
  margin: 5,
  fontSize: 20
};

type WebSocketResponse = {response_type: string, data: any};

export class Game extends Component {
  state: GameState;
  host = '192.168.2.101';
  port = 8000;
  constructor(props: any) {
    super(props);
    this.state = {
      game_id: props.route.params.game_id,
      socket: props.route.params.websocket,
      player_id: props.route.params.player_id,
      game_state: {
        player_hand: [],
        team1_points: [],
        team2_points: [],
        top_discard: undefined,
        turn: 0,
      },
    };
    this.state.socket.onmessage = (e) => {
      let json: WebSocketResponse = JSON.parse(e.data);
      switch (json.response_type) {
        case "GameState":
          alert!("Update")
          break;
      }
    };
    this.updateGameState();
  }

  updateGameState() {
    fetch('http://' + this.host + ':' + this.port + '/game-state/?game-id=' + this.state.game_id + '&player=' + this.state.player_id, {
      method: "GET",
      headers: {
        Accept: 'application/json',
        'Content-Type': 'application/json',
      },
    }).catch((e) => {alert("Could not join the game, room must be full."); throw e;} )
    .then((response) => response.json())
    .then((json: any ) => {
      this.state.game_state = {
        player_hand: json.player_hand.map((card: any) => <Card value={card.value} suit={card.suit}/>),
        team1_points: json.team1_points,
        team2_points: json.team2_points,
        turn: json.turn,
        top_discard: <Card value={json.top_discard.value} suit={json.top_discard.suit}/>,
      };
      this.forceUpdate();
    });
  }

  render(): JSX.Element {
      return <View style={{backgroundColor: 'darkgreen', height: '100%'}}>
        <View style={{flexDirection: 'row', flexWrap: 1}}>
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
            {this.state.game_state.top_discard}
          </View>
        </View>

        <View style={PLAYER_CONTAINER_VIEW_STYLE}>
          <Text style={TITLE_STYLES}>Player</Text>
          <FlatList
            horizontal
            style={{flexDirection:'row'}}
            data={this.state.game_state?.player_hand}
            renderItem={({item}: {item: any})=> item}
          />
        </View>
        <View style={GAME_ACTIONS_STYLE}>
          <View style={GAME_ACTION_STYLE}>
            <Button
              title="Draw Card"
              color="#678547"
              onPress={async()=>{}}/>
          </View>
          <View style={GAME_ACTION_STYLE}>
            <Button
              title="Pickup Deck"
              color="#678547"
              onPress={async()=>{}}/>
          </View>
          <View style={GAME_ACTION_STYLE}>
            <Button
              title="Open / Add Points"
              color="#678547"
              onPress={async()=>{}}/>
          </View>
          <View style={GAME_ACTION_STYLE}>
            <Button
              title="Discard"
              color="#678547"
              onPress={async()=>{}}/>
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
}

export default function(props: any) {
  const route = useRoute();

  return <Game {... props} route={route} />
}


    // let game = props.route.params;
    // let game_container_view_style = {
    //   left: '10%',
    //   margin: 20,
    //   width: '80%',
    //   height: 110,
    //   backgroundColor: 'white',
    // };
    // let game_section_title_style = {
    // };

// return <View style={{backgroundColor: 'darkgreen'}}>
//       {/* <View style ={game_container_view_style}>
//         <Text style={game_section_title_style}>Team 1's Points</Text>
//         {loadCardGroupedView(cards)}
//       </View>
//       <View style ={game_container_view_style}>
//         <Text style={game_section_title_style}>Team 2's Points</Text>
//         {loadCardGroupedView(cards)}
//       </View> */}
//       <View style ={game_container_view_style}>
//         {/* {loadCardsAsView(cards, cards)} */}
//         {cards[0].render()}
//       </View>
//     </View>
// }

// function loadCardsAsView(hand_cards: Card[], deck: Card[]) {
// let card_container_view_style = {
//   marginTop: 2, 
//   paddingLeft: 3,
//   paddingRight: 3,
// };
// return <View style={card_container_view_style}>
//     <View style={{paddingRight: 0.3, paddingBottom: 1, flexDirection: 'row', flex: 1, alignItems: 'center'}}>
//       <Text style={{fontWeight: 'bold', fontSize: 1.5,}}>Deck</Text>
//       {/* {loadDeck()} */}
//       {/* {loadFullCard(deck[0])} */}
//     </View>
//     <View style={{paddingRight: 0.3, paddingBottom: 1, flexDirection: 'row', flex: 1, alignItems: 'center'}}>
//       <Text style={{fontWeight: 'bold', fontSize: 1.5,}}>Hand</Text>

//       <FlatList
//         style={{flexDirection: 'row', flex: 1, marginLeft: 0.5,}}
//         horizontal
//         data={hand_cards}
//         renderItem={({item}) => item.render()}
//       />
//     </View>
//   </View>;
// }

// function loadCardGroupedView(cards: [Card]) {
//     var found_nums = {};
//     let uniqueCards = cards
//       .filter((c: Card, i: number, cards_b: Card[]) => {
//         return found_nums.hasOwnProperty(c.CardValue)? false : (found_nums[c.CardValue] = true);
//       })
//       .sort((a: Card, b: Card) => a.CardValue.localeCompare(b.CardValue));
//   var cardResponse: JSX.Element[] = [];
//     uniqueCards.forEach(card => cardResponse.push(loadFullCard(card)));

//   let card_container_view_style = {
//     marginTop: '2, 
//     flexDirection: 'row',
//     paddingLeft: '3,
//     paddingRight: '3,
//   };
//   return <View style={card_container_view_style}>
//     { cardResponse }
//   </View>;
// }