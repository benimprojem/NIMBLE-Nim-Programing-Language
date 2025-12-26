// src/codegen.rs

use crate::ast::{Decl, Expr, LiteralValue, Stmt, Type, BinOp, UnOp};
use std::collections::HashMap; 
// ================ // PLATFORM TANIMLARI // ===========================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetPlatform {
    Linux,
    Windows,
}

// Kod Üreticisi Yapısı
pub struct Codegen<'a> {
    data_section: String,
    text_section: String,
    label_counter: usize,
    stack_offsets: HashMap<String, i32>,
    current_stack_size: i32,
    variable_types: HashMap<String, Type>, // YENİ: Değişken tiplerini saklamak için
    target_platform: TargetPlatform,
    current_loop_start: Option<String>, // YENİ: `continue` için döngü başlangıcı
    current_loop_end: Option<String>,   // YENİ: `break` için döngü sonu
    current_function_name: Option<String>, // Mevcut fonksiyon adını saklamak için
    program: &'a [Decl], // Tüm program bildirimlerine referans
}

impl<'a> Codegen<'a> {
    pub fn new(program: &'a [Decl], platform: TargetPlatform) -> Self {
        Codegen {
            data_section: String::new(),
            text_section: String::new(),
            label_counter: 0,
            stack_offsets: HashMap::new(),
            current_stack_size: 0,
            variable_types: HashMap::new(), // YENİ: Başlat
            target_platform: platform,
            current_loop_start: None,
            current_loop_end: None,
            current_function_name: None,
            program,
        }
    }

    fn generate_label(&mut self, prefix: &str) -> String {
        self.label_counter += 1;
        format!("{}_{}", prefix, self.label_counter)
    }
    
