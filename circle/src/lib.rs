use std::f64::consts::PI;

#[derive(Debug, Clone, Copy)]
pub struct Point (
    pub f64,
    pub f64  

);



#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub radius: f64,
    pub center: Point,
}

impl Circle {
    ///// Methods
    pub fn diameter(self: Circle) -> f64{
        self.radius * 2 as f64
    }

    pub fn area(self: Circle) -> f64{ 
        PI * self.radius.powf(2.0) as f64
    }

    pub fn intersect(self: Circle, o: Circle) -> bool{
        // ((self.center.0 + self.radius as f64) > (O.center.0- O.radius as f64)) || ((self.center.1 + self.radius as f64) > (O.center.1 - O.radius as f64 ))  
        let d = self.center.distance(o.center);

        if (d > (self.radius + o.radius)) || (d == 0.0 && self.radius == o.radius){
            false
        }else{
            true
        }
    }
    /// Associated fn
    pub fn new(x: f64, y: f64, r: f64) -> Self{
        
        Self{
            center: Point(x, y), 
            radius: r,
        }
    }

}



impl Point {
    pub fn distance(self, b: Point) -> f64{
        ((self.0 - b.0).powf(2.0) + (self.1 - b.1).powf(2.0)).sqrt() 
    }
}