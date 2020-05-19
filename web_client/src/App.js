import React, { useState } from "react";

import { Layout, Button } from "antd";

// CSS
import "./assets/css/App.css";
// ASSETS
import KoompiLogoBlack from "./assets/svgs/koompi-logo-black.svg";
import KoompiIconBlack from "./assets/svgs/koompi-icon-black.svg";
// COMPONENTS
import RouteManager from "./routes/routes";
import AppMenu from "./components/menu/menu";
import AuthContextProvider from "./context/AuthContext";
import AuthBox from "./utils/authBox";
// VARS

const { Content, Footer, Sider } = Layout;

function App() {
	const [collapsed] = useState(false);
	const [authModal, setAuthMOdal] = useState(false);

	const toggleAuthModal = (status) => {
		setAuthMOdal(status);
	};

	return (
		<AuthContextProvider>
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
								src={
									collapsed
										? KoompiIconBlack
										: KoompiLogoBlack
								}
								alt="logo"
							/>
						</center>
						<br />

						<AppMenu />
					</Sider>
					<Layout
						className={
							collapsed
								? "site-layout expand"
								: "site-layout shrink"
						}
					>
						<Content
							style={{
								padding: "20px",
								overflow: "hidden",
								zIndex: 1,
							}}
						>
							<RouteManager />
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
					<Button
						type="primary"
						onClick={() => toggleAuthModal(true)}
					>
						Authenticator
					</Button>
				</div>

				<AuthBox
					authModal={authModal}
					toggleAuthModal={toggleAuthModal}
				/>
			</div>
		</AuthContextProvider>
	);
}

export default App;
