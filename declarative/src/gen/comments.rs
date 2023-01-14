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

pub struct DocComment<T>(pub T);

impl<T> tokens::FormatInto<Rust> for DocComment<T>
where
    T: IntoIterator,
    T::Item: Into<tokens::ItemStr>,
{
    fn format_into(self, tokens: &mut Tokens<Rust>) {
        for line in self.0 {
            tokens.push();
            tokens.append(tokens::static_literal("///"));
            tokens.space();
            tokens.append(line.into());
        }
    }
}


pub struct BeginSection<T>(pub T);

impl<T> tokens::FormatInto<Rust> for BeginSection<T>
where
    T: Into<tokens::ItemStr>,
{
    fn format_into(self, tokens: &mut Tokens<Rust>) {
        tokens.push();
        tokens.append(tokens::static_literal("// >>> Begin section @"));
        tokens.append(self.0.into());
}
}

pub struct EndSection<T>(pub T);
impl<T> tokens::FormatInto<Rust> for EndSection<T>
where
    T: Into<tokens::ItemStr>,
{
    fn format_into(self, tokens: &mut Tokens<Rust>) {
        tokens.push();
        tokens.append(tokens::static_literal("// >>> End section @"));
        tokens.append(self.0.into());
    }
}
