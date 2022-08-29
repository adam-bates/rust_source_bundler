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

    fn bundle_mod(&mut self, item_mod: &mut syn::ItemMod) -> Result<()> {
        let name = item_mod.ident.to_string();

        let (mut bundler, mut ast) = match fs::read_to_string(self.dir_path.join(format!("{name}.rs"))) {
            Ok(code) => (Bundler::new(self.dir_path.clone()), syn::parse_file(&code)?),
            _ => {
                let dir_path = self.dir_path.join(name);
                let code = fs::read_to_string(dir_path.join(String::from("mod.rs")))?;

                (Bundler::new(dir_path), syn::parse_file(&code)?)
            },
        };

        bundler.visit_file_mut(&mut ast);

        item_mod.content = Some((Default::default(), ast.items));

        return Ok(());
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
            self.bundle_mod(item_mod)
                .expect(&format!("Error bundling mod [{}] from: {:?}", item_mod.ident, self.dir_path));
        }
        
        if let Some((_, items)) = &mut item_mod.content {
            for item in items.iter_mut() {
                self.visit_item_mut(item);
            }
        }
    }
}

