import React, { useState } from "react";
import { Switch, Route, NavLink } from "react-router-dom";
import {
	Layout,
	Menu,
	Button,
	Modal,
	Form,
	Input,
	Checkbox,
	Radio,
} from "antd";
import {
	SettingOutlined,
	CloudSyncOutlined,
	DownloadOutlined,
	AppstoreOutlined,
	HomeOutlined,
} from "@ant-design/icons";

// CSS
import "./assets/css/App.css";
// ASSETS
import KoompiLogoBlack from "./assets/svgs/koompi-logo-black.svg";
import KoompiIconBlack from "./assets/svgs/koompi-icon-black.svg";
import Home from "./components/home";
import Categories from "./components/categories";
import Installed from "./components/installed";
import Update from "./components/update";
import Settings from "./components/settings/index";
import AppDetail from "./components/app_detail";
import PrivateRoute from "./utils/auth";
import ProtectedComponent from "./utils/protected";
import TokenSigner from "./utils/authSetter";
import jwt from "jsonwebtoken";
import AppMenu from "./components/menu/menu";
import AuthContextProvider from "./context/AuthContext";
// VARS

const { Content, Footer, Sider } = Layout;

const radioStyle = {
	display: "block",
	height: "30px",
	lineHeight: "30px",
};

function App() {
	const [collapsed] = useState(false);
	const [authModal, setAuthMOdal] = useState(false);
	const [auth, setAuth] = useState(0);

	let url = window.location.pathname;
	const toggleAuthModal = (status) => {
		setAuthMOdal(status);
	};

	const onChange = (e) => {
		TokenSigner(e.target.value);
	};

	return (
		<div className="App">
			<AuthContextProvider>
				<AppMenu />
			</AuthContextProvider>
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
							<PrivateRoute>
								<Route exact={true} path="/installed" component={Installed} />
								<Route exact={true} path="/updates" component={Update} />
								<Route exact={true} path="/settings" component={Settings} />
							</PrivateRoute>
						</Switch>
					</Content>
					<Footer></Footer>
				</Layout>
			</Layout>
			<div
				style={{
					position: "fixed",
					top: 0,
					right: 0,
					padding: "10px",
					zIndex: 1000,
				}}
			>
				<Button type="primary" onClick={() => toggleAuthModal(true)}>
					Authenticator
				</Button>
			</div>
			<Modal
				title="Authentication Simulation"
				centered
				visible={authModal}
				onOk={() => toggleAuthModal(false)}
				onCancel={() => toggleAuthModal(false)}
			>
				<Radio.Group onChange={(e) => onChange(e)} value={auth}>
					<Radio style={radioStyle} value={null}>
						Not logged in
					</Radio>
					<Radio style={radioStyle} value="user">
						Logged in as user
					</Radio>
					<Radio style={radioStyle} value="maintainer">
						Logged in as maintainer
					</Radio>
					<Radio style={radioStyle} value="admin">
						Logged in as koompi admin
					</Radio>
				</Radio.Group>
			</Modal>
		</div>
	);
}

export default App;
