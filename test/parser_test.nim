// Parser Testi: Karmaşık Yapılar (Generic, Array, Match)

fn main() {
    // 1. Array Tanımlamaları
    var numaralar:arr = [1, 2, 3, 4, 5];
    
    var saylr: i32[] = [1, 2, 3, 4, 5];
    var sayilar[]: i32 = [1, 2, 3, 4, 5];
    
    var matris[3][3]: i32 = [
        [1, 2, 1],
        [1, 1, 2],
        [1, 2, 1]
    ];
    
    // 2. Vector Tanımlamaları (yeni sözdizimi)
    // büyük bir ihtimalle v1:f64 =  gibi bir şekilde tanımlama gerekecek !!
    var v1 = Vec2[1.0, 2.0];
    var v2 = Vec3[4.0, 5.0, 6.0];
    var v3 = Vec4[4.0, 5.0, 6.0, 2.0];
    
    
    // 3. String birleştirme (.= operatörü)
    var mesaj: str = "Merhaba";
    mesaj .= " Dünya!";
    
    // 4. Match ifadesi (noktalı virgül isteğe bağlı)
    var durum: i32 = 200;
    var sonuc:str = match (durum) {
        200 => "OK",
        404 => "Not Found",
        500 => "Error",
        def => "Unknown"
    }
    
    echo(mesaj);
    echo(sonuc);
}
