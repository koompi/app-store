import React, { useState } from "react";
import { Switch, Route, NavLink } from "react-router-dom";
import { Layout, Menu, Breadcrumb, Row, Input, Affix, Col } from "antd";
import {
	UserOutlined,
	SettingOutlined,
	CloudSyncOutlined,
	DownloadOutlined,
	AppstoreOutlined,
	SearchOutlined,
} from "@ant-design/icons";

// CSS
import "./App.css";
// ASSETS
import KoompiLogoBlack from "./assets/svgs/koompi-logo-black.svg";
import KoompiIconBlack from "./assets/svgs/koompi-icon-black.svg";
import SearchComponent from "./components/search";
import Home from "./components/home";
import Categories from "./components/categories";
import Installed from "./components/installed";
import Update from "./components/update";
import Settings from "./components/settings";
import AppDetail from "./components/app_detail";
// VARS
const { Header, Content, Footer, Sider } = Layout;
const { SubMenu } = Menu;

function App() {
	const [collapsed, setCollapsed] = useState(false);

	const onCollapse = () => {
		setCollapsed(!collapsed);
	};
	let url = window.location.pathname;

	return (
		<div className="App">
			<Layout style={{ minHeight: "100vh" }}>
				<Sider
					width={200}
					className="site-layout-background site-sider"
					style={{ position: "fixed", minHeight: "100vh" }}
				>
					<br />
					<center>
						<img
							className="koompi-logo"
							src={collapsed ? KoompiIconBlack : KoompiLogoBlack}
							alt="logo"
						/>
					</center>
					<br />
					<Menu
						mode="inline"
						defaultSelectedKeys={[url]}
						style={{ height: "100%", borderRight: 0, zIndex: 999 }}
					>
						<Menu.Item key="/">
							<NavLink
								exact={true}
								to="/"
								style={{ color: "#333" }}
								activeStyle={{ color: "#1890ff" }}
							>
								<UserOutlined style={{ fontSize: "22px" }} />
								Home
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
								Installed
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
								Update
							</NavLink>
						</Menu.Item>
						<Menu.Item key="/settings">
							<NavLink
								exact={true}
								to="/settings"
								style={{ color: "#333" }}
								activeStyle={{ color: "#1890ff" }}
							>
								<SettingOutlined style={{ fontSize: "22px" }} />
								Settings
							</NavLink>
						</Menu.Item>
					</Menu>
				</Sider>
				<Layout
					className={collapsed ? "site-layout expand" : "site-layout shrink"}
				>
					<Content style={{ padding: "20px", overflow: "hidden", zIndex: 1 }}>
						<Switch>
							<Route exact={true} path="/" component={Home} />
							<Route exact={true} path="/apps/:name" component={AppDetail} />
							<Route exact={true} path="/categories" component={Categories} />
							<Route exact={true} path="/installed" component={Installed} />
							<Route exact={true} path="/updates" component={Update} />
							<Route exact={true} path="/settings" component={Settings} />
						</Switch>
					</Content>
					<Footer></Footer>
				</Layout>
			</Layout>
		</div>
	);
}

export default App;
