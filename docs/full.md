International Standard ISO/IEC 21122-1 Third edition 2024-07 Information technology — JPEG XS low-latency lightweight image coding system — Core coding system Part 1: Technologies de l'information — Système de codage d'images léger à faible latence JPEG XS — Partie 1: Système de codage de noyau Reference number © ISO/IEC 2024

<!-- page 1 -->

## COPYRIGHT PROTECTED DOCUMENT

© ISO/IEC 2024 All rights reserved. Unless otherwise specified, or required in the context of its implementation, no part of this publication may be reproduced or utilized otherwise in any form or by any means, electronic or mechanical, including photocopying, or posting on the internet or an intranet, without prior written permission. Permission can be requested from either ISO at the address below or ISO’s member body in the country of the requester. ISO copyright office CP 401 • Ch. de Blandonnet 8 CH-1214 Vernier, Geneva Phone: +41 22 749 01 11 Email: copyright@iso.org Website: www.iso.org Published in Switzerland © ISO/IEC 2024 – All rights reserved ii

<!-- page 2 -->

Contents Foreword Page iv Scope
.................................................................................................................................................................................................................................................... Normative references
............................................................................................................................................................................................................................................. Terms and definitions, abbreviated terms and symbols
.................................................................................................................................................................................................
### 3.1

### 3.2

Conventions
### 3.3

Terms and definitions ...................................................................................................................................................................................1 Abbreviated terms ...........................................................................................................................................................................................6 Symbols ......................................................................................................................................................................................................................6
........................................................................................................................................................................................................................
......................................................................................................
### 4.1

### 4.2

Conformance language ..............................................................................................................................................................................10 Operators ...............................................................................................................................................................................................................10 4.2.1 Arithmetic operators ................................................................................................................................................................10 4.2.2 Logical operators .........................................................................................................................................................................10 4.2.3 Relational operators ..................................................................................................................................................................10 4.2.4 Precedence order of operators ........................................................................................................................................10 4.2.5 Mathematical functions .........................................................................................................................................................11 Functional concepts
................................................................................................................................................................................................... Sample grid, sampling and components ....................................................................................................................................11
### 5.1

Interpretation of CFA data .................................................................................................................................................................... 12
### 5.2

5.3 Wavelet decomposition ............................................................................................................................................................................ 12 Encoder requirements Codestream......................................................................................................................................................................................................... 13
### 5.4

Decoder
............................................................................................................................................................................................
### 7.1

## Annex A

### 7.2

## Annex B

## Annex C

(normative)
................................................................................................................................................................................................................................... Decoding process general provisions ......................................................................................................................................... 13 Decoder requirements .............................................................................................................................................................................. 15
.............................................................................................................................................................. Image data structures Codestream syntax (normative) Entropy decoding
......................................................................................................................................................
## Annex D

(normative) Quantization
...................................................................................................................................................................
## Annex E

## Annex F

(normative) Discrete wavelet transformation
............................................................................................................................................................................... (normative) Multiple component transformations
..........................................................................................................................
## Annex G

(normative) DC level shifting, non-linear transform and output clipping
..............................................................................................................
## Annex H

(normative) Frame buffer
..................................................
## Annex I

(normative) Example weight tables
............................................................................................................................................................................... Bibliography (informative)
................................................................................................................................................
...................................................................................................................................................................................................................................... © ISO/IEC 2024 – All rights reserved iii

<!-- page 3 -->

Foreword ISO (the International Organization for Standardization) is a worldwide federation of national standards bodies (ISO member bodies). The work of preparing International Standards is normally carried out through ISO technical committees. Each member body interested in a subject for which a technical committee has been established has the right to be represented on that committee. International organizations, governmental and non-governmental, in liaison with ISO, also take part in the work. ISO collaborates closely with the International Electrotechnical Commission (IEC) on all matters of electrotechnical standardization. The procedures used to develop this document and those intended for its further maintenance are described in the ISO/IEC Directives, Part 1. In particular, the different approval criteria needed for the different types of ISO document should be noted. This document was drafted in accordance with the editorial rules of the ISO/IEC Directives, Part 2 (see www.iso.org/directives). ISO draws attention to the possibility that the implementation of this document may involve the use of (a) patent(s). ISO takes no position concerning the evidence, validity or applicability of any claimed patent rights in respect thereof. As of the date of publication of this document, ISO had received notice of (a) patent(s) which may be required to implement this document. However, implementers are cautioned that this may not represent the latest information, which may be obtained from the patent database available at www.iso.org/patents. ISO shall not be held responsible for identifying any or all such patent rights. Any trade name used in this document is information given for the convenience of users and does not constitute an endorsement. For an explanation of the voluntary nature of standards, the meaning of ISO specific terms and expressions related to conformity assessment, as well as information about ISO's adherence to the World Trade Information technology Organization (WTO) principles in the Technical Barriers to Trade (TBT), see www.iso.org/iso/foreword.html. Coding of audio, picture, multimedia and hypermedia information This document was prepared by Joint Technical Committee ISO/IEC JTC 1, Subcommittee SC 29,
.
, This third edition cancels and replaces the second edition (ISO/IEC 21122-1:2022), which has been technically revised. The main changes are as follows: — coding tools for improving the compression rates for screen content images have been added; — coding tools that enable lossless coding of images with up to 16 bits per sample have been added. A list of all parts in the ISO/IEC 21122 series can be found on the ISO and IEC websites. Any feedback or questions on this document should be directed to the user’s national standards body. A complete found at www.iso.org/members.html and listing of www.iec.ch/national-committees. these bodies can be © ISO/IEC 2024 – All rights reserved iv

<!-- page 4 -->

International Standard Information technology — JPEG XS low-latency lightweight image coding system — Core coding system Part 1:
## 1 Scope

This document specifies the syntax and an accompanying decompression process that is capable to represent continuous-tone grey-scale, or continuous-tone colour digital images without visual loss at moderate compression rates. Typical compression rates are between 2:1 and 18:1 but can also be higher depending on the nature of the image. In particular, the syntax and the decoding process specified in this document allow lightweight encoder and decoder implementations that limit the end-to-end latency to a fraction of the frame size. However, the definition of transmission channel buffer models necessary to ensure such latency is beyond the scope of this document. This document: — specifies decoding processes for converting compressed image data to reconstructed image data; — specifies a codestream syntax containing information for interpreting the compressed image data; — provides guidance on encoding processes for converting source image data to compressed image data.
## 2 Normative references

There are no normative references in this document.
## 3 Terms and definitions, abbreviated terms and symbols

### 3.1 Terms and definitions

For the purposes of this document, the following terms and definitions apply. ISO and IEC maintain terminology databases for use in standardization at the following addresses: — ISO Online browsing platform: available at https:// www .iso .org/ obp
#### 3.1.1

— IEC Electropedia: available at https:// www .electropedia .org/ band wavelet filter type components input data to a specific
#### 3.1.2

(3.1.14) of the image band type (3.1.54) that contributes to the generation of one of the single number collapsing the information on the component, and horizontal and vertical wavelet filter types that are applied in the filter cascade reconstructing spatial image samples from inversely quantized wavelet coefficients © ISO/IEC 2024 – All rights reserved

<!-- page 5 -->

#### 3.1.3

bit
#### 3.1.4

binary choice encoded as either 0 or 1 bitplane
#### 3.1.5

array of bits having all the same significance bitplane count number of significant bitplanes of a code group, counting from the LSB up to the most significant, non-empty
#### 3.1.6

bitplane bitplane count subpacket subset of a packet which decodes to the bitplane counts of all code groups within a packet, followed by padding and optional filler bytes
#### 3.1.7

Note 1 to entry: See subclause C.5.3. byte
#### 3.1.8

group of 8 bits colour filter array CFA rectangular array of sensor elements yielding a 1-component image where the colour to which a sensor
#### 3.1.9

element is sensitive to depends on the position of the sensor element codestream compressed image data representation that includes all necessary data to allow a (full or approximate)
#### 3.1.10

reconstruction of the sample values of a digital image code group
#### 3.1.11

group of quantization indices in sign-magnitude representation before inverse quantization coefficient
#### 3.1.12

input value to the inverse wavelet transformation resulting from inverse quantization coefficient group
#### 3.1.13

number of horizontally adjacent wavelet coefficients from the same band column
#### 3.1.14

set of vertically aligned precincts component two-dimensional array of samples having the same designation such as red, green or blue in the output or
#### 3.1.15

display device compression process of reducing the number of bits used to represent source image data © ISO/IEC 2024 – All rights reserved

<!-- page 6 -->

#### 3.1.16

continuous-tone image
#### 3.1.17

image whose components have more than one bit per sample data subpacket subset of a packet which consists of the quantization index magnitudes, followed by padding and optional filler bytes
#### 3.1.18

Note 1 to entry: See subclause C.5.4. deadzone quantizer
#### 3.1.19

quantizer whose zero bucket has a size different from all other buckets decoder
#### 3.1.20

embodiment of a decoding process decoding process
#### 3.1.21

process which takes as its input a codestream and outputs a continuous-tone image decomposition level
#### 3.1.22

number of times a wavelet filter is applied to reconstruct image data from wavelet coefficients encoder
#### 3.1.23

embodiment of an encoding process encoding process
#### 3.1.24

process which outputs compressed image data in the form of a codestream filler bytes
#### 3.1.25

integer number of bytes a decoder will skip over on decoding without interpreting the values of the bytes itself intra coding coding process which decodes data independent from data decoded in a previous frame
#### 3.1.26

Note 1 to entry: ISO/IEC 21122-1:2022 only defined intra coding tools. inverse quantization
#### 3.1.27

inverse procedure to quantization by which the decoder recovers a representation of the coefficients inverse reversible multiple component transformation inverse RCT inverse transform across multiple component sample values located at the same sample grid point that is invertible without loss
#### 3.1.28

Note 1 to entry: See subclauses F.3 and F.4. LL band input to a series of wavelet filters where only inverse low-pass filters are applied in horizontal and vertical direction © ISO/IEC 2024 – All rights reserved

<!-- page 7 -->

#### 3.1.29

lossless descriptive term for encoding and decoding processes and procedures in which the output of the decoding
#### 3.1.30

procedure(s) is identical to the input to the encoding procedure(s) lossless coding mode of operation which refers to any one of the coding processes defined in this document in which all of
#### 3.1.31

the procedures are lossless lossy
#### 3.1.32

descriptive term for encoding and decoding processes which are not lossless packet segment of the codestream containing entropy coded information on a single precinct, line and a subset of
#### 3.1.33

the bands within this precinct and line padding bits within the codestream whose only purpose is to align syntax elements to byte boundaries and that
#### 3.1.34

carry no information precinct
#### 3.1.35

collection of quantization indices of all bands contributing to a given spatial region of the image precision
#### 3.1.36

number of bits allocated to a particular sample, coefficient, or other binary numerical representation procedure
#### 3.1.37

set of steps which accomplishes one of the tasks which comprise an encoding or decoding process quantization
#### 3.1.38

method of reducing the precision of the individual coefficients quantization index
#### 3.1.39

input to the inverse quantization process which reconstructs the quantization index to a wavelet coefficient quantization index magnitude
#### 3.1.40

absolute value of a quantization index sample
#### 3.1.41

one element in the two-dimensional image array which comprises a component sample grid common coordinate system for all samples of an image, the samples at the top left edge of the image have the coordinates (0,0), the first coordinate increases towards the right, the second towards the bottom © ISO/IEC 2024 – All rights reserved

<!-- page 8 -->

#### 3.1.42

sign subpacket subset of a packet that consists of the sign information of all non-zero quantization indices within a packet, followed by padding and optional filler bytes
#### 3.1.43

Note 1 to entry: See subclause C.5.5. significance attribute of code groups that applies if, depending on the Run Mode flag in the picture header, either at least one of coefficients in the code group is non-zero, or the bitplane count prediction residual of the code group
#### 3.1.44

is non-zero significance group group of a horizontally adjacent code groups sharing the same significance information in the significance
#### 3.1.45

subpacket significance subpacket subset of a packet that identifies which significance groups within a packet are insignificant, followed by padding and optional filler bytes
#### 3.1.46

Note 1 to entry: see subclause C.5.2 slice
#### 3.1.47

integral number of precincts whose wavelet coefficients can be entropy-decoded independently star-tetrix decorrelation transformation that combines a spatial with an inter-component decorrelation transformation particularly tuned for CFA pattern compression
#### 3.1.48

Note 1 to entry: see subclause F.5 subpacket
#### 3.1.49

substructure of a packet containing information of one or multiple bands of one line of a single precinct super pixel 2×2 arrangement of sensor elements in a CFA pattern array containing at least one sensor element for each
#### 3.1.50

colour filter type temporal differential coding TDC
#### 3.1.51

coding process which decodes a differential signal relative to a sample decoded in a previous frame truncation position
#### 3.1.52

number of least significant bitplanes not included in the quantization index of a wavelet coefficient uniform quantizer
#### 3.1.53

quantizer whose buckets are all of equal size upsampling procedure by which the spatial resolution of a component is increased © ISO/IEC 2024 – All rights reserved

<!-- page 9 -->

#### 3.1.54

wavelet filter type single number that uniquely identifies each element of the wavelet filter with regard to the number and type of horizontal and vertical decompositions Note 1 to entry: Unlike the band type, the wavelet filter type does not include component information.
### 3.2 Abbreviated terms

## JPEG XS

informal name of this standard where XS stands for “extra speed” LSB least significant bit MSB
### 3.3 Symbols

most significant bit B i β [ ] b b β i b' x[ b
, ] Bw x[ ] Br Cpih c p λ b x c' [ p
, λ
, b
, x ] C [
,
,
, ] C s C t Cw f D p,b D [ p b ] D i[ p,s
, ] DCO r[ ] d β i d x[ β
, i ] δ c y[
, ] δ c x[ ] y[ ] i bit precision of component wavelet filter type band type β i band existence flag for filter type b in component
. 1 if the filter exists, 0 otherwise. band existence flag for band type
. 1 if the filter exists, 0 otherwise. nominal overall bit precision of the wavelet coefficients number of bits required to encode a bitplane count in raw colour transformation type p λ b x wavelet coefficient residual in precinct λ b
, line
, band and position x p wavelet coefficient in precinct
, line
, band and position width of precincts other than the rightmost precinct in sample grid positions colour transformation CFA pattern type derived from the component registration colour transformation reflection and extension flags width of precincts in multiples of 8 LL subsampled band sample grid positions p b bitplane count coding mode of band in precinct p b TDC mode of band in precinct s p raw coding mode override flag for packet in precinct DC offset β i horizontal decomposition level of wavelet filter type β of component i vertical decomposition level of wavelet filter type c of component horizontal position of component c in a CFA super pixel vertical position of component in a CFA super pixel © ISO/IEC 2024 – All rights reserved

<!-- page 10 -->

exponent of the slope of the linear region of the extended non-linearity colour transformation exponent of first chroma component colour transformation exponent of second chroma component p λ b x contents of the frame buffer at precinct
, line
, band and position sign packing flag slice coding mode number of fractional bits in the representation of wavelet coefficients b gain of band b gain of band under forced refresh β k height of filter type of component i in wavelet coefficients height of the component in sample points height of the image in sampling grid points height of a precinct in lines height of a slice in precincts slice TDC flag, set if wavelet coefficients within a slice may use TDC λ b p s E e e f p λ b x Fs [
,
,
, ] Fslc Fq G b G [ b ] H r[ β ] k H i b[
, ] H c[ ] H f H p I sl p I sl λ b s k [ δ
, δ
,
, ] line inclusion flag, set if line of band δ and precinct δ is included in packet
, reset otherwise L0 [ b p x, y] L1 p [ b
, ] Lcod
, [ ] L p s L cnt[ p
, s ] Lh dat[
, ] Lprc p L [ p ] s component within CFA super pixel at position x, y p b first line of band b in precinct p last line + 1 of band in precinct codestream length in bytes p s size of the bitplane count subpacket of precinct p s and packet in bytes size of the data subpacket of precinct and packet in bytes p long header flag in in the picture header, set if long headers are enforced, reset otherwise length of the entropy coded data in precinct p s L p sgn[ s
, ] size of the sign subpacket of precinct and packet p s in bytes p M sig[ λ
, b ] g size of the significance subpacket of precinct p λ b g and packet in bytes M [
, p
, λ
, b ] g bitplane count of precinct
, line
, band λ and code group p b g N top[
,
,
, ] vertical predictor of the bitplane count of precinct
, line
, band and code group N c p b N cg[
, ] β p number of components in an image b number of code groups in precinct and band number of bands per component © ISO/IEC 2024 – All rights reserved

<!-- page 11 -->

N N g p b N i[ p
, b ] N s[ t
, ] N p[ ] L N
, N' L x
, i N L
, x[ ] N' L y
, i N L
, y[ ] N p x
, N p y p O x c pc[ y ]
, [ c Ω
, x ] y P b [
,
, ] P [ b ] Plev r[ ] Ppih Ppoc Q p Q [ p ] Q’ f[ p ] Q f[ ] bi Q br Qpih Rl Rm R p R [ p ] f[ ] number of coefficients in a code group b p b number of TDC selection groups per line in band p of precinct number of significance groups per line in band t of precinct number of precincts in slice number of bands in the wavelet decomposition of the image (wavelet filter types times components) maximal number of horizontal decomposition levels i number of horizontal decomposition levels of component maximal number of vertical decomposition levels over all components i number of vertical decomposition levels of component number of precincts per sampling grid line number of precincts per sampling grid column p number of packets in precinct c x y unscaled output of the inverse wavelet transformation at coordinates x nent and y of the compo-
c output of the inverse multiple component transformation at position b
, for component priority of band b priority of band under forced refresh level a particular codestream complies to profile a particular codestream complies to progression order in which bands are transmitted in the codestream p quantization parameter of precinct p quantization parameter to which precinct will be quantized for storage in the frame buffer p quantization parameter of the data stored in the frame buffer corresponding to precinct quantization adjustment for intra-coded coefficients that are intra-coded due to a rate-decision quantization adjustment for intra-coded coefficients that are intra-coded due to refresh, overriding any rate-based TDC decision quantization type of the image raw-mode selection per packet flag run mode used for significance coding p refinement of precinct p refinement parameter of the quantization to which precinct in the frame buffer will be quantized for storage © ISO/IEC 2024 – All rights reserved

<!-- page 12 -->

p refinement parameter of the quantization of the data stored in the frame buffer corresponding to precinct y x c reconstructed sample value at position
, for component number of components for which wavelet decomposition is suppressed b intra refresh hash mask exponent of band size of a TDC selection group in code groups size of a significance group in code groups i sampling factor of component i in horizontal direction R’ p f[ c ] x y R Sd [
,
, ] S b S h[ ] S i s s i s i x[ ] s p y[ λ ] b x sampling factor of component p in vertical direction λ b x T [
,
,
, ] T T p b T [
, p ] b sign of the wavelet coefficient in precinct
, line
, band and position
. first threshold of the extended non-linearity second threshold of the extended non-linearity p b truncation position of precinct and band p b x β T top[ y
, ] vertical truncation position predictor of precinct β and band y x v x [ y
,
, ] v p
, [ λ ] b x [ W
,
, β
, k ] y temporary wavelet coefficient of filter type x at location
,
. sample value at the sample grid position x
, p λ b quantization index magnitude of the wavelet coefficient in precinct tion β k
, line
, band and posi-
W i b[
, ] width of filter type i of component in wavelet coefficients W c[ ] W f p width of component in samples p width of the image in sampling grid points W p[ p ] b width of the precinct b in sampling grid points p Wt pb[
, ] width of band of precinct in coefficients Wt x X y y Xcrg [ ] c Ycrg c [ ] Ysl [ ] Y p λ b k wavelet filter type for horizontal filtering wavelet filter type for vertical filtering one-dimensional temporal array of wavelet coefficients c horizontal component registration of component c relative to the sample grid vertical component registration of component relative to the sample grid slice index enumerating slices contiguously from top to bottom, starting at 0 p λ b k Y [ b
,
,
, ] TDC selection flag of precinct b
, line
, band and TDC selection group Z p h[ λ ] b j λ intra refresh position hash of band p b j [
,
,
, ] significance flag of precinct
, line
, band and significance group © ISO/IEC 2024 – All rights reserved

<!-- page 13 -->

## 4 Conventions

### 4.1 Conformance language

The keyword "reserved" indicates a provision that is not specified at this time, shall not be used, and may be specified in the future. The keyword "forbidden" indicates "reserved" and in addition indicates that the provision will never be specified in the future.
### 4.2 Operators

## NOTE

#### 4.2.1 Arithmetic operators

Many of the operators used in document are similar to those used in the C programming language. & + − × / << >> bitwise AND operation addition subtraction (as a binary operator) or negation (as a unary prefix operator) multiplication division without truncation or rounding x s x s left shift: x << s is defined as x ×2 s x right shift: a >> is defined as ⎿ /2 ⏌ a y Na x N umod
#### 4.2.2 Logical operators

umod is the unique value y between 0 and –1 for which + = with a suitable integer
|| && logical OR logical AND !
#### 4.2.3 Relational operators

logical NOT > ≥ < ≤ greater than greater than or equal to less than less than or equal to == equal to !=
#### 4.2.4 Precedence order of operators

not equal to
## NOTE

Operators are listed below in descending order of precedence. If several operators appear in the same line, they have equal precedence. When several operators of equal precedence appear at the same level in an expression, evaluation proceeds according to the associativity of the operator either from right to left or from left to right. © ISO/IEC 2024 – All rights reserved

<!-- page 14 -->

Operators Type of operation Associativity () [] – ×, / umod +, − <<, >> < , >, ≤, ≥
#### 4.2.5 Mathematical functions

