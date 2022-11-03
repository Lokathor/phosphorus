#![allow(clippy::upper_case_acronyms)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(clippy::unused_unit)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::too_many_arguments)]
type GLenum = u32;
type GLbitfield = u32;
#[cfg(any(target_os = "ios", target_os = "macos"))]
type GLhandleARB = *mut void;
#[cfg(not(any(target_os = "ios", target_os = "macos")))]
type GLhandleARB = GLuint;
type GLeglClientBufferEXT = *mut void;
type GLeglImageOES = *mut void;
type GLsync = *mut void;
type _cl_context = void;
type _cl_event = void;
type GLDEBUGPROC = Option<
  unsafe extern "system" fn(
    source: GLenum,
    type_: GLenum,
    id: GLuint,
    severity: GLenum,
    length: GLsizei,
    message: *const GLchar,
    userParam: *const void,
  ),
>;
type GLDEBUGPROCARB = GLDEBUGPROC;
type GLDEBUGPROCKHR = GLDEBUGPROC;
type GLDEBUGPROCAMD = Option<
  unsafe extern "system" fn(
    id: GLuint,
    category: GLenum,
    severity: GLenum,
    length: GLsizei,
    message: *const GLchar,
    userParam: *mut void,
  ),
>;
type GLVULKANPROCNV = Option<unsafe extern "system" fn()>;
type GLboolean = bool;
type GLbyte = i8;
type GLcharARB = u8;
type GLclampd = f64;
type GLclampf = f32;
type GLclampx = i32;
type GLdouble = f64;
type GLfixed = i32;
type GLfloat = f32;
type GLhalf = u16;
type GLhalfARB = u16;
type GLhalfNV = u16;
type GLint = i32;
type GLint64 = i64;
type GLint64EXT = i64;
type GLintptr = isize;
type GLintptrARB = isize;
type GLshort = i16;
type GLsizei = u32;
type GLsizeiptr = isize;
type GLsizeiptrARB = isize;
type GLubyte = u8;
type GLuint = u32;
type GLuint64 = u64;
type GLuint64EXT = u64;
type GLushort = u16;
type GLvdpauSurfaceNV = GLintptr;
type void = core::ffi::c_void;
type GLchar = u8;

