use rand::prelude::*;
use matrix;
use matrix::{TMatrix,TMatrixMut,matrix_f64,matrix_f32};
extern {
  fn init_time()->u64;
  fn time_me(tm:u64)->f64;
}

fn main(){
  let mut m = matrix::Matrix::<f64>::new(8,8);
  println!("{}",(&mut m).get(1,1));
  let a = &mut m;
  *a.get_mut(1,1) = 1.0;
  let b = &m + &m;
  println!("{}",(&b).get(1,1));
  let mut tst = matrix_f64::MatrixF64::new(17,15);
  let mut generic = matrix::Matrix::<f64>::new(17,15);
  for i in 0..17{
    for j in 0..15 {
      *generic.get_mut(i,j)=(i+j) as f64;
    }
  }
  for i in 0..17{
    for j in 0..15 {
      *tst.get_mut(i,j)=(i+j) as f64;
    }
  }
  let pf = |rc:&matrix_f64::MatrixF64|{
    for i in 0..17{
      for j in 0..15 {
        print!(" {}",rc.get(i,j));
      }
      println!("");
    }
  };
  println!("before");
  pf(&tst);
  let rc = &tst+&tst+&generic;
  let sc = &tst+&tst-&tst;
  println!("after");
  pf(&rc);
  println!("sub");
  pf(&sc);
  let mut tst = matrix_f32::MatrixF32::new(17,18);
  let mut generic = matrix::Matrix::<f32>::new(17,18);
  for i in 0..17{
    for j in 0..18 {
      *generic.get_mut(i,j)=(i+j) as f32;
    }
  }
  for i in 0..17{
    for j in 0..18 {
      *tst.get_mut(i,j)=(i+j) as f32;
    }
  }
  let pf = |rc:&matrix_f32::MatrixF32|{
    for i in 0..17{
      for j in 0..18 {
        print!(" {}",rc.get(i,j));
      }
      println!("");
    }
  };
  println!("before");
  pf(&tst);
  let rc = &tst+&tst+&generic;
  let sc = &tst+&tst-&tst;
  println!("after");
  pf(&rc);
  println!("sub");
  pf(&sc);
  fn f1<'a>(m:impl matrix::TMatrix<'a,f32>) {
    for i in 0..m.m() {
      for j in 0..m.n() {
        print!(" {}",m.get(i,j));
      }
      println!("");
    }
  }
  fn f2<'a>(m:impl matrix::TMatrix<'a,f64>){
    for i in 0..m.m() {
      for j in 0..m.n() {
        print!(" {}",m.get(i,j));
      }
      println!("");
    }
  }
  const DIM:u32 = 1000;
  let mut amxmat = matrix_f32::MatrixF32::new(DIM,DIM);
  {
    for i in 0..DIM{
      for j in 0..DIM{
        *amxmat.get_mut(i,j)=(i+j) as f32;
      }
    }
    let tm = unsafe{init_time()};
    let mut res=matrix_f32::MatrixF32::new(1,1);
    for _ in 0..10 {
        res = &amxmat+&amxmat-&amxmat*&amxmat;
    }
    println!("amx took {}",unsafe{time_me(tm)});
    for i in 0..DIM {
      for j in 0..DIM {
        *amxmat.get_mut(i,j) = *res.get(i,j);
      }
    }
  }
  let mut generic = matrix::Matrix::<f32>::new(DIM,DIM);
  {
    for i in 0..DIM{
      for j in 0..DIM{
        *generic.get_mut(i,j)=(i+j) as f32;
      }
    }
    let tm = unsafe{init_time()};
    let mut res=matrix::Matrix::<f32>::new(1,1);
    for _ in 0..10 {
      res = &generic+&generic-&generic*&generic;
    }
    println!("generic took {}",unsafe{time_me(tm)});
    for i in 0..DIM {
      for j in 0..DIM {
        *generic.get_mut(i,j) = *res.get(i,j);
      }
    }
  }
  for i in 0..DIM{
    for j in 0..DIM{
      if *amxmat.get(i,j)!=*generic.get(i,j) {println!("error");break;}
    }
  }
  println!("amxmat");
  //f1(&amxmat);
  println!("generic");
  //f1(&generic);

  let mut generic1 = matrix::Matrix::<f64>::new(4,3);
  let mut generic2 = matrix::Matrix::<f64>::new(3,2);
  for i in 0..generic1.m() {
    for j in 0..generic1.n() {
      *generic1.get_mut(i,j) = (i+j) as f64;
    }
  }
  println!("mat 1");
  f2(&generic1);
  for i in 0..generic2.m() {
    for j in 0..generic2.n() {
      *generic2.get_mut(i,j) = (i+j) as f64;
    }
  }
  println!("mat 2");
  f2(&generic2);
  let res = &generic1 * &generic2;
  println!("mult");
  f2(&res);
  let mut f64_1 = matrix_f64::MatrixF64::new(4,3);
  let mut f64_2 = matrix_f64::MatrixF64::new(3,2);
  for i in 0..f64_1.m() {
    for j in 0..f64_1.n() {
      *f64_1.get_mut(i,j) = (i+j) as f64;
    }
  }
  println!("amx mat 1");
  f2(&f64_1);
  for i in 0..f64_2.m() {
    for j in 0..f64_2.n() {
      *f64_2.get_mut(i,j) = (i+j) as f64;
    }
  }
  println!("amx mat 2");
  f2(&f64_2);
  let res = &f64_1 * &f64_2;
  println!("amx mult");
  f2(&res);
  let mut generic_det = matrix::Matrix::<f64>::new(4,4);
  let mut generic_det1 = matrix::Matrix::<f32>::new(3,3);
  let mut rnd = [0.0,1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0,9.0,10.0,11.0,12.0,13.0,14.0,15.0];
  let gd1 = [6.0,1.0,1.0,4.0,-2.0,5.0,2.0,8.0,7.0,0.0,0.0,0.0,0.0];
  let mut rng = rand::thread_rng();
  rnd.shuffle(&mut rng);
  let mut k = 1.0;
  for i in 0..generic_det.m() {
    for j in 0..generic_det.n(){
      *generic_det.get_mut(i,j) = rnd[(i*generic_det.n()+j) as usize]/* (i+j) as f64*/;
      k += 1.0;
    }
  }
  for i in 0..generic_det1.m() {
    for j in 0..generic_det1.n(){
      *generic_det1.get_mut(i,j) = gd1[(i*generic_det1.n()+j)as usize];
      k += 1.0;
    }
  }
  println!("generic");
  f2(&generic_det);
  println!("generic det {}",generic_det.det());
  println!("generic1");
  f1(&generic_det1);
  println!("generic det1 {}",generic_det1.det());
}
