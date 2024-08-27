use serde::Deserialize;
use serde::Serialize;
use strum::EnumIter;
use yew_router::prelude::*;

#[derive(Routable, Debug, Clone, PartialEq, Eq, Serialize, Deserialize, strum::Display)]
pub enum Route {
    #[not_found]
    #[at("/")]
    Home,
    #[at("/blog/*")]
    Blog,
    #[at("/projects/*")]
    Projects,
}

#[derive(
    Routable, Debug, Clone, PartialEq, Eq, Serialize, Deserialize, EnumIter, strum::Display,
)]
pub enum BlogRoute {
    #[at("/blog/*")]
    Root,
    #[at("/blog/clone-on-capture")]
    CloneOnCapture,
    #[at("/blog/rust-frontend")]
    RustFrontend,
    #[at("/blog/rust-full-stack-iac")]
    FullStackRustIac,
}

#[derive(
    Routable, Debug, Clone, PartialEq, Eq, Serialize, Deserialize, EnumIter, strum::Display,
)]
pub enum ProjectsRoute {
    #[at("/projects/*")]
    Root,
    #[at("/projects/managed-sccache-dist")]
    ManagedSccacheDist,
}
