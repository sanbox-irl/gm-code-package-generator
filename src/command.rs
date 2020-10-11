use heck::CamelCase;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use yy_typings::object_yy::EventType;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Command {
    pub command: String,
    title: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    category: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    enablement: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    icon: Option<String>,

    #[serde(skip)]
    pub event: EventType,
}
impl Command {
    pub fn new(event: EventType) -> Self {
        let nice_name = event.to_string();
        Self {
            command: format!("gmVfs.add{}", nice_name.to_camel_case()),
            title: nice_name.clone(),
            category: Some("Create".to_string()),
            enablement: Some(format!(
                "view == gmVfs && viewItem =~ /can{}Event/",
                nice_name.to_camel_case()
            )),
            icon: None,
            event,
        }
    }
}

impl PartialOrd for Command {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.event.partial_cmp(&other.event)
    }
}

impl Ord for Command {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.event.cmp(&other.event)
    }
}

pub fn create_command_lists() -> HashMap<String, Vec<Command>> {
    let event_names = [
        "Create", "Destroy", "CleanUp", "Step", "Alarm", "Draw", "Other",
    ];

    event_names
        .iter()
        .map(|name| {
            let mut values = (0..200)
                .filter_map(|i| EventType::parse_filename(name, i).ok().map(Command::new))
                .collect::<Vec<_>>();

            values.sort();

            (name.to_string(), values)
        })
        .collect()
}

pub fn default_commands() -> Vec<Command> {
    serde_json::from_str(
        r#"[
    {
        "command": "gmVfs.reloadWorkspace",
        "title": "Reload",
        "icon": "$(refresh)"
    },
    {
        "command": "gmVfs.createScript",
        "title": "New Script",
        "category": "Create",
        "icon": "$(new-file)"
    },
    {
        "command": "gmVfs.createObject",
        "title": "New Object",
        "category": "Create",
        "icon": "$(symbol-constructor)"
    },
    {
        "command": "gmVfs.createFolder",
        "title": "New Folder",
        "category": "Create",
        "icon": "$(new-folder)"
    },
    {
        "command": "gmVfs.deleteFolder",
        "title": "Delete",
        "category": "Delete"
    },
    {
        "command": "gmVfs.deleteResource",
        "title": "Delete",
        "category": "Delete"
    },
    {
        "command": "gmVfs.renameResource",
        "title": "Rename",
        "category": "Delete"
    },
    {
        "command": "gmVfs.renameFolder",
        "title": "Rename",
        "category": "Delete"
    },
    {
        "command": "gmVfs.deleteEvent",
        "title": "Delete",
        "category": "Delete"
    }]"#,
    )
    .unwrap()
}
