use proc_macro::TokenStream;
use syn::Data;

#[proc_macro_derive(Interpretable)]
pub fn derive_interpretable(input: TokenStream) -> TokenStream {
    let ast = syn::parse::<syn::DeriveInput>(input).unwrap();
    impl_interpretable(&ast)
}

fn impl_interpretable(ast: &syn::DeriveInput) -> TokenStream {
    match &ast.data {
        Data::Enum(data) => {
            let enum_name = &ast.ident;
            let variant_names = data.variants.iter().map(|x| &x.ident);
            quote::quote!(
                impl crate::Interpretable for #enum_name {
                    fn interpret<'a, 'b>(&'a self, declaration_stack: &'b mut DeclarationStack<'a>, variable_stack: &'b mut VariableStack<'a>) -> Result<InterpreterValue, InterpreterError> {
                        match self {
                            #(
                                Self::#variant_names(val) => val.interpret(declaration_stack, variable_stack),
                            )*
                        }
                    }
                }
            ).into()
        }
        _ => quote::quote! {}.into(),
    }
}
