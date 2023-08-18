# 매크로 작은 책 공부 

The Little Book of Rust Macros는 공부해야지 하면서 미뤄두고 있었다. 선언적 매크로 또는 
예시를 통한 매크로로 불리는 macro_rules!를 어느 정도 읽을 수 있는 수준은 되지만 직접 
제대로 매크로를 구현한 적은 없었다. 

제대로 프로젝트를 하려면 C++에서 템플릿 프로그래밍을 피할 수 없는 것처럼 러스트에서 
매크로 공부도 필수적이다. 과하게 사용하면 안 되겠지만 필요한 부분은 항상 있다. 



```rust
//! 드디어 macro 작은 책을 볼 때가 되었다.
//! a complete and thorough explanation of how the system works.
//!
//!

#[cfg(test)]
mod tests {

    #[test]
    fn read_syntax_extensions() {
        fn read_source_analysis() {
            // tokenization
            // AST
            // Token trees
            //   Grouping tokens
            // 매크로 작업에는 AST와 토큰 트리 모두 사용한다.
            //
        }

        fn read_macros_in_ast() {
            // 몇가지 syntax extension 형식이 있다.
            // $name ! $arg; e.g. println!("Hi!"), concat!("a", "b"), …
            // 여기서 $arg는 토큰 트리이다.
            //
        }

        fn read_macro_expansion() {
            // AST를 구성하고 의미 분석을 하기 전에 매크로 확장
            // 매크로의 위치에 맞게 확장
            //
        }

        read_source_analysis();
        read_macros_in_ast();
        read_macro_expansion();
    }

    #[test]
    fn read_macro_rules() {
        //
        macro_rules! gibberish {
            // pattern은 임의의 토큰 트리와 매칭된다.
            (4 fn ['spang "whammo"] @_@) => {
                3
            };
        }

        let v = gibberish!(4 fn ['spang "whammo"] @_@);
        assert_eq!(v, 3);

        // captures: 람다의 캡처와 같이
        //

        macro_rules! one_expression {
            ($e:expr) => {
                $e
            };
        }

        let v = one_expression!(3 + 5);
        assert_eq!(v, 3 + 5);

        // cargo-expand를 cargo install cargo-expand로 설치하면 볼 수 있다.
        // cargo expand --bin ractor --tests
        // 결과 값이 syntax highlight 되므로 보기 괜찮다.
        // 간단한 .rs 파일에 저장해서 살펴봐도 되고, 터미널에서 봐도 많이
        // 불편하지 않다.

        // repetitions:
    }

    #[test]
    fn read_repetitions() {
        macro_rules! vec_strs {
            (
                // Start a repetition:
                $(
                    // Each repeat must contain an expression...
                    $element:expr
                )
                // ...separated by commas...
                ,
                // ...zero or more times.
                *
            ) => {
                // Enclose the expansion in a block so that we can use
                // multiple statements.
                {
                    let mut v = Vec::new();

                    // Start a repetition:
                    $(
                        // Each repeat will contain the following statement, with
                        // $element replaced with the corresponding expression.
                        v.push(format!("{}", $element));
                    )*

                    v
                }
            };
        }

        let vs = vec_strs![3 + 5, 7, 10];
        assert_eq!(vs[0], "8".to_string());
    }

    #[test]
    fn read_captures_expansion() {
        // AST에 대해 동작한다. 한번 AST로 결정되면 다른 것으로 다시 시도되지 않는다.

        macro_rules! capture_then_match_tokens {
            ($e:expr) => {
                match_tokens!($e)
            };
        }

        macro_rules! match_tokens {
            ($a:tt + $b:tt) => {
                "got an addition"
            };
            (($i:ident)) => {
                "got an identifier"
            };
            ($($other:tt)*) => {
                "got something else"
            };
        }

        fn main() {
            println!(
                "{}\n{}\n{}\n",
                match_tokens!((caravan)),
                match_tokens!(3 + 6),
                match_tokens!(5)
            );
            println!(
                "{}\n{}\n{}",
                capture_then_match_tokens!((caravan)),
                capture_then_match_tokens!(3 + 6),
                capture_then_match_tokens!(5)
            );
        }

        main();

        // 이 부분이 중요한데 바로 이해하기가 쉬운 개념은 아니다.
        // 토큰 트리로 시작하지만, 매칭이 AST 타잎으로 이루어지면 고정되고, 백트랙은 없다.
    }

    #[test]
    fn read_hygiene() {
        // Each macro expansion is given a new, unique syntax context for its contents.
    }

    #[test]
    fn read_debug() {
        macro_rules! sing {
    () => {};
    ($tt:tt $($rest:tt)*) => {log_syntax!($tt); sing!($($rest)*);};
}

        sing! {
            ^ < @ < . @ *
            '\x08' '{' '"' _ # ' '
            - @ '$' && / _ %
            ! ( '\t' @ | = >
            ; '\x08' '\'' + '$' ? '\x7f'
            , # '"' ~ | ) '\x07'
        }
    }
}
```