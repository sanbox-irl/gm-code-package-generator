mod contributes;
pub use contributes::*;

mod command;
pub use command::Command;

mod menus;
pub use menus::{CommandContext, Context, Menus, SubMenuContext};

fn main() {
    let output = command::create_command_lists();

    let contributes = Contributes::new(output);

    println!("{}", serde_json::to_string_pretty(&contributes).unwrap());
}
