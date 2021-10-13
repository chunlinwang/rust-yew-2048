use yew::prelude::*;
use yew::services::ConsoleService;
use yew::{classes, html};

#[derive(Debug)]
pub struct GameCase {
    props: Props,
}

#[derive(Properties, Clone, PartialEq, Debug)]
pub struct Props {
    pub case: usize,
    pub x: usize,
    pub y: usize,
}

impl Component for GameCase {
    type Message = ();
    type Properties = Props;
    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        GameCase {
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
        let style = format!("tile tile-{} tile-position-{}-{} tile-new", self.props.case, self.props.x, self.props.y);
        let no = self.props.case == 0 ;
        html! {
            <div class=style hidden=no> { self.props.case } </div>
        }
    }
}
