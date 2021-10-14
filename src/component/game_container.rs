use yew::prelude::*;
use crate::component::game_line::GameLine;
use yew::services::ConsoleService;
use yew::{html, ChildrenWithProps, Component, Html, Properties};

#[derive(Properties, Clone, PartialEq, Debug)]
pub struct Props {
    pub girds: [[usize; 4]; 4],
    pub is_over: bool,
}

pub enum Msg {
    Retry,
}

#[derive(Debug)]
pub struct GameContainer {
    link: ComponentLink<GameContainer>,
    props: Props,
}

impl Component for GameContainer {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Retry => {
                let window = yew::utils::window();
                window.location().reload();
            }
        }

        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let changed = self.props != props;
        if changed {
            self.props = props;
        }

        changed
    }

    fn view(&self) -> Html {
        ConsoleService::info(format!("game container view {:?}", self.props.girds).as_ref());
        html! {
            <div class="game-container">
            {
                    if {self.props.is_over} {
                        html! {
                            <>
                            <div class=classes!("game-message", "game-over")>
                                <p>{"Game over!"}</p>
                            </div>
                            <div class="game-message">
                                <div class="lower">
                                    <a class="retry-button" onclick=self.link.callback(move |_| Msg::Retry)>{"Try again"}</a>
                                </div>
                            </div>
                            </>
                        }
                    } else {
                        html! {}
                    }
                }
                <div class="grid-container">
                    <div class="grid-row">
                        <div class="grid-cell"></div>
                        <div class="grid-cell"></div>
                        <div class="grid-cell"></div>
                        <div class="grid-cell"></div>
                    </div>
                    <div class="grid-row">
                        <div class="grid-cell"></div>
                        <div class="grid-cell"></div>
                        <div class="grid-cell"></div>
                        <div class="grid-cell"></div>
                    </div>
                    <div class="grid-row">
                          <div class="grid-cell"></div>
                          <div class="grid-cell"></div>
                          <div class="grid-cell"></div>
                          <div class="grid-cell"></div>
                    </div>
                    <div class="grid-row">
                          <div class="grid-cell"></div>
                          <div class="grid-cell"></div>
                          <div class="grid-cell"></div>
                          <div class="grid-cell"></div>
                    </div>
                </div>
                <div class="tile-container">
                    <GameLine line={self.props.girds[0]} y={1} />
                    <GameLine line={self.props.girds[1]} y={2}/>
                    <GameLine line={self.props.girds[2]} y={3}/>
                    <GameLine line={self.props.girds[3]} y={4}/>
                </div>
            </div>
        }
    }
}
