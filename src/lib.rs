use nom::{
    IResult,
    branch::alt,
    sequence::{
        preceded,
        terminated
    },
    bytes::complete::tag,
    number::complete::double,
    character::complete::{
        newline,
        digit1
    },
    multi::many0,
    combinator::{
        map_res,
        eof
    }
};

macro_rules! parse_instruction {
    ($name:ident, $variant:ident) => {
        fn $name(input: &str) -> IResult<&str, mpl_vm::Instructions> {
            let (rest, _) = tag(stringify!($name))(input)?;
            Ok((rest, mpl_vm::Instructions::$variant))
        }
    }
}

macro_rules! parse_instruction_num {
    ($name:ident, $variant:ident) => {
        fn $name(input: &str) -> IResult<&str, mpl_vm::Instructions> {
            let (rest, value) = preceded(tag(format!("{} ", stringify!($name)).as_str()), map_res(digit1, str::parse))(input)?;
            Ok((rest, mpl_vm::Instructions::$variant(value)))
        }
    }
}

pub fn parse(input: &str) -> mpl_vm::Program {
    program(input).expect("parsing faled").1
}

pub fn try_parse(input: &str) -> Option<mpl_vm::Program> {
    Some(program(input).ok()?.1)
}

pub fn parse_instruction(input: &str) -> mpl_vm::Instructions {
    instruction(input).unwrap().1
}

pub fn try_parse_instruction(input: &str) -> Option<mpl_vm::Instructions> {
    Some(instruction(input).ok()?.1)
}

fn program(input: &str) -> IResult<&str, mpl_vm::Program> {
    let (rest, value) = terminated(many0(instruction), eof)(input)?;
    Ok((rest, mpl_vm::Program::from(value)))
}

fn instruction(input: &str) -> IResult<&str, mpl_vm::Instructions> {
    terminated(alt((
        alt((
            psh,
            pfa,
            ptap,
            pta,
            gap,
            sap,
            pek,
            inp,
            dup,
            pop,
            swp,
            lsw,
            add,
            sub,
            mul,
            div,
            _mod,
            abs,
            max,
            min
        )),
        alt((
            jmp,
            jiz,
            jnz,
            ipta,
            jmpa,
            jiza,
            jnza,
            ret
        ))
    )), newline)(input)
}

fn psh(input: &str) -> IResult<&str, mpl_vm::Instructions> {
    let (rest, value) = preceded(tag("psh "), double)(input)?;
    Ok((rest, mpl_vm::Instructions::Psh(value)))
}

parse_instruction!(pfa, Pfa);
parse_instruction!(pta, Pta);
parse_instruction!(gap, Gap);
parse_instruction!(ptap, Ptap);
parse_instruction_num!(sap, Sap);
parse_instruction!(pek, Pek);
parse_instruction!(inp, Inp);
parse_instruction!(dup, Dup);
parse_instruction!(pop, Pop);
parse_instruction!(swp, Swp);
parse_instruction_num!(lsw, Lsw);
parse_instruction!(add, Add);
parse_instruction!(sub, Sub);
parse_instruction!(mul, Mul);
parse_instruction!(div, Div);

fn _mod(input: &str) -> IResult<&str, mpl_vm::Instructions> {
    let (rest, _) = tag("mod")(input)?;
    Ok((rest, mpl_vm::Instructions::Mod))
}

parse_instruction!(abs, Abs);
parse_instruction!(max, Max);
parse_instruction!(min, Min);
parse_instruction_num!(jmp, Jmp);
parse_instruction_num!(jiz, Jiz);
parse_instruction_num!(jnz, Jnz);
parse_instruction!(ipta, Ipta);
parse_instruction!(jmpa, Jmpa);
parse_instruction!(jiza, Jiza);
parse_instruction!(jnza, Jnza);
parse_instruction!(ret, Ret);
