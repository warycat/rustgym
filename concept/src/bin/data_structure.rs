trait Vehicle {
    fn drive(&self);
}

struct Truck;


impl Vehicle for Truck {
    fn drive(&self) {
        println!("The truck is Driving");
    }
}

fn main(){
    let t: Box<dyn Vehicle>;
    t = Box::new(Truck);
    t.drive();
}