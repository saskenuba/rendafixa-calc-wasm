use dioxus::{
    prelude::*,
    router::{Link, Redirect, Route, Router},
};

#[inline_props]
pub fn SidebarSection<'a>(cx: Scope<'a>, title: String, children: Element<'a>) -> Element {
    cx.render(rsx! {

        section {
            class: "flex flex-col gap-y-4",

            h5 { "{ title }"}
            children
        }
    })
}

pub fn sidebar(cx: Scope) -> Element {
    cx.render(rsx! {

        aside {
            class: "col-span-4",

            SidebarSection {
                title: "Renda Fixa".to_string()
                Link { class: "text-sm", to: "/", "Poupança, CDB e LCI" }
            },
            SidebarSection {
                title: "Outros".to_string()
                Link { to: "/outros", "Teste" }
            },
        }
    })
}
