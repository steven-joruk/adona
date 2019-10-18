mod addon;
mod database;
mod error;
mod settings;

use database::Database;
use error::Result;

use gtk::prelude::*;
use gtk::Inhibit;
use relm::{connect, connect_stream, Relm, Widget};
use relm_derive::{widget, Msg};

const PADDING: i32 = 10;
// TODO: Determine this using the `dirs` module.
const DATABASE_PATH: &str = "res/wow-classic.json";
const ADDON_SUBMISSION_URL: &str =
    "https://github.com/steven-joruk/adona/issues/new?assignees=&labels=addon+submission&template=addon-submission.md&title=Addon+submission";

pub struct Model {
    // TODO: Remove this if/when relm supports connecting chil signals inside
    // `view!`, specifically TreeSelection's `changed` signal.
    relm: Relm<Win>,
    available_addon_is_selected: bool,
    installed_addon_is_selected: bool,
}

impl Model {
    pub fn new(relm: Relm<Win>) -> Model {
        Model {
            relm,
            available_addon_is_selected: false,
            installed_addon_is_selected: false,
        }
    }
}

#[derive(Msg)]
pub enum Msg {
    AvailableViewSelectionChanged,
    DeleteAddon,
    InstallAddon,
    InstalledViewSelectionChanged,
    Quit,
    SubmitAddon,
    Update,
    UpdateDatabase,
}

