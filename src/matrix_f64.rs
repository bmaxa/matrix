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
impl Mul for &MatrixF64{
  type Output = MatrixF64;
  fn mul(self,rhs:Self)->Self::Output{
    op::<'*'>(self,rhs)
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
impl Mul<&MatrixF64> for MatrixF64{
  type Output = MatrixF64;
  fn mul(self,rhs:&Self)->Self::Output{
    op::<'*'>(self,rhs)
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
impl Mul<&Matrix<f64>> for &MatrixF64{
  type Output = MatrixF64;
  fn mul(self,rhs:&Matrix<f64>)->Self::Output{
    op::<'*'>(self,rhs)
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
impl Mul<&Matrix<f64>> for MatrixF64{
  type Output = Self;
  fn mul(self,rhs:&Matrix<f64>)->Self{
    op::<'*'>(self,rhs)
  }
}
fn op<'a,const OP:char>(lhs:impl TMatrix<'a, f64>,rhs:impl TMatrix<'a,f64>)->MatrixF64{
    if OP != '*' {
      assert!(lhs.m() == rhs.m() && lhs.n() == rhs.n());
    } else {
      assert!(lhs.n() == rhs.m())
    }
    let mut rc = if OP == '*' {MatrixF64::new(lhs.m(),rhs.n()) } else {MatrixF64::new(lhs.m(),lhs.n())};
    let _md8 = lhs.m()/8;
    let nd8 = lhs.n()/8;
    let _m8 = lhs.m()%8;
    let _n8 = lhs.n()%8;
    let rnd8 = rhs.n()/8;
    let rn8 = rhs.n()%8;
    let _rm8 = rhs.m()%8;
    let _rmd8 = rhs.m()/8;
    let mut ctx = amx::AmxCtx::new().unwrap();
    if nd8 == 0 {
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
                for k in 0..rhs.m() {
                  *rc.get_mut(i,j) += *lhs.get(i,k) * *rhs.get(k,j);
                }
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
          '*' => {
            let tmp = [0.0f64;8];
            for i in 0..lhs.m() {
              for j in 0..rnd8 {
                let mut m = 0;
                let mut chunk = 0;
                ctx.load512(&tmp,ZRow(1));
                for k in 0..rhs.m() {
                  if m == 0 {
                    if chunk >= rnd8*8+rn8 {
                      break
                    }
                    ctx.load512(lhs.get(i,chunk),XRow(0));
                    ctx.extr_yx(0,1);
                    ctx.fma64_mat_y(0,1);
                    chunk += 8;
                  }
                  ctx.load512(rhs.get(k,j*8),YRow(0));
                  ctx.extr_xh(m*8,0);
                  ctx.fma64_vec_xy(2,0,0,0);
                  ctx.extr_xh(2,0);
                  ctx.fma64_vec_xz(1,0);
                  m += 1;
                  if m == 8 { m=0; }
                }
                ctx.store512(rc.get_mut(i,j*8), ZRow(1));
              }
              for j in rnd8*8..rhs.n() {
                for k in 0..rhs.m() {
                  *rc.get_mut(i,j) += *lhs.get(i,k) * *rhs.get(k,j);
                }
              }
            }
          }
          _ => {}
        }
      }
    }
    rc
}
