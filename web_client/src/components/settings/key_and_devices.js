import React from "react";
import { Row, Col, Typography, Button, Card, Divider } from "antd";
import { KeyOutlined, RestOutlined, EditOutlined } from "@ant-design/icons";
const { Title, Text } = Typography;
const KeysAndDevices = () => {
	return (
		<>
			<Row>
				<Col
					span={14}
					style={{
						minHeight: "80vh",
						backgroundColor: "#fff",
						paddingTop: "20px",
					}}
				>
					<Title level={3}>KEY & DEVICES</Title>
					<Divider />
					<Card
						size="small"
						style={{ width: "95%" }}
						title="koompi@koompios"
						extra={[
							<Button
								type="link"
								style={{ padding: "0px 10px" }}
								icon={<EditOutlined style={{ fontSize: "22px" }} />}
							/>,
							<Button
								type="link"
								danger
								style={{ padding: "0px 10px" }}
								icon={<RestOutlined style={{ fontSize: "22px" }} />}
							/>,
						]}
					>
						<Text strong style={{ fontSize: "18px" }}>
							<KeyOutlined /> GPG
						</Text>
						<p style={{ overflow: "hidden" }}>
							{" "}
							AAAAB3NzaC1yc2EAAAADAQABAAABgQCpI69/MeppzrGMV4jHeTHkEqdA+9iwIFGk3G3diM
						</p>
					</Card>
					<Button type="primary" size="large" style={{ margin: "10px 0px" }}>
						Add Key
					</Button>
				</Col>
			</Row>
		</>
	);
};

export default KeysAndDevices;
