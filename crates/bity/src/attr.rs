use std::str::FromStr;

use syn::{Data, DeriveInput};

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Debug)]
pub struct TargetStruct {
    pub(crate) name: String,
    pub(crate) fields: Vec<Field>,
}

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Debug)]

pub(crate) struct Field {
    pub(crate) name: String,
    pub(crate) field_type: syn::Type,
    /// Attributes must maintain their order.
    pub(crate) byte_order: Option<Endianness>,
    pub(crate) bit_order: Option<Endianness>,
}

enum Attribute {
    ByteOrder(Endianness),
    BitOrder(Endianness),
}

struct FieldName(String);

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Debug)]

pub(crate) enum Endianness {
    Big,
    Little,
}

impl TryFrom<&syn::Attribute> for Attribute {
    type Error = ();

    fn try_from(value: &syn::Attribute) -> Result<Self, ()> {
        let syn::Meta::List(meta) = value.meta.clone() else {
            return Err(());
            // Proper error in the future.
        };
        if meta.tokens.is_empty() {
            return Err(());
            // Proper error in the future.
        }
        let Some(proc_macro2::TokenTree::Ident(ident)) = meta.tokens.into_iter().next() else {
            return Err(());
            // Proper error in the future.
        };

        let Ok(endianness) = Endianness::from_str(&ident.to_string()) else {
            return Err(());
        };
        println!("here");

        println!("\n\n\n\n\n{:#?}\n\n\n\n\n\n", endianness);

        match meta.path.segments[0].ident.to_string().as_str() {
            "bit_order" => Ok(Attribute::BitOrder(endianness)),
            "byte_order" => Ok(Attribute::ByteOrder(endianness)),
            _ => Err(()),
        }
    }
}

impl FromStr for Endianness {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        println!("S: {:#?}", s.as_bytes());
        println!("Ref: {:#?}", "little".as_bytes());
        if s.to_lowercase().eq("little") {
            Ok(Endianness::Little)
        } else if s.to_lowercase().eq("big") {
            Ok(Endianness::Big)
        } else {
            Err(())
        }
    }
}

impl From<Option<syn::Ident>> for FieldName {
    fn from(value: Option<syn::Ident>) -> Self {
        let name = if let Some(ident) = value {
            ident.to_string()
        } else {
            todo!("Anonymous identifiers not supported.");
        };

        FieldName(name)
    }
}

impl From<DeriveInput> for TargetStruct {
    fn from(input: DeriveInput) -> Self {
        let name = input.ident.to_string();
        let Data::Struct(data) = input.data else {
            unimplemented!("Enums and unions are not supported");
        };

        let syn::Fields::Named(named_fields) = data.fields else {
            unimplemented!("Unnamed and unit fields are not supported");
        };

        let mut fields: Vec<Field> = Vec::new();

        for field in named_fields.named.iter() {
            let formed_field: Field = field.into();
            fields.push(formed_field)
        }

        TargetStruct { name, fields }
    }
}

impl From<&syn::Field> for Field {
    fn from(value: &syn::Field) -> Self {
        let name: FieldName = value.ident.clone().into();

        let field_type = if let syn::Type::Path(_) = &value.ty {
            value.ty.clone()
        } else {
            unimplemented!("Only path types are supported.")
        };
        let mut byte_order = None;
        let mut bit_order = None;

        for attr in value.attrs.iter() {
            println!("attributes: {:#?}", attr);

            if let Ok(attr) = Attribute::try_from(attr) {
                match attr {
                    Attribute::ByteOrder(order) => byte_order = Some(order),
                    Attribute::BitOrder(order) => bit_order = Some(order),
                }
            }
        }

        // let byte_order = value.attrs.into();
        Field {
            name: name.0,
            field_type,
            byte_order,
            bit_order,
        }
    }
}
