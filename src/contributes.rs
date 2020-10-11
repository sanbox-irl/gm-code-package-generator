use crate::{Command, CommandContext, Menus};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use yy_typings::object_yy::{EventType, OtherEvent};

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
                "Alarm" => {
                    let id = menus.add_submenu_toplevel("Alarm", 4, None);

                    for (i, c) in c.into_iter().enumerate() {
                        menus.add_context_submenu(&id, CommandContext::new(&c.command, i));
                        commands.push(c);
                    }
                }
                "Draw" => {
                    let id = menus.add_submenu_toplevel("Drawn", 5, None);

                    for (i, c) in c.into_iter().enumerate() {
                        menus.add_context_submenu(&id, CommandContext::new(&c.command, i));
                        commands.push(c);
                    }
                }
                "Other" => {
                    let other_id = menus.add_submenu_toplevel("Other", 5, None);
                    let async_id = menus.add_submenu_toplevel("Asynchronous", 6, None);

                    let views = menus.add_submenu_submenu(&other_id, "Views", 2, None);
                    let user_events = menus.add_submenu_submenu(&other_id, "User Events", 11, None);

                    for (i, c) in c.into_iter().enumerate() {
                        match c.event {
                            EventType::Other(ev) => match ev {
                                OtherEvent::OutsideView(_) => {
                                    menus.add_context_submenu(
                                        &views,
                                        CommandContext::new(&c.command, i),
                                    );
                                    commands.push(c);
                                }
                                OtherEvent::IntersectView(_) => {
                                    menus.add_context_submenu(
                                        &views,
                                        CommandContext::new(&c.command, i),
                                    );
                                    commands.push(c);
                                }
                                OtherEvent::UserEvent(_) => {
                                    menus.add_context_submenu(
                                        &user_events,
                                        CommandContext::new(&c.command, i),
                                    );
                                    commands.push(c);
                                }
                                _ => {
                                    menus.add_context_submenu(
                                        &other_id,
                                        CommandContext::new(&c.command, i),
                                    );
                                    commands.push(c);
                                }
                            },
                            EventType::Async(_) => {
                                menus.add_context_submenu(
                                    &async_id,
                                    CommandContext::new(&c.command, i),
                                );
                                commands.push(c);
                            }
                            _ => unimplemented!(),
                        }
                    }
                }
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
