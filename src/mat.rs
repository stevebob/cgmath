use core::cast::transmute;
use core::cmp::{Eq, Ord};
use core::ptr::to_unsafe_ptr;
use core::sys::size_of;
use core::vec::raw::buf_as_slice;

use std::cmp::FuzzyEq;

use angle::Angle;
use dim::{Dimensional, ToPtr};
use funs::common::*;
use funs::exponential::*;
use funs::triganomic::{sin, cos};
use num::conv::cast;
use num::kinds::{Float, Number};
use quat::{Quat, ToQuat};
use vec::{NumericVector, Vec2, Vec3, Vec4};

/**
 * The base square matrix trait
 *
 * # Type parameters
 *
 * * `T` - The type of the elements of the matrix. Should be a floating point type.
 * * `V` - The type of the row and column vectors. Should have components of a
 *         floating point type and have the same number of dimensions as the
 *         number of rows and columns in the matrix.
 */
pub trait Matrix<T,V>: Dimensional<V>, ToPtr<T>, Eq, Neg<self> {
    /**
     * # Return value
     *
     * The column vector at `i`
     */
    pure fn col(&self, i: uint) -> V;
    
    /**
     * # Return value
     *
     * The row vector at `i`
     */
    pure fn row(&self, i: uint) -> V;
    
    /**
     * # Return value
     *
     * The identity matrix
     */
    static pure fn identity() -> self;
    
    /**
     * # Return value
     *
     * A matrix with all elements set to zero
     */
    static pure fn zero() -> self;
    
    /**
     * # Return value
     *
     * The scalar multiplication of this matrix and `value`
     */
    pure fn mul_t(&self, value: T) -> self;
    
    /**
     * # Return value
     *
     * The matrix vector product of the matrix and `vec`
     */
    pure fn mul_v(&self, vec: &V) -> V;
    
    /**
     * # Return value
     *
     * The matrix addition of the matrix and `other`
     */
    pure fn add_m(&self, other: &self) -> self;
    
    /**
     * # Return value
     *
     * The difference between the matrix and `other`
     */
    pure fn sub_m(&self, other: &self) -> self;
    
    /**
     * # Return value
     *
     * The matrix product of the matrix and `other`
     */
    pure fn mul_m(&self, other: &self) -> self;
    
    /**
     * # Return value
     *
     * The matrix dot product of the matrix and `other`
     */
    pure fn dot(&self, other: &self) -> T;
    
    /**
     * # Return value
     *
     * The determinant of the matrix
     */
    pure fn determinant(&self) -> T;
    
    /**
     * # Return value
     *
     * The sum of the main diagonal of the matrix
     */
    pure fn trace(&self) -> T;
    
    /**
     * Returns the inverse of the matrix
     * 
     * # Return value
     *
     * * `Some(m)` - if the inversion was successful, where `m` is the inverted matrix
     * * `None` - if the inversion was unsuccessful (because the matrix was not invertable)
     */
    pure fn inverse(&self) -> Option<self>;
    
    /**
     * # Return value
     *
     * The transposed matrix
     */
    pure fn transpose(&self) -> self;
    
    /**
     * Check to see if the matrix is an identity matrix
     *
     * # Return value
     * 
     * `true` if the matrix is approximately equal to the identity matrix
     */
    pure fn is_identity(&self) -> bool;
    
    /**
     * Check to see if the matrix is diagonal
     *
     * # Return value
     *
     * `true` all the elements outside the main diagonal are approximately
     * equal to zero.
     */
    pure fn is_diagonal(&self) -> bool;
    
    /**
     * Check to see if the matrix is rotated
     *
     * # Return value
     *
     * `true` if the matrix is not approximately equal to the identity matrix.
     */
    pure fn is_rotated(&self) -> bool;
    
    /**
     * Check to see if the matrix is symmetric
     *
     * # Return value
     *
     * `true` if the matrix is approximately equal to its transpose).
     */
    pure fn is_symmetric(&self) -> bool;
    
    /**
     * Check to see if the matrix is invertable
     *
     * # Return value
     *
     * `true` if  the matrix is invertable
     */
    pure fn is_invertible(&self) -> bool;
}

/**
 * A mutable matrix
 */
pub trait MutableMatrix<T,V>: Matrix<T,V> {
    /**
     * # Return value
     *
     * A mutable reference to the column at `i`
     */
    fn col_mut(&mut self, i: uint) -> &self/mut V;
    
    /**
     * Swap two columns of the matrix in place
     */
    fn swap_cols(&mut self, a: uint, b: uint);
    
    /**
     * Swap two rows of the matrix in place 
     */
    fn swap_rows(&mut self, a: uint, b: uint);
    
    /**
     * Sets the matrix to `other` 
     */
    fn set(&mut self, other: &self);
    
    /**
     * Sets the matrix to the identity matrix
     */
    fn to_identity(&mut self);
    
    /**
     * Sets each element of the matrix to zero
     */
    fn to_zero(&mut self);

    /**
     * Multiplies the matrix by a scalar
     */
    fn mul_self_t(&mut self, value: T);
    
    /**
     * Add the matrix `other` to `self`
     */
    fn add_self_m(&mut self, other: &self);
    
    /**
     * Subtract the matrix `other` from `self`
     */
    fn sub_self_m(&mut self, other: &self);
    
    /**
     * Sets the matrix to its inverse
     * 
     * # Failure
     *
     * Fails if the matrix is not invertable. Make sure you check with the
     * `is_invertible` method before you attempt this!
     */
    fn invert_self(&mut self);
    
    /**
     * Sets the matrix to its transpose
     */
    fn transpose_self(&mut self);
}

/**
 * A 2 x 2 matrix
 */
pub trait Matrix2<T,V>: Matrix<T,V> {
    pure fn to_mat3(&self) -> Mat3<T>;
    pure fn to_mat4(&self) -> Mat4<T>;
}

/**
 * A 3 x 3 matrix
 */
pub trait Matrix3<T,V>: Matrix<T,V> {
    static pure fn from_axis_angle<A:Angle<T>>(axis: &Vec3<T>, theta: A) -> Mat3<T>;
    pure fn to_mat4(&self) -> Mat4<T>;
}

/**
 * A 4 x 4 matrix
 */
pub trait Matrix4<T,V>: Matrix<T,V> {
}






/**
 *  A 2 x 2 column major matrix
 *
 * # Type parameters
 *
 * * `T` - The type of the elements of the matrix. Should be a floating point type.
 *
 * # Fields
 *
 * * `x` - the first column vector of the matrix
 * * `y` - the second column vector of the matrix
 * * `z` - the third column vector of the matrix
 */
pub struct Mat2<T> { x: Vec2<T>, y: Vec2<T> }

pub impl<T:Copy Float> Mat2<T> {
    /**
     * Construct a 2 x 2 matrix
     *
     * # Arguments
     *
     * * `c0r0`, `c0r1` - the first column of the matrix
     * * `c1r0`, `c1r1` - the second column of the matrix
     *
     * ~~~
     *        c0     c1
     *     +------+------+
     *  r0 | c0r0 | c1r0 |
     *     +------+------+
     *  r1 | c0r1 | c1r1 |
     *     +------+------+
     * ~~~
     */
    #[inline(always)]
    static pure fn new(c0r0: T, c0r1: T,
                       c1r0: T, c1r1: T) -> Mat2<T> {
        Mat2::from_cols(Vec2::new(move c0r0, move c0r1),
                        Vec2::new(move c1r0, move c1r1))
    }
    
