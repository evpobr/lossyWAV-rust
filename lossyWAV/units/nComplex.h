/**===========================================================================

    lossyWAV: Added noise WAV bit reduction method by David Robinson;
              Noise shaping coefficients by Sebastian Gesemann;

    Copyright (C) 2007-2016 Nick Currie, Copyleft.

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <http://www.gnu.org/licenses/>.

    Contact: lossywav <at> hotmail <dot> co <dot> uk

===========================================================================**/

#ifndef nComplex_h_
#define nComplex_h_

#include <complex>

typedef std::complex<double> tDComplex;

static inline tDComplex tDComplex_divided_by_i(const tDComplex& other)
{
    return tDComplex(other.imag(), -other.real());
}

static inline tDComplex tDComplex_multiplied_by_i(const tDComplex& other)
{
    return tDComplex(-other.imag(), other.real());
}

static inline tDComplex tDComplex_complex_exp(double a)
{
    double p_sin = 0.0;
    double p_cos = 0.0;
    sincos(a, &p_sin, &p_cos);
    tDComplex result {p_cos, p_sin};
    return result;
}

#endif // nComplex_h_
