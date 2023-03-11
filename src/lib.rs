//!# TMN(Too Many Numbers)
//!
//! Library for working with complex numbers and quaternions
//!
//! Библиотека для работы с комплексными числами и кватернионами
use std::ops::{Add, Mul, Neg};
use crate::complex::CNum;
use crate::quaternion::QNum;

pub mod complex;
pub mod quaternion;
pub mod cassette;

///Enum for convenient work with different types of numbers
///
///Перечисление для удобной работы с разными видами чисел
pub enum Nums{
    Real(f32),
    Complex(CNum),
    Quaternion(QNum)
}

impl Nums{
    ///The method for obtaining the conjugate number
    ///
    ///Метод для получения сопряженного числа
    ///
    /// # Example
    ///```
    /// use tmn::Nums;
    /// use tmn::quaternion::QNum;
    /// let a = Nums::Quaternion(QNum::make_from_r(1_f32, 1_f32, 1_f32, 1_f32));
    /// let b = a.conj();
    /// match b{
    ///    Nums::Quaternion(qnum)=>{
    ///        assert_eq!((1_f32, -1_f32, -1_f32, -1_f32), qnum.get())
    ///    },
    ///    _=>panic!("WrongType of Nums")
    /// }
    /// ```
    /// ```
    /// use tmn::Nums;
    /// use tmn::complex::CNum;
    /// let a = Nums::Complex(CNum::make(1_f32, 1_f32));
    /// let b = a.conj();
    /// match b{
    ///    Nums::Complex(cnum)=>{
    ///        assert_eq!((1_f32, -1_f32), cnum.get())
    ///    },
    ///    _=>panic!("WrongType of Nums")
    /// }
    /// ```
    pub fn conj(&self)->Self{
        match self{
            Nums::Real(re) => Nums::Real(*re),
            Nums::Complex(cnum) => Nums::Complex(cnum.conj()),
            Nums::Quaternion(qnum)=> Nums::Quaternion(qnum.conj())
        }
    }
    fn normalize(o:(f32, f32, f32)) -> (f32, f32, f32){//Нормализация вектора o
        let m = (o.0*o.0+o.1*o.1+o.2*o.2).powf(0.5);
        if m == 0_f32 {
            return (f32::NAN, f32::NAN, f32::NAN);
        }
        (o.0/m, o.1/m, o.2/m)
    }
    ///The method for rotating a number around the axis given by the vector 'o' by the angle 'ang' (Angle in degrees). The axis of rotation only affects the rotation of the quaternion.
    ///
    ///Метод для вращения числа вокруг оси, заданной вектором 'o' на угол 'ang' (Угол в градусах). Ось вращения влияет только на поворот кватерниона.
    ///
    /// # Example
    ///
    ///```
    /// use tmn::Nums;
    /// use tmn::quaternion::QNum;
    /// let mut a = Nums::Quaternion(QNum::make_from_r(0_f32, 1_f32, 0_f32, 0_f32));
    /// a = a.rot(90_f32, (0_f32, 0_f32, 1_f32));
    /// match a {
    ///    Nums::Quaternion(qnum)=>{
    ///        let (r, i, j, k) = qnum.get();//0.0000001 - точность расчетов
    ///        //Ожидаемый ответ 0, 0, 1, 0
    ///        assert!((r-0_f32).abs() < 0.0000001);
    ///        assert!((i-0_f32).abs() < 0.0000001);
    ///        assert!((j-1_f32).abs() < 0.0000001);
    ///        assert!((k-0_f32).abs() < 0.0000001);
    ///    },
    ///    _=>panic!("WrongType of Nums")
    /// }
    /// ```
    ///

    pub fn rot(&self, ang:f32, o:(f32, f32, f32)) -> Self{
        let o = Nums::normalize(o);
        match self {
            Nums::Real(re)=>Nums::Real(*re),
            Nums::Complex(cnum)=>Nums::Complex(cnum.pow(ang/90_f32)),
            Nums::Quaternion(qnum)=> {
                assert!(!o.0.is_nan());
                let q = QNum::make_from_a(ang*std::f32::consts::PI/180_f32, o);
                Nums::Quaternion(q.mult_q(qnum.clone()).mult_q(q.conj()))
            }
        }
    }
    ///The method for setting values to specific coefficients
    ///
    /// Метод для установки значений в конкретные коэффициенты
    ///
    /// # Example
    ///```
    /// use tmn::{Nums, complex};
    /// use tmn::complex::CNum;
    /// let mut a = Nums::Complex(CNum::make_zero());
    /// a = a.set(complex::R|complex::I, 3_f32);
    /// assert!(Nums::Complex(CNum::make(3_f32, 3_f32))==a);
    /// ```
    pub fn set(&self, c:u8, v:f32)->Self{
        match self {
            Nums::Real(re)=>Nums::Real(*re),
            Nums::Complex(cnum)=>Nums::Complex(cnum.set(c, v)),
            Nums::Quaternion(qnum)=>Nums::Quaternion(qnum.set(c, v))
        }
    }
    ///The method for cloning the Nums element
    ///
    ///Метод для клонирования элемента Nums
    ///
    /// # Example
    ///```
    /// use tmn::Nums;
    /// use tmn::quaternion::QNum;
    /// let a = Nums::Quaternion(QNum::make_zero());
    /// let b = a.clone();
    /// assert!(a==b);
    /// ```
    pub fn clone(&self)->Self{
        match self {
            Nums::Real(re) => Nums::Real(*re),
            Nums::Complex(cnum)=> Nums::Complex(cnum.clone()),
            Nums::Quaternion(qnum) => Nums::Quaternion(qnum.clone())
        }
    }
}

