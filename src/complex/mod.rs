pub struct CNum {
    r:f32,
    i:f32
}

impl CNum {
    pub fn make(r:f32, i:f32)->Self{ Self{r,i } }
    pub fn clone(&self) -> CNum{ CNum{r:self.r,i:self.i} }
    pub fn get(&self) -> (f32, f32){
        (self.r, self.i)
    }
    pub fn conj(&self) -> CNum{CNum{r:self.r, i:-self.i}}
    pub fn modl(&self) -> f32{self.mult_c(self.conj()).r.powf(0.5) }
    pub fn add_r(&self, v:f32) -> CNum{
        CNum{
            r:self.r + v,
            i:self.i
        }
    }
    pub fn add_c(&self, v:CNum) -> CNum{
        CNum{
            r:self.r + v.r,
            i:self.i + v.i
        }
    }
    pub fn mult_r(&self, v:f32) -> CNum{
        CNum{
            r: self.r * v,
            i: self.i * v
        }
    }
    pub fn mult_c(&self, v:CNum) -> CNum{
        let (r, i) = self.get();
        CNum{
            r:r * v.r - i * v.i,
            i:r * v.i + v.r * i
        }
    }
    pub fn div_r(&self, v:f32) -> CNum{
        CNum{
            r:self.r / v,
            i:self.i / v
        }
    }
    pub fn div_c(&self, v:CNum) -> CNum{
        let divisor = (v.mult_c(v.conj())).r;
        let numerator = self.mult_c(v.conj());
        numerator.div_r(divisor)
    }
    pub fn pow(&self, v:f32) ->CNum{
        CNum{
            r: self.r.powf(v)*(v * self.i.atan2(self.r)).cos(),
            i: self.r.powf(v)*(v * self.i.atan2(self.r)).sin()
        }
    }
}