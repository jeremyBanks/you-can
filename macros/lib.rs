use {
    crossterm::style::Stylize,
    proc_macro::TokenStream,
    quote::ToTokens,
    syn::{fold::Fold, parse_macro_input, parse_quote, visit::Visit},
};

#[proc_macro_attribute]
pub fn turn_off_the_borrow_checker(_attribute: TokenStream, input: TokenStream) -> TokenStream {
    let input: syn::File = parse_macro_input!(input);

    let output = BorrowCheckerSuppressor.fold_file(input);

    static DANGER: std::sync::Once = std::sync::Once::new();
    DANGER.call_once(|| {
        println!();
        println!(
            "{}  This project is using the the {}",
            " DANGER ".white().on_red().bold().slow_blink(),
            "#[you_can::turn_off_the_borrow_checker]".bold()
        );
        println!(
            "{}  macro, which is inherently unsafe, unsound, and unstable. This is not",
            " DANGER ".red().on_black().bold().slow_blink()
        );
        println!(
            "{}  suitable for any purpose beyond curious educational experimentation.",
            " DANGER ".black().on_white().bold().slow_blink()
        );
        println!();
    });

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
}

impl<'ast> Visit<'ast> for RefCollector {
    fn visit_pat_ident(&mut self, node: &'ast syn::PatIdent) {
        if node.by_ref.is_some() {
            self.refs.push(node.ident.clone());
        }
    }
}
