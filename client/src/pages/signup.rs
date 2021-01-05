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
    Callback, InputData, MouseEvent,
};

pub struct Signup {
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
}

#[derive(Debug, Clone)]
pub enum SignupMessage {
    EmailChanged(String),
    POneChanged(String),
    PTwoChanged(String),
    RunFetch,
    FetchResourceFailed,
    FetchResourceComplete,
}

impl Component for Signup {
    type Message = SignupMessage;
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
            SignupMessage::EmailChanged(s) => {
                ConsoleService::log(&s);
                self.props.email = s;
                true
            }
            SignupMessage::POneChanged(s) => {
                ConsoleService::log(&s);
                self.props.p1 = s;
                true
            }
            SignupMessage::PTwoChanged(s) => {
                ConsoleService::log(&s);
                self.props.p2 = s;
                true
            }
            SignupMessage::FetchResourceFailed => {
                ConsoleService::log("Failed");
                false
            }
            SignupMessage::FetchResourceComplete => {
                ConsoleService::log("Signup completed");
                false
            }
            SignupMessage::RunFetch => {
                let e = self.props.email.clone();
                let p = self.props.p1.clone();

                let body = SignUp::build_query(sign_up::Variables {
                    email: e,
                    password: p,
                });
                // ConsoleService::log(&format!("{:?}", body.query));

                let request = ::http::Request::post("http://0.0.0.0:4000/api")
                    .header("Credentials", "same-origin")
                    .header("Accept", "application/json")
                    .header("Content-Type", "application/json")
                    // .header("Authorization", "Bearer 59ea201e2a09a99126edad345f7cd1f0")
                    .body(Json(&body));
                match request {
                    Ok(r) => {
                        let callback = self.link.callback(
                            |response: http::Response<
                                Json<Result<Response<sign_up::ResponseData>, anyhow::Error>>,
                            >| {
                                let (meta, body) = response.into_parts();
                                if meta.status.is_success() {
                                    SignupMessage::FetchResourceFailed
                                } else {
                                    // Msg::Receive(data.ok())
                                    ConsoleService::log(&format!("{:#?}", &meta));
                                    ConsoleService::log(&format!("{:#?}", &body));
                                    SignupMessage::FetchResourceComplete
                                }
                            },
                        );

                        // let task = FetchService::fetch(r, callback);
                        // match task {
                        // Ok(t) => false,
                        // Err(e) => {
                        //     ConsoleService::log(&format!("{:#?}", e));
                        //     false
                        // }
                        // }
                        // match self.fetch::request {
                        // Ok(t) => false,
                        // Err(e) => {
                        //     ConsoleService::log(&format!("{:#?}", e));
                        //     false
                        // }
                        // }
                        self.fetch = Some(FetchService::fetch(r, callback).unwrap());
                        self.fetch.as_ref().unwrap();
                        false
                    }
                    Err(e) => {
                        ConsoleService::log(&format!("{:#?}", e));
                        false
                    }
                }
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div id="container" class="form-page">
                <div id="form-container">
                    <h2>{"Sign Up"}</h2>
                    <form action="">
                        <input
                            type="email"
                            name="email"
                            id="email"
                            placeholder="Email"
                            value=&self.props.email
                            oninput=self.link.callback(|e: InputData| SignupMessage::EmailChanged(e.value))
                        />
                        <input
                            type="password"
                            name="password"
                            id="password"
                            placeholder="Password"
                            value=&self.props.p1
                            oninput=self.link.callback(|e: InputData| SignupMessage::POneChanged(e.value))
                        />
                        <input
                            type="password"
                            name="password"
                            id="password"
                            placeholder="Confirm password"
                            value=&self.props.p2
                            oninput=self.link.callback(|e: InputData| SignupMessage::PTwoChanged(e.value))
                        />
                        <button
                            disabled={ self.props.email.is_empty() || self.props.p1.is_empty() || self.props.p2.is_empty() || (self.props.p1 != self.props.p2) }
                            onclick=self.link.callback(|e: MouseEvent| {
                                e.prevent_default();
                                SignupMessage::RunFetch
                            })
                        >{"Sign up"}</button>
                    </form>
                </div>
            </div>
        }
    }
}

#[derive(GraphQLQuery, Debug)]
#[graphql(
    query_path = "../schema/mutations/signup.graphql",
    schema_path = "../schema/schema.graphql",
    response_derives = "Debug,Serialize,Deserialize,PartialEq"
)]
pub struct SignUp;

#[derive(Debug, Serialize, Deserialize)]
pub struct Email {
    email: String,
}
// pub async fn signup(email: String, password: String) -> Result<Option<Email>, anyhow::Error> {
//     let req = SignUp::build_query(sign_up::Variables {
//         email: email,
//         password: password,
//     });

//     let client = reqwest::Client::new();
// let res = client
//     .post("http://127.0.0.1:4000/api")
//     .json(&req)
//     .send()
//     .await?;
//     let response_body: Response<sign_up::ResponseData> = res.json().await?;

//     match serde_json::to_string_pretty(&response_body) {
//         Ok(s) => {
//             let all_apps_data: Response<Email> = serde_json::from_str(s.as_str())?;
//             match all_apps_data.data {
//                 Some(d) => Ok(Some(d)),
//                 None => Ok(None),
//             }
//         }
//         Err(e) => Err(anyhow::Error::from(e)),
//     }
// }