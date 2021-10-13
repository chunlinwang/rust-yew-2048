use yew::prelude::*;
use crate::component::game_line::GameLine;
use yew::services::ConsoleService;
use yew::{html, ChildrenWithProps, Component, Html, Properties};

#[derive(Properties, Clone, PartialEq, Debug)]
pub struct Props {
    pub girds: [[usize; 4]; 4],
    // pub children: ChildrenWithProps<GameLine>,
}

#[derive(Debug)]
pub struct GameContainer {
    props: Props,
}

impl Component for GameContainer {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
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
                <div class="game-message">
                    <div class="lower">
                        <a class="retry-button">{"Try again"}</a>
                    </div>
                </div>
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
