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
  let mut amxmat = matrix_f32::MatrixF32::new(1000,1000);
  {
    for i in 0..1000{
      for j in 0..1000{
        *amxmat.get_mut(i,j)=(i+j) as f32;
      }
    }
    let tm = unsafe{init_time()};
    let mut res=matrix_f32::MatrixF32::new(1000,1000);
    for _ in 0..1000 {
        res = &amxmat+&amxmat-&amxmat+&amxmat;
    }
    println!("amx took {}",unsafe{time_me(tm)});
    for i in 0..1000 {
      for j in 0..1000 {
        *amxmat.get_mut(i,j) = *res.get(i,j);
      }
    }
  }
  let mut generic = matrix::Matrix::<f32>::new(1000,1000);
  {
    for i in 0..1000{
      for j in 0..1000{
        *generic.get_mut(i,j)=(i+j) as f32;
      }
    }
    let tm = unsafe{init_time()};
    let mut res=matrix::Matrix::<f32>::new(1000,1000);
    for _ in 0..1000 {
      res = &generic+&generic-&generic+&generic;
    }
    println!("generic took {}",unsafe{time_me(tm)});
    for i in 0..1000 {
      for j in 0..1000 {
        *generic.get_mut(i,j) = *res.get(i,j);
      }
    }
  }
  for i in 0..1000{
    for j in 0..1000{
      if *amxmat.get(i,j)!=*generic.get(i,j) {println!("error");break;}
    }
  }
}
