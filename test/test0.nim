// ==================================================
// NIMBLE Derleyici - Kapsamlı Test Dosyası
// ==================================================

// --- Test 1: Fonksiyon Tanımlama ve Çağırma ---

// Dönüş değeri olmayan basit bir fonksiyon
fn selamla(): void {
    echo("Test 1: Fonksiyonlar calisiyor.\n");
}

// Parametre alan ve değer döndüren bir fonksiyon
fn topla(a: i32, b: i32): i32 {
    var t:i32 = a + b;
    return t;
}

// --- Test 6.1: `for` Döngüsü ---
fn for_dongusu_testi1(): void {
    echo("\nTest 6.1: for dongusu...");
    // i otomatik -> 
    var i: i32;  
    for (i = 0, i < 5, i++) {
        echo("{i}, ");
    }
}

// --- Test 6.2: `for` Döngüsü ---
fn for_dongusu_testi2():void {
    echo("\nTest 6.2: for dongusu...");
    colors: arr = ["red", "green", "blue", "yellow"];
    for (color in colors) {
        if (color == "blue") break;
        echo("{color}, ");
    }
}

// --- Test 6.3: `for` Döngüsü ---
fn for_dongusu_testi3():void {
    echo("\nTest 6.3:  while(true) break loop dongusu...\n");
    var i: i32 = 0;
    while (true) {
        if (i == 5) {break;}
        i += 1;
        echo("{i}, ");
    }
}


fn main(): i32 {
    echo(">>> Testler Basliyor...\n");
    
    selamla(); // Basit fonksiyon çağrısı

    // --- Test 2: Fonksiyon Dönüş Değerini Değişkene Atama ---
    echo("\nTest 2: Fonksiyon donus degeri atamasi...\n");
    var sonuc: i32 = topla(15, 27);
    echo(sonuc); // Ekrana 42 yazdırmalı

    // --- Test 3: `if-elseif-else` Kontrol Yapısı ---
    print("\nTest 3: if-elseif-else kontrolu...\n");
    var skor: i32 = 85;
    if (skor > 90) {
        echo("Not: A");
    } elseif (skor > 75) {
        echo("Not: B  \n"); // Bu bloğun çalışması beklenir
    } else {
        echo("Not: C");
    }

    // --- Test 4: `while` Döngüsü ---
    print("\nTest 4: while dongusu...\n");
    var sayac: i32 = 0;
    while (sayac < 3) {
        //print();
        echo("Sayac: {sayac} \n");
        sayac += 1;
    }

    // --- Test 5: Değişkenler ve Aritmetik İşlemler ---
    print("Test 5: Degiskenler ve aritmetik...\n");
    var x: i32 = 100;
    var y: i32 = (x / 2) + 5; // 55
    echo(y);
 
    for_dongusu_testi1();
    for_dongusu_testi2();
    for_dongusu_testi3();

    // ==================================================
    // GELECEK ÖZELLİKLER İÇİN TESTLER (HENÜZ DESTEKLENMİYOR)
    // ==================================================


    // --- Test 7: `match` İfadesi ---
    fn match_testi(kod: i32): void {
        print("Test 7: match ifadesi...\n");
         match (kod) {
            200: { echo("Basarili"); },
            404: { echo("Bulunamadi"); },
            def: { echo("Bilinmeyen kod"); }
        }
    }



    echo("\n>>> Testler Bitti.\n");
    
    return 0;
}
