use std::ops::*;
use std::fmt::Debug;
use num::*;
pub mod matrix_f64;
pub mod matrix_f32;
pub struct Matrix<T:Clone+Default+One>{
  m:u32,
  n:u32,
  data: Vec<T>
}
pub trait TMatrix<'a,T:Add<Output=T>+Sub<Output=T>+Mul<Output=T>+ Neg<Output=T>+PartialEq
  +One+Div<Output=T>+Clone+Default+'a>:Add<&'a Matrix<T>>+Sub<&'a Matrix<T>>+Mul<&'a Matrix<T>>{
  fn m(&self)->u32;
  fn n(&self)->u32;
  fn get(&self,i:u32,j:u32)->&T;
  fn det(&self)->T;
}
pub trait TMatrixMut<'a,T:Add<Output=T>+Sub<Output=T>+Mul<Output=T>+One+Neg<Output=T>+PartialEq+
   Div<Output=T>+Clone+Default+'a>:TMatrix<'a,T>{
  fn get_mut(&mut self,i:u32,j:u32)->&mut T;
}
impl <T:Default+Debug+One+Clone+Add<Output=T>+Sub<Output=T>> Matrix<T> {
  pub fn new(m:u32,n:u32)->Self {
    assert!(m>0 && n>0);
    let mut v = Vec::new();
    v.resize((m*n) as usize,T::default());
    Self{m:m,n:n,data:v}
  }
}
impl<'a,T:Add<Output=T>+Sub<Output=T>+Mul<Output=T>+One+Neg<Output=T>+PartialEq+
  Div<Output=T>+Debug+Clone+Copy+Default+'a> TMatrix<'a,T> for Matrix<T>{
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
}
impl<'a,T:Add<Output=T>+Sub<Output=T>+Mul<Output=T>+One+Neg<Output=T>+PartialEq+
  Div<Output=T>+Debug+Clone+Copy+Default+'a> TMatrix<'a,T> for &'a Matrix<T>{
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
}
impl<'a,T:Add<Output=T>+Sub<Output=T>+Mul<Output=T>+Neg<Output=T>+PartialEq
  +Div<Output=T>+Clone+Debug+Copy+Default+One+'a> TMatrixMut<'a,T> for Matrix<T>{
  fn get_mut(&mut self,i:u32,j:u32)->&mut T{
    &mut self.data[(i*self.n +j) as usize]
  }
}
impl <T: Add<Output=T>+Mul<Output=T>+Neg<Output=T>+PartialEq+
  Div<Output=T>+Clone+Debug+Copy+Default+One+Sub<Output=T>> Add for Matrix<T>{
  type Output = Matrix<T>;
  fn add(self,rhs:Self)->Self::Output{
    op::<'+',T>(self,rhs)
  }
}
impl <T: Add<Output=T>+Mul<Output=T>+Neg<Output=T>+PartialEq+
  Div<Output=T>+Debug+Clone+Copy+Default+One+Sub<Output=T>> Mul for Matrix<T>{
  type Output = Matrix<T>;
  fn mul(self,rhs:Self)->Self::Output{
    op::<'*',T>(self,rhs)
  }
}
impl <T: Sub<Output=T>+Mul<Output=T>+Neg<Output=T>+PartialEq+
  Div<Output=T>+Clone+Debug+Default+Copy+One + Add<Output=T>> Sub for Matrix<T>{
  type Output = Matrix<T>;
  fn sub(self,rhs:Self)->Self::Output{
    op::<'-',T>(self,rhs)
  }
}
impl <'a,T: Add<Output=T>+Mul<Output=T>+Neg<Output=T>+PartialEq+
  Div<Output=T>+Clone+Debug+Copy+Default+One+Sub<Output=T>> Add for &'a Matrix<T>{
  type Output = Matrix<T>;
  fn add(self,rhs:Self)->Self::Output{
    op::<'+',T>(self,rhs)
  }
}
impl <'a,T: Add<Output=T>+Mul<Output=T>+Neg<Output=T>+PartialEq+
  Div<Output=T>+Clone+Debug+Copy+Default+One+Sub<Output=T>> Mul for &'a Matrix<T>{
  type Output = Matrix<T>;
  fn mul(self,rhs:Self)->Self::Output{
    op::<'*',T>(self,rhs)
  }
}
impl <'a,T: Sub<Output=T>+Mul<Output=T>+Neg<Output=T>+PartialEq+
  Div<Output=T>+Clone+Debug+Default+Copy+One + Add<Output=T>> Sub for &'a Matrix<T>{
  type Output = Matrix<T>;
  fn sub(self,rhs:Self)->Self::Output{
    op::<'-',T>(self,rhs)
  }
}
impl <T: Add<Output=T>+Mul<Output=T>+Neg<Output=T>+PartialEq+
  Div<Output=T>+Clone+Debug+Copy+Default+One + Sub<Output=T>> Add<&Matrix<T>> for Matrix<T>{
  type Output = Self;
  fn add(self,rhs:&Self)->Self{
    op::<'+',T>(self,rhs)
  }
}
impl <T: Add<Output=T>+Mul<Output=T>+Neg<Output=T>+PartialEq+
  Div<Output=T>+Clone+Debug+Copy+Default+One + Sub<Output=T>> Mul<&Matrix<T>> for Matrix<T>{
  type Output = Self;
  fn mul(self,rhs:&Self)->Self{
    op::<'*',T>(self,rhs)
  }
}
impl <'a,T: Sub<Output=T>+Mul<Output=T>+Neg<Output=T>+PartialEq+
  Div<Output=T>+One+Debug+Clone+Copy+Default + Add<Output=T>> Sub<&Matrix<T>> for Matrix<T>{
  type Output = Self;
  fn sub(self,rhs:&Self)->Self{
    op::<'-',T>(self,rhs)
  }
}
fn op<'a,const OP:char,T>(lhs:impl TMatrix<'a,T>,rhs:impl TMatrix<'a,T>)->Matrix<T>
where
  T:Default+'a,
  T:One,
  T:Clone,
  T:Copy,
  T:Debug,
  T:Add<Output=T>,
  T:Neg<Output=T>,
  T:PartialEq,
  T:Sub<Output=T>,
  T:Mul<Output=T>,
  T:Div<Output=T>
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
T:Default+'a,
T:One,
T:Clone,
T:Copy,
T:Debug,
T:Neg<Output=T>,
T:PartialEq,
T:Add<Output=T>,
T:Sub<Output=T>,
T:Mul<Output=T>,
T:Div<Output=T>{
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
