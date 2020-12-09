import React from "react";
import { Form, Input, Button, Row, Col, Card, message } from "antd";
import { useMutation } from "@apollo/client";
import { SignUpMutation } from "../graphql/mutation";

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

const SignUp = () => {
  const [singup] = useMutation(SignUpMutation);
  const [form] = Form.useForm();

  const onFinish = (values) => {
    singup({
      variables: { ...values },
    })
      .then(async (res) => {
        await message.success(
          "Signup successfully. Directing to login shortly..."
        );
        await form.resetFields();
        window.location.replace("/signin");
      })
      .catch((err) => console.log(err));
  };

  return (
    <Row align="middle" justify="space-around" style={{ height: "90vh" }}>
      <Col>
        <Card title="SIGN UP" style={{ width: 500 }}>
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
              label="User Name"
              name="name"
              rules={[
                {
                  required: true,
                  message: "Please input your username!",
                },
              ]}
            >
              <Input />
            </Form.Item>

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

export default SignUp;
