mod addon;
mod database;
mod error;
mod settings;
mod ui;
mod zip_archive;

use relm::Widget;

fn main() -> error::Result<()> {
    ui::Win::run(()).expect("Win::run failed");

    Ok(())
}
