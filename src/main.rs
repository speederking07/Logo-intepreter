
use std::collections::HashMap;

use crate::ast::ast::*;
use crate::effects::effects::*;
use crate::turtle::turtle::*;
use crate::run::run::*;

extern crate lalrpop;

pub mod ast;
pub mod effects;
pub mod turtle;
pub mod run;

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub parser);

fn create_functions_map(func: Vec<FuncDeclaration>) -> HashMap<String, FuncDeclaration> {
    let mut res = HashMap::new();
    for f in func {
        res.insert(f.name.clone(), f);
    }
    res
}

fn interprate_to_file(logo: &str, file_name: &str, size: u32) -> Result<(), String> {
    let logo : Logo = match parser::LogoParser::new().parse(logo) {
        Ok(l) => l,
        Err(e) => return Err(format!("Parse error {:?}", e))
    };

    let mut turtle = Turtle::new();

    let effects = run_instructions(logo.instructions, &mut HashMap::new(), &mut turtle, 
        &create_functions_map(logo.functions))?;

    match save_as_svg(file_name, effects, size as i32) {
        Ok(()) => Ok(()),
        Err(e) => Err(format!("Saving error {:?}", e))
    }
}

fn main() -> Result<(), String>{
    let star = "
    TO star
        repeat 5 [ fd 100 rt 144 ]
    END
    clearscreen
    star";

    let rand_square = "
    TO square :length
        repeat 4 [ fd :length rt 90 ]
    END
    TO randomcolor
        setcolor (pick [ red orange yellow green blue violet ])
    END
    clearscreen
    repeat 36 [ randomcolor square (random 200) rt 10 ]
    ";

    let text = "
    clearscreen window hideturtle
    repeat 144 [
      setlabelheight (repcount)
      penup
      fd (repcount) * (repcount) / 30
      label \"Logo
      bk (repcount) * (repcount) / 30
      pendown
      rt 10
      wait 5
    ]
    setcolor (blue)
    showturtle";

    let tree = "
    TO tree :size
        if :size < 5 [forward :size back :size stop]
        forward :size/3
        left 30 tree :size*2/3 right 30
        forward :size/6
        right 25 tree :size/2 left 25
        forward :size/3
        right 25 tree :size/2 left 25
        forward :size/6
        back :size
    END
    clearscreen
    pu bk 100 pd
    tree 150";

    let fern = "
    TO fern :size :sign
        if :size < 1 [ stop ]
        fd :size
        rt 70 * :sign fern :size * 0.5 :sign * (-1) lt 70 * :sign
        fd :size
        lt 70 * :sign fern :size * 0.5 :sign rt 70 * :sign
        rt 7 * :sign fern :size - 1 :sign lt 7 * :sign
        bk :size * 2
    END
    window clearscreen pu bk 150 pd
    fern 25 1
    ";

    interprate_to_file(star, "star.svg", 300)?;
    interprate_to_file(rand_square, "rand_square1.svg", 400)?;
    interprate_to_file(rand_square, "rand_square2.svg", 400)?;
    interprate_to_file(text, "text.svg", 600)?;
    interprate_to_file(tree, "tree.svg", 400)?;
    interprate_to_file(fern, "fern.svg", 600)?;

    Ok(())
}
