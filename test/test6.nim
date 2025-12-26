// test/test_arithmetic.nim

fn main(): void {
    echo("--- Aritmetik Testleri\r");

    var a: i32 = 15;
    var b: i32 = 7;
    
    echo("a is {a} \n\r"); //  print 15
    echo("b is {b} \n\r"); //  print 7

    var c: i32 = a + b;
    
    echo("c (a + b) is {c} \n\r"); //  print 22

    var d: i32 = a - 10;
    echo("d (a - 10) is {d} \n\r"); //  print 5

    var e: i32 = a * b;
    echo("e (a * b) is {e} \n\r"); // Should print 105

    var f: i32 = a / b;  
    echo("f (a / b) is {f} \n\r"); // Should print 2 (integer division)

    var g: i32 = a % b;
    echo("g (a % b) is {g} \n\r"); // Should print 1 (remainder)

    echo("--- Kayan Noktalı Sayı Testleri \n\r"); // \n kabul eder.  otomatik \n eklemez... 
    // print("Hata: {e}",error); // print otomatik \n ekler ve 
    // print("", <format>)  çıktıları formatlamak için seçenek sunar, kullanıcı bunu kendisi de tanımlaya bilmelidir.

    var x: f64 = 15.5;  
    var y: f64 = 2.5;

    echo("x is {x} \n\r"); // Should print 15.5
    echo("y is  {y} \n\r"); // Should print 2.5
    
    // var z: f64[4] = x / y; // z nin ondalık kısmını 4 rakamla sınırlar.
    // f64[4];  // gibi bir ifade bulunduğu kapsamda tüm float tiplerinin ondalık kısımlarını 4 rakamla sınırlar,
    // ve printf için format %.4f olarak gösterimi uygular. amaç echo da float format gösterimini otomatikleştirmek.

    echo("x + y is {x + y} \n\r"); // Should print 18.0
    echo("x - y is {x - y} \n\r"); // Should print 13.0
    echo("x * y is {x * y} \n\r"); // Should print 38.75
    echo("x / y is {x / y} \n\r"); // Should print 6.2

    echo("--- Çoklu Argüman Testleri\n");
    echo("a={a}, x={x}, y={y} \n\r"); //  Karışık int ve float
    echo("x={x}, y={y}, x*y={x*y} , a={a}, a={b}, a={c} \n\r"); // Sadece float
    echo("--- Arithmetic Tests End ---\r");
}