import React, { Fragment } from "react";
import { Redirect } from "react-router-dom";

const token = window.localStorage.getItem("token");

const PrivateRoute = (props) => {
	if (!token) {
		return "";
	} else {
		return <Fragment>{props.children}</Fragment>;
	}
};

export default PrivateRoute;
