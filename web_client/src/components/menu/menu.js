import React, { useContext } from "react";
import { AuthContext } from "../../context/AuthContext";

const AppMenu = (props) => {
	const { email } = useContext(AuthContext);
	return <div>conext is: {email}</div>;
};

export default AppMenu;
