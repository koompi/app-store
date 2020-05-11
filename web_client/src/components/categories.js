import React from "react";
import { Link } from "react-router-dom";
import SearchComponent from "./search";
import { Row, Col, Card, Avatar } from "antd";

const { Meta } = Card;

let cats = [
	{
		name: "Business",
		icon: "/images/app-categories/business.png",
		description: "Solutions for SME & Enterprise",
		link: "/categories/business",
	},
	{
		name: "Developer Tools",
		icon: "/images/app-categories/developer.png",
		description: "IDE, SDK, ENV, & Languages",
		link: "/categories/developer-tools",
	},
	{
		name: "Education",
		icon: "/images/app-categories/education.png",
		description: "Learn more online",
		link: "/categories/educations",
	},
	{
		name: "Entertainment",
		icon: "/images/app-categories/entertainment.png",
		description: "Have more fun online",
		link: "/categories/entertainment",
	},
	{
		name: "Finance",
		icon: "/images/app-categories/finance.png",
		description: "Manage your cashflow",
		link: "/categories/finance",
	},
	{
		name: "Games",
		icon: "/images/app-categories/games.png",
		description: "Improve your strategies",
		link: "/categories/games",
	},
	{
		name: "Graphic & Design",
		icon: "/images/app-categories/graphic_and_design.png",
		description: "Visual design concepts",
		link: "/categories/graphic-and-design",
	},
	{
		name: "Health & Fitness",
		icon: "/images/app-categories/health.png",
		description: "Let's be healthy",
		link: "/categories/health-and-fitness",
	},
	{
		name: "Lifestyle",
		icon: "/images/app-categories/lifestyle.png",
		description: "Live a great life",
		link: "/categories/lifestyles",
	},
	{
		name: "Medical",
		icon: "/images/app-categories/medical.png",
		description: "Personal medical wiki",
		link: "/categories/medical",
	},
	{
		name: "Music",
		icon: "/images/app-categories/music.png",
		description: "Chlling flows",
		link: "/categories/music",
	},
	{
		name: "News",
		icon: "/images/app-categories/news.png",
		description: "What's happening",
		link: "/categories/news",
	},
	{
		name: "Photo & Video",
		icon: "/images/app-categories/photo_and_video.png",
		description: "Edit, composite, and Publish",
		link: "/categories/photo-and-video",
	},
	{
		name: "Productivity",
		icon: "/images/app-categories/productivity.png",
		description: "Tools for work and study",
		link: "/categories/productivity",
	},
	{
		name: "Reference",
		icon: "/images/app-categories/reference.png",
		description: "Source of ideas",
		link: "/categories/reference",
	},
	{
		name: "Social Networking",
		icon: "/images/app-categories/social-network.png",
		description: "Virtually connecting people",
		link: "/categories/social-networking",
	},
	{
		name: "Sports",
		icon: "/images/app-categories/sports.png",
		description: "Who's the champion",
		link: "/categories/sports",
	},
	{
		name: "Travel",
		icon: "/images/app-categories/travel.png",
		description: "Refresh your life",
		link: "/categories/travel",
	},
	{
		name: "Utilities",
		icon: "/images/app-categories/utilities.png",
		description: "Do hard things easily",
		link: "/categories/utilities",
	},
	{
		name: "Weather",
		icon: "/images/app-categories/weather.png",
		description: "Don't get wet",
		link: "/categories/weather",
	},
];

const Categories = () => {
	return (
		<>
			<SearchComponent />
			<Row gutter={[20, 20]} style={{ padding: "0px 80px" }}>
				{cats.map((cat) => (
					<Col span={6}>
						<Link to={`${cat.link}`}>
							<Card hoverable>
								<Meta
									avatar={<Avatar size="large" shape="square" src={cat.icon} />}
									title={cat.name}
									description={cat.description}
								/>
							</Card>
						</Link>
					</Col>
				))}
			</Row>
		</>
	);
};

export default Categories;
