//!Complex Numbers


use std::ops::Neg;
use crate::cassette::cassette;

///Structure for storing complex numbers
///
/// Структура для хранения комплексных чисел
pub struct CNum {
    r:f32,
    i:f32
}

pub const R:u8 = 1;
pub const I:u8 = 2;

impl CNum {
    ///The function for creating a complex number with zero coefficients
    ///
    ///Функция для создания комплексного числа с нулевыми коэффициентами
    ///
    /// # Example
    ///
    ///```
    /// use tmn::complex::CNum;
    /// let a = CNum::make_zero();
    /// assert!(CNum::make(0_f32, 0_f32)==a);
    /// ```
    pub fn make_zero()->Self{
        Self{
            r:0_f32,
            i:0_f32
        }
    }

    ///The function for creating a complex number from the real and imaginary parts
    ///
    ///Функция для создания комплексного числа из действительной и мнимой части
    ///
    /// # Example
    ///
    ///```
    /// use tmn::complex::CNum;
    /// let a = CNum::make(4_f32, -2_f32);
    /// assert_eq!((4_f32, -2_f32), a.get());
    /// ```
    pub fn make(r:f32, i:f32)->Self{ Self{r,i } }
    ///The fun for cloning a complex number
    ///
    /// Метод для клонирования комплексного числа
    ///
    ///  # Example
    ///```
    /// use tmn::complex::CNum;
    /// let a = CNum::make(3_f32, 4_f32);
    /// let c = a.clone();
    /// assert!(CNum::make(3_f32, 4_f32)==c);
    /// ```
    pub fn clone(&self) -> CNum{ CNum{r:self.r,i:self.i} }
    /// The method that returns a tuple consisting of the real and imaginary parts of a complex number
    ///
    /// Метод, возвращающий кортеж состоящий из действительной и мнимой части комплексного числа
    ///
    /// # Example
    /// ```
    /// use tmn::complex::CNum;
    /// let a = CNum::make(43_f32, 21_f32);
    /// assert_eq!((43_f32, 21_f32), a.get());
    /// ```
    pub fn get(&self) -> (f32, f32){ (self.r, self.i) }
    /// The method that returns a complex conjugate number
    ///
    /// Метод, возвращающий комплексно сопряженное число
    ///
    /// # Example
    /// ```
    /// use tmn::complex::CNum;
    /// let a = CNum::make(1_f32, 1_f32);
    /// let c = a.conj();
    /// assert!(CNum::make(1_f32, -1_f32) == c);
    /// ```
    pub fn conj(&self) -> CNum{CNum{r:self.r, i:-self.i}}
    ///The method that returns the modulus of a complex number
    ///
    ///Метод, возвращающий модуль комплексного числа
    ///
    /// # Example
    ///```
    /// use tmn::complex::CNum;
    /// let a = CNum::make(3_f32, 4_f32);
    /// assert_eq!(5_f32, a.modl());
    /// ```
    pub fn modl(&self) -> f32{self.mult_c(self.conj()).r.powf(0.5) }
    ///The method that returns the sum of a complex and a real number
    ///
    /// Метод, возвращающий сумму комплексного и действительного числа
    ///
    /// # Example
    ///```
    /// use tmn::complex::CNum;
    /// let mut a = CNum::make(3_f32, 4_f32);
    /// a = a.add_r(7_f32);
    /// assert!(CNum::make(10_f32, 4_f32)==a);
    /// ```
    pub fn add_r(&self, v:f32) -> CNum{
        CNum{
            r:self.r + v,
            i:self.i
        }
    }
    ///The method that returns the sum of complex numbers
    ///
    ///Метод, возвращающий сумму комплексных чисел
    ///
    /// # Example
    ///```
    /// use tmn::complex::CNum;
    /// let a = CNum::make(6_f32, 2_f32);
    /// let b = CNum::make(4_f32, 8_f32);
    /// let c = a.add_c(b);
    /// assert!(CNum::make(10_f32, 10_f32)== c);
    /// ```
    pub fn add_c(&self, v:CNum) -> CNum{
        CNum{
            r:self.r + v.r,
            i:self.i + v.i
        }
    }
    /// The method that returns the product of complex and real numbers
    ///
    /// Метод, возвращающий произведение комплексного и действительного чисел
    ///
    /// # Example
    /// ```
    /// use tmn::complex::CNum;
    /// let mut a = CNum::make(4_f32, -2_f32);
    /// a = a.mult_r(2_f32);
    /// assert!(CNum::make(8_f32, -4_f32) == a);
    /// ```
    pub fn mult_r(&self, v:f32) -> CNum{
        CNum{
            r: self.r * v,
            i: self.i * v
        }
    }
    /// The method that returns the product of complex numbers
    ///
    /// Метод, возвращающий произведение комплексных чисел
    /// # Example
    /// ```
    /// use tmn::complex::CNum;
    /// let a = CNum::make(3_f32, 2_f32);
    /// let b = CNum::make(5_f32, 3_f32);
    /// let c = a.mult_c(b);
    /// assert!(CNum::make(9_f32, 19_f32) == c);
    /// ```
    pub fn mult_c(&self, v:CNum) -> CNum{
        let (r, i) = self.get();
        CNum{
            r:r * v.r - i * v.i,
            i:r * v.i + v.r * i
        }
    }
    ///The method for dividing complex numbers
    ///
    /// Метод для деления комплексных чисел
    /// # Example
    /// ```
    /// use tmn::complex::CNum;
    /// let a = CNum::make(3_f32, 2_f32);
    /// let b = CNum::make(5_f32, 3_f32);
    /// let c = a.div_c(b);
    /// assert!(CNum::make(21_f32/34_f32, 1_f32/34_f32)==c);
    /// ```
    pub fn div_c(&self, v:CNum) -> CNum{
        let divisor = (v.mult_c(v.conj())).r;
        let numerator = self.mult_c(v.conj());
        numerator.mult_r(1_f32/divisor)
    }
    /// The method for raising a complex number to a power. Degrees less than one (roots) are counted with k = 0
    ///
    /// Метод для возведения комплексного числа в степень. Степени меньше единицы (корни) считаются с k = 0
    ///
    /// # Example
    /// ```
    /// use tmn::complex::CNum;
    /// let mut a = CNum::make(3_f32, 2_f32);
    /// a = a.pow(2_f32);
    /// let (r, i) = a.get();
    /// assert!((r-5_f32).abs() < 0.000001);
    /// assert!((i-12_f32).abs() < 0.000001);
    /// ```
    pub fn pow(&self, v:f32) ->CNum{
        CNum{
            r: self.modl().powf(v)*(v * self.i.atan2(self.r)).cos(),
            i: self.modl().powf(v)*(v * self.i.atan2(self.r)).sin()
        }
    }
    ///The method for setting values to specific coefficients
    ///
    /// Метод для установки значений в конкретный коэффициенты
    ///
    /// # Example
    ///```
    /// use tmn::complex;
    /// use tmn::complex::CNum;
    /// let mut a = CNum::make(1_f32, 2_f32);
    /// a = a.set(complex::R|complex::I, 3_f32);
    /// assert!(CNum::make(3_f32, 3_f32)== a);
    /// ```
    pub fn set(&self, c:u8, v:f32) -> Self{
        let mut ret = self.clone();
        if cassette::eq(c, 0){
            ret.r = v;
        }
        if cassette::eq(c, 1){
            ret.i = v;
        }
        ret
    }
}

impl PartialEq for CNum{
    ///Redefined comparison operator
    ///
    ///Переопределенный оператор сравнения
    ///
    /// # Example
    ///```
    /// use tmn::complex::CNum;
    /// let cnum = CNum::make(3_f32, 4_f32);
    /// assert!(cnum == CNum::make(3_f32, 4_f32));
    /// ```
    fn eq(&self, other: &Self) -> bool {
        self.get()==other.get()
    }
}

impl Neg for CNum {
    type Output = Self;
    ///Redefined negative operator
    ///
    ///Переопределенный оператор отрицательного значения
    ///
    /// # Example
    ///```
    /// use tmn::complex::CNum;
    /// let cnum = -CNum::make(3_f32, 4_f32);
    /// assert!(cnum == CNum::make(-3_f32, -4_f32));
    /// ```
    fn neg(self) -> Self::Output {
        self.mult_r(-1_f32)
    }
}