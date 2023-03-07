use crate::complex::CNum;
use crate::quaternion::QNum;

pub mod complex;
pub mod quaternion;

pub enum Nums{
    Real(f32),
    Complex(CNum),
    Quaternion(QNum)
}

impl Nums{
    pub fn add(&self, v:Nums) -> Self{
        match self {
            Nums::Real(re)=>{
                match v {
                    Nums::Real(re1)=>Nums::Real(re + re1),
                    Nums::Complex(cnum) => Nums::Complex(cnum.add_r(*re)),
                    Nums::Quaternion(qnum)=> Nums::Quaternion(qnum.add_r(*re))
                }
            }
            Nums::Complex(cnum) =>{
                match v {
                    Nums::Real(re)=> Nums::Complex(cnum.add_r(re)),
                    Nums::Complex(cnum1) => Nums::Complex(cnum.add_c(cnum1)),
                    Nums::Quaternion(qnum)=> Nums::Quaternion(qnum.add_c(cnum.clone()))
                }
            },
            Nums::Quaternion(qnum)=>{
                match v {
                    Nums::Real(re)=> Nums::Quaternion(qnum.add_r(re)),
                    Nums::Complex(cnum) => Nums::Quaternion(qnum.add_c(cnum)),
                    Nums::Quaternion(qnum1)=> Nums::Quaternion(qnum.add_q(qnum1))
                }
            }
        }
    }
    pub fn mult(&self, v:Nums)->Self{
        match self {
            Nums::Real(re)=>{
                match v {
                    Nums::Real(re1)=>Nums::Real(re * re1),
                    Nums::Complex(cnum) => Nums::Complex(cnum.mult_r(*re)),
                    Nums::Quaternion(qnum)=> Nums::Quaternion(qnum.mult_r(*re))
                }
            }
            Nums::Complex(cnum) =>{
                match v {
                    Nums::Real(re)=> Nums::Complex(cnum.mult_r(re)),
                    Nums::Complex(cnum1) => Nums::Complex(cnum.mult_c(cnum1)),
                    Nums::Quaternion(qnum)=> Nums::Quaternion(qnum.mult_c(cnum.clone()))
                }
            },
            Nums::Quaternion(qnum)=>{
                match v {
                    Nums::Real(re)=> Nums::Quaternion(qnum.mult_r(re)),
                    Nums::Complex(cnum) => Nums::Quaternion(qnum.mult_c(cnum)),
                    Nums::Quaternion(qnum1)=> Nums::Quaternion(qnum.mult_q(qnum1))
                }
            }
        }
    }
    pub fn conj(&self)->Self{
        match self{
            Nums::Real(re) => Nums::Real(*re),
            Nums::Complex(cnum) => Nums::Complex(cnum.conj()),
            Nums::Quaternion(qnum)=> Nums::Quaternion(qnum.conj())
        }
    }
    fn normalize(o:(f32, f32, f32)) -> (f32, f32, f32){
        let m = (o.0*o.0+o.1*o.1+o.2*o.2).powf(0.5);
        (o.0/m, o.1/m, o.2/m)
    }
    pub fn rot(&self, ang:f32, o:(f32, f32, f32)) -> Self{
        let o = Nums::normalize(o);
        match self {
            Nums::Real(re)=>Nums::Real(*re),
            Nums::Complex(cnum)=>Nums::Complex(cnum.pow(ang/90_f32)),
            Nums::Quaternion(qnum)=> {
                let q = QNum::make_from_a(ang*std::f32::consts::PI/180_f32, o);
                Nums::Quaternion(q.mult_q(qnum.clone()).mult_q(q.conj()))
            }
        }
    }
}
