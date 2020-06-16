use seed::{
    *,
    prelude::*,
};
use crate::{
    *,
};
use rql::{
    *,
};
use std::str::{
    FromStr,
};

#[derive(Clone, Default)]
pub struct Model {
    navbar: navbar::Model, // the navigation bar
    page: page::Model, // the current page
}

#[derive(Clone)]
pub enum Msg {
    NavBar(navbar::Msg),
    Page(page::Msg),
    SetPage(page::Model),
}
#[derive(Clone)]
pub enum GMsg {
    Root(Msg),
}
impl From<page::Msg> for Msg {
    fn from(msg: page::Msg) -> Self {
        Self::Page(msg)
    }
}
impl From<navbar::Msg> for Msg {
    fn from(msg: navbar::Msg) -> Self {
        Self::NavBar(msg)
    }
}
pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg, GMsg>) {
    match msg {
        Msg::Page(msg) => {
            page::update(
                msg.clone(),
                &mut model.page,
                &mut orders.proxy(Msg::Page)
            );
            match msg {
                _ => {},
            }
        },
        Msg::NavBar(msg) => {
            navbar::update(
                msg.clone(),
                &mut model.navbar,
                &mut orders.proxy(Msg::NavBar)
            );
            match msg {
                _ => {},
            }
        },
        Msg::SetPage(page) => {
            model.page = page;
            page::update(
                page::Msg::FetchData,
                &mut model.page,
                &mut orders.proxy(Msg::Page)
            );
        },
    }
}
pub fn sink(msg: GMsg, model: &mut Model, orders: &mut impl Orders<Msg, GMsg>) {
    match msg {
        GMsg::Root(msg) => {
            orders.send_msg(msg);
        }
    }
}
pub fn view(model: &Model) -> impl View<Msg> {
    div![
        navbar::view(&model.navbar)
            .map_msg(Msg::NavBar),
        page::view(&model.page)
            .map_msg(Msg::Page),
    ]
}

fn routes(url: Url) -> Option<Msg> {
    if url.path.is_empty() {
        Some(Msg::SetPage(page::Model::home()))
    } else {
        match &url.path[0][..] {
            "login" => Some(Msg::SetPage(page::Model::login())),
            "register" => Some(Msg::SetPage(page::Model::register())),
            "users" =>
                if url.path.len() == 1 {
                    Some(Msg::SetPage(page::Model::users()))
                } else if url.path.len() == 2 {
                    match Id::from_str(&url.path[1]) {
                        Ok(id) => Some(Msg::SetPage(page::Model::user(id))),
                        Err(_e) => Some(Msg::SetPage(page::Model::NotFound)),
                    }
                } else {
                    Some(Msg::SetPage(page::Model::NotFound))
                },
            _ => Some(Msg::SetPage(page::Model::home())),
        }
    }
}
fn after_mount(_url: Url, orders: &mut impl Orders<Msg, GMsg>) -> AfterMount<Model> {
    AfterMount::default()
}
#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view)
        .routes(routes)
        .sink(sink)
        .after_mount(after_mount)
        .build_and_start();
}
