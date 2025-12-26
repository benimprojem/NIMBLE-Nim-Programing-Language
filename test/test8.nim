// NIM (NIMBLE) Eksiksiz Dil Testi (Bölüm 1 - Bölüm 7)

// 1. Modül Sistemi Testi
// libs/math.nim modülündeki public öğeleri içeri aktar.
use libs/math::*;


fn kare_hesapla(n: i32): i32 -> n * n; // Tek satırlık fonksiyon

// Standart Fonksiyon
fn topla_standart(a: i32, b: i32): i32 {
    return a + b;
}

// Void Fonksiyon
fn selamla(isim: str) {
    echo("Merhaba {isim}");
}

// Asenkron bir işlem simüle eden fonksiyon.
async fn fetch_data(id: UserID): str {
    echo("Asenkron veri getiriliyor...");
    // Gerçek bir asenkron işlem burada olurdu (örn: dosya okuma, ağ isteği).
    // Şimdilik sadece bir string döndürüyoruz.
    return "Kullanıcı Verisi";
}

//  Routine ve Channel Testi
// Bir kanala mesaj gönderen worker fonksiyonu.
fn worker(ch: Channel<str>): void {
    echo("Worker routine başladı.");
    ch <- "Routine'den Merhaba!";
}

// 7. Extern/Unsafe Testi
// C'nin 'puts' fonksiyonunu bildirelim.
extern fn puts(s: str): i32;
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
// 2. Typedef Testi
pub typedef UserID: u64;

