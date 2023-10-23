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
  type Output = MatrixF64;
  fn m(&self)->u32{
    self.m
  }
  fn n(&self)->u32{
    self.n
  }
  fn get(&self,i:u32,j:u32)->&f64{
    &self.data[(i*self.n +j) as usize]
  }
  fn det(&self)->f64{
    det(self)
  }
  fn inv(&self)->MatrixF64{
    inv(self)
  }
}
impl TMatrix<'_,f64> for &MatrixF64{
  type Output = MatrixF64;
  fn m(&self)->u32{
    self.m
  }
  fn n(&self)->u32{
    self.n
  }
  fn get(&self,i:u32,j:u32)->&f64{
    &self.data[(i*self.n +j) as usize]
  }
  fn det(&self)->f64{
    det(*self)
  }
  fn inv(&self)->MatrixF64{
    inv(*self)
  }
}
impl TMatrixMut<'_,f64> for MatrixF64{
  fn get_mut(&mut self,i:u32,j:u32)->&mut f64{
    &mut self.data[(i*self.n +j) as usize]
  }
}
impl Mul<f64> for MatrixF64 {
  type Output = MatrixF64;
  fn mul(self,rhs:f64)->MatrixF64{
    let mut rc = MatrixF64::new(self.m(),self.n());
    let mult8 = [rhs;8];
    let nd8 = self.n()/8;
    let mut ctx = amx::AmxCtx::new().unwrap();
    unsafe {ctx.load512(&mult8,YRow(0))};
    for i in 0..self.m() {
      for j in 0..nd8 {
        unsafe {
          ctx.load512(self.get(i,j*8),XRow(0));
          ctx.fma64_vec_xy(0,0,0,0);
          ctx.store512(rc.get_mut(i,j*8),ZRow(0));
        }
      }
      for j in nd8*8..self.n(){
        *rc.get_mut(i,j) = *self.get(i,j) * rhs;
      }
    }
    rc
  }
}
impl Sub for MatrixF64{
  type Output = MatrixF64;
  fn sub(self,rhs:Self)->Self::Output{
    op::<'-'>(self,rhs)
  }
}
impl Add for MatrixF64{
  type Output = MatrixF64;
  fn add(self,rhs:Self)->Self::Output{
    op::<'+'>(self,rhs)
  }
}
impl Mul for MatrixF64{
  type Output = MatrixF64;
  fn mul(self,rhs:Self)->Self::Output{
    op::<'*'>(self,rhs)
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
fn det<'a>(m:impl TMatrix<'a, f64>)->f64{
  if m.m() != m.n() { return 0.0 }
  let mut ctx = amx::AmxCtx::new().unwrap();
  let mut tmp = Matrix::<f64>::new(m.m(),m.n());
  let nd8 = m.n()/8;
  let _n8 = m.n()%8;
  for i in 0..m.m() {
    for j in 0..nd8 {
      unsafe {
        ctx.load512(m.get(i,j*8),XRow(0));
        ctx.store512(tmp.get_mut(i,j*8),XRow(0));
      }
    }
    for j in nd8*8..m.n() {
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
              ctx.load512(tmp.get(i,j*8),XRow(0));
              ctx.load512(tmp.get(row,j*8),XRow(1));
              ctx.store512(tmp.get_mut(i,j*8),XRow(1));
              ctx.store512(tmp.get_mut(row,j*8),XRow(0));
            }
          }
          for j in nd8*8..tmp.n() {
            let t = *tmp.get(i,j);
            *tmp.get_mut(i,j) = *tmp.get(row,j);
            *tmp.get_mut(row,j) = t;
          }
        } else { return 0.0 }
      }
      if *tmp.get(k,i) == 0.0 { continue }
      let mult = - *tmp.get(k,i) / *tmp.get(i,i);
      let mult8 = [mult;8];
      unsafe { ctx.load512(&mult8,YRow(0)); }
      let mut r = i;
      if tmp.n() - i >= 8 {
        for j in (i..tmp.n()).step_by(8) {
          if tmp.n() -j >= 8 {
            unsafe {
              ctx.load512(tmp.get(k,j),ZRow(0));
              ctx.load512(tmp.get(i,j),XRow(0));
              ctx.fma64_vec(0,0,0,0);
              ctx.store512(tmp.get_mut(k,j),ZRow(0));
            }
            r = j + 8;
          }
        }
      }
      for j in r..tmp.n(){
        *tmp.get_mut(k,j) = *tmp.get(k,j) + *tmp.get(i,j)*mult;
      }
    }
  }
  for i in 0..tmp.m() {
    d = d * *tmp.get(i,i);
  }
  d
}
fn inv<'a>(m:impl TMatrix<'a, f64>)->MatrixF64{
  let nd8 = m.n()/8;
  let mut tmp = MatrixF64::new(m.m(),m.n());
  if m.m() != m.n() { return tmp }
  let mut rc = MatrixF64::new(m.m(),m.n());
  let mut ctx = amx::AmxCtx::new().unwrap();
  for i in 0..rc.m() {
    *rc.get_mut(i,i) = 1.0;
  }
  for i in 0..tmp.m(){
    for j in 0..nd8 {
      unsafe {
        ctx.load512(m.get(i,j*8),XRow(0));
        ctx.store512(tmp.get_mut(i,j*8),XRow(0));
      }
    }
    for j in nd8*8..tmp.n() {
      *tmp.get_mut(i,j) = *m.get(i,j);
    }
  }
  for i in 0..tmp.m()-1 {
    for k in i+1..tmp.m() {
      let mut row = i;
      if *tmp.get(i,i) == 0.0 {
        for l in k..tmp.m() {
          if *tmp.get(l,i) != 0.0{
            row = l;
            break
          }
        }
        if row != i {
          for j in 0..nd8 {
            unsafe {
              ctx.load512(tmp.get(i,j*8),XRow(0));
              ctx.load512(tmp.get(row,j*8),XRow(1));
              ctx.store512(tmp.get_mut(i,j*8),XRow(1));
              ctx.store512(tmp.get_mut(row,j*8),XRow(0));
              ctx.load512(rc.get(i,j*8),XRow(0));
              ctx.load512(rc.get(row,j*8),XRow(1));
              ctx.store512(rc.get_mut(i,j*8),XRow(1));
              ctx.store512(rc.get_mut(row,j*8),XRow(0));
            }
          }
          for j in nd8*8..tmp.n() {
            let t = *tmp.get(i,j);
            let t1 = *rc.get(i,j);
            *tmp.get_mut(i,j) = *tmp.get(row,j);
            *rc.get_mut(i,j) = *rc.get(row,j);
            *tmp.get_mut(row,j) = t;
            *rc.get_mut(row,j) = t1;
          }
        } else { return rc }
      }
      if *tmp.get(k,i) == 0.0 { continue }
      let mult = - *tmp.get(k,i) / *tmp.get(i,i);
      let mult8 = [mult;8];
      unsafe {ctx.load512(&mult8,YRow(0))};
      let mut r = i;
      if tmp.n() - i >= 8 {
        for j in (i..tmp.n()).step_by(8){
          if tmp.n() -j >= 8 {
            unsafe{
              ctx.load512(tmp.get(k,j),ZRow(0));
              ctx.load512(tmp.get(i,j),XRow(0));
              ctx.fma64_vec(0,0,0,0);
              ctx.store512(tmp.get_mut(k,j),ZRow(0));
              ctx.load512(rc.get(k,j),ZRow(0));
              ctx.load512(rc.get(i,j),XRow(0));
              ctx.fma64_vec(0,0,0,0);
              ctx.store512(rc.get_mut(k,j),ZRow(0));
            }
            r = j + 8;
          }
        }
      }
      for j in r..tmp.n(){
        *tmp.get_mut(k,j) = *tmp.get(k,j) + *tmp.get(i,j)*mult;
        *rc.get_mut(k,j) = *rc.get(k,j) + *rc.get(i,j)*mult;
      }
    }
  }
  for i in (1..tmp.m()).rev() {
    for k in (0 ..i).rev() {
      let mut row = i;
      if *tmp.get(i,i) == 0.0 {
        for l in (0..k).rev() {
          if *tmp.get(l,i) != 0.0{
            row = l;
            break
          }
        }
        if row != i {
          for j in 0..nd8 {
            unsafe {
              ctx.load512(tmp.get(i,j*8),XRow(0));
              ctx.load512(tmp.get(row,j*8),XRow(1));
              ctx.store512(tmp.get_mut(i,j*8),XRow(1));
              ctx.store512(tmp.get_mut(row,j*8),XRow(0));
              ctx.load512(rc.get(i,j*8),XRow(0));
              ctx.load512(rc.get(row,j*8),XRow(1));
              ctx.store512(rc.get_mut(i,j*8),XRow(1));
              ctx.store512(rc.get_mut(row,j*8),XRow(0));
            }
          }
          for j in nd8*8..tmp.n() {
            let t = *tmp.get(i,j);
            let t1 = *rc.get(i,j);
            *tmp.get_mut(i,j) = *tmp.get(row,j);
            *rc.get_mut(i,j) = *rc.get(row,j);
            *tmp.get_mut(row,j) = t;
            *rc.get_mut(row,j) = t1;
          }
        } else { return rc }
      }
      if *tmp.get(k,i) == 0.0 { continue }
      let mult = - *tmp.get(k,i) / *tmp.get(i,i);
      let mult8 = [mult;8];
      unsafe {ctx.load512(&mult8,YRow(0))};
      let mut r = 0;
      if i >= 8 {
        for j in (0..i).step_by(8){
          if i - j >= 8 {
            unsafe{
              ctx.load512(tmp.get(k,j),ZRow(0));
              ctx.load512(tmp.get(i,j),XRow(0));
              ctx.fma64_vec(0,0,0,0);
              ctx.store512(tmp.get_mut(k,j),ZRow(0));
              ctx.load512(rc.get(k,j),ZRow(0));
              ctx.load512(rc.get(i,j),XRow(0));
              ctx.fma64_vec(0,0,0,0);
              ctx.store512(rc.get_mut(k,j),ZRow(0));
            }
            r = j + 8;
          }
        }
      }
      for j in (r..i+1).rev(){
        *tmp.get_mut(k,j) = *tmp.get(k,j) + *tmp.get(i,j)*mult;
        *rc.get_mut(k,j) = *rc.get(k,j) + *rc.get(i,j)*mult;
      }
    }
  }
  for i in 0..tmp.m() {
    let mult = 1.0 / *tmp.get(i,i);
    let mult8 = [mult;8];
    unsafe { ctx.load512(&mult8,YRow(0)) };
    for j in 0..nd8 {
      unsafe {
        ctx.load512(rc.get(i,j*8),XRow(0));
        ctx.fma64_vec_xy(0,0,0,0);
        ctx.store512(rc.get_mut(i,j*8),ZRow(0));
      }
    }
    for j in nd8*8..tmp.n() {
      *rc.get_mut(i,j) = *rc.get(i,j) * mult;
    }
  }
  rc
}
