//!Quaternions
use std::ops::Neg;
use crate::cassette;
use crate::complex::CNum;
///The structure storing the quaternion
///
/// Структура хранящая кватернион
pub struct QNum{r:f32, i:f32, j:f32, k:f32 }
pub const R:u8 = 1;
pub const I:u8 = 2;
pub const J:u8 = 4;
pub const K:u8 = 8;

impl QNum {
    ///The function for creating a quaternion with zero coefficients
    ///
    ///Функция для создания кватернионов с нулевыми коэффициентами
    ///
    /// # Example
    ///
    ///```
    /// use tmn::quaternion::QNum;
    /// let a = QNum::make_zero();
    /// assert!(QNum::make_from_r(0_f32, 0_f32, 0_f32, 0_f32)==a);
    /// ```

    pub fn make_zero()->Self{
        QNum{
            r:0_f32,
            i:0_f32,
            j:0_f32,
            k:0_f32
        }
    }
    ///The function that creates a quaternion from real coefficients
    ///
    ///Функция, создающая кватернион из действительных коэффициентов
    /// # Example
    ///```
    /// use tmn::quaternion::QNum;
    /// let c = QNum::make_from_r(1_f32, 2_f32, 3_f32, 4_f32);
    /// assert_eq!((1_f32, 2_f32, 3_f32, 4_f32), c.get());
    /// ```
    pub fn make_from_r(r:f32, i:f32, j:f32, k:f32) ->Self{ Self{r, i, j, k } }
    ///The function that creates a quaternion of 2 complex numbers
    ///
    ///Функция, создающая кватернион из 2 комплексных чисел
    /// # Example
    ///```
    /// use tmn::complex::CNum;
    /// use tmn::quaternion::QNum;
    /// let c = QNum::make_from_c(CNum::make(1_f32, 2_f32), CNum::make(3_f32, 4_f32));
    /// assert_eq!((1_f32, 2_f32, 3_f32, 4_f32), c.get());
    /// ```
    pub fn make_from_c(w1:CNum, w2:CNum) ->Self{
        let (r, i) = w1.get();
        let (j, k) = w2.get();
        Self{ r, i, j, k }
    }
    ///The function that creates a rotation quaternion from the angle 'ang' and the axis of rotation given by a vector in the form of a tuple
    ///
    ///Функция, создающая кватернион поворота из угла 'ang' и оси вращения, заданной вектором в виде кортежа
    /// # Example
    ///```
    /// use tmn::quaternion::QNum;
    /// let c = QNum::make_from_a(90_f32*std::f32::consts::PI/180_f32, (0_f32, 0_f32, 1_f32));
    /// assert_eq!(((2_f32).powf(0.5)/2_f32, 0_f32, 0_f32, (2_f32).powf(0.5)/2_f32), c.get());
    /// ```
    pub fn make_from_a(ang:f32, vec:(f32, f32, f32)) ->Self{
        Self{
            r:(ang/2.0).cos(),
            i:(ang/2.0).sin()*vec.0,
            j:(ang/2.0).sin()*vec.1,
            k:(ang/2.0).sin()*vec.2
        }
    }
    ///The method for cloning a quaternion
    ///
    /// Метод для клонирования кватерниона
    /// # Example
    /// ```
    /// use tmn::quaternion::QNum;
    /// let a = QNum::make_from_r(1_f32, 1_f32, 1_f32, 1_f32);
    /// let c = a.clone();
    /// assert_eq!((1_f32, 1_f32, 1_f32, 1_f32), c.get());
    /// ```
    pub fn clone(&self) -> QNum{QNum{r:self.r,i:self.i,j:self.j,k:self.k } }
    ///The method for obtaining quaternion coefficients in the form of a tuple
    ///
    /// Метод для получения коэффициентов кватерниона в виде кортежа
    ///
    /// # Example
    /// ```
    /// use tmn::quaternion::QNum;
    /// let c = QNum::make_from_r(1_f32, 2_f32, 3_f32, 4_f32);
    /// assert_eq!((1_f32, 2_f32, 3_f32, 4_f32), c.get());
    /// ```
    pub fn get(&self) -> (f32, f32, f32, f32){ (self.r, self.i, self.j, self.k) }
    ///The Method that returns the conjugate quaternion
    ///
    ///Метод, возвращающий сопряженный кватернион
    ///
    /// # Example
    ///```
    /// use tmn::quaternion::QNum;
    /// let mut a = QNum::make_from_r(1_f32, 1_f32, 1_f32, 1_f32);
    /// a = a.conj();
    /// assert_eq!((1_f32, -1_f32, -1_f32, -1_f32), a.get());
    /// ```
    pub fn conj(&self) -> QNum{QNum{r:self.r, i:-self.i, j:-self.j, k:-self.k}}
    ///The method that returns the quaternion norm
    ///
    /// Метод, возвращающий норму кватерниона
    ///
    /// # Example
    ///```
    /// use tmn::quaternion::QNum;
    /// let a = QNum::make_from_r(1_f32, 1_f32, 1_f32, 1_f32);
    /// assert_eq!(4_f32, a.norm());
    ///
    /// ```
    pub fn norm(&self) -> f32{self.mult_q(self.conj()).r}
    ///The method that returns the quaternion module
    ///
    /// Метод, возвращающий модуль кватерниона
    ///
    /// # Example
    ///```
    /// use tmn::quaternion::QNum;
    /// let a = QNum::make_from_r(1_f32, 1_f32, 1_f32, 1_f32);
    /// assert_eq!(2_f32, a.modl());
    ///
    /// ```
    pub fn modl(&self) -> f32{self.norm().powf(0.5)}
    /// The method that returns the sum of a quaternion and a real number
    ///
    /// Метод, возвращающий сумму кватерниона и действительного числа
    ///
    /// # Example
    /// ```
    /// use tmn::quaternion::QNum;
    /// let mut a = QNum::make_from_r(4_f32, 10_f32, 10_f32, 10_f32);
    /// a = a.add_r(6_f32);
    /// assert_eq!((10_f32, 10_f32, 10_f32, 10_f32), a.get());
    /// ```
    pub fn add_r(&self, v:f32) -> QNum{QNum {r:self.r+v, i:self.i, j:self.j, k:self.k} }
    /// The method that returns the sum of a quaternion and a complex number
    ///
    /// Метод, возвращающий сумму кватерниона и комплексного числа
    ///
    /// # Example
    /// ```
    /// use tmn::complex::CNum;
    /// use tmn::quaternion::QNum;
    /// let mut a = QNum::make_from_r(4_f32, 4_f32, 10_f32, 10_f32);
    /// a = a.add_c(CNum::make(6_f32, 6_f32));
    /// assert_eq!((10_f32, 10_f32, 10_f32, 10_f32), a.get());
    /// ```
    pub fn add_c(&self, v:CNum) -> QNum{
        let (r, i) = v.get();
        QNum {r:self.r+r, i:self.i+i, j:self.j, k:self.k}
    }
    /// The method that returns the sum of quaternions
    ///
    /// Метод, возвращающий сумму кватернионов
    ///
    /// # Example
    /// ```
    /// use tmn::quaternion::QNum;
    /// let mut a = QNum::make_from_r(4_f32, 4_f32, 4_f32, 4_f32);
    /// a = a.add_q(QNum::make_from_r(6_f32, 6_f32, 6_f32, 6_f32));
    /// assert_eq!((10_f32, 10_f32, 10_f32, 10_f32), a.get());
    /// ```
    pub fn add_q(&self, v:QNum) -> QNum{ QNum {r:self.r+v.r, i:self.i+v.i, j:self.j+v.j, k:self.k+v.k} }
    /// The method that returns the product of a quaternion and a real number
    ///
    /// Метод, возвращающий произведение кватерниона и действительного числа
    ///
    /// # Example
    /// ```
    /// use tmn::quaternion::QNum;
    /// let mut a = QNum::make_from_r(1_f32, 1_f32, 1_f32, 1_f32);
    /// a = a.mult_r(10_f32);
    /// assert_eq!((10_f32, 10_f32, 10_f32, 10_f32), a.get());
    /// ```
    pub fn mult_r(&self, v:f32) -> QNum{ QNum {r:self.r*v, i:self.i*v, j:self.j*v, k:self.k*v}}

