use crate::solutions::*;

//pub struct Solver;

type Num = u16;

type Sym<'a> = HashMap<&'a str, Num>;
type Def<'a> = HashMap<&'a str, &'a str>;

fn bin_operate<'a>(l: &'a str, op: &str, r: &'a str, sym: &mut Sym<'a>, def: &Def<'a>) -> Num {
    let (l, r) = (eval(l, sym, def), eval(r, sym, def));
    match op {
        "AND" => l & r,
        "OR" => l | r,
        "RSHIFT" => l >> r,
        "LSHIFT" => l << r,
        _ => unreachable!(),
    }
}

fn eval<'a>(id: &'a str, sym: &mut Sym<'a>, def: &Def<'a>) -> Num {
    sym.get(id).copied().unwrap_or_else(|| {
        if let Some(expr) = def.get(id) {
            if let Some(term) = expr.strip_prefix("NOT ") {
                let res = !eval(term, sym, def);
                sym.insert(id, res);
                res
            } else if let [l, op, r] = expr.split(" ").collect::<Vec<_>>().as_slice() {
                let val = bin_operate(l, op, r, sym, def);
                sym.insert(id, val);
                val
            } else {
                let val = eval(expr, sym, def);
                sym.insert(id, val);
                val
            }
        } else if let Ok(val) = id.parse::<Num>() {
            val
        } else {
            panic!("{id} not defined!");
        }
    })
}

//impl Solution for Solver {
//type Answer = Num;
//fn solve(input: Input) -> (Self::Answer, Self::Answer) {
r#macro::solution!(2015, 7, Num, {
    let mut def = HashMap::<&str, &str>::new();
    let mut sym = HashMap::<&str, Num>::new();
    for line in input.split('\n') {
        let (expr, id) = line.split_once(" -> ").unwrap();
        if let Ok(val) = expr.parse::<Num>() {
            sym.insert(id, val);
        }
        def.insert(id, expr);
    }
    let mut sym_cpy = sym.clone();
    let part1 = eval("a", &mut sym_cpy, &def);
    sym.insert("b", part1);
    let part2 = eval("a", &mut sym, &def);
    (part1, part2)
});
//}