    fn get_arg_registers(&self) -> &'static [&'static str] {
        match self.target_platform {
            TargetPlatform::Windows => &["rcx", "rdx", "r8", "r9"],
            TargetPlatform::Linux => &["rdi", "rsi", "rdx", "rcx", "r8", "r9"],
        }
    }

    fn get_variable_offset(&self, name: &str) -> Result<i32, String> {
        self.stack_offsets
            .get(name)
            .cloned()
            .ok_or_else(|| format!("Hata: Codegen'da tanımlanmamış değişken: '{}'", name))
    }

    // YENİ YARDIMCI FONKSİYON: İfadenin tipini belirler
    fn get_expr_type(&self, expr: &Expr) -> Type {
        match expr {
            Expr::Literal(lit) => match lit {
                LiteralValue::Int(_) => Type::I32, // Varsayılan tamsayı
                LiteralValue::Float(_) => Type::F64, // Varsayılan olarak F64
                LiteralValue::Str(_) => Type::Str(None),
                LiteralValue::Bool(_) => Type::Bool,
                LiteralValue::Null => Type::Null,
            },
            Expr::Variable(name) => self
                .variable_types
                .get(name)
                .cloned()
                .unwrap_or(Type::Unknown),
            Expr::Call { callee, args: _ } => {
                if let Expr::Variable(callee_name) = callee.as_ref() {
                    // Fonksiyon tanımını program içinde bul
                    if let Some(Decl::Function { return_type, .. }) = self.program.iter().find_map(|d| {
                        if let Decl::Function { name, .. } = d {
                            if name == callee_name { Some(d) } else { None }
                        } else { None }
                    }) {
                        return_type.clone()
                    } else {
                        // Bilinmeyen fonksiyon veya hata
                        Type::Unknown
                    }
                } else {
                    // Callee bir fonksiyon adı değilse (örn: (a+b)() gibi), şimdilik Unknown döndür.
                    Type::Unknown
                }
            },
            Expr::Binary { left, op, right } => {
                let left_type = self.get_expr_type(left);
                let right_type = self.get_expr_type(right);

                match op {
                    BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div | BinOp::Mod => {
                        // Eğer operandlardan biri float ise sonuç float olmalı.
                        if left_type.is_float() || right_type.is_float() {
                            // Hassasiyet korunur, eğer biri F64 ise sonuç F64 olmalı.
                            if left_type == Type::F64 || right_type == Type::F64 {
                                Type::F64
                            } else if left_type == Type::F32 || right_type == Type::F32 {
                                Type::F32
                            } else {
                                Type::Unknown // Float tipler arasında bilinmeyen durum
                            }
                        } else if left_type.is_integer() && right_type.is_integer() {
                            // İki operand da tamsayı ise en geniş olanı döndür
                            // (bu kısım TypeChecker tarafından daha iyi ele alınmalı, burada basit tutuyorum)
                            if left_type == Type::I64 || right_type == Type::I64 {
                                Type::I64
                            } else {
                                Type::I32 // Varsayılan tamsayı
                            }
                        } else {
                            Type::Unknown // Uyumsuz tipler
                        }
                    },
                    BinOp::Equal | BinOp::NotEqual | BinOp::Greater | BinOp::Less | BinOp::GreaterEqual | BinOp::LessEqual => {
                        Type::Bool // Karşılaştırma operatörlerinin sonucu her zaman Bool
                    },
                    BinOp::And | BinOp::Or => Type::Bool, // Mantıksal operatörlerin sonucu her zaman Bool
                    _ => Type::Unknown, // Diğer bilinmeyen operatörler
                }
            },
            Expr::Unary { op, right } => {
                let right_type = self.get_expr_type(right);
                match op {
                    UnOp::Neg => right_type, // Negatifleme operandın tipini korur
                    UnOp::Not => Type::Bool, // Mantıksal NOT her zaman Bool döndürür
                    // Artırma/Azaltma operandın tipini korur, aksi halde unknown.
                    UnOp::PreInc | UnOp::PreDec | UnOp::PostInc | UnOp::PostDec => right_type,
                }
            },
            Expr::Assign { name: _, value } => self.get_expr_type(value), // Atama ifadenin tipini döndürür
            Expr::ArrayAccess { name, .. } => {
                // Dizinin tipinden iç elemanın tipini al
                if let Some(Type::Array(inner_type, _)) = self.variable_types.get(name) {
                    *inner_type.clone()
                } else {
                    Type::Unknown
                }
            },
            Expr::MemberAccess { object: _object, member: _ } => {
                // Üye erişimi için TypeChecker'dan geçirilmiş bir yapıya ihtiyacımız var.
                // Şimdilik Unknown döndürüyorum.
                Type::Unknown
            },
            Expr::Conditional { cond: _, then_branch, else_branch } => {
                let then_type = self.get_expr_type(then_branch);
                let else_type = self.get_expr_type(else_branch);
                // Ternary'nin iki dalı da aynı tipte olmalı, TypeChecker bunu zaten sağlamalı.
                if then_type == else_type { then_type } else { Type::Unknown }
            },
            Expr::Match { .. } => Type::Unknown, // Match ifadesinin tipini belirlemek daha karmaşık
            Expr::Tuple(elements) => {
                let mut element_types = Vec::new();
                for elem in elements {
                    element_types.push(self.get_expr_type(elem));
                }
                Type::Tuple(element_types)
            },
            Expr::InterpolatedString(_) => Type::Str(None),
            Expr::DefaultCase => Type::Unknown,
            Expr::Range { start, end: _ } => self.get_expr_type(start), // Range başlangıç tipini döndürür
            Expr::Block { .. } => Type::Void, // Bir blok ifade olarak kullanıldığında genellikle void döndürür
        }
    }
	
    pub fn generate_program(&mut self) -> Result<String, String> {
        self.data_section.push_str("section .data\n");
        self.text_section.push_str("\nsection .text\n");
        
        match self.target_platform {
            TargetPlatform::Linux => {
                self.text_section.push_str("    global _start\n");
            },
            TargetPlatform::Windows => {
                self.text_section.push_str("    global main\n");
                self.text_section.push_str("    extern ExitProcess\n");
                self.text_section.push_str("    extern printf\n");
            },
        }
        
        for decl in self.program {
			if let Decl::Function { name, params, return_type, body } = decl {
				self.generate_function(name, params, return_type, body)?;
			}
		}
        
        Ok(format!("{}{}", self.data_section, self.text_section))
    }

    fn generate_function(&mut self, name: &str, params: &[(String, Type, Option<Expr>)], _return_type: &Type, body: &Stmt) -> Result<(), String> {
        self.stack_offsets.clear();
        self.variable_types.clear();
        self.current_stack_size = 0;
        self.current_function_name = Some(name.to_string()); // Fonksiyon adını kaydet

        let mut code = String::new();
        code.push_str(&format!("\n{}:\n", name));
        code.push_str("    push rbp\n");       
        code.push_str("    mov rbp, rsp\n");     

        let arg_regs = self.get_arg_registers();
        let xmm_regs = ["xmm0", "xmm1", "xmm2", "xmm3"]; // For Windows x64 float args

        let mut int_param_idx = 0;
        let mut float_param_idx = 0;

        for (p_name, p_type, _) in params.iter() {
            self.current_stack_size += 8; 
            let offset = -self.current_stack_size;
            self.variable_types.insert(p_name.clone(), p_type.clone());
            self.stack_offsets.insert(p_name.clone(), offset);

            if p_type.is_float() {
                if float_param_idx < xmm_regs.len() {
                    code.push_str(&format!("    movsd qword [rel rbp{}], {}\n", offset, xmm_regs[float_param_idx]));
                    float_param_idx += 1;
                } else {
                    return Err("Hata: Fonksiyonlar 4'ten fazla float parametresi kabul edemez.".to_string());
                }
            } else { // Integer, String, Bool, etc.
                if int_param_idx < arg_regs.len() {
                    code.push_str(&format!("    mov qword [rel rbp{}], {}\n", offset, arg_regs[int_param_idx]));
                    int_param_idx += 1;
                } else {
                    return Err("Hata: Fonksiyonlar 4'ten fazla tamsayı/pointer parametresi kabul edemez.".to_string());
                }
            }
        }
        
        // Pass function name to statement generator to handle main's exit
        code.push_str(&self.generate_stmt(body)?);
        
        // If the function is main, it will be handled by Stmt::Return.
        // If it's not main and not ending with a return, we add a default return for void functions.
        if name != "main" {
             if !code.trim().ends_with("ret") {
                code.push_str("    mov rsp, rbp\n"); 
                code.push_str("    pop rbp\n");      
                code.push_str("    ret\n");
             }
        }

        self.text_section.push_str(&code);
        self.current_function_name = None; // Fonksiyon bittiğinde temizle
        Ok(())
    }

    fn generate_stmt(&mut self, stmt: &Stmt) -> Result<String, String> {
        let mut code = String::new();

        match stmt {
            Stmt::Echo(expr) => {
                // YENİ MANTIK: InterpolatedString'i özel olarak ele al.
                if let Expr::InterpolatedString(s) = expr {
                    // String'i {var} ve literal kısımlarına ayır.
                    // Bu basit bir parser, daha karmaşık durumlar için regex gerekebilir.
                    let mut last_end = 0;
                    for (start, _part) in s.match_indices('{') {
                        // Değişkenden önceki literal kısmı yazdır.
                        if start > last_end {
                            let literal_part = &s[last_end..start];
                            code.push_str(&self.generate_print_str(literal_part)?);
                        }
                        
                        // Değişkeni bul ve yazdır.
                        if let Some(end) = s[start..].find('}') {
                            let var_name = &s[start + 1 .. start + end];
                            let var_expr = Expr::Variable(var_name.to_string());
                            code.push_str(&self.generate_expr(&var_expr)?); // Değişkenin değerini rax'e al.
                            
                            // Değişkenin tipine göre doğru format string'i kullan.
                            // YENİ: Tip bilgisini TypeChecker'dan almamız gerekiyor.
                            // Şimdilik, codegen'in kendi haritasından okuyoruz.
                            // Bu haritanın VarDecl sırasında doldurulduğundan emin olmalıyız.
                            let var_type = self.variable_types.get(var_name)
                                .cloned().unwrap_or(Type::Unknown);
                            let format_string = match var_type {
                                Type::Str(_) => "%s",
                                // Float için %f formatı
                                Type::F32 | Type::F64 => "%f",
                                _ => "%lld",
                            };
                            code.push_str(&self.generate_printf_call(format_string, &var_type)?);

                            last_end = start + end + 1;
                        }
                    }
                    // Sondaki literal kısmı yazdır.
                    if last_end < s.len() {
                        let literal_part = &s[last_end..];
                        code.push_str(&self.generate_print_str(literal_part)?);
                    }
                    

                } else {
                    // --- ESKİ MANTIK: Normal ifadeler için ---
                    code.push_str(&self.generate_expr(expr)?);
                    let expr_type = self.get_expr_type(expr);

                    let format_string = match expr_type {
                        Type::Str(_) => "%s",
                        Type::F32 | Type::F64 => "%f", // Float için %f formatı
                        _ => "%lld", // Varsayılan tamsayı
                    };
                    
                    code.push_str(&self.generate_printf_call(format_string, &expr_type)?);
                }
            },

            Stmt::While { condition, body } => {
                let label_start = self.generate_label("WHILE_START");
                let label_end = self.generate_label("WHILE_END");

                // Döngü etiketlerini kaydet
                self.current_loop_start = Some(label_start.clone());
                self.current_loop_end = Some(label_end.clone());

                code.push_str(&format!("{}:\n", label_start));
                code.push_str(&self.generate_expr(condition)?);
                code.push_str("    cmp rax, 0\n");
                code.push_str(&format!("    je {}\n", label_end));
                code.push_str(&self.generate_stmt(body)?);
                code.push_str(&format!("    jmp {}\n", label_start));
                code.push_str(&format!("{}:\n", label_end));

                // Döngü bitti, etiketleri temizle
                self.current_loop_start = None;
                self.current_loop_end = None;
            },

            // YENİ: For Döngüsü için kod üretimi
            Stmt::For { initializer, condition, increment, variable, iterable, body } => {
                if let (Some(var_name), Some(iter_expr)) = (variable, iterable) {
                    // --- for-in döngüsü kod üretimi ---
                    let label_start = self.generate_label("FOR_IN_START");
                    let label_end = self.generate_label("FOR_IN_END");
                    self.current_loop_start = Some(label_start.clone());
                    self.current_loop_end = Some(label_end.clone());

                    let index_var_name = format!("__index_{}", self.label_counter);

                    // 1. Döngü için gizli bir indeks değişkeni oluştur (örn: __index_1 = 0)
                    let index_decl = Stmt::VarDecl { name: index_var_name.clone(), ty: Type::I32, init: Some(Expr::Literal(LiteralValue::Int(0))), is_const: false, is_mutable: true };
                    code.push_str(&self.generate_stmt(&index_decl)?);

                    // 2. Döngü değişkeni için yığında yer ayır (örn: color)
                    let var_decl = Stmt::VarDecl { name: var_name.clone(), ty: Type::Unknown, init: None, is_const: true, is_mutable: false };
                    code.push_str(&self.generate_stmt(&var_decl)?);

                    // YENİ ve KRİTİK DÜZELTME: Döngü değişkeninin tipini `variable_types` haritasına kaydet.
                    // `iter_expr` (colors dizisi) tipini alıp, onun iç tipini (Str) kullanıyoruz.
                    if let Type::Array(inner_type, _) = self.get_expr_type(iter_expr) {
                         self.variable_types.insert(var_name.clone(), *inner_type);
                    }

                    let color_offset = self.get_variable_offset(var_name)?;

                    // 3. Döngü başlangıcı
                    code.push_str(&format!("{}:\n", label_start));

                    // 4. Koşul: __index < dizi_boyutu (Şimdilik boyut sabit: 4)
                    let index_offset = self.get_variable_offset(&index_var_name)?;
                    code.push_str(&format!("    mov rax, qword [rel rbp{}]\n", index_offset));
                    code.push_str("    cmp rax, 4\n"); // DİKKAT: Şimdilik dizi boyutu sabit.
                    code.push_str(&format!("    jge {}\n", label_end));

                    // 5. Döngü değişkenini ata: color = colors[__index]
                    let access_expr = Expr::ArrayAccess { name: if let Expr::Variable(n) = iter_expr { n.clone() } else { "".to_string() }, index: Box::new(Expr::Variable(index_var_name.clone())) };
                    code.push_str(&self.generate_expr(&access_expr)?); // Sonuç (elemanın adresi) rax'te.
                    code.push_str(&format!("    mov qword [rel rbp{}], rax\n", color_offset));

                    code.push_str(&self.generate_stmt(body)?);

                    // 7. İndeksi artır: __index++
                    code.push_str(&format!("    inc qword [rel rbp{}]\n", index_offset));

                    // 8. Döngünün başına dön.
                    code.push_str(&format!("    jmp {}\n", label_start));
                    code.push_str(&format!("{}:\n", label_end));

                    self.current_loop_start = None;
                    self.current_loop_end = None;

                } else {
                    // --- C-stili for döngüsü kod üretimi ---
                    let label_start = self.generate_label("FOR_START");
                    let label_end = self.generate_label("FOR_END");

                    self.current_loop_start = Some(label_start.clone());
                    self.current_loop_end = Some(label_end.clone());

                    // 1. Başlatıcı (initializer): Döngüden önce sadece bir kez çalışır.
                    if let Some(init_stmt) = initializer {
                        code.push_str(&self.generate_stmt(init_stmt)?);
                    }

                    // 2. Döngü başlangıç etiketi (koşulun kontrol edildiği yer)
                    code.push_str(&format!("{}:\n", label_start));

                    // 3. Koşul (condition): Varsa, kontrol et ve gerekirse döngüden çık.
                    if let Some(cond_expr) = condition {
                        code.push_str(&self.generate_expr(cond_expr)?);
                        code.push_str("    cmp rax, 0\n"); // Koşul yanlışsa (0 ise)
                        code.push_str(&format!("    je {}\n", label_end)); // Döngüyü sonlandır.
                    }

                    // 4. Gövde (body): Döngünün ana mantığını çalıştır.
                    code.push_str(&self.generate_stmt(body)?);

                    // 5. Artırım (increment): Gövdeden sonra çalışır.
                    if let Some(inc_expr) = increment {
                        code.push_str(&self.generate_expr(inc_expr)?);
                    }

                    // 6. Döngünün başına geri dön.
                    code.push_str(&format!("    jmp {}\n", label_start));

                    // 7. Döngü bitiş etiketi.
                    code.push_str(&format!("{}:\n", label_end));

                    self.current_loop_start = None;
                    self.current_loop_end = None;
                }
            },
            
            Stmt::VarDecl { name, ty, init, .. } => {
                // YENİ MANTIK: Eğer dizi tipi 'arr' (Unknown) ise ve bir başlangıç değeri varsa,
                // dizinin gerçek tipini başlangıç değerinden çıkarıp güncelleyelim.
                let mut final_ty = ty.clone();
                if let (Type::Array(inner, _), Some(init_expr)) = (&final_ty, init) {
                    if **inner == Type::Unknown {
                        if let Expr::Tuple(elements) = init_expr {
                            if !elements.is_empty() {
                                // Dizinin iç tipini, tuple'ın ilk elemanının tipi olarak güncelle.
                                let element_type = self.get_expr_type(&elements[0]);
                                final_ty = Type::Array(Box::new(element_type), None);
                            }
                        }
                    }
                }

                                let aligned_size = 8;
                                self.variable_types.insert(name.clone(), final_ty.clone()); // Güncellenmiş tipi kaydet
                                self.current_stack_size += aligned_size; 
                                let var_offset = -self.current_stack_size; 
                                self.stack_offsets.insert(name.clone(), var_offset);
                                
                                code.push_str(&format!("    sub rsp, {}\n", aligned_size));
                
                                if let Some(expr) = init {
                                    code.push_str(&self.generate_expr(expr)?); // rax/xmm0 has VALUE
                
                                    if final_ty.is_float() {
                                        code.push_str(&format!("    movsd [rel rbp{}], xmm0\n", var_offset));
                                    } else {
                                        code.push_str(&format!("    mov qword [rel rbp{}], rax\n", var_offset));
                                    }
                                }            },

            Stmt::Assign { name, value } => {
                code.push_str(&self.generate_expr(value)?); // rax/xmm0 has VALUE

                let var_type = self.variable_types.get(name).cloned().unwrap_or(Type::Unknown); // Hedef değişkenin tipini al
                let offset = self.get_variable_offset(name)?; // Get offset for the assigned variable

                if var_type.is_float() {
                    code.push_str(&format!("    movsd [rel rbp{}], xmm0\n", offset));
                } else {
                    code.push_str(&format!("    mov qword [rel rbp{}], rax\n", offset));
                }
            },

            Stmt::Block(stmts) => {
                for s in stmts {
                    code.push_str(&self.generate_stmt(s)?);
                }
            },

            Stmt::If { cond, then_branch, else_branch } => {
                let label_else = self.generate_label("IF_ELSE");
                let label_end = self.generate_label("IF_END");

                code.push_str(&self.generate_expr(cond)?);
                code.push_str("    cmp rax, 0\n");
                
                let skip_target = if else_branch.is_some() { &label_else } else { &label_end };
                code.push_str(&format!("    je {}\n", skip_target));

                // 'then' bloğunu üret
                code.push_str(&self.generate_stmt(then_branch)?);
                
                if let Some(else_stmt) = else_branch {
                    // Eğer 'else' bloğu varsa, 'then' bloğundan sonra 'else'i atlamak için jmp ekle.
                    code.push_str(&format!("    jmp {}\n", label_end));
                    code.push_str(&format!("{}:\n", label_else));
                    code.push_str(&self.generate_stmt(else_stmt)?);
                }

                code.push_str(&format!("{}:\n", label_end));
            },

            Stmt::Return(expr) => {
                if let Some(e) = expr {
                    code.push_str(&self.generate_expr(e)?); // rax'e dönüş değerinin adresi gelecek

                    let return_type = if let Some(func_name) = &self.current_function_name {
                        if let Some(Decl::Function { return_type, .. }) = self.program.iter().find_map(|d| {
                            if let Decl::Function { name, .. } = d {
                                if name == func_name { Some(d) } else { None }
                            } else { None }
                        }) {
                            return_type.clone()
                        } else {
                            Type::Unknown // Fonksiyon bulunamazsa
                        }
                    } else {
                        Type::Unknown // Fonksiyon dışında bir return
                    };

                    if return_type.is_float() {
                        // Value is already in XMM0.
                        // xmm0 zaten float dönüş registerıdır.
                    } else {
                        // Value is already in RAX.
                    }
                } else {
                    code.push_str("    xor rax, rax\n"); // Void dönüş için rax=0
                    // Float fonksiyonlar için xmm0'ı da sıfırlamak gerekebilir ama şimdilik pas geçiyorum.
                }

                if self.current_function_name.as_ref().map_or(false, |name| name == "main") {
                    code.push_str("    mov rcx, rax\n");
                    code.push_str("    call ExitProcess\n");
                } else {
                    code.push_str("    mov rsp, rbp\n");
                    code.push_str("    pop rbp\n");
                    code.push_str("    ret\n");
                }
            },

            // YENİ: break ve continue için kod üretimi
            Stmt::Break => {
                if let Some(end_label) = &self.current_loop_end {
                    code.push_str(&format!("    jmp {}\n", end_label));
                } else {
                    return Err("Hata: 'break' sadece bir döngü içinde kullanılabilir.".to_string());
                }
            },
            Stmt::Continue => {
                if let Some(start_label) = &self.current_loop_start {
                    code.push_str(&format!("    jmp {}\n", start_label));
                } else {
                    return Err("Hata: 'continue' sadece bir döngü içinde kullanılabilir.".to_string());
                }
            },

            Stmt::ExprStmt(expr) => {
                // YENİ ÖZEL KONTROL: Eğer ifade sadece `i++` veya `i--` ise,
                // bunu daha verimli ve rax'i kirletmeyecek şekilde üret.
                if let Expr::Unary { op, right } = expr {
                    if let Expr::Variable(name) = right.as_ref() {
                        match op {
                            UnOp::PostInc | UnOp::PreInc => {
                                let offset = self.get_variable_offset(name)?;
                                code.push_str(&format!("    inc qword [rel rbp{}]\n", offset));
                                return Ok(code); // Sadece artır ve çık. rax'e dokunma.
                            },
                            UnOp::PostDec | UnOp::PreDec => {
                                let offset = self.get_variable_offset(name)?;
                                code.push_str(&format!("    dec qword [rel rbp{}]\n", offset));
                                return Ok(code); // Sadece azalt ve çık. rax'e dokunma.
                            },
                            _ => {} // Diğer Unary operatörler için normal akışa devam et.
                        }
                    }
                }

                // `print()` çağrısını burada özel olarak ele alalım.
                if let Expr::Call { callee, args } = expr {
                    if let Expr::Variable(name) = callee.as_ref() {
                        if name == "print" {
                            if args.len() != 1 { return Err("Hata: print() fonksiyonu tam olarak bir argüman almalıdır.".to_string()); }
                            if args.len() != 1 { return Err("Hata: print() fonksiyonu tam olarak bir argüman almalıdır.".to_string()); }
                            let (_, arg_expr) = &args[0];
                            code.push_str(&self.generate_expr(arg_expr)?);
                            let expr_type = self.get_expr_type(arg_expr);

                            let format_string = match expr_type {
                                Type::Str(_) => "%s",
                                Type::F32 | Type::F64 => "%f", // Float için %f formatı
                                _ => "%lld", // Varsayılan tamsayı
                            };
                            
                            code.push_str(&self.generate_printf_call(format_string, &expr_type)?);
                            return Ok(code);
                        }
                    }
                }
                code.push_str(&self.generate_expr(expr)?); // Diğer tüm ifade deyimleri için normal kod üret.
            },
            
            // Empty ve Tag gibi durumları şimdilik görmezden geliyoruz.
            Stmt::Empty => {},
            Stmt::Tag { .. } => {},
            // Diğerleri hata vermeye devam etsin.
        }
        
        Ok(code)
    }

    // YARDIMCI FONKSİYON: printf çağrısını basitleştirir.
    // Değerin zaten `rax`'te olduğunu varsayar.
    fn generate_printf_call(&mut self, format_string: &str, expr_type: &Type) -> Result<String, String> {
        let mut code = String::new();
        let fmt_label = self.generate_label("FMT");
        let bytes: Vec<String> = format_string.bytes().map(|b| b.to_string()).collect();
        self.data_section.push_str(&format!("{}: db {}, 0\n", fmt_label, bytes.join(", ")));

        code.push_str(&format!("    mov rcx, {}\n", fmt_label)); // Format string'in adresi rcx'e

        if expr_type.is_float() {
            // Value is ALREADY in XMM0 from generate_expr.
            // For variadic functions on Windows x64, integer registers for float args need to be populated.
            // Copy the float value's bit pattern from XMM0 to RDX.
            code.push_str("    sub rsp, 8\n"); // Allocate space for float on stack
            code.push_str("    movsd qword [rsp], xmm0\n"); // Store XMM0 value (float) to stack
            code.push_str("    mov rdx, qword [rsp]\n"); // Load the qword bit pattern from stack to RDX
            code.push_str("    add rsp, 8\n"); // Deallocate stack space
        } else if matches!(expr_type, Type::Str(_)) {
            // rax already contains string's address (correct from Expr::Literal for Str).
            code.push_str("    mov rdx, rax\n"); 
        } else { // Integer / Boolean
            // rax already contains integer/boolean VALUE (correct from generate_expr for Int/Bool/Variable).
            code.push_str("    mov rdx, rax\n"); // Move VALUE from RAX to RDX
        }
        
        if self.target_platform == TargetPlatform::Windows {
            code.push_str("    sub rsp, 32\n"); // Shadow space
        }
        code.push_str("    call printf\n");
        if self.target_platform == TargetPlatform::Windows {
            code.push_str("    add rsp, 32\n"); // Shadow space geri
        }
        Ok(code)
    }

    // YARDIMCI FONKSİYON: Sadece sabit bir string'i basar.
    fn generate_print_str(&mut self, s: &str) -> Result<String, String> {
        let mut code = String::new();
        let str_label = self.generate_label("PRINT_STR");
        let bytes: Vec<String> = s.bytes().map(|b| b.to_string()).collect();
        self.data_section.push_str(&format!("{}: db {}, 0\n", str_label, bytes.join(", ")));
        code.push_str(&format!("    mov rax, {}\n", str_label));
        code.push_str(&self.generate_printf_call("%s", &Type::Str(None))?);
        Ok(code)
    }

    fn generate_expr(&mut self, expr: &Expr) -> Result<String, String> {
        let mut code = String::new();

        match expr {
            Expr::Literal(lit) => {
                match lit {
                    LiteralValue::Int(i) => {
                        code.push_str(&format!("    mov rax, {}\n", i)); // Value directly in RAX
                    },
                    LiteralValue::Str(s) => {
                        let label = self.generate_label("STR_LITERAL");
                        let bytes: Vec<String> = s.bytes().map(|b| b.to_string()).collect();
                        self.data_section.push_str(&format!("{}: db {}, 0\n", label, bytes.join(", ")));
                        code.push_str(&format!("    mov rax, {}\n", label)); // Address in RAX (for strings)
                    },
                    LiteralValue::Bool(b) => {
                        let val = if *b { 1 } else { 0 };
                        code.push_str(&format!("    mov rax, {}\n", val)); // Value directly in RAX
                    },
                    LiteralValue::Float(f) => {
                        let label = self.generate_label("FLT_LITERAL");
                        let float_bits = f.to_bits(); 
                        self.data_section.push_str(&format!("{}: dq 0x{:x}\n", label, float_bits));
                        code.push_str(&format!("    movsd xmm0, qword [{}]\n", label)); // Value directly in XMM0
                    },
                    LiteralValue::Null => {
                        code.push_str("    xor rax, rax\n"); // 0 in RAX
                    },
                }
            },
            
            Expr::Variable(name) => {
                let offset = self.get_variable_offset(name)?;
                let var_type = self.variable_types.get(name).cloned().unwrap_or(Type::Unknown);
                if var_type.is_float() {
                    code.push_str(&format!("    movsd xmm0, qword [rbp{}]\n", offset)); // Value in XMM0
                } else {
                    code.push_str(&format!("    mov rax, qword [rbp{}]\n", offset)); // Value in RAX
                }
            },

            Expr::Binary { left, op, right } => {
                let left_type = self.get_expr_type(left);
                let right_type = self.get_expr_type(right);

                // TypeChecker zaten tiplerin uyumlu olduğunu varsaymalı.
                // Burada sadece float veya tamsayıya göre kod üretiyoruz.
                if left_type.is_float() && right_type.is_float() {
                    // Float İşlemleri
                    // Sağ operandı hesapla ve sonucu xmm1'e al
                    code.push_str(&self.generate_expr(right)?); // Sağ operandın adresi rax'te
                    code.push_str("    movsd xmm1, [rax]\n"); // Adresteki float değeri xmm1'e

                    // Sol operandı hesapla ve sonucu xmm0'a al
                    code.push_str(&self.generate_expr(left)?); // Sol operandın adresi rax'te
                    code.push_str("    movsd xmm0, [rax]\n"); // Adresteki float değeri xmm0'a

                    match op {
                        BinOp::Add => code.push_str("    addsd xmm0, xmm1\n"),
                        BinOp::Sub => code.push_str("    subsd xmm0, xmm1\n"),
                        BinOp::Mul => code.push_str("    mulsd xmm0, xmm1\n"),
                        BinOp::Div => code.push_str("    divsd xmm0, xmm1\n"),
                        _ => return Err(format!("Hata: Float ikili operatör Codegen'da desteklenmiyor: {:?}", op)),
                    }

                    // Sonucu belleğe kaydet ve adresini rax'e döndür.
                    let result_label = self.generate_label("FLT_RESULT");
                    self.data_section.push_str(&format!("{}: dq 0.0\n", result_label));
                    code.push_str(&format!("    movsd [rel {}], xmm0\n", result_label)); // xmm0'daki float'ı belleğe yaz
                    code.push_str(&format!("    mov rax, {}\n", result_label)); // Bellekteki adresini rax'e yükle


                } else if left_type.is_integer() && right_type.is_integer() {
                    // Tamsayı İşlemleri (Mevcut mantık)
                    // Sağ operandın değeri rax'te (ama aslında adresi), bunu yığına kaydet.
                    code.push_str(&self.generate_expr(right)?); // Sağ operandın değeri rax'te
                    code.push_str("    push rax\n"); // Sağ operandın değerini yığına kaydet

                    // Sol operandın değeri rax'te (ama aslında adresi), bunu rax'e al.
                    code.push_str(&self.generate_expr(left)?); // Sol operandın değeri rax'te

                    // Yığından sağ operandı al.
                    code.push_str("    pop rbx\n"); // Sağ operandı rbx'e al

                    match op {
                        BinOp::Add => code.push_str("    add rax, rbx\n"),
                        BinOp::Sub => code.push_str("    sub rax, rbx\n"),
                        BinOp::Mul => code.push_str("    imul rax, rbx\n"),
                        BinOp::Div => {
                            code.push_str("    cqo\n");
                            code.push_str("    idiv rbx\n");
                        },
                        BinOp::Mod => {
                            code.push_str("    cqo\n");
                            code.push_str("    idiv rbx\n");
                            code.push_str("    mov rax, rdx\n"); // Kalanı rax'e taşı
                        },
                        // YENİ: Tamsayı karşılaştırma operatörleri
                        BinOp::Equal => {
                            code.push_str("    cmp rax, rbx\n");
                            code.push_str("    sete al\n");
                            code.push_str("    movzx rax, al\n"); // Sonucu 0 veya 1 olarak rax'e al
                        },
                        BinOp::NotEqual => {
                            code.push_str("    cmp rax, rbx\n");
                            code.push_str("    setne al\n");
                            code.push_str("    movzx rax, al\n");
                        },
                        BinOp::Greater => {
                            code.push_str("    cmp rax, rbx\n");
                            code.push_str("    setg al\n");
                            code.push_str("    movzx rax, al\n");
                        },
                        BinOp::Less => {
                            code.push_str("    cmp rax, rbx\n");
                            code.push_str("    setl al\n");
                            code.push_str("    movzx rax, al\n");
                        },
                        BinOp::GreaterEqual => {
                            code.push_str("    cmp rax, rbx\n");
                            code.push_str("    setge al\n");
                            code.push_str("    movzx rax, al\n");
                        },
                        BinOp::LessEqual => {
                            code.push_str("    cmp rax, rbx\n");
                            code.push_str("    setle al\n");
                            code.push_str("    movzx rax, al\n");
                        },
                        _ => return Err(format!("Hata: Tamsayı ikili operatör Codegen'da desteklenmiyor: {:?}", op)),
                    }
                    // Removed: result_label creation and mov qword [rel {}], rax / mov rax, {}.
                    // RAX already holds the correct integer value/boolean result.

                } else if left_type == Type::Str(None) && right_type == Type::Str(None) {
                    // String İşlemleri (Sadece Eşitlik ve Eşit Değil)
                    code.push_str(&self.generate_expr(right)?); // Sağ operandın adresi rax'te
                    code.push_str("    mov r8, qword [rax]\n"); // Adresteki string adresini r8'e taşı (sağ operand)
                    code.push_str("    push r8\n"); // r8'deki değeri yığına kaydet

                    code.push_str(&self.generate_expr(left)?); // Sol operandın adresi rax'te
                    code.push_str("    mov rax, qword [rax]\n"); // Adresteki string adresini rax'e taşı (sol operand)

                    code.push_str("    pop rbx\n"); // Yığındaki sağ operandı rbx'e al (string adresi)

                    match op {
                        BinOp::Equal => {
                            code.push_str("    cmp rax, rbx\n"); // String adreslerini karşılaştır
                            code.push_str("    sete al\n");
                            code.push_str("    movzx rax, al\n"); // Sonucu 0 veya 1 olarak rax'e al
                        },
                        BinOp::NotEqual => {
                            code.push_str("    cmp rax, rbx\n"); // String adreslerini karşılaştır
                            code.push_str("    setne al\n");
                            code.push_str("    movzx rax, al\n");
                        },
                        _ => return Err(format!("Hata: String ikili operatör Codegen'da desteklenmiyor: {:?}", op)),
                    }

                    // Sonucu belleğe kaydet ve adresini rax'e döndür.
                    let result_label = self.generate_label("BOOL_RESULT_STR");
                    self.data_section.push_str(&format!("{}: dq 0\n", result_label)); // 0 ile başlat
                    code.push_str(&format!("    mov qword [rel {}], rax\n", result_label)); // rax'teki sonucu belleğe yaz
                    code.push_str(&format!("    mov rax, {}\n", result_label)); // Bellekteki adresini rax'e yükle

                } else if left_type == Type::Bool && right_type == Type::Bool {
                    // Boolean Mantıksal İşlemler
                    code.push_str(&self.generate_expr(right)?); // Sağ operandın adresi rax'te
                    code.push_str("    mov r8, qword [rax]\n"); // Adresteki bool değerini r8'e taşı
                    code.push_str("    push r8\n"); // r8'deki değeri yığına kaydet

                    code.push_str(&self.generate_expr(left)?); // Sol operandın adresi rax'te
                    code.push_str("    mov rax, qword [rax]\n"); // Adresteki bool değerini rax'e taşı

                    code.push_str("    pop rbx\n"); // Yığındaki sağ operandı rbx'e al

                    match op {
                        BinOp::And => {
                            code.push_str("    and rax, rbx\n"); // Mantıksal AND
                        },
                        BinOp::Or => {
                            code.push_str("    or rax, rbx\n"); // Mantıksal OR
                        },
                        _ => return Err(format!("Hata: Boolean ikili operatör Codegen'da desteklenmiyor: {:?}", op)),
                    }

                    // Sonucu belleğe kaydet ve adresini rax'e döndür.
                    let result_label = self.generate_label("BOOL_RESULT_LOGIC");
                    self.data_section.push_str(&format!("{}: dq 0\n", result_label)); // 0 ile başlat
                    code.push_str(&format!("    mov qword [rel {}], rax\n", result_label)); // rax'teki sonucu belleğe yaz
                    code.push_str(&format!("    mov rax, {}\n", result_label)); // Bellekteki adresini rax'e yükle

                } else {
                    return Err(format!("Hata: İkili operatör için uyumsuz veya karışık tipler ({:?} ve {:?}). Otomatik tip dönüştürme desteklenmiyor.", left_type, right_type));
                }
            },
            
            Expr::Unary { op, right } => {
                // Artırma/Azaltma operatörleri için 'right' bir değişken olmalıdır.
                // Diğer tekli operatörler (Neg, Not) herhangi bir ifade alabilir.
                // Bu yüzden mantığı ayırıyoruz.
                match op {
                    UnOp::Neg => {
                        code.push_str(&self.generate_expr(right)?);
                        code.push_str("    neg rax\n");
                    },
                    UnOp::Not => {
                        code.push_str(&self.generate_expr(right)?);
                        code.push_str("    test rax, rax\n    sete al\n    movzx rax, al\n");
                    },
                    UnOp::PostInc | UnOp::PreInc | UnOp::PostDec | UnOp::PreDec => {
                        let var_name = if let Expr::Variable(name) = right.as_ref() {
                            name
                        } else {
                            return Err("Hata: Artırma/azaltma operatörleri sadece değişkenlere uygulanabilir.".to_string());
                        };
                        let offset = self.get_variable_offset(var_name)?;

                        match op {
                            UnOp::PostInc => { // i++
                                code.push_str(&format!("    mov rax, qword [rel rbp{}]\n", offset)); // Mevcut değeri rax'e al (döndürülecek değer)
                                code.push_str(&format!("    inc qword [rel rbp{}]\n", offset));     // Yığındaki değeri 1 artır
                            },
                            UnOp::PreInc => { // ++i
                                code.push_str(&format!("    inc qword [rel rbp{}]\n", offset));     // Yığındaki değeri 1 artır
                                code.push_str(&format!("    mov rax, qword [rel rbp{}]\n", offset)); // Yeni değeri rax'e al (döndürülecek değer)
                            },
                            UnOp::PostDec => { // i--
                                code.push_str(&format!("    mov rax, qword [rel rbp{}]\n", offset)); // Mevcut değeri rax'e al
                                code.push_str(&format!("    dec qword [rel rbp{}]\n", offset));     // Yığındaki değeri 1 azalt
                            },
                            UnOp::PreDec => { // --i
                                code.push_str(&format!("    dec qword [rel rbp{}]\n", offset));     // Yığındaki değeri 1 azalt
                                code.push_str(&format!("    mov rax, qword [rel rbp{}]\n", offset)); // Yeni değeri rax'e al
                            },
                            _ => unreachable!(), // Bu kola asla girilmemeli.
                        }
                    }
                }
            },

            Expr::Call { callee, args } => {
                let callee_name = match callee.as_ref() {
                    Expr::Variable(name) => name.clone(),
                    _ => return Err("Hata: Karmaşık fonksiyon çağrıları Codegen'da desteklenmiyor.".to_string()),
                };

                // Fonksiyon tanımını self.program içinden bul
                let func_decl = self.program.iter().find_map(|d| match d {
                    Decl::Function { name, .. } if name == &callee_name => Some(d.clone()),
                    _ => None,
                }).ok_or_else(|| format!("Hata: Codegen: Fonksiyon tanımı bulunamadı: {}", callee_name))?;

                let params_def = if let Decl::Function { params, return_type: _, .. } = func_decl {
                    params
                } else {
                    unreachable!(); // TypeChecker bunu zaten kontrol etmiş olmalı
                };
                
                // Nihai argüman listesini oluştur
                let mut final_arg_exprs: Vec<Expr> = Vec::with_capacity(params_def.len());
                let mut named_args_map: HashMap<String, Expr> = HashMap::new();
                let mut positional_args: Vec<Expr> = Vec::new();

                // İsimlendirilmiş ve pozisyonel argümanları ayır
                for (arg_name_opt, arg_expr) in args.iter() {
                    if let Some(arg_name) = arg_name_opt {
                        named_args_map.insert(arg_name.clone(), arg_expr.clone());
                    } else {
                        positional_args.push(arg_expr.clone());
                    }
                }
                
                let mut positional_arg_index = 0;
                for (param_name, _, default_value_expr) in params_def.iter() {
                    // 1. İsimlendirilmiş argüman var mı?
                    if let Some(arg_expr) = named_args_map.get(param_name) {
                        final_arg_exprs.push(arg_expr.clone());
                    } 
                    // 2. Pozisyonel argüman var mı?
                    else if positional_arg_index < positional_args.len() {
                        final_arg_exprs.push(positional_args[positional_arg_index].clone());
                        positional_arg_index += 1;
                    }
                    // 3. Default değer var mı?
                    else if let Some(default_val_expr) = default_value_expr {
                        final_arg_exprs.push(default_val_expr.clone());
                    } 
                    // 4. Hiçbiri yoksa ve parametre gerekli ise (TypeChecker yakalamış olmalı)
                    else {
                        return Err(format!("Hata: Codegen: '{}' fonksiyonu için gerekli olan '{}' parametresi sağlanmadı ve varsayılan değeri yok.", callee_name, param_name));
                    }
                }
                // Fazla pozisyonel argüman kontrolü (TypeChecker zaten yakalamalı)
                if positional_arg_index < positional_args.len() {
                    return Err(format!("Hata: Codegen: '{}' fonksiyonuna çok fazla pozisyonel argüman verildi.", callee_name));
                }

                let mut arg_temp_storage: Vec<(Type, String)> = Vec::new(); // Stores (type, temporary_stack_location)

                // Evaluate arguments and store them in temporary stack locations
                for arg_expr in final_arg_exprs.iter() {
                    let param_type = self.get_expr_type(arg_expr); // Get type from current argument expression
                    code.push_str(&self.generate_expr(arg_expr)?); // Value in RAX/XMM0

                    // Allocate space on stack for argument (8 bytes aligned)
                    self.current_stack_size += 8; 
                    let offset = -self.current_stack_size;
                    let temp_stack_loc = format!("[rel rbp{}]", offset);
                    
                    if param_type.is_float() {
                        code.push_str(&format!("    movsd qword {}, xmm0\n", temp_stack_loc)); // Store float to stack
                    } else {
                        code.push_str(&format!("    mov qword {}, rax\n", temp_stack_loc)); // Store int/ptr to stack
                    }
                    arg_temp_storage.push((param_type, temp_stack_loc));
                }

                // NEW LOGIC FOR ARGUMENT PASSING
                let mut int_reg_idx = 0;
                let mut float_reg_idx = 0;
                let xmm_regs = ["xmm0", "xmm1", "xmm2", "xmm3"]; // Windows x64 float arg registers
                let arg_regs = self.get_arg_registers(); // Fetch arg_regs once here

                // Arguments are passed in reverse order to registers (RCX, RDX, R8, R9)
                // This means the first argument should go into RCX, second into RDX etc.
                // We stored arguments in arg_temp_storage in declaration order.
                // So iterate through arg_temp_storage normally.
                for (param_type, temp_stack_loc) in arg_temp_storage.iter() {
                    if param_type.is_float() {
                        if float_reg_idx < xmm_regs.len() {
                            code.push_str(&format!("    movsd {}, qword {}\n", xmm_regs[float_reg_idx], temp_stack_loc));
                            float_reg_idx += 1;
                        } else {
                            // If more than 4 float args, they should be pushed to stack (right-to-left).
                            // This scenario is not fully handled yet.
                            return Err(format!("Hata: Codegen: '{0}' fonksiyonuna 4'ten fazla float parametresi desteklenmiyor.", callee_name));
                        }
                    } else { // Integer, String, Bool, Ptr, Ref
                        if int_reg_idx < arg_regs.len() {
                            code.push_str(&format!("    mov {}, qword {}\n", arg_regs[int_reg_idx], temp_stack_loc));
                            int_reg_idx += 1;
                        } else {
                            // If more than 4 integer/pointer args, they should be pushed to stack (right-to-left).
                            // This scenario is not fully handled yet.
                            return Err(format!("Hata: Codegen: '{}' fonksiyonuna 4'ten fazla tamsayı/pointer parametresi desteklenmiyor.", callee_name));
                        }
                    }
                }
                
                // Clean up temporary stack storage
                code.push_str(&format!("    add rsp, {}\n", arg_temp_storage.len() * 8));
                self.current_stack_size -= arg_temp_storage.len() as i32 * 8;


                if self.target_platform == TargetPlatform::Windows {
                    code.push_str("    sub rsp, 32\n");
                }

                code.push_str(&format!("    call {}\n", callee_name));

                if self.target_platform == TargetPlatform::Windows {
                    code.push_str("    add rsp, 32\n");
                }
            },

            // YENİ: İfade olarak atama (örn: for (i = 0, ...))
            Expr::Assign { name, value } => {
                // 1. Sağ tarafın değerini hesapla (sonuç rax'te olacak)
                code.push_str(&self.generate_expr(value)?);

                // 2. Değişkenin zaten tanımlı olup olmadığını kontrol et
                let offset = match self.get_variable_offset(name) {
                    Ok(off) => off, // Değişken zaten tanımlı, ofsetini kullan.
                    Err(_) => {
                        // --- ÖRTÜK TANIMLAMA ---
                        // Değişken tanımlı değil, yığında yeni bir yer ayır.
                        let aligned_size = 8;
                        self.current_stack_size += aligned_size; 
                        let new_offset = -self.current_stack_size;
                        self.stack_offsets.insert(name.clone(), new_offset);
                        code.push_str(&format!("    sub rsp, {}\n", aligned_size));
                        new_offset
                    }
                };
                // 3. rax'teki sonucu değişkenin yığındaki adresine taşı.
                code.push_str(&format!("    mov qword [rel rbp{}], rax\n", offset));            }

            // YENİ: Dizi literali için kod üretimi (Parser'dan Expr::Tuple olarak gelir)
            Expr::Tuple(elements) => {
                // Bu, bir dizi literali olarak kabul edilir: [1, 2, 3] veya ["a", "b", "c"]
                // 1. Her bir eleman için .data bölümünde etiketler veya doğrudan değerler oluştur.
                let mut element_data = Vec::new(); // Hem etiketleri hem de değerleri tutacak
                let mut element_types = Vec::new(); // Eleman tiplerini de takip edelim.

                for (i, elem) in elements.iter().enumerate() {
                    match elem {
                        Expr::Literal(LiteralValue::Str(s)) => {
                            let str_label = self.generate_label(&format!("ARR_STR_{}", i));
                            let bytes: Vec<String> = s.bytes().map(|b| b.to_string()).collect();
                            self.data_section.push_str(&format!("{}: db {}, 0\n", str_label, bytes.join(", ")));
                            element_data.push(str_label);
                            element_types.push(Type::Str(None));
                        },
                        Expr::Literal(LiteralValue::Int(i_val)) => {
                            let int_label = self.generate_label(&format!("ARR_INT_{}", i));
                            self.data_section.push_str(&format!("{}: dq {}\n", int_label, i_val));
                            element_data.push(int_label);
                            element_types.push(Type::I32); // Varsayılan olarak i32
                        },
                        _ => return Err("Hata: Dizi literalleri şimdilik sadece string veya tam sayı sabitleri içerebilir.".to_string()),
                    }
                }

                // 2. Bu etiketlerin/değerlerin adreslerini tutacak olan asıl diziyi .data bölümünde oluştur.
                let array_label = self.generate_label("ARRAY_LITERAL");
                // `dq` direktifi, 8 baytlık (qword) değerleri depolar.
                // Eğer stringler veya sayılar direk gömülüyorsa, adresleri değil, kendilerini dq ile referans vermeliyiz.
                // Şimdilik hepsi 8 byte'lık adres olarak düşünülmeli.
                self.data_section.push_str(&format!("{}: dq {}\n", array_label, element_data.join(", ")));

                // 3. Dizinin başlangıç adresini rax'e yükle.
                code.push_str(&format!("    mov rax, {}\n", array_label));
            },

            // YENİ: Dizi elemanına erişim için kod üretimi
            Expr::ArrayAccess { name, index } => {
                // 1. Dizinin başlangıç adresini al (bu bir pointer).
                let array_offset = self.get_variable_offset(name)?;
                code.push_str(&format!("    mov rbx, qword [rel rbp{}]\n", array_offset)); // rbx = &colors[0]

                // 2. İndeks değerini hesapla (sonuç rax'te olacak).
                code.push_str(&self.generate_expr(index)?); // rax = index

                // 3. Elemanın adresini hesapla: başlangıç_adresi + indeks * 8 (çünkü her eleman bir pointer, 8 byte).
                // lea: Load Effective Address
                code.push_str("    imul rax, 8\n"); // rax = index * 8
                code.push_str("    add rbx, rax\n"); // rbx = &colors[0] + (index * 8) -> &colors[index]
                code.push_str("    mov rax, qword [rbx]\n"); // rax = *(&colors[index]) -> elemanın değeri (string adresi)
            },

            Expr::Match { discriminant, cases } => {
                let end_label = self.generate_label("MATCH_END");
                code.push_str(&self.generate_expr(discriminant)?); // Discriminant'ı rax'e yükle (karşılaştırılacak değer)

                let mut default_case_label: Option<String> = None;
                let mut case_labels = Vec::new();

                // Her bir case'i işle
                for (i, (pattern, _result_expr)) in cases.iter().enumerate() {
                    let case_label = self.generate_label(&format!("MATCH_CASE_{}", i));
                    case_labels.push(case_label.clone());

                    if let Expr::DefaultCase = pattern {
                        default_case_label = Some(case_label.clone());
                        continue; // Default case'i şimdilik atla, en sona işlenecek
                    }

                    // Pattern'ı değerlendir
                    code.push_str(&format!("    push rax\n")); // Discriminant'ı yığına kaydet
                    code.push_str(&self.generate_expr(pattern)?); // Pattern değerini rax'e al
                    code.push_str(&format!("    mov rbx, rax\n")); // Pattern'ı rbx'e taşı
                    code.push_str(&format!("    pop rax\n")); // Discriminant'ı rax'e geri al

                    code.push_str("    cmp rax, rbx\n"); // Discriminant ile pattern'ı karşılaştır
                    code.push_str(&format!("    je {}\n", case_label)); // Eşitse case'e atla
                }

                // Default case varsa ona atla, yoksa doğrudan son etikete atla
                if let Some(label) = default_case_label {
                    code.push_str(&format!("    jmp {}\n", label));
                } else {
                    code.push_str(&format!("    jmp {}\n", end_label));
                }
                
                // Case bloklarını oluştur
                for (i, (_pattern, result_expr)) in cases.iter().enumerate() {
                    let current_case_label = &case_labels[i];
                    code.push_str(&format!("{}:\n", current_case_label));
                    
                    if let Expr::Block { statements } = result_expr.as_ref() {
                        // Eğer sonuç bir bloksa, içindeki tüm deyimleri üret
                        for stmt in statements {
                            code.push_str(&self.generate_stmt(stmt)?);
                        }
                    } else {
                        // Eğer sonuç tek bir ifadeyse, onu üret (örn: return 5)
                        code.push_str(&self.generate_expr(result_expr)?);
                    }
                    code.push_str(&format!("    jmp {}\n", end_label)); // Case bitince match'in sonuna atla
                }

                code.push_str(&format!("{}:\n", end_label)); // Match'in bitiş etiketi
                code.push_str("    xor rax, rax\n"); // Match ifadesi bir değer döndürmezse 0 döndür (void)
            },

            _ => return Err(format!("Hata: Desteklenmeyen ifade Codegen'da: {:?} ", expr)),
        }
        
        Ok(code)
    }
}
