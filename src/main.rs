mod contributes;
pub use contributes::*;

mod command;
pub use command::Command;

mod menus;
pub use menus::{CommandContext, Context, Menus, SubMenuContext};

mod ts_generation;
pub use ts_generation::TsGenerated;

fn main() {
    let output = command::create_command_lists();

    let contributes = Contributes::new(output);
    let generated = TsGenerated::new(&contributes.commands);

    std::fs::write(
        "contributes.json",
        serde_json::to_string_pretty(&contributes).unwrap(),
    )
    .unwrap();

    std::fs::write("ts_generated.ts", generated.to_string()).unwrap();
}
