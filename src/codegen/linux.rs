// src/codegen/linux.rs

use crate::codegen::Codegen;

#[allow(dead_code)] // Bu fonksiyon artık doğrudan kullanılmıyor.
pub fn generate_platform_code(_codegen: &mut Codegen) -> String {
    // Bu fonksiyon artık kullanılmıyor, mantık Codegen::generate_text_segment'e taşındı.
    // Ancak modül yapısını korumak için boş bir string döndürüyoruz.
    String::new()
}