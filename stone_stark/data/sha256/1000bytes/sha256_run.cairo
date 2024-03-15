%builtins output pedersen range_check bitwise

from starkware.cairo.common.cairo_builtins import HashBuiltin, BitwiseBuiltin
from starkware.cairo.common.registers import get_label_location
from starkware.cairo.common.invoke import invoke
from starkware.cairo.common.alloc import alloc

from sha256 import sha256, finalize_sha256

func hash_thousand_bytes{bitwise_ptr: BitwiseBuiltin*, range_check_ptr}() -> felt* {
    alloc_locals;

    let (input) = alloc();
    assert input[0] = 'abcd';
    assert input[1] = 'efgh';
    assert input[2] = 'bcde';
    assert input[3] = 'fghi';
    assert input[4] = 'cdef';
    assert input[5] = 'ghij';
    assert input[6] = 'defg';
    assert input[7] = 'hijk';
    assert input[8] = 'efgh';
    assert input[9] = 'ijkl';
    assert input[10] = 'fghi';
    assert input[11] = 'jklm';
    assert input[12] = 'ghij';
    assert input[13] = 'klmn';
    assert input[14] = 'hijk';
    assert input[15] = 'lmno';
    assert input[16] = 'ijkl';
    assert input[17] = 'mnop';
    assert input[18] = 'jklm';
    assert input[19] = 'nopq';
    assert input[20] = 'klmn';
    assert input[21] = 'opqr';
    assert input[22] = 'lmno';
    assert input[23] = 'pqrs';
    assert input[24] = 'mnop';    
    assert input[25] = 'nopq';
    assert input[26] = 'qrst';
    assert input[27] = 'opqr';
    assert input[28] = 'rstu';
    assert input[29] = 'pqrs';
    assert input[30] = 'stuv';
    assert input[31] = 'qrst';
    assert input[32] = 'tuvw';
    assert input[33] = 'rstu';
    assert input[34] = 'vwxy';
    assert input[35] = 'stuv';
    assert input[36] = 'wxyz';
    assert input[37] = 'tuvw';
    assert input[38] = 'yzab';
    assert input[39] = 'vwxy';
    assert input[40] = 'zabc';
    assert input[41] = 'wxyz';
    assert input[42] = 'abcd';
    assert input[43] = 'yzab';
    assert input[44] = 'bcde';
    assert input[45] = 'zabc';
    assert input[46] = 'cdef';
    assert input[47] = 'abcd';
    assert input[48] = 'defg';
    assert input[49] = 'bcde';
    assert input[50] = 'efgh';
    assert input[51] = 'fghij';
    assert input[52] = 'cdefg';
    assert input[53] = 'ghijk';
    assert input[54] = 'defgh';
    assert input[55] = 'hijkl';
    assert input[56] = 'efghi';
    assert input[57] = 'ijklm';
    assert input[58] = 'fghij';
    assert input[59] = 'jklmn';
    assert input[60] = 'ghijk';
    assert input[61] = 'klmno';
    assert input[62] = 'hijkl';
    assert input[63] = 'lmnop';
    assert input[64] = 'ijklm';
    assert input[65] = 'mnopq';
    assert input[66] = 'jklmn';
    assert input[67] = 'nopqr';
    assert input[68] = 'klmno';
    assert input[69] = 'opqrs';
    assert input[70] = 'lmnop';    
    assert input[71] = 'nopqr';
    assert input[72] = 'qrstu';
    assert input[73] = 'opqrs';
    assert input[74] = 'rstuv';
    assert input[75] = 'pqrsu';
    assert input[76] = 'stuvw';
    assert input[77] = 'rstuv';
    assert input[78] = 'tuvwx';
    assert input[79] = 'stuvw';
    assert input[80] = 'vwxyz';
    assert input[81] = 'tuvwx';
    assert input[82] = 'xyzab';
    assert input[83] = 'vwxyz';
    assert input[84] = 'zabcd';
    assert input[85] = 'wxyza';
    assert input[86] = 'abcde';
    assert input[87] = 'yzabc';
    assert input[88] = 'bcdef';
    assert input[89] = 'zabcd';
    assert input[90] = 'cdefg';
    assert input[91] = 'abcde';
    assert input[92] = 'defgh';
    assert input[93] = 'bcdef';
    assert input[94] = 'efghi';
    assert input[95] = 'cdefg';
    assert input[96] = 'fghij';
    assert input[97] = 'defgh';
    assert input[98] = 'ghijk';
    assert input[99] = 'efghi';
    assert input[100] = 'ijklm';
    assert input[101] = 'jklmn';
    assert input[102] = 'nopqr';
    assert input[103] = 'klmno';
    assert input[104] = 'opqrs';
    assert input[105] = 'lmnop';
    assert input[106] = 'nopqr';
    assert input[107] = 'qrstu';
    assert input[108] = 'opqrs';
    assert input[109] = 'rstuv';
    assert input[110] = 'pqrsu';
    assert input[111] = 'stuvw';
    assert input[112] = 'rstuv';
    assert input[113] = 'tuvwx';
    assert input[114] = 'stuvw';
    assert input[115] = 'vwxyz';
    assert input[116] = 'tuvwx';
    assert input[117] = 'xyzab';
    assert input[118] = 'vwxyz';
    assert input[119] = 'zabcd';
    assert input[120] = 'wxyza';
    assert input[121] = 'abcde';
    assert input[122] = 'yzabc';
    assert input[123] = 'bcdef';
    assert input[124] = 'zabcd';
    assert input[125] = 'cdefg';
    assert input[126] = 'abcde';
    assert input[127] = 'defgh';
    assert input[128] = 'bcdef';
    assert input[129] = 'efghi';
    assert input[130] = 'cdefg';
    assert input[131] = 'fghij';
    assert input[132] = 'defgh';
    assert input[133] = 'ghijk';
    assert input[134] = 'efghi';
    assert input[135] = 'ijklm';
    assert input[136] = 'jklmn';
    assert input[137] = 'nopqr';
    assert input[138] = 'klmno';
    assert input[139] = 'opqrs';
    assert input[140] = 'lmnop';
    assert input[141] = 'nopqr';
    assert input[142] = 'qrstu';
    assert input[143] = 'opqrs';
    assert input[144] = 'rstuv';
    assert input[145] = 'pqrsu';
    assert input[146] = 'stuvw';
    assert input[147] = 'rstuv';
    assert input[148] = 'tuvwx';
    assert input[149] = 'stuvw';
    assert input[150] = 'vwxyz';
    assert input[151] = 'tuvwx';
    assert input[152] = 'xyzab';
    assert input[153] = 'vwxyz';
    assert input[154] = 'zabcd';
    assert input[155] = 'wxyza';
    assert input[156] = 'abcde';
    assert input[157] = 'yzabc';
    assert input[158] = 'bcdef';
    assert input[159] = 'zabcd';
    assert input[160] = 'cdefg';
    assert input[161] = 'abcde';
    assert input[162] = 'defgh';
    assert input[163] = 'bcdef';
    assert input[164] = 'efghi';
    assert input[165] = 'cdefg';
    assert input[166] = 'fghij';
    assert input[167] = 'defgh';
    assert input[168] = 'ghijk';
    assert input[169] = 'efghi';
    assert input[170] = 'ijklm';
    assert input[171] = 'jklmn';
    assert input[172] = 'nopqr';
    assert input[173] = 'klmno';
    assert input[174] = 'opqrs';
    assert input[175] = 'lmnop';
    assert input[176] = 'nopqr';
    assert input[177] = 'qrstu';
    assert input[178] = 'opqrs';
    assert input[179] = 'rstuv';
    assert input[180] = 'pqrsu';
    assert input[181] = 'stuvw';
    assert input[182] = 'rstuv';
    assert input[183] = 'tuvwx';
    assert input[184] = 'stuvw';
    assert input[185] = 'vwxyz';
    assert input[186] = 'tuvwx';
    assert input[187] = 'xyzab';
    assert input[188] = 'vwxyz';
    assert input[189] = 'zabcd';
    assert input[190] = 'wxyza';
    assert input[191] = 'abcde';
    assert input[192] = 'yzabc';
    assert input[193] = 'bcdef';
    assert input[194] = 'zabcd';
    assert input[195] = 'cdefg';
    assert input[196] = 'abcde';
    assert input[197] = 'defgh';
    assert input[198] = 'bcdef';
    assert input[199] = 'efghi';
    assert input[200] = 'cdefg';
    assert input[201] = 'fghij';
    assert input[202] = 'defgh';
    assert input[203] = 'ghijk';
    assert input[204] = 'efghi';
    assert input[205] = 'ijklm';
    assert input[206] = 'jklmn';
    assert input[207] = 'nopqr';
    assert input[208] = 'klmno';
    assert input[209] = 'opqrs';
    assert input[210] = 'lmnop';
    assert input[211] = 'nopqr';
    assert input[212] = 'qrstu';
    assert input[213] = 'opqrs';
    assert input[214] = 'rstuv';
    assert input[215] = 'pqrsu';
    assert input[216] = 'stuvw';
    assert input[217] = 'rstuv';
    assert input[218] = 'tuvwx';
    assert input[219] = 'stuvw';
    assert input[220] = 'vwxyz';
    assert input[221] = 'tuvwx';
    assert input[222] = 'xyzab';
    assert input[223] = 'vwxyz';
    assert input[224] = 'zabcd';
    assert input[225] = 'wxyza';
    assert input[226] = 'abcde';
    assert input[227] = 'yzabc';
    assert input[228] = 'bcdef';
    assert input[229] = 'zabcd';
    assert input[230] = 'cdefg';
    assert input[231] = 'abcde';
    assert input[232] = 'defgh';
    assert input[233] = 'bcdef';
    assert input[234] = 'efghi';
    assert input[235] = 'cdefg';
    assert input[236] = 'fghij';
    assert input[237] = 'defgh';
    assert input[238] = 'ghijk';
    assert input[239] = 'efghi';
    assert input[240] = 'ijklm';
    assert input[241] = 'jklmn';
    assert input[242] = 'nopqr';
    assert input[243] = 'klmno';
    assert input[244] = 'opqrs';
    assert input[245] = 'lmnop';
    assert input[246] = 'nopqr';
    assert input[247] = 'qrstu';
    assert input[248] = 'opqrs';
    assert input[249] = 'rstuv';


    let (local sha256_ptr: felt*) = alloc();
    let sha256_ptr_start = sha256_ptr;
    let (hash) = sha256{sha256_ptr=sha256_ptr}(input, 1000);
    // Disable Verification
    //finalize_sha256(sha256_ptr_start=sha256_ptr_start, sha256_ptr_end=sha256_ptr);

    return hash;
}

func main(
    output_ptr: felt*, pedersen_ptr: HashBuiltin*, range_check_ptr, bitwise_ptr: BitwiseBuiltin*) -> (
           output_ptr: felt*, pedersen_ptr: HashBuiltin*, range_check_ptr: felt, bitwise_ptr: BitwiseBuiltin*) {

    alloc_locals;

    let res = hash_thousand_bytes{bitwise_ptr=bitwise_ptr, range_check_ptr=range_check_ptr}();
    assert output_ptr[0] = res[0];
    assert output_ptr[1] = res[1];
    assert output_ptr[2] = res[2];
    assert output_ptr[3] = res[3];
    assert output_ptr[4] = res[4];
    assert output_ptr[5] = res[5];
    assert output_ptr[6] = res[6];
    assert output_ptr[7] = res[7];

    // Return the updated output_ptr.
    return (
        output_ptr=&output_ptr[8], pedersen_ptr=pedersen_ptr, range_check_ptr=range_check_ptr, bitwise_ptr=bitwise_ptr
    );
}