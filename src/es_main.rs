// src/main.rs

// Tüm modüller
mod token;
mod lexer;
mod ast;
mod parser;
mod type_checker;
// mod codegen;

// doğrudan use ifadeleri
use lexer::Lexer;
use parser::Parser;
use token::TokenType;
use std::env;
use std::fs;
use std::process;
use crate::type_checker::TypeChecker;
use crate::ast::{Decl, TargetPlatform}; // YENİ: TargetPlatform'u ast'den al.
// use crate::codegen::{Codegen, TargetPlatform};
// use std::process::Command;

pub struct Config {
    pub include_paths: Vec<String>,
    pub input_file: String,
    // YENİ: Yapılandırmaya eklenen yeni alanlar.
    pub target_platform: TargetPlatform,
    pub show_help: bool,
}

fn parse_config(args: Vec<String>) -> Result<Config, String> {
    // YENİ: Varsayılan arama yollarına `./libs` eklendi.
    let mut include_paths = vec![".".to_string(), "./libs".to_string()];
    let mut input_file = String::new();

    // 1. Adım: `nim.conf` dosyasını oku (varsa)
    if let Ok(config_content) = fs::read_to_string("nim.conf") {
        for line in config_content.lines() {
            let line = line.trim();
            if line.starts_with('#') || line.is_empty() {
                continue;
            }
            if let Some((key, value)) = line.split_once('=') {
                if key.trim() == "include" {
                    include_paths.push(value.trim().to_string());
                }
            }
        }
    }

    // 2. Adım: Komut satırı argümanlarını ayrıştır (config dosyasını geçersiz kılabilir)
    let mut iter = args.into_iter().skip(1);
    let mut target_platform = TargetPlatform::Unknown;
    let mut show_help = false;

    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "-h" | "-help" | "--help" => {
                show_help = true;
                break; // Yardım bayrağı her şeyi geçersiz kılar.
            }
            "--target" => {
                if let Some(target_str) = iter.next() {
                    target_platform = match target_str.to_lowercase().as_str() {
                        "windows" => TargetPlatform::Windows,
                        "linux" => TargetPlatform::Linux,
                        "macos" => TargetPlatform::Macos,
                        _ => return Err(format!("Bilinmeyen hedef platform: '{}'. Geçerli olanlar: windows, linux, macos.", target_str)),
                    };
                } else {
                    return Err("'--target' bayrağı bir platform (windows, linux, macos) bekliyor.".to_string());
                }
            }
            _ if arg.starts_with("-I") => {
                // Hem -I/path hem de -I /path formatlarını destekle
                if arg.len() > 2 {
                    include_paths.push(arg[2..].to_string());
                } else if let Some(path) = iter.next() {
                    include_paths.push(path);
                } else {
                    return Err("'-I' bayrağı bir yol (path) bekliyor.".to_string());
                }
            }
            _ if arg.ends_with(".nim") => {
                if input_file.is_empty() {
                    input_file = arg;
                } else {
                    return Err("Şimdilik sadece tek bir kaynak dosya derlenebilir.".to_string());
                }
            }
            _ => {
                return Err(format!("Bilinmeyen argüman veya bayrak: '{}'", arg));
            }
        }
    }

    // YENİ: Eğer hedef platform belirtilmemişse, derleyicinin çalıştığı platformu varsay.
    if target_platform == TargetPlatform::Unknown {
        target_platform = match env::consts::OS {
            "windows" => TargetPlatform::Windows,
            "linux" => TargetPlatform::Linux,
            "macos" => TargetPlatform::Macos,
            unsupported_os => {
                println!("Uyarı: Bilinmeyen veya desteklenmeyen bir platformda ('{}') çalışılıyor. Platforma özel modüller yüklenirken hata oluşabilir.", unsupported_os);
                TargetPlatform::Unknown
            }
        };
    }

    // Eğer hiç kaynak dosya belirtilmemişse veya yardım istenmişse, yardım göster.
    if input_file.is_empty() {
        show_help = true;
    }

    Ok(Config { include_paths, input_file, target_platform, show_help })
}

// YENİ: Yardım mesajını gösteren fonksiyon.
fn print_help() {
    println!("NIMBLE Derleyici v0.1 - Kullanım Kılavuzu");
    println!("----------------------------------------");
    println!("Kullanım: nim <kaynak_dosya.nim> [seçenekler]\n");
    println!("Seçenekler:");
    println!("  -h, -help, --help      Bu yardım mesajını gösterir.");
    println!("  --target <platform>    Derleme hedefini belirtir. Platformlar: windows, linux, macos.");
    println!("                         (Varsayılan: Çalıştırıldığı sistem)");
    println!("  -I <yol>               Modül arama yollarına ek bir dizin ekler.");
    println!("\nÖrnek:");
    println!("  nim programim.nim --target windows -I ./ek_kutuphaneler");
}

fn main() {
    let config = match parse_config(env::args().collect()) {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Yapılandırma Hatası: {}", e);
            process::exit(1);
        }
    };

    // YENİ: Yardım gösterme kontrolü.
    if config.show_help {
        print_help();
        process::exit(0);
    }

    let source_code = fs::read_to_string(&config.input_file).unwrap_or_else(|_| {
        eprintln!("Hata: Dosya okunamadı: {}", &config.input_file);
        process::exit(1);
    });

    println!(">>> NIMBLE Derleyicisi v0.1");
    println!(">>> Aşama 1: Lexer (Sözcük Analizi)");
    
    // Lexer
    let mut lexer = Lexer::new(&source_code);
    let mut tokens = Vec::new();
    
    loop {
        let token = lexer.next_token();
        tokens.push(token.clone()); 
        if token.kind == TokenType::Eof { 
            break; 
        }
    }
    println!("  {} token üretildi.", tokens.len());
    println!("-------------------------------------\n");

    // Parser
    println!(">>> Aşama 2: Parser (Sözdizimi Analizi)");
    let mut parser = Parser::new(tokens); 
    let (program_root, errors) = parser.parse();

    if !errors.is_empty() {
        println!("\n--- Parser Hataları ---");
        for error in &errors {
            eprintln!("{}", error);
        }
        println!("-----------------------\n");
        eprintln!("Derleme, sözdizimi hataları nedeniyle durduruldu.");
        process::exit(1);
    }
  
    let program_decls: Vec<Decl> = match program_root {
        Decl::Program(decls) => decls, 
        _ => {
            eprintln!("Hata: Parser, Program kök yapısı yerine beklenmeyen bir Decl döndürdü.");
            process::exit(1);
        }
    };
	
    println!("✅ Parser başarıyla tamamlandı.");
    println!("\n--- Parser Çıktısı (AST) ---");
    println!("{:#?}", program_decls);
    println!("---------------------------\n");


    // Type Checker
    println!(">>> Aşama 3: Semantik Analiz (Tip Kontrolü)");
    let mut type_checker = TypeChecker::new(&program_decls, config.include_paths, config.target_platform);

    match type_checker.check_program() {
        Ok(_) => println!("✅ Tip Kontrolü Başarılı!"),
        Err(e) => {
            eprintln!("Tip Kontrolü Hatası: {}", e);
            process::exit(1);
        }
    }

}