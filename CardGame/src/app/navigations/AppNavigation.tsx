import React from 'react';
import { NavigationContainer } from '@react-navigation/native';
import { createStackNavigator } from '@react-navigation/stack';

import Game from '../components/Game';
import GameLobby from '../components/GameLobby';
// import { MainLobby } from '../components/Lobby';
import Login from '../components/Login';

const Stack = createStackNavigator<RootStackParamList>();

type RootStackParamList = {
    'Home': {};
    'Main Lobby': {};
    'Game Lobby': {};
    'Game': {};
};

const AppNavigator = () => <NavigationContainer>
        <Stack.Navigator initialRouteName="Home">
            <Stack.Screen name="Home" component={Login} />
            {/* <Stack.Screen name="Main Lobby" component={MainLobby} /> */}
            <Stack.Screen name="Game Lobby" component={GameLobby} />
            <Stack.Screen name="Game" component={Game} />
        </Stack.Navigator>
    </NavigationContainer>;
export default AppNavigator;