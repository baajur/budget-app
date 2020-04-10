#![recursion_limit = "1024"]
extern crate seed;
#[macro_use] extern crate stdweb;
extern crate wasm_bindgen;
#[macro_use] extern crate lazy_static;
extern crate http;
extern crate anyhow;

extern crate plans;

mod transaction;
mod transactions;
mod budget;
//mod router;
//mod userprofile;

use wasm_bindgen::prelude::*;
use seed::*;
pub use plans::{
    *,
    currency::*,
};
pub use crate::{
    budget::*,
};

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    App::builder(budget::update::<Euro>, budget::view::<Euro>)
        .build_and_run()
}
