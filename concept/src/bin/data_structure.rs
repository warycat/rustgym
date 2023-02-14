trait Vehicle {
    fn drive(&self);
}

struct Truck {
    name: String,
    next_truck:Option<Box<Truck>>,
}


impl Vehicle for Truck {
    fn drive(&self) {
       //print the next truck
         println!("Driving {}", self.name);
            match &self.next_truck {
                Some(truck) => truck.drive(),
                None => println!("No more trucks"),
            }
    }
}

fn main(){
    let t: Box<dyn Vehicle>;
    // create a list of Trucks
    let truc2 = Truck {name: "Truck2".to_string(), next_truck: None};
    let truc1 = Truck {name: "Truck1".to_string(), next_truck: Some(Box::new(truc2))};
    t = Box::new(truc1);
    t.drive();
}