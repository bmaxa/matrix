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
  fn get_mut(&mut self,i:u32,j:u32)->&mut f64{
    &mut self.data[(i*self.n +j) as usize]
  }
}
impl Sub for &MatrixF64{
  type Output = MatrixF64;
  fn sub(self,rhs:Self)->Self::Output{
    assert!(self.m == rhs.m && self.n == rhs.n);
    let mut rc = MatrixF64::new(self.m,self.n);
    let md8 = self.m/8;
    let nd8 = self.n/8;
    let m8 = self.m%8;
    let n8 = self.n%8;
    let mut ctx = amx::AmxCtx::new().unwrap();
    if nd8 == 0 {
      for i in 0..self.m {
        for j in 0..self.n {
          rc.data[(i*self.n+j)as usize] = self.data[(i*self.n+j)as usize]-rhs.data[(i*self.n+j)as usize];
        }
      }
    } else {
      for i in 0..self.m {
        for j in 0..nd8 {
          unsafe {
            ctx.load512(&self.data[(i*self.n+j*8) as usize],ZRow(0));
            ctx.load512(&rhs.data[(i*self.n+j*8) as usize],XRow(0));
            ctx.fms64_vec_xz(0,0);
            ctx.store512(&mut rc.data[(i*self.n+j*8) as usize],ZRow(0));
          }
        }
      }
      for i in 0..self.m {
        for j in nd8*8..self.n {
          rc.data[(i*self.n+j) as usize]= self.data[(i*self.n+j) as usize] - rhs.data[(i*self.n+j) as usize];
        }
      }
    }
    rc
  }
}
impl Add for &MatrixF64{
  type Output = MatrixF64;
  fn add(self,rhs:Self)->Self::Output{
    assert!(self.m == rhs.m && self.n == rhs.n);
    let mut rc = MatrixF64::new(self.m,self.n);
    let md8 = self.m/8;
    let nd8 = self.n/8;
    let m8 = self.m%8;
    let n8 = self.n%8;
    let mut ctx = amx::AmxCtx::new().unwrap();
    if nd8 == 0 {
      for i in 0..self.m {
        for j in 0..self.n {
          rc.data[(i*self.n+j)as usize] = self.data[(i*self.n+j)as usize]+rhs.data[(i*self.n+j)as usize];
        }
      }
    } else {
      for i in 0..self.m {
        for j in 0..nd8 {
          unsafe {
            ctx.load512(&self.data[(i*self.n+j*8) as usize],ZRow(0));
            ctx.load512(&rhs.data[(i*self.n+j*8) as usize],XRow(0));
            ctx.fma64_vec_xz(0,0);
            ctx.store512(&mut rc.data[(i*self.n+j*8) as usize],ZRow(0));
          }
        }
      }
      for i in 0..self.m {
        for j in nd8*8..self.n {
          rc.data[(i*self.n+j) as usize]= self.data[(i*self.n+j) as usize] + rhs.data[(i*self.n+j) as usize];
        }
      }
    }
    rc
  }
}
impl Sub<&MatrixF64> for MatrixF64{
  type Output = MatrixF64;
  fn sub(self,rhs:&Self)->Self::Output{
    assert!(self.m == rhs.m && self.n == rhs.n);
    let mut rc = MatrixF64::new(self.m,self.n);
    let md8 = self.m/8;
    let nd8 = self.n/8;
    let m8 = self.m%8;
    let n8 = self.n%8;
    let mut ctx = amx::AmxCtx::new().unwrap();
    if nd8 == 0 {
      for i in 0..self.m {
        for j in 0..self.n {
          rc.data[(i*self.n+j)as usize] = self.data[(i*self.n+j)as usize]-rhs.data[(i*self.n+j)as usize];
        }
      }
    } else {
      for i in 0..self.m {
        for j in 0..nd8 {
          unsafe {
            ctx.load512(&self.data[(i*self.n+j*8) as usize],ZRow(0));
            ctx.load512(&rhs.data[(i*self.n+j*8) as usize],XRow(0));
            ctx.fms64_vec_xz(0,0);
            ctx.store512(&mut rc.data[(i*self.n+j*8) as usize],ZRow(0));
          }
        }
      }
      for i in 0..self.m {
        for j in nd8*8..self.n {
          rc.data[(i*self.n+j) as usize]= self.data[(i*self.n+j) as usize] - rhs.data[(i*self.n+j) as usize];
        }
      }
    }
    rc
  }
}
impl Add<&MatrixF64> for MatrixF64{
  type Output = MatrixF64;
  fn add(self,rhs:&Self)->Self::Output{
    assert!(self.m == rhs.m && self.n == rhs.n);
    let mut rc = MatrixF64::new(self.m,self.n);
    let md8 = self.m/8;
    let nd8 = self.n/8;
    let m8 = self.m%8;
    let n8 = self.n%8;
    let mut ctx = amx::AmxCtx::new().unwrap();
    if nd8 == 0 {
      for i in 0..self.m {
        for j in 0..self.n {
          rc.data[(i*self.n+j)as usize] = self.data[(i*self.n+j)as usize]+rhs.data[(i*self.n+j)as usize];
        }
      }
    } else {
      for i in 0..self.m {
        for j in 0..nd8 {
          unsafe {
            ctx.load512(&self.data[(i*self.n+j*8) as usize],ZRow(0));
            ctx.load512(&rhs.data[(i*self.n+j*8) as usize],XRow(0));
            ctx.fma64_vec_xz(0,0);
            ctx.store512(&mut rc.data[(i*self.n+j*8) as usize],ZRow(0));
          }
        }
      }
      for i in 0..self.m {
        for j in nd8*8..self.n {
          rc.data[(i*self.n+j) as usize]= self.data[(i*self.n+j) as usize] + rhs.data[(i*self.n+j) as usize];
        }
      }
    }
    rc
  }
}
impl Sub<&Matrix<f64>> for &MatrixF64{
  type Output = MatrixF64;
  fn sub(self,rhs:&Matrix<f64>)->Self::Output{
    assert!(self.m == rhs.m && self.n == rhs.n);
    let mut rc = MatrixF64::new(self.m,self.n);
    let md8 = self.m/8;
    let nd8 = self.n/8;
    let m8 = self.m%8;
    let n8 = self.n%8;
    let mut ctx = amx::AmxCtx::new().unwrap();
    if nd8 == 0 {
      for i in 0..self.m {
        for j in 0..self.n {
          rc.data[(i*self.n+j)as usize] = self.data[(i*self.n+j)as usize]-rhs.data[(i*self.n+j)as usize];
        }
      }
    } else {
      for i in 0..self.m {
        for j in 0..nd8 {
          unsafe {
            ctx.load512(&self.data[(i*self.n+j*8) as usize],ZRow(0));
            ctx.load512(&rhs.data[(i*self.n+j*8) as usize],XRow(0));
            ctx.fms64_vec_xz(0,0);
            ctx.store512(&mut rc.data[(i*self.n+j*8) as usize],ZRow(0));
          }
        }
      }
      for i in 0..self.m {
        for j in nd8*8..self.n {
          rc.data[(i*self.n+j) as usize]= self.data[(i*self.n+j) as usize] - rhs.data[(i*self.n+j) as usize];
        }
      }
    }
    rc
  }
}
impl Add<&Matrix<f64>> for &MatrixF64{
  type Output = MatrixF64;
  fn add(self,rhs:&Matrix<f64>)->Self::Output{
    assert!(self.m == rhs.m && self.n == rhs.n);
    let mut rc = MatrixF64::new(self.m,self.n);
    let md8 = self.m/8;
    let nd8 = self.n/8;
    let m8 = self.m%8;
    let n8 = self.n%8;
    let mut ctx = amx::AmxCtx::new().unwrap();
    if nd8 == 0 {
      for i in 0..self.m {
        for j in 0..self.n {
          rc.data[(i*self.n+j)as usize] = self.data[(i*self.n+j)as usize]+rhs.data[(i*self.n+j)as usize];
        }
      }
    } else {
      for i in 0..self.m {
        for j in 0..nd8 {
          unsafe {
            ctx.load512(&self.data[(i*self.n+j*8) as usize],ZRow(0));
            ctx.load512(&rhs.data[(i*self.n+j*8) as usize],XRow(0));
            ctx.fma64_vec_xz(0,0);
            ctx.store512(&mut rc.data[(i*self.n+j*8) as usize],ZRow(0));
          }
        }
      }
      for i in 0..self.m {
        for j in nd8*8..self.n {
          rc.data[(i*self.n+j) as usize]= self.data[(i*self.n+j) as usize] + rhs.data[(i*self.n+j) as usize];
        }
      }
    }
    rc
  }
}
impl Sub<&Matrix<f64>> for MatrixF64{
  type Output = Self;
  fn sub(self,rhs:&Matrix<f64>)->Self{
    assert!(self.m == rhs.m && self.n == rhs.n);
    let mut rc = MatrixF64::new(self.m,self.n);
    let md8 = self.m/8;
    let nd8 = self.n/8;
    let m8 = self.m%8;
    let n8 = self.n%8;
    let mut ctx = amx::AmxCtx::new().unwrap();
    if nd8 == 0 {
      for i in 0..self.m {
        for j in 0..self.n {
          rc.data[(i*self.n+j)as usize] = self.data[(i*self.n+j)as usize]-rhs.data[(i*self.n+j)as usize];
        }
      }
    } else {
      for i in 0..self.m {
        for j in 0..nd8 {
          unsafe {
            ctx.load512(&self.data[(i*self.n+j*8) as usize],ZRow(0));
            ctx.load512(&rhs.data[(i*self.n+j*8) as usize],XRow(0));
            ctx.fms64_vec_xz(0,0);
            ctx.store512(&mut rc.data[(i*self.n+j*8) as usize],ZRow(0));
          }
        }
      }
      for i in 0..self.m {
        for j in nd8*8..self.n {
          rc.data[(i*self.n+j) as usize]= self.data[(i*self.n+j) as usize] - rhs.data[(i*self.n+j) as usize];
        }
      }
    }
    rc
  }
}
impl Add<&Matrix<f64>> for MatrixF64{
  type Output = Self;
  fn add(self,rhs:&Matrix<f64>)->Self{
    assert!(self.m == rhs.m && self.n == rhs.n);
    let mut rc = MatrixF64::new(self.m,self.n);
    let md8 = self.m/8;
    let nd8 = self.n/8;
    let m8 = self.m%8;
    let n8 = self.n%8;
    let mut ctx = amx::AmxCtx::new().unwrap();
    if nd8 == 0 {
      for i in 0..self.m {
        for j in 0..self.n {
          rc.data[(i*self.n+j)as usize] = self.data[(i*self.n+j)as usize]+rhs.data[(i*self.n+j)as usize];
        }
      }
    } else {
      for i in 0..self.m {
        for j in 0..nd8 {
          unsafe {
            ctx.load512(&self.data[(i*self.n+j*8) as usize],ZRow(0));
            ctx.load512(&rhs.data[(i*self.n+j*8) as usize],XRow(0));
            ctx.fma64_vec_xz(0,0);
            ctx.store512(&mut rc.data[(i*self.n+j*8) as usize],ZRow(0));
          }
        }
      }
      for i in 0..self.m {
        for j in nd8*8..self.n {
          rc.data[(i*self.n+j) as usize]= self.data[(i*self.n+j) as usize] + rhs.data[(i*self.n+j) as usize];
        }
      }
    }
    rc
  }
}
