use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct App {
  counter: u16,
}

pub enum Msg {
  Click,
}

impl Component for App {
  type Message = Msg;
  type Properties = ();

  fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
    App { counter: 0 }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Msg::Click => {
        self.counter += 1;
        true
      }
    }
  }

  fn view(&self) -> Html<Self> {
    html! {
      <div>
        <p>
          {format!("Clicked {} times", self.counter)}
        </p>
        <button onclick=|_| Msg::Click>{"Click"}</button>
      </div>
    }
  }
}