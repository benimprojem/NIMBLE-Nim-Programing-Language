// libs/os/windows.nim
//
// Bu dosya, sadece Windows platformu hedeflendiğinde 'os' modülüne dahil edilecek
// olan Windows'a özel fonksiyonları içerir.

export group windows {
    // Windows'a özel bir sistem sesi çalma fonksiyonu (kernel32.dll'den)
    pub extern fn Beep(dwFreq: u32, dwDuration: u32): bool;
}