import React from "react";
import { Row, Col, Typography, Card, Checkbox } from "antd";

const { Title } = Typography;

const Security = () => {
	return (
		<>
			<Row>
				<Col
					span={24}
					style={{
						minHeight: "80vh",
						maxHeight: "auto",
						backgroundColor: "#fff",
						padding: "20px 0px",
					}}
				>
					<Title level={3}>Security</Title>
					<Card
						size="small"
						title="Multiple factor authentication"
						style={{ width: "95%" }}
					>
						<Title level={4}>OTP (One Time Password)</Title>
						<p style={{ marginBottom: "20px" }}>
							<Checkbox>Email one time password</Checkbox>
						</p>
						<p style={{ marginBottom: "20px" }}>
							<Checkbox>Notify OTP to connected devices</Checkbox>
						</p>
						<Title level={4}>RTA (Real Time Authenticator)</Title>
						<p style={{ marginBottom: "20px" }}>
							<Checkbox>Email one click confirm</Checkbox>
						</p>
						<p style={{ marginBottom: "20px" }}>
							<Checkbox>Notify one click confirm</Checkbox>
						</p>
						<Title level={4}>OAuth</Title>
						<p style={{ marginBottom: "20px" }}>
							<Checkbox>Google OAuth</Checkbox>
						</p>
						<p style={{ marginBottom: "20px" }}>
							<Checkbox>Github OAuth</Checkbox>
						</p>
					</Card>
					<Card
						size="small"
						title="Login Restriction"
						style={{ width: "95%", marginTop: "20px" }}
					>
						<p style={{ marginBottom: "20px" }}>
							<Checkbox>Use CAPCHA</Checkbox>
						</p>
						<p style={{ marginBottom: "20px" }}>
							<Checkbox>Limit active sessions</Checkbox>
						</p>
						<p style={{ marginBottom: "20px" }}>
							<Checkbox>Limit unsuccessful login tries</Checkbox>
						</p>
					</Card>
				</Col>
			</Row>
		</>
	);
};
export default Security;
