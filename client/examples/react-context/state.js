import React, { useState, createContext } from "react";

// Initial state
const decoded_user = { email: "user@email.com", role: "user" };

// Create a context
export const StateContext = createContext();

const StateProvider = (props) => {
	const [user, setUser] = useState({ ...decoded_user });
	return (
		<StateContext.Provider value={user}>{props.children}</StateContext.Provider>
	);
};

export default StateProvider;