    /**
     * Construct a 2 x 2 matrix from column vectors
     *
     * # Arguments
     *
     * * `c0` - the first column vector of the matrix
     * * `c1` - the second column vector of the matrix
     *
     * ~~~
     *        c0     c1
     *     +------+------+
     *  r0 | c0.x | c1.x |
     *     +------+------+
     *  r1 | c0.y | c1.y |
     *     +------+------+
     * ~~~
     */
    #[inline(always)]
    static pure fn from_cols(c0: Vec2<T>, c1: Vec2<T>) -> Mat2<T> {
        Mat2 { x: move c0,
               y: move c1 }
    }
    
    /**
     * Construct a 2 x 2 diagonal matrix with the major diagonal set to `value`
     *
     * # Arguments
     *
     * * `value` - the value to set the major diagonal to
     *
     * ~~~
     *        c0    c1
     *     +-----+-----+
     *  r0 | val |   0 |
     *     +-----+-----+
     *  r1 |   0 | val |
     *     +-----+-----+
     * ~~~
     */
    #[inline(always)]
    static pure fn from_value(value: T) -> Mat2<T> {
        let _0 = cast(0);
        // let _0 = Number::from(0);    // FIXME: causes ICE
        Mat2::new(value,    _0,
                     _0, value)
    }
    
    // FIXME: An interim solution to the issues with static functions
    #[inline(always)]
    static pure fn identity() -> Mat2<T> {
        let _0 = cast(0);
        let _1 = cast(1);
        // let _0 = Number::from(0);    // FIXME: causes ICE
        // let _1 = Number::from(1);    // FIXME: causes ICE
        Mat2::new(_1, _0,
                  _0, _1)
    }
    
    // FIXME: An interim solution to the issues with static functions
    #[inline(always)]
    static pure fn zero() -> Mat2<T> {
        let _0 = cast(0);
        // let _0 = Number::from(0);    // FIXME: causes ICE
        Mat2::new(_0, _0,
                  _0, _0)
    }
}

pub impl<T:Copy Float> Mat2<T>: Matrix<T, Vec2<T>> {
    #[inline(always)]
    pure fn col(&self, i: uint) -> Vec2<T> { self[i] }
    
    #[inline(always)]
    pure fn row(&self, i: uint) -> Vec2<T> {
        Vec2::new(self[0][i],
                  self[1][i])
    }
    
    /**
     * Returns the multiplicative identity matrix
     * ~~~
     *       c0   c1
     *     +----+----+ 
     *  r0 |  1 |  0 |
     *     +----+----+
     *  r1 |  0 |  1 |
     *     +----+----+
     * ~~~
     */
    #[inline(always)]
    static pure fn identity() -> Mat2<T> {
        let _0 = Number::from(0);
        let _1 = Number::from(1);
        Mat2::new(_1, _0,
                  _0, _1)
    }
    
    /**
     * Returns the additive identity matrix
     * ~~~
     *       c0   c1
     *     +----+----+ 
     *  r0 |  0 |  0 |
     *     +----+----+
     *  r1 |  0 |  0 |
     *     +----+----+
     * ~~~
     */
    #[inline(always)]
    static pure fn zero() -> Mat2<T> {
        let _0 = Number::from(0);
        Mat2::new(_0, _0,
                  _0, _0)
    }
    
    #[inline(always)]
    pure fn mul_t(&self, value: T) -> Mat2<T> {
        Mat2::from_cols(self[0].mul_t(value),
                        self[1].mul_t(value))
    }
    
    #[inline(always)]
    pure fn mul_v(&self, vec: &Vec2<T>) -> Vec2<T> {
        Vec2::new(self.row(0).dot(vec),
                  self.row(1).dot(vec))
    }
    
    #[inline(always)]
    pure fn add_m(&self, other: &Mat2<T>) -> Mat2<T> {
        Mat2::from_cols(self[0].add_v(&other[0]),
                        self[1].add_v(&other[1]))
    }
    
    #[inline(always)]
    pure fn sub_m(&self, other: &Mat2<T>) -> Mat2<T> {
        Mat2::from_cols(self[0].sub_v(&other[0]),
                        self[1].sub_v(&other[1]))
    }
    
    #[inline(always)]
    pure fn mul_m(&self, other: &Mat2<T>) -> Mat2<T> {
        Mat2::new(self.row(0).dot(&other.col(0)), self.row(1).dot(&other.col(0)),
                  self.row(0).dot(&other.col(1)), self.row(1).dot(&other.col(1)))
    }

    pure fn dot(&self, other: &Mat2<T>) -> T {
        other.transpose().mul_m(self).trace()
    }
    
    pure fn determinant(&self) -> T {
       self[0][0] * self[1][1] - self[1][0] * self[0][1]
    }

    pure fn trace(&self) -> T {
        self[0][0] + self[1][1]
    }

    #[inline(always)]
    pure fn inverse(&self) -> Option<Mat2<T>> {
        let _0 = cast(0);
        // let _0 = Number::from(0);                // FIXME: causes ICE
        let d = self.determinant();
        if d.fuzzy_eq(&_0) {
            None
        } else {
            Some(Mat2::new( self[1][1]/d, -self[0][1]/d,
                           -self[1][0]/d,  self[0][0]/d))
        }
    }
    
    #[inline(always)]
    pure fn transpose(&self) -> Mat2<T> {
        Mat2::new(self[0][0], self[1][0],
                  self[0][1], self[1][1])
    }
    
    #[inline(always)]
    pure fn is_identity(&self) -> bool {
        // self.fuzzy_eq(&Matrix::identity())     // FIXME: there's something wrong with static functions here!
        self.fuzzy_eq(&Mat2::identity())
    }
    
    #[inline(always)]
    pure fn is_diagonal(&self) -> bool {
        let _0 = cast(0);
        // let _0 = Number::from(0);                // FIXME: causes ICE
        self[0][1].fuzzy_eq(&_0) &&
        self[1][0].fuzzy_eq(&_0)
    }
    
    #[inline(always)]
    pure fn is_rotated(&self) -> bool {
        // !self.fuzzy_eq(&Matrix::identity())     // FIXME: there's something wrong with static functions here!
        !self.fuzzy_eq(&Mat2::identity())
    }
    
    #[inline(always)]
    pure fn is_symmetric(&self) -> bool {
        self[0][1].fuzzy_eq(&self[1][0]) &&
        self[1][0].fuzzy_eq(&self[0][1])
    }

    #[inline(always)]
    pure fn is_invertible(&self) -> bool {
        let _0 = cast(0);
        // let _0 = Number::from(0);                // FIXME: causes ICE
        !self.determinant().fuzzy_eq(&_0)
    }
}

