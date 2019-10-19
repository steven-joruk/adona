use crate::database::Database;

use gtk::prelude::*;
use relm::{connect, Relm, Widget};
use relm_derive::{widget, Msg};

// TODO: Determine this using the `dirs` module.
const DATABASE_PATH: &str = "res/wow-classic.json";
const ADDON_SUBMISSION_URL: &str =
    "https://github.com/steven-joruk/adona/issues/new?assignees=&labels=addon+submission&template=addon-submission.md&title=Addon+submission";

#[derive(Msg)]
pub enum Msg {
    InstallAddon,
    SelectionChanged(gtk::TreeSelection),
    SubmitAddon,
    UpdateDatabase,
}

pub struct Model {
    has_selection: bool,
}

#[widget]
impl Widget for AvailableWidget {
    fn model(_: &Relm<Self>, _: ()) -> Model {
        Model {
            has_selection: false,
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::InstallAddon => println!("Install"),
            Msg::SelectionChanged(selection) => match selection.get_selected() {
                Some((tree_model, iter)) => {
                    self.model.has_selection = true;
                }
                None => self.model.has_selection = false,
            },
            Msg::SubmitAddon => {
                if let Err(e) = webbrowser::open(ADDON_SUBMISSION_URL) {
                    // TODO: It might be better to show an error dialog here,
                    // but I'd need to pass in the parent window for it to be
                    // positioned correctly.
                    eprintln!("Failed to open the browser: {}", e);
                }
            }
            Msg::UpdateDatabase => {
                println!("Update database");
                self.update_tree_view_model();
            }
        }
    }

    // TODO:
    // - Creating a new model for every modification might be terrible.
    // - Implement this as TryFrom?
    fn update_tree_view_model(&mut self) {
        let store = gtk::ListStore::new(&[String::static_type(), String::static_type()]);

        let available = match Database::load(DATABASE_PATH) {
            Ok(a) => a,
            Err(_) => {
                // TODO: Show an error dialog.
                return;
            }
        };

        for addon in available {
            store.insert_with_values(None, &[0, 1], &[&addon.name, &addon.description]);
        }

        self.tree_view.set_model(Some(&store));
    }

    fn init_view(&mut self) {
        let cell = gtk::CellRendererText::new();

        let column = gtk::TreeViewColumn::new();
        column.pack_start(&cell, true);
        column.set_title("Addon");
        column.add_attribute(&cell, "text", 0);
        self.tree_view.append_column(&column);

        let column = gtk::TreeViewColumn::new();
        column.pack_start(&cell, true);
        column.set_title("Description");
        column.add_attribute(&cell, "text", 1);
        self.tree_view.append_column(&column);

        self.tree_view.set_headers_clickable(true);
        self.tree_view
            .set_grid_lines(gtk::TreeViewGridLines::Horizontal);

        self.update_tree_view_model();
    }

    view! {
        gtk::Box {
            border_width: super::PADDING as u32,
            orientation: gtk::Orientation::Vertical,
            spacing: super::PADDING,
            gtk::Box {
                orientation: gtk::Orientation::Horizontal,
                spacing: super::PADDING,
                gtk::Button {
                    label: "Install",
                    sensitive: self.model.has_selection,
                    clicked => Msg::InstallAddon,
                },
                // TODO: Add a 'last updated' tooltip
                gtk::Button {
                    label: "Update database",
                    sensitive: true,
                    tooltip_text: Some("Test"),
                    clicked => Msg::UpdateDatabase,
                },
                // TODO: Combo box for category
                #[name="search_available"]
                gtk::SearchEntry {
                    placeholder_text: Some("Addon name"),
                    property_width_request: 300,
                    child: {
                        pack_type: gtk::PackType::End,
                    }
                },
                gtk::Button {
                    label: "Submit an addon",
                    clicked => Msg::SubmitAddon,
                    child: {
                        pack_type: gtk::PackType::End,
                    },
                },
            },
            #[name="tree_view"]
            gtk::TreeView {
                search_entry: Some(&search_available),
                search_column: 0,
                child: {
                    expand: true,
                },
                selection.changed(selection) => Msg::SelectionChanged(selection.clone()),
            },
        },
    }
}