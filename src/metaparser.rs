use std::{
    io::{Read, Write},
    process::ChildStdin,
};

use crate::symtab::{SymbolTable, SymbolTableBinding};

/// A meta-directive definition is
#[derive(Clone)]
pub struct Directive {
    pub name: &'static str,

    pub interpret: fn(&mut SymbolTable, &str, &str) -> InterpretationResult,
}

pub const NOTE_DIRECTIVE: Directive = Directive {
    name: "note",
    interpret: interpret_note,
};

pub const EXEC_DIRECTIVE: Directive = Directive {
    name: "exec",
    interpret: interpret_exec,
};

/*

{
    'meta':
        'directives': [
            MicroficheMetaDirective
            ShellMetaDirective
        ]
}
*/

pub fn interpret_note(_symtab: &mut SymbolTable, _harg: &str, _varg: &str) -> InterpretationResult {
    // usage:
    // microfiche dot_files:
    //    bashrc:
    // .      #!/bin/bash
    // .      ....
    // .  vimrc:
    /// .     ....
    // .... in regular code....
    // shell wc -l dot_files.vimrc
    // create a hash map and bind it to the entry "harg", this will store the contents of the files
    //

    //
    // next we'll split varg by lines
    // for each line
    // .  if the line ends in ':'
    //        read the indented lines underneath
    // .  add to the symbol table
    Ok(String::new())
}

pub fn interpret_exec(symtab: &mut SymbolTable, harg: &str, varg: &str) -> InterpretationResult {
    use std::process::{Command, Stdio};

    if harg.is_empty() {
        return Err(String::from("Empty shell command."));
    }
    //     pub stderr: String,

    let split_harg: Vec<&str> = harg.split_whitespace().collect();

    let mut child = Command::new(split_harg[0])
        .args(&split_harg[1..])
        .stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute shell");

    let mut stdin = child.stdin.take().expect("Failed to open stdin");

    let varg_copy = String::from(varg);
    std::thread::spawn(move || {
        stdin
            .write_all(varg_copy.as_bytes())
            .expect("Failed to write to stdin");
    });

    //let output = child.wait_with_output().expect("Failed to read stdout");

    let ecode = child.wait().expect("failed to wait on child");

    assert!(ecode.success());

    let mut stdout_string = String::new();

    if let Some(mut stdout) = child.stdout {
        stdout.read_to_string(&mut stdout_string);
    }

    Ok(stdout_string)
}

/// A meta-directive invocation is triple:
#[derive(Debug)]
pub struct MetaDirectiveInvocation {
    /// the name of the meta-directive being invoked.
    pub name: String,
    /// the "inline" argunments to the meta-directive (eg. "x in xs" in the sentence "for x in xs: ...")
    pub harg: String,
    /// the indent-adjusted lines hanging underneath the meta-directive
    pub varg: String,
}

pub type InterpretationResult = Result<String, String>;

pub fn interpret_invocation(
    symtab: &mut SymbolTable,
    MetaDirectiveInvocation { name, harg, varg }: &MetaDirectiveInvocation,
) -> InterpretationResult {
    if let Some(binding) = symtab.lookup(name) {
        match binding {
            SymbolTableBinding::Directive(directive) => (directive.interpret)(symtab, harg, varg),
            SymbolTableBinding::Data(_) => todo!(),
            SymbolTableBinding::Module(_) => todo!(),
        }
    } else {
        Err(format!("Undefined directive {name}"))
    }
}

pub fn read_directive(directive: &str) -> MetaDirectiveInvocation {
    //dbg!(directive);
    //dbg!(directive.len());
    let pos_eoh = directive.find('\n').unwrap_or(directive.len());
    let head = &directive[..pos_eoh];

    let pos_eod = head
        .find(|c: char| c.is_whitespace() || c.is_ascii_punctuation())
        .unwrap_or(head.len());
    let name: String = String::from(&head[..pos_eod]);

    let pos_boh = pos_eod + 1;
    let harg = String::from(if pos_boh < head.len() {
        &head[pos_boh..]
    } else {
        ""
    });

    let mut varg: String = String::new();
    //dbg!(pos_eoh);
    if pos_eoh + 1 < directive.len() {
        directive[pos_eoh + 1..].lines().for_each(|line| {
            assert!(&line[..4] == "    ");
            varg.push_str(&line[4..]);
            varg.push('\n');
        });
    }

    MetaDirectiveInvocation { name, harg, varg }
}

pub fn interpret_directive(symtab: &mut SymbolTable, directive: &str) -> InterpretationResult {
    let directive = read_directive(directive);
    interpret_invocation(symtab, &directive)
}

pub fn interpret(input: &str) -> InterpretationResult {
    let mut symtab = SymbolTable::root();

    let mut remaining: &str = input;
    let mut curr_directive: String = String::new();

    fn print_result(result: &InterpretationResult) {
        match result {
            Ok(output) => {
                print!("{}", output);
            }
            Err(error) => {
                panic!("Error: {}", error);
            }
        };
    }

    loop {
        match remaining.find('\n') {
            Some(pos) => {
                if remaining[pos + 1..].starts_with(' ') {
                    curr_directive.push_str(&remaining[..pos + 1]);
                } else {
                    curr_directive.push_str(&remaining[..pos]);
                    print_result(&interpret_directive(&mut symtab, &curr_directive));
                    curr_directive = String::new();
                }
                remaining = &remaining[pos + 1..];
            }
            None => {
                curr_directive.push_str(remaining);
                let result = interpret_directive(&mut symtab, &curr_directive);
                print_result(&result);
                break result;
            }
        };
    }
}
