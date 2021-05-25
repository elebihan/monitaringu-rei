# Building for Microsoft Windows

This document presents the method to build Monitaringu Rei for Microsoft Windows.

## From Linux

#### Requirements

To cross-compile from Linux to MS Windows on Fedora, install [mingw-w64](http://mingw-w64.org/):

```sh
dnf install -y \
    mingw64-gcc \
    mingw64-gtk3 \
    mingw64-winpthreads-static \
    mingw32-nsis
```

#### Building

To cross-compile, execute:

```sh
export MINGW_TARGET=x86_64-w64-mingw32
export MINGW_PREFIX=/usr/${MINGW_TARGET}/sys-root/mingw
export PKG_CONFIG_PATH=${MINGW_PREFIX}/lib/pkgconfig
export PKG_CONFIG_ALLOW_CROSS=1
export MINGW_DESTDIR=/tmp/monitaringu-rei/${MINGW_TARGET}
export WINDRES=${MINGW_TARGET}-windres
meson --prefix=${MINGW_DESTDIR} \
      --buildtype=release \
      --cross-file=extra/mswin/${MINGW_TARGET}.ini \
      _build/${MINGW_TARGET}
ninja -C _build/${MINGW_TARGET}
```

#### Packaging

The list of DLLs required by the program and which must be included in the
package can be generated using [mingw-ldd](https://github.com/nurupo/mingw-ldd).

First, collect the DLLs and copy them in the destination directory:

```sh
mkdir -p ${MINGW_DESTDIR}
mingw-ldd --dll-lookup-dirs=${MINGW_PREFIX}/bin \
    _build/${MINGW_TARGET}/target/x86_64-pc-windows-gnu/release/monitaringu-rei-gtk.exe | \
    awk '/dll$/ { print $3 }' | \
    xargs -I {} cp -a {} ${MINGW_DESTDIR}/bin
```

Then, copy some GTK files:

```sh
mkdir -p ${MINGW_DESTDIR}/share/glib-2.0/schemas
cp ${MINGW_PREFIX}/share/glib-2.0/schemas/gschemas.compiled ${MINGW_DESTDIR}/share/glib-2.0/schemas
cp -a ${MINGW_PREFIX}/share/icons ${MINGW_DESTDIR}/share/icons
```

Next, install a [GTK theme matching MS Windows 10 appearence](https://github.com/B00merang-Project/Windows-10):

```sh
mkdir -p ${MINGW_DESTDIR}/share/themes/Windows10/gtk-3.0
wget https://github.com/B00merang-Project/Windows-10/archive/refs/tags/3.2.tar.gz
tar --strip-components=2 -xzf 3.2.tar.gz -C ${MINGW_DESTDIR}/share/themes/Windows10/gtk-3.0 Windows-10-3.2/gtk-3.20
mkdir -p ${MINGW_DESTDIR}/share/gtk-3.0
cat <<EOF>${MINGW_DESTDIR}/share/gtk-3.0/settings.ini
[Settings]
gtk-theme-name = Windows10
gtk-font-name = Segoe UI 10
gtk-xft-rgba = rgb
EOF
```

Finally, install the program:

```sh
ninja -C _build/${MINGW_TARGET} install
```

The program can then be tested using [Wine](https://winehq.org):

```sh
XDG_DATA_DIRS=${MINGW_DESTDIR}/share wine ${MINGW_DESTDIR}/bin/monitaringu-rei-gtk.exe
```

#### Generating installer

To build a self-installer using [NSIS](https://nsis.sourceforge.io/), execute:

```sh
export PACKAGE_VERSION=$(sed -ne "s/\s*version : '\([0-9.]\+\)',/\1/p" meson.build)
sed -e "s%@PACKAGE_DIR@%${MINGW_DESTDIR}%g" \
    -e "s%@PACKAGE_VERSION@%${PACKAGE_VERSION}%g" \
    extra/mswin/installer.nsi.in > installer.nsi
makensis installer.nsi
```
