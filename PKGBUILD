# Maintainer: Alessio Biancalana <dottorblaster@gmail.com>
# Contributor: Orhun Parmaksız <orhun@archlinux.org>

pkgname=pacnews
pkgver=2.1.0
pkgrel=1
pkgdesc="Read Arch Linux News from your CLI"
arch=('i686' 'x86_64')
license=('MIT')
url="https://github.com/dottorblaster/pacnews"
makedepends=('cargo' 'pacman')
source=("https://github.com/dottorblaster/$pkgname/archive/v${pkgver}.tar.gz")
sha256sums=('17f120e94b1c61cc2b8a53e035345278b61130c05d82a51c43659018c2947a4b')

prepare() {
  cd "$srcdir/$pkgname-$pkgver"
  cargo fetch --locked --target "$CARCH-unknown-linux-gnu"
}

build() {
  cd "$srcdir/$pkgname-$pkgver"
  cargo build --release --frozen
}

package() {
  cd "$srcdir/$pkgname-$pkgver"
  install -DT "target/release/pacnews" "$pkgdir/usr/bin/pacnews"
  install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
  install -Dm644 README.md -t "$pkgdir/usr/share/doc/$pkgname"
}
