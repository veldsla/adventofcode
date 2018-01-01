use nom::{alpha, alphanumeric, digit, line_ending, multispace, IResult};
use std::u32;
use std::str;

use Direction;
use Action;
use State;
use Machine;
use Tape;

named!(
    number<usize>,
    map_res!(map_res!(digit, str::from_utf8), str::parse)
);

named!(
    header<(char, usize)>,
    do_parse!(
        tag!("Begin in state ") >> begin_state: alpha >> char!('.') >> line_ending
            >> tag!("Perform a diagnostic checksum after ") >> diag_steps: number
            >> tag!(" steps.") >> (begin_state[0] as char, diag_steps)
    )
);

named!(
    action<(u8, Action)>,
    do_parse!(
        tag!("If the current value is ") >> test_val: digit >> char!(':') >> multispace
            >> tag!("- Write the value ") >> write_val: digit >> char!('.') >> multispace
            >> tag!("- Move one slot to the ")
            >> direction:
                map_res!(
                    map_res!(alphanumeric, str::from_utf8),
                    str::parse::<Direction>
                ) >> char!('.') >> multispace >> tag!("- Continue with state ")
            >> next_state: alpha >> char!('.') >> multispace
            >> (
                test_val[0] - 48,
                Action {
                    write: write_val[0] - 48,
                    move_to: direction,
                    next_state: next_state[0] as char,
                }
            )
    )
);

named!(
    state<State>,
    do_parse!(
        opt!(multispace) >> tag!("In state ") >> name: alpha >> char!(':') >> multispace
            >> action_0: action >> action_1: action >> (State {
            id: name[0] as char,
            if_0: action_0.1,
            if_1: action_1.1,
        })
    )
);

named!(
    machine<Machine>,
    do_parse!(
        mhead: header >> states: many1!(state) >> (Machine {
            states: states,
            tape: Tape::new(),
            next_state: mhead.0,
            check_at: mhead.1,
            counter: 0,
        })
    )
);

pub(crate) fn parse_machine(b: &[u8]) -> Result<Machine, String> {
    match machine(b) {
        IResult::Done(_, o) => Ok(o),
        IResult::Error(_) => Err("Error parsing machine".to_owned()),
        IResult::Incomplete(_) => Err("Error parsing machine, incomplete file".to_owned()),
    }
}

#[test]
fn test_header() {
    let s = include_bytes!("../test.txt");
    let res = header(&s[..]);
    println!("{:?}", res);
    let remaining = match res.unwrap() {
        (i, o) => {
            assert_eq!(o, ('A', 6));
            i
        }
        _ => panic!(),
    };

    println!("{}", str::from_utf8(&remaining).unwrap());
    let res = state(&remaining);
    println!("{:?}", res);
    let remaining = res.unwrap();
}

#[test]
fn test() {
    let s = include_bytes!("../test.txt");
    let m = machine(&s[..]);
    println!("{:?}", m);

    let m = m.unwrap();
    println!("{}", str::from_utf8(&m.0).unwrap());
    assert_eq!(m.1.next_state, 'A');
    assert_eq!(m.1.states.len(), 2);
}
