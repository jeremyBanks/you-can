//! **Unsound and unsafe**! For edutainment purposes only.

#![feature(associated_type_defaults)]

use {
    crossterm::style::Stylize,
    proc_macro::{TokenStream, TokenTree},
    quote::ToTokens,
    std::{
        any::{Any, TypeId},
        borrow::Cow,
        panic::Location,
    },
    syn::parse_macro_input,
};

#[proc_macro_attribute]
pub fn turn_off_the_borrow_checker(_attribute: TokenStream, item: TokenStream) -> TokenStream {
    let item: syn::File = parse_macro_input!(item);

    assert_eq!(item.items.len(), 1, "expected exactly one item");
    let item_fn = match &item.items[0] {
        syn::Item::Fn(item_fn) => item_fn,
        _ => panic!("expected a function"),
    };

    let item = item_fn.with_borrow_checker_disabled();

    print("output", &item);

    item.into_token_stream().into()
}

trait WithBorrowCheckerDisabled: Sized {
    type Output = Self;
    fn with_borrow_checker_disabled(&self) -> Self::Output;
}

impl WithBorrowCheckerDisabled for syn::File {
    fn with_borrow_checker_disabled(&self) -> Self::Output {
        self.clone()
    }
}

impl WithBorrowCheckerDisabled for syn::ItemFn {
    fn with_borrow_checker_disabled(&self) -> Self::Output {
        self.clone()
    }
}

#[track_caller]
fn print(label: impl Into<Cow<'static, str>>, tokens: &impl Print) {
    println!();
    println!(
        "{} {}",
        label.into().to_owned().underlined().bold(),
        format!("logged at {}", Location::caller()).dim()
    );
    tokens.print();
    println!();
}

trait Print: Clone {
    fn print(&self);
}

impl Print for TokenTree {
    fn print(&self) {
        let stream = TokenStream::from(self.clone());
        stream.print()
    }
}

impl Print for syn::File {
    fn print(&self) {
        TokenStream::from(self.clone().into_token_stream()).print()
    }
}

impl Print for syn::Item {
    fn print(&self) {
        TokenStream::from(self.clone().into_token_stream()).print()
    }
}

impl Print for syn::ItemFn {
    fn print(&self) {
        TokenStream::from(self.clone().into_token_stream()).print()
    }
}

impl Print for TokenStream {
    fn print(&self) {
        let printed: eyre::Result<()> = (|| {
            let pseudo_file: syn::File = syn::parse(self.clone())?;
            let pretty = prettyplease::unparse(&pseudo_file);
            let mut printer = bat::PrettyPrinter::new();
            printer.input_from_bytes(pretty.as_ref()).language("rust");
            printer.print()?;
            Ok(())
        })();

        if printed.is_err() {
            let pretty = self.to_string();
            let mut printer = bat::PrettyPrinter::new();
            printer.input_from_bytes(pretty.as_ref()).language("rust");
            printer.print().unwrap();
        }
    }
}
