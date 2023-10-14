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
}
impl TMatrix<'_,f32> for &MatrixF32{
  fn m(&self)->u32{
    self.m
  }
  fn n(&self)->u32{
    self.n
  }
  fn get(&self,i:u32,j:u32)->&f32{
    &self.data[(i*self.n +j) as usize]
  }
}
impl TMatrixMut<'_,f32> for MatrixF32{
  fn get_mut(&mut self,i:u32,j:u32)->&mut f32{
    &mut self.data[(i*self.n +j) as usize]
  }
}
impl Sub for &MatrixF32{
  type Output = MatrixF32;
  fn sub(self,rhs:Self)->Self::Output{
    op::<'-'>(self,rhs)
  }
}
impl Add for &MatrixF32{
  type Output = MatrixF32;
  fn add(self,rhs:Self)->Self::Output{
    op::<'+'>(self,rhs)
  }
}
impl Mul for &MatrixF32{
  type Output = MatrixF32;
  fn mul(self,rhs:Self)->Self::Output{
    op::<'*'>(self,rhs)
  }
}
impl Sub<&MatrixF32> for MatrixF32{
  type Output = MatrixF32;
  fn sub(self,rhs:&Self)->Self::Output{
    op::<'-'>(self,rhs)
  }
}
impl Add<&MatrixF32> for MatrixF32{
  type Output = MatrixF32;
  fn add(self,rhs:&Self)->Self::Output{
    op::<'+'>(self,rhs)
  }
}
impl Mul<&MatrixF32> for MatrixF32{
  type Output = MatrixF32;
  fn mul(self,rhs:&Self)->Self::Output{
    op::<'*'>(self,rhs)
  }
}
impl Sub<&Matrix<f32>> for &MatrixF32{
  type Output = MatrixF32;
  fn sub(self,rhs:&Matrix<f32>)->Self::Output{
    op::<'-'>(self,rhs)
  }
}
impl Add<&Matrix<f32>> for &MatrixF32{
  type Output = MatrixF32;
  fn add(self,rhs:&Matrix<f32>)->Self::Output{
    op::<'+'>(self,rhs)
  }
}
impl Mul<&Matrix<f32>> for &MatrixF32{
  type Output = MatrixF32;
  fn mul(self,rhs:&Matrix<f32>)->Self::Output{
    op::<'*'>(self,rhs)
  }
}
impl Sub<&Matrix<f32>> for MatrixF32{
  type Output = Self;
  fn sub(self,rhs:&Matrix<f32>)->Self{
    op::<'-'>(self,rhs)
  }
}
impl Add<&Matrix<f32>> for MatrixF32{
  type Output = Self;
  fn add(self,rhs:&Matrix<f32>)->Self{
    op::<'+'>(self,rhs)
  }
}
impl Mul<&Matrix<f32>> for MatrixF32{
  type Output = Self;
  fn mul(self,rhs:&Matrix<f32>)->Self{
    op::<'*'>(self,rhs)
  }
}
fn op<'a,const OP:char>(lhs:impl TMatrix<'a, f32>,rhs:impl TMatrix<'a,f32>)->MatrixF32{
    if OP != '*' {
      assert!(lhs.m() == rhs.m() && lhs.n() == rhs.n());
    } else {
      assert!(lhs.n() == rhs.m())
    }
    let mut rc = if OP == '*' {MatrixF32::new(lhs.m(),rhs.n()) } else {MatrixF32::new(lhs.m(),lhs.n())};
    let _md8 = lhs.m()/16;
    let nd8 = lhs.n()/16;
    let _m8 = lhs.m()%16;
    let _n8 = lhs.n()%16;
    let rnd8 = rhs.n()/16;
    let _rmd8 = rhs.m()/16;
    let mut ctx = amx::AmxCtx::new().unwrap();
    if nd8 == 0 && OP != '*' {
      match OP {
        '+'|'-' =>
          {
            for i in 0..lhs.m() {
              for j in 0..lhs.n() {
                match OP {
                 '+' => {*rc.get_mut(i,j) = lhs.get(i,j)+rhs.get(i,j);}
                 '-' => {*rc.get_mut(i,j) = lhs.get(i,j)-rhs.get(i,j);}
                 _ => {}
                }
              }
            }
          }
          '*' => {
            for i in 0..lhs.m() {
              for j in 0..rhs.n() {
                let mut sum = 0.0;
                for k in 0..rhs.m() {
                  sum = sum + *lhs.get(i,k) * *rhs.get(k,j);
                }
                *rc.get_mut(i,j) = sum;
              }
            }
          }
          _ => {}
      }
    } else {
      unsafe {
        match OP {
          '+'|'-' => {
            for i in 0..lhs.m() {
              for j in 0..nd8 {
                ctx.load512(lhs.get(i,j*16),ZRow(0));
                ctx.load512(rhs.get(i,j*16),XRow(0));
                match OP {
                  '+' => { ctx.fma32_vec_xz(0,0); }
                  '-' => { ctx.fms32_vec_xz(0,0); }
                  _ => {}
                }
                ctx.store512(rc.get_mut(i,j*16),ZRow(0));
              }
            }
            for i in 0..lhs.m() {
              for j in nd8*16..lhs.n() {
                match OP {
                  '+' => { *rc.get_mut(i,j) = lhs.get(i,j) + rhs.get(i,j); }
                  '-' => { *rc.get_mut(i,j) = lhs.get(i,j) - rhs.get(i,j); }
                  _ => {}
                }
              }
            }
          }
          '*' => {
            let tmp = [0.0f32;16];
            for i in 0..lhs.m() {
              for j in 0..rnd8 {
                ctx.load512(&tmp,ZRow(1));
                ctx.load512(rhs.get(i,j*16),XRow(0));
                for k in 0..rhs.m() {
                  ctx.load512(lhs.get(k,j*16),YRow(0));
                  ctx.fma32_vec_xy(0,0,0,0);
                  ctx.extr_xh(0,1);
                  ctx.fma32_vec_xz(1,1);
                }
                ctx.store512(rc.get_mut(i,j*16),ZRow(1));
              }
            }
            for i in 0..lhs.m() {
              for j in nd8*16..rhs.n() {
                let mut sum =0.0;
                for k in 0 ..rhs.m() {
                  sum += *lhs.get(i,k) * *rhs.get(k,j);
                }
                *rc.get_mut(i,j) += sum;
              }
            }
          }
          _ => {}
        }
      }
    }
    rc
}