    /// The method that returns the product of a quaternion and a complex number
    ///
    /// Метод, возвращающий произведение кватерниона и комплексного числа
    ///
    /// # Example
    /// ```
    /// use tmn::complex::CNum;
    /// use tmn::quaternion::QNum;
    /// let mut a = QNum::make_from_r(4_f32, 4_f32, 10_f32, 10_f32);
    /// a = a.mult_c(CNum::make(6_f32, 6_f32));
    /// assert_eq!((0_f32, 48_f32, 120_f32, 0_f32), a.get());
    /// ```
    pub fn mult_c(&self, v:CNum) -> QNum{
        let (r, i) = v.get();
        let (r1, i1, j1, k1) = self.get();
        QNum {r:r1*r-i1*i, i:i1*r+r1*i, j:j1*r+k1*i, k:k1*r-j1*i}
    }
    /// The method that returns the product of quaternions
    ///
    /// Метод, возвращающий произведение кватернионов
    ///
    /// # Example
    /// ```
    /// use tmn::quaternion::QNum;
    /// let mut a = QNum::make_from_r(4_f32, 4_f32, 4_f32, 4_f32);
    /// a = a.mult_q(QNum::make_from_r(6_f32, 6_f32, 6_f32, 6_f32));
    /// assert_eq!((-48_f32, 48_f32, 48_f32, 48_f32), a.get());
    /// ```
    pub fn mult_q(&self, v:QNum) -> QNum{
        let (x1, y1, u1, v1) = self.get();
        let (x2, y2, u2, v2) = v.get();
        QNum {
            r:x1 * x2 - y1 * y2 - u1 * u2 - v1 * v2,
            i:x1 * y2 + y1 * x2 + u1 * v2 - v1 * u2,
            j:x1 * u2 - y1 * v2 + u1 * x2 + v1 * y2,
            k:x1 * v2 + y1 * u2 - u1 * y2 + v1 * x2}
    }
    ///The method that returns the inverse quaternion
    ///
    /// Метод, возвращающий обратный кватернион
    ///
    ///```
    /// use tmn::quaternion::QNum;
    /// let mut a = QNum::make_from_r(1_f32, 1_f32, 1_f32, 1_f32);
    /// a = a.inv();
    /// assert_eq!((0.25_f32, -0.25_f32, -0.25_f32, -0.25_f32), a.get());
    pub fn inv(&self) -> QNum{ self.conj().mult_r(1_f32/self.norm()) }
    ///The method for setting values to specific coefficients
    ///
    /// Метод для установки значений в конкретные коэффициенты
    ///
    /// # Example
    ///```
    /// use tmn::quaternion;
    /// use tmn::quaternion::QNum;
    /// let mut a = QNum::make_zero();
    /// a = a.set(quaternion::R|quaternion::J, 3_f32);
    /// assert_eq!((3_f32, 0_f32, 3_f32, 0_f32), a.get());
    /// ```

    pub fn set(&self, c:u8, v:f32) -> Self{
        let mut ret = self.clone();
        if cassette::cassette::eq(c, 0){
            ret.r = v;
        }
        if cassette::cassette::eq(c, 1){
            ret.i = v;
        }
        if cassette::cassette::eq(c, 2){
            ret.j = v;
        }
        if cassette::cassette::eq(c, 3){
            ret.k = v;
        }
        ret
    }
}

impl PartialEq for QNum{
    fn eq(&self, other: &Self) -> bool {
        self.get() == other.get()
    }
}
impl Neg for QNum {
    type Output = Self;
    ///Redefined negative operator
    ///
    ///Переопределенный оператор отрицательного значения
    ///
    /// # Example
    ///```
    /// use tmn::quaternion::QNum;
    /// let cnum = -QNum::make_from_r(3_f32, 4_f32, 1_f32, 2_f32);
    /// assert_eq!(cnum.get(), (-3_f32, -4_f32, -1_f32, -2_f32));
    /// ```
    fn neg(self) -> Self::Output {
        self.mult_r(-1_f32)
    }
}