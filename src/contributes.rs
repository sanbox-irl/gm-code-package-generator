use crate::{Command, CommandContext, Menus};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contributes {
    commands: Vec<Command>,
    #[serde(flatten)]
    menus: Menus,
    task_definitions: Vec<serde_json::Value>,
    configuration: serde_json::Value,
    views: serde_json::Value,
}

impl Contributes {
    pub fn new(input: HashMap<String, Vec<Command>>) -> Self {
        let mut commands = super::command::default_commands();
        let mut menus = Menus::new();

        for (key, mut c) in input {
            match key.as_ref() {
                "Create" => {
                    assert_eq!(c.len(), 1);
                    let command = c.remove(0);
                    menus.add_context_toplevel(CommandContext::new(&command.command, 0));
                    commands.push(command);
                }
                "Destroy" => {
                    assert_eq!(c.len(), 1);
                    let command = c.remove(0);
                    menus.add_context_toplevel(CommandContext::new(&command.command, 1));
                    commands.push(command);
                }
                "CleanUp" => {
                    assert_eq!(c.len(), 1);
                    let command = c.remove(0);
                    menus.add_context_toplevel(CommandContext::new(&command.command, 2));
                    commands.push(command);
                }
                "Step" => {
                    let id = menus.add_submenu_toplevel("Step", 3, None);

                    for (i, c) in c.into_iter().enumerate() {
                        menus.add_context_submenu(&id, CommandContext::new(&c.command, i));
                        commands.push(c);
                    }
                }
                "Alarm" => {}
                "Draw" => {}
                "Other" => {}
                _ => unimplemented!(),
            }
        }

        Self {
            commands,
            menus,
            task_definitions: vec![serde_json::json!(
                {
                    "type": "adam",
                    "required": [
                        "task"
                    ],
                    "properties": {
                        "task": {
                            "type": "string",
                            "description": "The adam task to run"
                        }
                    }
                }
            )],
            configuration: serde_json::json!(
                {
                    "title": "Gm Code",
                    "properties": {
                        "gmCode.overrideServerPath": {
                            "description": "Override path to the gm-code backend server. Used to develop the extension.",
                            "type": [
                                "null",
                                "string"
                            ]
                        }
                    }
                }
            ),
            views: serde_json::json!(
                {
                    "explorer": [
                        {
                            "id": "gmVfs",
                            "name": "Gm Code"
                        }
                    ]
                }
            ),
        }
    }
}
/*

        "configuration": {
        },
        "views": {
            "explorer": [
                {
                    "id": "gmVfs",
                    "name": "Gm Code"
                }
            ]
        },
*/
