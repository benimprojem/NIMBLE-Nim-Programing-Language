// src/codegen.rs

use crate::ast::{Decl, Stmt, Expr, LiteralValue, TargetPlatform, Type, BinOp}; // BinaryOp -> BinOp
use crate::type_checker::TypeChecker;

// Platforma özel kod üretimi modülleri
mod windows;
mod linux;
mod macos;

#[derive(Debug, Clone)]
struct VariableLocation {
    stack_offset: i32,
    // YENİ: Değişkenin tipini de burada saklayacağız.
    // Bu, TypeChecker'a tekrar sormamızı engeller.
    ty: Type,
}

// YENİ: Veri segmentindeki farklı öğeleri temsil etmek için.
#[derive(Debug, Clone)]
enum DataItem {
    String(String),
    Float64(f64),
}

pub struct Codegen<'a, 'b> {
    pub program: &'a [Decl], // Reference to the whole program AST
    pub type_checker: &'b mut TypeChecker<'a>, // Reference to the TypeChecker (now mutable)
    pub target_platform: TargetPlatform,
    pub current_function_name: String, // Hangi fonksiyonun kodunu ürettiğimizi takip etmek için
    // YENİ: string_literals yerine data_items kullanıyoruz.
    data_items: Vec<DataItem>,
    #[allow(dead_code)] // Şimdilik kullanılmıyor, ileride kontrol akışı için kullanılacak.
    pub label_counter: usize, // Benzersiz etiketler oluşturmak için
    variable_locations: std::collections::HashMap<String, VariableLocation>, // Değişkenlerin konumları
    stack_pointer: i32, // Mevcut stack offset'i
}


impl<'a, 'b> Codegen<'a, 'b> {
    pub fn new(
        program: &'a [Decl],
        type_checker: &'b mut TypeChecker<'a>, // Changed to &mut
        target_platform: TargetPlatform,
    ) -> Self {
        Self {
            program,
            type_checker,
            target_platform,
            current_function_name: String::new(),
            data_items: Vec::new(),
            label_counter: 0,
            variable_locations: std::collections::HashMap::new(),
            stack_pointer: 0,
        }
    }

    pub fn generate(&mut self) -> Result<String, String> {
        let mut full_asm = String::new();

        // 1. Metin segmentini (kod) oluştur. Bu aşamada string ve float literalleri toplanır.
        let text_segment = self.generate_text_segment()?;

        // 2. Toplanan literallerle veri segmentini oluştur ve ekle.
        // Veri segmenti genellikle koddan önce gelir.
        full_asm.push_str(&self.generate_data_segment());
        full_asm.push_str(&text_segment);

        Ok(full_asm)
    }

    fn generate_data_segment(&mut self) -> String {
        let mut asm = String::new();
        asm.push_str("section .data\n");
        for (i, item) in self.data_items.iter().enumerate() {
            match item {
                DataItem::String(s) => {
                    // YENİ: String'i NASM'ın anlayacağı, okunabilir bir formata dönüştür.
                    // String'i `\n`, `\r` gibi özel karakterlere göre parçalara ayır.
                    let mut parts: Vec<String> = Vec::new();
                    let mut current_part = String::new();

                    for char in s.chars() {
                        match char {
                            '\n' | '\r' | '`' | '\'' | '"' => {
                                if !current_part.is_empty() {
                                    parts.push(format!("`{}`", current_part));
                                    current_part.clear();
                                }
                                parts.push(format!("{}", char as u8));
                            }
                            _ => current_part.push(char),
                        }
                    }
                    if !current_part.is_empty() {
                        parts.push(format!("`{}`", current_part));
                    }

                    // Parçaları virgülle birleştir ve sonuna null sonlandırıcı ekle.
                    asm.push_str(&format!("    str_{} db {}, 0\n", i, parts.join(", ")));
                    /*
                    let bytes = s.as_bytes();
                    let byte_str = bytes.iter().map(|b| b.to_string()).collect::<Vec<String>>().join(", ");
                    asm.push_str(&format!("    str_{} db {}, 0\n", i, byte_str));
                    */
                }
                DataItem::Float64(f) => {
                    // NASM'ın anladığı formatta bir f64 sabiti tanımla.
                    asm.push_str(&format!("    float_{} dq {}\n", i, f));
                }
            }
        }
        asm.push_str("\n");
        asm
    }

