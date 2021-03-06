#[macro_use]
extern crate clap;
use clap::App;

use std::fs;
use std::path::PathBuf;
use std::time::Instant;

use rayon::prelude::*;
use peg_runtime;
use colored::Colorize;

mod grammar;
mod compiler;
mod error;
mod yolol;

use error::CompilerError;
use compiler::BuildConfig;

fn main() {

    // Try to configure the terminal to accept colours, disable colourisation if it fails
    if let Err(_) = colored::control::set_virtual_terminal(true) {
        colored::control::set_override(false);
    }

    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    let input = PathBuf::from(matches.value_of("input").unwrap());
    let output = PathBuf::from(matches.value_of("output").unwrap());
    let config = BuildConfig::from_matches(&matches);

    match compile(&input, &output, &config) {
        Ok(()) => {},
        Err(CompilerError::IO(path, io)) => io_err_handler(&path, io),
        Err(CompilerError::Parse(path, code, err)) => parser_error_handler(&path, &code, err),
        Err(CompilerError::NoMainBlock) => println!("\n{}", "# No `main` block in program!".red()),
        Err(CompilerError::ExplicitPanic(msg, pos)) => println!("{}", format!("\n# ({:?}) Explicit Panic: `{}`", pos, msg).red()),
        Err(CompilerError::DuplicateFieldDeclaration(name)) => println!("{}", format!("\n# Field `{}` has already been declared", name).red()),
        Err(CompilerError::AssigningUndeclaredField(path)) => println!("{}", format!("\n# Assigning to nonexistent field `{:?}`", path).red()),
        Err(CompilerError::CompilerStageNotImplemented(msg)) => println!("{}", format!("\n# Not Implemented: `{}`", msg).red()),
        Err(CompilerError::TypeCheckFailed(a, b)) => println!("{}", format!("\n# Cannot Assign `{}` to `{}`", b, a).red()),
        Err(CompilerError::CallableNotFound(name)) => println!("{}", format!("\n# Cannot find callable `{}`", name).red()),
        Err(CompilerError::IncorrectCallParameterCount(name, expected, actual)) => println!("{}", format!("\n# Incorrect number of parameters passed to `{}` (expected {}, got {})", name, expected, actual).red()),
        Err(CompilerError::FieldTypeNotKnown(path)) => println!("{}", format!("Cannot find type for field {:?}", path).red()),
        Err(CompilerError::ExpressionTypeInferenceFailed(expr)) => println!("{}", format!("Cannot infer type for expression {:?}", expr).red()),
        Err(CompilerError::StaticTypeError(cause, expr)) => println!("{}", format!("Static error caused by {} in expression `{:?}`", cause, expr).red()),
        Err(CompilerError::ConstructorExpression()) => println!("{}", format!("Must assign constructor expression to a field").red()),
        Err(CompilerError::FieldConstructorAssignment(typ, initialisers)) => println!("{}", format!("Cannot assign a field of type `{}` from constructor expression `{:?}`", typ, initialisers).red()),
    }
}

fn compile(input: &PathBuf, output: &PathBuf, config: &BuildConfig) -> Result<(), CompilerError> {

    println!("{} `{}`", "#".bright_blue(), input.display());
    let now = Instant::now();
    let ast = try_parse(&input, 0)?;
    println!("# {}ms", now.elapsed().as_millis());

    println!("");

    fn print_examples<T, F>(title: &str, items: &Vec<T>, extract: F) where F: FnMut(&T) -> &str {
        if items.len() > 0 {
            println!("| | {} {} (e.g. {})", items.len(), title, items.iter().take(5).map(extract).map(|x| x.clone()).collect::<Vec<_>>().join(", "));
        } else {
            println!("| | 0 {}", title);
        }
    }

    println!("# Compiling");
    let now = Instant::now();
    println!("| # Examples");
    print_examples("constants", &ast.constants, |x| &x.field.name);
    print_examples("enums", &ast.enums, |x| &x.name);
    print_examples("ranges", &ast.ranges, |x| &x.name);
    print_examples("structs", &ast.structs, |x| &x.name);
    print_examples("callables", &ast.callables, |x| &x.name);

    fn do_with_timing<R, F: FnOnce() -> R>(title: &str, f: F) -> R {
        let start = Instant::now();
        let r = f();
        let end = (start.elapsed().as_micros() as f64) / 1000.0f64;
        println!("| # {} ({}ms)", title, end);
        return r;
    }

    let blocks = do_with_timing("Build Blocks", || ast.build_blocks(config))?;
    println!("| | {} blocks", blocks.blocks.len());
    let blocks = do_with_timing("Copy Macros Inline", || blocks.inline_macros(config))?;
    let blocks = do_with_timing("Materialise Struct Fields", || blocks.materialise_structs(config))?;
    let blocks = do_with_timing("Blocks To Yolol AST", || blocks.covert_yolol_blocks())?;
    println!("| | {} type mappings", blocks.types.len());
    println!("| | {} const expr", blocks.consts.len());

    //todo: split blocks into smaller blocks (which can fit on a single line)
    //todo: layout lines in order

    fs::write(output, format!("{:#?}", blocks)).map_err(|x| CompilerError::IO(output.clone(), x))?;

    println!("# {}ms", now.elapsed().as_millis());

    Ok(())
}

