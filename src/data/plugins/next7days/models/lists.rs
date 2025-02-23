use serde::{Deserialize, Serialize};

use crate::data::models::generic::lists::GenericTaskList;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct LocalList {
	pub id_list: String,
	pub display_name: String,
	pub is_owner: bool,
	pub count: i32,
	pub icon_name: Option<String>,
	pub provider: String,
	pub is_smart: bool,
}

impl From<GenericTaskList> for LocalList {
	fn from(list: GenericTaskList) -> Self {
		Self {
			id_list: list.id_list,
			display_name: list.display_name,
			is_owner: list.is_owner,
			count: list.count,
			icon_name: list.icon_name,
			provider: list.provider,
			is_smart: false,
		}
	}
}