    fn generate_text_segment(&mut self) -> Result<String, String> {
        let mut asm = String::new();
        asm.push_str("section .text\n");

        // Platforma özel giriş noktası ve dış fonksiyon bildirimleri
        match self.target_platform {
            TargetPlatform::Windows => {
                asm.push_str("extern puts\n");
                asm.push_str("extern printf\n"); 
                asm.push_str("extern ExitProcess\n");
                asm.push_str("extern GetStdHandle\n"); // YENİ
                asm.push_str("extern WriteFile\n");    // YENİ
                asm.push_str("extern SetConsoleOutputCP\n"); 
                asm.push_str("global main\n\n");
            }
            TargetPlatform::Linux => {
                asm.push_str("extern printf\n");
                asm.push_str("global _start\n\n");
            }
            TargetPlatform::Macos => {
                asm.push_str("extern _printf\n");
                asm.push_str("extern _exit\n\n");
                asm.push_str("global _main\n\n");
            }
            _ => {}
        }

        // Ana program AST'sini gez ve kod üret
        for decl in self.program.iter() {
            if let Decl::Function { name, body, .. } = decl {
                if name == "main" {
                    self.current_function_name = name.clone();
                    self.stack_pointer = 0; // Her fonksiyon için stack'i sıfırla
                    self.variable_locations.clear();

                    asm.push_str(&format!("{}:\n", self.get_entry_point_label()));
                    // Fonksiyon başlangıcı (prologue)
                    asm.push_str("    push rbp\n");
                    asm.push_str("    mov rbp, rsp\n");
                    asm.push_str("    sub rsp, 256 ; Fonksiyon için 256 byte stack alanı ayır (geçici)\n\n");
                    
                    // YENİ: Windows'ta konsolun UTF-8'i doğru göstermesi için
                    if self.target_platform == TargetPlatform::Windows {
                        asm.push_str("    mov ecx, 65001 ; CP_UTF8\n    call SetConsoleOutputCP\n");
                    }

                    asm.push_str(&self.generate_stmt(body)?);
                    // main fonksiyonunun sonunda programı sonlandır
                    asm.push_str(&self.generate_exit_code());
                } else {
                    // Diğer fonksiyonlar şimdilik atlanıyor
                }
            }
        }

        // YENİ: Yardımcı fonksiyonları (platforma özel) en sona ekle
        if self.target_platform == TargetPlatform::Windows {
            asm.push_str("\n; --- Internal Helpers ---\n");
            asm.push_str(&self.generate_printf_assembly_helper());
        }

        Ok(asm)
    }

    // Platforma özel giriş noktası etiketini döndürür
    fn get_entry_point_label(&self) -> String {
        match self.target_platform {
            TargetPlatform::Windows => "main".to_string(),
            TargetPlatform::Linux => "_start".to_string(),
            TargetPlatform::Macos => "_main".to_string(),
            _ => "main".to_string(), // Varsayılan
        }
    }

    // Programı sonlandıran platforma özel assembly kodu
    fn generate_exit_code(&self) -> String {
        match self.target_platform {
            TargetPlatform::Windows => {
                "    xor rcx, rcx     ; ExitProcess için çıkış kodu 0\n    call ExitProcess\n".to_string()
            }
            TargetPlatform::Linux => {
                "    mov rax, 60        ; exit için syscall numarası\n    xor rdi, rdi       ; çıkış kodu 0\n    syscall\n".to_string()
            }
            TargetPlatform::Macos => {
                "    xor rdi, rdi     ; exit için çıkış kodu 0\n    call _exit\n".to_string()
            }
            _ => "".to_string(),
        }
    }

