import React, { useContext } from "react";
import { AuthContext } from "../context/AuthContext";
// import { GUEST, USER, DEVELOPER, ADMIN } from "../reducers/auth_reducer";
import {
  Form,
  Input,
  Button,
  Row,
  Col,
  // Space,
  Card,
  message,
  // Modal,
} from "antd";
import { useMutation, gql } from "@apollo/client";

const SignInMuation = gql`
  mutation signin($email: String!, $password: String!) {
    signin(email: $email, password: $password)
  }
`;

const layout = {
  labelCol: {
    span: 8,
  },
  wrapperCol: {
    span: 16,
  },
};
const tailLayout = {
  wrapperCol: {
    offset: 20,
    span: 2,
  },
};

const SignIn = () => {
  const [signin] = useMutation(SignInMuation);
  const { dispatch } = useContext(AuthContext);
  const [form] = Form.useForm();

  const onFinish = (values) => {
    signin({
      variables: { ...values },
    })
      .then(async (res) => {
        window.localStorage.setItem("token", res.data.signin.toString());
        await message.success(
          "Signup successfully. Directing to login shortly..."
        );
        form.resetFields();

        //   console.log(window.localStorage.getItem("token"));
        dispatch({ type: "signin", target: "auth" });
      })
      .catch((err) => console.log(err));
  };

  return (
    <Row align="middle" justify="space-around" style={{ height: "90vh" }}>
      <Col>
        <Card title="SIGN IN" style={{ width: 500 }}>
          <Form
            {...layout}
            name="basic"
            form={form}
            initialValues={{
              remember: true,
            }}
            onFinish={onFinish}
          >
            <Form.Item
              label="Email"
              name="email"
              rules={[
                {
                  required: true,
                  message: "Please input your email!",
                },
              ]}
            >
              <Input />
            </Form.Item>

            <Form.Item
              label="Password"
              name="password"
              rules={[
                {
                  required: true,
                  message: "Please input your password!",
                },
              ]}
            >
              <Input.Password />
            </Form.Item>

            <Form.Item {...tailLayout}>
              <Button type="primary" htmlType="submit">
                Submit
              </Button>
            </Form.Item>
          </Form>
        </Card>
      </Col>
    </Row>
  );
};

export default SignIn;

// export const AuthBox = (props) => {

// 	const { authModal, toggleAuthModal } = props;
// 	useEffect(() => console.log(user));
// 	return (
// 		<Modal
// 			title="Authentication Simulation"
// 			centered
// 			visible={authModal}
// 			onOk={() => toggleAuthModal(false)}
// 			onCancel={() => toggleAuthModal(false)}
// 		>
// 			<p>
// 				<Button onClick={() => dispatch({ type: GUEST })}>{GUEST}</Button>
// 			</p>
// 			<p>
// 				<Button onClick={() => dispatch({ type: USER })}>{USER}</Button>
// 			</p>
// 			<p>
// 				<Button onClick={() => dispatch({ type: DEVELOPER })}>
// 					{DEVELOPER}
// 				</Button>
// 			</p>
// 			<p>
// 				<Button onClick={() => dispatch({ type: ADMIN })}>{ADMIN}</Button>
// 			</p>
// 		</Modal>
// 	);
// };
