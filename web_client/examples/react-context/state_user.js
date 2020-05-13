import React, { useContext } from "react";
import { StateContext } from "./state";

const StateUser = () => {
	const user = useContext(StateContext);
	return <div>{user.email}</div>;
};

export default StateUser;
