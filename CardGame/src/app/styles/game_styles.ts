import { ViewStyle, TextStyle} from 'react-native';
export const GAME_ACTIONS_STYLE: ViewStyle = {
    height: 5, 
    alignContent: 'flex-start', 
    flexDirection:'row',
    flexWrap: 'wrap',
    flex: 1,
    width: '100%',
  };
  
  export const GAME_ACTION_STYLE: ViewStyle = {
    margin: 20,
    width: 160,
  };

  const CARD_CONTAINER_VIEW_STYLE: ViewStyle = {
    margin: 7,
    padding: 10,
    backgroundColor: 'white',
    borderRadius: 5,
    left: '0%',
  };
  
  export const TEAM_CONTAINER_VIEW_STYLE: ViewStyle = {
    flex:1,
    height: 180,
    ... CARD_CONTAINER_VIEW_STYLE
  };
  
  export const PLAYER_CONTAINER_VIEW_STYLE: ViewStyle = {
    height: 180,
    ... CARD_CONTAINER_VIEW_STYLE
  };
  
  export const DECK_CONTAINER_VIEW_STYLE: ViewStyle = {
    flex: 1,
    flexDirection: "row",
    flexWrap: 'wrap',
    height: 240,
    minWidth: 30,
    ... CARD_CONTAINER_VIEW_STYLE
  };
  
  export const TITLE_STYLES: TextStyle = {
    color: 'Black',
    fontWeight: 'bold',
    margin: 5,
    fontSize: 20
  };