fn try_parse(path: &PathBuf, depth: usize) -> Result<grammar::ast::Program, CompilerError> {

    // Parse this file
    //let now = Instant::now();
    let code = fs::read_to_string(path).map_err(|x| CompilerError::IO(path.clone(), x))?;
    let ast = grammar::parser::y_parser::program(&code).map_err(|x| CompilerError::Parse(path.clone(), code, x))?;
    //println!("# {}: {}us", path.display(), now.elapsed().as_micros());

    // Parse imported files and merge into this ast
    ast.imports.clone()
        .par_iter()
        .map(|x| {
            let parent = path.parent().unwrap();
            let p = [parent, &PathBuf::from(&x.path)].iter().collect::<PathBuf>();
            println!("{}{}{} `{}` (from `{}`)", "|-".blue(), "-".repeat(depth).bright_blue(), ">".blue(), p.display(), path.display());
            let ast = try_parse(&p, depth + 1)?;
            return Ok((ast, x.namespace.clone()));
        })
        .collect::<Vec<_>>()
        .into_iter()
        .try_fold(ast, |a, b| { let b = b?; Ok(a.combine(b.0, b.1).clear_imports()) })
}

fn parser_error_handler(path: &PathBuf, code: &str, err: peg_runtime::error::ParseError<peg_runtime::str::LineCol>) {

    fn print_lines(lines: &Vec<&str>) {

        if lines.len() == 0 {
            return;
        }
    
        // Find the index of the last line which is not blank (before the last)
        let mut last_index = None;
        for i in 0..lines.len() - 1 {
            if lines[i] != "" {
                last_index = Some(i);
            }
        }
    
        // Print from the last non-blank line to the error
        if last_index.is_some() {
            for i in last_index.unwrap() .. lines.len() {
                println!("{} {} {}", (i + 1).to_string().bright_blue(), "|".bright_blue(), lines[i]);
            }
        }
        else {
            println!("{} {} {}", lines.len().to_string().bright_blue(), "|".bright_blue(), lines.last().unwrap());
        }
    }

    let lines = code.lines().take(err.location.line).collect::<Vec<_>>();
    let spaces = err.location.line.to_string().len();

    println!();
    println!("{}{} {}:{}:{}", "-".repeat(spaces + 3).bright_blue(), ">".bright_blue(), path.to_string_lossy().red().bold(), err.location.line, err.location.column);

    print_lines(&lines);

    let msg = if err.expected.tokens().count() == 1 {
        format!("Parse Error, expected: `{}`", err.expected.tokens().nth(0).unwrap().to_string())
    } else {
        format!("Parse Error, expected one of: {}", err.expected.tokens().map(|x| x.to_string()).collect::<Vec<_>>().join(", "))
    };
    println!("{}{}{}{} {}", " ".repeat(spaces + 1), "=".bright_blue(), " ".repeat(err.location.column), "^".yellow(), msg.yellow());
}

fn io_err_handler(path: &PathBuf, err: std::io::Error) {
    println!("`{}`: {}", path.display(), err.to_string().red());
}