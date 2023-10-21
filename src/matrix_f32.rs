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
  type Output = MatrixF32;
  fn m(&self)->u32{
    self.m
  }
  fn n(&self)->u32{
    self.n
  }
  fn get(&self,i:u32,j:u32)->&f32{
    &self.data[(i*self.n +j) as usize]
  }
  fn det(&self)->f32{
    det(self)
  }
  fn inv(&self)->MatrixF32{
    inv(self)
  }
}
impl TMatrix<'_,f32> for &MatrixF32{
  type Output = MatrixF32;
  fn m(&self)->u32{
    self.m
  }
  fn n(&self)->u32{
    self.n
  }
  fn get(&self,i:u32,j:u32)->&f32{
    &self.data[(i*self.n +j) as usize]
  }
  fn det(&self)->f32{
    det(*self)
  }
  fn inv(&self)->MatrixF32{
    inv(*self)
  }
}
impl TMatrixMut<'_,f32> for MatrixF32{
  fn get_mut(&mut self,i:u32,j:u32)->&mut f32{
    &mut self.data[(i*self.n +j) as usize]
  }
}
impl Sub for MatrixF32{
  type Output = MatrixF32;
  fn sub(self,rhs:Self)->Self::Output{
    op::<'-'>(self,rhs)
  }
}
impl Add for MatrixF32{
  type Output = MatrixF32;
  fn add(self,rhs:Self)->Self::Output{
    op::<'+'>(self,rhs)
  }
}
impl Mul for MatrixF32{
  type Output = MatrixF32;
  fn mul(self,rhs:Self)->Self::Output{
    op::<'*'>(self,rhs)
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
    let rn8 = rhs.n()%16;
    let _rm8 = rhs.m()%16;
    let _rmd8 = rhs.m()/16;
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
                let mut m = 0;
                let mut chunk = 0;
                ctx.load512(&tmp,ZRow(1));
                for k in 0..rhs.m() {
                  if m == 0 {
                    if chunk >= rnd8*16+rn8 {
                      break
                    }
                    ctx.load512(lhs.get(i,chunk),XRow(0));
                    ctx.extr_yx(0,1);
                    ctx.fma32_mat_y(0,1);
                    chunk += 16;
                  }
                  ctx.load512(rhs.get(k,j*16),YRow(0));
                  ctx.extr_xh(m*4,0);
                  ctx.fma32_vec_xy(2,0,0,0);
                  ctx.extr_xh(2,0);
                  ctx.fma32_vec_xz(1,0);
                  m += 1;
                  if m == 16 { m=0; }
                }
                ctx.store512(rc.get_mut(i,j*16), ZRow(1));
              }
              for j in rnd8*16..rhs.n() {
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
fn det<'a>(m:impl TMatrix<'a, f32>)->f32{
  if m.m() != m.n() { return 0.0 }
  let mut ctx = amx::AmxCtx::new().unwrap();
  let mut tmp = Matrix::<f32>::new(m.m(),m.n());
  let nd8 = m.n()/16;
  let _n8 = m.n()%16;
  for i in 0..m.m() {
    for j in 0..nd8 {
      unsafe {
        ctx.load512(m.get(i,j*16),XRow(0));
        ctx.store512(tmp.get_mut(i,j*16),XRow(0));
      }
    }
    for j in nd8*16..m.n() {
      *tmp.get_mut(i,j) = *m.get(i,j);
    }
  }
  let mut d = 1.0;
  for i in 0..tmp.m()-1 {
    for k in i+1..tmp.m() {
      let mut row = i;
      if *tmp.get(i,i) == 0.0 {
        for l in k..tmp.m() {
          if *tmp.get(l,i) != 0.0 {
            row = l;
            break
          }
        }
        if row != i {
          d = -d;
          for j in 0..nd8 {
            unsafe {
              ctx.load512(tmp.get(i,j*16),XRow(0));
              ctx.load512(tmp.get(row,j*16),XRow(1));
              ctx.store512(tmp.get_mut(i,j*16),XRow(1));
              ctx.store512(tmp.get_mut(row,j*16),XRow(0));
            }
          }
          for j in nd8*16..tmp.n() {
            let t = *tmp.get(i,j);
            *tmp.get_mut(i,j) = *tmp.get(row,j);
            *tmp.get_mut(row,j) = t;
          }
        } else { return 0.0 }
      }
      if *tmp.get(k,i) == 0.0 { continue }
      let mult = - *tmp.get(k,i) / *tmp.get(i,i);
      let mult8 = [mult;16];
      unsafe { ctx.load512(&mult8,YRow(0)); }
      for j in i/16..(tmp.n()-i)/16 {
        unsafe {
          ctx.load512(tmp.get(k,j*16),ZRow(0));
          ctx.load512(tmp.get(i,j*16),XRow(0));
          ctx.fma32_vec(0,0,0,0);
          ctx.store512(tmp.get_mut(k,j*16),ZRow(0));
        }
      }
      for j in (tmp.n()-i)/16*16..tmp.n(){
        *tmp.get_mut(k,j) = *tmp.get(k,j) + *tmp.get(i,j)*mult;
      }
    }
  }
  for i in 0..tmp.m() {
    d = d * *tmp.get(i,i);
  }
  d
}
fn inv<'a>(m:impl TMatrix<'a, f32>)->MatrixF32{
  MatrixF32::new(m.m(),m.n())
}
