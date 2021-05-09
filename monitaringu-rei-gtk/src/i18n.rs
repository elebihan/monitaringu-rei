//
// This file is part of monitaringu-rei
//
// Copyright (C) 2021 Eric Le Bihan <eric.le.bihan.dev@free.fr>
//
// SPDX-License-Identifier: MIT
//

#[cfg(not(windows))]
use gettextrs::bindtextdomain;
use gettextrs::textdomain;

use crate::pkginfo::GETTEXT_PACKAGE;
#[cfg(not(windows))]
use crate::pkginfo::LOCALEDIR;

// FIXME: Building gettextrs with mingw32 fails due to missing wbindtextdomain,
// which seems to be only available on MS Windows...
pub fn init() {
    #[cfg(not(windows))]
    bindtextdomain(GETTEXT_PACKAGE, LOCALEDIR).expect("Failed to bind text domain");
    textdomain(GETTEXT_PACKAGE).expect("Failed to init text domain");
}
