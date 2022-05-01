# Maintainer: Eduardo Flores <edfloreshz@gmail.com>

pkgname=do-git
pkgrel=3
pkgver=0.1.2
pkgdesc="Do is a to-do app built for Linux with Rust and GTK."
arch=('x86_64')
url="https://github.com/edfloreshz/do"
license=('GPL2')
depends=('gtk4' 'libadwaita' 'pkg-config')
makedepends=('cargo' 'git' 'diesel_cli')
optdepends=()
provides=('todo')
conflicts=('todo' 'do')
source=("do-git::git+https://github.com/edfloreshz/do#branch=main")
md5sums=('SKIP')

prepare() {
	cd "$pkgname"
	echo "$(git rev-list --count HEAD).$(git rev-parse --short HEAD)"
}

build() {
	cd "$pkgname"
	cargo build --release
}

package() {
	cd "$pkgname"
	install -Dm644 src/resources/desktop/com.devloop.do.desktop "$pkgdir/usr/share/applications/do.edfloreshz.github.desktop"
	install -Dm644 src/resources/icons/com.devloop.do.svg "$pkgdir/usr/share/icons/hicolor/scalable/apps/do.svg"
	install -Dm644 src/resources/icons/com.devloop.do.svg "$pkgdir/usr/share/icons/hicolor/256x256/apps/do.svg"
	install -Dm644 src/resources/icons/com.devloop.do.svg "$pkgdir/usr/share/icons/hicolor/256x256/apps/do.svg"
	install -Dm644 src/resources/database/com.devloop.do.db "$pkgdir/usr/share/do/com.devloop.do.db"
	install -Dm644 migrations/* -t "$pkgdir/usr/share/do/migrations"
	install -Dm644 README.md "$pkgdir/usr/share/doc/do/README.md"
	install -Dm755 target/release/todo "$pkgdir/usr/bin/todo"
}

post_install() {
    cd "$pkgname"
	diesel migration run
}