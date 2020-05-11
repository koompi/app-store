import React from "react";
import { Row, Col, Form, Input, Button, Typography, Divider } from "antd";

const { Title } = Typography;

const layout = {
	labelCol: { span: 4 },
	wrapperCol: { span: 19 },
};

const Account = () => {
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
						<Title level={3}>Login information</Title>
						<Form.Item
							name={["user", "name"]}
							label="User Name"
							extra="The short name used for loggin in."
						>
							<Input />
						</Form.Item>
						<Form.Item
							name={["user", "email"]}
							label="Primary Email"
							rules={[{ type: "email" }]}
							extra="You must have at least one primary emails. If you want to remove an email, you must add a new one first and set it to primary email. Then you can the one."
						>
							<Input />
						</Form.Item>

						<Form.Item
							label="Password"
							extra="We recommend you change your periodically, so it is more secure."
						>
							<Button type="primary">Change password</Button>
						</Form.Item>
						<Divider />
						<Title level={3}>Export account data</Title>
						<Form.Item extra="The export process will take 24 hours to prepare, so you export now, you can download your data tomorrow.">
							<Button type="primary">Export Now</Button>
						</Form.Item>
						<Divider />
						<Title level={3}>Dangerous zone</Title>
						<Form.Item extra="After deactivating your account no one can see your account. Your account is still availble to in KOOMPI server, but in archive mode with encryption. After you proceed we will send a reactivation key for decrypting your data.">
							<Button type="danger" style={{ backgroundColor: "#d4b106" }}>
								Deactive Account
							</Button>
						</Form.Item>
						<Form.Item extra="This will permanently remove your account from KOOMPI platform. This action cannot be reverted. However to ensure security, This process will take 30 days to confirm. You confirm on the last day your account will be deleted for ever.">
							<Button type="danger">Delete Account</Button>
						</Form.Item>
					</Form>
				</Col>
			</Row>
		</>
	);
};
export default Account;
