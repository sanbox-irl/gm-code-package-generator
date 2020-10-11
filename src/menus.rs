use heck::CamelCase;
use serde::{Deserialize, Serialize, Serializer};
use std::collections::{BTreeMap, BTreeSet};

const DEFAULT_NAVIGATION_MENU: &str = include_str!("../defaults/navigation_menu.json");
const DEFAULT_VIEW_ITEM_CONTEXT: &str = include_str!("../defaults/view_item_context.json");

#[derive(Debug, Serialize, Deserialize)]
pub struct Menus {
    menus: BTreeMap<MenuKey, BTreeSet<Context>>,
    submenus: Vec<SubMenu>,
}

impl Menus {
    const NAVIGATION_KEY: &'static MenuKey = &MenuKey::Navigation;
    const CONTEXT_KEY: &'static MenuKey = &MenuKey::Context;

    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_context_toplevel(&mut self, cc: CommandContext) {
        self.add_context_submenu(Self::CONTEXT_KEY, cc)
    }

    pub fn add_submenu_toplevel(
        &mut self,
        submenu_name: &str,
        idx: usize,
        icon: Option<String>,
    ) -> MenuKey {
        self.add_submenu_submenu(Self::CONTEXT_KEY, submenu_name, idx, icon)
    }

    pub fn add_context_submenu(&mut self, submenu: &MenuKey, cc: CommandContext) {
        let inner = self.menus.get_mut(submenu).unwrap();
        inner.insert(Context::Command(cc));
    }

    pub fn add_submenu_submenu(
        &mut self,
        parent: &MenuKey,
        submenu_name: &str,
        idx: usize,
        icon: Option<String>,
    ) -> MenuKey {
        let id = format!("gmVfs.{}", submenu_name.to_camel_case());
        self.submenus.push(SubMenu {
            id: id.clone(),
            label: submenu_name.to_string(),
            icon,
        });

        let inner = self.menus.get_mut(parent).unwrap();
        inner.insert(Context::SubMenu(SubMenuContext::new(
            &submenu_name.to_camel_case(),
            idx,
        )));

        let key = MenuKey::Other(id);

        self.menus.insert(key.clone(), BTreeSet::new());

        key
    }
}

impl Default for Menus {
    fn default() -> Self {
        let mut menus: BTreeMap<MenuKey, BTreeSet<Context>> = Default::default();

        let normal_commands: BTreeSet<Context> =
            serde_json::from_str(DEFAULT_NAVIGATION_MENU).unwrap();
        menus.insert(Self::NAVIGATION_KEY.clone(), normal_commands);

        let normal_commands: BTreeSet<Context> =
            serde_json::from_str(DEFAULT_VIEW_ITEM_CONTEXT).unwrap();
        menus.insert(Self::CONTEXT_KEY.clone(), normal_commands);

        Self {
            menus,
            submenus: vec![],
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Ord, PartialOrd, Eq, PartialEq)]
#[serde(untagged)]
pub enum Context {
    Command(CommandContext),
    SubMenu(SubMenuContext),
}

#[derive(Debug, Serialize, Deserialize, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct CommandContext {
    group: String,
    command: String,
    when: String,
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

#[derive(Debug, Serialize, Deserialize, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct SubMenuContext {
    group: String,
    pub submenu: String,
    when: String,
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

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone, Hash, Deserialize)]
pub enum MenuKey {
    Navigation,
    Context,
    Other(String),
}

impl std::fmt::Display for MenuKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MenuKey::Navigation => write!(f, "view/title"),
            MenuKey::Context => write!(f, "view/title/context"),
            MenuKey::Other(s) => write!(f, "{}", s),
        }
    }
}

impl Serialize for MenuKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("{}", self))
    }
}
