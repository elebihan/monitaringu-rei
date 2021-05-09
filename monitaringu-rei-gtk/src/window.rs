//
// This file is part of monitaringu-rei
//
// Copyright (C) 2021 Eric Le Bihan <eric.le.bihan.dev@free.fr>
//
// SPDX-License-Identifier: MIT
//

use gettextrs::gettext;
use gio::{self, prelude::*, subclass::prelude::*};
use glib::subclass;
use glib::translate::*;
use glib::{clone, glib_object_impl, glib_object_subclass, glib_wrapper};
use gtk::prelude::*;
use gtk::subclass::application_window::ApplicationWindowImpl;
use gtk::subclass::prelude::*;
use once_cell::unsync::OnceCell;
use std::path::PathBuf;

use crate::header_bar;
use crate::supervisor::Settings;

#[derive(Debug)]
struct WindowWidgets {
    header_bar: gtk::HeaderBar,
    command_entry: gtk::Entry,
    pattern_entry: gtk::Entry,
    directory_chooser: gtk::FileChooserButton,
    start_button: gtk::ToolButton,
    stop_button: gtk::ToolButton,
    activity_spinner: gtk::Spinner,
    results_tree_view: gtk::TreeView,
}

pub struct ApplicationWindowPrivate {
    builder: gtk::Builder,
    widgets: OnceCell<WindowWidgets>,
    model: gtk::ListStore,
}

impl ObjectSubclass for ApplicationWindowPrivate {
    const NAME: &'static str = "ApplicationWindow";
    type ParentType = gtk::ApplicationWindow;
    type Instance = subclass::simple::InstanceStruct<Self>;
    type Class = subclass::simple::ClassStruct<Self>;

    glib_object_subclass!();

    fn new() -> Self {
        let builder =
            gtk::Builder::from_resource("/com/elebihan/monitaringu-rei-gtk/gtk/window.ui");
        // builder.set_translation_domain(Some("monitaringu-rei-gtk"));
        Self {
            builder,
            widgets: OnceCell::new(),
            model: gtk::ListStore::new(&[String::static_type()]),
        }
    }
}

impl ObjectImpl for ApplicationWindowPrivate {
    glib_object_impl!();

    fn constructed(&self, obj: &glib::Object) {
        self.parent_constructed(obj);

        let self_ = obj.downcast_ref::<ApplicationWindow>().unwrap();

        let header_bar = header_bar::create();
        self_.set_titlebar(Some(&header_bar));

        let builder = &self.builder;

        let command_entry = builder.get_object("command_entry").unwrap();
        let pattern_entry = builder.get_object("pattern_entry").unwrap();
        let directory_chooser = builder.get_object("directory_chooser").unwrap();
        let start_button = builder.get_object("start_button").unwrap();
        let stop_button = builder.get_object("stop_button").unwrap();
        let activity_spinner = builder.get_object("activity_spinner").unwrap();
        let results_scrolled_window: gtk::ScrolledWindow =
            builder.get_object("results_scrolled_window").unwrap();
        let results_tree_view: gtk::TreeView = builder.get_object("results_tree_view").unwrap();
        results_tree_view.set_model(Some(&self.model));
        results_tree_view.connect_size_allocate(
            clone!(@weak results_scrolled_window => move |_,_| {
                let adj = results_scrolled_window.get_vadjustment().unwrap();
                adj.set_value(adj.get_upper()- adj.get_page_size());
            }),
        );

        let main_box: gtk::Box = builder.get_object("main_box").unwrap();

        self_.add(&main_box);

        self.widgets
            .set(WindowWidgets {
                header_bar,
                command_entry,
                pattern_entry,
                directory_chooser,
                start_button,
                stop_button,
                activity_spinner,
                results_tree_view,
            })
            .expect("Failed to initialize ApplicationWindow state");
    }
}

impl WidgetImpl for ApplicationWindowPrivate {}

impl ContainerImpl for ApplicationWindowPrivate {}

impl BinImpl for ApplicationWindowPrivate {}

impl WindowImpl for ApplicationWindowPrivate {}

impl ApplicationWindowImpl for ApplicationWindowPrivate {}

glib_wrapper! {
    pub struct ApplicationWindow(
        Object<subclass::simple::InstanceStruct<ApplicationWindowPrivate>,
               subclass::simple::ClassStruct<ApplicationWindowPrivate>,
               ApplicationWindowClass>)
        @extends gtk::Widget, gtk::Container, gtk::Bin, gtk::Window, gtk::ApplicationWindow;

    match fn {
        get_type => || ApplicationWindowPrivate::get_type().to_glib(),
    }
}

impl ApplicationWindow {
    pub fn new(app: &gtk::Application) -> Self {
        let window = glib::Object::new(Self::static_type(), &[("application", app)])
            .expect("Failed to create ApplicationWindow")
            .downcast::<ApplicationWindow>()
            .expect("Created ApplicationWindow is of wrong type");
        window.setup_widgets();
        window.set_size_request(400, 480);
        window
    }

    fn get_widgets(&self) -> &WindowWidgets {
        ApplicationWindowPrivate::from_instance(self)
            .widgets
            .get()
            .unwrap()
    }

    fn setup_widgets(&self) {
        let col = gtk::TreeViewColumn::new();
        let cell = gtk::CellRendererText::new();
        col.pack_end(&cell, true);
        col.add_attribute(&cell, "text", 0);
        col.set_title(&gettext("Filename"));
        col.set_clickable(true);
        col.set_sort_indicator(true);
        col.set_sort_column_id(0);
        let widgets = self.get_widgets();
        widgets.results_tree_view.append_column(&col);
        widgets.stop_button.set_sensitive(false);

        let accel_group = gtk::AccelGroup::new();
        self.add_accel_group(&accel_group);

        let (key, modifier) = gtk::accelerator_parse("<Primary>s");
        widgets.start_button.add_accelerator(
            "clicked",
            &accel_group,
            key,
            modifier,
            gtk::AccelFlags::VISIBLE,
        );
        let (key, modifier) = gtk::accelerator_parse("<Primary>c");
        widgets.stop_button.add_accelerator(
            "clicked",
            &accel_group,
            key,
            modifier,
            gtk::AccelFlags::VISIBLE,
        );
    }

    pub fn set_busy(&self, busy: bool) {
        let widgets = self.get_widgets();
        if busy {
            widgets.activity_spinner.start();
            widgets.activity_spinner.show();
        } else {
            widgets.activity_spinner.stop();
            widgets.activity_spinner.hide();
        }
        widgets.command_entry.set_sensitive(!busy);
        widgets.pattern_entry.set_sensitive(!busy);
        widgets.directory_chooser.set_sensitive(!busy);
        widgets.start_button.set_sensitive(!busy);
        widgets.stop_button.set_sensitive(busy);
    }

    pub fn get_settings(&self) -> Option<Settings> {
        let widgets = self.get_widgets();
        let command = widgets.command_entry.get_text();
        let directory = widgets.directory_chooser.get_uri();
        let pattern = widgets.pattern_entry.get_text();
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
        let imp = ApplicationWindowPrivate::from_instance(self);
        let path = path
            .into_os_string()
            .into_string()
            .expect("Failed to convert path");
        let values: [&dyn ToValue; 1] = [&path];
        imp.model.set(&imp.model.append(), &[0], &values);
    }
}
