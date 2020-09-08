import React, { useState, useContext } from "react";
import { AuthContext } from "../../context/AuthContext";
import {
	Row,
	Col,
	Form,
	Input,
	Button,
	Typography,
	Divider,
	Modal,
} from "antd";

const { Title } = Typography;

const layout = {
	labelCol: { span: 4 },
	wrapperCol: { span: 19 },
};

const PasswordBox = (props) => {
	const { user, dispatch } = useContext(AuthContext);
	const [data, setData] = useState({
		current: "",
		new_password: "",
		confirm: "",
	});
	const { authModal, togglePasswordModal } = props;
	const onFinish = (values) => {
		setData(values);
	};
	const layout = {
		labelCol: { span: 8 },
		wrapperCol: { span: 16 },
	};
	return (
		<Modal
			title="Change Password"
			centered
			visible={authModal}
			onOk={() => togglePasswordModal(false)}
			onCancel={() => togglePasswordModal(false)}
		>
			<Form
				{...layout}
				name="nest-messages"
				onFinish={onFinish}
				size="large"
				initialValues={data}
			>
				<Form.Item name="current" label="Current Password">
					<Input
						type="password"
						onChange={(e) => setData({ ...data, current: e.target.value })}
					/>
				</Form.Item>
				<Form.Item name="new_password" label="New Password">
					<Input
						type="password"
						onChange={(e) => setData({ ...data, new_password: e.target.value })}
					/>
				</Form.Item>
				<Form.Item
					name="confirm"
					label="Confirm Password"
					extra={
						data.confirm === data.new_password ? "" : "Password mismatched!"
					}
				>
					<Input
						type="password"
						onChange={(e) => setData({ ...data, confirm: e.target.value })}
					/>
				</Form.Item>
			</Form>
		</Modal>
	);
};

const Account = () => {
	const { user } = useContext(AuthContext);
	console.log("EEE: ", user.email);
	const onFinish = (values) => {
		console.log(values);
	};

	const [authModal, setAuthMOdal] = useState(false);

	const togglePasswordModal = (status) => {
		setAuthMOdal(status);
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
						initialValues={{ email: user.email, name: user.name }}
					>
						<Title level={3}>Login information</Title>
						<Form.Item
							name="name"
							label="User Name"
							extra={"The short name used for loggin in."}
						>
							<Input disabled={true} />
						</Form.Item>
						<Form.Item
							name="email"
							label="Primary Email"
							rules={[{ type: "email" }]}
							extra={
								"You must have at least one primary email. If you want to remove an email, you must add a new one first and set it to primary email. Then you can the remove one."
							}
						>
							<Input disabled={true} />
						</Form.Item>

						<Form.Item
							label="Password"
							extra={
								"We recommend you change your periodically, so it is more secure."
							}
						>
							<Button type="primary" onClick={() => togglePasswordModal(true)}>
								Change password
							</Button>
						</Form.Item>
						<Divider />
						<Title level={3}>Export account data</Title>
						<Form.Item
							extra={
								"The export process will take 24 hours to prepare, so you export now, you can download your data tomorrow."
							}
						>
							<Button type="primary">Export Now</Button>
						</Form.Item>
						<Divider />
						<Title level={3}>Dangerous zone</Title>
						<Form.Item
							extra={
								"After deactivating your account no one can see your account. Your account is still availble on KOOMPI server, but in archive mode with encryption. After you proceed we will send a reactivation key for decrypting your data."
							}
						>
							<Button type="danger" style={{ backgroundColor: "#d4b106" }}>
								Deactive Account
							</Button>
						</Form.Item>
						<Form.Item
							extra={
								"This will permanently remove your account from KOOMPI platform. This action cannot be reverted. However to ensure security, this process will take 30 days to confirm. If you confirm on the last day your account will be deleted forever."
							}
						>
							<Button type="danger">Delete Account</Button>
						</Form.Item>
					</Form>
				</Col>
			</Row>
			<PasswordBox
				authModal={authModal}
				togglePasswordModal={togglePasswordModal}
			/>
		</>
	);
};
export default Account;
