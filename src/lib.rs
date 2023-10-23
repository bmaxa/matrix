use std::ops::*;
use std::fmt::Debug;
use num::*;
pub mod matrix_f64;
pub mod matrix_f32;
pub struct Matrix<T:Num+Clone+Default+One>{
  m:u32,
  n:u32,
  data: Vec<T>
}
pub trait TMatrix<'a,T:Num+ Neg<Output=T>+One+Clone+Default+'a>{
  type Output;
  fn m(&self)->u32;
  fn n(&self)->u32;
  fn get(&self,i:u32,j:u32)->&T;
  fn det(&self)->T;
  fn inv(&self)-><Self as TMatrix<'a,T>>::Output;
}
pub trait TMatrixMut<'a,T:Num+One+Neg<Output=T>+Clone+Default+'a>:TMatrix<'a,T>{
  fn get_mut(&mut self,i:u32,j:u32)->&mut T;
}
impl <T:Num+Default+Debug+One+Clone> Matrix<T> {
  pub fn new(m:u32,n:u32)->Self {
    assert!(m>0 && n>0);
    let mut v = Vec::new();
    v.resize((m*n) as usize,T::default());
    Self{m:m,n:n,data:v}
  }
}
impl<'a,T:Num+One+Neg<Output=T>+Debug+Clone+Copy+Default+'a> TMatrix<'a,T> for Matrix<T>{
  type Output = Matrix<T>;
  fn m(&self)->u32{
    self.m
  }
  fn n(&self)->u32{
    self.n
  }
  fn get(&self,i:u32,j:u32)->&T{
    &self.data[(i*self.n +j) as usize]
  }
  fn det(&self)->T{
    det(self)
  }
  fn inv(&self)->Matrix<T>{
    inv(self)
  }
}
impl<'a,T:Num+One+Neg<Output=T>+Debug+Clone+Copy+Default+'a> TMatrix<'a,T> for &'a Matrix<T>{
  type Output = Matrix<T>;
  fn m(&self)->u32{
    self.m
  }
  fn n(&self)->u32{
    self.n
  }
  fn get(&self,i:u32,j:u32)->&T{
    &self.data[(i*self.n +j) as usize]
  }
  fn det(&self)->T{
    det(*self)
  }
  fn inv(&self)->Matrix<T>{
    inv(*self)
  }
}
impl<'a,T:Num+Neg<Output=T>+Clone+Debug+Copy+Default+One+'a> TMatrixMut<'a,T> for Matrix<T>{
  fn get_mut(&mut self,i:u32,j:u32)->&mut T{
    &mut self.data[(i*self.n +j) as usize]
  }
}
impl<'a,T:Num+Debug+Neg<Output=T>+Clone+Copy+Default> Mul<T> for Matrix<T>{
  type Output = Matrix<T>;
  fn mul(self,rhs:T)->Matrix<T>{
    let mut rc = Matrix::<T>::new(self.m(),self.n());
    for i in 0..self.m() {
      for j in 0..self.n() {
        *rc.get_mut(i,j) = rhs * *self.get(i,j);
      }
    }
    rc
  }
}
impl <T:Num+Neg<Output=T>+Clone+Debug+Copy+Default+One> Add for Matrix<T>{
  type Output = Matrix<T>;
  fn add(self,rhs:Self)->Self::Output{
    op::<'+',T>(self,rhs)
  }
}
impl <T:Num+Neg<Output=T>+Debug+Clone+Copy+Default+One> Mul for Matrix<T>{
  type Output = Matrix<T>;
  fn mul(self,rhs:Self)->Self::Output{
    op::<'*',T>(self,rhs)
  }
}
impl <T:Num+Neg<Output=T>+Clone+Debug+Default+Copy+One> Sub for Matrix<T>{
  type Output = Matrix<T>;
  fn sub(self,rhs:Self)->Self::Output{
    op::<'-',T>(self,rhs)
  }
}
impl <'a,T:Num+Neg<Output=T>+Clone+Debug+Copy+Default+One> Add for &'a Matrix<T>{
  type Output = Matrix<T>;
  fn add(self,rhs:Self)->Self::Output{
    op::<'+',T>(self,rhs)
  }
}
impl <'a,T:Num+Neg<Output=T>+Clone+Debug+Copy+Default+One> Mul for &'a Matrix<T>{
  type Output = Matrix<T>;
  fn mul(self,rhs:Self)->Self::Output{
    op::<'*',T>(self,rhs)
  }
}
impl <'a,T:Num+ Sub<Output=T>+Mul<Output=T>+Neg<Output=T>+PartialEq+
  Div<Output=T>+Clone+Debug+Default+Copy+One + Add<Output=T>> Sub for &'a Matrix<T>{
  type Output = Matrix<T>;
  fn sub(self,rhs:Self)->Self::Output{
    op::<'-',T>(self,rhs)
  }
}
impl <T:Num+Neg<Output=T>+Clone+Debug+Copy+Default+One> Add<&Matrix<T>> for Matrix<T>{
  type Output = Self;
  fn add(self,rhs:&Self)->Self{
    op::<'+',T>(self,rhs)
  }
}
impl <T:Num+Neg<Output=T>+Clone+Debug+Copy+Default+One> Mul<&Matrix<T>> for Matrix<T>{
  type Output = Self;
  fn mul(self,rhs:&Self)->Self{
    op::<'*',T>(self,rhs)
  }
}
impl <'a,T:Num+Neg<Output=T>+One+Debug+Clone+Copy+Default> Sub<&Matrix<T>> for Matrix<T>{
  type Output = Self;
  fn sub(self,rhs:&Self)->Self{
    op::<'-',T>(self,rhs)
  }
}
fn op<'a,const OP:char,T>(lhs:impl TMatrix<'a,T>,rhs:impl TMatrix<'a,T>)->Matrix<T>
where
  T:Num,
  T:Default+'a,
  T:One,
  T:Clone,
  T:Copy,
  T:Debug,
  T:Neg<Output=T>
{
    if OP != '*' {
      assert!(lhs.m() == rhs.m() && lhs.n() == rhs.n());
    } else {
      assert!(lhs.n() == rhs.m());
    }
    let mut rc = if OP == '*' { Matrix::<T>::new(lhs.m(),rhs.n())} else {Matrix::<T>::new(lhs.m(),lhs.n())};
    match OP {
      '+'|'-' => {
        for i in 0..lhs.m() {
          for j in 0..lhs.n() {
            match OP {
              '+' => {*rc.get_mut(i,j) = *lhs.get(i,j)+*rhs.get(i,j);}
              '-' => {*rc.get_mut(i,j) = *lhs.get(i,j)-*rhs.get(i,j);}
              _ => {}
            }
          }
        }
      }
      '*' => {
        for i in 0..lhs.m() {
          for j in 0..rhs.n() {
            let mut sum = T::default();
            for k in 0..rhs.m() {
              sum = sum + *lhs.get(i,k) * *rhs.get(k,j);
            }
            *rc.get_mut(i,j) = sum;
          }
        }
      }
      _ => {}
    }
    rc
}
fn det<'a,T>(m:impl TMatrix<'a,T>)->T
where
T:Num,
T:Default+'a,
T:One,
T:Clone,
T:Copy,
T:Debug,
T:Neg<Output=T>{
  if m.m() != m.n() { return T::default(); }
  let mut tmp = Matrix::<T>::new(m.m(),m.n());
  for i in 0..m.m(){
    for j in 0..m.n() {
      *tmp.get_mut(i,j) = *m.get(i,j);
    }
  }
  let mut d = T::one();
  for i in 0..tmp.m()-1 {
    for k in i+1..tmp.m() {
      let mut row = i;
      if *tmp.get(i,i) == T::default() {
        for l in k..tmp.m() {
          if *tmp.get(l,i) != T::default(){
            row = l;
            break
          }
        }
        if row != i {
          d = - d;
          for j in 0..tmp.n() {
            let t = *tmp.get(i,j);
            *tmp.get_mut(i,j) = *tmp.get(row,j);
            *tmp.get_mut(row,j) = t;
          }
        } else { return T::default() }
      }
      if *tmp.get(k,i) == T::default() { continue }
      let mult = - *tmp.get(k,i) / *tmp.get(i,i);
      for j in i..tmp.n(){
        *tmp.get_mut(k,j) = *tmp.get(k,j) + *tmp.get(i,j)*mult;
      }
    }
  }
  for i in 0..tmp.m() {
    d = d * *tmp.get(i,i);
  }
  d
}
fn inv<'a,T>(m:impl TMatrix<'a,T>)->Matrix<T>
where
T:Num,
T:Default+'a,
T:One,
T:Clone,
T:Copy,
T:Debug,
T:Neg<Output=T>{
  let mut tmp = Matrix::<T>::new(m.m(),m.n());
  if m.m() != m.n() { return tmp }
  let mut rc = Matrix::<T>::new(m.m(),m.n());
  for i in 0..rc.m() {
    *rc.get_mut(i,i) = T::one();
  }
  for i in 0..m.m(){
    for j in 0..m.n() {
      *tmp.get_mut(i,j) = *m.get(i,j);
    }
  }
  for i in 0..tmp.m()-1 {
    for k in i+1..tmp.m() {
      let mut row = i;
      if *tmp.get(i,i) == T::default() {
        for l in k..tmp.m() {
          if *tmp.get(l,i) != T::default(){
            row = l;
            break
          }
        }
        if row != i {
          for j in 0..tmp.n() {
            let t = *tmp.get(i,j);
            let t1 = *rc.get(i,j);
            *tmp.get_mut(i,j) = *tmp.get(row,j);
            *rc.get_mut(i,j) = *rc.get(row,j);
            *tmp.get_mut(row,j) = t;
            *rc.get_mut(row,j) = t1;
          }
        } else { return rc }
      }
      if *tmp.get(k,i) == T::default() { continue }
      let mult = - *tmp.get(k,i) / *tmp.get(i,i);
      for j in i..tmp.n(){
        *tmp.get_mut(k,j) = *tmp.get(k,j) + *tmp.get(i,j)*mult;
        *rc.get_mut(k,j) = *rc.get(k,j) + *rc.get(i,j)*mult;
      }
    }
  }
  for i in (1..tmp.m()).rev() {
    for k in (0 ..i).rev() {
      let mut row = i;
      if *tmp.get(i,i) == T::default() {
        for l in (0..k).rev() {
          if *tmp.get(l,i) != T::default(){
            row = l;
            break
          }
        }
        if row != i {
          for j in (0..tmp.n()).rev() {
            let t = *tmp.get(i,j);
            *tmp.get_mut(i,j) = *tmp.get(row,j);
            *tmp.get_mut(row,j) = t;
            let t = *rc.get(i,j);
            *rc.get_mut(i,j) = *rc.get(row,j);
            *rc.get_mut(row,j) = t;
          }
        } else { return rc }
      }
      if *tmp.get(k,i) == T::default() { continue }
      let mult = - *tmp.get(k,i) / *tmp.get(i,i);
      for j in (0..i+1).rev(){
        *tmp.get_mut(k,j) = *tmp.get(k,j) + *tmp.get(i,j)*mult;
        *rc.get_mut(k,j) = *rc.get(k,j) + *rc.get(i,j)*mult;
      }
    }
  }
  for i in 0..tmp.m() {
    for j in 0..tmp.n() {
      *rc.get_mut(i,j) = *rc.get(i,j) / *tmp.get(i,i);
    }
  }
  rc
}
