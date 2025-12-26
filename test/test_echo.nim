// test/test_echo.nim
// Bu dosya, `echo` fonksiyonunun farklı kullanımlarını test eder.

fn topla(a: i32, b: i32): i32 {
    return a + b;
}

fn main(): i32 {
    echo("--- Echo Testleri Başladı ---");

    // 1. Basit string literal
    echo("Merhaba, dünya!");

    // 2. Değişken interpolasyonu (Bu özellik için codegen güncellenmeli)
    var isim: str = "NIM";
    echo("Benim adım {isim}.");

    echo("--- Echo Testleri Bitti ---");
    
    return 0;
}