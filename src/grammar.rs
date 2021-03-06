// auto-generated: "lalrpop 0.19.6"
// sha3: e69844c2209ecd8763e08831baf79c8b9beb8e7a2be0f1f2e12d359767727f
use std::str::FromStr;
use crate::ast::{Expr, Opcode};
use lalrpop_util::ParseError;
use lalrpop_util::ErrorRecovery;
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;
#[allow(unused_imports)]
use self::__lalrpop_util::state_machine as __state_machine;
extern crate core;
extern crate alloc;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__Expr {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use std::str::FromStr;
    use crate::ast::{Expr, Opcode};
    use lalrpop_util::ParseError;
    use lalrpop_util::ErrorRecovery;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    extern crate core;
    extern crate alloc;
    use self::__lalrpop_util::lexer::Token;
    #[allow(dead_code)]
    pub(crate) enum __Symbol<'input>
     {
        Variant0(&'input str),
        Variant1(__lalrpop_util::ErrorRecovery<usize, Token<'input>, &'static str>),
        Variant2(Box<Expr>),
        Variant3(alloc::vec::Vec<Box<Expr>>),
        Variant4(core::option::Option<Box<Expr>>),
        Variant5(Opcode),
        Variant6(Vec<Box<Expr>>),
        Variant7(i32),
    }
    const __ACTION: &[i8] = &[
        // State 0
        4, 0, 0, 0, 0, 0, 0, 11, 12,
        // State 1
        0, -6, 0, 13, 0, 14, 0, 0, 0,
        // State 2
        0, -12, 15, -12, 0, -12, 16, 0, 0,
        // State 3
        4, 0, 0, 0, 0, 0, 0, 11, 12,
        // State 4
        4, 0, 0, 0, 0, 0, 0, 11, 12,
        // State 5
        4, 0, 0, 0, 0, 0, 0, 11, 12,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, -24, 0, -24, 0, -24, 0, 0, 0,
        // State 8
        0, -20, -20, -20, 0, -20, -20, 0, 0,
        // State 9
        0, -26, -26, -26, 0, -26, -26, 0, 0,
        // State 10
        0, -19, -19, -19, 0, -19, -19, 0, 0,
        // State 11
        0, -22, -22, -22, 0, -22, -22, 0, 0,
        // State 12
        -9, 0, 0, 0, 0, 0, 0, -9, -9,
        // State 13
        -10, 0, 0, 0, 0, 0, 0, -10, -10,
        // State 14
        -13, 0, 0, 0, 0, 0, 0, -13, -13,
        // State 15
        -14, 0, 0, 0, 0, 0, 0, -14, -14,
        // State 16
        0, 20, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, -23, 0, -23, 0, -23, 0, 0, 0,
        // State 18
        0, -25, -25, -25, 0, -25, -25, 0, 0,
        // State 19
        0, -21, -21, -21, 0, -21, -21, 0, 0,
    ];
    fn __action(state: i8, integer: usize) -> i8 {
        __ACTION[(state as usize) * 9 + integer]
    }
    const __EOF_ACTION: &[i8] = &[
        // State 0
        0,
        // State 1
        -6,
        // State 2
        -12,
        // State 3
        0,
        // State 4
        0,
        // State 5
        0,
        // State 6
        -27,
        // State 7
        -24,
        // State 8
        -20,
        // State 9
        -26,
        // State 10
        -19,
        // State 11
        -22,
        // State 12
        0,
        // State 13
        0,
        // State 14
        0,
        // State 15
        0,
        // State 16
        0,
        // State 17
        -23,
        // State 18
        -25,
        // State 19
        -21,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            3 => match state {
                3 => 16,
                _ => 6,
            },
            5 => 4,
            7 => match state {
                4 => 17,
                _ => 7,
            },
            8 => 5,
            10 => 8,
            11 => match state {
                5 => 18,
                _ => 9,
            },
            12 => 1,
            13 => 2,
            _ => 0,
        }
    }
    fn __expected_tokens(__state: i8) -> alloc::vec::Vec<alloc::string::String> {
        const __TERMINAL: &[&str] = &[
            r###""(""###,
            r###"")""###,
            r###""*""###,
            r###""+""###,
            r###"",""###,
            r###""-""###,
            r###""/""###,
            r###"r#"[0-9]+"#"###,
        ];
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            let next_state = __action(__state, index);
            if next_state == 0 {
                None
            } else {
                Some(alloc::string::ToString::to_string(terminal))
            }
        }).collect()
    }
    pub(crate) struct __StateMachine<'input, 'err>
    where 'input: 'err, 'static: 'err
    {
        scale: i32,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
        input: &'input str,
        __phantom: core::marker::PhantomData<(&'input (), &'err ())>,
    }
    impl<'input, 'err> __state_machine::ParserDefinition for __StateMachine<'input, 'err>
    where 'input: 'err, 'static: 'err
    {
        type Location = usize;
        type Error = &'static str;
        type Token = Token<'input>;
        type TokenIndex = usize;
        type Symbol = __Symbol<'input>;
        type Success = Box<Expr>;
        type StateIndex = i8;
        type Action = i8;
        type ReduceIndex = i8;
        type NonterminalIndex = usize;

        #[inline]
        fn start_location(&self) -> Self::Location {
              Default::default()
        }

        #[inline]
        fn start_state(&self) -> Self::StateIndex {
              0
        }

        #[inline]
        fn token_to_index(&self, token: &Self::Token) -> Option<usize> {
            __token_to_integer(token, core::marker::PhantomData::<(&(), &())>)
        }

        #[inline]
        fn action(&self, state: i8, integer: usize) -> i8 {
            __action(state, integer)
        }

        #[inline]
        fn error_action(&self, state: i8) -> i8 {
            __action(state, 9 - 1)
        }

        #[inline]
        fn eof_action(&self, state: i8) -> i8 {
            __EOF_ACTION[state as usize]
        }

        #[inline]
        fn goto(&self, state: i8, nt: usize) -> i8 {
            __goto(state, nt)
        }

        fn token_to_symbol(&self, token_index: usize, token: Self::Token) -> Self::Symbol {
            __token_to_symbol(token_index, token, core::marker::PhantomData::<(&(), &())>)
        }

        fn expected_tokens(&self, state: i8) -> alloc::vec::Vec<alloc::string::String> {
            __expected_tokens(state)
        }

        #[inline]
        fn uses_error_recovery(&self) -> bool {
            true
        }

        #[inline]
        fn error_recovery_symbol(
            &self,
            recovery: __state_machine::ErrorRecovery<Self>,
        ) -> Self::Symbol {
            __Symbol::Variant1(recovery)
        }

        fn reduce(
            &mut self,
            action: i8,
            start_location: Option<&Self::Location>,
            states: &mut alloc::vec::Vec<i8>,
            symbols: &mut alloc::vec::Vec<__state_machine::SymbolTriple<Self>>,
        ) -> Option<__state_machine::ParseResult<Self>> {
            __reduce(
                self.scale,
                self.errors,
                self.input,
                action,
                start_location,
                states,
                symbols,
                core::marker::PhantomData::<(&(), &())>,
            )
        }

        fn simulate_reduce(&self, action: i8) -> __state_machine::SimulatedReduce<Self> {
            __simulate_reduce(action, core::marker::PhantomData::<(&(), &())>)
        }
    }
    fn __token_to_integer<
        'input,
        'err,
    >(
        __token: &Token<'input>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> Option<usize>
    {
        match *__token {
            Token(1, _) if true => Some(0),
            Token(2, _) if true => Some(1),
            Token(3, _) if true => Some(2),
            Token(4, _) if true => Some(3),
            Token(5, _) if true => Some(4),
            Token(6, _) if true => Some(5),
            Token(7, _) if true => Some(6),
            Token(0, _) if true => Some(7),
            _ => None,
        }
    }
    fn __token_to_symbol<
        'input,
        'err,
    >(
        __token_index: usize,
        __token: Token<'input>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> __Symbol<'input>
    {
        match __token_index {
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 => match __token {
                Token(1, __tok0) | Token(2, __tok0) | Token(3, __tok0) | Token(4, __tok0) | Token(5, __tok0) | Token(6, __tok0) | Token(7, __tok0) | Token(0, __tok0) if true => __Symbol::Variant0(__tok0),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    fn __simulate_reduce<
        'input,
        'err,
    >(
        __reduce_index: i8,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> __state_machine::SimulatedReduce<__StateMachine<'input, 'err>>
    where
        'input: 'err,
        'static: 'err,
    {
        match __reduce_index {
            0 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 0,
                }
            }
            1 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 1,
                }
            }
            2 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 1,
                }
            }
            3 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 2,
                }
            }
            4 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 2,
                }
            }
            5 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 3,
                }
            }
            6 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 4,
                }
            }
            7 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 4,
                }
            }
            8 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 5,
                }
            }
            9 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 5,
                }
            }
            10 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 6,
                }
            }
            11 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 7,
                }
            }
            12 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 8,
                }
            }
            13 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 8,
                }
            }
            14 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 9,
                }
            }
            15 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 9,
                }
            }
            16 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 9,
                }
            }
            17 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 9,
                }
            }
            18 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 10,
                }
            }
            19 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 11,
                }
            }
            20 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 11,
                }
            }
            21 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 11,
                }
            }
            22 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 12,
                }
            }
            23 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 12,
                }
            }
            24 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 13,
                }
            }
            25 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 13,
                }
            }
            26 => __state_machine::SimulatedReduce::Accept,
            27 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 15,
                }
            }
            _ => panic!("invalid reduction index {}", __reduce_index)
        }
    }
    pub struct ExprParser {
        builder: __lalrpop_util::lexer::MatcherBuilder,
        _priv: (),
    }

    impl ExprParser {
        pub fn new() -> ExprParser {
            let __builder = super::__intern_token::new_builder();
            ExprParser {
                builder: __builder,
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            'input,
            'err,
        >(
            &self,
            scale: i32,
            errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
            input: &'input str,
        ) -> Result<Box<Expr>, __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>
        {
            let mut __tokens = self.builder.matcher(input);
            __state_machine::Parser::drive(
                __StateMachine {
                    scale,
                    errors,
                    input,
                    __phantom: core::marker::PhantomData::<(&(), &())>,
                },
                __tokens,
            )
        }
    }
    fn __accepts<
        'input,
        'err,
    >(
        scale: i32,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
        input: &'input str,
        __error_state: i8,
        __states: & [i8],
        __opt_integer: Option<usize>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> bool
    {
        let mut __states = __states.to_vec();
        __states.push(__error_state);
        loop {
            let mut __states_len = __states.len();
            let __top = __states[__states_len - 1];
            let __action = match __opt_integer {
                None => __EOF_ACTION[__top as usize],
                Some(__integer) => __action(__top, __integer),
            };
            if __action == 0 { return false; }
            if __action > 0 { return true; }
            let (__to_pop, __nt) = match __simulate_reduce(-(__action + 1), core::marker::PhantomData::<(&(), &())>) {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop, nonterminal_produced
                } => (states_to_pop, nonterminal_produced),
                __state_machine::SimulatedReduce::Accept => return true,
            };
            __states_len -= __to_pop;
            __states.truncate(__states_len);
            let __top = __states[__states_len - 1];
            let __next_state = __goto(__top, __nt);
            __states.push(__next_state);
        }
    }
    pub(crate) fn __reduce<
        'input,
        'err,
    >(
        scale: i32,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut alloc::vec::Vec<i8>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> Option<Result<Box<Expr>,__lalrpop_util::ParseError<usize, Token<'input>, &'static str>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                __reduce0(scale, errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            1 => {
                __reduce1(scale, errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            2 => {
                __reduce2(scale, errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            3 => {
                __reduce3(scale, errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            4 => {
                __reduce4(scale, errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            5 => {
                __reduce5(scale, errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            6 => {
                __reduce6(scale, errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            7 => {
                __reduce7(scale, errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            8 => {
                __reduce8(scale, errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            9 => {
                __reduce9(scale, errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            10 => {
                __reduce10(scale, errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            11 => {
                __reduce11(scale, errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            12 => {
                __reduce12(scale, errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            13 => {
                __reduce13(scale, errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            14 => {
                __reduce14(scale, errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            15 => {
                __reduce15(scale, errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            16 => {
                __reduce16(scale, errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            17 => {
                __reduce17(scale, errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            18 => {
                // Num = r#"[0-9]+"# => ActionFn(12);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action12::<>(scale, errors, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant7(__nt), __end));
                (1, 10)
            }
            19 => {
                __reduce19(scale, errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            20 => {
                __reduce20(scale, errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            21 => {
                __reduce21(scale, errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            22 => {
                __reduce22(scale, errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            23 => {
                __reduce23(scale, errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            24 => {
                __reduce24(scale, errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            25 => {
                __reduce25(scale, errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            26 => {
                // __Expr = Expr => ActionFn(1);
                let __sym0 = __pop_Variant2(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(scale, errors, input, __sym0);
                return Some(Ok(__nt));
            }
            27 => {
                __reduce27(scale, errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        let __state = *__states.last().unwrap();
        let __next_state = __goto(__state, __nonterminal);
        __states.push(__next_state);
        None
    }
    #[inline(never)]
    fn __symbol_type_mismatch() -> ! {
        panic!("symbol type mismatch")
    }
    fn __pop_Variant2<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expr>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant5<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Opcode, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant5(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant6<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Box<Expr>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant6(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant1<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, __lalrpop_util::ErrorRecovery<usize, Token<'input>, &'static str>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant3<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, alloc::vec::Vec<Box<Expr>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant4<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, core::option::Option<Box<Expr>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant7<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, i32, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant7(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant0<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant0(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    pub(crate) fn __reduce0<
        'input,
        'err,
    >(
        scale: i32,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<Expr> ",") = Expr, "," => ActionFn(22);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action22::<>(scale, errors, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (2, 0)
    }
    pub(crate) fn __reduce1<
        'input,
        'err,
    >(
        scale: i32,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<Expr> ",")* =  => ActionFn(20);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action20::<>(scale, errors, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (0, 1)
    }
    pub(crate) fn __reduce2<
        'input,
        'err,
    >(
        scale: i32,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<Expr> ",")* = (<Expr> ",")+ => ActionFn(21);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action21::<>(scale, errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 1)
    }
    pub(crate) fn __reduce3<
        'input,
        'err,
    >(
        scale: i32,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<Expr> ",")+ = Expr, "," => ActionFn(25);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action25::<>(scale, errors, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 2)
    }
    pub(crate) fn __reduce4<
        'input,
        'err,
    >(
        scale: i32,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<Expr> ",")+ = (<Expr> ",")+, Expr, "," => ActionFn(26);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action26::<>(scale, errors, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 2)
    }
    pub(crate) fn __reduce5<
        'input,
        'err,
    >(
        scale: i32,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Expr = Tier<ExprOp, Factor> => ActionFn(3);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action3::<>(scale, errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 3)
    }
    pub(crate) fn __reduce6<
        'input,
        'err,
    >(
        scale: i32,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Expr? = Expr => ActionFn(18);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action18::<>(scale, errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 4)
    }
    pub(crate) fn __reduce7<
        'input,
        'err,
    >(
        scale: i32,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Expr? =  => ActionFn(19);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action19::<>(scale, errors, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (0, 4)
    }
    pub(crate) fn __reduce8<
        'input,
        'err,
    >(
        scale: i32,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ExprOp = "+" => ActionFn(4);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action4::<>(scale, errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    pub(crate) fn __reduce9<
        'input,
        'err,
    >(
        scale: i32,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ExprOp = "-" => ActionFn(5);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action5::<>(scale, errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    pub(crate) fn __reduce10<
        'input,
        'err,
    >(
        scale: i32,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Exprs = List<Expr> => ActionFn(2);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action2::<>(scale, errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce11<
        'input,
        'err,
    >(
        scale: i32,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Factor = Tier<FactorOp, Term> => ActionFn(6);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action6::<>(scale, errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 7)
    }
    pub(crate) fn __reduce12<
        'input,
        'err,
    >(
        scale: i32,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // FactorOp = "*" => ActionFn(7);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action7::<>(scale, errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 8)
    }
    pub(crate) fn __reduce13<
        'input,
        'err,
    >(
        scale: i32,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // FactorOp = "/" => ActionFn(8);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action8::<>(scale, errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 8)
    }
    pub(crate) fn __reduce14<
        'input,
        'err,
    >(
        scale: i32,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // List<Expr> = Expr => ActionFn(29);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action29::<>(scale, errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce15<
        'input,
        'err,
    >(
        scale: i32,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // List<Expr> =  => ActionFn(30);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action30::<>(scale, errors, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (0, 9)
    }
    pub(crate) fn __reduce16<
        'input,
        'err,
    >(
        scale: i32,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // List<Expr> = (<Expr> ",")+, Expr => ActionFn(31);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action31::<>(scale, errors, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (2, 9)
    }
    pub(crate) fn __reduce17<
        'input,
        'err,
    >(
        scale: i32,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // List<Expr> = (<Expr> ",")+ => ActionFn(32);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action32::<>(scale, errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce19<
        'input,
        'err,
    >(
        scale: i32,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Term = Num => ActionFn(9);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action9::<>(scale, errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce20<
        'input,
        'err,
    >(
        scale: i32,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Term = "(", Expr, ")" => ActionFn(10);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action10::<>(scale, errors, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (3, 11)
    }
    pub(crate) fn __reduce21<
        'input,
        'err,
    >(
        scale: i32,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Term = error => ActionFn(11);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action11::<>(scale, errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce22<
        'input,
        'err,
    >(
        scale: i32,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Tier<ExprOp, Factor> = Tier<ExprOp, Factor>, ExprOp, Factor => ActionFn(15);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant2(__symbols);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action15::<>(scale, errors, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (3, 12)
    }
    pub(crate) fn __reduce23<
        'input,
        'err,
    >(
        scale: i32,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Tier<ExprOp, Factor> = Factor => ActionFn(16);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action16::<>(scale, errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 12)
    }
    pub(crate) fn __reduce24<
        'input,
        'err,
    >(
        scale: i32,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Tier<FactorOp, Term> = Tier<FactorOp, Term>, FactorOp, Term => ActionFn(13);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant2(__symbols);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action13::<>(scale, errors, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (3, 13)
    }
    pub(crate) fn __reduce25<
        'input,
        'err,
    >(
        scale: i32,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Tier<FactorOp, Term> = Term => ActionFn(14);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action14::<>(scale, errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 13)
    }
    pub(crate) fn __reduce27<
        'input,
        'err,
    >(
        scale: i32,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // __Exprs = Exprs => ActionFn(0);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action0::<>(scale, errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 15)
    }
}
pub use self::__parse__Expr::ExprParser;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__Exprs {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use std::str::FromStr;
    use crate::ast::{Expr, Opcode};
    use lalrpop_util::ParseError;
    use lalrpop_util::ErrorRecovery;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    extern crate core;
    extern crate alloc;
    use self::__lalrpop_util::lexer::Token;
    #[allow(dead_code)]
    pub(crate) enum __Symbol<'input>
     {
        Variant0(&'input str),
        Variant1(__lalrpop_util::ErrorRecovery<usize, Token<'input>, &'static str>),
        Variant2(Box<Expr>),
        Variant3(alloc::vec::Vec<Box<Expr>>),
        Variant4(core::option::Option<Box<Expr>>),
        Variant5(Opcode),
        Variant6(Vec<Box<Expr>>),
        Variant7(i32),
    }
    const __ACTION: &[i8] = &[
        // State 0
        5, 0, 0, 0, 0, 0, 0, 14, 15,
        // State 1
        5, 0, 0, 0, 0, 0, 0, 14, 15,
        // State 2
        0, -6, 0, 18, -6, 19, 0, 0, 0,
        // State 3
        0, -12, 20, -12, -12, -12, 21, 0, 0,
        // State 4
        5, 0, 0, 0, 0, 0, 0, 14, 15,
        // State 5
        5, 0, 0, 0, 0, 0, 0, 14, 15,
        // State 6
        5, 0, 0, 0, 0, 0, 0, 14, 15,
        // State 7
        0, 0, 0, 0, 17, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, -24, 0, -24, -24, -24, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, -20, -20, -20, -20, -20, -20, 0, 0,
        // State 12
        0, -26, -26, -26, -26, -26, -26, 0, 0,
        // State 13
        0, -19, -19, -19, -19, -19, -19, 0, 0,
        // State 14
        0, -22, -22, -22, -22, -22, -22, 0, 0,
        // State 15
        0, 0, 0, 0, 23, 0, 0, 0, 0,
        // State 16
        -4, 0, 0, 0, 0, 0, 0, -4, -4,
        // State 17
        -9, 0, 0, 0, 0, 0, 0, -9, -9,
        // State 18
        -10, 0, 0, 0, 0, 0, 0, -10, -10,
        // State 19
        -13, 0, 0, 0, 0, 0, 0, -13, -13,
        // State 20
        -14, 0, 0, 0, 0, 0, 0, -14, -14,
        // State 21
        0, 26, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        -5, 0, 0, 0, 0, 0, 0, -5, -5,
        // State 23
        0, -23, 0, -23, -23, -23, 0, 0, 0,
        // State 24
        0, -25, -25, -25, -25, -25, -25, 0, 0,
        // State 25
        0, -21, -21, -21, -21, -21, -21, 0, 0,
    ];
    fn __action(state: i8, integer: usize) -> i8 {
        __ACTION[(state as usize) * 9 + integer]
    }
    const __EOF_ACTION: &[i8] = &[
        // State 0
        -16,
        // State 1
        -18,
        // State 2
        -6,
        // State 3
        -12,
        // State 4
        0,
        // State 5
        0,
        // State 6
        0,
        // State 7
        -15,
        // State 8
        -28,
        // State 9
        -24,
        // State 10
        -11,
        // State 11
        -20,
        // State 12
        -26,
        // State 13
        -19,
        // State 14
        -22,
        // State 15
        -17,
        // State 16
        -4,
        // State 17
        0,
        // State 18
        0,
        // State 19
        0,
        // State 20
        0,
        // State 21
        0,
        // State 22
        -5,
        // State 23
        -23,
        // State 24
        -25,
        // State 25
        -21,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            2 => 1,
            3 => match state {
                1 => 15,
                4 => 21,
                _ => 7,
            },
            5 => 5,
            6 => 8,
            7 => match state {
                5 => 23,
                _ => 9,
            },
            8 => 6,
            9 => 10,
            10 => 11,
            11 => match state {
                6 => 24,
                _ => 12,
            },
            12 => 2,
            13 => 3,
            _ => 0,
        }
    }
    fn __expected_tokens(__state: i8) -> alloc::vec::Vec<alloc::string::String> {
        const __TERMINAL: &[&str] = &[
            r###""(""###,
            r###"")""###,
            r###""*""###,
            r###""+""###,
            r###"",""###,
            r###""-""###,
            r###""/""###,
            r###"r#"[0-9]+"#"###,
        ];
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            let next_state = __action(__state, index);
            if next_state == 0 {
                None
            } else {
                Some(alloc::string::ToString::to_string(terminal))
            }
        }).collect()
    }
    pub(crate) struct __StateMachine<'input, 'err>
    where 'input: 'err, 'static: 'err
    {
        scale: i32,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
        input: &'input str,
        __phantom: core::marker::PhantomData<(&'input (), &'err ())>,
    }
    impl<'input, 'err> __state_machine::ParserDefinition for __StateMachine<'input, 'err>
    where 'input: 'err, 'static: 'err
    {
        type Location = usize;
        type Error = &'static str;
        type Token = Token<'input>;
        type TokenIndex = usize;
        type Symbol = __Symbol<'input>;
        type Success = Vec<Box<Expr>>;
        type StateIndex = i8;
        type Action = i8;
        type ReduceIndex = i8;
        type NonterminalIndex = usize;

        #[inline]
        fn start_location(&self) -> Self::Location {
              Default::default()
        }

        #[inline]
        fn start_state(&self) -> Self::StateIndex {
              0
        }

        #[inline]
        fn token_to_index(&self, token: &Self::Token) -> Option<usize> {
            __token_to_integer(token, core::marker::PhantomData::<(&(), &())>)
        }

        #[inline]
        fn action(&self, state: i8, integer: usize) -> i8 {
            __action(state, integer)
        }

        #[inline]
        fn error_action(&self, state: i8) -> i8 {
            __action(state, 9 - 1)
        }

        #[inline]
        fn eof_action(&self, state: i8) -> i8 {
            __EOF_ACTION[state as usize]
        }

        #[inline]
        fn goto(&self, state: i8, nt: usize) -> i8 {
            __goto(state, nt)
        }

        fn token_to_symbol(&self, token_index: usize, token: Self::Token) -> Self::Symbol {
            __token_to_symbol(token_index, token, core::marker::PhantomData::<(&(), &())>)
        }

        fn expected_tokens(&self, state: i8) -> alloc::vec::Vec<alloc::string::String> {
            __expected_tokens(state)
        }

        #[inline]
        fn uses_error_recovery(&self) -> bool {
            true
        }

        #[inline]
        fn error_recovery_symbol(
            &self,
            recovery: __state_machine::ErrorRecovery<Self>,
        ) -> Self::Symbol {
            __Symbol::Variant1(recovery)
        }

        fn reduce(
            &mut self,
            action: i8,
            start_location: Option<&Self::Location>,
            states: &mut alloc::vec::Vec<i8>,
            symbols: &mut alloc::vec::Vec<__state_machine::SymbolTriple<Self>>,
        ) -> Option<__state_machine::ParseResult<Self>> {
            __reduce(
                self.scale,
                self.errors,
                self.input,
                action,
                start_location,
                states,
                symbols,
                core::marker::PhantomData::<(&(), &())>,
            )
        }

        fn simulate_reduce(&self, action: i8) -> __state_machine::SimulatedReduce<Self> {
            __simulate_reduce(action, core::marker::PhantomData::<(&(), &())>)
        }
    }
    fn __token_to_integer<
        'input,
        'err,
    >(
        __token: &Token<'input>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> Option<usize>
    {
        match *__token {
            Token(1, _) if true => Some(0),
            Token(2, _) if true => Some(1),
            Token(3, _) if true => Some(2),
            Token(4, _) if true => Some(3),
            Token(5, _) if true => Some(4),
            Token(6, _) if true => Some(5),
            Token(7, _) if true => Some(6),
            Token(0, _) if true => Some(7),
            _ => None,
        }
    }
    fn __token_to_symbol<
        'input,
        'err,
    >(
        __token_index: usize,
        __token: Token<'input>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> __Symbol<'input>
    {
        match __token_index {
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 => match __token {
                Token(1, __tok0) | Token(2, __tok0) | Token(3, __tok0) | Token(4, __tok0) | Token(5, __tok0) | Token(6, __tok0) | Token(7, __tok0) | Token(0, __tok0) if true => __Symbol::Variant0(__tok0),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    fn __simulate_reduce<
        'input,
        'err,
    >(
        __reduce_index: i8,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> __state_machine::SimulatedReduce<__StateMachine<'input, 'err>>
    where
        'input: 'err,
        'static: 'err,
    {
        match __reduce_index {
            0 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 0,
                }
            }
            1 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 1,
                }
            }
            2 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 1,
                }
            }
            3 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 2,
                }
            }
            4 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 2,
                }
            }
            5 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 3,
                }
            }
            6 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 4,
                }
            }
            7 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 4,
                }
            }
            8 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 5,
                }
            }
            9 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 5,
                }
            }
            10 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 6,
                }
            }
            11 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 7,
                }
            }
            12 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 8,
                }
            }
            13 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 8,
                }
            }
            14 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 9,
                }
            }
            15 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 9,
                }
            }
            16 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 9,
                }
            }
            17 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 9,
                }
            }
            18 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 10,
                }
            }
            19 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 11,
                }
            }
            20 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 11,
                }
            }
            21 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 11,
                }
            }
            22 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 12,
                }
            }
            23 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 12,
                }
            }
            24 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 13,
                }
            }
            25 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 13,
                }
            }
            26 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 14,
                }
            }
            27 => __state_machine::SimulatedReduce::Accept,
            _ => panic!("invalid reduction index {}", __reduce_index)
        }
    }
    pub struct ExprsParser {
        builder: __lalrpop_util::lexer::MatcherBuilder,
        _priv: (),
    }

    impl ExprsParser {
        pub fn new() -> ExprsParser {
            let __builder = super::__intern_token::new_builder();
            ExprsParser {
                builder: __builder,
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            'input,
            'err,
        >(
            &self,
            scale: i32,
            errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
            input: &'input str,
        ) -> Result<Vec<Box<Expr>>, __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>
        {
            let mut __tokens = self.builder.matcher(input);
            __state_machine::Parser::drive(
                __StateMachine {
                    scale,
                    errors,
                    input,
                    __phantom: core::marker::PhantomData::<(&(), &())>,
                },
                __tokens,
            )
        }
    }
    fn __accepts<
        'input,
        'err,
    >(
        scale: i32,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
        input: &'input str,
        __error_state: i8,
        __states: & [i8],
        __opt_integer: Option<usize>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> bool
    {
        let mut __states = __states.to_vec();
        __states.push(__error_state);
        loop {
            let mut __states_len = __states.len();
            let __top = __states[__states_len - 1];
            let __action = match __opt_integer {
                None => __EOF_ACTION[__top as usize],
                Some(__integer) => __action(__top, __integer),
            };
            if __action == 0 { return false; }
            if __action > 0 { return true; }
            let (__to_pop, __nt) = match __simulate_reduce(-(__action + 1), core::marker::PhantomData::<(&(), &())>) {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop, nonterminal_produced
                } => (states_to_pop, nonterminal_produced),
                __state_machine::SimulatedReduce::Accept => return true,
            };
            __states_len -= __to_pop;
            __states.truncate(__states_len);
            let __top = __states[__states_len - 1];
            let __next_state = __goto(__top, __nt);
            __states.push(__next_state);
        }
    }
    pub(crate) fn __reduce<
        'input,
        'err,
    >(
        scale: i32,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut alloc::vec::Vec<i8>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> Option<Result<Vec<Box<Expr>>,__lalrpop_util::ParseError<usize, Token<'input>, &'static str>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                __reduce0(scale, errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            1 => {
                __reduce1(scale, errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            2 => {
                __reduce2(scale, errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            3 => {
                __reduce3(scale, errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            4 => {
                __reduce4(scale, errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            5 => {
                __reduce5(scale, errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            6 => {
                __reduce6(scale, errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            7 => {
                __reduce7(scale, errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            8 => {
                __reduce8(scale, errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            9 => {
                __reduce9(scale, errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            10 => {
                __reduce10(scale, errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            11 => {
                __reduce11(scale, errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            12 => {
                __reduce12(scale, errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            13 => {
                __reduce13(scale, errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            14 => {
                __reduce14(scale, errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            15 => {
                __reduce15(scale, errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            16 => {
                __reduce16(scale, errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            17 => {
                __reduce17(scale, errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            18 => {
                // Num = r#"[0-9]+"# => ActionFn(12);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action12::<>(scale, errors, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant7(__nt), __end));
                (1, 10)
            }
            19 => {
                __reduce19(scale, errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            20 => {
                __reduce20(scale, errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            21 => {
                __reduce21(scale, errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            22 => {
                __reduce22(scale, errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            23 => {
                __reduce23(scale, errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            24 => {
                __reduce24(scale, errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            25 => {
                __reduce25(scale, errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            26 => {
                __reduce26(scale, errors, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            27 => {
                // __Exprs = Exprs => ActionFn(0);
                let __sym0 = __pop_Variant6(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(scale, errors, input, __sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        let __state = *__states.last().unwrap();
        let __next_state = __goto(__state, __nonterminal);
        __states.push(__next_state);
        None
    }
    #[inline(never)]
    fn __symbol_type_mismatch() -> ! {
        panic!("symbol type mismatch")
    }
    fn __pop_Variant2<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expr>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant5<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Opcode, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant5(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant6<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Box<Expr>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant6(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant1<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, __lalrpop_util::ErrorRecovery<usize, Token<'input>, &'static str>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant3<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, alloc::vec::Vec<Box<Expr>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant4<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, core::option::Option<Box<Expr>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant7<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, i32, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant7(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant0<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant0(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    pub(crate) fn __reduce0<
        'input,
        'err,
    >(
        scale: i32,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<Expr> ",") = Expr, "," => ActionFn(22);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action22::<>(scale, errors, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (2, 0)
    }
    pub(crate) fn __reduce1<
        'input,
        'err,
    >(
        scale: i32,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<Expr> ",")* =  => ActionFn(20);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action20::<>(scale, errors, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (0, 1)
    }
    pub(crate) fn __reduce2<
        'input,
        'err,
    >(
        scale: i32,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<Expr> ",")* = (<Expr> ",")+ => ActionFn(21);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action21::<>(scale, errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 1)
    }
    pub(crate) fn __reduce3<
        'input,
        'err,
    >(
        scale: i32,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<Expr> ",")+ = Expr, "," => ActionFn(25);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action25::<>(scale, errors, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 2)
    }
    pub(crate) fn __reduce4<
        'input,
        'err,
    >(
        scale: i32,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // (<Expr> ",")+ = (<Expr> ",")+, Expr, "," => ActionFn(26);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action26::<>(scale, errors, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 2)
    }
    pub(crate) fn __reduce5<
        'input,
        'err,
    >(
        scale: i32,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Expr = Tier<ExprOp, Factor> => ActionFn(3);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action3::<>(scale, errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 3)
    }
    pub(crate) fn __reduce6<
        'input,
        'err,
    >(
        scale: i32,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Expr? = Expr => ActionFn(18);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action18::<>(scale, errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 4)
    }
    pub(crate) fn __reduce7<
        'input,
        'err,
    >(
        scale: i32,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Expr? =  => ActionFn(19);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action19::<>(scale, errors, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (0, 4)
    }
    pub(crate) fn __reduce8<
        'input,
        'err,
    >(
        scale: i32,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ExprOp = "+" => ActionFn(4);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action4::<>(scale, errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    pub(crate) fn __reduce9<
        'input,
        'err,
    >(
        scale: i32,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // ExprOp = "-" => ActionFn(5);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action5::<>(scale, errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    pub(crate) fn __reduce10<
        'input,
        'err,
    >(
        scale: i32,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Exprs = List<Expr> => ActionFn(2);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action2::<>(scale, errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce11<
        'input,
        'err,
    >(
        scale: i32,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Factor = Tier<FactorOp, Term> => ActionFn(6);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action6::<>(scale, errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 7)
    }
    pub(crate) fn __reduce12<
        'input,
        'err,
    >(
        scale: i32,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // FactorOp = "*" => ActionFn(7);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action7::<>(scale, errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 8)
    }
    pub(crate) fn __reduce13<
        'input,
        'err,
    >(
        scale: i32,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // FactorOp = "/" => ActionFn(8);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action8::<>(scale, errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 8)
    }
    pub(crate) fn __reduce14<
        'input,
        'err,
    >(
        scale: i32,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // List<Expr> = Expr => ActionFn(29);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action29::<>(scale, errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce15<
        'input,
        'err,
    >(
        scale: i32,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // List<Expr> =  => ActionFn(30);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action30::<>(scale, errors, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (0, 9)
    }
    pub(crate) fn __reduce16<
        'input,
        'err,
    >(
        scale: i32,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // List<Expr> = (<Expr> ",")+, Expr => ActionFn(31);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action31::<>(scale, errors, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (2, 9)
    }
    pub(crate) fn __reduce17<
        'input,
        'err,
    >(
        scale: i32,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // List<Expr> = (<Expr> ",")+ => ActionFn(32);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action32::<>(scale, errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce19<
        'input,
        'err,
    >(
        scale: i32,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Term = Num => ActionFn(9);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action9::<>(scale, errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce20<
        'input,
        'err,
    >(
        scale: i32,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Term = "(", Expr, ")" => ActionFn(10);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action10::<>(scale, errors, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (3, 11)
    }
    pub(crate) fn __reduce21<
        'input,
        'err,
    >(
        scale: i32,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Term = error => ActionFn(11);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action11::<>(scale, errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce22<
        'input,
        'err,
    >(
        scale: i32,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Tier<ExprOp, Factor> = Tier<ExprOp, Factor>, ExprOp, Factor => ActionFn(15);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant2(__symbols);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action15::<>(scale, errors, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (3, 12)
    }
    pub(crate) fn __reduce23<
        'input,
        'err,
    >(
        scale: i32,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Tier<ExprOp, Factor> = Factor => ActionFn(16);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action16::<>(scale, errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 12)
    }
    pub(crate) fn __reduce24<
        'input,
        'err,
    >(
        scale: i32,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Tier<FactorOp, Term> = Tier<FactorOp, Term>, FactorOp, Term => ActionFn(13);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant2(__symbols);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action13::<>(scale, errors, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (3, 13)
    }
    pub(crate) fn __reduce25<
        'input,
        'err,
    >(
        scale: i32,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // Tier<FactorOp, Term> = Term => ActionFn(14);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action14::<>(scale, errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 13)
    }
    pub(crate) fn __reduce26<
        'input,
        'err,
    >(
        scale: i32,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'err ())>,
    ) -> (usize, usize)
    {
        // __Expr = Expr => ActionFn(1);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action1::<>(scale, errors, input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 14)
    }
}
pub use self::__parse__Exprs::ExprsParser;
#[cfg_attr(rustfmt, rustfmt_skip)]
mod __intern_token {
    #![allow(unused_imports)]
    use std::str::FromStr;
    use crate::ast::{Expr, Opcode};
    use lalrpop_util::ParseError;
    use lalrpop_util::ErrorRecovery;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    extern crate core;
    extern crate alloc;
    pub fn new_builder() -> __lalrpop_util::lexer::MatcherBuilder {
        let __strs: &[(&str, bool)] = &[
            ("^([0-9]+)", false),
            ("^(\\()", false),
            ("^(\\))", false),
            ("^(\\*)", false),
            ("^(\\+)", false),
            ("^(,)", false),
            ("^(\\-)", false),
            ("^(/)", false),
            (r"^(\s*)", true),
        ];
        __lalrpop_util::lexer::MatcherBuilder::new(__strs.iter().copied()).unwrap()
    }
}
pub(crate) use self::__lalrpop_util::lexer::Token;

#[allow(unused_variables)]
fn __action0<
    'input,
    'err,
>(
    scale: i32,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
    input: &'input str,
    (_, __0, _): (usize, Vec<Box<Expr>>, usize),
) -> Vec<Box<Expr>>
{
    __0
}

#[allow(unused_variables)]
fn __action1<
    'input,
    'err,
>(
    scale: i32,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
    input: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    __0
}

#[allow(unused_variables)]
fn __action2<
    'input,
    'err,
>(
    scale: i32,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
    input: &'input str,
    (_, __0, _): (usize, Vec<Box<Expr>>, usize),
) -> Vec<Box<Expr>>
{
    __0
}

#[allow(unused_variables)]
fn __action3<
    'input,
    'err,
>(
    scale: i32,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
    input: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    __0
}

#[allow(unused_variables)]
fn __action4<
    'input,
    'err,
>(
    scale: i32,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Opcode
{
    Opcode::Add
}

#[allow(unused_variables)]
fn __action5<
    'input,
    'err,
>(
    scale: i32,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Opcode
{
    Opcode::Sub
}

#[allow(unused_variables)]
fn __action6<
    'input,
    'err,
>(
    scale: i32,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
    input: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    __0
}

#[allow(unused_variables)]
fn __action7<
    'input,
    'err,
>(
    scale: i32,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Opcode
{
    Opcode::Mul
}

#[allow(unused_variables)]
fn __action8<
    'input,
    'err,
>(
    scale: i32,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Opcode
{
    Opcode::Div
}

#[allow(unused_variables)]
fn __action9<
    'input,
    'err,
>(
    scale: i32,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
    input: &'input str,
    (_, n, _): (usize, i32, usize),
) -> Box<Expr>
{
    Box::new(Expr::Number(n))
}

#[allow(unused_variables)]
fn __action10<
    'input,
    'err,
>(
    scale: i32,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, Box<Expr>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Box<Expr>
{
    __0
}

#[allow(unused_variables)]
fn __action11<
    'input,
    'err,
>(
    scale: i32,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
    input: &'input str,
    (_, error, _): (usize, __lalrpop_util::ErrorRecovery<usize, Token<'input>, &'static str>, usize),
) -> Box<Expr>
{
    {
    errors.push(error);
    Box::new(Expr::Error)
  }
}

#[allow(unused_variables)]
fn __action12<
    'input,
    'err,
>(
    scale: i32,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
    input: &'input str,
    (_, s, _): (usize, &'input str, usize),
) -> Result<i32,__lalrpop_util::ParseError<usize,Token<'input>,&'static str>>
{
    {
  i32::from_str(s)
  .map(|x| x * scale)
  .map_err(|_| ParseError::User{ error: "number is too big" })
}
}

#[allow(unused_variables)]
fn __action13<
    'input,
    'err,
>(
    scale: i32,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
    input: &'input str,
    (_, left, _): (usize, Box<Expr>, usize),
    (_, op, _): (usize, Opcode, usize),
    (_, right, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    Box::new(Expr::Op(left, op, right))
}

#[allow(unused_variables)]
fn __action14<
    'input,
    'err,
>(
    scale: i32,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
    input: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    __0
}

#[allow(unused_variables)]
fn __action15<
    'input,
    'err,
>(
    scale: i32,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
    input: &'input str,
    (_, left, _): (usize, Box<Expr>, usize),
    (_, op, _): (usize, Opcode, usize),
    (_, right, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    Box::new(Expr::Op(left, op, right))
}

#[allow(unused_variables)]
fn __action16<
    'input,
    'err,
>(
    scale: i32,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
    input: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    __0
}

#[allow(unused_variables)]
fn __action17<
    'input,
    'err,
>(
    scale: i32,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
    input: &'input str,
    (_, mut xs, _): (usize, alloc::vec::Vec<Box<Expr>>, usize),
    (_, value, _): (usize, core::option::Option<Box<Expr>>, usize),
) -> Vec<Box<Expr>>
{
    match value {
    None => xs,
    Some(value) => {
      xs.push(value);
      xs
    }
  }
}

#[allow(unused_variables)]
fn __action18<
    'input,
    'err,
>(
    scale: i32,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
    input: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> core::option::Option<Box<Expr>>
{
    Some(__0)
}

#[allow(unused_variables)]
fn __action19<
    'input,
    'err,
>(
    scale: i32,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> core::option::Option<Box<Expr>>
{
    None
}

#[allow(unused_variables)]
fn __action20<
    'input,
    'err,
>(
    scale: i32,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> alloc::vec::Vec<Box<Expr>>
{
    alloc::vec![]
}

#[allow(unused_variables)]
fn __action21<
    'input,
    'err,
>(
    scale: i32,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<Box<Expr>>, usize),
) -> alloc::vec::Vec<Box<Expr>>
{
    v
}

#[allow(unused_variables)]
fn __action22<
    'input,
    'err,
>(
    scale: i32,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
    input: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Box<Expr>
{
    __0
}

#[allow(unused_variables)]
fn __action23<
    'input,
    'err,
>(
    scale: i32,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
    input: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> alloc::vec::Vec<Box<Expr>>
{
    alloc::vec![__0]
}

#[allow(unused_variables)]
fn __action24<
    'input,
    'err,
>(
    scale: i32,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<Box<Expr>>, usize),
    (_, e, _): (usize, Box<Expr>, usize),
) -> alloc::vec::Vec<Box<Expr>>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action25<
    'input,
    'err,
>(
    scale: i32,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
    input: &'input str,
    __0: (usize, Box<Expr>, usize),
    __1: (usize, &'input str, usize),
) -> alloc::vec::Vec<Box<Expr>>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action22(
        scale,
        errors,
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action23(
        scale,
        errors,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action26<
    'input,
    'err,
>(
    scale: i32,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
    input: &'input str,
    __0: (usize, alloc::vec::Vec<Box<Expr>>, usize),
    __1: (usize, Box<Expr>, usize),
    __2: (usize, &'input str, usize),
) -> alloc::vec::Vec<Box<Expr>>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action22(
        scale,
        errors,
        input,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action24(
        scale,
        errors,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action27<
    'input,
    'err,
>(
    scale: i32,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
    input: &'input str,
    __0: (usize, core::option::Option<Box<Expr>>, usize),
) -> Vec<Box<Expr>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action20(
        scale,
        errors,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action17(
        scale,
        errors,
        input,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action28<
    'input,
    'err,
>(
    scale: i32,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
    input: &'input str,
    __0: (usize, alloc::vec::Vec<Box<Expr>>, usize),
    __1: (usize, core::option::Option<Box<Expr>>, usize),
) -> Vec<Box<Expr>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action21(
        scale,
        errors,
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action17(
        scale,
        errors,
        input,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action29<
    'input,
    'err,
>(
    scale: i32,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
    input: &'input str,
    __0: (usize, Box<Expr>, usize),
) -> Vec<Box<Expr>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action18(
        scale,
        errors,
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action27(
        scale,
        errors,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action30<
    'input,
    'err,
>(
    scale: i32,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<Box<Expr>>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action19(
        scale,
        errors,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action27(
        scale,
        errors,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action31<
    'input,
    'err,
>(
    scale: i32,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
    input: &'input str,
    __0: (usize, alloc::vec::Vec<Box<Expr>>, usize),
    __1: (usize, Box<Expr>, usize),
) -> Vec<Box<Expr>>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action18(
        scale,
        errors,
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action28(
        scale,
        errors,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action32<
    'input,
    'err,
>(
    scale: i32,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, &'static str>>,
    input: &'input str,
    __0: (usize, alloc::vec::Vec<Box<Expr>>, usize),
) -> Vec<Box<Expr>>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action19(
        scale,
        errors,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action28(
        scale,
        errors,
        input,
        __0,
        __temp0,
    )
}

pub trait __ToTriple<'input, 'err, > {
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>;
}

impl<'input, 'err, > __ToTriple<'input, 'err, > for (usize, Token<'input>, usize) {
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, &'static str>> {
        Ok(value)
    }
}
impl<'input, 'err, > __ToTriple<'input, 'err, > for Result<(usize, Token<'input>, usize), &'static str> {
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, &'static str>> {
        match value {
            Ok(v) => Ok(v),
            Err(error) => Err(__lalrpop_util::ParseError::User { error }),
        }
    }
}
