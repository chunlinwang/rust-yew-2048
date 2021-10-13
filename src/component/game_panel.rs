use std::io;
use yew::prelude::*;

use gloo::events::EventListener;
use strum::IntoEnumIterator;
use yew::format::Json;
use yew::services::storage::{Area, StorageService};
use yew::web_sys::HtmlInputElement as InputElement;
use yew::{web_sys::{Event, HtmlElement, Window},
          classes, html, Component, ComponentLink, Html, InputData, NodeRef, ShouldRender, Callback};
use yew::events::{KeyboardEvent};
use yew::services::ConsoleService;
use yew::services::keyboard::{KeyboardService, KeyListenerHandle};

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
    game_listener: Option<KeyListenerHandle>,
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
            game_listener: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Up => {
                self.matrix.move_to(Direction::UP);
                self.state.edit_value = "".to_string();
            }
            Msg::Down => {
                self.matrix.move_to(Direction::DOWN);
            }
            Msg::Left => {
                self.matrix.move_to(Direction::LEFT);
            }
            Msg::Right => {
                self.matrix.move_to(Direction::RIGHT);
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
        ConsoleService::info(format!("game panel view {:?}", self.matrix.data).as_ref());

        html! {
            <div class="container" ref={self.focus_ref.clone()}>
                <div class="heading">
                  <h1 class="title">{"1024"}</h1>
                  <ScorePanel score={self.matrix.score} best={0} />
                </div>
                <p class="game-intro">{"Join the numbers and get to the"} <strong>{"1024 tile!"}</strong></p>
                <GameContainer girds={self.matrix.data} />
            </div>
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if !first_render {
            return;
        }

        let document = yew::utils::document();

        if let Some(element) = Some(document) {
            let onkeypress_callback = self.link.batch_callback(move |event: KeyboardEvent| {
                ConsoleService::info(format!("game panel onkeypress_callback {:?}", event.key()).as_ref());
                if event.key() == "w" || event.key() == "q" {
                    Some(Msg::Up)
                } else if event.key() == "s" {
                    Some(Msg::Down)
                } else if event.key() == "a" || event.key() == "q" {
                    Some(Msg::Left)
                } else if event.key() == "d"  {
                    Some(Msg::Right)
                } else {
                    None
                }
            });

            let listener =
                KeyboardService::register_key_press(
                    &element,
                    onkeypress_callback
                );

            self.game_listener = Some(listener);
        }
    }
}
