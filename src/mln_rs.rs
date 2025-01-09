use ::libc;
use ::c2rust_bitfields;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn __errno_location() -> *mut libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type __uint8_t = libc::c_uchar;
pub type size_t = libc::c_ulong;
pub type mln_u8_t = libc::c_uchar;
pub type mln_u32_t = libc::c_uint;
pub type mln_s32_t = libc::c_int;
pub type mln_u8ptr_t = *mut libc::c_uchar;
pub type mln_size_t = size_t;
pub type uint8_t = __uint8_t;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct mln_rs_matrix_t {
    pub row: mln_size_t,
    pub col: mln_size_t,
    pub data: mln_u8ptr_t,
    #[bitfield(name = "is_ref", ty = "mln_u32_t", bits = "0..=0")]
    pub is_ref: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mln_rs_result_t {
    pub data: mln_u8ptr_t,
    pub len: mln_size_t,
    pub num: mln_size_t,
}
static mut mln_rs_gfilog: [mln_u8_t; 256] = [
    1 as libc::c_int as mln_u8_t,
    2 as libc::c_int as mln_u8_t,
    4 as libc::c_int as mln_u8_t,
    8 as libc::c_int as mln_u8_t,
    16 as libc::c_int as mln_u8_t,
    32 as libc::c_int as mln_u8_t,
    64 as libc::c_int as mln_u8_t,
    128 as libc::c_int as mln_u8_t,
    45 as libc::c_int as mln_u8_t,
    90 as libc::c_int as mln_u8_t,
    180 as libc::c_int as mln_u8_t,
    69 as libc::c_int as mln_u8_t,
    138 as libc::c_int as mln_u8_t,
    57 as libc::c_int as mln_u8_t,
    114 as libc::c_int as mln_u8_t,
    228 as libc::c_int as mln_u8_t,
    229 as libc::c_int as mln_u8_t,
    231 as libc::c_int as mln_u8_t,
    227 as libc::c_int as mln_u8_t,
    235 as libc::c_int as mln_u8_t,
    251 as libc::c_int as mln_u8_t,
    219 as libc::c_int as mln_u8_t,
    155 as libc::c_int as mln_u8_t,
    27 as libc::c_int as mln_u8_t,
    54 as libc::c_int as mln_u8_t,
    108 as libc::c_int as mln_u8_t,
    216 as libc::c_int as mln_u8_t,
    157 as libc::c_int as mln_u8_t,
    23 as libc::c_int as mln_u8_t,
    46 as libc::c_int as mln_u8_t,
    92 as libc::c_int as mln_u8_t,
    184 as libc::c_int as mln_u8_t,
    93 as libc::c_int as mln_u8_t,
    186 as libc::c_int as mln_u8_t,
    89 as libc::c_int as mln_u8_t,
    178 as libc::c_int as mln_u8_t,
    73 as libc::c_int as mln_u8_t,
    146 as libc::c_int as mln_u8_t,
    9 as libc::c_int as mln_u8_t,
    18 as libc::c_int as mln_u8_t,
    36 as libc::c_int as mln_u8_t,
    72 as libc::c_int as mln_u8_t,
    144 as libc::c_int as mln_u8_t,
    13 as libc::c_int as mln_u8_t,
    26 as libc::c_int as mln_u8_t,
    52 as libc::c_int as mln_u8_t,
    104 as libc::c_int as mln_u8_t,
    208 as libc::c_int as mln_u8_t,
    141 as libc::c_int as mln_u8_t,
    55 as libc::c_int as mln_u8_t,
    110 as libc::c_int as mln_u8_t,
    220 as libc::c_int as mln_u8_t,
    149 as libc::c_int as mln_u8_t,
    7 as libc::c_int as mln_u8_t,
    14 as libc::c_int as mln_u8_t,
    28 as libc::c_int as mln_u8_t,
    56 as libc::c_int as mln_u8_t,
    112 as libc::c_int as mln_u8_t,
    224 as libc::c_int as mln_u8_t,
    237 as libc::c_int as mln_u8_t,
    247 as libc::c_int as mln_u8_t,
    195 as libc::c_int as mln_u8_t,
    171 as libc::c_int as mln_u8_t,
    123 as libc::c_int as mln_u8_t,
    246 as libc::c_int as mln_u8_t,
    193 as libc::c_int as mln_u8_t,
    175 as libc::c_int as mln_u8_t,
    115 as libc::c_int as mln_u8_t,
    230 as libc::c_int as mln_u8_t,
    225 as libc::c_int as mln_u8_t,
    239 as libc::c_int as mln_u8_t,
    243 as libc::c_int as mln_u8_t,
    203 as libc::c_int as mln_u8_t,
    187 as libc::c_int as mln_u8_t,
    91 as libc::c_int as mln_u8_t,
    182 as libc::c_int as mln_u8_t,
    65 as libc::c_int as mln_u8_t,
    130 as libc::c_int as mln_u8_t,
    41 as libc::c_int as mln_u8_t,
    82 as libc::c_int as mln_u8_t,
    164 as libc::c_int as mln_u8_t,
    101 as libc::c_int as mln_u8_t,
    202 as libc::c_int as mln_u8_t,
    185 as libc::c_int as mln_u8_t,
    95 as libc::c_int as mln_u8_t,
    190 as libc::c_int as mln_u8_t,
    81 as libc::c_int as mln_u8_t,
    162 as libc::c_int as mln_u8_t,
    105 as libc::c_int as mln_u8_t,
    210 as libc::c_int as mln_u8_t,
    137 as libc::c_int as mln_u8_t,
    63 as libc::c_int as mln_u8_t,
    126 as libc::c_int as mln_u8_t,
    252 as libc::c_int as mln_u8_t,
    213 as libc::c_int as mln_u8_t,
    135 as libc::c_int as mln_u8_t,
    35 as libc::c_int as mln_u8_t,
    70 as libc::c_int as mln_u8_t,
    140 as libc::c_int as mln_u8_t,
    53 as libc::c_int as mln_u8_t,
    106 as libc::c_int as mln_u8_t,
    212 as libc::c_int as mln_u8_t,
    133 as libc::c_int as mln_u8_t,
    39 as libc::c_int as mln_u8_t,
    78 as libc::c_int as mln_u8_t,
    156 as libc::c_int as mln_u8_t,
    21 as libc::c_int as mln_u8_t,
    42 as libc::c_int as mln_u8_t,
    84 as libc::c_int as mln_u8_t,
    168 as libc::c_int as mln_u8_t,
    125 as libc::c_int as mln_u8_t,
    250 as libc::c_int as mln_u8_t,
    217 as libc::c_int as mln_u8_t,
    159 as libc::c_int as mln_u8_t,
    19 as libc::c_int as mln_u8_t,
    38 as libc::c_int as mln_u8_t,
    76 as libc::c_int as mln_u8_t,
    152 as libc::c_int as mln_u8_t,
    29 as libc::c_int as mln_u8_t,
    58 as libc::c_int as mln_u8_t,
    116 as libc::c_int as mln_u8_t,
    232 as libc::c_int as mln_u8_t,
    253 as libc::c_int as mln_u8_t,
    215 as libc::c_int as mln_u8_t,
    131 as libc::c_int as mln_u8_t,
    43 as libc::c_int as mln_u8_t,
    86 as libc::c_int as mln_u8_t,
    172 as libc::c_int as mln_u8_t,
    117 as libc::c_int as mln_u8_t,
    234 as libc::c_int as mln_u8_t,
    249 as libc::c_int as mln_u8_t,
    223 as libc::c_int as mln_u8_t,
    147 as libc::c_int as mln_u8_t,
    11 as libc::c_int as mln_u8_t,
    22 as libc::c_int as mln_u8_t,
    44 as libc::c_int as mln_u8_t,
    88 as libc::c_int as mln_u8_t,
    176 as libc::c_int as mln_u8_t,
    77 as libc::c_int as mln_u8_t,
    154 as libc::c_int as mln_u8_t,
    25 as libc::c_int as mln_u8_t,
    50 as libc::c_int as mln_u8_t,
    100 as libc::c_int as mln_u8_t,
    200 as libc::c_int as mln_u8_t,
    189 as libc::c_int as mln_u8_t,
    87 as libc::c_int as mln_u8_t,
    174 as libc::c_int as mln_u8_t,
    113 as libc::c_int as mln_u8_t,
    226 as libc::c_int as mln_u8_t,
    233 as libc::c_int as mln_u8_t,
    255 as libc::c_int as mln_u8_t,
    211 as libc::c_int as mln_u8_t,
    139 as libc::c_int as mln_u8_t,
    59 as libc::c_int as mln_u8_t,
    118 as libc::c_int as mln_u8_t,
    236 as libc::c_int as mln_u8_t,
    245 as libc::c_int as mln_u8_t,
    199 as libc::c_int as mln_u8_t,
    163 as libc::c_int as mln_u8_t,
    107 as libc::c_int as mln_u8_t,
    214 as libc::c_int as mln_u8_t,
    129 as libc::c_int as mln_u8_t,
    47 as libc::c_int as mln_u8_t,
    94 as libc::c_int as mln_u8_t,
    188 as libc::c_int as mln_u8_t,
    85 as libc::c_int as mln_u8_t,
    170 as libc::c_int as mln_u8_t,
    121 as libc::c_int as mln_u8_t,
    242 as libc::c_int as mln_u8_t,
    201 as libc::c_int as mln_u8_t,
    191 as libc::c_int as mln_u8_t,
    83 as libc::c_int as mln_u8_t,
    166 as libc::c_int as mln_u8_t,
    97 as libc::c_int as mln_u8_t,
    194 as libc::c_int as mln_u8_t,
    169 as libc::c_int as mln_u8_t,
    127 as libc::c_int as mln_u8_t,
    254 as libc::c_int as mln_u8_t,
    209 as libc::c_int as mln_u8_t,
    143 as libc::c_int as mln_u8_t,
    51 as libc::c_int as mln_u8_t,
    102 as libc::c_int as mln_u8_t,
    204 as libc::c_int as mln_u8_t,
    181 as libc::c_int as mln_u8_t,
    71 as libc::c_int as mln_u8_t,
    142 as libc::c_int as mln_u8_t,
    49 as libc::c_int as mln_u8_t,
    98 as libc::c_int as mln_u8_t,
    196 as libc::c_int as mln_u8_t,
    165 as libc::c_int as mln_u8_t,
    103 as libc::c_int as mln_u8_t,
    206 as libc::c_int as mln_u8_t,
    177 as libc::c_int as mln_u8_t,
    79 as libc::c_int as mln_u8_t,
    158 as libc::c_int as mln_u8_t,
    17 as libc::c_int as mln_u8_t,
    34 as libc::c_int as mln_u8_t,
    68 as libc::c_int as mln_u8_t,
    136 as libc::c_int as mln_u8_t,
    61 as libc::c_int as mln_u8_t,
    122 as libc::c_int as mln_u8_t,
    244 as libc::c_int as mln_u8_t,
    197 as libc::c_int as mln_u8_t,
    167 as libc::c_int as mln_u8_t,
    99 as libc::c_int as mln_u8_t,
    198 as libc::c_int as mln_u8_t,
    161 as libc::c_int as mln_u8_t,
    111 as libc::c_int as mln_u8_t,
    222 as libc::c_int as mln_u8_t,
    145 as libc::c_int as mln_u8_t,
    15 as libc::c_int as mln_u8_t,
    30 as libc::c_int as mln_u8_t,
    60 as libc::c_int as mln_u8_t,
    120 as libc::c_int as mln_u8_t,
    240 as libc::c_int as mln_u8_t,
    205 as libc::c_int as mln_u8_t,
    183 as libc::c_int as mln_u8_t,
    67 as libc::c_int as mln_u8_t,
    134 as libc::c_int as mln_u8_t,
    33 as libc::c_int as mln_u8_t,
    66 as libc::c_int as mln_u8_t,
    132 as libc::c_int as mln_u8_t,
    37 as libc::c_int as mln_u8_t,
    74 as libc::c_int as mln_u8_t,
    148 as libc::c_int as mln_u8_t,
    5 as libc::c_int as mln_u8_t,
    10 as libc::c_int as mln_u8_t,
    20 as libc::c_int as mln_u8_t,
    40 as libc::c_int as mln_u8_t,
    80 as libc::c_int as mln_u8_t,
    160 as libc::c_int as mln_u8_t,
    109 as libc::c_int as mln_u8_t,
    218 as libc::c_int as mln_u8_t,
    153 as libc::c_int as mln_u8_t,
    31 as libc::c_int as mln_u8_t,
    62 as libc::c_int as mln_u8_t,
    124 as libc::c_int as mln_u8_t,
    248 as libc::c_int as mln_u8_t,
    221 as libc::c_int as mln_u8_t,
    151 as libc::c_int as mln_u8_t,
    3 as libc::c_int as mln_u8_t,
    6 as libc::c_int as mln_u8_t,
    12 as libc::c_int as mln_u8_t,
    24 as libc::c_int as mln_u8_t,
    48 as libc::c_int as mln_u8_t,
    96 as libc::c_int as mln_u8_t,
    192 as libc::c_int as mln_u8_t,
    173 as libc::c_int as mln_u8_t,
    119 as libc::c_int as mln_u8_t,
    238 as libc::c_int as mln_u8_t,
    241 as libc::c_int as mln_u8_t,
    207 as libc::c_int as mln_u8_t,
    179 as libc::c_int as mln_u8_t,
    75 as libc::c_int as mln_u8_t,
    150 as libc::c_int as mln_u8_t,
    0 as libc::c_int as mln_u8_t,
];
static mut mln_rs_gflog: [mln_u8_t; 256] = [
    255 as libc::c_int as mln_u8_t,
    0 as libc::c_int as mln_u8_t,
    1 as libc::c_int as mln_u8_t,
    240 as libc::c_int as mln_u8_t,
    2 as libc::c_int as mln_u8_t,
    225 as libc::c_int as mln_u8_t,
    241 as libc::c_int as mln_u8_t,
    53 as libc::c_int as mln_u8_t,
    3 as libc::c_int as mln_u8_t,
    38 as libc::c_int as mln_u8_t,
    226 as libc::c_int as mln_u8_t,
    133 as libc::c_int as mln_u8_t,
    242 as libc::c_int as mln_u8_t,
    43 as libc::c_int as mln_u8_t,
    54 as libc::c_int as mln_u8_t,
    210 as libc::c_int as mln_u8_t,
    4 as libc::c_int as mln_u8_t,
    195 as libc::c_int as mln_u8_t,
    39 as libc::c_int as mln_u8_t,
    114 as libc::c_int as mln_u8_t,
    227 as libc::c_int as mln_u8_t,
    106 as libc::c_int as mln_u8_t,
    134 as libc::c_int as mln_u8_t,
    28 as libc::c_int as mln_u8_t,
    243 as libc::c_int as mln_u8_t,
    140 as libc::c_int as mln_u8_t,
    44 as libc::c_int as mln_u8_t,
    23 as libc::c_int as mln_u8_t,
    55 as libc::c_int as mln_u8_t,
    118 as libc::c_int as mln_u8_t,
    211 as libc::c_int as mln_u8_t,
    234 as libc::c_int as mln_u8_t,
    5 as libc::c_int as mln_u8_t,
    219 as libc::c_int as mln_u8_t,
    196 as libc::c_int as mln_u8_t,
    96 as libc::c_int as mln_u8_t,
    40 as libc::c_int as mln_u8_t,
    222 as libc::c_int as mln_u8_t,
    115 as libc::c_int as mln_u8_t,
    103 as libc::c_int as mln_u8_t,
    228 as libc::c_int as mln_u8_t,
    78 as libc::c_int as mln_u8_t,
    107 as libc::c_int as mln_u8_t,
    125 as libc::c_int as mln_u8_t,
    135 as libc::c_int as mln_u8_t,
    8 as libc::c_int as mln_u8_t,
    29 as libc::c_int as mln_u8_t,
    162 as libc::c_int as mln_u8_t,
    244 as libc::c_int as mln_u8_t,
    186 as libc::c_int as mln_u8_t,
    141 as libc::c_int as mln_u8_t,
    180 as libc::c_int as mln_u8_t,
    45 as libc::c_int as mln_u8_t,
    99 as libc::c_int as mln_u8_t,
    24 as libc::c_int as mln_u8_t,
    49 as libc::c_int as mln_u8_t,
    56 as libc::c_int as mln_u8_t,
    13 as libc::c_int as mln_u8_t,
    119 as libc::c_int as mln_u8_t,
    153 as libc::c_int as mln_u8_t,
    212 as libc::c_int as mln_u8_t,
    199 as libc::c_int as mln_u8_t,
    235 as libc::c_int as mln_u8_t,
    91 as libc::c_int as mln_u8_t,
    6 as libc::c_int as mln_u8_t,
    76 as libc::c_int as mln_u8_t,
    220 as libc::c_int as mln_u8_t,
    217 as libc::c_int as mln_u8_t,
    197 as libc::c_int as mln_u8_t,
    11 as libc::c_int as mln_u8_t,
    97 as libc::c_int as mln_u8_t,
    184 as libc::c_int as mln_u8_t,
    41 as libc::c_int as mln_u8_t,
    36 as libc::c_int as mln_u8_t,
    223 as libc::c_int as mln_u8_t,
    253 as libc::c_int as mln_u8_t,
    116 as libc::c_int as mln_u8_t,
    138 as libc::c_int as mln_u8_t,
    104 as libc::c_int as mln_u8_t,
    193 as libc::c_int as mln_u8_t,
    229 as libc::c_int as mln_u8_t,
    86 as libc::c_int as mln_u8_t,
    79 as libc::c_int as mln_u8_t,
    171 as libc::c_int as mln_u8_t,
    108 as libc::c_int as mln_u8_t,
    165 as libc::c_int as mln_u8_t,
    126 as libc::c_int as mln_u8_t,
    145 as libc::c_int as mln_u8_t,
    136 as libc::c_int as mln_u8_t,
    34 as libc::c_int as mln_u8_t,
    9 as libc::c_int as mln_u8_t,
    74 as libc::c_int as mln_u8_t,
    30 as libc::c_int as mln_u8_t,
    32 as libc::c_int as mln_u8_t,
    163 as libc::c_int as mln_u8_t,
    84 as libc::c_int as mln_u8_t,
    245 as libc::c_int as mln_u8_t,
    173 as libc::c_int as mln_u8_t,
    187 as libc::c_int as mln_u8_t,
    204 as libc::c_int as mln_u8_t,
    142 as libc::c_int as mln_u8_t,
    81 as libc::c_int as mln_u8_t,
    181 as libc::c_int as mln_u8_t,
    190 as libc::c_int as mln_u8_t,
    46 as libc::c_int as mln_u8_t,
    88 as libc::c_int as mln_u8_t,
    100 as libc::c_int as mln_u8_t,
    159 as libc::c_int as mln_u8_t,
    25 as libc::c_int as mln_u8_t,
    231 as libc::c_int as mln_u8_t,
    50 as libc::c_int as mln_u8_t,
    207 as libc::c_int as mln_u8_t,
    57 as libc::c_int as mln_u8_t,
    147 as libc::c_int as mln_u8_t,
    14 as libc::c_int as mln_u8_t,
    67 as libc::c_int as mln_u8_t,
    120 as libc::c_int as mln_u8_t,
    128 as libc::c_int as mln_u8_t,
    154 as libc::c_int as mln_u8_t,
    248 as libc::c_int as mln_u8_t,
    213 as libc::c_int as mln_u8_t,
    167 as libc::c_int as mln_u8_t,
    200 as libc::c_int as mln_u8_t,
    63 as libc::c_int as mln_u8_t,
    236 as libc::c_int as mln_u8_t,
    110 as libc::c_int as mln_u8_t,
    92 as libc::c_int as mln_u8_t,
    176 as libc::c_int as mln_u8_t,
    7 as libc::c_int as mln_u8_t,
    161 as libc::c_int as mln_u8_t,
    77 as libc::c_int as mln_u8_t,
    124 as libc::c_int as mln_u8_t,
    221 as libc::c_int as mln_u8_t,
    102 as libc::c_int as mln_u8_t,
    218 as libc::c_int as mln_u8_t,
    95 as libc::c_int as mln_u8_t,
    198 as libc::c_int as mln_u8_t,
    90 as libc::c_int as mln_u8_t,
    12 as libc::c_int as mln_u8_t,
    152 as libc::c_int as mln_u8_t,
    98 as libc::c_int as mln_u8_t,
    48 as libc::c_int as mln_u8_t,
    185 as libc::c_int as mln_u8_t,
    179 as libc::c_int as mln_u8_t,
    42 as libc::c_int as mln_u8_t,
    209 as libc::c_int as mln_u8_t,
    37 as libc::c_int as mln_u8_t,
    132 as libc::c_int as mln_u8_t,
    224 as libc::c_int as mln_u8_t,
    52 as libc::c_int as mln_u8_t,
    254 as libc::c_int as mln_u8_t,
    239 as libc::c_int as mln_u8_t,
    117 as libc::c_int as mln_u8_t,
    233 as libc::c_int as mln_u8_t,
    139 as libc::c_int as mln_u8_t,
    22 as libc::c_int as mln_u8_t,
    105 as libc::c_int as mln_u8_t,
    27 as libc::c_int as mln_u8_t,
    194 as libc::c_int as mln_u8_t,
    113 as libc::c_int as mln_u8_t,
    230 as libc::c_int as mln_u8_t,
    206 as libc::c_int as mln_u8_t,
    87 as libc::c_int as mln_u8_t,
    158 as libc::c_int as mln_u8_t,
    80 as libc::c_int as mln_u8_t,
    189 as libc::c_int as mln_u8_t,
    172 as libc::c_int as mln_u8_t,
    203 as libc::c_int as mln_u8_t,
    109 as libc::c_int as mln_u8_t,
    175 as libc::c_int as mln_u8_t,
    166 as libc::c_int as mln_u8_t,
    62 as libc::c_int as mln_u8_t,
    127 as libc::c_int as mln_u8_t,
    247 as libc::c_int as mln_u8_t,
    146 as libc::c_int as mln_u8_t,
    66 as libc::c_int as mln_u8_t,
    137 as libc::c_int as mln_u8_t,
    192 as libc::c_int as mln_u8_t,
    35 as libc::c_int as mln_u8_t,
    252 as libc::c_int as mln_u8_t,
    10 as libc::c_int as mln_u8_t,
    183 as libc::c_int as mln_u8_t,
    75 as libc::c_int as mln_u8_t,
    216 as libc::c_int as mln_u8_t,
    31 as libc::c_int as mln_u8_t,
    83 as libc::c_int as mln_u8_t,
    33 as libc::c_int as mln_u8_t,
    73 as libc::c_int as mln_u8_t,
    164 as libc::c_int as mln_u8_t,
    144 as libc::c_int as mln_u8_t,
    85 as libc::c_int as mln_u8_t,
    170 as libc::c_int as mln_u8_t,
    246 as libc::c_int as mln_u8_t,
    65 as libc::c_int as mln_u8_t,
    174 as libc::c_int as mln_u8_t,
    61 as libc::c_int as mln_u8_t,
    188 as libc::c_int as mln_u8_t,
    202 as libc::c_int as mln_u8_t,
    205 as libc::c_int as mln_u8_t,
    157 as libc::c_int as mln_u8_t,
    143 as libc::c_int as mln_u8_t,
    169 as libc::c_int as mln_u8_t,
    82 as libc::c_int as mln_u8_t,
    72 as libc::c_int as mln_u8_t,
    182 as libc::c_int as mln_u8_t,
    215 as libc::c_int as mln_u8_t,
    191 as libc::c_int as mln_u8_t,
    251 as libc::c_int as mln_u8_t,
    47 as libc::c_int as mln_u8_t,
    178 as libc::c_int as mln_u8_t,
    89 as libc::c_int as mln_u8_t,
    151 as libc::c_int as mln_u8_t,
    101 as libc::c_int as mln_u8_t,
    94 as libc::c_int as mln_u8_t,
    160 as libc::c_int as mln_u8_t,
    123 as libc::c_int as mln_u8_t,
    26 as libc::c_int as mln_u8_t,
    112 as libc::c_int as mln_u8_t,
    232 as libc::c_int as mln_u8_t,
    21 as libc::c_int as mln_u8_t,
    51 as libc::c_int as mln_u8_t,
    238 as libc::c_int as mln_u8_t,
    208 as libc::c_int as mln_u8_t,
    131 as libc::c_int as mln_u8_t,
    58 as libc::c_int as mln_u8_t,
    69 as libc::c_int as mln_u8_t,
    148 as libc::c_int as mln_u8_t,
    18 as libc::c_int as mln_u8_t,
    15 as libc::c_int as mln_u8_t,
    16 as libc::c_int as mln_u8_t,
    68 as libc::c_int as mln_u8_t,
    17 as libc::c_int as mln_u8_t,
    121 as libc::c_int as mln_u8_t,
    149 as libc::c_int as mln_u8_t,
    129 as libc::c_int as mln_u8_t,
    19 as libc::c_int as mln_u8_t,
    155 as libc::c_int as mln_u8_t,
    59 as libc::c_int as mln_u8_t,
    249 as libc::c_int as mln_u8_t,
    70 as libc::c_int as mln_u8_t,
    214 as libc::c_int as mln_u8_t,
    250 as libc::c_int as mln_u8_t,
    168 as libc::c_int as mln_u8_t,
    71 as libc::c_int as mln_u8_t,
    201 as libc::c_int as mln_u8_t,
    156 as libc::c_int as mln_u8_t,
    64 as libc::c_int as mln_u8_t,
    60 as libc::c_int as mln_u8_t,
    237 as libc::c_int as mln_u8_t,
    130 as libc::c_int as mln_u8_t,
    111 as libc::c_int as mln_u8_t,
    20 as libc::c_int as mln_u8_t,
    93 as libc::c_int as mln_u8_t,
    122 as libc::c_int as mln_u8_t,
    177 as libc::c_int as mln_u8_t,
    150 as libc::c_int as mln_u8_t,
];
#[inline]
unsafe extern "C" fn mln_rs_power_calc(
    mut base: mln_size_t,
    mut exp: mln_size_t,
) -> mln_size_t {
    let mut i: mln_s32_t = 0;
    let mut save: mln_size_t = base;
    i = ((::core::mem::size_of::<mln_size_t>() as libc::c_ulong) << 3 as libc::c_int)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as mln_s32_t;
    while i >= 0 as libc::c_int {
        if exp & (1 as libc::c_int as mln_size_t) << i != 0 {
            break;
        }
        i -= 1;
        i;
    }
    let fresh0 = i;
    i = i - 1;
    if fresh0 < 0 as libc::c_int {
        return 1 as libc::c_int as mln_size_t;
    }
    while i >= 0 as libc::c_int {
        base = (base as libc::c_ulong).wrapping_mul(base) as mln_size_t as mln_size_t;
        if exp & (1 as libc::c_int as mln_size_t) << i != 0 {
            base = (base as libc::c_ulong).wrapping_mul(save) as mln_size_t
                as mln_size_t;
        }
        i -= 1;
        i;
    }
    return base;
}
unsafe extern "C" fn mln_rs_matrix_new(
    mut row: mln_size_t,
    mut col: mln_size_t,
    mut data: mln_u8ptr_t,
    mut is_ref: mln_u32_t,
) -> *mut mln_rs_matrix_t {
    let mut rm: *mut mln_rs_matrix_t = 0 as *mut mln_rs_matrix_t;
    rm = malloc(::core::mem::size_of::<mln_rs_matrix_t>() as libc::c_ulong)
        as *mut mln_rs_matrix_t;
    if rm.is_null() {
        return 0 as *mut mln_rs_matrix_t;
    }
    (*rm).row = row;
    (*rm).col = col;
    (*rm).data = data;
    (*rm).set_is_ref(is_ref);
    return rm;
}
unsafe extern "C" fn mln_rs_matrix_free(mut matrix: *mut mln_rs_matrix_t) {
    if matrix.is_null() {
        return;
    }
    if !((*matrix).data).is_null() && (*matrix).is_ref() == 0 {
        free((*matrix).data as *mut libc::c_void);
    }
    free(matrix as *mut libc::c_void);
}
#[inline]
unsafe extern "C" fn mln_rs_matrix_mul(
    mut m1: *mut mln_rs_matrix_t,
    mut m2: *mut mln_rs_matrix_t,
) -> *mut mln_rs_matrix_t {
    if (*m1).col != (*m2).row {
        *__errno_location() = 22 as libc::c_int;
        return 0 as *mut mln_rs_matrix_t;
    }
    let mut dst: mln_u8_t = 0;
    let mut src: mln_u8_t = 0;
    let mut tmp: mln_u8_t = 0;
    let mut data: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut i: mln_size_t = 0;
    let mut j: mln_size_t = 0;
    let mut k: mln_size_t = 0;
    let mut ret: *mut mln_rs_matrix_t = 0 as *mut mln_rs_matrix_t;
    let mut m1row: mln_size_t = (*m1).row;
    let mut m1col: mln_size_t = (*m1).col;
    let mut m2col: mln_size_t = (*m2).col;
    let mut m1data: mln_u8ptr_t = (*m1).data;
    let mut m2data: mln_u8ptr_t = (*m2).data;
    data = calloc(m1row, m2col) as mln_u8ptr_t;
    if data.is_null() {
        *__errno_location() = 12 as libc::c_int;
        return 0 as *mut mln_rs_matrix_t;
    }
    ret = mln_rs_matrix_new(m1row, m2col, data, 0 as libc::c_int as mln_u32_t);
    if ret.is_null() {
        free(data as *mut libc::c_void);
        *__errno_location() = 12 as libc::c_int;
        return 0 as *mut mln_rs_matrix_t;
    }
    i = 0 as libc::c_int as mln_size_t;
    while i < m1row {
        k = 0 as libc::c_int as mln_size_t;
        while k < m1col {
            tmp = *m1data.offset(i.wrapping_mul(m1col).wrapping_add(k) as isize);
            j = 0 as libc::c_int as mln_size_t;
            while j < m2col {
                dst = tmp;
                src = *m2data.offset(k.wrapping_mul(m2col).wrapping_add(j) as isize);
                dst = (if dst as libc::c_int == 0 as libc::c_int
                    || src as libc::c_int == 0 as libc::c_int
                {
                    0 as libc::c_int
                } else {
                    mln_rs_gfilog[((mln_rs_gflog[dst as usize] as libc::c_int
                        + mln_rs_gflog[src as usize] as libc::c_int)
                        % 255 as libc::c_int) as usize] as libc::c_int
                }) as mln_u8_t;
                let ref mut fresh1 = *data
                    .offset(i.wrapping_mul(m2col).wrapping_add(j) as isize);
                *fresh1 = (*fresh1 as libc::c_int ^ dst as libc::c_int) as libc::c_uchar;
                j = j.wrapping_add(1);
                j;
            }
            k = k.wrapping_add(1);
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return ret;
}
unsafe extern "C" fn mln_rs_matrix_inverse(
    mut matrix: *mut mln_rs_matrix_t,
) -> *mut mln_rs_matrix_t {
    if matrix.is_null() || (*matrix).row != (*matrix).col {
        *__errno_location() = 22 as libc::c_int;
        return 0 as *mut mln_rs_matrix_t;
    }
    let mut ret: *mut mln_rs_matrix_t = 0 as *mut mln_rs_matrix_t;
    let mut data: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut origin: mln_u8ptr_t = (*matrix).data;
    let mut tmp: mln_u8_t = 0;
    let mut dst: mln_u8_t = 0;
    let mut i: mln_size_t = 0;
    let mut j: mln_size_t = 0;
    let mut k: mln_size_t = 0;
    let mut m: mln_size_t = 0;
    let mut n: mln_size_t = ((*matrix).row).wrapping_mul((*matrix).col);
    let mut len: mln_size_t = (*matrix).row;
    data = malloc(n) as mln_u8ptr_t;
    if data.is_null() {
        *__errno_location() = 12 as libc::c_int;
        return 0 as *mut mln_rs_matrix_t;
    }
    ret = mln_rs_matrix_new(
        (*matrix).row,
        (*matrix).col,
        data,
        0 as libc::c_int as mln_u32_t,
    );
    if ret.is_null() {
        free(data as *mut libc::c_void);
        *__errno_location() = 12 as libc::c_int;
        return 0 as *mut mln_rs_matrix_t;
    }
    i = 0 as libc::c_int as mln_size_t;
    k = 0 as libc::c_int as mln_size_t;
    while i < n {
        j = 0 as libc::c_int as mln_size_t;
        while j < (*ret).col {
            *data
                .offset(
                    i.wrapping_add(j) as isize,
                ) = (if j == k { 1 as libc::c_int } else { 0 as libc::c_int })
                as libc::c_uchar;
            j = j.wrapping_add(1);
            j;
        }
        i = (i as libc::c_ulong).wrapping_add((*ret).col) as mln_size_t as mln_size_t;
        k = k.wrapping_add(1);
        k;
    }
    m = 0 as libc::c_int as mln_size_t;
    i = 0 as libc::c_int as mln_size_t;
    while i < n {
        tmp = *origin.offset(i.wrapping_add(m) as isize);
        k = i;
        j = i.wrapping_add(len);
        while j < n {
            if *origin.offset(j.wrapping_add(m) as isize) as libc::c_int
                > tmp as libc::c_int
            {
                tmp = *origin.offset(j.wrapping_add(m) as isize);
                k = j;
            }
            j = (j as libc::c_ulong).wrapping_add(len) as mln_size_t as mln_size_t;
        }
        if k != i {
            j = 0 as libc::c_int as mln_size_t;
            while j < len {
                tmp = *origin.offset(i.wrapping_add(j) as isize);
                *origin
                    .offset(
                        i.wrapping_add(j) as isize,
                    ) = *origin.offset(k.wrapping_add(j) as isize);
                *origin.offset(k.wrapping_add(j) as isize) = tmp;
                tmp = *data.offset(i.wrapping_add(j) as isize);
                *data
                    .offset(
                        i.wrapping_add(j) as isize,
                    ) = *data.offset(k.wrapping_add(j) as isize);
                *data.offset(k.wrapping_add(j) as isize) = tmp;
                j = j.wrapping_add(1);
                j;
            }
        }
        if *origin.offset(i.wrapping_add(m) as isize) == 0 {
            mln_rs_matrix_free(ret);
            *__errno_location() = 22 as libc::c_int;
            return 0 as *mut mln_rs_matrix_t;
        }
        tmp = *origin.offset(i.wrapping_add(m) as isize);
        j = 0 as libc::c_int as mln_size_t;
        while j < len {
            *origin
                .offset(
                    i.wrapping_add(j) as isize,
                ) = (if *origin.offset(i.wrapping_add(j) as isize) as libc::c_int
                == 0 as libc::c_int || tmp as libc::c_int == 0 as libc::c_int
            {
                0 as libc::c_int
            } else {
                mln_rs_gfilog[(if (mln_rs_gflog[*origin
                    .offset(i.wrapping_add(j) as isize) as usize] as libc::c_int)
                    < mln_rs_gflog[tmp as usize] as libc::c_int
                {
                    255 as libc::c_int
                        - (mln_rs_gflog[tmp as usize] as libc::c_int
                            - mln_rs_gflog[*origin.offset(i.wrapping_add(j) as isize)
                                as usize] as libc::c_int)
                } else {
                    mln_rs_gflog[*origin.offset(i.wrapping_add(j) as isize) as usize]
                        as libc::c_int - mln_rs_gflog[tmp as usize] as libc::c_int
                }) as usize] as libc::c_int
            }) as libc::c_uchar;
            *data
                .offset(
                    i.wrapping_add(j) as isize,
                ) = (if *data.offset(i.wrapping_add(j) as isize) as libc::c_int
                == 0 as libc::c_int || tmp as libc::c_int == 0 as libc::c_int
            {
                0 as libc::c_int
            } else {
                mln_rs_gfilog[(if (mln_rs_gflog[*data.offset(i.wrapping_add(j) as isize)
                    as usize] as libc::c_int) < mln_rs_gflog[tmp as usize] as libc::c_int
                {
                    255 as libc::c_int
                        - (mln_rs_gflog[tmp as usize] as libc::c_int
                            - mln_rs_gflog[*data.offset(i.wrapping_add(j) as isize)
                                as usize] as libc::c_int)
                } else {
                    mln_rs_gflog[*data.offset(i.wrapping_add(j) as isize) as usize]
                        as libc::c_int - mln_rs_gflog[tmp as usize] as libc::c_int
                }) as usize] as libc::c_int
            }) as libc::c_uchar;
            j = j.wrapping_add(1);
            j;
        }
        j = 0 as libc::c_int as mln_size_t;
        while j < n {
            if j != i {
                tmp = *origin.offset(j.wrapping_add(m) as isize);
                k = 0 as libc::c_int as mln_size_t;
                while k < len {
                    dst = *origin.offset(i.wrapping_add(k) as isize);
                    dst = (if dst as libc::c_int == 0 as libc::c_int
                        || tmp as libc::c_int == 0 as libc::c_int
                    {
                        0 as libc::c_int
                    } else {
                        mln_rs_gfilog[((mln_rs_gflog[dst as usize] as libc::c_int
                            + mln_rs_gflog[tmp as usize] as libc::c_int)
                            % 255 as libc::c_int) as usize] as libc::c_int
                    }) as mln_u8_t;
                    let ref mut fresh2 = *origin.offset(j.wrapping_add(k) as isize);
                    *fresh2 = (*fresh2 as libc::c_int ^ dst as libc::c_int)
                        as libc::c_uchar;
                    dst = *data.offset(i.wrapping_add(k) as isize);
                    dst = (if dst as libc::c_int == 0 as libc::c_int
                        || tmp as libc::c_int == 0 as libc::c_int
                    {
                        0 as libc::c_int
                    } else {
                        mln_rs_gfilog[((mln_rs_gflog[dst as usize] as libc::c_int
                            + mln_rs_gflog[tmp as usize] as libc::c_int)
                            % 255 as libc::c_int) as usize] as libc::c_int
                    }) as mln_u8_t;
                    let ref mut fresh3 = *data.offset(j.wrapping_add(k) as isize);
                    *fresh3 = (*fresh3 as libc::c_int ^ dst as libc::c_int)
                        as libc::c_uchar;
                    k = k.wrapping_add(1);
                    k;
                }
            }
            j = (j as libc::c_ulong).wrapping_add(len) as mln_size_t as mln_size_t;
        }
        i = (i as libc::c_ulong).wrapping_add(len) as mln_size_t as mln_size_t;
        m = m.wrapping_add(1);
        m;
    }
    return ret;
}
unsafe extern "C" fn mln_rs_matrix_co_matrix(
    mut row: mln_size_t,
    mut addition_row: mln_size_t,
) -> *mut mln_rs_matrix_t {
    let mut data: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut p: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut i: mln_size_t = 0;
    let mut j: mln_size_t = 0;
    let mut k: mln_size_t = 0;
    let mut matrix: *mut mln_rs_matrix_t = 0 as *mut mln_rs_matrix_t;
    data = malloc(row.wrapping_add(addition_row).wrapping_mul(row)) as mln_u8ptr_t;
    if data.is_null() {
        *__errno_location() = 12 as libc::c_int;
        return 0 as *mut mln_rs_matrix_t;
    }
    i = 0 as libc::c_int as mln_size_t;
    p = data;
    while i < row {
        k = 0 as libc::c_int as mln_size_t;
        while k < row {
            let fresh4 = p;
            p = p.offset(1);
            *fresh4 = (if k == i { 1 as libc::c_int } else { 0 as libc::c_int })
                as libc::c_uchar;
            k = k.wrapping_add(1);
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    j = 1 as libc::c_int as mln_size_t;
    while i < row.wrapping_add(addition_row) {
        k = 1 as libc::c_int as mln_size_t;
        while k <= row {
            let fresh5 = p;
            p = p.offset(1);
            *fresh5 = mln_rs_power_calc(
                k,
                j.wrapping_sub(1 as libc::c_int as libc::c_ulong),
            ) as mln_u8_t;
            k = k.wrapping_add(1);
            k;
        }
        i = i.wrapping_add(1);
        i;
        j = j.wrapping_add(1);
        j;
    }
    matrix = mln_rs_matrix_new(
        row.wrapping_add(addition_row),
        row,
        data,
        0 as libc::c_int as mln_u32_t,
    );
    if matrix.is_null() {
        free(data as *mut libc::c_void);
        *__errno_location() = 12 as libc::c_int;
        return 0 as *mut mln_rs_matrix_t;
    }
    return matrix;
}
unsafe extern "C" fn mln_rs_matrix_co_inverse_matrix(
    mut data_vector: *mut *mut uint8_t,
    mut len: size_t,
    mut n: size_t,
    mut k: size_t,
) -> *mut mln_rs_matrix_t {
    let mut i: mln_size_t = 0;
    let mut j: mln_size_t = 0;
    let mut row: mln_size_t = 0 as libc::c_int as mln_size_t;
    let mut matrix: *mut mln_rs_matrix_t = 0 as *mut mln_rs_matrix_t;
    let mut inverse: *mut mln_rs_matrix_t = 0 as *mut mln_rs_matrix_t;
    let mut p: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut pend: *mut mln_u8ptr_t = 0 as *mut mln_u8ptr_t;
    let mut data: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    data = malloc(n.wrapping_add(k).wrapping_mul(n)) as mln_u8ptr_t;
    if data.is_null() {
        *__errno_location() = 12 as libc::c_int;
        return 0 as *mut mln_rs_matrix_t;
    }
    p = data;
    pend = data_vector.offset(n as isize);
    i = 0 as libc::c_int as mln_size_t;
    while data_vector < pend {
        if !(*data_vector).is_null() {
            j = 0 as libc::c_int as mln_size_t;
            while j < n {
                let fresh6 = p;
                p = p.offset(1);
                *fresh6 = (if i == j { 1 as libc::c_int } else { 0 as libc::c_int })
                    as libc::c_uchar;
                j = j.wrapping_add(1);
                j;
            }
            row = row.wrapping_add(1);
            row;
        }
        data_vector = data_vector.offset(1);
        data_vector;
        i = i.wrapping_add(1);
        i;
    }
    pend = data_vector.offset(n as isize).offset(k as isize);
    i = 1 as libc::c_int as mln_size_t;
    while row < n && data_vector < pend {
        if !(*data_vector).is_null() {
            j = 1 as libc::c_int as mln_size_t;
            while j <= n {
                let fresh7 = p;
                p = p.offset(1);
                *fresh7 = mln_rs_power_calc(
                    j,
                    i.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                ) as mln_u8_t;
                j = j.wrapping_add(1);
                j;
            }
            row = row.wrapping_add(1);
            row;
        }
        data_vector = data_vector.offset(1);
        data_vector;
        i = i.wrapping_add(1);
        i;
    }
    matrix = mln_rs_matrix_new(n, n, data, 0 as libc::c_int as mln_u32_t);
    if matrix.is_null() {
        free(data as *mut libc::c_void);
        *__errno_location() = 12 as libc::c_int;
        return 0 as *mut mln_rs_matrix_t;
    }
    inverse = mln_rs_matrix_inverse(matrix);
    if inverse.is_null() {
        let mut err: libc::c_int = *__errno_location();
        mln_rs_matrix_free(matrix);
        *__errno_location() = err;
        return 0 as *mut mln_rs_matrix_t;
    }
    mln_rs_matrix_free(matrix);
    return inverse;
}
unsafe extern "C" fn mln_rs_matrix_data_matrix(
    mut data_vector: *mut *mut uint8_t,
    mut len: size_t,
    mut n: size_t,
    mut k: size_t,
) -> *mut mln_rs_matrix_t {
    let mut row: mln_size_t = 0 as libc::c_int as mln_size_t;
    let mut data: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut p: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut pend: *mut mln_u8ptr_t = 0 as *mut mln_u8ptr_t;
    let mut matrix: *mut mln_rs_matrix_t = 0 as *mut mln_rs_matrix_t;
    data = malloc(n.wrapping_mul(len)) as mln_u8ptr_t;
    if data.is_null() {
        *__errno_location() = 12 as libc::c_int;
        return 0 as *mut mln_rs_matrix_t;
    }
    p = data;
    pend = data_vector.offset(n as isize).offset(k as isize);
    while row < n && data_vector < pend {
        if !(*data_vector).is_null() {
            memcpy(p as *mut libc::c_void, *data_vector as *const libc::c_void, len);
            p = p.offset(len as isize);
            row = row.wrapping_add(1);
            row;
        }
        data_vector = data_vector.offset(1);
        data_vector;
    }
    matrix = mln_rs_matrix_new(n, len, data, 0 as libc::c_int as mln_u32_t);
    if matrix.is_null() {
        free(data as *mut libc::c_void);
        *__errno_location() = 22 as libc::c_int;
        return 0 as *mut mln_rs_matrix_t;
    }
    return matrix;
}
#[no_mangle]
pub unsafe extern "C" fn mln_rs_matrix_dump(mut matrix: *mut mln_rs_matrix_t) {
    if matrix.is_null() {
        return;
    }
    let mut i: mln_size_t = 0;
    let mut sum: mln_size_t = ((*matrix).row).wrapping_mul((*matrix).col);
    printf(
        b"Matrix row:%lu col:%lu\n \0" as *const u8 as *const libc::c_char,
        (*matrix).row,
        (*matrix).col,
    );
    i = 0 as libc::c_int as mln_size_t;
    while i < sum {
        if i != 0 && i.wrapping_rem((*matrix).col) == 0 {
            printf(b"\n \0" as *const u8 as *const libc::c_char);
        }
        printf(
            b"%u \0" as *const u8 as *const libc::c_char,
            *((*matrix).data).offset(i as isize) as libc::c_int,
        );
        i = i.wrapping_add(1);
        i;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn mln_rs_result_new(
    mut data: mln_u8ptr_t,
    mut num: mln_size_t,
    mut len: mln_size_t,
) -> *mut mln_rs_result_t {
    let mut rr: *mut mln_rs_result_t = 0 as *mut mln_rs_result_t;
    rr = malloc(::core::mem::size_of::<mln_rs_result_t>() as libc::c_ulong)
        as *mut mln_rs_result_t;
    if rr.is_null() {
        *__errno_location() = 12 as libc::c_int;
        return 0 as *mut mln_rs_result_t;
    }
    (*rr).data = data;
    (*rr).len = len;
    (*rr).num = num;
    return rr;
}
#[no_mangle]
pub unsafe extern "C" fn mln_rs_result_free(mut result: *mut mln_rs_result_t) {
    if result.is_null() {
        return;
    }
    if !((*result).data).is_null() {
        free((*result).data as *mut libc::c_void);
    }
    free(result as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn mln_rs_encode(
    mut data_vector: *mut uint8_t,
    mut len: size_t,
    mut n: size_t,
    mut k: size_t,
) -> *mut mln_rs_result_t {
    let mut result: *mut mln_rs_result_t = 0 as *mut mln_rs_result_t;
    let mut matrix: *mut mln_rs_matrix_t = 0 as *mut mln_rs_matrix_t;
    let mut co_matrix: *mut mln_rs_matrix_t = 0 as *mut mln_rs_matrix_t;
    let mut res_matrix: *mut mln_rs_matrix_t = 0 as *mut mln_rs_matrix_t;
    if data_vector.is_null() || len == 0 || n == 0 || k == 0 {
        *__errno_location() = 22 as libc::c_int;
        return 0 as *mut mln_rs_result_t;
    }
    matrix = mln_rs_matrix_new(n, len, data_vector, 1 as libc::c_int as mln_u32_t);
    if matrix.is_null() {
        *__errno_location() = 12 as libc::c_int;
        return 0 as *mut mln_rs_result_t;
    }
    co_matrix = mln_rs_matrix_co_matrix(n, k);
    if co_matrix.is_null() {
        mln_rs_matrix_free(matrix);
        *__errno_location() = 12 as libc::c_int;
        return 0 as *mut mln_rs_result_t;
    }
    res_matrix = mln_rs_matrix_mul(co_matrix, matrix);
    mln_rs_matrix_free(matrix);
    mln_rs_matrix_free(co_matrix);
    if res_matrix.is_null() {
        *__errno_location() = 12 as libc::c_int;
        return 0 as *mut mln_rs_result_t;
    }
    result = mln_rs_result_new(
        (*res_matrix).data,
        n.wrapping_add(k),
        n.wrapping_add(k).wrapping_mul(len),
    );
    if result.is_null() {
        mln_rs_matrix_free(res_matrix);
        *__errno_location() = 12 as libc::c_int;
        return 0 as *mut mln_rs_result_t;
    }
    (*res_matrix).set_is_ref(1 as libc::c_int as mln_u32_t);
    mln_rs_matrix_free(res_matrix);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn mln_rs_decode(
    mut data_vector: *mut *mut uint8_t,
    mut len: size_t,
    mut n: size_t,
    mut k: size_t,
) -> *mut mln_rs_result_t {
    let mut data: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut p: mln_u8ptr_t = 0 as *mut libc::c_uchar;
    let mut pp: *mut mln_u8ptr_t = 0 as *mut mln_u8ptr_t;
    let mut pend: *mut mln_u8ptr_t = 0 as *mut mln_u8ptr_t;
    let mut result: *mut mln_rs_result_t = 0 as *mut mln_rs_result_t;
    let mut co_matrix: *mut mln_rs_matrix_t = 0 as *mut mln_rs_matrix_t;
    let mut data_matrix: *mut mln_rs_matrix_t = 0 as *mut mln_rs_matrix_t;
    let mut result_matrix: *mut mln_rs_matrix_t = 0 as *mut mln_rs_matrix_t;
    if n == 0 && k == 0 {
        *__errno_location() = 22 as libc::c_int;
        return 0 as *mut mln_rs_result_t;
    }
    if k == 0 {
        data = malloc(len.wrapping_mul(n)) as mln_u8ptr_t;
        if data.is_null() {
            *__errno_location() = 12 as libc::c_int;
            return 0 as *mut mln_rs_result_t;
        }
        p = data;
        pp = data_vector;
        pend = data_vector.offset(n as isize);
        while pp < pend {
            memcpy(p as *mut libc::c_void, *pp as *const libc::c_void, len);
            p = p.offset(len as isize);
            pp = pp.offset(1);
            pp;
        }
        result = mln_rs_result_new(data, n, len.wrapping_mul(n));
        if result.is_null() {
            free(data as *mut libc::c_void);
            *__errno_location() = 12 as libc::c_int;
            return 0 as *mut mln_rs_result_t;
        }
        return result;
    }
    co_matrix = mln_rs_matrix_co_inverse_matrix(data_vector, len, n, k);
    if co_matrix.is_null() {
        return 0 as *mut mln_rs_result_t;
    }
    data_matrix = mln_rs_matrix_data_matrix(data_vector, len, n, k);
    if data_matrix.is_null() {
        let mut err: libc::c_int = *__errno_location();
        mln_rs_matrix_free(co_matrix);
        *__errno_location() = err;
        return 0 as *mut mln_rs_result_t;
    }
    result_matrix = mln_rs_matrix_mul(co_matrix, data_matrix);
    mln_rs_matrix_free(co_matrix);
    mln_rs_matrix_free(data_matrix);
    if result_matrix.is_null() {
        *__errno_location() = 12 as libc::c_int;
        return 0 as *mut mln_rs_result_t;
    }
    result = mln_rs_result_new(
        (*result_matrix).data,
        (*result_matrix).row,
        ((*result_matrix).row).wrapping_mul((*result_matrix).col),
    );
    if result.is_null() {
        mln_rs_matrix_free(result_matrix);
        *__errno_location() = 12 as libc::c_int;
        return 0 as *mut mln_rs_result_t;
    }
    (*result_matrix).data = 0 as mln_u8ptr_t;
    mln_rs_matrix_free(result_matrix);
    return result;
}
