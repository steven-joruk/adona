use super::available_widget::AvailableWidget;
use super::installed_widget::InstalledWidget;

use gtk::prelude::*;
use gtk::Inhibit;
use relm::{Relm, Widget};
use relm_derive::{widget, Msg};

pub struct Model;

#[derive(Msg)]
pub enum Msg {
    Quit,
}

#[widget]
impl Widget for Win {
    fn model(_: &Relm<Self>, _: ()) -> Model {
        Model
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::Quit => gtk::main_quit(),
        }
    }

    view! {
        gtk::Window {
            title: "Adona",
            border_width: super::PADDING as u32,
            property_width_request: 600,
            property_height_request: 450,
            gtk::Notebook {
                InstalledWidget {
                    child: {
                        tab_label: Some("Installed"),
                    }
                },
                AvailableWidget {
                    child: {
                        tab_label: Some("Available"),
                    },
                },
            },
            delete_event(_, _) => (Msg::Quit, Inhibit(false)),
        },
    }
}
