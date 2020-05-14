import React, { useContext } from "react";
import { AuthContext } from "../context/AuthContext";
import { GUEST, USER, MAINTAINER, ADMIN } from "../reducers/auth_reducer";

// COMPONENTS
import ProtectedRoutes from "./protected";
import GuestRoutes from "./guest";

const RouteManager = (props) => {
	const { user } = useContext(AuthContext);

	switch (user.role) {
		case USER:
			return <ProtectedRoutes />;
		case MAINTAINER:
			return <ProtectedRoutes />;
		case ADMIN:
			return <ProtectedRoutes />;
		default:
			return <GuestRoutes />;
	}
};

export default RouteManager;