/// * Group: `ProgramPropertyARB`
pub const GL_ACTIVE_ATTRIBUTES: u32 = 0x8B89;
/// * Group: `ProgramPropertyARB`
pub const GL_ACTIVE_ATTRIBUTE_MAX_LENGTH: u32 = 0x8B8A;
/// * Group: `GetPName`
pub const GL_ACTIVE_TEXTURE: u32 = 0x84E0;
/// * Group: `ProgramPropertyARB`
pub const GL_ACTIVE_UNIFORMS: u32 = 0x8B86;
/// * Group: `ProgramPropertyARB`
pub const GL_ACTIVE_UNIFORM_BLOCKS: u32 = 0x8A36;
/// * Group: `ProgramPropertyARB`
pub const GL_ACTIVE_UNIFORM_BLOCK_MAX_NAME_LENGTH: u32 = 0x8A35;
/// * Group: `ProgramPropertyARB`
pub const GL_ACTIVE_UNIFORM_MAX_LENGTH: u32 = 0x8B87;
/// * Group: `GetPName`
pub const GL_ALIASED_LINE_WIDTH_RANGE: u32 = 0x846E;
/// * Groups: `PixelTexGenModeSGIX`, `FragmentShaderValueRepATI`,
///   `TextureSwizzle`, `CombinerPortionNV`, `PathColorFormat`,
///   `CombinerComponentUsageNV`, `PixelFormat`
pub const GL_ALPHA: u32 = 0x1906;
/// * Group: `SyncStatus`
pub const GL_ALREADY_SIGNALED: u32 = 0x911A;
/// * Groups: `StencilFunction`, `IndexFunctionEXT`, `AlphaFunction`,
///   `DepthFunction`
pub const GL_ALWAYS: u32 = 0x0207;
/// * Group: `LogicOp`
pub const GL_AND: u32 = 0x1501;
/// * Group: `LogicOp`
pub const GL_AND_INVERTED: u32 = 0x1504;
/// * Group: `LogicOp`
pub const GL_AND_REVERSE: u32 = 0x1502;
/// * Group: `QueryTarget`
pub const GL_ANY_SAMPLES_PASSED: u32 = 0x8C2F;
/// * Groups: `CopyBufferSubDataTarget`, `BufferTargetARB`,
///   `BufferStorageTarget`
pub const GL_ARRAY_BUFFER: u32 = 0x8892;
/// * Group: `GetPName`
pub const GL_ARRAY_BUFFER_BINDING: u32 = 0x8894;
/// * Group: `ProgramPropertyARB`
pub const GL_ATTACHED_SHADERS: u32 = 0x8B85;
/// * Groups: `ColorBuffer`, `DrawBufferMode`, `ReadBufferMode`, `TriangleFace`
pub const GL_BACK: u32 = 0x0405;
/// * Groups: `ColorBuffer`, `DrawBufferMode`, `ReadBufferMode`
pub const GL_BACK_LEFT: u32 = 0x0402;
/// * Groups: `ColorBuffer`, `DrawBufferMode`, `ReadBufferMode`
pub const GL_BACK_RIGHT: u32 = 0x0403;
/// * Group: `PixelFormat`
pub const GL_BGR: u32 = 0x80E0;
/// * Group: `PixelFormat`
pub const GL_BGRA: u32 = 0x80E1;
/// * Group: `PixelFormat`
pub const GL_BGRA_INTEGER: u32 = 0x8D9B;
/// * Group: `PixelFormat`
pub const GL_BGR_INTEGER: u32 = 0x8D9A;
/// * Groups: `TextureEnvMode`, `EnableCap`, `GetPName`
pub const GL_BLEND: u32 = 0x0BE2;
/// * Group: `GetPName`
pub const GL_BLEND_COLOR: u32 = 0x8005;
/// * Group: `GetPName`
pub const GL_BLEND_DST: u32 = 0x0BE0;
/// * Group: `GetPName`
pub const GL_BLEND_DST_ALPHA: u32 = 0x80CA;
/// * Group: `GetPName`
pub const GL_BLEND_DST_RGB: u32 = 0x80C8;
/// * Group: `GetPName`
pub const GL_BLEND_EQUATION: u32 = 0x8009;
/// * Group: `GetPName`
pub const GL_BLEND_EQUATION_ALPHA: u32 = 0x883D;
/// * Group: `GetPName`
pub const GL_BLEND_EQUATION_RGB: u32 = 0x8009;
/// * Group: `GetPName`
pub const GL_BLEND_SRC: u32 = 0x0BE1;
/// * Group: `GetPName`
pub const GL_BLEND_SRC_ALPHA: u32 = 0x80CB;
/// * Group: `GetPName`
pub const GL_BLEND_SRC_RGB: u32 = 0x80C9;
/// * Groups: `FragmentShaderValueRepATI`, `TextureSwizzle`,
///   `CombinerComponentUsageNV`, `PixelFormat`
pub const GL_BLUE: u32 = 0x1905;
/// * Group: `PixelFormat`
pub const GL_BLUE_INTEGER: u32 = 0x8D96;
/// * Groups: `AttributeType`, `UniformType`
pub const GL_BOOL: u32 = 0x8B56;
/// * Groups: `AttributeType`, `UniformType`
pub const GL_BOOL_VEC2: u32 = 0x8B57;
/// * Groups: `AttributeType`, `UniformType`
pub const GL_BOOL_VEC3: u32 = 0x8B58;
/// * Groups: `AttributeType`, `UniformType`
pub const GL_BOOL_VEC4: u32 = 0x8B59;
/// * Group: `ObjectIdentifier`
pub const GL_BUFFER: u32 = 0x82E0;
/// * Group: `BufferPNameARB`
pub const GL_BUFFER_ACCESS: u32 = 0x88BB;
/// * Group: `BufferPNameARB`
pub const GL_BUFFER_ACCESS_FLAGS: u32 = 0x911F;
/// * Group: `BufferPNameARB`
pub const GL_BUFFER_MAPPED: u32 = 0x88BC;
/// * Group: `BufferPNameARB`
pub const GL_BUFFER_MAP_LENGTH: u32 = 0x9120;
/// * Group: `BufferPNameARB`
pub const GL_BUFFER_MAP_OFFSET: u32 = 0x9121;
/// * Group: `BufferPointerNameARB`
pub const GL_BUFFER_MAP_POINTER: u32 = 0x88BD;
/// * Group: `BufferPNameARB`
pub const GL_BUFFER_SIZE: u32 = 0x8764;
/// * Group: `BufferPNameARB`
pub const GL_BUFFER_USAGE: u32 = 0x8765;
/// * Groups: `VertexAttribIType`, `WeightPointerTypeARB`,
///   `TangentPointerTypeEXT`, `BinormalPointerTypeEXT`, `ColorPointerType`,
///   `ListNameType`, `NormalPointerType`, `PixelType`, `VertexAttribType`,
///   `VertexAttribPointerType`
pub const GL_BYTE: u32 = 0x1400;
/// * Group: `FrontFaceDirection`
pub const GL_CCW: u32 = 0x0901;
/// * Group: `ClampColorTargetARB`
pub const GL_CLAMP_READ_COLOR: u32 = 0x891C;
/// * Group: `TextureWrapMode`
pub const GL_CLAMP_TO_BORDER: u32 = 0x812D;
/// * Group: `TextureWrapMode`
pub const GL_CLAMP_TO_EDGE: u32 = 0x812F;
/// * Group: `LogicOp`
pub const GL_CLEAR: u32 = 0x1500;
/// * Groups: `EnableCap`, `ClipPlaneName`
pub const GL_CLIP_DISTANCE0: u32 = 0x3000;
/// * Groups: `EnableCap`, `ClipPlaneName`
pub const GL_CLIP_DISTANCE1: u32 = 0x3001;
/// * Groups: `EnableCap`, `ClipPlaneName`
pub const GL_CLIP_DISTANCE2: u32 = 0x3002;
/// * Groups: `EnableCap`, `ClipPlaneName`
pub const GL_CLIP_DISTANCE3: u32 = 0x3003;
/// * Groups: `EnableCap`, `ClipPlaneName`
pub const GL_CLIP_DISTANCE4: u32 = 0x3004;
/// * Groups: `EnableCap`, `ClipPlaneName`
pub const GL_CLIP_DISTANCE5: u32 = 0x3005;
/// * Groups: `EnableCap`, `ClipPlaneName`
pub const GL_CLIP_DISTANCE6: u32 = 0x3006;
/// * Groups: `EnableCap`, `ClipPlaneName`
pub const GL_CLIP_DISTANCE7: u32 = 0x3007;
/// * Groups: `Buffer`, `PixelCopyType`, `InvalidateFramebufferAttachment`
pub const GL_COLOR: u32 = 0x1800;
/// * Groups: `ColorBuffer`, `DrawBufferMode`, `ReadBufferMode`,
///   `FramebufferAttachment`, `InvalidateFramebufferAttachment`
pub const GL_COLOR_ATTACHMENT0: u32 = 0x8CE0;
/// * Groups: `ColorBuffer`, `DrawBufferMode`, `ReadBufferMode`,
///   `FramebufferAttachment`, `InvalidateFramebufferAttachment`
pub const GL_COLOR_ATTACHMENT1: u32 = 0x8CE1;
/// * Groups: `ColorBuffer`, `DrawBufferMode`, `ReadBufferMode`,
///   `FramebufferAttachment`, `InvalidateFramebufferAttachment`
pub const GL_COLOR_ATTACHMENT10: u32 = 0x8CEA;
/// * Groups: `ColorBuffer`, `DrawBufferMode`, `ReadBufferMode`,
///   `FramebufferAttachment`, `InvalidateFramebufferAttachment`
pub const GL_COLOR_ATTACHMENT11: u32 = 0x8CEB;
/// * Groups: `ColorBuffer`, `DrawBufferMode`, `ReadBufferMode`,
///   `FramebufferAttachment`, `InvalidateFramebufferAttachment`
pub const GL_COLOR_ATTACHMENT12: u32 = 0x8CEC;
/// * Groups: `ColorBuffer`, `DrawBufferMode`, `ReadBufferMode`,
///   `FramebufferAttachment`, `InvalidateFramebufferAttachment`
pub const GL_COLOR_ATTACHMENT13: u32 = 0x8CED;
/// * Groups: `ColorBuffer`, `DrawBufferMode`, `ReadBufferMode`,
///   `FramebufferAttachment`, `InvalidateFramebufferAttachment`
pub const GL_COLOR_ATTACHMENT14: u32 = 0x8CEE;
/// * Groups: `ColorBuffer`, `DrawBufferMode`, `ReadBufferMode`,
///   `FramebufferAttachment`, `InvalidateFramebufferAttachment`
pub const GL_COLOR_ATTACHMENT15: u32 = 0x8CEF;
/// * Groups: `ColorBuffer`, `DrawBufferMode`, `FramebufferAttachment`,
///   `InvalidateFramebufferAttachment`
pub const GL_COLOR_ATTACHMENT16: u32 = 0x8CF0;
/// * Groups: `ColorBuffer`, `DrawBufferMode`, `FramebufferAttachment`,
///   `InvalidateFramebufferAttachment`
pub const GL_COLOR_ATTACHMENT17: u32 = 0x8CF1;
/// * Groups: `ColorBuffer`, `DrawBufferMode`, `FramebufferAttachment`,
///   `InvalidateFramebufferAttachment`
pub const GL_COLOR_ATTACHMENT18: u32 = 0x8CF2;
/// * Groups: `ColorBuffer`, `DrawBufferMode`, `FramebufferAttachment`,
///   `InvalidateFramebufferAttachment`
pub const GL_COLOR_ATTACHMENT19: u32 = 0x8CF3;
/// * Groups: `ColorBuffer`, `DrawBufferMode`, `ReadBufferMode`,
///   `FramebufferAttachment`, `InvalidateFramebufferAttachment`
pub const GL_COLOR_ATTACHMENT2: u32 = 0x8CE2;
/// * Groups: `ColorBuffer`, `DrawBufferMode`, `FramebufferAttachment`,
///   `InvalidateFramebufferAttachment`
pub const GL_COLOR_ATTACHMENT20: u32 = 0x8CF4;
/// * Groups: `ColorBuffer`, `DrawBufferMode`, `FramebufferAttachment`,
///   `InvalidateFramebufferAttachment`
pub const GL_COLOR_ATTACHMENT21: u32 = 0x8CF5;
/// * Groups: `ColorBuffer`, `DrawBufferMode`, `FramebufferAttachment`,
///   `InvalidateFramebufferAttachment`
pub const GL_COLOR_ATTACHMENT22: u32 = 0x8CF6;
/// * Groups: `ColorBuffer`, `DrawBufferMode`, `FramebufferAttachment`,
///   `InvalidateFramebufferAttachment`
pub const GL_COLOR_ATTACHMENT23: u32 = 0x8CF7;
/// * Groups: `ColorBuffer`, `DrawBufferMode`, `FramebufferAttachment`,
///   `InvalidateFramebufferAttachment`
pub const GL_COLOR_ATTACHMENT24: u32 = 0x8CF8;
/// * Groups: `ColorBuffer`, `DrawBufferMode`, `FramebufferAttachment`,
///   `InvalidateFramebufferAttachment`
pub const GL_COLOR_ATTACHMENT25: u32 = 0x8CF9;
/// * Groups: `ColorBuffer`, `DrawBufferMode`, `FramebufferAttachment`,
///   `InvalidateFramebufferAttachment`
pub const GL_COLOR_ATTACHMENT26: u32 = 0x8CFA;
/// * Groups: `ColorBuffer`, `DrawBufferMode`, `FramebufferAttachment`,
///   `InvalidateFramebufferAttachment`
pub const GL_COLOR_ATTACHMENT27: u32 = 0x8CFB;
/// * Groups: `ColorBuffer`, `DrawBufferMode`, `FramebufferAttachment`,
///   `InvalidateFramebufferAttachment`
pub const GL_COLOR_ATTACHMENT28: u32 = 0x8CFC;
/// * Groups: `ColorBuffer`, `DrawBufferMode`, `FramebufferAttachment`,
///   `InvalidateFramebufferAttachment`
pub const GL_COLOR_ATTACHMENT29: u32 = 0x8CFD;
/// * Groups: `ColorBuffer`, `DrawBufferMode`, `ReadBufferMode`,
///   `FramebufferAttachment`, `InvalidateFramebufferAttachment`
pub const GL_COLOR_ATTACHMENT3: u32 = 0x8CE3;
/// * Groups: `ColorBuffer`, `DrawBufferMode`, `FramebufferAttachment`,
///   `InvalidateFramebufferAttachment`
pub const GL_COLOR_ATTACHMENT30: u32 = 0x8CFE;
/// * Groups: `ColorBuffer`, `DrawBufferMode`, `FramebufferAttachment`,
///   `InvalidateFramebufferAttachment`
pub const GL_COLOR_ATTACHMENT31: u32 = 0x8CFF;
/// * Groups: `ColorBuffer`, `DrawBufferMode`, `ReadBufferMode`,
///   `FramebufferAttachment`, `InvalidateFramebufferAttachment`
pub const GL_COLOR_ATTACHMENT4: u32 = 0x8CE4;
/// * Groups: `ColorBuffer`, `DrawBufferMode`, `ReadBufferMode`,
///   `FramebufferAttachment`, `InvalidateFramebufferAttachment`
pub const GL_COLOR_ATTACHMENT5: u32 = 0x8CE5;
/// * Groups: `ColorBuffer`, `DrawBufferMode`, `ReadBufferMode`,
///   `FramebufferAttachment`, `InvalidateFramebufferAttachment`
pub const GL_COLOR_ATTACHMENT6: u32 = 0x8CE6;
/// * Groups: `ColorBuffer`, `DrawBufferMode`, `ReadBufferMode`,
///   `FramebufferAttachment`, `InvalidateFramebufferAttachment`
pub const GL_COLOR_ATTACHMENT7: u32 = 0x8CE7;
/// * Groups: `ColorBuffer`, `DrawBufferMode`, `ReadBufferMode`,
///   `FramebufferAttachment`, `InvalidateFramebufferAttachment`
pub const GL_COLOR_ATTACHMENT8: u32 = 0x8CE8;
/// * Groups: `ColorBuffer`, `DrawBufferMode`, `ReadBufferMode`,
///   `FramebufferAttachment`, `InvalidateFramebufferAttachment`
pub const GL_COLOR_ATTACHMENT9: u32 = 0x8CE9;
/// * Groups: `ClearBufferMask`, `AttribMask`
pub const GL_COLOR_BUFFER_BIT: u32 = 0x00004000;
/// * Group: `GetPName`
pub const GL_COLOR_CLEAR_VALUE: u32 = 0x0C22;
/// * Groups: `GetPName`, `EnableCap`
pub const GL_COLOR_LOGIC_OP: u32 = 0x0BF2;
/// * Group: `GetPName`
pub const GL_COLOR_WRITEMASK: u32 = 0x0C23;
/// * Group: `TextureCompareMode`
pub const GL_COMPARE_REF_TO_TEXTURE: u32 = 0x884E;
/// * Group: `ShaderParameterName`
pub const GL_COMPILE_STATUS: u32 = 0x8B81;
/// * Group: `InternalFormat`
pub const GL_COMPRESSED_RED: u32 = 0x8225;
/// * Groups: `InternalFormat`, `SizedInternalFormat`
pub const GL_COMPRESSED_RED_RGTC1: u32 = 0x8DBB;
/// * Group: `InternalFormat`
pub const GL_COMPRESSED_RG: u32 = 0x8226;
/// * Group: `InternalFormat`
pub const GL_COMPRESSED_RGB: u32 = 0x84ED;
/// * Group: `InternalFormat`
pub const GL_COMPRESSED_RGBA: u32 = 0x84EE;
/// * Groups: `InternalFormat`, `SizedInternalFormat`
pub const GL_COMPRESSED_RG_RGTC2: u32 = 0x8DBD;
/// * Groups: `InternalFormat`, `SizedInternalFormat`
pub const GL_COMPRESSED_SIGNED_RED_RGTC1: u32 = 0x8DBC;
/// * Groups: `InternalFormat`, `SizedInternalFormat`
pub const GL_COMPRESSED_SIGNED_RG_RGTC2: u32 = 0x8DBE;
/// * Group: `InternalFormat`
pub const GL_COMPRESSED_SRGB: u32 = 0x8C48;
/// * Group: `InternalFormat`
pub const GL_COMPRESSED_SRGB_ALPHA: u32 = 0x8C49;
/// * Group: `GetPName`
pub const GL_COMPRESSED_TEXTURE_FORMATS: u32 = 0x86A3;
/// * Group: `SyncStatus`
pub const GL_CONDITION_SATISFIED: u32 = 0x911C;
/// * Group: `BlendingFactor`
pub const GL_CONSTANT_ALPHA: u32 = 0x8003;
/// * Group: `BlendingFactor`
pub const GL_CONSTANT_COLOR: u32 = 0x8001;
/// * Group: `ContextProfileMask`
pub const GL_CONTEXT_COMPATIBILITY_PROFILE_BIT: u32 = 0x00000002;
/// * Group: `ContextProfileMask`
pub const GL_CONTEXT_CORE_PROFILE_BIT: u32 = 0x00000001;
/// * Group: `GetPName`
pub const GL_CONTEXT_FLAGS: u32 = 0x821E;
/// * Group: `ContextFlagMask`
pub const GL_CONTEXT_FLAG_DEBUG_BIT: u32 = 0x00000002;
/// * Group: `ContextFlagMask`
pub const GL_CONTEXT_FLAG_FORWARD_COMPATIBLE_BIT: u32 = 0x00000001;
/// * Group: `GetPName`
pub const GL_CONTEXT_PROFILE_MASK: u32 = 0x9126;
/// * Group: `LogicOp`
pub const GL_COPY: u32 = 0x1503;
/// * Group: `LogicOp`
pub const GL_COPY_INVERTED: u32 = 0x150C;
/// * Groups: `CopyBufferSubDataTarget`, `BufferTargetARB`,
///   `BufferStorageTarget`
pub const GL_COPY_READ_BUFFER: u32 = 0x8F36;
/// * Groups: `CopyBufferSubDataTarget`, `BufferTargetARB`,
///   `BufferStorageTarget`
pub const GL_COPY_WRITE_BUFFER: u32 = 0x8F37;
/// * Groups: `GetPName`, `EnableCap`
pub const GL_CULL_FACE: u32 = 0x0B44;
/// * Group: `GetPName`
pub const GL_CULL_FACE_MODE: u32 = 0x0B45;
/// * Group: `GetPName`
pub const GL_CURRENT_PROGRAM: u32 = 0x8B8D;
/// * Group: `QueryParameterName`
pub const GL_CURRENT_QUERY: u32 = 0x8865;
/// * Groups: `VertexAttribEnum`, `VertexAttribPropertyARB`
pub const GL_CURRENT_VERTEX_ATTRIB: u32 = 0x8626;
/// * Group: `FrontFaceDirection`
pub const GL_CW: u32 = 0x0900;
/// * Group: `GetPointervPName`
pub const GL_DEBUG_CALLBACK_FUNCTION: u32 = 0x8244;
/// * Group: `GetPointervPName`
pub const GL_DEBUG_CALLBACK_USER_PARAM: u32 = 0x8245;
/// * Group: `GetPName`
pub const GL_DEBUG_GROUP_STACK_DEPTH: u32 = 0x826D;
pub const GL_DEBUG_LOGGED_MESSAGES: u32 = 0x9145;
pub const GL_DEBUG_NEXT_LOGGED_MESSAGE_LENGTH: u32 = 0x8243;
/// * Group: `EnableCap`
pub const GL_DEBUG_OUTPUT: u32 = 0x92E0;
/// * Group: `EnableCap`
pub const GL_DEBUG_OUTPUT_SYNCHRONOUS: u32 = 0x8242;
/// * Group: `DebugSeverity`
pub const GL_DEBUG_SEVERITY_HIGH: u32 = 0x9146;
/// * Group: `DebugSeverity`
pub const GL_DEBUG_SEVERITY_LOW: u32 = 0x9148;
/// * Group: `DebugSeverity`
pub const GL_DEBUG_SEVERITY_MEDIUM: u32 = 0x9147;
/// * Group: `DebugSeverity`
pub const GL_DEBUG_SEVERITY_NOTIFICATION: u32 = 0x826B;
/// * Group: `DebugSource`
pub const GL_DEBUG_SOURCE_API: u32 = 0x8246;
/// * Group: `DebugSource`
pub const GL_DEBUG_SOURCE_APPLICATION: u32 = 0x824A;
/// * Group: `DebugSource`
pub const GL_DEBUG_SOURCE_OTHER: u32 = 0x824B;
/// * Group: `DebugSource`
pub const GL_DEBUG_SOURCE_SHADER_COMPILER: u32 = 0x8248;
/// * Group: `DebugSource`
pub const GL_DEBUG_SOURCE_THIRD_PARTY: u32 = 0x8249;
/// * Group: `DebugSource`
pub const GL_DEBUG_SOURCE_WINDOW_SYSTEM: u32 = 0x8247;
/// * Group: `DebugType`
pub const GL_DEBUG_TYPE_DEPRECATED_BEHAVIOR: u32 = 0x824D;
/// * Group: `DebugType`
pub const GL_DEBUG_TYPE_ERROR: u32 = 0x824C;
/// * Group: `DebugType`
pub const GL_DEBUG_TYPE_MARKER: u32 = 0x8268;
/// * Group: `DebugType`
pub const GL_DEBUG_TYPE_OTHER: u32 = 0x8251;
/// * Group: `DebugType`
pub const GL_DEBUG_TYPE_PERFORMANCE: u32 = 0x8250;
/// * Group: `DebugType`
pub const GL_DEBUG_TYPE_POP_GROUP: u32 = 0x826A;
/// * Group: `DebugType`
pub const GL_DEBUG_TYPE_PORTABILITY: u32 = 0x824F;
/// * Group: `DebugType`
pub const GL_DEBUG_TYPE_PUSH_GROUP: u32 = 0x8269;
/// * Group: `DebugType`
pub const GL_DEBUG_TYPE_UNDEFINED_BEHAVIOR: u32 = 0x824E;
/// * Group: `StencilOp`
pub const GL_DECR: u32 = 0x1E03;
/// * Group: `StencilOp`
pub const GL_DECR_WRAP: u32 = 0x8508;
/// * Groups: `ProgramPropertyARB`, `ShaderParameterName`
pub const GL_DELETE_STATUS: u32 = 0x8B80;
/// * Groups: `Buffer`, `PixelCopyType`, `InvalidateFramebufferAttachment`
pub const GL_DEPTH: u32 = 0x1801;
/// * Groups: `InternalFormat`, `SizedInternalFormat`
pub const GL_DEPTH24_STENCIL8: u32 = 0x88F0;
/// * Groups: `InternalFormat`, `SizedInternalFormat`
pub const GL_DEPTH32F_STENCIL8: u32 = 0x8CAD;
/// * Groups: `InvalidateFramebufferAttachment`, `FramebufferAttachment`
pub const GL_DEPTH_ATTACHMENT: u32 = 0x8D00;
/// * Groups: `ClearBufferMask`, `AttribMask`
pub const GL_DEPTH_BUFFER_BIT: u32 = 0x00000100;
/// * Group: `EnableCap`
pub const GL_DEPTH_CLAMP: u32 = 0x864F;
/// * Group: `GetPName`
pub const GL_DEPTH_CLEAR_VALUE: u32 = 0x0B73;
/// * Groups: `InternalFormat`, `PixelFormat`, `DepthStencilTextureMode`
pub const GL_DEPTH_COMPONENT: u32 = 0x1902;
/// * Groups: `InternalFormat`, `SizedInternalFormat`
pub const GL_DEPTH_COMPONENT16: u32 = 0x81A5;
/// * Groups: `InternalFormat`, `SizedInternalFormat`
pub const GL_DEPTH_COMPONENT24: u32 = 0x81A6;
/// * Groups: `InternalFormat`, `SizedInternalFormat`
pub const GL_DEPTH_COMPONENT32: u32 = 0x81A7;
/// * Groups: `InternalFormat`, `SizedInternalFormat`
pub const GL_DEPTH_COMPONENT32F: u32 = 0x8CAC;
/// * Group: `GetPName`
pub const GL_DEPTH_FUNC: u32 = 0x0B74;
/// * Group: `GetPName`
pub const GL_DEPTH_RANGE: u32 = 0x0B70;
/// * Groups: `InternalFormat`, `PixelFormat`
pub const GL_DEPTH_STENCIL: u32 = 0x84F9;
/// * Groups: `FramebufferAttachment`, `InvalidateFramebufferAttachment`
pub const GL_DEPTH_STENCIL_ATTACHMENT: u32 = 0x821A;
/// * Groups: `GetPName`, `EnableCap`
pub const GL_DEPTH_TEST: u32 = 0x0B71;
/// * Group: `GetPName`
pub const GL_DEPTH_WRITEMASK: u32 = 0x0B72;
/// * Groups: `GetPName`, `EnableCap`
pub const GL_DITHER: u32 = 0x0BD0;
/// * Groups: `DebugSeverity`, `HintMode`, `DebugSource`, `DebugType`
pub const GL_DONT_CARE: u32 = 0x1100;
/// * Groups: `VertexAttribLType`, `MapTypeNV`, `SecondaryColorPointerTypeIBM`,
///   `WeightPointerTypeARB`, `TangentPointerTypeEXT`, `BinormalPointerTypeEXT`,
///   `FogCoordinatePointerType`, `FogPointerTypeEXT`, `FogPointerTypeIBM`,
///   `IndexPointerType`, `NormalPointerType`, `TexCoordPointerType`,
///   `VertexPointerType`, `VertexAttribType`, `AttributeType`, `UniformType`,
///   `VertexAttribPointerType`
pub const GL_DOUBLE: u32 = 0x140A;
/// * Groups: `GetFramebufferParameter`, `GetPName`
pub const GL_DOUBLEBUFFER: u32 = 0x0C32;
/// * Group: `GetPName`
pub const GL_DRAW_BUFFER: u32 = 0x0C01;
pub const GL_DRAW_BUFFER0: u32 = 0x8825;
pub const GL_DRAW_BUFFER1: u32 = 0x8826;
pub const GL_DRAW_BUFFER10: u32 = 0x882F;
pub const GL_DRAW_BUFFER11: u32 = 0x8830;
pub const GL_DRAW_BUFFER12: u32 = 0x8831;
pub const GL_DRAW_BUFFER13: u32 = 0x8832;
pub const GL_DRAW_BUFFER14: u32 = 0x8833;
pub const GL_DRAW_BUFFER15: u32 = 0x8834;
pub const GL_DRAW_BUFFER2: u32 = 0x8827;
pub const GL_DRAW_BUFFER3: u32 = 0x8828;
pub const GL_DRAW_BUFFER4: u32 = 0x8829;
pub const GL_DRAW_BUFFER5: u32 = 0x882A;
pub const GL_DRAW_BUFFER6: u32 = 0x882B;
pub const GL_DRAW_BUFFER7: u32 = 0x882C;
pub const GL_DRAW_BUFFER8: u32 = 0x882D;
pub const GL_DRAW_BUFFER9: u32 = 0x882E;
/// * Group: `FramebufferTarget`
pub const GL_DRAW_FRAMEBUFFER: u32 = 0x8CA9;
/// * Group: `GetPName`
pub const GL_DRAW_FRAMEBUFFER_BINDING: u32 = 0x8CA6;
/// * Group: `BlendingFactor`
pub const GL_DST_ALPHA: u32 = 0x0304;
/// * Group: `BlendingFactor`
pub const GL_DST_COLOR: u32 = 0x0306;
/// * Groups: `VertexBufferObjectUsage`, `BufferUsageARB`
pub const GL_DYNAMIC_COPY: u32 = 0x88EA;
/// * Groups: `VertexBufferObjectUsage`, `BufferUsageARB`
pub const GL_DYNAMIC_DRAW: u32 = 0x88E8;
/// * Groups: `VertexBufferObjectUsage`, `BufferUsageARB`
pub const GL_DYNAMIC_READ: u32 = 0x88E9;
/// * Groups: `CopyBufferSubDataTarget`, `BufferTargetARB`,
///   `BufferStorageTarget`
pub const GL_ELEMENT_ARRAY_BUFFER: u32 = 0x8893;
/// * Group: `GetPName`
pub const GL_ELEMENT_ARRAY_BUFFER_BINDING: u32 = 0x8895;
/// * Groups: `StencilFunction`, `IndexFunctionEXT`, `AlphaFunction`,
///   `DepthFunction`
pub const GL_EQUAL: u32 = 0x0202;
/// * Group: `LogicOp`
pub const GL_EQUIV: u32 = 0x1509;
/// * Group: `StringName`
pub const GL_EXTENSIONS: u32 = 0x1F03;
/// * Groups: `SpecialNumbers`, `Boolean`, `VertexShaderWriteMaskEXT`,
///   `ClampColorModeARB`
pub const GL_FALSE: u32 = 0;
/// * Group: `HintMode`
pub const GL_FASTEST: u32 = 0x1101;
/// * Groups: `PolygonMode`, `MeshMode2`
pub const GL_FILL: u32 = 0x1B02;
/// * Group: `VertexProvokingMode`
pub const GL_FIRST_VERTEX_CONVENTION: u32 = 0x8E4D;
/// * Group: `ClampColorModeARB`
pub const GL_FIXED_ONLY: u32 = 0x891D;
/// * Groups: `MapTypeNV`, `SecondaryColorPointerTypeIBM`,
///   `WeightPointerTypeARB`, `VertexWeightPointerTypeEXT`,
///   `TangentPointerTypeEXT`, `BinormalPointerTypeEXT`,
///   `FogCoordinatePointerType`, `FogPointerTypeEXT`, `FogPointerTypeIBM`,
///   `IndexPointerType`, `ListNameType`, `NormalPointerType`, `PixelType`,
///   `TexCoordPointerType`, `VertexPointerType`, `VertexAttribType`,
///   `AttributeType`, `UniformType`, `VertexAttribPointerType`
pub const GL_FLOAT: u32 = 0x1406;
/// * Group: `PixelType`
pub const GL_FLOAT_32_UNSIGNED_INT_24_8_REV: u32 = 0x8DAD;
/// * Groups: `AttributeType`, `UniformType`
pub const GL_FLOAT_MAT2: u32 = 0x8B5A;
/// * Groups: `AttributeType`, `UniformType`
#[allow(non_upper_case_globals)]
pub const GL_FLOAT_MAT2x3: u32 = 0x8B65;
/// * Groups: `AttributeType`, `UniformType`
#[allow(non_upper_case_globals)]
pub const GL_FLOAT_MAT2x4: u32 = 0x8B66;
/// * Groups: `AttributeType`, `UniformType`
pub const GL_FLOAT_MAT3: u32 = 0x8B5B;
/// * Groups: `AttributeType`, `UniformType`
#[allow(non_upper_case_globals)]
pub const GL_FLOAT_MAT3x2: u32 = 0x8B67;
/// * Groups: `AttributeType`, `UniformType`
#[allow(non_upper_case_globals)]
pub const GL_FLOAT_MAT3x4: u32 = 0x8B68;
/// * Groups: `AttributeType`, `UniformType`
pub const GL_FLOAT_MAT4: u32 = 0x8B5C;
/// * Groups: `AttributeType`, `UniformType`
#[allow(non_upper_case_globals)]
pub const GL_FLOAT_MAT4x2: u32 = 0x8B69;
/// * Groups: `AttributeType`, `UniformType`
#[allow(non_upper_case_globals)]
pub const GL_FLOAT_MAT4x3: u32 = 0x8B6A;
/// * Groups: `AttributeType`, `UniformType`
pub const GL_FLOAT_VEC2: u32 = 0x8B50;
/// * Groups: `AttributeType`, `UniformType`
pub const GL_FLOAT_VEC3: u32 = 0x8B51;
/// * Groups: `AttributeType`, `UniformType`
pub const GL_FLOAT_VEC4: u32 = 0x8B52;
/// * Groups: `PipelineParameterName`, `ShaderType`
pub const GL_FRAGMENT_SHADER: u32 = 0x8B30;
/// * Groups: `HintTarget`, `GetPName`
pub const GL_FRAGMENT_SHADER_DERIVATIVE_HINT: u32 = 0x8B8B;
/// * Groups: `ObjectIdentifier`, `FramebufferTarget`
pub const GL_FRAMEBUFFER: u32 = 0x8D40;
/// * Group: `FramebufferAttachmentParameterName`
pub const GL_FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE: u32 = 0x8215;
/// * Group: `FramebufferAttachmentParameterName`
pub const GL_FRAMEBUFFER_ATTACHMENT_BLUE_SIZE: u32 = 0x8214;
/// * Group: `FramebufferAttachmentParameterName`
pub const GL_FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING: u32 = 0x8210;
/// * Group: `FramebufferAttachmentParameterName`
pub const GL_FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE: u32 = 0x8211;
/// * Group: `FramebufferAttachmentParameterName`
pub const GL_FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE: u32 = 0x8216;
/// * Group: `FramebufferAttachmentParameterName`
pub const GL_FRAMEBUFFER_ATTACHMENT_GREEN_SIZE: u32 = 0x8213;
/// * Group: `FramebufferAttachmentParameterName`
pub const GL_FRAMEBUFFER_ATTACHMENT_LAYERED: u32 = 0x8DA7;
/// * Group: `FramebufferAttachmentParameterName`
pub const GL_FRAMEBUFFER_ATTACHMENT_OBJECT_NAME: u32 = 0x8CD1;
/// * Group: `FramebufferAttachmentParameterName`
pub const GL_FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE: u32 = 0x8CD0;
/// * Group: `FramebufferAttachmentParameterName`
pub const GL_FRAMEBUFFER_ATTACHMENT_RED_SIZE: u32 = 0x8212;
/// * Group: `FramebufferAttachmentParameterName`
pub const GL_FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE: u32 = 0x8217;
/// * Group: `FramebufferAttachmentParameterName`
pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE: u32 = 0x8CD3;
/// * Group: `FramebufferAttachmentParameterName`
pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER: u32 = 0x8CD4;
/// * Group: `FramebufferAttachmentParameterName`
pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL: u32 = 0x8CD2;
pub const GL_FRAMEBUFFER_BINDING: u32 = 0x8CA6;
/// * Group: `FramebufferStatus`
pub const GL_FRAMEBUFFER_COMPLETE: u32 = 0x8CD5;
pub const GL_FRAMEBUFFER_DEFAULT: u32 = 0x8218;
/// * Group: `FramebufferStatus`
pub const GL_FRAMEBUFFER_INCOMPLETE_ATTACHMENT: u32 = 0x8CD6;
/// * Group: `FramebufferStatus`
pub const GL_FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER: u32 = 0x8CDB;
/// * Group: `FramebufferStatus`
pub const GL_FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS: u32 = 0x8DA8;
/// * Group: `FramebufferStatus`
pub const GL_FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT: u32 = 0x8CD7;
/// * Group: `FramebufferStatus`
pub const GL_FRAMEBUFFER_INCOMPLETE_MULTISAMPLE: u32 = 0x8D56;
/// * Group: `FramebufferStatus`
pub const GL_FRAMEBUFFER_INCOMPLETE_READ_BUFFER: u32 = 0x8CDC;
/// * Group: `EnableCap`
pub const GL_FRAMEBUFFER_SRGB: u32 = 0x8DB9;
/// * Group: `FramebufferStatus`
pub const GL_FRAMEBUFFER_UNDEFINED: u32 = 0x8219;
/// * Group: `FramebufferStatus`
pub const GL_FRAMEBUFFER_UNSUPPORTED: u32 = 0x8CDD;
/// * Groups: `ColorBuffer`, `DrawBufferMode`, `ReadBufferMode`, `TriangleFace`
pub const GL_FRONT: u32 = 0x0404;
/// * Groups: `ColorBuffer`, `DrawBufferMode`, `TriangleFace`
pub const GL_FRONT_AND_BACK: u32 = 0x0408;
/// * Group: `GetPName`
pub const GL_FRONT_FACE: u32 = 0x0B46;
/// * Groups: `ColorBuffer`, `DrawBufferMode`, `ReadBufferMode`
pub const GL_FRONT_LEFT: u32 = 0x0400;
/// * Groups: `ColorBuffer`, `DrawBufferMode`, `ReadBufferMode`
pub const GL_FRONT_RIGHT: u32 = 0x0401;
/// * Group: `BlendEquationModeEXT`
pub const GL_FUNC_ADD: u32 = 0x8006;
/// * Group: `BlendEquationModeEXT`
pub const GL_FUNC_REVERSE_SUBTRACT: u32 = 0x800B;
/// * Group: `BlendEquationModeEXT`
pub const GL_FUNC_SUBTRACT: u32 = 0x800A;
/// * Group: `ProgramPropertyARB`
pub const GL_GEOMETRY_INPUT_TYPE: u32 = 0x8917;
/// * Group: `ProgramPropertyARB`
pub const GL_GEOMETRY_OUTPUT_TYPE: u32 = 0x8918;
/// * Groups: `PipelineParameterName`, `ShaderType`
pub const GL_GEOMETRY_SHADER: u32 = 0x8DD9;
/// * Group: `ProgramPropertyARB`
pub const GL_GEOMETRY_VERTICES_OUT: u32 = 0x8916;
/// * Groups: `StencilFunction`, `IndexFunctionEXT`, `AlphaFunction`,
///   `DepthFunction`
pub const GL_GEQUAL: u32 = 0x0206;
/// * Groups: `StencilFunction`, `IndexFunctionEXT`, `AlphaFunction`,
///   `DepthFunction`
pub const GL_GREATER: u32 = 0x0204;
/// * Groups: `FragmentShaderValueRepATI`, `TextureSwizzle`, `PixelFormat`
pub const GL_GREEN: u32 = 0x1904;
/// * Group: `PixelFormat`
pub const GL_GREEN_INTEGER: u32 = 0x8D95;
/// * Groups: `PixelType`, `VertexAttribPointerType`, `VertexAttribType`
pub const GL_HALF_FLOAT: u32 = 0x140B;
/// * Group: `StencilOp`
pub const GL_INCR: u32 = 0x1E02;
/// * Group: `StencilOp`
pub const GL_INCR_WRAP: u32 = 0x8507;
/// * Groups: `ProgramPropertyARB`, `ShaderParameterName`,
///   `PipelineParameterName`
pub const GL_INFO_LOG_LENGTH: u32 = 0x8B84;
/// * Groups: `VertexAttribIType`, `SecondaryColorPointerTypeIBM`,
///   `WeightPointerTypeARB`, `TangentPointerTypeEXT`, `BinormalPointerTypeEXT`,
///   `IndexPointerType`, `ListNameType`, `NormalPointerType`, `PixelType`,
///   `TexCoordPointerType`, `VertexPointerType`, `VertexAttribType`,
///   `AttributeType`, `UniformType`, `VertexAttribPointerType`
pub const GL_INT: u32 = 0x1404;
/// * Group: `TransformFeedbackBufferMode`
pub const GL_INTERLEAVED_ATTRIBS: u32 = 0x8C8C;
/// * Groups: `VertexAttribPointerType`, `VertexAttribType`
pub const GL_INT_2_10_10_10_REV: u32 = 0x8D9F;
/// * Groups: `AttributeType`, `UniformType`
pub const GL_INT_SAMPLER_1D: u32 = 0x8DC9;
/// * Groups: `AttributeType`, `UniformType`
pub const GL_INT_SAMPLER_1D_ARRAY: u32 = 0x8DCE;
/// * Groups: `AttributeType`, `UniformType`
pub const GL_INT_SAMPLER_2D: u32 = 0x8DCA;
/// * Groups: `AttributeType`, `UniformType`
pub const GL_INT_SAMPLER_2D_ARRAY: u32 = 0x8DCF;
/// * Groups: `AttributeType`, `UniformType`
pub const GL_INT_SAMPLER_2D_MULTISAMPLE: u32 = 0x9109;
/// * Groups: `AttributeType`, `UniformType`
pub const GL_INT_SAMPLER_2D_MULTISAMPLE_ARRAY: u32 = 0x910C;
/// * Groups: `AttributeType`, `UniformType`
pub const GL_INT_SAMPLER_2D_RECT: u32 = 0x8DCD;
/// * Groups: `AttributeType`, `UniformType`
pub const GL_INT_SAMPLER_3D: u32 = 0x8DCB;
/// * Groups: `AttributeType`, `UniformType`
pub const GL_INT_SAMPLER_BUFFER: u32 = 0x8DD0;
/// * Groups: `AttributeType`, `UniformType`
pub const GL_INT_SAMPLER_CUBE: u32 = 0x8DCC;
/// * Groups: `AttributeType`, `UniformType`
pub const GL_INT_VEC2: u32 = 0x8B53;
/// * Groups: `AttributeType`, `UniformType`
pub const GL_INT_VEC3: u32 = 0x8B54;
/// * Groups: `AttributeType`, `UniformType`
pub const GL_INT_VEC4: u32 = 0x8B55;
/// * Group: `ErrorCode`
pub const GL_INVALID_ENUM: u32 = 0x0500;
/// * Group: `ErrorCode`
pub const GL_INVALID_FRAMEBUFFER_OPERATION: u32 = 0x0506;
/// * Group: `SpecialNumbers`
pub const GL_INVALID_INDEX: u32 = 0xFFFFFFFF;
/// * Group: `ErrorCode`
pub const GL_INVALID_OPERATION: u32 = 0x0502;
/// * Group: `ErrorCode`
pub const GL_INVALID_VALUE: u32 = 0x0501;
/// * Groups: `PathFillMode`, `LogicOp`, `StencilOp`
pub const GL_INVERT: u32 = 0x150A;
/// * Group: `StencilOp`
pub const GL_KEEP: u32 = 0x1E00;
/// * Group: `VertexProvokingMode`
pub const GL_LAST_VERTEX_CONVENTION: u32 = 0x8E4E;
/// * Groups: `ColorBuffer`, `DrawBufferMode`, `ReadBufferMode`
pub const GL_LEFT: u32 = 0x0406;
/// * Groups: `StencilFunction`, `IndexFunctionEXT`, `AlphaFunction`,
///   `DepthFunction`
pub const GL_LEQUAL: u32 = 0x0203;
/// * Groups: `StencilFunction`, `IndexFunctionEXT`, `AlphaFunction`,
///   `DepthFunction`
pub const GL_LESS: u32 = 0x0201;
/// * Groups: `PolygonMode`, `MeshMode1`, `MeshMode2`
pub const GL_LINE: u32 = 0x1B01;
/// * Groups: `BlitFramebufferFilter`, `FogMode`, `TextureMagFilter`,
///   `TextureMinFilter`
pub const GL_LINEAR: u32 = 0x2601;
/// * Groups: `TextureWrapMode`, `TextureMinFilter`
pub const GL_LINEAR_MIPMAP_LINEAR: u32 = 0x2703;
/// * Group: `TextureMinFilter`
pub const GL_LINEAR_MIPMAP_NEAREST: u32 = 0x2701;
/// * Group: `PrimitiveType`
pub const GL_LINES: u32 = 0x0001;
/// * Group: `PrimitiveType`
pub const GL_LINES_ADJACENCY: u32 = 0x000A;
/// * Group: `PrimitiveType`
pub const GL_LINE_LOOP: u32 = 0x0002;
/// * Groups: `GetPName`, `EnableCap`
pub const GL_LINE_SMOOTH: u32 = 0x0B20;
/// * Groups: `HintTarget`, `GetPName`
pub const GL_LINE_SMOOTH_HINT: u32 = 0x0C52;
/// * Group: `PrimitiveType`
pub const GL_LINE_STRIP: u32 = 0x0003;
/// * Group: `PrimitiveType`
pub const GL_LINE_STRIP_ADJACENCY: u32 = 0x000B;
/// * Group: `GetPName`
pub const GL_LINE_WIDTH: u32 = 0x0B21;
/// * Group: `GetPName`
pub const GL_LINE_WIDTH_GRANULARITY: u32 = 0x0B23;
/// * Group: `GetPName`
pub const GL_LINE_WIDTH_RANGE: u32 = 0x0B22;
/// * Group: `ProgramPropertyARB`
pub const GL_LINK_STATUS: u32 = 0x8B82;
/// * Group: `GetPName`
pub const GL_LOGIC_OP_MODE: u32 = 0x0BF0;
/// * Group: `ClipControlOrigin`
pub const GL_LOWER_LEFT: u32 = 0x8CA1;
/// * Group: `GetPName`
pub const GL_MAJOR_VERSION: u32 = 0x821B;
/// * Group: `MapBufferAccessMask`
pub const GL_MAP_FLUSH_EXPLICIT_BIT: u32 = 0x0010;
/// * Group: `MapBufferAccessMask`
pub const GL_MAP_INVALIDATE_BUFFER_BIT: u32 = 0x0008;
/// * Group: `MapBufferAccessMask`
pub const GL_MAP_INVALIDATE_RANGE_BIT: u32 = 0x0004;
/// * Groups: `MapBufferAccessMask`, `BufferStorageMask`
pub const GL_MAP_READ_BIT: u32 = 0x0001;
/// * Group: `MapBufferAccessMask`
pub const GL_MAP_UNSYNCHRONIZED_BIT: u32 = 0x0020;
/// * Groups: `MapBufferAccessMask`, `BufferStorageMask`
pub const GL_MAP_WRITE_BIT: u32 = 0x0002;
/// * Group: `BlendEquationModeEXT`
pub const GL_MAX: u32 = 0x8008;
/// * Group: `GetPName`
pub const GL_MAX_3D_TEXTURE_SIZE: u32 = 0x8073;
/// * Group: `GetPName`
pub const GL_MAX_ARRAY_TEXTURE_LAYERS: u32 = 0x88FF;
/// * Group: `GetPName`
pub const GL_MAX_CLIP_DISTANCES: u32 = 0x0D32;
/// * Group: `GetPName`
pub const GL_MAX_COLOR_ATTACHMENTS: u32 = 0x8CDF;
/// * Group: `GetPName`
pub const GL_MAX_COLOR_TEXTURE_SAMPLES: u32 = 0x910E;
/// * Group: `GetPName`
pub const GL_MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS: u32 = 0x8A33;
/// * Group: `GetPName`
pub const GL_MAX_COMBINED_GEOMETRY_UNIFORM_COMPONENTS: u32 = 0x8A32;
/// * Group: `GetPName`
pub const GL_MAX_COMBINED_TEXTURE_IMAGE_UNITS: u32 = 0x8B4D;
/// * Group: `GetPName`
pub const GL_MAX_COMBINED_UNIFORM_BLOCKS: u32 = 0x8A2E;
/// * Group: `GetPName`
pub const GL_MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS: u32 = 0x8A31;
/// * Group: `GetPName`
pub const GL_MAX_CUBE_MAP_TEXTURE_SIZE: u32 = 0x851C;
/// * Group: `GetPName`
pub const GL_MAX_DEBUG_GROUP_STACK_DEPTH: u32 = 0x826C;
pub const GL_MAX_DEBUG_LOGGED_MESSAGES: u32 = 0x9144;
pub const GL_MAX_DEBUG_MESSAGE_LENGTH: u32 = 0x9143;
/// * Group: `GetPName`
pub const GL_MAX_DEPTH_TEXTURE_SAMPLES: u32 = 0x910F;
/// * Group: `GetPName`
pub const GL_MAX_DRAW_BUFFERS: u32 = 0x8824;
/// * Group: `GetPName`
pub const GL_MAX_DUAL_SOURCE_DRAW_BUFFERS: u32 = 0x88FC;
/// * Group: `GetPName`
pub const GL_MAX_ELEMENTS_INDICES: u32 = 0x80E9;
/// * Group: `GetPName`
pub const GL_MAX_ELEMENTS_VERTICES: u32 = 0x80E8;
/// * Group: `GetPName`
pub const GL_MAX_FRAGMENT_INPUT_COMPONENTS: u32 = 0x9125;
/// * Group: `GetPName`
pub const GL_MAX_FRAGMENT_UNIFORM_BLOCKS: u32 = 0x8A2D;
/// * Group: `GetPName`
pub const GL_MAX_FRAGMENT_UNIFORM_COMPONENTS: u32 = 0x8B49;
/// * Group: `GetPName`
pub const GL_MAX_GEOMETRY_INPUT_COMPONENTS: u32 = 0x9123;
/// * Group: `GetPName`
pub const GL_MAX_GEOMETRY_OUTPUT_COMPONENTS: u32 = 0x9124;
pub const GL_MAX_GEOMETRY_OUTPUT_VERTICES: u32 = 0x8DE0;
/// * Group: `GetPName`
pub const GL_MAX_GEOMETRY_TEXTURE_IMAGE_UNITS: u32 = 0x8C29;
pub const GL_MAX_GEOMETRY_TOTAL_OUTPUT_COMPONENTS: u32 = 0x8DE1;
/// * Group: `GetPName`
pub const GL_MAX_GEOMETRY_UNIFORM_BLOCKS: u32 = 0x8A2C;
/// * Group: `GetPName`
pub const GL_MAX_GEOMETRY_UNIFORM_COMPONENTS: u32 = 0x8DDF;
/// * Group: `GetPName`
pub const GL_MAX_INTEGER_SAMPLES: u32 = 0x9110;
/// * Group: `GetPName`
pub const GL_MAX_LABEL_LENGTH: u32 = 0x82E8;
/// * Group: `GetPName`
pub const GL_MAX_PROGRAM_TEXEL_OFFSET: u32 = 0x8905;
/// * Group: `GetPName`
pub const GL_MAX_RECTANGLE_TEXTURE_SIZE: u32 = 0x84F8;
/// * Group: `GetPName`
pub const GL_MAX_RENDERBUFFER_SIZE: u32 = 0x84E8;
pub const GL_MAX_SAMPLES: u32 = 0x8D57;
/// * Group: `GetPName`
pub const GL_MAX_SAMPLE_MASK_WORDS: u32 = 0x8E59;
/// * Group: `GetPName`
pub const GL_MAX_SERVER_WAIT_TIMEOUT: u32 = 0x9111;
/// * Group: `GetPName`
pub const GL_MAX_TEXTURE_BUFFER_SIZE: u32 = 0x8C2B;
/// * Group: `GetPName`
pub const GL_MAX_TEXTURE_IMAGE_UNITS: u32 = 0x8872;
/// * Group: `GetPName`
pub const GL_MAX_TEXTURE_LOD_BIAS: u32 = 0x84FD;
/// * Group: `GetPName`
pub const GL_MAX_TEXTURE_SIZE: u32 = 0x0D33;
pub const GL_MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS: u32 = 0x8C8A;
pub const GL_MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS: u32 = 0x8C8B;
pub const GL_MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS: u32 = 0x8C80;
/// * Group: `GetPName`
pub const GL_MAX_UNIFORM_BLOCK_SIZE: u32 = 0x8A30;
/// * Group: `GetPName`
pub const GL_MAX_UNIFORM_BUFFER_BINDINGS: u32 = 0x8A2F;
/// * Group: `GetPName`
pub const GL_MAX_VARYING_COMPONENTS: u32 = 0x8B4B;
/// * Group: `GetPName`
pub const GL_MAX_VARYING_FLOATS: u32 = 0x8B4B;
/// * Group: `GetPName`
pub const GL_MAX_VERTEX_ATTRIBS: u32 = 0x8869;
/// * Group: `GetPName`
pub const GL_MAX_VERTEX_OUTPUT_COMPONENTS: u32 = 0x9122;
/// * Group: `GetPName`
pub const GL_MAX_VERTEX_TEXTURE_IMAGE_UNITS: u32 = 0x8B4C;
/// * Group: `GetPName`
pub const GL_MAX_VERTEX_UNIFORM_BLOCKS: u32 = 0x8A2B;
/// * Group: `GetPName`
pub const GL_MAX_VERTEX_UNIFORM_COMPONENTS: u32 = 0x8B4A;
/// * Group: `GetPName`
pub const GL_MAX_VIEWPORT_DIMS: u32 = 0x0D3A;
/// * Group: `BlendEquationModeEXT`
pub const GL_MIN: u32 = 0x8007;
/// * Group: `GetPName`
pub const GL_MINOR_VERSION: u32 = 0x821C;
/// * Group: `GetPName`
pub const GL_MIN_PROGRAM_TEXEL_OFFSET: u32 = 0x8904;
/// * Group: `TextureWrapMode`
pub const GL_MIRRORED_REPEAT: u32 = 0x8370;
/// * Group: `EnableCap`
pub const GL_MULTISAMPLE: u32 = 0x809D;
/// * Group: `LogicOp`
pub const GL_NAND: u32 = 0x150E;
/// * Groups: `BlitFramebufferFilter`, `TextureMagFilter`, `TextureMinFilter`
pub const GL_NEAREST: u32 = 0x2600;
/// * Group: `TextureMinFilter`
pub const GL_NEAREST_MIPMAP_LINEAR: u32 = 0x2702;
/// * Group: `TextureMinFilter`
pub const GL_NEAREST_MIPMAP_NEAREST: u32 = 0x2700;
/// * Groups: `StencilFunction`, `IndexFunctionEXT`, `AlphaFunction`,
///   `DepthFunction`
pub const GL_NEVER: u32 = 0x0200;
/// * Group: `HintMode`
pub const GL_NICEST: u32 = 0x1102;
/// * Groups: `SpecialNumbers`, `FragmentShaderValueRepATI`,
///   `FragmentShaderDestModMaskATI`, `FragmentShaderDestMaskATI`,
///   `SyncBehaviorFlags`, `TextureCompareMode`, `PathColorFormat`,
///   `CombinerBiasNV`, `CombinerScaleNV`, `DrawBufferMode`,
///   `PixelTexGenModeSGIX`, `ReadBufferMode`, `ColorBuffer`, `PathGenMode`,
///   `PathTransformType`, `PathFontStyle`
pub const GL_NONE: u32 = 0;
/// * Group: `LogicOp`
pub const GL_NOOP: u32 = 0x1505;
/// * Group: `LogicOp`
pub const GL_NOR: u32 = 0x1508;
/// * Groups: `StencilFunction`, `IndexFunctionEXT`, `AlphaFunction`,
///   `DepthFunction`
pub const GL_NOTEQUAL: u32 = 0x0205;
/// * Groups: `SpecialNumbers`, `GraphicsResetStatus`, `ErrorCode`
pub const GL_NO_ERROR: u32 = 0;
/// * Group: `GetPName`
pub const GL_NUM_COMPRESSED_TEXTURE_FORMATS: u32 = 0x86A2;
/// * Group: `GetPName`
pub const GL_NUM_EXTENSIONS: u32 = 0x821D;
/// * Group: `SyncParameterName`
pub const GL_OBJECT_TYPE: u32 = 0x9112;
/// * Groups: `SpecialNumbers`, `TextureSwizzle`, `BlendingFactor`,
///   `FragmentShaderGenericSourceATI`
pub const GL_ONE: u32 = 1;
/// * Group: `BlendingFactor`
pub const GL_ONE_MINUS_CONSTANT_ALPHA: u32 = 0x8004;
/// * Group: `BlendingFactor`
pub const GL_ONE_MINUS_CONSTANT_COLOR: u32 = 0x8002;
/// * Group: `BlendingFactor`
pub const GL_ONE_MINUS_DST_ALPHA: u32 = 0x0305;
/// * Group: `BlendingFactor`
pub const GL_ONE_MINUS_DST_COLOR: u32 = 0x0307;
/// * Group: `BlendingFactor`
pub const GL_ONE_MINUS_SRC1_ALPHA: u32 = 0x88FB;
/// * Group: `BlendingFactor`
pub const GL_ONE_MINUS_SRC1_COLOR: u32 = 0x88FA;
/// * Group: `BlendingFactor`
pub const GL_ONE_MINUS_SRC_ALPHA: u32 = 0x0303;
/// * Group: `BlendingFactor`
pub const GL_ONE_MINUS_SRC_COLOR: u32 = 0x0301;
/// * Group: `LogicOp`
pub const GL_OR: u32 = 0x1507;
/// * Group: `LogicOp`
pub const GL_OR_INVERTED: u32 = 0x150D;
/// * Group: `LogicOp`
pub const GL_OR_REVERSE: u32 = 0x150B;
/// * Group: `ErrorCode`
pub const GL_OUT_OF_MEMORY: u32 = 0x0505;
/// * Groups: `PixelStoreParameter`, `GetPName`
pub const GL_PACK_ALIGNMENT: u32 = 0x0D05;
/// * Groups: `PixelStoreParameter`, `GetPName`
pub const GL_PACK_IMAGE_HEIGHT: u32 = 0x806C;
/// * Groups: `PixelStoreParameter`, `GetPName`
pub const GL_PACK_LSB_FIRST: u32 = 0x0D01;
/// * Groups: `PixelStoreParameter`, `GetPName`
pub const GL_PACK_ROW_LENGTH: u32 = 0x0D02;
/// * Groups: `PixelStoreParameter`, `GetPName`
pub const GL_PACK_SKIP_IMAGES: u32 = 0x806B;
/// * Groups: `PixelStoreParameter`, `GetPName`
pub const GL_PACK_SKIP_PIXELS: u32 = 0x0D04;
/// * Groups: `PixelStoreParameter`, `GetPName`
pub const GL_PACK_SKIP_ROWS: u32 = 0x0D03;
/// * Groups: `PixelStoreParameter`, `GetPName`
pub const GL_PACK_SWAP_BYTES: u32 = 0x0D00;
/// * Groups: `CopyBufferSubDataTarget`, `BufferTargetARB`,
///   `BufferStorageTarget`
pub const GL_PIXEL_PACK_BUFFER: u32 = 0x88EB;
/// * Group: `GetPName`
pub const GL_PIXEL_PACK_BUFFER_BINDING: u32 = 0x88ED;
/// * Groups: `CopyBufferSubDataTarget`, `BufferTargetARB`,
///   `BufferStorageTarget`
pub const GL_PIXEL_UNPACK_BUFFER: u32 = 0x88EC;
/// * Group: `GetPName`
pub const GL_PIXEL_UNPACK_BUFFER_BINDING: u32 = 0x88EF;
/// * Groups: `PolygonMode`, `MeshMode1`, `MeshMode2`
pub const GL_POINT: u32 = 0x1B00;
/// * Group: `PrimitiveType`
pub const GL_POINTS: u32 = 0x0000;
/// * Groups: `PointParameterNameARB`, `GetPName`
pub const GL_POINT_FADE_THRESHOLD_SIZE: u32 = 0x8128;
/// * Group: `GetPName`
pub const GL_POINT_SIZE: u32 = 0x0B11;
/// * Group: `GetPName`
pub const GL_POINT_SIZE_GRANULARITY: u32 = 0x0B13;
/// * Group: `GetPName`
pub const GL_POINT_SIZE_RANGE: u32 = 0x0B12;
pub const GL_POINT_SPRITE_COORD_ORIGIN: u32 = 0x8CA0;
/// * Group: `GetPName`
pub const GL_POLYGON_MODE: u32 = 0x0B40;
/// * Group: `GetPName`
pub const GL_POLYGON_OFFSET_FACTOR: u32 = 0x8038;
/// * Groups: `GetPName`, `EnableCap`
pub const GL_POLYGON_OFFSET_FILL: u32 = 0x8037;
/// * Groups: `GetPName`, `EnableCap`
pub const GL_POLYGON_OFFSET_LINE: u32 = 0x2A02;
/// * Groups: `GetPName`, `EnableCap`
pub const GL_POLYGON_OFFSET_POINT: u32 = 0x2A01;
/// * Group: `GetPName`
pub const GL_POLYGON_OFFSET_UNITS: u32 = 0x2A00;
/// * Groups: `GetPName`, `EnableCap`
pub const GL_POLYGON_SMOOTH: u32 = 0x0B41;
/// * Groups: `HintTarget`, `GetPName`
pub const GL_POLYGON_SMOOTH_HINT: u32 = 0x0C53;
/// * Group: `QueryTarget`
pub const GL_PRIMITIVES_GENERATED: u32 = 0x8C87;
/// * Group: `EnableCap`
pub const GL_PRIMITIVE_RESTART: u32 = 0x8F9D;
/// * Group: `GetPName`
pub const GL_PRIMITIVE_RESTART_INDEX: u32 = 0x8F9E;
/// * Group: `ObjectIdentifier`
pub const GL_PROGRAM: u32 = 0x82E2;
/// * Group: `ObjectIdentifier`
pub const GL_PROGRAM_PIPELINE: u32 = 0x82E4;
/// * Groups: `GetPName`, `EnableCap`
pub const GL_PROGRAM_POINT_SIZE: u32 = 0x8642;
/// * Group: `GetPName`
pub const GL_PROVOKING_VERTEX: u32 = 0x8E4F;
/// * Group: `TextureTarget`
pub const GL_PROXY_TEXTURE_1D: u32 = 0x8063;
/// * Group: `TextureTarget`
pub const GL_PROXY_TEXTURE_1D_ARRAY: u32 = 0x8C19;
/// * Group: `TextureTarget`
pub const GL_PROXY_TEXTURE_2D: u32 = 0x8064;
/// * Group: `TextureTarget`
pub const GL_PROXY_TEXTURE_2D_ARRAY: u32 = 0x8C1B;
/// * Group: `TextureTarget`
pub const GL_PROXY_TEXTURE_2D_MULTISAMPLE: u32 = 0x9101;
/// * Group: `TextureTarget`
pub const GL_PROXY_TEXTURE_2D_MULTISAMPLE_ARRAY: u32 = 0x9103;
/// * Group: `TextureTarget`
pub const GL_PROXY_TEXTURE_3D: u32 = 0x8070;
/// * Group: `TextureTarget`
pub const GL_PROXY_TEXTURE_CUBE_MAP: u32 = 0x851B;
/// * Group: `TextureTarget`
pub const GL_PROXY_TEXTURE_RECTANGLE: u32 = 0x84F7;
pub const GL_QUADS_FOLLOW_PROVOKING_VERTEX_CONVENTION: u32 = 0x8E4C;
/// * Group: `ObjectIdentifier`
pub const GL_QUERY: u32 = 0x82E3;
/// * Group: `ConditionalRenderMode`
pub const GL_QUERY_BY_REGION_NO_WAIT: u32 = 0x8E16;
/// * Group: `ConditionalRenderMode`
pub const GL_QUERY_BY_REGION_WAIT: u32 = 0x8E15;
/// * Group: `QueryParameterName`
pub const GL_QUERY_COUNTER_BITS: u32 = 0x8864;
/// * Group: `ConditionalRenderMode`
pub const GL_QUERY_NO_WAIT: u32 = 0x8E14;
/// * Group: `QueryObjectParameterName`
pub const GL_QUERY_RESULT: u32 = 0x8866;
/// * Group: `QueryObjectParameterName`
pub const GL_QUERY_RESULT_AVAILABLE: u32 = 0x8867;
/// * Group: `ConditionalRenderMode`
pub const GL_QUERY_WAIT: u32 = 0x8E13;
/// * Groups: `InternalFormat`, `SizedInternalFormat`
pub const GL_R11F_G11F_B10F: u32 = 0x8C3A;
/// * Groups: `InternalFormat`, `SizedInternalFormat`
pub const GL_R16: u32 = 0x822A;
/// * Groups: `InternalFormat`, `SizedInternalFormat`
pub const GL_R16F: u32 = 0x822D;
/// * Groups: `InternalFormat`, `SizedInternalFormat`
pub const GL_R16I: u32 = 0x8233;
/// * Groups: `InternalFormat`, `SizedInternalFormat`
pub const GL_R16UI: u32 = 0x8234;
/// * Groups: `InternalFormat`, `SizedInternalFormat`
pub const GL_R16_SNORM: u32 = 0x8F98;
/// * Groups: `InternalFormat`, `SizedInternalFormat`
pub const GL_R32F: u32 = 0x822E;
/// * Groups: `InternalFormat`, `SizedInternalFormat`
pub const GL_R32I: u32 = 0x8235;
/// * Groups: `InternalFormat`, `SizedInternalFormat`
pub const GL_R32UI: u32 = 0x8236;
/// * Groups: `InternalFormat`, `SizedInternalFormat`
pub const GL_R3_G3_B2: u32 = 0x2A10;
/// * Groups: `InternalFormat`, `SizedInternalFormat`
pub const GL_R8: u32 = 0x8229;
/// * Groups: `InternalFormat`, `SizedInternalFormat`
pub const GL_R8I: u32 = 0x8231;
/// * Groups: `InternalFormat`, `SizedInternalFormat`
pub const GL_R8UI: u32 = 0x8232;
/// * Groups: `InternalFormat`, `SizedInternalFormat`
pub const GL_R8_SNORM: u32 = 0x8F94;
/// * Group: `EnableCap`
pub const GL_RASTERIZER_DISCARD: u32 = 0x8C89;
/// * Group: `GetPName`
pub const GL_READ_BUFFER: u32 = 0x0C02;
/// * Group: `FramebufferTarget`
pub const GL_READ_FRAMEBUFFER: u32 = 0x8CA8;
/// * Group: `GetPName`
pub const GL_READ_FRAMEBUFFER_BINDING: u32 = 0x8CAA;
/// * Group: `BufferAccessARB`
pub const GL_READ_ONLY: u32 = 0x88B8;
/// * Group: `BufferAccessARB`
pub const GL_READ_WRITE: u32 = 0x88BA;
/// * Groups: `FragmentShaderValueRepATI`, `TextureSwizzle`, `PixelFormat`,
///   `InternalFormat`
pub const GL_RED: u32 = 0x1903;
/// * Group: `PixelFormat`
pub const GL_RED_INTEGER: u32 = 0x8D94;
/// * Groups: `ObjectIdentifier`, `RenderbufferTarget`,
///   `CopyImageSubDataTarget`, `TextureTarget`
pub const GL_RENDERBUFFER: u32 = 0x8D41;
/// * Group: `RenderbufferParameterName`
pub const GL_RENDERBUFFER_ALPHA_SIZE: u32 = 0x8D53;
/// * Group: `GetPName`
pub const GL_RENDERBUFFER_BINDING: u32 = 0x8CA7;
/// * Group: `RenderbufferParameterName`
pub const GL_RENDERBUFFER_BLUE_SIZE: u32 = 0x8D52;
/// * Group: `RenderbufferParameterName`
pub const GL_RENDERBUFFER_DEPTH_SIZE: u32 = 0x8D54;
/// * Group: `RenderbufferParameterName`
pub const GL_RENDERBUFFER_GREEN_SIZE: u32 = 0x8D51;
/// * Group: `RenderbufferParameterName`
pub const GL_RENDERBUFFER_HEIGHT: u32 = 0x8D43;
/// * Group: `RenderbufferParameterName`
pub const GL_RENDERBUFFER_INTERNAL_FORMAT: u32 = 0x8D44;
/// * Group: `RenderbufferParameterName`
pub const GL_RENDERBUFFER_RED_SIZE: u32 = 0x8D50;
/// * Group: `RenderbufferParameterName`
pub const GL_RENDERBUFFER_SAMPLES: u32 = 0x8CAB;
/// * Group: `RenderbufferParameterName`
pub const GL_RENDERBUFFER_STENCIL_SIZE: u32 = 0x8D55;
/// * Group: `RenderbufferParameterName`
pub const GL_RENDERBUFFER_WIDTH: u32 = 0x8D42;
/// * Group: `StringName`
pub const GL_RENDERER: u32 = 0x1F01;
/// * Group: `TextureWrapMode`
pub const GL_REPEAT: u32 = 0x2901;
/// * Groups: `StencilOp`, `LightEnvModeSGIX`
pub const GL_REPLACE: u32 = 0x1E01;
/// * Groups: `InternalFormat`, `PixelFormat`
pub const GL_RG: u32 = 0x8227;
/// * Groups: `InternalFormat`, `SizedInternalFormat`
pub const GL_RG16: u32 = 0x822C;
/// * Groups: `InternalFormat`, `SizedInternalFormat`
pub const GL_RG16F: u32 = 0x822F;
/// * Groups: `InternalFormat`, `SizedInternalFormat`
pub const GL_RG16I: u32 = 0x8239;
/// * Groups: `InternalFormat`, `SizedInternalFormat`
pub const GL_RG16UI: u32 = 0x823A;
/// * Groups: `InternalFormat`, `SizedInternalFormat`
pub const GL_RG16_SNORM: u32 = 0x8F99;
/// * Groups: `InternalFormat`, `SizedInternalFormat`
pub const GL_RG32F: u32 = 0x8230;
/// * Groups: `InternalFormat`, `SizedInternalFormat`
pub const GL_RG32I: u32 = 0x823B;
/// * Groups: `InternalFormat`, `SizedInternalFormat`
pub const GL_RG32UI: u32 = 0x823C;
/// * Groups: `InternalFormat`, `SizedInternalFormat`
pub const GL_RG8: u32 = 0x822B;
/// * Groups: `InternalFormat`, `SizedInternalFormat`
pub const GL_RG8I: u32 = 0x8237;
/// * Groups: `InternalFormat`, `SizedInternalFormat`
pub const GL_RG8UI: u32 = 0x8238;
/// * Groups: `InternalFormat`, `SizedInternalFormat`
pub const GL_RG8_SNORM: u32 = 0x8F95;
/// * Groups: `PixelTexGenModeSGIX`, `CombinerPortionNV`, `PathColorFormat`,
///   `CombinerComponentUsageNV`, `PixelFormat`, `InternalFormat`
pub const GL_RGB: u32 = 0x1907;
/// * Groups: `InternalFormat`, `SizedInternalFormat`
pub const GL_RGB10: u32 = 0x8052;
/// * Groups: `InternalFormat`, `SizedInternalFormat`
pub const GL_RGB10_A2: u32 = 0x8059;
/// * Groups: `InternalFormat`, `SizedInternalFormat`
pub const GL_RGB10_A2UI: u32 = 0x906F;
/// * Groups: `InternalFormat`, `SizedInternalFormat`
pub const GL_RGB12: u32 = 0x8053;
/// * Groups: `InternalFormat`, `SizedInternalFormat`
pub const GL_RGB16: u32 = 0x8054;
/// * Groups: `InternalFormat`, `SizedInternalFormat`
pub const GL_RGB16F: u32 = 0x881B;
/// * Groups: `InternalFormat`, `SizedInternalFormat`
pub const GL_RGB16I: u32 = 0x8D89;
/// * Groups: `InternalFormat`, `SizedInternalFormat`
pub const GL_RGB16UI: u32 = 0x8D77;
/// * Groups: `InternalFormat`, `SizedInternalFormat`
pub const GL_RGB16_SNORM: u32 = 0x8F9A;
/// * Groups: `InternalFormat`, `SizedInternalFormat`
pub const GL_RGB32F: u32 = 0x8815;
/// * Groups: `InternalFormat`, `SizedInternalFormat`
pub const GL_RGB32I: u32 = 0x8D83;
/// * Groups: `InternalFormat`, `SizedInternalFormat`
pub const GL_RGB32UI: u32 = 0x8D71;
/// * Groups: `InternalFormat`, `SizedInternalFormat`
pub const GL_RGB4: u32 = 0x804F;
/// * Groups: `InternalFormat`, `SizedInternalFormat`
pub const GL_RGB5: u32 = 0x8050;
/// * Groups: `InternalFormat`, `SizedInternalFormat`
pub const GL_RGB5_A1: u32 = 0x8057;
/// * Groups: `InternalFormat`, `SizedInternalFormat`
pub const GL_RGB8: u32 = 0x8051;
/// * Groups: `InternalFormat`, `SizedInternalFormat`
pub const GL_RGB8I: u32 = 0x8D8F;
/// * Groups: `InternalFormat`, `SizedInternalFormat`
pub const GL_RGB8UI: u32 = 0x8D7D;
/// * Groups: `InternalFormat`, `SizedInternalFormat`
pub const GL_RGB8_SNORM: u32 = 0x8F96;
/// * Groups: `InternalFormat`, `SizedInternalFormat`
pub const GL_RGB9_E5: u32 = 0x8C3D;
/// * Groups: `PixelTexGenModeSGIX`, `PathColorFormat`, `PixelFormat`,
///   `InternalFormat`
pub const GL_RGBA: u32 = 0x1908;
/// * Groups: `InternalFormat`, `SizedInternalFormat`
pub const GL_RGBA12: u32 = 0x805A;
/// * Groups: `InternalFormat`, `SizedInternalFormat`
pub const GL_RGBA16: u32 = 0x805B;
/// * Groups: `InternalFormat`, `SizedInternalFormat`
pub const GL_RGBA16F: u32 = 0x881A;
/// * Groups: `InternalFormat`, `SizedInternalFormat`
pub const GL_RGBA16I: u32 = 0x8D88;
/// * Groups: `InternalFormat`, `SizedInternalFormat`
pub const GL_RGBA16UI: u32 = 0x8D76;
/// * Groups: `InternalFormat`, `SizedInternalFormat`
pub const GL_RGBA16_SNORM: u32 = 0x8F9B;
/// * Groups: `InternalFormat`, `SizedInternalFormat`
pub const GL_RGBA2: u32 = 0x8055;
/// * Groups: `InternalFormat`, `SizedInternalFormat`
pub const GL_RGBA32F: u32 = 0x8814;
/// * Groups: `InternalFormat`, `SizedInternalFormat`
pub const GL_RGBA32I: u32 = 0x8D82;
/// * Groups: `InternalFormat`, `SizedInternalFormat`
pub const GL_RGBA32UI: u32 = 0x8D70;
/// * Groups: `InternalFormat`, `SizedInternalFormat`
pub const GL_RGBA4: u32 = 0x8056;
/// * Groups: `InternalFormat`, `SizedInternalFormat`
pub const GL_RGBA8: u32 = 0x8058;
/// * Groups: `InternalFormat`, `SizedInternalFormat`
pub const GL_RGBA8I: u32 = 0x8D8E;
/// * Groups: `InternalFormat`, `SizedInternalFormat`
pub const GL_RGBA8UI: u32 = 0x8D7C;
/// * Groups: `InternalFormat`, `SizedInternalFormat`
pub const GL_RGBA8_SNORM: u32 = 0x8F97;
/// * Group: `PixelFormat`
pub const GL_RGBA_INTEGER: u32 = 0x8D99;
/// * Group: `PixelFormat`
pub const GL_RGB_INTEGER: u32 = 0x8D98;
/// * Group: `PixelFormat`
pub const GL_RG_INTEGER: u32 = 0x8228;
/// * Groups: `ColorBuffer`, `DrawBufferMode`, `ReadBufferMode`
pub const GL_RIGHT: u32 = 0x0407;
/// * Group: `ObjectIdentifier`
pub const GL_SAMPLER: u32 = 0x82E6;
/// * Groups: `AttributeType`, `UniformType`
pub const GL_SAMPLER_1D: u32 = 0x8B5D;
/// * Group: `UniformType`
pub const GL_SAMPLER_1D_ARRAY: u32 = 0x8DC0;
/// * Groups: `AttributeType`, `UniformType`
pub const GL_SAMPLER_1D_ARRAY_SHADOW: u32 = 0x8DC3;
/// * Groups: `AttributeType`, `UniformType`
pub const GL_SAMPLER_1D_SHADOW: u32 = 0x8B61;
/// * Groups: `AttributeType`, `UniformType`
pub const GL_SAMPLER_2D: u32 = 0x8B5E;
/// * Group: `UniformType`
pub const GL_SAMPLER_2D_ARRAY: u32 = 0x8DC1;
/// * Groups: `AttributeType`, `UniformType`
pub const GL_SAMPLER_2D_ARRAY_SHADOW: u32 = 0x8DC4;
/// * Groups: `AttributeType`, `UniformType`
pub const GL_SAMPLER_2D_MULTISAMPLE: u32 = 0x9108;
/// * Groups: `AttributeType`, `UniformType`
pub const GL_SAMPLER_2D_MULTISAMPLE_ARRAY: u32 = 0x910B;
/// * Groups: `AttributeType`, `UniformType`
pub const GL_SAMPLER_2D_RECT: u32 = 0x8B63;
/// * Groups: `AttributeType`, `UniformType`
pub const GL_SAMPLER_2D_RECT_SHADOW: u32 = 0x8B64;
/// * Groups: `AttributeType`, `UniformType`
pub const GL_SAMPLER_2D_SHADOW: u32 = 0x8B62;
/// * Groups: `AttributeType`, `UniformType`
pub const GL_SAMPLER_3D: u32 = 0x8B5F;
/// * Group: `GetPName`
pub const GL_SAMPLER_BINDING: u32 = 0x8919;
/// * Groups: `AttributeType`, `UniformType`
pub const GL_SAMPLER_BUFFER: u32 = 0x8DC2;
/// * Groups: `AttributeType`, `UniformType`
pub const GL_SAMPLER_CUBE: u32 = 0x8B60;
/// * Groups: `AttributeType`, `UniformType`
pub const GL_SAMPLER_CUBE_SHADOW: u32 = 0x8DC5;
/// * Groups: `GetFramebufferParameter`, `GetPName`, `InternalFormatPName`
pub const GL_SAMPLES: u32 = 0x80A9;
/// * Group: `QueryTarget`
pub const GL_SAMPLES_PASSED: u32 = 0x8914;
/// * Group: `EnableCap`
pub const GL_SAMPLE_ALPHA_TO_COVERAGE: u32 = 0x809E;
/// * Group: `EnableCap`
pub const GL_SAMPLE_ALPHA_TO_ONE: u32 = 0x809F;
/// * Groups: `GetFramebufferParameter`, `GetPName`
pub const GL_SAMPLE_BUFFERS: u32 = 0x80A8;
/// * Group: `EnableCap`
pub const GL_SAMPLE_COVERAGE: u32 = 0x80A0;
/// * Group: `GetPName`
pub const GL_SAMPLE_COVERAGE_INVERT: u32 = 0x80AB;
/// * Group: `GetPName`
pub const GL_SAMPLE_COVERAGE_VALUE: u32 = 0x80AA;
/// * Group: `EnableCap`
pub const GL_SAMPLE_MASK: u32 = 0x8E51;
pub const GL_SAMPLE_MASK_VALUE: u32 = 0x8E52;
/// * Group: `GetMultisamplePNameNV`
pub const GL_SAMPLE_POSITION: u32 = 0x8E50;
/// * Group: `GetPName`
pub const GL_SCISSOR_BOX: u32 = 0x0C10;
/// * Groups: `GetPName`, `EnableCap`
pub const GL_SCISSOR_TEST: u32 = 0x0C11;
/// * Group: `TransformFeedbackBufferMode`
pub const GL_SEPARATE_ATTRIBS: u32 = 0x8C8D;
/// * Group: `LogicOp`
pub const GL_SET: u32 = 0x150F;
/// * Group: `ObjectIdentifier`
pub const GL_SHADER: u32 = 0x82E1;
/// * Group: `ShaderParameterName`
pub const GL_SHADER_SOURCE_LENGTH: u32 = 0x8B88;
/// * Group: `ShaderParameterName`
pub const GL_SHADER_TYPE: u32 = 0x8B4F;
/// * Group: `StringName`
pub const GL_SHADING_LANGUAGE_VERSION: u32 = 0x8B8C;
/// * Groups: `VertexAttribIType`, `SecondaryColorPointerTypeIBM`,
///   `WeightPointerTypeARB`, `TangentPointerTypeEXT`, `BinormalPointerTypeEXT`,
///   `IndexPointerType`, `ListNameType`, `NormalPointerType`, `PixelType`,
///   `TexCoordPointerType`, `VertexPointerType`, `VertexAttribType`,
///   `VertexAttribPointerType`
pub const GL_SHORT: u32 = 0x1402;
pub const GL_SIGNALED: u32 = 0x9119;
pub const GL_SIGNED_NORMALIZED: u32 = 0x8F9C;
/// * Group: `GetPName`
pub const GL_SMOOTH_LINE_WIDTH_GRANULARITY: u32 = 0x0B23;
/// * Group: `GetPName`
pub const GL_SMOOTH_LINE_WIDTH_RANGE: u32 = 0x0B22;
/// * Group: `GetPName`
pub const GL_SMOOTH_POINT_SIZE_GRANULARITY: u32 = 0x0B13;
/// * Group: `GetPName`
pub const GL_SMOOTH_POINT_SIZE_RANGE: u32 = 0x0B12;
/// * Groups: `TextureEnvParameter`, `BlendingFactor`
pub const GL_SRC1_ALPHA: u32 = 0x8589;
/// * Group: `BlendingFactor`
pub const GL_SRC1_COLOR: u32 = 0x88F9;
/// * Group: `BlendingFactor`
pub const GL_SRC_ALPHA: u32 = 0x0302;
/// * Group: `BlendingFactor`
pub const GL_SRC_ALPHA_SATURATE: u32 = 0x0308;
/// * Group: `BlendingFactor`
pub const GL_SRC_COLOR: u32 = 0x0300;
/// * Group: `InternalFormat`
pub const GL_SRGB: u32 = 0x8C40;
/// * Groups: `InternalFormat`, `SizedInternalFormat`
pub const GL_SRGB8: u32 = 0x8C41;
/// * Groups: `InternalFormat`, `SizedInternalFormat`
pub const GL_SRGB8_ALPHA8: u32 = 0x8C43;
/// * Group: `InternalFormat`
pub const GL_SRGB_ALPHA: u32 = 0x8C42;
/// * Group: `ErrorCode`
pub const GL_STACK_OVERFLOW: u32 = 0x0503;
/// * Group: `ErrorCode`
pub const GL_STACK_UNDERFLOW: u32 = 0x0504;
/// * Groups: `VertexBufferObjectUsage`, `BufferUsageARB`
pub const GL_STATIC_COPY: u32 = 0x88E6;
/// * Groups: `VertexBufferObjectUsage`, `BufferUsageARB`
pub const GL_STATIC_DRAW: u32 = 0x88E4;
/// * Groups: `VertexBufferObjectUsage`, `BufferUsageARB`
pub const GL_STATIC_READ: u32 = 0x88E5;
/// * Groups: `Buffer`, `PixelCopyType`, `InvalidateFramebufferAttachment`
pub const GL_STENCIL: u32 = 0x1802;
/// * Group: `FramebufferAttachment`
pub const GL_STENCIL_ATTACHMENT: u32 = 0x8D20;
/// * Group: `GetPName`
pub const GL_STENCIL_BACK_FAIL: u32 = 0x8801;
/// * Group: `GetPName`
pub const GL_STENCIL_BACK_FUNC: u32 = 0x8800;
/// * Group: `GetPName`
pub const GL_STENCIL_BACK_PASS_DEPTH_FAIL: u32 = 0x8802;
/// * Group: `GetPName`
pub const GL_STENCIL_BACK_PASS_DEPTH_PASS: u32 = 0x8803;
/// * Group: `GetPName`
pub const GL_STENCIL_BACK_REF: u32 = 0x8CA3;
/// * Group: `GetPName`
pub const GL_STENCIL_BACK_VALUE_MASK: u32 = 0x8CA4;
/// * Group: `GetPName`
pub const GL_STENCIL_BACK_WRITEMASK: u32 = 0x8CA5;
/// * Groups: `ClearBufferMask`, `AttribMask`
pub const GL_STENCIL_BUFFER_BIT: u32 = 0x00000400;
/// * Group: `GetPName`
pub const GL_STENCIL_CLEAR_VALUE: u32 = 0x0B91;
/// * Group: `GetPName`
pub const GL_STENCIL_FAIL: u32 = 0x0B94;
/// * Group: `GetPName`
pub const GL_STENCIL_FUNC: u32 = 0x0B92;
/// * Groups: `InternalFormat`, `PixelFormat`, `DepthStencilTextureMode`
pub const GL_STENCIL_INDEX: u32 = 0x1901;
/// * Groups: `InternalFormat`, `SizedInternalFormat`
pub const GL_STENCIL_INDEX1: u32 = 0x8D46;
/// * Groups: `InternalFormat`, `SizedInternalFormat`
pub const GL_STENCIL_INDEX16: u32 = 0x8D49;
/// * Groups: `InternalFormat`, `SizedInternalFormat`
pub const GL_STENCIL_INDEX4: u32 = 0x8D47;
/// * Groups: `InternalFormat`, `SizedInternalFormat`
pub const GL_STENCIL_INDEX8: u32 = 0x8D48;
/// * Group: `GetPName`
pub const GL_STENCIL_PASS_DEPTH_FAIL: u32 = 0x0B95;
/// * Group: `GetPName`
pub const GL_STENCIL_PASS_DEPTH_PASS: u32 = 0x0B96;
/// * Group: `GetPName`
pub const GL_STENCIL_REF: u32 = 0x0B97;
/// * Groups: `GetPName`, `EnableCap`
pub const GL_STENCIL_TEST: u32 = 0x0B90;
/// * Group: `GetPName`
pub const GL_STENCIL_VALUE_MASK: u32 = 0x0B93;
/// * Group: `GetPName`
pub const GL_STENCIL_WRITEMASK: u32 = 0x0B98;
/// * Groups: `GetFramebufferParameter`, `GetPName`
pub const GL_STEREO: u32 = 0x0C33;
/// * Groups: `VertexBufferObjectUsage`, `BufferUsageARB`
pub const GL_STREAM_COPY: u32 = 0x88E2;
/// * Groups: `VertexBufferObjectUsage`, `BufferUsageARB`
pub const GL_STREAM_DRAW: u32 = 0x88E0;
/// * Groups: `VertexBufferObjectUsage`, `BufferUsageARB`
pub const GL_STREAM_READ: u32 = 0x88E1;
/// * Group: `GetPName`
pub const GL_SUBPIXEL_BITS: u32 = 0x0D50;
/// * Group: `SyncParameterName`
pub const GL_SYNC_CONDITION: u32 = 0x9113;
pub const GL_SYNC_FENCE: u32 = 0x9116;
/// * Group: `SyncParameterName`
pub const GL_SYNC_FLAGS: u32 = 0x9115;
/// * Group: `SyncObjectMask`
pub const GL_SYNC_FLUSH_COMMANDS_BIT: u32 = 0x00000001;
/// * Group: `SyncCondition`
pub const GL_SYNC_GPU_COMMANDS_COMPLETE: u32 = 0x9117;
/// * Group: `SyncParameterName`
pub const GL_SYNC_STATUS: u32 = 0x9114;
/// * Groups: `ObjectIdentifier`, `MatrixMode`
pub const GL_TEXTURE: u32 = 0x1702;
/// * Groups: `TextureUnit`, `FragmentShaderTextureSourceATI`
pub const GL_TEXTURE0: u32 = 0x84C0;
/// * Groups: `TextureUnit`, `FragmentShaderTextureSourceATI`
pub const GL_TEXTURE1: u32 = 0x84C1;
/// * Groups: `TextureUnit`, `FragmentShaderTextureSourceATI`
pub const GL_TEXTURE10: u32 = 0x84CA;
/// * Groups: `TextureUnit`, `FragmentShaderTextureSourceATI`
pub const GL_TEXTURE11: u32 = 0x84CB;
/// * Groups: `TextureUnit`, `FragmentShaderTextureSourceATI`
pub const GL_TEXTURE12: u32 = 0x84CC;
/// * Groups: `TextureUnit`, `FragmentShaderTextureSourceATI`
pub const GL_TEXTURE13: u32 = 0x84CD;
/// * Groups: `TextureUnit`, `FragmentShaderTextureSourceATI`
pub const GL_TEXTURE14: u32 = 0x84CE;
/// * Groups: `TextureUnit`, `FragmentShaderTextureSourceATI`
pub const GL_TEXTURE15: u32 = 0x84CF;
/// * Groups: `TextureUnit`, `FragmentShaderTextureSourceATI`
pub const GL_TEXTURE16: u32 = 0x84D0;
/// * Groups: `TextureUnit`, `FragmentShaderTextureSourceATI`
pub const GL_TEXTURE17: u32 = 0x84D1;
/// * Groups: `TextureUnit`, `FragmentShaderTextureSourceATI`
pub const GL_TEXTURE18: u32 = 0x84D2;
/// * Groups: `TextureUnit`, `FragmentShaderTextureSourceATI`
pub const GL_TEXTURE19: u32 = 0x84D3;
/// * Groups: `TextureUnit`, `FragmentShaderTextureSourceATI`
pub const GL_TEXTURE2: u32 = 0x84C2;
/// * Groups: `TextureUnit`, `FragmentShaderTextureSourceATI`
pub const GL_TEXTURE20: u32 = 0x84D4;
/// * Groups: `TextureUnit`, `FragmentShaderTextureSourceATI`
pub const GL_TEXTURE21: u32 = 0x84D5;
/// * Groups: `TextureUnit`, `FragmentShaderTextureSourceATI`
pub const GL_TEXTURE22: u32 = 0x84D6;
/// * Groups: `TextureUnit`, `FragmentShaderTextureSourceATI`
pub const GL_TEXTURE23: u32 = 0x84D7;
/// * Groups: `TextureUnit`, `FragmentShaderTextureSourceATI`
pub const GL_TEXTURE24: u32 = 0x84D8;
/// * Groups: `TextureUnit`, `FragmentShaderTextureSourceATI`
pub const GL_TEXTURE25: u32 = 0x84D9;
/// * Groups: `TextureUnit`, `FragmentShaderTextureSourceATI`
pub const GL_TEXTURE26: u32 = 0x84DA;
/// * Groups: `TextureUnit`, `FragmentShaderTextureSourceATI`
pub const GL_TEXTURE27: u32 = 0x84DB;
/// * Groups: `TextureUnit`, `FragmentShaderTextureSourceATI`
pub const GL_TEXTURE28: u32 = 0x84DC;
/// * Groups: `TextureUnit`, `FragmentShaderTextureSourceATI`
pub const GL_TEXTURE29: u32 = 0x84DD;
/// * Groups: `TextureUnit`, `FragmentShaderTextureSourceATI`
pub const GL_TEXTURE3: u32 = 0x84C3;
/// * Groups: `TextureUnit`, `FragmentShaderTextureSourceATI`
pub const GL_TEXTURE30: u32 = 0x84DE;
/// * Groups: `TextureUnit`, `FragmentShaderTextureSourceATI`
pub const GL_TEXTURE31: u32 = 0x84DF;
/// * Groups: `TextureUnit`, `FragmentShaderTextureSourceATI`
pub const GL_TEXTURE4: u32 = 0x84C4;
/// * Groups: `TextureUnit`, `FragmentShaderTextureSourceATI`
pub const GL_TEXTURE5: u32 = 0x84C5;
/// * Groups: `TextureUnit`, `FragmentShaderTextureSourceATI`
pub const GL_TEXTURE6: u32 = 0x84C6;
/// * Groups: `TextureUnit`, `FragmentShaderTextureSourceATI`
pub const GL_TEXTURE7: u32 = 0x84C7;
/// * Groups: `TextureUnit`, `FragmentShaderTextureSourceATI`
pub const GL_TEXTURE8: u32 = 0x84C8;
/// * Groups: `TextureUnit`, `FragmentShaderTextureSourceATI`
pub const GL_TEXTURE9: u32 = 0x84C9;
/// * Groups: `CopyImageSubDataTarget`, `EnableCap`, `GetPName`, `TextureTarget`
pub const GL_TEXTURE_1D: u32 = 0x0DE0;
/// * Groups: `CopyImageSubDataTarget`, `TextureTarget`
pub const GL_TEXTURE_1D_ARRAY: u32 = 0x8C18;
/// * Groups: `CopyImageSubDataTarget`, `EnableCap`, `GetPName`, `TextureTarget`
pub const GL_TEXTURE_2D: u32 = 0x0DE1;
/// * Groups: `CopyImageSubDataTarget`, `TextureTarget`
pub const GL_TEXTURE_2D_ARRAY: u32 = 0x8C1A;
/// * Groups: `CopyImageSubDataTarget`, `TextureTarget`
pub const GL_TEXTURE_2D_MULTISAMPLE: u32 = 0x9100;
/// * Groups: `CopyImageSubDataTarget`, `TextureTarget`
pub const GL_TEXTURE_2D_MULTISAMPLE_ARRAY: u32 = 0x9102;
/// * Groups: `CopyImageSubDataTarget`, `TextureTarget`
pub const GL_TEXTURE_3D: u32 = 0x806F;
/// * Groups: `TextureParameterName`, `GetTextureParameter`
pub const GL_TEXTURE_ALPHA_SIZE: u32 = 0x805F;
pub const GL_TEXTURE_ALPHA_TYPE: u32 = 0x8C13;
/// * Group: `TextureParameterName`
pub const GL_TEXTURE_BASE_LEVEL: u32 = 0x813C;
/// * Group: `GetPName`
pub const GL_TEXTURE_BINDING_1D: u32 = 0x8068;
/// * Group: `GetPName`
pub const GL_TEXTURE_BINDING_1D_ARRAY: u32 = 0x8C1C;
/// * Group: `GetPName`
pub const GL_TEXTURE_BINDING_2D: u32 = 0x8069;
/// * Group: `GetPName`
pub const GL_TEXTURE_BINDING_2D_ARRAY: u32 = 0x8C1D;
/// * Group: `GetPName`
pub const GL_TEXTURE_BINDING_2D_MULTISAMPLE: u32 = 0x9104;
/// * Group: `GetPName`
pub const GL_TEXTURE_BINDING_2D_MULTISAMPLE_ARRAY: u32 = 0x9105;
/// * Group: `GetPName`
pub const GL_TEXTURE_BINDING_3D: u32 = 0x806A;
/// * Group: `GetPName`
pub const GL_TEXTURE_BINDING_BUFFER: u32 = 0x8C2C;
/// * Group: `GetPName`
pub const GL_TEXTURE_BINDING_CUBE_MAP: u32 = 0x8514;
/// * Group: `GetPName`
pub const GL_TEXTURE_BINDING_RECTANGLE: u32 = 0x84F6;
/// * Groups: `TextureParameterName`, `GetTextureParameter`
pub const GL_TEXTURE_BLUE_SIZE: u32 = 0x805E;
pub const GL_TEXTURE_BLUE_TYPE: u32 = 0x8C12;
/// * Groups: `SamplerParameterF`, `GetTextureParameter`, `TextureParameterName`
pub const GL_TEXTURE_BORDER_COLOR: u32 = 0x1004;
/// * Groups: `TextureTarget`, `CopyBufferSubDataTarget`, `BufferTargetARB`,
///   `BufferStorageTarget`
pub const GL_TEXTURE_BUFFER: u32 = 0x8C2A;
pub const GL_TEXTURE_BUFFER_DATA_STORE_BINDING: u32 = 0x8C2D;
/// * Groups: `SamplerParameterI`, `TextureParameterName`
pub const GL_TEXTURE_COMPARE_FUNC: u32 = 0x884D;
/// * Groups: `SamplerParameterI`, `TextureParameterName`
pub const GL_TEXTURE_COMPARE_MODE: u32 = 0x884C;
/// * Group: `InternalFormatPName`
pub const GL_TEXTURE_COMPRESSED: u32 = 0x86A1;
pub const GL_TEXTURE_COMPRESSED_IMAGE_SIZE: u32 = 0x86A0;
/// * Groups: `HintTarget`, `GetPName`
pub const GL_TEXTURE_COMPRESSION_HINT: u32 = 0x84EF;
/// * Groups: `CopyImageSubDataTarget`, `TextureTarget`, `EnableCap`
pub const GL_TEXTURE_CUBE_MAP: u32 = 0x8513;
/// * Group: `TextureTarget`
pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_X: u32 = 0x8516;
/// * Group: `TextureTarget`
pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_Y: u32 = 0x8518;
/// * Group: `TextureTarget`
pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_Z: u32 = 0x851A;
/// * Group: `TextureTarget`
pub const GL_TEXTURE_CUBE_MAP_POSITIVE_X: u32 = 0x8515;
/// * Group: `TextureTarget`
pub const GL_TEXTURE_CUBE_MAP_POSITIVE_Y: u32 = 0x8517;
/// * Group: `TextureTarget`
pub const GL_TEXTURE_CUBE_MAP_POSITIVE_Z: u32 = 0x8519;
/// * Group: `EnableCap`
pub const GL_TEXTURE_CUBE_MAP_SEAMLESS: u32 = 0x884F;
pub const GL_TEXTURE_DEPTH: u32 = 0x8071;
pub const GL_TEXTURE_DEPTH_SIZE: u32 = 0x884A;
pub const GL_TEXTURE_DEPTH_TYPE: u32 = 0x8C16;
pub const GL_TEXTURE_FIXED_SAMPLE_LOCATIONS: u32 = 0x9107;
/// * Groups: `TextureParameterName`, `GetTextureParameter`
pub const GL_TEXTURE_GREEN_SIZE: u32 = 0x805D;
pub const GL_TEXTURE_GREEN_TYPE: u32 = 0x8C11;
/// * Groups: `TextureParameterName`, `GetTextureParameter`
pub const GL_TEXTURE_HEIGHT: u32 = 0x1001;
/// * Groups: `TextureParameterName`, `GetTextureParameter`
pub const GL_TEXTURE_INTERNAL_FORMAT: u32 = 0x1003;
/// * Groups: `TextureParameterName`, `SamplerParameterF`
pub const GL_TEXTURE_LOD_BIAS: u32 = 0x8501;
/// * Groups: `SamplerParameterI`, `GetTextureParameter`, `TextureParameterName`
pub const GL_TEXTURE_MAG_FILTER: u32 = 0x2800;
/// * Group: `TextureParameterName`
pub const GL_TEXTURE_MAX_LEVEL: u32 = 0x813D;
/// * Groups: `SamplerParameterF`, `TextureParameterName`
pub const GL_TEXTURE_MAX_LOD: u32 = 0x813B;
/// * Groups: `SamplerParameterI`, `GetTextureParameter`, `TextureParameterName`
pub const GL_TEXTURE_MIN_FILTER: u32 = 0x2801;
/// * Groups: `SamplerParameterF`, `TextureParameterName`
pub const GL_TEXTURE_MIN_LOD: u32 = 0x813A;
/// * Groups: `CopyImageSubDataTarget`, `TextureTarget`, `EnableCap`
pub const GL_TEXTURE_RECTANGLE: u32 = 0x84F5;
/// * Groups: `TextureParameterName`, `GetTextureParameter`
pub const GL_TEXTURE_RED_SIZE: u32 = 0x805C;
pub const GL_TEXTURE_RED_TYPE: u32 = 0x8C10;
pub const GL_TEXTURE_SAMPLES: u32 = 0x9106;
pub const GL_TEXTURE_SHARED_SIZE: u32 = 0x8C3F;
pub const GL_TEXTURE_STENCIL_SIZE: u32 = 0x88F1;
/// * Group: `TextureParameterName`
pub const GL_TEXTURE_SWIZZLE_A: u32 = 0x8E45;
/// * Group: `TextureParameterName`
pub const GL_TEXTURE_SWIZZLE_B: u32 = 0x8E44;
/// * Group: `TextureParameterName`
pub const GL_TEXTURE_SWIZZLE_G: u32 = 0x8E43;
/// * Group: `TextureParameterName`
pub const GL_TEXTURE_SWIZZLE_R: u32 = 0x8E42;
/// * Group: `TextureParameterName`
pub const GL_TEXTURE_SWIZZLE_RGBA: u32 = 0x8E46;
/// * Groups: `TextureParameterName`, `GetTextureParameter`
pub const GL_TEXTURE_WIDTH: u32 = 0x1000;
/// * Groups: `SamplerParameterI`, `TextureParameterName`
pub const GL_TEXTURE_WRAP_R: u32 = 0x8072;
/// * Groups: `SamplerParameterI`, `GetTextureParameter`, `TextureParameterName`
pub const GL_TEXTURE_WRAP_S: u32 = 0x2802;
/// * Groups: `SamplerParameterI`, `GetTextureParameter`, `TextureParameterName`
pub const GL_TEXTURE_WRAP_T: u32 = 0x2803;
/// * Group: `SyncStatus`
pub const GL_TIMEOUT_EXPIRED: u32 = 0x911B;
/// * Group: `SpecialNumbers`
pub const GL_TIMEOUT_IGNORED: u64 = 0xFFFFFFFFFFFFFFFF;
/// * Groups: `QueryCounterTarget`, `GetPName`
pub const GL_TIMESTAMP: u32 = 0x8E28;
/// * Group: `QueryTarget`
pub const GL_TIME_ELAPSED: u32 = 0x88BF;
/// * Groups: `ProgramInterface`, `BufferTargetARB`, `BufferStorageTarget`,
///   `CopyBufferSubDataTarget`
pub const GL_TRANSFORM_FEEDBACK_BUFFER: u32 = 0x8C8E;
/// * Groups: `TransformFeedbackPName`, `GetPName`
pub const GL_TRANSFORM_FEEDBACK_BUFFER_BINDING: u32 = 0x8C8F;
/// * Group: `ProgramPropertyARB`
pub const GL_TRANSFORM_FEEDBACK_BUFFER_MODE: u32 = 0x8C7F;
/// * Groups: `TransformFeedbackPName`, `GetPName`
pub const GL_TRANSFORM_FEEDBACK_BUFFER_SIZE: u32 = 0x8C85;
/// * Groups: `TransformFeedbackPName`, `GetPName`
pub const GL_TRANSFORM_FEEDBACK_BUFFER_START: u32 = 0x8C84;
/// * Group: `QueryTarget`
pub const GL_TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN: u32 = 0x8C88;
/// * Group: `ProgramPropertyARB`
pub const GL_TRANSFORM_FEEDBACK_VARYINGS: u32 = 0x8C83;
/// * Group: `ProgramPropertyARB`
pub const GL_TRANSFORM_FEEDBACK_VARYING_MAX_LENGTH: u32 = 0x8C76;
/// * Group: `PrimitiveType`
pub const GL_TRIANGLES: u32 = 0x0004;
/// * Group: `PrimitiveType`
pub const GL_TRIANGLES_ADJACENCY: u32 = 0x000C;
/// * Group: `PrimitiveType`
pub const GL_TRIANGLE_FAN: u32 = 0x0006;
/// * Group: `PrimitiveType`
pub const GL_TRIANGLE_STRIP: u32 = 0x0005;
/// * Group: `PrimitiveType`
pub const GL_TRIANGLE_STRIP_ADJACENCY: u32 = 0x000D;
/// * Groups: `SpecialNumbers`, `Boolean`, `VertexShaderWriteMaskEXT`,
///   `ClampColorModeARB`
pub const GL_TRUE: u32 = 1;
/// * Group: `UniformPName`
pub const GL_UNIFORM_ARRAY_STRIDE: u32 = 0x8A3C;
/// * Group: `UniformBlockPName`
pub const GL_UNIFORM_BLOCK_ACTIVE_UNIFORMS: u32 = 0x8A42;
/// * Group: `UniformBlockPName`
pub const GL_UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES: u32 = 0x8A43;
/// * Group: `UniformBlockPName`
pub const GL_UNIFORM_BLOCK_BINDING: u32 = 0x8A3F;
/// * Group: `UniformBlockPName`
pub const GL_UNIFORM_BLOCK_DATA_SIZE: u32 = 0x8A40;
/// * Group: `UniformPName`
pub const GL_UNIFORM_BLOCK_INDEX: u32 = 0x8A3A;
/// * Group: `UniformBlockPName`
pub const GL_UNIFORM_BLOCK_NAME_LENGTH: u32 = 0x8A41;
/// * Group: `UniformBlockPName`
pub const GL_UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER: u32 = 0x8A46;
/// * Group: `UniformBlockPName`
pub const GL_UNIFORM_BLOCK_REFERENCED_BY_GEOMETRY_SHADER: u32 = 0x8A45;
/// * Group: `UniformBlockPName`
pub const GL_UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER: u32 = 0x8A44;
/// * Groups: `CopyBufferSubDataTarget`, `BufferTargetARB`,
///   `BufferStorageTarget`
pub const GL_UNIFORM_BUFFER: u32 = 0x8A11;
/// * Group: `GetPName`
pub const GL_UNIFORM_BUFFER_BINDING: u32 = 0x8A28;
/// * Group: `GetPName`
pub const GL_UNIFORM_BUFFER_OFFSET_ALIGNMENT: u32 = 0x8A34;
/// * Group: `GetPName`
pub const GL_UNIFORM_BUFFER_SIZE: u32 = 0x8A2A;
/// * Group: `GetPName`
pub const GL_UNIFORM_BUFFER_START: u32 = 0x8A29;
/// * Group: `UniformPName`
pub const GL_UNIFORM_IS_ROW_MAJOR: u32 = 0x8A3E;
/// * Group: `UniformPName`
pub const GL_UNIFORM_MATRIX_STRIDE: u32 = 0x8A3D;
/// * Groups: `SubroutineParameterName`, `UniformPName`
pub const GL_UNIFORM_NAME_LENGTH: u32 = 0x8A39;
/// * Group: `UniformPName`
pub const GL_UNIFORM_OFFSET: u32 = 0x8A3B;
/// * Groups: `SubroutineParameterName`, `UniformPName`
pub const GL_UNIFORM_SIZE: u32 = 0x8A38;
/// * Group: `UniformPName`
pub const GL_UNIFORM_TYPE: u32 = 0x8A37;
/// * Groups: `PixelStoreParameter`, `GetPName`
pub const GL_UNPACK_ALIGNMENT: u32 = 0x0CF5;
/// * Groups: `PixelStoreParameter`, `GetPName`
pub const GL_UNPACK_IMAGE_HEIGHT: u32 = 0x806E;
/// * Groups: `PixelStoreParameter`, `GetPName`
pub const GL_UNPACK_LSB_FIRST: u32 = 0x0CF1;
/// * Groups: `PixelStoreParameter`, `GetPName`
pub const GL_UNPACK_ROW_LENGTH: u32 = 0x0CF2;
/// * Groups: `PixelStoreParameter`, `GetPName`
pub const GL_UNPACK_SKIP_IMAGES: u32 = 0x806D;
/// * Groups: `PixelStoreParameter`, `GetPName`
pub const GL_UNPACK_SKIP_PIXELS: u32 = 0x0CF4;
/// * Groups: `PixelStoreParameter`, `GetPName`
pub const GL_UNPACK_SKIP_ROWS: u32 = 0x0CF3;
/// * Groups: `PixelStoreParameter`, `GetPName`
pub const GL_UNPACK_SWAP_BYTES: u32 = 0x0CF0;
pub const GL_UNSIGNALED: u32 = 0x9118;
/// * Groups: `VertexAttribIType`, `ScalarType`, `ReplacementCodeTypeSUN`,
///   `ElementPointerTypeATI`, `MatrixIndexPointerTypeARB`,
///   `WeightPointerTypeARB`, `ColorPointerType`, `DrawElementsType`,
///   `ListNameType`, `PixelType`, `VertexAttribType`, `VertexAttribPointerType`
pub const GL_UNSIGNED_BYTE: u32 = 0x1401;
/// * Group: `PixelType`
pub const GL_UNSIGNED_BYTE_2_3_3_REV: u32 = 0x8362;
/// * Group: `PixelType`
pub const GL_UNSIGNED_BYTE_3_3_2: u32 = 0x8032;
/// * Groups: `VertexAttribIType`, `ScalarType`, `ReplacementCodeTypeSUN`,
///   `ElementPointerTypeATI`, `MatrixIndexPointerTypeARB`,
///   `WeightPointerTypeARB`, `ColorPointerType`, `DrawElementsType`,
///   `ListNameType`, `PixelFormat`, `PixelType`, `VertexAttribType`,
///   `AttributeType`, `UniformType`, `VertexAttribPointerType`
pub const GL_UNSIGNED_INT: u32 = 0x1405;
/// * Groups: `PixelType`, `VertexAttribPointerType`, `VertexAttribType`
pub const GL_UNSIGNED_INT_10F_11F_11F_REV: u32 = 0x8C3B;
/// * Group: `PixelType`
pub const GL_UNSIGNED_INT_10_10_10_2: u32 = 0x8036;
/// * Group: `PixelType`
pub const GL_UNSIGNED_INT_24_8: u32 = 0x84FA;
/// * Groups: `PixelType`, `VertexAttribPointerType`, `VertexAttribType`
pub const GL_UNSIGNED_INT_2_10_10_10_REV: u32 = 0x8368;
/// * Group: `PixelType`
pub const GL_UNSIGNED_INT_5_9_9_9_REV: u32 = 0x8C3E;
/// * Group: `PixelType`
pub const GL_UNSIGNED_INT_8_8_8_8: u32 = 0x8035;
/// * Group: `PixelType`
pub const GL_UNSIGNED_INT_8_8_8_8_REV: u32 = 0x8367;
/// * Groups: `AttributeType`, `UniformType`
pub const GL_UNSIGNED_INT_SAMPLER_1D: u32 = 0x8DD1;
/// * Groups: `AttributeType`, `UniformType`
pub const GL_UNSIGNED_INT_SAMPLER_1D_ARRAY: u32 = 0x8DD6;
/// * Groups: `AttributeType`, `UniformType`
pub const GL_UNSIGNED_INT_SAMPLER_2D: u32 = 0x8DD2;
/// * Groups: `AttributeType`, `UniformType`
pub const GL_UNSIGNED_INT_SAMPLER_2D_ARRAY: u32 = 0x8DD7;
/// * Groups: `AttributeType`, `UniformType`
pub const GL_UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE: u32 = 0x910A;
/// * Groups: `AttributeType`, `UniformType`
pub const GL_UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE_ARRAY: u32 = 0x910D;
/// * Groups: `AttributeType`, `UniformType`
pub const GL_UNSIGNED_INT_SAMPLER_2D_RECT: u32 = 0x8DD5;
/// * Groups: `AttributeType`, `UniformType`
pub const GL_UNSIGNED_INT_SAMPLER_3D: u32 = 0x8DD3;
/// * Groups: `AttributeType`, `UniformType`
pub const GL_UNSIGNED_INT_SAMPLER_BUFFER: u32 = 0x8DD8;
/// * Groups: `AttributeType`, `UniformType`
pub const GL_UNSIGNED_INT_SAMPLER_CUBE: u32 = 0x8DD4;
/// * Groups: `AttributeType`, `UniformType`
pub const GL_UNSIGNED_INT_VEC2: u32 = 0x8DC6;
/// * Groups: `AttributeType`, `UniformType`
pub const GL_UNSIGNED_INT_VEC3: u32 = 0x8DC7;
/// * Groups: `AttributeType`, `UniformType`
pub const GL_UNSIGNED_INT_VEC4: u32 = 0x8DC8;
pub const GL_UNSIGNED_NORMALIZED: u32 = 0x8C17;
/// * Groups: `VertexAttribIType`, `ScalarType`, `ReplacementCodeTypeSUN`,
///   `ElementPointerTypeATI`, `MatrixIndexPointerTypeARB`,
///   `WeightPointerTypeARB`, `ColorPointerType`, `DrawElementsType`,
///   `ListNameType`, `PixelFormat`, `PixelType`, `VertexAttribType`,
///   `VertexAttribPointerType`
pub const GL_UNSIGNED_SHORT: u32 = 0x1403;
/// * Group: `PixelType`
pub const GL_UNSIGNED_SHORT_1_5_5_5_REV: u32 = 0x8366;
/// * Group: `PixelType`
pub const GL_UNSIGNED_SHORT_4_4_4_4: u32 = 0x8033;
/// * Group: `PixelType`
pub const GL_UNSIGNED_SHORT_4_4_4_4_REV: u32 = 0x8365;
/// * Group: `PixelType`
pub const GL_UNSIGNED_SHORT_5_5_5_1: u32 = 0x8034;
/// * Group: `PixelType`
pub const GL_UNSIGNED_SHORT_5_6_5: u32 = 0x8363;
/// * Group: `PixelType`
pub const GL_UNSIGNED_SHORT_5_6_5_REV: u32 = 0x8364;
/// * Group: `ClipControlOrigin`
pub const GL_UPPER_LEFT: u32 = 0x8CA2;
/// * Group: `ProgramPropertyARB`
pub const GL_VALIDATE_STATUS: u32 = 0x8B83;
/// * Group: `StringName`
pub const GL_VENDOR: u32 = 0x1F00;
/// * Group: `StringName`
pub const GL_VERSION: u32 = 0x1F02;
/// * Groups: `ObjectIdentifier`, `EnableCap`, `GetPName`
pub const GL_VERTEX_ARRAY: u32 = 0x8074;
/// * Group: `GetPName`
pub const GL_VERTEX_ARRAY_BINDING: u32 = 0x85B5;
/// * Groups: `VertexAttribEnum`, `VertexAttribPropertyARB`
pub const GL_VERTEX_ATTRIB_ARRAY_BUFFER_BINDING: u32 = 0x889F;
/// * Groups: `VertexAttribEnum`, `VertexAttribPropertyARB`, `VertexArrayPName`
pub const GL_VERTEX_ATTRIB_ARRAY_DIVISOR: u32 = 0x88FE;
/// * Groups: `VertexAttribEnum`, `VertexAttribPropertyARB`, `VertexArrayPName`
pub const GL_VERTEX_ATTRIB_ARRAY_ENABLED: u32 = 0x8622;
/// * Groups: `VertexAttribEnum`, `VertexAttribPropertyARB`, `VertexArrayPName`
pub const GL_VERTEX_ATTRIB_ARRAY_INTEGER: u32 = 0x88FD;
/// * Groups: `VertexAttribEnum`, `VertexAttribPropertyARB`, `VertexArrayPName`
pub const GL_VERTEX_ATTRIB_ARRAY_NORMALIZED: u32 = 0x886A;
/// * Group: `VertexAttribPointerPropertyARB`
pub const GL_VERTEX_ATTRIB_ARRAY_POINTER: u32 = 0x8645;
/// * Groups: `VertexAttribEnum`, `VertexAttribPropertyARB`, `VertexArrayPName`
pub const GL_VERTEX_ATTRIB_ARRAY_SIZE: u32 = 0x8623;
/// * Groups: `VertexAttribEnum`, `VertexAttribPropertyARB`, `VertexArrayPName`
pub const GL_VERTEX_ATTRIB_ARRAY_STRIDE: u32 = 0x8624;
/// * Groups: `VertexAttribEnum`, `VertexAttribPropertyARB`, `VertexArrayPName`
pub const GL_VERTEX_ATTRIB_ARRAY_TYPE: u32 = 0x8625;
pub const GL_VERTEX_PROGRAM_POINT_SIZE: u32 = 0x8642;
/// * Groups: `PipelineParameterName`, `ShaderType`
pub const GL_VERTEX_SHADER: u32 = 0x8B31;
/// * Group: `GetPName`
pub const GL_VIEWPORT: u32 = 0x0BA2;
/// * Group: `SyncStatus`
pub const GL_WAIT_FAILED: u32 = 0x911D;
/// * Group: `BufferAccessARB`
pub const GL_WRITE_ONLY: u32 = 0x88B9;
/// * Group: `LogicOp`
pub const GL_XOR: u32 = 0x1506;
/// * Groups: `SpecialNumbers`, `TextureSwizzle`, `StencilOp`, `BlendingFactor`,
///   `FragmentShaderGenericSourceATI`
pub const GL_ZERO: u32 = 0;