pub impl<T:Copy Float Sign> Mat2<T>: MutableMatrix<T, Vec2<T>> {
    #[inline(always)]
    fn col_mut(&mut self, i: uint) -> &self/mut Vec2<T> {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => fail(fmt!("index out of bounds: expected an index from 0 to 1, but found %u", i))
        }
    }
    
    #[inline(always)]
    fn swap_cols(&mut self, a: uint, b: uint) {
        util::swap(self.col_mut(a),
                   self.col_mut(b));
    }
    
    #[inline(always)]
    fn swap_rows(&mut self, a: uint, b: uint) {
        self.x.swap(a, b);
        self.y.swap(a, b);
    }
    
    #[inline(always)]
    fn set(&mut self, other: &Mat2<T>) {
        (*self) = (*other);
    }
    
    #[inline(always)]
    fn to_identity(&mut self) {
        (*self) = Mat2::identity();
    }
    
    #[inline(always)]
    fn to_zero(&mut self) {
        (*self) = Mat2::zero();
    }
    
    #[inline(always)]
    fn mul_self_t(&mut self, value: T) {
        self.col_mut(0).mul_self_t(&value);
        self.col_mut(1).mul_self_t(&value);
    }
    
    #[inline(always)]
    fn add_self_m(&mut self, other: &Mat2<T>) {
        self.col_mut(0).add_self_v(&other[0]);
        self.col_mut(1).add_self_v(&other[1]);
    }
    
    #[inline(always)]
    fn sub_self_m(&mut self, other: &Mat2<T>) {
        self.col_mut(0).sub_self_v(&other[0]);
        self.col_mut(1).sub_self_v(&other[1]);
    }
    
    #[inline(always)]
    fn invert_self(&mut self) {
        match self.inverse() {
            Some(m) => (*self) = m,
            None => fail(~"Couldn't invert the matrix!")
        }
    }
    
    #[inline(always)]
    fn transpose_self(&mut self) {
        util::swap(self.col_mut(0).index_mut(1), self.col_mut(1).index_mut(0));
        util::swap(self.col_mut(1).index_mut(0), self.col_mut(0).index_mut(1));
    }
}

pub impl<T:Copy Float> Mat2<T>: Matrix2<T, Vec2<T>> {
    #[inline(always)]
    pure fn to_mat3(&self) -> Mat3<T> {
        Mat3::from_Mat2(self)
    }
    
    #[inline(always)]
    pure fn to_mat4(&self) -> Mat4<T> {
        Mat4::from_Mat2(self)
    }
}

pub impl<T:Copy> Mat2<T>: Dimensional<Vec2<T>> {
    #[inline(always)]
    static pure fn dim() -> uint { 2 }
    
    #[inline(always)]
    static pure fn size_of() -> uint { size_of::<Mat2<T>>() }
}

pub impl<T:Copy> Mat2<T>: Index<uint, Vec2<T>> {
    #[inline(always)]
    pure fn index(&self, i: uint) -> Vec2<T> {
        unsafe { do buf_as_slice(
            transmute::<*Mat2<T>, *Vec2<T>>(
                to_unsafe_ptr(self)), 2) |slice| { slice[i] }
        }
    }
}

pub impl<T:Copy> Mat2<T>: ToPtr<T> {
    #[inline(always)]
    pure fn to_ptr(&self) -> *T {
        unsafe {
            transmute::<*Mat2<T>, *T>(
                to_unsafe_ptr(self)
            )
        }
    }
}

pub impl<T:Copy Float> Mat2<T>: Neg<Mat2<T>> {
    #[inline(always)]
    pure fn neg(&self) -> Mat2<T> {
        Mat2::from_cols(-self[0], -self[1])
    }
}

pub impl<T:Copy Float> Mat2<T>: Eq {
    #[inline(always)]
    pure fn eq(&self, other: &Mat2<T>) -> bool {
        self[0] == other[0] &&
        self[1] == other[1]
    }
    
    #[inline(always)]
    pure fn ne(&self, other: &Mat2<T>) -> bool {
        !(self == other)
    }
}

pub impl<T:Copy Float> Mat2<T>: FuzzyEq {
    #[inline(always)]
    pure fn fuzzy_eq(other: &Mat2<T>) -> bool {
        self[0].fuzzy_eq(&other[0]) &&
        self[1].fuzzy_eq(&other[1])
    }
}






/**
 *  A 3 x 3 column major matrix
 *
 * # Type parameters
 *
 * * `T` - The type of the elements of the matrix. Should be a floating point type.
 *
 * # Fields
 *
 * * `x` - the first column vector of the matrix
 * * `y` - the second column vector of the matrix
 * * `z` - the third column vector of the matrix
 */
pub struct Mat3<T> { x: Vec3<T>, y: Vec3<T>, z: Vec3<T> }

pub impl<T:Copy Float> Mat3<T> {
    /**
     * Construct a 3 x 3 matrix
     *
     * # Arguments
     *
     * * `c0r0`, `c0r1`, `c0r2` - the first column of the matrix
     * * `c1r0`, `c1r1`, `c1r2` - the second column of the matrix
     * * `c2r0`, `c2r1`, `c2r2` - the third column of the matrix
     *
     * ~~~
     *         c0     c1     c2
     *      +------+------+------+
     *   r0 | c0r0 | c1r0 | c2r0 |
     *      +------+------+------+
     *   r1 | c0r1 | c1r1 | c2r1 |
     *      +------+------+------+
     *   r2 | c0r2 | c1r2 | c2r2 |
     *      +------+------+------+
     * ~~~
     */
    #[inline(always)]
    static pure fn new(c0r0:T, c0r1:T, c0r2:T,
                       c1r0:T, c1r1:T, c1r2:T,
                       c2r0:T, c2r1:T, c2r2:T) -> Mat3<T> {
        Mat3::from_cols(Vec3::new(move c0r0, move c0r1, move c0r2),
                        Vec3::new(move c1r0, move c1r1, move c1r2),
                        Vec3::new(move c2r0, move c2r1, move c2r2))
    }
    
    /**
     * Construct a 3 x 3 matrix from column vectors
     *
     * # Arguments
     *
     * * `c0` - the first column vector of the matrix
     * * `c1` - the second column vector of the matrix
     * * `c2` - the third column vector of the matrix
     *
     * ~~~
     *        c0     c1     c2
     *     +------+------+------+
     *  r0 | c0.x | c1.y | c2.z |
     *     +------+------+------+
     *  r1 | c0.x | c1.y | c2.z |
     *     +------+------+------+
     *  r2 | c0.x | c1.y | c2.z |
     *     +------+------+------+
     * ~~~
     */
    #[inline(always)]
    static pure fn from_cols(c0: Vec3<T>, c1: Vec3<T>, c2: Vec3<T>) -> Mat3<T> {
        Mat3 { x: move c0,
               y: move c1,
               z: move c2 }
    }
    
    /**
     * Construct a 3 x 3 diagonal matrix with the major diagonal set to `value`
     *
     * # Arguments
     *
     * * `value` - the value to set the major diagonal to
     *
     * ~~~
     *        c0    c1    c2
     *     +-----+-----+-----+
     *  r0 | val |   0 |   0 |
     *     +-----+-----+-----+
     *  r1 |   0 | val |   0 |
     *     +-----+-----+-----+
     *  r2 |   0 |   0 | val |
     *     +-----+-----+-----+
     * ~~~
     */
    #[inline(always)]
    static pure fn from_value(value: T) -> Mat3<T> {
        let _0 = cast(0);
        // let _0 = Number::from(0);                // FIXME: causes ICE
        Mat3::new(value,    _0,    _0,
                     _0, value,    _0,
                     _0,    _0, value)
    }
    
