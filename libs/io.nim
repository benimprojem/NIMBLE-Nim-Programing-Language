/******************************************************************************
 libs/io.nim
 Bu bir kütüphane modülüdür.
 
 input("bir sayı giriniz? ")
 
 Echo() yerleşik olarak tanımlıdır.
 echo(string:str {:any})
 
 _printf() tüm print işlemlerini yapan asıl fonksiyon.
 
 _printf(string:str {:any},format:any,...);
 
 print(string,error) error, success, warning, info, çıktıyı formatlar.
 print("string:str {:any}",format)
 
 println() çıktı sonuna yeni satır ekler.
 println("string:str {:any}")
 

******************************************************************************/

// Kernel32.dll

// write fonksiyonu yazılacak ve winapi WriteFile () kullanıcak
