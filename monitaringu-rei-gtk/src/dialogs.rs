//
// This file is part of monitaringu-rei
//
// Copyright (C) 2021 Eric Le Bihan <eric.le.bihan.dev@free.fr>
//
// SPDX-License-Identifier: MIT
//

use gettextrs::gettext;
use gtk::prelude::*;

use crate::pkginfo::{APPLICATION_ID, APPLICATION_NAME, APPLICATION_VERSION};

pub fn show_about_dialog(application: &gtk::Application) {
    let dialog = gtk::AboutDialog::new();
    dialog.set_program_name(APPLICATION_NAME);
    dialog.set_comments(Some(
        gettext("Start a program and monitor the files it creates.").as_ref(),
    ));
    dialog.set_version(Some(&APPLICATION_VERSION));
    dialog.set_authors(&["Eric Le Bihan <eric.le.bihan.dev@free.fr>"]);
    dialog.set_copyright(Some("Copyright © 2021 Eric Le Bihan"));
    dialog.set_license_type(gtk::License::MitX11);
    dialog.set_logo_icon_name(Some(&APPLICATION_ID));
    dialog.set_transient_for(application.get_active_window().as_ref());
    dialog.set_modal(true);
    dialog.show_all();
}

pub fn show_error_dialog(parent: &gtk::Window, message: &str) {
    let dialog = gtk::MessageDialog::new(
        Some(parent),
        gtk::DialogFlags::MODAL,
        gtk::MessageType::Error,
        gtk::ButtonsType::Ok,
        message,
    );
    dialog.connect_response(|dialog, _| dialog.close());
    dialog.show_all();
}
