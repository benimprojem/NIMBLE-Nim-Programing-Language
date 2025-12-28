# NIMBLE Programming Language

**NIMBLE** (NIM Programlama Dili), sistem programlama için tasarlanmış, modern ve performanslı bir programlama dilidir.

## 🚀 Özellikler

- ✅ **Statik Tip Sistemi** - Güçlü tip kontrolü ve tip çıkarımı
- ✅ **Heterogeneous Arrays** - `arr` tipi ile farklı tipleri aynı dizide saklama
- ✅ **Modern Sözdizimi** - C, Rust ve Python'dan ilham alan temiz syntax
- ✅ **ANSI Stil Sistemi** - Renkli terminal çıktıları için yerleşik destek
- ✅ **GCC/GAS Backend** - Doğrudan assembly üretimi
- ✅ **Windows x64 ABI** - Tam uyumlu fonksiyon çağrıları

## 📦 Kurulum

### Gereksinimler
- Rust (1.70+)
- GCC (MinGW-w64 Windows için)

### Derleme
```bash
cargo build --release
```

## 🎯 Hızlı Başlangıç

### Merhaba Dünya
```nim
fn main(): i32 {
    echo("Merhaba, NIMBLE!\n");
    return 0;
}
```

### Diziler ve Döngüler
```nim
fn main(): i32 {
    var sayilar: arr = [1, 2, 3, 4, 5];
    
    for (sayi in sayilar) {
        echo("{sayi} ");
    }
    echo("\n");
    
    return 0;
}
```

### Renkli Çıktı
```nim
style basari = "\x1b[32m";

fn main(): i32 {
    print("İşlem başarılı!\n", "success");
    print("Özel stil\n", "basari");
    return 0;
}
```

## 📚 Dil Özellikleri

### Tipler
- **Tamsayılar**: `i8`, `i16`, `i32`, `i64`, `i128`, `u8`, `u16`, `u32`, `u64`, `u128`
- **Ondalık**: `f32`, `f64`
- **Diğer**: `bool`, `char`, `str`, `arr`, `void`

### Kontrol Yapıları
- `if-else` koşulları
- `while` döngüsü
- `for` döngüsü (range ve for-in)
- `loop` sonsuz döngü
- `break` ve `continue`

### Fonksiyonlar
- Tip güvenli parametreler
- Dönüş tipi kontrolü
- Recursion desteği

## 🧪 Test

Test dosyalarını çalıştırmak için:
```bash
cargo run -- test/test1.n
```

20 kapsamlı test dosyası mevcuttur (test1-test20).

## 📈 Geliştirme Durumu

**Mevcut Kapsam**: ~75%

### ✅ Tamamlanan
- Lexer & Parser
- Type Checker
- Codegen (temel)
- Array desteği
- String interpolation
- ANSI stil sistemi

### 🚧 Devam Eden
- Bitwise operatörler
- Pointer semantiği
- Match ifadeleri
- Enum codegen

### 📅 Planlanan
- Lambda fonksiyonlar
- Hata yönetimi (Result/Option)
- Inline assembly
- Async/await

## 🤝 Katkıda Bulunma

Katkılarınızı bekliyoruz! Lütfen bir issue açın veya pull request gönderin.

## 📄 Lisans

MIT License

## 👨‍💻 Geliştirici

NIMBLE, modern sistem programlama ihtiyaçları için geliştirilmektedir.

---

**Not**: Bu proje aktif geliştirme aşamasındadır. Production kullanımı için henüz hazır değildir.


