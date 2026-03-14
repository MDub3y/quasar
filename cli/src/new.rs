use {
    crate::{error::CliResult, style},
    std::{fs, path::Path},
};

pub fn run_instruction(name: &str) -> CliResult {
    let snake = name.replace('-', "_");
    let instructions_dir = Path::new("src").join("instructions");

    if !instructions_dir.exists() {
        eprintln!(
            "  {}",
            style::fail("src/instructions/ not found — are you in a Quasar project?")
        );
        std::process::exit(1);
    }

    let file_path = instructions_dir.join(format!("{snake}.rs"));
    if file_path.exists() {
        eprintln!(
            "  {}",
            style::fail(&format!("src/instructions/{snake}.rs already exists"))
        );
        std::process::exit(1);
    }

    // Write the instruction file
    let pascal = snake_to_pascal(&snake);
    let content = format!(
        r#"use quasar_core::prelude::*;

#[derive(Accounts)]
pub struct {pascal}<'info> {{
    pub payer: &'info mut Signer,
    pub system_program: &'info Program<System>,
}}

impl<'info> {pascal}<'info> {{
    #[inline(always)]
    pub fn {snake}(&self) -> Result<(), ProgramError> {{
        Ok(())
    }}
}}
"#
    );
    fs::write(&file_path, content).map_err(anyhow::Error::from)?;

    // Update mod.rs
    let mod_path = instructions_dir.join("mod.rs");
    let existing = fs::read_to_string(&mod_path).unwrap_or_default();

    if !existing.contains(&format!("mod {snake};")) {
        let new_line = format!("mod {snake};\npub use {snake}::*;\n");
        let updated = format!("{existing}{new_line}");
        fs::write(&mod_path, updated).map_err(anyhow::Error::from)?;
    }

    println!(
        "  {} src/instructions/{snake}.rs",
        style::success("created")
    );
    println!(
        "  {} src/instructions/mod.rs",
        style::success("updated")
    );

    Ok(())
}

fn snake_to_pascal(s: &str) -> String {
    s.split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect()
}
