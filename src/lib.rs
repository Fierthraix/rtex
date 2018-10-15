use std::{
    fs::File,
    io::{self, prelude::*},
    process::exit,
};

pub struct LaTeX {}

pub mod opt;

pub type Color = String;

fn generate(formula: Option<String>, opt: &opt::Opt) -> Result<(), String> {
    let formula = match formula {
        Some(f) => f,
        None => interactive(),
    };

    if formula == "" {
        eprintln!("No input formula");
        exit(1);
    }

    let (begin_env, end_env) = if opt.environment == "$" || opt.environment == "$$" {
        (opt.environment.clone(), opt.environment.clone())
    } else {
        (
            format!("\\begin{{{}}}", opt.environment),
            format!("\\end{{{}}}", opt.environment),
        )
    };

    let prefix = format!(
        "\\documentclass[{0}pt]{{article}}{1}\\pagestyle{{empty}}{2}\\begin{{document}}{3}",
        opt.font_size, opt.packages, opt.headers, begin_env
    );

    let suffix = format!("{}\\end{{document}}", end_env);

    let tex = format!("{}{}{}", prefix, formula, suffix);

    // Save tex file for debugging
    if opt.log {
        write_file(&opt.output_file, &tex);
    }

    let latex = create_latex_file(&tex)?;

    Ok(())
}

fn write_file(filename: &str, text: &str) -> io::Result<()> {
    let mut file = File::create(filename)?;
    file.write_all(text.as_bytes())?;
    Ok(())
}

/// Remove leading/trailing whitespace and collapse multiple spaces
fn clean(s: &mut String) {
    // collapse:
    // 's/[[:space:]]+/ /'
    unimplemented!()
}

/// Interactive prompt to create the formula
fn interactive() -> String {
    unimplemented!()
}

fn create_latex_file(tex: &str) -> Result<LaTeX, String> {
    /*
       latex -halt-on-error -interaction=nonstopmode -output-directory=$TMPDIR $TEXFILE \
       | tee -a $LOGFILE \
       | sed -n '/^!/,/^ /p' >&2
    */
    unimplemented!()
}