    #[inline(always)]
    static pure fn from_Mat2(m: &Mat2<T>) -> Mat3<T> {
        let _0 = cast(0);
        let _1 = cast(1);
        // let _0 = Number::from(0);                // FIXME: causes ICE
        // let _1 = Number::from(1);                // FIXME: causes ICE
        Mat3::new(m[0][0], m[0][1], _0,
                  m[1][0], m[1][1], _0,
                       _0,      _0, _1)
    }
    
    // FIXME: An interim solution to the issues with static functions
    #[inline(always)]
    static pure fn identity() -> Mat3<T> {
        let _0 = cast(0);
        let _1 = cast(1);
        Mat3::new(_1, _0, _0,
                  _0, _1, _0,
                  _0, _0, _1)
    }
    
    // FIXME: An interim solution to the issues with static functions
    #[inline(always)]
    static pure fn zero() -> Mat3<T> {
        let _0 = cast(0);
        Mat3::new(_0, _0, _0,
                  _0, _0, _0,
                  _0, _0, _0)
    }
}

pub impl<T:Copy Float> Mat3<T>: Matrix<T, Vec3<T>> {
    #[inline(always)]
    pure fn col(&self, i: uint) -> Vec3<T> { self[i] }
    
    #[inline(always)]
    pure fn row(&self, i: uint) -> Vec3<T> {
        Vec3::new(self[0][i],
                  self[1][i],
                  self[2][i])
    }
    
    /**
     * Returns the multiplicative identity matrix
     * ~~~
     *       c0   c1   c2
     *     +----+----+----+
     *  r0 |  1 |  0 |  0 |
     *     +----+----+----+
     *  r1 |  0 |  1 |  0 |
     *     +----+----+----+
     *  r2 |  0 |  0 |  1 |
     *     +----+----+----+
     * ~~~
     */
    #[inline(always)]
    static pure fn identity() -> Mat3<T> {
        // let _0 = cast(0);
        // let _1 = cast(1);
        let _0 = Number::from(0);
        let _1 = Number::from(1);
        Mat3::new(_1, _0, _0,
                  _0, _1, _0,
                  _0, _0, _1)
    }
    
    /**
     * Returns the additive identity matrix
     * ~~~
     *       c0   c1   c2
     *     +----+----+----+
     *  r0 |  0 |  0 |  0 |
     *     +----+----+----+
     *  r1 |  0 |  0 |  0 |
     *     +----+----+----+
     *  r2 |  0 |  0 |  0 |
     *     +----+----+----+
     * ~~~
     */
    #[inline(always)]
    static pure fn zero() -> Mat3<T> {
        let _0 = Number::from(0);
        Mat3::new(_0, _0, _0,
                  _0, _0, _0,
                  _0, _0, _0)
    }
    
    #[inline(always)]
    pure fn mul_t(&self, value: T) -> Mat3<T> {
        Mat3::from_cols(self[0].mul_t(value),
                        self[1].mul_t(value),
                        self[2].mul_t(value))
    }
    
    #[inline(always)]
    pure fn mul_v(&self, vec: &Vec3<T>) -> Vec3<T> {
        Vec3::new(self.row(0).dot(vec),
                  self.row(1).dot(vec),
                  self.row(2).dot(vec))
    }
    
    #[inline(always)]
    pure fn add_m(&self, other: &Mat3<T>) -> Mat3<T> {
        Mat3::from_cols(self[0].add_v(&other[0]),
                        self[1].add_v(&other[1]),
                        self[2].add_v(&other[2]))
    }
    
    #[inline(always)]
    pure fn sub_m(&self, other: &Mat3<T>) -> Mat3<T> {
        Mat3::from_cols(self[0].sub_v(&other[0]),
                        self[1].sub_v(&other[1]),
                        self[2].sub_v(&other[2]))
    }
    
    #[inline(always)]
    pure fn mul_m(&self, other: &Mat3<T>) -> Mat3<T> {
        Mat3::new(self.row(0).dot(&other.col(0)), self.row(1).dot(&other.col(0)), self.row(2).dot(&other.col(0)),
                  self.row(0).dot(&other.col(1)), self.row(1).dot(&other.col(1)), self.row(2).dot(&other.col(1)),
                  self.row(0).dot(&other.col(2)), self.row(1).dot(&other.col(2)), self.row(2).dot(&other.col(2)))
    }
    
    pure fn dot(&self, other: &Mat3<T>) -> T {
        other.transpose().mul_m(self).trace()
    }

    pure fn determinant(&self) -> T {
        self.col(0).dot(&self.col(1).cross(&self.col(2)))
    }

    pure fn trace(&self) -> T {
        self[0][0] + self[1][1] + self[2][2]
    }

    // #[inline(always)]
    pure fn inverse(&self) -> Option<Mat3<T>> {
        let d = self.determinant();
        let _0 = cast(0);
        // let _0 = Number::from(0);                // FIXME: causes ICE
        if d.fuzzy_eq(&_0) {
            None
        } else {
            Some(Mat3::from_cols(self[1].cross(&self[2]).div_t(d),
                                 self[2].cross(&self[0]).div_t(d),
                                 self[0].cross(&self[1]).div_t(d))
            .transpose())
        }
    }
    
    #[inline(always)]
    pure fn transpose(&self) -> Mat3<T> {
        Mat3::new(self[0][0], self[1][0], self[2][0],
                  self[0][1], self[1][1], self[2][1],
                  self[0][2], self[1][2], self[2][2])
    }
    
    #[inline(always)]
    pure fn is_identity(&self) -> bool {
        // self.fuzzy_eq(&Matrix::identity())     // FIXME: there's something wrong with static functions here!
        self.fuzzy_eq(&Mat3::identity())
    }
    
    #[inline(always)]
    pure fn is_diagonal(&self) -> bool {
        let _0 = cast(0);
        // let _0 = Number::from(0);                // FIXME: causes ICE
        self[0][1].fuzzy_eq(&_0) &&
        self[0][2].fuzzy_eq(&_0) &&
        
        self[1][0].fuzzy_eq(&_0) &&
        self[1][2].fuzzy_eq(&_0) &&
        
        self[2][0].fuzzy_eq(&_0) &&
        self[2][1].fuzzy_eq(&_0)
    }
    
    #[inline(always)]
    pure fn is_rotated(&self) -> bool {
        // !self.fuzzy_eq(&Matrix::identity())     // FIXME: there's something wrong with static functions here!
        !self.fuzzy_eq(&Mat3::identity())
    }
    
    #[inline(always)]
    pure fn is_symmetric(&self) -> bool {
        self[0][1].fuzzy_eq(&self[1][0]) &&
        self[0][2].fuzzy_eq(&self[2][0]) &&
        
        self[1][0].fuzzy_eq(&self[0][1]) &&
        self[1][2].fuzzy_eq(&self[2][1]) &&
        
        self[2][0].fuzzy_eq(&self[0][2]) &&
        self[2][1].fuzzy_eq(&self[1][2])
    }

    #[inline(always)]
    pure fn is_invertible(&self) -> bool {
        let _0 = cast(0);
        // let _0 = Number::from(0);                // FIXME: causes ICE
        !self.determinant().fuzzy_eq(&_0)
    }
}

