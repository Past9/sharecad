use proc_macro::TokenStream;
use std::{fs, path::PathBuf, str::FromStr};
use syn::{parse::Parse, parse_macro_input, LitStr};

struct ShaderSrcInfo {
    path: String,
}
impl Parse for ShaderSrcInfo {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(ShaderSrcInfo {
            path: input.parse::<LitStr>()?.value(),
        })
    }
}

#[proc_macro]
pub fn shader_src(tokens: TokenStream) -> TokenStream {
    let info = parse_macro_input!(tokens as ShaderSrcInfo);

    let mut base_path_buf =
        PathBuf::from_str(&std::env::var("CARGO_MANIFEST_DIR").unwrap()).unwrap();

    base_path_buf.push("src");
    base_path_buf.push(info.path);

    let file_name = base_path_buf
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();

    let base_dir = base_path_buf.parent().unwrap().to_path_buf();

    let content = read_file(base_dir, &file_name);

    TokenStream::from_str(&format!("r#\"{}\"#", content)).unwrap()
}

const INCLUDE_PREFIX: &'static str = "//# include ";

fn read_file(base_dir: PathBuf, file_name: &str) -> String {
    let file_path = {
        let mut base_dir = base_dir.clone();
        base_dir.push(file_name);
        fs::canonicalize(base_dir).unwrap()
    };

    let raw_content = fs::read_to_string(file_path).unwrap();

    let mut parts: Vec<String> = vec![];

    for line in raw_content.lines() {
        if line.starts_with(INCLUDE_PREFIX) {
            let dep_path = line
                .chars()
                .into_iter()
                .skip(INCLUDE_PREFIX.len())
                .take(line.len() - INCLUDE_PREFIX.len())
                .collect::<String>()
                .trim()
                .to_owned();

            parts.push(read_file(base_dir.clone(), &dep_path));
        } else {
            parts.push(line.to_owned());
        }
    }

    let content = parts.join("\n");

    content
}
