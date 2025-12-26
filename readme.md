# NIMBLE Derleyici Projesi (NimCompiler)

**Proje:** NIM (NIMBLE) Derleyicisi
**Durum:** Erken Geliştirme (Skeleton/Taslak Aşaması)

Bu belge, derleyicinin mevcut teknik durumunu, eksiklerini ve geliştirme yol haritasını en ince teknik detayına kadar listeler.

---

## 🛠️ 1. Kurulum ve Gereksinimler

Projenin derlenebilmesi için aşağıdaki araçlar **ZORUNLUDUR**.

---

## 📊 2. Mevcut Teknik Durum (Detaylı Analiz)

### 2.1 Lexer & Parser (Ön Yüz)
*   **Durum:** %85 Tamamlandı.
*   **Yetenekler:** Değişkenler, fonksiyonlar, struct/enum tanımları, akış kontrolü (`if/for/while`), `rolling` bloğu, fonksiyonları ayrıştırılabiliyor.
*   **Eksikler:** Karmaşık Generic sözdizimi ve bazı operatör öncelikleri test edilmeli.

### 2.2 Type Checker (Semantik Analiz)
*   **Durum:** %60 Tamamlandı.
*   **Yetenekler:**
    *   Değişken kapsamı (scope) takibi.
    *   Fonksiyon imza kontrolü.
    *   Basit tip uyuşmazlığı kontrolü (örn: `i32` yerine `str` atama).
    *   `struct` ve `enum` tanımlarının tanınması.
*   **Eksikler:**
    *   Pointer (`ptr`) aritmetiği kontrolü.
    *   Generic tip çıkarımı (Type Inference) çok temel seviyede.
    *   Borrow checker benzeri sahiplik kuralları ama daha esnek.

### 2.3 Codegen  🟥 KRİTİK EKSİK
*   **Durum:** %5 (Sadece İskelet).
*   **Mevcut Yetenekler:**
    *   Basit fonksiyon (`main`) çatısı oluşturma.
    *   Sadece `i64` tamsayı literalleri ve basit aritmetik (`+`, `-`, `*`, `/`) işlemleri.
    
*   **Eksik/Çalışmayan Parçalar:**
    *   Wait for implement: **Değişken Tanımlama:** `alloca` ve `store` komutları yok. Değişkenler bellekte yer kaplamıyor.
    *   Wait for implement: **Atama (`Assign`):** Değişkenlere değer atanamıyor.
    *   Wait for implement: **Echo/Print:** `printf` veya `puts` entegrasyonu yok (Sadece yorum satırı var).
    *   Wait for implement: **Akış Kontrolü:** `If`, `While`, `For` döngüleri için Temel Blok (Basic Block) ve Dallanma (Branch) mantığı **YOK**.
    *   Wait for implement: **Fonksiyon Çağrıları:** Parametre iletimi yok.
    *   Wait for implement: **String/Struct:** Karmaşık tipler tanımlı değil.
    
### 2.4 io.nim  🟥 KRİTİK EKSİK
    *   _printf fonksiyonu ve yardımcı fonksiyonları yazılacak.
    *   input   fonksiyonu yazılacak.
---

## 📝 3. Teknik Yol Haritası (TODO)

Sırasıyla yapılması gereken teknik görevler:

### Aşama 1: "Hello World" ve Temel Değişkenler (ÖNCELİKLİ)
Bu aşama, derleyicinin en basit programı çalıştırabilir hale gelmesi içindir.

1.  **Printf Entegrasyonu (`codegen.rs`):**
    *   Global string sabiti oluşturma mantığını ekle (format stringleri için).
    *   `Stmt::Echo` işleyicisini `_printf` çağıracak şekilde güncelle.
    
2.  **Stack Bellek Yönetimi:**
    *   `Stmt::VarDecl` için: Giriş bloğunda `build_alloca` ile yığın (stack) belleği ayır.
    *   Sembol tablosuna (`variables` HashMap) bu adresi kaydet.
    *   Başlangıç değeri varsa `build_store` ile değeri yaz.
    
3.  **Değişken Erişimi:**
    *   `Expr::Variable` için: Sembol tablosundan adresi bul ve `build_load` ile değeri oku.

### Aşama 2: Akış Kontrolü (Flow Control)
1.  **If - Else:**
    *   `then`, `else`, `merge` bloklarını oluştur.
    *   `build_conditional_branch` ile koşula göre zıplama mantığını kur.
    *   PHI node (seçim düğümü) ihtiyacını belirle.
    
2.  **Döngüler (While/Loop):**
    *   `check` ve `body` blokları oluştur.
    *   Koşul kontrolü ve döngü başı zıplamalarını ekle.

### Aşama 3: Fonksiyonlar
1.  **Fonksiyon Tanımı:**
    *   Parametrelerin LLVM tiplerine dönüştürülmesi.
    *   Parametrelerin fonksiyon girişinde stack'e kopyalanması (Mutable argümanlar için).
    
2.  **Return:**
    *   `Stmt::Return` için `build_return` ekle.

### Aşama 4: İleri Seviye Tipler
1.  **String:** `Struct { len: i64, data: i8* }` şeklinde  tipi oluştur.
2.  **Struct:** Kullanıcı tanımlı struct'ları struct tiplerine map et.

---

## 4. Hata Ayıklama Notları
*   `Decl::Function` içindeki `ret_type` alanı düzeltildi.
*   Codegen şu an sadece `i64` üzerinden gidiyor, `i32` ve diğerleri için tip dönüşümü (cast) gerekecek.
