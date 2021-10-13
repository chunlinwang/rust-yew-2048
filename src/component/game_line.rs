use std::io;
use yew::prelude::*;
use yew::services::ConsoleService;
use crate::game::Matrix;
use crate::component::game_case::GameCase;

#[derive(Debug)]
pub struct GameLine {
    props: Props,
}

#[derive(Properties, Clone, PartialEq, Debug)]
pub struct Props {
    pub line: [usize; 4],
    pub y: usize,
}

impl Component for GameLine {
    type Message = ();
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
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
        html! {
            <div class="grid-row">
                <GameCase case={self.props.line[0]} y={self.props.y} x={1}/>
                <GameCase case={self.props.line[1]} y={self.props.y} x={2}/>
                <GameCase case={self.props.line[2]} y={self.props.y} x={3}/>
                <GameCase case={self.props.line[3]} y={self.props.y} x={4}/>
            </div>
        }
    }
}
