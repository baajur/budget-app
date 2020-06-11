use yew::{
    *,
};
pub use plans::{
    *,
    user::*,
    credentials::*,
};
use crate::{
    *,
    root::*,
};
use rql::{
    *,
};
use url::{
    *,
};
use components::{
    fetch::*,
};
use updatable::{
    *,
};
use stdweb::web::{
    *,
    html_element::{InputElement},
};
use stdweb::unstable::TryInto;


#[derive(Properties, Clone, Debug)]
pub struct LoginData {
    pub login: Url,
    pub session_setter: Callback<UserSession>,
}
pub struct Login {
    link: ComponentLink<Self>,
    props: LoginData,
    credentials: Option<Credentials>,
}
pub enum Msg {
    LoginResponse(FetchResponse<UserSession>),
    UpdateCredentials(CredentialsUpdate),
    ToggleShowPassword,
    Login,
    Signup,
    Forgot,
}

impl Login {
    fn set_username_callback(&self) -> Callback<InputData> {
        self.link.callback(|input: InputData| {
            Msg::UpdateCredentials(
                Credentials::update()
                    .username(input.value)
            )
        })
    }
    fn set_password_callback(&self) -> Callback<InputData> {
        self.link.callback(|input: InputData| {
            Msg::UpdateCredentials(
                Credentials::update()
                .password(input.value)
            )
        })
    }
    fn toggle_show_password_callback(&self) -> Callback<ClickEvent> {
        self.link.callback(|_: ClickEvent| {
            let password_input: InputElement = stdweb::web::document()
                .query_selector("#password-input").unwrap()
                .expect("password-input not found")
                .try_into()
                .expect("Failed to cast to InputElement");
            password_input.set_attribute("type",
                match password_input.get_attribute("type") {
                    None => {
                        "password"
                    },
                    Some(s) => {
                        if s.contains("password") {
                            ""
                        } else {
                            "password"
                        }
                    }
                }
            ).unwrap();
            Msg::ToggleShowPassword
        })
    }
    fn login_callback(&self) -> Callback<ClickEvent> {
        self.link.callback(|_: ClickEvent| {
            Msg::Login
        })
    }
    fn login_responder(&self) -> Callback<FetchResponse<UserSession>> {
        self.link.callback(|response| {
            Msg::LoginResponse(response)
        })
    }
    fn signup_callback(&self) -> Callback<ClickEvent> {
        self.link.callback(|_: ClickEvent| {
            Msg::Signup
        })
    }
    fn forgot_callback(&self) -> Callback<ClickEvent> {
        self.link.callback(|_: ClickEvent| {
            Msg::Forgot
        })
    }
}
impl Component for Login {
    type Message = Msg;
    type Properties = LoginData;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let s = Self {
            props,
            link,
            credentials: None,
        };
        s
    }
    fn view(&self) -> Html {
        let credentials = self.credentials.clone().unwrap_or(Credentials::default());
        html!{
            <div id="login-container">
                <div id="username-label">{
                    "Username"
                }</div>
                <input id="username-input" oninput={self.set_username_callback()}/>
                <div id="username-invalid-icon">{
                    format!("{}", credentials.username_is_valid())
                }</div>
                <div id="username-invalid-text">{
                    credentials.username_invalid_text()
                }</div>
                <div id="password-label">{
                    "Password"
                }</div>
                <input
                    id="password-input"
                    type="password"
                    oninput={self.set_password_callback()}/>
                <button
                    id="show-password-button"
                    onclick={self.toggle_show_password_callback()}>{
                    "Show"
                }</button>
                <div id="password-invalid-icon">{
                    format!("{}", credentials.password_is_valid())
                }</div>
                <div id="password-invalid-text">{
                    credentials.password_invalid_text()
                }</div>
                <button
                    id="login-button"
                    onclick={self.login_callback()}>{
                    "Login"
                }</button>
                <button
                    id="signup-button"
                    onclick={self.signup_callback()}>{
                    "Signup"
                }</button>
                <button
                    id="forgot-button"
                    onclick={self.forgot_callback()}>{
                    "Forgot login?"
                }</button>
            </div>
        }
    }
    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateCredentials(update) => {
                console!(log, "UpdateCredentials");
                self.credentials =
                    self.credentials
                        .clone()
                        .or(Some(Credentials::new()))
                        .map(move |mut c| {
                            update.update(&mut c);
                            c
                        });
                true
            },
            Msg::ToggleShowPassword => {
                console!(log, "ToggleShowPassword");
                true
            },
            Msg::Login => {
                match self.credentials.clone() {
                    None => {
                        // Message "Fill in credentials"
                    },
                    Some(credentials) => {
                        // post login
                        Fetch::post(self.props.login.clone(), credentials)
                            .responder(self.login_responder())
                            .send()
                            .expect("Login Request failed");
                    },
                }
                true
            },
            Msg::LoginResponse(response) => {
                console!(log, format!("Response: {:?}", response));
                match response.into_inner() {
                    Ok(session) => {
                        self.props.session_setter.emit(session);
                    },
                    Err(e) => {
                        console!(log, format!("Error: {}", e));
                    },
                }
                true
            },
            Msg::Signup => {
                // redirect to signin page
                false
            },
            Msg::Forgot => {
                // redirect to password recovery page
                false
            },
        }
    }
}