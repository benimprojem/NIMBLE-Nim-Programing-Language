// test/test4.nim
// Bu dosya, Aşama 4'te eklenen özellikleri test eder:
// - Modül sistemi (use, pub)
// - Struct, Enum, Typedef
// - clone() fonksiyonu
// - async/await

// 1. Modül Sistemi Testi
// libs/math.nim modülündeki public öğeleri içeri aktar.
use libs/math::*;

// 2. Typedef Testi
pub typedef UserID: u64;

// 3. Enum Testi
pub enum Status {
    Idle,
    Running,
    Completed,
    Failed,
}

// 4. Struct Testi
pub struct TestStruct {
    id: i32;
    name: str;
    status: Status;
}

// 5. Async/Await Testi
// Asenkron bir işlem simüle eden fonksiyon.
async fn fetch_data(id: UserID): str {
    echo("Asenkron veri getiriliyor...");
    // Gerçek bir asenkron işlem burada olurdu (örn: dosya okuma, ağ isteği).
    // Şimdilik sadece bir string döndürüyoruz.
    return "Kullanıcı Verisi";
}

// 6. Routine ve Channel Testi
// Bir kanala mesaj gönderen worker fonksiyonu.
fn worker(ch: Channel<str>): void {
    echo("Worker routine başladı.");
    ch <- "Routine'den Merhaba!";
}

// 7. Extern/Unsafe Testi
// C'nin 'puts' fonksiyonunu bildirelim.
extern fn puts(s: str): i32;

// Ana giriş noktası
async fn main(): void {
    echo("--- Aşama 4 Test Başladı ---");

    // Modül testi: math.nim'den gelenleri kullan
    var sonuc: i32 = topla(20, 22);
    echo("Modül Testi: topla(20, 22) = {sonuc}");
    echo("Modül Testi: PI sabiti = {PI}");

    // Typedef testi
    var my_id: UserID = 12345;
    echo("Typedef Testi: UserID = {my_id}");

    // Struct ve Enum testi
    var s1: TestStruct = TestStruct {
        id: 1,
        name: "İlk Nesne",
        status: Status::Running,
    };
    echo("Struct Testi: s1.name = {s1.name}");

    // clone() testi
    var s2: TestStruct = clone(s1);
    s2.name = "Klonlanmış Nesne";
    echo("Clone Testi: s1.name = {s1.name}, s2.name = {s2.name}");

    // async/await testi
    var data: str = await fetch_data(my_id);
    echo("Async/Await Testi: Gelen veri = {data}");

    // routine ve channel testi
    // Not: `make_channel` henüz jenerik tipleri tam desteklemediği için `Channel<str>`'a atama yapıyoruz.
    var ch: Channel<str> = make_channel(0); // Kapasitesiz (bloklayan) kanal
    routine worker(ch);
    var msg: str = <-ch; // Worker'dan gelen mesajı bekle ve al.
    echo("Channel Testi: Gelen mesaj = {msg}");

    // unsafe ve extern testi
    echo("Unsafe/Extern Testi başlıyor...");
    unsafe {
        puts("Unsafe/Extern Test: Merhaba, C dünyası!");
    }

    // fastexec testi
    echo("FastExec Testi başlıyor...");
    fastexec {
        var a: i32 = 10;
        var b: i32 = 20;
        var c: i32 = a + b; // Bu blok potansiyel olarak daha hızlı çalıştırılacak
        echo("FastExec Test: c = {c}");

        // asm testi (fastexec içinde olmalı)
        echo("Asm Testi başlıyor...");
        asm:x86_64 { /* mov rax, 60; syscall; */ }
        echo("Asm Testi: (sadece ayrıştırma kontrolü)");
    }

    // for-in range testi
    echo("For-in range testi başlıyor...");
    //i:i32=0;
    for (i in 0..4) {
        echo("Döngü adımı: {i}");
    }

    echo("--- Aşama 4 Test Bitti ---");
}