use amx::{prelude::*,XRow,YRow,ZRow};
use super::*;

pub struct MatrixF32{
  m:u32,
  n:u32,
  data: Vec<f32>
}
impl MatrixF32{
  pub fn new(m:u32,n:u32)->Self {
    assert!(m>0 && n>0);
    let mut v = Vec::new();
    v.resize((m*n) as usize,0.0);
    Self{m:m,n:n,data:v}
  }
}
impl TMatrix<'_,f32> for MatrixF32{
  fn m(&self)->u32{
    self.m
  }
  fn n(&self)->u32{
    self.n
  }
  fn get(&self,i:u32,j:u32)->&f32{
    &self.data[(i*self.n +j) as usize]
  }
  fn get_mut(&mut self,i:u32,j:u32)->&mut f32{
    &mut self.data[(i*self.n +j) as usize]
  }
}
impl Sub for &mut MatrixF32{
  type Output = Self;
  fn sub(self,rhs:Self)->Self{
    todo!()
  }
}
impl Add for &mut MatrixF32{
  type Output = Self;
  fn add(self,rhs:Self)->Self{
    todo!()
  }
}
impl Sub<&Matrix<f32>> for &MatrixF32{
  type Output = Self;
  fn sub(self,rhs:&Matrix<f32>)->Self{
    todo!()
  }
}
impl Add<&Matrix<f32>> for &MatrixF32{
  type Output = Self;
  fn add(self,rhs:&Matrix<f32>)->Self{
    todo!()
  }
}
impl Sub<&Matrix<f32>> for MatrixF32{
  type Output = Self;
  fn sub(self,rhs:&Matrix<f32>)->Self{
    todo!()
  }
}
impl Add<&Matrix<f32>> for MatrixF32{
  type Output = Self;
  fn add(self,rhs:&Matrix<f32>)->Self{
    todo!()
  }
}
