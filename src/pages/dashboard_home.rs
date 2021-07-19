use yew::prelude::*;

use crate::components::{
    content::Content,
		sidebar::Sidebar,
};

pub struct DashboardHome {}

pub enum Msg {}

impl Component for DashboardHome {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        DashboardHome {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <> 
              <body class="bg-light">

                    <header class="section-header py-3">
                        <div class="container">
                            <h2>{"Untuk Nav Bar juga"} </h2>
                        </div>
                    </header> 
                  // <!-- section-header.// -->

                <div class="container-fluid">

                  <section class="section-content py-3">
                    <div class="row min-vh-100">
                      <aside class="col-lg-2">
                        <Sidebar/>
                      </aside>

                      <main class="col-lg-9 container">
                                  {"Ini dari Folder Components"}
                                  <Content/>
                      </main>
                    </div>
                  </section>

                </div>
                  // <!-- container   -->

              </body>
            </>
        }
    }
}