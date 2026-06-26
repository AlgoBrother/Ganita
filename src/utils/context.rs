pub struct MathContext{
    variable: std::collections::HashMap<String, f64>,
    // Future: constants, function definitions, etc.
}

impl MathContext{
    pub fn new() -> Self {
        let mut context = MathContext { variable:  std::collections::HashMap::new() };
        context.variable.insert("pi".to_string(), std::f64::consts::PI);
        context.variable.insert("e".to_string(), std::f64::consts::E);
        context.variable.insert("phi".to_string(), 1.618033988749895); // golden ratio
        context
    }
    
    pub fn get(&self, name: &str) -> Option<f64> {
        self.variable.get(name).copied()
    }

    pub fn set(&mut self, name: &str, value: f64) {
        self.variable.insert(name.to_string(), value);
    }

    pub fn get_variable(&self) -> &std::collections::HashMap<String, f64> {
        &self.variable
    }
}