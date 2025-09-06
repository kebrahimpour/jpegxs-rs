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