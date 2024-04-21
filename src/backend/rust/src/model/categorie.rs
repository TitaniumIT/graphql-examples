use juniper::graphql_object;


#[derive(Clone)]
pub struct Category {
    id: String,
    name: String
}

impl Category {
    pub fn new(id:&str,name:&str) -> Self {
        Self { id: id.to_string() , name:name.to_string() }
    }
}

#[graphql_object]
impl Category {
   pub fn id(&self) -> &String{
     &self.id
   }

   pub fn name(&self) -> &String{
      &self.name
   }
}