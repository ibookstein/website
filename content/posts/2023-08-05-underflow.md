---
title: "Underflow: A Subtle, Obscure Corner of IEEE754"
date: 2023-08-05T00:00:00+00:00
tags: ["rust", "ieee754"]
type: post
math: true
draft: true
showTableOfContents: true
---

## IEEE754 Exceptions: A Whirlwind Tour

The IEEE754 standard defines 5 exceptions. These help detect exceptional
conditions that arise during computation. The standard separates between
the exception, which behaves as an event, and the default handling for
the exception, which is a prescribed way for handling that event. The default
handling for an IEEE754 exception is to deliver a default result, set a
corresponding status flag (except in a few particular cases), and continue
execution.

The status flags are made available for inspection by implementations. This
allows one to perform an arbitrarily long chain of computations, and inspect
the status flags at the end to reason about whether the computation chain
reached an exceptional condition at any point in time.

The 5 exceptions defined are as follows:
* Division by zero. This one's self explanatory. The default result delivered
  will be an appropriately-signed infinity. It is also caused by computing
  the logarithm of zero, yielding negative infinity.
* Inexact. This one means that a computation's result differs from the precise
  mathematical result. A rounded or overflowed result is delivered. Calling
  this one an exception is a bit funny - the vast majority of computations
  will set this, so it's hard to argue that it is exceptional in the English
  sense of the word.
* Invalid operation. This one is for when an operation cannot produce a usefully
  defined result. Dividing zero by zero, subtracting infinity from infinity, you
  name it -- when the result is indeterminate, the invalid operation exception
  is signaled. The default result delivered is NaN. Only an operation producing
  a _new_ NaN or consuming a signaling NaN will generate this exception.
  Merely consuming a quiet NaN will not.
* Overflow. This one is for when the exponent a rounded result should have had
  (if the exponent range was unbounded) exceeds the maximum exponent of the
  destination format. Whether the result is $\pm\mathrm{Inf}$ or
  $\pm\mathrm{LargestFinite}$ is dependent on the rounding mode.
* Underflow. This one is for when we fall down to the subnormals range. It has
  two separate definitions, which we'll discuss below, and the delivered result
  isn't always a subnormal.

## The Underflow Exception

Reading the IEEE754 specification's section on underflow (7.5), one immediately
notices that there are two different, alternative definitions for the meaning
of the exception:

1. _after rounding_ — when a non-zero result computed as though the exponent
   range were unbounded  would lie strictly between $\pm b^{emin}$.
1. _before rounding_ — when a non-zero result computed as though both the
   exponent range and the precision were unbounded would lie strictly between
   $\pm b^{emin}$.

The implementer is at liberty to choose either for radix two (i.e. binary),
so long as the choice is uniform for all operations. But only the
_before rounding_ option is available for decimal formats.

In addition, the underflow exception is unique in that its default handling
does not always set the corresponding flag! If the result is *exact*, then the
default handling does *not* set the underflow flag.

So already we're in strange waters -- why are different implementations allowed
to choose differently, but only for radix two? Why is the default handling for
this exception uniquely dependent on another exception -- the Inexact exception?
The answers to these questions require more context than I can authoritatively
provide.

