import React from "react";

import { NavLink } from "react-router-dom";

import { Menu } from "antd";
import { AppstoreOutlined, ShoppingOutlined } from "@ant-design/icons";

let url = window.location.pathname;

const GuestMenu = (props) => {
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
		</Menu>
	);
};

export default GuestMenu;
