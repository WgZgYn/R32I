use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, TokenStream, TokenTree};
use std::collections::HashMap;
use std::str::FromStr;
// riscv_asm! {
//     addi a5, a0, 0;   // mv a5, a0
//     ble  a0, zero, L4;
//     li   a0, 1;
//     li   a4, 0;
//     j    L3;
// L5:
//     addi a0, a3, 0;   // mv a0, a3
// L3:
//     add  a3, a4, a0;
//     addi a5, a5, -1;
//     addi a4, a0, 0;   // mv a4, a0
//     bne  a5, zero, L5;
//     nop;
// L4:
//     li   a0, 0;
//     nop;
// };

// TODO:
// ;
// allow declare var
// support number label
// comments              ------------50%  support // /**/,          # not support yet
// label                 ------------50%
// imm with lw, sw... it need like  -4(t0) -> t0, -4
// offset                ------------50% distinguish the mem imm with others
// change to lowercase   ------------OK
// allow anonymous type  ------------OK
// import necessary mods ------------OK
#[proc_macro]
pub fn riscv_asm(input: TokenStream) -> TokenStream {
    // split to lines
    let mut vec = input.into_iter().collect::<Vec<_>>();

    let mut instructions = vec
        .split_mut(|token| {
            if let TokenTree::Punct(p) = token {
                p.as_char() == ';'
            } else {
                false
            }
        })
        .map(|tokens| tokens.to_vec())
        .collect::<Vec<_>>();

    process_data_directive(&mut instructions);

    // allow -4(a0)
    instructions.iter_mut().for_each(|tokens| {
        if let [.., literal, group] = &tokens[..] {
            if let TokenTree::Literal(_) = literal {
                if let TokenTree::Group(_) = group {
                    let mut has_op = false;
                    if let Some(TokenTree::Punct(p)) = tokens.get(tokens.len() - 3) {
                        has_op = p.as_char() == '-';
                    }
                    let TokenTree::Group(g) = tokens.pop().unwrap() else {
                        unreachable!()
                    };
                    let l = tokens.pop().unwrap();
                    let mut o = TokenTree::Punct(Punct::new('-', Spacing::Alone));
                    if has_op {
                        o = tokens.pop().unwrap();
                    }
                    tokens.push(g.stream().into_iter().next().unwrap());
                    tokens.push(TokenTree::Punct(Punct::new(',', Spacing::Alone)));
                    if has_op {
                        tokens.push(o);
                    }
                    tokens.push(l);
                }
            }
        }
    });

    // record the labels
    let labels: HashMap<String, i32> = instructions
        .iter()
        .enumerate()
        .filter_map(|(i, token)| {
            if let [TokenTree::Ident(id), TokenTree::Punct(p), ..] = &token[..] {
                if p.as_char() == ':' {
                    return Some((id.to_string(), i as i32));
                }
            }
            None
        })
        .collect();

    // replace the label offset
    instructions.iter_mut().enumerate().for_each(|(i, tokens)| {
        if let [.., token] = &mut tokens[..] {
            let key = token.to_string();
            if let Some(&target) = labels.get(&key) {
                *token = TokenTree::Literal(Literal::i32_unsuffixed((target - i as i32) * 2));
            }
        }
    });

    let mut output = TokenStream::new();
    instructions
        .iter_mut()
        .map(|tokens| {
            if let [TokenTree::Ident(_), TokenTree::Punct(p), ..] = &tokens[..] {
                if p.as_char() == ':' {
                    return &mut tokens[2..];
                }
            }
            tokens
        })
        .filter(|tokens| !tokens.is_empty())
        .map(|tokens| {
            if let [TokenTree::Ident(i), ..] = tokens {
                *i = Ident::new(&i.to_string().to_ascii_lowercase(), i.span())
            }
            tokens
        })
        .for_each(|tokens| {
            if let TokenTree::Literal(_) = &tokens[0] {
                output.extend(Some(tokens[0].clone()));
                output.extend(Some(TokenTree::Punct(Punct::new(',', Spacing::Alone))));
                return;
            }

            let mut v = Vec::new();
            v.push(tokens[0].clone());
            v.push(TokenTree::Group(Group::new(
                Delimiter::Parenthesis,
                TokenStream::from_iter(tokens.iter().cloned().skip(1)),
            )));
            v.push(TokenTree::Punct(Punct::new(',', Spacing::Alone)));
            output.extend(v);
        });

    TokenStream::from(wrap_and_import(output))
}

fn wrap_and_import(output: TokenStream) -> TokenTree {
    // import mods
    let mut import_mods = TokenStream::from_str(concat!(
        "use crate::instruct_info::prelude::*;",
        "use crate::register::alias::*;"
    ))
    .expect("import mods");
    let borrow_code = [
        TokenTree::Punct(Punct::new('&', Spacing::Alone)),
        TokenTree::Group(Group::new(Delimiter::Bracket, output)),
    ];
    import_mods.extend(borrow_code);

    // avoid leaking to outer
    TokenTree::Group(Group::new(Delimiter::Brace, import_mods))
}

fn process_data_directive(instructions: &mut Vec<Vec<TokenTree>>) {
    let mut byte_address = 0;
    instructions.iter_mut().for_each(|tokens| {
        byte_address += 4;

        let l = tokens.len();
        if l < 3 {
            return;
        }
        for i in 0..(l - 2) {
            if let TokenTree::Punct(p) = &tokens[i] {
                if p.as_char() != '.' {
                    continue;
                }
                if let TokenTree::Ident(id) = &tokens[i + 1] {
                    match id.to_string().as_str() {
                        "byte" => {
                            tokens.remove(i);
                            tokens.remove(i);
                            eprintln!("consider as u32");
                        }
                        "half" => {
                            tokens.remove(i);
                            tokens.remove(i);
                            eprintln!("consider as u32");
                        }
                        "word" => {
                            tokens.remove(i);
                            tokens.remove(i);
                            eprintln!("consider as u32");
                        }
                        _ => continue,
                    }
                }
            }
        }
    });
}

enum VarSize {
    Byte,
    Half,
    Word,
}

fn instruction_size(tokens: &[TokenTree]) -> VarSize {
    let len = tokens.len();
    if len < 3 {
        return VarSize::Word;
    }
    for i in 0..(len - 2) {
        if let TokenTree::Punct(p) = &tokens[i] {
            if p.as_char() == '.' {
                if let TokenTree::Ident(id) = &tokens[i + 1] {
                    if let TokenTree::Literal(_) = &tokens[i + 2] {
                        return match id.to_string().as_str() {
                            "byte" => VarSize::Byte,
                            "half" => VarSize::Half,
                            "word" => VarSize::Word,
                            _ => panic!("unknown identifier"),
                        };
                    }
                }
            }
        }
    }
    VarSize::Word
}

fn get_label(tokens: &[TokenTree]) -> Option<String> {
    if let [TokenTree::Ident(id), TokenTree::Punct(p), ..] = tokens {
        if p.as_char() == ':' {
            return Some(id.to_string());
        }
    }
    None
}
