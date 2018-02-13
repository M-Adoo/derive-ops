use OpsMacro;
use syn::FieldsUnnamed;
use quote::Tokens;

impl OpsMacro for FieldsUnnamed {
    fn add_impl(&self, ty: &Tokens) -> Tokens {
        quote!{ #ty(self.0 + other.0) }
    }
}