static glActiveTexture_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glActiveTexture(texture: GLenum) -> () {
  let u: usize = glActiveTexture_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLenum) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(texture)
}

static glAttachShader_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glAttachShader(program: GLuint, shader: GLuint) -> () {
  let u: usize = glAttachShader_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint, GLuint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(program, shader)
}

static glBeginConditionalRender_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glBeginConditionalRender(id: GLuint, mode: GLenum) -> () {
  let u: usize =
    glBeginConditionalRender_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint, GLenum) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(id, mode)
}

static glBeginQuery_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glBeginQuery(target: GLenum, id: GLuint) -> () {
  let u: usize = glBeginQuery_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLenum, GLuint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(target, id)
}

static glBeginTransformFeedback_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glBeginTransformFeedback(primitiveMode: GLenum) -> () {
  let u: usize =
    glBeginTransformFeedback_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLenum) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(primitiveMode)
}

static glBindAttribLocation_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glBindAttribLocation(
  program: GLuint, index: GLuint, name: *const GLchar,
) -> () {
  let u: usize =
    glBindAttribLocation_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint, GLuint, *const GLchar) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(program, index, name)
}

static glBindBuffer_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glBindBuffer(target: GLenum, buffer: GLuint) -> () {
  let u: usize = glBindBuffer_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLenum, GLuint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(target, buffer)
}

