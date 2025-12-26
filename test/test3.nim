// test/test2.nim
// Modül sistemini test eder.

// 1. Yöntem: Joker karakter ile tüm pub öğeleri içeri aktar
use math::*;

// 2. Yöntem: Belirli öğeleri takma ad ile içeri aktar
// use math::{topla as add, cikar}; sorunlu...

// 3. Yöntem: Modülü bir isim alanı olarak takma ad ile içeri aktar
// use math as m;

fn main(): void {
    echo("--- MODÜL SİSTEMİ TESTİ BAŞLADI ---");

    // `use math::*;` sayesinde `topla` doğrudan kullanılabilir.
    var sonuc1: i32 = topla(10, 5);
    echo("topla(10, 5) = {sonuc1}"); // 15 yazdırmalı

    // `cikar` da doğrudan kullanılabilir.
    var sonuc2: i32 = cikar(10, 5);
    echo("cikar(10, 5) = {sonuc2}"); // 5 yazdırmalı

    // Sabit de doğrudan kullanılabilir.
    echo("PI değeri: {PI}");

    // Struct da doğrudan kullanılabilir.
    var v: Vec2 = Vec2 { x: 1.0, y: 2.0 };
    echo("Vektör: ({v.x}, {v.y})");

    echo("--- MODÜL SİSTEMİ TESTİ BAŞARILI ---");
}