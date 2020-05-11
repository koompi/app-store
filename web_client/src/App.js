import React, { useState } from "react";
import { Switch, Route, NavLink } from "react-router-dom";
import { Layout, Menu } from "antd";
import {
	SettingOutlined,
	CloudSyncOutlined,
	DownloadOutlined,
	AppstoreOutlined,
	HomeOutlined,
} from "@ant-design/icons";

// CSS
import "./App.css";
// ASSETS
import KoompiLogoBlack from "./assets/svgs/koompi-logo-black.svg";
import KoompiIconBlack from "./assets/svgs/koompi-icon-black.svg";
import Home from "./components/home";
import Categories from "./components/categories";
import Installed from "./components/installed";
import Update from "./components/update";
import Settings from "./components/settings/index";
import AppDetail from "./components/app_detail";

// VARS
const { Content, Footer, Sider } = Layout;

function App() {
	const [collapsed] = useState(false);

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
