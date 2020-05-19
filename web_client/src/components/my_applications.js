import React from "react";
import { Row, Col, Avatar, Button, Tabs, List, Skeleton } from "antd";

import { Link } from "react-router-dom";

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

const MyApps = () => {
	return (
		<>
			<Row style={{ padding: "0px 20%" }}>
				<Col span={24} style={{ marginTop: "10px" }}>
					<Tabs defaultActiveKey={window.location.hash}>
						<TabPane
							className="alltabs"
							tab={
								<Link to="#updates" style={{ color: "#333" }}>
									Updates
								</Link>
							}
							key="#updates"
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
								<Link to="#installed" style={{ color: "#333" }}>
									Installed
								</Link>
							}
							key="#installed"
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
								<Link to="#history" style={{ color: "#333" }}>
									History
								</Link>
							}
							key="#history"
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
													Install
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

export default MyApps;
