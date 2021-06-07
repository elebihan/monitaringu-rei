#
# This file is part of monitaringu-rei
#
# Copyright (C) 2021 Eric Le Bihan <eric.le.bihan.dev@free.fr>
#
# SPDX-License-Identifier: MIT
#

subdir('icons')

gnome.compile_resources('monitaringu-rei-gtk',
                        'monitaringu-rei-gtk.gresource.xml',
                        gresource_bundle: true,
                        install: true,
                        install_dir: pkgdatadir,
)

i18n.merge_file(input : 'com.elebihan.monitaringu-rei-gtk.desktop.in',
                output : 'com.elebihan.monitaringu-rei-gtk.desktop',
                type : 'desktop',
                po_dir : join_paths(meson.source_root(), 'po'),
                install : true,
                install_dir : join_paths(datadir,'applications')
)
