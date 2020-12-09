import React, { useContext } from "react";
import { AuthContext } from "../context/AuthContext";
import { USER, DEVELOPER, ADMIN } from "../reducers/auth_reducer";

// COMPONENTS
// import UserRoutes from "./users";
// import DeveloperRoutes from "./developer";
import AdminRoutes from "./admins";
import GuestRoutes from "./guest";

const RouteManager = (props) => {
  const { user } = useContext(AuthContext);

  switch (user.role) {
    case USER:
      return <AdminRoutes />;
    case DEVELOPER:
      return <AdminRoutes />;
    case ADMIN:
      return <AdminRoutes />;
    default:
      return <GuestRoutes />;
  }
};

export default RouteManager;
