use std::process::Command;
use std::io;

pub fn run_finetune_script() -> io::Result<()> {
    let status = Command::new("python")
        .args(&["scripts/train.py"])
        .status()?;

    if status.success() {
        println!("Finetuning script finished successfully.");
    } else {
        eprintln!("Finetuning script failed.");
    }
    Ok(())
}