#[widget]
impl Widget for Win {
    fn model(relm: &Relm<Self>, _: ()) -> Model {
        Model::new(relm.clone())
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::AvailableViewSelectionChanged => {
                let selection = self.available_view.get_selection();
                match selection.get_selected() {
                    Some((tree_model, iter)) => {
                        self.model.available_addon_is_selected = true;
                    }
                    None => self.model.available_addon_is_selected = false,
                }
            }
            Msg::DeleteAddon => {
                println!("TODO: Delete the selected addon");
            }
            Msg::InstallAddon => {
                println!("TODO: Install the selected addon");
            }
            Msg::InstalledViewSelectionChanged => {
                let selection = self.installed_view.get_selection();
                match selection.get_selected() {
                    Some((tree_model, iter)) => {
                        self.model.installed_addon_is_selected = true;
                    }
                    None => self.model.installed_addon_is_selected = false,
                }
            }
            Msg::Quit => gtk::main_quit(),
            Msg::SubmitAddon => {
                webbrowser::open(ADDON_SUBMISSION_URL).unwrap();
            }
            Msg::Update => println!("Update addon"),
            Msg::UpdateDatabase => {
                self.update_available_tree_view();
            }
        }
    }

    // TODO:
    // - Creating a new model for every modification might be terrible.
    // - Implement this as TryFrom?
    fn update_installed_tree_view(&mut self) {
        let store = gtk::ListStore::new(&[
            String::static_type(),
            String::static_type(),
            String::static_type(),
            String::static_type(),
            String::static_type(),
        ]);

        store.insert_with_values(
            None,
            &[0, 1, 2, 3, 4],
            &[&"Questie", &"Up to date", &"0.1", &"7.0", &"Patreon"],
        );

        self.installed_view.set_model(Some(&store));
    }

    // TODO:
    // - Move this and the 'installed' view to separate widgets.
    // - Figure out how to make the headers stand out
    // - Add column padding/spacing
    // - Add icons
    // - Make columns sortable
    fn init_installed_tree_view(&mut self) {
        let selection = self.installed_view.get_selection();
        connect!(
            self.model.relm,
            selection,
            connect_changed(_),
            Msg::InstalledViewSelectionChanged
        );

        let cell = gtk::CellRendererText::new();

        let column = gtk::TreeViewColumn::new();
        column.pack_start(&cell, true);
        column.set_title("Addon");
        column.add_attribute(&cell, "text", 0);
        column.set_expand(true);
        self.installed_view.append_column(&column);

        let column = gtk::TreeViewColumn::new();
        column.pack_start(&cell, true);
        column.set_title("Status");
        column.add_attribute(&cell, "text", 1);
        self.installed_view.append_column(&column);

        let column = gtk::TreeViewColumn::new();
        column.pack_start(&cell, true);
        column.set_title("Version");
        column.add_attribute(&cell, "text", 2);
        self.installed_view.append_column(&column);

        let column = gtk::TreeViewColumn::new();
        column.pack_start(&cell, true);
        column.set_title("Game Version");
        column.add_attribute(&cell, "text", 3);
        self.installed_view.append_column(&column);

        let column = gtk::TreeViewColumn::new();
        column.pack_start(&cell, true);
        column.set_title("Support");
        column.add_attribute(&cell, "text", 4);
        self.installed_view.append_column(&column);

        self.installed_view.set_headers_clickable(true);
        self.installed_view
            .set_grid_lines(gtk::TreeViewGridLines::Horizontal);

        self.update_installed_tree_view();
    }

    // TODO:
    // - Creating a new model for every modification might be terrible.
    // - Implement this as TryFrom?
    fn update_available_tree_view(&mut self) {
        let store = gtk::ListStore::new(&[String::static_type(), String::static_type()]);

        // TODO: No unwraps
        let available = Database::load(DATABASE_PATH).unwrap();

        for addon in available {
            store.insert_with_values(None, &[0, 1], &[&addon.name, &addon.description]);
        }

        self.available_view.set_model(Some(&store));
    }

    fn init_available_tree_view(&mut self) {
        let selection = self.available_view.get_selection();
        connect!(
            self.model.relm,
            selection,
            connect_changed(_),
            Msg::AvailableViewSelectionChanged
        );

        let cell = gtk::CellRendererText::new();

        let column = gtk::TreeViewColumn::new();
        column.pack_start(&cell, true);
        column.set_title("Addon");
        column.add_attribute(&cell, "text", 0);
        column.set_reorderable(true);
        self.available_view.append_column(&column);

        let column = gtk::TreeViewColumn::new();
        column.pack_start(&cell, true);
        column.set_title("Description");
        column.add_attribute(&cell, "text", 1);
        self.available_view.append_column(&column);

        self.available_view.set_headers_clickable(true);
        self.available_view
            .set_grid_lines(gtk::TreeViewGridLines::Horizontal);

        self.update_available_tree_view();
    }

    fn init_view(&mut self) {
        self.init_installed_tree_view();
        self.init_available_tree_view();
    }

    view! {
        gtk::Window {
            title: "Adona",
            border_width: PADDING as u32,
            property_width_request: 600,
            property_height_request: 450,
            gtk::Notebook {
                gtk::Box {
                    child: {
                        tab_label: Some("Installed"),
                    },
                    border_width: PADDING as u32,
                    orientation: gtk::Orientation::Vertical,
                    spacing: PADDING,
                    gtk::Box {
                        orientation: gtk::Orientation::Horizontal,
                        spacing: PADDING,
                        gtk::Button {
                            label: "Update",
                            sensitive: self.model.installed_addon_is_selected,
                            clicked => Msg::Update,
                        },
                        gtk::Button {
                            label: "Delete",
                            sensitive: self.model.installed_addon_is_selected,
                            clicked => Msg::DeleteAddon,
                        },
                        #[name="search_installed"]
                        gtk::SearchEntry {
                            placeholder_text: Some("Addon name"),
                            property_width_request: 300,
                            child: {
                                pack_type: gtk::PackType::End,
                            },
                        },
                    },
                    #[name="installed_view"]
                    gtk::TreeView {
                        search_entry: Some(&search_installed),
                        search_column: 0,
                        child: {
                            expand: true,
                        },
                    },
                },
                gtk::Box {
                    child: {
                        tab_label: Some("Available"),
                    },
                    border_width: PADDING as u32,
                    orientation: gtk::Orientation::Vertical,
                    spacing: PADDING,
                    gtk::Box {
                        orientation: gtk::Orientation::Horizontal,
                        spacing: PADDING,
                        gtk::Button {
                            label: "Install",
                            sensitive: self.model.available_addon_is_selected,
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
                    #[name="available_view"]
                    gtk::TreeView {
                        search_entry: Some(&search_available),
                        search_column: 0,
                        child: {
                            expand: true,
                        },
                    },
                },
            },
            delete_event(_, _) => (Msg::Quit, Inhibit(false)),
        },
    }
}

fn main() -> Result<()> {
    Win::run(()).unwrap();

    Ok(())
}
