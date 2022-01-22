#![feature(proc_macro_diagnostic, proc_macro_span)]

use {
    proc_macro::{Span, TokenStream},
    quote::ToTokens,
    syn::{fold::Fold, parse_macro_input, parse_quote, spanned::Spanned, visit::Visit},
};

#[proc_macro_attribute]
pub fn turn_off_the_borrow_checker(_attribute: TokenStream, input: TokenStream) -> TokenStream {
    let input: syn::File = parse_macro_input!(input);

    let mut suppressor = BorrowCheckerSuppressor {
        suppressed_references: vec![],
    };
    let output = suppressor.fold_file(input);

    proc_macro::Diagnostic::spanned(
        vec![Span::call_site().parent().unwrap()],
        proc_macro::Level::Warning,
        "This macro suppresses the borrow checker in an unsafe, unsound, and unstable way that \
         produces undefined behaviour. This is not suitable for any purpose beyond educational \
         experimentation.\n",
    )
    .emit();

    if suppressor.suppressed_references.len() > 1 {
        proc_macro::Diagnostic::spanned(
            suppressor.suppressed_references,
            proc_macro::Level::Note,
            "The borrow checker is suppressed for the these references.",
        )
        .emit();
    }

    output.into_token_stream().into()
}
/// Replaces all references (&T or &mut T) with unbounded references by wrapping
/// them in calls to unbounded::reference().
#[derive(Debug, Default)]
struct BorrowCheckerSuppressor {
    suppressed_references: Vec<Span>,
}

impl Fold for BorrowCheckerSuppressor {
    fn fold_expr(&mut self, node: syn::Expr) -> syn::Expr {
        match node {
            syn::Expr::Reference(node) => {
                let node = syn::fold::fold_expr_reference(self, node);
                self.suppressed_references.push(node.span().unwrap());
                syn::Expr::Block(parse_quote! {
                    {
                        let r#ref = #node;
                        unsafe { ::unbounded::reference(r#ref) }
                    }
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
            self.suppressed_references.extend(ref_collector.spans);
            let then_stmts = node.then_branch.stmts.clone();
            node.then_branch = parse_quote! {
                {
                    #(let #refs = unsafe { ::unbounded::reference(#refs) };)*
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
        self.suppressed_references.extend(ref_collector.spans);
        let body = node.body.clone();
        node.body = parse_quote! {
            {
                #(let #refs = unsafe { ::unbounded::reference(#refs) };)*
                #body
            }
        };
        syn::fold::fold_arm(self, node)
    }
}

#[derive(Debug, Default)]
struct RefCollector {
    refs: Vec<syn::Ident>,
    spans: Vec<Span>,
}

impl<'ast> Visit<'ast> for RefCollector {
    fn visit_pat_ident(&mut self, node: &'ast syn::PatIdent) {
        if node.by_ref.is_some() {
            self.refs.push(node.ident.clone());
            self.spans.push(node.span().unwrap());
        }
    }
}
