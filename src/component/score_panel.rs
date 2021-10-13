use yew::prelude::*;
use yew::services::ConsoleService;
use yew::{html, Callback};

pub struct ScorePanel{
    props: Props,
}

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub score: u64,
    pub best: u64,
}

impl Component for ScorePanel {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        ScorePanel { props }
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
        html! {
            <div class="scores-container">
                <div class="score-container">{self.props.score}</div>
                // <div class="best-container">{self.props.best}</div>
            </div>
        }
    }
}
