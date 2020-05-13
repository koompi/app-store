import React from "react";

const token = window.localStorage.getItem("token");

const ProtectedComponent = (props) => {
	if (!token) {
		return "";
	} else {
		return <>{props.children}</>;
	}
};

export default ProtectedComponent;
