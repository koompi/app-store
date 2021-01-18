use crate::routes::AppRoute;
use anyhow;
use graphql_client::*;
use serde::{Deserialize, Serialize};
use yew::{
    format::Json,
    prelude::*,
    services::fetch::FetchTask,
    services::{
        fetch::{FetchService, Request},
        ConsoleService,
    },
    utils::window,
    Callback, InputData, MouseEvent,
};
use yew_router::prelude::*;
pub struct SignUpPage {
    pub props: Props,
    pub link: ComponentLink<Self>,
    pub fetch: Option<FetchTask>,
}

#[derive(Debug, Clone, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub email: String,
    #[prop_or_default]
    pub p1: String,
    #[prop_or_default]
    pub p2: String,
    #[prop_or_default]
    pub alert: bool,
    #[prop_or_default]
    pub errors: Vec<String>,
    #[prop_or_default]
    redirect: bool,
}

#[derive(Debug)]
pub enum Msg {
    EmailChanged(String),
    POneChanged(String),
    PTwoChanged(String),
    AlertChanged(bool),
    RunMutation,
    MutationCompleted(bool),
    MutationErrors(Vec<String>),
    MutationSucceeded(Option<sign_up_mutation::SignUpMutationSignup>),
}

#[derive(GraphQLQuery, Debug)]
#[graphql(
    query_path = "../schema/mutations/sign_up.graphql",
    schema_path = "../schema/schema.graphql",
    response_derives = "Debug,Serialize,Deserialize,PartialEq"
)]
pub struct SignUpMutation;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignUpMutationSignup {
    email: String,
}

impl Component for SignUpPage {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            fetch: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::EmailChanged(s) => {
                ConsoleService::log(&s);
                self.props.email = s;
                true
            }
            Msg::POneChanged(s) => {
                ConsoleService::log(&s);
                self.props.p1 = s;
                true
            }
            Msg::PTwoChanged(s) => {
                ConsoleService::log(&s);
                self.props.p2 = s;
                true
            }
            Msg::AlertChanged(b) => {
                ConsoleService::log(&format!("{}", b));
                self.props.alert = b;
                true
            }
            Msg::MutationCompleted(b) => {
                ConsoleService::log(&format!("Muation completed with status: {:#?}", b));
                b
            }
            Msg::MutationErrors(v) => {
                self.props.errors = Vec::new();
                for e in v.iter() {
                    self.props.errors.push(e.to_string())
                }
                true
            }
            Msg::RunMutation => {
                let e = self.props.email.clone();
                let p = self.props.p1.clone();

                let body = SignUpMutation::build_query(sign_up_mutation::Variables {
                    email: e,
                    password: p,
                });

                let request = ::http::Request::post("http://0.0.0.0:4000/api")
                    .header("Credentials", "same-origin")
                    .header("Accept", "application/json")
                    .header("Content-Type", "application/json")
                    .body(Json(&body));

                match request {
                    Ok(r) => {
                        let callback = self.link.batch_callback(
                            |http_res: http::Response<
                                Json<
                                    Result<Response<sign_up_mutation::ResponseData>, anyhow::Error>,
                                >,
                            >| {
                                let (network, gql) = http_res.into_parts();

                                if network.status.is_success() {
                                    ConsoleService::log(&format!("Muation finished running"));
                                }

                                if network.status.is_client_error() {
                                    ConsoleService::log(&format!(
                                        "There was some error in the client"
                                    ));
                                }

                                if network.status.is_server_error() {
                                    ConsoleService::log(&format!("Interal server error"));
                                }

                                match gql.0 {
                                    Ok(gql_res) => match gql_res.errors {
                                        Some(errors) => {
                                            let errs: Vec<String> =
                                                errors.iter().cloned().map(|e| e.message).collect();
                                            vec![
                                                Msg::MutationCompleted(false),
                                                Msg::MutationErrors(errs),
                                                Msg::AlertChanged(true),
                                            ]
                                        }
                                        None => match gql_res.data {
                                            Some(data) => {
                                                ConsoleService::log(&format!("{:#?}", data.signup));
                                                let w = window();
                                                let l = w.location();
                                                l.replace("/login").unwrap();
                                                vec![
                                                    Msg::MutationSucceeded(None),
                                                    Msg::AlertChanged(true),
                                                ]
                                            }
                                            None => vec![
                                                Msg::MutationSucceeded(None),
                                                Msg::AlertChanged(true),
                                            ],
                                        },
                                    },
                                    Err(e) => {
                                        vec![
                                            Msg::MutationCompleted(false),
                                            Msg::MutationErrors(vec![String::from(
                                                "INTERNAL_SERVER_ERROR",
                                            )]),
                                            Msg::AlertChanged(true),
                                        ]
                                    }
                                }
                            },
                        );

                        self.fetch = Some(FetchService::fetch(r, callback).unwrap());
                        self.fetch.as_ref().unwrap();

                        true
                    }
                    Err(e) => {
                        ConsoleService::log(&format!("{:#?}", e));
                        true
                    }
                }
            }
            _ => false,
        }
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
        let AlertBox = if self.props.alert {
            html! {
                <div class="message-container" >
                    {
                        self.props.errors.iter().map(|e| {
                            html! { <p>{e}</p> }
                        }).collect::<Html>()
                    }
                </div>
            }
        } else {
            html! {""}
        };

        ConsoleService::log(&self.props.alert.to_string());
        // router_agent.send(yew_router::Request::ChangeRoute(
        //     RouterTarget::Content.into(),
        // ));
        // yew_router::agent::RouteRequest::ChangeRoute(yew_router::route::Route::new_no_state("/"));
        // yew_router::agent::RouteRequest::ReplaceRoute(yew_router::route::Route::new_no_state("/"));
        // let redirect = Router::redirect(|route: Route| AppRoute::LoginPage);

        html! {
            <div id="container" class="form-page">
                // { if self.props.redirect { redirect.unwra } }
                <div id="form-container">
                    <h2>{"Sign Up"}</h2>
                    <form action="">
                        <input
                            type="email"
                            name="email"
                            id="email"
                            placeholder="Email"
                            value=&self.props.email
                            oninput=self.link.callback(|e: InputData| Msg::EmailChanged(e.value))
                        />
                        <input
                            type="password"
                            name="password"
                            id="password"
                            placeholder="Password"
                            value=&self.props.p1
                            oninput=self.link.callback(|e: InputData| Msg::POneChanged(e.value))
                        />
                        <input
                            type="password"
                            name="password"
                            id="password"
                            placeholder="Confirm password"
                            value=&self.props.p2
                            oninput=self.link.callback(|e: InputData| Msg::PTwoChanged(e.value))
                        />
                        <button
                            disabled={ self.props.email.is_empty() || self.props.p1.is_empty() || self.props.p2.is_empty() || (self.props.p1 != self.props.p2) }
                            onclick=self.link.callback(|e: MouseEvent| {
                                e.prevent_default();
                                Msg::RunMutation

                            })
                        >{"Sign up"}</button>
                        { AlertBox }
                    </form>

                </div>
            </div>
        }
    }
}
