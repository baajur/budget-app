pub use plans::{
    *,
    user::*,
};
use yew::{
    *,
    services::{
        *,
        fetch::{
            *,
            FetchTask,
        },
    },
    format::{
        Json,
        Nothing,
    },
};
use http::{
    *,
    request::*,
};
use crate::{
    *,
};
use anyhow::Error;
use std::result::Result;
use std::fmt::{Display, Debug};

pub enum Msg {
    GotUser(User),
    FetchUserError(String),
}

#[derive(Properties, Clone, Debug)]
pub struct UserProfile {
    user: User,
}
impl From<User> for UserProfile {
    fn from(user: User) -> Self {
        Self {
            user,
        }
    }
}
pub struct UserProfileView {
    link: ComponentLink<Self>,
    props: Option<UserProfile>,
    fetch_task: Option<FetchTask>,
    fetch_service: FetchService,
}

impl Component for UserProfileView {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let req = Request::get("/api/user")
            .body(Nothing).unwrap();
        let callback = link.callback(|response: Response<Json<Result<User, Error>>>| {
            console!(log, "Got response");
            let (meta, Json(body)) = response.into_parts();
            if meta.status.is_success() {
                console!(log, "Got user");
                Msg::GotUser(body.unwrap())
            } else {
                Msg::FetchUserError(
                    meta.status.clone()
                        .canonical_reason()
                        .map(ToString::to_string)
                        .unwrap_or(format!("Got StatusCode {}", meta.status))
                )
            }
        });
        let mut fetch_service = FetchService::new();
        let task = fetch_service.fetch(req, callback);
        let mut fetch_task = None;
        match task {
            Ok(task) => {
                fetch_task = Some(task)
            },
            Err(err) => {
                link.send_message(Msg::FetchUserError(err.to_string()))
            },
        }
        Self {
            link,
            props: None,
            fetch_service,
            fetch_task,
        }
    }
    fn view(&self) -> Html {
        if let Some(profile) = self.props.clone() {
            html!{
                <div class="user-profile">
                    <div class="user-name">
                        {format!("User Name: {}", profile.user.name())}
                    </div>
                    <img src="/dweeb.jpg"/>
                </div>
            }
        } else {
            html!{ }
        }
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GotUser(user) => {
                self.props = Some(UserProfile::from(user));
                true
            }
            Msg::FetchUserError(err) => {
                console!(log, "FetchUserError: {}", format!("{}", err));
                true
            },
        }
    }
}