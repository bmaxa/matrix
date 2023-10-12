use amx::{prelude::*,XRow,YRow,ZRow};
use super::*;

pub struct MatrixF64{
  m:u32,
  n:u32,
  data: Vec<f64>
}
impl MatrixF64{
  pub fn new(m:u32,n:u32)->Self {
    let mut v = Vec::new();
    v.resize((m*n) as usize,0.0);
    Self{m:m,n:n,data:v}
  }
}
impl TMatrix<'_,f64> for MatrixF64{
  fn m(&self)->u32{
    self.m
  }
  fn n(&self)->u32{
    self.n
  }
  fn get(&self,i:u32,j:u32)->&f64{
    &self.data[(i*self.m +j) as usize]
  }
  fn get_mut(&mut self,i:u32,j:u32)->&mut f64{
    &mut self.data[(i*self.m +j) as usize]
  }
}
impl Sub for &MatrixF64{
  type Output = Self;
  fn sub(self,rhs:Self)->Self{
    todo!()
  }
}
impl Add for &MatrixF64{
  type Output = Self;
  fn add(self,rhs:Self)->Self{
    todo!()
  }
}
impl Sub<&Matrix<f64>> for &MatrixF64{
  type Output = Self;
  fn sub(self,rhs:&Matrix<f64>)->Self{
    todo!()
  }
}
impl Add<&Matrix<f64>> for &MatrixF64{
  type Output = Self;
  fn add(self,rhs:&Matrix<f64>)->Self{
    todo!()
  }
}
impl Sub<&Matrix<f64>> for MatrixF64{
  type Output = Self;
  fn sub(self,rhs:&Matrix<f64>)->Self{
    todo!()
  }
}
impl Add<&Matrix<f64>> for MatrixF64{
  type Output = Self;
  fn add(self,rhs:&Matrix<f64>)->Self{
    todo!()
  }
}
