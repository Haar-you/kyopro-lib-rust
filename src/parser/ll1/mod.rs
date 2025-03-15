//! LL(1)構文解析
use std::{collections::HashMap, hash::Hash};

/// 構文解析用の入力文字列
#[derive(Clone, Debug)]
pub struct Input<Char> {
    input: Vec<Char>,
    pos: usize,
}

impl Input<char> {
    /// `str`から[`Input<char>`]を構築する。
    pub fn new(s: &str) -> Self {
        Self {
            input: s.chars().collect(),
            pos: 0,
        }
    }
}

impl<Char> Input<Char>
where
    Char: Copy + Eq,
{
    /// 現在の文字が`e`に等しければ、一文字だけ消費する。
    pub fn consume_eq(&mut self, e: Char) -> Option<Char> {
        let c = *self.input.get(self.pos)?;
        (c == e).then(|| {
            self.pos += 1;
            c
        })
    }

    /// 一文字だけ消費する。
    pub fn consume(&mut self) -> Option<Char> {
        let ret = *self.input.get(self.pos)?;
        self.pos += 1;
        Some(ret)
    }

    /// 現在の文字を返す。
    pub fn peek(&self) -> Option<Char> {
        self.input.get(self.pos).copied()
    }
}

/// LL(1)構文解析器
pub struct LL1Parser<'a, State, Char, Output> {
    rules: HashMap<
        State,
        (
            Option<Box<dyn 'a + Fn(&Self, &mut Input<Char>) -> Option<Output>>>,
            Vec<(
                Box<dyn 'a + Fn(Char) -> bool>,
                Box<dyn 'a + Fn(&Self, &mut Input<Char>) -> Option<Output>>,
            )>,
        ),
    >,
}

impl<'a, State, Char, Output> LL1Parser<'a, State, Char, Output>
where
    State: Copy + Eq + Hash,
    Char: Copy + Eq,
{
    /// [`LL1Parser`]を生成する。
    pub fn new() -> Self {
        Self {
            rules: HashMap::new(),
        }
    }

    /// 規則: $\mathtt{state} \rightarrow \alpha $ を導入する。
    ///
    /// $\alpha$は`proc`で解析される部分。
    ///
    /// $\alpha$の先頭は`check_first(c)`を満たす。
    pub fn add_rule<F1, FP>(&mut self, state: State, check_first: F1, proc: FP)
    where
        F1: 'a + Fn(Char) -> bool,
        FP: 'a + Fn(&Self, &mut Input<Char>) -> Option<Output>,
    {
        self.rules
            .entry(state)
            .or_default()
            .1
            .push((Box::new(check_first), Box::new(proc)));
    }

    /// 規則: $\mathtt{state} \rightarrow \varepsilon$ を導入する。
    pub fn add_rule_empty<FP>(&mut self, state: State, proc: FP)
    where
        FP: 'a + Fn(&Self, &mut Input<Char>) -> Option<Output>,
    {
        self.rules
            .entry(state)
            .or_default()
            .0
            .replace(Box::new(proc));
    }

    /// `state`を開始状態として、`input`を構文解析する。
    pub fn parse(&self, state: State, input: &mut Input<Char>) -> Option<Output> {
        for (check_first, proc) in self.rules.get(&state)?.1.iter() {
            if let Some(c) = input.peek() {
                if check_first(c) {
                    return proc(self, input);
                }
            }
        }

        if let Some(proc) = self.rules.get(&state)?.0.as_ref() {
            return proc(self, input);
        }

        None
    }
}

#[cfg(test)]
mod test_yc_265;
