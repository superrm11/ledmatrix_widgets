pkgname=ledmatrix_widgets
pkgver=0.1.0
pkgrel=1
makedepends=('rust' 'cargo')
arch=('i686' 'x86_64' 'armv6h' 'armv7h')
pkgdesc="A rust application for configuring and displaying widgets on the Framework 16 LED Matrix modules"
license=('GPL-3.0')

build() {
	cargo build --release
}

package() {
	mkdir -p "$pkgdir/usr/bin/"
	install -m 755 $startdir/target/release/ledmatrix_widgets $pkgdir/usr/bin/
}

