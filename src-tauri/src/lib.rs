// Bibliothèque partagée entre le binaire principal (app Tauri) et le serveur
// MCP stdio `alto-mcp` : moteurs PDF (PDFium), opérations, OCR et client LLM.
pub mod llm;
pub mod ocr;
pub mod pdf_compress;
pub mod pdf_edit;
pub mod pdf_forms;
pub mod pdf_engine;
pub mod pdf_ops;
pub mod pdf_sign;
pub mod pdf_tools;
