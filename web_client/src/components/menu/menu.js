import React, { useContext } from "react";
import { AuthContext } from "../../context/AuthContext";
import { GUEST, USER, MAINTAINER, ADMIN } from "../../reducers/auth_reducer";
import { NavLink } from "react-router-dom";

import { Menu } from "antd";
import {
	SettingOutlined,
	CloudSyncOutlined,
	DownloadOutlined,
	AppstoreOutlined,
	HomeOutlined,
} from "@ant-design/icons";

let url = window.location.pathname;

const PublicMenu = (props) => {
	return (
		<Menu
			mode="inline"
			defaultSelectedKeys={[url]}
			defaultOpenKeys={[url]}
			style={{ height: "100%", borderRight: 0, zIndex: 999 }}
		>
			<Menu.ItemGroup key="/" title="Explore">
				<Menu.Item key="/">
					<NavLink
						exact={true}
						to="/"
						style={{ color: "#333" }}
						activeStyle={{ color: "#1890ff" }}
					>
						<HomeOutlined style={{ fontSize: "22px" }} />
						Store
					</NavLink>
				</Menu.Item>
				<Menu.Item key="/categories">
					<NavLink
						exact={true}
						to="/categories"
						style={{ color: "#333" }}
						activeStyle={{ color: "#1890ff" }}
					>
						<AppstoreOutlined style={{ fontSize: "22px" }} />
						Categories
					</NavLink>
				</Menu.Item>
			</Menu.ItemGroup>
		</Menu>
	);
};

const UserMenu = (props) => {
	return (
		<Menu
			mode="inline"
			defaultSelectedKeys={[url]}
			defaultOpenKeys={[url]}
			style={{ height: "100%", borderRight: 0, zIndex: 999 }}
		>
			<Menu.ItemGroup key="/" title="Explore">
				<Menu.Item key="/">
					<NavLink
						exact={true}
						to="/"
						style={{ color: "#333" }}
						activeStyle={{ color: "#1890ff" }}
					>
						<HomeOutlined style={{ fontSize: "22px" }} />
						Store
					</NavLink>
				</Menu.Item>
				<Menu.Item key="/categories">
					<NavLink
						exact={true}
						to="/categories"
						style={{ color: "#333" }}
						activeStyle={{ color: "#1890ff" }}
					>
						<AppstoreOutlined style={{ fontSize: "22px" }} />
						Categories
					</NavLink>
				</Menu.Item>
			</Menu.ItemGroup>

			<Menu.ItemGroup key="/manage_apps" title="Manage">
				<Menu.Item key="/installed">
					<NavLink
						exact={true}
						to="/installed"
						style={{ color: "#333" }}
						activeStyle={{
							color: "#1890ff",
						}}
					>
						<DownloadOutlined style={{ fontSize: "22px" }} />
						My Applications
					</NavLink>
				</Menu.Item>
				<Menu.Item key="/updates">
					<NavLink
						exact={true}
						to="/updates"
						style={{ color: "#333" }}
						activeStyle={{ color: "#1890ff" }}
					>
						<CloudSyncOutlined style={{ fontSize: "22px" }} />
						Operating System
					</NavLink>
				</Menu.Item>
			</Menu.ItemGroup>

			<Menu.Item key="/settings">
				<NavLink
					exact={true}
					to="/settings"
					style={{ color: "#333" }}
					activeStyle={{ color: "#1890ff" }}
				>
					<SettingOutlined style={{ fontSize: "22px" }} /> Settings
				</NavLink>
			</Menu.Item>
		</Menu>
	);
};

const AppMenu = (props) => {
	const { user } = useContext(AuthContext);
	console.log("role is:", user.role);
	switch (user.role) {
		case USER:
			return <UserMenu />;
		case MAINTAINER:
			return <UserMenu />;
		case ADMIN:
			return <UserMenu />;
		default:
			return <PublicMenu />;
	}
};

export default AppMenu;
