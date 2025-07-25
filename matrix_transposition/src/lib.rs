#[derive(Debug)]
pub struct Matrix(pub (i32, i32), pub (i32, i32));

impl Matrix {
    fn new(t1:i32, t2:i32) -> (i32, i32){
        (t1, t2)
    }

    fn kamla(x:(i32, i32), y:(i32, i32)) -> Matrix{
        Matrix(x, y)
    }   
}
impl PartialEq for Matrix {
    fn eq(&self, y: &Self) -> bool{
        self.0.1 == y.1.0
    }
}

impl Eq for Matrix{}

pub fn transpose(m: Matrix) -> Matrix {
    
    let m1 = Matrix::new(m.0.0, m.0.1);
    let m2 = Matrix::new(m.1.0, m.1.1);
    
   
    let new_matrix1 = Matrix::new(m1.0, m2.0);
    let new_matrix2 = Matrix::new(m1.1, m2.1);
    let hold = Matrix::kamla(new_matrix1, new_matrix2);
    // println!("**************{:?}", hold);
    
    return hold
   

    /* println!("{:?}, {:?}", m.0, m.1);
    m */
}
