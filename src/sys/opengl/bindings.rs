use std::os::raw::c_char;
use std::os::raw::c_double;
use std::os::raw::c_float;
use std::os::raw::c_int;
use std::os::raw::c_short;
use std::os::raw::c_uchar;
use std::os::raw::c_uint;
use std::os::raw::c_ushort;
use std::os::raw::c_void;

type GLenum = c_uint;
type GLboolean = c_uchar;
type GLbitfield = c_uint;
type GLvoid = c_void;
type GLbyte = c_char;
type GLshort = c_short;
type GLint = c_int;
type GLclampx = c_int;
type GLubyte = c_uchar;
type GLushort = c_ushort;
type GLuint = c_uint;
type GLsizei = c_int;
type GLfloat = c_float;
type GLclampf = c_float;
type GLdouble = c_double;
type GLclampd = c_double;
type GLeglImageOES = *const c_void;
type GLchar = c_char;

type GLintptr = isize;
type GLsizeiptr = isize;
type GLint64 = i64;
type GLuint64 = u64;
type GLintptrARB = isize;
type GLsizeiptrARB = isize;
type GLint64EXT = i64;
type GLuint64EXT = u64;

pub const ACTIVE_ATTRIBUTES: GLenum = 0x8B89;
pub const ACTIVE_ATTRIBUTE_MAX_LENGTH: GLenum = 0x8B8A;
pub const ACTIVE_TEXTURE: GLenum = 0x84E0;
pub const ACTIVE_UNIFORMS: GLenum = 0x8B86;
pub const ACTIVE_UNIFORM_BLOCKS: GLenum = 0x8A36;
pub const ACTIVE_UNIFORM_BLOCK_MAX_NAME_LENGTH: GLenum = 0x8A35;
pub const ACTIVE_UNIFORM_MAX_LENGTH: GLenum = 0x8B87;
pub const ALIASED_LINE_WIDTH_RANGE: GLenum = 0x846E;
pub const ALPHA: GLenum = 0x1906;
pub const ALREADY_SIGNALED: GLenum = 0x911A;
pub const ALWAYS: GLenum = 0x0207;
pub const AND: GLenum = 0x1501;
pub const AND_INVERTED: GLenum = 0x1504;
pub const AND_REVERSE: GLenum = 0x1502;
pub const ANY_SAMPLES_PASSED: GLenum = 0x8C2F;
pub const ARRAY_BUFFER: GLenum = 0x8892;
pub const ARRAY_BUFFER_BINDING: GLenum = 0x8894;
pub const ATTACHED_SHADERS: GLenum = 0x8B85;
pub const BACK: GLenum = 0x0405;
pub const BACK_LEFT: GLenum = 0x0402;
pub const BACK_RIGHT: GLenum = 0x0403;
pub const BGR: GLenum = 0x80E0;
pub const BGRA: GLenum = 0x80E1;
pub const BGRA_INTEGER: GLenum = 0x8D9B;
pub const BGR_INTEGER: GLenum = 0x8D9A;
pub const BLEND: GLenum = 0x0BE2;
pub const BLEND_COLOR: GLenum = 0x8005;
pub const BLEND_DST: GLenum = 0x0BE0;
pub const BLEND_DST_ALPHA: GLenum = 0x80CA;
pub const BLEND_DST_RGB: GLenum = 0x80C8;
pub const BLEND_EQUATION: GLenum = 0x8009;
pub const BLEND_EQUATION_ALPHA: GLenum = 0x883D;
pub const BLEND_EQUATION_RGB: GLenum = 0x8009;
pub const BLEND_SRC: GLenum = 0x0BE1;
pub const BLEND_SRC_ALPHA: GLenum = 0x80CB;
pub const BLEND_SRC_RGB: GLenum = 0x80C9;
pub const BLUE: GLenum = 0x1905;
pub const BLUE_INTEGER: GLenum = 0x8D96;
pub const BOOL: GLenum = 0x8B56;
pub const BOOL_VEC2: GLenum = 0x8B57;
pub const BOOL_VEC3: GLenum = 0x8B58;
pub const BOOL_VEC4: GLenum = 0x8B59;
pub const BUFFER_ACCESS: GLenum = 0x88BB;
pub const BUFFER_ACCESS_FLAGS: GLenum = 0x911F;
pub const BUFFER_MAPPED: GLenum = 0x88BC;
pub const BUFFER_MAP_LENGTH: GLenum = 0x9120;
pub const BUFFER_MAP_OFFSET: GLenum = 0x9121;
pub const BUFFER_MAP_POINTER: GLenum = 0x88BD;
pub const BUFFER_SIZE: GLenum = 0x8764;
pub const BUFFER_USAGE: GLenum = 0x8765;
pub const BYTE: GLenum = 0x1400;
pub const CCW: GLenum = 0x0901;
pub const CLAMP_READ_COLOR: GLenum = 0x891C;
pub const CLAMP_TO_BORDER: GLenum = 0x812D;
pub const CLAMP_TO_EDGE: GLenum = 0x812F;
pub const CLEAR: GLenum = 0x1500;
pub const CLIP_DISTANCE0: GLenum = 0x3000;
pub const CLIP_DISTANCE1: GLenum = 0x3001;
pub const CLIP_DISTANCE2: GLenum = 0x3002;
pub const CLIP_DISTANCE3: GLenum = 0x3003;
pub const CLIP_DISTANCE4: GLenum = 0x3004;
pub const CLIP_DISTANCE5: GLenum = 0x3005;
pub const CLIP_DISTANCE6: GLenum = 0x3006;
pub const CLIP_DISTANCE7: GLenum = 0x3007;
pub const COLOR: GLenum = 0x1800;
pub const COLOR_ATTACHMENT0: GLenum = 0x8CE0;
pub const COLOR_ATTACHMENT1: GLenum = 0x8CE1;
pub const COLOR_ATTACHMENT10: GLenum = 0x8CEA;
pub const COLOR_ATTACHMENT11: GLenum = 0x8CEB;
pub const COLOR_ATTACHMENT12: GLenum = 0x8CEC;
pub const COLOR_ATTACHMENT13: GLenum = 0x8CED;
pub const COLOR_ATTACHMENT14: GLenum = 0x8CEE;
pub const COLOR_ATTACHMENT15: GLenum = 0x8CEF;
pub const COLOR_ATTACHMENT16: GLenum = 0x8CF0;
pub const COLOR_ATTACHMENT17: GLenum = 0x8CF1;
pub const COLOR_ATTACHMENT18: GLenum = 0x8CF2;
pub const COLOR_ATTACHMENT19: GLenum = 0x8CF3;
pub const COLOR_ATTACHMENT2: GLenum = 0x8CE2;
pub const COLOR_ATTACHMENT20: GLenum = 0x8CF4;
pub const COLOR_ATTACHMENT21: GLenum = 0x8CF5;
pub const COLOR_ATTACHMENT22: GLenum = 0x8CF6;
pub const COLOR_ATTACHMENT23: GLenum = 0x8CF7;
pub const COLOR_ATTACHMENT24: GLenum = 0x8CF8;
pub const COLOR_ATTACHMENT25: GLenum = 0x8CF9;
pub const COLOR_ATTACHMENT26: GLenum = 0x8CFA;
pub const COLOR_ATTACHMENT27: GLenum = 0x8CFB;
pub const COLOR_ATTACHMENT28: GLenum = 0x8CFC;
pub const COLOR_ATTACHMENT29: GLenum = 0x8CFD;
pub const COLOR_ATTACHMENT3: GLenum = 0x8CE3;
pub const COLOR_ATTACHMENT30: GLenum = 0x8CFE;
pub const COLOR_ATTACHMENT31: GLenum = 0x8CFF;
pub const COLOR_ATTACHMENT4: GLenum = 0x8CE4;
pub const COLOR_ATTACHMENT5: GLenum = 0x8CE5;
pub const COLOR_ATTACHMENT6: GLenum = 0x8CE6;
pub const COLOR_ATTACHMENT7: GLenum = 0x8CE7;
pub const COLOR_ATTACHMENT8: GLenum = 0x8CE8;
pub const COLOR_ATTACHMENT9: GLenum = 0x8CE9;
pub const COLOR_BUFFER_BIT: GLenum = 0x00004000;
pub const COLOR_CLEAR_VALUE: GLenum = 0x0C22;
pub const COLOR_LOGIC_OP: GLenum = 0x0BF2;
pub const COLOR_WRITEMASK: GLenum = 0x0C23;
pub const COMPARE_REF_TO_TEXTURE: GLenum = 0x884E;
pub const COMPILE_STATUS: GLenum = 0x8B81;
pub const COMPRESSED_RED: GLenum = 0x8225;
pub const COMPRESSED_RED_RGTC1: GLenum = 0x8DBB;
pub const COMPRESSED_RG: GLenum = 0x8226;
pub const COMPRESSED_RGB: GLenum = 0x84ED;
pub const COMPRESSED_RGBA: GLenum = 0x84EE;
pub const COMPRESSED_RG_RGTC2: GLenum = 0x8DBD;
pub const COMPRESSED_SIGNED_RED_RGTC1: GLenum = 0x8DBC;
pub const COMPRESSED_SIGNED_RG_RGTC2: GLenum = 0x8DBE;
pub const COMPRESSED_SRGB: GLenum = 0x8C48;
pub const COMPRESSED_SRGB_ALPHA: GLenum = 0x8C49;
pub const COMPRESSED_TEXTURE_FORMATS: GLenum = 0x86A3;
pub const CONDITION_SATISFIED: GLenum = 0x911C;
pub const CONSTANT_ALPHA: GLenum = 0x8003;
pub const CONSTANT_COLOR: GLenum = 0x8001;
pub const CONTEXT_COMPATIBILITY_PROFILE_BIT: GLenum = 0x00000002;
pub const CONTEXT_CORE_PROFILE_BIT: GLenum = 0x00000001;
pub const CONTEXT_FLAGS: GLenum = 0x821E;
pub const CONTEXT_FLAG_FORWARD_COMPATIBLE_BIT: GLenum = 0x00000001;
pub const CONTEXT_PROFILE_MASK: GLenum = 0x9126;
pub const COPY: GLenum = 0x1503;
pub const COPY_INVERTED: GLenum = 0x150C;
pub const COPY_READ_BUFFER: GLenum = 0x8F36;
pub const COPY_WRITE_BUFFER: GLenum = 0x8F37;
pub const CULL_FACE: GLenum = 0x0B44;
pub const CULL_FACE_MODE: GLenum = 0x0B45;
pub const CURRENT_PROGRAM: GLenum = 0x8B8D;
pub const CURRENT_QUERY: GLenum = 0x8865;
pub const CURRENT_VERTEX_ATTRIB: GLenum = 0x8626;
pub const CW: GLenum = 0x0900;
pub const DECR: GLenum = 0x1E03;
pub const DECR_WRAP: GLenum = 0x8508;
pub const DELETE_STATUS: GLenum = 0x8B80;
pub const DEPTH: GLenum = 0x1801;
pub const DEPTH24_STENCIL8: GLenum = 0x88F0;
pub const DEPTH32F_STENCIL8: GLenum = 0x8CAD;
pub const DEPTH_ATTACHMENT: GLenum = 0x8D00;
pub const DEPTH_BUFFER_BIT: GLenum = 0x00000100;
pub const DEPTH_CLAMP: GLenum = 0x864F;
pub const DEPTH_CLEAR_VALUE: GLenum = 0x0B73;
pub const DEPTH_COMPONENT: GLenum = 0x1902;
pub const DEPTH_COMPONENT16: GLenum = 0x81A5;
pub const DEPTH_COMPONENT24: GLenum = 0x81A6;
pub const DEPTH_COMPONENT32: GLenum = 0x81A7;
pub const DEPTH_COMPONENT32F: GLenum = 0x8CAC;
pub const DEPTH_FUNC: GLenum = 0x0B74;
pub const DEPTH_RANGE: GLenum = 0x0B70;
pub const DEPTH_STENCIL: GLenum = 0x84F9;
pub const DEPTH_STENCIL_ATTACHMENT: GLenum = 0x821A;
pub const DEPTH_TEST: GLenum = 0x0B71;
pub const DEPTH_WRITEMASK: GLenum = 0x0B72;
pub const DITHER: GLenum = 0x0BD0;
pub const DONT_CARE: GLenum = 0x1100;
pub const DOUBLE: GLenum = 0x140A;
pub const DOUBLEBUFFER: GLenum = 0x0C32;
pub const DRAW_BUFFER: GLenum = 0x0C01;
pub const DRAW_BUFFER0: GLenum = 0x8825;
pub const DRAW_BUFFER1: GLenum = 0x8826;
pub const DRAW_BUFFER10: GLenum = 0x882F;
pub const DRAW_BUFFER11: GLenum = 0x8830;
pub const DRAW_BUFFER12: GLenum = 0x8831;
pub const DRAW_BUFFER13: GLenum = 0x8832;
pub const DRAW_BUFFER14: GLenum = 0x8833;
pub const DRAW_BUFFER15: GLenum = 0x8834;
pub const DRAW_BUFFER2: GLenum = 0x8827;
pub const DRAW_BUFFER3: GLenum = 0x8828;
pub const DRAW_BUFFER4: GLenum = 0x8829;
pub const DRAW_BUFFER5: GLenum = 0x882A;
pub const DRAW_BUFFER6: GLenum = 0x882B;
pub const DRAW_BUFFER7: GLenum = 0x882C;
pub const DRAW_BUFFER8: GLenum = 0x882D;
pub const DRAW_BUFFER9: GLenum = 0x882E;
pub const DRAW_FRAMEBUFFER: GLenum = 0x8CA9;
pub const DRAW_FRAMEBUFFER_BINDING: GLenum = 0x8CA6;
pub const DST_ALPHA: GLenum = 0x0304;
pub const DST_COLOR: GLenum = 0x0306;
pub const DYNAMIC_COPY: GLenum = 0x88EA;
pub const DYNAMIC_DRAW: GLenum = 0x88E8;
pub const DYNAMIC_READ: GLenum = 0x88E9;
pub const ELEMENT_ARRAY_BUFFER: GLenum = 0x8893;
pub const ELEMENT_ARRAY_BUFFER_BINDING: GLenum = 0x8895;
pub const EQUAL: GLenum = 0x0202;
pub const EQUIV: GLenum = 0x1509;
pub const EXTENSIONS: GLenum = 0x1F03;
pub const FALSE: GLboolean = 0;
pub const FASTEST: GLenum = 0x1101;
pub const FILL: GLenum = 0x1B02;
pub const FIRST_VERTEX_CONVENTION: GLenum = 0x8E4D;
pub const FIXED_ONLY: GLenum = 0x891D;
pub const FLOAT: GLenum = 0x1406;
pub const FLOAT_32_UNSIGNED_INT_24_8_REV: GLenum = 0x8DAD;
pub const FLOAT_MAT2: GLenum = 0x8B5A;
pub const FLOAT_MAT2x3: GLenum = 0x8B65;
pub const FLOAT_MAT2x4: GLenum = 0x8B66;
pub const FLOAT_MAT3: GLenum = 0x8B5B;
pub const FLOAT_MAT3x2: GLenum = 0x8B67;
pub const FLOAT_MAT3x4: GLenum = 0x8B68;
pub const FLOAT_MAT4: GLenum = 0x8B5C;
pub const FLOAT_MAT4x2: GLenum = 0x8B69;
pub const FLOAT_MAT4x3: GLenum = 0x8B6A;
pub const FLOAT_VEC2: GLenum = 0x8B50;
pub const FLOAT_VEC3: GLenum = 0x8B51;
pub const FLOAT_VEC4: GLenum = 0x8B52;
pub const FRAGMENT_SHADER: GLenum = 0x8B30;
pub const FRAGMENT_SHADER_DERIVATIVE_HINT: GLenum = 0x8B8B;
pub const FRAMEBUFFER: GLenum = 0x8D40;
pub const FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE: GLenum = 0x8215;
pub const FRAMEBUFFER_ATTACHMENT_BLUE_SIZE: GLenum = 0x8214;
pub const FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING: GLenum = 0x8210;
pub const FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE: GLenum = 0x8211;
pub const FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE: GLenum = 0x8216;
pub const FRAMEBUFFER_ATTACHMENT_GREEN_SIZE: GLenum = 0x8213;
pub const FRAMEBUFFER_ATTACHMENT_LAYERED: GLenum = 0x8DA7;
pub const FRAMEBUFFER_ATTACHMENT_OBJECT_NAME: GLenum = 0x8CD1;
pub const FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE: GLenum = 0x8CD0;
pub const FRAMEBUFFER_ATTACHMENT_RED_SIZE: GLenum = 0x8212;
pub const FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE: GLenum = 0x8217;
pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE: GLenum = 0x8CD3;
pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER: GLenum = 0x8CD4;
pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL: GLenum = 0x8CD2;
pub const FRAMEBUFFER_BINDING: GLenum = 0x8CA6;
pub const FRAMEBUFFER_COMPLETE: GLenum = 0x8CD5;
pub const FRAMEBUFFER_DEFAULT: GLenum = 0x8218;
pub const FRAMEBUFFER_INCOMPLETE_ATTACHMENT: GLenum = 0x8CD6;
pub const FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER: GLenum = 0x8CDB;
pub const FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS: GLenum = 0x8DA8;
pub const FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT: GLenum = 0x8CD7;
pub const FRAMEBUFFER_INCOMPLETE_MULTISAMPLE: GLenum = 0x8D56;
pub const FRAMEBUFFER_INCOMPLETE_READ_BUFFER: GLenum = 0x8CDC;
pub const FRAMEBUFFER_SRGB: GLenum = 0x8DB9;
pub const FRAMEBUFFER_UNDEFINED: GLenum = 0x8219;
pub const FRAMEBUFFER_UNSUPPORTED: GLenum = 0x8CDD;
pub const FRONT: GLenum = 0x0404;
pub const FRONT_AND_BACK: GLenum = 0x0408;
pub const FRONT_FACE: GLenum = 0x0B46;
pub const FRONT_LEFT: GLenum = 0x0400;
pub const FRONT_RIGHT: GLenum = 0x0401;
pub const FUNC_ADD: GLenum = 0x8006;
pub const FUNC_REVERSE_SUBTRACT: GLenum = 0x800B;
pub const FUNC_SUBTRACT: GLenum = 0x800A;
pub const GEOMETRY_INPUT_TYPE: GLenum = 0x8917;
pub const GEOMETRY_OUTPUT_TYPE: GLenum = 0x8918;
pub const GEOMETRY_SHADER: GLenum = 0x8DD9;
pub const GEOMETRY_VERTICES_OUT: GLenum = 0x8916;
pub const GEQUAL: GLenum = 0x0206;
pub const GREATER: GLenum = 0x0204;
pub const GREEN: GLenum = 0x1904;
pub const GREEN_INTEGER: GLenum = 0x8D95;
pub const HALF_FLOAT: GLenum = 0x140B;
pub const INCR: GLenum = 0x1E02;
pub const INCR_WRAP: GLenum = 0x8507;
pub const INFO_LOG_LENGTH: GLenum = 0x8B84;
pub const INT: GLenum = 0x1404;
pub const INTERLEAVED_ATTRIBS: GLenum = 0x8C8C;
pub const INT_2_10_10_10_REV: GLenum = 0x8D9F;
pub const INT_SAMPLER_1D: GLenum = 0x8DC9;
pub const INT_SAMPLER_1D_ARRAY: GLenum = 0x8DCE;
pub const INT_SAMPLER_2D: GLenum = 0x8DCA;
pub const INT_SAMPLER_2D_ARRAY: GLenum = 0x8DCF;
pub const INT_SAMPLER_2D_MULTISAMPLE: GLenum = 0x9109;
pub const INT_SAMPLER_2D_MULTISAMPLE_ARRAY: GLenum = 0x910C;
pub const INT_SAMPLER_2D_RECT: GLenum = 0x8DCD;
pub const INT_SAMPLER_3D: GLenum = 0x8DCB;
pub const INT_SAMPLER_BUFFER: GLenum = 0x8DD0;
pub const INT_SAMPLER_CUBE: GLenum = 0x8DCC;
pub const INT_VEC2: GLenum = 0x8B53;
pub const INT_VEC3: GLenum = 0x8B54;
pub const INT_VEC4: GLenum = 0x8B55;
pub const INVALID_ENUM: GLenum = 0x0500;
pub const INVALID_FRAMEBUFFER_OPERATION: GLenum = 0x0506;
pub const INVALID_INDEX: GLuint = 0xFFFFFFFF;
pub const INVALID_OPERATION: GLenum = 0x0502;
pub const INVALID_VALUE: GLenum = 0x0501;
pub const INVERT: GLenum = 0x150A;
pub const KEEP: GLenum = 0x1E00;
pub const LAST_VERTEX_CONVENTION: GLenum = 0x8E4E;
pub const LEFT: GLenum = 0x0406;
pub const LEQUAL: GLenum = 0x0203;
pub const LESS: GLenum = 0x0201;
pub const LINE: GLenum = 0x1B01;
pub const LINEAR: GLenum = 0x2601;
pub const LINEAR_MIPMAP_LINEAR: GLenum = 0x2703;
pub const LINEAR_MIPMAP_NEAREST: GLenum = 0x2701;
pub const LINES: GLenum = 0x0001;
pub const LINES_ADJACENCY: GLenum = 0x000A;
pub const LINE_LOOP: GLenum = 0x0002;
pub const LINE_SMOOTH: GLenum = 0x0B20;
pub const LINE_SMOOTH_HINT: GLenum = 0x0C52;
pub const LINE_STRIP: GLenum = 0x0003;
pub const LINE_STRIP_ADJACENCY: GLenum = 0x000B;
pub const LINE_WIDTH: GLenum = 0x0B21;
pub const LINE_WIDTH_GRANULARITY: GLenum = 0x0B23;
pub const LINE_WIDTH_RANGE: GLenum = 0x0B22;
pub const LINK_STATUS: GLenum = 0x8B82;
pub const LOGIC_OP_MODE: GLenum = 0x0BF0;
pub const LOWER_LEFT: GLenum = 0x8CA1;
pub const MAJOR_VERSION: GLenum = 0x821B;
pub const MAP_FLUSH_EXPLICIT_BIT: GLenum = 0x0010;
pub const MAP_INVALIDATE_BUFFER_BIT: GLenum = 0x0008;
pub const MAP_INVALIDATE_RANGE_BIT: GLenum = 0x0004;
pub const MAP_READ_BIT: GLenum = 0x0001;
pub const MAP_UNSYNCHRONIZED_BIT: GLenum = 0x0020;
pub const MAP_WRITE_BIT: GLenum = 0x0002;
pub const MAX: GLenum = 0x8008;
pub const MAX_3D_TEXTURE_SIZE: GLenum = 0x8073;
pub const MAX_ARRAY_TEXTURE_LAYERS: GLenum = 0x88FF;
pub const MAX_CLIP_DISTANCES: GLenum = 0x0D32;
pub const MAX_COLOR_ATTACHMENTS: GLenum = 0x8CDF;
pub const MAX_COLOR_TEXTURE_SAMPLES: GLenum = 0x910E;
pub const MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS: GLenum = 0x8A33;
pub const MAX_COMBINED_GEOMETRY_UNIFORM_COMPONENTS: GLenum = 0x8A32;
pub const MAX_COMBINED_TEXTURE_IMAGE_UNITS: GLenum = 0x8B4D;
pub const MAX_COMBINED_UNIFORM_BLOCKS: GLenum = 0x8A2E;
pub const MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS: GLenum = 0x8A31;
pub const MAX_CUBE_MAP_TEXTURE_SIZE: GLenum = 0x851C;
pub const MAX_DEPTH_TEXTURE_SAMPLES: GLenum = 0x910F;
pub const MAX_DRAW_BUFFERS: GLenum = 0x8824;
pub const MAX_DUAL_SOURCE_DRAW_BUFFERS: GLenum = 0x88FC;
pub const MAX_ELEMENTS_INDICES: GLenum = 0x80E9;
pub const MAX_ELEMENTS_VERTICES: GLenum = 0x80E8;
pub const MAX_FRAGMENT_INPUT_COMPONENTS: GLenum = 0x9125;
pub const MAX_FRAGMENT_UNIFORM_BLOCKS: GLenum = 0x8A2D;
pub const MAX_FRAGMENT_UNIFORM_COMPONENTS: GLenum = 0x8B49;
pub const MAX_GEOMETRY_INPUT_COMPONENTS: GLenum = 0x9123;
pub const MAX_GEOMETRY_OUTPUT_COMPONENTS: GLenum = 0x9124;
pub const MAX_GEOMETRY_OUTPUT_VERTICES: GLenum = 0x8DE0;
pub const MAX_GEOMETRY_TEXTURE_IMAGE_UNITS: GLenum = 0x8C29;
pub const MAX_GEOMETRY_TOTAL_OUTPUT_COMPONENTS: GLenum = 0x8DE1;
pub const MAX_GEOMETRY_UNIFORM_BLOCKS: GLenum = 0x8A2C;
pub const MAX_GEOMETRY_UNIFORM_COMPONENTS: GLenum = 0x8DDF;
pub const MAX_INTEGER_SAMPLES: GLenum = 0x9110;
pub const MAX_PROGRAM_TEXEL_OFFSET: GLenum = 0x8905;
pub const MAX_RECTANGLE_TEXTURE_SIZE: GLenum = 0x84F8;
pub const MAX_RENDERBUFFER_SIZE: GLenum = 0x84E8;
pub const MAX_SAMPLES: GLenum = 0x8D57;
pub const MAX_SAMPLE_MASK_WORDS: GLenum = 0x8E59;
pub const MAX_SERVER_WAIT_TIMEOUT: GLenum = 0x9111;
pub const MAX_TEXTURE_BUFFER_SIZE: GLenum = 0x8C2B;
pub const MAX_TEXTURE_IMAGE_UNITS: GLenum = 0x8872;
pub const MAX_TEXTURE_LOD_BIAS: GLenum = 0x84FD;
pub const MAX_TEXTURE_SIZE: GLenum = 0x0D33;
pub const MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS: GLenum = 0x8C8A;
pub const MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS: GLenum = 0x8C8B;
pub const MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS: GLenum = 0x8C80;
pub const MAX_UNIFORM_BLOCK_SIZE: GLenum = 0x8A30;
pub const MAX_UNIFORM_BUFFER_BINDINGS: GLenum = 0x8A2F;
pub const MAX_VARYING_COMPONENTS: GLenum = 0x8B4B;
pub const MAX_VARYING_FLOATS: GLenum = 0x8B4B;
pub const MAX_VERTEX_ATTRIBS: GLenum = 0x8869;
pub const MAX_VERTEX_OUTPUT_COMPONENTS: GLenum = 0x9122;
pub const MAX_VERTEX_TEXTURE_IMAGE_UNITS: GLenum = 0x8B4C;
pub const MAX_VERTEX_UNIFORM_BLOCKS: GLenum = 0x8A2B;
pub const MAX_VERTEX_UNIFORM_COMPONENTS: GLenum = 0x8B4A;
pub const MAX_VIEWPORT_DIMS: GLenum = 0x0D3A;
pub const MIN: GLenum = 0x8007;
pub const MINOR_VERSION: GLenum = 0x821C;
pub const MIN_PROGRAM_TEXEL_OFFSET: GLenum = 0x8904;
pub const MIRRORED_REPEAT: GLenum = 0x8370;
pub const MULTISAMPLE: GLenum = 0x809D;
pub const NAND: GLenum = 0x150E;
pub const NEAREST: GLenum = 0x2600;
pub const NEAREST_MIPMAP_LINEAR: GLenum = 0x2702;
pub const NEAREST_MIPMAP_NEAREST: GLenum = 0x2700;
pub const NEVER: GLenum = 0x0200;
pub const NICEST: GLenum = 0x1102;
pub const NONE: GLenum = 0;
pub const NOOP: GLenum = 0x1505;
pub const NOR: GLenum = 0x1508;
pub const NOTEQUAL: GLenum = 0x0205;
pub const NO_ERROR: GLenum = 0;
pub const NUM_COMPRESSED_TEXTURE_FORMATS: GLenum = 0x86A2;
pub const NUM_EXTENSIONS: GLenum = 0x821D;
pub const OBJECT_TYPE: GLenum = 0x9112;
pub const ONE: GLenum = 1;
pub const ONE_MINUS_CONSTANT_ALPHA: GLenum = 0x8004;
pub const ONE_MINUS_CONSTANT_COLOR: GLenum = 0x8002;
pub const ONE_MINUS_DST_ALPHA: GLenum = 0x0305;
pub const ONE_MINUS_DST_COLOR: GLenum = 0x0307;
pub const ONE_MINUS_SRC1_ALPHA: GLenum = 0x88FB;
pub const ONE_MINUS_SRC1_COLOR: GLenum = 0x88FA;
pub const ONE_MINUS_SRC_ALPHA: GLenum = 0x0303;
pub const ONE_MINUS_SRC_COLOR: GLenum = 0x0301;
pub const OR: GLenum = 0x1507;
pub const OR_INVERTED: GLenum = 0x150D;
pub const OR_REVERSE: GLenum = 0x150B;
pub const OUT_OF_MEMORY: GLenum = 0x0505;
pub const PACK_ALIGNMENT: GLenum = 0x0D05;
pub const PACK_IMAGE_HEIGHT: GLenum = 0x806C;
pub const PACK_LSB_FIRST: GLenum = 0x0D01;
pub const PACK_ROW_LENGTH: GLenum = 0x0D02;
pub const PACK_SKIP_IMAGES: GLenum = 0x806B;
pub const PACK_SKIP_PIXELS: GLenum = 0x0D04;
pub const PACK_SKIP_ROWS: GLenum = 0x0D03;
pub const PACK_SWAP_BYTES: GLenum = 0x0D00;
pub const PIXEL_PACK_BUFFER: GLenum = 0x88EB;
pub const PIXEL_PACK_BUFFER_BINDING: GLenum = 0x88ED;
pub const PIXEL_UNPACK_BUFFER: GLenum = 0x88EC;
pub const PIXEL_UNPACK_BUFFER_BINDING: GLenum = 0x88EF;
pub const POINT: GLenum = 0x1B00;
pub const POINTS: GLenum = 0x0000;
pub const POINT_FADE_THRESHOLD_SIZE: GLenum = 0x8128;
pub const POINT_SIZE: GLenum = 0x0B11;
pub const POINT_SIZE_GRANULARITY: GLenum = 0x0B13;
pub const POINT_SIZE_RANGE: GLenum = 0x0B12;
pub const POINT_SPRITE_COORD_ORIGIN: GLenum = 0x8CA0;
pub const POLYGON_MODE: GLenum = 0x0B40;
pub const POLYGON_OFFSET_FACTOR: GLenum = 0x8038;
pub const POLYGON_OFFSET_FILL: GLenum = 0x8037;
pub const POLYGON_OFFSET_LINE: GLenum = 0x2A02;
pub const POLYGON_OFFSET_POINT: GLenum = 0x2A01;
pub const POLYGON_OFFSET_UNITS: GLenum = 0x2A00;
pub const POLYGON_SMOOTH: GLenum = 0x0B41;
pub const POLYGON_SMOOTH_HINT: GLenum = 0x0C53;
pub const PRIMITIVES_GENERATED: GLenum = 0x8C87;
pub const PRIMITIVE_RESTART: GLenum = 0x8F9D;
pub const PRIMITIVE_RESTART_INDEX: GLenum = 0x8F9E;
pub const PROGRAM_POINT_SIZE: GLenum = 0x8642;
pub const PROVOKING_VERTEX: GLenum = 0x8E4F;
pub const PROXY_TEXTURE_1D: GLenum = 0x8063;
pub const PROXY_TEXTURE_1D_ARRAY: GLenum = 0x8C19;
pub const PROXY_TEXTURE_2D: GLenum = 0x8064;
pub const PROXY_TEXTURE_2D_ARRAY: GLenum = 0x8C1B;
pub const PROXY_TEXTURE_2D_MULTISAMPLE: GLenum = 0x9101;
pub const PROXY_TEXTURE_2D_MULTISAMPLE_ARRAY: GLenum = 0x9103;
pub const PROXY_TEXTURE_3D: GLenum = 0x8070;
pub const PROXY_TEXTURE_CUBE_MAP: GLenum = 0x851B;
pub const PROXY_TEXTURE_RECTANGLE: GLenum = 0x84F7;
pub const QUADS_FOLLOW_PROVOKING_VERTEX_CONVENTION: GLenum = 0x8E4C;
pub const QUERY_BY_REGION_NO_WAIT: GLenum = 0x8E16;
pub const QUERY_BY_REGION_WAIT: GLenum = 0x8E15;
pub const QUERY_COUNTER_BITS: GLenum = 0x8864;
pub const QUERY_NO_WAIT: GLenum = 0x8E14;
pub const QUERY_RESULT: GLenum = 0x8866;
pub const QUERY_RESULT_AVAILABLE: GLenum = 0x8867;
pub const QUERY_WAIT: GLenum = 0x8E13;
pub const R11F_G11F_B10F: GLenum = 0x8C3A;
pub const R16: GLenum = 0x822A;
pub const R16F: GLenum = 0x822D;
pub const R16I: GLenum = 0x8233;
pub const R16UI: GLenum = 0x8234;
pub const R16_SNORM: GLenum = 0x8F98;
pub const R32F: GLenum = 0x822E;
pub const R32I: GLenum = 0x8235;
pub const R32UI: GLenum = 0x8236;
pub const R3_G3_B2: GLenum = 0x2A10;
pub const R8: GLenum = 0x8229;
pub const R8I: GLenum = 0x8231;
pub const R8UI: GLenum = 0x8232;
pub const R8_SNORM: GLenum = 0x8F94;
pub const RASTERIZER_DISCARD: GLenum = 0x8C89;
pub const READ_BUFFER: GLenum = 0x0C02;
pub const READ_FRAMEBUFFER: GLenum = 0x8CA8;
pub const READ_FRAMEBUFFER_BINDING: GLenum = 0x8CAA;
pub const READ_ONLY: GLenum = 0x88B8;
pub const READ_WRITE: GLenum = 0x88BA;
pub const RED: GLenum = 0x1903;
pub const RED_INTEGER: GLenum = 0x8D94;
pub const RENDERBUFFER: GLenum = 0x8D41;
pub const RENDERBUFFER_ALPHA_SIZE: GLenum = 0x8D53;
pub const RENDERBUFFER_BINDING: GLenum = 0x8CA7;
pub const RENDERBUFFER_BLUE_SIZE: GLenum = 0x8D52;
pub const RENDERBUFFER_DEPTH_SIZE: GLenum = 0x8D54;
pub const RENDERBUFFER_GREEN_SIZE: GLenum = 0x8D51;
pub const RENDERBUFFER_HEIGHT: GLenum = 0x8D43;
pub const RENDERBUFFER_INTERNAL_FORMAT: GLenum = 0x8D44;
pub const RENDERBUFFER_RED_SIZE: GLenum = 0x8D50;
pub const RENDERBUFFER_SAMPLES: GLenum = 0x8CAB;
pub const RENDERBUFFER_STENCIL_SIZE: GLenum = 0x8D55;
pub const RENDERBUFFER_WIDTH: GLenum = 0x8D42;
pub const RENDERER: GLenum = 0x1F01;
pub const REPEAT: GLenum = 0x2901;
pub const REPLACE: GLenum = 0x1E01;
pub const RG: GLenum = 0x8227;
pub const RG16: GLenum = 0x822C;
pub const RG16F: GLenum = 0x822F;
pub const RG16I: GLenum = 0x8239;
pub const RG16UI: GLenum = 0x823A;
pub const RG16_SNORM: GLenum = 0x8F99;
pub const RG32F: GLenum = 0x8230;
pub const RG32I: GLenum = 0x823B;
pub const RG32UI: GLenum = 0x823C;
pub const RG8: GLenum = 0x822B;
pub const RG8I: GLenum = 0x8237;
pub const RG8UI: GLenum = 0x8238;
pub const RG8_SNORM: GLenum = 0x8F95;
pub const RGB: GLenum = 0x1907;
pub const RGB10: GLenum = 0x8052;
pub const RGB10_A2: GLenum = 0x8059;
pub const RGB10_A2UI: GLenum = 0x906F;
pub const RGB12: GLenum = 0x8053;
pub const RGB16: GLenum = 0x8054;
pub const RGB16F: GLenum = 0x881B;
pub const RGB16I: GLenum = 0x8D89;
pub const RGB16UI: GLenum = 0x8D77;
pub const RGB16_SNORM: GLenum = 0x8F9A;
pub const RGB32F: GLenum = 0x8815;
pub const RGB32I: GLenum = 0x8D83;
pub const RGB32UI: GLenum = 0x8D71;
pub const RGB4: GLenum = 0x804F;
pub const RGB5: GLenum = 0x8050;
pub const RGB5_A1: GLenum = 0x8057;
pub const RGB8: GLenum = 0x8051;
pub const RGB8I: GLenum = 0x8D8F;
pub const RGB8UI: GLenum = 0x8D7D;
pub const RGB8_SNORM: GLenum = 0x8F96;
pub const RGB9_E5: GLenum = 0x8C3D;
pub const RGBA: GLenum = 0x1908;
pub const RGBA12: GLenum = 0x805A;
pub const RGBA16: GLenum = 0x805B;
pub const RGBA16F: GLenum = 0x881A;
pub const RGBA16I: GLenum = 0x8D88;
pub const RGBA16UI: GLenum = 0x8D76;
pub const RGBA16_SNORM: GLenum = 0x8F9B;
pub const RGBA2: GLenum = 0x8055;
pub const RGBA32F: GLenum = 0x8814;
pub const RGBA32I: GLenum = 0x8D82;
pub const RGBA32UI: GLenum = 0x8D70;
pub const RGBA4: GLenum = 0x8056;
pub const RGBA8: GLenum = 0x8058;
pub const RGBA8I: GLenum = 0x8D8E;
pub const RGBA8UI: GLenum = 0x8D7C;
pub const RGBA8_SNORM: GLenum = 0x8F97;
pub const RGBA_INTEGER: GLenum = 0x8D99;
pub const RGB_INTEGER: GLenum = 0x8D98;
pub const RG_INTEGER: GLenum = 0x8228;
pub const RIGHT: GLenum = 0x0407;
pub const SAMPLER_1D: GLenum = 0x8B5D;
pub const SAMPLER_1D_ARRAY: GLenum = 0x8DC0;
pub const SAMPLER_1D_ARRAY_SHADOW: GLenum = 0x8DC3;
pub const SAMPLER_1D_SHADOW: GLenum = 0x8B61;
pub const SAMPLER_2D: GLenum = 0x8B5E;
pub const SAMPLER_2D_ARRAY: GLenum = 0x8DC1;
pub const SAMPLER_2D_ARRAY_SHADOW: GLenum = 0x8DC4;
pub const SAMPLER_2D_MULTISAMPLE: GLenum = 0x9108;
pub const SAMPLER_2D_MULTISAMPLE_ARRAY: GLenum = 0x910B;
pub const SAMPLER_2D_RECT: GLenum = 0x8B63;
pub const SAMPLER_2D_RECT_SHADOW: GLenum = 0x8B64;
pub const SAMPLER_2D_SHADOW: GLenum = 0x8B62;
pub const SAMPLER_3D: GLenum = 0x8B5F;
pub const SAMPLER_BINDING: GLenum = 0x8919;
pub const SAMPLER_BUFFER: GLenum = 0x8DC2;
pub const SAMPLER_CUBE: GLenum = 0x8B60;
pub const SAMPLER_CUBE_SHADOW: GLenum = 0x8DC5;
pub const SAMPLES: GLenum = 0x80A9;
pub const SAMPLES_PASSED: GLenum = 0x8914;
pub const SAMPLE_ALPHA_TO_COVERAGE: GLenum = 0x809E;
pub const SAMPLE_ALPHA_TO_ONE: GLenum = 0x809F;
pub const SAMPLE_BUFFERS: GLenum = 0x80A8;
pub const SAMPLE_COVERAGE: GLenum = 0x80A0;
pub const SAMPLE_COVERAGE_INVERT: GLenum = 0x80AB;
pub const SAMPLE_COVERAGE_VALUE: GLenum = 0x80AA;
pub const SAMPLE_MASK: GLenum = 0x8E51;
pub const SAMPLE_MASK_VALUE: GLenum = 0x8E52;
pub const SAMPLE_POSITION: GLenum = 0x8E50;
pub const SCISSOR_BOX: GLenum = 0x0C10;
pub const SCISSOR_TEST: GLenum = 0x0C11;
pub const SEPARATE_ATTRIBS: GLenum = 0x8C8D;
pub const SET: GLenum = 0x150F;
pub const SHADER_SOURCE_LENGTH: GLenum = 0x8B88;
pub const SHADER_TYPE: GLenum = 0x8B4F;
pub const SHADING_LANGUAGE_VERSION: GLenum = 0x8B8C;
pub const SHORT: GLenum = 0x1402;
pub const SIGNALED: GLenum = 0x9119;
pub const SIGNED_NORMALIZED: GLenum = 0x8F9C;
pub const SMOOTH_LINE_WIDTH_GRANULARITY: GLenum = 0x0B23;
pub const SMOOTH_LINE_WIDTH_RANGE: GLenum = 0x0B22;
pub const SMOOTH_POINT_SIZE_GRANULARITY: GLenum = 0x0B13;
pub const SMOOTH_POINT_SIZE_RANGE: GLenum = 0x0B12;
pub const SRC1_ALPHA: GLenum = 0x8589;
pub const SRC1_COLOR: GLenum = 0x88F9;
pub const SRC_ALPHA: GLenum = 0x0302;
pub const SRC_ALPHA_SATURATE: GLenum = 0x0308;
pub const SRC_COLOR: GLenum = 0x0300;
pub const SRGB: GLenum = 0x8C40;
pub const SRGB8: GLenum = 0x8C41;
pub const SRGB8_ALPHA8: GLenum = 0x8C43;
pub const SRGB_ALPHA: GLenum = 0x8C42;
pub const STATIC_COPY: GLenum = 0x88E6;
pub const STATIC_DRAW: GLenum = 0x88E4;
pub const STATIC_READ: GLenum = 0x88E5;
pub const STENCIL: GLenum = 0x1802;
pub const STENCIL_ATTACHMENT: GLenum = 0x8D20;
pub const STENCIL_BACK_FAIL: GLenum = 0x8801;
pub const STENCIL_BACK_FUNC: GLenum = 0x8800;
pub const STENCIL_BACK_PASS_DEPTH_FAIL: GLenum = 0x8802;
pub const STENCIL_BACK_PASS_DEPTH_PASS: GLenum = 0x8803;
pub const STENCIL_BACK_REF: GLenum = 0x8CA3;
pub const STENCIL_BACK_VALUE_MASK: GLenum = 0x8CA4;
pub const STENCIL_BACK_WRITEMASK: GLenum = 0x8CA5;
pub const STENCIL_BUFFER_BIT: GLenum = 0x00000400;
pub const STENCIL_CLEAR_VALUE: GLenum = 0x0B91;
pub const STENCIL_FAIL: GLenum = 0x0B94;
pub const STENCIL_FUNC: GLenum = 0x0B92;
pub const STENCIL_INDEX: GLenum = 0x1901;
pub const STENCIL_INDEX1: GLenum = 0x8D46;
pub const STENCIL_INDEX16: GLenum = 0x8D49;
pub const STENCIL_INDEX4: GLenum = 0x8D47;
pub const STENCIL_INDEX8: GLenum = 0x8D48;
pub const STENCIL_PASS_DEPTH_FAIL: GLenum = 0x0B95;
pub const STENCIL_PASS_DEPTH_PASS: GLenum = 0x0B96;
pub const STENCIL_REF: GLenum = 0x0B97;
pub const STENCIL_TEST: GLenum = 0x0B90;
pub const STENCIL_VALUE_MASK: GLenum = 0x0B93;
pub const STENCIL_WRITEMASK: GLenum = 0x0B98;
pub const STEREO: GLenum = 0x0C33;
pub const STREAM_COPY: GLenum = 0x88E2;
pub const STREAM_DRAW: GLenum = 0x88E0;
pub const STREAM_READ: GLenum = 0x88E1;
pub const SUBPIXEL_BITS: GLenum = 0x0D50;
pub const SYNC_CONDITION: GLenum = 0x9113;
pub const SYNC_FENCE: GLenum = 0x9116;
pub const SYNC_FLAGS: GLenum = 0x9115;
pub const SYNC_FLUSH_COMMANDS_BIT: GLenum = 0x00000001;
pub const SYNC_GPU_COMMANDS_COMPLETE: GLenum = 0x9117;
pub const SYNC_STATUS: GLenum = 0x9114;
pub const TEXTURE: GLenum = 0x1702;
pub const TEXTURE0: GLenum = 0x84C0;
pub const TEXTURE1: GLenum = 0x84C1;
pub const TEXTURE10: GLenum = 0x84CA;
pub const TEXTURE11: GLenum = 0x84CB;
pub const TEXTURE12: GLenum = 0x84CC;
pub const TEXTURE13: GLenum = 0x84CD;
pub const TEXTURE14: GLenum = 0x84CE;
pub const TEXTURE15: GLenum = 0x84CF;
pub const TEXTURE16: GLenum = 0x84D0;
pub const TEXTURE17: GLenum = 0x84D1;
pub const TEXTURE18: GLenum = 0x84D2;
pub const TEXTURE19: GLenum = 0x84D3;
pub const TEXTURE2: GLenum = 0x84C2;
pub const TEXTURE20: GLenum = 0x84D4;
pub const TEXTURE21: GLenum = 0x84D5;
pub const TEXTURE22: GLenum = 0x84D6;
pub const TEXTURE23: GLenum = 0x84D7;
pub const TEXTURE24: GLenum = 0x84D8;
pub const TEXTURE25: GLenum = 0x84D9;
pub const TEXTURE26: GLenum = 0x84DA;
pub const TEXTURE27: GLenum = 0x84DB;
pub const TEXTURE28: GLenum = 0x84DC;
pub const TEXTURE29: GLenum = 0x84DD;
pub const TEXTURE3: GLenum = 0x84C3;
pub const TEXTURE30: GLenum = 0x84DE;
pub const TEXTURE31: GLenum = 0x84DF;
pub const TEXTURE4: GLenum = 0x84C4;
pub const TEXTURE5: GLenum = 0x84C5;
pub const TEXTURE6: GLenum = 0x84C6;
pub const TEXTURE7: GLenum = 0x84C7;
pub const TEXTURE8: GLenum = 0x84C8;
pub const TEXTURE9: GLenum = 0x84C9;
pub const TEXTURE_1D: GLenum = 0x0DE0;
pub const TEXTURE_1D_ARRAY: GLenum = 0x8C18;
pub const TEXTURE_2D: GLenum = 0x0DE1;
pub const TEXTURE_2D_ARRAY: GLenum = 0x8C1A;
pub const TEXTURE_2D_MULTISAMPLE: GLenum = 0x9100;
pub const TEXTURE_2D_MULTISAMPLE_ARRAY: GLenum = 0x9102;
pub const TEXTURE_3D: GLenum = 0x806F;
pub const TEXTURE_ALPHA_SIZE: GLenum = 0x805F;
pub const TEXTURE_ALPHA_TYPE: GLenum = 0x8C13;
pub const TEXTURE_BASE_LEVEL: GLenum = 0x813C;
pub const TEXTURE_BINDING_1D: GLenum = 0x8068;
pub const TEXTURE_BINDING_1D_ARRAY: GLenum = 0x8C1C;
pub const TEXTURE_BINDING_2D: GLenum = 0x8069;
pub const TEXTURE_BINDING_2D_ARRAY: GLenum = 0x8C1D;
pub const TEXTURE_BINDING_2D_MULTISAMPLE: GLenum = 0x9104;
pub const TEXTURE_BINDING_2D_MULTISAMPLE_ARRAY: GLenum = 0x9105;
pub const TEXTURE_BINDING_3D: GLenum = 0x806A;
pub const TEXTURE_BINDING_BUFFER: GLenum = 0x8C2C;
pub const TEXTURE_BINDING_CUBE_MAP: GLenum = 0x8514;
pub const TEXTURE_BINDING_RECTANGLE: GLenum = 0x84F6;
pub const TEXTURE_BLUE_SIZE: GLenum = 0x805E;
pub const TEXTURE_BLUE_TYPE: GLenum = 0x8C12;
pub const TEXTURE_BORDER_COLOR: GLenum = 0x1004;
pub const TEXTURE_BUFFER: GLenum = 0x8C2A;
pub const TEXTURE_BUFFER_DATA_STORE_BINDING: GLenum = 0x8C2D;
pub const TEXTURE_COMPARE_FUNC: GLenum = 0x884D;
pub const TEXTURE_COMPARE_MODE: GLenum = 0x884C;
pub const TEXTURE_COMPRESSED: GLenum = 0x86A1;
pub const TEXTURE_COMPRESSED_IMAGE_SIZE: GLenum = 0x86A0;
pub const TEXTURE_COMPRESSION_HINT: GLenum = 0x84EF;
pub const TEXTURE_CUBE_MAP: GLenum = 0x8513;
pub const TEXTURE_CUBE_MAP_NEGATIVE_X: GLenum = 0x8516;
pub const TEXTURE_CUBE_MAP_NEGATIVE_Y: GLenum = 0x8518;
pub const TEXTURE_CUBE_MAP_NEGATIVE_Z: GLenum = 0x851A;
pub const TEXTURE_CUBE_MAP_POSITIVE_X: GLenum = 0x8515;
pub const TEXTURE_CUBE_MAP_POSITIVE_Y: GLenum = 0x8517;
pub const TEXTURE_CUBE_MAP_POSITIVE_Z: GLenum = 0x8519;
pub const TEXTURE_CUBE_MAP_SEAMLESS: GLenum = 0x884F;
pub const TEXTURE_DEPTH: GLenum = 0x8071;
pub const TEXTURE_DEPTH_SIZE: GLenum = 0x884A;
pub const TEXTURE_DEPTH_TYPE: GLenum = 0x8C16;
pub const TEXTURE_FIXED_SAMPLE_LOCATIONS: GLenum = 0x9107;
pub const TEXTURE_GREEN_SIZE: GLenum = 0x805D;
pub const TEXTURE_GREEN_TYPE: GLenum = 0x8C11;
pub const TEXTURE_HEIGHT: GLenum = 0x1001;
pub const TEXTURE_INTERNAL_FORMAT: GLenum = 0x1003;
pub const TEXTURE_LOD_BIAS: GLenum = 0x8501;
pub const TEXTURE_MAG_FILTER: GLenum = 0x2800;
pub const TEXTURE_MAX_LEVEL: GLenum = 0x813D;
pub const TEXTURE_MAX_LOD: GLenum = 0x813B;
pub const TEXTURE_MIN_FILTER: GLenum = 0x2801;
pub const TEXTURE_MIN_LOD: GLenum = 0x813A;
pub const TEXTURE_RECTANGLE: GLenum = 0x84F5;
pub const TEXTURE_RED_SIZE: GLenum = 0x805C;
pub const TEXTURE_RED_TYPE: GLenum = 0x8C10;
pub const TEXTURE_SAMPLES: GLenum = 0x9106;
pub const TEXTURE_SHARED_SIZE: GLenum = 0x8C3F;
pub const TEXTURE_STENCIL_SIZE: GLenum = 0x88F1;
pub const TEXTURE_SWIZZLE_A: GLenum = 0x8E45;
pub const TEXTURE_SWIZZLE_B: GLenum = 0x8E44;
pub const TEXTURE_SWIZZLE_G: GLenum = 0x8E43;
pub const TEXTURE_SWIZZLE_R: GLenum = 0x8E42;
pub const TEXTURE_SWIZZLE_RGBA: GLenum = 0x8E46;
pub const TEXTURE_WIDTH: GLenum = 0x1000;
pub const TEXTURE_WRAP_R: GLenum = 0x8072;
pub const TEXTURE_WRAP_S: GLenum = 0x2802;
pub const TEXTURE_WRAP_T: GLenum = 0x2803;
pub const TIMEOUT_EXPIRED: GLenum = 0x911B;
pub const TIMEOUT_IGNORED: GLuint64 = 0xFFFFFFFFFFFFFFFF;
pub const TIMESTAMP: GLenum = 0x8E28;
pub const TIME_ELAPSED: GLenum = 0x88BF;
pub const TRANSFORM_FEEDBACK_BUFFER: GLenum = 0x8C8E;
pub const TRANSFORM_FEEDBACK_BUFFER_BINDING: GLenum = 0x8C8F;
pub const TRANSFORM_FEEDBACK_BUFFER_MODE: GLenum = 0x8C7F;
pub const TRANSFORM_FEEDBACK_BUFFER_SIZE: GLenum = 0x8C85;
pub const TRANSFORM_FEEDBACK_BUFFER_START: GLenum = 0x8C84;
pub const TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN: GLenum = 0x8C88;
pub const TRANSFORM_FEEDBACK_VARYINGS: GLenum = 0x8C83;
pub const TRANSFORM_FEEDBACK_VARYING_MAX_LENGTH: GLenum = 0x8C76;
pub const TRIANGLES: GLenum = 0x0004;
pub const TRIANGLES_ADJACENCY: GLenum = 0x000C;
pub const TRIANGLE_FAN: GLenum = 0x0006;
pub const TRIANGLE_STRIP: GLenum = 0x0005;
pub const TRIANGLE_STRIP_ADJACENCY: GLenum = 0x000D;
pub const TRUE: GLboolean = 1;
pub const UNIFORM_ARRAY_STRIDE: GLenum = 0x8A3C;
pub const UNIFORM_BLOCK_ACTIVE_UNIFORMS: GLenum = 0x8A42;
pub const UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES: GLenum = 0x8A43;
pub const UNIFORM_BLOCK_BINDING: GLenum = 0x8A3F;
pub const UNIFORM_BLOCK_DATA_SIZE: GLenum = 0x8A40;
pub const UNIFORM_BLOCK_INDEX: GLenum = 0x8A3A;
pub const UNIFORM_BLOCK_NAME_LENGTH: GLenum = 0x8A41;
pub const UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER: GLenum = 0x8A46;
pub const UNIFORM_BLOCK_REFERENCED_BY_GEOMETRY_SHADER: GLenum = 0x8A45;
pub const UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER: GLenum = 0x8A44;
pub const UNIFORM_BUFFER: GLenum = 0x8A11;
pub const UNIFORM_BUFFER_BINDING: GLenum = 0x8A28;
pub const UNIFORM_BUFFER_OFFSET_ALIGNMENT: GLenum = 0x8A34;
pub const UNIFORM_BUFFER_SIZE: GLenum = 0x8A2A;
pub const UNIFORM_BUFFER_START: GLenum = 0x8A29;
pub const UNIFORM_IS_ROW_MAJOR: GLenum = 0x8A3E;
pub const UNIFORM_MATRIX_STRIDE: GLenum = 0x8A3D;
pub const UNIFORM_NAME_LENGTH: GLenum = 0x8A39;
pub const UNIFORM_OFFSET: GLenum = 0x8A3B;
pub const UNIFORM_SIZE: GLenum = 0x8A38;
pub const UNIFORM_TYPE: GLenum = 0x8A37;
pub const UNPACK_ALIGNMENT: GLenum = 0x0CF5;
pub const UNPACK_IMAGE_HEIGHT: GLenum = 0x806E;
pub const UNPACK_LSB_FIRST: GLenum = 0x0CF1;
pub const UNPACK_ROW_LENGTH: GLenum = 0x0CF2;
pub const UNPACK_SKIP_IMAGES: GLenum = 0x806D;
pub const UNPACK_SKIP_PIXELS: GLenum = 0x0CF4;
pub const UNPACK_SKIP_ROWS: GLenum = 0x0CF3;
pub const UNPACK_SWAP_BYTES: GLenum = 0x0CF0;
pub const UNSIGNALED: GLenum = 0x9118;
pub const UNSIGNED_BYTE: GLenum = 0x1401;
pub const UNSIGNED_BYTE_2_3_3_REV: GLenum = 0x8362;
pub const UNSIGNED_BYTE_3_3_2: GLenum = 0x8032;
pub const UNSIGNED_INT: GLenum = 0x1405;
pub const UNSIGNED_INT_10F_11F_11F_REV: GLenum = 0x8C3B;
pub const UNSIGNED_INT_10_10_10_2: GLenum = 0x8036;
pub const UNSIGNED_INT_24_8: GLenum = 0x84FA;
pub const UNSIGNED_INT_2_10_10_10_REV: GLenum = 0x8368;
pub const UNSIGNED_INT_5_9_9_9_REV: GLenum = 0x8C3E;
pub const UNSIGNED_INT_8_8_8_8: GLenum = 0x8035;
pub const UNSIGNED_INT_8_8_8_8_REV: GLenum = 0x8367;
pub const UNSIGNED_INT_SAMPLER_1D: GLenum = 0x8DD1;
pub const UNSIGNED_INT_SAMPLER_1D_ARRAY: GLenum = 0x8DD6;
pub const UNSIGNED_INT_SAMPLER_2D: GLenum = 0x8DD2;
pub const UNSIGNED_INT_SAMPLER_2D_ARRAY: GLenum = 0x8DD7;
pub const UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE: GLenum = 0x910A;
pub const UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE_ARRAY: GLenum = 0x910D;
pub const UNSIGNED_INT_SAMPLER_2D_RECT: GLenum = 0x8DD5;
pub const UNSIGNED_INT_SAMPLER_3D: GLenum = 0x8DD3;
pub const UNSIGNED_INT_SAMPLER_BUFFER: GLenum = 0x8DD8;
pub const UNSIGNED_INT_SAMPLER_CUBE: GLenum = 0x8DD4;
pub const UNSIGNED_INT_VEC2: GLenum = 0x8DC6;
pub const UNSIGNED_INT_VEC3: GLenum = 0x8DC7;
pub const UNSIGNED_INT_VEC4: GLenum = 0x8DC8;
pub const UNSIGNED_NORMALIZED: GLenum = 0x8C17;
pub const UNSIGNED_SHORT: GLenum = 0x1403;
pub const UNSIGNED_SHORT_1_5_5_5_REV: GLenum = 0x8366;
pub const UNSIGNED_SHORT_4_4_4_4: GLenum = 0x8033;
pub const UNSIGNED_SHORT_4_4_4_4_REV: GLenum = 0x8365;
pub const UNSIGNED_SHORT_5_5_5_1: GLenum = 0x8034;
pub const UNSIGNED_SHORT_5_6_5: GLenum = 0x8363;
pub const UNSIGNED_SHORT_5_6_5_REV: GLenum = 0x8364;
pub const UPPER_LEFT: GLenum = 0x8CA2;
pub const VALIDATE_STATUS: GLenum = 0x8B83;
pub const VENDOR: GLenum = 0x1F00;
pub const VERSION: GLenum = 0x1F02;
pub const VERTEX_ARRAY_BINDING: GLenum = 0x85B5;
pub const VERTEX_ATTRIB_ARRAY_BUFFER_BINDING: GLenum = 0x889F;
pub const VERTEX_ATTRIB_ARRAY_DIVISOR: GLenum = 0x88FE;
pub const VERTEX_ATTRIB_ARRAY_ENABLED: GLenum = 0x8622;
pub const VERTEX_ATTRIB_ARRAY_INTEGER: GLenum = 0x88FD;
pub const VERTEX_ATTRIB_ARRAY_NORMALIZED: GLenum = 0x886A;
pub const VERTEX_ATTRIB_ARRAY_POINTER: GLenum = 0x8645;
pub const VERTEX_ATTRIB_ARRAY_SIZE: GLenum = 0x8623;
pub const VERTEX_ATTRIB_ARRAY_STRIDE: GLenum = 0x8624;
pub const VERTEX_ATTRIB_ARRAY_TYPE: GLenum = 0x8625;
pub const VERTEX_PROGRAM_POINT_SIZE: GLenum = 0x8642;
pub const VERTEX_SHADER: GLenum = 0x8B31;
pub const VIEWPORT: GLenum = 0x0BA2;
pub const WAIT_FAILED: GLenum = 0x911D;
pub const WRITE_ONLY: GLenum = 0x88B9;
pub const XOR: GLenum = 0x1506;
pub const ZERO: GLenum = 0;

