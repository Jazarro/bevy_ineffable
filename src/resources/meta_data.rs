use bevy::prelude::Resource;
use bevy::utils::HashMap;

use crate::input_action::InputKind;

#[derive(Debug, Default, Resource)]
pub(crate) struct IneffableMetaData {
    pub(crate) map: HashMap<&'static str, Vec<IneffableMetaItem>>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) struct IneffableMetaItem {
    pub(crate) group_id: String,
    pub(crate) action_id: String,
    pub(crate) kind: InputKind,
    /// The enum variant index.
    pub(crate) index: usize,
}

impl IneffableMetaData {
    pub(crate) fn group_exists(&self, group_id: &str) -> bool {
        self.map.contains_key(group_id)
    }
    pub(crate) fn group(&self, group_id: &str) -> Option<&Vec<IneffableMetaItem>> {
        self.map.get(group_id)
    }
    pub(crate) fn group_ids(&self) -> Vec<String> {
        self.map.keys().map(|key| (*key).to_string()).collect()
    }
    pub(crate) fn action_ids(&self, group_id: &str) -> Vec<String> {
        self.map
            .get(group_id)
            .expect("This method is only valid for registered group_ids.")
            .iter()
            .map(|key| key.action_id.clone())
            .collect()
    }
    pub(crate) fn action(&self, group_id: &str, action_id: &str) -> Option<&IneffableMetaItem> {
        self.map
            .get(group_id)
            .and_then(|group| group.iter().find(|action| action.action_id == action_id))
    }
    pub(crate) fn actions(&self, group_id: &str) -> &Vec<IneffableMetaItem> {
        self.map.get(group_id).expect("")
    }
}
