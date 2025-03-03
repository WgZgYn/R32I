use proc_macro::TokenStream;

// 在 proc-macro crate 中
#[proc_macro]
pub fn register_aliases(_: TokenStream) -> TokenStream {
    (0..32)
        .map(|i| {
            format!(
                "pub const x{}: u8 = {{ {} }};",
                i, i
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
        .parse()
        .unwrap()
}
