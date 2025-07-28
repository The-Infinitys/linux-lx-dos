pub struct Element {
    pub kind: ElementKind,
    _properties: Vec<ElementProperty>,
}

pub enum ElementProperty {
    Text(String),
    Color(u8, u8, u8, u8),
    // Add more properties as needed
}

pub enum ElementKind {
    Button,
    Label,
    TextInput,
    // Add more element types as needed
}
impl Element {
    pub fn property(&mut self, _property: ElementProperty) {
        todo!()
    }
    pub fn add_element(){
        todo!()
    }
}
