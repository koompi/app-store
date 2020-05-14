import React, { createContext, useReducer } from "react";
import { auth_reducer } from "../reducers/auth_reducer";

// Check token
const token = window.localStorage.getItem("token");
const user_data = JSON.parse(token);

export const AuthContext = createContext();

const AuthContextProvider = (props) => {
	const [user, dispatch] = useReducer(auth_reducer, {
		...user_data,
	});

	return (
		<AuthContext.Provider value={{ user, dispatch }}>
			{props.children}
		</AuthContext.Provider>
	);
};

export default AuthContextProvider;
