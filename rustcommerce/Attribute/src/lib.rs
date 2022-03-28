pub struct Attribut {
    pub id:i128,
    pub label:String,
    pub value:String
}


impl Attribut {
    pub fn new(id:i128,label:String, value:String)->Attribut{
        return Attribut{
            id:id,
            label:label,
            value:value
        }
    }

    pub fn toString(&self){
        println!("id {0}  label {1} value {2}",self.id, self.label,self.value )
    }
    
}


#[cfg(test)]
mod tests {
    use crate::Attribut;

    #[test]
    fn it_works() {
        let at=Attribut::new(0,String::from("new"), String::from("new"));
        at.toString();
        
    }
}