    // Deyimleri assembly koduna çevirir
    fn generate_stmt(&mut self, stmt: &Stmt) -> Result<String, String> {
        match stmt {
            Stmt::Block(stmts) => {
                let mut asm = String::new();
                for s in stmts {
                    asm.push_str(&self.generate_stmt(s)?);
                }
                Ok(asm)
            }
            Stmt::VarDecl { name, ty, init, .. } => {
                let mut code = String::new();
                if let Some(init_expr) = init {
                    // 1. Evaluate the initializer expression. The result will be in RAX.
                    code.push_str(&self.generate_expr(init_expr)?);
                    
                    // 2. Allocate space on the stack for the new variable.
                    self.stack_pointer += 8;
                    let offset = self.stack_pointer;
                    // YENİ: Değişkenin tipini de konumuyla birlikte sakla.
                    let location = VariableLocation { stack_offset: offset, ty: ty.clone() };
                    self.variable_locations.insert(name.clone(), location);
                    //eprintln!("DEBUG: Codegen: Declared variable '{}' at stack offset {}. Current map: {:?}", name, offset, self.variable_locations);

                    // 3. Move the result from RAX (integer/pointer) or XMM0 (float) to the variable's stack location.
                    if ty.is_float() {
                        // Kayan noktalı değer XMM0'da, stack'e movsd ile yaz.
                        code.push_str(&format!("    movsd [rbp - {}], xmm0 ; Store float variable '{}'\n", offset, name));
                    } else {
                        // Tamsayı veya pointer değeri RAX'ta, stack'e mov ile yaz.
                        code.push_str(&format!("    mov [rbp - {}], rax ; Store integer/pointer variable '{}'\n", offset, name));
                    }
                }
                Ok(code)
            }
            Stmt::Echo(expr) => self.generate_echo_call(expr),
            Stmt::Assign { left, value } => {
                let mut code = String::new();
                // 1. Sağ tarafı değerlendir
                code.push_str(&self.generate_expr(value)?);
                
                // 2. Sol tarafın konumunu bul ve ata
                if let Expr::Variable(name) = &*left {
                    let loc = self.variable_locations.get(name).ok_or_else(|| format!("Atama hatası: Bilinmeyen değişken '{}'", name))?;
                    if loc.ty.is_float() {
                        code.push_str(&format!("    movsd [rbp - {}], xmm0 ; Assign to float variable '{}'\n", loc.stack_offset, name));
                    } else {
                        code.push_str(&format!("    mov [rbp - {}], rax ; Assign to integer/pointer variable '{}'\n", loc.stack_offset, name));
                    }
                } else {
                    return Err("Şimdilik sadece değişkenlere atama destekleniyor.".to_string());
                }
                Ok(code)
            }
            _ => Ok("".to_string()),
        }
    }

    // YARDIMCI FONKSİYON: Benzersiz bir etiket oluşturur.
    #[allow(dead_code)] // Şimdilik kullanılmıyor, ileride kontrol akışı için kullanılacak.
    fn generate_label(&mut self, prefix: &str) -> String {
        let label = format!("{}_{}", prefix, self.label_counter);
        self.label_counter += 1;
        label
    }

