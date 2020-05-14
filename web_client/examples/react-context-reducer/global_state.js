import React, { useReducer, createContext } from "react";
import reducer from "./reducer";

export const GlobalStateContext = createContext();

const GlobalStateProvider = (props) => {
	const [state, dispatch] = useReducer(reducer, []);
	return (
		<GlobalStateContext.Provider value={{ state, dispatch }}>
			{props.children}
		</GlobalStateContext.Provider>
	);
};

export default GlobalStateProvider;
