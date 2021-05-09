#
# This file is part of monitaringu-rei
#
# Copyright (C) 2021 Eric Le Bihan <eric.le.bihan.dev@free.fr>
#
# SPDX-License-Identifier: MIT
#

project('monitaringu-rei', 'rust',
        version : '0.1.1',
        license : 'MIT',
        default_options : [
          'prefix=/usr/local',
          'sysconfdir=/etc',
        ],
        meson_version : '>= 0.50')

prefixdir = get_option('prefix')
bindir = join_paths(prefixdir, get_option('bindir'))
libdir = join_paths(prefixdir, get_option('libdir'))
sysconfdir = join_paths(prefixdir, get_option('sysconfdir'))
includedir = join_paths(prefixdir, get_option('includedir'))
datadir = join_paths(prefixdir, get_option('datadir'))
localedir = join_paths(prefixdir, get_option('localedir'))
pkgdatadir = join_paths(datadir, meson.project_name())

pkg_defs = [
  ['GETTEXT_PACKAGE', meson.project_name()],
  ['GETTEXT_PACKAGE_GTK', meson.project_name() + '-gtk'],
  ['LOCALEDIR', localedir],
  ['PACKAGE_NAME', meson.project_name()],
  ['PACKAGE_STRING', '@0@ @1@'.format(meson.project_name(),
                                      meson.project_version())],
  ['PACKAGE_VERSION', meson.project_version()],
  ['PACKAGE_DATADIR', pkgdatadir],
]

conf = configuration_data()

foreach def : pkg_defs
  conf.set_quoted(def[0], def[1])
endforeach


enable_man_pages = get_option('man-pages')
if enable_man_pages
  pandoc = find_program('pandoc', required : true)
  if pandoc.found()
    subdir('man')
  endif
endif

gnome = import('gnome')
i18n = import('i18n')

glib_dep = dependency('glib-2.0', version : '>= 2.44.0')
gtk_dep = dependency('gtk+-3.0', version : '>= 3.22.0')

if get_option('buildtype').startswith('debug')
  cargo_mode = 'debug'
else
  cargo_mode = 'release'
endif

cargo = find_program('cargo')
cargo_vendor = find_program('cargo-vendor', required: false)
cargo_wrapper = find_program('build-aux/cargo')
cargo_target = 'native'

if meson.is_cross_build()
  host_cmds = meson.get_compiler('rust', native: false).cmd_array()
  if host_cmds.contains('--target')
    cargo_target = host_cmds.get(2)
  endif
  message('Rust target is @0@'.format(cargo_target))
endif

monitaringu_rei_build_files = files(
  'Cargo.toml',
  'Cargo.lock',
  'monitaringu-rei-core/Cargo.toml',
  'monitaringu-rei-core/build.rs',
  'monitaringu-rei-gtk/Cargo.toml',
)

subdir('shell-completion')
subdir('monitaringu-rei-core')
subdir('monitaringu-rei-gtk')

archive = meson.project_name() + '-' + meson.project_version()
distribute = run_target('distribute',
                        depends : [monitaringu_rei, monitaringu_rei_gtk],
                        command : ['build-aux/distribute', archive])

status = [
  '@0@ @1@'.format(meson.project_name(), meson.project_version()),

  'prefix:         @0@'.format(prefixdir),
  'sysconfdir:     @0@'.format(sysconfdir),
  'libdir:         @0@'.format(libdir),
  'includedir:     @0@'.format(includedir),
  'bindir:         @0@'.format(bindir),
  '',
  'man:            @0@'.format(enable_man_pages),
]

message('\n         '.join(status))
