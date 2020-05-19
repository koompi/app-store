import React from "react";

import { Row, Col, List, Space, Typography } from "antd";
import { MessageOutlined, LikeOutlined, StarOutlined } from "@ant-design/icons";

const { Title } = Typography;

const listData = [];
for (let i = 0; i < 23; i++) {
	listData.push({
		href: "https://ant.design",
		title: `ant design part ${i}`,
		avatar:
			"https://zos.alipayobjects.com/rmsportal/ODTLcjxAfvqbxHnVXCYX.png",
		description:
			"Ant Design, a design language for background applications, is refined by Ant UED Team.",
		content:
			"We supply a series of design principles, practical patterns and high quality design resources (Sketch and Axure), to help people create their product prototypes beautifully and efficiently.",
	});
}

const IconText = ({ icon, text }) => (
	<Space>
		{React.createElement(icon)}
		{text}
	</Space>
);

const Organizations = () => {
	return (
		<Row style={{ padding: "10px 20%" }}>
			<Col span={24}>
				<Title>Organizations</Title>
			</Col>
			<Col span={24} style={{ backgroundColor: "#fff", padding: "20px" }}>
				<List
					itemLayout="vertical"
					size="large"
					pagination={{
						onChange: (page) => {
							console.log(page);
						},
						pageSize: 10,
					}}
					dataSource={listData}
					footer={
						<div>
							<b>ant design</b> footer part
						</div>
					}
					renderItem={(item) => (
						<List.Item
							key={item.title}
							actions={[
								<IconText
									icon={StarOutlined}
									text="156"
									key="list-vertical-star-o"
								/>,
								<IconText
									icon={LikeOutlined}
									text="156"
									key="list-vertical-like-o"
								/>,
								<IconText
									icon={MessageOutlined}
									text="2"
									key="list-vertical-message"
								/>,
							]}
							extra={
								<img
									width={272}
									alt="logo"
									src="https://gw.alipayobjects.com/zos/rmsportal/mqaQswcyDLcXyDKnZfES.png"
								/>
							}
						>
							<List.Item.Meta
								title={<a href={item.href}>{item.title}</a>}
								description={item.description}
							/>
							{item.content}
						</List.Item>
					)}
				/>
			</Col>
		</Row>
	);
};

export default Organizations;
