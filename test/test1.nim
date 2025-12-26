// test1.nim - AŞAMA 1 Özellik Testleri
// Bu dosya, nimtask.md'deki birinci aşamada tamamlanan tüm özellikleri test eder.

// Dönüş değeri olmayan basit bir fonksiyon
fn selamla(): void {
    echo("Test 1: Fonksiyonlar calisiyor.\n");
}

// Parametre alan ve değer döndüren bir fonksiyon
fn topla(a: i32, b: i32): i32 {
    return  a + b;
}


fn for_dongusu_testi1(): void {
    echo("\nTest 6.1: for dongusu...");
    for (i = 0, i < 5, i++) {
        echo("{i}, ");
    }
}


fn for_in_dongusu():void {
    echo("\nTest 6.2: for dongusu...");
    var colors: arr = ["red", "green", "blue", "yellow"];
    for (color in colors) {
        if (color == "blue") break;
        echo("{color}, ");
    }
}


fn while_true_testi():void {
    echo("\nTest 6.3:  while(true) break loop dongusu...\n");
    var ii: i32 = 0;
    while (true) {
        if (ii == 5) {break;}
        ii += 1;
        echo("{ii}, ");
    }
}


fn match_testi(kod: i32): void {
    echo("Test 7: match ifadesi...\n");
    match (kod) {
        200 => { echo("match_testi Basarili"); },
        404 => { echo("match_testi Bulunamadi"); },
        def => { echo("match_testi Bilinmeyen kod"); }
    }
}

struct Point {
    x: i32;
    y: i32;
}
/*
// YENİ: Point struct'ı için metotları tanımlayan group bloğu
group Point {
    distance => fn (self: Point): i32 -> {return 0;};
    add      => fn (self: Point): i32 ->{ 
                    // Değişkeni 'var' ile tanımla
                    var retx: i32 = self.x + self.y;
                    return retx;
                };
}
*/
enum Color {
    Red,    // Otomatik olarak 0
    Green,  // Otomatik olarak 1
    Blue    // Otomatik olarak 2
}

enum HttpStatus {
    Ok = 200,
    NotFound = 404
}

fn check_status(code:i32) {
    if (code == HttpStatus::Ok) {
        echo("Başarılı!");
    }
}


// Temel bir tip için takma ad
typedef UserID: u64;

// Karmaşık bir tuple tipi için takma ad (Bu hala geçerli)
typedef PointTuple: (i32, i32);

// YENİ ve DAHA İYİ SÖZDİZİMİ: Dizi tipi için takma ad
typedef Path[]: Point; // Path, bir Point struct dizisidir.

fn process_id(id: UserID) {
    echo("İşlenen Kullanıcı ID: {id}");
}

fn print_point(p: PointTuple) {
    echo("Nokta Tuple: ({p.0}, {p.1})");
}

fn print_point_struct(p: Point) {
    echo("Nokta Struct: (x={p.x}, y={p.y})");
}


