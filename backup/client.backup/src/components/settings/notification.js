import React from "react";
import { Row, Col, Typography, Checkbox, Divider } from "antd";

const { Title } = Typography;

const Notification = () => {
  return (
    <>
      <Row>
        <Col
          span={22}
          style={{
            minHeight: "80vh",
            backgroundColor: "#fff",
            paddingTop: "20px",
          }}
        >
          <Title level={3}>NOTIFICATION</Title>
          <Divider />
          <p style={{ marginBottom: "20px" }}>
            <Checkbox>New login</Checkbox>
          </p>
          <p style={{ marginBottom: "20px" }}>
            <Checkbox>Periodical Editor's choices</Checkbox>
          </p>
          <p style={{ marginBottom: "20px" }}>
            <Checkbox>Apps and operating system update</Checkbox>
          </p>
          <p style={{ marginBottom: "20px" }}>
            <Checkbox>News, tips and tricks for using KOOMPI OS</Checkbox>
          </p>

          <p style={{ marginBottom: "20px" }}>
            <Checkbox>Being mentioned in KOOMPI platform</Checkbox>
          </p>
        </Col>
      </Row>
    </>
  );
};
export default Notification;
