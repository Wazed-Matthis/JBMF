use proc_macro2::TokenStream;

#[derive(Debug)]
pub struct StructGenerationOptions<'a> {
    pub(crate) return_type: TokenStream,
    pub(crate) read_return_type: ReturnType,
    pub(crate) write_return_type: ReturnType,
    pub(crate) read_unwrap: bool,
    pub(crate) use_self: bool,
    pub(crate) exclude: Vec<&'a str>,
}

#[derive(Debug, Copy, Clone)]
pub enum ReturnType {
    Nothing,
    OkResult,
    EmptyOkResult,
}

impl<'a> StructGenerationOptions<'a> {
    pub fn new() -> Self {
        StructGenerationOptions {
            return_type: Default::default(),
            read_return_type: ReturnType::OkResult,
            write_return_type: ReturnType::EmptyOkResult,
            read_unwrap: true,
            use_self: true,
            exclude: Vec::new(),
        }
    }
    pub fn return_type(&mut self, return_type: TokenStream) -> &mut Self {
        self.return_type = return_type;
        self
    }
    pub fn read_return_type(&mut self, read_return_type: ReturnType) -> &mut Self {
        self.read_return_type = read_return_type;
        self
    }
    pub fn write_return_type(&mut self, write_return_type: ReturnType) -> &mut Self {
        self.write_return_type = write_return_type;
        self
    }
    pub fn read_unwrap(&mut self, read_unwrap: bool) -> &mut Self {
        self.read_unwrap = read_unwrap;
        self
    }
    pub fn use_self(&mut self, use_self: bool) -> &mut Self {
        self.use_self = use_self;
        self
    }
    pub fn exclude(&mut self, exclude: Vec<&'a str>) -> &mut Self {
        self.exclude = exclude;
        self
    }
}

impl<'a> Clone for StructGenerationOptions<'a> {
    fn clone(&self) -> Self {
        StructGenerationOptions {
            return_type: self.return_type.clone(),
            read_return_type: self.read_return_type,
            write_return_type: self.write_return_type,
            read_unwrap: self.read_unwrap,
            use_self: self.use_self,
            exclude: self.exclude.clone(),
        }
    }
}
