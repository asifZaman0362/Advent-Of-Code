extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Expr, LitInt, LitStr, Token,
};

struct SolutionArgs {
    year: LitInt,
    day: LitInt,
    answer_type: syn::Type,
    solution_body: Expr,
}

impl Parse for SolutionArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let year: LitInt = input.parse()?;
        input.parse::<Token![,]>()?;
        let day: LitInt = input.parse()?;
        input.parse::<Token![,]>()?;
        let answer_type: syn::Type = input.parse()?;
        input.parse::<Token![,]>()?;
        let solution_body: Expr = input.parse()?;

        Ok(SolutionArgs {
            year,
            day,
            answer_type,
            solution_body,
        })
    }
}

struct TestArgs {
    str: syn::Expr,
    part1: syn::ExprLit,
    part2: syn::ExprLit,
}

impl Parse for TestArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let str: Expr = input.parse()?;
        input.parse::<Token![,]>()?;
        let part1: syn::ExprLit = input.parse()?;
        input.parse::<Token![,]>()?;
        let part2: syn::ExprLit = input.parse()?;
        Ok(TestArgs { str, part1, part2 })
    }
}

#[proc_macro]
pub fn test_sol(input: TokenStream) -> TokenStream {
    let TestArgs { str, part1, part2 } = parse_macro_input!(input as TestArgs);
    TokenStream::from(quote! {
        asst(Solver::solve(#str), (#part1, #part2));
    })
}

#[proc_macro]
pub fn solution(input: TokenStream) -> TokenStream {
    let SolutionArgs {
        year,
        day,
        answer_type,
        solution_body,
    } = parse_macro_input!(input as SolutionArgs);

    // Convert year and day to usizes for the date() function
    let year_val: u16 = year.base10_parse().unwrap();
    let day_val: u8 = day.base10_parse().unwrap();

    // Generate the struct and Solution implementation
    let struct_name = format_ident!("Solver");
    let expanded = quote! {
        pub struct #struct_name;

        impl Solution for #struct_name {
            type Answer = #answer_type;

            fn solve(input: Input) -> (Self::Answer, Self::Answer) {
                #solution_body
            }

            fn date() -> (u16, u8) {
                (#year_val, #day_val)
            }
        }
    };

    TokenStream::from(expanded)
}
