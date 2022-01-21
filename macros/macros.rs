use {
    proc_macro::TokenStream,
    quote::ToTokens,
    syn::{fold::Fold, parse_macro_input, visit::Visit},
};

mod print;
use {print::print, syn::parse_quote};

#[proc_macro_attribute]
pub fn turn_off_the_borrow_checker(_attribute: TokenStream, input: TokenStream) -> TokenStream {
    let input: syn::File = parse_macro_input!(input);

    let output = BorrowCheckerSuppressor.fold_file(input);

    print(&output);

    output.into_token_stream().into()
}

/// Replaces all references (&T or &mut T) with unbounded references by wrapping
/// them in calls to unbounded::reference().
#[derive(Debug, Default)]
struct BorrowCheckerSuppressor;

impl Fold for BorrowCheckerSuppressor {
    fn fold_expr(&mut self, node: syn::Expr) -> syn::Expr {
        match node {
            syn::Expr::Reference(node) => {
                let node = syn::fold::fold_expr_reference(self, node);
                syn::Expr::Paren(parse_quote! {
                  (::unbounded::reference(#node))
                })
            },
            _ => syn::fold::fold_expr(self, node),
        }
    }

    fn fold_expr_if(&mut self, mut node: syn::ExprIf) -> syn::ExprIf {
        if matches!(*node.cond, syn::Expr::Let(_)) {
            let mut ref_collector = RefCollector::default();
            ref_collector.visit_expr(&node.cond);
            let refs = ref_collector.refs;
            let then_stmts = node.then_branch.stmts.clone();
            node.then_branch = parse_quote! {
                {
                    #(let #refs = ::unbounded::reference(#refs);)*
                    #(#then_stmts)*
                }
            };
        }
        syn::fold::fold_expr_if(self, node)
    }

    fn fold_arm(&mut self, mut node: syn::Arm) -> syn::Arm {
        let mut ref_collector = RefCollector::default();
        ref_collector.visit_pat(&node.pat);
        let refs = ref_collector.refs;
        let body = node.body.clone();
        node.body = parse_quote! {
            {
                #(let #refs = ::unbounded::reference(#refs);)*
                #body
            }
        };
        syn::fold::fold_arm(self, node)
    }
}

#[derive(Debug, Default)]
struct RefCollector {
    refs: Vec<syn::Ident>,
}

impl<'ast> Visit<'ast> for RefCollector {
    fn visit_pat_ident(&mut self, node: &'ast syn::PatIdent) {
        if node.by_ref.is_some() {
            self.refs.push(node.ident.clone());
        }
    }
}
