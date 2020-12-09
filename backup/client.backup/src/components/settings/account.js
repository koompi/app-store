import React, { useState, useContext } from "react";
import { AuthContext } from "../../context/AuthContext";
import { useMutation, gql } from "@apollo/client";
import {
  Row,
  Col,
  Form,
  Input,
  Button,
  Typography,
  Divider,
  Modal,
  message,
} from "antd";

const { Title } = Typography;

const layout = {
  labelCol: { span: 4 },
  wrapperCol: { span: 18 },
};

const ChangePasswordMuation = gql`
  mutation changePassword(
    $email: String!
    $currentP: String!
    $newP: String!
    $confirmP: String!
  ) {
    changePassword(
      email: $email
      currentP: $currentP
      newP: $newP
      confirmP: $confirmP
    )
  }
`;

const PasswordBox = (props) => {
  const [changePassword] = useMutation(ChangePasswordMuation);
  const [form] = Form.useForm();
  const { user } = useContext(AuthContext);

  const { authModal, togglePasswordModal } = props;
  const onFinish = (values) => {
    changePassword({
      variables: { email: user.email, ...values },
    })
      .then(async (res) => {
        form.resetFields();
        togglePasswordModal(false);
        await message.success(`${res.data.changePassword}`);
      })
      .catch(async (err) => {
        console.log(err);
        await message.error(`${err}`);
        form.resetFields();
      });
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
        // initialValues={data}
      >
        <Form.Item name="currentP" label="Current Password">
          <Input.Password />
        </Form.Item>
        <Form.Item name="newP" label="New Password">
          <Input.Password />
        </Form.Item>
        <Form.Item
          name="confirmP"
          label="Confirm Password"
          dependencies={["newP"]}
          rules={[
            {
              required: true,
              message: "Please confirm your password!",
            },
            ({ getFieldValue }) => ({
              validator(rule, value) {
                if (!value || getFieldValue("newP") === value) {
                  return Promise.resolve();
                }
                return Promise.reject(
                  "The two passwords that you entered do not match!"
                );
              },
            }),
          ]}
        >
          <Input.Password />
        </Form.Item>
        <Form.Item>
          <Button type="primary" htmlType="submit">
            Submit
          </Button>
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
            <Title level={3}>LOGIN INFORMATION</Title>
            <Divider />

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
                Update password
              </Button>
            </Form.Item>

            <Form.Item
              label="Forget Password"
              extra={
                "We will send an email to your primary email to confirm reseting password."
              }
            >
              <Button type="primary">Reset password</Button>
            </Form.Item>

            <Divider />
            {/* 
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
			*/}
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