pub impl<T:Copy Float Sign> Mat3<T>: MutableMatrix<T, Vec3<T>> {
    #[inline(always)]
    fn col_mut(&mut self, i: uint) -> &self/mut Vec3<T> {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => fail(fmt!("index out of bounds: expected an index from 0 to 2, but found %u", i))
        }
    }
    
    #[inline(always)]
    fn swap_cols(&mut self, a: uint, b: uint) {
        util::swap(self.col_mut(a),
                   self.col_mut(b));
    }
    
    #[inline(always)]
    fn swap_rows(&mut self, a: uint, b: uint) {
        self.x.swap(a, b);
        self.y.swap(a, b);
        self.z.swap(a, b);
    }
    
    #[inline(always)]
    fn set(&mut self, other: &Mat3<T>) {
        (*self) = (*other);
    }
    
    #[inline(always)]
    fn to_identity(&mut self) {
        (*self) = Mat3::identity();
    }
    
    #[inline(always)]
    fn to_zero(&mut self) {
        (*self) = Mat3::zero();
    }
    
    #[inline(always)]
    fn mul_self_t(&mut self, value: T) {
        self.col_mut(0).mul_self_t(&value);
        self.col_mut(1).mul_self_t(&value);
        self.col_mut(2).mul_self_t(&value);
    }
    
    #[inline(always)]
    fn add_self_m(&mut self, other: &Mat3<T>) {
        self.col_mut(0).add_self_v(&other[0]);
        self.col_mut(1).add_self_v(&other[1]);
        self.col_mut(2).add_self_v(&other[2]);
    }
    
    #[inline(always)]
    fn sub_self_m(&mut self, other: &Mat3<T>) {
        self.col_mut(0).sub_self_v(&other[0]);
        self.col_mut(1).sub_self_v(&other[1]);
        self.col_mut(2).sub_self_v(&other[2]);
    }
    
    #[inline(always)]
    fn invert_self(&mut self) {
        match self.inverse() {
            Some(m) => (*self) = m,
            None => fail(~"Couldn't invert the matrix!")
        }
    }
    
    #[inline(always)]
    fn transpose_self(&mut self) {
        util::swap(self.col_mut(0).index_mut(1), self.col_mut(1).index_mut(0));
        util::swap(self.col_mut(0).index_mut(2), self.col_mut(2).index_mut(0));
        
        util::swap(self.col_mut(1).index_mut(0), self.col_mut(0).index_mut(1));
        util::swap(self.col_mut(1).index_mut(2), self.col_mut(2).index_mut(1));
        
        util::swap(self.col_mut(2).index_mut(0), self.col_mut(0).index_mut(2));
        util::swap(self.col_mut(2).index_mut(1), self.col_mut(1).index_mut(2));
    }
}

pub impl<T:Copy Float> Mat3<T>: Matrix3<T, Vec3<T>> {
    #[inline(always)]
    static pure fn from_axis_angle<A:Angle<T>>(axis: &Vec3<T>, theta: A) -> Mat3<T> {
        let c:  T = cos(&theta.to_radians());
        let s:  T = sin(&theta.to_radians());
        let _0: T = cast(0);
        let _1: T = cast(1);
        // let _0: T = Number::from(0);    // FIXME: causes ICE
        // let _1: T = Number::from(1);    // FIXME: causes ICE
        let _1_c:  T = _1 - c;
        
        let x = axis.x;
        let y = axis.y;
        let z = axis.z;
        
        Mat3::new(_1_c * x * x + c,       _1_c * x * y + s * z,   _1_c * x * z - s * y,
                  _1_c * x * y - s * z,   _1_c * y * y + c,       _1_c * y * z + s * x,
                  _1_c * x * z + s * y,   _1_c * y * z - s * x,   _1_c * z * z + c)
    }
    
    #[inline(always)]
    pure fn to_mat4(&self) -> Mat4<T> {
        Mat4::from_Mat3(self)
    }
}

pub impl<T:Copy Float Exp> Mat3<T>: ToQuat<T> {
    pure fn to_Quat() -> Quat<T> {
        // Implemented using a mix of ideas from jMonkeyEngine and Ken Shoemake's
        // paper on Quaternions: http://www.cs.ucr.edu/~vbz/resources/Quatut.pdf
        
        let mut s;
        let w, x, y, z;
        let trace = self.trace();
        
        let _1:   T = Number::from(1.0);
        let half: T = Number::from(0.5);
        
        if trace >= Number::from(0) {
            s = (_1 + trace).sqrt();
            w = half * s;
            s = half / s;
            x = (self[1][2] - self[2][1]) * s;
            y = (self[2][0] - self[0][2]) * s;
            z = (self[0][1] - self[1][0]) * s;
        } else if (self[0][0] > self[1][1]) && (self[0][0] > self[2][2]) {
            s = (half + (self[0][0] - self[1][1] - self[2][2])).sqrt();
            w = half * s;
            s = half / s;
            x = (self[0][1] - self[1][0]) * s;
            y = (self[2][0] - self[0][2]) * s;
            z = (self[1][2] - self[2][1]) * s;
        } else if self[1][1] > self[2][2] {
            s = (half + (self[1][1] - self[0][0] - self[2][2])).sqrt();
            w = half * s;
            s = half / s;
            x = (self[0][1] - self[1][0]) * s;
            y = (self[1][2] - self[2][1]) * s;
            z = (self[2][0] - self[0][2]) * s;
        } else {
            s = (half + (self[2][2] - self[0][0] - self[1][1])).sqrt();
            w = half * s;
            s = half / s;
            x = (self[2][0] - self[0][2]) * s;
            y = (self[1][2] - self[2][1]) * s;
            z = (self[0][1] - self[1][0]) * s;
        }
        
        Quat::new(w, x, y, z)
    }
}

pub impl<T:Copy> Mat3<T>: Dimensional<Vec3<T>> {
    #[inline(always)]
    static pure fn dim() -> uint { 3 }
    
    #[inline(always)]
    static pure fn size_of() -> uint { size_of::<Mat3<T>>() }
}

pub impl<T:Copy> Mat3<T>: Index<uint, Vec3<T>> {
    #[inline(always)]
    pure fn index(&self, i: uint) -> Vec3<T> {
        unsafe { do buf_as_slice(
            transmute::<*Mat3<T>, *Vec3<T>>(
                to_unsafe_ptr(self)), 3) |slice| { slice[i] }
        }
    }
}

pub impl<T:Copy> Mat3<T>: ToPtr<T> {
    #[inline(always)]
    pure fn to_ptr(&self) -> *T {
        unsafe {
            transmute::<*Mat3<T>, *T>(
                to_unsafe_ptr(self)
            )
        }
    }
}

pub impl<T:Copy Float> Mat3<T>: Neg<Mat3<T>> {
    #[inline(always)]
    pure fn neg(&self) -> Mat3<T> {
        Mat3::from_cols(-self[0], -self[1], -self[2])
    }
}

pub impl<T:Copy Float> Mat3<T>: Eq {
    #[inline(always)]
    pure fn eq(&self, other: &Mat3<T>) -> bool {
        self[0] == other[0] &&
        self[1] == other[1] &&
        self[2] == other[2]
    }
    
