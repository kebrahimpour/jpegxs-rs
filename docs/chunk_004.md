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