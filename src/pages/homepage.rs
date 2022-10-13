use dioxus::events::{oninput, FormEvent};
use dioxus::fermi::use_atom_state;
use dioxus::{
    fermi::Atom,
    prelude::*,
    router::{Link, Redirect, Route, Router},
};
use std::str::FromStr;

use crate::rules::rule_poupanca;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

static INITIAL_AMOUNT: Atom<Decimal> = |_| dec!(0);
static TIME_RANGE: Atom<i32> = |_| 0;

pub fn main(cx: Scope) -> Element {
    let initial_amount = use_atom_state(&cx, INITIAL_AMOUNT);
    let time_range = use_atom_state(&cx, TIME_RANGE);

    let retorno_poupa = |amount| rule_poupanca(amount, dec!(0.1375), dec!(0.01781));

    cx.render(rsx! {
        main {
            class: "grid grid-cols-12 gap-4",

                crate::sidebar()

                div {
                    class: "col-span-8",





                    div {
                        class: "flex justify-start gap-x-6",

                        div {
                            class: "flex flex-col",

                            label {
                                class: "text-xs",
                                "Qual o montante que você deseja investir?"
                            }
                            input {
                                class: "rounded text-pink-500",
                                oninput:  move |event: FormEvent|  {
                                    let value_as_decimal = Decimal::from_str(&event.value).unwrap();
                                    initial_amount.with_mut(|v| *v = value_as_decimal);
                                }
                            }
                        }
                    }

                    div {

                    h5 { "Poupança "}
                    span { [retorno_poupa(**initial_amount).gross_amount.to_string()]}

                }

                    div {
                        span {  "amount {initial_amount}"  }

                    }
                }
        }
    })
}
