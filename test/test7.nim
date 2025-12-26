
fn main() {
    // 2. Değişkenler ve Sabitler
    var x: i32 = 10;
    //let y = 20; // Tip çıkarımı
    //mut let z: f64 = 3.14;
    const PI = 3.14159;

    // 3. Veri Tipleri
    var tam_sayi: i32 = 100;
    var ondalikli: f64 = 5.5;
    var metin: str = "Merhaba NIM";
    var mantiksal: bool = true;
    var karakter: char = 'A';
    var kucuk_sayi: i8 = 127;

    // 4. Operatörler
    var toplam = x + y;
    var carpim = x * 2;
    var mod = x % 3;
    
    metin .= " - Geleceğin Dili"; // String birleştirme
    
    var esit_mi = (x == 10);
    var buyuk_mu = (y > x);
    var mantiksal_ve = (mantiksal && true);

    // 5. Veri Yapıları
    // Diziler
    var sayilar: arr = [1, 2, 3, 4, 5];
    var dizi2d[2][2]: i32 = [[1, 2], [3, 4]];
    
    // Tuple
    var nokta = (10, 20, "Merkez");
    
    // Vector (yeni sözdizimi)
    var v1 = Vec3[1.0, 2.0, 3.0];
    var v2 = Vec2[x as f64, y as f64];

    // 6. Kontrol Akışı
    // If-Else
    if (x > 5) {
        echo("x 5'ten büyük");
    } else {
        echo("x 5'ten küçük veya eşit");
    }

    // While
    var sayac = 0;
    while (sayac < 3) {
        sayac = sayac + 1;
    }

    // Loop
    loop {
        if (sayac >= 5) break;
        sayac = sayac + 1;
    }

    // For In
    for i in 0..5 {
        // echo(i);
    }

    // Match
    var durum = 200;
    var mesaj = match (durum) {
        200 => "Başarılı",
        404 => "Bulunamadı",
        def => "Hata"
    }

    // Rolling
    rolling:TEST_BLOCK => {
        if ($rolling < 2) {
            rolling:TEST_BLOCK;
        }
    }

    echo("Testler başarıyla tamamlandı!");
}
