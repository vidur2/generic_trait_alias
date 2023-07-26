pub struct GenericTrait {
    ident: String,
    traits: Vec<String>
}

impl GenericTrait {
    pub fn new() -> Self { Self { ident: String::new(), traits: Vec::new() } }

    

    pub fn ident_mut(&mut self) -> &mut String {
        &mut self.ident
    }

    pub fn traits_mut(&mut self) -> &mut Vec<String> {
        &mut self.traits
    }

    pub fn traits(&self) -> &[String] {
        self.traits.as_ref()
    }

    pub fn ident(&self) -> &str {
        self.ident.as_ref()
    }
}