use OpsMacro;
use syn::FieldsNamed;
use quote::Tokens;

impl OpsMacro for FieldsNamed {
    fn add_impl(&self, ty: &Tokens) -> Tokens {
        let fields = self.named
            .iter()
            .map(|fd| {
                let ident = &fd.ident;
                quote! {#ident: self.#ident + other.#ident}
            })
            .collect::<Vec<_>>();
        quote!{
            #ty {
                #(#fields), *
            }
        }
    }
}
