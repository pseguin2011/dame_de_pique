import React from 'react';
import {View, Text} from 'react-native';
import { NavigationContainer } from '@react-navigation/native';
import { createStackNavigator } from '@react-navigation/stack';


const Stack = createStackNavigator<RootStackParamList>();

type RootStackParamList = {
    'Home': {};
};

const AppNavigator = () => <NavigationContainer>
        <Stack.Navigator initialRouteName="Home">
            {<Stack.Screen name="Home" component={HelloWorldComponent} />}
        </Stack.Navigator>
    </NavigationContainer>;
export default AppNavigator;

function HelloWorldComponent() {
    return <View><Text>Hello World</Text></View>;
}