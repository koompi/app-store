import React, { createContext, useState } from "react";
import jwt from "jsonwebtoken";

export const AuthContext = createContext();
const token = window.localStorage.getItem("token");
const user_data = jwt.decode(token);
const AuthContextProvider = (props) => {
	const [user, setUser] = useState({ ...user_data });

	return (
		<AuthContext.Provider value={user}>{props.children}</AuthContext.Provider>
	);
};

export default AuthContextProvider;
