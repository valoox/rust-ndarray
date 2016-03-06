// Copyright 2014-2016 bluss and ndarray developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use imp_prelude::*;
use dimension::{self, stride_offset};
use error::ShapeError;

/// # Methods for Array Views
///
/// Methods for read-only array views `ArrayView<'a, A, D>`
///
/// Note that array views implement traits like [`From`][f] and `IntoIterator` too.
/// [f]: #method.from
impl<'a, A, D> ArrayBase<ViewRepr<&'a A>, D>
    where D: Dimension,
{
    /// Create a read-only array view borrowing its data from a slice.
    ///
    /// Checks whether `dim` and `strides` are compatible with the slice's
    /// length, returning an `Err` if not compatible.
    ///
    /// ```
    /// use ndarray::ArrayView;
    /// use ndarray::arr3;
    ///
    /// let s = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
    /// let a = ArrayView::from_slice_dim_stride((2, 3, 2),
    ///                                          (1, 4, 2),
    ///                                          &s).unwrap();
    ///
    /// assert!(
    ///     a == arr3(&[[[0, 2],
    ///                  [4, 6],
    ///                  [8, 10]],
    ///                 [[1, 3],
    ///                  [5, 7],
    ///                  [9, 11]]])
    /// );
    /// assert!(a.strides() == &[1, 4, 2]);
    /// ```
    pub fn from_slice_dim_stride(dim: D, strides: D, xs: &'a [A])
        -> Result<Self, ShapeError>
    {
        dimension::can_index_slice(xs, &dim, &strides).map(|_| {
            unsafe {
                Self::new_(xs.as_ptr(), dim, strides)
            }
        })
    }

    /// Split the array along `axis` and return one view strictly before the
    /// split and one view after the split.
    ///
    /// **Panics** if `axis` or `index` is out of bounds.
    ///
    /// Below, an illustration of `.split_at(Axis(2), 2)` on
    /// an array with shape 3 × 5 × 5.
    ///
    /// <img src="split_at.svg" width="300px" height="271px">
    pub fn split_at(self, axis: Axis, index: Ix)
        -> (Self, Self)
    {
        // NOTE: Keep this in sync with the ArrayViewMut version
        let axis = axis.axis();
        assert!(index <= self.shape()[axis]);
        let left_ptr = self.ptr;
        let right_ptr = if index == self.shape()[axis] {
            self.ptr
        } else {
            let offset = stride_offset(index, self.strides.slice()[axis]);
            unsafe {
                self.ptr.offset(offset)
            }
        };

        let mut dim_left = self.dim.clone();
        dim_left.slice_mut()[axis] = index;
        let left = unsafe {
            Self::new_(left_ptr, dim_left, self.strides.clone())
        };

        let mut dim_right = self.dim;
        let right_len  = dim_right.slice()[axis] - index;
        dim_right.slice_mut()[axis] = right_len;
        let right = unsafe {
            Self::new_(right_ptr, dim_right, self.strides)
        };

        (left, right)
    }

}

/// Methods for read-write array views `ArrayViewMut<'a, A, D>`
///
/// Note that array views implement traits like [`From`][f] and `IntoIterator` too.
///
/// [f]: #method.from
impl<'a, A, D> ArrayBase<ViewRepr<&'a mut A>, D>
    where D: Dimension,
{
    /// Create a read-write array view borrowing its data from a slice.
    ///
    /// Checks whether `dim` and `strides` are compatible with the slice's
    /// length, returning an `Err` if not compatible.
    ///
    /// ```
    /// use ndarray::ArrayViewMut;
    /// use ndarray::arr3;
    ///
    /// let mut s = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
    /// let mut a = ArrayViewMut::from_slice_dim_stride((2, 3, 2),
    ///                                                 (1, 4, 2),
    ///                                                 &mut s).unwrap();
    ///
    /// a[[0, 0, 0]] = 1;
    /// assert!(
    ///     a == arr3(&[[[1, 2],
    ///                  [4, 6],
    ///                  [8, 10]],
    ///                 [[1, 3],
    ///                  [5, 7],
    ///                  [9, 11]]])
    /// );
    /// assert!(a.strides() == &[1, 4, 2]);
    /// ```
    pub fn from_slice_dim_stride(dim: D, strides: D, xs: &'a mut [A])
        -> Result<Self, ShapeError>
    {
        dimension::can_index_slice(xs, &dim, &strides).map(|_| {
            unsafe {
                Self::new_(xs.as_mut_ptr(), dim, strides)
            }
        })
    }

    /// Split the array along `axis` and return one mutable view strictly
    /// before the split and one mutable view after the split.
    ///
    /// **Panics** if `axis` or `index` is out of bounds.
    pub fn split_at(self, axis: Axis, index: Ix)
        -> (Self, Self)
    {
        // NOTE: Keep this in sync with the ArrayView version
        let axis = axis.axis();
        assert!(index <= self.shape()[axis]);
        let left_ptr = self.ptr;
        let right_ptr = if index == self.shape()[axis] {
            self.ptr
        }
        else {
            let offset = stride_offset(index, self.strides.slice()[axis]);
            unsafe {
                self.ptr.offset(offset)
            }
        };

        let mut dim_left = self.dim.clone();
        dim_left.slice_mut()[axis] = index;
        let left = unsafe {
            Self::new_(left_ptr, dim_left, self.strides.clone())
        };

        let mut dim_right = self.dim;
        let right_len  = dim_right.slice()[axis] - index;
        dim_right.slice_mut()[axis] = right_len;
        let right = unsafe {
            Self::new_(right_ptr, dim_right, self.strides)
        };

        (left, right)
    }

}

