import React from "react";
import { Switch, Route, Redirect } from "react-router-dom";

// Components
import Home from "../components/home";
import Categories from "../components/categories";
import AppDetail from "../components/app_detail";
import SignUp from "../components/signup";
import SignIn from "../components/signin";

const GuestRoutes = () => {
	return (
		<Switch>
			<Route exact={true} path="/" component={Home} />
			<Route exact={true} path="/apps/:name" component={AppDetail} />
			<Route exact={true} path="/categories" component={Categories} />
			<Route exact={true} path="/signup" component={SignUp} />
			<Route exact={true} path="/signin" component={SignIn} />
			<Route render={() => <Redirect to="/" />} />
		</Switch>
	);
};

export default GuestRoutes;