    #[inline(always)]
    pure fn ne(&self, other: &Mat3<T>) -> bool {
        !(self == other)
    }
}

pub impl<T:Copy Float> Mat3<T>: FuzzyEq {
    #[inline(always)]
    pure fn fuzzy_eq(other: &Mat3<T>) -> bool {
        self[0].fuzzy_eq(&other[0]) &&
        self[1].fuzzy_eq(&other[1]) &&
        self[2].fuzzy_eq(&other[2])
    }
}






/**
 *  A 4 x 4 column major matrix
 *
 * # Type parameters
 *
 * * `T` - The type of the elements of the matrix. Should be a floating point type.
 *
 * # Fields
 *
 * * `x` - the first column vector of the matrix
 * * `y` - the second column vector of the matrix
 * * `z` - the third column vector of the matrix
 * * `w` - the fourth column vector of the matrix
 */
pub struct Mat4<T> { x: Vec4<T>, y: Vec4<T>, z: Vec4<T>, w: Vec4<T> }

pub impl<T:Copy Float> Mat4<T> {
    /**
     * Construct a 4 x 4 matrix
     *
     * # Arguments
     *
     * * `c0r0`, `c0r1`, `c0r2`, `c0r3` - the first column of the matrix
     * * `c1r0`, `c1r1`, `c1r2`, `c1r3` - the second column of the matrix
     * * `c2r0`, `c2r1`, `c2r2`, `c2r3` - the third column of the matrix
     * * `c3r0`, `c3r1`, `c3r2`, `c3r3` - the fourth column of the matrix
     *
     * ~~~
     *        c0     c1     c2     c3
     *     +------+------+------+------+
     *  r0 | c0r0 | c1r0 | c2r0 | c3r0 |
     *     +------+------+------+------+
     *  r1 | c0r1 | c1r1 | c2r1 | c3r1 |
     *     +------+------+------+------+
     *  r2 | c0r2 | c1r2 | c2r2 | c3r2 |
     *     +------+------+------+------+
     *  r3 | c0r3 | c1r3 | c2r3 | c3r3 |
     *     +------+------+------+------+
     * ~~~
     */
    #[inline(always)]
    static pure fn new(c0r0: T, c0r1: T, c0r2: T, c0r3: T,
                       c1r0: T, c1r1: T, c1r2: T, c1r3: T,
                       c2r0: T, c2r1: T, c2r2: T, c2r3: T,
                       c3r0: T, c3r1: T, c3r2: T, c3r3: T) -> Mat4<T>  {
        Mat4::from_cols(Vec4::new(move c0r0, move c0r1, move c0r2, move c0r3),
                        Vec4::new(move c1r0, move c1r1, move c1r2, move c1r3),
                        Vec4::new(move c2r0, move c2r1, move c2r2, move c2r3),
                        Vec4::new(move c3r0, move c3r1, move c3r2, move c3r3))
    }
    
    /**
     * Construct a 4 x 4 matrix from column vectors
     *
     * # Arguments
     *
     * * `c0` - the first column vector of the matrix
     * * `c1` - the second column vector of the matrix
     * * `c2` - the third column vector of the matrix
     * * `c3` - the fourth column vector of the matrix
     *
     * ~~~
     *        c0     c1     c2     c3
     *     +------+------+------+------+
     *  r0 | c0.x | c1.x | c2.x | c3.x |
     *     +------+------+------+------+
     *  r1 | c0.y | c1.y | c2.y | c3.y |
     *     +------+------+------+------+
     *  r2 | c0.z | c1.z | c2.z | c3.z |
     *     +------+------+------+------+
     *  r3 | c0.w | c1.w | c2.w | c3.w |
     *     +------+------+------+------+
     * ~~~
     */
    #[inline(always)]
    static pure fn from_cols(c0: Vec4<T>, c1: Vec4<T>, c2: Vec4<T>, c3: Vec4<T>) -> Mat4<T> {
        Mat4 { x: move c0,
               y: move c1,
               z: move c2,
               w: move c3 }
    }
    
    /**
     * Construct a 4 x 4 diagonal matrix with the major diagonal set to `value`
     *
     * # Arguments
     *
     * * `value` - the value to set the major diagonal to
     *
     * ~~~
     *        c0    c1    c2    c3
     *     +-----+-----+-----+-----+
     *  r0 | val |   0 |   0 |   0 |
     *     +-----+-----+-----+-----+
     *  r1 |   0 | val |   0 |   0 |
     *     +-----+-----+-----+-----+
     *  r2 |   0 |   0 | val |   0 |
     *     +-----+-----+-----+-----+
     *  r3 |   0 |   0 |   0 | val |
     *     +-----+-----+-----+-----+
     * ~~~
     */
    #[inline(always)]
    static pure fn from_value(value: T) -> Mat4<T> {
        let _0 = cast(0);
        Mat4::new(value,    _0,    _0,    _0,
                     _0, value,    _0,    _0,
                     _0,    _0, value,    _0,
                     _0,    _0,    _0, value)
    }
    
    #[inline(always)]
    static pure fn from_Mat2(m: &Mat2<T>) -> Mat4<T> {
        let _0 = cast(0);
        let _1 = cast(1);
        Mat4::new(m[0][0], m[0][1], _0, _0,
                  m[1][0], m[1][1], _0, _0,
                       _0,      _0, _1, _0,
                       _0,      _0, _0, _1)
    }
    
    #[inline(always)]
    static pure fn from_Mat3(m: &Mat3<T>) -> Mat4<T> {
        let _0 = cast(0);
        let _1 = cast(1);
        Mat4::new(m[0][0], m[0][1], m[0][2], _0,
                  m[1][0], m[1][1], m[1][2], _0,
                  m[2][0], m[2][1], m[2][2], _0,
                       _0,      _0,      _0, _1)
    }
    
    // FIXME: An interim solution to the issues with static functions
    #[inline(always)]
    static pure fn identity() -> Mat4<T> {
        let _0 = cast(0);
        let _1 = cast(1);
        Mat4::new(_1, _0, _0, _0,
                  _0, _1, _0, _0,
                  _0, _0, _1, _0,
                  _0, _0, _0, _1)
    }
    
    // FIXME: An interim solution to the issues with static functions
    #[inline(always)]
    static pure fn zero() -> Mat4<T> {
        let _0 = cast(0);
        Mat4::new(_0, _0, _0, _0,
                  _0, _0, _0, _0,
                  _0, _0, _0, _0,
                  _0, _0, _0, _0)
    }
}

pub impl<T:Copy Float Sign> Mat4<T>: Matrix<T, Vec4<T>> {
    #[inline(always)]
    pure fn col(&self, i: uint) -> Vec4<T> { self[i] }
    
    #[inline(always)]
    pure fn row(&self, i: uint) -> Vec4<T> {
        Vec4::new(self[0][i],
                  self[1][i],
                  self[2][i],
                  self[3][i])
    }
    
