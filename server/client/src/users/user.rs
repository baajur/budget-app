use seed::{
    *,
    prelude::*,
};
use plans::{
    user::*,
};
use rql::{
    *,
};
use crate::{
    users::*,
    root::{
        GMsg,
    },
    status::*,
    request,
};
use database::{
    Entry,
};

#[derive(Clone, Default)]
pub struct Model {
    pub user_id: Id<User>,
    pub user: Status<User>,
}
impl Model {
    pub fn preview(&self) -> preview::Model {
        preview::Model::from(self.clone())
    }
    pub fn profile(&self) -> profile::Model {
        profile::Model::from(self.clone())
    }
    fn ready(user_id: Id<User>, user: User) -> Self {
        Self {
            user_id,
            user: Status::Ready(user),
        }
    }
    fn empty(user_id: Id<User>) -> Self {
        Self {
            user_id,
            user: Status::Empty,
        }
    }
}
impl From<&Entry<User>> for Model {
    fn from(entry: &Entry<User>) -> Self {
        Self::ready(*entry.id(), entry.data().clone())
    }
}
impl From<Id<User>> for Model {
    fn from(user_id: Id<User>) -> Self {
        Self::empty(user_id)
    }
}
pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg, GMsg>) {
    match msg {
        Msg::FetchUser => {
            orders.perform_cmd(request::fetch_user(model.user_id));
        },
        Msg::FetchedUser(res) => {
            match res {
                Ok(user) => {
                    model.user = Status::Ready(user);
                },
                Err(reason) => {
                    seed::log!(reason);
                },
            }
        },
    }
}

#[derive(Clone)]
pub enum Msg {
    FetchUser,
    FetchedUser(ResponseDataResult<User>),
}