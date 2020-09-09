import React, { useContext } from "react";
import { AuthContext } from "../../context/AuthContext";
import { useQuery, gql, useMutation } from "@apollo/client";
import {
	Row,
	Col,
	Form,
	Input,
	Button,
	Typography,
	Divider,
	message,
} from "antd";

const ProfileQuery = gql`
	query profilesByOwnerId($ownerId: String!) {
		profilesByOwnerId(ownerId: $ownerId) {
			id
			name
			bioDesc
			address
			avatar
			company
			website
		}
	}
`;

const UpdateProfileMutation = gql`
	mutation updateProfile(
		$id: String!
		$name: String!
		$bioDesc: String!
		$address: String!
		$avatar: String!
		$company: String!
		$website: String!
		$ownerId: String!
	) {
		updateProfile(
			id: $id
			name: $name
			bioDesc: $bioDesc
			address: $address
			avatar: $avatar
			company: $company
			website: $website
			ownerId: $ownerId
		) {
			id
			name
			bioDesc
			address
			avatar
			company
			website
			ownerId
		}
	}
`;

const { Title } = Typography;

const layout = {
	labelCol: { span: 4 },
	wrapperCol: { span: 10 },
};

const Profile = () => {
	const { user } = useContext(AuthContext);
	const [updateProfile] = useMutation(UpdateProfileMutation);
	const { loading, error, data } = useQuery(ProfileQuery, {
		variables: { ownerId: user.id },
	});
	const onFinish = (values) => {
		// console.log(values);
		updateProfile({
			variables: { ...values, ownerId: user.id },
			// refetchQueries: () => [ProfileQuery],
		})
			.then(async (res) => {
				if (res) await message.success("Updated profile successfully");
			})
			.catch((e) => console.log(e));
	};
	if (loading) {
		return "Loading...";
	}
	if (error) {
		console.log(error);
	}
	if (data) {
		return data.profilesByOwnerId.map((pf) => (
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
							initialValues={{
								id: pf.id,
								email: user.email,
								name: pf.name,
								bioDesc: pf.bioDesc,
								avatar: pf.avatar,
								address: pf.address,
								website: pf.website,
								company: pf.company,
							}}
						>
							<Title level={3}>PUBLIC PROFILE</Title>
							<Divider />
							<Form.Item name="id" label="ID" hidden={true}>
								<Input />
							</Form.Item>
							<Form.Item name="avatar" label="AVATAR" hidden={true}>
								<Input />
							</Form.Item>
							<Form.Item name="name" label="NAME">
								<Input />
							</Form.Item>
							<Form.Item
								name="email"
								label="EMAIL"
								rules={[{ type: "email" }]}
								extra="By default your email is set to private for your privacy concerns. If you want to show it to public, you can put it here. We suggest you use different email from your login email."
							>
								<Input disabled={true} />
							</Form.Item>
							<Form.Item
								name="bioDesc"
								label="BIOGRAPHY"
								extra="Say something about your self."
							>
								<Input.TextArea />
							</Form.Item>
							<Form.Item name="website" label="WEBSITE">
								<Input />
							</Form.Item>
							<Form.Item
								name="company"
								label="COMPANY"
								extra="You can @mention your company’s KOOMPI organization to link it."
							>
								<Input />
							</Form.Item>
							<Form.Item
								name="address"
								label="LOCATION"
								extra="We are open source and has no interest in your data, so we don't track your location."
							>
								<Input />
							</Form.Item>
							<Form.Item
							// wrapperCol={{ ...layout.wrapperCol, offset: 21, span: 2 }}
							>
								<Button type="primary" htmlType="submit">
									UPDATE
								</Button>
							</Form.Item>
						</Form>
					</Col>
				</Row>
			</>
		));
	}
};
export default Profile;