#[link(name = "GL")]
unsafe extern "system" {
  #[link_name = "glActiveTexture"]
  pub fn ActiveTexture(texture: GLenum);
  #[link_name = "glAttachShader"]
  pub fn AttachShader(program: GLuint, shader: GLuint);
  #[link_name = "glBeginConditionalRender"]
  pub fn BeginConditionalRender(id: GLuint, mode: GLenum);
  #[link_name = "glBeginQuery"]
  pub fn BeginQuery(target: GLenum, id: GLuint);
  #[link_name = "glBeginTransformFeedback"]
  pub fn BeginTransformFeedback(primitiveMode: GLenum);
  #[link_name = "glBindAttribLocation"]
  pub fn BindAttribLocation(program: GLuint, index: GLuint, name: *const GLchar);
  #[link_name = "glBindBuffer"]
  pub fn BindBuffer(target: GLenum, buffer: GLuint);
  #[link_name = "glBindBufferBase"]
  pub fn BindBufferBase(target: GLenum, index: GLuint, buffer: GLuint);
  #[link_name = "glBindBufferRange"]
  pub fn BindBufferRange(
    target: GLenum,
    index: GLuint,
    buffer: GLuint,
    offset: GLintptr,
    size: GLsizeiptr,
  );
  #[link_name = "glBindFragDataLocation"]
  pub fn BindFragDataLocation(program: GLuint, color: GLuint, name: *const GLchar);
  #[link_name = "glBindFragDataLocationIndexed"]
  pub fn BindFragDataLocationIndexed(
    program: GLuint,
    colorNumber: GLuint,
    index: GLuint,
    name: *const GLchar,
  );
  #[link_name = "glBindFramebuffer"]
  pub fn BindFramebuffer(target: GLenum, framebuffer: GLuint);
  #[link_name = "glBindRenderbuffer"]
  pub fn BindRenderbuffer(target: GLenum, renderbuffer: GLuint);
  #[link_name = "glBindSampler"]
  pub fn BindSampler(unit: GLuint, sampler: GLuint);
  #[link_name = "glBindTexture"]
  pub fn BindTexture(target: GLenum, texture: GLuint);
  #[link_name = "glBindVertexArray"]
  pub fn BindVertexArray(array: GLuint);
  #[link_name = "glBlendColor"]
  pub fn BlendColor(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat);
  #[link_name = "glBlendEquation"]
  pub fn BlendEquation(mode: GLenum);
  #[link_name = "glBlendEquationSeparate"]
  pub fn BlendEquationSeparate(modeRGB: GLenum, modeAlpha: GLenum);
  #[link_name = "glBlendFunc"]
  pub fn BlendFunc(sfactor: GLenum, dfactor: GLenum);
  #[link_name = "glBlendFuncSeparate"]
  pub fn BlendFuncSeparate(
    sfactorRGB: GLenum,
    dfactorRGB: GLenum,
    sfactorAlpha: GLenum,
    dfactorAlpha: GLenum,
  );
  #[link_name = "glBlitFramebuffer"]
  pub fn BlitFramebuffer(
    srcX0: GLint,
    srcY0: GLint,
    srcX1: GLint,
    srcY1: GLint,
    dstX0: GLint,
    dstY0: GLint,
    dstX1: GLint,
    dstY1: GLint,
    mask: GLbitfield,
    filter: GLenum,
  );
  #[link_name = "glBufferData"]
  pub fn BufferData(target: GLenum, size: GLsizeiptr, data: *const c_void, usage: GLenum);
  #[link_name = "glBufferSubData"]
  pub fn BufferSubData(target: GLenum, offset: GLintptr, size: GLsizeiptr, data: *const c_void);
  #[link_name = "glCheckFramebufferStatus"]
  pub fn CheckFramebufferStatus(target: GLenum) -> GLenum;
  #[link_name = "glClampColor"]
  pub fn ClampColor(target: GLenum, clamp: GLenum);
  #[link_name = "glClear"]
  pub fn Clear(mask: GLbitfield);
  #[link_name = "glClearBufferfi"]
  pub fn ClearBufferfi(buffer: GLenum, drawbuffer: GLint, depth: GLfloat, stencil: GLint);
  #[link_name = "glClearBufferfv"]
  pub fn ClearBufferfv(buffer: GLenum, drawbuffer: GLint, value: *const GLfloat);
  #[link_name = "glClearBufferiv"]
  pub fn ClearBufferiv(buffer: GLenum, drawbuffer: GLint, value: *const GLint);
  #[link_name = "glClearBufferuiv"]
  pub fn ClearBufferuiv(buffer: GLenum, drawbuffer: GLint, value: *const GLuint);
  #[link_name = "glClearColor"]
  pub fn ClearColor(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat);
  #[link_name = "glClearDepth"]
  pub fn ClearDepth(depth: GLdouble);
  #[link_name = "glClearStencil"]
  pub fn ClearStencil(s: GLint);
  #[link_name = "glColorMask"]
  pub fn ColorMask(red: GLboolean, green: GLboolean, blue: GLboolean, alpha: GLboolean);
  #[link_name = "glColorMaski"]
  pub fn ColorMaski(index: GLuint, r: GLboolean, g: GLboolean, b: GLboolean, a: GLboolean);
  #[link_name = "glColorP3ui"]
  pub fn ColorP3ui(type_: GLenum, color: GLuint);
  #[link_name = "glColorP3uiv"]
  pub fn ColorP3uiv(type_: GLenum, color: *const GLuint);
  #[link_name = "glColorP4ui"]
  pub fn ColorP4ui(type_: GLenum, color: GLuint);
  #[link_name = "glColorP4uiv"]
  pub fn ColorP4uiv(type_: GLenum, color: *const GLuint);
  #[link_name = "glCompileShader"]
  pub fn CompileShader(shader: GLuint);
  #[link_name = "glCompressedTexImage1D"]
  pub fn CompressedTexImage1D(
    target: GLenum,
    level: GLint,
    internalformat: GLenum,
    width: GLsizei,
    border: GLint,
    imageSize: GLsizei,
    data: *const c_void,
  );
  #[link_name = "glCompressedTexImage2D"]
  pub fn CompressedTexImage2D(
    target: GLenum,
    level: GLint,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
    border: GLint,
    imageSize: GLsizei,
    data: *const c_void,
  );
  #[link_name = "glCompressedTexImage3D"]
  pub fn CompressedTexImage3D(
    target: GLenum,
    level: GLint,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
    depth: GLsizei,
    border: GLint,
    imageSize: GLsizei,
    data: *const c_void,
  );
  #[link_name = "glCompressedTexSubImage1D"]
  pub fn CompressedTexSubImage1D(
    target: GLenum,
    level: GLint,
    xoffset: GLint,
    width: GLsizei,
    format: GLenum,
    imageSize: GLsizei,
    data: *const c_void,
  );
  #[link_name = "glCompressedTexSubImage2D"]
  pub fn CompressedTexSubImage2D(
    target: GLenum,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    width: GLsizei,
    height: GLsizei,
    format: GLenum,
    imageSize: GLsizei,
    data: *const c_void,
  );
  #[link_name = "glCompressedTexSubImage3D"]
  pub fn CompressedTexSubImage3D(
    target: GLenum,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    zoffset: GLint,
    width: GLsizei,
    height: GLsizei,
    depth: GLsizei,
    format: GLenum,
    imageSize: GLsizei,
    data: *const c_void,
  );
  #[link_name = "glCopyBufferSubData"]
  pub fn CopyBufferSubData(
    readTarget: GLenum,
    writeTarget: GLenum,
    readOffset: GLintptr,
    writeOffset: GLintptr,
    size: GLsizeiptr,
  );
  #[link_name = "glCopyTexImage1D"]
  pub fn CopyTexImage1D(
    target: GLenum,
    level: GLint,
    internalformat: GLenum,
    x: GLint,
    y: GLint,
    width: GLsizei,
    border: GLint,
  );
  #[link_name = "glCopyTexImage2D"]
  pub fn CopyTexImage2D(
    target: GLenum,
    level: GLint,
    internalformat: GLenum,
    x: GLint,
    y: GLint,
    width: GLsizei,
    height: GLsizei,
    border: GLint,
  );
  #[link_name = "glCopyTexSubImage1D"]
  pub fn CopyTexSubImage1D(
    target: GLenum,
    level: GLint,
    xoffset: GLint,
    x: GLint,
    y: GLint,
    width: GLsizei,
  );
  #[link_name = "glCopyTexSubImage2D"]
  pub fn CopyTexSubImage2D(
    target: GLenum,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    x: GLint,
    y: GLint,
    width: GLsizei,
    height: GLsizei,
  );
  #[link_name = "glCopyTexSubImage3D"]
  pub fn CopyTexSubImage3D(
    target: GLenum,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    zoffset: GLint,
    x: GLint,
    y: GLint,
    width: GLsizei,
    height: GLsizei,
  );
  #[link_name = "glCreateProgram"]
  pub fn CreateProgram() -> GLuint;
  #[link_name = "glCreateShader"]
  pub fn CreateShader(type_: GLenum) -> GLuint;
  #[link_name = "glCullFace"]
  pub fn CullFace(mode: GLenum);
  #[link_name = "glDeleteBuffers"]
  pub fn DeleteBuffers(n: GLsizei, buffers: *const GLuint);
  #[link_name = "glDeleteFramebuffers"]
  pub fn DeleteFramebuffers(n: GLsizei, framebuffers: *const GLuint);
  #[link_name = "glDeleteProgram"]
  pub fn DeleteProgram(program: GLuint);
  #[link_name = "glDeleteQueries"]
  pub fn DeleteQueries(n: GLsizei, ids: *const GLuint);
  #[link_name = "glDeleteRenderbuffers"]
  pub fn DeleteRenderbuffers(n: GLsizei, renderbuffers: *const GLuint);
  #[link_name = "glDeleteSamplers"]
  pub fn DeleteSamplers(count: GLsizei, samplers: *const GLuint);
  #[link_name = "glDeleteShader"]
  pub fn DeleteShader(shader: GLuint);
  #[link_name = "glDeleteTextures"]
  pub fn DeleteTextures(n: GLsizei, textures: *const GLuint);
  #[link_name = "glDeleteVertexArrays"]
  pub fn DeleteVertexArrays(n: GLsizei, arrays: *const GLuint);
  #[link_name = "glDepthFunc"]
  pub fn DepthFunc(func: GLenum);
  #[link_name = "glDepthMask"]
  pub fn DepthMask(flag: GLboolean);
  #[link_name = "glDepthRange"]
  pub fn DepthRange(n: GLdouble, f: GLdouble);
  #[link_name = "glDetachShader"]
  pub fn DetachShader(program: GLuint, shader: GLuint);
  #[link_name = "glDisable"]
  pub fn Disable(cap: GLenum);
  #[link_name = "glDisableVertexAttribArray"]
  pub fn DisableVertexAttribArray(index: GLuint);
  #[link_name = "glDisablei"]
  pub fn Disablei(target: GLenum, index: GLuint);
  #[link_name = "glDrawArrays"]
  pub fn DrawArrays(mode: GLenum, first: GLint, count: GLsizei);
  #[link_name = "glDrawArraysInstanced"]
  pub fn DrawArraysInstanced(mode: GLenum, first: GLint, count: GLsizei, instancecount: GLsizei);
  #[link_name = "glDrawBuffer"]
  pub fn DrawBuffer(buf: GLenum);
  #[link_name = "glDrawBuffers"]
  pub fn DrawBuffers(n: GLsizei, bufs: *const GLenum);
  #[link_name = "glDrawElements"]
  pub fn DrawElements(mode: GLenum, count: GLsizei, type_: GLenum, indices: *const c_void);
  #[link_name = "glDrawElementsBaseVertex"]
  pub fn DrawElementsBaseVertex(
    mode: GLenum,
    count: GLsizei,
    type_: GLenum,
    indices: *const c_void,
    basevertex: GLint,
  );
  #[link_name = "glDrawElementsInstanced"]
  pub fn DrawElementsInstanced(
    mode: GLenum,
    count: GLsizei,
    type_: GLenum,
    indices: *const c_void,
    instancecount: GLsizei,
  );
  #[link_name = "glDrawElementsInstancedBaseVertex"]
  pub fn DrawElementsInstancedBaseVertex(
    mode: GLenum,
    count: GLsizei,
    type_: GLenum,
    indices: *const c_void,
    instancecount: GLsizei,
    basevertex: GLint,
  );
  #[link_name = "glDrawRangeElements"]
  pub fn DrawRangeElements(
    mode: GLenum,
    start: GLuint,
    end: GLuint,
    count: GLsizei,
    type_: GLenum,
    indices: *const c_void,
  );
  #[link_name = "glDrawRangeElementsBaseVertex"]
  pub fn DrawRangeElementsBaseVertex(
    mode: GLenum,
    start: GLuint,
    end: GLuint,
    count: GLsizei,
    type_: GLenum,
    indices: *const c_void,
    basevertex: GLint,
  );
  #[link_name = "glEnable"]
  pub fn Enable(cap: GLenum);
  #[link_name = "glEnableVertexAttribArray"]
  pub fn EnableVertexAttribArray(index: GLuint);
  #[link_name = "glEnablei"]
  pub fn Enablei(target: GLenum, index: GLuint);
  #[link_name = "glEndConditionalRender"]
  pub fn EndConditionalRender();
  #[link_name = "glEndQuery"]
  pub fn EndQuery(target: GLenum);
  #[link_name = "glEndTransformFeedback"]
  pub fn EndTransformFeedback();
  #[link_name = "glFinish"]
  pub fn Finish();
  #[link_name = "glFlush"]
  pub fn Flush();
  #[link_name = "glFlushMappedBufferRange"]
  pub fn FlushMappedBufferRange(target: GLenum, offset: GLintptr, length: GLsizeiptr);
  #[link_name = "glFramebufferRenderbuffer"]
  pub fn FramebufferRenderbuffer(
    target: GLenum,
    attachment: GLenum,
    renderbuffertarget: GLenum,
    renderbuffer: GLuint,
  );
  #[link_name = "glFramebufferTexture"]
  pub fn FramebufferTexture(target: GLenum, attachment: GLenum, texture: GLuint, level: GLint);
  #[link_name = "glFramebufferTexture1D"]
  pub fn FramebufferTexture1D(
    target: GLenum,
    attachment: GLenum,
    textarget: GLenum,
    texture: GLuint,
    level: GLint,
  );
  #[link_name = "glFramebufferTexture2D"]
  pub fn FramebufferTexture2D(
    target: GLenum,
    attachment: GLenum,
    textarget: GLenum,
    texture: GLuint,
    level: GLint,
  );
  #[link_name = "glFramebufferTexture3D"]
  pub fn FramebufferTexture3D(
    target: GLenum,
    attachment: GLenum,
    textarget: GLenum,
    texture: GLuint,
    level: GLint,
    zoffset: GLint,
  );
  #[link_name = "glFramebufferTextureLayer"]
  pub fn FramebufferTextureLayer(
    target: GLenum,
    attachment: GLenum,
    texture: GLuint,
    level: GLint,
    layer: GLint,
  );
  #[link_name = "glFrontFace"]
  pub fn FrontFace(mode: GLenum);
  #[link_name = "glGenBuffers"]
  pub fn GenBuffers(n: GLsizei, buffers: *mut GLuint);
  #[link_name = "glGenFramebuffers"]
  pub fn GenFramebuffers(n: GLsizei, framebuffers: *mut GLuint);
  #[link_name = "glGenQueries"]
  pub fn GenQueries(n: GLsizei, ids: *mut GLuint);
  #[link_name = "glGenRenderbuffers"]
  pub fn GenRenderbuffers(n: GLsizei, renderbuffers: *mut GLuint);
  #[link_name = "glGenSamplers"]
  pub fn GenSamplers(count: GLsizei, samplers: *mut GLuint);
  #[link_name = "glGenTextures"]
  pub fn GenTextures(n: GLsizei, textures: *mut GLuint);
  #[link_name = "glGenVertexArrays"]
  pub fn GenVertexArrays(n: GLsizei, arrays: *mut GLuint);
  #[link_name = "glGenerateMipmap"]
  pub fn GenerateMipmap(target: GLenum);
  #[link_name = "glGetActiveAttrib"]
  pub fn GetActiveAttrib(
    program: GLuint,
    index: GLuint,
    bufSize: GLsizei,
    length: *mut GLsizei,
    size: *mut GLint,
    type_: *mut GLenum,
    name: *mut GLchar,
  );
  #[link_name = "glGetActiveUniform"]
  pub fn GetActiveUniform(
    program: GLuint,
    index: GLuint,
    bufSize: GLsizei,
    length: *mut GLsizei,
    size: *mut GLint,
    type_: *mut GLenum,
    name: *mut GLchar,
  );
  #[link_name = "glGetActiveUniformBlockName"]
  pub fn GetActiveUniformBlockName(
    program: GLuint,
    uniformBlockIndex: GLuint,
    bufSize: GLsizei,
    length: *mut GLsizei,
    uniformBlockName: *mut GLchar,
  );
  #[link_name = "glGetActiveUniformBlockiv"]
  pub fn GetActiveUniformBlockiv(
    program: GLuint,
    uniformBlockIndex: GLuint,
    pname: GLenum,
    params: *mut GLint,
  );
  #[link_name = "glGetActiveUniformName"]
  pub fn GetActiveUniformName(
    program: GLuint,
    uniformIndex: GLuint,
    bufSize: GLsizei,
    length: *mut GLsizei,
    uniformName: *mut GLchar,
  );
  #[link_name = "glGetActiveUniformsiv"]
  pub fn GetActiveUniformsiv(
    program: GLuint,
    uniformCount: GLsizei,
    uniformIndices: *const GLuint,
    pname: GLenum,
    params: *mut GLint,
  );
  #[link_name = "glGetAttachedShaders"]
  pub fn GetAttachedShaders(
    program: GLuint,
    maxCount: GLsizei,
    count: *mut GLsizei,
    shaders: *mut GLuint,
  );
  #[link_name = "glGetAttribLocation"]
  pub fn GetAttribLocation(program: GLuint, name: *const GLchar) -> GLint;
  #[link_name = "glGetBooleani_v"]
  pub fn GetBooleani_v(target: GLenum, index: GLuint, data: *mut GLboolean);
  #[link_name = "glGetBooleanv"]
  pub fn GetBooleanv(pname: GLenum, data: *mut GLboolean);
  #[link_name = "glGetBufferParameteri64v"]
  pub fn GetBufferParameteri64v(target: GLenum, pname: GLenum, params: *mut GLint64);
  #[link_name = "glGetBufferParameteriv"]
  pub fn GetBufferParameteriv(target: GLenum, pname: GLenum, params: *mut GLint);
  #[link_name = "glGetBufferPointerv"]
  pub fn GetBufferPointerv(target: GLenum, pname: GLenum, params: *const *mut c_void);
  #[link_name = "glGetBufferSubData"]
  pub fn GetBufferSubData(target: GLenum, offset: GLintptr, size: GLsizeiptr, data: *mut c_void);
  #[link_name = "glGetCompressedTexImage"]
  pub fn GetCompressedTexImage(target: GLenum, level: GLint, img: *mut c_void);
  #[link_name = "glGetDoublev"]
  pub fn GetDoublev(pname: GLenum, data: *mut GLdouble);
  #[link_name = "glGetError"]
  pub fn GetError() -> GLenum;
  #[link_name = "glGetFloatv"]
  pub fn GetFloatv(pname: GLenum, data: *mut GLfloat);
  #[link_name = "glGetFragDataIndex"]
  pub fn GetFragDataIndex(program: GLuint, name: *const GLchar) -> GLint;
  #[link_name = "glGetFragDataLocation"]
  pub fn GetFragDataLocation(program: GLuint, name: *const GLchar) -> GLint;
  #[link_name = "glGetFramebufferAttachmentParameteriv"]
  pub fn GetFramebufferAttachmentParameteriv(
    target: GLenum,
    attachment: GLenum,
    pname: GLenum,
    params: *mut GLint,
  );
  #[link_name = "glGetInteger64i_v"]
  pub fn GetInteger64i_v(target: GLenum, index: GLuint, data: *mut GLint64);
  #[link_name = "glGetInteger64v"]
  pub fn GetInteger64v(pname: GLenum, data: *mut GLint64);
  #[link_name = "glGetIntegeri_v"]
  pub fn GetIntegeri_v(target: GLenum, index: GLuint, data: *mut GLint);
  #[link_name = "glGetIntegerv"]
  pub fn GetIntegerv(pname: GLenum, data: *mut GLint);
  #[link_name = "glGetMultisamplefv"]
  pub fn GetMultisamplefv(pname: GLenum, index: GLuint, val: *mut GLfloat);
  #[link_name = "glGetProgramInfoLog"]
  pub fn GetProgramInfoLog(
    program: GLuint,
    bufSize: GLsizei,
    length: *mut GLsizei,
    infoLog: *mut GLchar,
  );
  #[link_name = "glGetProgramiv"]
  pub fn GetProgramiv(program: GLuint, pname: GLenum, params: *mut GLint);
  #[link_name = "glGetQueryObjecti64v"]
  pub fn GetQueryObjecti64v(id: GLuint, pname: GLenum, params: *mut GLint64);
  #[link_name = "glGetQueryObjectiv"]
  pub fn GetQueryObjectiv(id: GLuint, pname: GLenum, params: *mut GLint);
  #[link_name = "glGetQueryObjectui64v"]
  pub fn GetQueryObjectui64v(id: GLuint, pname: GLenum, params: *mut GLuint64);
  #[link_name = "glGetQueryObjectuiv"]
  pub fn GetQueryObjectuiv(id: GLuint, pname: GLenum, params: *mut GLuint);
  #[link_name = "glGetQueryiv"]
  pub fn GetQueryiv(target: GLenum, pname: GLenum, params: *mut GLint);
  #[link_name = "glGetRenderbufferParameteriv"]
  pub fn GetRenderbufferParameteriv(target: GLenum, pname: GLenum, params: *mut GLint);
  #[link_name = "glGetSamplerParameterIiv"]
  pub fn GetSamplerParameterIiv(sampler: GLuint, pname: GLenum, params: *mut GLint);
  #[link_name = "glGetSamplerParameterIuiv"]
  pub fn GetSamplerParameterIuiv(sampler: GLuint, pname: GLenum, params: *mut GLuint);
  #[link_name = "glGetSamplerParameterfv"]
  pub fn GetSamplerParameterfv(sampler: GLuint, pname: GLenum, params: *mut GLfloat);
  #[link_name = "glGetSamplerParameteriv"]
  pub fn GetSamplerParameteriv(sampler: GLuint, pname: GLenum, params: *mut GLint);
  #[link_name = "glGetShaderInfoLog"]
  pub fn GetShaderInfoLog(
    shader: GLuint,
    bufSize: GLsizei,
    length: *mut GLsizei,
    infoLog: *mut GLchar,
  );
  #[link_name = "glGetShaderSource"]
  pub fn GetShaderSource(
    shader: GLuint,
    bufSize: GLsizei,
    length: *mut GLsizei,
    source: *mut GLchar,
  );
  #[link_name = "glGetShaderiv"]
  pub fn GetShaderiv(shader: GLuint, pname: GLenum, params: *mut GLint);
  #[link_name = "glGetString"]
  pub fn GetString(name: GLenum) -> *const GLubyte;
  #[link_name = "glGetStringi"]
  pub fn GetStringi(name: GLenum, index: GLuint) -> *const GLubyte;
  #[link_name = "glGetTexImage"]
  pub fn GetTexImage(
    target: GLenum,
    level: GLint,
    format: GLenum,
    type_: GLenum,
    pixels: *mut c_void,
  );
  #[link_name = "glGetTexLevelParameterfv"]
  pub fn GetTexLevelParameterfv(target: GLenum, level: GLint, pname: GLenum, params: *mut GLfloat);
  #[link_name = "glGetTexLevelParameteriv"]
  pub fn GetTexLevelParameteriv(target: GLenum, level: GLint, pname: GLenum, params: *mut GLint);
  #[link_name = "glGetTexParameterIiv"]
  pub fn GetTexParameterIiv(target: GLenum, pname: GLenum, params: *mut GLint);
  #[link_name = "glGetTexParameterIuiv"]
  pub fn GetTexParameterIuiv(target: GLenum, pname: GLenum, params: *mut GLuint);
  #[link_name = "glGetTexParameterfv"]
  pub fn GetTexParameterfv(target: GLenum, pname: GLenum, params: *mut GLfloat);
  #[link_name = "glGetTexParameteriv"]
  pub fn GetTexParameteriv(target: GLenum, pname: GLenum, params: *mut GLint);
  #[link_name = "glGetTransformFeedbackVarying"]
  pub fn GetTransformFeedbackVarying(
    program: GLuint,
    index: GLuint,
    bufSize: GLsizei,
    length: *mut GLsizei,
    size: *mut GLsizei,
    type_: *mut GLenum,
    name: *mut GLchar,
  );
  #[link_name = "glGetUniformBlockIndex"]
  pub fn GetUniformBlockIndex(program: GLuint, uniformBlockName: *const GLchar) -> GLuint;
  #[link_name = "glGetUniformIndices"]
  pub fn GetUniformIndices(
    program: GLuint,
    uniformCount: GLsizei,
    uniformNames: *const *const GLchar,
    uniformIndices: *mut GLuint,
  );
  #[link_name = "glGetUniformLocation"]
  pub fn GetUniformLocation(program: GLuint, name: *const GLchar) -> GLint;
  #[link_name = "glGetUniformfv"]
  pub fn GetUniformfv(program: GLuint, location: GLint, params: *mut GLfloat);
  #[link_name = "glGetUniformiv"]
  pub fn GetUniformiv(program: GLuint, location: GLint, params: *mut GLint);
  #[link_name = "glGetUniformuiv"]
  pub fn GetUniformuiv(program: GLuint, location: GLint, params: *mut GLuint);
  #[link_name = "glGetVertexAttribIiv"]
  pub fn GetVertexAttribIiv(index: GLuint, pname: GLenum, params: *mut GLint);
  #[link_name = "glGetVertexAttribIuiv"]
  pub fn GetVertexAttribIuiv(index: GLuint, pname: GLenum, params: *mut GLuint);
  #[link_name = "glGetVertexAttribPointerv"]
  pub fn GetVertexAttribPointerv(index: GLuint, pname: GLenum, pointer: *const *mut c_void);
  #[link_name = "glGetVertexAttribdv"]
  pub fn GetVertexAttribdv(index: GLuint, pname: GLenum, params: *mut GLdouble);
  #[link_name = "glGetVertexAttribfv"]
  pub fn GetVertexAttribfv(index: GLuint, pname: GLenum, params: *mut GLfloat);
  #[link_name = "glGetVertexAttribiv"]
  pub fn GetVertexAttribiv(index: GLuint, pname: GLenum, params: *mut GLint);
  #[link_name = "glHint"]
  pub fn Hint(target: GLenum, mode: GLenum);
  #[link_name = "glIsBuffer"]
  pub fn IsBuffer(buffer: GLuint) -> GLboolean;
  #[link_name = "glIsEnabled"]
  pub fn IsEnabled(cap: GLenum) -> GLboolean;
  #[link_name = "glIsEnabledi"]
  pub fn IsEnabledi(target: GLenum, index: GLuint) -> GLboolean;
  #[link_name = "glIsFramebuffer"]
  pub fn IsFramebuffer(framebuffer: GLuint) -> GLboolean;
  #[link_name = "glIsProgram"]
  pub fn IsProgram(program: GLuint) -> GLboolean;
  #[link_name = "glIsQuery"]
  pub fn IsQuery(id: GLuint) -> GLboolean;
  #[link_name = "glIsRenderbuffer"]
  pub fn IsRenderbuffer(renderbuffer: GLuint) -> GLboolean;
  #[link_name = "glIsSampler"]
  pub fn IsSampler(sampler: GLuint) -> GLboolean;
  #[link_name = "glIsShader"]
  pub fn IsShader(shader: GLuint) -> GLboolean;
  #[link_name = "glIsTexture"]
  pub fn IsTexture(texture: GLuint) -> GLboolean;
  #[link_name = "glIsVertexArray"]
  pub fn IsVertexArray(array: GLuint) -> GLboolean;
  #[link_name = "glLineWidth"]
  pub fn LineWidth(width: GLfloat);
  #[link_name = "glLinkProgram"]
  pub fn LinkProgram(program: GLuint);
  #[link_name = "glLogicOp"]
  pub fn LogicOp(opcode: GLenum);
  #[link_name = "glMapBuffer"]
  pub fn MapBuffer(target: GLenum, access: GLenum) -> *mut c_void;
  #[link_name = "glMapBufferRange"]
  pub fn MapBufferRange(
    target: GLenum,
    offset: GLintptr,
    length: GLsizeiptr,
    access: GLbitfield,
  ) -> *mut c_void;
  #[link_name = "glMultiDrawArrays"]
  pub fn MultiDrawArrays(
    mode: GLenum,
    first: *const GLint,
    count: *const GLsizei,
    drawcount: GLsizei,
  );
  #[link_name = "glMultiDrawElements"]
  pub fn MultiDrawElements(
    mode: GLenum,
    count: *const GLsizei,
    type_: GLenum,
    indices: *const *const c_void,
    drawcount: GLsizei,
  );
  #[link_name = "glMultiDrawElementsBaseVertex"]
  pub fn MultiDrawElementsBaseVertex(
    mode: GLenum,
    count: *const GLsizei,
    type_: GLenum,
    indices: *const *const c_void,
    drawcount: GLsizei,
    basevertex: *const GLint,
  );
  #[link_name = "glMultiTexCoordP1ui"]
  pub fn MultiTexCoordP1ui(texture: GLenum, type_: GLenum, coords: GLuint);
  #[link_name = "glMultiTexCoordP1uiv"]
  pub fn MultiTexCoordP1uiv(texture: GLenum, type_: GLenum, coords: *const GLuint);
  #[link_name = "glMultiTexCoordP2ui"]
  pub fn MultiTexCoordP2ui(texture: GLenum, type_: GLenum, coords: GLuint);
  #[link_name = "glMultiTexCoordP2uiv"]
  pub fn MultiTexCoordP2uiv(texture: GLenum, type_: GLenum, coords: *const GLuint);
  #[link_name = "glMultiTexCoordP3ui"]
  pub fn MultiTexCoordP3ui(texture: GLenum, type_: GLenum, coords: GLuint);
  #[link_name = "glMultiTexCoordP3uiv"]
  pub fn MultiTexCoordP3uiv(texture: GLenum, type_: GLenum, coords: *const GLuint);
  #[link_name = "glMultiTexCoordP4ui"]
  pub fn MultiTexCoordP4ui(texture: GLenum, type_: GLenum, coords: GLuint);
  #[link_name = "glMultiTexCoordP4uiv"]
  pub fn MultiTexCoordP4uiv(texture: GLenum, type_: GLenum, coords: *const GLuint);
  #[link_name = "glNormalP3ui"]
  pub fn NormalP3ui(type_: GLenum, coords: GLuint);
  #[link_name = "glNormalP3uiv"]
  pub fn NormalP3uiv(type_: GLenum, coords: *const GLuint);
  #[link_name = "glPixelStoref"]
  pub fn PixelStoref(pname: GLenum, param: GLfloat);
  #[link_name = "glPixelStorei"]
  pub fn PixelStorei(pname: GLenum, param: GLint);
  #[link_name = "glPointParameterf"]
  pub fn PointParameterf(pname: GLenum, param: GLfloat);
  #[link_name = "glPointParameterfv"]
  pub fn PointParameterfv(pname: GLenum, params: *const GLfloat);
  #[link_name = "glPointParameteri"]
  pub fn PointParameteri(pname: GLenum, param: GLint);
  #[link_name = "glPointParameteriv"]
  pub fn PointParameteriv(pname: GLenum, params: *const GLint);
  #[link_name = "glPointSize"]
  pub fn PointSize(size: GLfloat);
  #[link_name = "glPolygonMode"]
  pub fn PolygonMode(face: GLenum, mode: GLenum);
  #[link_name = "glPolygonOffset"]
  pub fn PolygonOffset(factor: GLfloat, units: GLfloat);
  #[link_name = "glPrimitiveRestartIndex"]
  pub fn PrimitiveRestartIndex(index: GLuint);
  #[link_name = "glProvokingVertex"]
  pub fn ProvokingVertex(mode: GLenum);
  #[link_name = "glQueryCounter"]
  pub fn QueryCounter(id: GLuint, target: GLenum);
  #[link_name = "glReadBuffer"]
  pub fn ReadBuffer(src: GLenum);
  #[link_name = "glReadPixels"]
  pub fn ReadPixels(
    x: GLint,
    y: GLint,
    width: GLsizei,
    height: GLsizei,
    format: GLenum,
    type_: GLenum,
    pixels: *mut c_void,
  );
  #[link_name = "glRenderbufferStorage"]
  pub fn RenderbufferStorage(
    target: GLenum,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
  );
  #[link_name = "glRenderbufferStorageMultisample"]
  pub fn RenderbufferStorageMultisample(
    target: GLenum,
    samples: GLsizei,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
  );
  #[link_name = "glSampleCoverage"]
  pub fn SampleCoverage(value: GLfloat, invert: GLboolean);
  #[link_name = "glSampleMaski"]
  pub fn SampleMaski(maskNumber: GLuint, mask: GLbitfield);
  #[link_name = "glSamplerParameterIiv"]
  pub fn SamplerParameterIiv(sampler: GLuint, pname: GLenum, param: *const GLint);
  #[link_name = "glSamplerParameterIuiv"]
  pub fn SamplerParameterIuiv(sampler: GLuint, pname: GLenum, param: *const GLuint);
  #[link_name = "glSamplerParameterf"]
  pub fn SamplerParameterf(sampler: GLuint, pname: GLenum, param: GLfloat);
  #[link_name = "glSamplerParameterfv"]
  pub fn SamplerParameterfv(sampler: GLuint, pname: GLenum, param: *const GLfloat);
  #[link_name = "glSamplerParameteri"]
  pub fn SamplerParameteri(sampler: GLuint, pname: GLenum, param: GLint);
  #[link_name = "glSamplerParameteriv"]
  pub fn SamplerParameteriv(sampler: GLuint, pname: GLenum, param: *const GLint);
  #[link_name = "glScissor"]
  pub fn Scissor(x: GLint, y: GLint, width: GLsizei, height: GLsizei);
  #[link_name = "glSecondaryColorP3ui"]
  pub fn SecondaryColorP3ui(type_: GLenum, color: GLuint);
  #[link_name = "glSecondaryColorP3uiv"]
  pub fn SecondaryColorP3uiv(type_: GLenum, color: *const GLuint);
  #[link_name = "glShaderSource"]
  pub fn ShaderSource(
    shader: GLuint,
    count: GLsizei,
    string: *const *const GLchar,
    length: *const GLint,
  );
  #[link_name = "glStencilFunc"]
  pub fn StencilFunc(func: GLenum, ref_: GLint, mask: GLuint);
  #[link_name = "glStencilFuncSeparate"]
  pub fn StencilFuncSeparate(face: GLenum, func: GLenum, ref_: GLint, mask: GLuint);
  #[link_name = "glStencilMask"]
  pub fn StencilMask(mask: GLuint);
  #[link_name = "glStencilMaskSeparate"]
  pub fn StencilMaskSeparate(face: GLenum, mask: GLuint);
  #[link_name = "glStencilOp"]
  pub fn StencilOp(fail: GLenum, zfail: GLenum, zpass: GLenum);
  #[link_name = "glStencilOpSeparate"]
  pub fn StencilOpSeparate(face: GLenum, sfail: GLenum, dpfail: GLenum, dppass: GLenum);
  #[link_name = "glTexBuffer"]
  pub fn TexBuffer(target: GLenum, internalformat: GLenum, buffer: GLuint);
  #[link_name = "glTexCoordP1ui"]
  pub fn TexCoordP1ui(type_: GLenum, coords: GLuint);
  #[link_name = "glTexCoordP1uiv"]
  pub fn TexCoordP1uiv(type_: GLenum, coords: *const GLuint);
  #[link_name = "glTexCoordP2ui"]
  pub fn TexCoordP2ui(type_: GLenum, coords: GLuint);
  #[link_name = "glTexCoordP2uiv"]
  pub fn TexCoordP2uiv(type_: GLenum, coords: *const GLuint);
  #[link_name = "glTexCoordP3ui"]
  pub fn TexCoordP3ui(type_: GLenum, coords: GLuint);
  #[link_name = "glTexCoordP3uiv"]
  pub fn TexCoordP3uiv(type_: GLenum, coords: *const GLuint);
  #[link_name = "glTexCoordP4ui"]
  pub fn TexCoordP4ui(type_: GLenum, coords: GLuint);
  #[link_name = "glTexCoordP4uiv"]
  pub fn TexCoordP4uiv(type_: GLenum, coords: *const GLuint);
  #[link_name = "glTexImage1D"]
  pub fn TexImage1D(
    target: GLenum,
    level: GLint,
    internalformat: GLint,
    width: GLsizei,
    border: GLint,
    format: GLenum,
    type_: GLenum,
    pixels: *const c_void,
  );
  #[link_name = "glTexImage2D"]
  pub fn TexImage2D(
    target: GLenum,
    level: GLint,
    internalformat: GLint,
    width: GLsizei,
    height: GLsizei,
    border: GLint,
    format: GLenum,
    type_: GLenum,
    pixels: *const c_void,
  );
  #[link_name = "glTexImage2DMultisample"]
  pub fn TexImage2DMultisample(
    target: GLenum,
    samples: GLsizei,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
    fixedsamplelocations: GLboolean,
  );
  #[link_name = "glTexImage3D"]
  pub fn TexImage3D(
    target: GLenum,
    level: GLint,
    internalformat: GLint,
    width: GLsizei,
    height: GLsizei,
    depth: GLsizei,
    border: GLint,
    format: GLenum,
    type_: GLenum,
    pixels: *const c_void,
  );
  #[link_name = "glTexImage3DMultisample"]
  pub fn TexImage3DMultisample(
    target: GLenum,
    samples: GLsizei,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
    depth: GLsizei,
    fixedsamplelocations: GLboolean,
  );
  #[link_name = "glTexParameterIiv"]
  pub fn TexParameterIiv(target: GLenum, pname: GLenum, params: *const GLint);
  #[link_name = "glTexParameterIuiv"]
  pub fn TexParameterIuiv(target: GLenum, pname: GLenum, params: *const GLuint);
  #[link_name = "glTexParameterf"]
  pub fn TexParameterf(target: GLenum, pname: GLenum, param: GLfloat);
  #[link_name = "glTexParameterfv"]
  pub fn TexParameterfv(target: GLenum, pname: GLenum, params: *const GLfloat);
  #[link_name = "glTexParameteri"]
  pub fn TexParameteri(target: GLenum, pname: GLenum, param: GLint);
  #[link_name = "glTexParameteriv"]
  pub fn TexParameteriv(target: GLenum, pname: GLenum, params: *const GLint);
  #[link_name = "glTexSubImage1D"]
  pub fn TexSubImage1D(
    target: GLenum,
    level: GLint,
    xoffset: GLint,
    width: GLsizei,
    format: GLenum,
    type_: GLenum,
    pixels: *const c_void,
  );
  #[link_name = "glTexSubImage2D"]
  pub fn TexSubImage2D(
    target: GLenum,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    width: GLsizei,
    height: GLsizei,
    format: GLenum,
    type_: GLenum,
    pixels: *const c_void,
  );
  #[link_name = "glTexSubImage3D"]
  pub fn TexSubImage3D(
    target: GLenum,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    zoffset: GLint,
    width: GLsizei,
    height: GLsizei,
    depth: GLsizei,
    format: GLenum,
    type_: GLenum,
    pixels: *const c_void,
  );
  #[link_name = "glTransformFeedbackVaryings"]
  pub fn TransformFeedbackVaryings(
    program: GLuint,
    count: GLsizei,
    varyings: *const *const GLchar,
    bufferMode: GLenum,
  );
  #[link_name = "glUniform1f"]
  pub fn Uniform1f(location: GLint, v0: GLfloat);
  #[link_name = "glUniform1fv"]
  pub fn Uniform1fv(location: GLint, count: GLsizei, value: *const GLfloat);
  #[link_name = "glUniform1i"]
  pub fn Uniform1i(location: GLint, v0: GLint);
  #[link_name = "glUniform1iv"]
  pub fn Uniform1iv(location: GLint, count: GLsizei, value: *const GLint);
  #[link_name = "glUniform1ui"]
  pub fn Uniform1ui(location: GLint, v0: GLuint);
  #[link_name = "glUniform1uiv"]
  pub fn Uniform1uiv(location: GLint, count: GLsizei, value: *const GLuint);
  #[link_name = "glUniform2f"]
  pub fn Uniform2f(location: GLint, v0: GLfloat, v1: GLfloat);
  #[link_name = "glUniform2fv"]
  pub fn Uniform2fv(location: GLint, count: GLsizei, value: *const GLfloat);
  #[link_name = "glUniform2i"]
  pub fn Uniform2i(location: GLint, v0: GLint, v1: GLint);
  #[link_name = "glUniform2iv"]
  pub fn Uniform2iv(location: GLint, count: GLsizei, value: *const GLint);
  #[link_name = "glUniform2ui"]
  pub fn Uniform2ui(location: GLint, v0: GLuint, v1: GLuint);
  #[link_name = "glUniform2uiv"]
  pub fn Uniform2uiv(location: GLint, count: GLsizei, value: *const GLuint);
  #[link_name = "glUniform3f"]
  pub fn Uniform3f(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat);
  #[link_name = "glUniform3fv"]
  pub fn Uniform3fv(location: GLint, count: GLsizei, value: *const GLfloat);
  #[link_name = "glUniform3i"]
  pub fn Uniform3i(location: GLint, v0: GLint, v1: GLint, v2: GLint);
  #[link_name = "glUniform3iv"]
  pub fn Uniform3iv(location: GLint, count: GLsizei, value: *const GLint);
  #[link_name = "glUniform3ui"]
  pub fn Uniform3ui(location: GLint, v0: GLuint, v1: GLuint, v2: GLuint);
  #[link_name = "glUniform3uiv"]
  pub fn Uniform3uiv(location: GLint, count: GLsizei, value: *const GLuint);
  #[link_name = "glUniform4f"]
  pub fn Uniform4f(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat);
  #[link_name = "glUniform4fv"]
  pub fn Uniform4fv(location: GLint, count: GLsizei, value: *const GLfloat);
  #[link_name = "glUniform4i"]
  pub fn Uniform4i(location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint);
  #[link_name = "glUniform4iv"]
  pub fn Uniform4iv(location: GLint, count: GLsizei, value: *const GLint);
  #[link_name = "glUniform4ui"]
  pub fn Uniform4ui(location: GLint, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint);
  #[link_name = "glUniform4uiv"]
  pub fn Uniform4uiv(location: GLint, count: GLsizei, value: *const GLuint);
  #[link_name = "glUniformBlockBinding"]
  pub fn UniformBlockBinding(
    program: GLuint,
    uniformBlockIndex: GLuint,
    uniformBlockBinding: GLuint,
  );
  #[link_name = "glUniformMatrix2fv"]
  pub fn UniformMatrix2fv(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
  );
  #[link_name = "glUniformMatrix2x3fv"]
  pub fn UniformMatrix2x3fv(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
  );
  #[link_name = "glUniformMatrix2x4fv"]
  pub fn UniformMatrix2x4fv(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
  );
  #[link_name = "glUniformMatrix3fv"]
  pub fn UniformMatrix3fv(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
  );
  #[link_name = "glUniformMatrix3x2fv"]
  pub fn UniformMatrix3x2fv(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
  );
  #[link_name = "glUniformMatrix3x4fv"]
  pub fn UniformMatrix3x4fv(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
  );
  #[link_name = "glUniformMatrix4fv"]
  pub fn UniformMatrix4fv(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
  );
  #[link_name = "glUniformMatrix4x2fv"]
  pub fn UniformMatrix4x2fv(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
  );
  #[link_name = "glUniformMatrix4x3fv"]
  pub fn UniformMatrix4x3fv(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
  );
  #[link_name = "glUnmapBuffer"]
  pub fn UnmapBuffer(target: GLenum) -> GLboolean;
  #[link_name = "glUseProgram"]
  pub fn UseProgram(program: GLuint);
  #[link_name = "glValidateProgram"]
  pub fn ValidateProgram(program: GLuint);
  #[link_name = "glVertexAttrib1d"]
  pub fn VertexAttrib1d(index: GLuint, x: GLdouble);
  #[link_name = "glVertexAttrib1dv"]
  pub fn VertexAttrib1dv(index: GLuint, v: *const GLdouble);
  #[link_name = "glVertexAttrib1f"]
  pub fn VertexAttrib1f(index: GLuint, x: GLfloat);
  #[link_name = "glVertexAttrib1fv"]
  pub fn VertexAttrib1fv(index: GLuint, v: *const GLfloat);
  #[link_name = "glVertexAttrib1s"]
  pub fn VertexAttrib1s(index: GLuint, x: GLshort);
  #[link_name = "glVertexAttrib1sv"]
  pub fn VertexAttrib1sv(index: GLuint, v: *const GLshort);
  #[link_name = "glVertexAttrib2d"]
  pub fn VertexAttrib2d(index: GLuint, x: GLdouble, y: GLdouble);
  #[link_name = "glVertexAttrib2dv"]
  pub fn VertexAttrib2dv(index: GLuint, v: *const GLdouble);
  #[link_name = "glVertexAttrib2f"]
  pub fn VertexAttrib2f(index: GLuint, x: GLfloat, y: GLfloat);
  #[link_name = "glVertexAttrib2fv"]
  pub fn VertexAttrib2fv(index: GLuint, v: *const GLfloat);
  #[link_name = "glVertexAttrib2s"]
  pub fn VertexAttrib2s(index: GLuint, x: GLshort, y: GLshort);
  #[link_name = "glVertexAttrib2sv"]
  pub fn VertexAttrib2sv(index: GLuint, v: *const GLshort);
  #[link_name = "glVertexAttrib3d"]
  pub fn VertexAttrib3d(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble);
  #[link_name = "glVertexAttrib3dv"]
  pub fn VertexAttrib3dv(index: GLuint, v: *const GLdouble);
  #[link_name = "glVertexAttrib3f"]
  pub fn VertexAttrib3f(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat);
  #[link_name = "glVertexAttrib3fv"]
  pub fn VertexAttrib3fv(index: GLuint, v: *const GLfloat);
  #[link_name = "glVertexAttrib3s"]
  pub fn VertexAttrib3s(index: GLuint, x: GLshort, y: GLshort, z: GLshort);
  #[link_name = "glVertexAttrib3sv"]
  pub fn VertexAttrib3sv(index: GLuint, v: *const GLshort);
  #[link_name = "glVertexAttrib4Nbv"]
  pub fn VertexAttrib4Nbv(index: GLuint, v: *const GLbyte);
  #[link_name = "glVertexAttrib4Niv"]
  pub fn VertexAttrib4Niv(index: GLuint, v: *const GLint);
  #[link_name = "glVertexAttrib4Nsv"]
  pub fn VertexAttrib4Nsv(index: GLuint, v: *const GLshort);
  #[link_name = "glVertexAttrib4Nub"]
  pub fn VertexAttrib4Nub(index: GLuint, x: GLubyte, y: GLubyte, z: GLubyte, w: GLubyte);
  #[link_name = "glVertexAttrib4Nubv"]
  pub fn VertexAttrib4Nubv(index: GLuint, v: *const GLubyte);
  #[link_name = "glVertexAttrib4Nuiv"]
  pub fn VertexAttrib4Nuiv(index: GLuint, v: *const GLuint);
  #[link_name = "glVertexAttrib4Nusv"]
  pub fn VertexAttrib4Nusv(index: GLuint, v: *const GLushort);
  #[link_name = "glVertexAttrib4bv"]
  pub fn VertexAttrib4bv(index: GLuint, v: *const GLbyte);
  #[link_name = "glVertexAttrib4d"]
  pub fn VertexAttrib4d(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);
  #[link_name = "glVertexAttrib4dv"]
  pub fn VertexAttrib4dv(index: GLuint, v: *const GLdouble);
  #[link_name = "glVertexAttrib4f"]
  pub fn VertexAttrib4f(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);
  #[link_name = "glVertexAttrib4fv"]
  pub fn VertexAttrib4fv(index: GLuint, v: *const GLfloat);
  #[link_name = "glVertexAttrib4iv"]
  pub fn VertexAttrib4iv(index: GLuint, v: *const GLint);
  #[link_name = "glVertexAttrib4s"]
  pub fn VertexAttrib4s(index: GLuint, x: GLshort, y: GLshort, z: GLshort, w: GLshort);
  #[link_name = "glVertexAttrib4sv"]
  pub fn VertexAttrib4sv(index: GLuint, v: *const GLshort);
  #[link_name = "glVertexAttrib4ubv"]
  pub fn VertexAttrib4ubv(index: GLuint, v: *const GLubyte);
  #[link_name = "glVertexAttrib4uiv"]
  pub fn VertexAttrib4uiv(index: GLuint, v: *const GLuint);
  #[link_name = "glVertexAttrib4usv"]
  pub fn VertexAttrib4usv(index: GLuint, v: *const GLushort);
  #[link_name = "glVertexAttribDivisor"]
  pub fn VertexAttribDivisor(index: GLuint, divisor: GLuint);
  #[link_name = "glVertexAttribI1i"]
  pub fn VertexAttribI1i(index: GLuint, x: GLint);
  #[link_name = "glVertexAttribI1iv"]
  pub fn VertexAttribI1iv(index: GLuint, v: *const GLint);
  #[link_name = "glVertexAttribI1ui"]
  pub fn VertexAttribI1ui(index: GLuint, x: GLuint);
  #[link_name = "glVertexAttribI1uiv"]
  pub fn VertexAttribI1uiv(index: GLuint, v: *const GLuint);
  #[link_name = "glVertexAttribI2i"]
  pub fn VertexAttribI2i(index: GLuint, x: GLint, y: GLint);
  #[link_name = "glVertexAttribI2iv"]
  pub fn VertexAttribI2iv(index: GLuint, v: *const GLint);
  #[link_name = "glVertexAttribI2ui"]
  pub fn VertexAttribI2ui(index: GLuint, x: GLuint, y: GLuint);
  #[link_name = "glVertexAttribI2uiv"]
  pub fn VertexAttribI2uiv(index: GLuint, v: *const GLuint);
  #[link_name = "glVertexAttribI3i"]
  pub fn VertexAttribI3i(index: GLuint, x: GLint, y: GLint, z: GLint);
  #[link_name = "glVertexAttribI3iv"]
  pub fn VertexAttribI3iv(index: GLuint, v: *const GLint);
  #[link_name = "glVertexAttribI3ui"]
  pub fn VertexAttribI3ui(index: GLuint, x: GLuint, y: GLuint, z: GLuint);
  #[link_name = "glVertexAttribI3uiv"]
  pub fn VertexAttribI3uiv(index: GLuint, v: *const GLuint);
  #[link_name = "glVertexAttribI4bv"]
  pub fn VertexAttribI4bv(index: GLuint, v: *const GLbyte);
  #[link_name = "glVertexAttribI4i"]
  pub fn VertexAttribI4i(index: GLuint, x: GLint, y: GLint, z: GLint, w: GLint);
  #[link_name = "glVertexAttribI4iv"]
  pub fn VertexAttribI4iv(index: GLuint, v: *const GLint);
  #[link_name = "glVertexAttribI4sv"]
  pub fn VertexAttribI4sv(index: GLuint, v: *const GLshort);
  #[link_name = "glVertexAttribI4ubv"]
  pub fn VertexAttribI4ubv(index: GLuint, v: *const GLubyte);
  #[link_name = "glVertexAttribI4ui"]
  pub fn VertexAttribI4ui(index: GLuint, x: GLuint, y: GLuint, z: GLuint, w: GLuint);
  #[link_name = "glVertexAttribI4uiv"]
  pub fn VertexAttribI4uiv(index: GLuint, v: *const GLuint);
  #[link_name = "glVertexAttribI4usv"]
  pub fn VertexAttribI4usv(index: GLuint, v: *const GLushort);
  #[link_name = "glVertexAttribIPointer"]
  pub fn VertexAttribIPointer(
    index: GLuint,
    size: GLint,
    type_: GLenum,
    stride: GLsizei,
    pointer: *const c_void,
  );
  #[link_name = "glVertexAttribP1ui"]
  pub fn VertexAttribP1ui(index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint);
  #[link_name = "glVertexAttribP1uiv"]
  pub fn VertexAttribP1uiv(
    index: GLuint,
    type_: GLenum,
    normalized: GLboolean,
    value: *const GLuint,
  );
  #[link_name = "glVertexAttribP2ui"]
  pub fn VertexAttribP2ui(index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint);
  #[link_name = "glVertexAttribP2uiv"]
  pub fn VertexAttribP2uiv(
    index: GLuint,
    type_: GLenum,
    normalized: GLboolean,
    value: *const GLuint,
  );
  #[link_name = "glVertexAttribP3ui"]
  pub fn VertexAttribP3ui(index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint);
  #[link_name = "glVertexAttribP3uiv"]
  pub fn VertexAttribP3uiv(
    index: GLuint,
    type_: GLenum,
    normalized: GLboolean,
    value: *const GLuint,
  );
  #[link_name = "glVertexAttribP4ui"]
  pub fn VertexAttribP4ui(index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint);
  #[link_name = "glVertexAttribP4uiv"]
  pub fn VertexAttribP4uiv(
    index: GLuint,
    type_: GLenum,
    normalized: GLboolean,
    value: *const GLuint,
  );
  #[link_name = "glVertexAttribPointer"]
  pub fn VertexAttribPointer(
    index: GLuint,
    size: GLint,
    type_: GLenum,
    normalized: GLboolean,
    stride: GLsizei,
    pointer: *const c_void,
  );
  #[link_name = "glVertexP2ui"]
  pub fn VertexP2ui(type_: GLenum, value: GLuint);
  #[link_name = "glVertexP2uiv"]
  pub fn VertexP2uiv(type_: GLenum, value: *const GLuint);
  #[link_name = "glVertexP3ui"]
  pub fn VertexP3ui(type_: GLenum, value: GLuint);
  #[link_name = "glVertexP3uiv"]
  pub fn VertexP3uiv(type_: GLenum, value: *const GLuint);
  #[link_name = "glVertexP4ui"]
  pub fn VertexP4ui(type_: GLenum, value: GLuint);
  #[link_name = "glVertexP4uiv"]
  pub fn VertexP4uiv(type_: GLenum, value: *const GLuint);
  #[link_name = "glViewport"]
  pub fn Viewport(x: GLint, y: GLint, width: GLsizei, height: GLsizei);
}
