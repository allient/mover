//! Procedural macros for the mover automation library
//! 
//! This module provides convenient macros for common automation tasks.

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, Expr, Lit, parse::Parse, parse::ParseStream, Token, Ident, punctuated::Punctuated};

/// Macro for creating a simple automation script
#[proc_macro_attribute]
pub fn automation(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let fn_name = &input.sig.ident;
    
    let expanded = quote! {
        #input
        
        impl #fn_name {
            /// Runs the automation function with error handling
            pub fn run() -> Result<(), Box<dyn std::error::Error>> {
                // Initialize logging
                env_logger::init();
                
                // Run the automation
                #fn_name()?;
                
                Ok(())
            }
        }
    };
    
    TokenStream::from(expanded)
}

/// Macro for creating a mouse movement sequence
#[proc_macro]
pub fn mouse_sequence(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as MouseSequence);
    
    let moves = input.moves.iter().map(|(x, y)| {
        quote! {
            mover_mouse::move_to(#x, #y)?;
        }
    });
    
    let expanded = quote! {
        {
            #(#moves)*
            Ok(())
        }
    };
    
    TokenStream::from(expanded)
}

/// Macro for creating a click sequence
#[proc_macro]
pub fn click_sequence(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ClickSequence);
    
    let clicks = input.clicks.iter().map(|(x, y, button)| {
        quote! {
            mover_mouse::click_at(#x, #y, Some(mover_core::MouseButton::#button))?;
        }
    });
    
    let expanded = quote! {
        {
            #(#clicks)*
            Ok(())
        }
    };
    
    TokenStream::from(expanded)
}

/// Macro for creating a keyboard sequence
#[proc_macro]
pub fn keyboard_sequence(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as KeyboardSequence);
    
    let keys = input.keys.iter().map(|key| {
        quote! {
            mover_keyboard::press_key(#key)?;
        }
    });
    
    let expanded = quote! {
        {
            #(#keys)*
            Ok(())
        }
    };
    
    TokenStream::from(expanded)
}

/// Macro for creating a wait sequence
#[proc_macro]
pub fn wait_sequence(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as WaitSequence);
    
    let waits = input.waits.iter().map(|duration| {
        quote! {
            std::thread::sleep(std::time::Duration::from_secs_f64(#duration));
        }
    });
    
    let expanded = quote! {
        {
            #(#waits)*
        }
    };
    
    TokenStream::from(expanded)
}

// Parser structs for the macros

struct MouseSequence {
    moves: Vec<(i32, i32)>,
}

impl Parse for MouseSequence {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let moves = Punctuated::<Expr, Token![,]>::parse_terminated(input)?;
        
        let mut result = Vec::new();
        for expr in moves {
            if let Expr::Tuple(ref tuple) = expr {
                if tuple.elems.len() == 2 {
                    if let (Expr::Lit(x), Expr::Lit(y)) = (&tuple.elems[0], &tuple.elems[1]) {
                        if let (Lit::Int(x_lit), Lit::Int(y_lit)) = (&x.lit, &y.lit) {
                            result.push((x_lit.base10_parse::<i32>()?, y_lit.base10_parse::<i32>()?));
                            continue;
                        }
                    }
                }
            }
            return Err(syn::Error::new_spanned(&expr, "Expected tuple of two integers"));
        }
        
        Ok(MouseSequence { moves: result })
    }
}

struct ClickSequence {
    clicks: Vec<(i32, i32, Ident)>,
}

impl Parse for ClickSequence {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let clicks = Punctuated::<Expr, Token![,]>::parse_terminated(input)?;
        
        let mut result = Vec::new();
        for expr in clicks {
            if let Expr::Tuple(ref tuple) = expr {
                if tuple.elems.len() == 3 {
                    if let (Expr::Lit(x), Expr::Lit(y), Expr::Path(button)) = 
                        (&tuple.elems[0], &tuple.elems[1], &tuple.elems[2]) {
                        if let (Lit::Int(x_lit), Lit::Int(y_lit)) = (&x.lit, &y.lit) {
                            if let Some(ident) = button.path.get_ident() {
                                result.push((
                                    x_lit.base10_parse::<i32>()?, 
                                    y_lit.base10_parse::<i32>()?, 
                                    ident.clone()
                                ));
                                continue;
                            }
                        }
                    }
                }
            }
            return Err(syn::Error::new_spanned(&expr, "Expected tuple of two integers and button identifier"));
        }
        
        Ok(ClickSequence { clicks: result })
    }
}

struct KeyboardSequence {
    keys: Vec<String>,
}

impl Parse for KeyboardSequence {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let keys = Punctuated::<Lit, Token![,]>::parse_terminated(input)?;
        
        let mut result = Vec::new();
        for lit in keys {
            if let Lit::Str(s) = lit {
                result.push(s.value());
            } else {
                return Err(syn::Error::new_spanned(lit, "Expected string literal"));
            }
        }
        
        Ok(KeyboardSequence { keys: result })
    }
}

struct WaitSequence {
    waits: Vec<f64>,
}

impl Parse for WaitSequence {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let waits = Punctuated::<Lit, Token![,]>::parse_terminated(input)?;
        
        let mut result = Vec::new();
        for lit in waits {
            if let Lit::Float(f) = lit {
                result.push(f.base10_parse::<f64>()?);
            } else if let Lit::Int(i) = lit {
                result.push(i.base10_parse::<f64>()?);
            } else {
                return Err(syn::Error::new_spanned(lit, "Expected numeric literal"));
            }
        }
        
        Ok(WaitSequence { waits: result })
    }
}
