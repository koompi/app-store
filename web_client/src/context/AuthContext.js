import React, { createContext, useReducer } from "react";
import { auth_reducer } from "../reducers/auth_reducer";
import { decode } from "jsonwebtoken";
// Check token
const token = window.localStorage.getItem("token");
const user_data = decode(token);

console.log("User Data: ", user_data);
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
