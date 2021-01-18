use graphql_client::*;
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
pub struct Login {
    pub props: Props,
    pub link: ComponentLink<Self>,
    pub fetch: Option<FetchTask>,
}

#[derive(Debug, Clone, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub email: String,
    #[prop_or_default]
    pub password: String,
    #[prop_or_default]
    pub errors: Vec<String>,
    #[prop_or_default]
    pub alert: bool,
}

#[derive(Debug, Clone)]
pub enum Msg {
    EmailChanged(String),
    PasswordChanged(String),
    LoginMutation,
    MutationCompleted(bool),
    MutationErrors(Vec<String>),
    MutationSucceeded(String),
    Redirect,
    AlertChanged(bool),
}

#[derive(GraphQLQuery, Debug)]
#[graphql(
    query_path = "../schema/mutations/sign_in.graphql",
    schema_path = "../schema/schema.graphql",
    response_derives = "Debug,Serialize,Deserialize,PartialEq"
)]
pub struct SignInMutation;

#[derive(Debug, Clone)]
pub struct SignInMutationSignin(String);

impl Component for Login {
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
            Msg::EmailChanged(e) => {
                self.props.email = e;
                true
            }
            Msg::PasswordChanged(p) => {
                self.props.password = p;
                true
            }
            Msg::AlertChanged(b) => {
                self.props.alert = b;
                true
            }
            Msg::Redirect => {
                let w = window();
                let l = w.location();
                l.replace("/").unwrap();
                true
            }
            Msg::MutationSucceeded(s) => {
                ConsoleService::log(&s);
                let w = window();
                let storage = w.local_storage().unwrap().unwrap();
                storage.set_item("token", &s).unwrap();
                true
            }
            Msg::MutationErrors(v) => {
                self.props.errors = Vec::new();
                for e in v.iter() {
                    self.props.errors.push(e.to_string())
                }
                true
            }
            Msg::LoginMutation => {
                let e = self.props.email.clone();
                let p = self.props.password.clone();

                let body = SignInMutation::build_query(sign_in_mutation::Variables {
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
                                    Result<Response<sign_in_mutation::ResponseData>, anyhow::Error>,
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
                                                vec![
                                                    Msg::AlertChanged(false),
                                                    Msg::MutationSucceeded(data.signin),
                                                    Msg::Redirect,
                                                ]
                                            }
                                            None => vec![
                                                Msg::MutationErrors(vec![String::from(
                                                    "Invalid login",
                                                )]),
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
        let alert_box = if self.props.alert {
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
        html! {
            <div id="container" class="form-page">
            <div id="form-container">
                <h2>{"Login"}</h2>
                <form>
                    <input
                        type="email"
                        name="email"
                        id="email"
                        placeholder="user@koompi.com"
                        value=&self.props.email
                        oninput=self.link.callback(|e: InputData| Msg::EmailChanged(e.value))
                    />
                    <input
                        type="password"
                        name="password"
                        id="password"
                        placeholder="314159265359"
                        value=&self.props.password
                        oninput=self.link.callback(|e: InputData| Msg::PasswordChanged(e.value))
                    />
                    <button onclick={self.link.callback(|e: MouseEvent| {
                        e.prevent_default();
                        Msg::LoginMutation
                    })}>{"Login"}</button>
                    { alert_box }
                </form>
            </div>
        </div>
        }
    }
}
