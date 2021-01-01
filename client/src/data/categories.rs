pub struct Category {
    pub name: String,
    pub icon: String,
    pub description: String,
    pub link: String,
}
pub fn cats() -> Vec<Category> {
    vec![
        Category {
            name: "Business".to_string(),
            icon: "/icons/categories/business.png".to_string(),
            description: "Solutions for SME & Enterprise".to_string(),
            link: "/categories/business".to_string(),
        },
        Category {
            name: "Developer Tools".to_string(),
            icon: "/icons/categories/developer.png".to_string(),
            description: "IDE, SDK, ENV, & Languages".to_string(),
            link: "/categories/developer-tools".to_string(),
        },
        Category {
            name: "Education".to_string(),
            icon: "/icons/categories/education.png".to_string(),
            description: "Learn more online".to_string(),
            link: "/categories/educations".to_string(),
        },
        Category {
            name: "Entertainment".to_string(),
            icon: "/icons/categories/entertainment.png".to_string(),
            description: "Have more fun online".to_string(),
            link: "/categories/entertainment".to_string(),
        },
        Category {
            name: "Finance".to_string(),
            icon: "/icons/categories/finance.png".to_string(),
            description: "Manage your cashflow".to_string(),
            link: "/categories/finance".to_string(),
        },
        Category {
            name: "Games".to_string(),
            icon: "/icons/categories/games.png".to_string(),
            description: "Improve your strategies".to_string(),
            link: "/categories/games".to_string(),
        },
        Category {
            name: "Graphic & Design".to_string(),
            icon: "/icons/categories/graphic_and_design.png".to_string(),
            description: "Visual design concepts".to_string(),
            link: "/categories/graphic-and-design".to_string(),
        },
        Category {
            name: "Health & Fitness".to_string(),
            icon: "/icons/categories/health.png".to_string(),
            description: "Let's be healthy".to_string(),
            link: "/categories/health-and-fitness".to_string(),
        },
        Category {
            name: "Lifestyle".to_string(),
            icon: "/icons/categories/lifestyle.png".to_string(),
            description: "Live a great life".to_string(),
            link: "/categories/lifestyles".to_string(),
        },
        Category {
            name: "Medical".to_string(),
            icon: "/icons/categories/medical.png".to_string(),
            description: "Personal medical wiki".to_string(),
            link: "/categories/medical".to_string(),
        },
        Category {
            name: "Music".to_string(),
            icon: "/icons/categories/music.png".to_string(),
            description: "Chlling flows".to_string(),
            link: "/categories/music".to_string(),
        },
        Category {
            name: "News".to_string(),
            icon: "/icons/categories/news.png".to_string(),
            description: "What's happening".to_string(),
            link: "/categories/news".to_string(),
        },
        Category {
            name: "Photo & Video".to_string(),
            icon: "/icons/categories/photo_and_video.png".to_string(),
            description: "Edit, composite, and Publish".to_string(),
            link: "/categories/photo-and-video".to_string(),
        },
        Category {
            name: "Productivity".to_string(),
            icon: "/icons/categories/productivity.png".to_string(),
            description: "Tools for work and study".to_string(),
            link: "/categories/productivity".to_string(),
        },
        Category {
            name: "Reference".to_string(),
            icon: "/icons/categories/reference.png".to_string(),
            description: "Source of ideas".to_string(),
            link: "/categories/reference".to_string(),
        },
        Category {
            name: "Social Networking".to_string(),
            icon: "/icons/categories/social-network.png".to_string(),
            description: "Virtually connecting people".to_string(),
            link: "/categories/social-networking".to_string(),
        },
        Category {
            name: "Sports".to_string(),
            icon: "/icons/categories/sports.png".to_string(),
            description: "Who's the champion".to_string(),
            link: "/categories/sports".to_string(),
        },
        Category {
            name: "Travel".to_string(),
            icon: "/icons/categories/travel.png".to_string(),
            description: "Refresh your life".to_string(),
            link: "/categories/travel".to_string(),
        },
        Category {
            name: "Utilities".to_string(),
            icon: "/icons/categories/utilities.png".to_string(),
            description: "Do hard things easily".to_string(),
            link: "/categories/utilities".to_string(),
        },
        Category {
            name: "Weather".to_string(),
            icon: "/icons/categories/weather.png".to_string(),
            description: "Don't get wet".to_string(),
            link: "/categories/weather".to_string(),
        },
    ]
}