static glBindBufferBase_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glBindBufferBase(
  target: GLenum, index: GLuint, buffer: GLuint,
) -> () {
  let u: usize = glBindBufferBase_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLenum, GLuint, GLuint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(target, index, buffer)
}

static glBindBufferRange_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glBindBufferRange(
  target: GLenum, index: GLuint, buffer: GLuint, offset: GLintptr,
  size: GLsizeiptr,
) -> () {
  let u: usize =
    glBindBufferRange_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLenum,
    GLuint,
    GLuint,
    GLintptr,
    GLsizeiptr,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(target, index, buffer, offset, size)
}

static glBindFragDataLocation_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glBindFragDataLocation(
  program: GLuint, color: GLuint, name: *const GLchar,
) -> () {
  let u: usize =
    glBindFragDataLocation_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint, GLuint, *const GLchar) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(program, color, name)
}

static glBindFragDataLocationIndexed_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glBindFragDataLocationIndexed(
  program: GLuint, colorNumber: GLuint, index: GLuint, name: *const GLchar,
) -> () {
  let u: usize =
    glBindFragDataLocationIndexed_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLuint,
    GLuint,
    GLuint,
    *const GLchar,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(program, colorNumber, index, name)
}

static glBindFramebuffer_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glBindFramebuffer(target: GLenum, framebuffer: GLuint) -> () {
  let u: usize =
    glBindFramebuffer_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLenum, GLuint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(target, framebuffer)
}

