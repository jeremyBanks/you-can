use {
    crossterm::style::Stylize,
    proc_macro::{TokenStream, TokenTree},
    quote::ToTokens,
    std::{borrow::Cow, panic::Location},
};

#[track_caller]
pub fn print(label: impl Into<Cow<'static, str>>, tokens: &impl Print) {
    println!();
    println!(
        "{} {}",
        label.into().to_owned().underlined().bold(),
        format!("logged at {}", Location::caller()).dim()
    );
    tokens.print();
    println!();
}

pub trait Print {
    fn print(&self);
}

impl<T> Print for T where T: ToTokens {
    fn print(&self) {
        let tokens = &proc_macro::TokenStream::from(self.to_token_stream());

        let printed: eyre::Result<()> = (|| {
            let pseudo_file: syn::File = syn::parse(tokens.clone())?;
            let pretty = prettyplease::unparse(&pseudo_file);
            let mut printer = bat::PrettyPrinter::new();
            printer.input_from_bytes(pretty.as_ref()).language("rust");
            printer.print()?;
            Ok(())
        })();

        if printed.is_err() {
            let pretty = tokens.to_string();
            let mut printer = bat::PrettyPrinter::new();
            printer.input_from_bytes(pretty.as_ref()).language("rust");
            printer.print().unwrap();
        }
    }
}
