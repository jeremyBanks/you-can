use {
    crossterm::style::Stylize,
    proc_macro::{TokenStream, TokenTree},
    quote::ToTokens,
    std::{borrow::Cow, panic::Location},
    syn::parse_macro_input,
};

#[proc_macro_attribute]
pub fn turn_off_the_borrow_checker(_attribute: TokenStream, input: TokenStream) -> TokenStream {
    let input: syn::File = parse_macro_input!(input);

    print("input", &input);

    let output = input.clone().borrow_checker_suppressed();

    print("output", &output);

    output.into_token_stream().into()
}

trait BorrowCheckerSuppressed: Sized {
    type Output;
    fn borrow_checker_suppressed(self) -> Self::Output;
}

impl BorrowCheckerSuppressed for syn::Item {
    type Output = Self;

    fn borrow_checker_suppressed(mut self) -> Self::Output {
        match self {
            syn::Item::Fn(item_fn) => item_fn.borrow_checker_suppressed().into(),
            syn::Item::Impl(item_impl) => item_impl.borrow_checker_suppressed().into(),
            syn::Item::Mod(item_mod) => item_mod.borrow_checker_suppressed().into(),
            _ => self,
        }
    }
}

impl BorrowCheckerSuppressed for syn::ImplItem {
    type Output = Self;

    fn borrow_checker_suppressed(mut self) -> Self::Output {
        match self {
            syn::ImplItem::Method(impl_item_method) => impl_item_method.borrow_checker_suppressed().into(),
            _ => self,
        }
    }
}

impl BorrowCheckerSuppressed for syn::ImplItemMethod {
    type Output = Self;

    fn borrow_checker_suppressed(mut self) -> Self::Output {

    }
}

impl BorrowCheckerSuppressed for syn::ItemImpl {
    type Output = Self;

    fn borrow_checker_suppressed(mut self) -> Self::Output {
        for item in self.items.iter_mut() {
            *item = item.borrow_checker_suppressed();
        }
        self
    }
}

impl BorrowCheckerSuppressed for syn::ItemMod {
    type Output = Self;

    fn borrow_checker_suppressed(mut self) -> Self::Output {
        for item in self.content.unwrap_or_default().1.iter_mut() {
            *item = item.borrow_checker_suppressed();
        }
        self
    }
}

impl BorrowCheckerSuppressed for syn::File {
    type Output = Self;

    fn borrow_checker_suppressed(mut self) -> Self::Output {
        for item in self.items.iter_mut() {
            *item = item.borrow_checker_suppressed();
        }
        self
    }
}

impl BorrowCheckerSuppressed for syn::ItemFn {
    type Output = Self;

    fn borrow_checker_suppressed(mut self) -> Self::Output {
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
