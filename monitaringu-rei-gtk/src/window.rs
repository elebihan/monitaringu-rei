//
// This file is part of monitaringu-rei
//
// Copyright (C) 2021 Eric Le Bihan <eric.le.bihan.dev@free.fr>
//
// SPDX-License-Identifier: MIT
//

use gettextrs::gettext;
use gio::{self, prelude::*, subclass::prelude::*};
use glib::clone;
use gtk::prelude::*;
use gtk::subclass::application_window::ApplicationWindowImpl;
use gtk::subclass::prelude::*;
use gtk::CompositeTemplate;
use std::cell::RefCell;
use std::path::PathBuf;

use crate::app::Application;
use crate::header_bar;
use crate::supervisor::Settings;

mod imp {
    use super::*;

    #[derive(Default, Debug, CompositeTemplate)]
    #[template(resource = "/com/elebihan/monitaringu-rei-gtk/gtk/window.ui")]
    pub struct ApplicationWindow {
        #[template_child]
        pub command_entry: TemplateChild<gtk::Entry>,
        #[template_child]
        pub pattern_entry: TemplateChild<gtk::Entry>,
        #[template_child]
        pub directory_chooser: TemplateChild<gtk::FileChooserButton>,
        #[template_child]
        pub start_button: TemplateChild<gtk::ToolButton>,
        #[template_child]
        pub stop_button: TemplateChild<gtk::ToolButton>,
        #[template_child]
        pub activity_spinner: TemplateChild<gtk::Spinner>,
        #[template_child]
        pub results_scrolled_window: TemplateChild<gtk::ScrolledWindow>,
        #[template_child]
        pub results_tree_view: TemplateChild<gtk::TreeView>,
        pub model: RefCell<Option<gtk::ListStore>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ApplicationWindow {
        const NAME: &'static str = "ApplicationWindow";
        type Type = super::ApplicationWindow;
        type ParentType = gtk::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ApplicationWindow {
        fn constructed(&self, obj: &Self::Type) {
            obj.setup_model();
            obj.setup_widgets();
            self.parent_constructed(obj);
        }
    }

    impl WidgetImpl for ApplicationWindow {}

    impl ContainerImpl for ApplicationWindow {}

    impl BinImpl for ApplicationWindow {}

    impl WindowImpl for ApplicationWindow {}

    impl ApplicationWindowImpl for ApplicationWindow {}
}
glib::wrapper! {
    pub struct ApplicationWindow(ObjectSubclass<imp::ApplicationWindow>)
        @extends gtk::Widget, gtk::Container, gtk::Bin, gtk::Window, gtk::ApplicationWindow;
}

impl ApplicationWindow {
    pub fn new(app: &Application) -> Self {
        glib::Object::new::<ApplicationWindow>(&[("application", app)])
            .expect("Failed to create ApplicationWindow")
    }

    fn model(&self) -> gtk::ListStore {
        self.imp()
            .model
            .borrow()
            .clone()
            .expect("Failed to get model")
    }

    fn setup_model(&self) {
        let model = gtk::ListStore::new(&[String::static_type()]);
        self.imp().model.replace(Some(model));
    }

    fn setup_widgets(&self) {
        let header_bar = header_bar::create();
        self.set_titlebar(Some(&header_bar));

        let col = gtk::TreeViewColumn::new();
        let cell = gtk::CellRendererText::new();
        col.pack_end(&cell, true);
        col.add_attribute(&cell, "text", 0);
        col.set_title(&gettext("Filename"));
        col.set_clickable(true);
        col.set_sort_indicator(true);
        col.set_sort_column_id(0);

        let imp = imp::ApplicationWindow::from_instance(self);
        imp.results_tree_view.append_column(&col);
        imp.results_tree_view.set_model(Some(&self.model()));
        imp.results_tree_view
            .connect_size_allocate(clone!(@weak self as this => move |_,_| {
                let imp = imp::ApplicationWindow::from_instance(&this);
                let adj = imp.results_scrolled_window.vadjustment();
                adj.set_value(adj.upper() - adj.page_size());
            }));

        imp.stop_button.set_sensitive(false);

        let accel_group = gtk::AccelGroup::new();
        self.add_accel_group(&accel_group);

        let (key, modifier) = gtk::accelerator_parse("<Primary>s");
        imp.start_button.add_accelerator(
            "clicked",
            &accel_group,
            key,
            modifier,
            gtk::AccelFlags::VISIBLE,
        );
        let (key, modifier) = gtk::accelerator_parse("<Primary>c");
        imp.stop_button.add_accelerator(
            "clicked",
            &accel_group,
            key,
            modifier,
            gtk::AccelFlags::VISIBLE,
        );
    }

    pub fn set_busy(&self, busy: bool) {
        let imp = self.imp();
        if busy {
            imp.activity_spinner.start();
            imp.activity_spinner.show();
        } else {
            imp.activity_spinner.stop();
            imp.activity_spinner.hide();
        }
        imp.command_entry.set_sensitive(!busy);
        imp.pattern_entry.set_sensitive(!busy);
        imp.directory_chooser.set_sensitive(!busy);
        imp.start_button.set_sensitive(!busy);
        imp.stop_button.set_sensitive(busy);
    }

    pub fn get_settings(&self) -> Option<Settings> {
        let imp = self.imp();
        let command = imp.command_entry.text();
        let directory = imp.directory_chooser.uri();
        let pattern = imp.pattern_entry.text();
        if let Some(directory) = directory {
            if !command.as_str().is_empty() {
                return Some(Settings {
                    command: command.into(),
                    directory: directory.strip_prefix("file://").unwrap().into(),
                    pattern: pattern.into(),
                });
            }
        }
        None
    }

    pub fn add_result(&self, path: PathBuf) {
        let path = path
            .into_os_string()
            .into_string()
            .expect("Failed to convert path");
        let values: [(u32, &dyn ToValue); 1] = [(0, &path)];
        self.model().set(&self.model().append(), &values);
    }
}
