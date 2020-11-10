import React, { Component }  from 'react';
import { View, ViewStyle } from 'react-native';

const card_style: ViewStyle = {
    width: 90, 
    height: 100,
    margin: 5,
    top: 5,
    borderRadius: 7,
    borderStyle: 'solid',
    borderWidth: 3,
  };

const DECK_STACK_STYLE:ViewStyle = {
    borderRadius: 5,
    marginLeft: -1,
    paddingRight: 2, 
    width: '100%',
    height: '100%',
    borderStyle: 'solid',
    borderLeftWidth: 3,
    backgroundColor: '#3895D3',
};

export default class Deck extends Component<{}, {}> {

    constructor(props: any) {
        super(props);
    }

    render(): JSX.Element {
        return <View style={card_style}>
            <View style={{...DECK_STACK_STYLE, left: 1}}>
                <View style={{...DECK_STACK_STYLE, borderTopWidth: 0, borderBottomWidth: 0, left: 2}}>
                    <View style={{...DECK_STACK_STYLE, borderTopWidth: 0, borderBottomWidth: 0, left: 3}}>
                        <View style={{position: 'absolute', padding: 10, width: '40%', left: '5%', top: '10%', height: '40%', borderStyle: 'solid', borderWidth: 2,}}/>
                        <View style={{position: 'absolute', padding: 10, width: '40%', left: '5%', top: '55%', height: '40%', borderStyle: 'solid', borderWidth: 2,}}/>
                        <View style={{position: 'absolute', padding: 10, width: '40%', left: '30%', top: '30%', height: '40%', borderStyle: 'solid', borderWidth: 2,}}/>
                        <View style={{position: 'absolute', padding: 10, width: '40%', right: '5%', top: '55%', height: '40%', borderStyle: 'solid', borderWidth: 2,}}/>
                        <View style={{position: 'absolute', padding: 10, width: '40%', right: '5%', top: '10%', height: '40%', borderStyle: 'solid', borderWidth: 2,}}/>
                    </View>
                </View>
            </View>
        </View>;
    }
}