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
impl Sub for &MatrixF32{
  type Output = MatrixF32;
  fn sub(self,rhs:Self)->Self::Output{
    assert!(self.m == rhs.m && self.n == rhs.n);
    let mut rc = MatrixF32::new(self.m,self.n);
    let md8 = self.m/16;
    let nd8 = self.n/16;
    let m8 = self.m%16;
    let n8 = self.n%16;
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
            ctx.load512(&self.data[(i*self.n+j*16) as usize],ZRow(0));
            ctx.load512(&rhs.data[(i*self.n+j*16) as usize],XRow(0));
            ctx.fms32_vec_xz(0,0);
            ctx.store512(&mut rc.data[(i*self.n+j*16) as usize],ZRow(0));
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
impl Add for &MatrixF32{
  type Output = MatrixF32;
  fn add(self,rhs:Self)->Self::Output{
    assert!(self.m == rhs.m && self.n == rhs.n);
    let mut rc = MatrixF32::new(self.m,self.n);
    let md8 = self.m/16;
    let nd8 = self.n/16;
    let m8 = self.m%16;
    let n8 = self.n%16;
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
            ctx.load512(&self.data[(i*self.n+j*16) as usize],ZRow(0));
            ctx.load512(&rhs.data[(i*self.n+j*16) as usize],XRow(0));
            ctx.fma32_vec_xz(0,0);
            ctx.store512(&mut rc.data[(i*self.n+j*16) as usize],ZRow(0));
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
impl Sub<&MatrixF32> for MatrixF32{
  type Output = MatrixF32;
  fn sub(self,rhs:&Self)->Self::Output{
    assert!(self.m == rhs.m && self.n == rhs.n);
    let mut rc = MatrixF32::new(self.m,self.n);
    let md8 = self.m/16;
    let nd8 = self.n/16;
    let m8 = self.m%16;
    let n8 = self.n%16;
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
            ctx.load512(&self.data[(i*self.n+j*16) as usize],ZRow(0));
            ctx.load512(&rhs.data[(i*self.n+j*16) as usize],XRow(0));
            ctx.fms32_vec_xz(0,0);
            ctx.store512(&mut rc.data[(i*self.n+j*16) as usize],ZRow(0));
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
impl Add<&MatrixF32> for MatrixF32{
  type Output = MatrixF32;
  fn add(self,rhs:&Self)->Self::Output{
    assert!(self.m == rhs.m && self.n == rhs.n);
    let mut rc = MatrixF32::new(self.m,self.n);
    let md8 = self.m/16;
    let nd8 = self.n/16;
    let m8 = self.m%16;
    let n8 = self.n%16;
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
            ctx.load512(&self.data[(i*self.n+j*16) as usize],ZRow(0));
            ctx.load512(&rhs.data[(i*self.n+j*16) as usize],XRow(0));
            ctx.fma32_vec_xz(0,0);
            ctx.store512(&mut rc.data[(i*self.n+j*16) as usize],ZRow(0));
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
impl Sub<&Matrix<f32>> for &MatrixF32{
  type Output = MatrixF32;
  fn sub(self,rhs:&Matrix<f32>)->Self::Output{
    assert!(self.m == rhs.m && self.n == rhs.n);
    let mut rc = MatrixF32::new(self.m,self.n);
    let md8 = self.m/16;
    let nd8 = self.n/16;
    let m8 = self.m%16;
    let n8 = self.n%16;
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
            ctx.load512(&self.data[(i*self.n+j*16) as usize],ZRow(0));
            ctx.load512(&rhs.data[(i*self.n+j*16) as usize],XRow(0));
            ctx.fms32_vec_xz(0,0);
            ctx.store512(&mut rc.data[(i*self.n+j*16) as usize],ZRow(0));
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
impl Add<&Matrix<f32>> for &MatrixF32{
  type Output = MatrixF32;
  fn add(self,rhs:&Matrix<f32>)->Self::Output{
    assert!(self.m == rhs.m && self.n == rhs.n);
    let mut rc = MatrixF32::new(self.m,self.n);
    let md8 = self.m/16;
    let nd8 = self.n/16;
    let m8 = self.m%16;
    let n8 = self.n%16;
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
            ctx.load512(&self.data[(i*self.n+j*16) as usize],ZRow(0));
            ctx.load512(&rhs.data[(i*self.n+j*16) as usize],XRow(0));
            ctx.fma32_vec_xz(0,0);
            ctx.store512(&mut rc.data[(i*self.n+j*16) as usize],ZRow(0));
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
impl Sub<&Matrix<f32>> for MatrixF32{
  type Output = Self;
  fn sub(self,rhs:&Matrix<f32>)->Self{
    assert!(self.m == rhs.m && self.n == rhs.n);
    let mut rc = MatrixF32::new(self.m,self.n);
    let md8 = self.m/16;
    let nd8 = self.n/16;
    let m8 = self.m%16;
    let n8 = self.n%16;
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
            ctx.load512(&self.data[(i*self.n+j*16) as usize],ZRow(0));
            ctx.load512(&rhs.data[(i*self.n+j*16) as usize],XRow(0));
            ctx.fms32_vec_xz(0,0);
            ctx.store512(&mut rc.data[(i*self.n+j*16) as usize],ZRow(0));
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
impl Add<&Matrix<f32>> for MatrixF32{
  type Output = Self;
  fn add(self,rhs:&Matrix<f32>)->Self{
    assert!(self.m == rhs.m && self.n == rhs.n);
    let mut rc = MatrixF32::new(self.m,self.n);
    let md8 = self.m/16;
    let nd8 = self.n/16;
    let m8 = self.m%16;
    let n8 = self.n%16;
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
            ctx.load512(&self.data[(i*self.n+j*16) as usize],ZRow(0));
            ctx.load512(&rhs.data[(i*self.n+j*16) as usize],XRow(0));
            ctx.fma32_vec_xz(0,0);
            ctx.store512(&mut rc.data[(i*self.n+j*16) as usize],ZRow(0));
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
