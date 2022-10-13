mod components;
pub mod pages;
mod routes;
pub mod rules;
mod sidebar;

use sidebar::sidebar;

use dioxus::{
    prelude::*,
    router::{Link, Redirect, Route, Router},
};

fn bottom_bar(cx: Scope) -> Element {
    cx.render(rsx!(
    div {
        class: "flex justify-center",
            span { "Desenvolvido por Martin Mariano - Totalmente em WASM - " },
            a { href: "https://martinmariano.com", " martinmariano.com"}
    }))
}

pub fn app(cx: Scope) -> Element {
    cx.render(rsx! {

        Router {
            Route { to: "/", self::pages::homepage::main {} }
            Route { to: "/outros", self::pages::other::main {} }
        }

        bottom_bar()
    })
}
