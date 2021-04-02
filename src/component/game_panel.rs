use std::io;
use yew::prelude::*;

use strum::IntoEnumIterator;
use yew::format::Json;
use yew::services::storage::{Area, StorageService};
use yew::web_sys::HtmlInputElement as InputElement;
use yew::{classes, html, Component, ComponentLink, Html, InputData, NodeRef, ShouldRender, Callback};
use yew::{events::KeyboardEvent, Classes};
use yew::services::ConsoleService;

use crate::game::{Matrix, Direction};
use crate::component::score_panel::ScorePanel;
use crate::component::game_container::GameContainer;

// use crate::state;
use crate::state::{Entry, Filter, State};

const KEY: &str = "rust-1024";

pub enum Msg {
    Up,
    Down,
    Left,
    Right,
}

pub struct GamePanel {
    link: ComponentLink<GamePanel>,
    pub matrix: Matrix,
    pub storage: StorageService,
    pub state: State,
    focus_ref: NodeRef,
}

impl Component for GamePanel {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let storage = StorageService::new(Area::Local).expect("storage was disabled by the user");
        let entries = {
            if let Json(Ok(restored_model)) = storage.restore(KEY) {
                restored_model
            } else {
                Vec::new()
            }
        };
        let state = State {
            entries,
            filter: Filter::All,
            value: "".into(),
            edit_value: "".into(),
        };
        let focus_ref = NodeRef::default();
        let matrix = Matrix::new();
        Self {
            link,
            matrix,
            storage,
            state,
            focus_ref,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Up => {
                self.matrix.move_to(Direction::UP);
                ConsoleService::info(format!("UP {:?}", self.matrix).as_ref());
                self.state.edit_value = "".to_string();
            }
            Msg::Down => {
                self.matrix.move_to(Direction::DOWN);
                ConsoleService::info(format!("DOWN {:?}", self.matrix).as_ref());
            }
            Msg::Left => {
                self.matrix.move_to(Direction::LEFT);
                ConsoleService::info(format!("LEFT {:?}", self.matrix).as_ref());
            }
            Msg::Right => {
                self.matrix.move_to(Direction::RIGHT);
                ConsoleService::info(format!("RIGHT {:?}", self.matrix).as_ref());
            }
        }
        self.storage.store(KEY, Json(&self.state.entries));
        ConsoleService::info(format!("game panel update").as_ref());
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        ConsoleService::info(format!("game panel change").as_ref());
        false
    }

    fn view(&self) -> Html {
        let onkeypress_callback = self.link.batch_callback(move |event: KeyboardEvent| {
            // ConsoleService::info(format!("key {:?}", event.key()).as_ref());
            // ConsoleService::info(format!("key {:?}", event).as_ref());
            if event.key() == "ArrowUp" {
                Some(Msg::Up)
            } else if event.key() == "ArrowDown" {
                Some(Msg::Down)
            } else if event.key() == "ArrowLeft" {
                Some(Msg::Left)
            } else if event.key() == "ArrowRight"  {
                Some(Msg::Right)
            } else {
                None
            }
        });

        html! {
            <div class="container" >
                <button onclick=self.link.callback(move |_| Msg::Up)>{"u"}</button>
                <button onclick=self.link.callback(move |_| Msg::Down) >{"d"}</button>
                <button onclick=self.link.callback(move |_| Msg::Left) >{"l"}</button>
                <button onclick=self.link.callback(move |_| Msg::Right) >{"r"}</button>
                <div class="heading">
                  <h1 class="title">{"1024"}</h1>
                  <ScorePanel score={self.matrix.score} best={0} />
                </div>
                <p class="game-intro">{"Join the numbers and get to the"} <strong>{"1024 tile!"}</strong></p>
                <GameContainer girds={self.matrix.data} />
            </div>
        }
    }
}
