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
	Tag,
	List,
	Collapse,
	Divider,
	Descriptions,
	Comment,
} from "antd";
import {
	DownloadOutlined,
	CodeOutlined,
	PlusSquareFilled,
	TagOutlined,
} from "@ant-design/icons";
import ReactMarkdown from "react-markdown";
import { Link } from "react-router-dom";

const { Title, Text } = Typography;
const { TabPane } = Tabs;
const { Panel } = Collapse;
function callback(key) {
	console.log(key);
}
const ExampleComment = ({ children }) => (
	<Comment
		actions={[<span key="comment-nested-reply-to">Reply to</span>]}
		author={<a href="/">Han Solo</a>}
		avatar={
			<Avatar
				src="https://zos.alipayobjects.com/rmsportal/ODTLcjxAfvqbxHnVXCYX.png"
				alt="Han Solo"
			/>
		}
		content={
			<p>
				We supply a series of design principles, practical patterns and high
				quality design resources (Sketch and Axure).
			</p>
		}
	>
		{children}
	</Comment>
);
const data = [
	"Racing car sprays burning fuel into crowd.",
	"Japanese princess to wed commoner.",
	"Australian walks 100km after outback crash.",
	"Man charged over missing wedding girl.",
	"Los Angeles battles huge wildfires.",
];
const deps = [
	"electron7",
	"libsecret",
	"libx11",
	"libxkbfile",
	"ripgrep",
	"bash-completion (optional) - Bash completions",
	"x11-ssh-askpass (optional) - SSH authentication",
	"zsh-completions (optional) - ZSH completitons",
	"git (make)",
	"gulp (make)",
	"nodejs-lts-dubnium (make)",
	"npm (make)",
	"python2 (make)",
	"yarn (make)",
];
const AppDetail = (props) => {
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
							<CodeOutlined style={{ paddingLeft: "10px" }} />
						</Button>
					</Space>
					<br />
				</Col>

				<Col span={24} style={{ marginTop: "30px" }}>
					<Tabs defaultActiveKey={window.location.hash}>
						<TabPane
							className="alltabs"
							tab={
								<Link to="#description" style={{ color: "#333" }}>
									Description
								</Link>
							}
							key="1"
						>
							<Descriptions
								title="Package Info"
								layout="horizontal"
								bordered
								column={1}
								size="small"
							>
								<Descriptions.Item label="Official Name:">
									Visual Studio Code
								</Descriptions.Item>
								<Descriptions.Item label="Short Name:">
									vscode
								</Descriptions.Item>
								<Descriptions.Item label="Website">
									<Link to="https://code.visualstudio.com/">
										https://code.visualstudio.com/
									</Link>
								</Descriptions.Item>
								<Descriptions.Item label="Upstream" span={2}>
									<Link to="https://github.com/microsoft/vscode">
										https://github.com/microsoft/vscode
									</Link>
								</Descriptions.Item>
								<Descriptions.Item label="Licenses">
									MIT License
								</Descriptions.Item>
								<Descriptions.Item label="Owner">
									<Link to="www.microsoft.com">Microsoft</Link>
								</Descriptions.Item>
								<Descriptions.Item label="Maintainer">
									Brilliant PHAL
								</Descriptions.Item>
								<Descriptions.Item label="Package Size">
									9.1 MB
								</Descriptions.Item>
								<Descriptions.Item label="Installed Size">
									30.5 MB
								</Descriptions.Item>
							</Descriptions>
							<br />
							<Collapse defaultActiveKey={["1"]} onChange={callback}>
								<Panel header={<Text strong>DEPENDENCIES: </Text>} key="1">
									<List
										dataSource={deps}
										renderItem={(item) => (
											<List.Item>
												<Typography.Text>{item}</Typography.Text>
											</List.Item>
										)}
									/>
								</Panel>
							</Collapse>
							<br />
							<Collapse defaultActiveKey={["1"]} onChange={callback}>
								<Panel header={<Text strong>REQUIERD BY: </Text>} key="1">
									<List
										size="small"
										dataSource={deps}
										renderItem={(item) => (
											<List.Item>
												<Typography.Text>{item}</Typography.Text>
											</List.Item>
										)}
									/>
								</Panel>
							</Collapse>
						</TabPane>

						<TabPane
							className="alltabs"
							tab={
								<Link to="#releases" style={{ color: "#333" }}>
									Releases
								</Link>
							}
							key="#releases"
						>
							<Row>
								<Col span={24}>
									<Title level={4}>Visual Studio Code</Title>
									<Tag color="#108ee9">
										<TagOutlined /> 20200421
									</Tag>{" "}
									<Tag color="#87d068">latest</Tag>{" "}
									<Tag color="#87d068">stable</Tag>
									<br />
									<br />
									<Collapse defaultActiveKey={["1"]} onChange={callback}>
										<Panel header={<Text strong>RELEASE NOTES :</Text>} key="1">
											<List
												// header=
												dataSource={data}
												renderItem={(item) => (
													<List.Item>
														<Typography.Text>
															<PlusSquareFilled
																style={{ color: "#87d068", fontSize: "18px" }}
															/>{" "}
														</Typography.Text>{" "}
														{item}
													</List.Item>
												)}
											/>
										</Panel>
									</Collapse>
									<Divider />
								</Col>
								<Col span={24}>
									<Title level={4}>Visual Studio Code</Title>
									<Tag color="#108ee9">
										<TagOutlined /> 20200421
									</Tag>{" "}
									<Tag color="#87d068">latest</Tag>{" "}
									<Tag color="#87d068">stable</Tag>
									<br />
									<br />
									<Collapse defaultActiveKey={["1"]} onChange={callback}>
										<Panel header={<Text strong>RELEASE NOTES :</Text>} key="1">
											<List
												// header=
												dataSource={data}
												renderItem={(item) => (
													<List.Item>
														<Typography.Text>
															<PlusSquareFilled
																style={{ color: "#87d068", fontSize: "18px" }}
															/>{" "}
														</Typography.Text>{" "}
														{item}
													</List.Item>
												)}
											/>
										</Panel>
									</Collapse>
									<Divider />
								</Col>
							</Row>
						</TabPane>

						<TabPane
							className="alltabs"
							tab={
								<Link to="#reviews" style={{ color: "#333" }}>
									Reviews
								</Link>
							}
							key="#reviews"
						>
							<ExampleComment>
								<ExampleComment>
									<ExampleComment />
									<ExampleComment />
								</ExampleComment>
							</ExampleComment>
						</TabPane>
						<TabPane
							className="alltabs"
							tab={
								<Link to="#repository" style={{ color: "#333" }}>
									About Owner
								</Link>
							}
							key="#repository"
						>
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