fn main(): i32 {
    echo("--- AŞAMA 1: TEMEL SÖZDİZİMİ VE YAPILAR TESTİ BAŞLADI ---");

    // --- Veri Tipleri ve Değişkenler ---
    echo("1. Veri Tipleri ve Değişkenler Testi...");

    // Temel Tamsayılar
    //var my_i8: i8 = -100;  //Hata: 'my_i8' değişkenine atanmaya çalışılan tip (I32), deklare edilen tip (I8) ile uyuşmuyor.
    //var my_i16: i16 = 99999; //Hata: 'my_i16' değişkenine atanmaya çalışılan tip (I32), deklare edilen tip (I16) ile uyuşmuyor.
    var my_i32: i32 = 1166;
    var my_i64: i64 = 999999999;
    var my_i128: i128 = 999999999999;
    
    // İşaretsiz Tamsayılar
    var my_u8: u8 = 100;
    var my_u16: u16 = 9999;
    var my_u32: u32 = 2500;
    var my_u64: u64 = 999999999;
    var my_u128: u128 = 89988999999999;
    
    
    // Temel Kayan Noktalı Sayılar
    var my_f32: f32 = 3.14;
    var my_f64: f64 = 3.1415926535;
    var my_f80: f80 = 66.1415926535;
    var my_f128: f128 = 3545.1415926535;

    // Boolean ve String
    var is_active: bool = true;
    
    var message: str = "Merhaba, NIM!";

    // Değişken Tanımlama ve Atama
    var score: i32 = 0;
    score = 100;

    // Sabit Tanımlama
    const PI: f64 = 3.14159;
    //mut PI:f64 = 3.14159265;


    // Özel Tipler
    var initial: char = 'N'; 
    var flag[32]: bit = 10010110100101101001011010010110;
    // 32bit  bit[bitsayısı]/8 = 4 byte - bir bit tek başına bellekte saklanamaz en az 8 bit yani bir byte olmalı.
    // kontrol bitsayısı/8  tam bölünebilir olmalı ve sonuç kaç byte olduğunu verir
    
    var data: byte = 255; // 11111111 yada 0xFFFF  gibi
    var colorr: hex = 0xFF;

    // 'any' Tipi
    var dynamic_var: any = 10;
    dynamic_var = "şimdi bir string";

    // 'mut' Anahtar Kelimesi
    var mutable_val: i32 = 1;
    mutable_val = 2; // 'var' varsayılan olarak değiştirilebilir olduğu için bu geçerlidir.

    // 'null' Değeri
    var maybe_null:any = null; // null = ""  boş bir değer yada olmayan değer.. 

    // --- Operatörler ---
    echo("2. Operatörler Testi...");

    // Aritmetik Operatörler
    var x: i32 = 20;
    var y: i32 = 5;
    var arithmetic_res: i32 = (x + y) * (x - y) / (x % y + 1); // (25 * 15) / (0 + 1) = 375
    
    
    var xx:f32 = 2.222;
    var yy:f32 = 16.444;
    var zz: f32 = (yy/2)*(xx-yy) + ((yy-xx)*4);//


    // Karşılaştırma ve Mantıksal Operatörler
    var is_greater: bool = x > y;
    
    var is_equal: bool = x == 20;
    
    if (is_greater && is_equal) {
        echo("Karşılaştırma doğru.");
    }

    // Artırma/Azaltma (Postfix ve Prefix)
    x++;
    //--y;
    y--;


    // Bileşik Atama Operatörleri
    var total: i32 = 100;
    total += 50; // total = 150
    total /= 2;  // total = 75

    // Bitsel Operatörler
    var op1: i32 = 10; // 1010
    var op2: i32 = 12; // 1100
    var and_res: i32 = op1 & op2;  // 1000 = 8
    var or_res: i32 = op1 | op2;   // 1110 = 14
    var lshift: i32 = op1 << 1;    // 10100 = 20

    // Kimlik Operatörleri
    if (x === 21) {
        echo("Kimlik operatörü doğru.");
    }
    if (y !== 5) {
        echo("Kimlik değil operatörü doğru.");
    }

    // --- Kontrol Akışı ---
    echo("3. Kontrol Akışı Testi...");

    // if/elseif/else
    if (total > 100) {
        echo("Total 100'den büyük.");
    } elseif (total == 75) {
        echo("Total 75'e eşit.");
    } else {
        echo("Total 100'den küçük.");
    }


    // Ternary Operatör iptal.
   // var status:str = (is_active) ? "Aktif" : "Pasif"; // problemli.
    //echo(status);


    selamla(); // Basit fonksiyon çağrısı

    echo("\nTest 2: Fonksiyon donus degeri atamasi...\n");
    var sonuc: i32 = topla(15, 27);
    echo(sonuc); // Ekrana 42 yazdırmalı

    echo(topla(12, 57)); // Ekrana 69 yazdırmalı

    echo("\nTest 4: while dongusu...\n");
    var sayac: i32 = 0;
    while (sayac < 3) {
        echo("Sayac: {sayac} \n");
        sayac += 1;
    }

 
    for_dongusu_testi1();
    
    for_in_dongusu();
    
    while_true_testi();

    var kod:i32 = 200;
    match_testi(kod);  // Match ifadesindeki desen tipi (Unknown), kontrol edilen ifadenin tipiyle (I32) uyuşmuyor.

    // --- AŞAMA 3 TESTLERİ ---
    echo("\n--- AŞAMA 3: YARDIMCI FONKSİYONLAR TESTİ BAŞLADI ---");

    echo("\nTest 8: print fonksiyonu...");
    print("Bu bir hata mesajıdır.", "error");
    print("Bu bir uyarıdır.", "warning");
    print("Bu bir başarı mesajıdır.", "success");
    print("Bu normal bir bilgidir.", "info");

    echo("\nTest 9: input fonksiyonu...");
    echo("Lütfen adınızı girin: ");
    var ad: str = input();
    echo("Merhaba, {ad}!");

    echo("\nTest 10: strlen ve arrlen fonksiyonları...");
    var test_str: str = "NIM dili"; // 8 karakter
    var str_uzunluk: i32 = strlen(test_str);
    echo("'{test_str}' string uzunluğu: {str_uzunluk}"); // 8 yazdırmalı

    var test_arr: arr = [10, 20, 30, 40]; // 4 eleman
    var arr_uzunluk: i32 = arrlen(test_arr);
    echo("Dizinin eleman sayısı: {arr_uzunluk}"); // 4 yazdırmalı

 // --- AŞAMA 4 TESTLERİ ---
    echo("\n--- AŞAMA 4: YARDIMCI FONKSİYONLAR TESTİ BAŞLADI ---");

    echo("\n Struct test. \n");
    // YENİ: Struct oluşturma ifadesi kullanımı
    var p: Point = Point { x: 10, y: 20 };
    var x_val: i32 = p.x;
    echo("Noktanın x değeri: {x_val} \n");

    // Metot çağrısı testi (şimdilik sadece tip kontrolü)
    //var dist: f64 = p.distance();
    //echo("Mesafe (test): {dist}");
    
    echo("\n Enum test. \n");
    check_status(200);
    
    echo("red code: {Color::Red}\n");
    
    
    // 1. Temel takma ad ile değişken tanımlama
    var my_id: UserID = 12345;
    process_id(my_id);

    // 2. Karmaşık takma ad ile değişken tanımlama ve atama
    var start_point: PointTuple = (10, 20);
    print_point(start_point); // Artık PointTuple bekleyen fonksiyona PointTuple gönderiliyor.

    // 3. İç içe takma ad ile değişken tanımlama
    var my_path: Path = []; // Boş bir Path (Point dizisi)

    // 4. For döngüsünde takma ad kullanımı
    echo("Yol üzerindeki noktalar:");
    for (p in my_path) {
        print_point_struct(p); // Artık Point struct'ı bekleyen doğru fonksiyon çağrılıyor.
    }

    echo("typedef -> Testler tamamlandı.\n");

    echo("\n clone test. \n");
    var p1: Point = Point { x: 100, y: 200 };
    var p2: Point = clone(p1);
    p2.x = 300; // p2'deki değişiklik p1'i etkilememeli.

    echo("p1.x = {p1.x}"); // 100 yazdırmalı
    echo("p2.x = {p2.x}"); // 300 yazdırmalı

        
    
    
    
    
    
    
    
    
    panic("hata 123");
    echo("\nTest 11: exit fonksiyonu...");
    echo("Bu mesajı göreceksiniz...");
    exit(0); // Program burada başarı kodu (0) ile sonlanacak. Bu satır aktifken sonraki kodlar çalışmaz.

    
    echo("\n--- TÜM TESTLER BAŞARIYLA TAMAMLANDI ---");
    return 0;
}