import React, { useState, useEffect } from "react";
import {
	Row,
	Col,
	Avatar,
	Typography,
	Space,
	Rate,
	Button,
	Tabs,
	Carousel,
} from "antd";
import { DownloadOutlined, CopyOutlined } from "@ant-design/icons";
import ReactMarkdown from "react-markdown";
import description from "./description.md";
const { Title, Text } = Typography;
const { TabPane } = Tabs;

function callback(key) {
	console.log(key);
}

const AppDetail = () => {
	const [mdText, setMdText] = useState("");
	useEffect(() => {
		fetch("https://raw.githubusercontent.com/microsoft/vscode/master/README.md")
			.then((response) => {
				if (response.ok) return response.text();
				else return Promise.reject("Didn't fetch text correctly");
			})
			.then((text) => {
				setMdText(text);
			})
			.catch((error) => console.error(error));
	});
	return (
		<>
			<Row style={{ padding: "50px 20%" }}>
				<Col span={4}>
					<Avatar
						style={{ width: "100px", height: "100px" }}
						shape="square"
						src="https://upload.wikimedia.org/wikipedia/commons/2/2d/Visual_Studio_Code_1.18_icon.svg"
					/>
				</Col>
				<Col span={20}>
					<Title style={{ fontSize: "28px" }}>Visual Studio Code</Title>
					<Text>
						Code editing. Redefined. Free. Built on open source. Runs
						everywhere.
					</Text>
					<br />
					<Space size="small" style={{ margin: "10px 0" }}>
						<Avatar
							size="small"
							shape="square"
							src="https://upload.wikimedia.org/wikipedia/commons/4/44/Microsoft_logo.svg"
						/>
						<Text strong>Microsoft</Text> | <DownloadOutlined />
						<Text style={{ fontSize: "12px" }}> 4M+ Downloads</Text> |{" "}
						<Rate allowHalf defaultValue={4.5} />
					</Space>
					<br />
					<Space style={{ margin: "10px 0" }}>
						<Button>Install</Button>
						<Button>Download</Button>
						<Button>
							<code>pi install vscode</code>
							<CopyOutlined style={{ paddingLeft: "10px" }} />
						</Button>
					</Space>
					<br />
				</Col>

				<Col span={24} style={{ marginTop: "30px" }}>
					<Tabs>
						<TabPane className="alltabs" tab="Description" key="1">
							<ReactMarkdown
								className="mde"
								escapeHtml={false}
								source={mdText}
							/>
						</TabPane>
						<TabPane className="alltabs" tab="Releases" key="2">
							<ReactMarkdown
								className="mde"
								escapeHtml={false}
								source={mdText}
							/>
						</TabPane>
						<TabPane className="alltabs" tab="Reviews" key="3">
							<ReactMarkdown
								className="mde"
								escapeHtml={false}
								source={mdText}
							/>
						</TabPane>
					</Tabs>
				</Col>
			</Row>
		</>
	);
};

export default AppDetail;