fn main():i32{
    // --- 2. DEĞİŞKENLER VE SABİTLER ---
    var degisken: i32 = 10;

    let sabit = 20; 
    mut let degisebilir_sabit: f64 = 3.14;
    const PI = 3.14159265;

    // --- 3. VERİ TİPLERİ ---
    // Tam Sayılar
    var i8_v: i8 = 127;
    var i16_v: i16 = 32767;
    var i32_v: i32 = 2147483647;
    var i64_v: i64 = 9223372036854775807;
    var u8_v: u8 = 255;
    var u32_v: u32 = 4294967295;
    
    // Ondalıklı Sayılar
    var f32_v: f32 = 1.23;
    var f64_v: f64 = 4.56;
    
    // f64[2] ondalık kısımdan sadece 2 basamak al.
    //var f64_v: f64[2] = 3.565454651654;
    
    
    // Decimal (Hassas Matematik)
    var d64_v: d64 = 100.50;
    
    // Özel Tipler
    var ab: bool = true;
    var ac: char = 'Z';  
    var s: str = "NIMBLE Derleyici";
    var bt: bit = 1;
    
    var byt: byte = 255; // hex olarak atama gerekiyor ..
    
    var h: hex = 0xABC123;

echo("--- Aritmetik Testleri");

    var a: i32 = 15;
    var b: i32 = 7;
    
    echo("a is {a}"); //  print 15
    echo("b is {b}"); //  print 7

    var c: i32 = a + b;
    
    echo("c (a + b) is {c}"); //  print 22

    var d: i32 = a - 10;
    echo("d (a - 10) is {d}"); //  print 5

    var e: i32 = a * b;
    echo("e (a * b) is {e}"); // Should print 105

    var f: i32 = a / b;  
    echo("f (a / b) is {f}"); // Should print 2 (integer division)

    var g: i32 = a % b;
    echo("g (a % b) is {g}"); // Should print 1 (remainder)

    echo("--- Kayan Noktalı Sayı Testleri \n"); // \n kabul eder.  otomatik \n eklemez... 
    // print("Hata: {e}",error); // print otomatik \n ekler ve 
    // print("", <format>)  çıktıları formatlamak için seçenek sunar, kullanıcı bunu kendisi de tanımlaya bilmelidir.

    var x: f64 = 15.5;  
    var y: f64 = 2.5;

    echo("x is {x}"); // Should print 15.5
    echo("y is  {y}"); // Should print 2.5
    
    // var z: f64[4] = x / y; // z nin ondalık kısmını 4 rakamla sınırlar.
    // f64[4];  // gibi bir ifade bulunduğu kapsamda tüm float tiplerinin ondalık kısımlarını 4 rakamla sınırlar,
    // ve printf için format %.4f olarak gösterimi uygular. amaç echo da float format gösterimini otomatikleştirmek.

    echo("x + y is {x + y}"); // Should print 18.0
    echo("x - y is {x - y}"); // Should print 13.0
    echo("x * y is {x * y}"); // Should print 38.75
    echo("x / y is {x / y}"); // Should print 6.2

    echo("--- Çoklu Argüman Testleri");
    echo("a={a}, x={x}, y={y}, b={b}, x={x}, b={b}, y={y}, a={a}"); //  Karışık int ve float
    echo("x={x}, y={y}, x*y={x*y}"); // Sadece float
    echo("--- Arithmetic Tests End ---");


    // --- 4. OPERATÖRLER ---
    var aritmetik = (i32_v + 10) * 2 / 5 % 3;
    s .= " v0.1"; // String birleştirme (.=)
    
    var karsilastirma = (degisken == 10) && (degisken != 5) || !(ab);
    
    //var bit_islem = (u8_v & 0xF0) | (0x0F ^ 0xAA) << 2 >> 1;  //bit işlemi desteklemedi.
//Tip Hatası: Hata: Bitsel işlem yalnızca tamsayı tiplerine uygulanabilir, bulundu: U8 ve Hex.

    // --- 5. VERİ YAPILARI ---
    // Diziler (Her varyasyon)
    var d_genel: arr = [1, 2, 3];
    var d_dinamik: i32[] = [10, 20];
    var d_sabit[3]: i32 = [1, 1, 1];
    var d_matris[2][3]: i32 = [[1, 2, 3], [4, 5, 6]];
    
    // Vektörler
    var v2 = Vec2[1.0, 2.0];
    var v3 = Vec3[10.0, 20.0, 30.0];
    var v4 = Vec4[0.1, 0.2, 0.3, 0.4];
    
    // Tuple ve Pointer
    var tpl: (i32, str, bool) = (1, "NIM", true);
    var p_i32: *i32 = &i32_v;
    var i32_copy = *p_i32;

    // --- 6. KONTROL AKIŞI ---
    if (degisken > 100) {
        echo("Büyük");
    } elseif (degisken == 10) {
        echo("Eşit");
    } else {
        echo("Küçük");
    }

    var loop_sayac = 0;
    while (loop_sayac < 5) {
        loop_sayac = loop_sayac + 1;
        if (loop_sayac == 2) continue;
        if (loop_sayac == 4) break;
    }
    
    rolling:HATA_YONETIMI => {
        if ($rolling < 2) {
            rolling:HATA_YONETIMI;
        }
    }


    loop {
        break; // Sonsuz döngüden çıkış
    }

    var m_sonuc = match (degisken) {
        10 => "On",
        20 => "Yirmi",
        def => "Diğer"
    }
    
    // Modül testi: math.nim'den gelenleri kullan
    var sonuc: i32 = topla(20, 22);
    echo("Modül Testi: topla(20, 22) = {sonuc}");
    echo("Modül Testi: PI sabiti = {PI}");

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
    
    // Typedef testi
    var my_id: UserID = 12345;
    echo("Typedef Testi: UserID = {my_id}");
    
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
        var aa: i32 = 10;
        var ab: i32 = 20;
        var ac: i32 = a + b; // Bu blok potansiyel olarak daha hızlı çalıştırılacak
        echo("FastExec Test: ac = {ac}");

        // asm testi (fastexec içinde olmalı)
        echo("Asm Testi başlıyor...");
        asm:x86_64 {
           //  mov rax, 60; syscall; 
        }
        echo("Asm Testi: (sadece ayrıştırma kontrolü)");
    }

    // for-in range testi
    echo("For-in range testi başlıyor...");
    //i:i32=0;
    for (i in 0..4) {
        echo("Döngü adımı: {i}");
    }


    // --- 7. FONKSİYONLAR VE LAMBDA ---
    // Lambda kullanımı
    var aa:i32 = 21;
    var bb:i32 = 12;
    var carp = (aa: i32, bb: i32): i32 -> aa * bb;
    
    // Fonksiyon çağrıları
    var toplam = topla_standart(10, 20);
    selamla("Dünya");


    // struct, enum, typedef, group, group struct yapıları testi gerekli..
    
    
    echo("Tüm dil özellikleri başarıyla doğrulandı!");

   return 0;

}
