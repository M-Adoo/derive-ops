use OpsMacro;
use syn::FieldsUnnamed;
use quote::Tokens;

impl OpsMacro for FieldsUnnamed {
    fn add_impl(&self, ty: &Tokens) -> Tokens {
        quote!{ #ty(self.0 + other.0) }
    }

    fn add_assign_impl(&self, _ty: &Tokens) -> Tokens {
        quote!{ self.0 += other.0;}
    }
}
