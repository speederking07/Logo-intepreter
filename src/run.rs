pub mod run {
    use crate::{ast::ast::*, effects::effects::Effect};
    use crate::turtle::turtle::*;
    use rand::prelude::*;
    use std::{collections::HashMap};

    fn evaluate_arg(arg: Argumnet, context: &HashMap<String, Value>) -> Result<Value, String> {
        match arg {
            Argumnet::List(list) => Ok(Value::Instructions(list)),
            Argumnet::Comp(comp, l_expr, r_expr) => {
                let l_res = evaluate_expr(*l_expr, context)?;
                let r_res = evaluate_expr(*r_expr, context)?;
    
                match (l_res.clone(), r_res.clone()) {
                    (Value::Number(i1), Value::Number(i2)) => match comp {
                        CompOp::Less => Ok(Value::Boolean(i1 < i2)),
                        CompOp::LessEq => Ok(Value::Boolean(i1 <= i2)),
                        CompOp::Greater => Ok(Value::Boolean(i1 > i2)),
                        CompOp::GreaterEq => Ok(Value::Boolean(i1 >= i2)),
                        CompOp::Equal => Ok(Value::Boolean(i1 == i2)),
                    },
                    _ => Err(format!("Could not comperate since {:?} {:?} are not two numbers", l_res, r_res)),
                }
            }
            Argumnet::Expr(expr) => evaluate_expr(*expr, context),
        }
    }
    
    fn evaluate_expr(expr: Expression, context: &HashMap<String, Value>) -> Result<Value, String> {
        match expr {
            Expression::Function(Func {name, args}) => {
                let vaules = args.iter()
                    .map(|arg| evaluate_arg(arg.clone(), context))
                    .map(|x| x.unwrap())
                    .collect::<Vec<_>>();
                match name.as_str() {
                    "cos" => {
                        let val = get_ith_value_as_num(&vaules, 0)?;
                        return Ok(Value::Number(val.cos()))
                    },
                    "sin" => {
                        let val = get_ith_value_as_num(&vaules, 0)?;
                        Ok(Value::Number(val.sin()))
                    },
                    "exp" => {
                        let val = get_ith_value_as_num(&vaules, 0)?;
                        Ok(Value::Number(val.exp()))
                    },
                    "red" => Ok(Value::Word("rgb(255 0, 0)".to_string())),
                    "orange" => Ok(Value::Word("rgb(255,165,0)".to_string())),
                    "yellow" => Ok(Value::Word("rgb(255, 255, 0)".to_string())),
                    "green" => Ok(Value::Word("rgb(0, 255, 0)".to_string())),
                    "blue" => Ok(Value::Word("rgb(0, 0, 255)".to_string())),
                    "violet" => Ok(Value::Word("rgb(255, 87, 51)".to_string())),
                    "black" => Ok(Value::Word("rgb(0, 0, 0)".to_string())),
                    "white" => Ok(Value::Word("rgb(255, 255, 255)".to_string())),
                    "random" => {
                        let val = get_ith_value_as_num(&vaules, 0)?;
                        Ok(Value::Number(rand::thread_rng().gen_range(0 ..= val as u32) as f64))
                    }
                    "pick" => {
                        let val = get_ith_value_as_list(&vaules, 0)?;
                        let rand = rand::thread_rng().gen_range(0 .. val.len());
                        evaluate_expr(Expression::Function(val[rand].clone()), context)
                    }
                    "repcount" => {
                        match context.get("repcount") {
                            Some(v) => Ok(v.clone()),
                            None            => Err("Using repcount outside repeate".to_string())
                        }
                    }
                    _ => Err("".to_string())
                }
            }
            Expression::UnaryOperation(op, l_expr) => {
                let l_res = evaluate_expr(*l_expr, context)?;
                match op {
                    UnaryOp::Neg => match l_res.clone() {
                        Value::Number(i1) => Ok(Value::Number(- i1)),
                        _ => Err(format!("Could not neg since {:?} is not a number", l_res)),
                    }
                }
            }
            Expression::BinaryOperation(op, l_expr, r_expr) => {
                let l_res = evaluate_expr(*l_expr, context)?;
                let r_res = evaluate_expr(*r_expr, context)?;
                match op {
                    BinaryOp::Plus => match (l_res.clone(), r_res.clone()) {
                        (Value::Number(i1), Value::Number(i2)) => Ok(Value::Number(i1 + i2)),
                        _ => Err(format!("Could not add since {:?} {:?} are not two numbers", l_res, r_res)),
                    }
                    BinaryOp::Minus => match (l_res.clone(), r_res.clone()) {
                        (Value::Number(i1), Value::Number(i2)) => Ok(Value::Number(i1 - i2)),
                        _ => Err(format!("Could not sub since {:?} {:?} are not two numbers", l_res, r_res)),
                    }
                    BinaryOp::Multiply => match (l_res.clone(), r_res.clone()) {
                        (Value::Number(i1), Value::Number(i2)) => Ok(Value::Number(i1 * i2)),
                        _ => Err(format!("Could not mul since {:?} {:?} are not two numbers", l_res, r_res)),
                    }
                    BinaryOp::Divide => match (l_res.clone(), r_res.clone()) {
                        (Value::Number(i1), Value::Number(i2)) => Ok(Value::Number(i1 / i2)),
                        _ => Err(format!("Could not div since {:?} {:?} are not two numbers", l_res, r_res)),
                    }
                    BinaryOp::Modulo => match (l_res.clone(), r_res.clone()) {
                        (Value::Number(i1), Value::Number(i2)) => Ok(Value::Number(i1 % i2)),
                        _ => Err(format!("Could not mod since {:?} {:?} are not two numbers", l_res, r_res)),
                    }
                }
            }
            Expression::Variable(var) => {
                match context.get(&var) {
                    Some(res) => Ok(res.clone()),
                    None => Err(format!("Variable {} not found in context", var)),
                }
            },
            Expression::Number(num) => Ok(Value::Number(num)),
            Expression::Word(word) => Ok(Value::Word(word))
        }
    }

    fn get_ith_value_as_num (v: &Vec<Value>, i: usize) -> Result<f64, String> {
        if v.len() < i {
            Err(format!("No argument number {}", i).to_string())
        } else {
            match &v[i] {
                Value::Number(i)=> Ok(*i),
                s => Err(format!("Argument {:?} is not a number", s).to_string()),
            }
        }
    }

    fn get_ith_value_as_list (v: &Vec<Value>, i: usize) -> Result<Vec<Func>, String> {
        if v.len() < i {
            Err(format!("No argument number {}", i).to_string())
        } else {
            match &v[i] {
                Value::Instructions(i)=> Ok(i.clone()),
                s => Err(format!("Argument {:?} is not a list of instructions", s).to_string()),
            }
        }
    }

    fn get_ith_arg_as_num (instr: &Func, context: &HashMap<String, Value>, i: usize) -> Result<f64, String> {
        if instr.args.len() < i {
            Err(format!("No argument number {} for {:?}", i, instr).to_string())
        } else {
            match evaluate_arg(instr.args[i].clone(), &context) {
                Ok(Value::Number(i)) => Ok(i),
                Ok(s) => Err(format!("No argument {:?} is not a number for {:?}", s, instr).to_string()),
                Err(e) => Err(e),
            }
        }
    }

    fn get_ith_arg_as_bool (instr: &Func, context: &HashMap<String, Value>, i: usize) -> Result<bool, String> {
        if instr.args.len() < i {
            Err(format!("No argument number {} for {:?}", i, instr).to_string())
        } else {
            match evaluate_arg(instr.args[i].clone(), &context) {
                Ok(Value::Boolean(b)) => Ok(b),
                Ok(s) => Err(format!("No argument {:?} is not a boolean for {:?}", s, instr).to_string()),
                Err(e) => Err(e),
            }
        }
    }

    fn get_ith_arg_as_word (instr: &Func, context: &HashMap<String, Value>, i: usize) -> Result<String, String> {
        if instr.args.len() < i {
            Err(format!("No argument number {} for {:?}", i, instr).to_string())
        } else {
            match evaluate_arg(instr.args[i].clone(), &context) {
                Ok(Value::Word(s)) => Ok(s),
                Ok(s) => Err(format!("No argument {:?} is not a word for {:?}", s, instr).to_string()),
                Err(e) => Err(e),
            }
        }
    }

    fn get_ith_arg_as_instr (instr: &Func, context: &HashMap<String, Value>, i: usize) -> Result<Vec<Func>, String> {
        if instr.args.len() < i {
            Err(format!("No argument number {} for {:?}", i, instr).to_string())
        } else {
            match evaluate_arg(instr.args[i].clone(), &context) {
                Ok(Value::Instructions(i)) => Ok(i),
                Ok(s) => Err(format!("No argument {:?} is not a instructions for {:?}", s, instr).to_string()),
                Err(e) => Err(e),
            }
        }
    }

    fn get_args (instr: &Func, context: &HashMap<String, Value>) -> Result<Vec<Value>, String> {
        instr.args.iter()
            .map(|a| evaluate_arg(a.clone(), &context))
            .fold(Ok(vec![]), |acc, x| Ok(cons(acc?, x?)))
    }

    fn run_instruction(instr: Func, context: &mut HashMap<String, Value>, turtle: &mut Turtle,
                       functions: &HashMap<String, FuncDeclaration>) -> Result<Vec<Effect>, String> {
        match instr.name.as_str() {
            "fd" | "forward" => {
                let d = get_ith_arg_as_num(&instr, context, 0)?;
                if turtle.pen {
                    let (p_x, p_y) = (turtle.x, turtle.y);
                    turtle.fd(d);
                    Ok(vec![Effect::Line(p_x, p_y, turtle.x, turtle.y, turtle.color.clone())])
                } else {
                    turtle.fd(d);
                    Ok(vec![])
                }
            }
            "bk" | "back" => {
                let d = get_ith_arg_as_num(&instr, &context, 0)?;
                if turtle.pen {
                    let (p_x, p_y) = (turtle.x, turtle.y);
                    turtle.fd(-d);
                    Ok(vec![Effect::Line(p_x, p_y, turtle.x, turtle.y, turtle.color.clone())])
                } else {
                    turtle.fd(-d);
                    Ok(vec![])
                }
            }
            "left" | "lt" => {
                let t = get_ith_arg_as_num(&instr, &context, 0)?;
                turtle.turn(-t);
                Ok(vec![])
            }
            "right" | "rt" => {
                let t = get_ith_arg_as_num(&instr, &context, 0)?;
                turtle.turn(t);
                Ok(vec![])
            }
            "pu" | "penup" => {
                turtle.set_pen(false);
                Ok(vec![])
            } 
            "pd" | "pendown" => {
                turtle.set_pen(true);
                Ok(vec![])
            } 
            "repeat" => {
                let rep = get_ith_arg_as_num(&instr, &context, 0)?;
                let rep_instr = get_ith_arg_as_instr(&instr, &context, 1)?;
                let mut res = vec![];
                let mut loop_context = context.clone();
                for i in 0 .. rep.round() as u32 {
                    loop_context.insert("repcount".to_string(), Value::Number(i as f64));
                    res.append(&mut run_instructions(rep_instr.clone(), &mut loop_context, turtle, functions)?);
                }
                Ok(res)
            },
            "cls" | "clearscreen" => Ok(vec![Effect::Cls]),
            "window" | "hideturtle" | "showturtle" => Ok(vec![]),
            "wait" => {
                let length = get_ith_arg_as_num(&instr, &context, 0)?;
                Ok(vec![Effect::Wait(length)])
            }
            "stop" => Ok(vec![Effect::Stop]),
            "setcolor" => {
                let color = get_ith_arg_as_word(&instr, &context, 0)?;
                turtle.set_color(Pixel::from(color));
                Ok(vec![])
            }
            "label" => {
                let text = get_ith_arg_as_word(&instr, &context, 0)?;
                let size = match context.get("label_size") {
                    Some(Value::Number(s)) => *s as u32,
                    _ => 10,
                };
                let (x, y, rotate, color) = (turtle.x, turtle.y, turtle.angle, turtle.color.clone());
                Ok(vec![Effect::Text(text, x, y, size, rotate, color)])
            }
            "setlabelheight" => {
                let size = get_ith_arg_as_num(&instr, &context, 0)?;
                context.insert("label_size".to_string(), Value::Number(size));
                Ok(vec![])
            }
            "if" => {
                if get_ith_arg_as_bool(&instr, &context, 0)? {
                    let cond_instr = get_ith_arg_as_instr(&instr, &context, 1)?;
                    run_instructions(cond_instr, context, turtle, functions)
                } else {
                    Ok(vec![])
                }
            }
            "ifelse" => {
                if get_ith_arg_as_bool(&instr, &context, 0)? {
                    let cond_instr = get_ith_arg_as_instr(&instr, &context, 1)?;
                    run_instructions(cond_instr, context, turtle, functions)
                } else {
                    let cond_instr = get_ith_arg_as_instr(&instr, &context, 2)?;
                    run_instructions(cond_instr, context, turtle, functions)
                }
            }
            fun if functions.contains_key(fun) => {
                let function = functions.get(fun).unwrap();
                if function.args.len() != instr.args.len() {
                    return Err(format!("Wrong number of arguments for function {}", fun))
                }
                let mut fun_context = get_args(&instr, context)?.into_iter()
                    .zip(function.args.iter())
                    .fold(context.clone(), |mut ctx, (v, n)| {
                        ctx.insert(n.clone(), v);
                        ctx
                    });
                let fun_effect = run_instructions(function.instructions.clone(), &mut fun_context, turtle, functions)?;
                Ok(fun_effect.into_iter().take_while(|x| *x != Effect::Stop).collect::<Vec<Effect>>())

            }
            fun => Err(format!("Function {} does not exist", fun)),
        }
    }
   
    pub fn run_instructions(instr: Vec<Func>, context: &mut HashMap<String, Value>, turtle: &mut Turtle,
                        functions: &HashMap<String, FuncDeclaration>) -> Result<Vec<Effect>, String> {
        let mut result = vec![];
        for i in instr{
            let r = run_instruction(i, context, turtle, functions);
            result.append(&mut r?);
            if result.contains(&Effect::Stop) {
                break;
            }
        }
        Ok(result)
    }
}