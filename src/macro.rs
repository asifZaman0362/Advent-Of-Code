extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Expr, LitInt, Token,
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

#[proc_macro]
pub fn solution(input: TokenStream) -> TokenStream {
    let SolutionArgs {
        year,
        day,
        answer_type,
        solution_body,
    } = parse_macro_input!(input as SolutionArgs);

    // Convert year and day to usizes for the date() function
    let year_val: usize = year.base10_parse().unwrap();
    let day_val: usize = day.base10_parse().unwrap();

    // Generate the struct and Solution implementation
    let struct_name = format_ident!("Solver");
    let expanded = quote! {
        pub struct #struct_name;

        impl Solution for #struct_name {
            type Answer = #answer_type;

            fn solve(input: Input) -> (Self::Answer, Self::Answer) {
                #solution_body
            }

            fn date() -> (usize, usize) {
                (#year_val, #day_val)
            }
        }
    };

    TokenStream::from(expanded)
}
