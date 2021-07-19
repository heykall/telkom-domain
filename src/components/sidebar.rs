use yew::prelude::*;
use css_in_rust::Style;

pub struct Sidebar {
  style: Style,
}

pub enum Msg {}

impl Component for Sidebar {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        let style = match Style::create("Sidebar", include_str!("sidebar.scss")) {
            Ok(style) => style,
            Err(error) => {
                panic!("An error occured while creating the style: {}", error);
            }
        };

        Sidebar {
          style,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
          <div>
            <nav class="sidebar card py-2 mb-4">
              <ul class="nav flex-column" id="nav_accordion">

                <li class="nav-item">
                  <a class="nav-link" href="#"> {"Link name"} </a>
                </li>

                <li class="nav-item">
                  <a class="nav-link" data-bs-toggle="collapse" data-bs-target="#menu_item1" href="#"> {"Submenu links"} <i
                      class="bi small bi-caret-down-fill"></i> </a>
                  <ul id="menu_item1" class="submenu collapse" data-bs-parent="#nav_accordion">
                    <li><a class="nav-link" href="#">{"Submenu item 1"} </a></li>
                    <li><a class="nav-link" href="#">{"Submenu item 2"} </a></li>
                    <li><a class="nav-link" href="#">{"Submenu item 3"} </a> </li>
                  </ul>
                </li>

                <li class="nav-item">
                  <a class="nav-link" data-bs-toggle="collapse" data-bs-target="#menu_item2" href="#"> 
                                  {"More menus"} <i
                      class="bi small bi-caret-down-fill"></i> </a>
                  <ul id="menu_item2" class="submenu collapse" data-bs-parent="#nav_accordion">
                    <li><a class="nav-link" href="#">{"Submenu item 4"} </a></li>
                    <li><a class="nav-link" href="#">{"Submenu item 5"} </a></li>
                    <li><a class="nav-link" href="#">{"Submenu item 6"} </a></li>
                    <li><a class="nav-link" href="#">{"Submenu item 7"} </a></li>
                  </ul>
                </li>
                <li class="nav-item">
                  <a class="nav-link" href="#"> {"Another page"} </a>
                </li>
                <li class="nav-item">
                  <a class="nav-link" href="#"> {"Demo link"} </a>
                </li>
                <li class="nav-item">
                  <a class="nav-link" href="#"> {"Menu item"} </a>
                </li>
                <li class="nav-item">
                  <a class="nav-link" href="#"> {"Something"} </a>
                </li>
                <li class="nav-item">
                  <a class="nav-link" href="#"> {"Other link"} </a>
                </li>
              </ul>
            </nav>
          </div>
        }
    }
}