    // 'echo' çağrısını assembly koduna çevirir
    fn generate_echo_call(&mut self, expr: &Expr) -> Result<String, String> {
        let mut asm = String::new();
        match expr {
            Expr::Literal(LiteralValue::Str(s)) => {
                let s_with_newline = format!("{}\n", s);
                let str_to_print_index = self.add_string_literal(s_with_newline);
                if self.target_platform == TargetPlatform::Windows {
                    // Windows'ta bizim özel _printf fonksiyonumuzu çağırıyoruz.
                    asm.push_str(&format!("    lea rcx, [rel str_{}]\n", str_to_print_index));
                    asm.push_str("    call _printf\n");
                } else {
                    // Diğer platformlar için şimdilik placeholder.
                }
            }
            Expr::InterpolatedString(parts) => {
                let mut format_string = String::new();
                let mut args_to_pass: Vec<(crate::ast::Type, String)> = Vec::new(); // (tip, değeri RAX/XMM0'a yükleyen kod)

                for part in parts {
                    match part {
                        Expr::Literal(LiteralValue::Str(s)) => {
                            format_string.push_str(s);
                        }
                        Expr::Variable(var_name) => {
                            // Değişkenin tipini ve konumunu kendi haritamızdan al.
                            let loc = self.variable_locations.get(var_name).ok_or_else(|| format!("Kod üretimi hatası: Bilinmeyen değişken '{}'", var_name))?;
                            format_string.push_str(self.get_format_specifier(&loc.ty));
                            args_to_pass.push((loc.ty.clone(), format!("    {} {}, [rbp - {}]\n", if loc.ty.is_float() { "movsd" } else { "mov" }, if loc.ty.is_float() { "xmm0" } else { "rax" }, loc.stack_offset)));
                        }
                        // İnterpolasyon içinde doğrudan ifadeleri de işle.
                        expr => {
                            // TypeChecker'ı Codegen'in mevcut durumuyla senkronize et.
                            // Bu, type_of_expr çağrılmadan önce TypeChecker'ın kapsamının
                            // doğru değişkenlerle doldurulmasını sağlar.
                            
                            self.type_checker.push_scope(); // Yeni bir kapsam aç
                            for (name, loc) in &self.variable_locations {
                                let var_info = crate::type_checker::VarInfo {
                                    ty: loc.ty.clone(),
                                    is_const: false, // Bu aşamada const/mut bilgisi kritik değil
                                    _is_mutable: true,
                                };
                                self.type_checker.define_variable(name.clone(), var_info).unwrap(); // Hata beklemiyoruz
                            }

                            let expr_type = self.type_checker.type_of_expr(expr).map_err(|e| format!("Kod üretimi hatası: {}", e))?;
                            format_string.push_str(self.get_format_specifier(&expr_type));

                            // İfadeyi değerlendir. Sonuç RAX (int) veya XMM0 (float) içinde olacak.
                            args_to_pass.push((expr_type, self.generate_expr(expr)?));

                            // Senkronizasyon için açılan kapsamı temizle.
                            self.type_checker.pop_scope()?;
                        }
                    }
                }
                //format_string.push('\n'); // Sona newline ekle
                let str_index = self.add_string_literal(format_string);

                asm.push_str(&self.generate_printf_call_multi_arg(str_index, args_to_pass)?);
            }
            _ => return Err(format!("'echo' komutu bu ifade tipini desteklemiyor: {:?}", expr)),
        }
        Ok(asm)
    }

