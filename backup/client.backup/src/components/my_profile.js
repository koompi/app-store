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
	List,
	Skeleton,
} from "antd";
import { HeartTwoTone, BugOutlined } from "@ant-design/icons";

import { Link } from "react-router-dom";

const { Title, Text } = Typography;
const { TabPane } = Tabs;

const data = [
	{
		name: "Visual Studio Code",
		icon:
			"https://upload.wikimedia.org/wikipedia/commons/2/2d/Visual_Studio_Code_1.18_icon.svg",
		rate: 4.5,
		owner: "Microsoft",
		owner_logo:
			"https://upload.wikimedia.org/wikipedia/commons/4/44/Microsoft_logo.svg",
		downloadCount: "10M",
		link: "/apps/visual-studio-code",
	},
	{
		name: "Atom",
		icon:
			"https://upload.wikimedia.org/wikipedia/commons/7/7b/Icon_Atom.svg",
		rate: 4.6,
		owner: "Github",
		owner_logo:
			"https://upload.wikimedia.org/wikipedia/commons/a/ae/Github-desktop-logo-symbol.svg",
		downloadCount: "8M",
		link: "/apps/atom",
	},
	{
		name: "Android Studio",
		icon:
			"https://upload.wikimedia.org/wikipedia/commons/3/34/Android_Studio_icon.svg",
		rate: 4.6,
		owner: "Google",
		owner_logo:
			"https://upload.wikimedia.org/wikipedia/commons/5/53/Google_%22G%22_Logo.svg",
		downloadCount: "8M",
		link: "/apps/android-studio",
	},
	{
		name: "Arduino IDE",
		icon:
			"https://upload.wikimedia.org/wikipedia/commons/8/87/Arduino_Logo.svg",
		rate: 4.6,
		owner: "Arduino",
		owner_logo:
			"https://upload.wikimedia.org/wikipedia/commons/8/87/Arduino_Logo.svg",
		downloadCount: "8M",
		link: "/apps/android-studio",
	},
	{
		name: "Brackets",
		icon:
			"https://upload.wikimedia.org/wikipedia/commons/4/4c/Brackets_Icon.svg",
		rate: 4.6,
		owner: "Brackets",
		owner_logo:
			"https://upload.wikimedia.org/wikipedia/commons/4/4c/Brackets_Icon.svg",
		downloadCount: "8M",
		link: "/apps/brackets",
	},
	{
		name: "IntelliJ IDEA",
		icon:
			"https://upload.wikimedia.org/wikipedia/commons/d/d5/IntelliJ_IDEA_Logo.svg",
		rate: 4.6,
		owner: "IntelliJ IDEA",
		owner_logo:
			"https://upload.wikimedia.org/wikipedia/commons/d/d5/IntelliJ_IDEA_Logo.svg",
		downloadCount: "8M",
		link: "/apps/intellij-idea",
	},
	{
		name: "Eclipse IDE",
		icon:
			"https://flathub.org/repo/appstream/x86_64/icons/128x128/org.eclipse.Committers.png",
		rate: 4.6,
		owner: "Eclipse Foundation",
		owner_logo:
			"https://flathub.org/repo/appstream/x86_64/icons/128x128/org.eclipse.Committers.png",
		downloadCount: "8M",
		link: "/apps/eclipse-ide",
	},
	{
		name: "Blender",
		icon:
			"https://flathub.org/repo/appstream/x86_64/icons/128x128/org.blender.Blender.png",
		rate: 4.6,
		owner: "Blender",
		owner_logo:
			"https://flathub.org/repo/appstream/x86_64/icons/128x128/org.blender.Blender.png",
		downloadCount: "8M",
		link: "/apps/blender",
	},
	{
		name: "Inkscape",
		icon:
			"https://flathub.org/repo/appstream/x86_64/icons/128x128/org.inkscape.Inkscape.png",
		rate: 4.6,
		owner: "Inkscape",
		owner_logo:
			"https://flathub.org/repo/appstream/x86_64/icons/128x128/org.inkscape.Inkscape.png",
		downloadCount: "8M",
		link: "/apps/inkscape",
	},
	{
		name: "GIMP",
		icon:
			"https://flathub.org/repo/appstream/x86_64/icons/128x128/org.gimp.GIMP.png",
		rate: 4.6,
		owner: "Gimp",
		owner_logo:
			"https://flathub.org/repo/appstream/x86_64/icons/128x128/org.gimp.GIMP.png",
		downloadCount: "8M",
		link: "/apps/gimp",
	},
];

