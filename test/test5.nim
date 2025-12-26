// test/test5.nim
// Bu dosya, Aşama 5.1'de eklenen özellikleri test eder:
// - cpu modülü ve yerleşik (intrinsic) fonksiyonlar
// - Platforma bağımlı modül yükleme

// cpu modülünü ve içindeki sabitleri/fonksiyonları kullanıma aç
use cpu;
// os modülünü kullan. Bu, hedef platforma göre doğru alt modülü (örn: os/windows) yükleyecektir.
use os;

fn main(): void {
    echo("--- Aşama 5.1 Test Başladı ---");
    echo("CPU modülü ve yerleşik fonksiyonlar testi...");

    // Bu işlemler doğrudan donanıma müdahale ettiği için
    // hem 'unsafe' hem de performans odaklı olduğu için 'fastexec'
    // blokları içinde olmalıdır.
    unsafe {
        fastexec {
            // Bu kodun gerçek assembly'e çevrilmesi codegen aşamasında olacak.
            // Şimdilik sadece ayrıştırılıp tip kontrolünden geçmesi yeterli.
            cpu.mov(cpu.RAX, 60); // RAX register'ına 60 değerini taşı
            cpu.syscall();        // Sistem çağrısı yap

            echo("CPU yerleşikleri (mov, syscall, RAX) başarıyla ayrıştırıldı.");
        }

        // Platforma özel modül testi (Windows için)
        // 'os.nim' -> 'os/platform.nim' -> 'os/windows.nim' yolunu izleyerek 'windows' grubuna erişir.
        // Bu fonksiyon çağrısı sadece tip kontrolü içindir, çalıştırılmayacaktır.
        windows.Beep(440, 500); // 440 Hz (A4 notası) frekansında 500ms ses çal.
        echo("Platforma özel fonksiyon (windows.Beep) başarıyla ayrıştırıldı.");
    }

    echo("--- Aşama 5.1 Test Bitti ---");
}