use std::process::exit;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

mod attr;
use crate::attr::TargetStruct;

trait AttributeHelper {
    fn get_formatted_struct(&self) -> TargetStruct;
}

#[proc_macro_derive(Bity, attributes(byte_order, bit_order))]
pub fn derive_helper_attr(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    // println!("{:#?}", input);
    let target: TargetStruct = input.into();
    println!("{:#?}", &target);

    let name = target.name;

    TokenStream::new()
}

mod error {
    use std::fmt::Display;

    pub enum Error<'a> {
        InvalidBitOrder(&'a str),
        InvalidByteOrder(&'a str),
    }

    impl Display for Error<'_> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Error::InvalidBitOrder(order) => {
                    write!(f, "Invalid bit order: {order}. Use little or big.")
                }
                Error::InvalidByteOrder(order) => {
                    write!(f, "Invalid byte order: {order}. Use little or big.")
                }
            }
        }
    }
}

mod endianness {
    use syn::{Data, Fields};

    pub fn handle_data(data: &Data) {
        match data {
            Data::Struct(data_struct) => {
                handle_fields(&data_struct.fields);
            }
            _ => unimplemented!(),
        }
    }

    fn handle_fields(fields: &Fields) {
        match fields {
            Fields::Named(named_fields) => {
                for field in &named_fields.named {
                    let _ = field.attrs;
                }
            }
            _ => unimplemented!(),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
