import React from "react";
import { Row, Col, Form, Input, Button, Typography } from "antd";

const { Title } = Typography;

const layout = {
	labelCol: { span: 4 },
	wrapperCol: { span: 19 },
};

const Profile = () => {
	const onFinish = (values) => {
		console.log(values);
	};

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
					<Form
						{...layout}
						name="nest-messages"
						onFinish={onFinish}
						size="large"
					>
						<Title level={3}>Public profile</Title>
						<Form.Item name={["user", "name"]} label="Public Name">
							<Input />
						</Form.Item>
						<Form.Item
							name={["user", "email"]}
							label="Public Email"
							rules={[{ type: "email" }]}
							extra="By default your email is set to private for your privacy concerns. If you want to show it to public, you can put it here. We suggest you use different email from your login email."
						>
							<Input />
						</Form.Item>
						<Form.Item
							name={["user", "bio"]}
							label="Bio"
							extra="Say something about your self."
						>
							<Input.TextArea />
						</Form.Item>
						<Form.Item name={["user", "website"]} label="Website">
							<Input />
						</Form.Item>
						<Form.Item
							name={["user", "company"]}
							label="Company"
							extra="You can @mention your company’s KOOMPI organization to link it."
						>
							<Input />
						</Form.Item>
						<Form.Item
							name={["user", "location"]}
							label="Location"
							extra="We are open source and has no interest in your data, so we don't track your location."
						>
							<Input />
						</Form.Item>
						<Form.Item
							wrapperCol={{ ...layout.wrapperCol, offset: 21, span: 2 }}
						>
							<Button
								type="primary"
								htmlType="submit"
								style={{ width: "100%" }}
							>
								Update
							</Button>
						</Form.Item>
					</Form>
				</Col>
			</Row>
		</>
	);
};
export default Profile;
