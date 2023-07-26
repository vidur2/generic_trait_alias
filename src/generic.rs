pub struct GenericTrait {
    ident: String,
    traits: Vec<String>,
    is_pub: bool
}

impl GenericTrait {
    pub fn new() -> Self { Self { ident: String::new(), traits: Vec::new(), is_pub: false } }

    

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

    pub fn is_pub(&self) -> bool {
        self.is_pub
    }

    pub fn set_is_pub(&mut self, is_pub: bool) {
        self.is_pub = is_pub;
    }
}