use serde::Deserialize;
use serde::Serialize;
use strum::EnumIter;
use yew::html::ImplicitClone;
use yew_router::prelude::*;

#[derive(
    Routable, Debug, Clone, PartialEq, Eq, Serialize, Deserialize, EnumIter, strum::Display,
)]
pub enum Route {
    #[at("/")]
    Root,
    #[at("/clone-on-capture")]
    CloneOnCapture,
    #[at("/rust-frontend")]
    RustFrontend,
    #[at("/rust-full-stack-iac")]
    FullStackRustIac,
}

impl ImplicitClone for Route {}