Indeed, both underflow disciplines are extremely ubiquitous -- at a minimum,
x86 and RISC-V detect tininess _after_ rounding, whilst ARM and IBM POWER
detect tininess _before_ rounding. A more thorough survey can be found
[here](https://www.math.utah.edu/~beebe/ufl/pages-13-21.pdf).

Perhaps the reason for filtering out exact tiny results is that they do not
constitute the increased loss of precision that underflow captures when you
define it that way.

We'll start picking apart the meanings of these two definitions, and how one
might go about implementing them.

## Underflow Before Rounding
The easier of the two to both understand and implement is underflow _before_
rounding. Its meaning is basically that the ideal mathematical result of the
computation (i.e. a result supposing an arbitrary exponent range *and* arbitrary
precision) lies in the range of subnormal numbers.

To understand how one would capture this state of affairs in a hardware
implementation of a floating-point execution unit, we'll describe a simplified
single-precision floating-point multiplier, with pseudocode.

```Rust
u32 mul(u32 x, u32 y) {
  let xp = parse(x);
  let yp = parse(y);

  // Handle Zeros, Infinities, and NaNs.
  if let (Ok(r) = handle_singulars(xp, yp)) {
    return r;
  }

  // Mantissas contain the no-longer-hidden integer bit.
  // Exponents are signed, still biased, and zero-extended to two extra bits.
  // For subnormals, the exponents are bumped to 1 to match the mantissas'
  // bit significance, but we don't normalize. Therefore:
  // 1. These mantissas can contain leading zeros.
  // 2. These exponents don't always represent the number's true magnitude.
  let (x_sign, x_exp, x_mant): (u1, s10, u24) = xp.as_regular();
  let (y_sign, y_exp, y_mant): (u1, s10, u24) = yp.as_regular();

  let prod_sign = x_sign ^ y_sign;

  // `x_mant` and `y_mant` represent numbers in (0, 2), with the most
  // significant bit being the integer bit: i.xxx...xxx, j.yyy...yyy.
  // The range isn't [1, 2) because we didn't normalize subnormals on entry.
  // Zero is excluded because we handled it specially as a singular number.

  // The product's radix point has two digits to its left:  ab.zzz...zzz.
  // So we add 1 to the exponent to move the radix point to a.bzzz...zzz.
  // We also subtract the bias so that the result is not double-biased.
  // The range is: [1, 254] + [1, 254] + [-126, -126] = [-124, 382].
  let prod_initial_exp: s10 = x_exp + y_exp + 1 - 127;

  // Use an integer multiplier to get an initial mantissa for the product.
  // Because we compensated `prod_initial_exp` with an extra 1, it will still
  // represent a number in (0, 2), but with twice as many bits.
  let prod_initial_mant: u48 = unsigned_integer_mul(x_mant, y_mant);

  // Compute how much we need to shift left to normalize the product.
  let wanted_normalization = prod_initial_mant.leading_zeros() as s10;

  // We would like to subtract `wanted_normalization` from `prod_initial_exp`,
  // but it could leave us underwater - we need the result of this subtraction
  // to saturate to 1 (subnormals). We could actually already be underwater
  // even before this subtraction, when `prod_initial_exp` is in [-124, 0].
  let (mant_to_normalize, actual_normalization, low_tail) = if prod_initial_exp < 1 {
    (0u48, 0s10, 1u1)
  } else {
    (prod_initial_mant, min(wanted_normalization, prod_initial_exp - 1), 0u1)
  };
  let = min(0, prod_initial_exp - 1, wanted_normalization);

  // We computed `prod_initial_exp` such that it is the significance of the MSB
  // of `prod_initial_mant`.
  let (normalized_mant, low_tail): (u48, u1) =
      if prod_initial_exp < -126 - 48 {
        (0u48, 1u1)
      } else {
        ()
      };

  let true_exponent = prod_initial_exp - wanted_normalization;
  if true_exponent < -126 - 23 {
    (0u24, )
  }
  // We must not underflow the exponent, so saturate.
  let actual_normalization = min(wanted_normalization, prod_initial_exp);

}
```

This situation will present itself in a hardware floating-point implementation
when:

But [this stackoverflow post's](https://stackoverflow.com/a/76424791)
quotes from the stds-754 list do match my experience well.


> the definition of tininess after rounding depends on a hypothetical rounding
> to a hypothetical intermediate format that has confused everybody who has ever
> tried to implement underflow.

The advantages of detecting tininess _after rounding_ are that it is symmetric
with the definition of overflow, and that it avoids some 'spurious' underflows
(situations where underflow occurs but you haven't really lost effective precision).

* _after rounding_
* Detecting tininess _before rounding_ is simpler to implement, is vastly easier
  to understand, and is not dependent on
  .
