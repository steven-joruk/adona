use gtk::prelude::*;
use relm::{Relm, Widget};
use relm_derive::{widget, Msg};

#[derive(Msg)]
pub enum Msg {
    UninstallAddon,
    UpdateAddon,
    SelectionChanged(gtk::TreeSelection),
}

pub struct Model {
    selected_addon_name: Option<String>,
}

#[widget]
impl Widget for InstalledWidget {
    fn model(_: &Relm<Self>, _: ()) -> Model {
        Model {
            selected_addon_name: None,
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::UninstallAddon => {
                if let Some(name) = &self.model.selected_addon_name {
                    println!("TODO: Uninstall {}", name);
                };
            }
            Msg::UpdateAddon => {
                if let Some(name) = &self.model.selected_addon_name {
                    println!("TODO: Update {}", name);
                };
            }
            Msg::SelectionChanged(selection) => match selection.get_selected() {
                Some((tree_model, iter)) => {
                    self.model.selected_addon_name = tree_model.get_value(&iter, 0).get();
                }
                None => self.model.selected_addon_name = None,
            },
        }
    }

    fn update_tree_view_model(&mut self) {
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

        self.tree_view.set_model(Some(&store));
    }

    fn init_view(&mut self) {
        let cell = gtk::CellRendererText::new();

        let column = gtk::TreeViewColumn::new();
        column.pack_start(&cell, true);
        column.set_title("Addon");
        column.add_attribute(&cell, "text", 0);
        column.set_expand(true);
        self.tree_view.append_column(&column);

        let column = gtk::TreeViewColumn::new();
        column.pack_start(&cell, true);
        column.set_title("Status");
        column.add_attribute(&cell, "text", 1);
        self.tree_view.append_column(&column);

        let column = gtk::TreeViewColumn::new();
        column.pack_start(&cell, true);
        column.set_title("Version");
        column.add_attribute(&cell, "text", 2);
        self.tree_view.append_column(&column);

        let column = gtk::TreeViewColumn::new();
        column.pack_start(&cell, true);
        column.set_title("Game Version");
        column.add_attribute(&cell, "text", 3);
        self.tree_view.append_column(&column);

        let column = gtk::TreeViewColumn::new();
        column.pack_start(&cell, true);
        column.set_title("Support");
        column.add_attribute(&cell, "text", 4);
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
                    label: "Update",
                    sensitive: self.model.selected_addon_name.is_some(),
                    clicked => Msg::UpdateAddon,
                },
                gtk::Button {
                    label: "Uninstall",
                    sensitive: self.model.selected_addon_name.is_some(),
                    clicked => Msg::UninstallAddon,
                },
                #[name="search_entry"]
                gtk::SearchEntry {
                    placeholder_text: Some("Addon name"),
                    property_width_request: 300,
                    child: {
                        pack_type: gtk::PackType::End,
                    },
                },
            },
            #[name="tree_view"]
            gtk::TreeView {
                search_entry: Some(&search_entry),
                search_column: 0,
                child: {
                    expand: true,
                },
                selection.changed(selection) => Msg::SelectionChanged(selection.clone()),
            },
        },
    }
}
