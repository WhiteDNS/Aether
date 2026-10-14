# boring-sys 5.2.0 desktop build patch

Source: the official crates.io `boring-sys` 5.2.0 package.
Published crate checksum: `6993efa9d9f0ebf3b0c02c8e868cfe37d0854cd021d51204480a91ce23d6e444`.
The original crate metadata, BoringSSL sources, patches and licences are preserved.

One source change: quote the generated NASM include path in
`deps/boringssl/CMakeLists.txt` so a Windows Cargo output path containing spaces works.
The original failed with MSB3721 and NASM receiving a split `-I` argument in
`D:/codex-build/whiteaesther core23`. The same quoting correction is documented
in the Android integration, but its NDK executable/toolchain changes are not carried.

No TLS, cryptography or certificate-validation code in this dependency is changed.
