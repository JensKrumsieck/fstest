use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{
    Ident, LitBool, Token,
    parse::{Parse, ParseStream},
    parse_macro_input,
};

#[derive(Default)]
struct FsTestArgs {
    pub repo: bool,
    pub files: Vec<String>,
}

impl Parse for FsTestArgs {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let mut repo = false;
        let mut files = Vec::new();

        while !input.is_empty() {
            let ident: Ident = input.parse()?;
            input.parse::<Token![=]>()?;

            if ident == "repo" {
                let val: LitBool = input.parse()?;
                repo = val.value();
            } else if ident == "files" {
                let content;
                syn::bracketed!(content in input);
                while !content.is_empty() {
                    let val: syn::LitStr = content.parse()?;
                    files.push(val.value());

                    if content.peek(Token![,]) {
                        content.parse::<Token![,]>()?;
                    } else {
                        break;
                    }
                }
            } else {
                return Err(syn::Error::new(ident.span(), "Unknown argument"));
            }
            let _ = input.parse::<Token![,]>();
        }

        Ok(FsTestArgs { repo, files })
    }
}

/// Attribute macro to create file-system-isolated integration tests with optional git repo setup.
///
/// This macro generates a `#[test]` function that:
/// - Creates a temporary directory
/// - Optionally initializes a Git repository (via `repo = true`)
/// - Optionally copies specified files into the temp directory (via `files = "path1", "path2", ...`)
/// - Invokes the annotated function (renamed with `_inner` suffix) with the temp directory path
/// - Resets the working directory after the test
///
/// # Parameters
///
/// - `repo`: `bool` — If `true`, initializes a Git repository in the temp directory before the test.
/// - `files`: One or more string literals (not a list!) representing relative file paths to copy.
///
/// # Example
///
/// ```rust
/// use fstest::fstest;
///
/// #[fstest(repo = true, files = ["tests/data/config.toml", "tests/data/input.txt"])]
/// fn integration_example(tempdir: &std::path::Path) {
///     let config_path = tempdir.join("config.toml");
///     assert!(config_path.exists());
/// }
/// ```
#[proc_macro_attribute]
pub fn fstest(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = if attr.is_empty() {
        FsTestArgs::default()
    } else {
        parse_macro_input!(attr as FsTestArgs)
    };
    let repo = args.repo;
    let files = args.files;
    let quoted_files = files.iter().map(|file| {
        let file_str = syn::LitStr::new(file, Span::call_site());
        quote! { #file_str }
    });

    let input_fn = parse_macro_input!(item as syn::ItemFn);
    let fn_name = &input_fn.sig.ident;
    let inner_fn_name = Ident::new(&format!("{}_inner", fn_name), fn_name.span());

    let vis = &input_fn.vis;
    let attrs = &input_fn.attrs;
    let inputs = &input_fn.sig.inputs;
    let fn_body = &input_fn.block;

    let generated = quote! {
        #(#attrs)*
        #vis fn #inner_fn_name(#inputs) {
            #fn_body
        }
        #[test]
        #[serial]
        fn #fn_name() {
            use fstest::serial_test::serial;
            use fstest::create_repo_and_commit;
            use std::fs;

            let tmpdir = fstest::tempfile::tempdir().expect("Could not create tempdir");
            let current = std::env::current_dir().expect("Could not get current dir");

            // Copy files to tempdir
            #(
                let file_path = Path::new(#quoted_files);
                let target_path = tmpdir.path().join(file_path.file_name().unwrap());
                fs::copy(file_path, target_path).expect(&format!("Could not copy file {:?}", #quoted_files));
            )*

            std::env::set_current_dir(&tmpdir).expect("Could not set current dir");

            //create repo if needed
            if #repo {
                create_repo_and_commit(tmpdir.path()).expect("Could not create repo");
            }

            #inner_fn_name(tmpdir.path());

            std::env::set_current_dir(current).expect("Could not set current dir");
        }
    };

    generated.into()
}
