pkgname=ledmatrix_widgets
pkgver=0.1.0
pkgrel=1
makedepends=('rust' 'cargo')
arch=('i686' 'x86_64' 'armv6h' 'armv7h')
pkgdesc="A rust application for configuring and displaying widgets on the Framework 16 LED Matrix modules"
license=('GPL-3.0')

build() {
    return 0
}

package() {
	mkdir -p "$pkgdir/usr/"
    cargo install --root="$pkgdir/usr/" ledmatrix_widgets
	rm -f "$pkgdir/usr/.crates.toml" "$pkgdir/usr/.crates2.json"
	
}

