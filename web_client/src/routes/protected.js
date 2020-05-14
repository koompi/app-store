import React from "react";
import { Switch, Route, Redirect } from "react-router-dom";

// Components
import Home from "../components/home";
import Categories from "../components/categories";
import Installed from "../components/installed";
import Update from "../components/update";
import Settings from "../components/settings/index";
import AppDetail from "../components/app_detail";

const ProtectedRoutes = (props) => {
	return (
		<Switch>
			<Route exact={true} path="/" component={Home} />
			<Route exact={true} path="/apps/:name" component={AppDetail} />
			<Route exact={true} path="/categories" component={Categories} />
			<Route exact={true} path="/installed" component={Installed} />
			<Route exact={true} path="/updates" component={Update} />
			<Route exact={true} path="/settings" component={Settings} />
			<Route render={() => <Redirect to="/" />} />
		</Switch>
	);
};

export default ProtectedRoutes;