static glBindRenderbuffer_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glBindRenderbuffer(target: GLenum, renderbuffer: GLuint) -> () {
  let u: usize =
    glBindRenderbuffer_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLenum, GLuint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(target, renderbuffer)
}

static glBindSampler_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glBindSampler(unit: GLuint, sampler: GLuint) -> () {
  let u: usize = glBindSampler_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint, GLuint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(unit, sampler)
}

static glBindTexture_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glBindTexture(target: GLenum, texture: GLuint) -> () {
  let u: usize = glBindTexture_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLenum, GLuint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(target, texture)
}

static glBindVertexArray_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glBindVertexArray(array: GLuint) -> () {
  let u: usize =
    glBindVertexArray_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(array)
}

static glBlendColor_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glBlendColor(
  red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat,
) -> () {
  let u: usize = glBlendColor_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLfloat,
    GLfloat,
    GLfloat,
    GLfloat,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(red, green, blue, alpha)
}

static glBlendEquation_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glBlendEquation(mode: GLenum) -> () {
  let u: usize = glBlendEquation_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLenum) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(mode)
}

static glBlendEquationSeparate_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glBlendEquationSeparate(
  modeRGB: GLenum, modeAlpha: GLenum,
) -> () {
  let u: usize =
    glBlendEquationSeparate_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLenum, GLenum) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(modeRGB, modeAlpha)
}

static glBlendFunc_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glBlendFunc(sfactor: GLenum, dfactor: GLenum) -> () {
  let u: usize = glBlendFunc_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLenum, GLenum) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(sfactor, dfactor)
}

static glBlendFuncSeparate_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glBlendFuncSeparate(
  sfactorRGB: GLenum, dfactorRGB: GLenum, sfactorAlpha: GLenum,
  dfactorAlpha: GLenum,
) -> () {
  let u: usize =
    glBlendFuncSeparate_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLenum, GLenum, GLenum, GLenum) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(sfactorRGB, dfactorRGB, sfactorAlpha, dfactorAlpha)
}

static glBlitFramebuffer_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glBlitFramebuffer(
  srcX0: GLint, srcY0: GLint, srcX1: GLint, srcY1: GLint, dstX0: GLint,
  dstY0: GLint, dstX1: GLint, dstY1: GLint, mask: GLbitfield, filter: GLenum,
) -> () {
  let u: usize =
    glBlitFramebuffer_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLint,
    GLint,
    GLint,
    GLint,
    GLint,
    GLint,
    GLint,
    GLint,
    GLbitfield,
    GLenum,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(srcX0, srcY0, srcX1, srcY1, dstX0, dstY0, dstX1, dstY1, mask, filter)
}

static glBufferData_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glBufferData(
  target: GLenum, size: GLsizeiptr, data: *const void, usage: GLenum,
) -> () {
  let u: usize = glBufferData_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLenum,
    GLsizeiptr,
    *const void,
    GLenum,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(target, size, data, usage)
}

static glBufferSubData_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glBufferSubData(
  target: GLenum, offset: GLintptr, size: GLsizeiptr, data: *const void,
) -> () {
  let u: usize = glBufferSubData_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLenum,
    GLintptr,
    GLsizeiptr,
    *const void,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(target, offset, size, data)
}

static glCheckFramebufferStatus_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glCheckFramebufferStatus(target: GLenum) -> GLenum {
  let u: usize =
    glCheckFramebufferStatus_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLenum) -> GLenum =
    unsafe { core::mem::transmute(u) };
  _func_p(target)
}

static glClampColor_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glClampColor(target: GLenum, clamp: GLenum) -> () {
  let u: usize = glClampColor_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLenum, GLenum) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(target, clamp)
}

static glClear_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glClear(mask: GLbitfield) -> () {
  let u: usize = glClear_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLbitfield) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(mask)
}

static glClearBufferfi_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glClearBufferfi(
  buffer: GLenum, drawbuffer: GLint, depth: GLfloat, stencil: GLint,
) -> () {
  let u: usize = glClearBufferfi_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLenum, GLint, GLfloat, GLint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(buffer, drawbuffer, depth, stencil)
}

static glClearBufferfv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glClearBufferfv(
  buffer: GLenum, drawbuffer: GLint, value: *const GLfloat,
) -> () {
  let u: usize = glClearBufferfv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLenum, GLint, *const GLfloat) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(buffer, drawbuffer, value)
}

static glClearBufferiv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glClearBufferiv(
  buffer: GLenum, drawbuffer: GLint, value: *const GLint,
) -> () {
  let u: usize = glClearBufferiv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLenum, GLint, *const GLint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(buffer, drawbuffer, value)
}

static glClearBufferuiv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glClearBufferuiv(
  buffer: GLenum, drawbuffer: GLint, value: *const GLuint,
) -> () {
  let u: usize = glClearBufferuiv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLenum, GLint, *const GLuint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(buffer, drawbuffer, value)
}

static glClearColor_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glClearColor(
  red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat,
) -> () {
  let u: usize = glClearColor_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLfloat,
    GLfloat,
    GLfloat,
    GLfloat,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(red, green, blue, alpha)
}

static glClearDepth_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glClearDepth(depth: GLdouble) -> () {
  let u: usize = glClearDepth_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLdouble) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(depth)
}

static glClearStencil_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glClearStencil(s: GLint) -> () {
  let u: usize = glClearStencil_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(s)
}

static glClientWaitSync_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glClientWaitSync(
  sync: GLsync, flags: GLbitfield, timeout: GLuint64,
) -> GLenum {
  let u: usize = glClientWaitSync_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLsync,
    GLbitfield,
    GLuint64,
  ) -> GLenum = unsafe { core::mem::transmute(u) };
  _func_p(sync, flags, timeout)
}

static glColorMask_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glColorMask(
  red: GLboolean, green: GLboolean, blue: GLboolean, alpha: GLboolean,
) -> () {
  let u: usize = glColorMask_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLboolean,
    GLboolean,
    GLboolean,
    GLboolean,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(red, green, blue, alpha)
}

static glColorMaski_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glColorMaski(
  index: GLuint, r: GLboolean, g: GLboolean, b: GLboolean, a: GLboolean,
) -> () {
  let u: usize = glColorMaski_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLuint,
    GLboolean,
    GLboolean,
    GLboolean,
    GLboolean,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(index, r, g, b, a)
}

static glCompileShader_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glCompileShader(shader: GLuint) -> () {
  let u: usize = glCompileShader_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(shader)
}

static glCompressedTexImage1D_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glCompressedTexImage1D(
  target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei,
  border: GLint, imageSize: GLsizei, data: *const void,
) -> () {
  let u: usize =
    glCompressedTexImage1D_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLenum,
    GLint,
    GLenum,
    GLsizei,
    GLint,
    GLsizei,
    *const void,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(target, level, internalformat, width, border, imageSize, data)
}

static glCompressedTexImage2D_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glCompressedTexImage2D(
  target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei,
  height: GLsizei, border: GLint, imageSize: GLsizei, data: *const void,
) -> () {
  let u: usize =
    glCompressedTexImage2D_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLenum,
    GLint,
    GLenum,
    GLsizei,
    GLsizei,
    GLint,
    GLsizei,
    *const void,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(target, level, internalformat, width, height, border, imageSize, data)
}

static glCompressedTexImage3D_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glCompressedTexImage3D(
  target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei,
  height: GLsizei, depth: GLsizei, border: GLint, imageSize: GLsizei,
  data: *const void,
) -> () {
  let u: usize =
    glCompressedTexImage3D_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLenum,
    GLint,
    GLenum,
    GLsizei,
    GLsizei,
    GLsizei,
    GLint,
    GLsizei,
    *const void,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(
    target,
    level,
    internalformat,
    width,
    height,
    depth,
    border,
    imageSize,
    data,
  )
}

static glCompressedTexSubImage1D_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glCompressedTexSubImage1D(
  target: GLenum, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum,
  imageSize: GLsizei, data: *const void,
) -> () {
  let u: usize =
    glCompressedTexSubImage1D_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLenum,
    GLint,
    GLint,
    GLsizei,
    GLenum,
    GLsizei,
    *const void,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(target, level, xoffset, width, format, imageSize, data)
}

static glCompressedTexSubImage2D_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glCompressedTexSubImage2D(
  target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei,
  height: GLsizei, format: GLenum, imageSize: GLsizei, data: *const void,
) -> () {
  let u: usize =
    glCompressedTexSubImage2D_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLenum,
    GLint,
    GLint,
    GLint,
    GLsizei,
    GLsizei,
    GLenum,
    GLsizei,
    *const void,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(
    target, level, xoffset, yoffset, width, height, format, imageSize, data,
  )
}

static glCompressedTexSubImage3D_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glCompressedTexSubImage3D(
  target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint,
  width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum,
  imageSize: GLsizei, data: *const void,
) -> () {
  let u: usize =
    glCompressedTexSubImage3D_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLenum,
    GLint,
    GLint,
    GLint,
    GLint,
    GLsizei,
    GLsizei,
    GLsizei,
    GLenum,
    GLsizei,
    *const void,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(
    target, level, xoffset, yoffset, zoffset, width, height, depth, format,
    imageSize, data,
  )
}

static glCopyBufferSubData_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glCopyBufferSubData(
  readTarget: GLenum, writeTarget: GLenum, readOffset: GLintptr,
  writeOffset: GLintptr, size: GLsizeiptr,
) -> () {
  let u: usize =
    glCopyBufferSubData_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLenum,
    GLenum,
    GLintptr,
    GLintptr,
    GLsizeiptr,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(readTarget, writeTarget, readOffset, writeOffset, size)
}

static glCopyTexImage1D_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glCopyTexImage1D(
  target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint,
  width: GLsizei, border: GLint,
) -> () {
  let u: usize = glCopyTexImage1D_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLenum,
    GLint,
    GLenum,
    GLint,
    GLint,
    GLsizei,
    GLint,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(target, level, internalformat, x, y, width, border)
}

static glCopyTexImage2D_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glCopyTexImage2D(
  target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint,
  width: GLsizei, height: GLsizei, border: GLint,
) -> () {
  let u: usize = glCopyTexImage2D_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLenum,
    GLint,
    GLenum,
    GLint,
    GLint,
    GLsizei,
    GLsizei,
    GLint,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(target, level, internalformat, x, y, width, height, border)
}

static glCopyTexSubImage1D_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glCopyTexSubImage1D(
  target: GLenum, level: GLint, xoffset: GLint, x: GLint, y: GLint,
  width: GLsizei,
) -> () {
  let u: usize =
    glCopyTexSubImage1D_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLenum,
    GLint,
    GLint,
    GLint,
    GLint,
    GLsizei,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(target, level, xoffset, x, y, width)
}

static glCopyTexSubImage2D_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glCopyTexSubImage2D(
  target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, x: GLint,
  y: GLint, width: GLsizei, height: GLsizei,
) -> () {
  let u: usize =
    glCopyTexSubImage2D_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLenum,
    GLint,
    GLint,
    GLint,
    GLint,
    GLint,
    GLsizei,
    GLsizei,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(target, level, xoffset, yoffset, x, y, width, height)
}

static glCopyTexSubImage3D_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glCopyTexSubImage3D(
  target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint,
  x: GLint, y: GLint, width: GLsizei, height: GLsizei,
) -> () {
  let u: usize =
    glCopyTexSubImage3D_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLenum,
    GLint,
    GLint,
    GLint,
    GLint,
    GLint,
    GLint,
    GLsizei,
    GLsizei,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(target, level, xoffset, yoffset, zoffset, x, y, width, height)
}

static glCreateProgram_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glCreateProgram() -> GLuint {
  let u: usize = glCreateProgram_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn() -> GLuint =
    unsafe { core::mem::transmute(u) };
  _func_p()
}

static glCreateShader_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glCreateShader(type_: GLenum) -> GLuint {
  let u: usize = glCreateShader_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLenum) -> GLuint =
    unsafe { core::mem::transmute(u) };
  _func_p(type_)
}

static glCullFace_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glCullFace(mode: GLenum) -> () {
  let u: usize = glCullFace_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLenum) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(mode)
}

static glDebugMessageCallback_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glDebugMessageCallback(
  callback: GLDEBUGPROC, userParam: *const void,
) -> () {
  let u: usize =
    glDebugMessageCallback_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLDEBUGPROC, *const void) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(callback, userParam)
}

static glDebugMessageControl_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glDebugMessageControl(
  source: GLenum, type_: GLenum, severity: GLenum, count: GLsizei,
  ids: *const GLuint, enabled: GLboolean,
) -> () {
  let u: usize =
    glDebugMessageControl_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLenum,
    GLenum,
    GLenum,
    GLsizei,
    *const GLuint,
    GLboolean,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(source, type_, severity, count, ids, enabled)
}

static glDebugMessageInsert_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glDebugMessageInsert(
  source: GLenum, type_: GLenum, id: GLuint, severity: GLenum, length: GLsizei,
  buf: *const GLchar,
) -> () {
  let u: usize =
    glDebugMessageInsert_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLenum,
    GLenum,
    GLuint,
    GLenum,
    GLsizei,
    *const GLchar,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(source, type_, id, severity, length, buf)
}

static glDeleteBuffers_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glDeleteBuffers(n: GLsizei, buffers: *const GLuint) -> () {
  let u: usize = glDeleteBuffers_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLsizei, *const GLuint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(n, buffers)
}

static glDeleteFramebuffers_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glDeleteFramebuffers(
  n: GLsizei, framebuffers: *const GLuint,
) -> () {
  let u: usize =
    glDeleteFramebuffers_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLsizei, *const GLuint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(n, framebuffers)
}

static glDeleteProgram_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glDeleteProgram(program: GLuint) -> () {
  let u: usize = glDeleteProgram_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(program)
}

static glDeleteQueries_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glDeleteQueries(n: GLsizei, ids: *const GLuint) -> () {
  let u: usize = glDeleteQueries_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLsizei, *const GLuint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(n, ids)
}

static glDeleteRenderbuffers_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glDeleteRenderbuffers(
  n: GLsizei, renderbuffers: *const GLuint,
) -> () {
  let u: usize =
    glDeleteRenderbuffers_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLsizei, *const GLuint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(n, renderbuffers)
}

static glDeleteSamplers_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glDeleteSamplers(count: GLsizei, samplers: *const GLuint) -> () {
  let u: usize = glDeleteSamplers_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLsizei, *const GLuint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(count, samplers)
}

static glDeleteShader_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glDeleteShader(shader: GLuint) -> () {
  let u: usize = glDeleteShader_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(shader)
}

static glDeleteSync_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glDeleteSync(sync: GLsync) -> () {
  let u: usize = glDeleteSync_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLsync) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(sync)
}

static glDeleteTextures_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glDeleteTextures(n: GLsizei, textures: *const GLuint) -> () {
  let u: usize = glDeleteTextures_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLsizei, *const GLuint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(n, textures)
}

static glDeleteVertexArrays_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glDeleteVertexArrays(n: GLsizei, arrays: *const GLuint) -> () {
  let u: usize =
    glDeleteVertexArrays_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLsizei, *const GLuint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(n, arrays)
}

static glDepthFunc_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glDepthFunc(func: GLenum) -> () {
  let u: usize = glDepthFunc_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLenum) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(func)
}

static glDepthMask_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glDepthMask(flag: GLboolean) -> () {
  let u: usize = glDepthMask_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLboolean) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(flag)
}

static glDepthRange_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glDepthRange(n: GLdouble, f: GLdouble) -> () {
  let u: usize = glDepthRange_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLdouble, GLdouble) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(n, f)
}

static glDetachShader_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glDetachShader(program: GLuint, shader: GLuint) -> () {
  let u: usize = glDetachShader_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint, GLuint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(program, shader)
}

static glDisable_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glDisable(cap: GLenum) -> () {
  let u: usize = glDisable_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLenum) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(cap)
}

static glDisableVertexAttribArray_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glDisableVertexAttribArray(index: GLuint) -> () {
  let u: usize =
    glDisableVertexAttribArray_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(index)
}

static glDisablei_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glDisablei(target: GLenum, index: GLuint) -> () {
  let u: usize = glDisablei_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLenum, GLuint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(target, index)
}

static glDrawArrays_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glDrawArrays(mode: GLenum, first: GLint, count: GLsizei) -> () {
  let u: usize = glDrawArrays_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLenum, GLint, GLsizei) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(mode, first, count)
}

static glDrawArraysInstanced_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glDrawArraysInstanced(
  mode: GLenum, first: GLint, count: GLsizei, instancecount: GLsizei,
) -> () {
  let u: usize =
    glDrawArraysInstanced_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLenum,
    GLint,
    GLsizei,
    GLsizei,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(mode, first, count, instancecount)
}

static glDrawBuffer_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glDrawBuffer(buf: GLenum) -> () {
  let u: usize = glDrawBuffer_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLenum) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(buf)
}

static glDrawBuffers_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glDrawBuffers(n: GLsizei, bufs: *const GLenum) -> () {
  let u: usize = glDrawBuffers_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLsizei, *const GLenum) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(n, bufs)
}

static glDrawElements_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glDrawElements(
  mode: GLenum, count: GLsizei, type_: GLenum, indices: *const void,
) -> () {
  let u: usize = glDrawElements_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLenum,
    GLsizei,
    GLenum,
    *const void,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(mode, count, type_, indices)
}

static glDrawElementsBaseVertex_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glDrawElementsBaseVertex(
  mode: GLenum, count: GLsizei, type_: GLenum, indices: *const void,
  basevertex: GLint,
) -> () {
  let u: usize =
    glDrawElementsBaseVertex_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLenum,
    GLsizei,
    GLenum,
    *const void,
    GLint,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(mode, count, type_, indices, basevertex)
}

static glDrawElementsInstanced_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glDrawElementsInstanced(
  mode: GLenum, count: GLsizei, type_: GLenum, indices: *const void,
  instancecount: GLsizei,
) -> () {
  let u: usize =
    glDrawElementsInstanced_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLenum,
    GLsizei,
    GLenum,
    *const void,
    GLsizei,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(mode, count, type_, indices, instancecount)
}

static glDrawElementsInstancedBaseVertex_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glDrawElementsInstancedBaseVertex(
  mode: GLenum, count: GLsizei, type_: GLenum, indices: *const void,
  instancecount: GLsizei, basevertex: GLint,
) -> () {
  let u: usize = glDrawElementsInstancedBaseVertex_p
    .load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLenum,
    GLsizei,
    GLenum,
    *const void,
    GLsizei,
    GLint,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(mode, count, type_, indices, instancecount, basevertex)
}

static glDrawRangeElements_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glDrawRangeElements(
  mode: GLenum, start: GLuint, end: GLuint, count: GLsizei, type_: GLenum,
  indices: *const void,
) -> () {
  let u: usize =
    glDrawRangeElements_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLenum,
    GLuint,
    GLuint,
    GLsizei,
    GLenum,
    *const void,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(mode, start, end, count, type_, indices)
}

static glDrawRangeElementsBaseVertex_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glDrawRangeElementsBaseVertex(
  mode: GLenum, start: GLuint, end: GLuint, count: GLsizei, type_: GLenum,
  indices: *const void, basevertex: GLint,
) -> () {
  let u: usize =
    glDrawRangeElementsBaseVertex_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLenum,
    GLuint,
    GLuint,
    GLsizei,
    GLenum,
    *const void,
    GLint,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(mode, start, end, count, type_, indices, basevertex)
}

static glEnable_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glEnable(cap: GLenum) -> () {
  let u: usize = glEnable_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLenum) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(cap)
}

static glEnableVertexAttribArray_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glEnableVertexAttribArray(index: GLuint) -> () {
  let u: usize =
    glEnableVertexAttribArray_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(index)
}

static glEnablei_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glEnablei(target: GLenum, index: GLuint) -> () {
  let u: usize = glEnablei_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLenum, GLuint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(target, index)
}

static glEndConditionalRender_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glEndConditionalRender() -> () {
  let u: usize =
    glEndConditionalRender_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn() -> () =
    unsafe { core::mem::transmute(u) };
  _func_p()
}

static glEndQuery_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glEndQuery(target: GLenum) -> () {
  let u: usize = glEndQuery_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLenum) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(target)
}

static glEndTransformFeedback_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glEndTransformFeedback() -> () {
  let u: usize =
    glEndTransformFeedback_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn() -> () =
    unsafe { core::mem::transmute(u) };
  _func_p()
}

static glFenceSync_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glFenceSync(condition: GLenum, flags: GLbitfield) -> GLsync {
  let u: usize = glFenceSync_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLenum, GLbitfield) -> GLsync =
    unsafe { core::mem::transmute(u) };
  _func_p(condition, flags)
}

static glFinish_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glFinish() -> () {
  let u: usize = glFinish_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn() -> () =
    unsafe { core::mem::transmute(u) };
  _func_p()
}

static glFlush_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glFlush() -> () {
  let u: usize = glFlush_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn() -> () =
    unsafe { core::mem::transmute(u) };
  _func_p()
}

static glFlushMappedBufferRange_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glFlushMappedBufferRange(
  target: GLenum, offset: GLintptr, length: GLsizeiptr,
) -> () {
  let u: usize =
    glFlushMappedBufferRange_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLenum, GLintptr, GLsizeiptr) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(target, offset, length)
}

static glFramebufferRenderbuffer_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glFramebufferRenderbuffer(
  target: GLenum, attachment: GLenum, renderbuffertarget: GLenum,
  renderbuffer: GLuint,
) -> () {
  let u: usize =
    glFramebufferRenderbuffer_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLenum, GLenum, GLenum, GLuint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(target, attachment, renderbuffertarget, renderbuffer)
}

static glFramebufferTexture_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glFramebufferTexture(
  target: GLenum, attachment: GLenum, texture: GLuint, level: GLint,
) -> () {
  let u: usize =
    glFramebufferTexture_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLenum, GLenum, GLuint, GLint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(target, attachment, texture, level)
}

static glFramebufferTexture1D_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glFramebufferTexture1D(
  target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint,
  level: GLint,
) -> () {
  let u: usize =
    glFramebufferTexture1D_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLenum,
    GLenum,
    GLenum,
    GLuint,
    GLint,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(target, attachment, textarget, texture, level)
}

static glFramebufferTexture2D_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glFramebufferTexture2D(
  target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint,
  level: GLint,
) -> () {
  let u: usize =
    glFramebufferTexture2D_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLenum,
    GLenum,
    GLenum,
    GLuint,
    GLint,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(target, attachment, textarget, texture, level)
}

static glFramebufferTexture3D_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glFramebufferTexture3D(
  target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint,
  level: GLint, zoffset: GLint,
) -> () {
  let u: usize =
    glFramebufferTexture3D_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLenum,
    GLenum,
    GLenum,
    GLuint,
    GLint,
    GLint,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(target, attachment, textarget, texture, level, zoffset)
}

static glFramebufferTextureLayer_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glFramebufferTextureLayer(
  target: GLenum, attachment: GLenum, texture: GLuint, level: GLint,
  layer: GLint,
) -> () {
  let u: usize =
    glFramebufferTextureLayer_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLenum,
    GLenum,
    GLuint,
    GLint,
    GLint,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(target, attachment, texture, level, layer)
}

static glFrontFace_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glFrontFace(mode: GLenum) -> () {
  let u: usize = glFrontFace_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLenum) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(mode)
}

static glGenBuffers_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glGenBuffers(n: GLsizei, buffers: *mut GLuint) -> () {
  let u: usize = glGenBuffers_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLsizei, *mut GLuint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(n, buffers)
}

static glGenFramebuffers_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glGenFramebuffers(n: GLsizei, framebuffers: *mut GLuint) -> () {
  let u: usize =
    glGenFramebuffers_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLsizei, *mut GLuint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(n, framebuffers)
}

static glGenQueries_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glGenQueries(n: GLsizei, ids: *mut GLuint) -> () {
  let u: usize = glGenQueries_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLsizei, *mut GLuint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(n, ids)
}

static glGenRenderbuffers_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glGenRenderbuffers(n: GLsizei, renderbuffers: *mut GLuint) -> () {
  let u: usize =
    glGenRenderbuffers_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLsizei, *mut GLuint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(n, renderbuffers)
}

static glGenSamplers_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glGenSamplers(count: GLsizei, samplers: *mut GLuint) -> () {
  let u: usize = glGenSamplers_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLsizei, *mut GLuint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(count, samplers)
}

static glGenTextures_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glGenTextures(n: GLsizei, textures: *mut GLuint) -> () {
  let u: usize = glGenTextures_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLsizei, *mut GLuint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(n, textures)
}

static glGenVertexArrays_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glGenVertexArrays(n: GLsizei, arrays: *mut GLuint) -> () {
  let u: usize =
    glGenVertexArrays_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLsizei, *mut GLuint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(n, arrays)
}

static glGenerateMipmap_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glGenerateMipmap(target: GLenum) -> () {
  let u: usize = glGenerateMipmap_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLenum) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(target)
}

static glGetActiveAttrib_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glGetActiveAttrib(
  program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei,
  size: *mut GLint, type_: *mut GLenum, name: *mut GLchar,
) -> () {
  let u: usize =
    glGetActiveAttrib_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLuint,
    GLuint,
    GLsizei,
    *mut GLsizei,
    *mut GLint,
    *mut GLenum,
    *mut GLchar,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(program, index, bufSize, length, size, type_, name)
}

static glGetActiveUniform_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glGetActiveUniform(
  program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei,
  size: *mut GLint, type_: *mut GLenum, name: *mut GLchar,
) -> () {
  let u: usize =
    glGetActiveUniform_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLuint,
    GLuint,
    GLsizei,
    *mut GLsizei,
    *mut GLint,
    *mut GLenum,
    *mut GLchar,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(program, index, bufSize, length, size, type_, name)
}

static glGetActiveUniformBlockName_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glGetActiveUniformBlockName(
  program: GLuint, uniformBlockIndex: GLuint, bufSize: GLsizei,
  length: *mut GLsizei, uniformBlockName: *mut GLchar,
) -> () {
  let u: usize =
    glGetActiveUniformBlockName_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLuint,
    GLuint,
    GLsizei,
    *mut GLsizei,
    *mut GLchar,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(program, uniformBlockIndex, bufSize, length, uniformBlockName)
}

