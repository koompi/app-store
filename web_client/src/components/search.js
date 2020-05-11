import React from "react";
import { Row, Col, Input } from "antd";
import { AudioOutlined } from "@ant-design/icons";

const { Search } = Input;

const suffix = (
	<AudioOutlined
		style={{
			fontSize: 16,
			color: "#1890ff",
		}}
	/>
);

const SearchComponent = () => {
	return (
		<Row>
			<Col span={8} offset={8} style={{ padding: "50px 0px" }}>
				<Search
					placeholder="input search text"
					enterButton="Search"
					size="large"
					suffix={suffix}
					onSearch={(value) => console.log(value)}
				/>
			</Col>
		</Row>
	);
};

export default SearchComponent;
