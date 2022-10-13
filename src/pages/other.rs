use dioxus::prelude::*;

pub fn main(cx: Scope) -> Element {
    cx.render(rsx! {
        main {
            class: "grid grid-cols-12 gap-4",

                crate::sidebar()

                div {
                    class: "col-span-8 bg-red-500",
                    p { "Eae!!"}
                }
        }
    })
}
