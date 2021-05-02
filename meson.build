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

enable_man_pages = get_option('man-pages')
if enable_man_pages
  pandoc = find_program('pandoc', required : true)
  if pandoc.found()
    subdir('man')
  endif
endif

if get_option('buildtype').startswith('debug')
  cargo_mode = 'debug'
else
  cargo_mode = 'release'
endif

cargo = find_program('cargo')
cargo_vendor = find_program('cargo-vendor', required: false)
cargo_wrapper = find_program('build-aux/cargo')

monitaringu_rei_build_files = files(
  'Cargo.toml',
  'Cargo.lock',
  'monitaringu-rei-core/Cargo.toml',
  'monitaringu-rei-core/build.rs',
)

subdir('shell-completion')
subdir('monitaringu-rei-core')

archive = meson.project_name() + '-' + meson.project_version()
distribute = run_target('distribute',
                        depends : monitaringu_rei,
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
