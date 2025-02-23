use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use uuid::Uuid;

use crate::data::models::generic::tasks::GenericTask;
use crate::schema::tasks;

#[allow(clippy::extra_unused_lifetimes)]
#[derive(Debug, Clone, Insertable, Queryable)]
#[table_name = "tasks"]
pub struct QueryableTask {
	pub id_task: String,
	pub id_list: String,
	pub title: String,
	pub body: Option<String>,
	pub importance: Option<String>,
	pub favorite: bool,
	pub is_reminder_on: bool,
	pub status: Option<String>,
	pub completed_on: Option<NaiveDateTime>,
	pub due_date: Option<NaiveDateTime>,
	pub reminder_date: Option<NaiveDateTime>,
	pub created_date_time: Option<NaiveDateTime>,
	pub last_modified_date_time: Option<NaiveDateTime>,
}

impl QueryableTask {
	pub fn new(title: String, id_list: String) -> Self {
		Self {
			id_task: Uuid::new_v4().to_string(),
			id_list,
			title,
			body: None,
			completed_on: None,
			due_date: None,
			importance: Some("normal".to_string()),
			favorite: false,
			is_reminder_on: false,
			reminder_date: None,
			status: Some("notStarted".to_string()),
			created_date_time: None,
			last_modified_date_time: None,
		}
	}
}

impl From<GenericTask> for QueryableTask {
	fn from(task: GenericTask) -> Self {
		Self {
			id_task: task.id_task,
			id_list: task.id_list,
			title: task.title,
			body: task.body,
			completed_on: task.completed_on,
			due_date: task.due_date,
			importance: Some(task.importance.to_string()),
			favorite: task.favorite,
			is_reminder_on: task.is_reminder_on,
			reminder_date: task.reminder_date,
			status: Some(task.status.to_string()),
			created_date_time: task.created_date_time,
			last_modified_date_time: task.last_modified_date_time,
		}
	}
}