const MyProfile = () => {
	const { user } = useContext(AuthContext);

	return (
		<>
			<Row style={{ padding: "0px 20%" }}>
				<Col span={4}>
					<Avatar
						// style={{ width: "100px", height: "100px" }}
						// shape="square"
						size={150}
						style={{
							boxShadow: "2px 2px 20px -3px rgba(0,0,0,0.3)",
						}}
						// icon={<UserOutlined />}
						src="https://scontent.fpnh10-1.fna.fbcdn.net/v/t1.0-9/s960x960/74643680_1263736627132541_7418475720780808192_o.jpg?_nc_cat=102&_nc_sid=85a577&_nc_eui2=AeHqXnzPdv1tlVt4Z00FEQmOZoPlq9lPcN9mg-Wr2U9w37m0M5paQIg-obRfVEu2L2Q2a4lBKoVwf0-Yb7CAI10x&_nc_ohc=gkBrapA1PsAAX_0rdL8&_nc_ht=scontent.fpnh10-1.fna&_nc_tp=7&oh=ee55eb957e5bbfac8b89e372c0c0b558&oe=5EEA0A03"
					/>
				</Col>
				<Col span={20}>
					<Title style={{ fontSize: "28px" }}>SAN Vuthy</Title>
					<Text>
						I enjoy programming and making things for the web.
					</Text>
					<br />
					<Space size="small" style={{ margin: "10px 0" }}>
						<Avatar
							size="small"
							shape="square"
							src="http://localhost:3001/static/media/koompi-icon-black.c816c8da.svg"
						/>
						<Text strong>KOOMPI</Text> |{" "}
						<HeartTwoTone twoToneColor="#eb2f96" />
						<Text style={{ fontSize: "12px" }}>
							{" "}
							4M+ Followers
						</Text>{" "}
						| <Rate allowHalf defaultValue={4.5} />
					</Space>
					<br />
					<Space style={{ margin: "10px 0" }}>
						<Button disabled={user.role === "GUEST" ? true : false}>
							Follow
						</Button>
						<Button>Sponsor</Button>
						<Button>
							<code>Report</code>
							<BugOutlined style={{ paddingLeft: "10px" }} />
						</Button>
					</Space>
					<br />
				</Col>
				<Col span={24} style={{ marginTop: "10px" }}>
					<Tabs defaultActiveKey={window.location.hash}>
						<TabPane
							className="alltabs"
							tab={
								<Link to="#personal" style={{ color: "#333" }}>
									My Store
								</Link>
							}
							key="#my_store"
						>
							<List
								className="demo-loadmore-list"
								itemLayout="horizontal"
								dataSource={data}
								renderItem={(item) => (
									<List.Item
										actions={[
											<Link key={item.link}>
												<Button type="primary">
													Update
												</Button>
											</Link>,
										]}
									>
										<Skeleton
											avatar
											title={false}
											loading={item.loading}
											active
										>
											<List.Item.Meta
												avatar={
													<Avatar
														src={item.icon}
														shape="square"
														size="large"
													/>
												}
												title={
													<a href="https://ant.design">
														{item.name}
													</a>
												}
												description={item.description}
											/>
										</Skeleton>
									</List.Item>
								)}
							/>
						</TabPane>

						<TabPane
							className="alltabs"
							tab={
								<Link
									to="#organizations"
									style={{ color: "#333" }}
								>
									My Packages
								</Link>
							}
							key="#my_packages"
						>
							<List
								className="demo-loadmore-list"
								itemLayout="horizontal"
								dataSource={data}
								renderItem={(item) => (
									<List.Item
										actions={[
											<Link key={item.link}>
												<Button type="primary" danger>
													Delete
												</Button>
											</Link>,
										]}
									>
										<Skeleton
											avatar
											title={false}
											loading={item.loading}
											active
										>
											<List.Item.Meta
												avatar={
													<Avatar
														src={item.icon}
														shape="square"
														size="large"
													/>
												}
												title={
													<a href="https://ant.design">
														{item.name}
													</a>
												}
												description={item.description}
											/>
										</Skeleton>
									</List.Item>
								)}
							/>
						</TabPane>

						<TabPane
							className="alltabs"
							tab={
								<Link to="#assets" style={{ color: "#333" }}>
									My Files
								</Link>
							}
							key="#my_files"
						>
							<List
								className="demo-loadmore-list"
								itemLayout="horizontal"
								dataSource={data}
								renderItem={(item) => (
									<List.Item
										actions={[
											<Link key={item.link}>
												<Button type="primary">
													Update
												</Button>
											</Link>,
										]}
									>
										<Skeleton
											avatar
											title={false}
											loading={item.loading}
											active
										>
											<List.Item.Meta
												avatar={
													<Avatar
														src={item.icon}
														shape="square"
														size="large"
													/>
												}
												title={
													<a href="https://ant.design">
														{item.name}
													</a>
												}
												description={item.description}
											/>
										</Skeleton>
									</List.Item>
								)}
							/>
						</TabPane>
					</Tabs>
				</Col>
			</Row>
		</>
	);
};

export default MyProfile;
