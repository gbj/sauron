use sauron::html::attributes::{class, id, style};
use sauron::html::events::on_click;
use sauron::html::{div, text};
use sauron::prelude::*;
use sauron::{Cmd, Component, Node, Program};
use web_sys::HtmlAudioElement;

pub enum Msg {
    Click,
    HighlightEnd,
}

pub struct FuiButton {
    click: bool,
}

impl FuiButton {
    pub fn new() -> Self {
        FuiButton { click: false }
    }

    fn play_sound(&self) {
        let audio = HtmlAudioElement::new_with_src("/sounds/click.mp3")
            .expect("must not fail");
        let _ = audio.play().expect("must play");
    }

    fn child(&self) -> Node<Msg> {
        text("Fui Button")
    }

    pub fn style(&self) -> Vec<String> {
        vec![r#"
        .fui_button {
            display: inline-block;
            padding: 1px;
            position: relative;
        }

        .fui_button__border {
            border-color: #029dbb;
            box-shadow: 0 0 4px rgba(2,157,187,0.65);
        }

        .hide .fui_button__border {
          height: 0;
          width: 0;
        }

        .fui_button__border-left {
            top: 50%;
            left: 0;
            height: 100%;
            transform: translate(0, -50%);
            border-width: 0 0 0 1px;
        }


        .fui_button__border-anim {
            z-index: 1;
            opacity: 1;
            position: absolute;
            transition: all 250ms ease-in;
            border-style: solid;
        }

        .fui_button__border-right {
            top: 50%;
            right: 0;
            height: 100%;
            transform: translate(0, -50%);
            border-width: 0 0 0 1px;
        }


        .fui_button__border-top {
            top: 0;
            left: 50%;
            width: 100%;
            transform: translate(-50%, 0);
            border-width: 1px 0 0 0;
        }


        .fui_button__border-bottom {
            left: 50%;
            width: 100%;
            bottom: 0;
            transform: translate(-50%, 0);
            border-width: 1px 0 0 0;
        }


        .fui_button__corner {
            width: 24px;
            height: 24px;
            border-color: #26dafd;
            box-shadow: 0 0 4px -2px rgba(38,218,253,0.65);
        }

        .hide .fui_button__corner{
            width: 0;
            height: 0;
            opacity: 0;
        }

        .fui_button__corner-anim {
            z-index: 2;
            opacity: 1;
            position: absolute;
            transition: all 250ms ease-in;
            border-style: solid;
        }

        .fui_button_corner__top-left {
            left: -2px;
            top: -2px;
            border-width: 2px 0 0 2px;
        }


        .fui_button_corner__bottom-left {
            left: -2px;
            bottom: -2px;
            border-width: 0 0 2px 2px;
        }


        .fui_button_corner__top-right {
            right: -2px;
            top: -2px;
            border-width: 2px 2px 0 0;
        }


        .fui_button_corner__bottom-right {
            right: -2px;
            bottom: -2px;
            border-width: 0 2px 2px 0;
        }


        .fui_button-text {
            background-color: rgba(4,35,41,0.65);
        }

        .hide .fui_button-text {
            background-color: transparent;
        }

        .fui_button-text-anim {
            z-index: 3;
            display: block;
            position: relative;
            overflow: hidden;
            transition: background-color 250ms ease-in;
        }

        .fui_button__button {
            color: #acf9fb;
            cursor: pointer;
        }
        .fui_button__button-anim {
            margin: 0;
            border: none;
            z-index: 2;
            display: inline-block;
            padding: 10px 20px;
            outline: none;
            position: relative;
            font-size: 15.75px;
            background: transparent;
            transition: all 250ms ease-out;
            line-height: 1;
            user-select: none;
            vertical-align: middle;
        }


        .fui_button__highlight{
              z-index: 1;
              position: absolute;
              left: 0;
              right: 0;
              top: 0;
              bottom: 0;
              background-color: transparent;
              opacity: 0;
        }

        .click .fui_button__highlight{
            opacity: 1;
            background-color: #029dbb
        }

        .fui_button__highlight-anim{
            transition: all 50ms ease-out;
        }
        "#
        .to_string()]
    }

    pub fn update(&mut self, msg: Msg) -> Cmd<Self, Msg> {
        match msg {
            Msg::Click => {
                self.play_sound();
                self.click = true;
            }
            Msg::HighlightEnd => {
                self.click = false;
            }
        }
        Cmd::none()
    }

    pub fn view(&self) -> Node<Msg> {
        div(
            vec![
                class("fui_button"),
                classes_flag([("click",self.click)]),
            ],
            vec![
                div(vec![class("fui_button__border fui_button__border-anim fui_button__border-left")], vec![]),
                div(vec![class("fui_button__border fui_button__border-anim fui_button__border-right")], vec![]),
                div(vec![class("fui_button__border fui_button__border-anim fui_button__border-top")], vec![]),
                div(vec![class("fui_button__border fui_button__border-anim fui_button__border-bottom")], vec![]),
                div(vec![class("fui_button__corner fui_button__corner-anim fui_button_corner__top-left")], vec![]),
                div(
                    vec![class("fui_button__corner fui_button__corner-anim fui_button_corner__bottom-left")],
                    vec![],
                ),
                div(
                    vec![class("fui_button__corner fui_button__corner-anim fui_button_corner__top-right")],
                    vec![],
                ),
                div(
                    vec![class("fui_button__corner fui_button__corner-anim fui_button_corner__bottom-right")],
                    vec![],
                ),
                div(vec![class("fui_button__wrap")],
                    vec![
                        div(
                            vec![class("fui_button-text fui_button-text-anim")],
                            vec![
                                button(
                                    vec![
                                        class("fui_button__button fui_button__button-anim"),
                                        on_click(|_|Msg::Click)
                                    ],
                                    vec![self.child()]
                                )
                            ],
                        ),
                        div(vec![
                            class("fui_button__highlight fui_button__highlight-anim"),
                            on_transitionend(|_|Msg::HighlightEnd),
                            ],
                            vec![]
                        )
                    ]
                ),
            ],
        )
    }
}