impl PartialEq for Nums{
    fn eq(&self, other: &Self) -> bool {
        match self{
            Nums::Real(re)=>{
                match other {
                    Nums::Real(re1) => re1==re,
                    _=>false
                }
            },
            Nums::Complex(cnum)=>{
                match other{
                    Nums::Complex(cnum1)=>cnum==cnum1,
                    _=>false
                }
            },
            Nums::Quaternion(qnum)=>{
                match other {
                    Nums::Quaternion(qnum1)=>qnum==qnum1,
                    _=>false
                }
            }
        }
    }
}

impl Add for Nums{
    type Output = Nums;
    ///
    /// The method returns the sum of two Nums elements
    ///
    /// Метод возвращает сумму двух элементов Nums
    ///
    /// # Examples
    ///```
    /// use tmn::Nums;
    /// use tmn::complex::CNum;
    /// use tmn::quaternion::QNum;
    ///
    /// let a = Nums::Quaternion(QNum::make_from_r(0_f32, 0_f32, 1_f32, 1_f32));
    /// let b = Nums::Complex(CNum::make(1_f32, 1_f32));
    ///
    /// let c = a+b;
    /// assert!(Nums::Quaternion(QNum::make_from_r(1_f32,1_f32,1_f32,1_f32))==c);
    /// ```
    fn add(self, rhs: Self) -> Self::Output{
        match self {
            Nums::Real(re)=>{
                match rhs {
                    Nums::Real(re1)=>Nums::Real(re + re1),
                    Nums::Complex(cnum) => Nums::Complex(cnum.add_r(re)),
                    Nums::Quaternion(qnum)=> Nums::Quaternion(qnum.add_r(re))
                }
            }
            Nums::Complex(cnum) =>{
                match rhs {
                    Nums::Real(re)=> Nums::Complex(cnum.add_r(re)),
                    Nums::Complex(cnum1) => Nums::Complex(cnum.add_c(cnum1)),
                    Nums::Quaternion(qnum)=> Nums::Quaternion(qnum.add_c(cnum.clone()))
                }
            },
            Nums::Quaternion(qnum)=>{
                match rhs {
                    Nums::Real(re)=> Nums::Quaternion(qnum.add_r(re)),
                    Nums::Complex(cnum) => Nums::Quaternion(qnum.add_c(cnum)),
                    Nums::Quaternion(qnum1)=> Nums::Quaternion(qnum.add_q(qnum1))
                }
            }
        }
    }
}

impl Mul for Nums{
    type Output = Self;

    ///The method returns the product of two Nums elements
    ///
    /// Метод возвращает произведение двух элементов Nums
    ///
    /// # Examples
    ///```
    /// use tmn::Nums;
    /// use tmn::complex::CNum;
    /// use tmn::quaternion::QNum;
    ///
    /// let a = Nums::Quaternion(QNum::make_from_r(0_f32, 4_f32, 7_f32, 1_f32));
    /// let b = Nums::Complex(CNum::make(43_f32, 2_f32));
    ///
    /// let c = a*b;
    /// assert!(Nums::Quaternion(QNum::make_from_r(-8_f32, 172_f32, 303_f32, 29_f32))==c);
    /// ```
    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            Nums::Real(re)=>{
                match rhs {
                    Nums::Real(re1)=>Nums::Real(re * re1),
                    Nums::Complex(cnum) => Nums::Complex(cnum.mult_r(re)),
                    Nums::Quaternion(qnum)=> Nums::Quaternion(qnum.mult_r(re))
                }
            }
            Nums::Complex(cnum) =>{
                match rhs {
                    Nums::Real(re)=> Nums::Complex(cnum.mult_r(re)),
                    Nums::Complex(cnum1) => Nums::Complex(cnum.mult_c(cnum1)),
                    Nums::Quaternion(qnum)=> Nums::Quaternion(qnum.mult_c(cnum.clone()))
                }
            },
            Nums::Quaternion(qnum)=>{
                match rhs {
                    Nums::Real(re)=> Nums::Quaternion(qnum.mult_r(re)),
                    Nums::Complex(cnum) => Nums::Quaternion(qnum.mult_c(cnum)),
                    Nums::Quaternion(qnum1)=> Nums::Quaternion(qnum.mult_q(qnum1))
                }
            }
        }
    }
}

impl Neg for Nums {
    type Output = Self;
    ///Redefined negative operator
    ///
    ///Переопределенный оператор отрицательного значения
    ///
    /// # Example
    ///```
    /// use tmn::Nums;
    /// use tmn::quaternion::QNum;
    /// let qnum = Nums::Quaternion(-QNum::make_from_r(3_f32, 4_f32, 1_f32, 2_f32));
    /// assert!(qnum== Nums::Quaternion(QNum::make_from_r(-3_f32, -4_f32, -1_f32, -2_f32)));
    /// ```
    fn neg(self) -> Self::Output {
        self.mul(Nums::Real(-1_f32))
    }
}