& expression indexing of arrays unary negation multiplication, division modulo (remainder) addition and subtraction left shift and right shift relational bitwise AND left to right left to right left to right left to right left to right left to right left to right left to right x  x  x | | x log2( x ) sign( x ) x min max
, )
, clamp( xi i max a ( b ) max( i xi
, ) min a ( b ) x x ceil of x : returns the smallest integer that is greater than or equal to x floor of x : returns the largest integer that is less than or equal to x x x x absolute value of
, | | equals – for < 0, otherwise logarithm to the basis of 2, e.g. log2(1)=0 and log2(2)=1 x x x sign of x
, 0 if x is 0, +1 if is positive, -1 if y is negative y y x min square root of x max min x to the range [
,
, i.e. non-negative number max min
, ) equals clamp clamp( ] if min < x xi
,
, max such that max x × = x if > i or otherwise b a maximum of a sequence of numbers { a b } enumerated by the index if >
, otherwise xi i a minimum of a sequence of numbers { a b b } enumerated by the index
, < min(
## 5 Functional concepts

if )
, otherwise
### 5.1 Sample grid, sampling and components

W H W width H height f sample positions horizontally and An image is defined as a rectangular array of scalar or vectorial samples regularly aligned along a sample f sample positions vertically. f is grid of components N of the image. The vector dimension of the image samples corresponds to the number of called the component colour components present and is indicated by of the image. Each dimension of this vectorial data corresponds to one of the image, and typically represents one of multiple colour channels of the data. Components may be red, green and blue, or Luma (Y) and Chroma (Cb,Cr). These sampling factor are only non-exhaustive examples of components, and other uses are possible. c, the number of f is called the and i A given component may or may not populate every point on the sample grid. The distance, or
, s between sample points of a component shall be constant in each spatial dimension throughout the image. s ] respectively The horizontal and vertical sampling factors of component y[ enumerates the components. Annex B provides further specifications on component sampling. of an image are denoted by ] where does not x[ i i i This document specify how to interpret the sample values, or how to reconstruct from subsampled components an array of samples that populates the entire sample grid, i.e. it does not specify how to © ISO/IEC 2024 – All rights reserved

<!-- page 15 -->

upsample components to the full resolution of the sampling grid. The sampling grid provides only an abstract coordinate system for the computation of positions and dimensions of codestream elements.
### 5.2

Interpretation of CFA data This document defines coding tools and signalling for compression of Bayer-type CFA image data. According to this specification, each sampling grid point represents a super pixel of four sensor elements containing at least one sample of each component. Thus CFA data is interpreted as an image having four components, where each sampling grid point describes four spatially disjoint sensor elements (one element per channel). Squares represent individual sensor elements and circles represent sampling grid points. Groups of four sensor elements overlapping with the same sampling grid point form one super pixel. Figure 1 — Example of the interpretation of a GRBG Bayer-type CFA image Moreover, regardless of the CFA sensor spatial subpixel arrangement, the Star-Tetrix colour transform of this document defines a strict order on the components assigning the red channel to component 0, the green channels to components 1 and 2, and the blue channel to component 3. The spatial subpixel arrangement is signalled by the CRG marker. Figure 1 shows only one of the four potential subpixel arrangements of a Bayertype CFA.
### 5.3 Wavelet decomposition

bands This document provides an efficient representation of image signals through the mathematical tool of wavelet analysis. The wavelet filter process specified in Annex E separates each component into multiple
, where each band consists of multiple describing the image signal of a given component within a frequency domain specific to the wavelet filter type coefficients precincts
, i.e. the particular filter corresponding to the band. packets Wavelet coefficients are grouped into that contribute to a spatial region of the image. Each precinct is encoded into one or multiple codestream syntax specified in Annex A.
, where each precinct includes all coefficients over all bands in the slices Precincts are furthermore grouped into
. Wavelet coefficients in precincts that are part of different slices can be decoded independently from each other. Note, however, that the wavelet transformation runs across slice boundaries. A slice always extends over the full width of the image, but may only cover parts of its height. Bands, band types, precincts and slices are formally defined in Annex B. © ISO/IEC 2024 – All rights reserved

<!-- page 16 -->

### 5.4 Codestream

The codestream is a linear stream of bits from the first bit to the last bit. For convenience, it can be divided into (8-bit) bytes, starting with the first bit of the codestream. Bits within bytes are enumerated from the LSB to the MSB, with the least significant bit having the index zero.
## Annex A specifies the codestream syntax that defines the coded representation of compressed image data

for exchange between application environments. Any compressed image data shall comply with the syntax and code assignments appropriate for the decoding processes defined in this document. entropy coded data The codestream consists of multiple syntax elements: to steer the decoding process, and itself. Packets are further grouped into magnitude, signs or significance of parts of the encoded image data. define control information necessary that represent image information
, each of which includes particular information such as marker segments organized in subpackets packets All marker segments defined in this document are specified in Annex A. This annex also provides an overview on the organization of the codestream. Packets and subpackets are specified in Annex C.
## 6 Encoder requirements

An encoder is an embodiment of a process that generates a codestream that conforms to the syntactical requirements specified in Annex A. Annex C to Annex G include informative subclauses that indicate how an encoder may be implemented.
## 7 Decoder

### 7.1 Decoding process general provisions

Figure 2 provides an overview on the decoding process and the layout of this document. Codestream decoding can be grouped into a syntax analysis part in block 1, an entropy decoding stage consisting of multiple blocks
### 2.1 to 2.5, an inverse quantization in block 3, an optional inverse temporal decorrelation in block 7, an inverse

wavelet transformation in block 4 and an inverse multiple component transformation in block 5. In block 6, sample values are scaled, a DC offset is added, and they are clamped to their nominal ranges. © ISO/IEC 2024 – All rights reserved

<!-- page 17 -->

Figure 2 — Decoder overview In Block 1, described in Annex A, the decoder analyses the codestream syntax and retrieves information on the layout of the sampling grid, and the dimensions of slices and precincts. The subpackets of the entropy coded data segment of the codestream are then decoded by the procedures given in Annex C to form significance information, sign information, bitplane count information, quantization indices and optionally TDC selection flags. This operation is performed in blocks 2.1 to 2.5 in Figure 2. significance subpacket b In block 2.1, significance information is decoded from the j Denoted by the array
, b p within the precinct header, it either contains non-zero coefficients, or has a non-zero bitplane count prediction residual. as specified in subclause C.5.2. ], significance information indicates the presence of significant code groups Rm significance group. Each significance group corresponds to a run of code groups indexed by in the picture
. A code group is significant if, depending on the Run Mode flag and band
, line p th Z λ λ [ j
,
, bitplane count subpacket M In block 2.2, bitplane counts are decoded from the p the procedures specified in subclause E.6 The integer array b λ indexed by precinct wavelet coefficients in the code group λ b λ
, [
, p
, line p p g g v x b as specified in subclause C.5.3 by ] indicates the bitplane counts of the x
, λ and band b
. data subpacket In block 2.3, Quantization index magnitudes are decoded from the ] in precinct
, λ p as specified in subclause C.5.4. b x s [
,
,
, line
, band
, and horizontal position In block 2.4, the signs of the quantization indices k included in a separate sign subpacket as specified in subclause C.5.5. ] are either interleaved in the data subpacket, or TDC subpacket Y p b λ [
,
,
, as specified in In block 2.5, the TDC selection flags subclause C.5.6. This subpacket only exists for slices that enable TDC, see subclause A.4.15. Otherwise, the λ p TDC selection flags shall be inferred to be 0, indicating intra-coding. ] are decoded from the x c
, In block 3, decoded quantization index magnitudes by the dequantizer specified in Annex D, giving wavelet coefficient residuals ] and signs p ] are then inversely quantized [ ]. p b b b λ λ v x x s [ [ [
,
,
,
,
,
,
,
,
,
,
, © ISO/IEC 2024 – All rights reserved

<!-- page 18 -->

c p λ b x f x λ b p In block 7, wavelet coefficient residuals the frame buffer
, b are the wavelet coefficients
, line that does not use TDC, then c’ specified in Annex H. [ [
,
,
,
,
, p c’ [ x λ c’
, ] are optionally inversely temporally predicted by means of b x ] and TDC selection flags ] if they are present. The result of this operation λ p ]. If the wavelet coefficient residuals are part of a precinct, band and
, ]. Inverse temporal decorrelation is
, b ] shall be identical to [ p
, λ
, x p b λ x c [ [
,
,
,
,
,
, Y p λ b k y In block 4, wavelet coefficients
## Annex E. This process generates spatial sample values for all components, denoted by

and x are here subsampled sampling grid positions of component O y c c [
,
,
,
. [
,
, ]. Coordinates x ] are inversely wavelet transformed by the procedure specified in O y x c Ω x y c In block 5, spatial sample values giving intermediate image sample values specified in Annex F. [
,
, ] undergo optionally an inverse multiple component transformation, ]. The inverse multiple component transformation is c y Ω x [
,
, ], an optional non-linear In block 6, a DC offset is added to the decorrelated sample values y transformation is applied, they are scaled to their nominal range and then clamped to the range of the bit-
] populating the sample precision of the output, giving the final reconstructed output sample values grid positions ×
### 7.2 Decoder requirements

]. This procedure is specified in Annex G. y[ x[ ],
, R × y
, x x c c c s s [ [
,
, A decoder is an embodiment of the decoding process. The decoding process converts a codestream by performing the process specified in this document to sample values arranged on a rectangular sampling grid. Annex A to Annex H describe and normatively specify the decoding process. All decoding processes are normative. Decoder conformance and test procedures to test for conformance are specified in ISO/IEC 21122-4. There is no normative or required specification for the particular internal steps or ordering of internal operations to be performed within the decoder that is used to produce the normatively specified result. Only the result that is externally observable as the decoded output image produced by the decoder is required to match the result produced by the decoding processes specified in this document up to a conformance-level dependent error bound that is specified in ISO/IEC 21122-4. The descriptions use particular implementation techniques for illustrative purposes only, and any implementation that is able to reproduce the same results as those generated by the algorithms specified herein is conforming to this document. © ISO/IEC 2024 – All rights reserved

<!-- page 19 -->

## Annex A

(normative) Codestream syntax A.1 General A.1.1 Marker segments and entropy coded data entropy coded data The compressed data format consists of an ordered collection of syntax elements. This document
. distinguishes between three types of syntax elements: Markers serve to identify the various structural parts of the codestream. Most markers start marker segments, where marker segments signal the characteristics of the encoded image and encapsulate parameters configuring the decoder. Some markers stand alone. Entropy coded data consists of the input to the decoding procedure described in Annex C to Annex H which reconstructs this data to the output image. A.1.2 Key to syntax information marker segments Marker and
, fixed-length variable-length JPEG XS codestream syntax elements belong to one of two categories: numerical values, or codes. In the syntax tables, the “Syntax” column indicates the category to which each codestream syntax element belongs, in the “Size” column the size of each field is identified (if applicable). Fixed-length numerical values and are unsigned integers and are denoted by is the number of bits used to represent the value. Variable-length codes are denoted by
, see subclause C.7 for the pad(n) normative decoding procedure of variable length codes. Bit strings and variable-length codes appear in the n codestream with the left bit first; numerical values appear most-significant bit first. The notation
- bit boundary, i.e. to an integer indicates a variable number padding bits. Padding aligns the bitstream to an multiple of −1 bits depending on bits relative to the start of the bitstream. Thus, the position within the bitstream. While padding bits can have arbitrary values, a decoder shall ignore their value. The notation indicates an arbitrary number of filler bytes a decoder shall remove without interpreting their value. The amount of filler bytes can be inferred from a length field of a corresponding syntax element. expands to 0 to
, where fill() pad(n) u(n) vlc n n n if Syntax elements may be conditionally included in the codestream; this is indicated by syntax column of the syntax tables. All syntactical elements enclosed in curly brackets following the clause are only included if the expression following the clause is non-zero. if for if clauses in the clauses in the Syntax column of the syntax The sequence of multiple similar elements is indicated by tables. The elements to repeat are enclosed in curly braces. The loop itself is specified through three syntax elements: an initializer setting a dummy count variable indicating the current iteration position of the loop, a condition on the count variable for continuing the loop, and an iteration statement that updates the count variable for the next loop. The three expressions are separated by semicola. The loop syntax and the syntax for conditional inclusion of elements follow closely the syntax of the C
## NOTE

programming language. A.2 Codestream syntax general provisions image components A JPEG XS codestream describes an aligned along a regular rectangular sampling grid. Each component is a rectangular arrangement of integer sample values on the sampling grid of the image. The samples of a component need not populate every possible position on the sampling grid, horizontal see subclause B.1. The horizontal and vertical spacing between populated sample positions of a component relative to the sampling grid are denoted the of the component ]. Subsampling factors may be either 1 or 2. The codestream and are indicated by the symbols vertical sampling factors consisting of 1 to 8 ] and and y[ x[ s s i i © ISO/IEC 2024 – All rights reserved

<!-- page 20 -->

reconstruction process described by this document assigns to each sample of each image component an integer precision between 8 and 16 bits. Picture Header. The dimensions of the image, along with the number of components, the sampling factors and the precision of the components are encoded in a syntax element denoted as The samples in the image are reconstructed by an inverse wavelet transformation and potentially from an inverse temporal prediction. This reconstruction uses entropy coded wavelet coefficients and/or wavelet coefficient residuals arranged in multiple wavelet bands, see Annex B for details, and potentially the contents of a frame buffer, see
## Annex H. The wavelet reconstruction algorithm is specified in Annex E.

precinct Wavelet coefficients are grouped into precincts. A includes entropy coded data decoding to a entropy rectangular array of wavelet coefficients per bands such that the included wavelet coefficients contribute to coded data a given spatial region of the image. A precinct is represented in the codestream by a precinct header filler bytes
, packets s λ followed by b
. p of a precinct
, where each packet significance, bitplane counts, magnitude and The entropy coded data is organized in multiple
. Each packet consists of multiple subpackets where each subpacket one or multiple bands contributes to one aspect of the data, such as Filler bytes and padding does not have any impact on the decoded image. The purpose of the filler bytes is to prevent buffer underflow of a potential transmission buffer, the purpose of padding is to align the syntax elements in the bitstream to byte boundaries. Decoders learn the number of filler bytes following the precinct from the precinct header specified in subclause C.2, and the number of filler bytes in the subpackets from the subpacket header in specified in subclause C.3. Decoders shall ignore the value of the filler bytes. contributes to one line signs. and pad()
## NOTE 1

is to ensure alignment of the codestream to byte boundaries only. Filler bytes are in no relation to the output of the function defined in subclause A.1.2 whose purpose
## NOTE 2

document. Buffer models, profiles and levels are specified in ISO/IEC 21122-2 and are beyond the scope of this slices Precincts are grouped into
. Each slice consists of an integral number of precincts, and extends over the full width of the image. Even though the wavelet coefficients/wavelet coefficient residuals within each slice precincts slice header can be decoded independently, the wavelet transformation runs across slices. A slice is represented in the codestream by a following the slice header. and one or multiple Figure 2 gives an overview of the hierarchy of JPEG XS codestream syntax structures, Table A.1 defines the overall codestream syntax. While Table A.1 lists the syntax elements of JPEG XS, it does not attempt to define a particular order of the markers in a JPEG XS codestream, and the order presented in this table is rather an illustration of one possible syntactically correct order. Requirements on the placement of a specific marker are normatively defined in the subclause of Annex A defining the marker. Table A.1 — JPEG XS codestream syntax overview Syntax Notes Defined in Picture() { SOC_marker() capabilities_marker() picture_header() component_table() weights_table() refresh_weights_table() nonlinearity_marker() Identifies this codestream as JPEG XS codestream Identifies the capabilities a decoder needs to support to be able to decode the codestream Defines the overall structure of the codestream Defines the precision and sampling factors of all components in the image Defines weight and gain factors that steer the decoding process. Defines weights and gain factors for forcibly refreshed bands in TDC-enabled slices Optional definition of non-linearities for component reconstruction Table A.3 Table A.6 Table A.7 Table A.15 Table A.25 Table A.26 Table A.16 © ISO/IEC 2024 – All rights reserved

<!-- page 21 -->

Table A.1 (continued) Table A.1 (continued) Syntax Notes Defined in cwd_marker() cts_marker() crg_marker() tpc_marker() extension_marker() for(t=0,p=0;!endofimage;t=t+1) { slice_header() or tdc_slice_header() for(u=0;u<Np[t];p=p+1,u=u+1) { compute_packet_inclusion(p) precinct_header(p) for(s=0;s<Npc[p];s=s+1) { packet_header(p,s) packet_body(p,s) } fill() } } EOC_marker() } Optionally disable wavelet decomposition on some components Colour transformation specification for the star-tetrix transformation Optional component registration, mandatory if the star-tetrix transformation is used Optional indication of TDC modes and refresh groups. Table A.18 Table A.19 Table A.21 Table A.22 Optional extension of the codestream syntax t Table A.23 Loop over all slices until all wavelet coeffip cients of the image have been decoded, where is the slice index and the precinct index Identifies the ordering of slices and identifies the TDC mode of the slice. The slice header shall be either an SLH or SLI header, depending on the profile of the codestream. p t Loops over all precincts in a slice, where the slice index, enumerates precincts within a slice is the precinct index and u is Table A.27, Table A.28 Determine the packets that are part of this precinct. Defines prediction modes and the quantization of the precinct Loop over all packets of this precinct Defines flags and sizes of the packet Contains the entropy coded data of this packet End of loop over subpackets Possible byte-aligned filler bytes at the end of the precinct to reach the target bitrate. A decoder shall ignore this data. Subclause C.2 specifies how to determine the number of filler bytes. End of loop over precincts within a slice End of loop over slices Table B.4 Table C.1 Table C.4 Table C.5 Subclause C.2 Identifies the end of the JPEG XS codestream Table A.4 N p p compute_packet_ inclusion(p)
## NOTE 3

The number of packets pc[ ] depends on the precinct index and is computed by the function specified in Table B.4; in particular, the last precinct of the image will, by this procedure, include less bands than all other precincts if the image height is not divisible by the precinct height. A.3 Markers and marker segments marker Markers serve the purpose to identify the various structural parts of the codestream format. Markers segment may either stand alone, or may start containing a related group of parameters. A marker segments payload data consists of a marker, followed by a two-byte length field, followed by the parameters in the marker in the following. The two-byte length field identifies the length of the segment, denoted as marker segment, which consists of the length of the payload data in bytes, and the size of the length field itself (two bytes). The length field does not include the size of the marker. Parameters are encoded with the most significant byte first, a convention often denoted as big endian. All markers are assigned two-byte codes: a 0xff byte followed by a byte that is not equal to 0x00 or 0xff. Table A.2 lists all markers used by this document, and, in combination with referenced tables from Table A.1, © ISO/IEC 2024 – All rights reserved

<!-- page 22 -->

specifies whether they stand alone or introduce a marker segment. The semantics of each marker and associated marker segment are further specified in subclause A.4. Some marker segments are currently reserved for future ISO/IEC use. Marker segments can be mandatory, optional, or mandatory only if indicated by a capability signalled in the capability marker segment. Optional marker segments may be ignored by decoder implementations by skipping over the marker segment by using its length field. The capability marker may indicate the marker segments required to correctly decode a given codestream. If a decoder encounters a capability it does not implement, it should abort decoding. If the length field of a marker segment does not match the specified value or is not in the specified range, the codestream is ill-formed. For that, decoders should check whether the marker length field has its specified value, or is within the range specified in this document.
## NOTE 1

It is possible that future editions of this document will include additional fields in the marker segments and hence require an increase in the size the marker segments. Such additional fields can be necessary to decode the codestream. The above ensures that decoders fail properly when attempting to decode an extended codestream syntax that they do not support. NOTE 2 Other International Standards implement bit-stuffing or byte-stuffing procedures to ensure that markers can be identified uniquely without decoding or interpreting entropy coded data segments. This is not the case for this document. If a decoder loses synchronization with the codestream, lower-level transport mechanisms are required to regain synchronization as it is not possible to search the codestream for markers; bit patterns within the entropy coded data segment can replicate the byte sequences used to identify marker segments. Table A.2 — JPEG XS codestream markers Code assignment Symbol Description Mandatory/Optional Reference 0xff10 0xff11 0xff12 0xff13 0xff14 0xff15 0xff16 0xff17 0xff18 0xff19 0xff1a 0xff1b 0xff20 0xff21 0xff50 SOC EOC PIH CDT WGT COM NLT CWD CTS CRG TPC WGR SLH SLI CAP Start of codestream Mandatory End of codestream Mandatory Picture header Component table Weights table Mandatory Mandatory Mandatory Extension marker Optional Nonlinearity marker Optional Component-dependent wavelet decompo-
sition marker Colour transformation specification marker Component registration marker Temporal prediction control marker Optional Cpih Mandatory if =3, shall not be present otherwise. Cpih Optional, mandatory if =3 Optional Refresh Weights table Optional Slice header Depending on profile, see ISO/IEC 21122-2 TDC enabling slice header Depending on profile, see ISO/IEC 21122-2 Capabilities Marker Mandatory A.4.1 A.4.2 A.4.4 A.4.5 A.4.12 A.4.10 A.4.6 A.4.7 A.4.8 A.4.9 A.4.10 A.4.13 A.4.14 A.4.15 A.4.3 All other values Optional Reserved for future ISO/ IEC purposes © ISO/IEC 2024 – All rights reserved

<!-- page 23 -->

A.4 Syntax description of marker segments A.4.1 Start of codestream Function: Usage: Identifies the codestream as containing an image represented in accordance with this document. Shall be the first marker segment in a codestream. There shall be only one SOC marker at the beginning of each JPEG XS codestream. Table A.3 — Start of codestream marker syntax Syntax Notes Size Values start_of_codestream() { SOC } A.4.2 End of codestream Function: u(16) 0xff10 Usage: Identifies the end of a JPEG XS codestream. Shall be the last marker segment in a codestream. There shall be exactly one EOC marker at the end of each JPEG XS codestream. Table A.4 — End of codestream marker syntax Syntax Notes Size Values end_of_codestream() { EOC } A.4.3 Capabilities marker Function: Usage: Identifies capabilities required to decode a JPEG XS codestream. u(16) 0xff11 Shall be the second marker segment in a codestream. There shall be exactly one CAP marker which shall be placed behind the SOC marker. cap
## NOTE 1

The above implies that the CAP marker is always present, even when the [] array is empty. Lcap Lcap cap Lcap cap Lcap Table A.5 specifies the assignment of capabilities to bits, Table A.6 the syntax of the CAP marker segment. −2) ×8−1] are not all 0. >2, Furthermore, shall be selected such that for −2) ×8−8] to [( [(
## NOTE 2

This condition on the cap array can always be arranged by selecting a smaller Lcap.
## NOTE 3

Bit 0 of the capability marker is intentially unused. © ISO/IEC 2024 – All rights reserved

<!-- page 24 -->

Table A.5 — Capability bit assignment Bit number i in cap[] array Bit value Meaning All other bits Support for Star-Tetrix transform not required, and support for CTS marker not required Support for Star-Tetrix transform and CTS marker required Support for quadratic non-linear transform not required Support for quadratic non-linear transform required Support for extended non-linear transform not required i s ]=1 for all components i Support for extended non-linear transs form required y[ component i with y[ Support for component-dependent wavelet decomposition not required ]>1 present i Support for component-dependent wavelet decomposition required Support for lossless decoding not required Support for lossless decoding required Support for packet-based raw-mode switch not required Support for packet-based raw-mode switch required Support for TDC not required Support for TDC required, a frame buffer is required Table A.6 — Capabilities marker syntax Reserved for ISO/IEC purposes Syntax Notes Size Values capabilities_marker() { CAP Lcap for(i=0;i<(Lcap−2)×8;i=i+1) { cap[i] } padding } A.4.4 Picture header Size of the capabilities marker in bytes (not including the marker) Loop over capabilities bits i cap i u(16) u(16) 0xff50 Variable Requirement of capability i. if capability codestream, and 0 otherwise. is required for decoding a [ ] is 1 u(1) End of loop over capabilities bits Pad to an integer number of bytes pad(8) Table A.7 defines the syntax of the picture header. © ISO/IEC 2024 – All rights reserved

<!-- page 25 -->

Function: Provides information on the dimensions of the image, the precision of its component and the Usage: configuration of the decoder. Shall be the third marker segment in a codestream directly after the CAP marker. There shall be exactly one PIH marker in a JPEG XS codestream. Table A.7 — Picture header syntax Syntax Notes Size Values picture_header() { PIH Lpih Lcod Ppih Plev Wf Hf Cw Hsl Nc Ng Ss Bw Fq Br Fslc Ppoc Cpih a Size of the segment in bytes (not including the marker) u(16) 32— (2 − 1) u(16) 0xff12 Size of the entire codestream in bytes from SOC to EOC, including all markers, if constant bitrate coding is used. 0 if variable bitrate coding is used. Profile this codestream complies to. Decoders should abort decoding if they identify a profile they do not implement. Level and sublevel to which this codestream complies. Decoders should abort decoding if they identify a level they do not support. u(32) u(16)
## 0 for no restrictions, see ISO/IEC 21122-2

for profiles. u(16)
## 0 for no restrictions, see ISO/IEC 21222-2

for additional levels. max
, — ( L x i   ) × N ) − ( s x i Width of the image in sample grid positions u(16) max i Height of the image in sample grid positions max ( i s Width of a precinct in multiples of [ ] ix NL x ), )× × u(16) y s ( N
, — ( L y ) − i   ) × )− — ( Cw max ( × i =0 or NL x
, s x [ ] i )× NL x ), ( umod × C W w f such that either max ( ) [ ] × ix i s ≥ — ( )− sample positions, other than the rightmost precincts. If this field is 0, precincts are as wide as the image. See subclause B.5 for details. u(16) Height of a slice in precincts other than the last slice u(16) Number of components in the image u(8) 1—8 Number of coefficients per code group Number of code groups per significance group u(8) u(8) B Nominal bit precision of the wavelet coefficients u(8) 20,18 or a [0] Number of fractional bits in the wavelet coefficients Number of bits to encode a bitplane count in raw Slice coding mode Progression order of bands within precincts u(4) u(4) u(1) u(3) a 8,6 or 0 a
## 4 or 5

See Table A.14 See Table A.13 Colour transformation to be used for
, B inverse decorrelation B F u(4) See Table A.9 See Table A.8 for valid combinations of w r and q and further restrictions. © ISO/IEC 2024 – All rights reserved

<!-- page 26 -->

Table A.7 (continued) Table A.7 (continued) Syntax Notes Size Values NL,x NL,y Lh Rl Qpih Number of horizontal wavelet transformations u(4) 1–8 i s i N
, Number of vertical wavelet transformations of non-vertically subsam-
pled components Long header enforcement flag u(4) u(1) Raw-mode selection per packet flag u(1) max (log2( y[ ]))–min( L x,6)
## 0 or 1

## 0 or 1

Fs Rm a Inverse quantizer type Sign handling strategy B
, B F u(2) u(2) u(2) See Table A.10 See Table A.11 See Table A.12 Run mode Cw See Table A.8 for valid combinations of w r and q and further restrictions. ensures that all but the rightmost precincts have in the LL band at least 8 samples, and
## NOTE 1

that all bands of the rightmost precincts are non-empty. N
, N The condition on Wf s p L In case
## NOTE 2

× Lh sgn[ ] ≥ 2048 or happen that allows to enforce long headers. L
, c>16376 and L y=0, or for components that do participate in the wavelet decomposition, it can flag dat[p,s] ≥ 32768. As such sizes cannot be expressed by the short header, the can safely remain 0 as long as at least one vertical decomposition is performed. Table A.8 — Valid combinations of Bw, Br, B[i] and Fq Lh Bw B B [0] [0] Cpih 2,4−15 Qpih 2–3 Fq Br Additional constraints B i B i Notes [0] for all [ ]= B[0] ≤ 12 B B i [0] for all [ ]= B[0] > 12 i
.
. A NLT marker segment shall be present. Bit 2 or bit 3 of the CAP marker segment shall be 1, see subclause A.4.3 This combination indicates lossless coding for source data with bit depth ≤ 12. This combination indicates lossless coding for source data with bit depth > 12. This combination is used in case a non-linearity is present. Bits 2 and 3 of the CAP marker segment not present or shall be 0. Table A.9 — Colour transformation Regular case. Meaning No colour transform Reversible colour transformation (see Annex F) Star-Tetrix transform (see Annex F) Reserved for ISO/IEC purposes Table A.10 — Quantizer type Meaning Deadzone quantizer (see Annex D) Uniform quantizer (see Annex D) Reserved for ISO/IEC purposes © ISO/IEC 2024 – All rights reserved

<!-- page 27 -->

Table A.11 — Sign handling strategy Meaning Signs encoded jointly with the data Signs encoded separately Reserved for ISO/IEC purposes Table A.12 — Run mode Meaning Runs indicate zero prediction residuals Runs indicate zero coefficients Reserved for ISO/IEC purposes Table A.13 — Progression order Fs 2–3 Rm 2–3 Ppoc Meaning 1–7 Fslc The progression order as defined by subclause B.7 (resolution-line-band-component) Reserved for ISO/IEC purposes Table A.14 — Slice coding mode Meaning The wavelet transformation runs across slice boundaries A.4.5 Component table Reserved for ISO/IEC purposes Function: Table A.15 defines the syntax of the component table. N Usage: component in the image. The number of components itself is given by the This marker segment specifies the component precision and the sampling factors of each c parameter of the picture header. There shall be exactly one component table in each JPEG XS codestream. It shall precede the first slice header. Table A.15 — Component table syntax Syntax Notes Size Values component_table() { CDT Lcdt for(c=0;c<Nc;c=c+1) { B[c] sx[c] sy[c] } } +Nc × 0xff13 8–16
## 1 or 2 for components 1 and 2, 1 for

all other compoc s nents x[ 1– ] Size of the segment in bytes, not including the marker Loop over components. The number of components is specified in the picture header. c c Bit precision of component Horizontal sampling factor of component u(16) u(16) u(8) u(4) c Vertical sampling factor of component u(4) End of loop over components © ISO/IEC 2024 – All rights reserved

<!-- page 28 -->

A.4.6 Nonlinearity marker Function: Table A.16 defines the syntax of the nonlinearity marker segment. Defines an optional non-linear transform to be applied after inverse multiple component Usage: transformation. Zero or one nonlinearity marker segments may be present in a codestream. If present, any nonlinearity marker segment shall precede the first slice header in the codestream and shall follow the =0, i.e. in the lossless mode. picture header. This marker shall not be present if Table A.16 — Nonlinearity marker segment syntax Fq Syntax Notes Size Values nonlinearity_marker(){ NLT Lnlt Tnlt if(Tnlt == 1) { σ α DCO=α−σ×215 } if(Tnlt == 2) { T1 T2 E } } Size of the marker segment, not including the marker u(16) u(16) 0xff16
## 5 or 12

Type of the non-linearity u(8) See Table A.17 Additional data for quadratic non-linearity Sign bit of the DC offset DCO Remaining bits of the DC offset Compute the representation. offset from two’s complement Bw Additional data for extended non-linearity Bw Upper threshold for region 1, in units of Upper threshold for region 2, in units of Exponent of the linear slope in region 2 u(1) u(15) u(32) u(32) u(8) 0—1 0—2 −1 Bw Bw −1 −1 1—2 1—2 1—4 Tnlt Meaning Table A.17 — Tnlt encoding Quadratic non-linearity Extended non-linearity All other values A.4.7 Component-dependent wavelet decomposition marker Reserved for ISO/IEC use Function: Table A.18 defines the syntax of the component-dependent decomposition marker. Optionally suppresses the wavelet decomposition for one or more components. If this marker is Sd Usage: not present, the value of shall be 0. N Zero or one component-dependent wavelet decomposition marker segments may be present in a c>3. If present, shall precede the first slice header in the codestream codestream. Shall only be present if and shall follow the picture header. © ISO/IEC 2024 – All rights reserved

<!-- page 29 -->

Table A.18 — Component-dependent wavelet decomposition marker segment syntax Syntax cwd_marker(){ CWD Lcwd Sd } Notes Size Values Size of the marker segment, not including the marker Number of components for which the wavelet decomposition is suppressed. u(16) u(16) 0xff17 N u(8) c−1 1— Furthermore, Sd shall be selected such that all components that are c excluded from the wavelet transformation have ]=1 and ]=1 y[ x[ c s s A.4.8 Colour transformation specification marker Function: Table A.19 defines the syntax of the colour transformation specification marker segment. Usage: Defines parameters of the Star-Tetrix transformation. Cpih Zero or one colour transformation specification markers shall be present in a codestream. Shall be =3, i.e. if the Star-Tetrix transformation is in use. Shall precede the first slice header present if and only if in the codestream and shall follow the picture header. Table A.19 — Colour transformation specification marker segment syntax Syntax cts_marker(){ CTS Lcts Reserved Cf e1 e2 } Cf Notes Size Values Size of the marker segment, not including the marker Reserved for ISO/IEC purposes Size and extent of the transformation Exponent of first chroma component Exponent of second chroma component Table A.20 — Cf encoding Meaning u(16) u(16) u(4) u(4) u(4) u(4) 0xff18 See Table A.20 0..3 0..3 Full transformation, access to the line below and above required Restricted in-line transformation, no access to neighbouring lines All other values Reserved for ISO/IEC use A.4.9 Component registration marker Function: Table A.21 defines the syntax of the component registration marker segment. Defines the relative placement of the component to the sample grid. If not present, the components Usage: are placed at the vertices of the sampling grid. Cpih Zero or one component registration markers shall be present in a codestream. If =3, exactly one CRG marker segment shall be present. If present, any CRG marker segment shall precede the first slice header in the codestream and shall follow the picture header. © ISO/IEC 2024 – All rights reserved

<!-- page 30 -->

Table A.21 — Component registration marker segment syntax Syntax crg_marker(){ CRG Lcrg for(c=0;c<Nc;c=c+1) { Xcrg[c] Ycrg[c] } } Notes Size Values u(16) u(16) 0xff19 2+4×Nc u(16) 0—65535 u(16) 0—65535 Size of the marker segment, not including the marker Loop over all components Relative horizontal placement of component c to the sample grid points in units of 1/65536. 0 indicates placement at the horizontal position of the sample grid, a positive value a displacement to the right of the sample grid position. A value of 32768 corresponds to a placement mid-way between the sample grid point and the next sample grid point to the right. Relative vertical placement of component c to the sample grid points in units of 1/65536. 0 indicates placement at the vertical position of the sample grid, a positive value a displacement to the bottom of the sample grid position. A value of 32768 corresponds to a placement mid-way between the sample grid point and the next sample grid point below. End of loop over all components A.4.10 Temporal prediction control marker Function: Table A.22 defines the syntax of the temporal prediction control marker segment. Q Q Defines which TDC selection groups are forcibly encoded without temporal prediction and identifies the quantization adjustment for TDC selection groups. If the TPC marker is not present, the inferred values of ] shall be −∞, indicating that no position matches the hash position. That is, if this marker is not present, the temporal prediction of TDC selection groups only depends on the TDC mode selection ] in the precinct header, see subclause C.2. br shall be 0, the inferred value of Si shall be 8, and the inferred values of bi and h[ D i[ p b b S
, Usage:
## NOTE 1

The value for Si is always equal to 8, even in the case that no TPC marker is present. Zero or one temporal prediction control markers shall be present in a codestream. Table A.22 — Temporal prediction control marker segment syntax Syntax Notes Size Values tpc_marker(){ TPC Ltpc Si σbi αbi Size of the marker segment, not including the marker Size of a TDC selection group in code groups Sign bit of Qbi Remaining bits of Qbi u(16) u(16) u(8) u(1) u(3) 0xff1a 4+2×NL — — © ISO/IEC 2024 – All rights reserved

<!-- page 31 -->

Table A.22 (continued) Table A.22 (continued) Syntax Notes Size Values Qbi=αbi−σbi×23 σbr αbr Qbr=αbr−σbr×23 for(b=0;b<NL;b=b+1) { Yh[b] Sh[b] } } Y D [p,λ,b,k] and Compute the quantization adjustment of wavelet coefficient residuals that are intra coded due to TDC selection i[p,b] TDC mode flags selection Sign bit of Qbr Remaining bits of Qbr Compute the quantization adjustment of wavelet coefficient residuals that are intra coded due to positional match, see subclause H.7 for the condition and details. Loop over all bands Positional hash value of the intra-refresh of band b Hash mask exponent for intra-refresh of band b u(1) u(3) u(8) u(8) — — — — Y b S b h[ ]
## NOTE 2

forcefully coded without prediction from the frame buffer, see Annex H.3 for details. A.4.11 Extension marker and the hash mask exponent The hash value h[ ] are used to identify coefficient positions that are Function: Table A.23 defines the syntax of the extension marker segment. Usage: Extends the codestream by generic or vendor-specific metadata. Zero or more extension marker segments may be present in a codestream. If present, any extension marker segment shall precede the first slice header in the codestream. Table A.23 — Extension marker segment syntax Syntax extension_marker(){ COM Lcom Tcom Dcom padding } Notes Size Values Size of the marker segment, not including the marker Type of the extension User-defined data u(16) u(16) 0xff15 Variable, at least 4 u(16) See Table A.24 Variable Variable Padding to an integer number of bytes pad(8) © ISO/IEC 2024 – All rights reserved

<!-- page 32 -->

Table A.24 — Tcom encoding Tcom Meaning 0x0000 0x0001 Vendor of the encoder, Dcom is a zero-terminated, ISO 10646 encoded string identifying the vendor of the encoder Copyright statement that the codestream creator want to convey to users of the codestream. The interpretation of this statement is beyond the scope of this document. Dcom is a zero-terminated, ISO/IEC 10646 encoded string identifying this statement. 0x8000-0xffff Vendor-specific information. Tcom identifies the type of extension and the vendor. All other values A.4.12 Weights table Reserved for ISO/IEC use Function: Table A.25 defines the syntax of the weights table. This marker segment contains parameters required to set the gain of each band relative to the precinct quantization. Together with the parameters in the precinct header, see subclause C.2, this allows to determine the quantization of the wavelet coefficients in this band. Details on how to use these parameters are specified in subclause C.6.2. The number of wavelet bands and the relation between band, component Usage: and wavelet filter type are specified in Annex B. There shall be exactly one weights table in each JPEG XS codestream. This marker shall appear before the first slice header in the codestream and shall follow the picture header. Table A.25 — Weights table syntax Syntax Notes Size Values weights_table() { WGT Lwgt for(b=0;b < NL; b = b+1) { if(b’x[b]) { G[b] P[b] } } } u(16) u(16) 0xff14 Variable Size of the segment in bytes, not including the marker Loop over all bands. b Check whether band exists b b Gain of band Priority of band b u(8) u(8) 0—15 0—255 End of test whether exists. End of loop over bands
## NOTE

Example weights tables for various configurations are given in Annex I. These configurations have been optimized for PSNR performance of the encoder. Other choices are possible and can result in improved visual quality for a certain viewing distance. A.4.13 Refresh Weights table Function: Table A.25 defines the syntax of the Refresh Weights table. b p D This marker segment contains parameters required to set the gain of each band relative to the precinct quantization for wavelet bands that select explicitly intra-prediction for refresh purposes through the TDC selection flags ] in the precinct header, see subclause C.2. Together with the parameters in the precinct header, this allows to determine the quantization of the wavelet coefficients in this band. Details on how to use these parameters are specified in subclause C.6.2. The number of wavelet bands and the relation between band, component and wavelet filter type are specified in Annex B. If this marker is not present, the Usage: inferred values for
, and the inferred values for ] shall be ] shall be ] for all ] for all r[ r[ i[ G G P P b b b b b b [ [
,
. Zero or one Refresh Weights tables shall be present in each JPEG XS codestream. This marker shall appear before the first slice header in the codestream and shall follow the picture header. © ISO/IEC 2024 – All rights reserved

<!-- page 33 -->

Table A.26 — Refresh Weights table syntax Syntax Notes Size Values refresh_weights_table() { WGR Lwgr for(b=0;b < NL; b = b+1) { if(b’x[b]) { Gr[b] Pr[b] } } } Size of the segment in bytes, not including the marker Loop over all bands. Check whether band Refresh-Gain of band b Refresh-Priority of band b b exists b End of test whether exists. End of loop over bands u(16) u(16) 0xff1b Variable u(8) u(8) 0—15 0—255
## NOTE

The Refresh Weights table allows encoders to prioritize refreshed bands differently from bands for which the intra-mode is selected due to large temporal differences; such bands are usually given lower priorities than the the weights in the (regular) Weights table. A.4.14 Slice header Function: Table A.27 defines the syntax of the slice header. This marker segment identifies the start of an intra-only slice and provides sufficient information Usage: for the order of the slices within the image. A codestream contains one or more slice headers. The entropy coded data for one slice follows the slice header and extends either to the next slice header or the end of the codestream. Even though the slice header includes the relative order of the slice in the image, a codestream shall contain slices in incremental order, i.e. progressing from the top of the image to the bottom of the image. This particular slice header identifies slices that only use intra coding tools, i.e. no data from a previous λ frame is required to decode the content of an intra-only slice. For wavelet coefficients within an intra-coded
,
,
, slice, the wavelet coefficients Table A.27 — Slice header syntax ] are set to c’ ]. p p b b λ x x c [ [
,
,
, Syntax slice_header() { SLH Lslh Ysl Isl = 0 } Notes Size Values Size of the segment in bytes, not including the marker Index of the slice regardless of the slice type, counting from line 0 (at the top of the image) downwards (towards the bottom of the image) Indicate that the data in this slice is intra-only coded. u(16) u(16) u(16) 0xff20 0— (2 −1)
## NOTE

A.4.15 TDC enabling slice header Slice indices ease to regain synchronization in case transmission errors corrupted the codestream. Function: Table A.28 defines the syntax of the TDC enabling slice header. This marker segment identifies the start of a TDC slice and provides sufficient information for the order of the slices within the image. © ISO/IEC 2024 – All rights reserved

<!-- page 34 -->

Usage: Similar to the slice header specified in subclause A.4.13, this marker introduces a slice, though all precincts within this slice can reference data from the frame buffer. The entropy coded data for one slice follows the slice header and extends either to the next slice header or the end of the codestream. Even though the slice header includes the relative order of the slice in the image, a codestream shall contain slices in incremental order, i.e. progressing from the top of the image to the bottom of the image. λ p c’ The TDC enabling slice header identifies slices that can reference wavelet coefficients from the frame buffer; a per-band flag recorded in the precinct header, see subclause C.2, indicates for entire bands within this slice whether wavelet coefficients are temporally predicted. For temporally predicted wavelet coefficients, the inverse temporal decorrelation procedure specified in subclause H.4 is applied to reconstruct the wavelet ] decoded through the procedures coefficients ]. Temporal prediction, specified in Annex C. For intra-coded wavelet coefficients, and the selection of the prediction mechanism is specified in Annex H. p c x b λ p
, ] from the wavelet coefficient residuals [ ] is set to
, [ Table A.28 — TDC enabling slice header syntax c’ p b b b λ λ x x x c [ [
,
,
,
,
,
,
,
,
,
, Syntax Notes Size Values tdc_slice_header() { SLI Lsli Ysl Isl = 1 } Size of the segment in bytes, not including the marker Index of the slice regardless of the slice type, counting from line 0 (at the top of the image) downwards (towards the bottom of the image) Indicate that the data in this slice can use temporal prediction. u(16) u(16) u(16) 0xff21 0— (2 −1) © ISO/IEC 2024 – All rights reserved

<!-- page 35 -->

## Annex B

(normative) Image data structures B.1 Dimensions of chroma subsampled image planes N c components aligned along a regular rectangular sampling grid. Each component is An image consists of a rectangular arrangement of integer sample values aligned to the sampling grid of the image. The samples of a component are not required to populate every possible position on the sampling grid. The horizontal spacing between samples of component
. The
. Both, horizontal vertical spacing of component and vertical sampling factors, are signalled in the component table specified in subclause A.4.5. Figure B.1 provides an example of how samples are assigned to positions on the sampling grid. i is denoted by y[ x[ ] and is called the horizontal sampling factor vertical sampling factor ] and is called the is denoted by s s i i i i The number of samples in every row of component is given by W i c [ ] =    W f [ ] i s x    The number of samples in each column of component H H i c [ ] =     s y f [ ] i     i is given by W H f and f are the horizontal and vertical dimensions of the sampling grid of the image as signalled in the picture header. Figure B.1 — Sampling grid illustration © ISO/IEC 2024 – All rights reserved

<!-- page 36 -->

s i s i i In Figure B.1, s s s i s x[0]= y[ x[ ]=2 y[0]=1 is indicated by round dots, ]=1 coresponding to 4:2:2 sampling is
## NOTE 1

indicated by crosses, ]=2 corresponding to 4:2:0 sampling is indicated by boxes and arrows point towards increasing x and y position. Even though this figure indicates the position of the sample values on the sampling grid from which inclusion or exclusion of sample values in image structures such as bands or precincts is derived, the figure cannot be taken as an indication how a full-scale image should be reconstructed from a 4:2:2 or 4:2:0 sampled image or how components are registered relative to each other; such information is either provided by the CRG marker, if present, or can be derived by means beyond this document. In particular, both centred and co-sited positioning of subsampled components are possible. ]=2 y[ x[
## NOTE 2

B.2 Division of the subsampled image plane into bands For 4:4:4 sampling, each sample grid point is populated by sample values of all components. Each component [ ] i
, L x decomposition levels, where s := N
, L x log
, L y
, L y N N N − ′ ′ [ ] i ( y is decomposed by a wavelet transformation with := := N L x ′
,, for i N < c − Sd
, and [ ] i [ ] i N ′ L y
, := for i [ ] i ) N
, N’
, i N’
, i L x[ ] horizontal and L y[ ] vertical i N ≥ c − Sd i i s is given by the maximal That is, the number of horizontal decomposition levels applied to component x and the vertical wavelet decomposition depth depends on the vertical horizontal decomposition level y for all components that participate in the subsampling factor wavelet decomposition and is otherwise 0. For vertically subsampled components, the number of vertical decomposition levels is reduced by 1. ] and the maximal decomposition level y[ N L L
, d Band types are enumerated by two letters indicating the horizontal and vertical filter type, each of which can be either H indicating high-pass filtering or L for low-pass filtering, where the first letter corresponds to the horizontal filter type and the second letter corresponds to the vertical filter type. The filter type is x and the vertical decomposition followed by two subscripts indicating the horizontal decomposition level y. The algorithm of the wavelet transformation itself is specified in Annex E. The filter type, as the level collection of horizontal and vertical filtering and horizontal and vertical decomposition depth, is collapsed into a single number i by a procedure given by subclause B.3. W β β β β d d i i
,  The width ] = [ W β b   i [ ] W i b[ c
, ] of filter type
, ] [ i d β x    at x[
, [ ] W i    c is given by: ] horizontal decompositions of component ] =   
, ]− [ i d β    W b β [ i x
, for a horizontally low-pass filtered band and d horizontally high-pass filtered band. H β β i β i i /    for a  The height
, ] = H β   [ b i [ ] b[ H i ] of filter type
, c
, ] [ d i β y    at y[
, [ ] H i ] vertical decomposition levels of component c
, ]− [ d i β ] = β H [ b i y
,       /  is given by:      for a vertically low-pass filtered band and for a vertically high-pass filtered band. β H In case of 0 vertical wavelet decompositions, a vertical high-pass does not exist and i i H β i b[
, ] is identical to c[ ] for all filter types contributing to component
. B.3 Band indices, horizontal and vertical decomposition levels β N N N i β Bands are the result of the wavelet decomposition of a component type is an index in the range of 0 to with a wavelet filter β enumerates the number of different wavelet filters.
. The wavelet filter β β−1, where © ISO/IEC 2024 – All rights reserved

<!-- page 37 -->

is computed from the number of horizontal decomposition levels max ( N β = as follows: min( ) +
, L y
, L y
, L x
, L x N N N N × + ) ( )
,
, N
, N
, L x and vertical decomposition levels L y d The wavelet filter type y as follows: Set depth N (  if the band is a vertical high-pass, otherwise set   ( ) ) +  β τ is computed from the horizontal decomposition depth x to 1 if the band is a horizontal high-pass, otherwise set if y to 0. Then for > d d
, L x xx y otherwise ) + τ x ( + × d − τ y x≥ τ x
, L y
, L y
, L x β N N N N N − = + − d d τ y x L L
.
,
, y compute τ τ x and vertical decomposition y to 1 x to 0. Similarly, set N
, N
, N
, N
, d d
## NOTE 1

document. For L x< L y interchange L x with L y and x with y. However, this case needs not to be considered in this For 5 horizontal and 0 vertical levels, the above formula results in Table B.1, for 5 horizontal and 1
## NOTE 2

vertical levels, the above formula results in Table B.2; for 5 horizontal and 2 vertical levels, it results in Table B.3. The enumeration of bands in the tables follows the language of subclause B.2. It is important to observe that the does not depend on sampling, i.e. the assignment is identical for 4:4:4, 4:2:2 and assignment of wavelet filter types Sd 4:2:0 sampling. N N N N N β β β s i i
,
,
,
, <
## NOTE 3

N populated and do not carry any data. See also subclause B.4. For components i and ]>1, the wavelet filter types β y[ c− Sd = L x+2− L y and = L x+3− L y are not
## NOTE 4

For components > c−
, only the wavelet filter type =0 is populated, all other filter types do not carry data. Figure B.2 — Wavelet filter types and band indices for 4:2:0 sampling
## NOTE 5

Figure B.2 shows the wavelet filter types in a), the band indices of the luma component in b), the band indices for the Cb component in c) and the band indices for the Cr component in d) for a decomposition with 5 horizontal and 2 vertical levels, 3 components with 4:2:0 sampling and no decomposition suppression. © ISO/IEC 2024 – All rights reserved

<!-- page 38 -->

d β i d β i i β
,
, x[ ] map a component index ] and d y[ The functions i β vertical decomposition level. d d
## NOTE 6

y[0,0]=2, but N Bands are enumerated by a single sequential number The functions y[0,1]=1. N ] and y[ N x[ b β d i
,
, L−1, where N follows:N L = c ( L counts the number of bands. ) Sd N × − Sd + β ] are in general component dependent; in the example from Figure B.2, N N L is computed from
. This sequential number is an index in the range 0 to c as β and the number of components and and a wavelet filter type into a horizontal or N
## NOTE 7

p I If some components are subsampled in the vertical direction or omitted from the wavelet decomposition, L deviates from the total number of wavelet bands accumulated over all components. This is intentional. Wavelet b filter types that are not present in some components are instead excluded by the mechanism of the band-inclusion i flags ] and the band-existence flags N [ x[ β b ]. β b λ s i i
,
, Sd
,
, is computed from the wavelet filter type and the component index in the c− < For ( Sd b N = following way:
, the band index β × + − ) c i i N Sd β For ( b N = c− ≥ c ) Sd N × −
, only + i =0 is populated, all other bands are empty, and the band index β b is given by b By the above convention, iterating over increasing
## NOTE 8

corresponds to a progression order with the component index as fast and the wavelet filter type as slow variable. Components that participate in the wavelet decomposition are transmitted in the first block, followed by all components which are not decomposed. B.4 Band existence flags N b β wavelet filter types are present for all In case the wavelet decomposition is suppressed, not all of the i components, see Figure B.2 for an example. The presence of a band given a filter type and a component is ]. The value of encoded in the array
, and is 0 if it does not exist. The value of this array shall be computed as as follows: ] is 1 if the wavelet filter type or is present in in component x[
, and i N ≥ x[ if Sd β > − β β β b c i i
, b x
, [ β i ] = if max ( N
, L y − d y [ β ]
, ) [ τ β × y ] umod ss iy [ ] ≠ otherwise That is, a band is excluded if either the wavelet filter type is nonzero and the component is not decomposed, b bx β,1 [ or the first line of the wavelet band is not divisible by the vertical subsampling factor. Note that the right ], the first line of the band, defined in subclause B.6.
, hand side of the middle condition is identical to The shaded areas of Figure B.2 corresponds to wavelet filter types b for which ] = L0 b’ β p β [ The band-indexed band-existence flag ′ ( [ )× + = component b N β c x for b x [ ] ]
, exists, and is 0 otherwise. It can be derived from the band existence flags Sd − i N < Sd β − c i i x[b] is 1 if and only if the band
. β b corresponding to filter type x[ ] as follows: i
, and        i ′ b x (  N c − )× Sd N + i  = b x β β, [ i ] for i N ≥ c − Sd © ISO/IEC 2024 – All rights reserved

<!-- page 39 -->

Table B.1 — Wavelet filter types for 0 vertical and 5 horizontal decomposition levels Wavelet filtering and decomposition depths d d x and y) Table B.2 — Wavelet filter types for 1 vertical and 5 horizontal decomposition levels Wavelet filtering and decomposition depths d d x and y) β Wavelet filter type β Wavelet filter type (subscripts are LL5,0 HL5,0 HL4,0 HL3,0 HL2,0 HL1,0 (subscripts are LL5,1 HL5,1 HL4,1 HL3,1 HL2,1 HL1,1 LH1,1 HH1,1 Table B.3 — Wavelet filter types for 2 vertical and 5 horizontal decomposition levels β Wavelet filter type Wavelet filtering and decomposition depths d d x and y) (subscripts are LL5,2 HL5,2 HL4,2 HL3,2 HL2,2 LH2,2 HH2,2 HL1,1 LH1,1 HH1,1
## NOTE

In the above tables, the wavelet filter types are indicated by two capital levels, giving the type of the wavelet filter in horizontal and vertical direction, and two subscripts, counting the number of decompositions that have been applied in horizontal and vertical direction. A letter H indicates high-pass filtering, a letter L low-pass filtering. For all of the above, =0. Sd © ISO/IEC 2024 – All rights reserved

<!-- page 40 -->

B.5 Division of the wavelet-transformed image into precincts The wavelet coefficients are partitioned into a rectangular grid of number of precincts per line of the sampling grid, and column other than the rightmost column is [ ] )× i C w i otherwise W max if
, L x = × × > C C w N ( s p x s f     N
, N
, N
, N
, x is the y is the number of precincts per column. Each y precincts, where x× p p p W Cw s i sample grid positions wide, where ] are signalled in the component table. Each precinct contains coefficients from a rectangular area of coefficients from all bands. These coefficients, in turn, correspond to a rectangular region of image data. are signalled in the picture header, and the Cw f and x[
## NOTE

the rightmost column is approximately of the same size as that of all other columns. While not a requirement specified in this document, it is generally advisable to select N N
,
, in such a way that p W The number  f p x
,  C  N = s W N
, N
, and x of precincts per line and the number H  f  N
, L y  p y
, N =       p y of precincts per column are defined as follows: f is the width of the sampling grid, where sampling grid positions and the picture header specified in subclause A.4.3. p L N
, H C H f is the height of the sampling grid, f, p precinct indices W y the number of vertical decomposition levels. N
, s is the column width in y are signalled in f and
, N N L
, Precincts are assigned sequential numbers with a raster scan manner, i.e. firstly from left to right, and secondly from top to bottom. y−1 y defined as above. The sequential number enumerates the precincts on the sampling grid in p runs from 0 to
, denoted as NL y = 2 ,
, where x and x× p p p p W pp [ ] Hp Precinct number is latter is computed as [ ] = − W p p   (( W f C s umod ) C s ) + N p x
, otherwise sampling grid lines high and p x
, umod N if p < sampling grid columns wide, where the − i i i β b x β Denote by (xb,yb) the coefficient positions within band H β W H as defined in subclause B.3. Then and component p x b[ b[ ] the band dimensions as defined in subclause B.2. A sample in band ] and
, ]−1, with ] d b) is part of precinct b,   [ ]× i s y × [ ]×  i   =  N   C L y
, s     is the band corresponding to filter type b runs from 0 to at
, position (    
, where b runs from 0 to if and only if and p ]−1 and
## 2 β,