    /**
     * Returns the multiplicative identity matrix
     * ~~~
     *       c0   c1   c2   c3
     *     +----+----+----+----+
     *  r0 |  1 |  0 |  0 |  0 |
     *     +----+----+----+----+
     *  r1 |  0 |  1 |  0 |  0 |
     *     +----+----+----+----+
     *  r2 |  0 |  0 |  1 |  0 |
     *     +----+----+----+----+
     *  r3 |  0 |  0 |  0 |  1 |
     *     +----+----+----+----+
     * ~~~
     */
    #[inline(always)]
    static pure fn identity() -> Mat4<T> {
        let _0 = Number::from(0);
        let _1 = Number::from(1);
        Mat4::new(_1, _0, _0, _0,
                  _0, _1, _0, _0,
                  _0, _0, _1, _0,
                  _0, _0, _0, _1)
    }
    
    /**
     * Returns the additive identity matrix
     * ~~~
     *       c0   c1   c2   c3
     *     +----+----+----+----+
     *  r0 |  0 |  0 |  0 |  0 |
     *     +----+----+----+----+
     *  r1 |  0 |  0 |  0 |  0 |
     *     +----+----+----+----+
     *  r2 |  0 |  0 |  0 |  0 |
     *     +----+----+----+----+
     *  r3 |  0 |  0 |  0 |  0 |
     *     +----+----+----+----+
     * ~~~
     */
    #[inline(always)]
    static pure fn zero() -> Mat4<T> {
        let _0 = Number::from(0);
        Mat4::new(_0, _0, _0, _0,
                  _0, _0, _0, _0,
                  _0, _0, _0, _0,
                  _0, _0, _0, _0)
    }
    
    #[inline(always)]
    pure fn mul_t(&self, value: T) -> Mat4<T> {
        Mat4::from_cols(self[0].mul_t(value),
                        self[1].mul_t(value),
                        self[2].mul_t(value),
                        self[3].mul_t(value))
    }
    
    #[inline(always)]
    pure fn mul_v(&self, vec: &Vec4<T>) -> Vec4<T> {
        Vec4::new(self.row(0).dot(vec),
                  self.row(1).dot(vec),
                  self.row(2).dot(vec),
                  self.row(3).dot(vec))
    }
    
    #[inline(always)]
    pure fn add_m(&self, other: &Mat4<T>) -> Mat4<T> {
        Mat4::from_cols(self[0].add_v(&other[0]),
                        self[1].add_v(&other[1]),
                        self[2].add_v(&other[2]),
                        self[3].add_v(&other[3]))
    }
    
    #[inline(always)]
    pure fn sub_m(&self, other: &Mat4<T>) -> Mat4<T> {
        Mat4::from_cols(self[0].sub_v(&other[0]),
                        self[1].sub_v(&other[1]),
                        self[2].sub_v(&other[2]),
                        self[3].sub_v(&other[3]))
    }
    
    #[inline(always)]
    pure fn mul_m(&self, other: &Mat4<T>) -> Mat4<T> {
        // Surprisingly when building with optimisation turned on this is actually
        // faster than writing out the matrix multiplication in expanded form.
        // If you don't believe me, see ./test/performance/matrix_mul.rs
        Mat4::new(self.row(0).dot(&other.col(0)), self.row(1).dot(&other.col(0)), self.row(2).dot(&other.col(0)), self.row(3).dot(&other.col(0)),
                  self.row(0).dot(&other.col(1)), self.row(1).dot(&other.col(1)), self.row(2).dot(&other.col(1)), self.row(3).dot(&other.col(1)),
                  self.row(0).dot(&other.col(2)), self.row(1).dot(&other.col(2)), self.row(2).dot(&other.col(2)), self.row(3).dot(&other.col(2)),
                  self.row(0).dot(&other.col(3)), self.row(1).dot(&other.col(3)), self.row(2).dot(&other.col(3)), self.row(3).dot(&other.col(3)))
    }
    
    pure fn dot(&self, other: &Mat4<T>) -> T {
        other.transpose().mul_m(self).trace()
    }

    pure fn determinant(&self) -> T {
        self[0][0]*Mat3::new(self[1][1], self[2][1], self[3][1],
                             self[1][2], self[2][2], self[3][2],
                             self[1][3], self[2][3], self[3][3]).determinant() -
        self[1][0]*Mat3::new(self[0][1], self[2][1], self[3][1],
                             self[0][2], self[2][2], self[3][2],
                             self[0][3], self[2][3], self[3][3]).determinant() +
        self[2][0]*Mat3::new(self[0][1], self[1][1], self[3][1],
                             self[0][2], self[1][2], self[3][2],
                             self[0][3], self[1][3], self[3][3]).determinant() -
        self[3][0]*Mat3::new(self[0][1], self[1][1], self[2][1],
                             self[0][2], self[1][2], self[2][2],
                             self[0][3], self[1][3], self[2][3]).determinant()
    }

    pure fn trace(&self) -> T {
        self[0][0] + self[1][1] + self[2][2] + self[3][3]
    }

    pure fn inverse(&self) -> Option<Mat4<T>> unsafe {
        let d = self.determinant();
        // let _0 = Number::from(0);    // FIXME: Triggers ICE
        let _0 = cast(0);
        if d.fuzzy_eq(&_0) {
            None
        } else {

            // Gauss Jordan Elimination with partial pivoting
            // So take this matrix, A, augmented with the identity
            // and essentially reduce [A|I]
            
            let mut A = *self;
            // let mut I: Mat4<T> = Matrix::identity();     // FIXME: there's something wrong with static functions here!
            let mut I = Mat4::identity();

            for uint::range(0, 4) |j| {
                // Find largest element in col j
                let mut i1 = j;
                for uint::range(j + 1, 4) |i| {
                    if abs(&A[j][i]) > abs(&A[j][i1]) {
                        i1 = i;
                    }
                }

                // Swap columns i1 and j in A and I to
                // put pivot on diagonal
                A.swap_cols(i1, j);
                I.swap_cols(i1, j);

                // Scale col j to have a unit diagonal
                I.col_mut(j).div_self_t(&A[j][j]);
                A.col_mut(j).div_self_t(&A[j][j]);

                // Eliminate off-diagonal elems in col j of A,
                // doing identical ops to I
                for uint::range(0, 4) |i| {
                    if i != j {
                        I.col_mut(i).sub_self_v(&I[j].mul_t(A[i][j]));
                        A.col_mut(i).sub_self_v(&A[j].mul_t(A[i][j]));
                    }
                }
            }
            Some(I)
        }
    }
    
    #[inline(always)]
    pure fn transpose(&self) -> Mat4<T> {
        Mat4::new(self[0][0], self[1][0], self[2][0], self[3][0],
                  self[0][1], self[1][1], self[2][1], self[3][1],
                  self[0][2], self[1][2], self[2][2], self[3][2],
                  self[0][3], self[1][3], self[2][3], self[3][3])
    }
    
    #[inline(always)]
    pure fn is_identity(&self) -> bool {
        // self.fuzzy_eq(&Matrix::identity())     // FIXME: there's something wrong with static functions here!
        self.fuzzy_eq(&Mat4::identity())
    }
    
    #[inline(always)]
    pure fn is_diagonal(&self) -> bool {
        let _0 = cast(0);
        self[0][1].fuzzy_eq(&_0) &&
        self[0][2].fuzzy_eq(&_0) &&
        self[0][3].fuzzy_eq(&_0) &&
        
        self[1][0].fuzzy_eq(&_0) &&
        self[1][2].fuzzy_eq(&_0) &&
        self[1][3].fuzzy_eq(&_0) &&
        
        self[2][0].fuzzy_eq(&_0) &&
        self[2][1].fuzzy_eq(&_0) &&
        self[2][3].fuzzy_eq(&_0) &&
        
        self[3][0].fuzzy_eq(&_0) &&
        self[3][1].fuzzy_eq(&_0) &&
        self[3][2].fuzzy_eq(&_0)
    }
    
