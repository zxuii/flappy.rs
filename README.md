# flappy.rs

flappy.rs adalah game cloningan yang dibuat dari 0 menggunakan library bernama `macroquad` dan juga `rand` untuk RNG (karena gw males pake builtin rng macroquad, ribet).
untuk code-qualitynya sendiri, jujur aja sampah, karena banyak hal yang entah gimana itu works, yep. ingat kata pepatah, 

> If it works,
> Don't touch.

Yang jelas, aku aja ga nyangka ini berhasil, sebelumnya aku membuat video di channel [@KentangCeplok](https://youtu.be/HGcTwEuKkmg?si=X2-xZcugKPba3AbT) untuk flappy bird juga, tapi versi yang divideo menggunakan `C++`, kalau ditanya kenapa... ya gatau juga buset, udahlah pake `Raylib` ditambah `C++` lagi, dan yep, proyeknya gagal karena collision handling nya ribet banget dan banyak hal unfinished. aku terselamatkan dengan banyak fitur rust seperti tuple, result type dan lain sebagainya.

## Building

Oh iya hampir lupa, nih cara ngebuildnya:

```bash
cargo build --release
```

Kalo mau nge-run bisa langsung:

```bash
cargo run --release
```

Jangan lupa untuk menginstall rust compiler di [rust-lang.org](https://rust-lang.org/tools/install/) ya!

---

Yang penting Just Works™
