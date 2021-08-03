use yew::prelude::*;
use yew_router::components::RouterAnchor;
use crate::app::AppRoute;
use super::quickstart::Quickstart;
use super::tab_settings::TabSettings;

pub enum Content {
    Quickstart,
    Settings
}

pub struct Settings {
    content: Content,
    link: ComponentLink<Self>
}

pub enum Msg {
    ChangeContent(Content)
}

impl Component for Settings {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Settings {
            content: Content::Quickstart,
            link
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ChangeContent(content) => {
                self.content = content;
                true
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        type Anchor = RouterAnchor<AppRoute>;
        html! {
            <div
                class="py-5 px-4 m-auto"
                style="max-width: 1048px;"
            >
                <Anchor
                    route=AppRoute::ApisHome
                    classes="text-decoration-none domain-link-dark"
                >
                    <i class="bi bi-arrow-left me-2"></i>
                    {"Back to Apis"}
                </Anchor>

                <div
                    class="d-flex mb-5 mt-3"
                >
                    <div
                        style="flex: 0 0 auto; width: 64px; height: 64px; background-color: #eff0f2;"
                        class="d-flex justify-content-center align-items-center rounded me-4"
                    >
                        <i class="bi bi-server fs-3"></i>
                    </div>

                    <div
                        class="d-flex flex-column"
                    >
                        <h2>{"Testing Name"}</h2>
                        <div
                            class="text-muted"
                        >
                            <span
                                class="me-4"
                            >
                                {"Custom API"}
                            </span>
                            <span>
                                {"Identifier"}
                            </span>
                            <span
                                class="rounded ms-2"
                                style="
                                    background-color: #eff0f2;
                                    white-space: nowrap;
                                    text-overflow: ellipsis;
                                    overflow: hidden;
                                    font-size: 14px;
                                    padding: 2px 6px;
                                    font-family: 'Roboto Mono', monospace;
                                "
                            >
                                {"https://test-api/"}
                            </span>
                        </div>
                    </div>
                </div>

                <div
                    class="mb-4"
                >
                    <ul class="nav nav-tabs">
                        <li
                            onclick=self.link.callback(|_| Msg::ChangeContent(Content::Quickstart))
                            class="nav-item"
                        >
                        <a
                            // class="nav-link active"
                            class={
                                match self.content {
                                    Content::Quickstart => "nav-link active",
                                    _ => "nav-link"
                                }
                            }
                            aria-current="page"
                            href="#"
                        >
                            {"Quick Start"}</a>
                        </li>
                        <li
                            onclick=self.link.callback(|_| Msg::ChangeContent(Content::Settings))
                            class="nav-item">
                        <a
                            // class="nav-link"
                            class={
                                match self.content {
                                    Content::Settings => "nav-link active",
                                    _ => "nav-link"
                                }
                            }
                            href="#">{"Settings"}</a>
                        </li>
                        <li class="nav-item">
                        <a class="nav-link" href="#">{"Permissions"}</a>
                        </li>
                        <li class="nav-item">
                            <a class="nav-link" href="#">{"Machine to Machine Applications"}</a>
                        </li>
                        <li class="nav-item">
                            <a class="nav-link" href="#">{"Test"}</a>
                        </li>
                    </ul>
                </div>

                // <Quickstart/>
                // <TabSettings/>

                {
                    match self.content {
                        Content::Quickstart => html! { <Quickstart/> },
                        Content::Settings => html! { <TabSettings/> }
                    }
                }
            </div>
        }
    }
}
