//
// This file is part of monitaringu-rei
//
// Copyright (C) 2021 Eric Le Bihan <eric.le.bihan.dev@free.fr>
//
// SPDX-License-Identifier: MIT
//

use gettextrs::gettext;
use gio::{self, prelude::*, subclass::prelude::*, ApplicationFlags};
use glib::{clone, Receiver, Sender};
use gtk::prelude::*;
use gtk::subclass::application::GtkApplicationImpl;
use once_cell::unsync::OnceCell;
use std::cell::RefCell;
use std::path::PathBuf;

use crate::{
    dialogs::{show_about_dialog, show_error_dialog},
    pkginfo::APPLICATION_ID,
    supervisor::{Message, Settings, Supervisor},
    window::ApplicationWindow,
};

mod imp {

    use super::*;

    pub struct Application {
        pub window: OnceCell<ApplicationWindow>,
        pub supervisor: RefCell<Option<Supervisor>>,
        pub receiver: RefCell<Option<Receiver<Message>>>,
        pub sender: Sender<Message>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Application {
        const NAME: &'static str = "Application";
        type Type = super::Application;
        type ParentType = gtk::Application;

        fn new() -> Self {
            let (sender, receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
            let receiver = RefCell::new(Some(receiver));
            let window = OnceCell::new();
            let supervisor = RefCell::new(None);
            Self {
                window,
                supervisor,
                receiver,
                sender,
            }
        }
    }

    impl ObjectImpl for Application {}

    impl GtkApplicationImpl for Application {}

    impl ApplicationImpl for Application {
        fn activate(&self, app: &Self::Type) {
            let app = app
                .clone()
                .downcast::<super::Application>()
                .expect("Failed to downcast Application");
            app.setup_gactions();

            let window = self
                .window
                .get()
                .expect("ApplicationWindow not initialized");

            let receiver = self.receiver.borrow_mut().take().unwrap();
            receiver.attach(None, move |message| match message {
                Message::FileCreated(path_buf) => app.add_result(path_buf),
                Message::Error(e) => {
                    app.notify_error(&gettext!("An error occurred: {}", e));
                    app.stop_supervisor();
                    glib::Continue(true)
                }
            });

            window.show_all();
            window.present();
        }

        fn startup(&self, app: &Self::Type) {
            self.parent_startup(app);
            let window = ApplicationWindow::new(&app);
            self.window
                .set(window)
                .expect("Failed to initialize ApplicationWindow");
            app.set_accels_for_action("app.start", &["<Primary>x"]);
            app.set_accels_for_action("app.stop", &["<Primary>d"]);
            app.set_accels_for_action("app.quit", &["<Primary>q"]);
        }
    }
}

glib::wrapper! {
    pub struct Application(
       ObjectSubclass<imp::Application>)
        @extends gio::Application, gtk::Application;
}

impl Application {
    pub fn new() -> Self {
        glib::Object::new::<Application>(&[
            ("application-id", &APPLICATION_ID),
            ("flags", &ApplicationFlags::NON_UNIQUE),
        ])
        .expect("Failed to create Application")
        .downcast()
        .expect("Wrong type for Application")
    }

    fn setup_gactions(&self) {
        let win = self.window();
        let app = self.clone().upcast::<gtk::Application>();
        let quit = gio::SimpleAction::new("quit", None);
        quit.connect_activate(clone!(@weak app => move |_,_| {
            app.quit();
        }));
        let about = gio::SimpleAction::new("about", None);
        about.connect_activate(clone!(@weak app => move |_,_| {
            show_about_dialog(&app);
        }));
        let start = gio::SimpleAction::new("start", None);
        start.connect_activate(clone!(@weak self as app, @weak win => move |_,_| {
            match win.settings() {
                Some(settings) => {
                    app.start_supervisor(settings);
                },
                None => show_error_dialog(&win.upcast::<gtk::Window>(), &gettext("Missing parameters")),
            }

        }));
        let stop = gio::SimpleAction::new("stop", None);
        stop.connect_activate(clone!(@weak self as app => move |_,_| {
            app.stop_supervisor();
        }));
        stop.set_enabled(false);
        app.add_action(&about);
        app.add_action(&quit);
        app.add_action(&start);
        app.add_action(&stop);
    }

    fn window(&self) -> &ApplicationWindow {
        imp::Application::from_instance(self).window.get().unwrap()
    }

    fn set_busy(&self, busy: bool) {
        self.window().set_busy(busy);
        let app = self.clone().upcast::<gtk::Application>();
        let start = app
            .lookup_action("start")
            .unwrap()
            .downcast::<gio::SimpleAction>()
            .unwrap();
        start.set_enabled(!busy);
        let stop = app
            .lookup_action("stop")
            .unwrap()
            .downcast::<gio::SimpleAction>()
            .unwrap();
        stop.set_enabled(busy);
    }

    fn notify_error(&self, message: &str) {
        let win = self.window().clone();
        show_error_dialog(&win.upcast::<gtk::Window>(), message);
    }

    fn start_supervisor(&self, settings: Settings) {
        let imp = imp::Application::from_instance(self);
        if imp.supervisor.borrow().is_some() {
            self.stop_supervisor();
        }
        match Supervisor::spawn(settings, imp.sender.clone()) {
            Ok(s) => {
                *imp.supervisor.borrow_mut() = Some(s);
                self.set_busy(true);
            }
            Err(e) => {
                self.notify_error(&gettext!("Failed to start: {:?}", e));
            }
        }
    }

    fn stop_supervisor(&self) {
        let imp = imp::Application::from_instance(self);
        if let Some(supervisor) = imp.supervisor.take() {
            if let Err(e) = supervisor.kill() {
                self.notify_error(&gettext!("Failed to stop: {:?}", e));
            }
        }
        *imp.supervisor.borrow_mut() = None;
        self.set_busy(false);
    }

    fn add_result(&self, path_buf: PathBuf) -> glib::Continue {
        let win = self.window();
        win.add_result(path_buf);
        glib::Continue(true)
    }
}
