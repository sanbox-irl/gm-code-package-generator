use heck::CamelCase;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const DEFAULT_NAVIGATION_MENU: &str = include_str!("../defaults/navigation_menu.json");
const DEFAULT_VIEW_ITEM_CONTEXT: &str = include_str!("../defaults/view_item_context.json");

#[derive(Debug, Serialize, Deserialize)]
pub struct Menus {
    menus: HashMap<String, Vec<Context>>,
    submenus: Vec<SubMenu>,
}

impl Menus {
    const NAVIGATION_KEY: &'static str = "view/title";
    const CONTEXT_KEY: &'static str = "view/title/context";

    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_context_toplevel(&mut self, cc: CommandContext) {
        let inner = self.menus.get_mut(Self::CONTEXT_KEY).unwrap();
        inner.push(Context::Command(cc));
    }

    pub fn add_context_submenu(&mut self, submenu: &str, cc: CommandContext) {
        let inner = self.menus.get_mut(submenu).unwrap();
        inner.push(Context::Command(cc));
    }

    pub fn add_submenu_toplevel(
        &mut self,
        submenu_name: &str,
        idx: usize,
        icon: Option<String>,
    ) -> String {
        let id = format!("gmVfs.{}", submenu_name.to_camel_case());
        self.submenus.push(SubMenu {
            id: id.clone(),
            label: submenu_name.to_string(),
            icon,
        });

        let inner = self.menus.get_mut(Self::CONTEXT_KEY).unwrap();
        inner.push(Context::SubMenu(SubMenuContext::new(
            &submenu_name.to_camel_case(),
            idx,
        )));

        self.menus.insert(id.clone(), vec![]);
        id
    }

    // pub fn add_submenu_submenu(&mut self, submenu: &str, sm: SubMenu, cc: SubMenuContext) {
    //     self.submenus.push(sm);

    //     let inner = self.menus.get_mut(submenu).unwrap();
    //     inner.push(Context::SubMenu(cc));
    // }
}

impl Default for Menus {
    fn default() -> Self {
        let mut menus: HashMap<String, Vec<Context>> = Default::default();

        let normal_commands: Vec<Context> = serde_json::from_str(DEFAULT_NAVIGATION_MENU).unwrap();
        menus.insert(Self::NAVIGATION_KEY.to_string(), normal_commands);

        let normal_commands: Vec<Context> =
            serde_json::from_str(DEFAULT_VIEW_ITEM_CONTEXT).unwrap();
        menus.insert(Self::CONTEXT_KEY.to_string(), normal_commands);

        Self {
            menus,
            submenus: vec![],
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Context {
    Command(CommandContext),
    SubMenu(SubMenuContext),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommandContext {
    command: String,
    when: String,
    group: String,
}

impl CommandContext {
    pub fn new(command_name: &str, idx: usize) -> Self {
        Self {
            command: command_name.to_string(),
            when: "view == gmVfs && viewItem =~ /objectItem/".to_string(),
            group: format!("create@{}", idx),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SubMenuContext {
    pub submenu: String,
    when: String,
    group: String,
}

impl SubMenuContext {
    pub fn new(submenu_name: &str, idx: usize) -> Self {
        Self {
            submenu: format!("gmVfs.{}", submenu_name),
            when: "view == gmVfs && viewItem =~ /objectItem/".to_string(),
            group: format!("create@{}", idx),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubMenu {
    id: String,
    label: String,
    
    #[serde(default, skip_serializing_if = "Option::is_none")]
    icon: Option<String>,
}
