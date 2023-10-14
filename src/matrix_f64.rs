use amx::{prelude::*,XRow,YRow,ZRow};
use super::*;

pub struct MatrixF64{
  m:u32,
  n:u32,
  data: Vec<f64>
}
impl MatrixF64{
  pub fn new(m:u32,n:u32)->Self {
    assert!(m>0 && n>0);
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
    &self.data[(i*self.n +j) as usize]
  }
}
impl TMatrix<'_,f64> for &MatrixF64{
  fn m(&self)->u32{
    self.m
  }
  fn n(&self)->u32{
    self.n
  }
  fn get(&self,i:u32,j:u32)->&f64{
    &self.data[(i*self.n +j) as usize]
  }
}
impl TMatrixMut<'_,f64> for MatrixF64{
  fn get_mut(&mut self,i:u32,j:u32)->&mut f64{
    &mut self.data[(i*self.n +j) as usize]
  }
}
impl Sub for &MatrixF64{
  type Output = MatrixF64;
  fn sub(self,rhs:Self)->Self::Output{
    op::<'-'>(self,rhs)
  }
}
impl Add for &MatrixF64{
  type Output = MatrixF64;
  fn add(self,rhs:Self)->Self::Output{
    op::<'+'>(self,rhs)
  }
}
impl Sub<&MatrixF64> for MatrixF64{
  type Output = MatrixF64;
  fn sub(self,rhs:&Self)->Self::Output{
    op::<'-'>(self,rhs)
  }
}
impl Add<&MatrixF64> for MatrixF64{
  type Output = MatrixF64;
  fn add(self,rhs:&Self)->Self::Output{
    op::<'+'>(self,rhs)
  }
}
impl Sub<&Matrix<f64>> for &MatrixF64{
  type Output = MatrixF64;
  fn sub(self,rhs:&Matrix<f64>)->Self::Output{
    op::<'-'>(self,rhs)
  }
}
impl Add<&Matrix<f64>> for &MatrixF64{
  type Output = MatrixF64;
  fn add(self,rhs:&Matrix<f64>)->Self::Output{
    op::<'+'>(self,rhs)
  }
}
impl Sub<&Matrix<f64>> for MatrixF64{
  type Output = Self;
  fn sub(self,rhs:&Matrix<f64>)->Self{
    op::<'-'>(self,rhs)
  }
}
impl Add<&Matrix<f64>> for MatrixF64{
  type Output = Self;
  fn add(self,rhs:&Matrix<f64>)->Self{
    op::<'+'>(self,rhs)
  }
}
fn op<'a,const OP:char>(lhs:impl TMatrix<'a, f64>,rhs:impl TMatrix<'a,f64>)->MatrixF64{
    assert!(lhs.m() == rhs.m() && lhs.n() == rhs.n());
    let mut rc = MatrixF64::new(lhs.m(),lhs.n());
    let _md8 = lhs.m()/8;
    let nd8 = lhs.n()/8;
    let _m8 = lhs.m()%8;
    let _n8 = lhs.n()%8;
    let mut ctx = amx::AmxCtx::new().unwrap();
    if nd8 == 0 {
      for i in 0..lhs.m() {
        for j in 0..lhs.n() {
          match OP {
           '+' => {*rc.get_mut(i,j) = lhs.get(i,j)+rhs.get(i,j);}
           '-' => {*rc.get_mut(i,j) = lhs.get(i,j)+rhs.get(i,j);}
           _ => {}
          }
        }
      }
    } else {
      for i in 0..lhs.m() {
        for j in 0..nd8 {
          unsafe {
            ctx.load512(lhs.get(i,j*8),ZRow(0));
            ctx.load512(rhs.get(i,j*8),XRow(0));
            match OP {
              '+' => { ctx.fma64_vec_xz(0,0); }
              '-' => { ctx.fms64_vec_xz(0,0); }
              _ => {}
            }
            ctx.store512(rc.get_mut(i,j*8),ZRow(0));
          }
        }
      }
      for i in 0..lhs.m() {
        for j in nd8*8..lhs.n() {
          match OP {
            '+' => { *rc.get_mut(i,j) = lhs.get(i,j) + rhs.get(i,j); }
            '-' => { *rc.get_mut(i,j) = lhs.get(i,j) - rhs.get(i,j); }
            _ => {}
          }
        }
      }
    }
    rc
}
