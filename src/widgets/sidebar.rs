use std::ops::Index;
use gtk4::prelude::*;
use gtk4 as gtk;
use relm4::{ComponentUpdate, Model, Widgets, send, Sender, MicroComponent, WidgetPlus};
use crate::{AppModel};
use crate::services::local::lists::{get_lists, post_list};
use crate::models::list::List;
use crate::widgets::app::AppMsg;

#[derive(Default)]
pub(crate) struct SidebarModel {
    lists: Vec<MicroComponent<List>>,
}

pub enum SidebarMsg {
    Delete(usize),
    AddList(String),
    SelectList(usize),
    Rename(usize, String),
}

impl Model for SidebarModel {
    type Msg = SidebarMsg;
    type Widgets = SidebarWidgets;
    type Components = ();
}

impl ComponentUpdate<AppModel> for SidebarModel {
    fn init_model(_parent_model: &AppModel) -> Self {
        let mut lists = vec![
            MicroComponent::new(List::new("Inbox", "list-add-symbolic"), ()),
            MicroComponent::new(List::new("Today", "list-add-symbolic"), ()),
            MicroComponent::new(List::new("Next 7 Days", "list-add-symbolic"), ()),
            MicroComponent::new(List::new("All", "list-add-symbolic"), ()),
        ];
        let fe = &mut get_lists().unwrap().iter().map(|list| {
            MicroComponent::new(list.to_owned(), ())
        }).collect();
        lists.append(fe);
        SidebarModel {
            lists,
        }
    }

    fn update(&mut self, msg: Self::Msg, _components: &Self::Components, _sender: Sender<Self::Msg>, parent_sender: Sender<AppMsg>) {
        match msg {
            SidebarMsg::Delete(i) => println!("Deleting list at index {i}"),
            SidebarMsg::AddList(name) => {
                post_list(name.clone()).unwrap();
                self.lists.push(MicroComponent::new(List::new(&*name, ""), ()))
            },
            SidebarMsg::SelectList(i) => {
                let list = self.lists.index(i);
                let model = list.model_mut().unwrap();
                let id_list = &model.id_list;
                parent_sender.send(AppMsg::ListSelected(id_list.to_owned())).expect("Failed to get task list.");
            },
            SidebarMsg::Rename(i, name) => println!("Renaming list at index {i} to {name}")
        }
    }
}

#[relm4_macros::widget(pub)]
impl Widgets<SidebarModel, AppModel> for SidebarWidgets {
    view! {
        list_container = &gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            append: scroll_window = &gtk::ScrolledWindow {
                set_child: list = Some(&gtk::ListBox) {
                    set_selection_mode: gtk::SelectionMode::Single,
                    set_vexpand: true,
                    set_margin_all: 2,
                    set_css_classes: &["navigation-sidebar"],
                    connect_row_activated(sender) => move |listbox, _| {
                        let index = listbox.selected_row().unwrap().index() as usize;
                        send!(sender, SidebarMsg::SelectList(index))
                    },
                    append: iterate! {
                        model.lists.iter().map(|list| {
                            list.root_widget() as &gtk::Box
                        }).collect::<Vec<&gtk::Box>>()
                    }
                },
            },
            append: action_buttons = &gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                set_spacing: 10,
                set_margin_top: 10,
                set_margin_bottom: 10,
                set_margin_start: 10,
                set_margin_end: 10,
                set_halign: gtk::Align::Center,
                append: add_list_button = &gtk::MenuButton {
                    set_label: "Add List",
                    set_popover: new_list_popover = Some(&gtk::Popover) {
                        set_child: new_list_entry = Some(&gtk::Entry) {
                            connect_activate(sender) => move |entry| {
                                let buffer = entry.buffer();
                                send!(sender, SidebarMsg::AddList(buffer.text()))
                            }
                        }
                    }
                },
                append: add_group_button = &gtk::MenuButton {
                    set_label: "Add Group",
                }
            },
        }
    }
    fn post_view() {
        for list in &model.lists {
            if !list.is_connected() {
                self.list.append(list.root_widget())
            }
        }
    }
}