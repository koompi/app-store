import React from "react";
import { Switch, Route, Redirect } from "react-router-dom";

// Components
import Home from "../components/home";
import Categories from "../components/categories";
import MyApps from "../components/my_applications";
import Update from "../components/update";
import Settings from "../components/settings/index";
import AppDetail from "../components/app_detail";

const UserRoutes = (props) => {
	return (
		<Switch>
			<Route exact={true} path="/" component={Home} />
			<Route exact={true} path="/apps/:name" component={AppDetail} />
			<Route exact={true} path="/categories" component={Categories} />
			<Route exact={true} path="/my_applications" component={MyApps} />
			<Route exact={true} path="/my_system" component={Update} />
			<Route exact={true} path="/settings" component={Settings} />
			<Route render={() => <Redirect to="/" />} />
		</Switch>
	);
};

export default UserRoutes;
