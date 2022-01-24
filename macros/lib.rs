#![cfg_attr(rustc_is_unstable, feature(proc_macro_diagnostic, proc_macro_span))]

use {
    proc_macro::{Span, TokenStream},
    quote::{quote, quote_spanned, ToTokens},
    syn::{fold::Fold, parse_quote_spanned, spanned::Spanned, visit::Visit},
};

/// Runs one of two branches depending on whether we're running on a stable
/// version of the compiler (stable, beta), or an unstable version (nightly,
/// dev, or anywhere that `RUSTC_BOOTSTRAP=1`).
macro_rules! if_unstable {
    { then { $($then:tt)* } else { $($else:tt)* } } => {
        #[allow(unreachable_code)]
        if cfg!(rustc_is_unstable) {
            #[cfg(not(rustc_is_unstable))] {
                unreachable!();
            }
            #[cfg(rustc_is_unstable)] {
                $($then)*
            }
        } else {
            #[cfg(rustc_is_unstable)] {
                unreachable!();
            }
            $($else)*
        }
    }
}

#[proc_macro_attribute]
pub fn turn_off_the_borrow_checker(_attribute: TokenStream, input: TokenStream) -> TokenStream {
    let mut suppressor = BorrowCheckerSuppressor {
        suppressed_references: vec![],
    };

    let output = if let Ok(as_file) = syn::parse(input.clone()) {
        suppressor.fold_file(as_file).to_token_stream()
    } else if let Ok(as_expr) = syn::parse(input.clone()) {
        suppressor.fold_expr(as_expr).to_token_stream()
    } else if let Ok(as_stmt) = syn::parse(input) {
        suppressor.fold_stmt(as_stmt).to_token_stream()
    } else {
        return quote! { compile_error!("unsupported use of #[turn_off_the_borrow_checker]") }
            .into();
    };

    if_unstable! {
        then {
            proc_macro::Diagnostic::spanned(
                vec![Span::call_site().parent().unwrap_or_else(Span::call_site)],
                proc_macro::Level::Warning,
                "this suppresses the borrow checker in an unsafe, unsound, and unstable way \
                that produces undefined behaviour. this is not suitable for any purpose beyond \
                educational experimentation.",
            ).emit();

            if suppressor.suppressed_references.len() > 1 {
                proc_macro::Diagnostic::spanned(
                    suppressor.suppressed_references,
                    proc_macro::Level::Warning,
                    "the borrow checker is suppressed for these references.",
                ).emit();
            }

            output.into_token_stream().into()
        } else {
            static DANGER: std::sync::Once = std::sync::Once::new();
            DANGER.call_once(|| {
                eprintln!();
                eprintln!(" DANGER   This project is using the the #[you_can::turn_off_the_borrow_checker]");
                eprintln!(" DANGER   macro, which is inherently unsafe, unsound, and unstable. This is not");
                eprintln!(" DANGER   suitable for any purpose beyond educational experimentation.");
                eprintln!();
            });

            quote_spanned! {
                Span::call_site().into() =>
                #[warn(unsafe_code)]
                #output
            }.into_token_stream().into()
        }
    }
}

/// Replaces all references (&T or &mut T) with unbounded references by wrapping
/// them in calls to you_can::borrow_unchecked().
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
                syn::Expr::Block(parse_quote_spanned! { node.span() =>
                    {
                        let r#ref = #node;
                        unsafe { ::you_can::borrow_unchecked(r#ref) }
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
            node.then_branch = parse_quote_spanned! { node.span() =>
                {
                    #(let #refs = unsafe { ::you_can::borrow_unchecked(#refs) };)*
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
        node.body = parse_quote_spanned! { node.span() =>
            {
                #(let #refs = unsafe { ::you_can::borrow_unchecked(#refs) };)*
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
