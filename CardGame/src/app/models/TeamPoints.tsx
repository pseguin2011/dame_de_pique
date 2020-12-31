import React, { Component }  from 'react';
import { View, Text} from 'react-native';
import { FlatList } from "react-native-gesture-handler";

import {Card, CARD_SUIT, CARD_VALUE} from '../models/Card';
import {TITLE_STYLES, TEAM_CONTAINER_VIEW_STYLE} from '../styles/game_styles';

type CardType = {value: CARD_VALUE, suit: CARD_SUIT};
type TeamPointsType = {
    'A'?: CardType[], '2'?: CardType[], '3'?: CardType[], '4'?: CardType[],
    '5'?: CardType[], '6'?: CardType[], '7'?: CardType[], '8'?: CardType[],
    '9'?: CardType[], '10'?: CardType[], 'J'?: CardType[], 'Q'?: CardType[],
    'K'?: CardType[], 'Joker'?:CardType[], ''?: CardType[]
  };

const POINT_VALUES = {
    '': {'': 0},
    'A': {'Spades':15, 'Hearts':15,'Diamonds':15, 'Clubs':15},
    '2': {'Spades':20, 'Hearts':20,'Diamonds':20, 'Clubs':20},
    '3': {'Spades':5, 'Hearts':5,'Diamonds':5, 'Clubs':5},
    '4': {'Spades':5, 'Hearts':5,'Diamonds':5, 'Clubs':5},
    '5': {'Spades':5, 'Hearts':5,'Diamonds':5, 'Clubs':5},
    '6': {'Spades':5, 'Hearts':5,'Diamonds':5, 'Clubs':5},
    '7': {'Spades':5, 'Hearts':5,'Diamonds':5, 'Clubs':5},
    '8': {'Spades':5, 'Hearts':5,'Diamonds':5, 'Clubs':5},
    '9': {'Spades':5, 'Hearts':5,'Diamonds':5, 'Clubs':5},
    '10': {'Spades':10, 'Hearts':10, 'Diamonds':10, 'Clubs':10},
    'J':  {'Spades':10, 'Hearts':10, 'Diamonds':10, 'Clubs':10},
    'Q':  {'Spades':100, 'Hearts':10, 'Diamonds':10, 'Clubs':10},
    'K':  {'Spades':10, 'Hearts':10, 'Diamonds':10, 'Clubs':10},
    'Joker': {'Black':50, 'Red':50},
  };

export default class TeamPoints extends Component<{}, {}> {
    props: { team_points: TeamPointsType, team_name?: string } = {team_points: {'':[]}};

    constructor(props: any) {
        super(props);
    }

    calculateTeamRoundPoints(): number {
        var total = 0;
        Object.entries(this.props.team_points).forEach(value => {
          value[1]?.forEach((card:CardType) => {
            total += (POINT_VALUES as any)[card.value][card.suit];
          });
        });
        return total;
      }

      render() {
        return <View style={TEAM_CONTAINER_VIEW_STYLE}>
        <View style={{flexDirection: 'row'}}>
          <View style={{flexDirection: 'column'}}>
            <Text style={TITLE_STYLES}>{this.props.team_name}</Text>
            <Text>Round Points {this.calculateTeamRoundPoints()}</Text>
          </View>
          <FlatList
            horizontal
            style={{flexDirection:'row', overflow: 'scroll'}}
            data={Object.entries(this.props.team_points)}
            renderItem={({item}: {item: [string, CardType[] | undefined]}) => {
              var cards = item[1]?.map(card => {
                return <View style={{marginRight: -55, paddingBottom: 30 }}>
                  <Card value={card.value} suit={card.suit} selected={false}/>
                </View>;
              });
              return <View style={{flexDirection: 'row', paddingRight: 50}}>{cards}</View>;
            }}
          />
        </View>
      </View>;
      }
}