[ d i x b[
, [ i β umod         W
, p x
, p x b[ N N = × β p β b b y y x s bb b y x i i ] y
,
, d d β i β.
, i β N
, ] are the horizontal respectively vertical decomposition depth of component
, i y are the number of horizontal and vertical decomposition levels, and L ] and x[ s i i x[ where filter type are the horizontal and vertical sampling factors of component y[ x and N
, ] and L i s and y[ ] W p b b p
. pb [ W p b ] =
,     [ ] W p p [ d
## 2 β

x ] [ ] i     s x [ ]× i The width pb[
, ] of band for a horizontally [ W p b in precinct 
,  pb   ] =     is computed by [ ]  W p p  [ d β x   [ ]× i ]− [ ] i x /     s low-pass filtered band and p b W for a horizontally high-pass filtered band. By this definition, index magnitudes per line in precinct
, pb[ ] indicates the number of wavelet coefficients and also the number of quantization p b and band
. © ISO/IEC 2024 – All rights reserved

<!-- page 41 -->

B.6 Division of precincts into lines By the conditions in subclause B.4, each precinct includes coefficients from at most  wavelet coefficients. The line index within a precinct varies between 0 and the precinct height
, Hp  λ∈ − p−1: λ H NL y p = 2 , H lines of b p p is included in lines in precinct Band
,
, ) ( ] [ d i N β in precinct (exclusive) end line of band y L y
, [ ] = [ L p b τ β ⋅ y max b ] −
, λ L ≥ 0[
. and L p p b b p λ b p L < L1[ ] and ], where 1[ ] are computed as follows: L
,
,
, ] and b 0[
,
## 0 is the start line and

L
## 1 the

[ L p b
, ] = [ L p b
, ] + min ( H b
, [ β i ] − p N p x
,         max × ( N L y
, − [ d i y
, ββ ]
, ) max ( N L y
, − [ d i y
, β ]
, ) )
, β i b where image is divided into precincts, lines, bands and packets. are computed from and as indicated in subclause B.3. Figure B.3 provides an example how an By this definition, line numbers enumerate lines in the sampling grid, irrespectively of the vertical
## NOTE 1

subsampling factor of a component. For vertically subsampled components, odd line numbers are excluded by means of the line inclusion flags, see subclause B.7.
## NOTE 2

bands in all other precincts. In particular, some of these bands can be even empty. By these formulae, the bottommost precinct of a image can contain bands that contain fewer lines than the © ISO/IEC 2024 – All rights reserved

<!-- page 42 -->

Figure B.3 — Precincts, slices and packets
## NOTE 3

In Figure B.3, a 5 level horizontal and 2 level vertical decomposition using 4:4:4 or 4:2:2 sampling with a slice-height of 16 is presented. Medium lines indicate precinct boundaries, thin lines band boundaries and thick lines the image boundary. Dotted lines belong to a separate slice. The precinct denoted by a) depicts the band indices for all bands that contribute to component 0, the precinct indicated by b) depicts the packet indices. The shaded area in b) consists of a single packet. Note that some bands extend over two lines. Subpackets can extend over several bands, but include only a single line of a group of bands. Band indices for component 0 only are shown for precinct =1, the grouping of lines of bands into packets is demonstrated for precinct =3. Columns are disabled and precincts extend over the full image. Note further that the presence of a band in the precinct for the last precinct of the image cannot be easily inferred from Figure B.3. B.7 Grouping of lines and bands into packets p p p N s N is encoded in Each precinct p Each packet contains entropy coded data of one or multiple bands s and precinct s p in packet p precinct precinct b pc−1. of band b is included and
, and zero if it is not. As indicated by Table A.1, the line inclusion flags for s is encoded in the line inclusion flags is included in packet are computed at the start of this precinct. pc packets enumerated by the packet index b
. All bands within a packet are coded jointly. Whether line p b
, but only of one single line ]. This flag is non-zero in case line
, which runs from 0 to in precinct of band of band N p p b b λ λ λ λ λ s [ I I
,
,
, The algorithm In Table B.4 computes the line inclusion flags precinct: [
,
,
, ] and the number of packets pc per © ISO/IEC 2024 – All rights reserved

<!-- page 43 -->

Input: Output: bands precinct
. p N Horizontal and vertical decomposition depth p
, ] and p 1[ 0[ p L L b b λ s I
, β, line start and end positions p Line inclusion flags [
,
, N
, N b L ] for all bands, precinct index x and b y, number of components L λ s
. p N c, number of N p
,
,
, band Table B.4 — Computation of the line inclusion flags ] per precinct and packet
, line
, number of packets pc[ ] for Name Notes compute_packet_inclusion(p) { for(b=0;b<NL;b=b+1) { for(λ=0;λ< 2 NL,y ;λ=λ+1) { for(s=0;s<NL× 2 I[p,λ,b,s]=0 NL,y ;s=s+1) { } } } s=0 β1=max(NL,x,NL,y)−min(NL,x,NL,y)+1 for(β=0;β<β1;β=β+1) { for(i=0;i<Nc−Sd;i=i+1) { if(bx[β,i]) { b=(Nc−Sd)×β+i I[p,0,b,s]=1 } } } for(β0=β1;β0<Nβ;β0=β0+3) { N for(λ=0;λ< 2 for(β=β0;β<β0+3;β=β+1) { L,y β[
- d 0, y ]
;λ=λ+1) { r=1 for(i=0;i<Nc−Sd;i=i+1) { if(bx[β,i]) { b=(Nc−Sd)×β+i if((λ + L0[p,b]) umod sy[i] == 0) { if(λ+L0[p,b]<L1[p,b]) { s=s+r I[p,λ+L0[p,b],b,s]=1 r=0 } } } Iterate over all band indices Iterate over all lines Iterate over all possible packet indices Reset packet inclusion flag End of loop over packet indices End of loop over lines End of loop over bands Reset packet index Number of included bands in the first packet Loop over filter types Loop over wavelet-decomposed components β Check whether the band corresponding to filter type and component i exists Compute the band index from the filter type and the β1 component, see subclause B.3 bands of all components included in the first packet End of loop over wavelet-decomposed components End of loop over wavelet filter types Loop over proxy levels until all wavelet types are covered Loop over all lines within the band Loop over all filter types within the resolution level Indicate to create a new packet Loop over components β Check whether the band corresponding to filter type and component i exists Compute the band index from the filter type and the component, see subclause B.3 Check whether the band is excluded due to 4:2:0 subsampling Check whether the line is in the precinct Potentially start a new packet Include the line in the precinct Packet has been created for this band type End check whether the line is included End check whether subsampling allows line inclusion End check whether band exists © ISO/IEC 2024 – All rights reserved

<!-- page 44 -->

Table B.4 (continued) Table B.4 (continued) Name Notes } } } } for(λ=0;λ< 2 for(i=Nc−Sd;i<Nc;i=i+1) { NL,y ;λ=λ+1) { b=(Nc−Sd)×Nβ+i if(λ+L0[p,b]<L1[p,b]) { s=s+1 I[p,λ+L0[p,b],b,s]=1 } } } Npc[p]=s+1 } Sd End of loop over components End of loop over filter types End of loop over lines End of loop over proxy levels Loop over lines Loop over components that do not participate in the wavelet decomposition Compute the band from the component. The filter type is always 0 Check whether the line is in the precinct Create a new packet Include the line in the precinct End loop over components End loop over lines Define the number of packets in total For
## NOTE

=0, the above algorithm results for 3 components, 5 horizontal and 0 vertical wavelet decomposition and 4:4:4 or 4:2:2 sampling in the line inclusion flags as listed in Table B.5, for 3 components, 5 horizontal and 1 vertical wavelet decomposition in Table B.6 and for 3 components, 5 horizontal and 2 vertical wavelet decomposition in Table B.7. For 3 components, 5 horizontal levels and 4:2:0 sampling, Table B.8 lists the lines and included bands >0, the components that do not for 1 vertical decomposition level, Table B.9 for 2 vertical decomposition levels. For participate in the wavelet decomposition follow the regular components, with the component as fast and the line as slow variable. For =1, 4 components and 4:4:4:4 sampling and 1 vertical decomposition level, the above algorithm results in the packet layout indicated in Table B.10, for the same configuration and 2 vertical decomposition levels in Table B.11. Table B.5 — Line inclusion flags for zero vertical decomposition level and 4:4:4 or 4:2:2 sampling and Sd=0 Sd Sd s Packet index λ Line number Table B.6 — Line inclusion flags for one vertical decomposition level and 4:4:4 or 4:2:2 sampling and Sd=0 Included bands (0,1,2) (3,4,5) (6,7,8) (9,10,11) (12,13,14), (15,16,17) s λ Packet index Line number Included bands (0,1,2) (3,4,5) (6,7,8) (9,10,11) (12,13,14) (15,16,17) (18,19,20) (21,22,23) © ISO/IEC 2024 – All rights reserved

<!-- page 45 -->

Table B.7 — Line inclusion flags for two vertical decomposition levels and 4:4:4 or 4:2:2 sampling and Sd=0 s λ Packet index Line number Included bands (0,1,2) (3,4,5) (6,7,8) (9,10,11) (12,13,14) (15,16,17) (18,19,20) (21,22,23) (24,25,26) (27,28,29) (21,22,23) (24,25,26) Table B.8 — Line inclusion flags for one vertical decomposition level and 4:2:0 sampling and Sd=0 (27,28,29) s λ λ Packet index Line number of luma Line number of chro-
Included bands component ma components
- 
(0,1,2) (3,4,5) (6,7,8) (9,10,11),(12,13,14) (15,16,17) (18) Table B.9 — Line inclusion flags for two vertical decomposition levels and 4:2:0 sampling and Sd=0 (21) s λ λ Packet index Line number of luma Line number of chro-
Included bands component ma components
- 
- 
- 
- 
(0,1,2) (3,4,5) (6,7,8) (9,10,11) (12,13,14) (15) (18) (21,22,23) (24,25,26) (27,28,29) (21) (24)
- 
- 
Table B.10 — Line inclusion flags for one vertical decomposition level and 4:4:4:4 sampling with Sd=1 (27) s λ Packet index Line number Included bands (0,1,2) (3,4,5) (6,7,8) (9,10,11) (12,13,14) (15,16,17) (18,19,20) (21,22,23) (24) (24) © ISO/IEC 2024 – All rights reserved

<!-- page 46 -->

Table B.11 — Line inclusion flags for two vertical decomposition levels and 4:4:4:4 sampling and Sd=1 s λ Packet index Line number Included bands (0,1,2) (3,4,5) (6,7,8) (9,10,11) (12,13,14) (15,16,17) (18,19,20) (21,22,23) (24,25,26) (27,28,29) (21,22,23) (24,25,26) (27,28,29) (30) (30) (30) (30) B.8 Division of precinct lines into code groups λ p b N are grouped into code groups for the purpose of in precinct Consecutive coefficients of line g and is constant throughout joined coding. The number of coefficients within one code group is denoted by all bands and precincts. The first sample of the first code group in a line of a precinct corresponds to the first coefficient of that line. In case the width of the line is not a multiple of the code group size, the last code g samples. An encoder can output arbitrary values for these samples. A decoder group is padded to include N shall ignore samples resulted from padding in all subsequent steps such as the wavelet transformation. and band N p p b b [ , ] cg[ W p b N The number of code groups g [ , ] N p b / pb cg = 
,  ] of precinct and band is computed as follows: W p b p b N
, pb[ where coefficients. B.9 Grouping of code groups into significance groups ] is the width of precinct and band in coefficients and g is the size of a code group in S λ p b If significance coding is enabled, multiple code groups are furthermore grouped into significance groups. A significance group comprises
. The first code λ group of the first significance group corresponds to the first code group of the line
. The last significance may cover only a smaller number of code groups. A significance group is significant if at group of a line least one code group within the significance group contains at least one non-zero coefficient or one code b group has a non-zero bitplane count prediction residual, depending on the selection of the run-mode s consecutive code groups of a line λ and band in precinct Rm N λ
. S The number of significance groups s [ , ] W p b [ , ] N p b / ( pb N × g s =  ) s[p,b] in band  in every line is computed as follows: W p,b b p N ] is the width of the band in precinct in coefficients, g is the number of coefficients in a S where code group and pb[ s is the size of a significance group expressed in number of code groups. © ISO/IEC 2024 – All rights reserved

<!-- page 47 -->

B.10 Grouping of code groups into TDC selection groups SLI λ S
, see subclause A.4.15), multiple If TDC is enabled within a slice due to the TDC enabling slice header ( i code groups are furthermore grouped into TDC selection groups. A TDC selection group comprises λ
. The first code group of the first TDC selection and band consecutive code groups of a line group corresponds to the first code group of a line may cover only a smaller number of code groups. The TDC subpacket specified in subclause C.5.6 decodes one flag per TDC selection group. This flag selects per TDC selection group for all wavelet coefficients within this group whether the inverse temporal decorrelation transformation specified in subclause H.4 is applied to them, or whether ], i.e. no decorrelation is performed. For details, see Annex H.
. The last TDC selection group of a line in precinct ] is set to N c’ p p p p b b b b b λ λ λ λ x x c [ [
,
,
,
,
,
, )  The number of TDC selection groups pb [ , ] W p b [ , ] N p b =  / ( N × S g i i i[
, ] in band in every line is computed as follows: W p b b p N S pb[
, where code group and B.11 Grouping of precincts into slices ] is the width of the band i is the size of a TDC selection group expressed in number of code groups. in precinct in coefficients, g is the number of coefficients in a One or multiple precincts are grouped into slices. Restrictions on bitplane count decoding ensures that vertical prediction is disabled across slice boundaries; this ensures that wavelet coefficients that are part of  H different slices can be decoded independently of each other. Slice number f  H  [ ] = N t N consist of
, if (t+1)× umod H H                
, p x H H × > sl sl p p p t f         Hsl otherwise H N
, sl is signalled in the picture x is the number of precincts p H H precincts, such that the first slice is aligned to the top of the image, where p is the height of a precinct in lines and header, per row. f is the height of the image, © ISO/IEC 2024 – All rights reserved

<!-- page 48 -->

## Annex C

(normative) Entropy decoding C.1 Entropy decoding general provisions Encoded image data is structured in slices, see subclause B.11, where each slice includes the wavelet coefficients necessary to reconstruct a horizontal stripe of the image. Slices are represented in the codestream by slice headers and subsequent precincts, see subclause A.4.13 for the syntax of a slice header. Data following the slice header represents one or multiple precincts, where precincts are included in raster scan order, left to right, top to bottom. Precincts are not enclosed in markers. Each precinct consists of a precinct header, one or multiple packets, and optional filler bytes, see Table A.1 for details. Subclause C.2 specifies the structure of the precinct header. p s of a precinct consists of a packet header and a packet body which itself includes multiple Each packet subpackets. Subclause C.3 specifies the structure of the packet header, and subclause C.4 the structure of p,λ,b,x] v the packet body. Each subpacket contributes directly or indirectly to the quantization index magnitudes [ ]. Some subpackets are optional; their existence is indicated
, by flags in the precinct header or picture header, depending on the type of the subpacket. and wavelet coefficient signs p b λ x s [
,
,
## NOTE 1

This document does not define a mechanism to resynchronize the decoder to marker or packet boundaries. The entropy coded data can contain byte sequences that reassemble markers or marker segments. A lower-level transport protocol beyond the scope of this document is needed to ensure proper resynchronization to frame or slice boundaries. I D The TDC selection subpacket includes for each TDC selection group a single bit that, if it is set, indicates that all code groups within the corresponding TDC selection group are temporally predicted, i.e. they encode residuals relative to the contents of the frame buffer. The TDC selection subpacket is an optional subpacket that shall be included in precincts if
. That is, the TDC selection subpacket is 0xff21
, marker type present if the slice coding mode indicates that the precinct is part of a TDC enabling slice ( indicate flagsl=0 that is, for precincts that are part of an intra-only slice ] recorded in the precinct header SLH based TDC selection. It shall not be present for p ( is different from 3 for all bands
. The TDC selection subpacket is specified in subclause C.5.6.
, see subclause A.4.15) and the TDC mode of the precinct
, see subclause A.4.13) or if the TDC mode ]=3 for any band p
, marker type and a band b p of precinct of precinct sl=1 and SLI p 0xff20 i[ I D i[ p b b b b
,
, The significance subpacket includes for each significance group a single bit that, if set, indicates that all code groups within the corresponding significance groups are insignificant. A code group is insignificant if contains only zero coefficients, or its bitplane count prediction residual is zero, depending on the Run Mode flag Rm in the picture header. The significance subpacket is an optional packet that is only included if bit 1 of the bitplane count coding mode ] field in the precinct header is non-zero and the raw mode override flag ] field in the packet header is 0. The significance subpacket is defined in subclause C.5.2.
, b λ p r[ M D D T p p p g g b b s [
, The bitplane count subpacket defines for all code groups in significant significance groups the bitplane count ] specifies the number of bitplanes included for all coefficients within a code group. The bitplane count subpacket is a mandatory packet that is always present. It is specified in subclause C.5.3.
. The bitplane count together with the truncation position ] of a code group p b λ v x [ [
,
,
,
, Fs The data subpacket defines the quantization index magnitudes x b count is larger than the truncation position. If the
, contains the signs ] for all code groups whose bitplane flag of the picture header is 0, the data subpacket also p ]. The data subpacket is defined in subclause C.5.4. p p b b λ λ λ v x x s s [ [
,
,
,
,
, Fs ] of
, The sign subpacket defines for all non-zero quantization index magnitudes this quantization index. The sign subpacket is an optional packet that exists only if the flag in the picture header is non-zero. If the sign subpacket does not exist, signs are included in the data subpacket. The sign subpacket is defined in subclause C.5.5. ] the sign [ [
,
,
,
,
, © ISO/IEC 2024 – All rights reserved

<!-- page 49 -->

The bitplane count, data and sign subpackets may contain an arbitrary number of filler bytes at their end. A decoder can infer the number of filler bytes from the corresponding length field in the packet header. The value of the filler bytes shall be ignored by a decoder. The entropy coded data segment does not use markers to indicate the presence or absence of particular packet
## NOTE 2

types. Instead, the picture header includes all necessary information to decide upon the presence of a particular packet. C.2 Syntax of the precinct L p A precinct is represented in the codestream by a precinct header, one or multiple packets, and – optionally – filler bytes following the entropy coded data. The amount of filler bytes following the precinct can be ] field in the precinct header. Decoders shall ignore the filler bytes and skip over it, inferred from the without interpreting the data stored there. Table C.1 specifies the syntax of the precinct header. Table C.2 Input: specifies the encoding of the bitplane count coding modes. prc[ p Output Precinct index p R of the precinct whose header is to be decoded. L p D p b : Size of the precinct in bytes including filler bytes prc[ refinement [ ] and bitplane count coding modes ] and TDC modes
, Table C.1 — Precinct header syntax [ p D Q b ], precinct quantization ] for all bands i[ b
,
. p [ ], precinct Name Notes Size Values precinct_header(p) { Lprc[p] Q[p] R[p] if(Isl > 0) { Qf[p] Rf[p] } else { Qf[p]=32 Rf[p]=0 } Length of the entropy coded data in this precinct including filler bytes measured in bytes. The number of bytes in this field counts from the end of the precinct header of this precinct up to, but not including the first byte of the next precinct header, slice header or EOC. Precinct quantization. This field is input to the algorithm specified in subclause C.6.2 to select the truncation positions this precinct. ] of all bands of T p b [
, Precinct refinement. This field is input to the T algorithm specified in subclause C.6.2 to select the truncation position this precinct. ] of all bands of p b [
, Is the precinct part of a TDC enabling slice? Precinct frame buffer quantization level. This field is input to the inverse temporal prediction algorithm specified in subclause H.4 to select a quantization value for the data in the frame buffer. Precinct frame buffer refinement level. This field is input to the inverse temporal prediction algorithm specified in subclause H.4 to select a refinement value for the data in the frame buffer. End of TDC enabling slice syntax Initialize the frame buffer quantization to maximum for intra-coding Initialize the frame buffer refinement for intra coding. u(24) 1—(2 −1) u(8) 0—31 N u(8) 0—(2 L−1) u(8) 0—31 N u(8) 0—(2 L−1) for(b=0;b<NL;b=b+1) { if(b'x[b]) { Loop over all bands b Check whether band exists © ISO/IEC 2024 – All rights reserved

<!-- page 50 -->

Table C.1 (continued) Table C.1 (continued) Name Notes Size Values b Bitplane count coding mode of band
. u(2) See Table C.2 D[p,b] } } if(Isl > 0) { End of loop over bands for(b=0;b<NL;b=b+1) { if(b'x[b]) { Di[p,b] } } } padding } D p b For TDC enabling slices, signal the TDC modes of all bands b Loop over all bands b Check whether band TDC mode of band
. exists u(2) See Table C.3 End of loop over all bands End of TDC enabling slice syntax Pad to next byte boundary pad(8)
, [ s p ] field consists of two consecutive bits per band in the precinct header. It specifies how the bitplane The counts of the code groups of wavelet coefficients are encoded. Table C.2 lists valid encodings for this field in binary, where an “x” indicates a bit position whose value shall be ignored for the purpose of determining D D ] fields can be overridden by the a specific function. The bitplane count encoding mode selected by the b ] is non-zero, bitplane counts in the corresponding s p ] field. The same subpacket are encoded in raw mode, see subclause C.6.4, regardless of the value of the number of ] fields and regardless whether ] fields shall be present, regardless of the values of the b D some bands are not included at all because the last precinct is partially cut off at the bottom of the sampling grid. ] field in the subpacket header, see Table C.4. If [ D r[ r[ r[ D D D p p p p p b b s [ [
,
,
,
,
,
,
, [ The ] flags shall be populated in such a way that vertical prediction is never selected for the precinct at the top of a slice; this condition ensures that wavelet coefficients in different slices can be entropy decoded independently of each other. Table C.2 — Bitplane count coding modes D[p,b] Bitplane count coding mode x0 x1 0x 1x prediction from zero Vertical prediction Significance coding disabled Significance coding enabled
## NOTE 1

vertical or no prediction is selected. The above table indicates that bit #1 indicates the presence of significance coding and bit #0 whether b D p i[
, c’ The ] field shall be only present in precincts following TDC enabling slice headers. It consists of two consecutive bits per band in the precinct header that specify how to reconstruct the temporally predicted c wavelet coefficients ] and wavelet coefficient [ residuals ] from the contents of the frame buffer ]. Table C.3 lists valid encodings for this field in binary. p p p b b b λ λ λ x x x [ [ f
,
,
,
,
,
,
,
,
, © ISO/IEC 2024 – All rights reserved

<!-- page 51 -->

Table C.3 — TDC modes Di[p,b] TDC mode b p are transmitted directly No temporal prediction, all wavelet coefficients of band and inverse temporal prediction is not applied to them, using the regular Weights table to b determine the truncation position. in precinct b except for those in TDC selection h[ ] and marker, see Annex A.4.10 ] are signalled in the temporal prediction control marker ( k p Y Temporal prediction for all coefficients of band S groups whose positions match the hash through h[ for details). h[ b λ b TPC ], see subclause H.4. Y b Y p S b in precinct ] and h[ S b b Y Temporal prediction selection through TDC selection flags ] transmitted in the TDC subpacket. Wavelet coefficients in the TDC selection groups whose positions match the hash through ] are not temporally predicted (see subclause H.4), regardless of the ] and value of their TDC selection flag h[ h[ ]. Y p p b b k λ [ [
,
,
,
,
,
, No temporal prediction, all wavelet coefficients of band are transmitted directly and inverse temporal prediction is not applied to them, using the Refresh Weights table for determining the truncation position. in precinct C.3 Packet header p s b λ p
, s s s p p D r[ D p and one line of the precinct consists of one or multiple packets, where each packet Data following the precinct header of precinct represents the quantization indices of one or multiple bands and all bands within this packet, see Table A.1 for a breakdown of the syntax. A packet consists of a packet header and one or multiple subpackets. Table C.4 specifies the syntax of the packet header. Subpackets are specified in p subclause C.5. p The p ] flag in the packet header indicates whether the bitplane count information of packet [ in precinct ] mode selection in the precinct header. Regardless of the value of ] flags shall be present in the precinct header. Packets encoded in the raw D mode do not include significance information and the significance subpacket shall not be present for packets whose D D is encoded in raw. By that, [ ] field is non-zero. D In addition to the above, the following constraint shall hold if the given precinct D precinct same precinct. Formally: for all precincts line indices field of the picture header is 0: For a that include band ] flag shall be identical for all packets within p D p s
, i.e. raw and non-raw coding of bitplane counts shall not be mixed within the same band in the and λ’ and all packets
, Rl ] if there is a band ]=1. This restriction does not apply if ] overrides the
, the λ p I and band such that ] = 1 and r[ s b ], the b r[
, and and is 1. ] == Rl r[ r[ r[ r[ D λ’ s’ s’ s’ p p p p p p b b b b b λ
, s s s s s [ [ I
,
,
,
,
,
,
,
,
,
,
,
, Figure C.1 — A valid selection of raw mode override flags for Rl=0 © ISO/IEC 2024 – All rights reserved

<!-- page 52 -->

Rl=0 Figure C.1 demonstrates a valid composition of raw-mode override flags for 5 horizontal and 2 vertical
## NOTE 1

decomposition levels and two precincts for
. Thick lines indicate precinct boundaries, thin lines band boundaries, dotted lines packet boundaries. The shaded region to the top left is represented by a single packet. Note that raw-mode flags can vary between bands and precincts, but are identical within the same band. Not all raw mode override flags are shown. Input: Table C.4 specifies the syntax of the packet header. s p p D s and packet index Output: precinct index s p L Raw mode override flag p s p L ], length in bytes of the bitplane count subpacket
, ] of precinct
, dat[ sgn[ coefficients in the bands included in packet ] for precinct and packet r[ s
, L p and packet s and is not signalled. Table C.4 — Syntax of the packet header
, length in bytes of the data subpacket ], and length in bytes of the sign subpacket
. The length of the significance packet is inferred from the number of cnt[ s
, s of the packet whose packet header is to be decoded. p Name Notes Size Values packet_header(p,s) { ph=0 for(b=0;b<NL;b=b+1) { for(λ=L0[p,b];λ<L1[p,b];λ=λ+1) { if(I[p,λ,b,s]) { ph=1 } } } if(ph==1) { Dr[p,s] if(Wf×Nc<32752 && Lh==0) { Ldat[p,s] Lcnt[p,s] Lsgn[p,s] } else { Ldat[p,s] Lcnt[p,s] Lsgn[p,s] Assume the packet header is not present Loop over all bands Loop over all lines of this band Include only if the line is present in the given band and packet, see subclause B.7. Include the packet header End of line is included End of loop over lines End of loop over bands Only include data if the packet is non-empty Raw mode override flag. If this bit is non-zero, bitplane count information of this packet is encoded in raw mode, regardless of the value of the ] flags in the precinct header. D p b [
, Depending on the width of the image and the number of components, select the syntax of the header. See Table A.7 for the definition of c and Lh W f, N
. Size of the data subpacket in bytes. Size of the bitplane count subpacket in bytes Fs Size of the sign subpacket in bytes if Fs Fs =1. If =0, this field shall be present, but is ignored. picture header, see Table A.7. is specified in the End of short packet header Size of the data subpacket in bytes. Size of the bitplane count subpacket in bytes Fs Size of the sign subpacket in bytes if Fs Fs u(1) 0,1 u(15) u(13) 0–32767 0–8191 u(11) 0–2047 u(20) u(20) 0–1048575 0–1048575 u(15) 0–32767 =1. If =0, this field shall be present, but is ignored. picture header, see Table A.7. is specified in the © ISO/IEC 2024 – All rights reserved

<!-- page 53 -->

Table C.4 (continued) Table C.4 (continued) Name Notes Size Values } } } End of condition for packet header size End of test for non-empty packet In case a component is not vertically subsampled or excluded from the wavelet transformation by means
## NOTE 2

of the CWD marker, the data subpackets of such a component can grow larger than 32768 bytes or the sign subpacket can grow larger than 2048 bytes, even if =1 if the codestream contains components that are not vertically decomposed or do not participate in the wavelet filtering process, and if C.4 Packet body c < 32752. It is thus advisable to enforce long headers with c≥16376. Lh W W f× f× N N p s. s λ b p in Table C.5 specifies the syntax of the packet body precinct x b b g p M consists of multiple subpackets, each of which contributes directly or indirectly to the bitplane precinct λ counts
, ]
,
,
, Input: of a single line ], the quantization index magnitudes s and one or multiple bands ] or wavelet coefficient residual signs The packet body of packet λ [ of precinct and packet x λ p p
, p p b v s [ [
,
,
,
,
. Output: λ and packet index M p b Precinct index x b s p Bitplane counts
, λ [
,
,
, [ ] for precinct
,
,
. λ v g s p ], quantization index magnitudes [
. in subpacket
, line Table C.5 — Syntax of the packet body
, and all bands b residual signs p λ b x
,
,
, ] and wavelet coefficient Name Notes Reference packet_body(p,s) { if(Isl > 0) { unpack_tdc_flags(p,s) } unpack_significance(p,s) unpack_bitplane_count(p,s) unpack_data(p,s) if(Fs==1) { unpack_signs(p,s) } } Check whether this packet is part of a TDC enabling slice Subclause A.4.13 and subclause A.4.15 Subclause C.5.6 Table C.6 Table C.9 Table C.10 Decode TDC selection flags. The size of the TDC subpacket is not included in the packet header and can be inferred from the size of the included D lines, bands and the TDC mode p b i[
, ] flags. End of decision for TDC. Decode significance information. The size of the significance subpacket is not included in the packet header and can be inferred from the size of the line and the band. L p s Decode bitplane count information. This subpacket includes bytes. L Decode wavelet magnitude data. This subpacket includes cnt[ s ] bytes. dat[ ] p
,
, Fs The sign subpacket is only included if sign coding is enabled in the picture header, see Table A.7 for the definition of L Decode wavelet magnitude data. This subpacket includes ] bytes. sgn[ p s
,
. End of if sign packing enabled © ISO/IEC 2024 – All rights reserved

<!-- page 54 -->

C.5 Subpackets C.5.1 Nomenclature The entropy coded data segment following the packet header is transmitted in multiple
. Each subpacket contains data of a specific type that is relevant to one line but one or multiple bands of the precinct indicated by the packet header. Depending on configuration, not all subpackets may be present. C.5.2 Significance subpacket subpackets Table C.6 specifies the syntax of the significance subpacket. This subpacket includes for every significance group of code groups one bit that identifies whether all code groups in the significance group are insignificant. The bitplane count subpacket does not include information for insignificant significance groups and the bitplane counts of the code groups within such significance groups are inferred. This subpacket is optional. D ] field of the precinct header is set to 1 and the raw mode override flag It is only included if bit #1 of the D p p b s [
, r[
, ] is set to 0. See subclause C.3 for the specification of the precinct header.
## NOTE

p Input: the included bands. s The packet header does not include the size of the significance subpacket, it can be inferred from the size of Output: Precinct s Z and packet p whose significance data is to be decoded b λ j p packet Significance flags
. [
,
,
, ] of all significance groups and all bands of the given precinct and Table C.6 — Syntax of the significance subpacket Name Semantics Size Values unpack_significance(p,s) { for(b=0;b<NL;b=b+1) { for(λ=L0[p,b];λ<L1[p,b];λ=λ+1) { if(I[p,λ,b,s]) { if(Dr[p,s] == 0) { if(D[p,b] & 2) { for(j=0;j<Ns[p,b];j=j+1) { Z[p,λ,b,j] } } } } } } padding } Loop over all bands Loop over all lines of this band Include only if the line is present in the given band and packet, see subclause B.7. Include only if the raw override flag is not set D p b Significance information is only present if indicated by bit #1 of precinct header is set. ] in the [
, b p Loop over all significance groups of this N precinct, band and line. A definition of ] is given in subclause B.9. s[
, Significance information of this significance group u(1) 0,1 End of loop over significance groups End of significance coding enabled End of raw override is not set End of line included End of loop over lines End of loop over bands Pad to the next byte boundary pad(8) © ISO/IEC 2024 – All rights reserved

<!-- page 55 -->

C.5.3 Bitplane count subpacket C.5.3.1 Purpose of the bitplane count subpacket D p b s p
, s p p D D r[
. The The bitplane count subpacket decodes to the bitplane counts of the code groups of a packet ] of the precinct header and the raw [ syntax of the packet depends on the bitplane count coding mode p D b ] signalled in the packet header. Additional constraints apply to the selection of the mode override flag ]. Which constraints apply depend bitplane count coding mode =0, the on the raw-mode selection per packet flag constraints indicated in subclause C.5.3.2 apply. In case =1, the constraints indicated in subclause C.5.3.3 apply. Subclause C.5.3.4 specifies an algorithm that tests the correctness of the mode selection, and codestreams shall be constructed in such a way that this algorithm succeeds. Subclause C.5.3.5 specifies the syntax and the decoding algorithm for the bitplane count subpacket.
, of the picture header, see subclause A.4.4. In case ] and the raw mode override flag of precinct Rl Rl Rl r[ s [
,
,
## NOTE

The purpose of these constraints are to ensure an upper bound for the buffer size a decoder has to reserve for entropy-coded bitplane count data. An encoder can always satisfy the constraints by selecting the raw mode for the bitplane count coding mode if the size of the bitplane count subpacket becomes too large. C.5.3.2 Bitplane count mode selection for Rl=0 Rl b b =0, the codestream shall be constructed in such a way that for all bands In case bitplane count subpackets and significance subpackets contributing to sizes of encoding the bitplane count of the same subpackets in the raw mode.
, the sum of the sizes of all is at most as large as the sum of all s p L s p ] be the size of the bitplane count subpacket of precinct p and packet s ] be the size of the significance subpacket of precinct and packet in bytes defined
, or 0 if no p L cnt[ Formally: Let in Table C.4. Let significance data is included.
, sig[ p L s s
, sig[
, While ] )× by the number of bytes generated by the algorithm specified in Table C.6, or equivalently by ] is not explicitly signalled, it can be inferred from the size of the bands contributing to ∑ λ ] >>    [ N p b [ D p s
, [ I p b [ D p s
, p s     ] =
, λ ]× sig − × L ] [ ( s ( ) b r s
,
,
,
,
, L ′tot ]
, [ p b p Let precinct L tot ′ :
, p b [ ] := be the sum of sizes of all bitplane count and significance subpackets contributing to band N
, − pc [ ]− L p b ∑ ∑ = [ L p b λ = s
, [ , I p b ]
,
, ] s λ × ( L cnt
, p s [[ ] + L sig
, p s [ ] ) raw , [ L p b tot ] Let N raw coding mode: [ raw L tot
, p b := ] be the amount of bytes required to encode the bitplane count of band
, [ I p b
, p b
, λ ]× cg pc N [ − − ′ ′
, ll
, Br [ ]− L p b ∑ ∑ = [ L p b λ = s
,
, [ I p b ]
,
, λ s ]×    ∑ b ′ = ]× s N    b p in precinct in the Then, the codestream shall be constructed in such a way that for all and
, holds. b p L ′tot
, p b [ ] ≤ raw L tot
, p b ] [ © ISO/IEC 2024 – All rights reserved s
, either b in

<!-- page 56 -->

Figure C.2 — Rate constraints for band and packet mode selections p s p Figure C.2 demonstrates which comparisons are made to test the validity of a mode decision for a single
## NOTE

precinct with a 5 level horizontal, 2 level vertical wavelet decomposition. Thick lines represent precinct boundaries, thin lines band boundaries and dotted lines packet boundaries. The shaded area to the top left is encoded in a single p packet. The overall rate of the bitplane count subpackets in the shaded areas is computed once for all packets covering D ] as given (top row) and once in raw mode [ the area encoded with the bitplane count coding mode D ]=1 (bottom row). Areas over which the rate is summed are defined in such a way that the raw mode flags ] and raw mode are consistent with the condition specified in subclause C.3. The bitplane count coding modes override flags ] in a codestream are populated in such a way that the rate in any of the shaded areas is not larger D than the rate of the same area in the bottom row. A trivial, but suboptimal way to satisfy this constraint would be to select C.5.3.3 Bitplane count mode selection for Rl=1 ]=1 for all packets. ] and r[ r[ r[ r[ D D D p p p p b b s s s [
,
,
,
,
,
, Rl ′ [ L l
, ] Formally: Let p s tot s p Let l
, and packet p s tot L ′ [ =1, the bitstream shall be constructed in such a way that that for all bands In case of the sizes of all bitplane count subpackets and significance subpackets contributing to as large as the sum of all sizes of encoding the bitplane count of the same subpackets in the raw mode. p p L L
, the sum is at most s s n c t[
, ] and sig[
, ] be defined as in subclause C.5.3.2. be the sum of sizes of all bitplane count and significance subpackets contributing to precinct ] : :=
, p s
, p s L cnt L sig ] + [ [ ] b λ b λ and all lines in line l raw, L tot
, p s ] [ p s Let N raw coding mode: [ :=
, l raw L tot
, p s ] l [ ]− L p b ∑ ∑ = [ L p b λ = b
, ]    ]× s N    be the amount of bytes required to encode the bitplane count of precinct
,,b Br ]×
, [ I p b
, λ cg p [ −
,
, and packet in the Then, the codestream shall be constructed in such a way that for all and
, holds. p l L ′ tot s
, p s [ ] ≤
, l raw L tot
, p s ] [ © ISO/IEC 2024 – All rights reserved

<!-- page 57 -->

C.5.3.4 Validation algorithm for bitplane count and raw mode override selection b p Table C.7 specifies an algorithm that checks the validity of the encoding of a precinct p by checking the Input above condition for all bands in p : Output : Precinct index valid : An indicator that is 1 in case the precinct mode selection is valid, or 0 in case it is invalid. Table C.7 — Testing the validity of a precinct encoding Name Notes is_encoding_valid(p){ valid=1 if(Rl==0) { for(b=0;b<NL;b=b+1) { rawsize=0 bytesize=0 for(s=0;s<Npc;s=s+1) { for(λ=L0[p,b];λ<L1[p,b];λ=λ+1) { if(I[p,λ,b,s]) { bytesize = bytesize + Lcnt[p,s] bytesize = bytesize + Lsig[p,s] for(b’=0;b’<NL;b’= b’+1) { if(I[p,λ,b’,s]) { rawsize = rawsize + Br×Ncg[p,b’] } } } } } if(bytesize > rawsize/8 ) { ⎾ ⏋ valid=0 } } } else { for(s=0;s<Npc;s=s+1) { rawsize = 0 bytesize = Lcnt[p,s] + Lsig[p,s] for(b=0;b<NL;b=b+1) { for(λ=L0[p,b];λ<L1[p,b];λ=λ+1) { if(I[p,λ,b,s]) { Assume validity Check for packet-based raw-mode switch Loop over all bands of the precinct Number of bits required to encode this band in raw mode Number of bytes Loop over all packets in the precinct Loop over all lines in the band s b Include only if the band et
, line and precinct, see subclause B.7. is present in the given pack-
Include number of bytes required for bitplane count coding Include number of bytes required for significance coding s Loop over all bands that contribute to the same packet band b is part of b’ Only if band Br is included in the same subpacket p b’ N Reserve each line in this precinct. b’ subclause B.8. bits per included bitplane count for cg[ ] is specified in
, s End of band is included in subpacket b’ End of loop over bands End of if line included End of loop over lines End of loop over subpackets Check buffer limit condition Invalid if buffer size constraint violated End of check whether bitrate constraint is satisfied End of loop over bands End of band-based raw-mode switch, start of packet-based switch. Loop over all packets in the precinct Include number of bytes required for bitplane count coding and significance coding Loop over all bands of the precinct Loop over all lines in the band Only if band b is included in the subpacket © ISO/IEC 2024 – All rights reserved

<!-- page 58 -->

Table C.7 (continued) Table C.7 (continued) rawsize = rawsize + Br×Ncg[p, b] Br Name Notes N p b } } } if(bytesize> rawsize/8 ) { valid=0 ⎾ ⏋ } } } } C.5.3.5 Bitplane count subpacket syntax Reserve each line in this precinct. subclause B.8. bits per included bitplane count for cg[ ] is specified in
, End of check for band inclusion End of loop over lines End of loop over bands Check buffer limit condition Invalid if buffer size constraint violated End of loop over all packets End of line-based raw-mode switch D p b D p s Table C.8 specifies the syntax of the bitplane count subpacket depending on reference to the corresponding subclauses. [
, ] and r[
, ] and gives s L p
## NOTE

Input: of the packet header. The number of filler bytes at the end of the bitplane count subpacket can be inferred from the x b p Z λ cnt[
, ] field precinct and packet whose bitplane counts are to be decoded, significance flags [
,
,
, ] of the line Output: and precinct if significance coding is enabled. M p g b λ p s Bitplane counts [ Table C.8 — Syntax of the bitplane count subpacket ] in all bands of the precinct and packet
,
,
,
. Name Notes Size Reference unpack_bitplane_count(p,s){ for(b=0;b<NL;b=b+1) { for(λ=L0[p,b];λ<L1[p,b];λ=λ+1) { Loop over all bands if(I[p,λ,b,s]) { if(Dr[p,s]==1) { unpack_raw(p,b,λ); } else if((D[p,b] & 1) == 0) { unpack_nopred(p,b,λ); } else { unpack_vertical(p,b,λ); Loop over all lines of this band Include only if the line is present in the given band and packet, see subclause B.7. Detect whether raw coding is enabled for this packet Decode with the raw coding mode Variable Subclause C.6.4 Select the bitplane count coding mode No prediction with or without sig-flags Variable Subclause C.6.6 } } } } Padding filler bytes } Variable Subclause C.6.5 Vertical prediction with or without sigflags End of bitplane count coding mode selection End of if line is present in subpacket and band End of loop over lines End of loop over bands Pad to the next byte boundary Arbitrary number of filler bytes pad(8) fill() © ISO/IEC 2024 – All rights reserved

<!-- page 59 -->

C.5.4 Data subpacket Table C.9 specifies the syntax of the data subpacket. This subpacket includes the coefficient data of all significant code groups of a given precinct and line within a precinct. It also requires the bitplane count of each code group and the truncation position of each band. p L s
## NOTE

Input: packet header. The number of filler bytes at the end of the data subpacket can be inferred from the dat[
, ] field of the precinct and packet whose coefficient data is to be decoded, bitplane counts of all code groups of all bands of the given line and precinct, truncation positions of all lines and bands of the given precinct and line. The truncation positions ] are computed from the information in the precinct header specified in subclause C.2 and the weights table specified in subclause A.4.6 according to the algorithm given in Output: subclause C.6.2. T p p p b b v x s [
, s Quantization index magnitudes [
,λ,
, packing is disabled, additionally wavelet coefficient signs packet
. Table C.9 — Syntax of the data subpacket s p x b ] in all bands of the precinct
,
,λ, [ and packet ] of all bands in the given precinct p
, and if sign and Name Notes Size Values unpack_data(p,s) { for(b=0;b<NL;b=b+1) { for(λ=L0[p,b];λ<L1[p,b];λ=λ+1) { if(I[p,λ,b,s]) { for(g=0;g<Ncg[p,b];g=g+1) { for(k=0;k<Ng;k=k+1) { v[p,λ,b,Ng×g+k] = 0 } if(M[p,λ,b,g]>T[p,b]) { if(Fs == 0) { for(k=0;k<Ng;k=k+1) { s[p,λ,b,Ng×g+k] } } for(i=M[p,λ,b,g]−T[p,b]−1; i≥0;i=i–1) { for(k=0;k<Ng;k=k+1) { Loop over all bands of the precinct Loop over all lines of this band Include only if the line is present in the given band and packet, see subclause B.7 Include data for all groups in this precinct and line. Iterate over all members of the code group Reset quantization index magnitude End of loop over code groups Include only signs if a non-zero number of bitplanes is included Check whether the sign subpacket is disabled Loop over all members of the N code group. The definition of g is specified the group size in subclause B.8 Sign bit of the coefficient in the current band, line and group End of loop over coefficients End of sign inclusion Loop over all bit positions Loop over all members of the code group u(1) 0,1 © ISO/IEC 2024 – All rights reserved

<!-- page 60 -->

Table C.9 (continued) Table C.9 (continued) Name Notes Size Values d v[p,λ,b,Ng×g+k]=v[p,λ,b,Ng×g+k]+(d<<i) } } } } } } } padding filler bytes } C.5.5 Sign subpacket u(1) 0,1 pad(8) fill() Binary data of the quantization index magnitude in the precinct, line, band and group Set the corresponding bit in the quantization index magnitude End of loop over code group members End of loop over bitplanes End of non-zero number of bitplanes included End of loop over code groups End of line and band included in subpacket End of loop over lines End of loop over bands Pad to the next byte boundary Arbitrary number of filler bytes Table C.10 specifies the syntax of the sign subpacket. This subpacket includes the sign information of all coefficients of all code groups of a given precinct and line within a precinct. This subpacket shall only be present if the sign packing flag specified in subclause A.4.3 is set to 1. Fs p L s
## NOTE 1

Input: packet header. The number of filler bytes at the end of the sign subpacket can be inferred from the sgn[
, ] field of the precinct and packet whose sign data is to be decoded, decoded coefficient magnitudes of the precinct Output: and subpacket. s p λ b x array of signs [
,
,
, ] of all bands in the given precinct and packet, coefficient array of all coefficients in the precinct and line.Table C.10 — Syntax of the sign subpacket Name Semantics Size Values unpack_signs(p,s) { for(b=0;b<NL;b=b+1) { for(λ=L0[p,b];λ<L1[p,b];λ=λ+1) { if(I[p,λ,b,s]) { for(g=0;g<Ncg[p,b];g=g+1) { for(k=0;k<Ng;k=k+1) { if(v[p,λ,b,Ng×g+k]!=0) { s[p,λ,b,Ng×g+k] Loop over all bands Loop over all lines of this band Include only if the band is present in the given line and packet, see subclause B.7. N Include data for all groups in this precinct and line. See B.8 for a definition of cg. Iterate over all members of the code group. Only include sign information if the quantization index magnitude is non-zero u(1) Sign bit of non-zero quantization index magnitude 0,1 © ISO/IEC 2024 – All rights reserved

<!-- page 61 -->

Table C.10 (continued) Table C.10 (continued) Name Semantics Size Values } } } } } } Padding filler bytes } End of non-zero code group End of loop over coefficients End of loop over groups End of line and band included in subpacket End of loop over lines End of loop over bands Pad to the next byte boundary Arbitrary number of filler bytes pad(8) fill()
## NOTE 2

As the data subpacket always transmits coefficients in groups of 4, it can happen that it includes meaningless coefficients near the right edge of a wavelet band. By the above table, the sign subpacket includes sign bits even for such meaningless coefficients whenever they are non-zero. It is advisable, though not necessary, to force such meaningless coefficients to 0 at the encoder side. C.5.6 TDC subpacket Table C.6 specifies the syntax of the TDC subpacket. This subpacket includes for every TDC selection group of code groups one bit that identifies whether all coefficient groups in the TDC selection group are temporally predicted. A set bit indicates TDC such that decoded wavelet coefficient residuals can be prediction corrections relative to the content of the frame buffer, see Annex H, in particular subclause H.4. sl>0, i.e. if the packet is part of a TDC enabling slice. See This subpacket is optional. It is only included if subclause A.4.13 and A.4.15 for the specification of the slice headers. I b D p b p
## NOTE

Input: included bands The packet header does not include the size of the TDC subpacket, it can be inferred from the size of the
. s ] flags of the bands of precinct p
, and the i[
, Output: Precinct s Y and packet p whose TDC selection flags are to be decoded. b k λ packet TDC selection flags
. [
,
, ] of all TDC selection groups and all bands of the given precinct
, Table C.11 — Syntax of the TDC subpacket p and Name Semantics Size Values unpack_tdc_flags(p,s) { for(b=0;b<NL;b=b+1) { if(Di[p,b]==3) { for(λ=L0[p,b];λ<L1[p,b];λ=λ+1) { if(I[p,λ,b,s]) { for(k=0;k<Ni[p,b];k=k+1) { Y[p,λ,b,k] } } } } Loop over all bands Include TDC selection flags only if enabled in the precinct header, see subclause C.2. Loop over all lines of this band Include only if the line is present in the given band and packet, see subclause B.7. b p N Loop over all TDC selection groups of this precinct, band and line. A definition of ] is given in subclause B.10. i[
, TDC mode of this TDC selection group u(1) 0,1 End of loop over TDC selection groups End of line included End of loop over lines End of TDC selection flags enabled © ISO/IEC 2024 – All rights reserved

<!-- page 62 -->

Table C.11 (continued) Table C.11 (continued) Name Semantics Size Values } padding } End of loop over bands Pad to the next byte boundary pad(8) C.6 Bitplane count decoding C.6.1 Bitplane count decoding general provisions M p λ b g p,b The bitplane count decoding process decodes the bitplane counts ] from the contents of the Bitplane D D count subpacket by a process specified in subclause C.5.3 that depends on the bitplane count coding mode ] in the packet header. The decoding process may optionally predict bitplane counts vertically and can optionally employ significance flags to skip over insignificant code groups. ] in the precinct header and the raw mode override flag [
, p,s p,b r[ T [
,
, The bitplane count decoding process requires furthermore the truncation position ], which defines the bitplane at which transmission of coefficient data stops, and hence indirectly determines the quantization step size. The computation of the truncation position is specified in subclause C.6.2. p,b T M Vertical prediction modes also require access to the bitplane counts ] and truncation position T top[ ] ] is specified in subclause C.6.3. The codestream shall be constructed in such a way that vertical and prediction is never selected as bitplane count coding mode for the topmost lines of the topmost precinct of a slice or the image. ] of the line directly above the current line within the same band. The computation of p,λ,b,g p,λ,b,g top[ top[ top[ M p b [
,
## NOTE

independently of each other. C.6.2 Computation of the truncation position The above requirement ensures that wavelet coefficients within different slices can be decoded T p b b p r, Table C.12 specifies the computation of the truncation position precinct quantization q and precinct refinement subclause C.2, and the band priority [ Refresh Weights table specified in subclause A.4.13. The truncation position inverse prediction processes as well as for the inverse quantization defined in subclause D.1. from the ] of band both specified in the precinct header specified in p ] defined by the Weights table specified in subclause A.4.12 or the ] is required for multiple and precinct b T T P p b b [ [
,
, For the purpose of Annexes C and D, the truncation position [
, ] shall be computed as follows: b Q p R p D p b T p b [
, ]=compute_truncation( p Q
, [ ], [ ], i[
, ]) R b p D and [ where b is the band index,
, ] the refinement of precinct i[ decoded from the precinct header as specified in subclause C.2, and ] is the TDC mode decoded from b p sl = 0 indicating a slice that does not use TDC and thus the precinct header as specified in subclause C.2. If Input the precinct header does not signal ] is the quantization of precinct D ] shall be 0. D i[ p p p p b b r [ I
,
,
, b G G : Band index r[ p i[ b P
, precinct quantization
, precinct index b [ ] and band priorities ], the inferred value of P ], r[ ], ]. p q b b Output: gains [
, precinct refinement q r and TDC mode d, band d Truncation position of band in precinct with quantization and refinement in TDC mode
.. © ISO/IEC 2024 – All rights reserved

<!-- page 63 -->

Table C.12 — Computation of the truncation position Syntax Notes compute_truncation(b,q,r,d) { if(d!=1) { p=P[b] g=G[b] } else { p=Pr[b] g=Gr[b] } if(p<r) { s = 1 } else { s = 0 Check whether the regular tables are to be used Use priority from regular Weights table Use gain from regular Weights table Use priority from Refresh Weights table Use gain from Refresh Weights table Compare the priority as specified in the selected weights table with the refinement threshold r An additional bitplane is included for bands with priorities below the refinement threshold } return clamp(q−g−s,0,2Br−1) No refinement otherwise } Compute the truncation position as the precinct quantization minus the band gain from the selected Weights Table, minus the number of additional refinement bitplanes, then clamp to the valid range. C.6.3 Computation of the vertical bitplane count predictor and truncation position predictor Vertical prediction modes require an entire row of bitplane counts above the current line as the source of the prediction. In addition, the truncation position of the line above is also required. The codestream shall be constructed in such a way that vertical prediction is not selected at the first line of a slice, and hence in particular not at the top of the image. M p p g b b λ b p top[ The process specified in Table C.13 computes from a given precinct L1 L0 vertical predictor first line
, Input: subclause B.6. N s b ] and the vertical truncation predictor
, ] and the last line L0 Precinct index and precinct b p, p p p b b [
,
,
, i x, first line [ M ] of the component the band
, line index ] and last line p p Output: y[ b
,
, b p ] of band [ λ L1
, band index [ b λ is part of.
, g ] of band p b
, bitplane counts of precinct g where λ p M s [
. Let λ λ g b p T and band
, line L1 b p L0 top[
,
,
, ] and the corresponding b p ]. For that, it requires the ] are specified in p p [
,
, [ g b
,
, ] of precinct –
, y be the vertical subsampling factor and precinct p λ b T Vertical predictors p b top[
,
,
, ] of all code groups of precinct
, line and band
; truncation position predictor top[
, ]
## NOTE

algorithm as specified here always accesses bitplane counts within the current slice and within the image. Vertical prediction cannot be selected at the start of the slice, and hence at the top of the image. Hence, the © ISO/IEC 2024 – All rights reserved

<!-- page 64 -->

Table C.13 — Computation of the vertical bitplane count predictor Syntax compute_predictor(p,b,λ) { for(g=0;g<Ncg[b];g=g+1) { if(λ–sy<L0[p,b]) { Mtop[p,λ,b,g]=M[p–Np,x,L1[p,b]–sy,b,g] } else { Mtop[p,λ,b,g]=M[p,λ–sy,b,g] } } if(λ–sy<L0[p,b]) { Ttop[p,b]=T[p–Np,x,b] } else { Ttop[p,b]=T[p,b] } } Notes Loop over all code groups of this band Check whether the given line is the first line of the band in the precinct λ Predict from the last line of the precinct above if line of the band in the precinct λ is the top Predict from the line above precinct if this line is still in the same End of loop over all code groups Check whether the given line is the first line of the band in the precinct λ Predict from the precinct above if band in the current precinct is the top line of the Predict from the line above if still in the precinct, see subclause C.6.2 C.6.4 Bitplane count decoding for the raw mode D p s λ b g p The syntax of the bitplane count subpacket specified in this subclause is selected if ] in the packet M header is 1, indicating the raw mode. The bitplane count subpacket contains in this case bitplane counts bits per code group. Table C.14 specifies the decoding of the bitplane counts in ] directly, using Br
, r[
,
,
, [ Input: the raw mode. p, b, λ Output: precinct index λ p M band b and line index g whose bitplane counts are to be decoded. p, b g λ Bitplane counts [
,
,
, ] for all other code groups Table C.14 — Raw mode of precinct band and line
. Name Notes Size Values unpack_raw(p,b,λ) { for(g=0;g<Ncg[p,b];g=g+1) { M[p,λ,b,g] } } N cg is specified in subclause B.8. Include predicted bitplane counts for all code groups. Bitplane count encoded in Br bits End of loop over code groups Br Br u( ) 0–(2
- 1) C.6.5 Differential bitplane count decoding for vertical prediction D p b
,
, [ s p r[ The syntax of the bitplane count subpacket specified in this subclause is selected if the bitplane count D ] in the precinct header indicates vertical prediction and the raw mode override flag coding mode ] in the packet header is 0. Table C.15 specifies how to decode bitplane counts in the vertical mode. The bitplane count subpacket contains in this case bitplane count prediction residuals which are used to p M recover the bitplane counts from the residuals by inverse vertical prediction. The codestream shall always be constructed in such a way that the bitplane counts ] are between 0 and (2
- 1). Br g b λ [
,
,
, © ISO/IEC 2024 – All rights reserved

<!-- page 65 -->

Input: p b λ λ p precinct index g b whose magnitude data is to be decoded, significance Z information of the precinct and line whose magnitude information is to be decoded. Significance flags g Output:
,
, ] in case significance coding is enabled by the bitplane count coding mode and line index
, band M λ
, D ]. p p p g b b b λ [ [
, Bitplane counts [
,
,
, ] for all code groups of precinct Table C.15 — Vertical mode
, band and line
. Name Notes Size Values unpack_vertical(p,b,λ) { compute_predictor(p,b,λ) for(g=0;g<Ncg[p,b];g=g+1) { t = max(T[p,b],Ttop[p,b]) mtop = max(Mtop[p,λ,b,g],t)
if((D[p,b] & 2)==0 || Z[p,λ,b, ]==0) { g/Ss ⎿ ⏌ Δm=vlc(mtop,T[p,b]) } else { if(Rm == 0) { Δm=0 } else { Δm=T[p,b]−mtop } } M[p,λ,b,g] = mtop+Δm } } Compute the prediction values, see subclause C.6.3 N Include predicted bitplane counts for all code groups. cg is specified in subclause B.8. Compute effective truncation position for prediction Compute predictor Decode prediction residual only if either significance information was not included, or the corresponding significance group was signalled as significant Decode prediction residual encoded with variable length code Test the run mode Non-significant groups have a zero bitplane count prediction residual. T p b m T p b vlc( top, [
, ]) Non-significant groups have a bitplane count of ]. [
, End of run mode selection m End of test on significance of code group Predict from top End of loop over code groups Br 0–(2
- 1) C.6.6 Variable length bitplane count decoding without prediction D p s D p b ] in the precinct
, The syntax of the bitplane count subpacket specified in this subclause is selected if header indicates no prediction and the raw mode override flag ] in the packet header is 0. Table C.16 specifies how to decode bitplane counts in the no-prediction mode. The bitplane count subpacket contains p in this case variable length encoded bitplane counts. The codestream shall always be constructed in such a Input: way that the bitplane counts λ ] are between 0 and (2 −1). b
, M p, r[ Br g b λ [ [
,
,
, λ p precinct index g b whose magnitude data is to be decoded, significance Z information of the precinct and line whose magnitude information is to be decoded. Significance flags g Output:
,
, ] in case significance coding is enabled by the bitplane count coding mode and line index band M λ
, D ]. p p p g b b b λ [ [
, Bitplane counts [
,
,
, ] for all code groups of precinct
, band and line
. © ISO/IEC 2024 – All rights reserved

<!-- page 66 -->

Table C.16 — No-prediction mode Name Notes Size Values unpack_nopred(p,b,λ) { for(g=0;g<Ncg[p,b];g=g+1) { mtop = T[p,b]
if((D[p,b] & 2)==0 || Z[p,λ,b, ]==0) { g/Ss ⎿ ⏌ Δm=vlc(mtop,T[p,b]) } else { Δm=0 } m = mtop+Δm M[p,λ,b,g] = m } } N Include predicted bitplane counts for all code groups. cg is specified in subclause B.8. Set predictor to the truncation position Decode prediction residual only if either significance information was not included, or the corresponding significance group was signalled as significant Decode prediction residual encoded with variable length code m T p b vlc( top, [
, ]) b p T Non-significant groups have bitplane count [
, ]. m End of significance included Predict from top End of loop over code groups End of test on significance of code group Br 0–(2
- 1) C.7 Elementary variable length coding and decoding primitives C.7.1 Variable length decoding primitive r t Table C.17 specifies the variable length decoder vlc() primitive which decodes a signed quantity in the
. This coding primitive is used throughout this annex. context (r,t) of a predictor r indicates the A codestream shall not contain more than 2 number of bits in raw coding as indicated in the picture header, see subclause A.4.4 for details. Detecting such a condition indicates that the decoder has lost synchronization with the source. This establishes an Input: error condition whose handling is beyond the scope of this document. 1-bits as input for the vlc decoder, where and a truncation position Br+1 B B t, r Output: A predictor and a truncation position
. the number of bits to encode a bitplane count in raw mode r. a signed quantity Table C.17 — Decoding a signed quantity with vlc Syntax Semantics Size Values vlc(r,t) { θ=max(r−t,0) n=0 do { b if(b) { n=n+1 } } while(b && n < 2Br+1) if(n ≥ 2Br+1) { error() Compute the threshold for the alphabet switch Reset the bitcounter Read the next bit from the codestream u(1) 0,1 Count the number of 1-bits Repeat as long as 1-bits are found in the stream B A codestream shall not contain more than 32 consecutive B 1-bits for r=5. for r=4 and no more than 64 consecutive 1-bits © ISO/IEC 2024 – All rights reserved

<!-- page 67 -->

Table C.17 (continued) Table C.17 (continued) Semantics Size Values Check whether this is the unary sub-alphabet If so, decode unary subalphabet Check for non-zero symbol, signed sub-alphabet Check for an odd codeword Return a negative value for odd codewords Return a positive value for an even codeword Return zero for a zero codeword Syntax } if(n > 2×θ) { return n−θ } else if(n > 0) { if(n & 1) { return −  } else { n / 2  return n / 2   } } else { return 0 } } C.7.2 Variable length encoding primitive x r t Table C.18 provides guidance on the implementation of an algorithm that inverts the vlc() decoder primitive Input: and encodes a signed quantity in the context of a predictor and a truncation position x r t
. Output: A predictor
, a truncation position x and a signed quantity t to be encoded. r a sequence of bits encoding given the context consisting of Table C.18 — Encoding a signed quantity with vlc and Syntax vlc_encode(x,r,t) { θ = max(r−t,0) if(x > θ) { n = x + θ } else { x = x × 2 if(x < 0) { n = -x – 1 } else { n = x } } for(i=0;i<n;i=i+1) { out(1) } out(0) } } Semantics r t Encode x in the context of and Compute the threshold for the alphabet switch Check for the unary sub-alphabet case Compute the number of 1-bits to write in the unary case Instead in the binary sub-alphabet case Reserve two bits per symbol in the binary sub-alphabet Encode negative numbers with an odd number of bits Encode positive numbers with an even number of bits End of binary alphabet coding n Write out a sequence of 1-bits Write out a single 1-bit End of loop writing 1-bits Write out 0 as the comma bit © ISO/IEC 2024 – All rights reserved

<!-- page 68 -->

## Annex D

(normative) Quantization D.1 General x Inverse quantization computes the wavelet coefficient residuals from the decoded quantization index magnitudes and positions M quantization is controlled by the truncation position bitplane count position shall be computed according to subclause C.6.2. Qpih This document offers multiple inverse quantization processes, the selection of which is controlled by the x b λ p x b λ p
, bands ] in all precincts
,
,
, ]. Inverse ] and their signs b T ] which depends on the precinct and band, and the ] ] of the code group the quantized wavelet coefficient is part of. The truncation p λ x b λ p s
, lines
,
,
, [ c v [ [ T p p p p g b b λ [ [ [
,
,
,
,
,
,
,
, elements of the picture header, see subclause A.4.3 for details. c x λ b x p In case the codestream does not use TDC and thus does not depend on a frame buffer, the wavelet coefficient [ ] are identical to wavelet coefficients ]. For details, see Annex H.2. c’ p b λ [
,
,
,
,
,
,
## NOTE

residuals D.2 Inverse deadzone quantization Qpih Table D.1 specifies the inverse deadzone quantization process. The inverse deadzone quantizer is selected if the element of the picture header is 0. The zero bucket of the deadzone quantizer is twice the size of all regular buckets, and the reconstruction point of this quantizer is in the middle of each bucket. The Input: quantization bucket size is given by the truncation position ] of the precinct and band T T p p p b b b b λ [
,
, band index
, truncation positions [
, ] of the precinct, line and band and quantization index magnitudes
. v ] of this precinct and band, ] and p b λ x [
,
,
, λ Precinct index λ p
, bitplane counts Output: their signs
, M p x b [
, ]. s [
,
, p b g
, line index
, c p λ b x Wavelet coefficient residuals [
,
,
, ] Table D.1 — Inverse deadzone quantization Syntax Notes deadzone_dequant(p,λ,b) { for(x=0;x<Wpb[p,b];x=x+1) { g = x/Ng ⎿ ⏌ if(M[p,λ,b,g]>T[p,b] && v[p,λ,b,x] != 0) { r = (1 << T[p,b]) >> 1 σ = 1–2s[p,λ,b,x] c[p,λ,b,x] = σ×((v[p,λ,b,x]<< T[p,b]) + r) } else { c[p,λ,b,x] = 0 b p Iterate over all wavelet coefficient residuals of band in precinct Compute the code group index from the wavelet coefficient residual position Check whether a non-zero number of bitplanes is included in this code group and whether the wavelet coefficient residual is non-zero Compute the reconstruction point Compute the sign of the reconstructed wavelet coefficient residual Reconstruct the wavelet coefficient residual No bitplanes included or wavelet coefficient residual is zero Set to zero © ISO/IEC 2024 – All rights reserved

<!-- page 69 -->

Table D.1 (continued) Table D.1 (continued) Syntax Notes } } } D.3 Inverse uniform quantization End of test for sufficient bitplanes and non-zero wavelet coefficient residual End of loop Qpih Table D.2 specifies the inverse uniform quantization process. The inverse uniform quantizer is selected if element of the picture header is 1. The uniform quantizer uses all equally-sized buckets whose size the ]. Compared to the inverse deadzone quantizer, the inverse is determined from the truncation position b Input: uniform quantizer requires an additional scaling step. T T p p b b λ [
,
, band index
, truncation positions [
, ] of the precinct, line and band and quantization index magnitudes v ] of this precinct and band, ] and p b λ x [
,
,
, λ Precinct index λ p bitplane counts
, Output:
, their signs M p x b [
, ]. s [
,
, p b g
, line index
, c p λ b x Wavelet coefficient residuals [
,
,
, ] ∆= + M + − T − M x
## NOTE

v The bucket size of the uniform inverse quantizer is given by σ p λ procedure as given by this subclause is identical to the multiplication of × [
, x b x
, −
,
. The reconstruction x k − = ] with Δ within the limits of the k ∞∑
## 1 1

− = / x = implementation precision. This can be seen from the Neumann series ∆
. While multiplication with can also be carried out explicitly, readers should be aware that a single-precision floating point implementation of the above formula will typically generate results different from the algorithm in the following table and is hence not acceptable. Table D.2 — Inverse uniform quantization Syntax Notes uniform_dequant(p,λ,b) { for(x=0;x<Wpb[p,b];x=x+1) { g = x/Ng ⎿ ⏌ if(M[p,λ,b,g] > T[p,b] && v[p,λ,b,x] != 0) { σ = 1–2s[p,λ,b,x] φ = v[p,λ,b,x]<<T[p,b] ζ = M[p,λ,b,g] – T[p,b] + 1 for(ρ = 0; φ > 0; φ = φ >> ζ) { ρ = ρ + φ } c[p,λ,b,x] = σ × ρ } else { c[p,λ,b,x] = 0 b p Iterate over all wavelet coefficient residuals of band in precinct Compute the code group index from the wavelet coefficient residual position Check whether a non-zero number of bitplanes is included and whether the wavelet coefficient residual is non-zero Compute the sign of the reconstructed wavelet coefficient residual Get zero-order approximation Extract the scale value Sum over the Neumann series Sum up partial terms Insert the sign and reconstruct the wavelet coefficient residual No bitplanes included or wavelet coefficient residual is zero Set to zero © ISO/IEC 2024 – All rights reserved

<!-- page 70 -->

Table D.2 (continued) Table D.2 (continued) Syntax Notes } } } D.4 Deadzone quantization End of test for sufficient bitplanes and non-zero wavelet coefficient residual End of loop Table D.3 provides guidance on the implementation of a deadzone quantizer whose output is compatible Input: with the normative inverse deadzone quantization procedure specified in subclause D.2. T p b b λ p b λ Precinct index
, M p [
, Output: bitplane counts g
, line index
, ] of the precinct, line and band and wavelet coefficient residuals
, band index p b λ v x
, truncation positions x p b λ s [
, c b p λ ] of this precinct and band,
,
, ]. x [
, Quantization index magnitudes
, Table D.3 — Deadzone quantization ] and their signs [ [
,
,
,
,
, ] Notes Syntax deadzone_quant(p,λ,b) { for(x=0;x<Wpb[p,b];x=x+1) { if(c[p,λ,b,x] < 0) { s[p,λ,b,x] = 1 v[p,λ,b,x] = (-c[p,λ,b,x])>>T[p,b] } else { s[p,λ,b,x] = 0 v[p,λ,b,x] = c[p,λ,b,x]>>T[p,b] } } } D.5 Uniform quantization b p Iterate over all wavelet coefficient residuals of band precinct in Test for the sign of the wavelet coefficient residual Wavelet coefficient residual is negative Compute the amplitude from the wavelet coefficient residual Wavelet coefficient residual is positive Compute the amplitude from the wavelet coefficient residual End of the sign check of the wavelet coefficient residual End of loop Table D.4 provides guidance on the implementation of a uniform quantizer whose output is compatible with Input: the normative inverse uniform quantization procedure specified in subclause D.3. T p b b λ p b λ Precinct index
, M p [
, Output: bitplane counts g
, line index
, ] of the precinct, line and band and wavelet coefficient residuals
, band index p b λ v x
, truncation positions λ p s [ b
, x c b p λ ] of this precinct and band,
,
, ]. x [
, Quantization index magnitudes [
,
,
, ] and and their signs [
,
,
, ]
## NOTE

The procedure given here is equivalent to mid-point quantization of a scalar quantizer with ∆= M + M + − T − bucket size
. © ISO/IEC 2024 – All rights reserved

<!-- page 71 -->

Table D.4 — Uniform quantization Syntax Notes uniform_quant(p,λ,b) { for(x=0;x<Wpb[p,b];x=x+1) { g = x/Ng ⎿ ⏌ if(M[p,λ,b,g] > T[p,b]) { Ζ = M[p,λ,b,g] – T[p,b] + 1 if(c[p,λ,b,x] < 0) { s[p,λ,b,x] = 1 d = -c[p,λ,b,x] } else { s[p,λ,b,x] = 0 d = c[p,λ,b,x] } v[p,λ,b,x] = ((d << ζ) – d + (1 << M[p,λ,b,g])) >> (M[p,λ,b,g]+1) } else { s[p,b,λ,x] = 0 v[p,b,λ,x] = 0 } } } b p Iterate over all wavelet coefficient residuals of band precinct in Compute the code group index from the wavelet coefficient residual position Does the wavelet coefficient residual contain sufficient bitplanes? Extract the scale value Test for the sign of the wavelet coefficient residual Wavelet coefficient residual is negative Compute the amplitude from the wavelet coefficient residual Wavelet coefficient residual is positive Compute the amplitude from the wavelet coefficient residual End of sign check Quantize and round to nearest No bitplanes included for this wavelet coefficient residual Quantize to zero End of check for number of included bitplanes End of loop D.6 Bitplane count computation p λ b g M p λ b g M p λ b g
,
,
, Table D.5 provides guidance on the computation of the bitplane counts residuals c[ ]. The bitplane counts λ Input: are encoded in the bitplane count subpacket of the precinct b λ c
, line index Precinct index g λ p Output: wavelet coefficient residuals
, [ x
, band index p
,
, truncation positions
, b M b
, ]. p p b λ [
,
,
. T p b ] from the wavelet coefficient ] are input to the uniform or deadzone quantization and p [
,
,
, [
, ] of this precinct and band and ] of precinct
, Table D.5 — Bitplane count computation and band
, line
. Bitplane counts [
,
, Syntax compute_bitplane_counts(p,λ,b) { for(g=0;g<Ncg[b];g=g+1) { vmax=0 for(k=0;k<Ng;k=k+1) { x = Ng×g+k if(x<Wpb[p,b]) { if(c[p,λ,b,x]<0) { if(-c[p,λ,b,x]>vmax) { vmax = -c[p,λ,b,x] Notes b Iterate over all code groups of the band Set the maximum of the wavelet coefficient residual amplitude to zero Iterate over all members of the code group Compute position of the quantized wavelet coefficient residual Test whether the position is within the band Test for the sign of the wavelet coefficient residual Check for a new maximum Update the maximum of the wavelet coefficient residual amplitude © ISO/IEC 2024 – All rights reserved

<!-- page 72 -->

Table D.5 (continued) Table D.5 (continued) Syntax Notes } } else { if(c[p,λ,b,x]>vmax) { vmax = c[p,λ,b,x] } } } } for(m=0;vmax>0;vmax = vmax>>1) { m = m+1 } M[p,λ,b,g] = m } } End of test for a new maximum End of test for the sign of the wavelet coefficient residual Check for a new maximum Update the maximum of the wavelet coefficient residual amplitude End of test for a new maximum End of test for the sign of the wavelet coefficient residual End of test whether coefficient is in the band of the precinct End of loop over all code group members v Loop over bitplanes of v Include an additional bitplane max End of loop over bitplanes of max Install bitplane count End of loop over all code groups © ISO/IEC 2024 – All rights reserved

<!-- page 73 -->

## Annex E

(normative) Discrete wavelet transformation E.1 General In this annex, the flow charts and tables are normative only in the sense that they are defining an output that alternative implementations shall duplicate.
## NOTE 1

In order to achieve a low-latency requirement and to conform to one or multiple profiles specified in ISO/IEC 21122-2, decoder implementations would need to run the inverse wavelet transformation steps specified in this annex interleaved with the entropy decoding steps of Annex C and the inverse quantization steps of Annex D. The algorithms given in this annex assume for the ease of presentation that all wavelet coefficients of an image are available entirely. This annex describes the forward discrete wavelet transformation applied to one component and specifies the inverse discrete wavelet transformation used to reconstruct the component. The algorithms specified in this annex operate on the wavelet coefficients λ
## NOTE 2

b codestream only uses the intra-coding mode, i.e. does not depend on a frame buffer, the wavelet coefficients c’
, are identical to the wavelet coefficient residuals
, see Annex H.7 which specifies in detail how to obtain
, [ E.2 Discrete inverse wavelet transformation p ]. In case the x ] [ p ] which are the output of the inverse quantization. For details
, ] in general. ] from c’ c’ p p p b b b b λ λ λ λ x x x x c c [ [ [
,
,
,
,
,
,
,
,
,
,
, The algorithm specified in Table E.1 takes the wavelet coefficients b λ Input: as input and transforms them by inverse wavelet transformation into the sample values c’ p x [
,
,
, O x k ] of all precincts, lines and bands
, ]. y [
, c’ p λ b x Output: Wavelet coefficients [
,
,
, ] of all precincts, all lines, all bands for all positions. O k x y Inversely wavelet transformed sample values Table E.1 — Inverse wavelet transformation [ ]
,
, Syntax Notes inverse_transformation() { for(k=0;k<Nc;k=k+1) { reorder_coefficients(k) Dx=min(N’L,x[k],N’L,y[k])) for(dx=N’L,x[k];dx>Dx;dx=dx−1) { hor_transform(k,LLdx-1,N’Ly[k],LLdx,N’Ly[k],HLdx,N’Ly[k]) Loop over components Rearranges components from all precincts into a rectangular grid Compute number of initial horizontal transformations to perform d d Loop over horizontal decomposition k levels k k Ly[ Horizontally transform the LL N’ ] bands of component and HL k into the LL ] band of component
. The output band is a temporary band N’ d Ly[ x-1, Ly[ ] N’ x, x, k } for(d=Dx;d>0;d=d–1) { that is only required for the inverse wavelet transformation End of the horizontal decompositions Loop over the horizontal and vertical decomposition levels © ISO/IEC 2024 – All rights reserved

<!-- page 74 -->

Table E.1 (continued) Table E.1 (continued) hor_transform(k,LLd-1,d,LLd,d,HLd,d) Syntax Notes d d hor_transform(k,LHd-1,d,LHd,d,HHd,d) Ver_transform(k,LLd-1,d-1,LLd-1,d,LHd-1,d) } assign_output(k,LL0,0) } } E.3 Coefficient reordering and scaling d d d d d d
,
, and band of component k bands of component k into the LL Horizontally transform the LL d HL 1,
. The output band is a temporary band that is only required for the inverse computation of the wavelet transformation d d
- 
k
,
, k and band of component Horizontally transform the LH d into the LH HH bands of component 1,
. The output band is a temporary band that is only required for the inverse computation of the wavelet transformation. d d d
- 
- 1,
- 1 band of component
- 1, k band of component Vertically transform the LL d d and LH
- 1, LL band is a temporary band that is only required for the inverse computation of the wavelet transformation. d k band into the
. The output End of horizontal and vertical decompositions k Assign the output of component values of the temporary band LL0,0 End of loop over components to the c’ p λ b x x k T β ] from all precincts and The algorithm specified in Table E.2 assigns the wavelet coefficients the sampling
, [ component N k position. It also applies an additional scaling step that improves the precision of the wavelet transformation. β are The temporary bands are required as input to the inverse wavelet filter. The symbols Input: defined in subclause B.3, indicates the wavelet filter type and to temporary bands ], where and β b β, ] and k x[ β p β b b k λ y y x x [ [
,
,
,
.
, W b Component index Output: positions. Width b[ β ] and heights
. c’ ] in subclause B.4. b b and wavelet coefficients ] of all bands y x b[ H T
. [
,
,
, ] of all precincts, all lines, all bands and all Temporary band array [
,
, ] as input to the wavelet filter. Table E.2 — Coefficient reordering Syntax Notes reorder_coefficients(k) { for(β=0;β<Nβ;β=β+1) { if(bx[β,k]==1) { if(k<Nc−Sd) { b=(Nc−Sd)×β+k } else { b=(Nc−Sd)×Nβ+k } k β Iterate over all bands of component k Check whether filter type component exists in Check whether this is a regular component β Compute the band from the filter type k and the component k Compute the band from the component for non-decomposed components End of decision for band computation © ISO/IEC 2024 – All rights reserved

<!-- page 75 -->

Table E.2 (continued) Table E.2 (continued) for(y=0;y<Hb[β,k];y=y+1) { Syntax Notes b for(x=0;x<Wb[β,k];x=x+1) { ]
, β y s × y p = N
, p x ×     [ ]× k N
, L y + [ d k y     [ [ ]− s k d k y y
, β ] λ = N y umod
, L y log −     x s k x × [ ]× C s [ d k x
, β ]     ξ = x   umod   C s [ d k x2 s k x [ ]×    
, β ] T[β,x,y]=c’[p,λ,b,ξ] << Fq } } } } } E.4 Inverse horizontal filtering Iterate over all rows of band p Iterate over all colums of band x b y Compute the precinct index the horizontal position position
. from and vertical y Compute the line within the precinct from the vertical position x Compute the position within the precinct from the horizontal position p λ Assign and scale the wavelet coefb band ficient in the precinct T and horizontal position y x β temporary band coefficient ξ line to the in band
, column and row
. End of loop over columns End of loop over all rows End of check over filter existence End of loop over all wavelet filter types The algorithm specified in Table E.3 applies an inverse horizontal wavelet filter on a low-pass and high-pass β Input: input band and generates coefficients in a temporary output band. β y x β
## 0 and two input filter types, low-pass

, Output: H and wavelet coefficients in temporary bands
, output wavelet filter type L, Component index L and high-pass x β0 T ] and H, T T y [ ]. β β k y x [
, Wavelet coefficients in temporary output band Table E.3 — Horizontal inverse wavelet transformation [ ]
,
, Syntax Notes hor_transform(k,βo,βL,βH) { for(y=0;y<Hb[β0,k];y=y+1) { for(x=0;x<Wb[β0,k];x=x+1) { i = x/2 if(x umod 2 = 0) { ⎿ ⏌ X[x]=T[βL,i,y] } else { X[x]=T[βH,i,y] } } extend_samples(Wb[β0,k]) inverse_filter_1D(Wb[β0,k]) β β β Horizontally inverse transform the low-pass coefficients b high-pass coefficients o. H to the output band b Iterate over all rows of band L and Iterate over all columns of band Compute the input sample position in the source band If even sample position X Assign the low-pass input to the even samples of the temporary array Else odd sample position X Assign the high-pass samples to the odd samples of the temporary array End of check for even/odd coefficients X End of loop over columns Symmetrically extend the samples across the boundary X Performs an inverse filtering on the temporary array © ISO/IEC 2024 – All rights reserved

<!-- page 76 -->

Table E.3 (continued) Table E.3 (continued) Notes b Iterate over all columns of band Assign inversely transformed wavelet coefficients to the output band End of loop over columns End of loop over all rows Syntax for(x=0;x<Wb[β0,k];x=x+1) { T[β0,x,y]=Y[x] } } } E.5 Inverse vertical filtering The algorithm specified in Table E.4 applies an inverse vertical wavelet filter on a low-pass and high-pass β Input: input band and generates coefficients in a temporary output band. β y x β
## 0 and two input filter types, low-pass

, Output: H and wavelet coefficients in temporary bands
, output wavelet filter type L, Component index L and high-pass x β T ] and H, T T y [ ]. β β k y x [
, Wavelet coefficients in temporary output band Table E.4 — Vertical inverse wavelet transformation [ ]
, 0, Syntax Notes ver_transform(k,βo,βL,βH) { β β β for(x=0;x<Wb[β0,k];x=x+1) { for(y=0;y<Hb[β0,k];y=y+1) { i = y/2 if(y umod 2 = 0) { ⎿ ⏌ X[y]=T[βL,x,i] } else { X[y]=T[βH,y,i] } } extend_samples(Hb[β0,k]) inverse_filter_1D(Hb[β0,k]); for(y=0;y<Hb[β0,k];y=y+1) { T[β0,x,y]=Y[y] } } } E.6 Symmetric extension k L and high-
Vertically inverse transform the low-pass coefficients pass coefficients o in component b
. H to the output band b Iterate over all columns of band Iterate over all rows of band Compute the input sample position in the source band If even sample position X Assign the low-pass input to the even samples of the temporary array Else odd sample position X Assign the high-pass samples to the odd samples of the temporary array End of test for even/odd coefficients X End of loop over columns Symmetrically extend the samples across the boundary Performs an inverse filtering on the temporary array b X Iterate over all columns of band Assign inversely transformed wavelet coefficients to the output band End of loop over columns End of loop over all rows X X The algorithm specified in Table E.5 extends the samples in the temporary array Input: the band and prepares the sample array for the inverse wavelet transformation. X x X Z across the boundaries of Array positions 0 to ] of wavelet coefficients and size Z of the array [ –1.
. The array is assumed to be filled for © ISO/IEC 2024 – All rights reserved

<!-- page 77 -->

Output: X Array of wavelet coefficients that have been symmetrically extended. Table E.5 — Symmetric coefficient extension Syntax Notes extend_samples(Z) { for(i=1;i≤2;i=i+1) { X[-i]=X[i] X[Z+i–1]=X[Z–i–1] } } Loop over two samples beyond the edge of the temporary array Reflect sample at the left boundary Reflect samples at the right boundary End of loop over sample extension Z Due to requirements formulated for the picture header elements
## NOTE

empty bands or bands of length E.7 Inverse wavelet filtering with the 5-3 filter =1 do not appear, such that bands are at least two coefficients wide or high. W H Cw f, f and
, pathological cases such as Y The algorithm specified in Table E.6 computes the inverse wavelet transformation with the 5-3 wavelet output samples in the output filter. It generates from the interleaved low-pass and high-pass input samples Input: array X Z x
. X Output: Array Y [ x ] of wavelet coefficients and size of the array
. Z Array [ ] of inversely wavelet transformed coefficients, valid at least for the indices 0 to Table E.6 — Inverse wavelet filtering with the 5-3 filter –1. Syntax inverse_filter_1D(Z) { for(i=0;i<Z+1;i=i+2) { Y[i]=X[i]−((X[i–1]+X[i+1]+2)>>2) } for(i=1;i<Z;i=i+2) { Y[i]=X[i]+((Y[i–1]+Y[i+1])>>1) } } Notes Loop over even samples Reconstruct even samples from low-pass End of loop over even samples Loop over odd samples Reconstruct odd samples from high-pass End of loop over odd samples E.8 Assignment of output coefficients k Table E.7 provides guidance on the assignment of the output of the inverse wavelet transformation contained Input:
, [ in the temporary array T ] to the output array k
, O T ]. β β y y x x [
,
, indicating an LL0,0 band and temporary array of wavelet y x β Component index c ] of that band.
, O [
, and wavelet type x y Output: coefficients y x β T Output array [
,
, ]. [
,
, ] filled with wavelet coefficients from the LL0,0 band of the temporary array © ISO/IEC 2024 – All rights reserved

<!-- page 78 -->

Table E.7 — Output assignment Syntax Notes assign_output(k,β) { for(y=0;y<Hc[k];y=y+1) { for(x=0;x<Wc[k];x=x+1) { O[k,x,y]=T[β,x,y] } } } k β Assign the output of component k H from the temporary band Loop over the columns of the band data. The height of the component denoted by k W ] and has been specified in subclause B.1. c[ c c is Loop over the row of the band data. The width of the component ed by ] and has been specified in subclause B.1. c[ O T β x y is denot-
Assign to the output coefficients porary wavelet band [ ]
,
, End of loop over rows End of loop over colums the reconstructed values from the tem-
E.9 Discrete forwards wavelet transformations O k x y p c’ Table E.8 provides guidelines for implementing a forwards wavelet transformation at encoder side. The discrete wavelet transformation takes sample values ] and computes from them the wavelet Input: coefficients O ]. b k k λ y y x x x [ [
,
,
,
,
, Output: Sample values [
,
, x c’ ] of all components p b λ at all sample positions and
. Wavelet coefficients [
,
,
, ] of all precincts, all lines, all bands for all positions. Table E.8 provides the steps necessary to implement a forwards wavelet transformation. Table E.8 — Wavelet transformation Syntax Notes forwards_transformation() { for(k=0;k<Nc;k=k+1) { assign_input(k) Dx=min(N’L,x[k],N’L,y[k]) for(d=1;d≤Dx;d=d+1) { ver_fwd_transform(k,LLd-1,d-1,LLd-1,d,LHd-1,d) hor_fwd_transform(k,LLd-1,d,LLd,d,HLd,d) hor_fwd_transform(k,LHd-1,d,LHd,d,HHd,d) } for(dx=Dx+1;dx≤N’L,x[k];dx=dx+1) { hor_fwd_transform(k,LLdx-1,N’Ly[k],LLdx,N’Ly[k],HLdx,N’Ly[k]) © ISO/IEC 2024 – All rights reserved Loop over components Place data of component k into the input buffer of the wavelet filter Compute the number of initial transformations to perform Loop over the horizontal and vertical decomposition levels d d Vertically transform the LL k and LH band into the LL bands of component
- 1,
. d d
- 1 d d
- 1,
- 1, d d d d d d Horizontally transform the LL k band into the LL
. of component and HL
,
,
- 1, bands d d d d d d Horizontally transform the LH k band into the LH
. of component and HH
,
,
- 1, bands End of horizontal and vertical decomposition d N Loop over horizontal-only decom-
N position levels d Horizontally transform the LL N and HL k from the LL
. nent ly d ly bands of component k ly band of compo-
x-1, x, x,

<!-- page 79 -->

Table E.8 (continued) Table E.8 (continued) Syntax Notes End of the horizontal decompositions Places coefficients of components into precincts End of loop over components } insert_coefficients(k) } } E.10 Input coefficient assignment T β x y O k x y [
,
, ] to the temporary Table E.9 provides guidance on how to assign the sample values in the input array Input: wavelet band O ]. k k y x [
,
, Output: Component index T and an array of input sample values β y x [
,
, ]. Temporary array [
,
, ] filled with input data as wavelet coefficients of the LL0,0 band. Table E.9 — Input assignment Syntax Notes assign_input(k) { for(y=0;y<Hc[k];y=y+1) { for(x=0;x<Wc[k];x=x+1) { T[LL0,0,x,y]=O[k,x,y] } } } k Assign the output of component c H to the temporary LL0,0 band Loop over the columns of the band data. The height of the component denoted by W ] and has been specified in subclause B.1. c[ c c c is Loop over the row of the band data. The width of the component O ed by ] and has been specified in subclause B.1. c[ T x y is denot-
Assign to the input coefficients rary wavelet band [LL0,0,
, the reconstructed values to the tempo-
], i.e. set the LL0,0 band to the input data End of loop over rows End of loop over colums E.11 Horizontal wavelet transformation Table E.10 provides guidance on how to perform a horizontal wavelet filter from a temporary input band β Input: and to generate low-pass and high-pass output in temporary output bands. β β β k y x Output: H; wavelet coefficients in temporary output band T
, output wavelet filter type [ Component index
## 0 and two input filter types, low-pass

T 0, T ] β β y y x x
, L and high-pass Filtered wavelet coefficients in temporary bands Table E.10 — Horizontal forward wavelet transformation ] and ]. [ [
,
, H, L, Syntax Notes hor_fwd_transform(k,βo,βL,βH) { β β β Horizontally forward transform the low-pass coefficients b high-pass coefficients o in component H to the output band b Iterate over all rows of band k L and
. for(y=0;y<Hb[β0,k];y=y+1) { for(x=0;x<Wb[β0,k];x=x+1) { X[x]=T[β0,x,y] } extend_samples(Wb[β0,k]) fwd_filter_1D(Wb[β0,k]); Iterate over all columns of band Copy input coefficients to temporary row X End of loop over columns Symmetrically extend the samples across the boundary X Performs wavelet filtering on the temporary array © ISO/IEC 2024 – All rights reserved

<!-- page 80 -->

Table E.10 (continued) Table E.10 (continued) Syntax Notes for(x=0;x<Wb[b];x=x+1) { i = x/2 if(x umod 2 = 0) { ⎿ ⏌ T[βL,i,y]=Y[x] } else { T[βH,i,y]=Y[x] } } } } b Iterate over all columns of band Compute the input sample position in the source band If even sample position Assign the even samples to the low-pass output. Else odd sample position Assign the odd samples to the high-pass output. End of even/odd decision End of loop over columns End of loop over all rows E.12 Vertical wavelet transformation Table E.11 provides guidance how to perform a vertical wavelet filter from wavelet coefficients in a β0 Input: temporary band and to create low-pass and high-pass output bands. T β and two input filter types, low-pass β [ Output: H and wavelet coefficients in a temporary input band
, output wavelet filter type Component index L and high-pass β0 β
, T T x ] β k y y y x x
, Wavelet coefficients in temporary output bands Table E.11 — Vertical forward wavelet transformation ] and ]. [ [
,
, H, L, Syntax Notes ver_fwd_transform(k,βo,βL,βH) { β β β for(x=0;x<Wb[β0,k];x=x+1) { for(y=0;y<Hb[β0,k];y=y+1) { X[y]=T[β0,x,y] } extend_samples(Hb[β0,k]) fwd_filter_1D(Hb[β0,k]); for(y=0;y<Hb[β0,k];y=y+1) { i = y/2 if(y umod 2 = 0) { ⎿ ⏌ T[βL,x,i]=Y[y] } else { T[βH,y,i]=Y[y] } } } } Vertical forward transform the coefficients in the input band the low-pass coefficients L and high-pass coefficients H. Iterate over all columns Iterate over all rows o to Retrieve one column of the wavelet coefficients and store them in the temporary column End of loop over colums Symmetrically extend the samples across the boundary X X Performs wavelet filtering on the temporary array Iterate over all rows Compute the input sample position in the source band If even sample position Assign the even samples to the low-pass output Else odd sample position Assign the odd samples to the high-pass output. End of even/odd sample decision End of loop over all columns End of loop over all rows © ISO/IEC 2024 – All rights reserved

<!-- page 81 -->

E.13 Forwards wavelet filtering with the 5-3 filter X Y Table E.12 provides guidance on how to implement the forwards wavelet transformation with the 5-3 interleaved low-pass and high-pass output wavelet filter. It generates from the input samples in the array Z Input: in the array X
. x Output: Array Y [ x ] of wavelet coefficients and size of the array
. Z Array [ ] of wavelet transformed coefficients, valid at least for the indices 0 to Table E.12 — Forward wavelet filtering with the 5-3 filter –1. Syntax fwd_filter_1D(Z) { for(i=-1;i<Z+1;i=i+2) { Y[i]=X[i]−((X[i–1]+X[i+1])>>1) } for(i=0;i<Z;i=i+2) { Y[i]=X[i]+((Y[i–1]+Y[i+1]+2)>>2) } } Notes Loop over odd samples Generate the high-pass in the odd samples End of loop over odd samples Loop over even samples Update the even samples to generate the low-pass End of loop over even samples E.14 Insertion of coefficients into precincts Table E.13 provides guidance on how to assign the wavelet coefficients Input: the precincts. W H T β b k y x b Component index Output: step, width b[ c’ ] and heights and temporary band array b λ ] of all bands. p b[ x [
,
, ] containing the output of the wavelet filter c’ p λ b x [
,
,
, ] in the temporary bands to Wavelet coefficients Table E.13 — Coefficient insertion into precinct ] of all precincts, lines, bands and positions. [
,
,
, Syntax insert_coefficients(k) { for(β=0;β<Nβ;β=β+1) { if(bx[β,k] == 1) { if(k < Nc−Sd) { b=(Nc−Sd)×β+k } else { b=(Nc−Sd)×Nβ+k } for(y=0;y<Hb[β,k];y=y+1) { for(x=0;x<Wb[β,k];x=x+1) { p N =
, p x × y s × y    
, β ] [ d k y [ ]× k N
, L y     +     x s k x × [ ]× C s [ d k x
, β ]     Notes k Iterate over all bands of component k Check whether the corresponding filter type exists in component Check whether this is a regular component k Compute the band from the filter type ponent β and the com-
k for non-de-
Compute the band from the component composed components End of decision for band computation b b Iterate over all colums of band Iterate over all rows of band p y Compute the precinct index tion x and vertical position
. from the horizontal posi-
λ = y umod N
, L y log − [ [ ]− s k d k y y
, β ] y Compute the line within the precinct from the vertical position © ISO/IEC 2024 – All rights reserved

<!-- page 82 -->

Table E.13 (continued) Table E.13 (continued) Syntax ξ = x umod     C s [ d k x s k x [ ]×    
, β ] r = (1<< Fq) >> 1 if(T[β,x,y] >= 0) { c’[p,λ,b,ξ]=(T[β,x,y]+r)>>Fq } else { c’[p,λ,b,ξ]=-((-T[β,x,y]+r)>> Fq) } } } } } } Notes x Compute the position within the precinct from the horizontal position Compute the rounding offset β Check for the sign of the coefficient, if non-negative p Insert the scaled wavelet coefficient from the tempox rary band
. position into precinct band line and horizontal b λ If negative β p Insert the scaled wavelet coefficient from the tempox rary band
. position into precinct band line and horizontal b λ End of check for sign End of loop over columns End of loop over all rows End of check if band exists End of loop over all wavelet filter types © ISO/IEC 2024 – All rights reserved

<!-- page 83 -->

## Annex F

(normative) Multiple component transformations F.1 General In this annex, the flow charts and tables are normative only in the sense that they are defining an output that alternative implementations shall duplicate.
## NOTE

In order to achieve a low-latency requirement and to conform to one or multiple profiles specified in ISO/IEC 21122-2, decoder implementations would need to run the inverse multiple component steps specified in this annex interleaved with the inverse discrete wavelet transformation steps of Annex E. The algorithms given in this annex assume for the ease of presentation that all sample values of an image are available entirely. F.2 Inverse multiple component transformation O c x y Cpih i i s Cpih i An inverse multiple component transformation shall be applied to the output sample values wavelet transformation if the s 0, see Table A.9. x[ components, i.e. if ] of the N element of the picture header specified in subclause A.4.3 is different from c<3, or if any of the sampling factors i shall be smaller than 2 if there are less than 4 <3 are different from 1. ] and shall be 0 if there are less than 3 components, i.e. if N <3 are different from 1. Furthermore, c<3, or if any of the sampling factors Cpih i s ] and ] for ] for Cpih y[ y[ x[ s [ i
,
, The Inverse multiple component transformation shall be selected from Table F.1 — Selection of the inverse multiple component transformation according to Table F.1: Cpih Inverse Multiple Component Transformation Ω c x y O c x y c x y ] for all components c O y Ω x c
, all columns x y and all [
,
, ] = [
,
, No transformation, set rows
. c inverse_rct(), see subclause F.3, to compute c Ω nents [ Set ] for all components y x <3.
,
, O y x c c ≥3. [
,
, ] = c c O <4.
, [ x y Ω inverse_star_tetrix(), see subclause F.5, to compute c components ] =
, [ Set ] for all components ≥4. y x c
,
, [
,
, ] from [
,
, ] for all compo-
Ω c x y O c x y [
,
, ] from [
,
, ] for all All other values Reserved for ISO/IEC purposes. F.3 Inverse reversible multiple component transformation (inverse RCT) Cpih W Input: Table F.2 specifies the inverse reversible multiple component transformation that is applied if O y x c H equals 1: The output array of the inverse wavelet transformation [
,
, ] and the dimensions f and f of the Output: sampling grid. Ω c x y The intermediate image samples values [
,
, ] of the image. © ISO/IEC 2024 – All rights reserved

<!-- page 84 -->

Table F.2 — Inverse reversible multiple component transformation Syntax inverse_rct() { for(y=0;y<Hf;y=y+1) { for(x=0;x<Wf;x=x+1) { i0 = O[0,x,y] i1 = O[1,x,y] i2 = O[2,x,y] o1 = i0–((i1+i2)>>2) o0 = o1 + i2 o2 = o1 + i1 Ω[0,x,y] = o0 Ω[1,x,y] = o1 Ω[2,x,y] = o2 } } } Notes Loop over rows of the image Loop over the columns of the image Retrieve input components Reconstruct the green component Reconstruct the red component Reconstruct the blue component Assign the red output Assign the green output Assign the blue output End of loop over columns End of loop over rows F.4 Forward reversible multiple component transformation Table F.3 provides guidance on the forward multiple component colour decorrelation transformation that is y x Input: inverted by the procedure specified in subclause F.3 at the decoder side. W H Ω c Output: Scaled intermediate image sample values O c x y [
,
, ] and the dimensions of the sampling grid f and f. Decorrelated sample values Table F.3 — Forwards reversible multiple component transformation ] suitable as input of the forward wavelet transformation. [
,
, Syntax forward_rct() { for(y=0;y<Hf;y=y+1) { for(x=0;x<Wf;x=x+1) { i0 = Ω[0,x,y] i1 = Ω[1,x,y] i2 = Ω[2,x,y] o0 = (i0+2×i1+i2)>>2 o1 = i2 – i1 o2 = i0 – i1 O[0,x,y] = o0 O[1,x,y] = o1 O[2,x,y] = o2 } } } Notes Loop over rows of the image Loop over the columns of the image Retrieve input components Compute the luma component Compute the Cb chroma component Compute the Cr chroma component Assign the luma output Assign Cb Assign Cr End of loop over columns End of loop over rows © ISO/IEC 2024 – All rights reserved

<!-- page 85 -->

F.5 Inverse Star-Tetrix transform F.5.1 Reconstruction with the Star-Tetrix transformation Table F.4 specifies the inverse Star-Tetrix transformation that is applied if equals 3: Cpih
## NOTE

This algorithm assigns the red decoder output to component 0, the green decoder outputs to components
## 1 and 2, and the blue component output to component 3, consistent with the rest of this document, in particular with

Input: Table F.8. W H O y x c The output array of the inverse wavelet transformation [
,
, ] and the dimensions f and f of the Output: sampling grid. Ω c x y The intermediate image samples values
, Table F.4 — Inverse Star-Tetrix transform ] of the image. [
, Syntax Notes Reference inverse_star_tetrix() { inv_avg_step() inv_delta_step() inv_Y_step() inv_CbCr_step() for(y=0;y<Hf;y=y+1) { for(x=0;x<Wf;x=x+1) { Ω[0,x,y] = ω4[2,x,y] Ω[1,x,y] = ω4[3,x,y] Ω[2,x,y] = ω4[0,x,y] Ω[3,x,y] = ω4[1,x,y] } } } F.5.2 Inverse average step Reconstruct Y2 from Ya and Δ Reconstruct Y1 from Δ and Y2 Reconstruct G1 and G2 from Y1, Y2 and Cr, Cb Reconstruct R and B from Cr, Cb and G1, G2 Loop over rows of the image Loop over the columns of the image Table F.5 Table F.6 Table F.7 Table F.8 Assign the red output Assign the first green output Assign the second green output Assign the blue output End of loop over columns End of loop over rows Table F.5 specifies the inverse average step which reconstructs the Y2 component from Ya and Δ. The access() Input: function is specified in subclause F.5.7. O y x c The output array of the inverse wavelet transformation [
,
, ] and the dimensions Wf and Hf of the Output: sampling grid. ω1 c x y The first lifting step output Table F.5 — Inverse average step ] of the image. [
,
, Syntax Notes inv_avg_step() { for(y=0;y<Hf;y=y+1) { for(x=0;x<Wf;x=x+1) { Δlt = O[access(0,x,y,−1, −1)] Δrt = O[access(0,x,y,+1,−1)] Δlb = O[access(0,x,y,−1,+1)] Loop over rows of the image Loop over the columns of the image Read the delta component to the top-left Read the delta component to the top-right Read the delta component to the bottom-left © ISO/IEC 2024 – All rights reserved

<!-- page 86 -->

Table F.5 (continued) Table F.5 (continued) Syntax Notes Δrb = O[access(0,x,y,+1,+1)] ω1[0,x,y]=O[0,x,y]-
∆ ∆  lb      ∆ ∆ rb + + + rt lt ω1[1,x,y]=O[1,x,y] ω1[2,x,y]=O[2,x,y] ω1[3,x,y]=O[3,x,y] } } } F.5.3 Inverse delta step Read the delta component to the bottom-right Reconstruct Y2 from Ya and Δ Copy remaining components over Copy remaining components over Copy remaining components over End of loop over columns End of loop over rows Table F.6 specifies the inverse delta step which reconstructs the Y1 component from Y2 and Δ. The access() c Input: function is specified in subclause F.5.7. ω1 W H y x Output: The output array of the inverse average step ω2 c x y [
,
, ] and the dimensions f and f of the sampling grid. The second lifting step output ] of the image. Table F.6 — Inverse delta step [
,
, Syntax Notes inv_delta_step() { for(y=0;y<Hf;y=y+1) { for(x=0;x<Wf;x=x+1) { Ylt = ω1[access(3,x,y,−1, −1)] Yrt = ω1[access(3,x,y,+1,−1)] Ylb = ω1[access(3,x,y,−1,+1)] Yrb = ω1[access(3,x,y,+1,+1)] ω2[3,x,y]=ω1[3,x,y]+ Y + Y + Y + Y  lb lt      rb rt ω2[1,x,y]=ω1[1,x,y] ω2[2,x,y]=ω1[2,x,y] ω2[0,x,y]=ω1[0,x,y] } } } F.5.4 Inverse Y step Loop over rows of the image Loop over the columns of the image Read the Y2 component to the top-left Read the Y2 component to the top-right Read the Y2 component to the bottom-left Read the Y2 component to the bottom-right Reconstruct Y1 from Y2 and Δ Copy remaining components over Copy remaining components over Copy remaining components over End of loop over columns End of loop over rows Table F.7 specifies the inverse Y step which reconstructs the G1 and G2 components from Y1, Y2 and Cb, Cr. c Input: The access() function is specified in subclause F.5.7. ω2 y e e x W H The output array of the second lifting step Output: the dimensions f and y f of the sampling grid. x c ω3 [
,
, ], the chroma weighting exponents
## 1 and

## 2 and

The third lifting step output [
,
, ] of the image. © ISO/IEC 2024 – All rights reserved

<!-- page 87 -->

Table F.7 — Inverse Y step Syntax Notes inv_Y_step() { for(y=0;y<Hf;y=y+1) { for(x=0;x<Wf;x=x+1) { Bl = ω2[access(0,x,y,−1,0)] Br = ω2[access(0,x,y,+1,0)] Rt = ω2[access(0,x,y,0,−1)] Rb = ω2[access(0,x,y,0,+1)] ω3[0,x,y]=ω2[0,x,y]− e e B + B + 2 r R + R t l ) ( ( b Bt= ω2[access(3,x,y,0,−1)] Bb= ω2[access(3,x,y,0,+1)] Rl = ω2[access(3,x,y,−1,0)] Rr = ω2[access(3,x,y,+1,0)] ω3[3,x,y]=ω2[3,x,y]− e e B + B + 2 b R + R l t ( ) ( r         ω3[1,x,y]=ω2[1,x,y] ω3[2,x,y]=ω2[2,x,y] } } } F.5.5 Inverse CbCr step )     )     Loop over rows of the image Loop over the columns of the image Read the Cb component to the left of G2 Read the Cb component to the right of G2 Read the Cr component to the top of G2 Read the Cr component to the bottom of G2 Compute G2 from Y2 and neighbouring Cb and Cr samples Read the Cb component to the top of G1 Read the Cb component to the bottom of G1 Read the Cr component to the left of G1 Read the Cr component to the right of G1 Compute G1 from Y1 and neighbouring Cb and Cr samples Copy remaining components over Copy remaining components over End of loop over columns End of loop over rows Table F.8 specifies the inverse CbCr step which reconstructs the R and B components from G1 and G2 and Cb, y x Input: Cr. The access() function is specified in subclause F.5.7. ω3 W H c Output: The output array of the third lifting step ω4 c x y [
,
, ] and the dimensions f and f of the sampling grid. The fourth lifting step output [ ] of the image. Table F.8 — Inverse CbCr step
,
, Syntax Notes inv_cbcr_step() { for(y=0;y<Hf;y=y+1) { for(x=0;x<Wf;x=x+1) { Gl = ω3[access(1,x,y,−1,0)] Gr = ω3[access(1,x,y,+1,0)] Gt = ω3[access(1,x,y,0,−1)] Gb = ω3[access(1,x,y,0,+1)] ω4[1,x,y]=ω3[1,x,y]+ b l G + G + G + G r t       Gl = ω3[access(2,x,y,−1,0)] Loop over rows of the image Loop over the columns of the image Read the G component to the left of Cb Read the G component to the right of Cb Read the G component to the top of Cb Read the G component to the bottom of Cb Compute B from Cb and neighbouring G samples Read the G component to the left of Cr © ISO/IEC 2024 – All rights reserved

<!-- page 88 -->

Table F.8 (continued) Table F.8 (continued) Syntax Notes Gr = ω3[access(2,x,y,+1,0)] Gt = ω3[access(2,x,y,0,−1)] Gb = ω3[access(2,x,y,0,+1)] ω4[2,x,y]=ω3[2,x,y]+ b l G + G + G + G r t       ω4[0,x,y]=ω3[0,x,y] ω4[3,x,y]=ω3[3,x,y] } } } F.5.6 Super pixel look-up tables Read the G component to the right of Cr Read the G component to the top of Cr Read the G component to the bottom of Cr Compute R from Cr and neighbouring G samples Copy remaining components over Copy remaining components over End of loop over columns End of loop over rows c c yCrg Xcrg Table F.9 specifies the CFA pattern type Ct depending on the values of the component registration values Table F.9 — CFA Pattern type derived from the component registration ] found in the component registration marker, see subclause A.4.9. ], [ [ Component index c Xcrg[c] Ycrg[c] CFA Pattern Type Ct
## 32768

## 32768

## 32768

## 32768

## 32768

## 32768

## 32768

## 32768

## 32768

## 32768

## 32768

## 32768

## 32768

## 32768

## 32768

## 32768

Xcrg Ycrg Notes (Subpixel arrangement and chroma exponent assignment) e e
## RGGB

## 1 is the Cr weight

## 2 is the Cb weight

e
## BGGR

e
## 1 is the Cb weight

## 2 is the Cr weight

e
## GRBG

e2
## 1 is the Cr weight

is the Cb weight e
## GBRG

e
## 1 is the Cb weight

## 2 is the Cr weight

All other combinations of [] and [] Reserved for ISO/IEC purposes C e e
## NOTE

marker segment as indicated in the table. Exchanging the registration of R and B does not change c C C t, but can require exchanging
## 1 and

## 2 in the CTS

δ δ Table F.10 specifies for each component index pixel, dependent on the CFA pattern type t. The value of t is defined in Table F.9. between 0 the displacement vector x, y within a CFA super © ISO/IEC 2024 – All rights reserved

<!-- page 89 -->

Table F.10 — Component displacement vector by component index Component index c CFA pattern type Ct Displacement vector δx,δy 0,1 1,1 0,0 1,0 1,1 0,1 δ δ k Table F.11 specifies its inverse function [ y] evaluates to the component index [ x, δ δ k c x, at the given displacement vector within a CFA super pixel. y]. Given a displacement vector 1,0 δ 0,0 y and a CFA pattern type x, δ C t, Table F.11 — Component index by displacement vector Displacement vector δx,δy CFA pattern type Ct Component index k[δx,δy] 0,1 1,1 0,0 1,0 1,1 0,1 1,0 F.5.7 Coordinate access function 0,0 Table F.12 specifies a function that computes the component and coordinates of a sub-pixel relative to a subx Input: pixel at a given coordinate and of a given component H r The coordinates ( x and ) and component c in the sample grid of a super pixel of a CFA array, a sample t and the C f, the colour transformation CFA pattern type y the sample grid dimensions f and W C y r
, offset Output: colour transformation reflection and extension flags y x x c c y f. A triple (
,
, ) of component Table F.12 — Coordinate access function position within the sample grid. position and
, Syntax access(c,x,y,rx,ry){
if((2x+rx+δx[c] < 0) || (2x+rx+δx[c] >= 2Wf)) { rx =−rx }
if((Cf == 3 && ry+δy[c] < 0)||
(Cf == 3 && ry+δy[c] > 1)||
(2y+ry+δy[c] < 0) || (2y+ry+δy[c] >= 2Hf)) { ry =−ry } Notes Check whether the access would go beyond the sample grid If so, reflect back into the line Check whether the access is in-line and an access is attempted to the line above, or the access is in-line, and an access is made to the line below, or the access would go beyond the sample grid If so, reflect back into the line © ISO/IEC 2024 – All rights reserved

<!-- page 90 -->

Table F.12 (continued) Table F.12 (continued) Syntax Notes    2x + r + x y r + y + xδ c [ ] δ y c [ ]       x = y=    c=k[(rx+δx[c])umod2,(ry+δy[c])umod2] return (c,x,y) } Compute horizontal sample position of the resulting component and pixel Compute vertical sample position of the resulting component and pixel Compute component index at the CFA sample position Provide results to caller F.6 Forward Star-Tetrix transform F.6.1 Guidance on the encoder implementation Table F.13 provides guidance on the implementation of a forward Star-Tetrix transformation that is compatible to the inverse transformation specified in subclause F.5.
## NOTE

This algorithm assigns the average luma signal to the first component, the blue-green chroma difference to the second component, the red-green chroma difference to the third component and the differential luma signal to the y x Input: third component, consistent with the rest of this document. Ω c Output: Scaled intermediate image sample values O c x y [
,
, ] and the dimensions of the sampling grid Wf and Hf. Decorrelated sample values Table F.13 — Forward Star-Tetrix transform ] suitable as input of the forward wavelet transformation. [
,
, Syntax Notes References forward_star_tetrix() { for(y=0;y<Hf;y=y+1) { for(x=0;x<Wf;x=x+1) { ω4[2,x,y] = Ω[0,x,y] ω4[3,x,y] = Ω[1,x,y] ω4[0,x,y] = Ω[2,x,y] ω4[1,x,y] = Ω[3,x,y] } } CbCr_step() Y_step() delta_step() avg_step() } F.6.2 Forward CbCr step Loop over rows of the image Loop over the columns of the image Assign the red output Assign the first green output Assign the second green output Assign the blue output End of loop over columns End of loop over rows Compute Cr and Cb from R, B and G1, G2 Compute Y1 and Y2 from G1, G2 and Cr
,Cb Compute Δ from Y1 and Y2 Compute Ya from Y2 and Δ Table F.14 Table F.15 Table F.16 Table F.17 Table F.14 provides guidance on the forward CbCr step which computes Cb and Cr from the components R, B Input: and G1, G2. The access() function is specified in subclause F.5.7. ω4 y x c Output: The array of re-ordered input data ω3 [ c
, x
, y ] and the dimensions Wf and Hf of the sampling grid. The output of the first lifting step [
,
, ] of the image. © ISO/IEC 2024 – All rights reserved

<!-- page 91 -->

Table F.14 — Forward CbCr step Syntax Notes CbCr_step() { for(y=0;y<Hf;y=y+1) { for(x=0;x<Wf;x=x+1) { Gl = ω4[access(1,x,y,−1,0)] Gr = ω4[access(1,x,y,+1,0)] Gt = ω4[access(1,x,y,0,−1)] Gb = ω4[access(1,x,y,0,+1)] ω3[1,x,y]=ω4[1,x,y]b l G + G + G + G r t Gl = ω4[access(2,x,y,−1,0)] Gr = ω4[access(2,x,y,+1,0)] Gt = ω4[access(2,x,y,0,−1)] Gb = ω4[access(2,x,y,0,+1)] ω3[2,x,y]=ω4[2,x,y]b l G + G + G + G r t       ω3[0,x,y]=ω4[0,x,y] ω3[3,x,y]=ω4[3,x,y] } } } F.6.3 Forward Y step Loop over rows of the image Loop over the columns of the image Read the G component to the left of B Read the G component to the right of B Read the G component to the top of B Read the G component to the bottom of B Compute Cb from neighbouring G samples Read the G component to the left of R Read the G component to the right of R Read the G component to the top of R Read the G component to the bottom of R Compute Cr from R and neighbouring G samples Copy remaining components over Copy remaining components over End of loop over columns End of loop over rows Table F.15 provides guidance on the forward Y step which computes Y1, Y2 from G1, G2 and Cb, Cr. The access() y x Input: function is specified in subclause F.5.7. H ω3 W e e c
## 1 and

## 2 and the

The output array of the first lifting step [
,
, ], the choma weighting exponents Output: dimensions f and f of the sampling grid. [c,x,y] of the image. The output of the second lifting step ω Table F.15 — Forward Y step Syntax Notes Y_step() { for(y=0;y<Hf;y=y+1) { for(x=0;x<Wf;x=x+1) { Bl = ω3[access(0,x,y,−1,0)] Br = ω3[access(0,x,y,+1,0)] Rt = ω3[access(0,x,y,0,−1)] Rb = ω3[access(0,x,y,0,+1)] ω2[0,x,y]=ω3[0,x,y]+ e e B + B + 2 r R + R t l ) ( ( b Bt= ω3[access(3,x,y,0,−1)] Loop over rows of the image Loop over the columns of the image Read the Cb component to the left of G2 Read the Cb component to the right of G2 Read the Cr component to the top of G2 Read the Cr component to the bottom of G2 Compute Y2 from G2 and neighbouring Cb and Cr samples Read the Cb component to the top of G1 )     © ISO/IEC 2024 – All rights reserved          

<!-- page 92 -->

Table F.15 (continued) Table F.15 (continued) Syntax Notes Bb= ω3[access(3,x,y,0,+1)] Rl = ω3[access(3,x,y,−1,0)] Rr = ω3[access(3,x,y,+1,0)] ω2[3,x,y]=ω3[3,x,y]+ e e B + B + 2 b R + R l t ( ) ( r     Read the Cb component to the bottom of G1 Read the Cr component to the left of G1 Read the Cr component to the right of G1 Compute Y1 from G1 and neighbouring Cb and Cr samples )     ω2[1,x,y]=ω3[1,x,y] ω2[2,x,y]=ω3[2,x,y] } } } F.6.4 Forward delta step Copy remaining components over Copy remaining components over End of loop over columns End of loop over rows Table F.16 provides guidance on the forward delta step which computes Δ from the Y1 and Y2 components. y x c Input: The access() function is specified in subclause F.5.7. ω2 W H Output: The output array of the forward Y step ω1 c x y [
,
, ] and the dimensions f and f of the sampling grid. The third lifting step output Table F.16 — Forward delta step ] of the image. [
,
, Syntax Notes delta_step() { for(y=0;y<Hf;y=y+1) { for(x=0;x<Wf;x=x+1) { Ylt = ω2[access(3,x,y,−1, −1)] Yrt = ω2[access(3,x,y,+1,−1)] Ylb = ω2[access(3,x,y,−1,+1)] Yrb = ω2[access(3,x,y,+1,+1)] ω1[3,x,y]=ω2[3,x,y]-
Y + Y + Y + Y lt rb lb rt       Ω1[1,x,y]=ω2[1,x,y] ω1[2,x,y]=ω2[2,x,y] ω1[0,x,y]=ω2[0,x,y] } } } F.6.5 Forward average step Loop over rows of the image Loop over the columns of the image Read the Y2 component to the top-left Read the Y2 component to the top-right Read the Y2 component to the bottom-left Read the Y2 component to the bottom-right Compute Δ from Y1 and Y2. Copy remaining components over Copy remaining components over Copy remaining components over End of loop over columns End of loop over rows Table F.17 provides guidance on the forward average step which computes Ya from Y2 and Δ. The access() Input: function is specified in subclause F.5.7. ω1 W H y x c The output of the third lifting step Output: sampling grid. [
,
, ] of the image and the dimensions O c x y f and f of the The input to the forward wavelet transformation [
,
, ]. © ISO/IEC 2024 – All rights reserved

<!-- page 93 -->

Table F.17 — Forward average step Syntax Notes avg_step() { for(y=0;y<Hf;y=y+1) { for(x=0;x<Wf;x=x+1) { Δlt = ω1[access(0,x,y,−1, −1)] Δrt = ω1[access(0,x,y,+1,−1)] Δlb = ω1[access(0,x,y,−1,+1)] Δrb = ω1[access(0,x,y,+1,+1)] O[0,x,y]=ω1[0,x,y]+ + ∆ lt ∆ ∆ ∆ rb + + lb rt       O[1,x,y]=ω1[1,x,y] O[2,x,y]=ω1[2,x,y] O[3,x,y]=ω1[3,x,y] } } } Loop over rows of the image Loop over the columns of the image Read the delta component to the top-left Read the delta component to the top-right Read the delta component to the bottom-left Read the delta component to the bottom-right Compute Ya from Y2 and Δ Copy remaining components over Copy remaining components over Copy remaining components over End of loop over columns End of loop over rows © ISO/IEC 2024 – All rights reserved

<!-- page 94 -->

## Annex G

(normative) DC level shifting, non-linear transform and output clipping G.1 General In this annex, the flowcharts and tables are normative only in the sense that they are defining an output that alternative implementations shall duplicate.
## NOTE

In order to achieve a low-latency requirement and to conform to one or multiple profiles specified in ISO/IEC 21122-2, decoder implementations would need to run the output scaling, DC level shifting, the optional nonlinear transformation and the output clipping specified in this annex interleaved with the inverse multiple component decorrelation transformation steps of Annex F. The algorithms given in this annex assume for the ease of presentation that all sample values of an image are available entirely. G.2 Output scaling, DC level shifting and output clipping The processing steps specified in annex scale the output of the wavelet transformation or inverse multiple component decorrelation transformation to the component precision indicated in the component table, apply DC level shifting to convert the signed output of the wavelet transformation or inverse multiple component transformation to unsigned sample values, and clamp the coefficients to the valid sample range. Table G.1 selects the output scaling, DC level shifting and output clipping in accordance to the presence of an NLT marker, and the Tnlt value signalled in this marker if present: Table G.1 — Selection of the Output Scaling NLT marker and Tnlt value Output scaling method Tnlt NLT marker not present Tnlt NLT marker present and NLT marker present and =1 =2 G.3 Linear output scaling Linear output scaling according to subclause G.3 Quadratic output scaling according to subclause G.4 Extended output scaling according to subclause G.5 Table G.2 specifies the linear output scaling and clipping function that shall be used to reconstruct output Input: sample values in case no NLT marker is present in the codestream. Ω y x i W i i Intermediate image sample values
, H component transformation specified in Table F.2, rows and columns, dimensions of all components ] of all components as generated by the inverse multi-
] and ] as indicated by the component table ] computed according to subclause B.1, and sample precisions c[ c[ B [ [ i
, Output: specified in subclause A.4.5. R i x y Reconstructed sample values [
,
, ] of all components, all rows and columns. © ISO/IEC 2024 – All rights reserved

<!-- page 95 -->

Table G.2 — Linear output scaling and level shifting Syntax linear_output_scaling() { for(i=0;i<Nc;i=i+1) { ζ=Bw–B[i] m=(1<<B[i])–1 for(y=0;y<Hc[i];y=y+1) { for(x=0;x<Wc[i];x=x+1) { v=Ω[i,x,y] v=v+((1<<Bw)>>1) v=(v+((1<<ζ)>>1))>>ζ v=clamp(v,0,m) R[i,x,y]=v } } } } Notes Compute scaling value from the nominal bit precision of the wavelet coefficients signalled in the picture header and the component precision of component i
. Compute maximal value Loop over rows of the component Loop over the columns of the component Retrieve output sample from the inverse multiple component transformation Apply inverse DC level shift. Shift to the output range Clamp to target range Store final reconstructed value End of loop over columns End of loop over rows End of loop over components As indicated in the table, the output scaling runs over all components, including those that are excluded
## NOTE

from the wavelet transformation. G.4 Quadratic output scaling Tnlt Table G.3 specifies the linear output scaling and clipping function that shall be used to reconstruct output field therein sample values in case the codestream contains an NLT marker, see subclause A.4.6, and the i Ω Input: has the value 1. H Intermediate image sample values i [
,
, component transformation specified in Table F.2, rows and columns, dimensions of all components and table specified in subclause 0. The DC offset value Output: subclause A.4.6. ] computed according to subclause B.1, and sample precisions i ] of all components as generated by the inverse multi-
] ] as indicated by the component is defined in the NLT marker segment specified in DCO W c[ c[ B R y y x x [ i i Reconstructed sample values Table G.3 — Quadratic output scaling and level shifting ] of all components, all rows and columns. [
,
, Syntax Notes quadratic_output_scaling() { for(i=0;i<Nc;i=i+1) { ζ=2×Bw–B[i] m=(1<<B[i])–1 for(y=0;y<Hc[i];y=y+1) { for(x=0;x<Wc[i];x=x+1) { v=Ω[i,x,y] v=v+((1 << Bw)>>1) Compute scaling value from the nominal bit precision of the i wavelet coefficients signalled in the picture header and the com-
. ponent precision of component Compute maximal value Loop over rows of the component Loop over the columns of the component Retrieve output sample from the inverse multiple component transformation Apply inverse DC level shift. © ISO/IEC 2024 – All rights reserved

<!-- page 96 -->

Syntax v=clamp(v,0,(1<<Bw)-1) v=v×v v=(v+((1<<ζ)>>1))>>ζ v=v+DCO v=clamp(v,0,m) R[i,x,y]=v } } } } Table G.3 (continued) Table G.3 (continued) Notes Clamp to range Inverse gamma correction Shift to the output range Apply variable DC level shift. Clamp to target range Store final reconstructed value End of loop over columns End of loop over rows End of loop over components Bw The inverse gamma correction step can produce intermediate results that can exceed 32 bit precision. For
## NOTE

a value of G.5 Extended output scaling =18, v can obtain values as large as 2
. Tnlt Table G.4 specifies the linear output scaling and clipping function that shall be used to reconstruct output sample values in case the codestream contains an NLT marker, see subclause A.4.6, and the field therein i Ω Input: has the value 2. y x H Intermediate image sample values i [
,
, component transformation specified in Table F.2, rows and columns, dimensions of all components c[ and Output: specified in subclause A.4.5, NLT marker parameters ] computed according to subclause B.1, sample precisions according to subclause A.4.6.
## 2 and

1, R T y x i i ] of all components as generated by the inverse multi-
] ] as indicated by the component table W c[ B T E [ i Reconstructed sample values Table G.4 — Extended output scaling and level shifting ] of all components, all rows and columns. [
,
, Syntax Notes extended_output_scaling() { for(i=0;i<Nc;i=i+1) { B2=T1×T1 A1=B2+(T1<<(Bw–E))+(1<<(2×Bw–2–2×E)) B1=T1+(1<<(Bw–E–1)) A3=B2+(T2<<(Bw–E))–(1<<(2×Bw–2–2×E)) B3=T2−(1<<(Bw–E–1)) ε=Bw–E ζ=2×Bw−B[i] m=(1<<B[i])–1 for(y=0;y<Hc[i];y=y+1) { for(x=0;x<Wc[i];x=x+1) { v=Ω[i,x,y] v=v+((1<<Bw)>>1) if(v<T1) { © ISO/IEC 2024 – All rights reserved Compute linear region offset value Compute black region Compute black region A1 B1 offset A3 offset B3 Compute normal region offset Compute normal region offset Compute shift in linear region Compute shift to output range Compute maximal value Loop over rows of the component Loop over the columns of the component Retrieve output sample from the inverse multiple component transformation Apply inverse DC level shift. Check whether the sample is in the black region

<!-- page 97 -->

Table G.4 (continued) Table G.4 (continued) Syntax Notes v=B1–v v=clamp(v,0,(1<<Bw)–1) v=A1–v×v } else if(v<T2) { v=(v<<ε)+B2 } else { v=v–B3 v=clamp(v,0,(1<<Bw)–1) v=A3+v×v } v=(v+((1<<ζ)>>1))>>ζ v=clamp(v,0,m) R[i,x,y]=v } } } } Apply inverse offset in black region Clamp to range Apply inverse gamma in black region Check for linear region Apply scale and shift in linear region Regular region Apply inverse offset in regular region Clamp to range Apply inverse gamma correction in regular region Shift to the output range Clamp to target range Store final reconstructed value End of loop over columns End of loop over rows End of loop over components v Bw The inverse gamma correction step can produce intermediate results that can exceed 32-bit precision. For A3 B3 A1
## NOTE

a value of G.6 Input DC level shift and scaling and =18,
,
, can require precisions as large as 2
. Table G.5 provides an overview on input scaling methods and the corresponding signalling mechanisms, as well as references to additional subclauses within this document which provide guidance on their implementation. Table G.5 — Input scaling selection and signalling Input scaling method Reference Signalling linear input scaling quadratic input scaling extended input scaling subclause G.7 subclause G.8 subclause G.9 G.7 Linear input DC level shift and scaling No NLT marker Tnlt Tnlt NLT marker with NLT marker with =1 =2 Table G.6 provides guidance on how to implement an input scaling and input DC level shift at the encoder Input: side that is compatible to the output level shift and output scaling step specified in subclause G.3. i W H B R y x i i i Image sample values [
,
, ] computed according to subclause B.1, sample precisions ] of all components, rows and columns, dimensions of all components ] as indicated by the component c[ ] and [ c[ x i Output: table specified in subclause A.4.5. Ω y Intermediate sample [
,
, ] of all components, all rows and columns. © ISO/IEC 2024 – All rights reserved

<!-- page 98 -->

Table G.6 — Linear Input scaling and level shifting Syntax linear_input_scaling() { for(i=0;i<Nc;i=i+1) { ζ=Bw–B[i] for(y=0;y<Hc[i];y=y+1) { for(x=0;x<Wc[i];x=x+1) { v=R[i,x,y] v=v<<ζ v=v–((1<<Bw)>>1) Ω[i,x,y]=v } } } } Notes Compute scale value from the nominal bit precision of the wavelet coefficients signalled in the picture header and the component precision. Loop over rows of the component Loop over the columns of the component Retrieve input samples Extend the bit precision Apply DC level shift. Store scaled and level-shifted value End of loop over columns End of loop over rows End of loop over components G.8 Quadratic input DC level shift and scaling Table G.7 provides guidance on how to implement an input scaling and input DC level shift at the encoder Input: side that is compatible to the output level shift and output scaling step specified in subclause G.4. i W H B R y x i i i Image sample values [
,
, c[ ] and c[ ] computed according to subclause B.1, sample precisions ] of all components, rows and columns, dimensions of all components ] as indicated by the component is defined in the NLT marker specified in DCO [ table specified in subclause A.4.5. The DC offset value Output: subclause A.4.6. Ω y x i Intermediate sample Table G.7 — Quadratic input scaling and level shifting ] of all components, all rows and columns. [
,
, Syntax Notes quadratic_input_scaling() { for(i=0;i<Nc;i=i+1) { ζ=Bw–B[i] for(y=0;y<Hc[i];y=y+1) { for(x=0;x<Wc[i];x=x+1) { v=R[i,x,y] v=v−DCO v=clamp(v,0,(1<<B[i])−1) v=v<<ζ ρ=0 Compute scale value from the nominal bit precision of the wavelet coefficients signalled in the picture header and the component precision. Loop over rows of the component Loop over the columns of the component Retrieve input samples Apply variable DC level shift. Clamp to nominal input range Extend the bit precision for(σ=0;σ<Bw;σ=σ+1) { Intermediate result of the square root ρ=ρ<<1 v=v<<2 if((v>>Bw)>ρ) { v=v–((ρ+1)<<Bw) ρ=ρ+2 Iterate over output bits Shift the next bit in Shift the next bits in Next square available? Subtract partial root Next odd number © ISO/IEC 2024 – All rights reserved

<!-- page 99 -->

Table G.7 (continued) Table G.7 (continued) Syntax Notes } } v=ρ>>1 v=v−((1<<Bw)>>1) Ω[i,x,y]=v } } } } End of root adjustment End of loop over bits Compute final output of the square root Apply DC level shift. Store scaled and level-shifted value End of loop over columns End of loop over rows End of loop over components v Bw
## NOTE

v The above algorithm can produce intermediate results in that can exceed 32-bit precision. For a value of v × 2 =18, can obtain values as large as 2
. The algorithm in lines 8 to 17 computes the value of
. Other    B w    algorithms for computing the square root can be used as well. G.9 Extended input DC level shift and scaling Table G.8 provides guidance on how to implement an input scaling and input DC level shift at the encoder Input: side that is compatible to the output level shift and output scaling step specified in subclause G.5. i W H B R y x i i i Image sample values [
,
, ] computed according to subclause B.1, sample precisions ] of all components, rows and columns, dimensions of all components ] as indicated by the component c[ ] and [ c[ x i Output: table specified in subclause A.4.5. Ω y Intermediate sample Table G.8 — Extended input scaling and level shifting ] of all components, all rows and columns. [
,
, Syntax Notes extended_input_scaling() { for(i=0;i<Nc;i=i+1) { B2=T1×T1 A1=B2+(T1<<(Bw–E))+(1<<(2×Bw–2–2×E)) B1=T1+(1<<(Bw–E–1)) A3=B2+(T2<<(Bw–E))–(1<<(2×Bw–2–2×E)) B3=T2−(1<<(Bw–E–1)) Q1=B2+(T1<<(Bw–E)) Q2=B2+(T2<<(Bw–E)) ε=Bw–E ζ=2×Bw−B[i] for(y=0;y<Hc[i];y=y+1) { for(x=0;x<Wc[i];x=x+1) { v=R[i,x,y] v=v<<ζ if(v<Q1) { v = B1 − A1 v − © ISO/IEC 2024 – All rights reserved Compute linear region offset value Compute black region Compute black region A1 B1 offset A3 offset B3 Compute normal region offset Compute normal region offset Compute threshold of black region Compute threshold of linear domain Compute shift in linear region Compute shift to output range Loop over rows of the component Loop over the columns of the component Retrieve input samples Extend the bit precision Check for black region Compute gamma correction in black region

<!-- page 100 -->

Table G.8 (continued) Table G.8 (continued) Syntax Notes } else if(v<Q2) { v=(v−B2)>>ε } else { v = B3+ v A3− } v=v−((1<<Bw)>>1) Ω[i,x,y]=v } } } } Check for linear region Shift and scale to range in linear region Is instead in the regular region Compute gamma correction in regular region Apply DC level shift. Store scaled and level-shifted value End of loop over columns End of loop over rows End of loop over components v A1 B2 A3 Q1 Q2 Bw
## NOTE

=18, The above algorithm can produce intermediate results in v that can exceed 32-bit precision. For a value of
. An algorithm for computing the square root can be
, 2. This −1 and is limited from above by 2 can grow larger than 2 −1 T + v
,
, and
, taken from Table G.7. The output value of does not establish an error condition. G.10 Selection of the decoder thresholds T1 and T2 can obtain values as large as 2 Bw Bw Θ Θ T T Θ Θ B i
## 1 and

Θ
## 2 in the input domain of the encoder such that source signals between 0 and

## 1 and

## 2 in the NLT marker from

This subclause provides guidance on how to derive the threshold values thresholds Θ Θ ] −1 are gamma corrected with an exponent of ½, and input signals and the positive maximal amplitude 2
## 1 is the optical black value of the

between
## 2 is the toe-region of the gammainput signal, i.e. the signal level that maps to a total black sample, and

T T correction of the signal. Since the data in the NLT marker steers the decoding process, the threshold values Θ
## 2 are subject to a linear scaling. In typical applications,

## 1 and

## 1 and

[
## 1 and

## 2 indicated there apply in the gamma-corrected domain, and not in the source domain.

T T Θ Θ Bw
## 2 from the encoder input domain

The following formulae can be used to derive the decoder values is the nominal bit precision of the wavelet coefficients indicated in the picture thresholds header, see subclause A.4.4,
, indicated in the component table, see Bw subclause A.4.5. For the formulae below, it is assumed that the bit precisions of all components are identical. × × ] is the bit precision of the component [ ×
## 1 and

− [ ] Bw B i
## 1 and

i
## 2 2

− − × Bw E Θ − − 2. B T = + − E i ( Θ = T × × − [ ] Bw B i − T × T )
## 1 2

× E Bw − E E While the exponent value are source dependent and no generally valid advice on its selection can be given here. can be freely selected, =3 is a suggested default value. The values of Θ Θ
## 1 and

© ISO/IEC 2024 – All rights reserved

<!-- page 101 -->

## Annex H

(normative) Frame buffer H.1 General In this annex, the flowcharts and tables are normative only in the sense that they are defining an output that alternative implementations shall duplicate. This annex specifies the operation of the JPEG XS frame buffer, which holds a copy of all wavelet coefficients reconstructed from the previous frame in a stream of JPEG XS codestreams. not NOTE 1 Unlike many other standards, the JPEG XS frame buffer keeps copies of the wavelet coefficients, i.e. operates in the wavelet domain, and λ, in the spatial domain. x p p p b b b λ λ x x c f λ p ] ] wavelet coefficients ] which then further undergo inverse wavelet transformation and inverse multi-component The inverse temporal prediction computes from the reconstructed wavelet coefficient residuals x b c’ in all precincts
, decorrelation. λ and the frame buffer content and positions bands
, lines c’ p p b b λ x x c [ [ [ I I
,
,
,
,
,
,
,
, This inverse decorrelation is only effective for precincts within TDC enabling slices, i.e. whenever = 0, then ], i.e. JPEG XS operates without a frame buffer in intra-mode. ] is set to [ [
,
,
,
,
,
, sl > 0. If sl
## NOTE 2

In order to achieve a low-latency requirement and to conform to one or multiple profiles specified in ISO/IEC 21122-2, decoder implementations would need to run the output scaling, DC level shifting, the optional nonlinear transformation, the output clipping and the inverse temporal decorrelation specified in this annex interleaved with the inverse multiple component decorrelation transformation steps of Annex F. The algorithms given in this annex assume for the ease of presentation that all sample values of an image are available entirely. H.2 Inverse temporal decorrelation c p λ b x ’ The algorithm specified in Table H.1 reconstructs the wavelet coefficients c coefficient residuals [ Input: shall be performed on all slices of the image. ] of all precincts and the contents of the frame buffer t I
,
,
, p λ f
, [ b p
, x λ
,
, x b ] from the wavelet ]. This algorithm
,
, [ p Slice index
, coefficient residuals c[ Output: and frame buffer contents x
, and slice TDC flag λ p ], frame buffer quantization values
,
, [ p sl of the slice, see subclauses A.4.13 and A.4.15. Reconstructed wavelet ] b λ ] of all precincts, all lines, all bands for all positions. ], frame buffer refinement values b f p
, f[ f[ Q R p p b b λ λ x x x f ’
,
, Wavelet coefficients c [
,
,
, ], and updated frame buffer content [
,
,
, ]. © ISO/IEC 2024 – All rights reserved

<!-- page 102 -->

Table H.1 — Inverse temporal decorrelation Syntax inverse_temporal_decorrelation(t,Qf[p],Rf[p],Isl) { p=t×Np,x×Hsl for(u=0;u<Np[t];p=p+1,u=u+1) { if(Isl == 0) { intra_copy(p) } else { tdc_update(p,Q’f[p],R’f[p]) } update_framebuffer(p,Qf[p],Rf[p]) Q’f[p]=Qf[p] R’f[p]=Rf[p] } Q p R p Notes p t of the first pre-
H Compute the index cinct in the slice of a slice in precincts number of precincts per line from the height sl and the p,x. N Loop over the precincts in the slice x b c’ Check whether the slice disables TDC. p λ p c Copy the wavelet coefficients [
, [ ] to see subclause H.3. ] unchanged, b λ x
,
,
,
,
, x x λ b b p c’ λ p c Update the wavelet coefficients [
, frame buffer, see subclause H.4. ] to ] by using the [
,
,
,
,
, End of TDC/intra switch per slice. Update the contents of the frame buffer, see subclause H.5. Remember the frame buffer quantization parameter for the next iteration Remember the frame buffer refiment parameter for the next iteration By the above definition, the frame buffer contents is dequantized with the frame buffer quantization and ]) of the previous frame, and the updated frame buffer is then re-quantized with the ]) of the current frame received in the precinct header, see subclause C.2 ’f[ f[ R ], p p
## NOTE

refinement values ( f[ quantization and refinement values ( for the encoding of the precinct header. H.3 Precinct intra-copy ’f[ Q ], ’ p λ b x p λ b x The algorithm specified in Table H.2 copies the reconstructed wavelet coefficient residuals c[ the inversely temporally decorrelated wavelet coefficients c Input: account, and the precinct is reconstructed in intra-mode. ] to ], i.e. the frame buffer is not taken into p b λ x [
,
,
,
,
,
, Output: b λ Precinct index p. Reconstructed wavelet coefficient residuals c[ p ’ Inversely temporally decorrelated wavelet coefficients c
, Table H.2 — Precinct intra-copy [
,
,
, ] for the precinct p. x
,
, ]. Syntax Notes intra_copy(p) { for(s=0;s<Npc;s=s+1) { for(b=0;b<NL;b=b+1) { for(λ=L0[p,b];λ<L1[p,b];λ=λ+1) { if(I[p,λ,b,s]) { for(x=0;x<Wpb[p,b];x=x+1) { Loop over all packets in the precinct Loop over all bands Loop over all lines of this band Include only if the line is present in the given band and packet, see subclause B.7. p b Iterate over all coefficients of band precinct in © ISO/IEC 2024 – All rights reserved

<!-- page 103 -->

Table H.2 (continued) Table H.2 (continued) c’[p,λ,b,x]=c[p,λ,b,x] Syntax Notes } } } } } } H.4 Precinct TDC update Copy the reconstructed wavelet coefficients over. End of loop over one line of indices End of line is present. End of loop over lines End of loop over bands End of loop over packets x b λ p ’ The algorithm specified in Table H.3 derives the inversely temporally decorrelated wavelet coefficients x b λ p f
,
,
, [ c ] and the frame buffer contents [ ] from the reconstructed wavelet coefficient residuals c[ ] . p b λ x
,
,
,
,
,
, This document does not define the values in the frame buffer if no previous frame defined its content due
## NOTE 1

to the frame buffer update step specified in subclause H.5. A possible choice is set all data in the frame buffer to zero beforehand. A sequence of codestreams is ill-formed if decoding depends on the choice of the initial value of the frame buffer.
## NOTE 2

From the above follows that at least the initial frame of a series of well-formed codestreams encodes all its coefficients in intra mode without temporal prediction. Despite these restrictions, it can happen in practical applications that a JPEG XS decoder is instructed to decode a particular frame without having the initial values of the frame buffer available, for example when switching into an already established connection. Despite this requirement, decoders should handle such ill-formed code-stream sequences gracefully, and encoders should prepare codestream sequences in such a way that every wavelet coefficient undergoes an Input: intra_copy() step once in a while. p q r Precinct index x b p λ f
, frame buffer quantization parameter x Y p p λ the data in the frame buffer and slice TDC flag Output: ] for the precinct content
, TDC selection flags [
,
,
, I b p sl. Wavelet coefficient residuals b ’ ], TDC mode
, p λ x [ [
,
,
, λ
, frame buffer refinement parameter
, of ] and frame buffer Di p b x
,
, c b [ ]. Inversely temporally decorrelated wavelet coefficients c
, Table H.3 — Precinct TDC update [
,
, ]. Syntax Notes tdc_update(p,q,r) { Qad=max(Qbi,Qbr,0) for(s=0;s<Npc;s=s+1) { for(b=0;b<NL;b=b+1) { for(λ=L0[p,b];λ<L1[p,b];λ=λ+1) { if(I[p,λ,b,s]) { for(x=0;x<Wpb[p,b];x=x+1) { Compute maximal quantization adjustment Loop over all packets in the precinct Loop over all bands Loop over all lines of this band Include only if the line is present in the given band and packet, see subclause B.7. b p =     x S Ni × g     k
if(Di[p,b]==0 || Di[p,b]==1 || (Di[p,b]==3 && Y[p,λ,b,k]==0)) { Iterate over all coefficients of band precinct x in Compute the TDC selection group index from the coefficient position
. Check whether the coefficient is intra-coded due to rate decisions of the encoder © ISO/IEC 2024 – All rights reserved

<!-- page 104 -->

Table H.3 (continued) Table H.3 (continued) Syntax Notes c’[p,λ,b,x]=c[p,λ,b,x]<<(Qad−Qbi) } else if(Sh[b]≥0 && ((k-Yh[b]) umod 2Sh[b])==0) { c’[p,λ,b,x]=c[p,λ,b,x]<<(Qad−Qbr) } else { T = compute_truncation(b,q,r,0) f’ = f[p,λ,b,x]<<T c’[p,λ,b,x]=(c[p,λ,b,x]<<Qad)+f’ } } } } } } } H.5 Frame buffer update ’ p λ b x Q Reconstruct including the intra adjustment, see Annex A.4.10 for the definition of bi. Check whether the coefficient position matches the position hash Reconstruct including the refresh adjust-
Q ment, see Annex A.4.10 for the definition of br. Compute truncation position from frame buffer refinement and quantization, see subclause C.6.2 for details. Q p Dequantize the frame buffer content according to the frame buffer quantization signalled in the precinct header. f[ ] Inversely predict the wavelet coefficient from the reconstructed residual value by means of the frame buffer. End of loop over one line of indices End of line is present. End of loop over lines End of loop over bands End of loop over packets The algorithm specified in Table H.4 updates the contents of the frame buffer to the most recently inversely temporally predicted wavelet coefficients c ] and by that prepares the contents of the frame buffer for the next frame. [
,
,
,
## NOTE

Input: subclass need not to be performed as the frame buffer content is never used. If a decoder implements only intra-coding according to ISO/IEC 21122-1:2022, the steps defined in this r p q ’ p λ b x p Precinct index Output: Inversely temporally predicted wavelet coefficients c
, frame buffer quantization parameter λ and frame buffer refinement parameter x b p ] for the precinct f
, [
,
,
.
. Updated frame buffer contents for the next frame Table H.4 — Frame buffer update ]. [
,
,
, Syntax Notes update_framebuffer(p,q,r) { for(s=0;s<Npc;s=s+1) { for(b=0;b<NL;b=b+1) { for(λ=L0[p,b];λ<L1[p,b];λ=λ+1) { if(I[p,λ,b,s]) { for(x=0;x<Wpb[p,b];x=x+1) { Loop over all packets in the precinct Loop over all bands Loop over all lines of this band Include only if the line is present in the given band and packet, see subclause B.7. b p Iterate over all coefficients of band precinct in © ISO/IEC 2024 – All rights reserved

<!-- page 105 -->

Table H.4 (continued) Table H.4 (continued) Syntax Notes T = compute_truncation(b,q,r,0) if(c’[p,λ,b,x]>=0){ f[p,λ,b,x]=c’[p,λ,b,x]>>T } else { f[p,λ,b,x]=−(−c’[p,λ,b,x])>>T } } } } } } H.6 Temporal frame decorrelation Compute truncation position from frame buffer refinement and quantization, see also subclause C.6.2. Check the sign of the wavelet coefficient, non-negative case Quantize and copy the inversely temporally predicted wavelet coefficients into the frame buffer Check the sign of the wavelet coefficient, negative case Quantize and copy the inversely temporally predicted wavelet coefficients into the frame buffer End of loop over one line of indices End of line is present. End of loop over lines End of loop over bands End of loop over packets This subclause provides guidance on how to implement a temporal decorrelation algorithm that generates c’ an output that is compatible to the decoder algorithm specified in subclause H.2. x c f The algorithm specified in Table H.5 takes the wavelet coefficients x f ] and computes from them wavelet coefficient residuals [ ], which is then itself quantized. [ ], and the frame buffer contents ] and updates the frame buffer x b λ p x b λ p
,
,
, [ [ p p
,
, b b
,
, λ λ
,
,
,
,
, Q p R p D p b
## NOTE

This International Standard does not provide guidance on how to arrive at quantization values for the I wavelet coefficients or frame buffer quantization ] and sl. Any choice is acceptable as long as the resulting codestream conforms to the requirements of this document and Input: ISO/IEC 21122-2. x b λ p Slice index Output: c’[
,
, p sl of the slice, see subclauses A.4.13 and A.4.15. Wavelet coefficients [ I f
, and TDC mode ] and frame buffer contents x ] of all precincts, all lines, all bands for all positions. ] and refinement values ], or at the TDC mode p
, b
, λ
, f[ f[ p b b i[ λ λ x x t f
,
, Wavelet coefficient residuals c[ Table H.5 — TDC decorrelation ], and updated frame buffer content
,
,
, [
,
,
, ]. tdc_decorrelation(t,Isl) { Syntax p=t×Np,x×Hsl for(u=0;u<Np[t];p=p+1,u=u+1) { Di[p,b]=select_tdc_mode(p,b) if(Di[p,b]==3) { for(b=0;b<NL;b=b+1) { for(λ=L0[p,b];λ<L1[p,b];λ=λ+1) { Notes p t Compute the index from the height of a slice in precincts number of precincts per line N H of the first precinct in the slice sl and the p p,x. b Loop over the precincts in the slice Select the TDC mode for band in precinct
. Check whether TDC selection flags are required. Loop over all bands Loop over all lines of this band © ISO/IEC 2024 – All rights reserved

<!-- page 106 -->

Table H.5 (continued) Table H.5 (continued) Syntax Notes if(I[p,λ,b,s]) { for(k=0;k<Ni[p,b];k=k+1) { Y[p,b,λ,k]=select_tdc(p,b,λ,k) } } } } } quantize_residuals(p) (Qf[p],Rf[p])=compute_fb_qr(p) dequantize_residuals(p) if(Isl == 0) { intra_copy(p) } else { tdc_update(p,Qf[p],Rf[p]) update_framebuffer(p,Qf[p],Rf[p]) } } Include only if the line is present in the given band and packet, see subclause B.7. b p Iterate over all TDC selection groups of band precinct in Select the TDC selection group decision for TDC or intra coding. End of loop over all TDC selection groups End of check whether band exists End of loop over all lines End of loop over all bands End of test whether TDC selection flags are enabled. Run the quantization algorithm from subclause D.4 or subclause D.5 according to the quantization parameters and the quantizer selected in the picture header. Determine a refinement and quantization parameter for the frame buffer. This parameter is repre-
sented in the precinct header, see subclause C.2. This document does not specify this algorithm, and any choice is possible as long as the resulting code stream is compliant to ISO/IEC 21122-2. Run the dequantization algorithm from subclause D.2 or subclause D.3 according to the quantization parameters in the picture header. x After applying this algorithm, the wavelet coefficient residuals c[
, er would arrive at. ] mirror the values a decod-
p b λ
,
, Check whether the slice is intra-predicted c p λ b x c’ p λ b x Copy the wavelet coefficients I unchanged, see subclause H.3. Otherwise, if λ p c’ b x sl == 1 [
,
,
, ] to [
,
,
, ] c p λ b x Update the wavelet coefficients to
,
, subclause H.4. [
, ] by using the frame buffer, see [
,
,
, ] Update the contents of the frame buffer, see subclause H.5. End of TDC/intra switch per slice. End of inverse frame buffer decorrelation. H.7 Computation of wavelet coefficient residuals within a precinct p λ b x f p λ b x p λ b x
, ] from the frame buffer contents The algorithm specified in Table H.6 provides guidance on how to compute the wavelet coefficient residuals ] such that the
,
, c[ λ p Input: residuals are compatible to the algorithm specified in subclause H.4. Di x b p Y Precinct index Output: ], TDC mode
,
, p sl. Wavelet coefficients and slice TDC flag λ p ] ], frame buffer quantization [ ] and the wavelet coefficients c’[ b
, and refinement R’f[p] ] for the precinct of the previous frame.
, TDC selection flags Q’f[p]
, p c’ p b b λ x x [ [ [ I
,
,
,
,
,
,
,
,
, Wavelet coefficient residuals c[
,
,
, ]. © ISO/IEC 2024 – All rights reserved

<!-- page 107 -->

Table H.6 — Precinct temporal predict Syntax Notes temporal_predict(p,Q’f[p],R’f[p]) { Qad=max(Qbi,Qbr,0) for(s=0;s<Npc;s=s+1) { for(b=0;b<NL;b=b+1) { for(λ=L0[p,b];λ<L1[p,b];λ=λ+1) { T = compute_truncation(b,Q’f[p],R’f[p],0) if(I[p,λ,b,s]) { for(x=0;x<Wpb[p,b];x=x+1) { =     x S Ni × g     k
if(Di[p,b]==0 || Di[p,b]==1 || (Di[p,b]==3 && Y[p,λ,b,k]==0)) { c[p,λ,b,x]=c’[p,λ,b,x]>>(Qad−Qbi) } else if(Sh[b]≥0 && ((k-Yh[b]) umod 2Sh[b]) == 0) { c[p,λ,b,x]=c’[p,λ,b,x]>>(Qad−Qbr) } else { f’ = f[p,λ,b,x]<<T c[p,λ,b,x]=(c’[p,λ,b,x]-f’ )>>Qad } } } } } } } H.8 Selecting TDC modes Compute maximal quantization adjustment Loop over all packets in the precinct Loop over all bands Loop over all lines of this band Compute truncation position from frame buffer refinement and quantization, see subclause C.6.2 for details. Include only if the line is present in the given band and packet, see subclause B.7. p b Iterate over all coefficients of band precinct in Compute the TDC selection group index from the coefficient position x. Check whether the coefficient is intra-coded due to rate decisions of the encoder Check whether the coefficient position matches the position hash For intra refresh, copy the wavelet coefficient to the wavelet coefficient residual with a quantization adjustment, Q see Annex A.4.10 for the definition of br. p Q Dequantize the frame buffer content according to the frame buffer quanti-
] signalled in the precinct zation header. f[ Predict the wavelet coefficent from the frame buffer content and the wavelet coefficient. End of loop over one line of indices End of line is present. End of loop over lines End of loop over bands End of loop over packets select_tdc_mode() This subclause provides guidance on how to make a decision between the intra and TDC modes per band or per TDC selection group as required for the steps. © ISO/IEC 2024 – All rights reserved

<!-- page 108 -->

While no normative requirement is given how such a decision should be made, the following observations should be taken into consideration: — A possible decision criterion between TDC and intra coding can be derived from the sum of bitplane counts of the wavelet coefficient residuals c’[p,λ,b,x] in TDC or intra mode. In general, the decision leading to the lower sum of bitplane counts provides better coding efficiency. For best performance, it is Q beneficial to compute the sum of bitplane counts for making this decision without taking the values of S ], br of the TDC marker and the gradual refresh mechanism due to the positional hash values h[ Q Y b B b bi and h[ ] into account. — It can happen that a candidate wavelet coefficient residual c[p,λ,b,x] has a bitplane count of 2 r or larger (16 in most configurations). As the entropy coding steps outlined in Annex C cannot encode such values without significant error, it is advisable to select intra coding for such bands or such TDC selection groups where this condition arises. — Despite selecting the coding mode providing the better coding efficiency, it is advisable to encode each coefficient peridically in an intra-coding mode to ensure that a decoder that did not receive the intial codestream of a sequence of codestreams is able to fully recover and provide a faithful reconstruction despite missing the initial value of the frame buffer. The TPC marker specified in subclause A.4.10 provides a syntax to implement such a mechanism. Such refresh should be rotated throughout all bands, precincts, slices or TDC selection groups of the frame. H.9 Frame buffer quantization p R compute_fb_qr(p) This subclause provides guidance on the frame buffer quantization f[ ] and frame buffer refinement value f[ ] and by that on the algorithm referenced in subclause H.6. Q p p The purpose of this quantization is to limit the transmission rate from an encoder or decoder core to external memory implementing the frame buffer by reducing the number of bits that need to be transmitted to such an external device. An encoder should mirror the fill state of a (real or emulated) output buffer to the R frame buffer and should adjust the frame buffer quantization value ] and frame buffer refinement value ] such that the number of transmitted bits from the encoder core plus the available rate in the output R buffer does not exceed the capacity of the link to external frame buffer memory. This can be achieved by iterating over all possible ( ]) pairs and selecting the pair that provides the largest possible rate not exceeding the threshold. f[ f[ f[ f[ Q Q Q R ], p p p p p Additional normative requirements on ( f[ ], f[ ]) are specified in ISO/IEC 21122-2. © ISO/IEC 2024 – All rights reserved

<!-- page 109 -->

## Annex I

(informative) Example weight tables Table I.1 to Table I.11 lists example configurations for the weights table specified in subclause A.4.12 that have been optimized for PSNR performance of the encoder. Other choices are possible, and can result in improved visual quality for a certain viewing distance. Table I.1 — Gains and weights for 4:4:4 sampling, RCT enabled, 5 horizontal and 0 vertical decomposition levels Band b Gain G[b] Priority P[b] Table I.2 — Gains and weights for 4:4:4 sampling, RCT enabled, 5 horizontal and 1 vertical decomposition levels Band b Gain G[b] Priority P[b] © ISO/IEC 2024 – All rights reserved

<!-- page 110 -->

Table I.2 (continued) Table I.2 (continued) Band b Gain G[b] Priority P[b] Table I.3 — Gains and weights for 4:4:4 sampling, RCT enabled, 5 horizontal and 2 vertical decomposition levels Band b Gain G[b] Priority P[b] © ISO/IEC 2024 – All rights reserved

<!-- page 111 -->

Table I.3 (continued) Table I.3 (continued) Band b Gain G[b] Priority P[b] Table I.4 — Gains and weights for 4:2:2 sampling, RCT disabled, 5 horizontal and 0 vertical decomposition levels Band b Gain G[b] Priority P[b] Table I.5 — Gains and weights for 4:2:2 sampling, RCT disabled, 5 horizontal and 1 vertical decomposition levels Band b Gain G[b] Priority P[b] © ISO/IEC 2024 – All rights reserved

<!-- page 112 -->

Table I.5 (continued) Table I.5 (continued) Band b Gain G[b] Priority P[b] Table I.6 — Gains and weights for 4:2:2 sampling, RCT disabled, 5 horizontal and 2 vertical decomposition levels Band b Gain G[b] Priority P[b] © ISO/IEC 2024 – All rights reserved

<!-- page 113 -->

Table I.7 — Gains and weights for 4:2:0 sampling, RCT disabled, 5 horizontal and 1 vertical decomposition levels Band b Gain G[b] Priority P[b]
- 
- 
- 
- 
- 
- 
- 
- 
- 
- 
- 
- 
- 
- 
- 
- 
- 
Bands indicated by do not exist and gains and priorities of them are not included in the Weights table (see subclause A.4.12). Table I.8 — Gains and weights for 4:2:0 sampling, RCT disabled, 5 horizontal and 2 vertical decomposition levels Band b Gain G[b] Priority P[b] Bands indicated by
- 
do not exist and gains and priorities of them are not included in the Weights table (see subclause A.4.12). © ISO/IEC 2024 – All rights reserved

<!-- page 114 -->

Table I.8 (continued) Table I.8 (continued) Band b Gain G[b] Priority P[b]
- 
- 
- 
- 
- 
- 
- 
- 
- 
- 
- 
- 
- 
- 
- 
- 
- 
Bands indicated by do not exist and gains and priorities of them are not included in the Weights table (see subclause A.4.12). Table I.9 — Gains and weights for CFA sample compression with the star-tetrix transformation, Sd=1, 5 horizontal and 0 vertical decomposition levels Band b Cf=0 Cf=3 Gain G[b] Priority P[b] Gain G[b] Priority P[b] © ISO/IEC 2024 – All rights reserved

<!-- page 115 -->

Table I.10 — Gains and weights for CFA sample compression with the star-tetrix transformation, Sd=1, 5 horizontal and 1 vertical decomposition levels Band b Cf=0 Cf=3 Gain G[b] Priority P[b] Gain G[b] Priority P[b] Table I.11 — Gains and weights for CFA sample compression with the star-tetrix transformation, Sd=1, 5 horizontal and 2 vertical decomposition levels Band b Cf=0 Cf=3 Gain G[b] Priority P[b] Gain G[b] Priority P[b] © ISO/IEC 2024 – All rights reserved

<!-- page 116 -->

Table I.11 (continued) Table I.11 (continued) Band b Cf=0 Cf=3 Gain G[b] Priority P[b] Gain G[b] Priority P[b] © ISO/IEC 2024 – All rights reserved

<!-- page 117 -->

Bibliography Information technology — Universal coded character set (UCS) [1] ISO/IEC 10646, © ISO/IEC 2024 – All rights reserved

<!-- page 118 -->

<!-- page 119 -->

ICS 35.040.30; 35.040.40 Price based on 114 pages © ISO/IEC 2024 All rights reserved iso.org Tm9ybWVuLURvd25sb2FkLURJTiBNZWRpYS1BdmlzZW5uYSBFRFYgdW5kIE1hcmtldGluZy1LZE5y Ljg0NzE5NTgtTGZOci4xMTU5MTMxNDAwMS0yMDI1LTA5LTA2IDIwOjA2 Normen-Download-DIN Media-Avisenna EDV und Marketing-KdNr.8471958-LfNr.11591314001-2025-09-06 20:06

<!-- page 120 -->