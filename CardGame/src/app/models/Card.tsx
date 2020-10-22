import React from 'react';
import { Component } from 'react';
import { View, Text, TextStyle, ViewStyle, Pressable } from 'react-native';

const card_style: ViewStyle = {
    borderStyle: 'solid',
    borderWidth: 3,
    borderRadius: 3,
    width: 80, 
    height: 100,
    backgroundColor: 'white',
    margin: 5,
    marginRight: -40,
};
const card_value_style: TextStyle = {
    position: 'absolute',
    fontWeight: 'bold',
    includeFontPadding: false,
    fontSize: 25,
};
const card_value_top_left_style: TextStyle = {
    top: 0,
    left: 0,
    fontSize: 25,
    ... card_value_style
};
const card_value_bottom_right_style: TextStyle = {
    right: 0,
    bottom: 0,
    transform:[{rotate: '180deg'}],
    ... card_value_style
};

const SUIT_STYLE: TextStyle = {
    position: 'absolute',
    textAlign: 'center',
    textAlignVertical: 'bottom',
    alignContent: 'center',
    fontSize: 40,
};


const suit_logo_style: TextStyle = {
    justifyContent: 'center',
    alignItems: 'center',
    flex: 1,
    width: '100%',
    height: '100%',
};

const SUIT = {
    'Hearts':     {style: { color: 'red',   ... SUIT_STYLE}, value: '♥'},
    'Diamonds':   {style: { color: 'red',   ... SUIT_STYLE}, value: '♦'},
    'Red':        {style: { color: 'red',   ... SUIT_STYLE}, value: ''},
    'Spades':     {style: { color: 'black', ... SUIT_STYLE}, value: '♠'},
    'Clubs':      {style: { color: 'black', ... SUIT_STYLE}, value: '♣'},
    'Black':      {style: { color: 'black', ... SUIT_STYLE}, value: ''},
};


export type CARD_SUIT = 'Spades'|'Diamonds'|'Hearts'|'Clubs'|'Red'|'Black';
export type CARD_VALUE = 'A'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9'|'10'|'J'|'Q'|'K'|'Joker';

export class Card extends Component {
    props: {suit: CARD_SUIT, value: CARD_VALUE} = {suit: 'Black', value: '3'};
    state: {selected: boolean} = {selected: false};
     
    render() {
        let value = this.props.value;
        let suit = SUIT[this.props.suit];
        
        return <Pressable onPress={async () =>{this.state.selected = !this.state.selected; this.forceUpdate()}}>
            <View style={card_style}>
                <View style={(this.state.selected)?{opacity: 0.5, width: '100%', height: '100%'}:{width: '100%', height: '100%'} }>
                    <Text style={card_value_top_left_style}>{value}</Text>
                    <View style={suit_logo_style}>
                        <Text style={suit.style}>{suit.value}</Text>
                    </View>
                    <Text style={card_value_bottom_right_style}>{value}</Text>
                </View>
            </View>
        </Pressable>;
    }
    
}
  