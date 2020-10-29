import React, { Component }  from 'react';
import { View, ViewStyle } from 'react-native';

const card_style: ViewStyle = {
    borderStyle: 'solid',
    borderWidth: 1,
    borderRadius: 3,
    width: 80, 
    height: 100,
    margin: 5,
    backgroundColor: 'red',
  };

export default class Deck extends Component<{}, {}> {

    constructor(props: any) {
        super(props);
    }

    render(): JSX.Element {
        return <View style={card_style}>
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
        </View>;
    }
}