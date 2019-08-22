use yew::prelude::*;
use yew_router::prelude::*;
use std::usize;
use yew::Properties;
use yew_router::route::RouteInfo;
use std::collections::HashMap;
use std::str::FromStr;
use yew_router::path_matcher::FromMatchesError;
use yew_router::path_matcher::FromMatches;

pub struct BModel {
    number: Option<usize>,
    sub_path: Option<String>,
    router: Box<dyn Bridge<SimpleRouterAgent>>
}

#[derive(PartialEq, Properties)]
pub struct Props {
    #[props(required)]
    number: Option<usize>, // TODO remove these options
    #[props(required)]
    sub_path: Option<String>
}

pub enum Msg {
    Navigate(Vec<Msg>), // Navigate after performing other actions
    Increment,
    Decrement,
    UpdateSubpath(String),
    HandleRoute(SimpleRouteInfo)
}


impl Component for BModel {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, mut link: ComponentLink<Self>) -> Self {

        let callback = link.send_back(|route: SimpleRouteInfo| Msg::HandleRoute(route));
        let mut router = SimpleRouterAgent::bridge(callback);

        router.send(RouterRequest::GetCurrentRoute);

        BModel {
            number: props.number,
            sub_path: props.sub_path,
            router
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Navigate(msgs) => {
                // Perform the wrapped updates first
                for msg in msgs{
                    self.update(msg);
                }

                // The path dictating that this component be instantiated must be provided
                let mut path_segments = vec!["b".into()];
                if let Some(ref sub_path) = self.sub_path {
                    path_segments.push(sub_path.clone())
                }

                let fragment: Option<String> = self.number.map(|x: usize | x.to_string());

                let route = RouteInfo {
                    path_segments,
                    query: None,
                    fragment,
                    state: (),
                };

                // Don't tell the router to alert its subscribers,
                // because the changes made here only affect the current component,
                // so mutation might as well be contained to the core component update loop
                // instead of being sent through the router.
                self.router.send(RouterRequest::ChangeRouteNoBroadcast(route));
                true
            }
            Msg::HandleRoute(route) => {
                // Instead of each component selecting which parts of the path are important to it,
                // it is also possible to match on the `route.to_route_string().as_str()` once
                // and create enum variants representing the different children and pass them as props.
                self.sub_path = route.path_segments.get(1).map(String::clone);
                self.number = route.fragment.and_then(|x| usize::from_str_radix(&x, 10).ok());

                true
            }
            Msg::Increment => {
                let n = if let Some(number) = self.number{
                    number + 1
                } else {
                    1
                };
                self.number = Some(n);
                true
            }
            Msg::Decrement => {
                let n: usize = if let Some(number) = self.number{
                    if number > 0 {
                        number - 1
                    } else {
                        number
                    }
                } else {
                    0
                };
                self.number = Some(n);
                true
            }
            Msg::UpdateSubpath(path) => {
                self.sub_path = Some(path);
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }
}


impl Renderable<BModel> for BModel {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <div>
                    { self.display_number() }
                    <button onclick=|_| Msg::Navigate(vec![Msg::Increment]),>{ "Increment" }</button>
                    <button onclick=|_| Msg::Navigate(vec![Msg::Decrement]),>{ "Decrement" }</button>
                </div>

                { self.display_subpath_input() }

            </div>
        }
    }
}



impl FromMatches for Props {

    fn from_matches(matches: &HashMap<String, String>) -> Result<Self, FromMatchesError> {

        let number = matches.get(&"number".to_string()).map(|n: &String| usize::from_str(&n).map_err(|_| FromMatchesError::UnknownErr) ).transpose()?;

        let props = Props {
            number,
            sub_path: matches.get(&"sub_path".to_string()).cloned()
        };
        Ok(props)
    }
}

impl BModel {
    fn display_number(&self) -> String {
        if let Some(number) = self.number {
            format!("Number: {}", number)
        } else {
            format!("Number: None")
        }
    }
    fn display_subpath_input(&self) -> Html<BModel> {
        let sub_path = self.sub_path.clone();
        html! {
            <input
                placeholder="subpath",
                value=sub_path.unwrap_or("".into()),
                oninput=|e| Msg::Navigate(vec![Msg::UpdateSubpath(e.value)]),
                />
        }
    }
}