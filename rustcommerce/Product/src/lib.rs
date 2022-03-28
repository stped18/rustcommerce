use std::collections::HashMap;
use Attribute;
#[derive(Default)]
struct Product{
    id: i128,
    body:HashMap<String,Attribute>,

}

impl Product {
    pub fn new(id:i128)->Self{
        return Self{
            id:id,
            body: HashMap::new()
        }
    }
    pub fn addAttribut(&self,label:String, attribut:Attribute){
        self.body.insert(lable, attribut);
    }
    pub fn removeAttribut(&self,label:String){
        self.body.remove(&label);
    }
    pub fn validate(&self, label:String){
        return self.contains_key(label);
    }
}




#[cfg(test)]
mod tests {
    use crate::Attribut;
    use crate::Product;
    #[test]
    fn it_works() {
        let prod = Product::new(0);
        let at1 = Attribut::new(0,String::from("test1"), String::from("value1"));
        prod.addAttribut(String::from("tester"), at1);
        
    }
}
