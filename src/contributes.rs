use crate::{Command, CommandContext, Menus};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use yy_typings::object_yy::{DrawEvent, EventType, OtherEvent, Stage};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contributes {
    pub commands: Vec<Command>,
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
                    menus.add_context_toplevel(CommandContext::new(&command.command, "create", 0));
                    commands.push(command);
                }
                "Destroy" => {
                    assert_eq!(c.len(), 1);
                    let command = c.remove(0);
                    menus.add_context_toplevel(CommandContext::new(&command.command, "create", 1));
                    commands.push(command);
                }
                "CleanUp" => {
                    assert_eq!(c.len(), 1);
                    let command = c.remove(0);
                    menus.add_context_toplevel(CommandContext::new(&command.command, "create", 2));
                    commands.push(command);
                }
                "Step" => {
                    let id = menus.add_submenu_toplevel("Step", 3, None);

                    for (i, c) in c.into_iter().enumerate() {
                        menus
                            .add_context_submenu(&id, CommandContext::new(&c.command, "create", i));
                        commands.push(c);
                    }
                }
                "Alarm" => {
                    let id = menus.add_submenu_toplevel("Alarm", 4, None);

                    for (i, c) in c.into_iter().enumerate() {
                        menus
                            .add_context_submenu(&id, CommandContext::new(&c.command, "create", i));
                        commands.push(c);
                    }
                }
                "Draw" => {
                    let id = menus.add_submenu_toplevel("Draw", 5, None);

                    for (i, c) in c.into_iter().enumerate() {
                        let group = match c.event {
                            EventType::Draw(e) => match e {
                                DrawEvent::DrawGui(s) | DrawEvent::Draw(s) => match s {
                                    Stage::Main => "drawmain",
                                    Stage::Begin => "drawpost",
                                    Stage::End => "drawpost",
                                },
                                DrawEvent::PreDraw => "prepost",
                                DrawEvent::PostDraw => "prepost",
                                DrawEvent::WindowResize => "window",
                            },
                            _ => unimplemented!(),
                        };

                        menus.add_context_submenu(&id, CommandContext::new(&c.command, group, i));
                        commands.push(c);
                    }
                }
                "Other" => {
                    let other_id = menus.add_submenu_toplevel("Other", 5, None);
                    let async_id = menus.add_submenu_toplevel("Asynchronous", 6, None);

                    let views = menus.add_submenu_submenu(&other_id, "Views", 2, None);
                    let user_events = menus.add_submenu_submenu(&other_id, "User Events", 10, None);

                    let mut main_c = 0;
                    let mut views_c = 0;
                    let mut user_events_c = 0;
                    let mut async_c = 0;

                    for c in c {
                        match c.event {
                            EventType::Other(ev) => match ev {
                                OtherEvent::OutsideView(_) | OtherEvent::IntersectView(_) => {
                                    menus.add_context_submenu(
                                        &views,
                                        CommandContext::new(&c.command, "create", views_c),
                                    );
                                    commands.push(c);
                                    views_c += 1;
                                }
                                OtherEvent::UserEvent(_) => {
                                    menus.add_context_submenu(
                                        &user_events,
                                        CommandContext::new(&c.command, "create", user_events_c),
                                    );
                                    commands.push(c);
                                    user_events_c += 1;
                                }
                                _ => {
                                    menus.add_context_submenu(
                                        &other_id,
                                        CommandContext::new(&c.command, "create", main_c),
                                    );
                                    commands.push(c);
                                    main_c += 1;
                                    if main_c == 10 {
                                        main_c += 1;
                                    }
                                }
                            },
                            EventType::Async(_) => {
                                menus.add_context_submenu(
                                    &async_id,
                                    CommandContext::new(&c.command, "create", async_c),
                                );
                                commands.push(c);
                                async_c += 1;
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
