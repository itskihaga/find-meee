use yew::prelude::*;
use yew_router::{agent::RouteRequest, prelude::*};
use crate::repository::{create_room};
use crate::components::text_input::{Input};

use crate::switch::AppRoute;

pub struct Home {
    state: State,
    props: Props
}

pub enum State {
    Init {
        on_submit: Callback<String>
    },
}

pub enum Msg {
    CreateRoom(String)
}

#[derive(Properties,Clone)]
pub struct Props {
    pub on_error: Callback<()>
}
impl Component for Home {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            state: State::Init {
                on_submit: link.callback(Msg::CreateRoom)
            },
            props
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::CreateRoom(name) => {
                let on_error = self.props.on_error.clone();
                let id = create_room(
                    &name,
                    Box::new(|| {}),
                    Box::new(move || on_error.clone().emit(()))
                );
                let route = AppRoute::Room(id);
                let mut dispatcher = RouteAgentDispatcher::new();
                dispatcher.send(RouteRequest::ChangeRoute(route.into_route()));
            },
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        match &self.state {
            State::Init { on_submit } => {
                html! {
                    <div>
                        <h2>{ "Roll Role" }</h2>
                        <p>{"This app determines your role at random by rolling. "}</p>
                        <Input on_submit=on_submit value="host" button="Join"/>
                    </div>
                }
            },
        }
        
    }
}
