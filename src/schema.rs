table! {
	lists (id_list) {
		id_list -> Text,
		display_name -> Text,
		is_owner -> Bool,
		count -> Integer,
		icon_name -> Nullable<Text>,
		provider -> Text,
	}
}

table! {
	tasks (id_task) {
		id_task -> Text,
		id_list -> Text,
		title -> Text,
		body -> Nullable<Text>,
		importance -> Nullable<Text>,
		favorite -> Bool,
		is_reminder_on -> Bool,
		status -> Nullable<Text>,
		completed_on -> Nullable<Timestamp>,
		due_date -> Nullable<Timestamp>,
		reminder_date -> Nullable<Timestamp>,
		created_date_time -> Nullable<Timestamp>,
		last_modified_date_time -> Nullable<Timestamp>,
	}
}

joinable!(tasks -> lists (id_list));

allow_tables_to_appear_in_same_query!(lists, tasks,);