static glGetActiveUniformBlockiv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glGetActiveUniformBlockiv(
  program: GLuint, uniformBlockIndex: GLuint, pname: GLenum, params: *mut GLint,
) -> () {
  let u: usize =
    glGetActiveUniformBlockiv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLuint,
    GLuint,
    GLenum,
    *mut GLint,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(program, uniformBlockIndex, pname, params)
}

static glGetActiveUniformName_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glGetActiveUniformName(
  program: GLuint, uniformIndex: GLuint, bufSize: GLsizei,
  length: *mut GLsizei, uniformName: *mut GLchar,
) -> () {
  let u: usize =
    glGetActiveUniformName_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLuint,
    GLuint,
    GLsizei,
    *mut GLsizei,
    *mut GLchar,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(program, uniformIndex, bufSize, length, uniformName)
}

static glGetActiveUniformsiv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glGetActiveUniformsiv(
  program: GLuint, uniformCount: GLsizei, uniformIndices: *const GLuint,
  pname: GLenum, params: *mut GLint,
) -> () {
  let u: usize =
    glGetActiveUniformsiv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLuint,
    GLsizei,
    *const GLuint,
    GLenum,
    *mut GLint,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(program, uniformCount, uniformIndices, pname, params)
}

static glGetAttachedShaders_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glGetAttachedShaders(
  program: GLuint, maxCount: GLsizei, count: *mut GLsizei, shaders: *mut GLuint,
) -> () {
  let u: usize =
    glGetAttachedShaders_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLuint,
    GLsizei,
    *mut GLsizei,
    *mut GLuint,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(program, maxCount, count, shaders)
}

static glGetAttribLocation_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glGetAttribLocation(
  program: GLuint, name: *const GLchar,
) -> GLint {
  let u: usize =
    glGetAttribLocation_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint, *const GLchar) -> GLint =
    unsafe { core::mem::transmute(u) };
  _func_p(program, name)
}

static glGetBooleani_v_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glGetBooleani_v(
  target: GLenum, index: GLuint, data: *mut GLboolean,
) -> () {
  let u: usize = glGetBooleani_v_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLenum, GLuint, *mut GLboolean) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(target, index, data)
}

static glGetBooleanv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glGetBooleanv(pname: GLenum, data: *mut GLboolean) -> () {
  let u: usize = glGetBooleanv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLenum, *mut GLboolean) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(pname, data)
}

static glGetBufferParameteri64v_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glGetBufferParameteri64v(
  target: GLenum, pname: GLenum, params: *mut GLint64,
) -> () {
  let u: usize =
    glGetBufferParameteri64v_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLenum, GLenum, *mut GLint64) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(target, pname, params)
}

static glGetBufferParameteriv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glGetBufferParameteriv(
  target: GLenum, pname: GLenum, params: *mut GLint,
) -> () {
  let u: usize =
    glGetBufferParameteriv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLenum, GLenum, *mut GLint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(target, pname, params)
}

static glGetBufferPointerv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glGetBufferPointerv(
  target: GLenum, pname: GLenum, params: *mut *mut void,
) -> () {
  let u: usize =
    glGetBufferPointerv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLenum, GLenum, *mut *mut void) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(target, pname, params)
}

static glGetBufferSubData_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glGetBufferSubData(
  target: GLenum, offset: GLintptr, size: GLsizeiptr, data: *mut void,
) -> () {
  let u: usize =
    glGetBufferSubData_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLenum,
    GLintptr,
    GLsizeiptr,
    *mut void,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(target, offset, size, data)
}

static glGetCompressedTexImage_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glGetCompressedTexImage(
  target: GLenum, level: GLint, img: *mut void,
) -> () {
  let u: usize =
    glGetCompressedTexImage_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLenum, GLint, *mut void) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(target, level, img)
}

static glGetDebugMessageLog_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glGetDebugMessageLog(
  count: GLuint, bufSize: GLsizei, sources: *mut GLenum, types: *mut GLenum,
  ids: *mut GLuint, severities: *mut GLenum, lengths: *mut GLsizei,
  messageLog: *mut GLchar,
) -> GLuint {
  let u: usize =
    glGetDebugMessageLog_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLuint,
    GLsizei,
    *mut GLenum,
    *mut GLenum,
    *mut GLuint,
    *mut GLenum,
    *mut GLsizei,
    *mut GLchar,
  ) -> GLuint = unsafe { core::mem::transmute(u) };
  _func_p(count, bufSize, sources, types, ids, severities, lengths, messageLog)
}

static glGetDoublev_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glGetDoublev(pname: GLenum, data: *mut GLdouble) -> () {
  let u: usize = glGetDoublev_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLenum, *mut GLdouble) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(pname, data)
}

static glGetError_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glGetError() -> GLenum {
  let u: usize = glGetError_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn() -> GLenum =
    unsafe { core::mem::transmute(u) };
  _func_p()
}

static glGetFloatv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glGetFloatv(pname: GLenum, data: *mut GLfloat) -> () {
  let u: usize = glGetFloatv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLenum, *mut GLfloat) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(pname, data)
}

static glGetFragDataIndex_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glGetFragDataIndex(
  program: GLuint, name: *const GLchar,
) -> GLint {
  let u: usize =
    glGetFragDataIndex_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint, *const GLchar) -> GLint =
    unsafe { core::mem::transmute(u) };
  _func_p(program, name)
}

static glGetFragDataLocation_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glGetFragDataLocation(
  program: GLuint, name: *const GLchar,
) -> GLint {
  let u: usize =
    glGetFragDataLocation_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint, *const GLchar) -> GLint =
    unsafe { core::mem::transmute(u) };
  _func_p(program, name)
}

static glGetFramebufferAttachmentParameteriv_p:
  core::sync::atomic::AtomicUsize = core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glGetFramebufferAttachmentParameteriv(
  target: GLenum, attachment: GLenum, pname: GLenum, params: *mut GLint,
) -> () {
  let u: usize = glGetFramebufferAttachmentParameteriv_p
    .load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLenum,
    GLenum,
    GLenum,
    *mut GLint,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(target, attachment, pname, params)
}

static glGetInteger64i_v_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glGetInteger64i_v(
  target: GLenum, index: GLuint, data: *mut GLint64,
) -> () {
  let u: usize =
    glGetInteger64i_v_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLenum, GLuint, *mut GLint64) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(target, index, data)
}

static glGetInteger64v_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glGetInteger64v(pname: GLenum, data: *mut GLint64) -> () {
  let u: usize = glGetInteger64v_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLenum, *mut GLint64) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(pname, data)
}

static glGetIntegeri_v_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glGetIntegeri_v(
  target: GLenum, index: GLuint, data: *mut GLint,
) -> () {
  let u: usize = glGetIntegeri_v_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLenum, GLuint, *mut GLint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(target, index, data)
}

static glGetIntegerv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glGetIntegerv(pname: GLenum, data: *mut GLint) -> () {
  let u: usize = glGetIntegerv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLenum, *mut GLint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(pname, data)
}

static glGetMultisamplefv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glGetMultisamplefv(
  pname: GLenum, index: GLuint, val: *mut GLfloat,
) -> () {
  let u: usize =
    glGetMultisamplefv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLenum, GLuint, *mut GLfloat) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(pname, index, val)
}

static glGetObjectLabel_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glGetObjectLabel(
  identifier: GLenum, name: GLuint, bufSize: GLsizei, length: *mut GLsizei,
  label: *mut GLchar,
) -> () {
  let u: usize = glGetObjectLabel_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLenum,
    GLuint,
    GLsizei,
    *mut GLsizei,
    *mut GLchar,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(identifier, name, bufSize, length, label)
}

static glGetObjectPtrLabel_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glGetObjectPtrLabel(
  ptr: *const void, bufSize: GLsizei, length: *mut GLsizei, label: *mut GLchar,
) -> () {
  let u: usize =
    glGetObjectPtrLabel_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    *const void,
    GLsizei,
    *mut GLsizei,
    *mut GLchar,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(ptr, bufSize, length, label)
}

static glGetPointerv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glGetPointerv(pname: GLenum, params: *mut *mut void) -> () {
  let u: usize = glGetPointerv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLenum, *mut *mut void) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(pname, params)
}

static glGetProgramInfoLog_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glGetProgramInfoLog(
  program: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar,
) -> () {
  let u: usize =
    glGetProgramInfoLog_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLuint,
    GLsizei,
    *mut GLsizei,
    *mut GLchar,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(program, bufSize, length, infoLog)
}

static glGetProgramiv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glGetProgramiv(
  program: GLuint, pname: GLenum, params: *mut GLint,
) -> () {
  let u: usize = glGetProgramiv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint, GLenum, *mut GLint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(program, pname, params)
}

static glGetQueryObjecti64v_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glGetQueryObjecti64v(
  id: GLuint, pname: GLenum, params: *mut GLint64,
) -> () {
  let u: usize =
    glGetQueryObjecti64v_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint, GLenum, *mut GLint64) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(id, pname, params)
}

static glGetQueryObjectiv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glGetQueryObjectiv(
  id: GLuint, pname: GLenum, params: *mut GLint,
) -> () {
  let u: usize =
    glGetQueryObjectiv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint, GLenum, *mut GLint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(id, pname, params)
}

static glGetQueryObjectui64v_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glGetQueryObjectui64v(
  id: GLuint, pname: GLenum, params: *mut GLuint64,
) -> () {
  let u: usize =
    glGetQueryObjectui64v_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint, GLenum, *mut GLuint64) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(id, pname, params)
}

static glGetQueryObjectuiv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glGetQueryObjectuiv(
  id: GLuint, pname: GLenum, params: *mut GLuint,
) -> () {
  let u: usize =
    glGetQueryObjectuiv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint, GLenum, *mut GLuint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(id, pname, params)
}

static glGetQueryiv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glGetQueryiv(
  target: GLenum, pname: GLenum, params: *mut GLint,
) -> () {
  let u: usize = glGetQueryiv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLenum, GLenum, *mut GLint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(target, pname, params)
}

static glGetRenderbufferParameteriv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glGetRenderbufferParameteriv(
  target: GLenum, pname: GLenum, params: *mut GLint,
) -> () {
  let u: usize =
    glGetRenderbufferParameteriv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLenum, GLenum, *mut GLint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(target, pname, params)
}

static glGetSamplerParameterIiv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glGetSamplerParameterIiv(
  sampler: GLuint, pname: GLenum, params: *mut GLint,
) -> () {
  let u: usize =
    glGetSamplerParameterIiv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint, GLenum, *mut GLint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(sampler, pname, params)
}

static glGetSamplerParameterIuiv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glGetSamplerParameterIuiv(
  sampler: GLuint, pname: GLenum, params: *mut GLuint,
) -> () {
  let u: usize =
    glGetSamplerParameterIuiv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint, GLenum, *mut GLuint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(sampler, pname, params)
}

static glGetSamplerParameterfv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glGetSamplerParameterfv(
  sampler: GLuint, pname: GLenum, params: *mut GLfloat,
) -> () {
  let u: usize =
    glGetSamplerParameterfv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint, GLenum, *mut GLfloat) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(sampler, pname, params)
}

static glGetSamplerParameteriv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glGetSamplerParameteriv(
  sampler: GLuint, pname: GLenum, params: *mut GLint,
) -> () {
  let u: usize =
    glGetSamplerParameteriv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint, GLenum, *mut GLint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(sampler, pname, params)
}

static glGetShaderInfoLog_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glGetShaderInfoLog(
  shader: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar,
) -> () {
  let u: usize =
    glGetShaderInfoLog_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLuint,
    GLsizei,
    *mut GLsizei,
    *mut GLchar,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(shader, bufSize, length, infoLog)
}

static glGetShaderSource_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glGetShaderSource(
  shader: GLuint, bufSize: GLsizei, length: *mut GLsizei, source: *mut GLchar,
) -> () {
  let u: usize =
    glGetShaderSource_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLuint,
    GLsizei,
    *mut GLsizei,
    *mut GLchar,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(shader, bufSize, length, source)
}

static glGetShaderiv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glGetShaderiv(
  shader: GLuint, pname: GLenum, params: *mut GLint,
) -> () {
  let u: usize = glGetShaderiv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint, GLenum, *mut GLint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(shader, pname, params)
}

static glGetString_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glGetString(name: GLenum) -> *const GLubyte {
  let u: usize = glGetString_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLenum) -> *const GLubyte =
    unsafe { core::mem::transmute(u) };
  _func_p(name)
}

static glGetStringi_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glGetStringi(name: GLenum, index: GLuint) -> *const GLubyte {
  let u: usize = glGetStringi_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLenum, GLuint) -> *const GLubyte =
    unsafe { core::mem::transmute(u) };
  _func_p(name, index)
}

static glGetSynciv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glGetSynciv(
  sync: GLsync, pname: GLenum, count: GLsizei, length: *mut GLsizei,
  values: *mut GLint,
) -> () {
  let u: usize = glGetSynciv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLsync,
    GLenum,
    GLsizei,
    *mut GLsizei,
    *mut GLint,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(sync, pname, count, length, values)
}

static glGetTexImage_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glGetTexImage(
  target: GLenum, level: GLint, format: GLenum, type_: GLenum,
  pixels: *mut void,
) -> () {
  let u: usize = glGetTexImage_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLenum,
    GLint,
    GLenum,
    GLenum,
    *mut void,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(target, level, format, type_, pixels)
}

static glGetTexLevelParameterfv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glGetTexLevelParameterfv(
  target: GLenum, level: GLint, pname: GLenum, params: *mut GLfloat,
) -> () {
  let u: usize =
    glGetTexLevelParameterfv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLenum,
    GLint,
    GLenum,
    *mut GLfloat,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(target, level, pname, params)
}

static glGetTexLevelParameteriv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glGetTexLevelParameteriv(
  target: GLenum, level: GLint, pname: GLenum, params: *mut GLint,
) -> () {
  let u: usize =
    glGetTexLevelParameteriv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLenum,
    GLint,
    GLenum,
    *mut GLint,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(target, level, pname, params)
}

static glGetTexParameterIiv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glGetTexParameterIiv(
  target: GLenum, pname: GLenum, params: *mut GLint,
) -> () {
  let u: usize =
    glGetTexParameterIiv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLenum, GLenum, *mut GLint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(target, pname, params)
}

static glGetTexParameterIuiv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glGetTexParameterIuiv(
  target: GLenum, pname: GLenum, params: *mut GLuint,
) -> () {
  let u: usize =
    glGetTexParameterIuiv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLenum, GLenum, *mut GLuint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(target, pname, params)
}

static glGetTexParameterfv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glGetTexParameterfv(
  target: GLenum, pname: GLenum, params: *mut GLfloat,
) -> () {
  let u: usize =
    glGetTexParameterfv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLenum, GLenum, *mut GLfloat) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(target, pname, params)
}

static glGetTexParameteriv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glGetTexParameteriv(
  target: GLenum, pname: GLenum, params: *mut GLint,
) -> () {
  let u: usize =
    glGetTexParameteriv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLenum, GLenum, *mut GLint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(target, pname, params)
}

static glGetTransformFeedbackVarying_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glGetTransformFeedbackVarying(
  program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei,
  size: *mut GLsizei, type_: *mut GLenum, name: *mut GLchar,
) -> () {
  let u: usize =
    glGetTransformFeedbackVarying_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLuint,
    GLuint,
    GLsizei,
    *mut GLsizei,
    *mut GLsizei,
    *mut GLenum,
    *mut GLchar,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(program, index, bufSize, length, size, type_, name)
}

static glGetUniformBlockIndex_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glGetUniformBlockIndex(
  program: GLuint, uniformBlockName: *const GLchar,
) -> GLuint {
  let u: usize =
    glGetUniformBlockIndex_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint, *const GLchar) -> GLuint =
    unsafe { core::mem::transmute(u) };
  _func_p(program, uniformBlockName)
}

static glGetUniformIndices_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glGetUniformIndices(
  program: GLuint, uniformCount: GLsizei, uniformNames: *const *const GLchar,
  uniformIndices: *mut GLuint,
) -> () {
  let u: usize =
    glGetUniformIndices_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLuint,
    GLsizei,
    *const *const GLchar,
    *mut GLuint,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(program, uniformCount, uniformNames, uniformIndices)
}

static glGetUniformLocation_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glGetUniformLocation(
  program: GLuint, name: *const GLchar,
) -> GLint {
  let u: usize =
    glGetUniformLocation_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint, *const GLchar) -> GLint =
    unsafe { core::mem::transmute(u) };
  _func_p(program, name)
}

static glGetUniformfv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glGetUniformfv(
  program: GLuint, location: GLint, params: *mut GLfloat,
) -> () {
  let u: usize = glGetUniformfv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint, GLint, *mut GLfloat) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(program, location, params)
}

static glGetUniformiv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glGetUniformiv(
  program: GLuint, location: GLint, params: *mut GLint,
) -> () {
  let u: usize = glGetUniformiv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint, GLint, *mut GLint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(program, location, params)
}

static glGetUniformuiv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glGetUniformuiv(
  program: GLuint, location: GLint, params: *mut GLuint,
) -> () {
  let u: usize = glGetUniformuiv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint, GLint, *mut GLuint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(program, location, params)
}

static glGetVertexAttribIiv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glGetVertexAttribIiv(
  index: GLuint, pname: GLenum, params: *mut GLint,
) -> () {
  let u: usize =
    glGetVertexAttribIiv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint, GLenum, *mut GLint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(index, pname, params)
}

static glGetVertexAttribIuiv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glGetVertexAttribIuiv(
  index: GLuint, pname: GLenum, params: *mut GLuint,
) -> () {
  let u: usize =
    glGetVertexAttribIuiv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint, GLenum, *mut GLuint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(index, pname, params)
}

static glGetVertexAttribPointerv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glGetVertexAttribPointerv(
  index: GLuint, pname: GLenum, pointer: *mut *mut void,
) -> () {
  let u: usize =
    glGetVertexAttribPointerv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint, GLenum, *mut *mut void) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(index, pname, pointer)
}

static glGetVertexAttribdv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glGetVertexAttribdv(
  index: GLuint, pname: GLenum, params: *mut GLdouble,
) -> () {
  let u: usize =
    glGetVertexAttribdv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint, GLenum, *mut GLdouble) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(index, pname, params)
}

static glGetVertexAttribfv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glGetVertexAttribfv(
  index: GLuint, pname: GLenum, params: *mut GLfloat,
) -> () {
  let u: usize =
    glGetVertexAttribfv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint, GLenum, *mut GLfloat) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(index, pname, params)
}

static glGetVertexAttribiv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glGetVertexAttribiv(
  index: GLuint, pname: GLenum, params: *mut GLint,
) -> () {
  let u: usize =
    glGetVertexAttribiv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint, GLenum, *mut GLint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(index, pname, params)
}

static glHint_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glHint(target: GLenum, mode: GLenum) -> () {
  let u: usize = glHint_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLenum, GLenum) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(target, mode)
}

static glIsBuffer_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glIsBuffer(buffer: GLuint) -> GLboolean {
  let u: usize = glIsBuffer_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint) -> GLboolean =
    unsafe { core::mem::transmute(u) };
  _func_p(buffer)
}

static glIsEnabled_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glIsEnabled(cap: GLenum) -> GLboolean {
  let u: usize = glIsEnabled_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLenum) -> GLboolean =
    unsafe { core::mem::transmute(u) };
  _func_p(cap)
}

static glIsEnabledi_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glIsEnabledi(target: GLenum, index: GLuint) -> GLboolean {
  let u: usize = glIsEnabledi_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLenum, GLuint) -> GLboolean =
    unsafe { core::mem::transmute(u) };
  _func_p(target, index)
}

static glIsFramebuffer_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glIsFramebuffer(framebuffer: GLuint) -> GLboolean {
  let u: usize = glIsFramebuffer_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint) -> GLboolean =
    unsafe { core::mem::transmute(u) };
  _func_p(framebuffer)
}

static glIsProgram_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glIsProgram(program: GLuint) -> GLboolean {
  let u: usize = glIsProgram_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint) -> GLboolean =
    unsafe { core::mem::transmute(u) };
  _func_p(program)
}

static glIsQuery_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glIsQuery(id: GLuint) -> GLboolean {
  let u: usize = glIsQuery_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint) -> GLboolean =
    unsafe { core::mem::transmute(u) };
  _func_p(id)
}

static glIsRenderbuffer_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glIsRenderbuffer(renderbuffer: GLuint) -> GLboolean {
  let u: usize = glIsRenderbuffer_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint) -> GLboolean =
    unsafe { core::mem::transmute(u) };
  _func_p(renderbuffer)
}

static glIsSampler_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glIsSampler(sampler: GLuint) -> GLboolean {
  let u: usize = glIsSampler_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint) -> GLboolean =
    unsafe { core::mem::transmute(u) };
  _func_p(sampler)
}

static glIsShader_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glIsShader(shader: GLuint) -> GLboolean {
  let u: usize = glIsShader_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint) -> GLboolean =
    unsafe { core::mem::transmute(u) };
  _func_p(shader)
}

static glIsSync_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glIsSync(sync: GLsync) -> GLboolean {
  let u: usize = glIsSync_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLsync) -> GLboolean =
    unsafe { core::mem::transmute(u) };
  _func_p(sync)
}

static glIsTexture_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glIsTexture(texture: GLuint) -> GLboolean {
  let u: usize = glIsTexture_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint) -> GLboolean =
    unsafe { core::mem::transmute(u) };
  _func_p(texture)
}

static glIsVertexArray_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glIsVertexArray(array: GLuint) -> GLboolean {
  let u: usize = glIsVertexArray_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint) -> GLboolean =
    unsafe { core::mem::transmute(u) };
  _func_p(array)
}

static glLineWidth_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glLineWidth(width: GLfloat) -> () {
  let u: usize = glLineWidth_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLfloat) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(width)
}

static glLinkProgram_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glLinkProgram(program: GLuint) -> () {
  let u: usize = glLinkProgram_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(program)
}

static glLogicOp_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glLogicOp(opcode: GLenum) -> () {
  let u: usize = glLogicOp_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLenum) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(opcode)
}

static glMapBuffer_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glMapBuffer(target: GLenum, access: GLenum) -> *mut void {
  let u: usize = glMapBuffer_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLenum, GLenum) -> *mut void =
    unsafe { core::mem::transmute(u) };
  _func_p(target, access)
}

static glMapBufferRange_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glMapBufferRange(
  target: GLenum, offset: GLintptr, length: GLsizeiptr, access: GLbitfield,
) -> *mut void {
  let u: usize = glMapBufferRange_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLenum,
    GLintptr,
    GLsizeiptr,
    GLbitfield,
  ) -> *mut void = unsafe { core::mem::transmute(u) };
  _func_p(target, offset, length, access)
}

static glMultiDrawArrays_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glMultiDrawArrays(
  mode: GLenum, first: *const GLint, count: *const GLsizei, drawcount: GLsizei,
) -> () {
  let u: usize =
    glMultiDrawArrays_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLenum,
    *const GLint,
    *const GLsizei,
    GLsizei,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(mode, first, count, drawcount)
}

static glMultiDrawElements_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glMultiDrawElements(
  mode: GLenum, count: *const GLsizei, type_: GLenum,
  indices: *const *const void, drawcount: GLsizei,
) -> () {
  let u: usize =
    glMultiDrawElements_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLenum,
    *const GLsizei,
    GLenum,
    *const *const void,
    GLsizei,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(mode, count, type_, indices, drawcount)
}

static glMultiDrawElementsBaseVertex_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glMultiDrawElementsBaseVertex(
  mode: GLenum, count: *const GLsizei, type_: GLenum,
  indices: *const *const void, drawcount: GLsizei, basevertex: *const GLint,
) -> () {
  let u: usize =
    glMultiDrawElementsBaseVertex_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLenum,
    *const GLsizei,
    GLenum,
    *const *const void,
    GLsizei,
    *const GLint,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(mode, count, type_, indices, drawcount, basevertex)
}

static glObjectLabel_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glObjectLabel(
  identifier: GLenum, name: GLuint, length: GLsizei, label: *const GLchar,
) -> () {
  let u: usize = glObjectLabel_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLenum,
    GLuint,
    GLsizei,
    *const GLchar,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(identifier, name, length, label)
}

static glObjectPtrLabel_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glObjectPtrLabel(
  ptr: *const void, length: GLsizei, label: *const GLchar,
) -> () {
  let u: usize = glObjectPtrLabel_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    *const void,
    GLsizei,
    *const GLchar,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(ptr, length, label)
}

static glPixelStoref_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glPixelStoref(pname: GLenum, param: GLfloat) -> () {
  let u: usize = glPixelStoref_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLenum, GLfloat) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(pname, param)
}

static glPixelStorei_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glPixelStorei(pname: GLenum, param: GLint) -> () {
  let u: usize = glPixelStorei_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLenum, GLint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(pname, param)
}

static glPointParameterf_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glPointParameterf(pname: GLenum, param: GLfloat) -> () {
  let u: usize =
    glPointParameterf_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLenum, GLfloat) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(pname, param)
}

static glPointParameterfv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glPointParameterfv(pname: GLenum, params: *const GLfloat) -> () {
  let u: usize =
    glPointParameterfv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLenum, *const GLfloat) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(pname, params)
}

static glPointParameteri_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glPointParameteri(pname: GLenum, param: GLint) -> () {
  let u: usize =
    glPointParameteri_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLenum, GLint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(pname, param)
}

static glPointParameteriv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glPointParameteriv(pname: GLenum, params: *const GLint) -> () {
  let u: usize =
    glPointParameteriv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLenum, *const GLint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(pname, params)
}

static glPointSize_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glPointSize(size: GLfloat) -> () {
  let u: usize = glPointSize_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLfloat) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(size)
}

static glPolygonMode_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glPolygonMode(face: GLenum, mode: GLenum) -> () {
  let u: usize = glPolygonMode_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLenum, GLenum) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(face, mode)
}

static glPolygonOffset_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glPolygonOffset(factor: GLfloat, units: GLfloat) -> () {
  let u: usize = glPolygonOffset_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLfloat, GLfloat) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(factor, units)
}

static glPopDebugGroup_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glPopDebugGroup() -> () {
  let u: usize = glPopDebugGroup_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn() -> () =
    unsafe { core::mem::transmute(u) };
  _func_p()
}

static glPrimitiveRestartIndex_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glPrimitiveRestartIndex(index: GLuint) -> () {
  let u: usize =
    glPrimitiveRestartIndex_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(index)
}

static glProvokingVertex_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glProvokingVertex(mode: GLenum) -> () {
  let u: usize =
    glProvokingVertex_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLenum) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(mode)
}

static glPushDebugGroup_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glPushDebugGroup(
  source: GLenum, id: GLuint, length: GLsizei, message: *const GLchar,
) -> () {
  let u: usize = glPushDebugGroup_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLenum,
    GLuint,
    GLsizei,
    *const GLchar,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(source, id, length, message)
}

static glQueryCounter_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glQueryCounter(id: GLuint, target: GLenum) -> () {
  let u: usize = glQueryCounter_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint, GLenum) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(id, target)
}

static glReadBuffer_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glReadBuffer(src: GLenum) -> () {
  let u: usize = glReadBuffer_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLenum) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(src)
}

static glReadPixels_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glReadPixels(
  x: GLint, y: GLint, width: GLsizei, height: GLsizei, format: GLenum,
  type_: GLenum, pixels: *mut void,
) -> () {
  let u: usize = glReadPixels_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLint,
    GLint,
    GLsizei,
    GLsizei,
    GLenum,
    GLenum,
    *mut void,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(x, y, width, height, format, type_, pixels)
}

static glRenderbufferStorage_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glRenderbufferStorage(
  target: GLenum, internalformat: GLenum, width: GLsizei, height: GLsizei,
) -> () {
  let u: usize =
    glRenderbufferStorage_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLenum,
    GLenum,
    GLsizei,
    GLsizei,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(target, internalformat, width, height)
}

static glRenderbufferStorageMultisample_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glRenderbufferStorageMultisample(
  target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei,
  height: GLsizei,
) -> () {
  let u: usize = glRenderbufferStorageMultisample_p
    .load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLenum,
    GLsizei,
    GLenum,
    GLsizei,
    GLsizei,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(target, samples, internalformat, width, height)
}

static glSampleCoverage_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glSampleCoverage(value: GLfloat, invert: GLboolean) -> () {
  let u: usize = glSampleCoverage_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLfloat, GLboolean) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(value, invert)
}

static glSampleMaski_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glSampleMaski(maskNumber: GLuint, mask: GLbitfield) -> () {
  let u: usize = glSampleMaski_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint, GLbitfield) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(maskNumber, mask)
}

static glSamplerParameterIiv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glSamplerParameterIiv(
  sampler: GLuint, pname: GLenum, param: *const GLint,
) -> () {
  let u: usize =
    glSamplerParameterIiv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint, GLenum, *const GLint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(sampler, pname, param)
}

static glSamplerParameterIuiv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glSamplerParameterIuiv(
  sampler: GLuint, pname: GLenum, param: *const GLuint,
) -> () {
  let u: usize =
    glSamplerParameterIuiv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint, GLenum, *const GLuint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(sampler, pname, param)
}

static glSamplerParameterf_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glSamplerParameterf(
  sampler: GLuint, pname: GLenum, param: GLfloat,
) -> () {
  let u: usize =
    glSamplerParameterf_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint, GLenum, GLfloat) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(sampler, pname, param)
}

static glSamplerParameterfv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glSamplerParameterfv(
  sampler: GLuint, pname: GLenum, param: *const GLfloat,
) -> () {
  let u: usize =
    glSamplerParameterfv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint, GLenum, *const GLfloat) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(sampler, pname, param)
}

