use std::{
    fs,
    path::PathBuf,
};

use syn::{
    File as AST,
    visit_mut::VisitMut,
};

use anyhow::Result;

#[derive(Default)]
pub struct Bundler {
    dir_path: PathBuf,
}

impl Bundler {
    pub fn new(dir_path: impl Into<PathBuf>) -> Self {
        return Self {
            dir_path: dir_path.into(),
        };
    }

    pub fn bundle_to_ast(mut self, filename: String) -> Result<AST> {
        let root_code = fs::read_to_string(self.dir_path.join(filename))?;

        let mut ast = syn::parse_file(&root_code)?;

        self.visit_file_mut(&mut ast);

        return Ok(ast);
    }

    fn generate_mod_items(&mut self, mod_name: String) -> Result<Vec<syn::Item>> {
        let (mut bundler, mut ast) =
            match fs::read_to_string(self.dir_path.join(format!("{mod_name}.rs"))) {
                Ok(code) => (Bundler::new(self.dir_path.clone()), syn::parse_file(&code)?),
                _ => {
                    let dir_path = self.dir_path.join(mod_name);
                    let code = fs::read_to_string(dir_path.join(String::from("mod.rs")))?;

                    (Bundler::new(dir_path), syn::parse_file(&code)?)
                }
            };

        bundler.visit_file_mut(&mut ast);

        return Ok(ast.items);
    }
}

impl VisitMut for Bundler {
    fn visit_item_mod_mut(&mut self, item_mod: &mut syn::ItemMod) {
        for attr in &mut item_mod.attrs {
            self.visit_attribute_mut(attr);
        }

        self.visit_visibility_mut(&mut item_mod.vis);
        self.visit_ident_mut(&mut item_mod.ident);

        if item_mod.content.is_none() {
            let items = self.generate_mod_items(item_mod.ident.to_string())
                .expect(&format!("Error bundling mod [{}] from: {:?}", item_mod.ident, self.dir_path));

            item_mod.content = Some((Default::default(), items));
        }

        let items = match &mut item_mod.content {
            Some((_, items)) => items,
            _ => unreachable!(),
        };

        for item in items.iter_mut() {
            self.visit_item_mut(item);
        }
    }
}

