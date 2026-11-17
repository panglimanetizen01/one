# Instalasi dan Lingkungan

One v0.1 menyediakan CLI yang dapat dibangun dari source. **Installer siap-pasang dan binary release resmi belum diterbitkan**, jadi jangan menganggap perintah build di bawah sebagai unduhan installer.

## Persyaratan

- Git.
- Rust toolchain yang memenuhi `rust-version = "1.97"` pada workspace (rustc dan Cargo 1.97 atau lebih baru).
- C/C++ build tools. Workspace memakai SQLite bundled melalui `rusqlite`, sehingga build mengompilasi kode C SQLite dari source.
- Terminal Linux atau PowerShell pada Windows.

### Linux (Debian/Ubuntu)

Pasang compiler dan build tools:

```sh
sudo apt update
sudo apt install build-essential
```

Distribusi Linux lain memerlukan paket yang menyediakan GCC atau Clang dan `make`.

### Windows

Pasang **Visual Studio Build Tools 2022 atau lebih baru** melalui Visual Studio Installer, pilih workload **Desktop development with C++**, dan pastikan MSVC x64/x86 build tools serta Windows SDK ikut terpasang. Jalankan perintah build dari **Developer PowerShell for Visual Studio** agar compiler dan linker tersedia.

Periksa toolchain:

```powershell
where.exe cl
where.exe link
rustc --version
cargo --version
```

Linux:

```sh
cc --version
make --version
rustc --version
cargo --version
```

## Ambil source

```sh
git clone https://github.com/panglimanetizen01/one.git
cd one
```

## Build CLI release

Jalankan dari root repository:

```sh
cargo build --release --locked -p one-cli --bin one
```

Binary yang dihasilkan:

- Linux: `target/release/one`
- Windows: `target/release/one.exe`

Build pertama membutuhkan koneksi internet untuk mengunduh dependency Rust yang belum ada di cache Cargo. Opsi `--locked` meminta Cargo menggunakan dependency yang tercatat di `Cargo.lock`, tanpa memperbarui lockfile.

## Verifikasi CLI

Linux:

```sh
./target/release/one version
```

Windows PowerShell:

```powershell
.\target\release\one.exe version
```

Keduanya seharusnya menampilkan:

```text
one 0.1.0
```

## Jalankan program pertama

Buat file bernama `halo.one`:

```text
nama adalah "Budi"
tampilkan nama
```

Linux:

```sh
./target/release/one jalankan halo.one
```

Windows PowerShell:

```powershell
.\target\release\one.exe jalankan halo.one
```

Output yang diharapkan:

```text
Budi
```

Untuk perintah lain, lihat [perintah CLI](perintah-cli.md) dan [program pertama](program-pertama.md).

## Windows dan Linux

Perubahan CLI dan dokumentasi diuji melalui workflow CI pada Ubuntu dan Windows Server. Klaim verifikasi tetap terikat pada source SHA yang tercatat pada workflow; keberhasilan build satu revisi tidak otomatis membuktikan revisi lain.

## Batas distribusi

- `one bangun` menghasilkan artefak intermediate `.oneir`, bukan executable native.
- `one package` menghasilkan `.onepkg`, bukan platform installer.
- Build dari source adalah jalur penggunaan yang tersedia saat ini; belum ada klaim installer, package-manager distribution, auto-update, atau binary release resmi.

Halaman ini harus diperbarui ketika distribusi resmi berubah. Jangan mempertahankan instruksi instalasi yang sudah tidak cocok dengan artefak yang benar-benar diterbitkan.
