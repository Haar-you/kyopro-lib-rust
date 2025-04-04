use super::*;

#[derive(Clone, Debug, PartialEq)]
struct Poly {
    pub data: [i64; 11],
}

impl Poly {
    fn new() -> Self {
        Self { data: [0; 11] }
    }

    fn one() -> Self {
        let mut ret = Self::new();
        ret.data[0] = 1;
        ret
    }

    fn x() -> Self {
        let mut ret = Self::new();
        ret.data[1] = 1;
        ret
    }

    fn add(a: Self, b: Self) -> Self {
        let mut ret = Poly::new();
        for i in 0..=10 {
            ret.data[i] = a.data[i] + b.data[i];
        }
        ret
    }

    fn mul(a: Self, b: Self) -> Self {
        let mut ret = Poly::new();
        for i in 0..=10 {
            for j in 0..=10 {
                if i + j <= 10 {
                    ret.data[i + j] += a.data[i] * b.data[j];
                }
            }
        }
        ret
    }

    fn differentiate(a: Self) -> Self {
        let mut ret = Poly::new();
        for i in 0..10 {
            ret.data[i] = a.data[i + 1] * (i + 1) as i64;
        }
        ret
    }
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
enum State {
    Number,
    Factor,
    Term,
    Term2,
    Expr,
    Expr2,
}

#[test]
fn test() {
    // https://yukicoder.me/problems/no/265

    let mut parser = LL1Parser::<State, char, Poly>::new();

    // Number -> 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9
    parser.add_rule(
        State::Number,
        |c| c.is_ascii_digit(),
        |_, input| {
            let n = input.consume()?.to_digit(10)?;
            let mut ret = Poly::new();
            ret.data[0] = n as i64;
            Some(ret)
        },
    );

    // Factor -> Number
    parser.add_rule(
        State::Factor,
        |c| c.is_ascii_digit(),
        |slf, input| slf.parse(State::Number, input),
    );

    // Factor -> `x`
    parser.add_rule(
        State::Factor,
        |c| c == 'x',
        |_, input| {
            input.consume_eq('x')?;
            Some(Poly::x())
        },
    );

    // Factor -> `d` `{` Expr `}`
    parser.add_rule(
        State::Factor,
        |c| c == 'd',
        |slf, input| {
            input.consume_eq('d')?;
            input.consume_eq('{')?;
            let temp = slf.parse(State::Expr, input)?;
            let ret = Poly::differentiate(temp);
            input.consume_eq('}')?;
            Some(ret)
        },
    );

    // Term2 -> `*` Factor Term2
    parser.add_rule(
        State::Term2,
        |c| c == '*',
        |slf, input| {
            input.consume_eq('*')?;
            let a = slf.parse(State::Factor, input)?;
            let b = slf.parse(State::Term2, input)?;
            Some(Poly::mul(a, b))
        },
    );

    // Term2 -> Empty
    parser.add_rule_empty(State::Term2, |_, _| Some(Poly::one()));

    // Term -> Factor Term2
    parser.add_rule(
        State::Term,
        |_| true,
        |slf, input| {
            let a = slf.parse(State::Factor, input)?;
            let b = slf.parse(State::Term2, input)?;
            Some(Poly::mul(a, b))
        },
    );

    // Expr2 -> `+` Term Expr2
    parser.add_rule(
        State::Expr2,
        |c| c == '+',
        |slf, input| {
            input.consume_eq('+')?;
            let a = slf.parse(State::Term, input)?;
            let b = slf.parse(State::Expr2, input)?;
            Some(Poly::add(a, b))
        },
    );

    // Expr2 -> Empty
    parser.add_rule_empty(State::Expr2, |_, _| Some(Poly::new()));

    // Expr -> Term Expr2
    parser.add_rule(
        State::Expr,
        |_| true,
        |slf, input| {
            let a = slf.parse(State::Term, input)?;
            let b = slf.parse(State::Expr2, input)?;
            Some(Poly::add(a, b))
        },
    );

    let res = parser.parse(State::Expr, &mut Input::new("d{2*x+d{3*x*x*x}}+x"));
    assert_eq!(
        res,
        Some(Poly {
            data: [2, 19, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        })
    );
}