static glSamplerParameteri_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glSamplerParameteri(
  sampler: GLuint, pname: GLenum, param: GLint,
) -> () {
  let u: usize =
    glSamplerParameteri_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint, GLenum, GLint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(sampler, pname, param)
}

static glSamplerParameteriv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glSamplerParameteriv(
  sampler: GLuint, pname: GLenum, param: *const GLint,
) -> () {
  let u: usize =
    glSamplerParameteriv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint, GLenum, *const GLint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(sampler, pname, param)
}

static glScissor_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glScissor(
  x: GLint, y: GLint, width: GLsizei, height: GLsizei,
) -> () {
  let u: usize = glScissor_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLint, GLint, GLsizei, GLsizei) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(x, y, width, height)
}

static glShaderSource_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glShaderSource(
  shader: GLuint, count: GLsizei, string: *const *const GLchar,
  length: *const GLint,
) -> () {
  let u: usize = glShaderSource_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLuint,
    GLsizei,
    *const *const GLchar,
    *const GLint,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(shader, count, string, length)
}

static glStencilFunc_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glStencilFunc(func: GLenum, ref_: GLint, mask: GLuint) -> () {
  let u: usize = glStencilFunc_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLenum, GLint, GLuint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(func, ref_, mask)
}

static glStencilFuncSeparate_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glStencilFuncSeparate(
  face: GLenum, func: GLenum, ref_: GLint, mask: GLuint,
) -> () {
  let u: usize =
    glStencilFuncSeparate_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLenum, GLenum, GLint, GLuint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(face, func, ref_, mask)
}

static glStencilMask_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glStencilMask(mask: GLuint) -> () {
  let u: usize = glStencilMask_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(mask)
}

static glStencilMaskSeparate_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glStencilMaskSeparate(face: GLenum, mask: GLuint) -> () {
  let u: usize =
    glStencilMaskSeparate_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLenum, GLuint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(face, mask)
}

static glStencilOp_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glStencilOp(fail: GLenum, zfail: GLenum, zpass: GLenum) -> () {
  let u: usize = glStencilOp_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLenum, GLenum, GLenum) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(fail, zfail, zpass)
}

static glStencilOpSeparate_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glStencilOpSeparate(
  face: GLenum, sfail: GLenum, dpfail: GLenum, dppass: GLenum,
) -> () {
  let u: usize =
    glStencilOpSeparate_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLenum, GLenum, GLenum, GLenum) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(face, sfail, dpfail, dppass)
}

static glTexBuffer_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glTexBuffer(
  target: GLenum, internalformat: GLenum, buffer: GLuint,
) -> () {
  let u: usize = glTexBuffer_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLenum, GLenum, GLuint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(target, internalformat, buffer)
}

static glTexImage1D_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glTexImage1D(
  target: GLenum, level: GLint, internalformat: GLint, width: GLsizei,
  border: GLint, format: GLenum, type_: GLenum, pixels: *const void,
) -> () {
  let u: usize = glTexImage1D_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLenum,
    GLint,
    GLint,
    GLsizei,
    GLint,
    GLenum,
    GLenum,
    *const void,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(target, level, internalformat, width, border, format, type_, pixels)
}

static glTexImage2D_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glTexImage2D(
  target: GLenum, level: GLint, internalformat: GLint, width: GLsizei,
  height: GLsizei, border: GLint, format: GLenum, type_: GLenum,
  pixels: *const void,
) -> () {
  let u: usize = glTexImage2D_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLenum,
    GLint,
    GLint,
    GLsizei,
    GLsizei,
    GLint,
    GLenum,
    GLenum,
    *const void,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(
    target,
    level,
    internalformat,
    width,
    height,
    border,
    format,
    type_,
    pixels,
  )
}

static glTexImage2DMultisample_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glTexImage2DMultisample(
  target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei,
  height: GLsizei, fixedsamplelocations: GLboolean,
) -> () {
  let u: usize =
    glTexImage2DMultisample_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLenum,
    GLsizei,
    GLenum,
    GLsizei,
    GLsizei,
    GLboolean,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(target, samples, internalformat, width, height, fixedsamplelocations)
}

static glTexImage3D_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glTexImage3D(
  target: GLenum, level: GLint, internalformat: GLint, width: GLsizei,
  height: GLsizei, depth: GLsizei, border: GLint, format: GLenum,
  type_: GLenum, pixels: *const void,
) -> () {
  let u: usize = glTexImage3D_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLenum,
    GLint,
    GLint,
    GLsizei,
    GLsizei,
    GLsizei,
    GLint,
    GLenum,
    GLenum,
    *const void,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(
    target,
    level,
    internalformat,
    width,
    height,
    depth,
    border,
    format,
    type_,
    pixels,
  )
}

static glTexImage3DMultisample_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glTexImage3DMultisample(
  target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei,
  height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean,
) -> () {
  let u: usize =
    glTexImage3DMultisample_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLenum,
    GLsizei,
    GLenum,
    GLsizei,
    GLsizei,
    GLsizei,
    GLboolean,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(
    target,
    samples,
    internalformat,
    width,
    height,
    depth,
    fixedsamplelocations,
  )
}

static glTexParameterIiv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glTexParameterIiv(
  target: GLenum, pname: GLenum, params: *const GLint,
) -> () {
  let u: usize =
    glTexParameterIiv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLenum, GLenum, *const GLint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(target, pname, params)
}

static glTexParameterIuiv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glTexParameterIuiv(
  target: GLenum, pname: GLenum, params: *const GLuint,
) -> () {
  let u: usize =
    glTexParameterIuiv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLenum, GLenum, *const GLuint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(target, pname, params)
}

static glTexParameterf_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glTexParameterf(
  target: GLenum, pname: GLenum, param: GLfloat,
) -> () {
  let u: usize = glTexParameterf_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLenum, GLenum, GLfloat) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(target, pname, param)
}

static glTexParameterfv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glTexParameterfv(
  target: GLenum, pname: GLenum, params: *const GLfloat,
) -> () {
  let u: usize = glTexParameterfv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLenum, GLenum, *const GLfloat) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(target, pname, params)
}

static glTexParameteri_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glTexParameteri(
  target: GLenum, pname: GLenum, param: GLint,
) -> () {
  let u: usize = glTexParameteri_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLenum, GLenum, GLint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(target, pname, param)
}

static glTexParameteriv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glTexParameteriv(
  target: GLenum, pname: GLenum, params: *const GLint,
) -> () {
  let u: usize = glTexParameteriv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLenum, GLenum, *const GLint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(target, pname, params)
}

static glTexSubImage1D_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glTexSubImage1D(
  target: GLenum, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum,
  type_: GLenum, pixels: *const void,
) -> () {
  let u: usize = glTexSubImage1D_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLenum,
    GLint,
    GLint,
    GLsizei,
    GLenum,
    GLenum,
    *const void,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(target, level, xoffset, width, format, type_, pixels)
}

static glTexSubImage2D_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glTexSubImage2D(
  target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei,
  height: GLsizei, format: GLenum, type_: GLenum, pixels: *const void,
) -> () {
  let u: usize = glTexSubImage2D_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLenum,
    GLint,
    GLint,
    GLint,
    GLsizei,
    GLsizei,
    GLenum,
    GLenum,
    *const void,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(target, level, xoffset, yoffset, width, height, format, type_, pixels)
}

static glTexSubImage3D_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glTexSubImage3D(
  target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint,
  width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum,
  type_: GLenum, pixels: *const void,
) -> () {
  let u: usize = glTexSubImage3D_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLenum,
    GLint,
    GLint,
    GLint,
    GLint,
    GLsizei,
    GLsizei,
    GLsizei,
    GLenum,
    GLenum,
    *const void,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(
    target, level, xoffset, yoffset, zoffset, width, height, depth, format,
    type_, pixels,
  )
}

static glTransformFeedbackVaryings_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glTransformFeedbackVaryings(
  program: GLuint, count: GLsizei, varyings: *const *const GLchar,
  bufferMode: GLenum,
) -> () {
  let u: usize =
    glTransformFeedbackVaryings_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLuint,
    GLsizei,
    *const *const GLchar,
    GLenum,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(program, count, varyings, bufferMode)
}

static glUniform1f_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glUniform1f(location: GLint, v0: GLfloat) -> () {
  let u: usize = glUniform1f_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLint, GLfloat) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(location, v0)
}

static glUniform1fv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glUniform1fv(
  location: GLint, count: GLsizei, value: *const GLfloat,
) -> () {
  let u: usize = glUniform1fv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLint, GLsizei, *const GLfloat) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(location, count, value)
}

static glUniform1i_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glUniform1i(location: GLint, v0: GLint) -> () {
  let u: usize = glUniform1i_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLint, GLint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(location, v0)
}

static glUniform1iv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glUniform1iv(
  location: GLint, count: GLsizei, value: *const GLint,
) -> () {
  let u: usize = glUniform1iv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLint, GLsizei, *const GLint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(location, count, value)
}

static glUniform1ui_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glUniform1ui(location: GLint, v0: GLuint) -> () {
  let u: usize = glUniform1ui_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLint, GLuint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(location, v0)
}

static glUniform1uiv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glUniform1uiv(
  location: GLint, count: GLsizei, value: *const GLuint,
) -> () {
  let u: usize = glUniform1uiv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLint, GLsizei, *const GLuint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(location, count, value)
}

static glUniform2f_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glUniform2f(location: GLint, v0: GLfloat, v1: GLfloat) -> () {
  let u: usize = glUniform2f_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLint, GLfloat, GLfloat) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(location, v0, v1)
}

static glUniform2fv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glUniform2fv(
  location: GLint, count: GLsizei, value: *const GLfloat,
) -> () {
  let u: usize = glUniform2fv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLint, GLsizei, *const GLfloat) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(location, count, value)
}

static glUniform2i_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glUniform2i(location: GLint, v0: GLint, v1: GLint) -> () {
  let u: usize = glUniform2i_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLint, GLint, GLint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(location, v0, v1)
}

static glUniform2iv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glUniform2iv(
  location: GLint, count: GLsizei, value: *const GLint,
) -> () {
  let u: usize = glUniform2iv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLint, GLsizei, *const GLint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(location, count, value)
}

static glUniform2ui_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glUniform2ui(location: GLint, v0: GLuint, v1: GLuint) -> () {
  let u: usize = glUniform2ui_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLint, GLuint, GLuint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(location, v0, v1)
}

static glUniform2uiv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glUniform2uiv(
  location: GLint, count: GLsizei, value: *const GLuint,
) -> () {
  let u: usize = glUniform2uiv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLint, GLsizei, *const GLuint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(location, count, value)
}

static glUniform3f_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glUniform3f(
  location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat,
) -> () {
  let u: usize = glUniform3f_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLint,
    GLfloat,
    GLfloat,
    GLfloat,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(location, v0, v1, v2)
}

static glUniform3fv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glUniform3fv(
  location: GLint, count: GLsizei, value: *const GLfloat,
) -> () {
  let u: usize = glUniform3fv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLint, GLsizei, *const GLfloat) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(location, count, value)
}

static glUniform3i_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glUniform3i(
  location: GLint, v0: GLint, v1: GLint, v2: GLint,
) -> () {
  let u: usize = glUniform3i_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLint, GLint, GLint, GLint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(location, v0, v1, v2)
}

static glUniform3iv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glUniform3iv(
  location: GLint, count: GLsizei, value: *const GLint,
) -> () {
  let u: usize = glUniform3iv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLint, GLsizei, *const GLint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(location, count, value)
}

static glUniform3ui_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glUniform3ui(
  location: GLint, v0: GLuint, v1: GLuint, v2: GLuint,
) -> () {
  let u: usize = glUniform3ui_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLint, GLuint, GLuint, GLuint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(location, v0, v1, v2)
}

static glUniform3uiv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glUniform3uiv(
  location: GLint, count: GLsizei, value: *const GLuint,
) -> () {
  let u: usize = glUniform3uiv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLint, GLsizei, *const GLuint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(location, count, value)
}

static glUniform4f_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glUniform4f(
  location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat,
) -> () {
  let u: usize = glUniform4f_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLint,
    GLfloat,
    GLfloat,
    GLfloat,
    GLfloat,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(location, v0, v1, v2, v3)
}

static glUniform4fv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glUniform4fv(
  location: GLint, count: GLsizei, value: *const GLfloat,
) -> () {
  let u: usize = glUniform4fv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLint, GLsizei, *const GLfloat) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(location, count, value)
}

static glUniform4i_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glUniform4i(
  location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint,
) -> () {
  let u: usize = glUniform4i_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLint,
    GLint,
    GLint,
    GLint,
    GLint,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(location, v0, v1, v2, v3)
}

static glUniform4iv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glUniform4iv(
  location: GLint, count: GLsizei, value: *const GLint,
) -> () {
  let u: usize = glUniform4iv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLint, GLsizei, *const GLint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(location, count, value)
}

static glUniform4ui_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glUniform4ui(
  location: GLint, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint,
) -> () {
  let u: usize = glUniform4ui_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLint,
    GLuint,
    GLuint,
    GLuint,
    GLuint,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(location, v0, v1, v2, v3)
}

static glUniform4uiv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glUniform4uiv(
  location: GLint, count: GLsizei, value: *const GLuint,
) -> () {
  let u: usize = glUniform4uiv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLint, GLsizei, *const GLuint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(location, count, value)
}

static glUniformBlockBinding_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glUniformBlockBinding(
  program: GLuint, uniformBlockIndex: GLuint, uniformBlockBinding: GLuint,
) -> () {
  let u: usize =
    glUniformBlockBinding_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint, GLuint, GLuint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(program, uniformBlockIndex, uniformBlockBinding)
}

static glUniformMatrix2fv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glUniformMatrix2fv(
  location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat,
) -> () {
  let u: usize =
    glUniformMatrix2fv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLint,
    GLsizei,
    GLboolean,
    *const GLfloat,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(location, count, transpose, value)
}

static glUniformMatrix2x3fv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glUniformMatrix2x3fv(
  location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat,
) -> () {
  let u: usize =
    glUniformMatrix2x3fv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLint,
    GLsizei,
    GLboolean,
    *const GLfloat,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(location, count, transpose, value)
}

static glUniformMatrix2x4fv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glUniformMatrix2x4fv(
  location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat,
) -> () {
  let u: usize =
    glUniformMatrix2x4fv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLint,
    GLsizei,
    GLboolean,
    *const GLfloat,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(location, count, transpose, value)
}

static glUniformMatrix3fv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glUniformMatrix3fv(
  location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat,
) -> () {
  let u: usize =
    glUniformMatrix3fv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLint,
    GLsizei,
    GLboolean,
    *const GLfloat,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(location, count, transpose, value)
}

static glUniformMatrix3x2fv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glUniformMatrix3x2fv(
  location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat,
) -> () {
  let u: usize =
    glUniformMatrix3x2fv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLint,
    GLsizei,
    GLboolean,
    *const GLfloat,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(location, count, transpose, value)
}

static glUniformMatrix3x4fv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glUniformMatrix3x4fv(
  location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat,
) -> () {
  let u: usize =
    glUniformMatrix3x4fv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLint,
    GLsizei,
    GLboolean,
    *const GLfloat,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(location, count, transpose, value)
}

static glUniformMatrix4fv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glUniformMatrix4fv(
  location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat,
) -> () {
  let u: usize =
    glUniformMatrix4fv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLint,
    GLsizei,
    GLboolean,
    *const GLfloat,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(location, count, transpose, value)
}

static glUniformMatrix4x2fv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glUniformMatrix4x2fv(
  location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat,
) -> () {
  let u: usize =
    glUniformMatrix4x2fv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLint,
    GLsizei,
    GLboolean,
    *const GLfloat,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(location, count, transpose, value)
}

static glUniformMatrix4x3fv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glUniformMatrix4x3fv(
  location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat,
) -> () {
  let u: usize =
    glUniformMatrix4x3fv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLint,
    GLsizei,
    GLboolean,
    *const GLfloat,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(location, count, transpose, value)
}

static glUnmapBuffer_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glUnmapBuffer(target: GLenum) -> GLboolean {
  let u: usize = glUnmapBuffer_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLenum) -> GLboolean =
    unsafe { core::mem::transmute(u) };
  _func_p(target)
}

static glUseProgram_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glUseProgram(program: GLuint) -> () {
  let u: usize = glUseProgram_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(program)
}

static glValidateProgram_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glValidateProgram(program: GLuint) -> () {
  let u: usize =
    glValidateProgram_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(program)
}

static glVertexAttrib1d_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glVertexAttrib1d(index: GLuint, x: GLdouble) -> () {
  let u: usize = glVertexAttrib1d_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint, GLdouble) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(index, x)
}

static glVertexAttrib1dv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glVertexAttrib1dv(index: GLuint, v: *const GLdouble) -> () {
  let u: usize =
    glVertexAttrib1dv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint, *const GLdouble) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(index, v)
}

static glVertexAttrib1f_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glVertexAttrib1f(index: GLuint, x: GLfloat) -> () {
  let u: usize = glVertexAttrib1f_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint, GLfloat) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(index, x)
}

static glVertexAttrib1fv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glVertexAttrib1fv(index: GLuint, v: *const GLfloat) -> () {
  let u: usize =
    glVertexAttrib1fv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint, *const GLfloat) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(index, v)
}

static glVertexAttrib1s_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glVertexAttrib1s(index: GLuint, x: GLshort) -> () {
  let u: usize = glVertexAttrib1s_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint, GLshort) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(index, x)
}

static glVertexAttrib1sv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glVertexAttrib1sv(index: GLuint, v: *const GLshort) -> () {
  let u: usize =
    glVertexAttrib1sv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint, *const GLshort) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(index, v)
}

static glVertexAttrib2d_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glVertexAttrib2d(index: GLuint, x: GLdouble, y: GLdouble) -> () {
  let u: usize = glVertexAttrib2d_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint, GLdouble, GLdouble) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(index, x, y)
}

static glVertexAttrib2dv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glVertexAttrib2dv(index: GLuint, v: *const GLdouble) -> () {
  let u: usize =
    glVertexAttrib2dv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint, *const GLdouble) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(index, v)
}

static glVertexAttrib2f_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glVertexAttrib2f(index: GLuint, x: GLfloat, y: GLfloat) -> () {
  let u: usize = glVertexAttrib2f_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint, GLfloat, GLfloat) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(index, x, y)
}

static glVertexAttrib2fv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glVertexAttrib2fv(index: GLuint, v: *const GLfloat) -> () {
  let u: usize =
    glVertexAttrib2fv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint, *const GLfloat) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(index, v)
}

static glVertexAttrib2s_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glVertexAttrib2s(index: GLuint, x: GLshort, y: GLshort) -> () {
  let u: usize = glVertexAttrib2s_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint, GLshort, GLshort) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(index, x, y)
}

static glVertexAttrib2sv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glVertexAttrib2sv(index: GLuint, v: *const GLshort) -> () {
  let u: usize =
    glVertexAttrib2sv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint, *const GLshort) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(index, v)
}

static glVertexAttrib3d_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glVertexAttrib3d(
  index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble,
) -> () {
  let u: usize = glVertexAttrib3d_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLuint,
    GLdouble,
    GLdouble,
    GLdouble,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(index, x, y, z)
}

static glVertexAttrib3dv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glVertexAttrib3dv(index: GLuint, v: *const GLdouble) -> () {
  let u: usize =
    glVertexAttrib3dv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint, *const GLdouble) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(index, v)
}

static glVertexAttrib3f_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glVertexAttrib3f(
  index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat,
) -> () {
  let u: usize = glVertexAttrib3f_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLuint,
    GLfloat,
    GLfloat,
    GLfloat,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(index, x, y, z)
}

static glVertexAttrib3fv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glVertexAttrib3fv(index: GLuint, v: *const GLfloat) -> () {
  let u: usize =
    glVertexAttrib3fv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint, *const GLfloat) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(index, v)
}

static glVertexAttrib3s_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glVertexAttrib3s(
  index: GLuint, x: GLshort, y: GLshort, z: GLshort,
) -> () {
  let u: usize = glVertexAttrib3s_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLuint,
    GLshort,
    GLshort,
    GLshort,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(index, x, y, z)
}

static glVertexAttrib3sv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glVertexAttrib3sv(index: GLuint, v: *const GLshort) -> () {
  let u: usize =
    glVertexAttrib3sv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint, *const GLshort) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(index, v)
}

static glVertexAttrib4Nbv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glVertexAttrib4Nbv(index: GLuint, v: *const GLbyte) -> () {
  let u: usize =
    glVertexAttrib4Nbv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint, *const GLbyte) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(index, v)
}

static glVertexAttrib4Niv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glVertexAttrib4Niv(index: GLuint, v: *const GLint) -> () {
  let u: usize =
    glVertexAttrib4Niv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint, *const GLint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(index, v)
}

static glVertexAttrib4Nsv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glVertexAttrib4Nsv(index: GLuint, v: *const GLshort) -> () {
  let u: usize =
    glVertexAttrib4Nsv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint, *const GLshort) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(index, v)
}

static glVertexAttrib4Nub_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glVertexAttrib4Nub(
  index: GLuint, x: GLubyte, y: GLubyte, z: GLubyte, w: GLubyte,
) -> () {
  let u: usize =
    glVertexAttrib4Nub_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLuint,
    GLubyte,
    GLubyte,
    GLubyte,
    GLubyte,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(index, x, y, z, w)
}

static glVertexAttrib4Nubv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glVertexAttrib4Nubv(index: GLuint, v: *const GLubyte) -> () {
  let u: usize =
    glVertexAttrib4Nubv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint, *const GLubyte) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(index, v)
}

static glVertexAttrib4Nuiv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glVertexAttrib4Nuiv(index: GLuint, v: *const GLuint) -> () {
  let u: usize =
    glVertexAttrib4Nuiv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint, *const GLuint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(index, v)
}

static glVertexAttrib4Nusv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glVertexAttrib4Nusv(index: GLuint, v: *const GLushort) -> () {
  let u: usize =
    glVertexAttrib4Nusv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint, *const GLushort) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(index, v)
}

static glVertexAttrib4bv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glVertexAttrib4bv(index: GLuint, v: *const GLbyte) -> () {
  let u: usize =
    glVertexAttrib4bv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint, *const GLbyte) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(index, v)
}

static glVertexAttrib4d_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glVertexAttrib4d(
  index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble,
) -> () {
  let u: usize = glVertexAttrib4d_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLuint,
    GLdouble,
    GLdouble,
    GLdouble,
    GLdouble,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(index, x, y, z, w)
}

static glVertexAttrib4dv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glVertexAttrib4dv(index: GLuint, v: *const GLdouble) -> () {
  let u: usize =
    glVertexAttrib4dv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint, *const GLdouble) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(index, v)
}

static glVertexAttrib4f_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glVertexAttrib4f(
  index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat,
) -> () {
  let u: usize = glVertexAttrib4f_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLuint,
    GLfloat,
    GLfloat,
    GLfloat,
    GLfloat,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(index, x, y, z, w)
}

static glVertexAttrib4fv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glVertexAttrib4fv(index: GLuint, v: *const GLfloat) -> () {
  let u: usize =
    glVertexAttrib4fv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint, *const GLfloat) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(index, v)
}

static glVertexAttrib4iv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glVertexAttrib4iv(index: GLuint, v: *const GLint) -> () {
  let u: usize =
    glVertexAttrib4iv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint, *const GLint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(index, v)
}

static glVertexAttrib4s_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glVertexAttrib4s(
  index: GLuint, x: GLshort, y: GLshort, z: GLshort, w: GLshort,
) -> () {
  let u: usize = glVertexAttrib4s_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLuint,
    GLshort,
    GLshort,
    GLshort,
    GLshort,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(index, x, y, z, w)
}

static glVertexAttrib4sv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glVertexAttrib4sv(index: GLuint, v: *const GLshort) -> () {
  let u: usize =
    glVertexAttrib4sv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint, *const GLshort) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(index, v)
}

static glVertexAttrib4ubv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glVertexAttrib4ubv(index: GLuint, v: *const GLubyte) -> () {
  let u: usize =
    glVertexAttrib4ubv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint, *const GLubyte) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(index, v)
}

static glVertexAttrib4uiv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glVertexAttrib4uiv(index: GLuint, v: *const GLuint) -> () {
  let u: usize =
    glVertexAttrib4uiv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint, *const GLuint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(index, v)
}

static glVertexAttrib4usv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glVertexAttrib4usv(index: GLuint, v: *const GLushort) -> () {
  let u: usize =
    glVertexAttrib4usv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint, *const GLushort) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(index, v)
}

static glVertexAttribDivisor_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glVertexAttribDivisor(index: GLuint, divisor: GLuint) -> () {
  let u: usize =
    glVertexAttribDivisor_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint, GLuint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(index, divisor)
}

static glVertexAttribI1i_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glVertexAttribI1i(index: GLuint, x: GLint) -> () {
  let u: usize =
    glVertexAttribI1i_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint, GLint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(index, x)
}

static glVertexAttribI1iv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glVertexAttribI1iv(index: GLuint, v: *const GLint) -> () {
  let u: usize =
    glVertexAttribI1iv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint, *const GLint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(index, v)
}

static glVertexAttribI1ui_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glVertexAttribI1ui(index: GLuint, x: GLuint) -> () {
  let u: usize =
    glVertexAttribI1ui_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint, GLuint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(index, x)
}

static glVertexAttribI1uiv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glVertexAttribI1uiv(index: GLuint, v: *const GLuint) -> () {
  let u: usize =
    glVertexAttribI1uiv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint, *const GLuint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(index, v)
}

static glVertexAttribI2i_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glVertexAttribI2i(index: GLuint, x: GLint, y: GLint) -> () {
  let u: usize =
    glVertexAttribI2i_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint, GLint, GLint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(index, x, y)
}

static glVertexAttribI2iv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glVertexAttribI2iv(index: GLuint, v: *const GLint) -> () {
  let u: usize =
    glVertexAttribI2iv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint, *const GLint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(index, v)
}

static glVertexAttribI2ui_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glVertexAttribI2ui(index: GLuint, x: GLuint, y: GLuint) -> () {
  let u: usize =
    glVertexAttribI2ui_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint, GLuint, GLuint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(index, x, y)
}

static glVertexAttribI2uiv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glVertexAttribI2uiv(index: GLuint, v: *const GLuint) -> () {
  let u: usize =
    glVertexAttribI2uiv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint, *const GLuint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(index, v)
}

static glVertexAttribI3i_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glVertexAttribI3i(
  index: GLuint, x: GLint, y: GLint, z: GLint,
) -> () {
  let u: usize =
    glVertexAttribI3i_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint, GLint, GLint, GLint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(index, x, y, z)
}

static glVertexAttribI3iv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glVertexAttribI3iv(index: GLuint, v: *const GLint) -> () {
  let u: usize =
    glVertexAttribI3iv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint, *const GLint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(index, v)
}

static glVertexAttribI3ui_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glVertexAttribI3ui(
  index: GLuint, x: GLuint, y: GLuint, z: GLuint,
) -> () {
  let u: usize =
    glVertexAttribI3ui_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint, GLuint, GLuint, GLuint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(index, x, y, z)
}

static glVertexAttribI3uiv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glVertexAttribI3uiv(index: GLuint, v: *const GLuint) -> () {
  let u: usize =
    glVertexAttribI3uiv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint, *const GLuint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(index, v)
}

static glVertexAttribI4bv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glVertexAttribI4bv(index: GLuint, v: *const GLbyte) -> () {
  let u: usize =
    glVertexAttribI4bv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint, *const GLbyte) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(index, v)
}

static glVertexAttribI4i_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glVertexAttribI4i(
  index: GLuint, x: GLint, y: GLint, z: GLint, w: GLint,
) -> () {
  let u: usize =
    glVertexAttribI4i_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLuint,
    GLint,
    GLint,
    GLint,
    GLint,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(index, x, y, z, w)
}

static glVertexAttribI4iv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glVertexAttribI4iv(index: GLuint, v: *const GLint) -> () {
  let u: usize =
    glVertexAttribI4iv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint, *const GLint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(index, v)
}

static glVertexAttribI4sv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glVertexAttribI4sv(index: GLuint, v: *const GLshort) -> () {
  let u: usize =
    glVertexAttribI4sv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint, *const GLshort) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(index, v)
}

static glVertexAttribI4ubv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glVertexAttribI4ubv(index: GLuint, v: *const GLubyte) -> () {
  let u: usize =
    glVertexAttribI4ubv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint, *const GLubyte) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(index, v)
}

static glVertexAttribI4ui_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glVertexAttribI4ui(
  index: GLuint, x: GLuint, y: GLuint, z: GLuint, w: GLuint,
) -> () {
  let u: usize =
    glVertexAttribI4ui_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLuint,
    GLuint,
    GLuint,
    GLuint,
    GLuint,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(index, x, y, z, w)
}

static glVertexAttribI4uiv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glVertexAttribI4uiv(index: GLuint, v: *const GLuint) -> () {
  let u: usize =
    glVertexAttribI4uiv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint, *const GLuint) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(index, v)
}

static glVertexAttribI4usv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glVertexAttribI4usv(index: GLuint, v: *const GLushort) -> () {
  let u: usize =
    glVertexAttribI4usv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLuint, *const GLushort) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(index, v)
}

static glVertexAttribIPointer_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glVertexAttribIPointer(
  index: GLuint, size: GLint, type_: GLenum, stride: GLsizei,
  pointer: *const void,
) -> () {
  let u: usize =
    glVertexAttribIPointer_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLuint,
    GLint,
    GLenum,
    GLsizei,
    *const void,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(index, size, type_, stride, pointer)
}

