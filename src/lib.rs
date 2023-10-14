use std::ops::*;
pub mod matrix_f64;
pub mod matrix_f32;
pub struct Matrix<T:Clone+Default> where  T:Sub<Output = T>,T:Add<Output=T>{
  m:u32,
  n:u32,
  data: Vec<T>
}
pub trait TMatrix<'a,T:Add<Output=T>+Sub<Output=T>+Clone+Default+'a>:Add<&'a Matrix<T>>+Sub<&'a Matrix<T>>{
  fn m(&self)->u32;
  fn n(&self)->u32;
  fn get(&self,i:u32,j:u32)->&T;
}
pub trait TMatrixMut<'a,T:Add<Output=T>+Sub<Output=T>+Clone+Default+'a>:TMatrix<'a,T>{
  fn get_mut(&mut self,i:u32,j:u32)->&mut T;
}
impl <T:Default+Clone+Add<Output=T>+Sub<Output=T>> Matrix<T> {
  pub fn new(m:u32,n:u32)->Self {
    assert!(m>0 && n>0);
    let mut v = Vec::new();
    v.resize((m*n) as usize,T::default());
    Self{m:m,n:n,data:v}
  }
}
impl<'a,T:Add<Output=T>+Sub<Output=T>+Clone+Copy+Default+'a> TMatrix<'a,T> for Matrix<T>{
  fn m(&self)->u32{
    self.m
  }
  fn n(&self)->u32{
    self.n
  }
  fn get(&self,i:u32,j:u32)->&T{
    &self.data[(i*self.n +j) as usize]
  }
}
impl<'a,T:Add<Output=T>+Sub<Output=T>+Clone+Copy+Default+'a> TMatrix<'a,T> for &'a Matrix<T>{
  fn m(&self)->u32{
    self.m
  }
  fn n(&self)->u32{
    self.n
  }
  fn get(&self,i:u32,j:u32)->&T{
    &self.data[(i*self.n +j) as usize]
  }
}
impl<'a,T:Add<Output=T>+Sub<Output=T>+Clone+Copy+Default+'a> TMatrixMut<'a,T> for Matrix<T>{
  fn get_mut(&mut self,i:u32,j:u32)->&mut T{
    &mut self.data[(i*self.n +j) as usize]
  }
}
impl <'a,T: Add<Output=T>+Clone+Copy+Default+Sub<Output=T>> Add for &'a Matrix<T>{
  type Output = Matrix<T>;
  fn add(self,rhs:Self)->Self::Output{
    op::<'+',T>(self,rhs)
  }
}
impl <'a,T: Sub<Output=T>+Clone+Default+Copy + Add<Output=T>> Sub for &'a Matrix<T>{
  type Output = Matrix<T>;
  fn sub(self,rhs:Self)->Self::Output{
    op::<'-',T>(self,rhs)
  }
}
impl <T: Add<Output=T>+Clone+Copy+Default + Sub<Output=T>> Add<&Matrix<T>> for Matrix<T>{
  type Output = Self;
  fn add(self,rhs:&Self)->Self{
    op::<'+',T>(self,rhs)
  }
}
impl <'a,T: Sub<Output=T>+Clone+Copy+Default + Add<Output=T>> Sub<&Matrix<T>> for Matrix<T>{
  type Output = Self;
  fn sub(self,rhs:&Self)->Self{
    op::<'-',T>(self,rhs)
  }
}
fn op<'a,const OP:char,T>(lhs:impl TMatrix<'a,T>,rhs:impl TMatrix<'a,T>)->Matrix<T>
where
  T:Default+'a,
  T:Clone,
  T:Copy,
  T:Add<Output=T>,
  T:Sub<Output=T>
{
    let mut rc = Matrix::<T>::new(lhs.m(),lhs.n());
    for i in 0..lhs.m() {
      for j in 0..lhs.n() {
        match OP {
          '+' => {*rc.get_mut(i,j) = *lhs.get(i,j)+*rhs.get(i,j);}
          '-' => {*rc.get_mut(i,j) = *lhs.get(i,j)-*rhs.get(i,j);}
          _ => {}
        }
      }
    }
    rc
}
