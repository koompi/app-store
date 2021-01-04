use crate::routes::{AppRoute, AppRouteAnchor};
use web_sys::console::log_1;
use yew::{prelude::*, services::console::ConsoleService, utils::window};
use yew_router::service::RouteService;
pub struct SideNav {
    props: Props,
    link: ComponentLink<Self>,
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub active_route: AppRoute,
}

impl Component for SideNav {
    type Message = AppRoute;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let w = window();
        let l = w.location();
        let p = l.pathname().unwrap();

        html! {
            <nav>
                <h1>{"Open Store"}</h1>
                <br />
                <h3>{"Explore"}</h3>
                <ul>
                    <li class={ if p == "/" { "active" } else { "" } }>
                        <AppRouteAnchor classes="home-logo-link" route={AppRoute::IndexPage}>
                            <img src="/icons/explore-black-18dp.svg" alt="" />
                            {"Featured"}
                        </AppRouteAnchor>
                    </li>
                    <li class={ if p == "/apps" { "active" } else { "" } }>
                        <AppRouteAnchor classes="home-logo-link" route={AppRoute::AppsPage}>
                            <img src="/icons/widgets-black-18dp.svg" alt="" />
                            {"Apps"}
                        </AppRouteAnchor>
                    </li>
                    <li class={ if p == "/categories" { "active" } else { "" } }>
                        <AppRouteAnchor classes="home-logo-link" route={AppRoute::CategoriesPage}>
                            <img src="/icons/category-black-18dp.svg" alt="" />
                            {"Categories"}
                        </AppRouteAnchor>
                    </li>
                </ul>
                <h3>{"Manage"}</h3>
                <ul>
                    <li class={ if p == "/updates" { "active" } else { "" } }>
                        <AppRouteAnchor classes="home-logo-link" route={AppRoute::UpdatesPage}>
                            <img src="/icons/update-black-18dp.svg" alt="" />
                            {"Updates"}
                        </AppRouteAnchor>
                    </li>
                    <li class={ if p == "/settings" { "active" } else { "" } }>
                        <AppRouteAnchor classes="home-logo-link" route={AppRoute::SettingsPage}>
                            <img src="/icons/settings-black-18dp.svg" alt="" />
                            {"Settings"}
                        </AppRouteAnchor>
                    </li>
                </ul>
                <h3>{"Develop"}</h3>
                <ul>
                    <li class={ if p == "/add" { "active" } else { "" } }>
                        <AppRouteAnchor classes="home-logo-link" route={AppRoute::AddPage}>
                            <img
                                src="/icons/cloud_upload-black-18dp.svg"
                                alt=""
                            />
                            {"Add"}
                        </AppRouteAnchor>
                    </li>
                    <li class={ if p == "/assets" { "active" } else { "" } }>
                        <AppRouteAnchor classes="home-logo-link" route={AppRoute::AssetsPage}>
                            <img
                                src="/icons/folder_shared-black-18dp.svg"
                                alt=""
                            />
                            {"Assets"}
                        </AppRouteAnchor>
                    </li>
                </ul>
            </nav>
        }
    }
}
