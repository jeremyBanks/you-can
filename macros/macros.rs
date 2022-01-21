use {
    proc_macro::{TokenStream},
    quote::ToTokens,
    syn::{parse_macro_input, visit_mut::VisitMut},
};

mod print;
use print::print;
use syn::{parse_quote, parse_quote_spanned};

#[proc_macro_attribute]
pub fn turn_off_the_borrow_checker(_attribute: TokenStream, input: TokenStream) -> TokenStream {
    let mut tree: syn::File = parse_macro_input!(input);

    print("input", &tree);

    BorrowCheckerSuppressor.visit_file_mut(&mut tree);

    print("output", &tree);

    tree.into_token_stream().into()
}

struct BorrowCheckerSuppressor;

/// Replaces all references (&T or &mut T) with unbounded references by wrapping
/// them in calls to unbounded::reference().
impl VisitMut for BorrowCheckerSuppressor {
    fn visit_expr_reference_mut(&mut self, node: &mut syn::ExprReference) {
        print("ExprReference", &node);
        let s: syn::Expr = parse_quote!(#node);
        *node = s;

        // syn::visit_mut::visit_expr_reference_mut(self, node);
    }

    fn visit_pat_reference_mut(&mut self, node: &mut syn::PatReference) {
        print("PatReference", &node);
        syn::visit_mut::visit_pat_reference_mut(self, node);
    }

    fn visit_type_reference_mut(&mut self, node: &mut syn::TypeReference) {
        print("TypeReference", &node);
        syn::visit_mut::visit_type_reference_mut(self, node);
    }
}
