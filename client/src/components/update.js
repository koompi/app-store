import React, { useContext } from "react";
import { AuthContext } from "../context/AuthContext";
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
	Empty,
} from "antd";
import {
	DownloadOutlined,
	CodeOutlined,
	PlusSquareFilled,
	TagOutlined,
} from "@ant-design/icons";
import { Link } from "react-router-dom";
import koompi_icon from "../assets/svgs/koompi-icon-black.svg";

const { Title, Text } = Typography;
const { TabPane } = Tabs;
const { Panel } = Collapse;
function callback(key) {
	console.log(key);
}

const data = [
	"Racing car sprays burning fuel into crowd.",
	"Japanese princess to wed commoner.",
	"Australian walks 100km after outback crash.",
	"Man charged over missing wedding girl.",
	"Los Angeles battles huge wildfires.",
];

const Update = (props) => {
	const { user } = useContext(AuthContext);

	return (
		<>
			<Row style={{ padding: "50px 20%" }}>
				<Col span={4}>
					<Avatar
						style={{ width: "100px", height: "100px" }}
						shape="square"
						src={koompi_icon}
					/>
				</Col>
				<Col span={20}>
					<Title style={{ fontSize: "28px" }}>KOOMPI OS</Title>
					<Text>Version 2.5.0</Text>
					<br />
					<Space size="small" style={{ margin: "10px 0" }}>
						<Avatar size="small" shape="square" src={koompi_icon} />
						<Text strong>KOOMPI CO., LTD.</Text> |{" "}
						<DownloadOutlined />
						<Text style={{ fontSize: "12px" }}>
							{" "}
							1M+ Downloads
						</Text>{" "}
						| <Rate allowHalf defaultValue={5} />
					</Space>
					<br />
					<Space style={{ margin: "10px 0" }}>
						<Button disabled={user.role === "GUEST" ? true : false}>
							Update
						</Button>

						<Button>
							<code>pi os-update</code>
							<CodeOutlined style={{ paddingLeft: "10px" }} />
						</Button>
						<Button>Download</Button>
					</Space>
					<br />
				</Col>

				<Col span={24} style={{ marginTop: "30px" }}>
					<Tabs defaultActiveKey={window.location.hash}>
						<TabPane
							className="alltabs"
							tab={
								<Link to="#update" style={{ color: "#333" }}>
									Update
								</Link>
							}
							key="#update"
						>
							<Empty
								image="https://gw.alipayobjects.com/zos/antfincdn/ZHrcdLPrvN/empty.svg"
								imageStyle={{
									height: 250,
								}}
								description={[
									<Title level={4}>
										No Updates Available.
									</Title>,
									<Text>
										You already have the latest version of
										KOOMPI OS.
									</Text>,
								]}
							/>
						</TabPane>

						<TabPane
							className="alltabs"
							tab={
								<Link
									to="#description"
									style={{ color: "#333" }}
								>
									Description
								</Link>
							}
							key="#description"
						>
							<Descriptions
								title="General Info"
								layout="horizontal"
								bordered
								column={1}
								size="small"
							>
								<Descriptions.Item label="Official Name:">
									KOOMPI OS
								</Descriptions.Item>
								<Descriptions.Item label="Short Name:">
									KOOMPI OS
								</Descriptions.Item>
								<Descriptions.Item label="Website">
									<Link to="https://koompi.org">
										https://koompi.org
									</Link>
								</Descriptions.Item>
								<Descriptions.Item label="Upstream" span={2}>
									<Link to="https://github.com/koompi/koompi-os">
										https://github.com/koompi/koompi-os
									</Link>
								</Descriptions.Item>
								<Descriptions.Item label="Licenses">
									GNU GPL version 3
								</Descriptions.Item>
								<Descriptions.Item label="Owner">
									<Link to="https://www.koompi.com">
										KOOMPI CO., LTD.
									</Link>
								</Descriptions.Item>
								<Descriptions.Item label="DEVELOPER">
									Brilliant PHAL
								</Descriptions.Item>
								<Descriptions.Item label="ISO Size">
									3.5 GB
								</Descriptions.Item>
								<Descriptions.Item label="Installed Size">
									8.1 GB
								</Descriptions.Item>
							</Descriptions>
							<br />
							<Collapse
								defaultActiveKey={["1"]}
								onChange={callback}
							>
								<Panel
									header={
										<Text strong>
											Software Specifications:{" "}
										</Text>
									}
									key="1"
								>
									<Descriptions
										title="Core System"
										layout="horizontal"
										bordered
										column={1}
										size="small"
									>
										<Descriptions.Item label="Kernel">
											Linux 5.6.13{" "}
											<Tag color="#87d068">stable</Tag>
										</Descriptions.Item>
										<Descriptions.Item label="Boot Loader">
											GNU GRUB 2:2.04-6
										</Descriptions.Item>
										<Descriptions.Item label="System Service Manager">
											systemd 245.5-2
										</Descriptions.Item>
										<Descriptions.Item label="Shell">
											GNU BASH 5.0.016-1
										</Descriptions.Item>
									</Descriptions>
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
									<Title level={4}>KOOMPI OS</Title>
									<Tag color="#108ee9">
										<TagOutlined /> 2.5.0
									</Tag>{" "}
									<Tag color="#87d068">latest</Tag>{" "}
									<Tag color="#87d068">stable</Tag>
									<br />
									<br />
									<Collapse
										defaultActiveKey={["1"]}
										onChange={callback}
									>
										<Panel
											header={
												<Text strong>
													RELEASE NOTES :
												</Text>
											}
											key="1"
										>
											<List
												// header=
												dataSource={data}
												renderItem={(item) => (
													<List.Item>
														<Typography.Text>
															<PlusSquareFilled
																style={{
																	color:
																		"#87d068",
																	fontSize:
																		"18px",
																}}
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
									<Title level={4}>KOOMPI OS</Title>
									<Tag color="#108ee9">
										<TagOutlined /> 2.3.1
									</Tag>{" "}
									<Tag color="#87d068">stable</Tag>
									<br />
									<br />
									<Collapse
										defaultActiveKey={["1"]}
										onChange={callback}
									>
										<Panel
											header={
												<Text strong>
													RELEASE NOTES :
												</Text>
											}
											key="1"
										>
											<List
												dataSource={data}
												renderItem={(item) => (
													<List.Item>
														<Typography.Text>
															<PlusSquareFilled
																style={{
																	color:
																		"#87d068",
																	fontSize:
																		"18px",
																}}
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
					</Tabs>
				</Col>
			</Row>
		</>
	);
};

export default Update;
