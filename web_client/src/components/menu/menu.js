import React, { useContext } from "react";
import { AuthContext } from "../../context/AuthContext";
import { USER, DEVELOPER, ADMIN } from "../../reducers/auth_reducer";

import UserMenu from "./user";
import DeveloperMenu from "./developer";
import AdminMenu from "./admin";
import GuestMenu from "./guest";

const AppMenu = (props) => {
	const { user } = useContext(AuthContext);

	switch (user.role) {
		case USER:
			return <UserMenu />;
		case DEVELOPER:
			return <DeveloperMenu />;
		case ADMIN:
			return <AdminMenu />;
		default:
			return <GuestMenu />;
	}
};

export default AppMenu;
