import React from "react";
import { Link } from "react-router-dom";
import { Row, Col, Tabs } from "antd";
import {
	UserOutlined,
	SolutionOutlined,
	SafetyCertificateOutlined,
	MailOutlined,
	NotificationOutlined,
	KeyOutlined,
} from "@ant-design/icons";
import Account from "./account";
import Email from "./email";
import Profile from "./profile";
import Notification from "./notification";
import Security from "./security";
import KeysAndDevices from "./key_and_devices";
const { TabPane } = Tabs;
const Settings = () => {
	return (
		<>
			<Row style={{ padding: "50px 10%" }}>
				<Col
					span={24}
					style={{
						minHeight: "80vh",
						backgroundColor: "#fff",
						paddingTop: "20px",
					}}
				>
					<Tabs
						type="line"
						tabPosition="left"
						defaultActiveKey={window.location.hash}
					>
						<TabPane
							tab={
								<Link
									to="#profile"
									style={{
										display: "block",
										padding: "5px 24px",
										color: "#333",
										textAlign: "left",
									}}
								>
									<SolutionOutlined style={{ fontSize: "22px" }} /> Profile
								</Link>
							}
							key="#profile"
						>
							<Profile />
						</TabPane>
						<TabPane
							// className="alltabs"
							tab={
								<Link
									to="#account"
									style={{
										display: "block",
										padding: "5px 24px",
										color: "#333",
										textAlign: "left",
									}}
								>
									<UserOutlined style={{ fontSize: "22px" }} /> Account
								</Link>
							}
							key="#account"
						>
							<Account />
						</TabPane>
						<TabPane
							// className="alltabs"
							tab={
								<Link
									to="#email"
									style={{
										display: "block",
										padding: "5px 24px",
										color: "#333",
										textAlign: "left",
									}}
								>
									<MailOutlined style={{ fontSize: "22px" }} /> Email
								</Link>
							}
							key="#email"
						>
							<Email />
						</TabPane>
						<TabPane
							// className="alltabs"
							tab={
								<Link
									to="#notification"
									style={{
										display: "block",
										padding: "5px 24px",
										color: "#333",
										textAlign: "left",
									}}
								>
									<NotificationOutlined style={{ fontSize: "22px" }} />{" "}
									Notification
								</Link>
							}
							key="#notification"
						>
							<Notification />
						</TabPane>
						<TabPane
							// className="alltabs"
							tab={
								<Link
									to="#security"
									style={{
										display: "block",
										padding: "5px 24px",
										color: "#333",
										textAlign: "left",
									}}
								>
									<SafetyCertificateOutlined style={{ fontSize: "22px" }} />{" "}
									Security
								</Link>
							}
							key="#security"
						>
							<Security />
						</TabPane>
						<TabPane
							// className="alltabs"
							tab={
								<Link
									to="#keys_and_devices"
									style={{
										display: "block",
										padding: "5px 24px",
										color: "#333",
										textAlign: "left",
									}}
								>
									<KeyOutlined style={{ fontSize: "22px" }} /> Keys & Devices
								</Link>
							}
							key="#keys_and_devices"
						>
							<KeysAndDevices />
						</TabPane>
					</Tabs>
				</Col>
			</Row>
		</>
	);
};
export default Settings;
