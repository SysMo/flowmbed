use genco::{tokens, Tokens};
use genco::prelude::Rust;

pub struct Comment<T>(pub T);

impl<T> tokens::FormatInto<Rust> for Comment<T>
where
    T: IntoIterator,
    T::Item: Into<tokens::ItemStr>,
{
    fn format_into(self, tokens: &mut Tokens<Rust>) {
        for line in self.0 {
            tokens.push();
            tokens.append(tokens::static_literal("//"));
            tokens.space();
            tokens.append(line.into());
        }
    }
}