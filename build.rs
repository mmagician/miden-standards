use anyhow::{Context, Result};
use miden_lib::{transaction::TransactionKernel, utils::Serializable};
use std::{fs, path::Path, sync::Arc};

// Build script: discovers MASM contracts and produces MASL libraries for use at runtime.
//
// It looks for modules shaped as:
//   src/<module>/assets/contract.masm
// assembles them with the transaction kernel preloaded, and writes the result to:
//   src/<module>/generated/library.masl
// The generated artifact is later embedded by the corresponding Rust module.
use miden_assembly::{Assembler, DefaultSourceManager, LibraryPath};
use miden_objects::assembly::{Module, ModuleKind};
// We auto-discover modules under src/<module>/assets/contract.masm

fn main() -> Result<()> {
    // Scan src/ for modules following the standard layout:
    // src/<module>/assets/contract.masm → src/<module>/generated/library.masl
    let src_dir = Path::new("src");
    if src_dir.exists() {
        for entry in fs::read_dir(src_dir).with_context(|| "reading src directory")? {
            let entry = entry?;
            let meta = entry.metadata()?;
            if !meta.is_dir() {
                continue;
            }

            let module_name = entry
                .file_name()
                .to_string_lossy()
                .to_string();

            let masm_path = entry.path().join("assets").join("contract.masm");
            if masm_path.exists() {
                // Use the module name under the "standards" namespace as the library path component
                let module_path = format!("standards::{}", module_name);
                build_one(&module_name, &module_path, &masm_path)?;
            }
        }
    }

    Ok(())
}

fn build_one(name: &str, module_path: &str, masm_path: &Path) -> Result<()> {
    // 1) Read MASM source
    let src = fs::read_to_string(masm_path).with_context(|| {
        format!("reading MASM source at {}", masm_path.display())
    })?;

    // 2) Prepare assembler (kernel-preloaded, same as in tests)
    let assembler: Assembler = TransactionKernel::assembler();

    // 3) Parse the MASM module as a Library
    let source_mgr = Arc::new(DefaultSourceManager::default());
    let path = LibraryPath::new(module_path)
        .with_context(|| format!("invalid module path: {module_path}"))?;
    let module = Module::parser(ModuleKind::Library)
        .parse_str(path, &src, &source_mgr)
        .map_err(|e| anyhow::anyhow!("parsing MASM into Module: {}", e))?;

    // 4) Assemble the Library
    let library = assembler
        .clone()
        .assemble_library([module])
        .map_err(|e| anyhow::anyhow!("assembling Library: {}", e))?;

    // 5) Serialize & write to src/<name>/generated/library.masl
    let out_path = Path::new("src").join(name).join("generated").join("library.masl");
    ensure_parent_dir(&out_path)?;
    let bytes = library.to_bytes();
    fs::write(&out_path, &bytes)
        .with_context(|| format!("writing Library to {}", out_path.display()))?;

    Ok(())
}

fn ensure_parent_dir(path: &Path) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).with_context(|| format!("creating dir {}", parent.display()))?;
    }
    Ok(())
}