static glVertexAttribP1ui_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glVertexAttribP1ui(
  index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint,
) -> () {
  let u: usize =
    glVertexAttribP1ui_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLuint,
    GLenum,
    GLboolean,
    GLuint,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(index, type_, normalized, value)
}

static glVertexAttribP1uiv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glVertexAttribP1uiv(
  index: GLuint, type_: GLenum, normalized: GLboolean, value: *const GLuint,
) -> () {
  let u: usize =
    glVertexAttribP1uiv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLuint,
    GLenum,
    GLboolean,
    *const GLuint,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(index, type_, normalized, value)
}

static glVertexAttribP2ui_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glVertexAttribP2ui(
  index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint,
) -> () {
  let u: usize =
    glVertexAttribP2ui_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLuint,
    GLenum,
    GLboolean,
    GLuint,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(index, type_, normalized, value)
}

static glVertexAttribP2uiv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glVertexAttribP2uiv(
  index: GLuint, type_: GLenum, normalized: GLboolean, value: *const GLuint,
) -> () {
  let u: usize =
    glVertexAttribP2uiv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLuint,
    GLenum,
    GLboolean,
    *const GLuint,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(index, type_, normalized, value)
}

static glVertexAttribP3ui_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glVertexAttribP3ui(
  index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint,
) -> () {
  let u: usize =
    glVertexAttribP3ui_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLuint,
    GLenum,
    GLboolean,
    GLuint,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(index, type_, normalized, value)
}

static glVertexAttribP3uiv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glVertexAttribP3uiv(
  index: GLuint, type_: GLenum, normalized: GLboolean, value: *const GLuint,
) -> () {
  let u: usize =
    glVertexAttribP3uiv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLuint,
    GLenum,
    GLboolean,
    *const GLuint,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(index, type_, normalized, value)
}

static glVertexAttribP4ui_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glVertexAttribP4ui(
  index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint,
) -> () {
  let u: usize =
    glVertexAttribP4ui_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLuint,
    GLenum,
    GLboolean,
    GLuint,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(index, type_, normalized, value)
}

static glVertexAttribP4uiv_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glVertexAttribP4uiv(
  index: GLuint, type_: GLenum, normalized: GLboolean, value: *const GLuint,
) -> () {
  let u: usize =
    glVertexAttribP4uiv_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLuint,
    GLenum,
    GLboolean,
    *const GLuint,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(index, type_, normalized, value)
}

static glVertexAttribPointer_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glVertexAttribPointer(
  index: GLuint, size: GLint, type_: GLenum, normalized: GLboolean,
  stride: GLsizei, pointer: *const void,
) -> () {
  let u: usize =
    glVertexAttribPointer_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(
    GLuint,
    GLint,
    GLenum,
    GLboolean,
    GLsizei,
    *const void,
  ) -> () = unsafe { core::mem::transmute(u) };
  _func_p(index, size, type_, normalized, stride, pointer)
}

static glViewport_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glViewport(
  x: GLint, y: GLint, width: GLsizei, height: GLsizei,
) -> () {
  let u: usize = glViewport_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLint, GLint, GLsizei, GLsizei) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(x, y, width, height)
}

static glWaitSync_p: core::sync::atomic::AtomicUsize =
  core::sync::atomic::AtomicUsize::new(0);
#[inline]
pub unsafe fn glWaitSync(
  sync: GLsync, flags: GLbitfield, timeout: GLuint64,
) -> () {
  let u: usize = glWaitSync_p.load(core::sync::atomic::Ordering::Acquire);
  assert!(u != 0);
  let _func_p: unsafe extern "system" fn(GLsync, GLbitfield, GLuint64) -> () =
    unsafe { core::mem::transmute(u) };
  _func_p(sync, flags, timeout)
}

pub unsafe fn load_gl_functions(
  load_fn: &dyn Fn(*const u8) -> *const void,
) -> Result<(), &'static str> {
  let command_info = [
    ("glActiveTexture\0", &glActiveTexture_p),
    ("glAttachShader\0", &glAttachShader_p),
    ("glBeginConditionalRender\0", &glBeginConditionalRender_p),
    ("glBeginQuery\0", &glBeginQuery_p),
    ("glBeginTransformFeedback\0", &glBeginTransformFeedback_p),
    ("glBindAttribLocation\0", &glBindAttribLocation_p),
    ("glBindBuffer\0", &glBindBuffer_p),
    ("glBindBufferBase\0", &glBindBufferBase_p),
    ("glBindBufferRange\0", &glBindBufferRange_p),
    ("glBindFragDataLocation\0", &glBindFragDataLocation_p),
    ("glBindFragDataLocationIndexed\0", &glBindFragDataLocationIndexed_p),
    ("glBindFramebuffer\0", &glBindFramebuffer_p),
    ("glBindRenderbuffer\0", &glBindRenderbuffer_p),
    ("glBindSampler\0", &glBindSampler_p),
    ("glBindTexture\0", &glBindTexture_p),
    ("glBindVertexArray\0", &glBindVertexArray_p),
    ("glBlendColor\0", &glBlendColor_p),
    ("glBlendEquation\0", &glBlendEquation_p),
    ("glBlendEquationSeparate\0", &glBlendEquationSeparate_p),
    ("glBlendFunc\0", &glBlendFunc_p),
    ("glBlendFuncSeparate\0", &glBlendFuncSeparate_p),
    ("glBlitFramebuffer\0", &glBlitFramebuffer_p),
    ("glBufferData\0", &glBufferData_p),
    ("glBufferSubData\0", &glBufferSubData_p),
    ("glCheckFramebufferStatus\0", &glCheckFramebufferStatus_p),
    ("glClampColor\0", &glClampColor_p),
    ("glClear\0", &glClear_p),
    ("glClearBufferfi\0", &glClearBufferfi_p),
    ("glClearBufferfv\0", &glClearBufferfv_p),
    ("glClearBufferiv\0", &glClearBufferiv_p),
    ("glClearBufferuiv\0", &glClearBufferuiv_p),
    ("glClearColor\0", &glClearColor_p),
    ("glClearDepth\0", &glClearDepth_p),
    ("glClearStencil\0", &glClearStencil_p),
    ("glClientWaitSync\0", &glClientWaitSync_p),
    ("glColorMask\0", &glColorMask_p),
    ("glColorMaski\0", &glColorMaski_p),
    ("glCompileShader\0", &glCompileShader_p),
    ("glCompressedTexImage1D\0", &glCompressedTexImage1D_p),
    ("glCompressedTexImage2D\0", &glCompressedTexImage2D_p),
    ("glCompressedTexImage3D\0", &glCompressedTexImage3D_p),
    ("glCompressedTexSubImage1D\0", &glCompressedTexSubImage1D_p),
    ("glCompressedTexSubImage2D\0", &glCompressedTexSubImage2D_p),
    ("glCompressedTexSubImage3D\0", &glCompressedTexSubImage3D_p),
    ("glCopyBufferSubData\0", &glCopyBufferSubData_p),
    ("glCopyTexImage1D\0", &glCopyTexImage1D_p),
    ("glCopyTexImage2D\0", &glCopyTexImage2D_p),
    ("glCopyTexSubImage1D\0", &glCopyTexSubImage1D_p),
    ("glCopyTexSubImage2D\0", &glCopyTexSubImage2D_p),
    ("glCopyTexSubImage3D\0", &glCopyTexSubImage3D_p),
    ("glCreateProgram\0", &glCreateProgram_p),
    ("glCreateShader\0", &glCreateShader_p),
    ("glCullFace\0", &glCullFace_p),
    ("glDebugMessageCallback\0", &glDebugMessageCallback_p),
    ("glDebugMessageControl\0", &glDebugMessageControl_p),
    ("glDebugMessageInsert\0", &glDebugMessageInsert_p),
    ("glDeleteBuffers\0", &glDeleteBuffers_p),
    ("glDeleteFramebuffers\0", &glDeleteFramebuffers_p),
    ("glDeleteProgram\0", &glDeleteProgram_p),
    ("glDeleteQueries\0", &glDeleteQueries_p),
    ("glDeleteRenderbuffers\0", &glDeleteRenderbuffers_p),
    ("glDeleteSamplers\0", &glDeleteSamplers_p),
    ("glDeleteShader\0", &glDeleteShader_p),
    ("glDeleteSync\0", &glDeleteSync_p),
    ("glDeleteTextures\0", &glDeleteTextures_p),
    ("glDeleteVertexArrays\0", &glDeleteVertexArrays_p),
    ("glDepthFunc\0", &glDepthFunc_p),
    ("glDepthMask\0", &glDepthMask_p),
    ("glDepthRange\0", &glDepthRange_p),
    ("glDetachShader\0", &glDetachShader_p),
    ("glDisable\0", &glDisable_p),
    ("glDisableVertexAttribArray\0", &glDisableVertexAttribArray_p),
    ("glDisablei\0", &glDisablei_p),
    ("glDrawArrays\0", &glDrawArrays_p),
    ("glDrawArraysInstanced\0", &glDrawArraysInstanced_p),
    ("glDrawBuffer\0", &glDrawBuffer_p),
    ("glDrawBuffers\0", &glDrawBuffers_p),
    ("glDrawElements\0", &glDrawElements_p),
    ("glDrawElementsBaseVertex\0", &glDrawElementsBaseVertex_p),
    ("glDrawElementsInstanced\0", &glDrawElementsInstanced_p),
    (
      "glDrawElementsInstancedBaseVertex\0",
      &glDrawElementsInstancedBaseVertex_p,
    ),
    ("glDrawRangeElements\0", &glDrawRangeElements_p),
    ("glDrawRangeElementsBaseVertex\0", &glDrawRangeElementsBaseVertex_p),
    ("glEnable\0", &glEnable_p),
    ("glEnableVertexAttribArray\0", &glEnableVertexAttribArray_p),
    ("glEnablei\0", &glEnablei_p),
    ("glEndConditionalRender\0", &glEndConditionalRender_p),
    ("glEndQuery\0", &glEndQuery_p),
    ("glEndTransformFeedback\0", &glEndTransformFeedback_p),
    ("glFenceSync\0", &glFenceSync_p),
    ("glFinish\0", &glFinish_p),
    ("glFlush\0", &glFlush_p),
    ("glFlushMappedBufferRange\0", &glFlushMappedBufferRange_p),
    ("glFramebufferRenderbuffer\0", &glFramebufferRenderbuffer_p),
    ("glFramebufferTexture\0", &glFramebufferTexture_p),
    ("glFramebufferTexture1D\0", &glFramebufferTexture1D_p),
    ("glFramebufferTexture2D\0", &glFramebufferTexture2D_p),
    ("glFramebufferTexture3D\0", &glFramebufferTexture3D_p),
    ("glFramebufferTextureLayer\0", &glFramebufferTextureLayer_p),
    ("glFrontFace\0", &glFrontFace_p),
    ("glGenBuffers\0", &glGenBuffers_p),
    ("glGenFramebuffers\0", &glGenFramebuffers_p),
    ("glGenQueries\0", &glGenQueries_p),
    ("glGenRenderbuffers\0", &glGenRenderbuffers_p),
    ("glGenSamplers\0", &glGenSamplers_p),
    ("glGenTextures\0", &glGenTextures_p),
    ("glGenVertexArrays\0", &glGenVertexArrays_p),
    ("glGenerateMipmap\0", &glGenerateMipmap_p),
    ("glGetActiveAttrib\0", &glGetActiveAttrib_p),
    ("glGetActiveUniform\0", &glGetActiveUniform_p),
    ("glGetActiveUniformBlockName\0", &glGetActiveUniformBlockName_p),
    ("glGetActiveUniformBlockiv\0", &glGetActiveUniformBlockiv_p),
    ("glGetActiveUniformName\0", &glGetActiveUniformName_p),
    ("glGetActiveUniformsiv\0", &glGetActiveUniformsiv_p),
    ("glGetAttachedShaders\0", &glGetAttachedShaders_p),
    ("glGetAttribLocation\0", &glGetAttribLocation_p),
    ("glGetBooleani_v\0", &glGetBooleani_v_p),
    ("glGetBooleanv\0", &glGetBooleanv_p),
    ("glGetBufferParameteri64v\0", &glGetBufferParameteri64v_p),
    ("glGetBufferParameteriv\0", &glGetBufferParameteriv_p),
    ("glGetBufferPointerv\0", &glGetBufferPointerv_p),
    ("glGetBufferSubData\0", &glGetBufferSubData_p),
    ("glGetCompressedTexImage\0", &glGetCompressedTexImage_p),
    ("glGetDebugMessageLog\0", &glGetDebugMessageLog_p),
    ("glGetDoublev\0", &glGetDoublev_p),
    ("glGetError\0", &glGetError_p),
    ("glGetFloatv\0", &glGetFloatv_p),
    ("glGetFragDataIndex\0", &glGetFragDataIndex_p),
    ("glGetFragDataLocation\0", &glGetFragDataLocation_p),
    (
      "glGetFramebufferAttachmentParameteriv\0",
      &glGetFramebufferAttachmentParameteriv_p,
    ),
    ("glGetInteger64i_v\0", &glGetInteger64i_v_p),
    ("glGetInteger64v\0", &glGetInteger64v_p),
    ("glGetIntegeri_v\0", &glGetIntegeri_v_p),
    ("glGetIntegerv\0", &glGetIntegerv_p),
    ("glGetMultisamplefv\0", &glGetMultisamplefv_p),
    ("glGetObjectLabel\0", &glGetObjectLabel_p),
    ("glGetObjectPtrLabel\0", &glGetObjectPtrLabel_p),
    ("glGetPointerv\0", &glGetPointerv_p),
    ("glGetProgramInfoLog\0", &glGetProgramInfoLog_p),
    ("glGetProgramiv\0", &glGetProgramiv_p),
    ("glGetQueryObjecti64v\0", &glGetQueryObjecti64v_p),
    ("glGetQueryObjectiv\0", &glGetQueryObjectiv_p),
    ("glGetQueryObjectui64v\0", &glGetQueryObjectui64v_p),
    ("glGetQueryObjectuiv\0", &glGetQueryObjectuiv_p),
    ("glGetQueryiv\0", &glGetQueryiv_p),
    ("glGetRenderbufferParameteriv\0", &glGetRenderbufferParameteriv_p),
    ("glGetSamplerParameterIiv\0", &glGetSamplerParameterIiv_p),
    ("glGetSamplerParameterIuiv\0", &glGetSamplerParameterIuiv_p),
    ("glGetSamplerParameterfv\0", &glGetSamplerParameterfv_p),
    ("glGetSamplerParameteriv\0", &glGetSamplerParameteriv_p),
    ("glGetShaderInfoLog\0", &glGetShaderInfoLog_p),
    ("glGetShaderSource\0", &glGetShaderSource_p),
    ("glGetShaderiv\0", &glGetShaderiv_p),
    ("glGetString\0", &glGetString_p),
    ("glGetStringi\0", &glGetStringi_p),
    ("glGetSynciv\0", &glGetSynciv_p),
    ("glGetTexImage\0", &glGetTexImage_p),
    ("glGetTexLevelParameterfv\0", &glGetTexLevelParameterfv_p),
    ("glGetTexLevelParameteriv\0", &glGetTexLevelParameteriv_p),
    ("glGetTexParameterIiv\0", &glGetTexParameterIiv_p),
    ("glGetTexParameterIuiv\0", &glGetTexParameterIuiv_p),
    ("glGetTexParameterfv\0", &glGetTexParameterfv_p),
    ("glGetTexParameteriv\0", &glGetTexParameteriv_p),
    ("glGetTransformFeedbackVarying\0", &glGetTransformFeedbackVarying_p),
    ("glGetUniformBlockIndex\0", &glGetUniformBlockIndex_p),
    ("glGetUniformIndices\0", &glGetUniformIndices_p),
    ("glGetUniformLocation\0", &glGetUniformLocation_p),
    ("glGetUniformfv\0", &glGetUniformfv_p),
    ("glGetUniformiv\0", &glGetUniformiv_p),
    ("glGetUniformuiv\0", &glGetUniformuiv_p),
    ("glGetVertexAttribIiv\0", &glGetVertexAttribIiv_p),
    ("glGetVertexAttribIuiv\0", &glGetVertexAttribIuiv_p),
    ("glGetVertexAttribPointerv\0", &glGetVertexAttribPointerv_p),
    ("glGetVertexAttribdv\0", &glGetVertexAttribdv_p),
    ("glGetVertexAttribfv\0", &glGetVertexAttribfv_p),
    ("glGetVertexAttribiv\0", &glGetVertexAttribiv_p),
    ("glHint\0", &glHint_p),
    ("glIsBuffer\0", &glIsBuffer_p),
    ("glIsEnabled\0", &glIsEnabled_p),
    ("glIsEnabledi\0", &glIsEnabledi_p),
    ("glIsFramebuffer\0", &glIsFramebuffer_p),
    ("glIsProgram\0", &glIsProgram_p),
    ("glIsQuery\0", &glIsQuery_p),
    ("glIsRenderbuffer\0", &glIsRenderbuffer_p),
    ("glIsSampler\0", &glIsSampler_p),
    ("glIsShader\0", &glIsShader_p),
    ("glIsSync\0", &glIsSync_p),
    ("glIsTexture\0", &glIsTexture_p),
    ("glIsVertexArray\0", &glIsVertexArray_p),
    ("glLineWidth\0", &glLineWidth_p),
    ("glLinkProgram\0", &glLinkProgram_p),
    ("glLogicOp\0", &glLogicOp_p),
    ("glMapBuffer\0", &glMapBuffer_p),
    ("glMapBufferRange\0", &glMapBufferRange_p),
    ("glMultiDrawArrays\0", &glMultiDrawArrays_p),
    ("glMultiDrawElements\0", &glMultiDrawElements_p),
    ("glMultiDrawElementsBaseVertex\0", &glMultiDrawElementsBaseVertex_p),
    ("glObjectLabel\0", &glObjectLabel_p),
    ("glObjectPtrLabel\0", &glObjectPtrLabel_p),
    ("glPixelStoref\0", &glPixelStoref_p),
    ("glPixelStorei\0", &glPixelStorei_p),
    ("glPointParameterf\0", &glPointParameterf_p),
    ("glPointParameterfv\0", &glPointParameterfv_p),
    ("glPointParameteri\0", &glPointParameteri_p),
    ("glPointParameteriv\0", &glPointParameteriv_p),
    ("glPointSize\0", &glPointSize_p),
    ("glPolygonMode\0", &glPolygonMode_p),
    ("glPolygonOffset\0", &glPolygonOffset_p),
    ("glPopDebugGroup\0", &glPopDebugGroup_p),
    ("glPrimitiveRestartIndex\0", &glPrimitiveRestartIndex_p),
    ("glProvokingVertex\0", &glProvokingVertex_p),
    ("glPushDebugGroup\0", &glPushDebugGroup_p),
    ("glQueryCounter\0", &glQueryCounter_p),
    ("glReadBuffer\0", &glReadBuffer_p),
    ("glReadPixels\0", &glReadPixels_p),
    ("glRenderbufferStorage\0", &glRenderbufferStorage_p),
    ("glRenderbufferStorageMultisample\0", &glRenderbufferStorageMultisample_p),
    ("glSampleCoverage\0", &glSampleCoverage_p),
    ("glSampleMaski\0", &glSampleMaski_p),
    ("glSamplerParameterIiv\0", &glSamplerParameterIiv_p),
    ("glSamplerParameterIuiv\0", &glSamplerParameterIuiv_p),
    ("glSamplerParameterf\0", &glSamplerParameterf_p),
    ("glSamplerParameterfv\0", &glSamplerParameterfv_p),
    ("glSamplerParameteri\0", &glSamplerParameteri_p),
    ("glSamplerParameteriv\0", &glSamplerParameteriv_p),
    ("glScissor\0", &glScissor_p),
    ("glShaderSource\0", &glShaderSource_p),
    ("glStencilFunc\0", &glStencilFunc_p),
    ("glStencilFuncSeparate\0", &glStencilFuncSeparate_p),
    ("glStencilMask\0", &glStencilMask_p),
    ("glStencilMaskSeparate\0", &glStencilMaskSeparate_p),
    ("glStencilOp\0", &glStencilOp_p),
    ("glStencilOpSeparate\0", &glStencilOpSeparate_p),
    ("glTexBuffer\0", &glTexBuffer_p),
    ("glTexImage1D\0", &glTexImage1D_p),
    ("glTexImage2D\0", &glTexImage2D_p),
    ("glTexImage2DMultisample\0", &glTexImage2DMultisample_p),
    ("glTexImage3D\0", &glTexImage3D_p),
    ("glTexImage3DMultisample\0", &glTexImage3DMultisample_p),
    ("glTexParameterIiv\0", &glTexParameterIiv_p),
    ("glTexParameterIuiv\0", &glTexParameterIuiv_p),
    ("glTexParameterf\0", &glTexParameterf_p),
    ("glTexParameterfv\0", &glTexParameterfv_p),
    ("glTexParameteri\0", &glTexParameteri_p),
    ("glTexParameteriv\0", &glTexParameteriv_p),
    ("glTexSubImage1D\0", &glTexSubImage1D_p),
    ("glTexSubImage2D\0", &glTexSubImage2D_p),
    ("glTexSubImage3D\0", &glTexSubImage3D_p),
    ("glTransformFeedbackVaryings\0", &glTransformFeedbackVaryings_p),
    ("glUniform1f\0", &glUniform1f_p),
    ("glUniform1fv\0", &glUniform1fv_p),
    ("glUniform1i\0", &glUniform1i_p),
    ("glUniform1iv\0", &glUniform1iv_p),
    ("glUniform1ui\0", &glUniform1ui_p),
    ("glUniform1uiv\0", &glUniform1uiv_p),
    ("glUniform2f\0", &glUniform2f_p),
    ("glUniform2fv\0", &glUniform2fv_p),
    ("glUniform2i\0", &glUniform2i_p),
    ("glUniform2iv\0", &glUniform2iv_p),
    ("glUniform2ui\0", &glUniform2ui_p),
    ("glUniform2uiv\0", &glUniform2uiv_p),
    ("glUniform3f\0", &glUniform3f_p),
    ("glUniform3fv\0", &glUniform3fv_p),
    ("glUniform3i\0", &glUniform3i_p),
    ("glUniform3iv\0", &glUniform3iv_p),
    ("glUniform3ui\0", &glUniform3ui_p),
    ("glUniform3uiv\0", &glUniform3uiv_p),
    ("glUniform4f\0", &glUniform4f_p),
    ("glUniform4fv\0", &glUniform4fv_p),
    ("glUniform4i\0", &glUniform4i_p),
    ("glUniform4iv\0", &glUniform4iv_p),
    ("glUniform4ui\0", &glUniform4ui_p),
    ("glUniform4uiv\0", &glUniform4uiv_p),
    ("glUniformBlockBinding\0", &glUniformBlockBinding_p),
    ("glUniformMatrix2fv\0", &glUniformMatrix2fv_p),
    ("glUniformMatrix2x3fv\0", &glUniformMatrix2x3fv_p),
    ("glUniformMatrix2x4fv\0", &glUniformMatrix2x4fv_p),
    ("glUniformMatrix3fv\0", &glUniformMatrix3fv_p),
    ("glUniformMatrix3x2fv\0", &glUniformMatrix3x2fv_p),
    ("glUniformMatrix3x4fv\0", &glUniformMatrix3x4fv_p),
    ("glUniformMatrix4fv\0", &glUniformMatrix4fv_p),
    ("glUniformMatrix4x2fv\0", &glUniformMatrix4x2fv_p),
    ("glUniformMatrix4x3fv\0", &glUniformMatrix4x3fv_p),
    ("glUnmapBuffer\0", &glUnmapBuffer_p),
    ("glUseProgram\0", &glUseProgram_p),
    ("glValidateProgram\0", &glValidateProgram_p),
    ("glVertexAttrib1d\0", &glVertexAttrib1d_p),
    ("glVertexAttrib1dv\0", &glVertexAttrib1dv_p),
    ("glVertexAttrib1f\0", &glVertexAttrib1f_p),
    ("glVertexAttrib1fv\0", &glVertexAttrib1fv_p),
    ("glVertexAttrib1s\0", &glVertexAttrib1s_p),
    ("glVertexAttrib1sv\0", &glVertexAttrib1sv_p),
    ("glVertexAttrib2d\0", &glVertexAttrib2d_p),
    ("glVertexAttrib2dv\0", &glVertexAttrib2dv_p),
    ("glVertexAttrib2f\0", &glVertexAttrib2f_p),
    ("glVertexAttrib2fv\0", &glVertexAttrib2fv_p),
    ("glVertexAttrib2s\0", &glVertexAttrib2s_p),
    ("glVertexAttrib2sv\0", &glVertexAttrib2sv_p),
    ("glVertexAttrib3d\0", &glVertexAttrib3d_p),
    ("glVertexAttrib3dv\0", &glVertexAttrib3dv_p),
    ("glVertexAttrib3f\0", &glVertexAttrib3f_p),
    ("glVertexAttrib3fv\0", &glVertexAttrib3fv_p),
    ("glVertexAttrib3s\0", &glVertexAttrib3s_p),
    ("glVertexAttrib3sv\0", &glVertexAttrib3sv_p),
    ("glVertexAttrib4Nbv\0", &glVertexAttrib4Nbv_p),
    ("glVertexAttrib4Niv\0", &glVertexAttrib4Niv_p),
    ("glVertexAttrib4Nsv\0", &glVertexAttrib4Nsv_p),
    ("glVertexAttrib4Nub\0", &glVertexAttrib4Nub_p),
    ("glVertexAttrib4Nubv\0", &glVertexAttrib4Nubv_p),
    ("glVertexAttrib4Nuiv\0", &glVertexAttrib4Nuiv_p),
    ("glVertexAttrib4Nusv\0", &glVertexAttrib4Nusv_p),
    ("glVertexAttrib4bv\0", &glVertexAttrib4bv_p),
    ("glVertexAttrib4d\0", &glVertexAttrib4d_p),
    ("glVertexAttrib4dv\0", &glVertexAttrib4dv_p),
    ("glVertexAttrib4f\0", &glVertexAttrib4f_p),
    ("glVertexAttrib4fv\0", &glVertexAttrib4fv_p),
    ("glVertexAttrib4iv\0", &glVertexAttrib4iv_p),
    ("glVertexAttrib4s\0", &glVertexAttrib4s_p),
    ("glVertexAttrib4sv\0", &glVertexAttrib4sv_p),
    ("glVertexAttrib4ubv\0", &glVertexAttrib4ubv_p),
    ("glVertexAttrib4uiv\0", &glVertexAttrib4uiv_p),
    ("glVertexAttrib4usv\0", &glVertexAttrib4usv_p),
    ("glVertexAttribDivisor\0", &glVertexAttribDivisor_p),
    ("glVertexAttribI1i\0", &glVertexAttribI1i_p),
    ("glVertexAttribI1iv\0", &glVertexAttribI1iv_p),
    ("glVertexAttribI1ui\0", &glVertexAttribI1ui_p),
    ("glVertexAttribI1uiv\0", &glVertexAttribI1uiv_p),
    ("glVertexAttribI2i\0", &glVertexAttribI2i_p),
    ("glVertexAttribI2iv\0", &glVertexAttribI2iv_p),
    ("glVertexAttribI2ui\0", &glVertexAttribI2ui_p),
    ("glVertexAttribI2uiv\0", &glVertexAttribI2uiv_p),
    ("glVertexAttribI3i\0", &glVertexAttribI3i_p),
    ("glVertexAttribI3iv\0", &glVertexAttribI3iv_p),
    ("glVertexAttribI3ui\0", &glVertexAttribI3ui_p),
    ("glVertexAttribI3uiv\0", &glVertexAttribI3uiv_p),
    ("glVertexAttribI4bv\0", &glVertexAttribI4bv_p),
    ("glVertexAttribI4i\0", &glVertexAttribI4i_p),
    ("glVertexAttribI4iv\0", &glVertexAttribI4iv_p),
    ("glVertexAttribI4sv\0", &glVertexAttribI4sv_p),
    ("glVertexAttribI4ubv\0", &glVertexAttribI4ubv_p),
    ("glVertexAttribI4ui\0", &glVertexAttribI4ui_p),
    ("glVertexAttribI4uiv\0", &glVertexAttribI4uiv_p),
    ("glVertexAttribI4usv\0", &glVertexAttribI4usv_p),
    ("glVertexAttribIPointer\0", &glVertexAttribIPointer_p),
    ("glVertexAttribP1ui\0", &glVertexAttribP1ui_p),
    ("glVertexAttribP1uiv\0", &glVertexAttribP1uiv_p),
    ("glVertexAttribP2ui\0", &glVertexAttribP2ui_p),
    ("glVertexAttribP2uiv\0", &glVertexAttribP2uiv_p),
    ("glVertexAttribP3ui\0", &glVertexAttribP3ui_p),
    ("glVertexAttribP3uiv\0", &glVertexAttribP3uiv_p),
    ("glVertexAttribP4ui\0", &glVertexAttribP4ui_p),
    ("glVertexAttribP4uiv\0", &glVertexAttribP4uiv_p),
    ("glVertexAttribPointer\0", &glVertexAttribPointer_p),
    ("glViewport\0", &glViewport_p),
    ("glWaitSync\0", &glWaitSync_p),
  ];
  for (command_name, command_p) in command_info.iter() {
    let addr: usize = match load_fn(command_name.as_ptr()) as usize {
      0 | 1 | 2 | 3 | usize::MAX => {
        return Err(&command_name[..command_name.len() - 1]);
      }
      other => other,
    };
    command_p.store(addr, core::sync::atomic::Ordering::Release);
  }
  Ok(())
}