    #[inline(always)]
    pure fn is_rotated(&self) -> bool {
        // !self.fuzzy_eq(&Matrix::identity())     // FIXME: there's something wrong with static functions here!
        !self.fuzzy_eq(&Mat4::identity())
    }
    
    #[inline(always)]
    pure fn is_symmetric(&self) -> bool {
        self[0][1].fuzzy_eq(&self[1][0]) &&
        self[0][2].fuzzy_eq(&self[2][0]) &&
        self[0][3].fuzzy_eq(&self[3][0]) &&
        
        self[1][0].fuzzy_eq(&self[0][1]) &&
        self[1][2].fuzzy_eq(&self[2][1]) &&
        self[1][3].fuzzy_eq(&self[3][1]) &&
        
        self[2][0].fuzzy_eq(&self[0][2]) &&
        self[2][1].fuzzy_eq(&self[1][2]) &&
        self[2][3].fuzzy_eq(&self[3][2]) &&
        
        self[3][0].fuzzy_eq(&self[0][3]) &&
        self[3][1].fuzzy_eq(&self[1][3]) &&
        self[3][2].fuzzy_eq(&self[2][3])
    }

    #[inline(always)]
    pure fn is_invertible(&self) -> bool {
        let _0 = cast(0);
        !self.determinant().fuzzy_eq(&_0)
    }
}

pub impl<T:Copy Float Sign> Mat4<T>: MutableMatrix<T, Vec4<T>> {
    #[inline(always)]
    fn col_mut(&mut self, i: uint) -> &self/mut Vec4<T> {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => fail(fmt!("index out of bounds: expected an index from 0 to 3, but found %u", i))
        }
    }
    
    #[inline(always)]
    fn swap_cols(&mut self, a: uint, b: uint) {
        util::swap(self.col_mut(a),
                   self.col_mut(b));
    }
    
    #[inline(always)]
    fn swap_rows(&mut self, a: uint, b: uint) {
        self.x.swap(a, b);
        self.y.swap(a, b);
        self.z.swap(a, b);
        self.w.swap(a, b);
    }
    
    #[inline(always)]
    fn set(&mut self, other: &Mat4<T>) {
        (*self) = (*other);
    }
    
    #[inline(always)]
    fn to_identity(&mut self) {
        (*self) = Mat4::identity();
    }
    
    #[inline(always)]
    fn to_zero(&mut self) {
        (*self) = Mat4::zero();
    }
    
    #[inline(always)]
    fn mul_self_t(&mut self, value: T) {
        self.col_mut(0).mul_self_t(&value);
        self.col_mut(1).mul_self_t(&value);
        self.col_mut(2).mul_self_t(&value);
        self.col_mut(3).mul_self_t(&value);
    }
    
    #[inline(always)]
    fn add_self_m(&mut self, other: &Mat4<T>) {
        self.col_mut(0).add_self_v(&other[0]);
        self.col_mut(1).add_self_v(&other[1]);
        self.col_mut(2).add_self_v(&other[2]);
        self.col_mut(3).add_self_v(&other[3]);
    }
    
    #[inline(always)]
    fn sub_self_m(&mut self, other: &Mat4<T>) {
        self.col_mut(0).sub_self_v(&other[0]);
        self.col_mut(1).sub_self_v(&other[1]);
        self.col_mut(2).sub_self_v(&other[2]);
        self.col_mut(3).sub_self_v(&other[3]);
    }
    
    #[inline(always)]
    fn invert_self(&mut self) {
        match self.inverse() {
            Some(m) => (*self) = m,
            None => fail(~"Couldn't invert the matrix!")
        }
    }
    
    #[inline(always)]
    fn transpose_self(&mut self) {
        util::swap(self.col_mut(0).index_mut(1), self.col_mut(1).index_mut(0));
        util::swap(self.col_mut(0).index_mut(2), self.col_mut(2).index_mut(0));
        util::swap(self.col_mut(0).index_mut(3), self.col_mut(3).index_mut(0));
        
        util::swap(self.col_mut(1).index_mut(0), self.col_mut(0).index_mut(1));
        util::swap(self.col_mut(1).index_mut(2), self.col_mut(2).index_mut(1));
        util::swap(self.col_mut(1).index_mut(3), self.col_mut(3).index_mut(1));
        
        util::swap(self.col_mut(2).index_mut(0), self.col_mut(0).index_mut(2));
        util::swap(self.col_mut(2).index_mut(1), self.col_mut(1).index_mut(2));
        util::swap(self.col_mut(2).index_mut(3), self.col_mut(3).index_mut(2));
        
        util::swap(self.col_mut(3).index_mut(0), self.col_mut(0).index_mut(3));
        util::swap(self.col_mut(3).index_mut(1), self.col_mut(1).index_mut(3));
        util::swap(self.col_mut(3).index_mut(2), self.col_mut(2).index_mut(3));
    }
}

pub impl<T> Mat4<T>: Matrix4<T, Vec4<T>> {
}

pub impl<T:Copy Float> Mat4<T>: Neg<Mat4<T>> {
    #[inline(always)]
    pure fn neg(&self) -> Mat4<T> {
        Mat4::from_cols(-self[0], -self[1], -self[2], -self[3])
    }
}

pub impl<T> Mat4<T>: Dimensional<Vec4<T>> {
    #[inline(always)]
    static pure fn dim() -> uint { 4 }
    
    #[inline(always)]
    static pure fn size_of() -> uint { size_of::<Mat4<T>>() }
}

pub impl<T:Copy> Mat4<T>: Index<uint, Vec4<T>> {
    #[inline(always)]
    pure fn index(&self, i: uint) -> Vec4<T> {
        unsafe { do buf_as_slice(
            transmute::<*Mat4<T>, *Vec4<T>>(
                to_unsafe_ptr(self)), 4) |slice| { slice[i] }
        }
    }
}

pub impl<T:Copy> Mat4<T>: ToPtr<T> {
    #[inline(always)]
    pure fn to_ptr(&self) -> *T {
        unsafe {
            transmute::<*Mat4<T>, *T>(
                to_unsafe_ptr(self)
            )
        }
    }
}

pub impl<T:Copy Float> Mat4<T>: Eq {
    #[inline(always)]
    pure fn eq(&self, other: &Mat4<T>) -> bool {
        self[0] == other[0] &&
        self[1] == other[1] &&
        self[2] == other[2] &&
        self[3] == other[3]
    }
    
    #[inline(always)]
    pure fn ne(&self, other: &Mat4<T>) -> bool {
        !(self == other)
    }
}

pub impl<T:Copy Float> Mat4<T>: FuzzyEq {
    #[inline(always)]
    pure fn fuzzy_eq(other: &Mat4<T>) -> bool {
        self[0].fuzzy_eq(&other[0]) &&
        self[1].fuzzy_eq(&other[1]) &&
        self[2].fuzzy_eq(&other[2]) &&
        self[3].fuzzy_eq(&other[3])
    }
}