    // YENİ: printf için format belirleyiciyi döndürür.
    fn get_format_specifier(&self, ty: &crate::ast::Type) -> &'static str {
        if matches!(ty, Type::Str(_)) {
            "%s"
        } else if ty.is_float() {
            "%f" // Kayan noktalı sayılar için format
        } else {
            "%d" // Tamsayılar için format
        }
    }

    // todo: bu fonksiyon yanlış çalışıyor yenilenecek..
    // format_str_index: format string'in data_items listesindeki indeksi.
    // args_to_pass: (tip, değeri RAX/XMM0'a yükleyen assembly kodu) çiftlerinin vektörü.
    fn generate_printf_call_multi_arg(&mut self, format_str_index: usize, args_to_pass: Vec<(crate::ast::Type, String)>) -> Result<String, String> {
        let mut code = String::new();
        
        let arg_int_regs = ["rcx", "rdx", "r8", "r9"];
        let arg_float_regs = ["xmm0", "xmm1", "xmm2", "xmm3"];

        // 1. AŞAMA: Değerlendirme ve Stack'e Kaydetme
        let mut temp_stack_offsets = Vec::new();
        for (_, arg_gen_code) in &args_to_pass {
            code.push_str(arg_gen_code);
            self.stack_pointer += 8;
            let offset = self.stack_pointer;
            temp_stack_offsets.push(offset);
            code.push_str(&format!("    mov [rbp - {}], rax\n", offset));
        }

        // 2. AŞAMA: Argümanları Yerleştirme
        // Not: 5. ve sonraki argümanlar stack'te olmalı (sağdan sola doğru pushlanmalı).
        let mut stack_args_pushed = 0;
        if args_to_pass.len() > 3 { // 1 format + 3 regs = 4 total
            for i in (3..args_to_pass.iter().len()).rev() {
                 let offset = temp_stack_offsets[i];
                 code.push_str(&format!("    mov rax, [rbp - {}]\n", offset));
                 code.push_str("    push rax\n");
                 stack_args_pushed += 1;
            }
        }

        // Format ve ilk 3 argümanı yükle
        code.push_str(&format!("    lea rcx, [rel str_{}]\n", format_str_index));
        for i in 0..std::cmp::min(args_to_pass.len(), 3) {
            let offset = temp_stack_offsets[i];
            let int_reg = arg_int_regs[i + 1];
            let float_reg = arg_float_regs[i + 1];
            let is_float = args_to_pass[i].0.is_float();

            if is_float {
                code.push_str(&format!("    movsd {}, [rbp - {}]\n", float_reg, offset));
                code.push_str(&format!("    mov {}, [rbp - {}]\n", int_reg, offset));
            } else {
                code.push_str(&format!("    mov {}, [rbp - {}]\n", int_reg, offset));
            }
        }

        // 3. ÇAĞRI
        if self.target_platform == TargetPlatform::Windows {
            code.push_str("    sub rsp, 32      ; Shadow space\n");
            code.push_str("    call _printf\n");
            code.push_str(&format!("    add rsp, {}      ; Cleanup shadow space + stack args\n", 32 + (stack_args_pushed * 8)));
        } else {
            code.push_str("    call _printf\n");
        }

        self.stack_pointer -= (temp_stack_offsets.len() * 8) as i32;
        Ok(code)
    }
    

    fn generate_expr(&mut self, expr: &Expr) -> Result<String, String> {
        //eprintln!("DEBUG: Codegen: Generating expression: {:?}", expr);
        match expr {
            Expr::Literal(LiteralValue::Int(val)) => {
                Ok(format!("    mov rax, {}\n", val))
            }
            Expr::Literal(LiteralValue::Float(val)) => { // f32, f64, f80, f128 için
                // Kayan noktalı literali bellekte bir yere koyup oradan XMM0'a yükle.
                // Tüm float tiplerini şimdilik f64 olarak işliyoruz.
                // Değeri data_items'a ekle ve indeksini al.
                let float_index = self.add_data_item(DataItem::Float64(*val));
                // Etiketi indekse göre oluştur ve değeri yükle.
                Ok(format!("    movsd xmm0, [rel float_{}]\n", float_index))
            }
            Expr::Literal(LiteralValue::Str(s)) => {
                let str_index = self.add_string_literal(s.clone());
                Ok(format!("    lea rax, [rel str_{}]\n", str_index))
            }
            Expr::Variable(name) => {
                //eprintln!("DEBUG: Codegen: Looking up variable '{}'", name);
                if let Some(loc) = self.variable_locations.get(name) {
                    //eprintln!("DEBUG: Codegen: Found variable '{}' at offset {}", name, loc.stack_offset);
                    if loc.ty.is_float() {
                        Ok(format!("    movsd xmm0, [rbp - {}] ; Load float variable '{}'\n", loc.stack_offset, name))
                    } else {
                        Ok(format!("    mov rax, [rbp - {}] ; Load integer/pointer variable '{}'\n", loc.stack_offset, name))
                    }
                } else {
                    //eprintln!("DEBUG: Codegen: Variable '{}' NOT FOUND in variable_locations. Current map: {:?}", name, self.variable_locations);
                    Err(format!("Kod üretimi hatası: Bilinmeyen değişken '{}'", name))
                }
            }
            Expr::Binary { left, op, right } => {
                // TypeChecker'ı Codegen'in mevcut durumuyla senkronize et.
                // doğru değişkenlerle doldurulmasını sağlar.
                self.type_checker.push_scope(); // Yeni bir kapsam aç
                for (name, loc) in &self.variable_locations {
                    let var_info = crate::type_checker::VarInfo {
                        ty: loc.ty.clone(),
                        is_const: false, // Bu aşamada const/mut bilgisi kritik değil
                        _is_mutable: true,
                    };
                    self.type_checker.define_variable(name.clone(), var_info).unwrap(); // Hata beklemiyoruz
                }

                let left_type = self.type_checker.type_of_expr(left).map_err(|e| format!("Kod üretimi hatası: {}", e))?;
                let right_type = self.type_checker.type_of_expr(right).map_err(|e| format!("Kod üretimi hatası: {}", e))?;

                let mut code = String::new();

                // Eğer operasyon kayan noktalı ise
                if left_type.is_float() || right_type.is_float() { // f32, f64, f80, f128
                    // Kayan noktalı sayı aritmetiği (XMM register'ları kullanılır)
                    // 1. Sağ tarafı değerlendir. Sonuç XMM0'da olacak.
                    code.push_str(&self.generate_expr(right)?);
                    code.push_str("    movsd xmm1, xmm0 ; Sağ tarafı xmm1'e taşı\n");

                    // 2. Sol tarafı değerlendir. Sonuç XMM0'da olacak.
                    code.push_str(&self.generate_expr(left)?);
                    // 3. İşlemi yap. Sonuç XMM0'da kalır.
                    match op {
                        BinOp::Add => code.push_str("    addsd xmm0, xmm1\n"),
                        BinOp::Sub => code.push_str("    subsd xmm0, xmm1\n"),
                        BinOp::Mul => code.push_str("    mulsd xmm0, xmm1\n"),
                        BinOp::Div => code.push_str("    divsd xmm0, xmm1\n"),
                        // Kayan noktalı sayılar için modül operatörü genellikle yoktur.
                        // Bunun yerine fmod C fonksiyonu çağrılabilir. Şimdilik hata verelim.
                        BinOp::Mod => return Err("Kayan noktalı sayılar için '%' operatörü desteklenmiyor.".to_string()),
                        _ => return Err(format!("Desteklenmeyen ikili operatör (float): {:?}. Sadece +, -, *, / desteklenir.", op)),
                    }
                } else if left_type.is_integer() || right_type.is_integer() {
                    // Tamsayı aritmetiği (RAX, RBX register'ları kullanılır)
                    // 1. Sağ tarafı değerlendir ve stack'e push'la.
                    code.push_str(&self.generate_expr(right)?);
                    code.push_str("    push rax\n");

                    // 2. Sol tarafı değerlendir. Sonuç RAX'ta.
                    code.push_str(&self.generate_expr(left)?);
                    // 3. Sağ tarafı stack'ten RBX'e pop'la.
                    code.push_str("    pop rbx\n");

                    // 4. İşlemi yap.
                    match op {
                        BinOp::Add => code.push_str("    add rax, rbx\n"),
                        BinOp::Sub => code.push_str("    sub rax, rbx\n"),
                        BinOp::Mul => code.push_str("    imul rbx\n"), // Signed multiplication: RAX = RAX * RBX
                        BinOp::Div => {
                            code.push_str("    cqo             ; Sign-extend RAX to RDX:RAX\n");
                            code.push_str("    idiv rbx        ; Signed division\n");
                        }
                        BinOp::Mod => {
                            code.push_str("    cqo\n    idiv rbx\n    mov rax, rdx ; Remainder is in RDX\n");
                        }
                        _ => return Err(format!("Desteklenmeyen ikili operatör (int): {:?}", op)),
                    }
                } else {
                    return Err(format!("Desteklenmeyen ikili operatör tipleri: {:?} ve {:?}.", left_type, right_type));
                }

                // Senkronizasyon için açılan kapsamı temizle.
                self.type_checker.pop_scope()?;
                Ok(code)
            }
            _ => Err("Bu ifade tipi için kod üretimi henüz desteklenmiyor.".to_string()),
        }
    }

    // YENİ: String literal'ini kaydeder ve indeksini döndürür
    fn add_string_literal(&mut self, s: String) -> usize {
        // Sadece stringleri kontrol et
        if let Some(pos) = self.data_items.iter().position(|item| matches!(item, DataItem::String(existing_s) if existing_s == &s)) {
            return pos;
        } else {
            self.data_items.push(DataItem::String(s));
            return self.data_items.len() - 1;
        }
    }

    // YENİ: Genel bir veri öğesi ekler ve indeksini döndürür.
    fn add_data_item(&mut self, item: DataItem) -> usize {
        self.data_items.push(item);
        self.data_items.len() - 1
    }

    // YENİ: Windows WriteFile kullanan detaylı _printf yardımcısı
    fn generate_printf_assembly_helper(&mut self) -> String {
        let mut asm = String::new();
        
        // --- DATA SECTION FOR HELPERS ---
        // Not: Bu kısım aslında .data veya .bss'e gitmeli ama biz fonksiyon sonuna ekliyoruz.
        // NASM'da section değiştirip geri dönebiliriz.
        asm.push_str("section .bss\n");
        asm.push_str("    _printf_buf resb 4096\n");
        asm.push_str("    _printf_tmp resb 64\n");
        asm.push_str("section .text\n\n");

        asm.push_str("_printf:\n");
        asm.push_str("    push rbp\n");
        asm.push_str("    mov rbp, rsp\n");
        asm.push_str("    sub rsp, 128 ; Local storage and shadow space\n");
        
        // 1. Argümanları Sakla (Windows x64 ABI)
        asm.push_str("    mov [rbp+16], rdx ; 2. arg\n");
        asm.push_str("    mov [rbp+24], r8  ; 3. arg\n");
        asm.push_str("    mov [rbp+32], r9  ; 4. arg\n");
        asm.push_str("    ; RCX zaten format string adresi\n");
        
        asm.push_str("    lea rsi, [rcx]    ; RSI = format string\n");
        asm.push_str("    lea rdi, [rel _printf_buf] ; RDI = destination buffer\n");
        asm.push_str("    lea r12, [rbp+16] ; R12 = pointer to next argument\n");
        
        asm.push_str(".loop:\n");
        asm.push_str("    lodsb             ; AL = *rsi++\n");
        asm.push_str("    test al, al\n");
        asm.push_str("    jz .done\n");
        asm.push_str("    cmp al, '%'\n");
        asm.push_str("    jne .emit_char\n");
        
        asm.push_str("    lodsb             ; Specifier\n");
        asm.push_str("    cmp al, 'd'\n");
        asm.push_str("    je .handle_int\n");
        asm.push_str("    cmp al, 's'\n");
        asm.push_str("    je .handle_str\n");
        asm.push_str("    cmp al, 'f'\n");
        asm.push_str("    je .handle_float\n");
        asm.push_str("    cmp al, '%'\n");
        asm.push_str("    je .emit_char\n");
        asm.push_str("    jmp .loop\n");

        asm.push_str(".emit_char:\n");
        asm.push_str("    stosb\n");
        asm.push_str("    jmp .loop\n");

        asm.push_str(".handle_int:\n");
        asm.push_str("    mov rax, [r12]\n");
        asm.push_str("    add r12, 8\n");
        asm.push_str("    call .itoa_internal\n");
        asm.push_str("    jmp .loop\n");

        asm.push_str(".handle_str:\n");
        asm.push_str("    mov r9, [r12]\n");
        asm.push_str("    add r12, 8\n");
        asm.push_str(".str_loop:\n");
        asm.push_str("    mov al, [r9]\n");
        asm.push_str("    test al, al\n");
        asm.push_str("    jz .loop\n");
        asm.push_str("    stosb\n");
        asm.push_str("    inc r9\n");
        asm.push_str("    jmp .str_loop\n");

        asm.push_str(".handle_float:\n");
        asm.push_str("    ; Basitleştirilmiş: Sadece tamsayı kısmını bas ve .00 ekle\n");
        asm.push_str("    mov rax, [r12]\n");
        asm.push_str("    add r12, 8\n");
        asm.push_str("    ; Şimdilik float'ı bir kenara bırakalım veya integer gibi basalım\n");
        asm.push_str("    mov r13, rax ; Save for later\n");
        asm.push_str("    ; Bu kısım daha detaylandırılacak\n");
        asm.push_str("    call .itoa_internal\n");
        asm.push_str("    mov al, '.'\n    stosb\n    mov al, '0'\n    stosb\n    stosb\n");
        asm.push_str("    jmp .loop\n");

        asm.push_str(".done:\n");
        asm.push_str("    xor al, al\n    stosb\n");
        asm.push_str("    ; --- WriteFile çağrısı ---\n");
        asm.push_str("    mov ecx, -11     ; STD_OUTPUT_HANDLE\n");
        asm.push_str("    call GetStdHandle\n");
        asm.push_str("    mov rcx, rax     ; hFile\n");
        asm.push_str("    lea rdx, [rel _printf_buf] ; lpBuffer\n");
        asm.push_str("    ; Boyut hesapla\n");
        asm.push_str("    lea rax, [rel _printf_buf]\n");
        asm.push_str("    mov r8, rdi\n");
        asm.push_str("    sub r8, rax\n");
        asm.push_str("    dec r8           ; length (null hariç)\n");
        asm.push_str("    lea r9, [rbp-8]  ; lpNumberOfBytesWritten\n");
        asm.push_str("    mov qword [rsp+32], 0\n");
        asm.push_str("    call WriteFile\n");
        
        asm.push_str("    add rsp, 128\n");
        asm.push_str("    pop rbp\n");
        asm.push_str("    ret\n");

        // --- Yardımcı Rutin: ITOA ---
        asm.push_str(".itoa_internal:\n");
        asm.push_str("    push rdi\n");
        asm.push_str("    lea rdi, [rel _printf_tmp + 63]\n");
        asm.push_str("    mov byte [rdi], 0\n");
        asm.push_str("    mov rbx, 10\n");
        asm.push_str("    mov r10, rax\n");
        asm.push_str("    test rax, rax\n");
        asm.push_str("    jns .pos\n");
        asm.push_str("    neg rax\n");
        asm.push_str(".pos:\n");
        asm.push_str(".itoa_loop:\n");
        asm.push_str("    xor rdx, rdx\n");
        asm.push_str("    div rbx\n");
        asm.push_str("    add dl, '0'\n");
        asm.push_str("    dec rdi\n");
        asm.push_str("    mov [rdi], dl\n");
        asm.push_str("    test rax, rax\n");
        asm.push_str("    jnz .itoa_loop\n");
        asm.push_str("    test r10, r10\n");
        asm.push_str("    jns .itoa_end\n");
        asm.push_str("    dec rdi\n");
        asm.push_str("    mov byte [rdi], '-'\n");
        asm.push_str(".itoa_end:\n");
        asm.push_str("    mov rsi, rdi\n");
        asm.push_str("    pop rdi\n");
        asm.push_str(".copy_loop:\n");
        asm.push_str("    lodsb\n");
        asm.push_str("    test al, al\n");
        asm.push_str("    jz .itoa_ret\n");
        asm.push_str("    stosb\n");
        asm.push_str("    jmp .copy_loop\n");
        asm.push_str(".itoa_ret: ret\n");
        asm
    }
}
