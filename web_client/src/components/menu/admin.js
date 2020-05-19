import React from "react";
import { NavLink } from "react-router-dom";

import { Menu } from "antd";
import {
	SettingOutlined,
	LaptopOutlined,
	DownloadOutlined,
	AppstoreOutlined,
	ShoppingOutlined,
	FolderOutlined,
	TeamOutlined,
	BankOutlined,
} from "@ant-design/icons";

let url = window.location.pathname;

const AdminMenu = (props) => {
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
						<ShoppingOutlined style={{ fontSize: "22px" }} />
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
				<Menu.Item key="/my_applications">
					<NavLink
						exact={true}
						to="/my_applications"
						style={{ color: "#333" }}
						activeStyle={{
							color: "#1890ff",
						}}
					>
						<DownloadOutlined style={{ fontSize: "22px" }} />
						My Apps
					</NavLink>
				</Menu.Item>
				<Menu.Item key="/my_system">
					<NavLink
						exact={true}
						to="/my_system"
						style={{ color: "#333" }}
						activeStyle={{ color: "#1890ff" }}
					>
						<LaptopOutlined style={{ fontSize: "22px" }} />
						My System
					</NavLink>
				</Menu.Item>
			</Menu.ItemGroup>

			<Menu.ItemGroup key="/my_profile" title="Developer">
				<Menu.Item key="/my_profile">
					<NavLink
						exact={true}
						to="/my_profile"
						style={{ color: "#333" }}
						activeStyle={{
							color: "#1890ff",
						}}
					>
						<FolderOutlined style={{ fontSize: "22px" }} />
						My Profile
					</NavLink>
				</Menu.Item>
			</Menu.ItemGroup>

			<Menu.ItemGroup key="/my_admin" title="Administrator">
				<Menu.Item key="/users">
					<NavLink
						exact={true}
						to="/users"
						style={{ color: "#333" }}
						activeStyle={{
							color: "#1890ff",
						}}
					>
						<TeamOutlined style={{ fontSize: "22px" }} />
						Users
					</NavLink>
				</Menu.Item>
				<Menu.Item key="/organizations">
					<NavLink
						exact={true}
						to="/organizations"
						style={{ color: "#333" }}
						activeStyle={{
							color: "#1890ff",
						}}
					>
						<BankOutlined style={{ fontSize: "22px" }} />
						Organizations
					</NavLink>
				</Menu.Item>
			</Menu.ItemGroup>
			<Menu.ItemGroup key="/my_admin" title="Preferences">
				<Menu.Item key="/settings">
					<NavLink
						exact={true}
						to="/settings"
						style={{ color: "#333" }}
						activeStyle={{ color: "#1890ff" }}
					>
						<SettingOutlined style={{ fontSize: "22px" }} />{" "}
						Settings
					</NavLink>
				</Menu.Item>
			</Menu.ItemGroup>
		</Menu>
	);
};

export default AdminMenu;
