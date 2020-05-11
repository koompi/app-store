import React from "react";
import { Row, Col, Input, Button, Typography, Card, Checkbox } from "antd";
import { RestOutlined, PlusOutlined } from "@ant-design/icons";

const { Title } = Typography;

const Email = () => {
	return (
		<>
			<Row>
				<Col
					span={24}
					style={{
						minHeight: "80vh",
						backgroundColor: "#fff",
						paddingTop: "20px",
					}}
				>
					<Title level={3}>Emails</Title>
					<Card
						size="small"
						title="phalbrilliant@gmail.com"
						extra={
							<Button
								type="link"
								danger
								icon={<RestOutlined style={{ fontSize: "18px" }} />}
							/>
						}
						style={{ width: "95%" }}
					>
						<Title level={4}>Status</Title>
						<p style={{ marginBottom: "20px" }}>
							<Checkbox>Verified</Checkbox>
						</p>
						<p style={{ marginBottom: "20px" }}>
							<Checkbox>Primary</Checkbox>
						</p>
						<Title level={4}>Privacy</Title>
						<p style={{ marginBottom: "20px" }}>
							<Checkbox>Visible to public</Checkbox>
						</p>
						<Title level={4}>Feeds and Alert</Title>
						<p style={{ marginBottom: "20px" }}>
							<Checkbox>Security alert</Checkbox>
						</p>
						<p style={{ marginBottom: "20px" }}>
							<Checkbox>Apps updates</Checkbox>
						</p>
						<p style={{ marginBottom: "20px" }}>
							<Checkbox>Operating system updates</Checkbox>
						</p>
						<p style={{ marginBottom: "20px" }}>
							<Checkbox>Subcribe to KOOMPI's news and updates</Checkbox>
						</p>
					</Card>
					<Card
						size="small"
						title="Recovery Email"
						style={{ width: "95%", marginTop: "20px" }}
						extra={<Button type="primary" icon={<PlusOutlined />} />}
					>
						<Input style={{ width: "100%" }} />
					</Card>
				</Col>
			</Row>
		</>
	);
};
export default Email;
