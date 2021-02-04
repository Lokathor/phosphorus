use super::*;

///
/// * Group: [`SamplePatternSGIS`], [`SamplePatternEXT`]
pub const GL_1PASS_EXT: GLenum = GLenum(0x80A1);

///
/// * Group: [`SamplePatternSGIS`]
pub const GL_1PASS_SGIS: GLenum = GLenum(0x80A1);

///
/// * Group: [`FeedbackType`]
pub const GL_2D: GLenum = GLenum(0x0600);

///
/// * Group: [`SamplePatternSGIS`], [`SamplePatternEXT`]
pub const GL_2PASS_0_EXT: GLenum = GLenum(0x80A2);

///
/// * Group: [`SamplePatternSGIS`]
pub const GL_2PASS_0_SGIS: GLenum = GLenum(0x80A2);

///
/// * Group: [`SamplePatternSGIS`], [`SamplePatternEXT`]
pub const GL_2PASS_1_EXT: GLenum = GLenum(0x80A3);

///
/// * Group: [`SamplePatternSGIS`]
pub const GL_2PASS_1_SGIS: GLenum = GLenum(0x80A3);

///
/// * Group: [`FragmentShaderDestModMaskATI`]
pub const GL_2X_BIT_ATI: GLbitfield = GLbitfield(0x00000001);

///
/// * Group: [`ListNameType`]
pub const GL_2_BYTES: GLenum = GLenum(0x1407);

pub const GL_2_BYTES_NV: GLenum = GLenum(0x1407);

///
/// * Group: [`FeedbackType`]
pub const GL_3D: GLenum = GLenum(0x0601);

pub const GL_3DC_XY_AMD: GLenum = GLenum(0x87FA);

pub const GL_3DC_X_AMD: GLenum = GLenum(0x87F9);

///
/// * Group: [`FeedbackType`]
pub const GL_3D_COLOR: GLenum = GLenum(0x0602);

///
/// * Group: [`FeedbackType`]
pub const GL_3D_COLOR_TEXTURE: GLenum = GLenum(0x0603);

///
/// * Group: [`ListNameType`]
pub const GL_3_BYTES: GLenum = GLenum(0x1408);

pub const GL_3_BYTES_NV: GLenum = GLenum(0x1408);

pub const GL_422_AVERAGE_EXT: GLenum = GLenum(0x80CE);

pub const GL_422_EXT: GLenum = GLenum(0x80CC);

pub const GL_422_REV_AVERAGE_EXT: GLenum = GLenum(0x80CF);

pub const GL_422_REV_EXT: GLenum = GLenum(0x80CD);

///
/// * Group: [`FeedbackType`]
pub const GL_4D_COLOR_TEXTURE: GLenum = GLenum(0x0604);

///
/// * Group: [`SamplePatternSGIS`], [`SamplePatternEXT`]
pub const GL_4PASS_0_EXT: GLenum = GLenum(0x80A4);

///
/// * Group: [`SamplePatternSGIS`]
pub const GL_4PASS_0_SGIS: GLenum = GLenum(0x80A4);

///
/// * Group: [`SamplePatternSGIS`], [`SamplePatternEXT`]
pub const GL_4PASS_1_EXT: GLenum = GLenum(0x80A5);

///
/// * Group: [`SamplePatternSGIS`]
pub const GL_4PASS_1_SGIS: GLenum = GLenum(0x80A5);

///
/// * Group: [`SamplePatternSGIS`], [`SamplePatternEXT`]
pub const GL_4PASS_2_EXT: GLenum = GLenum(0x80A6);

///
/// * Group: [`SamplePatternSGIS`]
pub const GL_4PASS_2_SGIS: GLenum = GLenum(0x80A6);

///
/// * Group: [`SamplePatternSGIS`], [`SamplePatternEXT`]
pub const GL_4PASS_3_EXT: GLenum = GLenum(0x80A7);

///
/// * Group: [`SamplePatternSGIS`]
pub const GL_4PASS_3_SGIS: GLenum = GLenum(0x80A7);

///
/// * Group: [`FragmentShaderDestModMaskATI`]
pub const GL_4X_BIT_ATI: GLbitfield = GLbitfield(0x00000002);

///
/// * Group: [`ListNameType`]
pub const GL_4_BYTES: GLenum = GLenum(0x1409);

pub const GL_4_BYTES_NV: GLenum = GLenum(0x1409);

///
/// * Group: [`FragmentShaderDestModMaskATI`]
pub const GL_8X_BIT_ATI: GLbitfield = GLbitfield(0x00000004);

///
/// * Group: [`PixelFormat`]
pub const GL_ABGR_EXT: GLenum = GLenum(0x8000);

///
/// * Group: [`AccumOp`]
pub const GL_ACCUM: GLenum = GLenum(0x0100);

///
/// * Group: [`PathListMode`]
pub const GL_ACCUM_ADJACENT_PAIRS_NV: GLenum = GLenum(0x90AD);

///
/// * Group: [`GetPName`]
pub const GL_ACCUM_ALPHA_BITS: GLenum = GLenum(0x0D5B);

///
/// * Group: [`GetPName`]
pub const GL_ACCUM_BLUE_BITS: GLenum = GLenum(0x0D5A);

///
/// * Group: [`ClearBufferMask`], [`AttribMask`]
pub const GL_ACCUM_BUFFER_BIT: GLbitfield = GLbitfield(0x00000200);

///
/// * Group: [`GetPName`]
pub const GL_ACCUM_CLEAR_VALUE: GLenum = GLenum(0x0B80);

///
/// * Group: [`GetPName`]
pub const GL_ACCUM_GREEN_BITS: GLenum = GLenum(0x0D59);

///
/// * Group: [`GetPName`]
pub const GL_ACCUM_RED_BITS: GLenum = GLenum(0x0D58);

///
/// * Group: [`ProgramPropertyARB`]
pub const GL_ACTIVE_ATOMIC_COUNTER_BUFFERS: GLenum = GLenum(0x92D9);

///
/// * Group: [`ProgramPropertyARB`]
pub const GL_ACTIVE_ATTRIBUTES: GLenum = GLenum(0x8B89);

///
/// * Group: [`ProgramPropertyARB`]
pub const GL_ACTIVE_ATTRIBUTE_MAX_LENGTH: GLenum = GLenum(0x8B8A);

///
/// * Group: [`PipelineParameterName`]
pub const GL_ACTIVE_PROGRAM: GLenum = GLenum(0x8259);

/// For the OpenGL ES version of `EXT_separate_shader_objects`.
#[doc(alias = "GL_ACTIVE_PROGRAM_EXT")]
pub const GL_ACTIVE_PROGRAM_EXT_ES: GLenum = GLenum(0x8259);

/// For the OpenGL version of `EXT_separate_shader_objects`.
#[doc(alias = "GL_ACTIVE_PROGRAM_EXT")]
pub const GL_ACTIVE_PROGRAM_EXT_GL: GLenum = GLenum(0x8259);

///
/// * Group: [`ProgramInterfacePName`]
pub const GL_ACTIVE_RESOURCES: GLenum = GLenum(0x92F5);

pub const GL_ACTIVE_STENCIL_FACE_EXT: GLenum = GLenum(0x8911);

///
/// * Group: [`ProgramStagePName`]
pub const GL_ACTIVE_SUBROUTINES: GLenum = GLenum(0x8DE5);

///
/// * Group: [`ProgramStagePName`]
pub const GL_ACTIVE_SUBROUTINE_MAX_LENGTH: GLenum = GLenum(0x8E48);

///
/// * Group: [`ProgramStagePName`]
pub const GL_ACTIVE_SUBROUTINE_UNIFORMS: GLenum = GLenum(0x8DE6);

///
/// * Group: [`ProgramStagePName`]
pub const GL_ACTIVE_SUBROUTINE_UNIFORM_LOCATIONS: GLenum = GLenum(0x8E47);

///
/// * Group: [`ProgramStagePName`]
pub const GL_ACTIVE_SUBROUTINE_UNIFORM_MAX_LENGTH: GLenum = GLenum(0x8E49);

///
/// * Group: [`GetPName`]
pub const GL_ACTIVE_TEXTURE: GLenum = GLenum(0x84E0);

pub const GL_ACTIVE_TEXTURE_ARB: GLenum = GLenum(0x84E0);

///
/// * Group: [`ProgramPropertyARB`]
pub const GL_ACTIVE_UNIFORMS: GLenum = GLenum(0x8B86);

///
/// * Group: [`ProgramPropertyARB`]
pub const GL_ACTIVE_UNIFORM_BLOCKS: GLenum = GLenum(0x8A36);

///
/// * Group: [`ProgramPropertyARB`]
pub const GL_ACTIVE_UNIFORM_BLOCK_MAX_NAME_LENGTH: GLenum = GLenum(0x8A35);

///
/// * Group: [`ProgramPropertyARB`]
pub const GL_ACTIVE_UNIFORM_MAX_LENGTH: GLenum = GLenum(0x8B87);

///
/// * Group: [`ProgramResourceProperty`]
pub const GL_ACTIVE_VARIABLES: GLenum = GLenum(0x9305);

pub const GL_ACTIVE_VARYINGS_NV: GLenum = GLenum(0x8C81);

pub const GL_ACTIVE_VARYING_MAX_LENGTH_NV: GLenum = GLenum(0x8C82);

pub const GL_ACTIVE_VERTEX_UNITS_ARB: GLenum = GLenum(0x86A5);

///
/// * Group: [`TextureEnvMode`], [`AccumOp`], [`LightEnvModeSGIX`]
pub const GL_ADD: GLenum = GLenum(0x0104);

///
/// * Group: [`FragmentOpATI`]
pub const GL_ADD_ATI: GLenum = GLenum(0x8963);

pub const GL_ADD_BLEND_IMG: GLenum = GLenum(0x8C09);

pub const GL_ADD_SIGNED: GLenum = GLenum(0x8574);

pub const GL_ADD_SIGNED_ARB: GLenum = GLenum(0x8574);

pub const GL_ADD_SIGNED_EXT: GLenum = GLenum(0x8574);

///
/// * Group: [`PathListMode`]
pub const GL_ADJACENT_PAIRS_NV: GLenum = GLenum(0x90AE);

///
/// * Group: [`PathTransformType`]
pub const GL_AFFINE_2D_NV: GLenum = GLenum(0x9092);

///
/// * Group: [`PathTransformType`]
pub const GL_AFFINE_3D_NV: GLenum = GLenum(0x9094);

///
/// * Group: [`GetPName`]
pub const GL_ALIASED_LINE_WIDTH_RANGE: GLenum = GLenum(0x846E);

///
/// * Group: [`GetPName`]
pub const GL_ALIASED_POINT_SIZE_RANGE: GLenum = GLenum(0x846D);

///
/// * Group: [`HintTarget`]
pub const GL_ALLOW_DRAW_FRG_HINT_PGI: GLenum = GLenum(0x1A210);

///
/// * Group: [`HintTarget`]
pub const GL_ALLOW_DRAW_MEM_HINT_PGI: GLenum = GLenum(0x1A211);

///
/// * Group: [`HintTarget`]
pub const GL_ALLOW_DRAW_OBJ_HINT_PGI: GLenum = GLenum(0x1A20E);

///
/// * Group: [`HintTarget`]
pub const GL_ALLOW_DRAW_WIN_HINT_PGI: GLenum = GLenum(0x1A20F);

/// Guaranteed to mark all attribute groups at once
/// * Group: [`AttribMask`]
pub const GL_ALL_ATTRIB_BITS: GLbitfield = GLbitfield(0xFFFFFFFF);

///
/// * Group: [`MemoryBarrierMask`]
pub const GL_ALL_BARRIER_BITS: GLbitfield = GLbitfield(0xFFFFFFFF);

///
/// * Group: [`MemoryBarrierMask`]
pub const GL_ALL_BARRIER_BITS_EXT: GLbitfield = GLbitfield(0xFFFFFFFF);

///
/// * Group: [`FenceConditionNV`]
pub const GL_ALL_COMPLETED_NV: GLenum = GLenum(0x84F2);

pub const GL_ALL_PIXELS_AMD: GLenum = GLenum(0xFFFFFFFF);

///
/// * Group: [`UseProgramStageMask`]
pub const GL_ALL_SHADER_BITS: GLbitfield = GLbitfield(0xFFFFFFFF);

///
/// * Group: [`UseProgramStageMask`]
pub const GL_ALL_SHADER_BITS_EXT: GLbitfield = GLbitfield(0xFFFFFFFF);

pub const GL_ALL_STATIC_DATA_IBM: GLenum = GLenum(103060);

///
/// * Group: [`TextureSwizzle`], [`CombinerPortionNV`], [`PathColorFormat`],
///   [`CombinerComponentUsageNV`], [`PixelFormat`]
pub const GL_ALPHA: GLenum = GLenum(0x1906);

///
/// * Group: [`InternalFormat`]
pub const GL_ALPHA12: GLenum = GLenum(0x803D);

pub const GL_ALPHA12_EXT: GLenum = GLenum(0x803D);

///
/// * Group: [`InternalFormat`]
pub const GL_ALPHA16: GLenum = GLenum(0x803E);

pub const GL_ALPHA16F_ARB: GLenum = GLenum(0x881C);

pub const GL_ALPHA16F_EXT: GLenum = GLenum(0x881C);

pub const GL_ALPHA16I_EXT: GLenum = GLenum(0x8D8A);

pub const GL_ALPHA16UI_EXT: GLenum = GLenum(0x8D78);

pub const GL_ALPHA16_EXT: GLenum = GLenum(0x803E);

pub const GL_ALPHA16_SNORM: GLenum = GLenum(0x9018);

pub const GL_ALPHA32F_ARB: GLenum = GLenum(0x8816);

pub const GL_ALPHA32F_EXT: GLenum = GLenum(0x8816);

pub const GL_ALPHA32I_EXT: GLenum = GLenum(0x8D84);

pub const GL_ALPHA32UI_EXT: GLenum = GLenum(0x8D72);

///
/// * Group: [`InternalFormat`]
pub const GL_ALPHA4: GLenum = GLenum(0x803B);

pub const GL_ALPHA4_EXT: GLenum = GLenum(0x803B);

///
/// * Group: [`InternalFormat`]
pub const GL_ALPHA8: GLenum = GLenum(0x803C);

pub const GL_ALPHA8I_EXT: GLenum = GLenum(0x8D90);

pub const GL_ALPHA8UI_EXT: GLenum = GLenum(0x8D7E);

pub const GL_ALPHA8_EXT: GLenum = GLenum(0x803C);

pub const GL_ALPHA8_OES: GLenum = GLenum(0x803C);

pub const GL_ALPHA8_SNORM: GLenum = GLenum(0x9014);

///
/// * Group: [`PixelTransferParameter`], [`GetPName`]
pub const GL_ALPHA_BIAS: GLenum = GLenum(0x0D1D);

///
/// * Group: [`GetPName`]
pub const GL_ALPHA_BITS: GLenum = GLenum(0x0D55);

pub const GL_ALPHA_FLOAT16_APPLE: GLenum = GLenum(0x881C);

pub const GL_ALPHA_FLOAT16_ATI: GLenum = GLenum(0x881C);

pub const GL_ALPHA_FLOAT32_APPLE: GLenum = GLenum(0x8816);

pub const GL_ALPHA_FLOAT32_ATI: GLenum = GLenum(0x8816);

pub const GL_ALPHA_INTEGER: GLenum = GLenum(0x8D97);

pub const GL_ALPHA_INTEGER_EXT: GLenum = GLenum(0x8D97);

pub const GL_ALPHA_MAX_CLAMP_INGR: GLenum = GLenum(0x8567);

///
/// * Group: [`BlendEquationModeEXT`]
pub const GL_ALPHA_MAX_SGIX: GLenum = GLenum(0x8321);

pub const GL_ALPHA_MIN_CLAMP_INGR: GLenum = GLenum(0x8563);

///
/// * Group: [`BlendEquationModeEXT`]
pub const GL_ALPHA_MIN_SGIX: GLenum = GLenum(0x8320);

///
/// * Group: [`CommandOpcodesNV`]
pub const GL_ALPHA_REF_COMMAND_NV: GLenum = GLenum(0x000F);

///
/// * Group: [`PixelTransferParameter`], [`GetPName`]
pub const GL_ALPHA_SCALE: GLenum = GLenum(0x0D1C);

pub const GL_ALPHA_SNORM: GLenum = GLenum(0x9010);

///
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_ALPHA_TEST: GLenum = GLenum(0x0BC0);

///
/// * Group: [`GetPName`]
pub const GL_ALPHA_TEST_FUNC: GLenum = GLenum(0x0BC1);

///
/// * Group: [`GetPName`]
pub const GL_ALPHA_TEST_FUNC_QCOM: GLenum = GLenum(0x0BC1);

///
/// * Group: [`GetPName`]
pub const GL_ALPHA_TEST_QCOM: GLenum = GLenum(0x0BC0);

///
/// * Group: [`GetPName`]
pub const GL_ALPHA_TEST_REF: GLenum = GLenum(0x0BC2);

///
/// * Group: [`GetPName`]
pub const GL_ALPHA_TEST_REF_QCOM: GLenum = GLenum(0x0BC2);

pub const GL_ALPHA_TO_COVERAGE_DITHER_DEFAULT_NV: GLenum = GLenum(0x934D);

pub const GL_ALPHA_TO_COVERAGE_DITHER_DISABLE_NV: GLenum = GLenum(0x934F);

pub const GL_ALPHA_TO_COVERAGE_DITHER_ENABLE_NV: GLenum = GLenum(0x934E);

pub const GL_ALPHA_TO_COVERAGE_DITHER_MODE_NV: GLenum = GLenum(0x92BF);

///
/// * Group: [`SyncStatus`]
pub const GL_ALREADY_SIGNALED: GLenum = GLenum(0x911A);

pub const GL_ALREADY_SIGNALED_APPLE: GLenum = GLenum(0x911A);

///
/// * Group: [`StencilFunction`], [`IndexFunctionEXT`], [`AlphaFunction`],
///   [`DepthFunction`]
pub const GL_ALWAYS: GLenum = GLenum(0x0207);

///
/// * Group: [`HintTarget`]
pub const GL_ALWAYS_FAST_HINT_PGI: GLenum = GLenum(0x1A20C);

///
/// * Group: [`HintTarget`]
pub const GL_ALWAYS_SOFT_HINT_PGI: GLenum = GLenum(0x1A20D);

///
/// * Group: [`MaterialParameter`], [`FragmentLightParameterSGIX`],
///   [`ColorMaterialParameter`]
pub const GL_AMBIENT: GLenum = GLenum(0x1200);

///
/// * Group: [`MaterialParameter`], [`ColorMaterialParameter`]
pub const GL_AMBIENT_AND_DIFFUSE: GLenum = GLenum(0x1602);

///
/// * Group: [`LogicOp`]
pub const GL_AND: GLenum = GLenum(0x1501);

///
/// * Group: [`LogicOp`]
pub const GL_AND_INVERTED: GLenum = GLenum(0x1504);

///
/// * Group: [`LogicOp`]
pub const GL_AND_REVERSE: GLenum = GLenum(0x1502);

///
/// * Group: [`QueryTarget`]
pub const GL_ANY_SAMPLES_PASSED: GLenum = GLenum(0x8C2F);

///
/// * Group: [`QueryTarget`]
pub const GL_ANY_SAMPLES_PASSED_CONSERVATIVE: GLenum = GLenum(0x8D6A);

pub const GL_ANY_SAMPLES_PASSED_CONSERVATIVE_EXT: GLenum = GLenum(0x8D6A);

pub const GL_ANY_SAMPLES_PASSED_EXT: GLenum = GLenum(0x8C2F);

///
/// * Group: [`PathCoordType`]
pub const GL_ARC_TO_NV: GLenum = GLenum(0xFE);

///
/// * Group: [`CopyBufferSubDataTarget`], [`BufferTargetARB`],
///   [`BufferStorageTarget`]
pub const GL_ARRAY_BUFFER: GLenum = GLenum(0x8892);

pub const GL_ARRAY_BUFFER_ARB: GLenum = GLenum(0x8892);

///
/// * Group: [`GetPName`]
pub const GL_ARRAY_BUFFER_BINDING: GLenum = GLenum(0x8894);

pub const GL_ARRAY_BUFFER_BINDING_ARB: GLenum = GLenum(0x8894);

pub const GL_ARRAY_ELEMENT_LOCK_COUNT_EXT: GLenum = GLenum(0x81A9);

pub const GL_ARRAY_ELEMENT_LOCK_FIRST_EXT: GLenum = GLenum(0x81A8);

pub const GL_ARRAY_OBJECT_BUFFER_ATI: GLenum = GLenum(0x8766);

pub const GL_ARRAY_OBJECT_OFFSET_ATI: GLenum = GLenum(0x8767);

///
/// * Group: [`ProgramResourceProperty`]
pub const GL_ARRAY_SIZE: GLenum = GLenum(0x92FB);

///
/// * Group: [`ProgramResourceProperty`]
pub const GL_ARRAY_STRIDE: GLenum = GLenum(0x92FE);

///
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_ASYNC_DRAW_PIXELS_SGIX: GLenum = GLenum(0x835D);

///
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_ASYNC_HISTOGRAM_SGIX: GLenum = GLenum(0x832C);

///
/// * Group: [`GetPName`]
pub const GL_ASYNC_MARKER_SGIX: GLenum = GLenum(0x8329);

///
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_ASYNC_READ_PIXELS_SGIX: GLenum = GLenum(0x835E);

///
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_ASYNC_TEX_IMAGE_SGIX: GLenum = GLenum(0x835C);

pub const GL_ATC_RGBA_EXPLICIT_ALPHA_AMD: GLenum = GLenum(0x8C93);

pub const GL_ATC_RGBA_INTERPOLATED_ALPHA_AMD: GLenum = GLenum(0x87EE);

pub const GL_ATC_RGB_AMD: GLenum = GLenum(0x8C92);

///
/// * Group: [`MemoryBarrierMask`]
pub const GL_ATOMIC_COUNTER_BARRIER_BIT: GLbitfield = GLbitfield(0x00001000);

///
/// * Group: [`MemoryBarrierMask`]
pub const GL_ATOMIC_COUNTER_BARRIER_BIT_EXT: GLbitfield = GLbitfield(0x00001000);

///
/// * Group: [`CopyBufferSubDataTarget`], [`BufferTargetARB`],
///   [`BufferStorageTarget`]
pub const GL_ATOMIC_COUNTER_BUFFER: GLenum = GLenum(0x92C0);

///
/// * Group: [`AtomicCounterBufferPName`]
pub const GL_ATOMIC_COUNTER_BUFFER_ACTIVE_ATOMIC_COUNTERS: GLenum = GLenum(0x92C5);

///
/// * Group: [`AtomicCounterBufferPName`]
pub const GL_ATOMIC_COUNTER_BUFFER_ACTIVE_ATOMIC_COUNTER_INDICES: GLenum = GLenum(0x92C6);

///
/// * Group: [`AtomicCounterBufferPName`]
pub const GL_ATOMIC_COUNTER_BUFFER_BINDING: GLenum = GLenum(0x92C1);

///
/// * Group: [`AtomicCounterBufferPName`]
pub const GL_ATOMIC_COUNTER_BUFFER_DATA_SIZE: GLenum = GLenum(0x92C4);

///
/// * Group: [`ProgramResourceProperty`]
pub const GL_ATOMIC_COUNTER_BUFFER_INDEX: GLenum = GLenum(0x9301);

///
/// * Group: [`AtomicCounterBufferPName`]
pub const GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_COMPUTE_SHADER: GLenum = GLenum(0x90ED);

///
/// * Group: [`AtomicCounterBufferPName`]
pub const GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_FRAGMENT_SHADER: GLenum = GLenum(0x92CB);

///
/// * Group: [`AtomicCounterBufferPName`]
pub const GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_GEOMETRY_SHADER: GLenum = GLenum(0x92CA);

pub const GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_MESH_SHADER_NV: GLenum = GLenum(0x959E);

pub const GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_TASK_SHADER_NV: GLenum = GLenum(0x959F);

///
/// * Group: [`AtomicCounterBufferPName`]
pub const GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_TESS_CONTROL_SHADER: GLenum = GLenum(0x92C8);

///
/// * Group: [`AtomicCounterBufferPName`]
pub const GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_TESS_EVALUATION_SHADER: GLenum = GLenum(0x92C9);

///
/// * Group: [`AtomicCounterBufferPName`]
pub const GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_VERTEX_SHADER: GLenum = GLenum(0x92C7);

pub const GL_ATOMIC_COUNTER_BUFFER_SIZE: GLenum = GLenum(0x92C3);

pub const GL_ATOMIC_COUNTER_BUFFER_START: GLenum = GLenum(0x92C2);

pub const GL_ATTACHED_MEMORY_OBJECT_NV: GLenum = GLenum(0x95A4);

pub const GL_ATTACHED_MEMORY_OFFSET_NV: GLenum = GLenum(0x95A5);

///
/// * Group: [`ProgramPropertyARB`]
pub const GL_ATTACHED_SHADERS: GLenum = GLenum(0x8B85);

///
/// * Group: [`LightTexturePNameEXT`]
pub const GL_ATTENUATION_EXT: GLenum = GLenum(0x834D);

///
/// * Group: [`CommandOpcodesNV`]
pub const GL_ATTRIBUTE_ADDRESS_COMMAND_NV: GLenum = GLenum(0x0009);

pub const GL_ATTRIB_ARRAY_POINTER_NV: GLenum = GLenum(0x8645);

pub const GL_ATTRIB_ARRAY_SIZE_NV: GLenum = GLenum(0x8623);

pub const GL_ATTRIB_ARRAY_STRIDE_NV: GLenum = GLenum(0x8624);

pub const GL_ATTRIB_ARRAY_TYPE_NV: GLenum = GLenum(0x8625);

///
/// * Group: [`GetPName`]
pub const GL_ATTRIB_STACK_DEPTH: GLenum = GLenum(0x0BB0);

/// Should be deprecated
/// * Group: [`InternalFormatPName`]
pub const GL_AUTO_GENERATE_MIPMAP: GLenum = GLenum(0x8295);

///
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_AUTO_NORMAL: GLenum = GLenum(0x0D80);

///
/// * Group: [`ReadBufferMode`], [`DrawBufferMode`]
pub const GL_AUX0: GLenum = GLenum(0x0409);

///
/// * Group: [`ReadBufferMode`], [`DrawBufferMode`]
pub const GL_AUX1: GLenum = GLenum(0x040A);

///
/// * Group: [`ReadBufferMode`], [`DrawBufferMode`]
pub const GL_AUX2: GLenum = GLenum(0x040B);

///
/// * Group: [`ReadBufferMode`], [`DrawBufferMode`]
pub const GL_AUX3: GLenum = GLenum(0x040C);

///
/// * Group: [`GetPName`]
pub const GL_AUX_BUFFERS: GLenum = GLenum(0x0C00);

pub const GL_AUX_DEPTH_STENCIL_APPLE: GLenum = GLenum(0x8A14);

pub const GL_AVERAGE_EXT: GLenum = GLenum(0x8335);

pub const GL_AVERAGE_HP: GLenum = GLenum(0x8160);

///
/// * Group: [`ColorBuffer`], [`ColorMaterialFace`], [`CullFaceMode`],
///   [`DrawBufferMode`], [`ReadBufferMode`], [`StencilFaceDirection`],
///   [`MaterialFace`]
pub const GL_BACK: GLenum = GLenum(0x0405);

///
/// * Group: [`ColorBuffer`], [`DrawBufferMode`], [`ReadBufferMode`]
pub const GL_BACK_LEFT: GLenum = GLenum(0x0402);

///
/// * Group: [`HintTarget`]
pub const GL_BACK_NORMALS_HINT_PGI: GLenum = GLenum(0x1A223);

pub const GL_BACK_PRIMARY_COLOR_NV: GLenum = GLenum(0x8C77);

///
/// * Group: [`ColorBuffer`], [`DrawBufferMode`], [`ReadBufferMode`]
pub const GL_BACK_RIGHT: GLenum = GLenum(0x0403);

pub const GL_BACK_SECONDARY_COLOR_NV: GLenum = GLenum(0x8C78);

pub const GL_BEVEL_NV: GLenum = GLenum(0x90A6);

///
/// * Group: [`PixelFormat`]
pub const GL_BGR: GLenum = GLenum(0x80E0);

///
/// * Group: [`PixelFormat`]
pub const GL_BGRA: GLenum = GLenum(0x80E1);

pub const GL_BGRA8_EXT: GLenum = GLenum(0x93A1);

pub const GL_BGRA_EXT: GLenum = GLenum(0x80E1);

pub const GL_BGRA_IMG: GLenum = GLenum(0x80E1);

///
/// * Group: [`PixelFormat`]
pub const GL_BGRA_INTEGER: GLenum = GLenum(0x8D9B);

pub const GL_BGRA_INTEGER_EXT: GLenum = GLenum(0x8D9B);

pub const GL_BGR_EXT: GLenum = GLenum(0x80E0);

///
/// * Group: [`PixelFormat`]
pub const GL_BGR_INTEGER: GLenum = GLenum(0x8D9A);

pub const GL_BGR_INTEGER_EXT: GLenum = GLenum(0x8D9A);

///
/// * Group: [`FragmentShaderColorModMaskATI`]
pub const GL_BIAS_BIT_ATI: GLbitfield = GLbitfield(0x00000008);

///
/// * Group: [`CombinerBiasNV`]
pub const GL_BIAS_BY_NEGATIVE_ONE_HALF_NV: GLenum = GLenum(0x8541);

///
/// * Group: [`HintTarget`]
pub const GL_BINNING_CONTROL_HINT_QCOM: GLenum = GLenum(0x8FB0);

pub const GL_BINORMAL_ARRAY_EXT: GLenum = GLenum(0x843A);

pub const GL_BINORMAL_ARRAY_POINTER_EXT: GLenum = GLenum(0x8443);

pub const GL_BINORMAL_ARRAY_STRIDE_EXT: GLenum = GLenum(0x8441);

pub const GL_BINORMAL_ARRAY_TYPE_EXT: GLenum = GLenum(0x8440);

///
/// * Group: [`PixelType`]
pub const GL_BITMAP: GLenum = GLenum(0x1A00);

///
/// * Group: [`FeedBackToken`]
pub const GL_BITMAP_TOKEN: GLenum = GLenum(0x0704);

pub const GL_BLACKHOLE_RENDER_INTEL: GLenum = GLenum(0x83FC);

///
/// * Group: [`TextureEnvMode`], [`EnableCap`], [`GetPName`]
pub const GL_BLEND: GLenum = GLenum(0x0BE2);

pub const GL_BLEND_ADVANCED_COHERENT_KHR: GLenum = GLenum(0x9285);

pub const GL_BLEND_ADVANCED_COHERENT_NV: GLenum = GLenum(0x9285);

///
/// * Group: [`GetPName`]
pub const GL_BLEND_COLOR: GLenum = GLenum(0x8005);

///
/// * Group: [`CommandOpcodesNV`]
pub const GL_BLEND_COLOR_COMMAND_NV: GLenum = GLenum(0x000B);

///
/// * Group: [`GetPName`]
pub const GL_BLEND_COLOR_EXT: GLenum = GLenum(0x8005);

///
/// * Group: [`GetPName`]
pub const GL_BLEND_DST: GLenum = GLenum(0x0BE0);

///
/// * Group: [`GetPName`]
pub const GL_BLEND_DST_ALPHA: GLenum = GLenum(0x80CA);

pub const GL_BLEND_DST_ALPHA_EXT: GLenum = GLenum(0x80CA);

pub const GL_BLEND_DST_ALPHA_OES: GLenum = GLenum(0x80CA);

///
/// * Group: [`GetPName`]
pub const GL_BLEND_DST_RGB: GLenum = GLenum(0x80C8);

pub const GL_BLEND_DST_RGB_EXT: GLenum = GLenum(0x80C8);

pub const GL_BLEND_DST_RGB_OES: GLenum = GLenum(0x80C8);

pub const GL_BLEND_EQUATION: GLenum = GLenum(0x8009);

///
/// * Group: [`GetPName`]
pub const GL_BLEND_EQUATION_ALPHA: GLenum = GLenum(0x883D);

pub const GL_BLEND_EQUATION_ALPHA_EXT: GLenum = GLenum(0x883D);

pub const GL_BLEND_EQUATION_ALPHA_OES: GLenum = GLenum(0x883D);

///
/// * Group: [`GetPName`]
pub const GL_BLEND_EQUATION_EXT: GLenum = GLenum(0x8009);

pub const GL_BLEND_EQUATION_OES: GLenum = GLenum(0x8009);

///
/// * Group: [`GetPName`]
pub const GL_BLEND_EQUATION_RGB: GLenum = GLenum(0x8009);

pub const GL_BLEND_EQUATION_RGB_EXT: GLenum = GLenum(0x8009);

pub const GL_BLEND_EQUATION_RGB_OES: GLenum = GLenum(0x8009);

pub const GL_BLEND_OVERLAP_NV: GLenum = GLenum(0x9281);

pub const GL_BLEND_PREMULTIPLIED_SRC_NV: GLenum = GLenum(0x9280);

///
/// * Group: [`GetPName`]
pub const GL_BLEND_SRC: GLenum = GLenum(0x0BE1);

///
/// * Group: [`GetPName`]
pub const GL_BLEND_SRC_ALPHA: GLenum = GLenum(0x80CB);

pub const GL_BLEND_SRC_ALPHA_EXT: GLenum = GLenum(0x80CB);

pub const GL_BLEND_SRC_ALPHA_OES: GLenum = GLenum(0x80CB);

///
/// * Group: [`GetPName`]
pub const GL_BLEND_SRC_RGB: GLenum = GLenum(0x80C9);

pub const GL_BLEND_SRC_RGB_EXT: GLenum = GLenum(0x80C9);

pub const GL_BLEND_SRC_RGB_OES: GLenum = GLenum(0x80C9);

///
/// * Group: [`ProgramResourceProperty`]
pub const GL_BLOCK_INDEX: GLenum = GLenum(0x92FD);

///
/// * Group: [`TextureSwizzle`], [`CombinerComponentUsageNV`], [`PixelFormat`]
pub const GL_BLUE: GLenum = GLenum(0x1905);

///
/// * Group: [`PixelTransferParameter`], [`GetPName`]
pub const GL_BLUE_BIAS: GLenum = GLenum(0x0D1B);

///
/// * Group: [`GetPName`]
pub const GL_BLUE_BITS: GLenum = GLenum(0x0D54);

///
/// * Group: [`FragmentShaderDestMaskATI`]
pub const GL_BLUE_BIT_ATI: GLbitfield = GLbitfield(0x00000004);

///
/// * Group: [`PixelFormat`]
pub const GL_BLUE_INTEGER: GLenum = GLenum(0x8D96);

pub const GL_BLUE_INTEGER_EXT: GLenum = GLenum(0x8D96);

pub const GL_BLUE_MAX_CLAMP_INGR: GLenum = GLenum(0x8566);

pub const GL_BLUE_MIN_CLAMP_INGR: GLenum = GLenum(0x8562);

pub const GL_BLUE_NV: GLenum = GLenum(0x1905);

///
/// * Group: [`PixelTransferParameter`], [`GetPName`]
pub const GL_BLUE_SCALE: GLenum = GLenum(0x0D1A);

///
/// * Group: [`PathFontStyle`]
pub const GL_BOLD_BIT_NV: GLbitfield = GLbitfield(0x01);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_BOOL: GLenum = GLenum(0x8B56);

///
/// * Group: [`AttributeType`]
pub const GL_BOOL_ARB: GLenum = GLenum(0x8B56);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_BOOL_VEC2: GLenum = GLenum(0x8B57);

///
/// * Group: [`AttributeType`]
pub const GL_BOOL_VEC2_ARB: GLenum = GLenum(0x8B57);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_BOOL_VEC3: GLenum = GLenum(0x8B58);

///
/// * Group: [`AttributeType`]
pub const GL_BOOL_VEC3_ARB: GLenum = GLenum(0x8B58);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_BOOL_VEC4: GLenum = GLenum(0x8B59);

///
/// * Group: [`AttributeType`]
pub const GL_BOOL_VEC4_ARB: GLenum = GLenum(0x8B59);

///
/// * Group: [`PathCoverMode`]
pub const GL_BOUNDING_BOX_NV: GLenum = GLenum(0x908D);

///
/// * Group: [`PathCoverMode`]
pub const GL_BOUNDING_BOX_OF_BOUNDING_BOXES_NV: GLenum = GLenum(0x909C);

pub const GL_BROWSER_DEFAULT_WEBGL: GLenum = GLenum(0x9244);

///
/// * Group: [`ObjectIdentifier`]
pub const GL_BUFFER: GLenum = GLenum(0x82E0);

///
/// * Group: [`VertexBufferObjectParameter`], [`BufferPNameARB`]
pub const GL_BUFFER_ACCESS: GLenum = GLenum(0x88BB);

///
/// * Group: [`BufferPNameARB`]
pub const GL_BUFFER_ACCESS_ARB: GLenum = GLenum(0x88BB);

///
/// * Group: [`VertexBufferObjectParameter`], [`BufferPNameARB`]
pub const GL_BUFFER_ACCESS_FLAGS: GLenum = GLenum(0x911F);

pub const GL_BUFFER_ACCESS_OES: GLenum = GLenum(0x88BB);

///
/// * Group: [`ProgramResourceProperty`]
pub const GL_BUFFER_BINDING: GLenum = GLenum(0x9302);

///
/// * Group: [`ProgramResourceProperty`]
pub const GL_BUFFER_DATA_SIZE: GLenum = GLenum(0x9303);

pub const GL_BUFFER_FLUSHING_UNMAP_APPLE: GLenum = GLenum(0x8A13);

pub const GL_BUFFER_GPU_ADDRESS_NV: GLenum = GLenum(0x8F1D);

///
/// * Group: [`VertexBufferObjectParameter`], [`BufferPNameARB`]
pub const GL_BUFFER_IMMUTABLE_STORAGE: GLenum = GLenum(0x821F);

pub const GL_BUFFER_IMMUTABLE_STORAGE_EXT: GLenum = GLenum(0x821F);

pub const GL_BUFFER_KHR: GLenum = GLenum(0x82E0);

///
/// * Group: [`VertexBufferObjectParameter`], [`BufferPNameARB`]
pub const GL_BUFFER_MAPPED: GLenum = GLenum(0x88BC);

///
/// * Group: [`BufferPNameARB`]
pub const GL_BUFFER_MAPPED_ARB: GLenum = GLenum(0x88BC);

pub const GL_BUFFER_MAPPED_OES: GLenum = GLenum(0x88BC);

///
/// * Group: [`VertexBufferObjectParameter`], [`BufferPNameARB`]
pub const GL_BUFFER_MAP_LENGTH: GLenum = GLenum(0x9120);

///
/// * Group: [`VertexBufferObjectParameter`], [`BufferPNameARB`]
pub const GL_BUFFER_MAP_OFFSET: GLenum = GLenum(0x9121);

///
/// * Group: [`BufferPointerNameARB`]
pub const GL_BUFFER_MAP_POINTER: GLenum = GLenum(0x88BD);

///
/// * Group: [`BufferPointerNameARB`]
pub const GL_BUFFER_MAP_POINTER_ARB: GLenum = GLenum(0x88BD);

pub const GL_BUFFER_MAP_POINTER_OES: GLenum = GLenum(0x88BD);

pub const GL_BUFFER_OBJECT_APPLE: GLenum = GLenum(0x85B3);

pub const GL_BUFFER_OBJECT_EXT: GLenum = GLenum(0x9151);

pub const GL_BUFFER_SERIALIZED_MODIFY_APPLE: GLenum = GLenum(0x8A12);

///
/// * Group: [`VertexBufferObjectParameter`], [`BufferPNameARB`]
pub const GL_BUFFER_SIZE: GLenum = GLenum(0x8764);

///
/// * Group: [`BufferPNameARB`]
pub const GL_BUFFER_SIZE_ARB: GLenum = GLenum(0x8764);

///
/// * Group: [`VertexBufferObjectParameter`], [`BufferPNameARB`]
pub const GL_BUFFER_STORAGE_FLAGS: GLenum = GLenum(0x8220);

pub const GL_BUFFER_STORAGE_FLAGS_EXT: GLenum = GLenum(0x8220);

///
/// * Group: [`MemoryBarrierMask`]
pub const GL_BUFFER_UPDATE_BARRIER_BIT: GLbitfield = GLbitfield(0x00000200);

///
/// * Group: [`MemoryBarrierMask`]
pub const GL_BUFFER_UPDATE_BARRIER_BIT_EXT: GLbitfield = GLbitfield(0x00000200);

///
/// * Group: [`VertexBufferObjectParameter`], [`BufferPNameARB`]
pub const GL_BUFFER_USAGE: GLenum = GLenum(0x8765);

///
/// * Group: [`BufferPNameARB`]
pub const GL_BUFFER_USAGE_ARB: GLenum = GLenum(0x8765);

///
/// * Group: [`ProgramInterface`]
pub const GL_BUFFER_VARIABLE: GLenum = GLenum(0x92E5);

pub const GL_BUMP_ENVMAP_ATI: GLenum = GLenum(0x877B);

///
/// * Group: [`GetTexBumpParameterATI`]
pub const GL_BUMP_NUM_TEX_UNITS_ATI: GLenum = GLenum(0x8777);

///
/// * Group: [`GetTexBumpParameterATI`], [`TexBumpParameterATI`]
pub const GL_BUMP_ROT_MATRIX_ATI: GLenum = GLenum(0x8775);

///
/// * Group: [`GetTexBumpParameterATI`]
pub const GL_BUMP_ROT_MATRIX_SIZE_ATI: GLenum = GLenum(0x8776);

pub const GL_BUMP_TARGET_ATI: GLenum = GLenum(0x877C);

///
/// * Group: [`GetTexBumpParameterATI`]
pub const GL_BUMP_TEX_UNITS_ATI: GLenum = GLenum(0x8778);

///
/// * Group: [`VertexAttribIType`], [`WeightPointerTypeARB`],
///   [`TangentPointerTypeEXT`], [`BinormalPointerTypeEXT`],
///   [`ColorPointerType`], [`ListNameType`], [`NormalPointerType`],
///   [`PixelType`], [`VertexAttribType`], [`VertexAttribPointerType`]
pub const GL_BYTE: GLenum = GLenum(0x1400);

///
/// * Group: [`InterleavedArrayFormat`]
pub const GL_C3F_V3F: GLenum = GLenum(0x2A24);

///
/// * Group: [`InterleavedArrayFormat`]
pub const GL_C4F_N3F_V3F: GLenum = GLenum(0x2A26);

///
/// * Group: [`InterleavedArrayFormat`]
pub const GL_C4UB_V2F: GLenum = GLenum(0x2A22);

///
/// * Group: [`InterleavedArrayFormat`]
pub const GL_C4UB_V3F: GLenum = GLenum(0x2A23);

///
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_CALLIGRAPHIC_FRAGMENT_SGIX: GLenum = GLenum(0x8183);

pub const GL_CAVEAT_SUPPORT: GLenum = GLenum(0x82B8);

///
/// * Group: [`FrontFaceDirection`]
pub const GL_CCW: GLenum = GLenum(0x0901);

///
/// * Group: [`PathCoordType`]
pub const GL_CIRCULAR_CCW_ARC_TO_NV: GLenum = GLenum(0xF8);

///
/// * Group: [`PathCoordType`]
pub const GL_CIRCULAR_CW_ARC_TO_NV: GLenum = GLenum(0xFA);

///
/// * Group: [`PathCoordType`]
pub const GL_CIRCULAR_TANGENT_ARC_TO_NV: GLenum = GLenum(0xFC);

///
/// * Group: [`TextureWrapMode`]
pub const GL_CLAMP: GLenum = GLenum(0x2900);

pub const GL_CLAMP_FRAGMENT_COLOR: GLenum = GLenum(0x891B);

///
/// * Group: [`ClampColorTargetARB`]
pub const GL_CLAMP_FRAGMENT_COLOR_ARB: GLenum = GLenum(0x891B);

///
/// * Group: [`ClampColorTargetARB`]
pub const GL_CLAMP_READ_COLOR: GLenum = GLenum(0x891C);

///
/// * Group: [`ClampColorTargetARB`]
pub const GL_CLAMP_READ_COLOR_ARB: GLenum = GLenum(0x891C);

///
/// * Group: [`TextureWrapMode`]
pub const GL_CLAMP_TO_BORDER: GLenum = GLenum(0x812D);

///
/// * Group: [`TextureWrapMode`]
pub const GL_CLAMP_TO_BORDER_ARB: GLenum = GLenum(0x812D);

pub const GL_CLAMP_TO_BORDER_EXT: GLenum = GLenum(0x812D);

///
/// * Group: [`TextureWrapMode`]
pub const GL_CLAMP_TO_BORDER_NV: GLenum = GLenum(0x812D);

pub const GL_CLAMP_TO_BORDER_OES: GLenum = GLenum(0x812D);

///
/// * Group: [`TextureWrapMode`]
pub const GL_CLAMP_TO_BORDER_SGIS: GLenum = GLenum(0x812D);

///
/// * Group: [`TextureWrapMode`]
pub const GL_CLAMP_TO_EDGE: GLenum = GLenum(0x812F);

///
/// * Group: [`TextureWrapMode`]
pub const GL_CLAMP_TO_EDGE_SGIS: GLenum = GLenum(0x812F);

pub const GL_CLAMP_VERTEX_COLOR: GLenum = GLenum(0x891A);

///
/// * Group: [`ClampColorTargetARB`]
pub const GL_CLAMP_VERTEX_COLOR_ARB: GLenum = GLenum(0x891A);

///
/// * Group: [`LogicOp`]
pub const GL_CLEAR: GLenum = GLenum(0x1500);

///
/// * Group: [`InternalFormatPName`]
pub const GL_CLEAR_BUFFER: GLenum = GLenum(0x82B4);

///
/// * Group: [`InternalFormatPName`]
pub const GL_CLEAR_TEXTURE: GLenum = GLenum(0x9365);

pub const GL_CLIENT_ACTIVE_TEXTURE: GLenum = GLenum(0x84E1);

pub const GL_CLIENT_ACTIVE_TEXTURE_ARB: GLenum = GLenum(0x84E1);

///
/// * Group: [`ClientAttribMask`]
pub const GL_CLIENT_ALL_ATTRIB_BITS: GLbitfield = GLbitfield(0xFFFFFFFF);

///
/// * Group: [`GetPName`]
pub const GL_CLIENT_ATTRIB_STACK_DEPTH: GLenum = GLenum(0x0BB1);

///
/// * Group: [`MemoryBarrierMask`]
pub const GL_CLIENT_MAPPED_BUFFER_BARRIER_BIT: GLbitfield = GLbitfield(0x00004000);

///
/// * Group: [`MemoryBarrierMask`]
pub const GL_CLIENT_MAPPED_BUFFER_BARRIER_BIT_EXT: GLbitfield = GLbitfield(0x00004000);

///
/// * Group: [`ClientAttribMask`]
pub const GL_CLIENT_PIXEL_STORE_BIT: GLbitfield = GLbitfield(0x00000001);

///
/// * Group: [`BufferStorageMask`]
pub const GL_CLIENT_STORAGE_BIT: GLbitfield = GLbitfield(0x0200);

///
/// * Group: [`BufferStorageMask`]
pub const GL_CLIENT_STORAGE_BIT_EXT: GLbitfield = GLbitfield(0x0200);

///
/// * Group: [`ClientAttribMask`]
pub const GL_CLIENT_VERTEX_ARRAY_BIT: GLbitfield = GLbitfield(0x00000002);

pub const GL_CLIPPING_INPUT_PRIMITIVES: GLenum = GLenum(0x82F6);

///
/// * Alias Of: [`GL_CLIPPING_INPUT_PRIMITIVES`]
pub const GL_CLIPPING_INPUT_PRIMITIVES_ARB: GLenum = GL_CLIPPING_INPUT_PRIMITIVES;

pub const GL_CLIPPING_OUTPUT_PRIMITIVES: GLenum = GLenum(0x82F7);

///
/// * Alias Of: [`GL_CLIPPING_OUTPUT_PRIMITIVES`]
pub const GL_CLIPPING_OUTPUT_PRIMITIVES_ARB: GLenum = GL_CLIPPING_OUTPUT_PRIMITIVES;

pub const GL_CLIP_DEPTH_MODE: GLenum = GLenum(0x935D);

///
/// * Alias Of: [`GL_CLIP_DEPTH_MODE`]
pub const GL_CLIP_DEPTH_MODE_EXT: GLenum = GL_CLIP_DEPTH_MODE;

///
/// * Group: [`EnableCap`], [`ClipPlaneName`]
/// * Alias Of: [`GL_CLIP_PLANE0`]
pub const GL_CLIP_DISTANCE0: GLenum = GL_CLIP_PLANE0;

pub const GL_CLIP_DISTANCE0_APPLE: GLenum = GLenum(0x3000);

///
/// * Alias Of: [`GL_CLIP_PLANE0`]
pub const GL_CLIP_DISTANCE0_EXT: GLenum = GL_CLIP_PLANE0;

///
/// * Group: [`EnableCap`], [`ClipPlaneName`]
/// * Alias Of: [`GL_CLIP_PLANE1`]
pub const GL_CLIP_DISTANCE1: GLenum = GL_CLIP_PLANE1;

pub const GL_CLIP_DISTANCE1_APPLE: GLenum = GLenum(0x3001);

///
/// * Alias Of: [`GL_CLIP_PLANE1`]
pub const GL_CLIP_DISTANCE1_EXT: GLenum = GL_CLIP_PLANE1;

///
/// * Group: [`EnableCap`], [`ClipPlaneName`]
/// * Alias Of: [`GL_CLIP_PLANE2`]
pub const GL_CLIP_DISTANCE2: GLenum = GL_CLIP_PLANE2;

pub const GL_CLIP_DISTANCE2_APPLE: GLenum = GLenum(0x3002);

///
/// * Alias Of: [`GL_CLIP_PLANE2`]
pub const GL_CLIP_DISTANCE2_EXT: GLenum = GL_CLIP_PLANE2;

///
/// * Group: [`EnableCap`], [`ClipPlaneName`]
/// * Alias Of: [`GL_CLIP_PLANE3`]
pub const GL_CLIP_DISTANCE3: GLenum = GL_CLIP_PLANE3;

pub const GL_CLIP_DISTANCE3_APPLE: GLenum = GLenum(0x3003);

///
/// * Alias Of: [`GL_CLIP_PLANE3`]
pub const GL_CLIP_DISTANCE3_EXT: GLenum = GL_CLIP_PLANE3;

///
/// * Group: [`EnableCap`], [`ClipPlaneName`]
/// * Alias Of: [`GL_CLIP_PLANE4`]
pub const GL_CLIP_DISTANCE4: GLenum = GL_CLIP_PLANE4;

pub const GL_CLIP_DISTANCE4_APPLE: GLenum = GLenum(0x3004);

///
/// * Alias Of: [`GL_CLIP_PLANE4`]
pub const GL_CLIP_DISTANCE4_EXT: GLenum = GL_CLIP_PLANE4;

///
/// * Group: [`EnableCap`], [`ClipPlaneName`]
/// * Alias Of: [`GL_CLIP_PLANE5`]
pub const GL_CLIP_DISTANCE5: GLenum = GL_CLIP_PLANE5;

pub const GL_CLIP_DISTANCE5_APPLE: GLenum = GLenum(0x3005);

///
/// * Alias Of: [`GL_CLIP_PLANE5`]
pub const GL_CLIP_DISTANCE5_EXT: GLenum = GL_CLIP_PLANE5;

///
/// * Group: [`EnableCap`], [`ClipPlaneName`]
pub const GL_CLIP_DISTANCE6: GLenum = GLenum(0x3006);

pub const GL_CLIP_DISTANCE6_APPLE: GLenum = GLenum(0x3006);

///
/// * Alias Of: [`GL_CLIP_DISTANCE6`]
pub const GL_CLIP_DISTANCE6_EXT: GLenum = GL_CLIP_DISTANCE6;

///
/// * Group: [`EnableCap`], [`ClipPlaneName`]
pub const GL_CLIP_DISTANCE7: GLenum = GLenum(0x3007);

pub const GL_CLIP_DISTANCE7_APPLE: GLenum = GLenum(0x3007);

///
/// * Alias Of: [`GL_CLIP_DISTANCE7`]
pub const GL_CLIP_DISTANCE7_EXT: GLenum = GL_CLIP_DISTANCE7;

pub const GL_CLIP_DISTANCE_NV: GLenum = GLenum(0x8C7A);

///
/// * Group: [`HintTarget`]
pub const GL_CLIP_FAR_HINT_PGI: GLenum = GLenum(0x1A221);

///
/// * Group: [`HintTarget`]
pub const GL_CLIP_NEAR_HINT_PGI: GLenum = GLenum(0x1A220);

pub const GL_CLIP_ORIGIN: GLenum = GLenum(0x935C);

///
/// * Alias Of: [`GL_CLIP_ORIGIN`]
pub const GL_CLIP_ORIGIN_EXT: GLenum = GL_CLIP_ORIGIN;

///
/// * Group: [`GetPName`], [`ClipPlaneName`], [`EnableCap`]
pub const GL_CLIP_PLANE0: GLenum = GLenum(0x3000);

pub const GL_CLIP_PLANE0_IMG: GLenum = GLenum(0x3000);

///
/// * Group: [`GetPName`], [`ClipPlaneName`], [`EnableCap`]
pub const GL_CLIP_PLANE1: GLenum = GLenum(0x3001);

pub const GL_CLIP_PLANE1_IMG: GLenum = GLenum(0x3001);

///
/// * Group: [`GetPName`], [`ClipPlaneName`], [`EnableCap`]
pub const GL_CLIP_PLANE2: GLenum = GLenum(0x3002);

pub const GL_CLIP_PLANE2_IMG: GLenum = GLenum(0x3002);

///
/// * Group: [`GetPName`], [`ClipPlaneName`], [`EnableCap`]
pub const GL_CLIP_PLANE3: GLenum = GLenum(0x3003);

pub const GL_CLIP_PLANE3_IMG: GLenum = GLenum(0x3003);

///
/// * Group: [`GetPName`], [`ClipPlaneName`], [`EnableCap`]
pub const GL_CLIP_PLANE4: GLenum = GLenum(0x3004);

pub const GL_CLIP_PLANE4_IMG: GLenum = GLenum(0x3004);

///
/// * Group: [`GetPName`], [`ClipPlaneName`], [`EnableCap`]
pub const GL_CLIP_PLANE5: GLenum = GLenum(0x3005);

pub const GL_CLIP_PLANE5_IMG: GLenum = GLenum(0x3005);

///
/// * Group: [`HintTarget`]
pub const GL_CLIP_VOLUME_CLIPPING_HINT_EXT: GLenum = GLenum(0x80F0);

///
/// * Group: [`PathCoordType`]
pub const GL_CLOSE_PATH_NV: GLenum = GLenum(0x00);

///
/// * Group: [`PixelFormat`]
pub const GL_CMYKA_EXT: GLenum = GLenum(0x800D);

///
/// * Group: [`PixelFormat`]
pub const GL_CMYK_EXT: GLenum = GLenum(0x800C);

///
/// * Group: [`FragmentOpATI`]
pub const GL_CND0_ATI: GLenum = GLenum(0x896B);

///
/// * Group: [`FragmentOpATI`]
pub const GL_CND_ATI: GLenum = GLenum(0x896A);

///
/// * Group: [`MapQuery`], [`GetMapQuery`]
pub const GL_COEFF: GLenum = GLenum(0x0A00);

///
/// * Group: [`Buffer`], [`PixelCopyType`], [`InvalidateFramebufferAttachment`]
pub const GL_COLOR: GLenum = GLenum(0x1800);

///
/// * Group: [`VertexHintsMaskPGI`]
pub const GL_COLOR3_BIT_PGI: GLbitfield = GLbitfield(0x00010000);

///
/// * Group: [`VertexHintsMaskPGI`]
pub const GL_COLOR4_BIT_PGI: GLbitfield = GLbitfield(0x00020000);

pub const GL_COLORBURN: GLenum = GLenum(0x929A);

pub const GL_COLORBURN_KHR: GLenum = GLenum(0x929A);

pub const GL_COLORBURN_NV: GLenum = GLenum(0x929A);

pub const GL_COLORDODGE: GLenum = GLenum(0x9299);

pub const GL_COLORDODGE_KHR: GLenum = GLenum(0x9299);

pub const GL_COLORDODGE_NV: GLenum = GLenum(0x9299);

pub const GL_COLOR_ALPHA_PAIRING_ATI: GLenum = GLenum(0x8975);

///
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_COLOR_ARRAY: GLenum = GLenum(0x8076);

pub const GL_COLOR_ARRAY_ADDRESS_NV: GLenum = GLenum(0x8F23);

pub const GL_COLOR_ARRAY_BUFFER_BINDING: GLenum = GLenum(0x8898);

pub const GL_COLOR_ARRAY_BUFFER_BINDING_ARB: GLenum = GLenum(0x8898);

///
/// * Group: [`GetPName`]
pub const GL_COLOR_ARRAY_COUNT_EXT: GLenum = GLenum(0x8084);

pub const GL_COLOR_ARRAY_EXT: GLenum = GLenum(0x8076);

pub const GL_COLOR_ARRAY_LENGTH_NV: GLenum = GLenum(0x8F2D);

pub const GL_COLOR_ARRAY_LIST_IBM: GLenum = GLenum(103072);

pub const GL_COLOR_ARRAY_LIST_STRIDE_IBM: GLenum = GLenum(103082);

pub const GL_COLOR_ARRAY_PARALLEL_POINTERS_INTEL: GLenum = GLenum(0x83F7);

///
/// * Group: [`GetPointervPName`]
pub const GL_COLOR_ARRAY_POINTER: GLenum = GLenum(0x8090);

///
/// * Group: [`GetPointervPName`]
pub const GL_COLOR_ARRAY_POINTER_EXT: GLenum = GLenum(0x8090);

///
/// * Group: [`GetPName`]
pub const GL_COLOR_ARRAY_SIZE: GLenum = GLenum(0x8081);

pub const GL_COLOR_ARRAY_SIZE_EXT: GLenum = GLenum(0x8081);

///
/// * Group: [`GetPName`]
pub const GL_COLOR_ARRAY_STRIDE: GLenum = GLenum(0x8083);

pub const GL_COLOR_ARRAY_STRIDE_EXT: GLenum = GLenum(0x8083);

///
/// * Group: [`GetPName`]
pub const GL_COLOR_ARRAY_TYPE: GLenum = GLenum(0x8082);

pub const GL_COLOR_ARRAY_TYPE_EXT: GLenum = GLenum(0x8082);

///
/// * Group: [`ColorBuffer`], [`DrawBufferMode`], [`ReadBufferMode`],
///   [`FramebufferAttachment`], [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT0: GLenum = GLenum(0x8CE0);

///
/// * Group: [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT0_EXT: GLenum = GLenum(0x8CE0);

///
/// * Group: [`InvalidateFramebufferAttachment`], [`DrawBufferModeATI`]
pub const GL_COLOR_ATTACHMENT0_NV: GLenum = GLenum(0x8CE0);

///
/// * Group: [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT0_OES: GLenum = GLenum(0x8CE0);

///
/// * Group: [`ColorBuffer`], [`DrawBufferMode`], [`ReadBufferMode`],
///   [`FramebufferAttachment`], [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT1: GLenum = GLenum(0x8CE1);

///
/// * Group: [`ColorBuffer`], [`DrawBufferMode`], [`ReadBufferMode`],
///   [`FramebufferAttachment`], [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT10: GLenum = GLenum(0x8CEA);

///
/// * Group: [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT10_EXT: GLenum = GLenum(0x8CEA);

///
/// * Group: [`InvalidateFramebufferAttachment`], [`DrawBufferModeATI`]
pub const GL_COLOR_ATTACHMENT10_NV: GLenum = GLenum(0x8CEA);

///
/// * Group: [`ColorBuffer`], [`DrawBufferMode`], [`ReadBufferMode`],
///   [`FramebufferAttachment`], [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT11: GLenum = GLenum(0x8CEB);

///
/// * Group: [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT11_EXT: GLenum = GLenum(0x8CEB);

///
/// * Group: [`InvalidateFramebufferAttachment`], [`DrawBufferModeATI`]
pub const GL_COLOR_ATTACHMENT11_NV: GLenum = GLenum(0x8CEB);

///
/// * Group: [`ColorBuffer`], [`DrawBufferMode`], [`ReadBufferMode`],
///   [`FramebufferAttachment`], [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT12: GLenum = GLenum(0x8CEC);

///
/// * Group: [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT12_EXT: GLenum = GLenum(0x8CEC);

///
/// * Group: [`InvalidateFramebufferAttachment`], [`DrawBufferModeATI`]
pub const GL_COLOR_ATTACHMENT12_NV: GLenum = GLenum(0x8CEC);

///
/// * Group: [`ColorBuffer`], [`DrawBufferMode`], [`ReadBufferMode`],
///   [`FramebufferAttachment`], [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT13: GLenum = GLenum(0x8CED);

///
/// * Group: [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT13_EXT: GLenum = GLenum(0x8CED);

///
/// * Group: [`InvalidateFramebufferAttachment`], [`DrawBufferModeATI`]
pub const GL_COLOR_ATTACHMENT13_NV: GLenum = GLenum(0x8CED);

///
/// * Group: [`ColorBuffer`], [`DrawBufferMode`], [`ReadBufferMode`],
///   [`FramebufferAttachment`], [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT14: GLenum = GLenum(0x8CEE);

///
/// * Group: [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT14_EXT: GLenum = GLenum(0x8CEE);

///
/// * Group: [`InvalidateFramebufferAttachment`], [`DrawBufferModeATI`]
pub const GL_COLOR_ATTACHMENT14_NV: GLenum = GLenum(0x8CEE);

///
/// * Group: [`ColorBuffer`], [`DrawBufferMode`], [`ReadBufferMode`],
///   [`FramebufferAttachment`], [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT15: GLenum = GLenum(0x8CEF);

///
/// * Group: [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT15_EXT: GLenum = GLenum(0x8CEF);

///
/// * Group: [`InvalidateFramebufferAttachment`], [`DrawBufferModeATI`]
pub const GL_COLOR_ATTACHMENT15_NV: GLenum = GLenum(0x8CEF);

///
/// * Group: [`ColorBuffer`], [`DrawBufferMode`], [`FramebufferAttachment`],
///   [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT16: GLenum = GLenum(0x8CF0);

///
/// * Group: [`ColorBuffer`], [`DrawBufferMode`], [`FramebufferAttachment`],
///   [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT17: GLenum = GLenum(0x8CF1);

///
/// * Group: [`ColorBuffer`], [`DrawBufferMode`], [`FramebufferAttachment`],
///   [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT18: GLenum = GLenum(0x8CF2);

///
/// * Group: [`ColorBuffer`], [`DrawBufferMode`], [`FramebufferAttachment`],
///   [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT19: GLenum = GLenum(0x8CF3);

///
/// * Group: [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT1_EXT: GLenum = GLenum(0x8CE1);

///
/// * Group: [`InvalidateFramebufferAttachment`], [`DrawBufferModeATI`]
pub const GL_COLOR_ATTACHMENT1_NV: GLenum = GLenum(0x8CE1);

///
/// * Group: [`ColorBuffer`], [`DrawBufferMode`], [`ReadBufferMode`],
///   [`FramebufferAttachment`], [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT2: GLenum = GLenum(0x8CE2);

///
/// * Group: [`ColorBuffer`], [`DrawBufferMode`], [`FramebufferAttachment`],
///   [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT20: GLenum = GLenum(0x8CF4);

///
/// * Group: [`ColorBuffer`], [`DrawBufferMode`], [`FramebufferAttachment`],
///   [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT21: GLenum = GLenum(0x8CF5);

///
/// * Group: [`ColorBuffer`], [`DrawBufferMode`], [`FramebufferAttachment`],
///   [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT22: GLenum = GLenum(0x8CF6);

///
/// * Group: [`ColorBuffer`], [`DrawBufferMode`], [`FramebufferAttachment`],
///   [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT23: GLenum = GLenum(0x8CF7);

///
/// * Group: [`ColorBuffer`], [`DrawBufferMode`], [`FramebufferAttachment`],
///   [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT24: GLenum = GLenum(0x8CF8);

///
/// * Group: [`ColorBuffer`], [`DrawBufferMode`], [`FramebufferAttachment`],
///   [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT25: GLenum = GLenum(0x8CF9);

///
/// * Group: [`ColorBuffer`], [`DrawBufferMode`], [`FramebufferAttachment`],
///   [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT26: GLenum = GLenum(0x8CFA);

///
/// * Group: [`ColorBuffer`], [`DrawBufferMode`], [`FramebufferAttachment`],
///   [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT27: GLenum = GLenum(0x8CFB);

///
/// * Group: [`ColorBuffer`], [`DrawBufferMode`], [`FramebufferAttachment`],
///   [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT28: GLenum = GLenum(0x8CFC);

///
/// * Group: [`ColorBuffer`], [`DrawBufferMode`], [`FramebufferAttachment`],
///   [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT29: GLenum = GLenum(0x8CFD);

///
/// * Group: [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT2_EXT: GLenum = GLenum(0x8CE2);

///
/// * Group: [`InvalidateFramebufferAttachment`], [`DrawBufferModeATI`]
pub const GL_COLOR_ATTACHMENT2_NV: GLenum = GLenum(0x8CE2);

///
/// * Group: [`ColorBuffer`], [`DrawBufferMode`], [`ReadBufferMode`],
///   [`FramebufferAttachment`], [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT3: GLenum = GLenum(0x8CE3);

///
/// * Group: [`ColorBuffer`], [`DrawBufferMode`], [`FramebufferAttachment`],
///   [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT30: GLenum = GLenum(0x8CFE);

///
/// * Group: [`ColorBuffer`], [`DrawBufferMode`], [`FramebufferAttachment`],
///   [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT31: GLenum = GLenum(0x8CFF);

///
/// * Group: [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT3_EXT: GLenum = GLenum(0x8CE3);

///
/// * Group: [`InvalidateFramebufferAttachment`], [`DrawBufferModeATI`]
pub const GL_COLOR_ATTACHMENT3_NV: GLenum = GLenum(0x8CE3);

///
/// * Group: [`ColorBuffer`], [`DrawBufferMode`], [`ReadBufferMode`],
///   [`FramebufferAttachment`], [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT4: GLenum = GLenum(0x8CE4);

///
/// * Group: [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT4_EXT: GLenum = GLenum(0x8CE4);

///
/// * Group: [`InvalidateFramebufferAttachment`], [`DrawBufferModeATI`]
pub const GL_COLOR_ATTACHMENT4_NV: GLenum = GLenum(0x8CE4);

///
/// * Group: [`ColorBuffer`], [`DrawBufferMode`], [`ReadBufferMode`],
///   [`FramebufferAttachment`], [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT5: GLenum = GLenum(0x8CE5);

///
/// * Group: [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT5_EXT: GLenum = GLenum(0x8CE5);

///
/// * Group: [`InvalidateFramebufferAttachment`], [`DrawBufferModeATI`]
pub const GL_COLOR_ATTACHMENT5_NV: GLenum = GLenum(0x8CE5);

///
/// * Group: [`ColorBuffer`], [`DrawBufferMode`], [`ReadBufferMode`],
///   [`FramebufferAttachment`], [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT6: GLenum = GLenum(0x8CE6);

///
/// * Group: [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT6_EXT: GLenum = GLenum(0x8CE6);

///
/// * Group: [`InvalidateFramebufferAttachment`], [`DrawBufferModeATI`]
pub const GL_COLOR_ATTACHMENT6_NV: GLenum = GLenum(0x8CE6);

///
/// * Group: [`ColorBuffer`], [`DrawBufferMode`], [`ReadBufferMode`],
///   [`FramebufferAttachment`], [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT7: GLenum = GLenum(0x8CE7);

///
/// * Group: [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT7_EXT: GLenum = GLenum(0x8CE7);

///
/// * Group: [`InvalidateFramebufferAttachment`], [`DrawBufferModeATI`]
pub const GL_COLOR_ATTACHMENT7_NV: GLenum = GLenum(0x8CE7);

///
/// * Group: [`ColorBuffer`], [`DrawBufferMode`], [`ReadBufferMode`],
///   [`FramebufferAttachment`], [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT8: GLenum = GLenum(0x8CE8);

///
/// * Group: [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT8_EXT: GLenum = GLenum(0x8CE8);

///
/// * Group: [`InvalidateFramebufferAttachment`], [`DrawBufferModeATI`]
pub const GL_COLOR_ATTACHMENT8_NV: GLenum = GLenum(0x8CE8);

///
/// * Group: [`ColorBuffer`], [`DrawBufferMode`], [`ReadBufferMode`],
///   [`FramebufferAttachment`], [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT9: GLenum = GLenum(0x8CE9);

///
/// * Group: [`InvalidateFramebufferAttachment`]
pub const GL_COLOR_ATTACHMENT9_EXT: GLenum = GLenum(0x8CE9);

///
/// * Group: [`InvalidateFramebufferAttachment`], [`DrawBufferModeATI`]
pub const GL_COLOR_ATTACHMENT9_NV: GLenum = GLenum(0x8CE9);

pub const GL_COLOR_ATTACHMENT_EXT: GLenum = GLenum(0x90F0);

///
/// * Group: [`ClearBufferMask`], [`AttribMask`]
pub const GL_COLOR_BUFFER_BIT: GLbitfield = GLbitfield(0x00004000);

///
/// * Group: [`BufferBitQCOM`]
pub const GL_COLOR_BUFFER_BIT0_QCOM: GLbitfield = GLbitfield(0x00000001);

///
/// * Group: [`BufferBitQCOM`]
pub const GL_COLOR_BUFFER_BIT1_QCOM: GLbitfield = GLbitfield(0x00000002);

///
/// * Group: [`BufferBitQCOM`]
pub const GL_COLOR_BUFFER_BIT2_QCOM: GLbitfield = GLbitfield(0x00000004);

///
/// * Group: [`BufferBitQCOM`]
pub const GL_COLOR_BUFFER_BIT3_QCOM: GLbitfield = GLbitfield(0x00000008);

///
/// * Group: [`BufferBitQCOM`]
pub const GL_COLOR_BUFFER_BIT4_QCOM: GLbitfield = GLbitfield(0x00000010);

///
/// * Group: [`BufferBitQCOM`]
pub const GL_COLOR_BUFFER_BIT5_QCOM: GLbitfield = GLbitfield(0x00000020);

///
/// * Group: [`BufferBitQCOM`]
pub const GL_COLOR_BUFFER_BIT6_QCOM: GLbitfield = GLbitfield(0x00000040);

///
/// * Group: [`BufferBitQCOM`]
pub const GL_COLOR_BUFFER_BIT7_QCOM: GLbitfield = GLbitfield(0x00000080);

pub const GL_COLOR_CLEAR_UNCLAMPED_VALUE_ATI: GLenum = GLenum(0x8835);

///
/// * Group: [`GetPName`]
pub const GL_COLOR_CLEAR_VALUE: GLenum = GLenum(0x0C22);

///
/// * Group: [`InternalFormatPName`]
pub const GL_COLOR_COMPONENTS: GLenum = GLenum(0x8283);

///
/// * Group: [`InternalFormatPName`]
pub const GL_COLOR_ENCODING: GLenum = GLenum(0x8296);

///
/// * Group: [`PixelCopyType`]
pub const GL_COLOR_EXT: GLenum = GLenum(0x1800);

pub const GL_COLOR_FLOAT_APPLE: GLenum = GLenum(0x8A0F);

///
/// * Group: [`PixelFormat`]
pub const GL_COLOR_INDEX: GLenum = GLenum(0x1900);

pub const GL_COLOR_INDEX12_EXT: GLenum = GLenum(0x80E6);

pub const GL_COLOR_INDEX16_EXT: GLenum = GLenum(0x80E7);

pub const GL_COLOR_INDEX1_EXT: GLenum = GLenum(0x80E2);

pub const GL_COLOR_INDEX2_EXT: GLenum = GLenum(0x80E3);

pub const GL_COLOR_INDEX4_EXT: GLenum = GLenum(0x80E4);

pub const GL_COLOR_INDEX8_EXT: GLenum = GLenum(0x80E5);

///
/// * Group: [`MaterialParameter`]
pub const GL_COLOR_INDEXES: GLenum = GLenum(0x1603);

///
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_COLOR_LOGIC_OP: GLenum = GLenum(0x0BF2);

///
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_COLOR_MATERIAL: GLenum = GLenum(0x0B57);

///
/// * Group: [`GetPName`]
pub const GL_COLOR_MATERIAL_FACE: GLenum = GLenum(0x0B55);

///
/// * Group: [`GetPName`]
pub const GL_COLOR_MATERIAL_PARAMETER: GLenum = GLenum(0x0B56);

pub const GL_COLOR_MATRIX: GLenum = GLenum(0x80B1);

///
/// * Group: [`GetPName`]
pub const GL_COLOR_MATRIX_SGI: GLenum = GLenum(0x80B1);

pub const GL_COLOR_MATRIX_STACK_DEPTH: GLenum = GLenum(0x80B2);

///
/// * Group: [`GetPName`]
pub const GL_COLOR_MATRIX_STACK_DEPTH_SGI: GLenum = GLenum(0x80B2);

///
/// * Group: [`InternalFormatPName`]
pub const GL_COLOR_RENDERABLE: GLenum = GLenum(0x8286);

pub const GL_COLOR_SAMPLES_NV: GLenum = GLenum(0x8E20);

pub const GL_COLOR_SUM: GLenum = GLenum(0x8458);

pub const GL_COLOR_SUM_ARB: GLenum = GLenum(0x8458);

pub const GL_COLOR_SUM_CLAMP_NV: GLenum = GLenum(0x854F);

pub const GL_COLOR_SUM_EXT: GLenum = GLenum(0x8458);

///
/// * Group: [`ColorTableTarget`], [`ColorTableTargetSGI`], [`EnableCap`]
pub const GL_COLOR_TABLE: GLenum = GLenum(0x80D0);

///
/// * Group: [`GetColorTableParameterPNameSGI`],
///   [`ColorTableParameterPNameSGI`], [`GetColorTableParameterPName`],
///   [`ColorTableParameterPName`]
pub const GL_COLOR_TABLE_ALPHA_SIZE: GLenum = GLenum(0x80DD);

///
/// * Group: [`GetColorTableParameterPNameSGI`], [`ColorTableParameterPNameSGI`]
pub const GL_COLOR_TABLE_ALPHA_SIZE_SGI: GLenum = GLenum(0x80DD);

///
/// * Group: [`GetColorTableParameterPNameSGI`],
///   [`ColorTableParameterPNameSGI`], [`GetColorTableParameterPName`],
///   [`ColorTableParameterPName`]
pub const GL_COLOR_TABLE_BIAS: GLenum = GLenum(0x80D7);

///
/// * Group: [`GetColorTableParameterPNameSGI`], [`ColorTableParameterPNameSGI`]
pub const GL_COLOR_TABLE_BIAS_SGI: GLenum = GLenum(0x80D7);

///
/// * Group: [`GetColorTableParameterPNameSGI`],
///   [`ColorTableParameterPNameSGI`], [`GetColorTableParameterPName`],
///   [`ColorTableParameterPName`]
pub const GL_COLOR_TABLE_BLUE_SIZE: GLenum = GLenum(0x80DC);

///
/// * Group: [`GetColorTableParameterPNameSGI`], [`ColorTableParameterPNameSGI`]
pub const GL_COLOR_TABLE_BLUE_SIZE_SGI: GLenum = GLenum(0x80DC);

///
/// * Group: [`GetColorTableParameterPNameSGI`],
///   [`ColorTableParameterPNameSGI`], [`GetColorTableParameterPName`],
///   [`ColorTableParameterPName`]
pub const GL_COLOR_TABLE_FORMAT: GLenum = GLenum(0x80D8);

///
/// * Group: [`GetColorTableParameterPNameSGI`], [`ColorTableParameterPNameSGI`]
pub const GL_COLOR_TABLE_FORMAT_SGI: GLenum = GLenum(0x80D8);

///
/// * Group: [`GetColorTableParameterPNameSGI`],
///   [`ColorTableParameterPNameSGI`], [`GetColorTableParameterPName`],
///   [`ColorTableParameterPName`]
pub const GL_COLOR_TABLE_GREEN_SIZE: GLenum = GLenum(0x80DB);

///
/// * Group: [`GetColorTableParameterPNameSGI`], [`ColorTableParameterPNameSGI`]
pub const GL_COLOR_TABLE_GREEN_SIZE_SGI: GLenum = GLenum(0x80DB);

///
/// * Group: [`GetColorTableParameterPNameSGI`],
///   [`ColorTableParameterPNameSGI`], [`GetColorTableParameterPName`],
///   [`ColorTableParameterPName`]
pub const GL_COLOR_TABLE_INTENSITY_SIZE: GLenum = GLenum(0x80DF);

///
/// * Group: [`GetColorTableParameterPNameSGI`], [`ColorTableParameterPNameSGI`]
pub const GL_COLOR_TABLE_INTENSITY_SIZE_SGI: GLenum = GLenum(0x80DF);

///
/// * Group: [`GetColorTableParameterPNameSGI`],
///   [`ColorTableParameterPNameSGI`], [`GetColorTableParameterPName`],
///   [`ColorTableParameterPName`]
pub const GL_COLOR_TABLE_LUMINANCE_SIZE: GLenum = GLenum(0x80DE);

///
/// * Group: [`GetColorTableParameterPNameSGI`], [`ColorTableParameterPNameSGI`]
pub const GL_COLOR_TABLE_LUMINANCE_SIZE_SGI: GLenum = GLenum(0x80DE);

///
/// * Group: [`GetColorTableParameterPNameSGI`],
///   [`ColorTableParameterPNameSGI`], [`GetColorTableParameterPName`],
///   [`ColorTableParameterPName`]
pub const GL_COLOR_TABLE_RED_SIZE: GLenum = GLenum(0x80DA);

///
/// * Group: [`GetColorTableParameterPNameSGI`], [`ColorTableParameterPNameSGI`]
pub const GL_COLOR_TABLE_RED_SIZE_SGI: GLenum = GLenum(0x80DA);

///
/// * Group: [`GetColorTableParameterPNameSGI`],
///   [`ColorTableParameterPNameSGI`], [`GetColorTableParameterPName`],
///   [`ColorTableParameterPName`]
pub const GL_COLOR_TABLE_SCALE: GLenum = GLenum(0x80D6);

///
/// * Group: [`GetColorTableParameterPNameSGI`], [`ColorTableParameterPNameSGI`]
pub const GL_COLOR_TABLE_SCALE_SGI: GLenum = GLenum(0x80D6);

///
/// * Group: [`GetPName`], [`ColorTableTargetSGI`], [`EnableCap`]
pub const GL_COLOR_TABLE_SGI: GLenum = GLenum(0x80D0);

///
/// * Group: [`GetColorTableParameterPNameSGI`],
///   [`ColorTableParameterPNameSGI`], [`GetColorTableParameterPName`],
///   [`ColorTableParameterPName`]
pub const GL_COLOR_TABLE_WIDTH: GLenum = GLenum(0x80D9);

///
/// * Group: [`GetColorTableParameterPNameSGI`], [`ColorTableParameterPNameSGI`]
pub const GL_COLOR_TABLE_WIDTH_SGI: GLenum = GLenum(0x80D9);

///
/// * Group: [`GetPName`]
pub const GL_COLOR_WRITEMASK: GLenum = GLenum(0x0C23);

pub const GL_COMBINE: GLenum = GLenum(0x8570);

pub const GL_COMBINE4_NV: GLenum = GLenum(0x8503);

///
/// * Group: [`CombinerStageNV`]
pub const GL_COMBINER0_NV: GLenum = GLenum(0x8550);

///
/// * Group: [`CombinerStageNV`]
pub const GL_COMBINER1_NV: GLenum = GLenum(0x8551);

///
/// * Group: [`CombinerStageNV`]
pub const GL_COMBINER2_NV: GLenum = GLenum(0x8552);

///
/// * Group: [`CombinerStageNV`]
pub const GL_COMBINER3_NV: GLenum = GLenum(0x8553);

///
/// * Group: [`CombinerStageNV`]
pub const GL_COMBINER4_NV: GLenum = GLenum(0x8554);

///
/// * Group: [`CombinerStageNV`]
pub const GL_COMBINER5_NV: GLenum = GLenum(0x8555);

///
/// * Group: [`CombinerStageNV`]
pub const GL_COMBINER6_NV: GLenum = GLenum(0x8556);

///
/// * Group: [`CombinerStageNV`]
pub const GL_COMBINER7_NV: GLenum = GLenum(0x8557);

pub const GL_COMBINER_AB_DOT_PRODUCT_NV: GLenum = GLenum(0x8545);

pub const GL_COMBINER_AB_OUTPUT_NV: GLenum = GLenum(0x854A);

pub const GL_COMBINER_BIAS_NV: GLenum = GLenum(0x8549);

pub const GL_COMBINER_CD_DOT_PRODUCT_NV: GLenum = GLenum(0x8546);

pub const GL_COMBINER_CD_OUTPUT_NV: GLenum = GLenum(0x854B);

///
/// * Group: [`CombinerParameterNV`]
pub const GL_COMBINER_COMPONENT_USAGE_NV: GLenum = GLenum(0x8544);

///
/// * Group: [`CombinerParameterNV`]
pub const GL_COMBINER_INPUT_NV: GLenum = GLenum(0x8542);

///
/// * Group: [`CombinerParameterNV`]
pub const GL_COMBINER_MAPPING_NV: GLenum = GLenum(0x8543);

pub const GL_COMBINER_MUX_SUM_NV: GLenum = GLenum(0x8547);

pub const GL_COMBINER_SCALE_NV: GLenum = GLenum(0x8548);

pub const GL_COMBINER_SUM_OUTPUT_NV: GLenum = GLenum(0x854C);

pub const GL_COMBINE_ALPHA: GLenum = GLenum(0x8572);

pub const GL_COMBINE_ALPHA_ARB: GLenum = GLenum(0x8572);

pub const GL_COMBINE_ALPHA_EXT: GLenum = GLenum(0x8572);

pub const GL_COMBINE_ARB: GLenum = GLenum(0x8570);

pub const GL_COMBINE_EXT: GLenum = GLenum(0x8570);

pub const GL_COMBINE_RGB: GLenum = GLenum(0x8571);

pub const GL_COMBINE_RGB_ARB: GLenum = GLenum(0x8571);

pub const GL_COMBINE_RGB_EXT: GLenum = GLenum(0x8571);

///
/// * Group: [`MemoryBarrierMask`]
pub const GL_COMMAND_BARRIER_BIT: GLbitfield = GLbitfield(0x00000040);

///
/// * Group: [`MemoryBarrierMask`]
pub const GL_COMMAND_BARRIER_BIT_EXT: GLbitfield = GLbitfield(0x00000040);

pub const GL_COMPARE_REF_DEPTH_TO_TEXTURE_EXT: GLenum = GLenum(0x884E);

///
/// * Group: [`TextureCompareMode`]
/// * Alias Of: [`GL_COMPARE_R_TO_TEXTURE`]
pub const GL_COMPARE_REF_TO_TEXTURE: GLenum = GL_COMPARE_R_TO_TEXTURE;

pub const GL_COMPARE_REF_TO_TEXTURE_EXT: GLenum = GLenum(0x884E);

///
/// * Group: [`TextureCompareMode`]
pub const GL_COMPARE_R_TO_TEXTURE: GLenum = GLenum(0x884E);

pub const GL_COMPARE_R_TO_TEXTURE_ARB: GLenum = GLenum(0x884E);

///
/// * Group: [`ProgramResourceProperty`], [`SubroutineParameterName`]
pub const GL_COMPATIBLE_SUBROUTINES: GLenum = GLenum(0x8E4B);

///
/// * Group: [`ListMode`]
pub const GL_COMPILE: GLenum = GLenum(0x1300);

///
/// * Group: [`ListMode`]
pub const GL_COMPILE_AND_EXECUTE: GLenum = GLenum(0x1301);

///
/// * Group: [`ShaderParameterName`]
pub const GL_COMPILE_STATUS: GLenum = GLenum(0x8B81);

///
/// * Alias Of: [`GL_COMPLETION_STATUS_KHR`]
pub const GL_COMPLETION_STATUS_ARB: GLenum = GL_COMPLETION_STATUS_KHR;

pub const GL_COMPLETION_STATUS_KHR: GLenum = GLenum(0x91B1);

pub const GL_COMPRESSED_ALPHA: GLenum = GLenum(0x84E9);

pub const GL_COMPRESSED_ALPHA_ARB: GLenum = GLenum(0x84E9);

pub const GL_COMPRESSED_INTENSITY: GLenum = GLenum(0x84EC);

pub const GL_COMPRESSED_INTENSITY_ARB: GLenum = GLenum(0x84EC);

pub const GL_COMPRESSED_LUMINANCE: GLenum = GLenum(0x84EA);

pub const GL_COMPRESSED_LUMINANCE_ALPHA: GLenum = GLenum(0x84EB);

/// Defined by Mesa but not ATI
pub const GL_COMPRESSED_LUMINANCE_ALPHA_3DC_ATI: GLenum = GLenum(0x8837);

pub const GL_COMPRESSED_LUMINANCE_ALPHA_ARB: GLenum = GLenum(0x84EB);

pub const GL_COMPRESSED_LUMINANCE_ALPHA_LATC2_EXT: GLenum = GLenum(0x8C72);

pub const GL_COMPRESSED_LUMINANCE_ARB: GLenum = GLenum(0x84EA);

pub const GL_COMPRESSED_LUMINANCE_LATC1_EXT: GLenum = GLenum(0x8C70);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_R11_EAC: GLenum = GLenum(0x9270);

pub const GL_COMPRESSED_R11_EAC_OES: GLenum = GLenum(0x9270);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RED: GLenum = GLenum(0x8225);

pub const GL_COMPRESSED_RED_GREEN_RGTC2_EXT: GLenum = GLenum(0x8DBD);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RED_RGTC1: GLenum = GLenum(0x8DBB);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RED_RGTC1_EXT: GLenum = GLenum(0x8DBB);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RG: GLenum = GLenum(0x8226);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RG11_EAC: GLenum = GLenum(0x9272);

pub const GL_COMPRESSED_RG11_EAC_OES: GLenum = GLenum(0x9272);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGB: GLenum = GLenum(0x84ED);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGB8_ETC2: GLenum = GLenum(0x9274);

pub const GL_COMPRESSED_RGB8_ETC2_OES: GLenum = GLenum(0x9274);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGB8_PUNCHTHROUGH_ALPHA1_ETC2: GLenum = GLenum(0x9276);

pub const GL_COMPRESSED_RGB8_PUNCHTHROUGH_ALPHA1_ETC2_OES: GLenum = GLenum(0x9276);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGBA: GLenum = GLenum(0x84EE);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGBA8_ETC2_EAC: GLenum = GLenum(0x9278);

pub const GL_COMPRESSED_RGBA8_ETC2_EAC_OES: GLenum = GLenum(0x9278);

pub const GL_COMPRESSED_RGBA_ARB: GLenum = GLenum(0x84EE);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGBA_ASTC_10x10: GLenum = GLenum(0x93BB);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGBA_ASTC_10x10_KHR: GLenum = GLenum(0x93BB);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGBA_ASTC_10x5: GLenum = GLenum(0x93B8);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGBA_ASTC_10x5_KHR: GLenum = GLenum(0x93B8);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGBA_ASTC_10x6: GLenum = GLenum(0x93B9);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGBA_ASTC_10x6_KHR: GLenum = GLenum(0x93B9);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGBA_ASTC_10x8: GLenum = GLenum(0x93BA);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGBA_ASTC_10x8_KHR: GLenum = GLenum(0x93BA);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGBA_ASTC_12x10: GLenum = GLenum(0x93BC);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGBA_ASTC_12x10_KHR: GLenum = GLenum(0x93BC);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGBA_ASTC_12x12: GLenum = GLenum(0x93BD);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGBA_ASTC_12x12_KHR: GLenum = GLenum(0x93BD);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGBA_ASTC_3x3x3_OES: GLenum = GLenum(0x93C0);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGBA_ASTC_4x3x3_OES: GLenum = GLenum(0x93C1);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGBA_ASTC_4x4: GLenum = GLenum(0x93B0);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGBA_ASTC_4x4_KHR: GLenum = GLenum(0x93B0);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGBA_ASTC_4x4x3_OES: GLenum = GLenum(0x93C2);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGBA_ASTC_4x4x4_OES: GLenum = GLenum(0x93C3);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGBA_ASTC_5x4: GLenum = GLenum(0x93B1);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGBA_ASTC_5x4_KHR: GLenum = GLenum(0x93B1);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGBA_ASTC_5x4x4_OES: GLenum = GLenum(0x93C4);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGBA_ASTC_5x5: GLenum = GLenum(0x93B2);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGBA_ASTC_5x5_KHR: GLenum = GLenum(0x93B2);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGBA_ASTC_5x5x4_OES: GLenum = GLenum(0x93C5);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGBA_ASTC_5x5x5_OES: GLenum = GLenum(0x93C6);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGBA_ASTC_6x5: GLenum = GLenum(0x93B3);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGBA_ASTC_6x5_KHR: GLenum = GLenum(0x93B3);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGBA_ASTC_6x5x5_OES: GLenum = GLenum(0x93C7);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGBA_ASTC_6x6: GLenum = GLenum(0x93B4);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGBA_ASTC_6x6_KHR: GLenum = GLenum(0x93B4);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGBA_ASTC_6x6x5_OES: GLenum = GLenum(0x93C8);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGBA_ASTC_6x6x6_OES: GLenum = GLenum(0x93C9);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGBA_ASTC_8x5: GLenum = GLenum(0x93B5);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGBA_ASTC_8x5_KHR: GLenum = GLenum(0x93B5);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGBA_ASTC_8x6: GLenum = GLenum(0x93B6);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGBA_ASTC_8x6_KHR: GLenum = GLenum(0x93B6);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGBA_ASTC_8x8: GLenum = GLenum(0x93B7);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGBA_ASTC_8x8_KHR: GLenum = GLenum(0x93B7);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGBA_BPTC_UNORM: GLenum = GLenum(0x8E8C);

pub const GL_COMPRESSED_RGBA_BPTC_UNORM_ARB: GLenum = GLenum(0x8E8C);

pub const GL_COMPRESSED_RGBA_BPTC_UNORM_EXT: GLenum = GLenum(0x8E8C);

pub const GL_COMPRESSED_RGBA_FXT1_3DFX: GLenum = GLenum(0x86B1);

pub const GL_COMPRESSED_RGBA_PVRTC_2BPPV1_IMG: GLenum = GLenum(0x8C03);

pub const GL_COMPRESSED_RGBA_PVRTC_2BPPV2_IMG: GLenum = GLenum(0x9137);

pub const GL_COMPRESSED_RGBA_PVRTC_4BPPV1_IMG: GLenum = GLenum(0x8C02);

pub const GL_COMPRESSED_RGBA_PVRTC_4BPPV2_IMG: GLenum = GLenum(0x9138);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGBA_S3TC_DXT1_EXT: GLenum = GLenum(0x83F1);

pub const GL_COMPRESSED_RGBA_S3TC_DXT3_ANGLE: GLenum = GLenum(0x83F2);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGBA_S3TC_DXT3_EXT: GLenum = GLenum(0x83F2);

pub const GL_COMPRESSED_RGBA_S3TC_DXT5_ANGLE: GLenum = GLenum(0x83F3);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGBA_S3TC_DXT5_EXT: GLenum = GLenum(0x83F3);

pub const GL_COMPRESSED_RGB_ARB: GLenum = GLenum(0x84ED);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGB_BPTC_SIGNED_FLOAT: GLenum = GLenum(0x8E8E);

pub const GL_COMPRESSED_RGB_BPTC_SIGNED_FLOAT_ARB: GLenum = GLenum(0x8E8E);

pub const GL_COMPRESSED_RGB_BPTC_SIGNED_FLOAT_EXT: GLenum = GLenum(0x8E8E);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGB_BPTC_UNSIGNED_FLOAT: GLenum = GLenum(0x8E8F);

pub const GL_COMPRESSED_RGB_BPTC_UNSIGNED_FLOAT_ARB: GLenum = GLenum(0x8E8F);

pub const GL_COMPRESSED_RGB_BPTC_UNSIGNED_FLOAT_EXT: GLenum = GLenum(0x8E8F);

pub const GL_COMPRESSED_RGB_FXT1_3DFX: GLenum = GLenum(0x86B0);

pub const GL_COMPRESSED_RGB_PVRTC_2BPPV1_IMG: GLenum = GLenum(0x8C01);

pub const GL_COMPRESSED_RGB_PVRTC_4BPPV1_IMG: GLenum = GLenum(0x8C00);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RGB_S3TC_DXT1_EXT: GLenum = GLenum(0x83F0);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_RG_RGTC2: GLenum = GLenum(0x8DBD);

pub const GL_COMPRESSED_SIGNED_LUMINANCE_ALPHA_LATC2_EXT: GLenum = GLenum(0x8C73);

pub const GL_COMPRESSED_SIGNED_LUMINANCE_LATC1_EXT: GLenum = GLenum(0x8C71);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SIGNED_R11_EAC: GLenum = GLenum(0x9271);

pub const GL_COMPRESSED_SIGNED_R11_EAC_OES: GLenum = GLenum(0x9271);

pub const GL_COMPRESSED_SIGNED_RED_GREEN_RGTC2_EXT: GLenum = GLenum(0x8DBE);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SIGNED_RED_RGTC1: GLenum = GLenum(0x8DBC);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SIGNED_RED_RGTC1_EXT: GLenum = GLenum(0x8DBC);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SIGNED_RG11_EAC: GLenum = GLenum(0x9273);

pub const GL_COMPRESSED_SIGNED_RG11_EAC_OES: GLenum = GLenum(0x9273);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SIGNED_RG_RGTC2: GLenum = GLenum(0x8DBE);

pub const GL_COMPRESSED_SLUMINANCE: GLenum = GLenum(0x8C4A);

pub const GL_COMPRESSED_SLUMINANCE_ALPHA: GLenum = GLenum(0x8C4B);

pub const GL_COMPRESSED_SLUMINANCE_ALPHA_EXT: GLenum = GLenum(0x8C4B);

pub const GL_COMPRESSED_SLUMINANCE_EXT: GLenum = GLenum(0x8C4A);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB: GLenum = GLenum(0x8C48);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x10: GLenum = GLenum(0x93DB);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x10_KHR: GLenum = GLenum(0x93DB);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x5: GLenum = GLenum(0x93D8);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x5_KHR: GLenum = GLenum(0x93D8);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x6: GLenum = GLenum(0x93D9);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x6_KHR: GLenum = GLenum(0x93D9);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x8: GLenum = GLenum(0x93DA);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x8_KHR: GLenum = GLenum(0x93DA);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_12x10: GLenum = GLenum(0x93DC);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_12x10_KHR: GLenum = GLenum(0x93DC);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_12x12: GLenum = GLenum(0x93DD);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_12x12_KHR: GLenum = GLenum(0x93DD);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_3x3x3_OES: GLenum = GLenum(0x93E0);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_4x3x3_OES: GLenum = GLenum(0x93E1);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_4x4: GLenum = GLenum(0x93D0);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_4x4_KHR: GLenum = GLenum(0x93D0);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_4x4x3_OES: GLenum = GLenum(0x93E2);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_4x4x4_OES: GLenum = GLenum(0x93E3);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_5x4: GLenum = GLenum(0x93D1);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_5x4_KHR: GLenum = GLenum(0x93D1);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_5x4x4_OES: GLenum = GLenum(0x93E4);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_5x5: GLenum = GLenum(0x93D2);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_5x5_KHR: GLenum = GLenum(0x93D2);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_5x5x4_OES: GLenum = GLenum(0x93E5);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_5x5x5_OES: GLenum = GLenum(0x93E6);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_6x5: GLenum = GLenum(0x93D3);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_6x5_KHR: GLenum = GLenum(0x93D3);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_6x5x5_OES: GLenum = GLenum(0x93E7);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_6x6: GLenum = GLenum(0x93D4);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_6x6_KHR: GLenum = GLenum(0x93D4);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_6x6x5_OES: GLenum = GLenum(0x93E8);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_6x6x6_OES: GLenum = GLenum(0x93E9);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_8x5: GLenum = GLenum(0x93D5);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_8x5_KHR: GLenum = GLenum(0x93D5);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_8x6: GLenum = GLenum(0x93D6);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_8x6_KHR: GLenum = GLenum(0x93D6);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_8x8: GLenum = GLenum(0x93D7);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_8x8_KHR: GLenum = GLenum(0x93D7);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB8_ALPHA8_ETC2_EAC: GLenum = GLenum(0x9279);

pub const GL_COMPRESSED_SRGB8_ALPHA8_ETC2_EAC_OES: GLenum = GLenum(0x9279);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB8_ETC2: GLenum = GLenum(0x9275);

pub const GL_COMPRESSED_SRGB8_ETC2_OES: GLenum = GLenum(0x9275);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB8_PUNCHTHROUGH_ALPHA1_ETC2: GLenum = GLenum(0x9277);

pub const GL_COMPRESSED_SRGB8_PUNCHTHROUGH_ALPHA1_ETC2_OES: GLenum = GLenum(0x9277);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB_ALPHA: GLenum = GLenum(0x8C49);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB_ALPHA_BPTC_UNORM: GLenum = GLenum(0x8E8D);

pub const GL_COMPRESSED_SRGB_ALPHA_BPTC_UNORM_ARB: GLenum = GLenum(0x8E8D);

pub const GL_COMPRESSED_SRGB_ALPHA_BPTC_UNORM_EXT: GLenum = GLenum(0x8E8D);

pub const GL_COMPRESSED_SRGB_ALPHA_EXT: GLenum = GLenum(0x8C49);

pub const GL_COMPRESSED_SRGB_ALPHA_PVRTC_2BPPV1_EXT: GLenum = GLenum(0x8A56);

pub const GL_COMPRESSED_SRGB_ALPHA_PVRTC_2BPPV2_IMG: GLenum = GLenum(0x93F0);

pub const GL_COMPRESSED_SRGB_ALPHA_PVRTC_4BPPV1_EXT: GLenum = GLenum(0x8A57);

pub const GL_COMPRESSED_SRGB_ALPHA_PVRTC_4BPPV2_IMG: GLenum = GLenum(0x93F1);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB_ALPHA_S3TC_DXT1_EXT: GLenum = GLenum(0x8C4D);

pub const GL_COMPRESSED_SRGB_ALPHA_S3TC_DXT1_NV: GLenum = GLenum(0x8C4D);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB_ALPHA_S3TC_DXT3_EXT: GLenum = GLenum(0x8C4E);

pub const GL_COMPRESSED_SRGB_ALPHA_S3TC_DXT3_NV: GLenum = GLenum(0x8C4E);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB_ALPHA_S3TC_DXT5_EXT: GLenum = GLenum(0x8C4F);

pub const GL_COMPRESSED_SRGB_ALPHA_S3TC_DXT5_NV: GLenum = GLenum(0x8C4F);

pub const GL_COMPRESSED_SRGB_EXT: GLenum = GLenum(0x8C48);

pub const GL_COMPRESSED_SRGB_PVRTC_2BPPV1_EXT: GLenum = GLenum(0x8A54);

pub const GL_COMPRESSED_SRGB_PVRTC_4BPPV1_EXT: GLenum = GLenum(0x8A55);

///
/// * Group: [`InternalFormat`]
pub const GL_COMPRESSED_SRGB_S3TC_DXT1_EXT: GLenum = GLenum(0x8C4C);

pub const GL_COMPRESSED_SRGB_S3TC_DXT1_NV: GLenum = GLenum(0x8C4C);

///
/// * Group: [`GetPName`]
pub const GL_COMPRESSED_TEXTURE_FORMATS: GLenum = GLenum(0x86A3);

pub const GL_COMPRESSED_TEXTURE_FORMATS_ARB: GLenum = GLenum(0x86A3);

///
/// * Group: [`ProgramTarget`]
pub const GL_COMPUTE_PROGRAM_NV: GLenum = GLenum(0x90FB);

pub const GL_COMPUTE_PROGRAM_PARAMETER_BUFFER_NV: GLenum = GLenum(0x90FC);

///
/// * Group: [`ShaderType`]
pub const GL_COMPUTE_SHADER: GLenum = GLenum(0x91B9);

///
/// * Group: [`UseProgramStageMask`]
pub const GL_COMPUTE_SHADER_BIT: GLbitfield = GLbitfield(0x00000020);

pub const GL_COMPUTE_SHADER_INVOCATIONS: GLenum = GLenum(0x82F5);

///
/// * Alias Of: [`GL_COMPUTE_SHADER_INVOCATIONS`]
pub const GL_COMPUTE_SHADER_INVOCATIONS_ARB: GLenum = GL_COMPUTE_SHADER_INVOCATIONS;

///
/// * Group: [`ProgramInterface`]
pub const GL_COMPUTE_SUBROUTINE: GLenum = GLenum(0x92ED);

///
/// * Group: [`ProgramInterface`]
pub const GL_COMPUTE_SUBROUTINE_UNIFORM: GLenum = GLenum(0x92F3);

///
/// * Group: [`InternalFormatPName`]
pub const GL_COMPUTE_TEXTURE: GLenum = GLenum(0x82A0);

///
/// * Group: [`ProgramPropertyARB`]
pub const GL_COMPUTE_WORK_GROUP_SIZE: GLenum = GLenum(0x8267);

///
/// * Group: [`FragmentShaderColorModMaskATI`]
pub const GL_COMP_BIT_ATI: GLbitfield = GLbitfield(0x00000002);

///
/// * Group: [`SyncStatus`]
pub const GL_CONDITION_SATISFIED: GLenum = GLenum(0x911C);

pub const GL_CONDITION_SATISFIED_APPLE: GLenum = GLenum(0x911C);

pub const GL_CONFORMANT_NV: GLenum = GLenum(0x9374);

///
/// * Group: [`PathCoordType`]
pub const GL_CONIC_CURVE_TO_NV: GLenum = GLenum(0x1A);

pub const GL_CONJOINT_NV: GLenum = GLenum(0x9284);

pub const GL_CONSERVATIVE_RASTERIZATION_INTEL: GLenum = GLenum(0x83FE);

pub const GL_CONSERVATIVE_RASTERIZATION_NV: GLenum = GLenum(0x9346);

pub const GL_CONSERVATIVE_RASTER_DILATE_GRANULARITY_NV: GLenum = GLenum(0x937B);

pub const GL_CONSERVATIVE_RASTER_DILATE_NV: GLenum = GLenum(0x9379);

pub const GL_CONSERVATIVE_RASTER_DILATE_RANGE_NV: GLenum = GLenum(0x937A);

pub const GL_CONSERVATIVE_RASTER_MODE_NV: GLenum = GLenum(0x954D);

pub const GL_CONSERVATIVE_RASTER_MODE_POST_SNAP_NV: GLenum = GLenum(0x954E);

pub const GL_CONSERVATIVE_RASTER_MODE_PRE_SNAP_NV: GLenum = GLenum(0x9550);

pub const GL_CONSERVATIVE_RASTER_MODE_PRE_SNAP_TRIANGLES_NV: GLenum = GLenum(0x954F);

///
/// * Group: [`HintTarget`]
pub const GL_CONSERVE_MEMORY_HINT_PGI: GLenum = GLenum(0x1A1FD);

///
/// * Group: [`PathGenMode`]
pub const GL_CONSTANT: GLenum = GLenum(0x8576);

///
/// * Group: [`BlendingFactor`]
pub const GL_CONSTANT_ALPHA: GLenum = GLenum(0x8003);

pub const GL_CONSTANT_ALPHA_EXT: GLenum = GLenum(0x8003);

pub const GL_CONSTANT_ARB: GLenum = GLenum(0x8576);

///
/// * Group: [`LightParameter`], [`FragmentLightParameterSGIX`]
pub const GL_CONSTANT_ATTENUATION: GLenum = GLenum(0x1207);

pub const GL_CONSTANT_BORDER: GLenum = GLenum(0x8151);

pub const GL_CONSTANT_BORDER_HP: GLenum = GLenum(0x8151);

///
/// * Group: [`BlendingFactor`]
pub const GL_CONSTANT_COLOR: GLenum = GLenum(0x8001);

pub const GL_CONSTANT_COLOR0_NV: GLenum = GLenum(0x852A);

pub const GL_CONSTANT_COLOR1_NV: GLenum = GLenum(0x852B);

pub const GL_CONSTANT_COLOR_EXT: GLenum = GLenum(0x8001);

pub const GL_CONSTANT_EXT: GLenum = GLenum(0x8576);

pub const GL_CONSTANT_NV: GLenum = GLenum(0x8576);

pub const GL_CONST_EYE_NV: GLenum = GLenum(0x86E5);

///
/// * Group: [`ContextProfileMask`]
pub const GL_CONTEXT_COMPATIBILITY_PROFILE_BIT: GLbitfield = GLbitfield(0x00000002);

///
/// * Group: [`ContextProfileMask`]
pub const GL_CONTEXT_CORE_PROFILE_BIT: GLbitfield = GLbitfield(0x00000001);

///
/// * Group: [`GetPName`]
pub const GL_CONTEXT_FLAGS: GLenum = GLenum(0x821E);

///
/// * Group: [`ContextFlagMask`]
pub const GL_CONTEXT_FLAG_DEBUG_BIT: GLbitfield = GLbitfield(0x00000002);

///
/// * Group: [`ContextFlagMask`]
pub const GL_CONTEXT_FLAG_DEBUG_BIT_KHR: GLbitfield = GLbitfield(0x00000002);

///
/// * Group: [`ContextFlagMask`]
pub const GL_CONTEXT_FLAG_FORWARD_COMPATIBLE_BIT: GLbitfield = GLbitfield(0x00000001);

///
/// * Group: [`ContextFlagMask`]
pub const GL_CONTEXT_FLAG_NO_ERROR_BIT: GLbitfield = GLbitfield(0x00000008);

///
/// * Group: [`ContextFlagMask`]
/// * Alias Of: [`GL_CONTEXT_FLAG_NO_ERROR_BIT`]
pub const GL_CONTEXT_FLAG_NO_ERROR_BIT_KHR: GLbitfield = GL_CONTEXT_FLAG_NO_ERROR_BIT;

///
/// * Group: [`ContextFlagMask`]
pub const GL_CONTEXT_FLAG_PROTECTED_CONTENT_BIT_EXT: GLbitfield = GLbitfield(0x00000010);

///
/// * Group: [`ContextFlagMask`]
pub const GL_CONTEXT_FLAG_ROBUST_ACCESS_BIT: GLbitfield = GLbitfield(0x00000004);

///
/// * Group: [`ContextFlagMask`]
pub const GL_CONTEXT_FLAG_ROBUST_ACCESS_BIT_ARB: GLbitfield = GLbitfield(0x00000004);

pub const GL_CONTEXT_LOST: GLenum = GLenum(0x0507);

pub const GL_CONTEXT_LOST_KHR: GLenum = GLenum(0x0507);

pub const GL_CONTEXT_LOST_WEBGL: GLenum = GLenum(0x9242);

///
/// * Group: [`GetPName`]
pub const GL_CONTEXT_PROFILE_MASK: GLenum = GLenum(0x9126);

pub const GL_CONTEXT_RELEASE_BEHAVIOR: GLenum = GLenum(0x82FB);

pub const GL_CONTEXT_RELEASE_BEHAVIOR_FLUSH: GLenum = GLenum(0x82FC);

pub const GL_CONTEXT_RELEASE_BEHAVIOR_FLUSH_KHR: GLenum = GLenum(0x82FC);

pub const GL_CONTEXT_RELEASE_BEHAVIOR_KHR: GLenum = GLenum(0x82FB);

pub const GL_CONTEXT_ROBUST_ACCESS: GLenum = GLenum(0x90F3);

pub const GL_CONTEXT_ROBUST_ACCESS_EXT: GLenum = GLenum(0x90F3);

pub const GL_CONTEXT_ROBUST_ACCESS_KHR: GLenum = GLenum(0x90F3);

pub const GL_CONTINUOUS_AMD: GLenum = GLenum(0x9007);

pub const GL_CONTRAST_NV: GLenum = GLenum(0x92A1);

///
/// * Group: [`PathCoverMode`]
pub const GL_CONVEX_HULL_NV: GLenum = GLenum(0x908B);

///
/// * Group: [`ConvolutionTarget`], [`ConvolutionTargetEXT`]
pub const GL_CONVOLUTION_1D: GLenum = GLenum(0x8010);

///
/// * Group: [`GetPName`], [`ConvolutionTargetEXT`], [`EnableCap`]
pub const GL_CONVOLUTION_1D_EXT: GLenum = GLenum(0x8010);

///
/// * Group: [`ConvolutionTarget`], [`ConvolutionTargetEXT`]
pub const GL_CONVOLUTION_2D: GLenum = GLenum(0x8011);

///
/// * Group: [`GetPName`], [`ConvolutionTargetEXT`], [`EnableCap`]
pub const GL_CONVOLUTION_2D_EXT: GLenum = GLenum(0x8011);

///
/// * Group: [`GetConvolutionParameter`]
pub const GL_CONVOLUTION_BORDER_COLOR: GLenum = GLenum(0x8154);

pub const GL_CONVOLUTION_BORDER_COLOR_HP: GLenum = GLenum(0x8154);

///
/// * Group: [`GetConvolutionParameter`], [`ConvolutionParameterEXT`]
pub const GL_CONVOLUTION_BORDER_MODE: GLenum = GLenum(0x8013);

///
/// * Group: [`GetConvolutionParameter`], [`ConvolutionParameterEXT`]
pub const GL_CONVOLUTION_BORDER_MODE_EXT: GLenum = GLenum(0x8013);

///
/// * Group: [`GetConvolutionParameter`], [`ConvolutionParameterEXT`]
pub const GL_CONVOLUTION_FILTER_BIAS: GLenum = GLenum(0x8015);

///
/// * Group: [`GetConvolutionParameter`], [`ConvolutionParameterEXT`]
pub const GL_CONVOLUTION_FILTER_BIAS_EXT: GLenum = GLenum(0x8015);

///
/// * Group: [`GetConvolutionParameter`], [`ConvolutionParameterEXT`]
pub const GL_CONVOLUTION_FILTER_SCALE: GLenum = GLenum(0x8014);

///
/// * Group: [`GetConvolutionParameter`], [`ConvolutionParameterEXT`]
pub const GL_CONVOLUTION_FILTER_SCALE_EXT: GLenum = GLenum(0x8014);

///
/// * Group: [`GetConvolutionParameter`]
pub const GL_CONVOLUTION_FORMAT: GLenum = GLenum(0x8017);

///
/// * Group: [`GetConvolutionParameter`]
pub const GL_CONVOLUTION_FORMAT_EXT: GLenum = GLenum(0x8017);

///
/// * Group: [`GetConvolutionParameter`]
pub const GL_CONVOLUTION_HEIGHT: GLenum = GLenum(0x8019);

///
/// * Group: [`GetConvolutionParameter`]
pub const GL_CONVOLUTION_HEIGHT_EXT: GLenum = GLenum(0x8019);

///
/// * Group: [`HintTarget`], [`GetPName`]
pub const GL_CONVOLUTION_HINT_SGIX: GLenum = GLenum(0x8316);

///
/// * Group: [`GetConvolutionParameter`]
pub const GL_CONVOLUTION_WIDTH: GLenum = GLenum(0x8018);

///
/// * Group: [`GetConvolutionParameter`]
pub const GL_CONVOLUTION_WIDTH_EXT: GLenum = GLenum(0x8018);

pub const GL_CON_0_ATI: GLenum = GLenum(0x8941);

pub const GL_CON_10_ATI: GLenum = GLenum(0x894B);

pub const GL_CON_11_ATI: GLenum = GLenum(0x894C);

pub const GL_CON_12_ATI: GLenum = GLenum(0x894D);

pub const GL_CON_13_ATI: GLenum = GLenum(0x894E);

pub const GL_CON_14_ATI: GLenum = GLenum(0x894F);

pub const GL_CON_15_ATI: GLenum = GLenum(0x8950);

pub const GL_CON_16_ATI: GLenum = GLenum(0x8951);

pub const GL_CON_17_ATI: GLenum = GLenum(0x8952);

pub const GL_CON_18_ATI: GLenum = GLenum(0x8953);

pub const GL_CON_19_ATI: GLenum = GLenum(0x8954);

pub const GL_CON_1_ATI: GLenum = GLenum(0x8942);

pub const GL_CON_20_ATI: GLenum = GLenum(0x8955);

pub const GL_CON_21_ATI: GLenum = GLenum(0x8956);

pub const GL_CON_22_ATI: GLenum = GLenum(0x8957);

pub const GL_CON_23_ATI: GLenum = GLenum(0x8958);

pub const GL_CON_24_ATI: GLenum = GLenum(0x8959);

pub const GL_CON_25_ATI: GLenum = GLenum(0x895A);

pub const GL_CON_26_ATI: GLenum = GLenum(0x895B);

pub const GL_CON_27_ATI: GLenum = GLenum(0x895C);

pub const GL_CON_28_ATI: GLenum = GLenum(0x895D);

pub const GL_CON_29_ATI: GLenum = GLenum(0x895E);

pub const GL_CON_2_ATI: GLenum = GLenum(0x8943);

pub const GL_CON_30_ATI: GLenum = GLenum(0x895F);

pub const GL_CON_31_ATI: GLenum = GLenum(0x8960);

pub const GL_CON_3_ATI: GLenum = GLenum(0x8944);

pub const GL_CON_4_ATI: GLenum = GLenum(0x8945);

pub const GL_CON_5_ATI: GLenum = GLenum(0x8946);

pub const GL_CON_6_ATI: GLenum = GLenum(0x8947);

pub const GL_CON_7_ATI: GLenum = GLenum(0x8948);

pub const GL_CON_8_ATI: GLenum = GLenum(0x8949);

pub const GL_CON_9_ATI: GLenum = GLenum(0x894A);

pub const GL_COORD_REPLACE: GLenum = GLenum(0x8862);

pub const GL_COORD_REPLACE_ARB: GLenum = GLenum(0x8862);

pub const GL_COORD_REPLACE_NV: GLenum = GLenum(0x8862);

pub const GL_COORD_REPLACE_OES: GLenum = GLenum(0x8862);

///
/// * Group: [`LogicOp`]
pub const GL_COPY: GLenum = GLenum(0x1503);

///
/// * Group: [`LogicOp`]
pub const GL_COPY_INVERTED: GLenum = GLenum(0x150C);

///
/// * Group: [`FeedBackToken`]
pub const GL_COPY_PIXEL_TOKEN: GLenum = GLenum(0x0706);

///
/// * Group: [`CopyBufferSubDataTarget`], [`BufferTargetARB`],
///   [`BufferStorageTarget`]
pub const GL_COPY_READ_BUFFER: GLenum = GLenum(0x8F36);

///
/// * Alias Of: [`GL_COPY_READ_BUFFER`]
pub const GL_COPY_READ_BUFFER_BINDING: GLenum = GL_COPY_READ_BUFFER;

pub const GL_COPY_READ_BUFFER_NV: GLenum = GLenum(0x8F36);

///
/// * Group: [`CopyBufferSubDataTarget`], [`BufferTargetARB`],
///   [`BufferStorageTarget`]
pub const GL_COPY_WRITE_BUFFER: GLenum = GLenum(0x8F37);

///
/// * Alias Of: [`GL_COPY_WRITE_BUFFER`]
pub const GL_COPY_WRITE_BUFFER_BINDING: GLenum = GL_COPY_WRITE_BUFFER;

pub const GL_COPY_WRITE_BUFFER_NV: GLenum = GLenum(0x8F37);

pub const GL_COUNTER_RANGE_AMD: GLenum = GLenum(0x8BC1);

pub const GL_COUNTER_TYPE_AMD: GLenum = GLenum(0x8BC0);

///
/// * Group: [`PathFillMode`]
pub const GL_COUNT_DOWN_NV: GLenum = GLenum(0x9089);

///
/// * Group: [`PathFillMode`]
pub const GL_COUNT_UP_NV: GLenum = GLenum(0x9088);

pub const GL_COVERAGE_ALL_FRAGMENTS_NV: GLenum = GLenum(0x8ED5);

pub const GL_COVERAGE_ATTACHMENT_NV: GLenum = GLenum(0x8ED2);

pub const GL_COVERAGE_AUTOMATIC_NV: GLenum = GLenum(0x8ED7);

pub const GL_COVERAGE_BUFFERS_NV: GLenum = GLenum(0x8ED3);

/// Collides with AttribMask bit GL_HINT_BIT. OK since this token is for OpenGL
/// ES 2, which doesn't have attribute groups.
/// * Group: [`ClearBufferMask`]
pub const GL_COVERAGE_BUFFER_BIT_NV: GLbitfield = GLbitfield(0x00008000);

pub const GL_COVERAGE_COMPONENT4_NV: GLenum = GLenum(0x8ED1);

pub const GL_COVERAGE_COMPONENT_NV: GLenum = GLenum(0x8ED0);

pub const GL_COVERAGE_EDGE_FRAGMENTS_NV: GLenum = GLenum(0x8ED6);

pub const GL_COVERAGE_MODULATION_NV: GLenum = GLenum(0x9332);

pub const GL_COVERAGE_MODULATION_TABLE_NV: GLenum = GLenum(0x9331);

pub const GL_COVERAGE_MODULATION_TABLE_SIZE_NV: GLenum = GLenum(0x9333);

pub const GL_COVERAGE_SAMPLES_NV: GLenum = GLenum(0x8ED4);

pub const GL_CPU_OPTIMIZED_QCOM: GLenum = GLenum(0x8FB1);

///
/// * Group: [`PathCoordType`]
pub const GL_CUBIC_CURVE_TO_NV: GLenum = GLenum(0x0C);

pub const GL_CUBIC_EXT: GLenum = GLenum(0x8334);

pub const GL_CUBIC_HP: GLenum = GLenum(0x815F);

pub const GL_CUBIC_IMG: GLenum = GLenum(0x9139);

pub const GL_CUBIC_MIPMAP_LINEAR_IMG: GLenum = GLenum(0x913B);

pub const GL_CUBIC_MIPMAP_NEAREST_IMG: GLenum = GLenum(0x913A);

///
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_CULL_FACE: GLenum = GLenum(0x0B44);

///
/// * Group: [`GetPName`]
pub const GL_CULL_FACE_MODE: GLenum = GLenum(0x0B45);

pub const GL_CULL_FRAGMENT_NV: GLenum = GLenum(0x86E7);

pub const GL_CULL_MODES_NV: GLenum = GLenum(0x86E0);

pub const GL_CULL_VERTEX_EXT: GLenum = GLenum(0x81AA);

///
/// * Group: [`CullParameterEXT`]
pub const GL_CULL_VERTEX_EYE_POSITION_EXT: GLenum = GLenum(0x81AB);

pub const GL_CULL_VERTEX_IBM: GLenum = GLenum(103050);

///
/// * Group: [`CullParameterEXT`]
pub const GL_CULL_VERTEX_OBJECT_POSITION_EXT: GLenum = GLenum(0x81AC);

pub const GL_CURRENT_ATTRIB_NV: GLenum = GLenum(0x8626);

pub const GL_CURRENT_BINORMAL_EXT: GLenum = GLenum(0x843C);

///
/// * Group: [`AttribMask`]
pub const GL_CURRENT_BIT: GLbitfield = GLbitfield(0x00000001);

///
/// * Group: [`GetPName`]
pub const GL_CURRENT_COLOR: GLenum = GLenum(0x0B00);

///
/// * Alias Of: [`GL_CURRENT_FOG_COORDINATE`]
pub const GL_CURRENT_FOG_COORD: GLenum = GL_CURRENT_FOG_COORDINATE;

pub const GL_CURRENT_FOG_COORDINATE: GLenum = GLenum(0x8453);

pub const GL_CURRENT_FOG_COORDINATE_EXT: GLenum = GLenum(0x8453);

///
/// * Group: [`GetPName`]
pub const GL_CURRENT_INDEX: GLenum = GLenum(0x0B01);

pub const GL_CURRENT_MATRIX_ARB: GLenum = GLenum(0x8641);

pub const GL_CURRENT_MATRIX_INDEX_ARB: GLenum = GLenum(0x8845);

pub const GL_CURRENT_MATRIX_NV: GLenum = GLenum(0x8641);

pub const GL_CURRENT_MATRIX_STACK_DEPTH_ARB: GLenum = GLenum(0x8640);

pub const GL_CURRENT_MATRIX_STACK_DEPTH_NV: GLenum = GLenum(0x8640);

///
/// * Group: [`GetPName`]
pub const GL_CURRENT_NORMAL: GLenum = GLenum(0x0B02);

pub const GL_CURRENT_OCCLUSION_QUERY_ID_NV: GLenum = GLenum(0x8865);

pub const GL_CURRENT_PALETTE_MATRIX_ARB: GLenum = GLenum(0x8843);

pub const GL_CURRENT_PALETTE_MATRIX_OES: GLenum = GLenum(0x8843);

///
/// * Group: [`GetPName`]
pub const GL_CURRENT_PROGRAM: GLenum = GLenum(0x8B8D);

///
/// * Group: [`QueryParameterName`]
pub const GL_CURRENT_QUERY: GLenum = GLenum(0x8865);

pub const GL_CURRENT_QUERY_ARB: GLenum = GLenum(0x8865);

pub const GL_CURRENT_QUERY_EXT: GLenum = GLenum(0x8865);

///
/// * Group: [`GetPName`]
pub const GL_CURRENT_RASTER_COLOR: GLenum = GLenum(0x0B04);

///
/// * Group: [`GetPName`]
pub const GL_CURRENT_RASTER_DISTANCE: GLenum = GLenum(0x0B09);

///
/// * Group: [`GetPName`]
pub const GL_CURRENT_RASTER_INDEX: GLenum = GLenum(0x0B05);

pub const GL_CURRENT_RASTER_NORMAL_SGIX: GLenum = GLenum(0x8406);

///
/// * Group: [`GetPName`]
pub const GL_CURRENT_RASTER_POSITION: GLenum = GLenum(0x0B07);

///
/// * Group: [`GetPName`]
pub const GL_CURRENT_RASTER_POSITION_VALID: GLenum = GLenum(0x0B08);

pub const GL_CURRENT_RASTER_SECONDARY_COLOR: GLenum = GLenum(0x845F);

///
/// * Group: [`GetPName`]
pub const GL_CURRENT_RASTER_TEXTURE_COORDS: GLenum = GLenum(0x0B06);

pub const GL_CURRENT_SECONDARY_COLOR: GLenum = GLenum(0x8459);

pub const GL_CURRENT_SECONDARY_COLOR_EXT: GLenum = GLenum(0x8459);

pub const GL_CURRENT_TANGENT_EXT: GLenum = GLenum(0x843B);

///
/// * Group: [`GetPName`], [`VertexShaderTextureUnitParameter`]
pub const GL_CURRENT_TEXTURE_COORDS: GLenum = GLenum(0x0B03);

pub const GL_CURRENT_TIME_NV: GLenum = GLenum(0x8E28);

///
/// * Group: [`VertexAttribEnum`], [`VertexAttribPropertyARB`]
pub const GL_CURRENT_VERTEX_ATTRIB: GLenum = GLenum(0x8626);

pub const GL_CURRENT_VERTEX_ATTRIB_ARB: GLenum = GLenum(0x8626);

///
/// * Group: [`VertexShaderParameterEXT`]
pub const GL_CURRENT_VERTEX_EXT: GLenum = GLenum(0x87E2);

pub const GL_CURRENT_VERTEX_WEIGHT_EXT: GLenum = GLenum(0x850B);

pub const GL_CURRENT_WEIGHT_ARB: GLenum = GLenum(0x86A8);

///
/// * Group: [`FrontFaceDirection`]
pub const GL_CW: GLenum = GLenum(0x0900);

///
/// * Group: [`SemaphoreParameterName`]
pub const GL_D3D12_FENCE_VALUE_EXT: GLenum = GLenum(0x9595);

pub const GL_DARKEN: GLenum = GLenum(0x9297);

pub const GL_DARKEN_KHR: GLenum = GLenum(0x9297);

pub const GL_DARKEN_NV: GLenum = GLenum(0x9297);

pub const GL_DATA_BUFFER_AMD: GLenum = GLenum(0x9151);

/// NOT an alias. Accidental reuse of GL_PROXY_TEXTURE_1D_STACK_MESAX
pub const GL_DEBUG_ASSERT_MESA: GLenum = GLenum(0x875B);

///
/// * Group: [`GetPointervPName`]
pub const GL_DEBUG_CALLBACK_FUNCTION: GLenum = GLenum(0x8244);

pub const GL_DEBUG_CALLBACK_FUNCTION_ARB: GLenum = GLenum(0x8244);

pub const GL_DEBUG_CALLBACK_FUNCTION_KHR: GLenum = GLenum(0x8244);

///
/// * Group: [`GetPointervPName`]
pub const GL_DEBUG_CALLBACK_USER_PARAM: GLenum = GLenum(0x8245);

pub const GL_DEBUG_CALLBACK_USER_PARAM_ARB: GLenum = GLenum(0x8245);

pub const GL_DEBUG_CALLBACK_USER_PARAM_KHR: GLenum = GLenum(0x8245);

pub const GL_DEBUG_CATEGORY_API_ERROR_AMD: GLenum = GLenum(0x9149);

pub const GL_DEBUG_CATEGORY_APPLICATION_AMD: GLenum = GLenum(0x914F);

pub const GL_DEBUG_CATEGORY_DEPRECATION_AMD: GLenum = GLenum(0x914B);

pub const GL_DEBUG_CATEGORY_OTHER_AMD: GLenum = GLenum(0x9150);

pub const GL_DEBUG_CATEGORY_PERFORMANCE_AMD: GLenum = GLenum(0x914D);

pub const GL_DEBUG_CATEGORY_SHADER_COMPILER_AMD: GLenum = GLenum(0x914E);

pub const GL_DEBUG_CATEGORY_UNDEFINED_BEHAVIOR_AMD: GLenum = GLenum(0x914C);

pub const GL_DEBUG_CATEGORY_WINDOW_SYSTEM_AMD: GLenum = GLenum(0x914A);

///
/// * Group: [`GetPName`]
pub const GL_DEBUG_GROUP_STACK_DEPTH: GLenum = GLenum(0x826D);

pub const GL_DEBUG_GROUP_STACK_DEPTH_KHR: GLenum = GLenum(0x826D);

pub const GL_DEBUG_LOGGED_MESSAGES: GLenum = GLenum(0x9145);

pub const GL_DEBUG_LOGGED_MESSAGES_AMD: GLenum = GLenum(0x9145);

pub const GL_DEBUG_LOGGED_MESSAGES_ARB: GLenum = GLenum(0x9145);

pub const GL_DEBUG_LOGGED_MESSAGES_KHR: GLenum = GLenum(0x9145);

pub const GL_DEBUG_NEXT_LOGGED_MESSAGE_LENGTH: GLenum = GLenum(0x8243);

pub const GL_DEBUG_NEXT_LOGGED_MESSAGE_LENGTH_ARB: GLenum = GLenum(0x8243);

pub const GL_DEBUG_NEXT_LOGGED_MESSAGE_LENGTH_KHR: GLenum = GLenum(0x8243);

/// NOT an alias. Accidental reuse of GL_TEXTURE_1D_STACK_MESAX
pub const GL_DEBUG_OBJECT_MESA: GLenum = GLenum(0x8759);

///
/// * Group: [`EnableCap`]
pub const GL_DEBUG_OUTPUT: GLenum = GLenum(0x92E0);

pub const GL_DEBUG_OUTPUT_KHR: GLenum = GLenum(0x92E0);

///
/// * Group: [`EnableCap`]
pub const GL_DEBUG_OUTPUT_SYNCHRONOUS: GLenum = GLenum(0x8242);

pub const GL_DEBUG_OUTPUT_SYNCHRONOUS_ARB: GLenum = GLenum(0x8242);

pub const GL_DEBUG_OUTPUT_SYNCHRONOUS_KHR: GLenum = GLenum(0x8242);

/// NOT an alias. Accidental reuse of GL_TEXTURE_2D_STACK_MESAX
pub const GL_DEBUG_PRINT_MESA: GLenum = GLenum(0x875A);

///
/// * Group: [`DebugSeverity`]
pub const GL_DEBUG_SEVERITY_HIGH: GLenum = GLenum(0x9146);

pub const GL_DEBUG_SEVERITY_HIGH_AMD: GLenum = GLenum(0x9146);

pub const GL_DEBUG_SEVERITY_HIGH_ARB: GLenum = GLenum(0x9146);

pub const GL_DEBUG_SEVERITY_HIGH_KHR: GLenum = GLenum(0x9146);

///
/// * Group: [`DebugSeverity`]
pub const GL_DEBUG_SEVERITY_LOW: GLenum = GLenum(0x9148);

pub const GL_DEBUG_SEVERITY_LOW_AMD: GLenum = GLenum(0x9148);

pub const GL_DEBUG_SEVERITY_LOW_ARB: GLenum = GLenum(0x9148);

pub const GL_DEBUG_SEVERITY_LOW_KHR: GLenum = GLenum(0x9148);

///
/// * Group: [`DebugSeverity`]
pub const GL_DEBUG_SEVERITY_MEDIUM: GLenum = GLenum(0x9147);

pub const GL_DEBUG_SEVERITY_MEDIUM_AMD: GLenum = GLenum(0x9147);

pub const GL_DEBUG_SEVERITY_MEDIUM_ARB: GLenum = GLenum(0x9147);

pub const GL_DEBUG_SEVERITY_MEDIUM_KHR: GLenum = GLenum(0x9147);

///
/// * Group: [`DebugSeverity`]
pub const GL_DEBUG_SEVERITY_NOTIFICATION: GLenum = GLenum(0x826B);

pub const GL_DEBUG_SEVERITY_NOTIFICATION_KHR: GLenum = GLenum(0x826B);

///
/// * Group: [`DebugSource`]
pub const GL_DEBUG_SOURCE_API: GLenum = GLenum(0x8246);

pub const GL_DEBUG_SOURCE_API_ARB: GLenum = GLenum(0x8246);

pub const GL_DEBUG_SOURCE_API_KHR: GLenum = GLenum(0x8246);

///
/// * Group: [`DebugSource`]
pub const GL_DEBUG_SOURCE_APPLICATION: GLenum = GLenum(0x824A);

pub const GL_DEBUG_SOURCE_APPLICATION_ARB: GLenum = GLenum(0x824A);

pub const GL_DEBUG_SOURCE_APPLICATION_KHR: GLenum = GLenum(0x824A);

///
/// * Group: [`DebugSource`]
pub const GL_DEBUG_SOURCE_OTHER: GLenum = GLenum(0x824B);

pub const GL_DEBUG_SOURCE_OTHER_ARB: GLenum = GLenum(0x824B);

pub const GL_DEBUG_SOURCE_OTHER_KHR: GLenum = GLenum(0x824B);

///
/// * Group: [`DebugSource`]
pub const GL_DEBUG_SOURCE_SHADER_COMPILER: GLenum = GLenum(0x8248);

pub const GL_DEBUG_SOURCE_SHADER_COMPILER_ARB: GLenum = GLenum(0x8248);

pub const GL_DEBUG_SOURCE_SHADER_COMPILER_KHR: GLenum = GLenum(0x8248);

///
/// * Group: [`DebugSource`]
pub const GL_DEBUG_SOURCE_THIRD_PARTY: GLenum = GLenum(0x8249);

pub const GL_DEBUG_SOURCE_THIRD_PARTY_ARB: GLenum = GLenum(0x8249);

pub const GL_DEBUG_SOURCE_THIRD_PARTY_KHR: GLenum = GLenum(0x8249);

///
/// * Group: [`DebugSource`]
pub const GL_DEBUG_SOURCE_WINDOW_SYSTEM: GLenum = GLenum(0x8247);

pub const GL_DEBUG_SOURCE_WINDOW_SYSTEM_ARB: GLenum = GLenum(0x8247);

pub const GL_DEBUG_SOURCE_WINDOW_SYSTEM_KHR: GLenum = GLenum(0x8247);

///
/// * Group: [`DebugType`]
pub const GL_DEBUG_TYPE_DEPRECATED_BEHAVIOR: GLenum = GLenum(0x824D);

pub const GL_DEBUG_TYPE_DEPRECATED_BEHAVIOR_ARB: GLenum = GLenum(0x824D);

pub const GL_DEBUG_TYPE_DEPRECATED_BEHAVIOR_KHR: GLenum = GLenum(0x824D);

///
/// * Group: [`DebugType`]
pub const GL_DEBUG_TYPE_ERROR: GLenum = GLenum(0x824C);

pub const GL_DEBUG_TYPE_ERROR_ARB: GLenum = GLenum(0x824C);

pub const GL_DEBUG_TYPE_ERROR_KHR: GLenum = GLenum(0x824C);

///
/// * Group: [`DebugType`]
pub const GL_DEBUG_TYPE_MARKER: GLenum = GLenum(0x8268);

pub const GL_DEBUG_TYPE_MARKER_KHR: GLenum = GLenum(0x8268);

///
/// * Group: [`DebugType`]
pub const GL_DEBUG_TYPE_OTHER: GLenum = GLenum(0x8251);

pub const GL_DEBUG_TYPE_OTHER_ARB: GLenum = GLenum(0x8251);

pub const GL_DEBUG_TYPE_OTHER_KHR: GLenum = GLenum(0x8251);

///
/// * Group: [`DebugType`]
pub const GL_DEBUG_TYPE_PERFORMANCE: GLenum = GLenum(0x8250);

pub const GL_DEBUG_TYPE_PERFORMANCE_ARB: GLenum = GLenum(0x8250);

pub const GL_DEBUG_TYPE_PERFORMANCE_KHR: GLenum = GLenum(0x8250);

///
/// * Group: [`DebugType`]
pub const GL_DEBUG_TYPE_POP_GROUP: GLenum = GLenum(0x826A);

pub const GL_DEBUG_TYPE_POP_GROUP_KHR: GLenum = GLenum(0x826A);

///
/// * Group: [`DebugType`]
pub const GL_DEBUG_TYPE_PORTABILITY: GLenum = GLenum(0x824F);

pub const GL_DEBUG_TYPE_PORTABILITY_ARB: GLenum = GLenum(0x824F);

pub const GL_DEBUG_TYPE_PORTABILITY_KHR: GLenum = GLenum(0x824F);

///
/// * Group: [`DebugType`]
pub const GL_DEBUG_TYPE_PUSH_GROUP: GLenum = GLenum(0x8269);

pub const GL_DEBUG_TYPE_PUSH_GROUP_KHR: GLenum = GLenum(0x8269);

///
/// * Group: [`DebugType`]
pub const GL_DEBUG_TYPE_UNDEFINED_BEHAVIOR: GLenum = GLenum(0x824E);

pub const GL_DEBUG_TYPE_UNDEFINED_BEHAVIOR_ARB: GLenum = GLenum(0x824E);

pub const GL_DEBUG_TYPE_UNDEFINED_BEHAVIOR_KHR: GLenum = GLenum(0x824E);

///
/// * Group: [`TextureEnvMode`]
pub const GL_DECAL: GLenum = GLenum(0x2101);

pub const GL_DECODE_EXT: GLenum = GLenum(0x8A49);

///
/// * Group: [`StencilOp`]
pub const GL_DECR: GLenum = GLenum(0x1E03);

///
/// * Group: [`StencilOp`]
pub const GL_DECR_WRAP: GLenum = GLenum(0x8508);

pub const GL_DECR_WRAP_EXT: GLenum = GLenum(0x8508);

pub const GL_DECR_WRAP_OES: GLenum = GLenum(0x8508);

///
/// * Group: [`MemoryObjectParameterName`]
pub const GL_DEDICATED_MEMORY_OBJECT_EXT: GLenum = GLenum(0x9581);

///
/// * Group: [`GetPName`]
pub const GL_DEFORMATIONS_MASK_SGIX: GLenum = GLenum(0x8196);

///
/// * Group: [`ProgramPropertyARB`], [`ShaderParameterName`]
pub const GL_DELETE_STATUS: GLenum = GLenum(0x8B80);

pub const GL_DEPENDENT_AR_TEXTURE_2D_NV: GLenum = GLenum(0x86E9);

pub const GL_DEPENDENT_GB_TEXTURE_2D_NV: GLenum = GLenum(0x86EA);

pub const GL_DEPENDENT_HILO_TEXTURE_2D_NV: GLenum = GLenum(0x8858);

pub const GL_DEPENDENT_RGB_TEXTURE_3D_NV: GLenum = GLenum(0x8859);

pub const GL_DEPENDENT_RGB_TEXTURE_CUBE_MAP_NV: GLenum = GLenum(0x885A);

///
/// * Group: [`Buffer`], [`PixelCopyType`], [`InvalidateFramebufferAttachment`]
pub const GL_DEPTH: GLenum = GLenum(0x1801);

///
/// * Group: [`InternalFormat`]
pub const GL_DEPTH24_STENCIL8: GLenum = GLenum(0x88F0);

///
/// * Group: [`InternalFormat`]
pub const GL_DEPTH24_STENCIL8_EXT: GLenum = GLenum(0x88F0);

///
/// * Group: [`InternalFormat`]
pub const GL_DEPTH24_STENCIL8_OES: GLenum = GLenum(0x88F0);

///
/// * Group: [`InternalFormat`]
pub const GL_DEPTH32F_STENCIL8: GLenum = GLenum(0x8CAD);

///
/// * Group: [`InternalFormat`]
pub const GL_DEPTH32F_STENCIL8_NV: GLenum = GLenum(0x8DAC);

///
/// * Group: [`InvalidateFramebufferAttachment`], [`FramebufferAttachment`]
pub const GL_DEPTH_ATTACHMENT: GLenum = GLenum(0x8D00);

///
/// * Group: [`InvalidateFramebufferAttachment`]
pub const GL_DEPTH_ATTACHMENT_EXT: GLenum = GLenum(0x8D00);

///
/// * Group: [`InvalidateFramebufferAttachment`]
pub const GL_DEPTH_ATTACHMENT_OES: GLenum = GLenum(0x8D00);

///
/// * Group: [`PixelTransferParameter`], [`GetPName`]
pub const GL_DEPTH_BIAS: GLenum = GLenum(0x0D1F);

///
/// * Group: [`GetPName`]
pub const GL_DEPTH_BITS: GLenum = GLenum(0x0D56);

pub const GL_DEPTH_BOUNDS_EXT: GLenum = GLenum(0x8891);

pub const GL_DEPTH_BOUNDS_TEST_EXT: GLenum = GLenum(0x8890);

///
/// * Group: [`ClearBufferMask`], [`AttribMask`]
pub const GL_DEPTH_BUFFER_BIT: GLbitfield = GLbitfield(0x00000100);

///
/// * Group: [`BufferBitQCOM`]
pub const GL_DEPTH_BUFFER_BIT0_QCOM: GLbitfield = GLbitfield(0x00000100);

///
/// * Group: [`BufferBitQCOM`]
pub const GL_DEPTH_BUFFER_BIT1_QCOM: GLbitfield = GLbitfield(0x00000200);

///
/// * Group: [`BufferBitQCOM`]
pub const GL_DEPTH_BUFFER_BIT2_QCOM: GLbitfield = GLbitfield(0x00000400);

///
/// * Group: [`BufferBitQCOM`]
pub const GL_DEPTH_BUFFER_BIT3_QCOM: GLbitfield = GLbitfield(0x00000800);

///
/// * Group: [`BufferBitQCOM`]
pub const GL_DEPTH_BUFFER_BIT4_QCOM: GLbitfield = GLbitfield(0x00001000);

///
/// * Group: [`BufferBitQCOM`]
pub const GL_DEPTH_BUFFER_BIT5_QCOM: GLbitfield = GLbitfield(0x00002000);

///
/// * Group: [`BufferBitQCOM`]
pub const GL_DEPTH_BUFFER_BIT6_QCOM: GLbitfield = GLbitfield(0x00004000);

///
/// * Group: [`BufferBitQCOM`]
pub const GL_DEPTH_BUFFER_BIT7_QCOM: GLbitfield = GLbitfield(0x00008000);

pub const GL_DEPTH_BUFFER_FLOAT_MODE_NV: GLenum = GLenum(0x8DAF);

///
/// * Group: [`EnableCap`]
pub const GL_DEPTH_CLAMP: GLenum = GLenum(0x864F);

pub const GL_DEPTH_CLAMP_EXT: GLenum = GLenum(0x864F);

pub const GL_DEPTH_CLAMP_FAR_AMD: GLenum = GLenum(0x901F);

pub const GL_DEPTH_CLAMP_NEAR_AMD: GLenum = GLenum(0x901E);

pub const GL_DEPTH_CLAMP_NV: GLenum = GLenum(0x864F);

///
/// * Group: [`GetPName`]
pub const GL_DEPTH_CLEAR_VALUE: GLenum = GLenum(0x0B73);

///
/// * Group: [`InternalFormat`], [`PixelFormat`]
pub const GL_DEPTH_COMPONENT: GLenum = GLenum(0x1902);

///
/// * Group: [`InternalFormat`]
pub const GL_DEPTH_COMPONENT16: GLenum = GLenum(0x81A5);

///
/// * Group: [`InternalFormat`]
pub const GL_DEPTH_COMPONENT16_ARB: GLenum = GLenum(0x81A5);

pub const GL_DEPTH_COMPONENT16_NONLINEAR_NV: GLenum = GLenum(0x8E2C);

///
/// * Group: [`InternalFormat`]
pub const GL_DEPTH_COMPONENT16_OES: GLenum = GLenum(0x81A5);

///
/// * Group: [`InternalFormat`]
pub const GL_DEPTH_COMPONENT16_SGIX: GLenum = GLenum(0x81A5);

pub const GL_DEPTH_COMPONENT24: GLenum = GLenum(0x81A6);

///
/// * Group: [`InternalFormat`]
pub const GL_DEPTH_COMPONENT24_ARB: GLenum = GLenum(0x81A6);

///
/// * Group: [`InternalFormat`]
pub const GL_DEPTH_COMPONENT24_OES: GLenum = GLenum(0x81A6);

///
/// * Group: [`InternalFormat`]
pub const GL_DEPTH_COMPONENT24_SGIX: GLenum = GLenum(0x81A6);

pub const GL_DEPTH_COMPONENT32: GLenum = GLenum(0x81A7);

///
/// * Group: [`InternalFormat`]
pub const GL_DEPTH_COMPONENT32F: GLenum = GLenum(0x8CAC);

///
/// * Group: [`InternalFormat`]
pub const GL_DEPTH_COMPONENT32F_NV: GLenum = GLenum(0x8DAB);

///
/// * Group: [`InternalFormat`]
pub const GL_DEPTH_COMPONENT32_ARB: GLenum = GLenum(0x81A7);

///
/// * Group: [`InternalFormat`]
pub const GL_DEPTH_COMPONENT32_OES: GLenum = GLenum(0x81A7);

///
/// * Group: [`InternalFormat`]
pub const GL_DEPTH_COMPONENT32_SGIX: GLenum = GLenum(0x81A7);

pub const GL_DEPTH_COMPONENTS: GLenum = GLenum(0x8284);

///
/// * Group: [`PixelCopyType`]
pub const GL_DEPTH_EXT: GLenum = GLenum(0x1801);

///
/// * Group: [`GetPName`]
pub const GL_DEPTH_FUNC: GLenum = GLenum(0x0B74);

pub const GL_DEPTH_PASS_INSTRUMENT_COUNTERS_SGIX: GLenum = GLenum(0x8311);

pub const GL_DEPTH_PASS_INSTRUMENT_MAX_SGIX: GLenum = GLenum(0x8312);

pub const GL_DEPTH_PASS_INSTRUMENT_SGIX: GLenum = GLenum(0x8310);

///
/// * Group: [`GetPName`]
pub const GL_DEPTH_RANGE: GLenum = GLenum(0x0B70);

///
/// * Group: [`InternalFormatPName`]
pub const GL_DEPTH_RENDERABLE: GLenum = GLenum(0x8287);

pub const GL_DEPTH_SAMPLES_NV: GLenum = GLenum(0x932D);

///
/// * Group: [`PixelTransferParameter`], [`GetPName`]
pub const GL_DEPTH_SCALE: GLenum = GLenum(0x0D1E);

///
/// * Group: [`InternalFormat`], [`PixelFormat`]
pub const GL_DEPTH_STENCIL: GLenum = GLenum(0x84F9);

///
/// * Group: [`InvalidateFramebufferAttachment`]
pub const GL_DEPTH_STENCIL_ATTACHMENT: GLenum = GLenum(0x821A);

///
/// * Group: [`InternalFormat`]
pub const GL_DEPTH_STENCIL_EXT: GLenum = GLenum(0x84F9);

///
/// * Group: [`InternalFormat`]
pub const GL_DEPTH_STENCIL_MESA: GLenum = GLenum(0x8750);

///
/// * Group: [`InternalFormat`]
pub const GL_DEPTH_STENCIL_NV: GLenum = GLenum(0x84F9);

///
/// * Group: [`InternalFormat`]
pub const GL_DEPTH_STENCIL_OES: GLenum = GLenum(0x84F9);

///
/// * Group: [`TextureParameterName`]
pub const GL_DEPTH_STENCIL_TEXTURE_MODE: GLenum = GLenum(0x90EA);

pub const GL_DEPTH_STENCIL_TO_BGRA_NV: GLenum = GLenum(0x886F);

pub const GL_DEPTH_STENCIL_TO_RGBA_NV: GLenum = GLenum(0x886E);

///
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_DEPTH_TEST: GLenum = GLenum(0x0B71);

pub const GL_DEPTH_TEXTURE_MODE: GLenum = GLenum(0x884B);

pub const GL_DEPTH_TEXTURE_MODE_ARB: GLenum = GLenum(0x884B);

///
/// * Group: [`GetPName`]
pub const GL_DEPTH_WRITEMASK: GLenum = GLenum(0x0B72);

pub const GL_DETACHED_BUFFERS_NV: GLenum = GLenum(0x95AB);

pub const GL_DETACHED_MEMORY_INCARNATION_NV: GLenum = GLenum(0x95A9);

pub const GL_DETACHED_TEXTURES_NV: GLenum = GLenum(0x95AA);

///
/// * Group: [`GetPName`]
pub const GL_DETAIL_TEXTURE_2D_BINDING_SGIS: GLenum = GLenum(0x8096);

///
/// * Group: [`TextureTarget`]
pub const GL_DETAIL_TEXTURE_2D_SGIS: GLenum = GLenum(0x8095);

///
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_DETAIL_TEXTURE_FUNC_POINTS_SGIS: GLenum = GLenum(0x809C);

///
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_DETAIL_TEXTURE_LEVEL_SGIS: GLenum = GLenum(0x809A);

///
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_DETAIL_TEXTURE_MODE_SGIS: GLenum = GLenum(0x809B);

///
/// * Group: [`GetPName`]
pub const GL_DEVICE_LUID_EXT: GLenum = GLenum(0x9599);

///
/// * Group: [`GetPName`]
pub const GL_DEVICE_NODE_MASK_EXT: GLenum = GLenum(0x959A);

///
/// * Group: [`GetPName`]
pub const GL_DEVICE_UUID_EXT: GLenum = GLenum(0x9597);

pub const GL_DIFFERENCE: GLenum = GLenum(0x929E);

pub const GL_DIFFERENCE_KHR: GLenum = GLenum(0x929E);

pub const GL_DIFFERENCE_NV: GLenum = GLenum(0x929E);

///
/// * Group: [`MaterialParameter`], [`FragmentLightParameterSGIX`],
///   [`ColorMaterialParameter`]
pub const GL_DIFFUSE: GLenum = GLenum(0x1201);

///
/// * Group: [`PreserveModeATI`]
pub const GL_DISCARD_ATI: GLenum = GLenum(0x8763);

///
/// * Group: [`CombinerRegisterNV`]
pub const GL_DISCARD_NV: GLenum = GLenum(0x8530);

pub const GL_DISCRETE_AMD: GLenum = GLenum(0x9006);

pub const GL_DISJOINT_NV: GLenum = GLenum(0x9283);

///
/// * Group: [`CopyBufferSubDataTarget`], [`BufferTargetARB`],
///   [`BufferStorageTarget`]
pub const GL_DISPATCH_INDIRECT_BUFFER: GLenum = GLenum(0x90EE);

///
/// * Group: [`GetPName`]
pub const GL_DISPATCH_INDIRECT_BUFFER_BINDING: GLenum = GLenum(0x90EF);

pub const GL_DISPLAY_LIST: GLenum = GLenum(0x82E7);

///
/// * Group: [`PointParameterNameSGIS`]
pub const GL_DISTANCE_ATTENUATION_EXT: GLenum = GLenum(0x8129);

///
/// * Group: [`PointParameterNameSGIS`], [`GetPName`]
pub const GL_DISTANCE_ATTENUATION_SGIS: GLenum = GLenum(0x8129);

///
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_DITHER: GLenum = GLenum(0x0BD0);

pub const GL_DMP_PROGRAM_BINARY_DMP: GLenum = GLenum(0x9253);

///
/// * Group: [`MapQuery`], [`GetMapQuery`]
pub const GL_DOMAIN: GLenum = GLenum(0x0A02);

///
/// * Group: [`DebugSeverity`], [`HintMode`], [`DebugSource`], [`DebugType`]
pub const GL_DONT_CARE: GLenum = GLenum(0x1100);

///
/// * Group: [`FragmentOpATI`]
pub const GL_DOT2_ADD_ATI: GLenum = GLenum(0x896C);

///
/// * Group: [`FragmentOpATI`]
pub const GL_DOT3_ATI: GLenum = GLenum(0x8966);

pub const GL_DOT3_RGB: GLenum = GLenum(0x86AE);

pub const GL_DOT3_RGBA: GLenum = GLenum(0x86AF);

pub const GL_DOT3_RGBA_ARB: GLenum = GLenum(0x86AF);

pub const GL_DOT3_RGBA_EXT: GLenum = GLenum(0x8741);

pub const GL_DOT3_RGBA_IMG: GLenum = GLenum(0x86AF);

pub const GL_DOT3_RGB_ARB: GLenum = GLenum(0x86AE);

pub const GL_DOT3_RGB_EXT: GLenum = GLenum(0x8740);

///
/// * Group: [`FragmentOpATI`]
pub const GL_DOT4_ATI: GLenum = GLenum(0x8967);

pub const GL_DOT_PRODUCT_AFFINE_DEPTH_REPLACE_NV: GLenum = GLenum(0x885D);

pub const GL_DOT_PRODUCT_CONST_EYE_REFLECT_CUBE_MAP_NV: GLenum = GLenum(0x86F3);

pub const GL_DOT_PRODUCT_DEPTH_REPLACE_NV: GLenum = GLenum(0x86ED);

pub const GL_DOT_PRODUCT_DIFFUSE_CUBE_MAP_NV: GLenum = GLenum(0x86F1);

pub const GL_DOT_PRODUCT_NV: GLenum = GLenum(0x86EC);

pub const GL_DOT_PRODUCT_PASS_THROUGH_NV: GLenum = GLenum(0x885B);

pub const GL_DOT_PRODUCT_REFLECT_CUBE_MAP_NV: GLenum = GLenum(0x86F2);

pub const GL_DOT_PRODUCT_TEXTURE_1D_NV: GLenum = GLenum(0x885C);

pub const GL_DOT_PRODUCT_TEXTURE_2D_NV: GLenum = GLenum(0x86EE);

pub const GL_DOT_PRODUCT_TEXTURE_3D_NV: GLenum = GLenum(0x86EF);

pub const GL_DOT_PRODUCT_TEXTURE_CUBE_MAP_NV: GLenum = GLenum(0x86F0);

pub const GL_DOT_PRODUCT_TEXTURE_RECTANGLE_NV: GLenum = GLenum(0x864E);

///
/// * Group: [`VertexAttribLType`], [`MapTypeNV`],
///   [`SecondaryColorPointerTypeIBM`], [`WeightPointerTypeARB`],
///   [`TangentPointerTypeEXT`], [`BinormalPointerTypeEXT`],
///   [`FogCoordinatePointerType`], [`FogPointerTypeEXT`],
///   [`FogPointerTypeIBM`], [`IndexPointerType`], [`NormalPointerType`],
///   [`TexCoordPointerType`], [`VertexPointerType`], [`VertexAttribType`],
///   [`AttributeType`], [`UniformType`], [`VertexAttribPointerType`],
///   [`GlslTypeToken`]
pub const GL_DOUBLE: GLenum = GLenum(0x140A);

///
/// * Group: [`GetFramebufferParameter`], [`GetPName`]
pub const GL_DOUBLEBUFFER: GLenum = GLenum(0x0C32);

///
/// * Group: [`BinormalPointerTypeEXT`], [`TangentPointerTypeEXT`]
pub const GL_DOUBLE_EXT: GLenum = GLenum(0x140A);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_DOUBLE_MAT2: GLenum = GLenum(0x8F46);

pub const GL_DOUBLE_MAT2_EXT: GLenum = GLenum(0x8F46);

///
/// * Group: [`UniformType`], [`AttributeType`]
pub const GL_DOUBLE_MAT2x3: GLenum = GLenum(0x8F49);

pub const GL_DOUBLE_MAT2x3_EXT: GLenum = GLenum(0x8F49);

///
/// * Group: [`UniformType`], [`AttributeType`]
pub const GL_DOUBLE_MAT2x4: GLenum = GLenum(0x8F4A);

pub const GL_DOUBLE_MAT2x4_EXT: GLenum = GLenum(0x8F4A);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_DOUBLE_MAT3: GLenum = GLenum(0x8F47);

pub const GL_DOUBLE_MAT3_EXT: GLenum = GLenum(0x8F47);

///
/// * Group: [`UniformType`], [`AttributeType`]
pub const GL_DOUBLE_MAT3x2: GLenum = GLenum(0x8F4B);

pub const GL_DOUBLE_MAT3x2_EXT: GLenum = GLenum(0x8F4B);

///
/// * Group: [`UniformType`], [`AttributeType`]
pub const GL_DOUBLE_MAT3x4: GLenum = GLenum(0x8F4C);

pub const GL_DOUBLE_MAT3x4_EXT: GLenum = GLenum(0x8F4C);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_DOUBLE_MAT4: GLenum = GLenum(0x8F48);

pub const GL_DOUBLE_MAT4_EXT: GLenum = GLenum(0x8F48);

///
/// * Group: [`UniformType`], [`AttributeType`]
pub const GL_DOUBLE_MAT4x2: GLenum = GLenum(0x8F4D);

pub const GL_DOUBLE_MAT4x2_EXT: GLenum = GLenum(0x8F4D);

///
/// * Group: [`UniformType`], [`AttributeType`]
pub const GL_DOUBLE_MAT4x3: GLenum = GLenum(0x8F4E);

pub const GL_DOUBLE_MAT4x3_EXT: GLenum = GLenum(0x8F4E);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_DOUBLE_VEC2: GLenum = GLenum(0x8FFC);

pub const GL_DOUBLE_VEC2_EXT: GLenum = GLenum(0x8FFC);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_DOUBLE_VEC3: GLenum = GLenum(0x8FFD);

pub const GL_DOUBLE_VEC3_EXT: GLenum = GLenum(0x8FFD);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_DOUBLE_VEC4: GLenum = GLenum(0x8FFE);

pub const GL_DOUBLE_VEC4_EXT: GLenum = GLenum(0x8FFE);

pub const GL_DOWNSAMPLE_SCALES_IMG: GLenum = GLenum(0x913E);

///
/// * Group: [`CommandOpcodesNV`]
pub const GL_DRAW_ARRAYS_COMMAND_NV: GLenum = GLenum(0x0003);

///
/// * Group: [`CommandOpcodesNV`]
pub const GL_DRAW_ARRAYS_INSTANCED_COMMAND_NV: GLenum = GLenum(0x0007);

///
/// * Group: [`CommandOpcodesNV`]
pub const GL_DRAW_ARRAYS_STRIP_COMMAND_NV: GLenum = GLenum(0x0005);

///
/// * Group: [`GetPName`]
pub const GL_DRAW_BUFFER: GLenum = GLenum(0x0C01);

pub const GL_DRAW_BUFFER0: GLenum = GLenum(0x8825);

pub const GL_DRAW_BUFFER0_ARB: GLenum = GLenum(0x8825);

pub const GL_DRAW_BUFFER0_ATI: GLenum = GLenum(0x8825);

pub const GL_DRAW_BUFFER0_EXT: GLenum = GLenum(0x8825);

pub const GL_DRAW_BUFFER0_NV: GLenum = GLenum(0x8825);

pub const GL_DRAW_BUFFER1: GLenum = GLenum(0x8826);

pub const GL_DRAW_BUFFER10: GLenum = GLenum(0x882F);

pub const GL_DRAW_BUFFER10_ARB: GLenum = GLenum(0x882F);

pub const GL_DRAW_BUFFER10_ATI: GLenum = GLenum(0x882F);

pub const GL_DRAW_BUFFER10_EXT: GLenum = GLenum(0x882F);

pub const GL_DRAW_BUFFER10_NV: GLenum = GLenum(0x882F);

pub const GL_DRAW_BUFFER11: GLenum = GLenum(0x8830);

pub const GL_DRAW_BUFFER11_ARB: GLenum = GLenum(0x8830);

pub const GL_DRAW_BUFFER11_ATI: GLenum = GLenum(0x8830);

pub const GL_DRAW_BUFFER11_EXT: GLenum = GLenum(0x8830);

pub const GL_DRAW_BUFFER11_NV: GLenum = GLenum(0x8830);

pub const GL_DRAW_BUFFER12: GLenum = GLenum(0x8831);

pub const GL_DRAW_BUFFER12_ARB: GLenum = GLenum(0x8831);

pub const GL_DRAW_BUFFER12_ATI: GLenum = GLenum(0x8831);

pub const GL_DRAW_BUFFER12_EXT: GLenum = GLenum(0x8831);

pub const GL_DRAW_BUFFER12_NV: GLenum = GLenum(0x8831);

pub const GL_DRAW_BUFFER13: GLenum = GLenum(0x8832);

pub const GL_DRAW_BUFFER13_ARB: GLenum = GLenum(0x8832);

pub const GL_DRAW_BUFFER13_ATI: GLenum = GLenum(0x8832);

pub const GL_DRAW_BUFFER13_EXT: GLenum = GLenum(0x8832);

pub const GL_DRAW_BUFFER13_NV: GLenum = GLenum(0x8832);

pub const GL_DRAW_BUFFER14: GLenum = GLenum(0x8833);

pub const GL_DRAW_BUFFER14_ARB: GLenum = GLenum(0x8833);

pub const GL_DRAW_BUFFER14_ATI: GLenum = GLenum(0x8833);

pub const GL_DRAW_BUFFER14_EXT: GLenum = GLenum(0x8833);

pub const GL_DRAW_BUFFER14_NV: GLenum = GLenum(0x8833);

pub const GL_DRAW_BUFFER15: GLenum = GLenum(0x8834);

pub const GL_DRAW_BUFFER15_ARB: GLenum = GLenum(0x8834);

pub const GL_DRAW_BUFFER15_ATI: GLenum = GLenum(0x8834);

pub const GL_DRAW_BUFFER15_EXT: GLenum = GLenum(0x8834);

pub const GL_DRAW_BUFFER15_NV: GLenum = GLenum(0x8834);

pub const GL_DRAW_BUFFER1_ARB: GLenum = GLenum(0x8826);

pub const GL_DRAW_BUFFER1_ATI: GLenum = GLenum(0x8826);

pub const GL_DRAW_BUFFER1_EXT: GLenum = GLenum(0x8826);

pub const GL_DRAW_BUFFER1_NV: GLenum = GLenum(0x8826);

pub const GL_DRAW_BUFFER2: GLenum = GLenum(0x8827);

pub const GL_DRAW_BUFFER2_ARB: GLenum = GLenum(0x8827);

pub const GL_DRAW_BUFFER2_ATI: GLenum = GLenum(0x8827);

pub const GL_DRAW_BUFFER2_EXT: GLenum = GLenum(0x8827);

pub const GL_DRAW_BUFFER2_NV: GLenum = GLenum(0x8827);

pub const GL_DRAW_BUFFER3: GLenum = GLenum(0x8828);

pub const GL_DRAW_BUFFER3_ARB: GLenum = GLenum(0x8828);

pub const GL_DRAW_BUFFER3_ATI: GLenum = GLenum(0x8828);

pub const GL_DRAW_BUFFER3_EXT: GLenum = GLenum(0x8828);

pub const GL_DRAW_BUFFER3_NV: GLenum = GLenum(0x8828);

pub const GL_DRAW_BUFFER4: GLenum = GLenum(0x8829);

pub const GL_DRAW_BUFFER4_ARB: GLenum = GLenum(0x8829);

pub const GL_DRAW_BUFFER4_ATI: GLenum = GLenum(0x8829);

pub const GL_DRAW_BUFFER4_EXT: GLenum = GLenum(0x8829);

pub const GL_DRAW_BUFFER4_NV: GLenum = GLenum(0x8829);

pub const GL_DRAW_BUFFER5: GLenum = GLenum(0x882A);

pub const GL_DRAW_BUFFER5_ARB: GLenum = GLenum(0x882A);

pub const GL_DRAW_BUFFER5_ATI: GLenum = GLenum(0x882A);

pub const GL_DRAW_BUFFER5_EXT: GLenum = GLenum(0x882A);

pub const GL_DRAW_BUFFER5_NV: GLenum = GLenum(0x882A);

pub const GL_DRAW_BUFFER6: GLenum = GLenum(0x882B);

pub const GL_DRAW_BUFFER6_ARB: GLenum = GLenum(0x882B);

pub const GL_DRAW_BUFFER6_ATI: GLenum = GLenum(0x882B);

pub const GL_DRAW_BUFFER6_EXT: GLenum = GLenum(0x882B);

pub const GL_DRAW_BUFFER6_NV: GLenum = GLenum(0x882B);

pub const GL_DRAW_BUFFER7: GLenum = GLenum(0x882C);

pub const GL_DRAW_BUFFER7_ARB: GLenum = GLenum(0x882C);

pub const GL_DRAW_BUFFER7_ATI: GLenum = GLenum(0x882C);

pub const GL_DRAW_BUFFER7_EXT: GLenum = GLenum(0x882C);

pub const GL_DRAW_BUFFER7_NV: GLenum = GLenum(0x882C);

pub const GL_DRAW_BUFFER8: GLenum = GLenum(0x882D);

pub const GL_DRAW_BUFFER8_ARB: GLenum = GLenum(0x882D);

pub const GL_DRAW_BUFFER8_ATI: GLenum = GLenum(0x882D);

pub const GL_DRAW_BUFFER8_EXT: GLenum = GLenum(0x882D);

pub const GL_DRAW_BUFFER8_NV: GLenum = GLenum(0x882D);

pub const GL_DRAW_BUFFER9: GLenum = GLenum(0x882E);

pub const GL_DRAW_BUFFER9_ARB: GLenum = GLenum(0x882E);

pub const GL_DRAW_BUFFER9_ATI: GLenum = GLenum(0x882E);

pub const GL_DRAW_BUFFER9_EXT: GLenum = GLenum(0x882E);

pub const GL_DRAW_BUFFER9_NV: GLenum = GLenum(0x882E);

///
/// * Group: [`GetPName`]
pub const GL_DRAW_BUFFER_EXT: GLenum = GLenum(0x0C01);

///
/// * Group: [`CommandOpcodesNV`]
pub const GL_DRAW_ELEMENTS_COMMAND_NV: GLenum = GLenum(0x0002);

///
/// * Group: [`CommandOpcodesNV`]
pub const GL_DRAW_ELEMENTS_INSTANCED_COMMAND_NV: GLenum = GLenum(0x0006);

///
/// * Group: [`CommandOpcodesNV`]
pub const GL_DRAW_ELEMENTS_STRIP_COMMAND_NV: GLenum = GLenum(0x0004);

///
/// * Group: [`CheckFramebufferStatusTarget`], [`FramebufferTarget`]
pub const GL_DRAW_FRAMEBUFFER: GLenum = GLenum(0x8CA9);

pub const GL_DRAW_FRAMEBUFFER_ANGLE: GLenum = GLenum(0x8CA9);

pub const GL_DRAW_FRAMEBUFFER_APPLE: GLenum = GLenum(0x8CA9);

///
/// * Group: [`GetPName`]
pub const GL_DRAW_FRAMEBUFFER_BINDING: GLenum = GLenum(0x8CA6);

pub const GL_DRAW_FRAMEBUFFER_BINDING_ANGLE: GLenum = GLenum(0x8CA6);

pub const GL_DRAW_FRAMEBUFFER_BINDING_APPLE: GLenum = GLenum(0x8CA6);

pub const GL_DRAW_FRAMEBUFFER_BINDING_EXT: GLenum = GLenum(0x8CA6);

pub const GL_DRAW_FRAMEBUFFER_BINDING_NV: GLenum = GLenum(0x8CA6);

pub const GL_DRAW_FRAMEBUFFER_EXT: GLenum = GLenum(0x8CA9);

pub const GL_DRAW_FRAMEBUFFER_NV: GLenum = GLenum(0x8CA9);

pub const GL_DRAW_INDIRECT_ADDRESS_NV: GLenum = GLenum(0x8F41);

///
/// * Group: [`CopyBufferSubDataTarget`], [`BufferTargetARB`],
///   [`BufferStorageTarget`]
pub const GL_DRAW_INDIRECT_BUFFER: GLenum = GLenum(0x8F3F);

pub const GL_DRAW_INDIRECT_BUFFER_BINDING: GLenum = GLenum(0x8F43);

pub const GL_DRAW_INDIRECT_LENGTH_NV: GLenum = GLenum(0x8F42);

pub const GL_DRAW_INDIRECT_UNIFIED_NV: GLenum = GLenum(0x8F40);

///
/// * Group: [`ObjectTypeAPPLE`]
pub const GL_DRAW_PIXELS_APPLE: GLenum = GLenum(0x8A0A);

///
/// * Group: [`FeedBackToken`]
pub const GL_DRAW_PIXEL_TOKEN: GLenum = GLenum(0x0705);

///
/// * Group: [`GetPName`]
pub const GL_DRIVER_UUID_EXT: GLenum = GLenum(0x9598);

pub const GL_DSDT8_MAG8_INTENSITY8_NV: GLenum = GLenum(0x870B);

pub const GL_DSDT8_MAG8_NV: GLenum = GLenum(0x870A);

pub const GL_DSDT8_NV: GLenum = GLenum(0x8709);

pub const GL_DSDT_MAG_INTENSITY_NV: GLenum = GLenum(0x86DC);

pub const GL_DSDT_MAG_NV: GLenum = GLenum(0x86F6);

pub const GL_DSDT_MAG_VIB_NV: GLenum = GLenum(0x86F7);

pub const GL_DSDT_NV: GLenum = GLenum(0x86F5);

///
/// * Group: [`BlendingFactor`]
pub const GL_DST_ALPHA: GLenum = GLenum(0x0304);

pub const GL_DST_ATOP_NV: GLenum = GLenum(0x928F);

///
/// * Group: [`BlendingFactor`]
pub const GL_DST_COLOR: GLenum = GLenum(0x0306);

pub const GL_DST_IN_NV: GLenum = GLenum(0x928B);

pub const GL_DST_NV: GLenum = GLenum(0x9287);

pub const GL_DST_OUT_NV: GLenum = GLenum(0x928D);

pub const GL_DST_OVER_NV: GLenum = GLenum(0x9289);

pub const GL_DS_BIAS_NV: GLenum = GLenum(0x8716);

pub const GL_DS_SCALE_NV: GLenum = GLenum(0x8710);

pub const GL_DT_BIAS_NV: GLenum = GLenum(0x8717);

pub const GL_DT_SCALE_NV: GLenum = GLenum(0x8711);

pub const GL_DU8DV8_ATI: GLenum = GLenum(0x877A);

///
/// * Group: [`InternalFormat`]
pub const GL_DUAL_ALPHA12_SGIS: GLenum = GLenum(0x8112);

///
/// * Group: [`InternalFormat`]
pub const GL_DUAL_ALPHA16_SGIS: GLenum = GLenum(0x8113);

///
/// * Group: [`InternalFormat`]
pub const GL_DUAL_ALPHA4_SGIS: GLenum = GLenum(0x8110);

///
/// * Group: [`InternalFormat`]
pub const GL_DUAL_ALPHA8_SGIS: GLenum = GLenum(0x8111);

///
/// * Group: [`InternalFormat`]
pub const GL_DUAL_INTENSITY12_SGIS: GLenum = GLenum(0x811A);

///
/// * Group: [`InternalFormat`]
pub const GL_DUAL_INTENSITY16_SGIS: GLenum = GLenum(0x811B);

///
/// * Group: [`InternalFormat`]
pub const GL_DUAL_INTENSITY4_SGIS: GLenum = GLenum(0x8118);

///
/// * Group: [`InternalFormat`]
pub const GL_DUAL_INTENSITY8_SGIS: GLenum = GLenum(0x8119);

///
/// * Group: [`InternalFormat`]
pub const GL_DUAL_LUMINANCE12_SGIS: GLenum = GLenum(0x8116);

///
/// * Group: [`InternalFormat`]
pub const GL_DUAL_LUMINANCE16_SGIS: GLenum = GLenum(0x8117);

///
/// * Group: [`InternalFormat`]
pub const GL_DUAL_LUMINANCE4_SGIS: GLenum = GLenum(0x8114);

///
/// * Group: [`InternalFormat`]
pub const GL_DUAL_LUMINANCE8_SGIS: GLenum = GLenum(0x8115);

///
/// * Group: [`InternalFormat`]
pub const GL_DUAL_LUMINANCE_ALPHA4_SGIS: GLenum = GLenum(0x811C);

///
/// * Group: [`InternalFormat`]
pub const GL_DUAL_LUMINANCE_ALPHA8_SGIS: GLenum = GLenum(0x811D);

///
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_DUAL_TEXTURE_SELECT_SGIS: GLenum = GLenum(0x8124);

pub const GL_DUDV_ATI: GLenum = GLenum(0x8779);

///
/// * Group: [`PathCoordType`]
pub const GL_DUP_FIRST_CUBIC_CURVE_TO_NV: GLenum = GLenum(0xF2);

///
/// * Group: [`PathCoordType`]
pub const GL_DUP_LAST_CUBIC_CURVE_TO_NV: GLenum = GLenum(0xF4);

///
/// * Group: [`ArrayObjectUsageATI`]
pub const GL_DYNAMIC_ATI: GLenum = GLenum(0x8761);

///
/// * Group: [`VertexBufferObjectUsage`], [`BufferUsageARB`]
pub const GL_DYNAMIC_COPY: GLenum = GLenum(0x88EA);

pub const GL_DYNAMIC_COPY_ARB: GLenum = GLenum(0x88EA);

///
/// * Group: [`VertexBufferObjectUsage`], [`BufferUsageARB`]
pub const GL_DYNAMIC_DRAW: GLenum = GLenum(0x88E8);

pub const GL_DYNAMIC_DRAW_ARB: GLenum = GLenum(0x88E8);

///
/// * Group: [`VertexBufferObjectUsage`], [`BufferUsageARB`]
pub const GL_DYNAMIC_READ: GLenum = GLenum(0x88E9);

pub const GL_DYNAMIC_READ_ARB: GLenum = GLenum(0x88E9);

///
/// * Group: [`BufferStorageMask`]
pub const GL_DYNAMIC_STORAGE_BIT: GLbitfield = GLbitfield(0x0100);

///
/// * Group: [`BufferStorageMask`]
pub const GL_DYNAMIC_STORAGE_BIT_EXT: GLbitfield = GLbitfield(0x0100);

///
/// * Group: [`VertexHintsMaskPGI`]
pub const GL_EDGEFLAG_BIT_PGI: GLbitfield = GLbitfield(0x00040000);

///
/// * Group: [`GetPName`]
pub const GL_EDGE_FLAG: GLenum = GLenum(0x0B43);

///
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_EDGE_FLAG_ARRAY: GLenum = GLenum(0x8079);

pub const GL_EDGE_FLAG_ARRAY_ADDRESS_NV: GLenum = GLenum(0x8F26);

pub const GL_EDGE_FLAG_ARRAY_BUFFER_BINDING: GLenum = GLenum(0x889B);

pub const GL_EDGE_FLAG_ARRAY_BUFFER_BINDING_ARB: GLenum = GLenum(0x889B);

///
/// * Group: [`GetPName`]
pub const GL_EDGE_FLAG_ARRAY_COUNT_EXT: GLenum = GLenum(0x808D);

pub const GL_EDGE_FLAG_ARRAY_EXT: GLenum = GLenum(0x8079);

pub const GL_EDGE_FLAG_ARRAY_LENGTH_NV: GLenum = GLenum(0x8F30);

pub const GL_EDGE_FLAG_ARRAY_LIST_IBM: GLenum = GLenum(103075);

pub const GL_EDGE_FLAG_ARRAY_LIST_STRIDE_IBM: GLenum = GLenum(103085);

///
/// * Group: [`GetPointervPName`]
pub const GL_EDGE_FLAG_ARRAY_POINTER: GLenum = GLenum(0x8093);

///
/// * Group: [`GetPointervPName`]
pub const GL_EDGE_FLAG_ARRAY_POINTER_EXT: GLenum = GLenum(0x8093);

///
/// * Group: [`GetPName`]
pub const GL_EDGE_FLAG_ARRAY_STRIDE: GLenum = GLenum(0x808C);

pub const GL_EDGE_FLAG_ARRAY_STRIDE_EXT: GLenum = GLenum(0x808C);

pub const GL_EFFECTIVE_RASTER_SAMPLES_EXT: GLenum = GLenum(0x932C);

///
/// * Group: [`FragmentShaderDestModMaskATI`]
pub const GL_EIGHTH_BIT_ATI: GLbitfield = GLbitfield(0x00000020);

///
/// * Group: [`CommandOpcodesNV`]
pub const GL_ELEMENT_ADDRESS_COMMAND_NV: GLenum = GLenum(0x0008);

pub const GL_ELEMENT_ARRAY_ADDRESS_NV: GLenum = GLenum(0x8F29);

pub const GL_ELEMENT_ARRAY_APPLE: GLenum = GLenum(0x8A0C);

pub const GL_ELEMENT_ARRAY_ATI: GLenum = GLenum(0x8768);

///
/// * Group: [`MemoryBarrierMask`]
pub const GL_ELEMENT_ARRAY_BARRIER_BIT: GLbitfield = GLbitfield(0x00000002);

///
/// * Group: [`MemoryBarrierMask`]
pub const GL_ELEMENT_ARRAY_BARRIER_BIT_EXT: GLbitfield = GLbitfield(0x00000002);

///
/// * Group: [`CopyBufferSubDataTarget`], [`BufferTargetARB`],
///   [`BufferStorageTarget`]
pub const GL_ELEMENT_ARRAY_BUFFER: GLenum = GLenum(0x8893);

pub const GL_ELEMENT_ARRAY_BUFFER_ARB: GLenum = GLenum(0x8893);

///
/// * Group: [`GetPName`]
pub const GL_ELEMENT_ARRAY_BUFFER_BINDING: GLenum = GLenum(0x8895);

pub const GL_ELEMENT_ARRAY_BUFFER_BINDING_ARB: GLenum = GLenum(0x8895);

pub const GL_ELEMENT_ARRAY_LENGTH_NV: GLenum = GLenum(0x8F33);

pub const GL_ELEMENT_ARRAY_POINTER_APPLE: GLenum = GLenum(0x8A0E);

pub const GL_ELEMENT_ARRAY_POINTER_ATI: GLenum = GLenum(0x876A);

pub const GL_ELEMENT_ARRAY_TYPE_APPLE: GLenum = GLenum(0x8A0D);

pub const GL_ELEMENT_ARRAY_TYPE_ATI: GLenum = GLenum(0x8769);

pub const GL_ELEMENT_ARRAY_UNIFIED_NV: GLenum = GLenum(0x8F1F);

pub const GL_EMBOSS_CONSTANT_NV: GLenum = GLenum(0x855E);

pub const GL_EMBOSS_LIGHT_NV: GLenum = GLenum(0x855D);

pub const GL_EMBOSS_MAP_NV: GLenum = GLenum(0x855F);

///
/// * Group: [`MaterialParameter`], [`ColorMaterialParameter`]
pub const GL_EMISSION: GLenum = GLenum(0x1600);

///
/// * Group: [`AttribMask`]
pub const GL_ENABLE_BIT: GLbitfield = GLbitfield(0x00002000);

///
/// * Group: [`StencilFunction`], [`IndexFunctionEXT`], [`AlphaFunction`],
///   [`DepthFunction`]
pub const GL_EQUAL: GLenum = GLenum(0x0202);

///
/// * Group: [`LogicOp`]
pub const GL_EQUIV: GLenum = GLenum(0x1509);

pub const GL_ETC1_RGB8_OES: GLenum = GLenum(0x8D64);

pub const GL_ETC1_SRGB8_NV: GLenum = GLenum(0x88EE);

///
/// * Group: [`EvalTargetNV`]
pub const GL_EVAL_2D_NV: GLenum = GLenum(0x86C0);

///
/// * Group: [`AttribMask`]
pub const GL_EVAL_BIT: GLbitfield = GLbitfield(0x00010000);

pub const GL_EVAL_FRACTIONAL_TESSELLATION_NV: GLenum = GLenum(0x86C5);

///
/// * Group: [`EvalTargetNV`]
pub const GL_EVAL_TRIANGULAR_2D_NV: GLenum = GLenum(0x86C1);

pub const GL_EVAL_VERTEX_ATTRIB0_NV: GLenum = GLenum(0x86C6);

pub const GL_EVAL_VERTEX_ATTRIB10_NV: GLenum = GLenum(0x86D0);

pub const GL_EVAL_VERTEX_ATTRIB11_NV: GLenum = GLenum(0x86D1);

pub const GL_EVAL_VERTEX_ATTRIB12_NV: GLenum = GLenum(0x86D2);

pub const GL_EVAL_VERTEX_ATTRIB13_NV: GLenum = GLenum(0x86D3);

pub const GL_EVAL_VERTEX_ATTRIB14_NV: GLenum = GLenum(0x86D4);

pub const GL_EVAL_VERTEX_ATTRIB15_NV: GLenum = GLenum(0x86D5);

pub const GL_EVAL_VERTEX_ATTRIB1_NV: GLenum = GLenum(0x86C7);

pub const GL_EVAL_VERTEX_ATTRIB2_NV: GLenum = GLenum(0x86C8);

pub const GL_EVAL_VERTEX_ATTRIB3_NV: GLenum = GLenum(0x86C9);

pub const GL_EVAL_VERTEX_ATTRIB4_NV: GLenum = GLenum(0x86CA);

pub const GL_EVAL_VERTEX_ATTRIB5_NV: GLenum = GLenum(0x86CB);

pub const GL_EVAL_VERTEX_ATTRIB6_NV: GLenum = GLenum(0x86CC);

pub const GL_EVAL_VERTEX_ATTRIB7_NV: GLenum = GLenum(0x86CD);

pub const GL_EVAL_VERTEX_ATTRIB8_NV: GLenum = GLenum(0x86CE);

pub const GL_EVAL_VERTEX_ATTRIB9_NV: GLenum = GLenum(0x86CF);

pub const GL_EXCLUSION: GLenum = GLenum(0x92A0);

pub const GL_EXCLUSION_KHR: GLenum = GLenum(0x92A0);

pub const GL_EXCLUSION_NV: GLenum = GLenum(0x92A0);

pub const GL_EXCLUSIVE_EXT: GLenum = GLenum(0x8F11);

///
/// * Group: [`FogMode`]
pub const GL_EXP: GLenum = GLenum(0x0800);

///
/// * Group: [`FogMode`]
pub const GL_EXP2: GLenum = GLenum(0x0801);

///
/// * Group: [`CombinerMappingNV`]
pub const GL_EXPAND_NEGATE_NV: GLenum = GLenum(0x8539);

///
/// * Group: [`CombinerMappingNV`]
pub const GL_EXPAND_NORMAL_NV: GLenum = GLenum(0x8538);

///
/// * Group: [`StringName`]
pub const GL_EXTENSIONS: GLenum = GLenum(0x1F03);

///
/// * Group: [`BufferStorageMask`]
pub const GL_EXTERNAL_STORAGE_BIT_NVX: GLbitfield = GLbitfield(0x2000);

pub const GL_EXTERNAL_VIRTUAL_MEMORY_BUFFER_AMD: GLenum = GLenum(0x9160);

///
/// * Group: [`TextureGenMode`]
pub const GL_EYE_DISTANCE_TO_LINE_SGIS: GLenum = GLenum(0x81F2);

///
/// * Group: [`TextureGenMode`]
pub const GL_EYE_DISTANCE_TO_POINT_SGIS: GLenum = GLenum(0x81F0);

///
/// * Group: [`PathGenMode`], [`TextureGenMode`]
pub const GL_EYE_LINEAR: GLenum = GLenum(0x2400);

pub const GL_EYE_LINEAR_NV: GLenum = GLenum(0x2400);

///
/// * Group: [`TextureGenParameter`]
pub const GL_EYE_LINE_SGIS: GLenum = GLenum(0x81F6);

///
/// * Group: [`TextureGenParameter`]
pub const GL_EYE_PLANE: GLenum = GLenum(0x2502);

pub const GL_EYE_PLANE_ABSOLUTE_NV: GLenum = GLenum(0x855C);

///
/// * Group: [`TextureGenParameter`]
pub const GL_EYE_POINT_SGIS: GLenum = GLenum(0x81F4);

pub const GL_EYE_RADIAL_NV: GLenum = GLenum(0x855B);

pub const GL_E_TIMES_F_NV: GLenum = GLenum(0x8531);

pub const GL_FACTOR_ALPHA_MODULATE_IMG: GLenum = GLenum(0x8C07);

pub const GL_FACTOR_MAX_AMD: GLenum = GLenum(0x901D);

pub const GL_FACTOR_MIN_AMD: GLenum = GLenum(0x901C);

pub const GL_FAILURE_NV: GLenum = GLenum(0x9030);

///
/// * Group: [`Boolean`], [`VertexShaderWriteMaskEXT`], [`ClampColorModeARB`]
pub const GL_FALSE: GLenum = GLenum(0);

///
/// * Group: [`HintMode`]
pub const GL_FASTEST: GLenum = GLenum(0x1101);

///
/// * Group: [`RenderingMode`]
pub const GL_FEEDBACK: GLenum = GLenum(0x1C01);

///
/// * Group: [`GetPointervPName`]
pub const GL_FEEDBACK_BUFFER_POINTER: GLenum = GLenum(0x0DF0);

///
/// * Group: [`GetPName`]
pub const GL_FEEDBACK_BUFFER_SIZE: GLenum = GLenum(0x0DF1);

///
/// * Group: [`GetPName`]
pub const GL_FEEDBACK_BUFFER_TYPE: GLenum = GLenum(0x0DF2);

///
/// * Group: [`ObjectTypeAPPLE`]
pub const GL_FENCE_APPLE: GLenum = GLenum(0x8A0B);

///
/// * Group: [`FenceParameterNameNV`]
pub const GL_FENCE_CONDITION_NV: GLenum = GLenum(0x84F4);

///
/// * Group: [`FenceParameterNameNV`]
pub const GL_FENCE_STATUS_NV: GLenum = GLenum(0x84F3);

pub const GL_FETCH_PER_SAMPLE_ARM: GLenum = GLenum(0x8F65);

pub const GL_FIELDS_NV: GLenum = GLenum(0x8E27);

pub const GL_FIELD_LOWER_NV: GLenum = GLenum(0x9023);

pub const GL_FIELD_UPPER_NV: GLenum = GLenum(0x9022);

///
/// * Group: [`PathFontTarget`]
pub const GL_FILE_NAME_NV: GLenum = GLenum(0x9074);

///
/// * Group: [`PolygonMode`], [`MeshMode2`]
pub const GL_FILL: GLenum = GLenum(0x1B02);

///
/// * Group: [`EvalMapsModeNV`]
pub const GL_FILL_NV: GLenum = GLenum(0x1B02);

pub const GL_FILL_RECTANGLE_NV: GLenum = GLenum(0x933C);

///
/// * Group: [`InternalFormatPName`]
pub const GL_FILTER: GLenum = GLenum(0x829A);

///
/// * Group: [`TextureMinFilter`], [`TextureFilterSGIS`],
///   [`TextureFilterFuncSGIS`], [`TextureMagFilter`]
pub const GL_FILTER4_SGIS: GLenum = GLenum(0x8146);

///
/// * Group: [`PathListMode`]
pub const GL_FIRST_TO_REST_NV: GLenum = GLenum(0x90AF);

///
/// * Group: [`VertexProvokingMode`]
pub const GL_FIRST_VERTEX_CONVENTION: GLenum = GLenum(0x8E4D);

pub const GL_FIRST_VERTEX_CONVENTION_EXT: GLenum = GLenum(0x8E4D);

pub const GL_FIRST_VERTEX_CONVENTION_OES: GLenum = GLenum(0x8E4D);

///
/// * Group: [`VertexAttribPointerType`], [`VertexAttribType`]
pub const GL_FIXED: GLenum = GLenum(0x140C);

pub const GL_FIXED_OES: GLenum = GLenum(0x140C);

///
/// * Group: [`ClampColorModeARB`]
pub const GL_FIXED_ONLY: GLenum = GLenum(0x891D);

///
/// * Group: [`ClampColorModeARB`]
pub const GL_FIXED_ONLY_ARB: GLenum = GLenum(0x891D);

///
/// * Group: [`ShadingModel`]
pub const GL_FLAT: GLenum = GLenum(0x1D00);

///
/// * Group: [`GlslTypeToken`], [`MapTypeNV`], [`SecondaryColorPointerTypeIBM`],
///   [`WeightPointerTypeARB`], [`VertexWeightPointerTypeEXT`],
///   [`TangentPointerTypeEXT`], [`BinormalPointerTypeEXT`],
///   [`FogCoordinatePointerType`], [`FogPointerTypeEXT`],
///   [`FogPointerTypeIBM`], [`IndexPointerType`], [`ListNameType`],
///   [`NormalPointerType`], [`PixelType`], [`TexCoordPointerType`],
///   [`VertexPointerType`], [`VertexAttribType`], [`AttributeType`],
///   [`UniformType`], [`VertexAttribPointerType`]
pub const GL_FLOAT: GLenum = GLenum(0x1406);

pub const GL_FLOAT16_MAT2_AMD: GLenum = GLenum(0x91C5);

pub const GL_FLOAT16_MAT2x3_AMD: GLenum = GLenum(0x91C8);

pub const GL_FLOAT16_MAT2x4_AMD: GLenum = GLenum(0x91C9);

pub const GL_FLOAT16_MAT3_AMD: GLenum = GLenum(0x91C6);

pub const GL_FLOAT16_MAT3x2_AMD: GLenum = GLenum(0x91CA);

pub const GL_FLOAT16_MAT3x4_AMD: GLenum = GLenum(0x91CB);

pub const GL_FLOAT16_MAT4_AMD: GLenum = GLenum(0x91C7);

pub const GL_FLOAT16_MAT4x2_AMD: GLenum = GLenum(0x91CC);

pub const GL_FLOAT16_MAT4x3_AMD: GLenum = GLenum(0x91CD);

pub const GL_FLOAT16_NV: GLenum = GLenum(0x8FF8);

pub const GL_FLOAT16_VEC2_NV: GLenum = GLenum(0x8FF9);

pub const GL_FLOAT16_VEC3_NV: GLenum = GLenum(0x8FFA);

pub const GL_FLOAT16_VEC4_NV: GLenum = GLenum(0x8FFB);

pub const GL_FLOAT_32_UNSIGNED_INT_24_8_REV: GLenum = GLenum(0x8DAD);

pub const GL_FLOAT_32_UNSIGNED_INT_24_8_REV_NV: GLenum = GLenum(0x8DAD);

pub const GL_FLOAT_CLEAR_COLOR_VALUE_NV: GLenum = GLenum(0x888D);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_FLOAT_MAT2: GLenum = GLenum(0x8B5A);

///
/// * Group: [`AttributeType`]
pub const GL_FLOAT_MAT2_ARB: GLenum = GLenum(0x8B5A);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_FLOAT_MAT2x3: GLenum = GLenum(0x8B65);

///
/// * Group: [`AttributeType`]
pub const GL_FLOAT_MAT2x3_NV: GLenum = GLenum(0x8B65);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_FLOAT_MAT2x4: GLenum = GLenum(0x8B66);

///
/// * Group: [`AttributeType`]
pub const GL_FLOAT_MAT2x4_NV: GLenum = GLenum(0x8B66);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_FLOAT_MAT3: GLenum = GLenum(0x8B5B);

///
/// * Group: [`AttributeType`]
pub const GL_FLOAT_MAT3_ARB: GLenum = GLenum(0x8B5B);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_FLOAT_MAT3x2: GLenum = GLenum(0x8B67);

///
/// * Group: [`AttributeType`]
pub const GL_FLOAT_MAT3x2_NV: GLenum = GLenum(0x8B67);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_FLOAT_MAT3x4: GLenum = GLenum(0x8B68);

///
/// * Group: [`AttributeType`]
pub const GL_FLOAT_MAT3x4_NV: GLenum = GLenum(0x8B68);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_FLOAT_MAT4: GLenum = GLenum(0x8B5C);

///
/// * Group: [`AttributeType`]
pub const GL_FLOAT_MAT4_ARB: GLenum = GLenum(0x8B5C);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_FLOAT_MAT4x2: GLenum = GLenum(0x8B69);

///
/// * Group: [`AttributeType`]
pub const GL_FLOAT_MAT4x2_NV: GLenum = GLenum(0x8B69);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_FLOAT_MAT4x3: GLenum = GLenum(0x8B6A);

///
/// * Group: [`AttributeType`]
pub const GL_FLOAT_MAT4x3_NV: GLenum = GLenum(0x8B6A);

pub const GL_FLOAT_R16_NV: GLenum = GLenum(0x8884);

pub const GL_FLOAT_R32_NV: GLenum = GLenum(0x8885);

pub const GL_FLOAT_RG16_NV: GLenum = GLenum(0x8886);

pub const GL_FLOAT_RG32_NV: GLenum = GLenum(0x8887);

pub const GL_FLOAT_RGB16_NV: GLenum = GLenum(0x8888);

pub const GL_FLOAT_RGB32_NV: GLenum = GLenum(0x8889);

pub const GL_FLOAT_RGBA16_NV: GLenum = GLenum(0x888A);

pub const GL_FLOAT_RGBA32_NV: GLenum = GLenum(0x888B);

pub const GL_FLOAT_RGBA_MODE_NV: GLenum = GLenum(0x888E);

pub const GL_FLOAT_RGBA_NV: GLenum = GLenum(0x8883);

pub const GL_FLOAT_RGB_NV: GLenum = GLenum(0x8882);

pub const GL_FLOAT_RG_NV: GLenum = GLenum(0x8881);

pub const GL_FLOAT_R_NV: GLenum = GLenum(0x8880);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_FLOAT_VEC2: GLenum = GLenum(0x8B50);

///
/// * Group: [`AttributeType`]
pub const GL_FLOAT_VEC2_ARB: GLenum = GLenum(0x8B50);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_FLOAT_VEC3: GLenum = GLenum(0x8B51);

///
/// * Group: [`AttributeType`]
pub const GL_FLOAT_VEC3_ARB: GLenum = GLenum(0x8B51);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_FLOAT_VEC4: GLenum = GLenum(0x8B52);

///
/// * Group: [`AttributeType`]
pub const GL_FLOAT_VEC4_ARB: GLenum = GLenum(0x8B52);

///
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_FOG: GLenum = GLenum(0x0B60);

///
/// * Group: [`AttribMask`]
pub const GL_FOG_BIT: GLbitfield = GLbitfield(0x00000080);

///
/// * Group: [`GetPName`], [`FogParameter`]
pub const GL_FOG_COLOR: GLenum = GLenum(0x0B66);

///
/// * Alias Of: [`GL_FOG_COORDINATE`]
pub const GL_FOG_COORD: GLenum = GL_FOG_COORDINATE;

pub const GL_FOG_COORDINATE: GLenum = GLenum(0x8451);

pub const GL_FOG_COORDINATE_ARRAY: GLenum = GLenum(0x8457);

pub const GL_FOG_COORDINATE_ARRAY_BUFFER_BINDING: GLenum = GLenum(0x889D);

pub const GL_FOG_COORDINATE_ARRAY_BUFFER_BINDING_ARB: GLenum = GLenum(0x889D);

pub const GL_FOG_COORDINATE_ARRAY_EXT: GLenum = GLenum(0x8457);

pub const GL_FOG_COORDINATE_ARRAY_LIST_IBM: GLenum = GLenum(103076);

pub const GL_FOG_COORDINATE_ARRAY_LIST_STRIDE_IBM: GLenum = GLenum(103086);

pub const GL_FOG_COORDINATE_ARRAY_POINTER: GLenum = GLenum(0x8456);

pub const GL_FOG_COORDINATE_ARRAY_POINTER_EXT: GLenum = GLenum(0x8456);

pub const GL_FOG_COORDINATE_ARRAY_STRIDE: GLenum = GLenum(0x8455);

pub const GL_FOG_COORDINATE_ARRAY_STRIDE_EXT: GLenum = GLenum(0x8455);

pub const GL_FOG_COORDINATE_ARRAY_TYPE: GLenum = GLenum(0x8454);

pub const GL_FOG_COORDINATE_ARRAY_TYPE_EXT: GLenum = GLenum(0x8454);

pub const GL_FOG_COORDINATE_EXT: GLenum = GLenum(0x8451);

pub const GL_FOG_COORDINATE_SOURCE: GLenum = GLenum(0x8450);

pub const GL_FOG_COORDINATE_SOURCE_EXT: GLenum = GLenum(0x8450);

///
/// * Alias Of: [`GL_FOG_COORDINATE_ARRAY`]
pub const GL_FOG_COORD_ARRAY: GLenum = GL_FOG_COORDINATE_ARRAY;

pub const GL_FOG_COORD_ARRAY_ADDRESS_NV: GLenum = GLenum(0x8F28);

///
/// * Alias Of: [`GL_FOG_COORDINATE_ARRAY_BUFFER_BINDING`]
pub const GL_FOG_COORD_ARRAY_BUFFER_BINDING: GLenum = GL_FOG_COORDINATE_ARRAY_BUFFER_BINDING;

pub const GL_FOG_COORD_ARRAY_LENGTH_NV: GLenum = GLenum(0x8F32);

///
/// * Alias Of: [`GL_FOG_COORDINATE_ARRAY_POINTER`]
pub const GL_FOG_COORD_ARRAY_POINTER: GLenum = GL_FOG_COORDINATE_ARRAY_POINTER;

///
/// * Alias Of: [`GL_FOG_COORDINATE_ARRAY_STRIDE`]
pub const GL_FOG_COORD_ARRAY_STRIDE: GLenum = GL_FOG_COORDINATE_ARRAY_STRIDE;

///
/// * Alias Of: [`GL_FOG_COORDINATE_ARRAY_TYPE`]
pub const GL_FOG_COORD_ARRAY_TYPE: GLenum = GL_FOG_COORDINATE_ARRAY_TYPE;

///
/// * Group: [`FogPName`]
/// * Alias Of: [`GL_FOG_COORDINATE_SOURCE`]
pub const GL_FOG_COORD_SRC: GLenum = GL_FOG_COORDINATE_SOURCE;

///
/// * Group: [`FogPName`], [`FogParameter`], [`GetPName`]
pub const GL_FOG_DENSITY: GLenum = GLenum(0x0B62);

pub const GL_FOG_DISTANCE_MODE_NV: GLenum = GLenum(0x855A);

///
/// * Group: [`FogPName`], [`FogParameter`], [`GetPName`]
pub const GL_FOG_END: GLenum = GLenum(0x0B64);

///
/// * Group: [`GetPName`]
pub const GL_FOG_FUNC_POINTS_SGIS: GLenum = GLenum(0x812B);

///
/// * Group: [`FogMode`]
pub const GL_FOG_FUNC_SGIS: GLenum = GLenum(0x812A);

///
/// * Group: [`HintTarget`], [`GetPName`]
pub const GL_FOG_HINT: GLenum = GLenum(0x0C54);

///
/// * Group: [`FogPName`], [`FogParameter`], [`GetPName`]
pub const GL_FOG_INDEX: GLenum = GLenum(0x0B61);

///
/// * Group: [`FogPName`], [`FogParameter`], [`GetPName`]
pub const GL_FOG_MODE: GLenum = GLenum(0x0B65);

///
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_FOG_OFFSET_SGIX: GLenum = GLenum(0x8198);

///
/// * Group: [`GetPName`], [`FogParameter`]
pub const GL_FOG_OFFSET_VALUE_SGIX: GLenum = GLenum(0x8199);

pub const GL_FOG_SPECULAR_TEXTURE_WIN: GLenum = GLenum(0x80EC);

///
/// * Group: [`FogPName`], [`FogParameter`], [`GetPName`]
pub const GL_FOG_START: GLenum = GLenum(0x0B63);

///
/// * Group: [`PathMetricMask`]
pub const GL_FONT_ASCENDER_BIT_NV: GLbitfield = GLbitfield(0x00200000);

///
/// * Group: [`PathMetricMask`]
pub const GL_FONT_DESCENDER_BIT_NV: GLbitfield = GLbitfield(0x00400000);

pub const GL_FONT_GLYPHS_AVAILABLE_NV: GLenum = GLenum(0x9368);

///
/// * Group: [`PathMetricMask`]
pub const GL_FONT_HAS_KERNING_BIT_NV: GLbitfield = GLbitfield(0x10000000);

///
/// * Group: [`PathMetricMask`]
pub const GL_FONT_HEIGHT_BIT_NV: GLbitfield = GLbitfield(0x00800000);

///
/// * Group: [`PathMetricMask`]
pub const GL_FONT_MAX_ADVANCE_HEIGHT_BIT_NV: GLbitfield = GLbitfield(0x02000000);

///
/// * Group: [`PathMetricMask`]
pub const GL_FONT_MAX_ADVANCE_WIDTH_BIT_NV: GLbitfield = GLbitfield(0x01000000);

///
/// * Group: [`PathMetricMask`]
pub const GL_FONT_NUM_GLYPH_INDICES_BIT_NV: GLbitfield = GLbitfield(0x20000000);

pub const GL_FONT_TARGET_UNAVAILABLE_NV: GLenum = GLenum(0x9369);

pub const GL_FONT_UNAVAILABLE_NV: GLenum = GLenum(0x936A);

///
/// * Group: [`PathMetricMask`]
pub const GL_FONT_UNDERLINE_POSITION_BIT_NV: GLbitfield = GLbitfield(0x04000000);

///
/// * Group: [`PathMetricMask`]
pub const GL_FONT_UNDERLINE_THICKNESS_BIT_NV: GLbitfield = GLbitfield(0x08000000);

pub const GL_FONT_UNINTELLIGIBLE_NV: GLenum = GLenum(0x936B);

///
/// * Group: [`PathMetricMask`]
pub const GL_FONT_UNITS_PER_EM_BIT_NV: GLbitfield = GLbitfield(0x00100000);

///
/// * Group: [`PathMetricMask`]
pub const GL_FONT_X_MAX_BOUNDS_BIT_NV: GLbitfield = GLbitfield(0x00040000);

///
/// * Group: [`PathMetricMask`]
pub const GL_FONT_X_MIN_BOUNDS_BIT_NV: GLbitfield = GLbitfield(0x00010000);

///
/// * Group: [`PathMetricMask`]
pub const GL_FONT_Y_MAX_BOUNDS_BIT_NV: GLbitfield = GLbitfield(0x00080000);

///
/// * Group: [`PathMetricMask`]
pub const GL_FONT_Y_MIN_BOUNDS_BIT_NV: GLbitfield = GLbitfield(0x00020000);

pub const GL_FORCE_BLUE_TO_ONE_NV: GLenum = GLenum(0x8860);

pub const GL_FORMAT_SUBSAMPLE_244_244_OML: GLenum = GLenum(0x8983);

pub const GL_FORMAT_SUBSAMPLE_24_24_OML: GLenum = GLenum(0x8982);

///
/// * Group: [`FoveationConfigBitQCOM`]
pub const GL_FOVEATION_ENABLE_BIT_QCOM: GLbitfield = GLbitfield(0x00000001);

///
/// * Group: [`FoveationConfigBitQCOM`]
pub const GL_FOVEATION_SCALED_BIN_METHOD_BIT_QCOM: GLbitfield = GLbitfield(0x00000002);

///
/// * Group: [`FoveationConfigBitQCOM`]
pub const GL_FOVEATION_SUBSAMPLED_LAYOUT_METHOD_BIT_QCOM: GLbitfield = GLbitfield(0x00000004);

pub const GL_FRACTIONAL_EVEN: GLenum = GLenum(0x8E7C);

pub const GL_FRACTIONAL_EVEN_EXT: GLenum = GLenum(0x8E7C);

pub const GL_FRACTIONAL_EVEN_OES: GLenum = GLenum(0x8E7C);

pub const GL_FRACTIONAL_ODD: GLenum = GLenum(0x8E7B);

pub const GL_FRACTIONAL_ODD_EXT: GLenum = GLenum(0x8E7B);

pub const GL_FRACTIONAL_ODD_OES: GLenum = GLenum(0x8E7B);

pub const GL_FRAGMENTS_INSTRUMENT_COUNTERS_SGIX: GLenum = GLenum(0x8314);

pub const GL_FRAGMENTS_INSTRUMENT_MAX_SGIX: GLenum = GLenum(0x8315);

pub const GL_FRAGMENTS_INSTRUMENT_SGIX: GLenum = GLenum(0x8313);

pub const GL_FRAGMENT_ALPHA_MODULATE_IMG: GLenum = GLenum(0x8C08);

///
/// * Group: [`LightTextureModeEXT`]
pub const GL_FRAGMENT_COLOR_EXT: GLenum = GLenum(0x834C);

///
/// * Group: [`GetPName`]
pub const GL_FRAGMENT_COLOR_MATERIAL_FACE_SGIX: GLenum = GLenum(0x8402);

///
/// * Group: [`GetPName`]
pub const GL_FRAGMENT_COLOR_MATERIAL_PARAMETER_SGIX: GLenum = GLenum(0x8403);

///
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_FRAGMENT_COLOR_MATERIAL_SGIX: GLenum = GLenum(0x8401);

pub const GL_FRAGMENT_COVERAGE_COLOR_NV: GLenum = GLenum(0x92DE);

pub const GL_FRAGMENT_COVERAGE_TO_COLOR_NV: GLenum = GLenum(0x92DD);

pub const GL_FRAGMENT_DEPTH: GLenum = GLenum(0x8452);

///
/// * Group: [`LightTextureModeEXT`]
pub const GL_FRAGMENT_DEPTH_EXT: GLenum = GLenum(0x8452);

pub const GL_FRAGMENT_INPUT_NV: GLenum = GLenum(0x936D);

pub const GL_FRAGMENT_INTERPOLATION_OFFSET_BITS: GLenum = GLenum(0x8E5D);

pub const GL_FRAGMENT_INTERPOLATION_OFFSET_BITS_OES: GLenum = GLenum(0x8E5D);

///
/// * Group: [`LightName`], [`FragmentLightNameSGIX`], [`EnableCap`],
///   [`GetPName`]
pub const GL_FRAGMENT_LIGHT0_SGIX: GLenum = GLenum(0x840C);

///
/// * Group: [`LightName`], [`FragmentLightNameSGIX`], [`EnableCap`]
pub const GL_FRAGMENT_LIGHT1_SGIX: GLenum = GLenum(0x840D);

///
/// * Group: [`LightName`], [`FragmentLightNameSGIX`], [`EnableCap`]
pub const GL_FRAGMENT_LIGHT2_SGIX: GLenum = GLenum(0x840E);

///
/// * Group: [`LightName`], [`FragmentLightNameSGIX`], [`EnableCap`]
pub const GL_FRAGMENT_LIGHT3_SGIX: GLenum = GLenum(0x840F);

///
/// * Group: [`LightName`], [`FragmentLightNameSGIX`], [`EnableCap`]
pub const GL_FRAGMENT_LIGHT4_SGIX: GLenum = GLenum(0x8410);

///
/// * Group: [`LightName`], [`FragmentLightNameSGIX`], [`EnableCap`]
pub const GL_FRAGMENT_LIGHT5_SGIX: GLenum = GLenum(0x8411);

///
/// * Group: [`LightName`], [`FragmentLightNameSGIX`], [`EnableCap`]
pub const GL_FRAGMENT_LIGHT6_SGIX: GLenum = GLenum(0x8412);

///
/// * Group: [`LightName`], [`FragmentLightNameSGIX`], [`EnableCap`]
pub const GL_FRAGMENT_LIGHT7_SGIX: GLenum = GLenum(0x8413);

///
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_FRAGMENT_LIGHTING_SGIX: GLenum = GLenum(0x8400);

///
/// * Group: [`GetPName`], [`FragmentLightModelParameterSGIX`]
pub const GL_FRAGMENT_LIGHT_MODEL_AMBIENT_SGIX: GLenum = GLenum(0x840A);

///
/// * Group: [`GetPName`], [`FragmentLightModelParameterSGIX`]
pub const GL_FRAGMENT_LIGHT_MODEL_LOCAL_VIEWER_SGIX: GLenum = GLenum(0x8408);

///
/// * Group: [`GetPName`], [`FragmentLightModelParameterSGIX`]
pub const GL_FRAGMENT_LIGHT_MODEL_NORMAL_INTERPOLATION_SGIX: GLenum = GLenum(0x840B);

///
/// * Group: [`GetPName`], [`FragmentLightModelParameterSGIX`]
pub const GL_FRAGMENT_LIGHT_MODEL_TWO_SIDE_SGIX: GLenum = GLenum(0x8409);

///
/// * Group: [`LightTextureModeEXT`]
pub const GL_FRAGMENT_MATERIAL_EXT: GLenum = GLenum(0x8349);

///
/// * Group: [`LightTextureModeEXT`]
pub const GL_FRAGMENT_NORMAL_EXT: GLenum = GLenum(0x834A);

///
/// * Group: [`ProgramTarget`]
pub const GL_FRAGMENT_PROGRAM_ARB: GLenum = GLenum(0x8804);

pub const GL_FRAGMENT_PROGRAM_BINDING_NV: GLenum = GLenum(0x8873);

pub const GL_FRAGMENT_PROGRAM_CALLBACK_DATA_MESA: GLenum = GLenum(0x8BB3);

pub const GL_FRAGMENT_PROGRAM_CALLBACK_FUNC_MESA: GLenum = GLenum(0x8BB2);

pub const GL_FRAGMENT_PROGRAM_CALLBACK_MESA: GLenum = GLenum(0x8BB1);

pub const GL_FRAGMENT_PROGRAM_INTERPOLATION_OFFSET_BITS_NV: GLenum = GLenum(0x8E5D);

pub const GL_FRAGMENT_PROGRAM_NV: GLenum = GLenum(0x8870);

pub const GL_FRAGMENT_PROGRAM_PARAMETER_BUFFER_NV: GLenum = GLenum(0x8DA4);

pub const GL_FRAGMENT_PROGRAM_POSITION_MESA: GLenum = GLenum(0x8BB0);

///
/// * Group: [`PipelineParameterName`], [`ShaderType`]
pub const GL_FRAGMENT_SHADER: GLenum = GLenum(0x8B30);

///
/// * Group: [`ShaderType`]
pub const GL_FRAGMENT_SHADER_ARB: GLenum = GLenum(0x8B30);

pub const GL_FRAGMENT_SHADER_ATI: GLenum = GLenum(0x8920);

///
/// * Group: [`UseProgramStageMask`]
pub const GL_FRAGMENT_SHADER_BIT: GLbitfield = GLbitfield(0x00000002);

///
/// * Group: [`UseProgramStageMask`]
pub const GL_FRAGMENT_SHADER_BIT_EXT: GLbitfield = GLbitfield(0x00000002);

///
/// * Group: [`HintTarget`], [`GetPName`]
pub const GL_FRAGMENT_SHADER_DERIVATIVE_HINT: GLenum = GLenum(0x8B8B);

///
/// * Group: [`HintTarget`]
pub const GL_FRAGMENT_SHADER_DERIVATIVE_HINT_ARB: GLenum = GLenum(0x8B8B);

///
/// * Group: [`HintTarget`]
pub const GL_FRAGMENT_SHADER_DERIVATIVE_HINT_OES: GLenum = GLenum(0x8B8B);

pub const GL_FRAGMENT_SHADER_DISCARDS_SAMPLES_EXT: GLenum = GLenum(0x8A52);

pub const GL_FRAGMENT_SHADER_FRAMEBUFFER_FETCH_MRT_ARM: GLenum = GLenum(0x8F66);

pub const GL_FRAGMENT_SHADER_INVOCATIONS: GLenum = GLenum(0x82F4);

///
/// * Alias Of: [`GL_FRAGMENT_SHADER_INVOCATIONS`]
pub const GL_FRAGMENT_SHADER_INVOCATIONS_ARB: GLenum = GL_FRAGMENT_SHADER_INVOCATIONS;

///
/// * Group: [`ProgramInterface`]
pub const GL_FRAGMENT_SUBROUTINE: GLenum = GLenum(0x92EC);

///
/// * Group: [`ProgramInterface`]
pub const GL_FRAGMENT_SUBROUTINE_UNIFORM: GLenum = GLenum(0x92F2);

///
/// * Group: [`InternalFormatPName`]
pub const GL_FRAGMENT_TEXTURE: GLenum = GLenum(0x829F);

///
/// * Group: [`ObjectIdentifier`], [`FramebufferTarget`],
///   [`CheckFramebufferStatusTarget`]
pub const GL_FRAMEBUFFER: GLenum = GLenum(0x8D40);

///
/// * Group: [`FramebufferAttachmentParameterName`]
pub const GL_FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE: GLenum = GLenum(0x8215);

pub const GL_FRAMEBUFFER_ATTACHMENT_ANGLE: GLenum = GLenum(0x93A3);

///
/// * Group: [`FramebufferAttachmentParameterName`]
pub const GL_FRAMEBUFFER_ATTACHMENT_BLUE_SIZE: GLenum = GLenum(0x8214);

///
/// * Group: [`FramebufferAttachmentParameterName`]
pub const GL_FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING: GLenum = GLenum(0x8210);

///
/// * Group: [`FramebufferAttachmentParameterName`]
pub const GL_FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING_EXT: GLenum = GLenum(0x8210);

///
/// * Group: [`FramebufferAttachmentParameterName`]
pub const GL_FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE: GLenum = GLenum(0x8211);

///
/// * Group: [`FramebufferAttachmentParameterName`]
pub const GL_FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE_EXT: GLenum = GLenum(0x8211);

///
/// * Group: [`FramebufferAttachmentParameterName`]
pub const GL_FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE: GLenum = GLenum(0x8216);

///
/// * Group: [`FramebufferAttachmentParameterName`]
pub const GL_FRAMEBUFFER_ATTACHMENT_GREEN_SIZE: GLenum = GLenum(0x8213);

///
/// * Group: [`FramebufferAttachmentParameterName`]
pub const GL_FRAMEBUFFER_ATTACHMENT_LAYERED: GLenum = GLenum(0x8DA7);

///
/// * Group: [`FramebufferAttachmentParameterName`]
pub const GL_FRAMEBUFFER_ATTACHMENT_LAYERED_ARB: GLenum = GLenum(0x8DA7);

///
/// * Group: [`FramebufferAttachmentParameterName`]
pub const GL_FRAMEBUFFER_ATTACHMENT_LAYERED_EXT: GLenum = GLenum(0x8DA7);

///
/// * Group: [`FramebufferAttachmentParameterName`]
pub const GL_FRAMEBUFFER_ATTACHMENT_LAYERED_OES: GLenum = GLenum(0x8DA7);

///
/// * Group: [`FramebufferAttachmentParameterName`]
pub const GL_FRAMEBUFFER_ATTACHMENT_OBJECT_NAME: GLenum = GLenum(0x8CD1);

///
/// * Group: [`FramebufferAttachmentParameterName`]
pub const GL_FRAMEBUFFER_ATTACHMENT_OBJECT_NAME_EXT: GLenum = GLenum(0x8CD1);

///
/// * Group: [`FramebufferAttachmentParameterName`]
pub const GL_FRAMEBUFFER_ATTACHMENT_OBJECT_NAME_OES: GLenum = GLenum(0x8CD1);

///
/// * Group: [`FramebufferAttachmentParameterName`]
pub const GL_FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE: GLenum = GLenum(0x8CD0);

///
/// * Group: [`FramebufferAttachmentParameterName`]
pub const GL_FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE_EXT: GLenum = GLenum(0x8CD0);

///
/// * Group: [`FramebufferAttachmentParameterName`]
pub const GL_FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE_OES: GLenum = GLenum(0x8CD0);

///
/// * Group: [`FramebufferAttachmentParameterName`]
pub const GL_FRAMEBUFFER_ATTACHMENT_RED_SIZE: GLenum = GLenum(0x8212);

///
/// * Group: [`FramebufferAttachmentParameterName`]
pub const GL_FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE: GLenum = GLenum(0x8217);

///
/// * Group: [`FramebufferAttachmentParameterName`]
pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_3D_ZOFFSET_EXT: GLenum = GLenum(0x8CD4);

///
/// * Group: [`FramebufferAttachmentParameterName`]
pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_3D_ZOFFSET_OES: GLenum = GLenum(0x8CD4);

///
/// * Group: [`FramebufferAttachmentParameterName`]
pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_BASE_VIEW_INDEX_OVR: GLenum = GLenum(0x9632);

///
/// * Group: [`FramebufferAttachmentParameterName`]
pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE: GLenum = GLenum(0x8CD3);

///
/// * Group: [`FramebufferAttachmentParameterName`]
pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE_EXT: GLenum = GLenum(0x8CD3);

///
/// * Group: [`FramebufferAttachmentParameterName`]
pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE_OES: GLenum = GLenum(0x8CD3);

///
/// * Group: [`FramebufferAttachmentParameterName`]
pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER: GLenum = GLenum(0x8CD4);

///
/// * Group: [`FramebufferAttachmentParameterName`]
pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER_EXT: GLenum = GLenum(0x8CD4);

///
/// * Group: [`FramebufferAttachmentParameterName`]
pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL: GLenum = GLenum(0x8CD2);

///
/// * Group: [`FramebufferAttachmentParameterName`]
pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL_EXT: GLenum = GLenum(0x8CD2);

///
/// * Group: [`FramebufferAttachmentParameterName`]
pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL_OES: GLenum = GLenum(0x8CD2);

///
/// * Group: [`FramebufferAttachmentParameterName`]
pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_NUM_VIEWS_OVR: GLenum = GLenum(0x9630);

///
/// * Group: [`FramebufferAttachmentParameterName`]
pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_SAMPLES_EXT: GLenum = GLenum(0x8D6C);

///
/// * Group: [`FramebufferAttachmentParameterName`]
pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_SCALE_IMG: GLenum = GLenum(0x913F);

///
/// * Group: [`MemoryBarrierMask`]
pub const GL_FRAMEBUFFER_BARRIER_BIT: GLbitfield = GLbitfield(0x00000400);

///
/// * Group: [`MemoryBarrierMask`]
pub const GL_FRAMEBUFFER_BARRIER_BIT_EXT: GLbitfield = GLbitfield(0x00000400);

pub const GL_FRAMEBUFFER_BINDING: GLenum = GLenum(0x8CA6);

pub const GL_FRAMEBUFFER_BINDING_ANGLE: GLenum = GLenum(0x8CA6);

pub const GL_FRAMEBUFFER_BINDING_EXT: GLenum = GLenum(0x8CA6);

pub const GL_FRAMEBUFFER_BINDING_OES: GLenum = GLenum(0x8CA6);

///
/// * Group: [`InternalFormatPName`]
pub const GL_FRAMEBUFFER_BLEND: GLenum = GLenum(0x828B);

///
/// * Group: [`FramebufferStatus`]
pub const GL_FRAMEBUFFER_COMPLETE: GLenum = GLenum(0x8CD5);

pub const GL_FRAMEBUFFER_COMPLETE_EXT: GLenum = GLenum(0x8CD5);

pub const GL_FRAMEBUFFER_COMPLETE_OES: GLenum = GLenum(0x8CD5);

pub const GL_FRAMEBUFFER_DEFAULT: GLenum = GLenum(0x8218);

///
/// * Group: [`GetFramebufferParameter`], [`FramebufferParameterName`]
pub const GL_FRAMEBUFFER_DEFAULT_FIXED_SAMPLE_LOCATIONS: GLenum = GLenum(0x9314);

///
/// * Group: [`GetFramebufferParameter`], [`FramebufferParameterName`]
pub const GL_FRAMEBUFFER_DEFAULT_HEIGHT: GLenum = GLenum(0x9311);

///
/// * Group: [`GetFramebufferParameter`], [`FramebufferParameterName`]
pub const GL_FRAMEBUFFER_DEFAULT_LAYERS: GLenum = GLenum(0x9312);

pub const GL_FRAMEBUFFER_DEFAULT_LAYERS_EXT: GLenum = GLenum(0x9312);

pub const GL_FRAMEBUFFER_DEFAULT_LAYERS_OES: GLenum = GLenum(0x9312);

///
/// * Group: [`GetFramebufferParameter`], [`FramebufferParameterName`]
pub const GL_FRAMEBUFFER_DEFAULT_SAMPLES: GLenum = GLenum(0x9313);

///
/// * Group: [`GetFramebufferParameter`], [`FramebufferParameterName`]
pub const GL_FRAMEBUFFER_DEFAULT_WIDTH: GLenum = GLenum(0x9310);

pub const GL_FRAMEBUFFER_EXT: GLenum = GLenum(0x8D40);

///
/// * Group: [`FramebufferFetchNoncoherent`]
pub const GL_FRAMEBUFFER_FETCH_NONCOHERENT_QCOM: GLenum = GLenum(0x96A2);

pub const GL_FRAMEBUFFER_FLIP_X_MESA: GLenum = GLenum(0x8BBC);

pub const GL_FRAMEBUFFER_FLIP_Y_MESA: GLenum = GLenum(0x8BBB);

///
/// * Group: [`FramebufferStatus`]
pub const GL_FRAMEBUFFER_INCOMPLETE_ATTACHMENT: GLenum = GLenum(0x8CD6);

pub const GL_FRAMEBUFFER_INCOMPLETE_ATTACHMENT_EXT: GLenum = GLenum(0x8CD6);

pub const GL_FRAMEBUFFER_INCOMPLETE_ATTACHMENT_OES: GLenum = GLenum(0x8CD6);

pub const GL_FRAMEBUFFER_INCOMPLETE_DIMENSIONS: GLenum = GLenum(0x8CD9);

pub const GL_FRAMEBUFFER_INCOMPLETE_DIMENSIONS_EXT: GLenum = GLenum(0x8CD9);

pub const GL_FRAMEBUFFER_INCOMPLETE_DIMENSIONS_OES: GLenum = GLenum(0x8CD9);

///
/// * Group: [`FramebufferStatus`]
pub const GL_FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER: GLenum = GLenum(0x8CDB);

pub const GL_FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER_EXT: GLenum = GLenum(0x8CDB);

pub const GL_FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER_OES: GLenum = GLenum(0x8CDB);

pub const GL_FRAMEBUFFER_INCOMPLETE_FORMATS_EXT: GLenum = GLenum(0x8CDA);

pub const GL_FRAMEBUFFER_INCOMPLETE_FORMATS_OES: GLenum = GLenum(0x8CDA);

pub const GL_FRAMEBUFFER_INCOMPLETE_FOVEATION_QCOM: GLenum = GLenum(0x8BFF);

pub const GL_FRAMEBUFFER_INCOMPLETE_INSUFFICIENT_SHADER_COMBINED_LOCAL_STORAGE_EXT: GLenum = GLenum(0x9652);

pub const GL_FRAMEBUFFER_INCOMPLETE_LAYER_COUNT_ARB: GLenum = GLenum(0x8DA9);

pub const GL_FRAMEBUFFER_INCOMPLETE_LAYER_COUNT_EXT: GLenum = GLenum(0x8DA9);

///
/// * Group: [`FramebufferStatus`]
pub const GL_FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS: GLenum = GLenum(0x8DA8);

pub const GL_FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS_ARB: GLenum = GLenum(0x8DA8);

pub const GL_FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS_EXT: GLenum = GLenum(0x8DA8);

pub const GL_FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS_OES: GLenum = GLenum(0x8DA8);

///
/// * Group: [`FramebufferStatus`]
pub const GL_FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT: GLenum = GLenum(0x8CD7);

pub const GL_FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT_EXT: GLenum = GLenum(0x8CD7);

pub const GL_FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT_OES: GLenum = GLenum(0x8CD7);

///
/// * Group: [`FramebufferStatus`]
pub const GL_FRAMEBUFFER_INCOMPLETE_MULTISAMPLE: GLenum = GLenum(0x8D56);

pub const GL_FRAMEBUFFER_INCOMPLETE_MULTISAMPLE_AND_DOWNSAMPLE_IMG: GLenum = GLenum(0x913C);

pub const GL_FRAMEBUFFER_INCOMPLETE_MULTISAMPLE_ANGLE: GLenum = GLenum(0x8D56);

pub const GL_FRAMEBUFFER_INCOMPLETE_MULTISAMPLE_APPLE: GLenum = GLenum(0x8D56);

pub const GL_FRAMEBUFFER_INCOMPLETE_MULTISAMPLE_EXT: GLenum = GLenum(0x8D56);

pub const GL_FRAMEBUFFER_INCOMPLETE_MULTISAMPLE_IMG: GLenum = GLenum(0x9134);

pub const GL_FRAMEBUFFER_INCOMPLETE_MULTISAMPLE_NV: GLenum = GLenum(0x8D56);

///
/// * Group: [`FramebufferStatus`]
pub const GL_FRAMEBUFFER_INCOMPLETE_READ_BUFFER: GLenum = GLenum(0x8CDC);

pub const GL_FRAMEBUFFER_INCOMPLETE_READ_BUFFER_EXT: GLenum = GLenum(0x8CDC);

pub const GL_FRAMEBUFFER_INCOMPLETE_READ_BUFFER_OES: GLenum = GLenum(0x8CDC);

pub const GL_FRAMEBUFFER_INCOMPLETE_VIEW_TARGETS_OVR: GLenum = GLenum(0x9633);

///
/// * Group: [`FramebufferTarget`]
pub const GL_FRAMEBUFFER_OES: GLenum = GLenum(0x8D40);

pub const GL_FRAMEBUFFER_PROGRAMMABLE_SAMPLE_LOCATIONS_ARB: GLenum = GLenum(0x9342);

pub const GL_FRAMEBUFFER_PROGRAMMABLE_SAMPLE_LOCATIONS_NV: GLenum = GLenum(0x9342);

///
/// * Group: [`InternalFormatPName`]
pub const GL_FRAMEBUFFER_RENDERABLE: GLenum = GLenum(0x8289);

///
/// * Group: [`InternalFormatPName`]
pub const GL_FRAMEBUFFER_RENDERABLE_LAYERED: GLenum = GLenum(0x828A);

pub const GL_FRAMEBUFFER_SAMPLE_LOCATION_PIXEL_GRID_ARB: GLenum = GLenum(0x9343);

pub const GL_FRAMEBUFFER_SAMPLE_LOCATION_PIXEL_GRID_NV: GLenum = GLenum(0x9343);

///
/// * Group: [`EnableCap`]
pub const GL_FRAMEBUFFER_SRGB: GLenum = GLenum(0x8DB9);

pub const GL_FRAMEBUFFER_SRGB_CAPABLE_EXT: GLenum = GLenum(0x8DBA);

pub const GL_FRAMEBUFFER_SRGB_EXT: GLenum = GLenum(0x8DB9);

pub const GL_FRAMEBUFFER_SWAP_XY_MESA: GLenum = GLenum(0x8BBD);

///
/// * Group: [`FramebufferStatus`]
pub const GL_FRAMEBUFFER_UNDEFINED: GLenum = GLenum(0x8219);

pub const GL_FRAMEBUFFER_UNDEFINED_OES: GLenum = GLenum(0x8219);

///
/// * Group: [`FramebufferStatus`]
pub const GL_FRAMEBUFFER_UNSUPPORTED: GLenum = GLenum(0x8CDD);

pub const GL_FRAMEBUFFER_UNSUPPORTED_EXT: GLenum = GLenum(0x8CDD);

pub const GL_FRAMEBUFFER_UNSUPPORTED_OES: GLenum = GLenum(0x8CDD);

///
/// * Group: [`GetPName`]
pub const GL_FRAMEZOOM_FACTOR_SGIX: GLenum = GLenum(0x818C);

///
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_FRAMEZOOM_SGIX: GLenum = GLenum(0x818B);

pub const GL_FRAME_NV: GLenum = GLenum(0x8E26);

///
/// * Group: [`ColorBuffer`], [`ColorMaterialFace`], [`CullFaceMode`],
///   [`DrawBufferMode`], [`ReadBufferMode`], [`StencilFaceDirection`],
///   [`MaterialFace`]
pub const GL_FRONT: GLenum = GLenum(0x0404);

///
/// * Group: [`ColorBuffer`], [`ColorMaterialFace`], [`CullFaceMode`],
///   [`DrawBufferMode`], [`StencilFaceDirection`], [`MaterialFace`]
pub const GL_FRONT_AND_BACK: GLenum = GLenum(0x0408);

///
/// * Group: [`GetPName`]
pub const GL_FRONT_FACE: GLenum = GLenum(0x0B46);

///
/// * Group: [`CommandOpcodesNV`]
pub const GL_FRONT_FACE_COMMAND_NV: GLenum = GLenum(0x0012);

///
/// * Group: [`ColorBuffer`], [`DrawBufferMode`], [`ReadBufferMode`]
pub const GL_FRONT_LEFT: GLenum = GLenum(0x0400);

///
/// * Group: [`ColorBuffer`], [`DrawBufferMode`], [`ReadBufferMode`]
pub const GL_FRONT_RIGHT: GLenum = GLenum(0x0401);

///
/// * Group: [`ParameterRangeEXT`]
pub const GL_FULL_RANGE_EXT: GLenum = GLenum(0x87E1);

///
/// * Group: [`HintTarget`]
pub const GL_FULL_STIPPLE_HINT_PGI: GLenum = GLenum(0x1A219);

pub const GL_FULL_SUPPORT: GLenum = GLenum(0x82B7);

///
/// * Group: [`BlendEquationModeEXT`]
pub const GL_FUNC_ADD: GLenum = GLenum(0x8006);

///
/// * Group: [`BlendEquationModeEXT`]
pub const GL_FUNC_ADD_EXT: GLenum = GLenum(0x8006);

pub const GL_FUNC_ADD_OES: GLenum = GLenum(0x8006);

///
/// * Group: [`BlendEquationModeEXT`]
pub const GL_FUNC_REVERSE_SUBTRACT: GLenum = GLenum(0x800B);

///
/// * Group: [`BlendEquationModeEXT`]
pub const GL_FUNC_REVERSE_SUBTRACT_EXT: GLenum = GLenum(0x800B);

pub const GL_FUNC_REVERSE_SUBTRACT_OES: GLenum = GLenum(0x800B);

///
/// * Group: [`BlendEquationModeEXT`]
pub const GL_FUNC_SUBTRACT: GLenum = GLenum(0x800A);

///
/// * Group: [`BlendEquationModeEXT`]
pub const GL_FUNC_SUBTRACT_EXT: GLenum = GLenum(0x800A);

pub const GL_FUNC_SUBTRACT_OES: GLenum = GLenum(0x800A);

///
/// * Group: [`ShaderBinaryFormat`]
pub const GL_GCCSO_SHADER_BINARY_FJ: GLenum = GLenum(0x9260);

///
/// * Group: [`InternalFormatPName`], [`TextureParameterName`]
pub const GL_GENERATE_MIPMAP: GLenum = GLenum(0x8191);

///
/// * Group: [`HintTarget`]
pub const GL_GENERATE_MIPMAP_HINT: GLenum = GLenum(0x8192);

///
/// * Group: [`HintTarget`], [`GetPName`]
pub const GL_GENERATE_MIPMAP_HINT_SGIS: GLenum = GLenum(0x8192);

///
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_GENERATE_MIPMAP_SGIS: GLenum = GLenum(0x8191);

pub const GL_GENERIC_ATTRIB_NV: GLenum = GLenum(0x8C7D);

///
/// * Group: [`FfdMaskSGIX`]
pub const GL_GEOMETRY_DEFORMATION_BIT_SGIX: GLbitfield = GLbitfield(0x00000002);

///
/// * Group: [`MapTarget`], [`FfdTargetSGIX`]
pub const GL_GEOMETRY_DEFORMATION_SGIX: GLenum = GLenum(0x8194);

///
/// * Group: [`ProgramPropertyARB`]
pub const GL_GEOMETRY_INPUT_TYPE: GLenum = GLenum(0x8917);

pub const GL_GEOMETRY_INPUT_TYPE_ARB: GLenum = GLenum(0x8DDB);

pub const GL_GEOMETRY_INPUT_TYPE_EXT: GLenum = GLenum(0x8DDB);

pub const GL_GEOMETRY_LINKED_INPUT_TYPE_EXT: GLenum = GLenum(0x8917);

pub const GL_GEOMETRY_LINKED_INPUT_TYPE_OES: GLenum = GLenum(0x8917);

pub const GL_GEOMETRY_LINKED_OUTPUT_TYPE_EXT: GLenum = GLenum(0x8918);

pub const GL_GEOMETRY_LINKED_OUTPUT_TYPE_OES: GLenum = GLenum(0x8918);

pub const GL_GEOMETRY_LINKED_VERTICES_OUT_EXT: GLenum = GLenum(0x8916);

pub const GL_GEOMETRY_LINKED_VERTICES_OUT_OES: GLenum = GLenum(0x8916);

///
/// * Group: [`ProgramPropertyARB`]
pub const GL_GEOMETRY_OUTPUT_TYPE: GLenum = GLenum(0x8918);

pub const GL_GEOMETRY_OUTPUT_TYPE_ARB: GLenum = GLenum(0x8DDC);

pub const GL_GEOMETRY_OUTPUT_TYPE_EXT: GLenum = GLenum(0x8DDC);

///
/// * Group: [`ProgramTarget`]
pub const GL_GEOMETRY_PROGRAM_NV: GLenum = GLenum(0x8C26);

pub const GL_GEOMETRY_PROGRAM_PARAMETER_BUFFER_NV: GLenum = GLenum(0x8DA3);

///
/// * Group: [`PipelineParameterName`], [`ShaderType`]
pub const GL_GEOMETRY_SHADER: GLenum = GLenum(0x8DD9);

pub const GL_GEOMETRY_SHADER_ARB: GLenum = GLenum(0x8DD9);

///
/// * Group: [`UseProgramStageMask`]
pub const GL_GEOMETRY_SHADER_BIT: GLbitfield = GLbitfield(0x00000004);

///
/// * Group: [`UseProgramStageMask`]
pub const GL_GEOMETRY_SHADER_BIT_EXT: GLbitfield = GLbitfield(0x00000004);

///
/// * Group: [`UseProgramStageMask`]
pub const GL_GEOMETRY_SHADER_BIT_OES: GLbitfield = GLbitfield(0x00000004);

pub const GL_GEOMETRY_SHADER_EXT: GLenum = GLenum(0x8DD9);

pub const GL_GEOMETRY_SHADER_INVOCATIONS: GLenum = GLenum(0x887F);

pub const GL_GEOMETRY_SHADER_INVOCATIONS_EXT: GLenum = GLenum(0x887F);

pub const GL_GEOMETRY_SHADER_INVOCATIONS_OES: GLenum = GLenum(0x887F);

pub const GL_GEOMETRY_SHADER_OES: GLenum = GLenum(0x8DD9);

pub const GL_GEOMETRY_SHADER_PRIMITIVES_EMITTED: GLenum = GLenum(0x82F3);

///
/// * Alias Of: [`GL_GEOMETRY_SHADER_PRIMITIVES_EMITTED`]
pub const GL_GEOMETRY_SHADER_PRIMITIVES_EMITTED_ARB: GLenum = GL_GEOMETRY_SHADER_PRIMITIVES_EMITTED;

///
/// * Group: [`ProgramInterface`]
pub const GL_GEOMETRY_SUBROUTINE: GLenum = GLenum(0x92EB);

///
/// * Group: [`ProgramInterface`]
pub const GL_GEOMETRY_SUBROUTINE_UNIFORM: GLenum = GLenum(0x92F1);

///
/// * Group: [`InternalFormatPName`]
pub const GL_GEOMETRY_TEXTURE: GLenum = GLenum(0x829E);

///
/// * Group: [`ProgramPropertyARB`]
pub const GL_GEOMETRY_VERTICES_OUT: GLenum = GLenum(0x8916);

pub const GL_GEOMETRY_VERTICES_OUT_ARB: GLenum = GLenum(0x8DDA);

pub const GL_GEOMETRY_VERTICES_OUT_EXT: GLenum = GLenum(0x8DDA);

///
/// * Group: [`StencilFunction`], [`IndexFunctionEXT`], [`AlphaFunction`],
///   [`DepthFunction`]
pub const GL_GEQUAL: GLenum = GLenum(0x0206);

///
/// * Group: [`InternalFormatPName`]
pub const GL_GET_TEXTURE_IMAGE_FORMAT: GLenum = GLenum(0x8291);

///
/// * Group: [`InternalFormatPName`]
pub const GL_GET_TEXTURE_IMAGE_TYPE: GLenum = GLenum(0x8292);

pub const GL_GLOBAL_ALPHA_FACTOR_SUN: GLenum = GLenum(0x81DA);

pub const GL_GLOBAL_ALPHA_SUN: GLenum = GLenum(0x81D9);

///
/// * Group: [`PathMetricMask`]
pub const GL_GLYPH_HAS_KERNING_BIT_NV: GLbitfield = GLbitfield(0x100);

///
/// * Group: [`PathMetricMask`]
pub const GL_GLYPH_HEIGHT_BIT_NV: GLbitfield = GLbitfield(0x02);

///
/// * Group: [`PathMetricMask`]
pub const GL_GLYPH_HORIZONTAL_BEARING_ADVANCE_BIT_NV: GLbitfield = GLbitfield(0x10);

///
/// * Group: [`PathMetricMask`]
pub const GL_GLYPH_HORIZONTAL_BEARING_X_BIT_NV: GLbitfield = GLbitfield(0x04);

///
/// * Group: [`PathMetricMask`]
pub const GL_GLYPH_HORIZONTAL_BEARING_Y_BIT_NV: GLbitfield = GLbitfield(0x08);

///
/// * Group: [`PathMetricMask`]
pub const GL_GLYPH_VERTICAL_BEARING_ADVANCE_BIT_NV: GLbitfield = GLbitfield(0x80);

///
/// * Group: [`PathMetricMask`]
pub const GL_GLYPH_VERTICAL_BEARING_X_BIT_NV: GLbitfield = GLbitfield(0x20);

///
/// * Group: [`PathMetricMask`]
pub const GL_GLYPH_VERTICAL_BEARING_Y_BIT_NV: GLbitfield = GLbitfield(0x40);

///
/// * Group: [`PathMetricMask`]
pub const GL_GLYPH_WIDTH_BIT_NV: GLbitfield = GLbitfield(0x01);

pub const GL_GPU_ADDRESS_NV: GLenum = GLenum(0x8F34);

pub const GL_GPU_DISJOINT_EXT: GLenum = GLenum(0x8FBB);

pub const GL_GPU_MEMORY_INFO_CURRENT_AVAILABLE_VIDMEM_NVX: GLenum = GLenum(0x9049);

pub const GL_GPU_MEMORY_INFO_DEDICATED_VIDMEM_NVX: GLenum = GLenum(0x9047);

pub const GL_GPU_MEMORY_INFO_EVICTED_MEMORY_NVX: GLenum = GLenum(0x904B);

pub const GL_GPU_MEMORY_INFO_EVICTION_COUNT_NVX: GLenum = GLenum(0x904A);

pub const GL_GPU_MEMORY_INFO_TOTAL_AVAILABLE_MEMORY_NVX: GLenum = GLenum(0x9048);

pub const GL_GPU_OPTIMIZED_QCOM: GLenum = GLenum(0x8FB2);

///
/// * Group: [`StencilFunction`], [`IndexFunctionEXT`], [`AlphaFunction`],
///   [`DepthFunction`]
pub const GL_GREATER: GLenum = GLenum(0x0204);

///
/// * Group: [`TextureSwizzle`], [`PixelFormat`]
pub const GL_GREEN: GLenum = GLenum(0x1904);

///
/// * Group: [`PixelTransferParameter`], [`GetPName`]
pub const GL_GREEN_BIAS: GLenum = GLenum(0x0D19);

///
/// * Group: [`GetPName`]
pub const GL_GREEN_BITS: GLenum = GLenum(0x0D53);

///
/// * Group: [`FragmentShaderDestMaskATI`]
pub const GL_GREEN_BIT_ATI: GLbitfield = GLbitfield(0x00000002);

///
/// * Group: [`PixelFormat`]
pub const GL_GREEN_INTEGER: GLenum = GLenum(0x8D95);

pub const GL_GREEN_INTEGER_EXT: GLenum = GLenum(0x8D95);

pub const GL_GREEN_MAX_CLAMP_INGR: GLenum = GLenum(0x8565);

pub const GL_GREEN_MIN_CLAMP_INGR: GLenum = GLenum(0x8561);

pub const GL_GREEN_NV: GLenum = GLenum(0x1904);

///
/// * Group: [`PixelTransferParameter`], [`GetPName`]
pub const GL_GREEN_SCALE: GLenum = GLenum(0x0D18);

pub const GL_GS_PROGRAM_BINARY_MTK: GLenum = GLenum(0x9641);

pub const GL_GS_SHADER_BINARY_MTK: GLenum = GLenum(0x9640);

///
/// * Group: [`GraphicsResetStatus`]
pub const GL_GUILTY_CONTEXT_RESET: GLenum = GLenum(0x8253);

pub const GL_GUILTY_CONTEXT_RESET_ARB: GLenum = GLenum(0x8253);

pub const GL_GUILTY_CONTEXT_RESET_EXT: GLenum = GLenum(0x8253);

pub const GL_GUILTY_CONTEXT_RESET_KHR: GLenum = GLenum(0x8253);

pub const GL_HALF_APPLE: GLenum = GLenum(0x140B);

///
/// * Group: [`CombinerMappingNV`]
pub const GL_HALF_BIAS_NEGATE_NV: GLenum = GLenum(0x853B);

///
/// * Group: [`CombinerMappingNV`]
pub const GL_HALF_BIAS_NORMAL_NV: GLenum = GLenum(0x853A);

///
/// * Group: [`FragmentShaderDestModMaskATI`]
pub const GL_HALF_BIT_ATI: GLbitfield = GLbitfield(0x00000008);

///
/// * Group: [`VertexAttribPointerType`], [`VertexAttribType`]
pub const GL_HALF_FLOAT: GLenum = GLenum(0x140B);

pub const GL_HALF_FLOAT_ARB: GLenum = GLenum(0x140B);

pub const GL_HALF_FLOAT_NV: GLenum = GLenum(0x140B);

pub const GL_HALF_FLOAT_OES: GLenum = GLenum(0x8D61);

///
/// * Group: [`ExternalHandleType`]
pub const GL_HANDLE_TYPE_D3D11_IMAGE_EXT: GLenum = GLenum(0x958B);

///
/// * Group: [`ExternalHandleType`]
pub const GL_HANDLE_TYPE_D3D11_IMAGE_KMT_EXT: GLenum = GLenum(0x958C);

///
/// * Group: [`ExternalHandleType`]
pub const GL_HANDLE_TYPE_D3D12_FENCE_EXT: GLenum = GLenum(0x9594);

///
/// * Group: [`ExternalHandleType`]
pub const GL_HANDLE_TYPE_D3D12_RESOURCE_EXT: GLenum = GLenum(0x958A);

///
/// * Group: [`ExternalHandleType`]
pub const GL_HANDLE_TYPE_D3D12_TILEPOOL_EXT: GLenum = GLenum(0x9589);

///
/// * Group: [`ExternalHandleType`]
pub const GL_HANDLE_TYPE_OPAQUE_FD_EXT: GLenum = GLenum(0x9586);

///
/// * Group: [`ExternalHandleType`]
pub const GL_HANDLE_TYPE_OPAQUE_WIN32_EXT: GLenum = GLenum(0x9587);

///
/// * Group: [`ExternalHandleType`]
pub const GL_HANDLE_TYPE_OPAQUE_WIN32_KMT_EXT: GLenum = GLenum(0x9588);

pub const GL_HARDLIGHT: GLenum = GLenum(0x929B);

pub const GL_HARDLIGHT_KHR: GLenum = GLenum(0x929B);

pub const GL_HARDLIGHT_NV: GLenum = GLenum(0x929B);

pub const GL_HARDMIX_NV: GLenum = GLenum(0x92A9);

///
/// * Group: [`PrecisionType`]
pub const GL_HIGH_FLOAT: GLenum = GLenum(0x8DF2);

///
/// * Group: [`PrecisionType`]
pub const GL_HIGH_INT: GLenum = GLenum(0x8DF5);

pub const GL_HILO16_NV: GLenum = GLenum(0x86F8);

pub const GL_HILO8_NV: GLenum = GLenum(0x885E);

pub const GL_HILO_NV: GLenum = GLenum(0x86F4);

///
/// * Group: [`AttribMask`]
pub const GL_HINT_BIT: GLbitfield = GLbitfield(0x00008000);

///
/// * Group: [`HistogramTarget`], [`HistogramTargetEXT`]
pub const GL_HISTOGRAM: GLenum = GLenum(0x8024);

///
/// * Group: [`GetHistogramParameterPNameEXT`]
pub const GL_HISTOGRAM_ALPHA_SIZE: GLenum = GLenum(0x802B);

///
/// * Group: [`GetHistogramParameterPNameEXT`]
pub const GL_HISTOGRAM_ALPHA_SIZE_EXT: GLenum = GLenum(0x802B);

///
/// * Group: [`GetHistogramParameterPNameEXT`]
pub const GL_HISTOGRAM_BLUE_SIZE: GLenum = GLenum(0x802A);

///
/// * Group: [`GetHistogramParameterPNameEXT`]
pub const GL_HISTOGRAM_BLUE_SIZE_EXT: GLenum = GLenum(0x802A);

///
/// * Group: [`HistogramTargetEXT`], [`EnableCap`], [`GetPName`]
pub const GL_HISTOGRAM_EXT: GLenum = GLenum(0x8024);

///
/// * Group: [`GetHistogramParameterPNameEXT`]
pub const GL_HISTOGRAM_FORMAT: GLenum = GLenum(0x8027);

///
/// * Group: [`GetHistogramParameterPNameEXT`]
pub const GL_HISTOGRAM_FORMAT_EXT: GLenum = GLenum(0x8027);

///
/// * Group: [`GetHistogramParameterPNameEXT`]
pub const GL_HISTOGRAM_GREEN_SIZE: GLenum = GLenum(0x8029);

///
/// * Group: [`GetHistogramParameterPNameEXT`]
pub const GL_HISTOGRAM_GREEN_SIZE_EXT: GLenum = GLenum(0x8029);

///
/// * Group: [`GetHistogramParameterPNameEXT`]
pub const GL_HISTOGRAM_LUMINANCE_SIZE: GLenum = GLenum(0x802C);

///
/// * Group: [`GetHistogramParameterPNameEXT`]
pub const GL_HISTOGRAM_LUMINANCE_SIZE_EXT: GLenum = GLenum(0x802C);

///
/// * Group: [`GetHistogramParameterPNameEXT`]
pub const GL_HISTOGRAM_RED_SIZE: GLenum = GLenum(0x8028);

///
/// * Group: [`GetHistogramParameterPNameEXT`]
pub const GL_HISTOGRAM_RED_SIZE_EXT: GLenum = GLenum(0x8028);

///
/// * Group: [`GetHistogramParameterPNameEXT`]
pub const GL_HISTOGRAM_SINK: GLenum = GLenum(0x802D);

///
/// * Group: [`GetHistogramParameterPNameEXT`]
pub const GL_HISTOGRAM_SINK_EXT: GLenum = GLenum(0x802D);

///
/// * Group: [`GetHistogramParameterPNameEXT`]
pub const GL_HISTOGRAM_WIDTH: GLenum = GLenum(0x8026);

///
/// * Group: [`GetHistogramParameterPNameEXT`]
pub const GL_HISTOGRAM_WIDTH_EXT: GLenum = GLenum(0x8026);

pub const GL_HI_BIAS_NV: GLenum = GLenum(0x8714);

pub const GL_HI_SCALE_NV: GLenum = GLenum(0x870E);

///
/// * Group: [`PathCoordType`]
pub const GL_HORIZONTAL_LINE_TO_NV: GLenum = GLenum(0x06);

pub const GL_HSL_COLOR: GLenum = GLenum(0x92AF);

pub const GL_HSL_COLOR_KHR: GLenum = GLenum(0x92AF);

pub const GL_HSL_COLOR_NV: GLenum = GLenum(0x92AF);

pub const GL_HSL_HUE: GLenum = GLenum(0x92AD);

pub const GL_HSL_HUE_KHR: GLenum = GLenum(0x92AD);

pub const GL_HSL_HUE_NV: GLenum = GLenum(0x92AD);

pub const GL_HSL_LUMINOSITY: GLenum = GLenum(0x92B0);

pub const GL_HSL_LUMINOSITY_KHR: GLenum = GLenum(0x92B0);

pub const GL_HSL_LUMINOSITY_NV: GLenum = GLenum(0x92B0);

pub const GL_HSL_SATURATION: GLenum = GLenum(0x92AE);

pub const GL_HSL_SATURATION_KHR: GLenum = GLenum(0x92AE);

pub const GL_HSL_SATURATION_NV: GLenum = GLenum(0x92AE);

pub const GL_IDENTITY_NV: GLenum = GLenum(0x862A);

pub const GL_IGNORE_BORDER_HP: GLenum = GLenum(0x8150);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`]
pub const GL_IMAGE_1D: GLenum = GLenum(0x904C);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`]
pub const GL_IMAGE_1D_ARRAY: GLenum = GLenum(0x9052);

pub const GL_IMAGE_1D_ARRAY_EXT: GLenum = GLenum(0x9052);

pub const GL_IMAGE_1D_EXT: GLenum = GLenum(0x904C);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`]
pub const GL_IMAGE_2D: GLenum = GLenum(0x904D);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`]
pub const GL_IMAGE_2D_ARRAY: GLenum = GLenum(0x9053);

pub const GL_IMAGE_2D_ARRAY_EXT: GLenum = GLenum(0x9053);

pub const GL_IMAGE_2D_EXT: GLenum = GLenum(0x904D);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`]
pub const GL_IMAGE_2D_MULTISAMPLE: GLenum = GLenum(0x9055);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`]
pub const GL_IMAGE_2D_MULTISAMPLE_ARRAY: GLenum = GLenum(0x9056);

pub const GL_IMAGE_2D_MULTISAMPLE_ARRAY_EXT: GLenum = GLenum(0x9056);

pub const GL_IMAGE_2D_MULTISAMPLE_EXT: GLenum = GLenum(0x9055);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`]
pub const GL_IMAGE_2D_RECT: GLenum = GLenum(0x904F);

pub const GL_IMAGE_2D_RECT_EXT: GLenum = GLenum(0x904F);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`]
pub const GL_IMAGE_3D: GLenum = GLenum(0x904E);

pub const GL_IMAGE_3D_EXT: GLenum = GLenum(0x904E);

pub const GL_IMAGE_BINDING_ACCESS: GLenum = GLenum(0x8F3E);

pub const GL_IMAGE_BINDING_ACCESS_EXT: GLenum = GLenum(0x8F3E);

pub const GL_IMAGE_BINDING_FORMAT: GLenum = GLenum(0x906E);

pub const GL_IMAGE_BINDING_FORMAT_EXT: GLenum = GLenum(0x906E);

pub const GL_IMAGE_BINDING_LAYER: GLenum = GLenum(0x8F3D);

pub const GL_IMAGE_BINDING_LAYERED: GLenum = GLenum(0x8F3C);

pub const GL_IMAGE_BINDING_LAYERED_EXT: GLenum = GLenum(0x8F3C);

pub const GL_IMAGE_BINDING_LAYER_EXT: GLenum = GLenum(0x8F3D);

pub const GL_IMAGE_BINDING_LEVEL: GLenum = GLenum(0x8F3B);

pub const GL_IMAGE_BINDING_LEVEL_EXT: GLenum = GLenum(0x8F3B);

pub const GL_IMAGE_BINDING_NAME: GLenum = GLenum(0x8F3A);

pub const GL_IMAGE_BINDING_NAME_EXT: GLenum = GLenum(0x8F3A);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`]
pub const GL_IMAGE_BUFFER: GLenum = GLenum(0x9051);

pub const GL_IMAGE_BUFFER_EXT: GLenum = GLenum(0x9051);

pub const GL_IMAGE_BUFFER_OES: GLenum = GLenum(0x9051);

pub const GL_IMAGE_CLASS_10_10_10_2: GLenum = GLenum(0x82C3);

pub const GL_IMAGE_CLASS_11_11_10: GLenum = GLenum(0x82C2);

pub const GL_IMAGE_CLASS_1_X_16: GLenum = GLenum(0x82BE);

pub const GL_IMAGE_CLASS_1_X_32: GLenum = GLenum(0x82BB);

pub const GL_IMAGE_CLASS_1_X_8: GLenum = GLenum(0x82C1);

pub const GL_IMAGE_CLASS_2_X_16: GLenum = GLenum(0x82BD);

pub const GL_IMAGE_CLASS_2_X_32: GLenum = GLenum(0x82BA);

pub const GL_IMAGE_CLASS_2_X_8: GLenum = GLenum(0x82C0);

pub const GL_IMAGE_CLASS_4_X_16: GLenum = GLenum(0x82BC);

pub const GL_IMAGE_CLASS_4_X_32: GLenum = GLenum(0x82B9);

pub const GL_IMAGE_CLASS_4_X_8: GLenum = GLenum(0x82BF);

///
/// * Group: [`InternalFormatPName`]
pub const GL_IMAGE_COMPATIBILITY_CLASS: GLenum = GLenum(0x82A8);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`]
pub const GL_IMAGE_CUBE: GLenum = GLenum(0x9050);

pub const GL_IMAGE_CUBE_EXT: GLenum = GLenum(0x9050);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`]
pub const GL_IMAGE_CUBE_MAP_ARRAY: GLenum = GLenum(0x9054);

pub const GL_IMAGE_CUBE_MAP_ARRAY_EXT: GLenum = GLenum(0x9054);

pub const GL_IMAGE_CUBE_MAP_ARRAY_OES: GLenum = GLenum(0x9054);

///
/// * Group: [`ImageTransformPNameHP`]
pub const GL_IMAGE_CUBIC_WEIGHT_HP: GLenum = GLenum(0x815E);

pub const GL_IMAGE_FORMAT_COMPATIBILITY_BY_CLASS: GLenum = GLenum(0x90C9);

pub const GL_IMAGE_FORMAT_COMPATIBILITY_BY_SIZE: GLenum = GLenum(0x90C8);

///
/// * Group: [`InternalFormatPName`]
pub const GL_IMAGE_FORMAT_COMPATIBILITY_TYPE: GLenum = GLenum(0x90C7);

///
/// * Group: [`ImageTransformPNameHP`]
pub const GL_IMAGE_MAG_FILTER_HP: GLenum = GLenum(0x815C);

///
/// * Group: [`ImageTransformPNameHP`]
pub const GL_IMAGE_MIN_FILTER_HP: GLenum = GLenum(0x815D);

///
/// * Group: [`InternalFormatPName`]
pub const GL_IMAGE_PIXEL_FORMAT: GLenum = GLenum(0x82A9);

///
/// * Group: [`InternalFormatPName`]
pub const GL_IMAGE_PIXEL_TYPE: GLenum = GLenum(0x82AA);

///
/// * Group: [`ImageTransformPNameHP`]
pub const GL_IMAGE_ROTATE_ANGLE_HP: GLenum = GLenum(0x8159);

///
/// * Group: [`ImageTransformPNameHP`]
pub const GL_IMAGE_ROTATE_ORIGIN_X_HP: GLenum = GLenum(0x815A);

///
/// * Group: [`ImageTransformPNameHP`]
pub const GL_IMAGE_ROTATE_ORIGIN_Y_HP: GLenum = GLenum(0x815B);

///
/// * Group: [`ImageTransformPNameHP`]
pub const GL_IMAGE_SCALE_X_HP: GLenum = GLenum(0x8155);

///
/// * Group: [`ImageTransformPNameHP`]
pub const GL_IMAGE_SCALE_Y_HP: GLenum = GLenum(0x8156);

///
/// * Group: [`InternalFormatPName`]
pub const GL_IMAGE_TEXEL_SIZE: GLenum = GLenum(0x82A7);

///
/// * Group: [`ImageTransformTargetHP`]
pub const GL_IMAGE_TRANSFORM_2D_HP: GLenum = GLenum(0x8161);

///
/// * Group: [`ImageTransformPNameHP`]
pub const GL_IMAGE_TRANSLATE_X_HP: GLenum = GLenum(0x8157);

///
/// * Group: [`ImageTransformPNameHP`]
pub const GL_IMAGE_TRANSLATE_Y_HP: GLenum = GLenum(0x8158);

///
/// * Group: [`GetFramebufferParameter`], [`GetPName`]
pub const GL_IMPLEMENTATION_COLOR_READ_FORMAT: GLenum = GLenum(0x8B9B);

pub const GL_IMPLEMENTATION_COLOR_READ_FORMAT_OES: GLenum = GLenum(0x8B9B);

///
/// * Group: [`GetFramebufferParameter`], [`GetPName`]
pub const GL_IMPLEMENTATION_COLOR_READ_TYPE: GLenum = GLenum(0x8B9A);

pub const GL_IMPLEMENTATION_COLOR_READ_TYPE_OES: GLenum = GLenum(0x8B9A);

pub const GL_INCLUSIVE_EXT: GLenum = GLenum(0x8F10);

///
/// * Group: [`StencilOp`]
pub const GL_INCR: GLenum = GLenum(0x1E02);

///
/// * Group: [`StencilOp`]
pub const GL_INCR_WRAP: GLenum = GLenum(0x8507);

pub const GL_INCR_WRAP_EXT: GLenum = GLenum(0x8507);

pub const GL_INCR_WRAP_OES: GLenum = GLenum(0x8507);

pub const GL_INDEX: GLenum = GLenum(0x8222);

///
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_INDEX_ARRAY: GLenum = GLenum(0x8077);

pub const GL_INDEX_ARRAY_ADDRESS_NV: GLenum = GLenum(0x8F24);

pub const GL_INDEX_ARRAY_BUFFER_BINDING: GLenum = GLenum(0x8899);

pub const GL_INDEX_ARRAY_BUFFER_BINDING_ARB: GLenum = GLenum(0x8899);

///
/// * Group: [`GetPName`]
pub const GL_INDEX_ARRAY_COUNT_EXT: GLenum = GLenum(0x8087);

pub const GL_INDEX_ARRAY_EXT: GLenum = GLenum(0x8077);

pub const GL_INDEX_ARRAY_LENGTH_NV: GLenum = GLenum(0x8F2E);

pub const GL_INDEX_ARRAY_LIST_IBM: GLenum = GLenum(103073);

pub const GL_INDEX_ARRAY_LIST_STRIDE_IBM: GLenum = GLenum(103083);

///
/// * Group: [`GetPointervPName`]
pub const GL_INDEX_ARRAY_POINTER: GLenum = GLenum(0x8091);

///
/// * Group: [`GetPointervPName`]
pub const GL_INDEX_ARRAY_POINTER_EXT: GLenum = GLenum(0x8091);

///
/// * Group: [`GetPName`]
pub const GL_INDEX_ARRAY_STRIDE: GLenum = GLenum(0x8086);

pub const GL_INDEX_ARRAY_STRIDE_EXT: GLenum = GLenum(0x8086);

///
/// * Group: [`GetPName`]
pub const GL_INDEX_ARRAY_TYPE: GLenum = GLenum(0x8085);

pub const GL_INDEX_ARRAY_TYPE_EXT: GLenum = GLenum(0x8085);

///
/// * Group: [`GetPName`]
pub const GL_INDEX_BITS: GLenum = GLenum(0x0D51);

///
/// * Group: [`VertexHintsMaskPGI`]
pub const GL_INDEX_BIT_PGI: GLbitfield = GLbitfield(0x00080000);

///
/// * Group: [`GetPName`]
pub const GL_INDEX_CLEAR_VALUE: GLenum = GLenum(0x0C20);

///
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_INDEX_LOGIC_OP: GLenum = GLenum(0x0BF1);

pub const GL_INDEX_MATERIAL_EXT: GLenum = GLenum(0x81B8);

pub const GL_INDEX_MATERIAL_FACE_EXT: GLenum = GLenum(0x81BA);

pub const GL_INDEX_MATERIAL_PARAMETER_EXT: GLenum = GLenum(0x81B9);

///
/// * Group: [`GetPName`]
pub const GL_INDEX_MODE: GLenum = GLenum(0x0C30);

///
/// * Group: [`PixelTransferParameter`], [`IndexMaterialParameterEXT`],
///   [`GetPName`]
pub const GL_INDEX_OFFSET: GLenum = GLenum(0x0D13);

///
/// * Group: [`PixelTransferParameter`], [`GetPName`]
pub const GL_INDEX_SHIFT: GLenum = GLenum(0x0D12);

pub const GL_INDEX_TEST_EXT: GLenum = GLenum(0x81B5);

pub const GL_INDEX_TEST_FUNC_EXT: GLenum = GLenum(0x81B6);

pub const GL_INDEX_TEST_REF_EXT: GLenum = GLenum(0x81B7);

///
/// * Group: [`GetPName`]
pub const GL_INDEX_WRITEMASK: GLenum = GLenum(0x0C21);

///
/// * Group: [`ProgramPropertyARB`], [`ShaderParameterName`],
///   [`PipelineParameterName`]
pub const GL_INFO_LOG_LENGTH: GLenum = GLenum(0x8B84);

///
/// * Group: [`GraphicsResetStatus`]
pub const GL_INNOCENT_CONTEXT_RESET: GLenum = GLenum(0x8254);

pub const GL_INNOCENT_CONTEXT_RESET_ARB: GLenum = GLenum(0x8254);

pub const GL_INNOCENT_CONTEXT_RESET_EXT: GLenum = GLenum(0x8254);

pub const GL_INNOCENT_CONTEXT_RESET_KHR: GLenum = GLenum(0x8254);

///
/// * Group: [`GetPointervPName`]
pub const GL_INSTRUMENT_BUFFER_POINTER_SGIX: GLenum = GLenum(0x8180);

///
/// * Group: [`GetPName`]
pub const GL_INSTRUMENT_MEASUREMENTS_SGIX: GLenum = GLenum(0x8181);

///
/// * Group: [`VertexAttribIType`], [`SecondaryColorPointerTypeIBM`],
///   [`WeightPointerTypeARB`], [`TangentPointerTypeEXT`],
///   [`BinormalPointerTypeEXT`], [`IndexPointerType`], [`ListNameType`],
///   [`NormalPointerType`], [`PixelType`], [`TexCoordPointerType`],
///   [`VertexPointerType`], [`VertexAttribType`], [`AttributeType`],
///   [`UniformType`], [`VertexAttribPointerType`], [`GlslTypeToken`]
pub const GL_INT: GLenum = GLenum(0x1404);

pub const GL_INT16_NV: GLenum = GLenum(0x8FE4);

pub const GL_INT16_VEC2_NV: GLenum = GLenum(0x8FE5);

pub const GL_INT16_VEC3_NV: GLenum = GLenum(0x8FE6);

pub const GL_INT16_VEC4_NV: GLenum = GLenum(0x8FE7);

///
/// * Group: [`VertexAttribPointerType`], [`AttributeType`]
pub const GL_INT64_ARB: GLenum = GLenum(0x140E);

///
/// * Group: [`VertexAttribPointerType`], [`AttributeType`]
pub const GL_INT64_NV: GLenum = GLenum(0x140E);

///
/// * Group: [`AttributeType`]
pub const GL_INT64_VEC2_ARB: GLenum = GLenum(0x8FE9);

pub const GL_INT64_VEC2_NV: GLenum = GLenum(0x8FE9);

///
/// * Group: [`AttributeType`]
pub const GL_INT64_VEC3_ARB: GLenum = GLenum(0x8FEA);

pub const GL_INT64_VEC3_NV: GLenum = GLenum(0x8FEA);

///
/// * Group: [`AttributeType`]
pub const GL_INT64_VEC4_ARB: GLenum = GLenum(0x8FEB);

pub const GL_INT64_VEC4_NV: GLenum = GLenum(0x8FEB);

pub const GL_INT8_NV: GLenum = GLenum(0x8FE0);

pub const GL_INT8_VEC2_NV: GLenum = GLenum(0x8FE1);

pub const GL_INT8_VEC3_NV: GLenum = GLenum(0x8FE2);

pub const GL_INT8_VEC4_NV: GLenum = GLenum(0x8FE3);

///
/// * Group: [`InternalFormat`], [`PathColorFormat`]
pub const GL_INTENSITY: GLenum = GLenum(0x8049);

///
/// * Group: [`InternalFormat`]
pub const GL_INTENSITY12: GLenum = GLenum(0x804C);

pub const GL_INTENSITY12_EXT: GLenum = GLenum(0x804C);

///
/// * Group: [`InternalFormat`]
pub const GL_INTENSITY16: GLenum = GLenum(0x804D);

pub const GL_INTENSITY16F_ARB: GLenum = GLenum(0x881D);

pub const GL_INTENSITY16I_EXT: GLenum = GLenum(0x8D8B);

pub const GL_INTENSITY16UI_EXT: GLenum = GLenum(0x8D79);

pub const GL_INTENSITY16_EXT: GLenum = GLenum(0x804D);

pub const GL_INTENSITY16_SNORM: GLenum = GLenum(0x901B);

pub const GL_INTENSITY32F_ARB: GLenum = GLenum(0x8817);

pub const GL_INTENSITY32I_EXT: GLenum = GLenum(0x8D85);

pub const GL_INTENSITY32UI_EXT: GLenum = GLenum(0x8D73);

///
/// * Group: [`InternalFormat`]
pub const GL_INTENSITY4: GLenum = GLenum(0x804A);

pub const GL_INTENSITY4_EXT: GLenum = GLenum(0x804A);

///
/// * Group: [`InternalFormat`]
pub const GL_INTENSITY8: GLenum = GLenum(0x804B);

pub const GL_INTENSITY8I_EXT: GLenum = GLenum(0x8D91);

pub const GL_INTENSITY8UI_EXT: GLenum = GLenum(0x8D7F);

pub const GL_INTENSITY8_EXT: GLenum = GLenum(0x804B);

pub const GL_INTENSITY8_SNORM: GLenum = GLenum(0x9017);

pub const GL_INTENSITY_EXT: GLenum = GLenum(0x8049);

pub const GL_INTENSITY_FLOAT16_APPLE: GLenum = GLenum(0x881D);

pub const GL_INTENSITY_FLOAT16_ATI: GLenum = GLenum(0x881D);

pub const GL_INTENSITY_FLOAT32_APPLE: GLenum = GLenum(0x8817);

pub const GL_INTENSITY_FLOAT32_ATI: GLenum = GLenum(0x8817);

pub const GL_INTENSITY_SNORM: GLenum = GLenum(0x9013);

pub const GL_INTERLACE_OML: GLenum = GLenum(0x8980);

pub const GL_INTERLACE_READ_INGR: GLenum = GLenum(0x8568);

pub const GL_INTERLACE_READ_OML: GLenum = GLenum(0x8981);

///
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_INTERLACE_SGIX: GLenum = GLenum(0x8094);

///
/// * Group: [`TransformFeedbackBufferMode`]
pub const GL_INTERLEAVED_ATTRIBS: GLenum = GLenum(0x8C8C);

pub const GL_INTERLEAVED_ATTRIBS_EXT: GLenum = GLenum(0x8C8C);

pub const GL_INTERLEAVED_ATTRIBS_NV: GLenum = GLenum(0x8C8C);

///
/// * Group: [`InternalFormatPName`]
pub const GL_INTERNALFORMAT_ALPHA_SIZE: GLenum = GLenum(0x8274);

///
/// * Group: [`InternalFormatPName`]
pub const GL_INTERNALFORMAT_ALPHA_TYPE: GLenum = GLenum(0x827B);

///
/// * Group: [`InternalFormatPName`]
pub const GL_INTERNALFORMAT_BLUE_SIZE: GLenum = GLenum(0x8273);

///
/// * Group: [`InternalFormatPName`]
pub const GL_INTERNALFORMAT_BLUE_TYPE: GLenum = GLenum(0x827A);

///
/// * Group: [`InternalFormatPName`]
pub const GL_INTERNALFORMAT_DEPTH_SIZE: GLenum = GLenum(0x8275);

///
/// * Group: [`InternalFormatPName`]
pub const GL_INTERNALFORMAT_DEPTH_TYPE: GLenum = GLenum(0x827C);

///
/// * Group: [`InternalFormatPName`]
pub const GL_INTERNALFORMAT_GREEN_SIZE: GLenum = GLenum(0x8272);

///
/// * Group: [`InternalFormatPName`]
pub const GL_INTERNALFORMAT_GREEN_TYPE: GLenum = GLenum(0x8279);

///
/// * Group: [`InternalFormatPName`]
pub const GL_INTERNALFORMAT_PREFERRED: GLenum = GLenum(0x8270);

///
/// * Group: [`InternalFormatPName`]
pub const GL_INTERNALFORMAT_RED_SIZE: GLenum = GLenum(0x8271);

///
/// * Group: [`InternalFormatPName`]
pub const GL_INTERNALFORMAT_RED_TYPE: GLenum = GLenum(0x8278);

///
/// * Group: [`InternalFormatPName`]
pub const GL_INTERNALFORMAT_SHARED_SIZE: GLenum = GLenum(0x8277);

///
/// * Group: [`InternalFormatPName`]
pub const GL_INTERNALFORMAT_STENCIL_SIZE: GLenum = GLenum(0x8276);

///
/// * Group: [`InternalFormatPName`]
pub const GL_INTERNALFORMAT_STENCIL_TYPE: GLenum = GLenum(0x827D);

///
/// * Group: [`InternalFormatPName`]
pub const GL_INTERNALFORMAT_SUPPORTED: GLenum = GLenum(0x826F);

pub const GL_INTERPOLATE: GLenum = GLenum(0x8575);

pub const GL_INTERPOLATE_ARB: GLenum = GLenum(0x8575);

pub const GL_INTERPOLATE_EXT: GLenum = GLenum(0x8575);

pub const GL_INT_10_10_10_2_OES: GLenum = GLenum(0x8DF7);

///
/// * Group: [`VertexAttribPointerType`], [`VertexAttribType`]
pub const GL_INT_2_10_10_10_REV: GLenum = GLenum(0x8D9F);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`]
pub const GL_INT_IMAGE_1D: GLenum = GLenum(0x9057);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`]
pub const GL_INT_IMAGE_1D_ARRAY: GLenum = GLenum(0x905D);

pub const GL_INT_IMAGE_1D_ARRAY_EXT: GLenum = GLenum(0x905D);

pub const GL_INT_IMAGE_1D_EXT: GLenum = GLenum(0x9057);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`]
pub const GL_INT_IMAGE_2D: GLenum = GLenum(0x9058);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`]
pub const GL_INT_IMAGE_2D_ARRAY: GLenum = GLenum(0x905E);

pub const GL_INT_IMAGE_2D_ARRAY_EXT: GLenum = GLenum(0x905E);

pub const GL_INT_IMAGE_2D_EXT: GLenum = GLenum(0x9058);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`]
pub const GL_INT_IMAGE_2D_MULTISAMPLE: GLenum = GLenum(0x9060);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`]
pub const GL_INT_IMAGE_2D_MULTISAMPLE_ARRAY: GLenum = GLenum(0x9061);

pub const GL_INT_IMAGE_2D_MULTISAMPLE_ARRAY_EXT: GLenum = GLenum(0x9061);

pub const GL_INT_IMAGE_2D_MULTISAMPLE_EXT: GLenum = GLenum(0x9060);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`]
pub const GL_INT_IMAGE_2D_RECT: GLenum = GLenum(0x905A);

pub const GL_INT_IMAGE_2D_RECT_EXT: GLenum = GLenum(0x905A);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`]
pub const GL_INT_IMAGE_3D: GLenum = GLenum(0x9059);

pub const GL_INT_IMAGE_3D_EXT: GLenum = GLenum(0x9059);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`]
pub const GL_INT_IMAGE_BUFFER: GLenum = GLenum(0x905C);

pub const GL_INT_IMAGE_BUFFER_EXT: GLenum = GLenum(0x905C);

pub const GL_INT_IMAGE_BUFFER_OES: GLenum = GLenum(0x905C);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`]
pub const GL_INT_IMAGE_CUBE: GLenum = GLenum(0x905B);

pub const GL_INT_IMAGE_CUBE_EXT: GLenum = GLenum(0x905B);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`]
pub const GL_INT_IMAGE_CUBE_MAP_ARRAY: GLenum = GLenum(0x905F);

pub const GL_INT_IMAGE_CUBE_MAP_ARRAY_EXT: GLenum = GLenum(0x905F);

pub const GL_INT_IMAGE_CUBE_MAP_ARRAY_OES: GLenum = GLenum(0x905F);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_INT_SAMPLER_1D: GLenum = GLenum(0x8DC9);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_INT_SAMPLER_1D_ARRAY: GLenum = GLenum(0x8DCE);

pub const GL_INT_SAMPLER_1D_ARRAY_EXT: GLenum = GLenum(0x8DCE);

pub const GL_INT_SAMPLER_1D_EXT: GLenum = GLenum(0x8DC9);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_INT_SAMPLER_2D: GLenum = GLenum(0x8DCA);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_INT_SAMPLER_2D_ARRAY: GLenum = GLenum(0x8DCF);

pub const GL_INT_SAMPLER_2D_ARRAY_EXT: GLenum = GLenum(0x8DCF);

pub const GL_INT_SAMPLER_2D_EXT: GLenum = GLenum(0x8DCA);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_INT_SAMPLER_2D_MULTISAMPLE: GLenum = GLenum(0x9109);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_INT_SAMPLER_2D_MULTISAMPLE_ARRAY: GLenum = GLenum(0x910C);

pub const GL_INT_SAMPLER_2D_MULTISAMPLE_ARRAY_OES: GLenum = GLenum(0x910C);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_INT_SAMPLER_2D_RECT: GLenum = GLenum(0x8DCD);

pub const GL_INT_SAMPLER_2D_RECT_EXT: GLenum = GLenum(0x8DCD);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_INT_SAMPLER_3D: GLenum = GLenum(0x8DCB);

pub const GL_INT_SAMPLER_3D_EXT: GLenum = GLenum(0x8DCB);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_INT_SAMPLER_BUFFER: GLenum = GLenum(0x8DD0);

pub const GL_INT_SAMPLER_BUFFER_AMD: GLenum = GLenum(0x9002);

pub const GL_INT_SAMPLER_BUFFER_EXT: GLenum = GLenum(0x8DD0);

pub const GL_INT_SAMPLER_BUFFER_OES: GLenum = GLenum(0x8DD0);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_INT_SAMPLER_CUBE: GLenum = GLenum(0x8DCC);

pub const GL_INT_SAMPLER_CUBE_EXT: GLenum = GLenum(0x8DCC);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_INT_SAMPLER_CUBE_MAP_ARRAY: GLenum = GLenum(0x900E);

pub const GL_INT_SAMPLER_CUBE_MAP_ARRAY_ARB: GLenum = GLenum(0x900E);

pub const GL_INT_SAMPLER_CUBE_MAP_ARRAY_EXT: GLenum = GLenum(0x900E);

pub const GL_INT_SAMPLER_CUBE_MAP_ARRAY_OES: GLenum = GLenum(0x900E);

pub const GL_INT_SAMPLER_RENDERBUFFER_NV: GLenum = GLenum(0x8E57);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_INT_VEC2: GLenum = GLenum(0x8B53);

///
/// * Group: [`AttributeType`]
pub const GL_INT_VEC2_ARB: GLenum = GLenum(0x8B53);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_INT_VEC3: GLenum = GLenum(0x8B54);

///
/// * Group: [`AttributeType`]
pub const GL_INT_VEC3_ARB: GLenum = GLenum(0x8B54);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_INT_VEC4: GLenum = GLenum(0x8B55);

///
/// * Group: [`AttributeType`]
pub const GL_INT_VEC4_ARB: GLenum = GLenum(0x8B55);

///
/// * Group: [`ErrorCode`]
pub const GL_INVALID_ENUM: GLenum = GLenum(0x0500);

///
/// * Group: [`ErrorCode`]
pub const GL_INVALID_FRAMEBUFFER_OPERATION: GLenum = GLenum(0x0506);

///
/// * Group: [`ErrorCode`]
pub const GL_INVALID_FRAMEBUFFER_OPERATION_EXT: GLenum = GLenum(0x0506);

///
/// * Group: [`ErrorCode`]
pub const GL_INVALID_FRAMEBUFFER_OPERATION_OES: GLenum = GLenum(0x0506);

/// Tagged as uint
pub const GL_INVALID_INDEX: c_uint = 0xFFFFFFFF;

///
/// * Group: [`ErrorCode`]
pub const GL_INVALID_OPERATION: GLenum = GLenum(0x0502);

///
/// * Group: [`ErrorCode`]
pub const GL_INVALID_VALUE: GLenum = GLenum(0x0501);

pub const GL_INVARIANT_DATATYPE_EXT: GLenum = GLenum(0x87EB);

///
/// * Group: [`VertexShaderStorageTypeEXT`]
pub const GL_INVARIANT_EXT: GLenum = GLenum(0x87C2);

pub const GL_INVARIANT_VALUE_EXT: GLenum = GLenum(0x87EA);

pub const GL_INVERSE_NV: GLenum = GLenum(0x862B);

pub const GL_INVERSE_TRANSPOSE_NV: GLenum = GLenum(0x862D);

///
/// * Group: [`PathFillMode`], [`LogicOp`], [`StencilOp`]
pub const GL_INVERT: GLenum = GLenum(0x150A);

pub const GL_INVERTED_SCREEN_W_REND: GLenum = GLenum(0x8491);

pub const GL_INVERT_OVG_NV: GLenum = GLenum(0x92B4);

pub const GL_INVERT_RGB_NV: GLenum = GLenum(0x92A3);

///
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_IR_INSTRUMENT1_SGIX: GLenum = GLenum(0x817F);

pub const GL_ISOLINES: GLenum = GLenum(0x8E7A);

pub const GL_ISOLINES_EXT: GLenum = GLenum(0x8E7A);

pub const GL_ISOLINES_OES: GLenum = GLenum(0x8E7A);

///
/// * Group: [`ProgramResourceProperty`]
pub const GL_IS_PER_PATCH: GLenum = GLenum(0x92E7);

pub const GL_IS_PER_PATCH_EXT: GLenum = GLenum(0x92E7);

pub const GL_IS_PER_PATCH_OES: GLenum = GLenum(0x92E7);

///
/// * Group: [`ProgramResourceProperty`]
pub const GL_IS_ROW_MAJOR: GLenum = GLenum(0x9300);

///
/// * Group: [`PathFontStyle`]
pub const GL_ITALIC_BIT_NV: GLbitfield = GLbitfield(0x02);

pub const GL_IUI_N3F_V2F_EXT: GLenum = GLenum(0x81AF);

pub const GL_IUI_N3F_V3F_EXT: GLenum = GLenum(0x81B0);

pub const GL_IUI_V2F_EXT: GLenum = GLenum(0x81AD);

pub const GL_IUI_V3F_EXT: GLenum = GLenum(0x81AE);

///
/// * Group: [`StencilOp`]
pub const GL_KEEP: GLenum = GLenum(0x1E00);

///
/// * Group: [`PathCoordType`]
pub const GL_LARGE_CCW_ARC_TO_NV: GLenum = GLenum(0x16);

///
/// * Group: [`PathCoordType`]
pub const GL_LARGE_CW_ARC_TO_NV: GLenum = GLenum(0x18);

///
/// * Group: [`VertexProvokingMode`]
pub const GL_LAST_VERTEX_CONVENTION: GLenum = GLenum(0x8E4E);

pub const GL_LAST_VERTEX_CONVENTION_EXT: GLenum = GLenum(0x8E4E);

pub const GL_LAST_VERTEX_CONVENTION_OES: GLenum = GLenum(0x8E4E);

pub const GL_LAST_VIDEO_CAPTURE_STATUS_NV: GLenum = GLenum(0x9027);

pub const GL_LAYER_NV: GLenum = GLenum(0x8DAA);

///
/// * Group: [`GetPName`]
pub const GL_LAYER_PROVOKING_VERTEX: GLenum = GLenum(0x825E);

pub const GL_LAYER_PROVOKING_VERTEX_EXT: GLenum = GLenum(0x825E);

pub const GL_LAYER_PROVOKING_VERTEX_OES: GLenum = GLenum(0x825E);

///
/// * Group: [`TextureLayout`]
pub const GL_LAYOUT_COLOR_ATTACHMENT_EXT: GLenum = GLenum(0x958E);

///
/// * Group: [`MapTextureFormatINTEL`]
pub const GL_LAYOUT_DEFAULT_INTEL: GLenum = GLenum(0);

///
/// * Group: [`TextureLayout`]
pub const GL_LAYOUT_DEPTH_ATTACHMENT_STENCIL_READ_ONLY_EXT: GLenum = GLenum(0x9531);

///
/// * Group: [`TextureLayout`]
pub const GL_LAYOUT_DEPTH_READ_ONLY_STENCIL_ATTACHMENT_EXT: GLenum = GLenum(0x9530);

///
/// * Group: [`TextureLayout`]
pub const GL_LAYOUT_DEPTH_STENCIL_ATTACHMENT_EXT: GLenum = GLenum(0x958F);

///
/// * Group: [`TextureLayout`]
pub const GL_LAYOUT_DEPTH_STENCIL_READ_ONLY_EXT: GLenum = GLenum(0x9590);

///
/// * Group: [`TextureLayout`]
pub const GL_LAYOUT_GENERAL_EXT: GLenum = GLenum(0x958D);

///
/// * Group: [`MapTextureFormatINTEL`]
pub const GL_LAYOUT_LINEAR_CPU_CACHED_INTEL: GLenum = GLenum(2);

///
/// * Group: [`MapTextureFormatINTEL`]
pub const GL_LAYOUT_LINEAR_INTEL: GLenum = GLenum(1);

///
/// * Group: [`TextureLayout`]
pub const GL_LAYOUT_SHADER_READ_ONLY_EXT: GLenum = GLenum(0x9591);

///
/// * Group: [`TextureLayout`]
pub const GL_LAYOUT_TRANSFER_DST_EXT: GLenum = GLenum(0x9593);

///
/// * Group: [`TextureLayout`]
pub const GL_LAYOUT_TRANSFER_SRC_EXT: GLenum = GLenum(0x9592);

///
/// * Group: [`ColorBuffer`], [`DrawBufferMode`], [`ReadBufferMode`]
pub const GL_LEFT: GLenum = GLenum(0x0406);

///
/// * Group: [`StencilFunction`], [`IndexFunctionEXT`], [`AlphaFunction`],
///   [`DepthFunction`]
pub const GL_LEQUAL: GLenum = GLenum(0x0203);

///
/// * Group: [`FragmentOpATI`]
pub const GL_LERP_ATI: GLenum = GLenum(0x8969);

///
/// * Group: [`StencilFunction`], [`IndexFunctionEXT`], [`AlphaFunction`],
///   [`DepthFunction`]
pub const GL_LESS: GLenum = GLenum(0x0201);

///
/// * Group: [`BufferStorageMask`]
pub const GL_LGPU_SEPARATE_STORAGE_BIT_NVX: GLbitfield = GLbitfield(0x0800);

///
/// * Group: [`LightName`], [`EnableCap`], [`GetPName`]
pub const GL_LIGHT0: GLenum = GLenum(0x4000);

///
/// * Group: [`LightName`], [`EnableCap`], [`GetPName`]
pub const GL_LIGHT1: GLenum = GLenum(0x4001);

///
/// * Group: [`LightName`], [`EnableCap`], [`GetPName`]
pub const GL_LIGHT2: GLenum = GLenum(0x4002);

///
/// * Group: [`LightName`], [`EnableCap`], [`GetPName`]
pub const GL_LIGHT3: GLenum = GLenum(0x4003);

///
/// * Group: [`LightName`], [`EnableCap`], [`GetPName`]
pub const GL_LIGHT4: GLenum = GLenum(0x4004);

///
/// * Group: [`LightName`], [`EnableCap`], [`GetPName`]
pub const GL_LIGHT5: GLenum = GLenum(0x4005);

///
/// * Group: [`LightName`], [`EnableCap`], [`GetPName`]
pub const GL_LIGHT6: GLenum = GLenum(0x4006);

///
/// * Group: [`LightName`], [`EnableCap`], [`GetPName`]
pub const GL_LIGHT7: GLenum = GLenum(0x4007);

pub const GL_LIGHTEN: GLenum = GLenum(0x9298);

pub const GL_LIGHTEN_KHR: GLenum = GLenum(0x9298);

pub const GL_LIGHTEN_NV: GLenum = GLenum(0x9298);

///
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_LIGHTING: GLenum = GLenum(0x0B50);

///
/// * Group: [`AttribMask`]
pub const GL_LIGHTING_BIT: GLbitfield = GLbitfield(0x00000040);

///
/// * Group: [`LightEnvParameterSGIX`], [`GetPName`]
pub const GL_LIGHT_ENV_MODE_SGIX: GLenum = GLenum(0x8407);

///
/// * Group: [`LightModelParameter`], [`GetPName`]
pub const GL_LIGHT_MODEL_AMBIENT: GLenum = GLenum(0x0B53);

///
/// * Group: [`LightModelParameter`], [`GetPName`]
pub const GL_LIGHT_MODEL_COLOR_CONTROL: GLenum = GLenum(0x81F8);

///
/// * Group: [`LightModelParameter`]
pub const GL_LIGHT_MODEL_COLOR_CONTROL_EXT: GLenum = GLenum(0x81F8);

///
/// * Group: [`LightModelParameter`], [`GetPName`]
pub const GL_LIGHT_MODEL_LOCAL_VIEWER: GLenum = GLenum(0x0B51);

pub const GL_LIGHT_MODEL_SPECULAR_VECTOR_APPLE: GLenum = GLenum(0x85B0);

///
/// * Group: [`LightModelParameter`], [`GetPName`]
pub const GL_LIGHT_MODEL_TWO_SIDE: GLenum = GLenum(0x0B52);

///
/// * Group: [`PolygonMode`], [`MeshMode1`], [`MeshMode2`]
pub const GL_LINE: GLenum = GLenum(0x1B01);

///
/// * Group: [`BlitFramebufferFilter`], [`FogMode`], [`TextureMagFilter`],
///   [`TextureMinFilter`]
pub const GL_LINEAR: GLenum = GLenum(0x2601);

pub const GL_LINEARBURN_NV: GLenum = GLenum(0x92A5);

pub const GL_LINEARDODGE_NV: GLenum = GLenum(0x92A4);

pub const GL_LINEARLIGHT_NV: GLenum = GLenum(0x92A7);

///
/// * Group: [`LightParameter`], [`FragmentLightParameterSGIX`]
pub const GL_LINEAR_ATTENUATION: GLenum = GLenum(0x1208);

///
/// * Group: [`TextureMinFilter`]
pub const GL_LINEAR_CLIPMAP_LINEAR_SGIX: GLenum = GLenum(0x8170);

///
/// * Group: [`TextureMinFilter`]
pub const GL_LINEAR_CLIPMAP_NEAREST_SGIX: GLenum = GLenum(0x844F);

///
/// * Group: [`TextureMagFilter`]
pub const GL_LINEAR_DETAIL_ALPHA_SGIS: GLenum = GLenum(0x8098);

///
/// * Group: [`TextureMagFilter`]
pub const GL_LINEAR_DETAIL_COLOR_SGIS: GLenum = GLenum(0x8099);

///
/// * Group: [`TextureMagFilter`]
pub const GL_LINEAR_DETAIL_SGIS: GLenum = GLenum(0x8097);

///
/// * Group: [`TextureWrapMode`], [`TextureMinFilter`]
pub const GL_LINEAR_MIPMAP_LINEAR: GLenum = GLenum(0x2703);

///
/// * Group: [`TextureMinFilter`]
pub const GL_LINEAR_MIPMAP_NEAREST: GLenum = GLenum(0x2701);

///
/// * Group: [`TextureMagFilter`]
pub const GL_LINEAR_SHARPEN_ALPHA_SGIS: GLenum = GLenum(0x80AE);

///
/// * Group: [`TextureMagFilter`]
pub const GL_LINEAR_SHARPEN_COLOR_SGIS: GLenum = GLenum(0x80AF);

///
/// * Group: [`TextureMagFilter`]
pub const GL_LINEAR_SHARPEN_SGIS: GLenum = GLenum(0x80AD);

pub const GL_LINEAR_TILING_EXT: GLenum = GLenum(0x9585);

///
/// * Group: [`PrimitiveType`]
pub const GL_LINES: GLenum = GLenum(0x0001);

///
/// * Group: [`PrimitiveType`]
pub const GL_LINES_ADJACENCY: GLenum = GLenum(0x000A);

///
/// * Group: [`PrimitiveType`]
pub const GL_LINES_ADJACENCY_ARB: GLenum = GLenum(0x000A);

///
/// * Group: [`PrimitiveType`]
pub const GL_LINES_ADJACENCY_EXT: GLenum = GLenum(0x000A);

pub const GL_LINES_ADJACENCY_OES: GLenum = GLenum(0x000A);

///
/// * Group: [`AttribMask`]
pub const GL_LINE_BIT: GLbitfield = GLbitfield(0x00000004);

///
/// * Group: [`PrimitiveType`]
pub const GL_LINE_LOOP: GLenum = GLenum(0x0002);

pub const GL_LINE_NV: GLenum = GLenum(0x1B01);

///
/// * Group: [`HintTarget`]
pub const GL_LINE_QUALITY_HINT_SGIX: GLenum = GLenum(0x835B);

///
/// * Group: [`FeedBackToken`]
pub const GL_LINE_RESET_TOKEN: GLenum = GLenum(0x0707);

///
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_LINE_SMOOTH: GLenum = GLenum(0x0B20);

///
/// * Group: [`HintTarget`], [`GetPName`]
pub const GL_LINE_SMOOTH_HINT: GLenum = GLenum(0x0C52);

///
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_LINE_STIPPLE: GLenum = GLenum(0x0B24);

///
/// * Group: [`GetPName`]
pub const GL_LINE_STIPPLE_PATTERN: GLenum = GLenum(0x0B25);

///
/// * Group: [`GetPName`]
pub const GL_LINE_STIPPLE_REPEAT: GLenum = GLenum(0x0B26);

///
/// * Group: [`PrimitiveType`]
pub const GL_LINE_STRIP: GLenum = GLenum(0x0003);

///
/// * Group: [`PrimitiveType`]
pub const GL_LINE_STRIP_ADJACENCY: GLenum = GLenum(0x000B);

///
/// * Group: [`PrimitiveType`]
pub const GL_LINE_STRIP_ADJACENCY_ARB: GLenum = GLenum(0x000B);

///
/// * Group: [`PrimitiveType`]
pub const GL_LINE_STRIP_ADJACENCY_EXT: GLenum = GLenum(0x000B);

pub const GL_LINE_STRIP_ADJACENCY_OES: GLenum = GLenum(0x000B);

///
/// * Group: [`FeedBackToken`]
pub const GL_LINE_TOKEN: GLenum = GLenum(0x0702);

///
/// * Group: [`PathCoordType`]
pub const GL_LINE_TO_NV: GLenum = GLenum(0x04);

///
/// * Group: [`GetPName`]
pub const GL_LINE_WIDTH: GLenum = GLenum(0x0B21);

///
/// * Group: [`CommandOpcodesNV`]
pub const GL_LINE_WIDTH_COMMAND_NV: GLenum = GLenum(0x000D);

///
/// * Group: [`GetPName`]
pub const GL_LINE_WIDTH_GRANULARITY: GLenum = GLenum(0x0B23);

///
/// * Group: [`GetPName`]
pub const GL_LINE_WIDTH_RANGE: GLenum = GLenum(0x0B22);

///
/// * Group: [`ProgramPropertyARB`]
pub const GL_LINK_STATUS: GLenum = GLenum(0x8B82);

///
/// * Group: [`GetPName`]
pub const GL_LIST_BASE: GLenum = GLenum(0x0B32);

///
/// * Group: [`AttribMask`]
pub const GL_LIST_BIT: GLbitfield = GLbitfield(0x00020000);

///
/// * Group: [`GetPName`]
pub const GL_LIST_INDEX: GLenum = GLenum(0x0B33);

///
/// * Group: [`GetPName`]
pub const GL_LIST_MODE: GLenum = GLenum(0x0B30);

///
/// * Group: [`ListParameterName`]
pub const GL_LIST_PRIORITY_SGIX: GLenum = GLenum(0x8182);

///
/// * Group: [`AccumOp`]
pub const GL_LOAD: GLenum = GLenum(0x0101);

pub const GL_LOCAL_CONSTANT_DATATYPE_EXT: GLenum = GLenum(0x87ED);

///
/// * Group: [`VertexShaderStorageTypeEXT`]
pub const GL_LOCAL_CONSTANT_EXT: GLenum = GLenum(0x87C3);

pub const GL_LOCAL_CONSTANT_VALUE_EXT: GLenum = GLenum(0x87EC);

///
/// * Group: [`VertexShaderStorageTypeEXT`]
pub const GL_LOCAL_EXT: GLenum = GLenum(0x87C4);

///
/// * Group: [`ProgramResourceProperty`]
pub const GL_LOCATION: GLenum = GLenum(0x930E);

///
/// * Group: [`ProgramResourceProperty`]
pub const GL_LOCATION_COMPONENT: GLenum = GLenum(0x934A);

///
/// * Group: [`ProgramResourceProperty`]
pub const GL_LOCATION_INDEX: GLenum = GLenum(0x930F);

pub const GL_LOCATION_INDEX_EXT: GLenum = GLenum(0x930F);

///
/// * Group: [`GetPName`]
pub const GL_LOGIC_OP: GLenum = GLenum(0x0BF1);

///
/// * Group: [`GetPName`]
pub const GL_LOGIC_OP_MODE: GLenum = GLenum(0x0BF0);

pub const GL_LOSE_CONTEXT_ON_RESET: GLenum = GLenum(0x8252);

pub const GL_LOSE_CONTEXT_ON_RESET_ARB: GLenum = GLenum(0x8252);

pub const GL_LOSE_CONTEXT_ON_RESET_EXT: GLenum = GLenum(0x8252);

pub const GL_LOSE_CONTEXT_ON_RESET_KHR: GLenum = GLenum(0x8252);

///
/// * Group: [`ClipControlOrigin`]
pub const GL_LOWER_LEFT: GLenum = GLenum(0x8CA1);

///
/// * Alias Of: [`GL_LOWER_LEFT`]
pub const GL_LOWER_LEFT_EXT: GLenum = GL_LOWER_LEFT;

///
/// * Group: [`PrecisionType`]
pub const GL_LOW_FLOAT: GLenum = GLenum(0x8DF0);

///
/// * Group: [`PrecisionType`]
pub const GL_LOW_INT: GLenum = GLenum(0x8DF3);

pub const GL_LO_BIAS_NV: GLenum = GLenum(0x8715);

pub const GL_LO_SCALE_NV: GLenum = GLenum(0x870F);

pub const GL_LUID_SIZE_EXT: GLenum = GLenum(8);

///
/// * Group: [`PixelTexGenMode`], [`PathColorFormat`], [`PixelFormat`]
pub const GL_LUMINANCE: GLenum = GLenum(0x1909);

///
/// * Group: [`InternalFormat`]
pub const GL_LUMINANCE12: GLenum = GLenum(0x8041);

///
/// * Group: [`InternalFormat`]
pub const GL_LUMINANCE12_ALPHA12: GLenum = GLenum(0x8047);

pub const GL_LUMINANCE12_ALPHA12_EXT: GLenum = GLenum(0x8047);

///
/// * Group: [`InternalFormat`]
pub const GL_LUMINANCE12_ALPHA4: GLenum = GLenum(0x8046);

pub const GL_LUMINANCE12_ALPHA4_EXT: GLenum = GLenum(0x8046);

pub const GL_LUMINANCE12_EXT: GLenum = GLenum(0x8041);

///
/// * Group: [`InternalFormat`]
pub const GL_LUMINANCE16: GLenum = GLenum(0x8042);

pub const GL_LUMINANCE16F_ARB: GLenum = GLenum(0x881E);

pub const GL_LUMINANCE16F_EXT: GLenum = GLenum(0x881E);

pub const GL_LUMINANCE16I_EXT: GLenum = GLenum(0x8D8C);

pub const GL_LUMINANCE16UI_EXT: GLenum = GLenum(0x8D7A);

///
/// * Group: [`InternalFormat`]
pub const GL_LUMINANCE16_ALPHA16: GLenum = GLenum(0x8048);

pub const GL_LUMINANCE16_ALPHA16_EXT: GLenum = GLenum(0x8048);

pub const GL_LUMINANCE16_ALPHA16_SNORM: GLenum = GLenum(0x901A);

pub const GL_LUMINANCE16_EXT: GLenum = GLenum(0x8042);

pub const GL_LUMINANCE16_SNORM: GLenum = GLenum(0x9019);

pub const GL_LUMINANCE32F_ARB: GLenum = GLenum(0x8818);

pub const GL_LUMINANCE32F_EXT: GLenum = GLenum(0x8818);

pub const GL_LUMINANCE32I_EXT: GLenum = GLenum(0x8D86);

pub const GL_LUMINANCE32UI_EXT: GLenum = GLenum(0x8D74);

///
/// * Group: [`InternalFormat`]
pub const GL_LUMINANCE4: GLenum = GLenum(0x803F);

///
/// * Group: [`InternalFormat`]
pub const GL_LUMINANCE4_ALPHA4: GLenum = GLenum(0x8043);

pub const GL_LUMINANCE4_ALPHA4_EXT: GLenum = GLenum(0x8043);

pub const GL_LUMINANCE4_ALPHA4_OES: GLenum = GLenum(0x8043);

pub const GL_LUMINANCE4_EXT: GLenum = GLenum(0x803F);

///
/// * Group: [`InternalFormat`]
pub const GL_LUMINANCE6_ALPHA2: GLenum = GLenum(0x8044);

pub const GL_LUMINANCE6_ALPHA2_EXT: GLenum = GLenum(0x8044);

///
/// * Group: [`InternalFormat`]
pub const GL_LUMINANCE8: GLenum = GLenum(0x8040);

pub const GL_LUMINANCE8I_EXT: GLenum = GLenum(0x8D92);

pub const GL_LUMINANCE8UI_EXT: GLenum = GLenum(0x8D80);

///
/// * Group: [`InternalFormat`]
pub const GL_LUMINANCE8_ALPHA8: GLenum = GLenum(0x8045);

pub const GL_LUMINANCE8_ALPHA8_EXT: GLenum = GLenum(0x8045);

pub const GL_LUMINANCE8_ALPHA8_OES: GLenum = GLenum(0x8045);

pub const GL_LUMINANCE8_ALPHA8_SNORM: GLenum = GLenum(0x9016);

pub const GL_LUMINANCE8_EXT: GLenum = GLenum(0x8040);

pub const GL_LUMINANCE8_OES: GLenum = GLenum(0x8040);

pub const GL_LUMINANCE8_SNORM: GLenum = GLenum(0x9015);

///
/// * Group: [`PixelTexGenMode`], [`PathColorFormat`], [`PixelFormat`]
pub const GL_LUMINANCE_ALPHA: GLenum = GLenum(0x190A);

pub const GL_LUMINANCE_ALPHA16F_ARB: GLenum = GLenum(0x881F);

pub const GL_LUMINANCE_ALPHA16F_EXT: GLenum = GLenum(0x881F);

pub const GL_LUMINANCE_ALPHA16I_EXT: GLenum = GLenum(0x8D8D);

pub const GL_LUMINANCE_ALPHA16UI_EXT: GLenum = GLenum(0x8D7B);

pub const GL_LUMINANCE_ALPHA32F_ARB: GLenum = GLenum(0x8819);

pub const GL_LUMINANCE_ALPHA32F_EXT: GLenum = GLenum(0x8819);

pub const GL_LUMINANCE_ALPHA32I_EXT: GLenum = GLenum(0x8D87);

pub const GL_LUMINANCE_ALPHA32UI_EXT: GLenum = GLenum(0x8D75);

pub const GL_LUMINANCE_ALPHA8I_EXT: GLenum = GLenum(0x8D93);

pub const GL_LUMINANCE_ALPHA8UI_EXT: GLenum = GLenum(0x8D81);

pub const GL_LUMINANCE_ALPHA_FLOAT16_APPLE: GLenum = GLenum(0x881F);

pub const GL_LUMINANCE_ALPHA_FLOAT16_ATI: GLenum = GLenum(0x881F);

pub const GL_LUMINANCE_ALPHA_FLOAT32_APPLE: GLenum = GLenum(0x8819);

pub const GL_LUMINANCE_ALPHA_FLOAT32_ATI: GLenum = GLenum(0x8819);

pub const GL_LUMINANCE_ALPHA_INTEGER_EXT: GLenum = GLenum(0x8D9D);

pub const GL_LUMINANCE_ALPHA_SNORM: GLenum = GLenum(0x9012);

pub const GL_LUMINANCE_FLOAT16_APPLE: GLenum = GLenum(0x881E);

pub const GL_LUMINANCE_FLOAT16_ATI: GLenum = GLenum(0x881E);

pub const GL_LUMINANCE_FLOAT32_APPLE: GLenum = GLenum(0x8818);

pub const GL_LUMINANCE_FLOAT32_ATI: GLenum = GLenum(0x8818);

pub const GL_LUMINANCE_INTEGER_EXT: GLenum = GLenum(0x8D9C);

pub const GL_LUMINANCE_SNORM: GLenum = GLenum(0x9011);

///
/// * Group: [`FragmentOpATI`]
pub const GL_MAD_ATI: GLenum = GLenum(0x8968);

pub const GL_MAGNITUDE_BIAS_NV: GLenum = GLenum(0x8718);

pub const GL_MAGNITUDE_SCALE_NV: GLenum = GLenum(0x8712);

///
/// * Group: [`GetPName`]
pub const GL_MAJOR_VERSION: GLenum = GLenum(0x821B);

pub const GL_MALI_PROGRAM_BINARY_ARM: GLenum = GLenum(0x8F61);

///
/// * Group: [`ShaderBinaryFormat`]
pub const GL_MALI_SHADER_BINARY_ARM: GLenum = GLenum(0x8F60);

pub const GL_MANUAL_GENERATE_MIPMAP: GLenum = GLenum(0x8294);

pub const GL_MAP1_BINORMAL_EXT: GLenum = GLenum(0x8446);

///
/// * Group: [`MapTarget`], [`EnableCap`], [`GetPName`]
pub const GL_MAP1_COLOR_4: GLenum = GLenum(0x0D90);

///
/// * Group: [`GetPName`]
pub const GL_MAP1_GRID_DOMAIN: GLenum = GLenum(0x0DD0);

///
/// * Group: [`GetPName`]
pub const GL_MAP1_GRID_SEGMENTS: GLenum = GLenum(0x0DD1);

///
/// * Group: [`MapTarget`], [`EnableCap`], [`GetPName`]
pub const GL_MAP1_INDEX: GLenum = GLenum(0x0D91);

///
/// * Group: [`MapTarget`], [`EnableCap`], [`GetPName`]
pub const GL_MAP1_NORMAL: GLenum = GLenum(0x0D92);

pub const GL_MAP1_TANGENT_EXT: GLenum = GLenum(0x8444);

///
/// * Group: [`MapTarget`], [`EnableCap`], [`GetPName`]
pub const GL_MAP1_TEXTURE_COORD_1: GLenum = GLenum(0x0D93);

///
/// * Group: [`MapTarget`], [`EnableCap`], [`GetPName`]
pub const GL_MAP1_TEXTURE_COORD_2: GLenum = GLenum(0x0D94);

///
/// * Group: [`MapTarget`], [`EnableCap`], [`GetPName`]
pub const GL_MAP1_TEXTURE_COORD_3: GLenum = GLenum(0x0D95);

///
/// * Group: [`MapTarget`], [`EnableCap`], [`GetPName`]
pub const GL_MAP1_TEXTURE_COORD_4: GLenum = GLenum(0x0D96);

///
/// * Group: [`MapTarget`], [`EnableCap`], [`GetPName`]
pub const GL_MAP1_VERTEX_3: GLenum = GLenum(0x0D97);

///
/// * Group: [`MapTarget`], [`EnableCap`], [`GetPName`]
pub const GL_MAP1_VERTEX_4: GLenum = GLenum(0x0D98);

pub const GL_MAP1_VERTEX_ATTRIB0_4_NV: GLenum = GLenum(0x8660);

pub const GL_MAP1_VERTEX_ATTRIB10_4_NV: GLenum = GLenum(0x866A);

pub const GL_MAP1_VERTEX_ATTRIB11_4_NV: GLenum = GLenum(0x866B);

pub const GL_MAP1_VERTEX_ATTRIB12_4_NV: GLenum = GLenum(0x866C);

pub const GL_MAP1_VERTEX_ATTRIB13_4_NV: GLenum = GLenum(0x866D);

pub const GL_MAP1_VERTEX_ATTRIB14_4_NV: GLenum = GLenum(0x866E);

pub const GL_MAP1_VERTEX_ATTRIB15_4_NV: GLenum = GLenum(0x866F);

pub const GL_MAP1_VERTEX_ATTRIB1_4_NV: GLenum = GLenum(0x8661);

pub const GL_MAP1_VERTEX_ATTRIB2_4_NV: GLenum = GLenum(0x8662);

pub const GL_MAP1_VERTEX_ATTRIB3_4_NV: GLenum = GLenum(0x8663);

pub const GL_MAP1_VERTEX_ATTRIB4_4_NV: GLenum = GLenum(0x8664);

pub const GL_MAP1_VERTEX_ATTRIB5_4_NV: GLenum = GLenum(0x8665);

pub const GL_MAP1_VERTEX_ATTRIB6_4_NV: GLenum = GLenum(0x8666);

pub const GL_MAP1_VERTEX_ATTRIB7_4_NV: GLenum = GLenum(0x8667);

pub const GL_MAP1_VERTEX_ATTRIB8_4_NV: GLenum = GLenum(0x8668);

pub const GL_MAP1_VERTEX_ATTRIB9_4_NV: GLenum = GLenum(0x8669);

pub const GL_MAP2_BINORMAL_EXT: GLenum = GLenum(0x8447);

///
/// * Group: [`MapTarget`], [`EnableCap`], [`GetPName`]
pub const GL_MAP2_COLOR_4: GLenum = GLenum(0x0DB0);

///
/// * Group: [`GetPName`]
pub const GL_MAP2_GRID_DOMAIN: GLenum = GLenum(0x0DD2);

///
/// * Group: [`GetPName`]
pub const GL_MAP2_GRID_SEGMENTS: GLenum = GLenum(0x0DD3);

///
/// * Group: [`MapTarget`], [`EnableCap`], [`GetPName`]
pub const GL_MAP2_INDEX: GLenum = GLenum(0x0DB1);

///
/// * Group: [`MapTarget`], [`EnableCap`], [`GetPName`]
pub const GL_MAP2_NORMAL: GLenum = GLenum(0x0DB2);

pub const GL_MAP2_TANGENT_EXT: GLenum = GLenum(0x8445);

///
/// * Group: [`MapTarget`], [`EnableCap`], [`GetPName`]
pub const GL_MAP2_TEXTURE_COORD_1: GLenum = GLenum(0x0DB3);

///
/// * Group: [`MapTarget`], [`EnableCap`], [`GetPName`]
pub const GL_MAP2_TEXTURE_COORD_2: GLenum = GLenum(0x0DB4);

///
/// * Group: [`MapTarget`], [`EnableCap`], [`GetPName`]
pub const GL_MAP2_TEXTURE_COORD_3: GLenum = GLenum(0x0DB5);

///
/// * Group: [`MapTarget`], [`EnableCap`], [`GetPName`]
pub const GL_MAP2_TEXTURE_COORD_4: GLenum = GLenum(0x0DB6);

///
/// * Group: [`MapTarget`], [`EnableCap`], [`GetPName`]
pub const GL_MAP2_VERTEX_3: GLenum = GLenum(0x0DB7);

///
/// * Group: [`MapTarget`], [`EnableCap`], [`GetPName`]
pub const GL_MAP2_VERTEX_4: GLenum = GLenum(0x0DB8);

pub const GL_MAP2_VERTEX_ATTRIB0_4_NV: GLenum = GLenum(0x8670);

pub const GL_MAP2_VERTEX_ATTRIB10_4_NV: GLenum = GLenum(0x867A);

pub const GL_MAP2_VERTEX_ATTRIB11_4_NV: GLenum = GLenum(0x867B);

pub const GL_MAP2_VERTEX_ATTRIB12_4_NV: GLenum = GLenum(0x867C);

pub const GL_MAP2_VERTEX_ATTRIB13_4_NV: GLenum = GLenum(0x867D);

pub const GL_MAP2_VERTEX_ATTRIB14_4_NV: GLenum = GLenum(0x867E);

pub const GL_MAP2_VERTEX_ATTRIB15_4_NV: GLenum = GLenum(0x867F);

pub const GL_MAP2_VERTEX_ATTRIB1_4_NV: GLenum = GLenum(0x8671);

pub const GL_MAP2_VERTEX_ATTRIB2_4_NV: GLenum = GLenum(0x8672);

pub const GL_MAP2_VERTEX_ATTRIB3_4_NV: GLenum = GLenum(0x8673);

pub const GL_MAP2_VERTEX_ATTRIB4_4_NV: GLenum = GLenum(0x8674);

pub const GL_MAP2_VERTEX_ATTRIB5_4_NV: GLenum = GLenum(0x8675);

pub const GL_MAP2_VERTEX_ATTRIB6_4_NV: GLenum = GLenum(0x8676);

pub const GL_MAP2_VERTEX_ATTRIB7_4_NV: GLenum = GLenum(0x8677);

pub const GL_MAP2_VERTEX_ATTRIB8_4_NV: GLenum = GLenum(0x8678);

pub const GL_MAP2_VERTEX_ATTRIB9_4_NV: GLenum = GLenum(0x8679);

///
/// * Group: [`MapAttribParameterNV`]
pub const GL_MAP_ATTRIB_U_ORDER_NV: GLenum = GLenum(0x86C3);

///
/// * Group: [`MapAttribParameterNV`]
pub const GL_MAP_ATTRIB_V_ORDER_NV: GLenum = GLenum(0x86C4);

///
/// * Group: [`MapBufferAccessMask`], [`BufferStorageMask`]
pub const GL_MAP_COHERENT_BIT: GLbitfield = GLbitfield(0x0080);

///
/// * Group: [`MapBufferAccessMask`], [`BufferStorageMask`]
pub const GL_MAP_COHERENT_BIT_EXT: GLbitfield = GLbitfield(0x0080);

///
/// * Group: [`PixelTransferParameter`], [`GetPName`]
pub const GL_MAP_COLOR: GLenum = GLenum(0x0D10);

///
/// * Group: [`MapBufferAccessMask`]
pub const GL_MAP_FLUSH_EXPLICIT_BIT: GLbitfield = GLbitfield(0x0010);

///
/// * Group: [`MapBufferAccessMask`]
pub const GL_MAP_FLUSH_EXPLICIT_BIT_EXT: GLbitfield = GLbitfield(0x0010);

///
/// * Group: [`MapBufferAccessMask`]
pub const GL_MAP_INVALIDATE_BUFFER_BIT: GLbitfield = GLbitfield(0x0008);

///
/// * Group: [`MapBufferAccessMask`]
pub const GL_MAP_INVALIDATE_BUFFER_BIT_EXT: GLbitfield = GLbitfield(0x0008);

///
/// * Group: [`MapBufferAccessMask`]
pub const GL_MAP_INVALIDATE_RANGE_BIT: GLbitfield = GLbitfield(0x0004);

///
/// * Group: [`MapBufferAccessMask`]
pub const GL_MAP_INVALIDATE_RANGE_BIT_EXT: GLbitfield = GLbitfield(0x0004);

///
/// * Group: [`MapBufferAccessMask`], [`BufferStorageMask`]
pub const GL_MAP_PERSISTENT_BIT: GLbitfield = GLbitfield(0x0040);

///
/// * Group: [`MapBufferAccessMask`], [`BufferStorageMask`]
pub const GL_MAP_PERSISTENT_BIT_EXT: GLbitfield = GLbitfield(0x0040);

///
/// * Group: [`MapBufferAccessMask`], [`BufferStorageMask`]
pub const GL_MAP_READ_BIT: GLbitfield = GLbitfield(0x0001);

///
/// * Group: [`MapBufferAccessMask`], [`BufferStorageMask`]
pub const GL_MAP_READ_BIT_EXT: GLbitfield = GLbitfield(0x0001);

///
/// * Group: [`PixelTransferParameter`], [`GetPName`]
pub const GL_MAP_STENCIL: GLenum = GLenum(0x0D11);

///
/// * Group: [`MapParameterNV`]
pub const GL_MAP_TESSELLATION_NV: GLenum = GLenum(0x86C2);

///
/// * Group: [`MapBufferAccessMask`]
pub const GL_MAP_UNSYNCHRONIZED_BIT: GLbitfield = GLbitfield(0x0020);

///
/// * Group: [`MapBufferAccessMask`]
pub const GL_MAP_UNSYNCHRONIZED_BIT_EXT: GLbitfield = GLbitfield(0x0020);

///
/// * Group: [`MapBufferAccessMask`], [`BufferStorageMask`]
pub const GL_MAP_WRITE_BIT: GLbitfield = GLbitfield(0x0002);

///
/// * Group: [`MapBufferAccessMask`], [`BufferStorageMask`]
pub const GL_MAP_WRITE_BIT_EXT: GLbitfield = GLbitfield(0x0002);

///
/// * Group: [`HintTarget`], [`HintTargetPGI`]
pub const GL_MATERIAL_SIDE_HINT_PGI: GLenum = GLenum(0x1A22C);

pub const GL_MATRIX0_ARB: GLenum = GLenum(0x88C0);

pub const GL_MATRIX0_NV: GLenum = GLenum(0x8630);

pub const GL_MATRIX10_ARB: GLenum = GLenum(0x88CA);

pub const GL_MATRIX11_ARB: GLenum = GLenum(0x88CB);

pub const GL_MATRIX12_ARB: GLenum = GLenum(0x88CC);

pub const GL_MATRIX13_ARB: GLenum = GLenum(0x88CD);

pub const GL_MATRIX14_ARB: GLenum = GLenum(0x88CE);

pub const GL_MATRIX15_ARB: GLenum = GLenum(0x88CF);

pub const GL_MATRIX16_ARB: GLenum = GLenum(0x88D0);

pub const GL_MATRIX17_ARB: GLenum = GLenum(0x88D1);

pub const GL_MATRIX18_ARB: GLenum = GLenum(0x88D2);

pub const GL_MATRIX19_ARB: GLenum = GLenum(0x88D3);

pub const GL_MATRIX1_ARB: GLenum = GLenum(0x88C1);

pub const GL_MATRIX1_NV: GLenum = GLenum(0x8631);

pub const GL_MATRIX20_ARB: GLenum = GLenum(0x88D4);

pub const GL_MATRIX21_ARB: GLenum = GLenum(0x88D5);

pub const GL_MATRIX22_ARB: GLenum = GLenum(0x88D6);

pub const GL_MATRIX23_ARB: GLenum = GLenum(0x88D7);

pub const GL_MATRIX24_ARB: GLenum = GLenum(0x88D8);

pub const GL_MATRIX25_ARB: GLenum = GLenum(0x88D9);

pub const GL_MATRIX26_ARB: GLenum = GLenum(0x88DA);

pub const GL_MATRIX27_ARB: GLenum = GLenum(0x88DB);

pub const GL_MATRIX28_ARB: GLenum = GLenum(0x88DC);

pub const GL_MATRIX29_ARB: GLenum = GLenum(0x88DD);

pub const GL_MATRIX2_ARB: GLenum = GLenum(0x88C2);

pub const GL_MATRIX2_NV: GLenum = GLenum(0x8632);

pub const GL_MATRIX30_ARB: GLenum = GLenum(0x88DE);

pub const GL_MATRIX31_ARB: GLenum = GLenum(0x88DF);

pub const GL_MATRIX3_ARB: GLenum = GLenum(0x88C3);

pub const GL_MATRIX3_NV: GLenum = GLenum(0x8633);

pub const GL_MATRIX4_ARB: GLenum = GLenum(0x88C4);

pub const GL_MATRIX4_NV: GLenum = GLenum(0x8634);

pub const GL_MATRIX5_ARB: GLenum = GLenum(0x88C5);

pub const GL_MATRIX5_NV: GLenum = GLenum(0x8635);

pub const GL_MATRIX6_ARB: GLenum = GLenum(0x88C6);

pub const GL_MATRIX6_NV: GLenum = GLenum(0x8636);

pub const GL_MATRIX7_ARB: GLenum = GLenum(0x88C7);

pub const GL_MATRIX7_NV: GLenum = GLenum(0x8637);

pub const GL_MATRIX8_ARB: GLenum = GLenum(0x88C8);

pub const GL_MATRIX9_ARB: GLenum = GLenum(0x88C9);

///
/// * Group: [`DataTypeEXT`]
pub const GL_MATRIX_EXT: GLenum = GLenum(0x87C0);

pub const GL_MATRIX_INDEX_ARRAY_ARB: GLenum = GLenum(0x8844);

pub const GL_MATRIX_INDEX_ARRAY_BUFFER_BINDING_OES: GLenum = GLenum(0x8B9E);

pub const GL_MATRIX_INDEX_ARRAY_OES: GLenum = GLenum(0x8844);

pub const GL_MATRIX_INDEX_ARRAY_POINTER_ARB: GLenum = GLenum(0x8849);

pub const GL_MATRIX_INDEX_ARRAY_POINTER_OES: GLenum = GLenum(0x8849);

pub const GL_MATRIX_INDEX_ARRAY_SIZE_ARB: GLenum = GLenum(0x8846);

pub const GL_MATRIX_INDEX_ARRAY_SIZE_OES: GLenum = GLenum(0x8846);

pub const GL_MATRIX_INDEX_ARRAY_STRIDE_ARB: GLenum = GLenum(0x8848);

pub const GL_MATRIX_INDEX_ARRAY_STRIDE_OES: GLenum = GLenum(0x8848);

pub const GL_MATRIX_INDEX_ARRAY_TYPE_ARB: GLenum = GLenum(0x8847);

pub const GL_MATRIX_INDEX_ARRAY_TYPE_OES: GLenum = GLenum(0x8847);

///
/// * Group: [`GetPName`]
pub const GL_MATRIX_MODE: GLenum = GLenum(0x0BA0);

pub const GL_MATRIX_PALETTE_ARB: GLenum = GLenum(0x8840);

pub const GL_MATRIX_PALETTE_OES: GLenum = GLenum(0x8840);

///
/// * Group: [`ProgramResourceProperty`]
pub const GL_MATRIX_STRIDE: GLenum = GLenum(0x92FF);

///
/// * Group: [`VertexHintsMaskPGI`]
pub const GL_MAT_AMBIENT_AND_DIFFUSE_BIT_PGI: GLbitfield = GLbitfield(0x00200000);

///
/// * Group: [`VertexHintsMaskPGI`]
pub const GL_MAT_AMBIENT_BIT_PGI: GLbitfield = GLbitfield(0x00100000);

///
/// * Group: [`VertexHintsMaskPGI`]
pub const GL_MAT_COLOR_INDEXES_BIT_PGI: GLbitfield = GLbitfield(0x01000000);

///
/// * Group: [`VertexHintsMaskPGI`]
pub const GL_MAT_DIFFUSE_BIT_PGI: GLbitfield = GLbitfield(0x00400000);

///
/// * Group: [`VertexHintsMaskPGI`]
pub const GL_MAT_EMISSION_BIT_PGI: GLbitfield = GLbitfield(0x00800000);

///
/// * Group: [`VertexHintsMaskPGI`]
pub const GL_MAT_SHININESS_BIT_PGI: GLbitfield = GLbitfield(0x02000000);

///
/// * Group: [`VertexHintsMaskPGI`]
pub const GL_MAT_SPECULAR_BIT_PGI: GLbitfield = GLbitfield(0x04000000);

///
/// * Group: [`BlendEquationModeEXT`]
pub const GL_MAX: GLenum = GLenum(0x8008);

///
/// * Group: [`GetPName`]
pub const GL_MAX_3D_TEXTURE_SIZE: GLenum = GLenum(0x8073);

///
/// * Group: [`GetPName`]
pub const GL_MAX_3D_TEXTURE_SIZE_EXT: GLenum = GLenum(0x8073);

pub const GL_MAX_3D_TEXTURE_SIZE_OES: GLenum = GLenum(0x8073);

///
/// * Group: [`GetPName`]
pub const GL_MAX_4D_TEXTURE_SIZE_SGIS: GLenum = GLenum(0x8138);

///
/// * Group: [`GetPName`]
pub const GL_MAX_ACTIVE_LIGHTS_SGIX: GLenum = GLenum(0x8405);

///
/// * Group: [`GetPName`]
pub const GL_MAX_ARRAY_TEXTURE_LAYERS: GLenum = GLenum(0x88FF);

pub const GL_MAX_ARRAY_TEXTURE_LAYERS_EXT: GLenum = GLenum(0x88FF);

///
/// * Group: [`GetPName`]
pub const GL_MAX_ASYNC_DRAW_PIXELS_SGIX: GLenum = GLenum(0x8360);

///
/// * Group: [`GetPName`]
pub const GL_MAX_ASYNC_HISTOGRAM_SGIX: GLenum = GLenum(0x832D);

///
/// * Group: [`GetPName`]
pub const GL_MAX_ASYNC_READ_PIXELS_SGIX: GLenum = GLenum(0x8361);

///
/// * Group: [`GetPName`]
pub const GL_MAX_ASYNC_TEX_IMAGE_SGIX: GLenum = GLenum(0x835F);

pub const GL_MAX_ATOMIC_COUNTER_BUFFER_BINDINGS: GLenum = GLenum(0x92DC);

pub const GL_MAX_ATOMIC_COUNTER_BUFFER_SIZE: GLenum = GLenum(0x92D8);

///
/// * Group: [`GetPName`]
pub const GL_MAX_ATTRIB_STACK_DEPTH: GLenum = GLenum(0x0D35);

pub const GL_MAX_BINDABLE_UNIFORM_SIZE_EXT: GLenum = GLenum(0x8DED);

///
/// * Group: [`GetPName`]
pub const GL_MAX_CLIENT_ATTRIB_STACK_DEPTH: GLenum = GLenum(0x0D3B);

///
/// * Group: [`GetPName`]
pub const GL_MAX_CLIPMAP_DEPTH_SGIX: GLenum = GLenum(0x8177);

///
/// * Group: [`GetPName`]
pub const GL_MAX_CLIPMAP_VIRTUAL_DEPTH_SGIX: GLenum = GLenum(0x8178);

///
/// * Group: [`GetPName`]
/// * Alias Of: [`GL_MAX_CLIP_PLANES`]
pub const GL_MAX_CLIP_DISTANCES: GLenum = GL_MAX_CLIP_PLANES;

pub const GL_MAX_CLIP_DISTANCES_APPLE: GLenum = GLenum(0x0D32);

///
/// * Alias Of: [`GL_MAX_CLIP_PLANES`]
pub const GL_MAX_CLIP_DISTANCES_EXT: GLenum = GL_MAX_CLIP_PLANES;

///
/// * Group: [`GetPName`]
pub const GL_MAX_CLIP_PLANES: GLenum = GLenum(0x0D32);

pub const GL_MAX_CLIP_PLANES_IMG: GLenum = GLenum(0x0D32);

pub const GL_MAX_COARSE_FRAGMENT_SAMPLES_NV: GLenum = GLenum(0x955F);

pub const GL_MAX_COLOR_ATTACHMENTS: GLenum = GLenum(0x8CDF);

pub const GL_MAX_COLOR_ATTACHMENTS_EXT: GLenum = GLenum(0x8CDF);

pub const GL_MAX_COLOR_ATTACHMENTS_NV: GLenum = GLenum(0x8CDF);

pub const GL_MAX_COLOR_FRAMEBUFFER_SAMPLES_AMD: GLenum = GLenum(0x91B3);

pub const GL_MAX_COLOR_FRAMEBUFFER_STORAGE_SAMPLES_AMD: GLenum = GLenum(0x91B4);

pub const GL_MAX_COLOR_MATRIX_STACK_DEPTH: GLenum = GLenum(0x80B3);

///
/// * Group: [`GetPName`]
pub const GL_MAX_COLOR_MATRIX_STACK_DEPTH_SGI: GLenum = GLenum(0x80B3);

///
/// * Group: [`GetPName`]
pub const GL_MAX_COLOR_TEXTURE_SAMPLES: GLenum = GLenum(0x910E);

///
/// * Group: [`GetPName`]
pub const GL_MAX_COMBINED_ATOMIC_COUNTERS: GLenum = GLenum(0x92D7);

pub const GL_MAX_COMBINED_ATOMIC_COUNTER_BUFFERS: GLenum = GLenum(0x92D1);

pub const GL_MAX_COMBINED_CLIP_AND_CULL_DISTANCES: GLenum = GLenum(0x82FA);

///
/// * Alias Of: [`GL_MAX_COMBINED_CLIP_AND_CULL_DISTANCES`]
pub const GL_MAX_COMBINED_CLIP_AND_CULL_DISTANCES_EXT: GLenum = GL_MAX_COMBINED_CLIP_AND_CULL_DISTANCES;

///
/// * Group: [`GetPName`]
pub const GL_MAX_COMBINED_COMPUTE_UNIFORM_COMPONENTS: GLenum = GLenum(0x8266);

pub const GL_MAX_COMBINED_DIMENSIONS: GLenum = GLenum(0x8282);

///
/// * Group: [`GetPName`]
pub const GL_MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS: GLenum = GLenum(0x8A33);

///
/// * Group: [`GetPName`]
pub const GL_MAX_COMBINED_GEOMETRY_UNIFORM_COMPONENTS: GLenum = GLenum(0x8A32);

pub const GL_MAX_COMBINED_GEOMETRY_UNIFORM_COMPONENTS_EXT: GLenum = GLenum(0x8A32);

pub const GL_MAX_COMBINED_GEOMETRY_UNIFORM_COMPONENTS_OES: GLenum = GLenum(0x8A32);

pub const GL_MAX_COMBINED_IMAGE_UNIFORMS: GLenum = GLenum(0x90CF);

pub const GL_MAX_COMBINED_IMAGE_UNITS_AND_FRAGMENT_OUTPUTS: GLenum = GLenum(0x8F39);

pub const GL_MAX_COMBINED_IMAGE_UNITS_AND_FRAGMENT_OUTPUTS_EXT: GLenum = GLenum(0x8F39);

pub const GL_MAX_COMBINED_MESH_UNIFORM_COMPONENTS_NV: GLenum = GLenum(0x8E67);

///
/// * Alias Of: [`GL_MAX_COMBINED_IMAGE_UNITS_AND_FRAGMENT_OUTPUTS`]
pub const GL_MAX_COMBINED_SHADER_OUTPUT_RESOURCES: GLenum = GL_MAX_COMBINED_IMAGE_UNITS_AND_FRAGMENT_OUTPUTS;

///
/// * Group: [`GetPName`]
pub const GL_MAX_COMBINED_SHADER_STORAGE_BLOCKS: GLenum = GLenum(0x90DC);

pub const GL_MAX_COMBINED_TASK_UNIFORM_COMPONENTS_NV: GLenum = GLenum(0x8E6F);

pub const GL_MAX_COMBINED_TESS_CONTROL_UNIFORM_COMPONENTS: GLenum = GLenum(0x8E1E);

pub const GL_MAX_COMBINED_TESS_CONTROL_UNIFORM_COMPONENTS_EXT: GLenum = GLenum(0x8E1E);

pub const GL_MAX_COMBINED_TESS_CONTROL_UNIFORM_COMPONENTS_OES: GLenum = GLenum(0x8E1E);

pub const GL_MAX_COMBINED_TESS_EVALUATION_UNIFORM_COMPONENTS: GLenum = GLenum(0x8E1F);

pub const GL_MAX_COMBINED_TESS_EVALUATION_UNIFORM_COMPONENTS_EXT: GLenum = GLenum(0x8E1F);

pub const GL_MAX_COMBINED_TESS_EVALUATION_UNIFORM_COMPONENTS_OES: GLenum = GLenum(0x8E1F);

///
/// * Group: [`GetPName`]
pub const GL_MAX_COMBINED_TEXTURE_IMAGE_UNITS: GLenum = GLenum(0x8B4D);

pub const GL_MAX_COMBINED_TEXTURE_IMAGE_UNITS_ARB: GLenum = GLenum(0x8B4D);

///
/// * Group: [`GetPName`]
pub const GL_MAX_COMBINED_UNIFORM_BLOCKS: GLenum = GLenum(0x8A2E);

///
/// * Group: [`GetPName`]
pub const GL_MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS: GLenum = GLenum(0x8A31);

///
/// * Group: [`GetPName`]
pub const GL_MAX_COMPUTE_ATOMIC_COUNTERS: GLenum = GLenum(0x8265);

///
/// * Group: [`GetPName`]
pub const GL_MAX_COMPUTE_ATOMIC_COUNTER_BUFFERS: GLenum = GLenum(0x8264);

///
/// * Alias Of: [`GL_MAX_COMPUTE_WORK_GROUP_INVOCATIONS`]
pub const GL_MAX_COMPUTE_FIXED_GROUP_INVOCATIONS_ARB: GLenum = GL_MAX_COMPUTE_WORK_GROUP_INVOCATIONS;

///
/// * Alias Of: [`GL_MAX_COMPUTE_WORK_GROUP_SIZE`]
pub const GL_MAX_COMPUTE_FIXED_GROUP_SIZE_ARB: GLenum = GL_MAX_COMPUTE_WORK_GROUP_SIZE;

pub const GL_MAX_COMPUTE_IMAGE_UNIFORMS: GLenum = GLenum(0x91BD);

///
/// * Group: [`GetPName`]
pub const GL_MAX_COMPUTE_SHADER_STORAGE_BLOCKS: GLenum = GLenum(0x90DB);

pub const GL_MAX_COMPUTE_SHARED_MEMORY_SIZE: GLenum = GLenum(0x8262);

///
/// * Group: [`GetPName`]
pub const GL_MAX_COMPUTE_TEXTURE_IMAGE_UNITS: GLenum = GLenum(0x91BC);

///
/// * Group: [`GetPName`]
pub const GL_MAX_COMPUTE_UNIFORM_BLOCKS: GLenum = GLenum(0x91BB);

///
/// * Group: [`GetPName`]
pub const GL_MAX_COMPUTE_UNIFORM_COMPONENTS: GLenum = GLenum(0x8263);

pub const GL_MAX_COMPUTE_VARIABLE_GROUP_INVOCATIONS_ARB: GLenum = GLenum(0x9344);

pub const GL_MAX_COMPUTE_VARIABLE_GROUP_SIZE_ARB: GLenum = GLenum(0x9345);

///
/// * Group: [`GetPName`]
pub const GL_MAX_COMPUTE_WORK_GROUP_COUNT: GLenum = GLenum(0x91BE);

///
/// * Group: [`GetPName`]
pub const GL_MAX_COMPUTE_WORK_GROUP_INVOCATIONS: GLenum = GLenum(0x90EB);

///
/// * Group: [`GetPName`]
pub const GL_MAX_COMPUTE_WORK_GROUP_SIZE: GLenum = GLenum(0x91BF);

///
/// * Group: [`GetConvolutionParameter`]
pub const GL_MAX_CONVOLUTION_HEIGHT: GLenum = GLenum(0x801B);

///
/// * Group: [`GetConvolutionParameter`]
pub const GL_MAX_CONVOLUTION_HEIGHT_EXT: GLenum = GLenum(0x801B);

///
/// * Group: [`GetConvolutionParameter`]
pub const GL_MAX_CONVOLUTION_WIDTH: GLenum = GLenum(0x801A);

///
/// * Group: [`GetConvolutionParameter`]
pub const GL_MAX_CONVOLUTION_WIDTH_EXT: GLenum = GLenum(0x801A);

///
/// * Group: [`GetPName`]
pub const GL_MAX_CUBE_MAP_TEXTURE_SIZE: GLenum = GLenum(0x851C);

pub const GL_MAX_CUBE_MAP_TEXTURE_SIZE_ARB: GLenum = GLenum(0x851C);

pub const GL_MAX_CUBE_MAP_TEXTURE_SIZE_EXT: GLenum = GLenum(0x851C);

pub const GL_MAX_CUBE_MAP_TEXTURE_SIZE_OES: GLenum = GLenum(0x851C);

pub const GL_MAX_CULL_DISTANCES: GLenum = GLenum(0x82F9);

///
/// * Alias Of: [`GL_MAX_CULL_DISTANCES`]
pub const GL_MAX_CULL_DISTANCES_EXT: GLenum = GL_MAX_CULL_DISTANCES;

///
/// * Group: [`GetPName`]
pub const GL_MAX_DEBUG_GROUP_STACK_DEPTH: GLenum = GLenum(0x826C);

pub const GL_MAX_DEBUG_GROUP_STACK_DEPTH_KHR: GLenum = GLenum(0x826C);

pub const GL_MAX_DEBUG_LOGGED_MESSAGES: GLenum = GLenum(0x9144);

pub const GL_MAX_DEBUG_LOGGED_MESSAGES_AMD: GLenum = GLenum(0x9144);

pub const GL_MAX_DEBUG_LOGGED_MESSAGES_ARB: GLenum = GLenum(0x9144);

pub const GL_MAX_DEBUG_LOGGED_MESSAGES_KHR: GLenum = GLenum(0x9144);

pub const GL_MAX_DEBUG_MESSAGE_LENGTH: GLenum = GLenum(0x9143);

pub const GL_MAX_DEBUG_MESSAGE_LENGTH_AMD: GLenum = GLenum(0x9143);

pub const GL_MAX_DEBUG_MESSAGE_LENGTH_ARB: GLenum = GLenum(0x9143);

pub const GL_MAX_DEBUG_MESSAGE_LENGTH_KHR: GLenum = GLenum(0x9143);

pub const GL_MAX_DEEP_3D_TEXTURE_DEPTH_NV: GLenum = GLenum(0x90D1);

pub const GL_MAX_DEEP_3D_TEXTURE_WIDTH_HEIGHT_NV: GLenum = GLenum(0x90D0);

pub const GL_MAX_DEFORMATION_ORDER_SGIX: GLenum = GLenum(0x8197);

///
/// * Group: [`InternalFormatPName`]
pub const GL_MAX_DEPTH: GLenum = GLenum(0x8280);

pub const GL_MAX_DEPTH_STENCIL_FRAMEBUFFER_SAMPLES_AMD: GLenum = GLenum(0x91B5);

///
/// * Group: [`GetPName`]
pub const GL_MAX_DEPTH_TEXTURE_SAMPLES: GLenum = GLenum(0x910F);

pub const GL_MAX_DETACHED_BUFFERS_NV: GLenum = GLenum(0x95AD);

pub const GL_MAX_DETACHED_TEXTURES_NV: GLenum = GLenum(0x95AC);

///
/// * Group: [`GetPName`]
pub const GL_MAX_DRAW_BUFFERS: GLenum = GLenum(0x8824);

pub const GL_MAX_DRAW_BUFFERS_ARB: GLenum = GLenum(0x8824);

pub const GL_MAX_DRAW_BUFFERS_ATI: GLenum = GLenum(0x8824);

pub const GL_MAX_DRAW_BUFFERS_EXT: GLenum = GLenum(0x8824);

pub const GL_MAX_DRAW_BUFFERS_NV: GLenum = GLenum(0x8824);

pub const GL_MAX_DRAW_MESH_TASKS_COUNT_NV: GLenum = GLenum(0x953D);

///
/// * Group: [`GetPName`]
pub const GL_MAX_DUAL_SOURCE_DRAW_BUFFERS: GLenum = GLenum(0x88FC);

pub const GL_MAX_DUAL_SOURCE_DRAW_BUFFERS_EXT: GLenum = GLenum(0x88FC);

///
/// * Group: [`GetPName`]
pub const GL_MAX_ELEMENTS_INDICES: GLenum = GLenum(0x80E9);

pub const GL_MAX_ELEMENTS_INDICES_EXT: GLenum = GLenum(0x80E9);

///
/// * Group: [`GetPName`]
pub const GL_MAX_ELEMENTS_VERTICES: GLenum = GLenum(0x80E8);

pub const GL_MAX_ELEMENTS_VERTICES_EXT: GLenum = GLenum(0x80E8);

///
/// * Group: [`GetPName`]
pub const GL_MAX_ELEMENT_INDEX: GLenum = GLenum(0x8D6B);

///
/// * Group: [`GetPName`]
pub const GL_MAX_EVAL_ORDER: GLenum = GLenum(0x0D30);

///
/// * Group: [`BlendEquationModeEXT`]
pub const GL_MAX_EXT: GLenum = GLenum(0x8008);

///
/// * Group: [`GetPName`]
pub const GL_MAX_FOG_FUNC_POINTS_SGIS: GLenum = GLenum(0x812C);

///
/// * Group: [`GetPName`]
pub const GL_MAX_FRAGMENT_ATOMIC_COUNTERS: GLenum = GLenum(0x92D6);

pub const GL_MAX_FRAGMENT_ATOMIC_COUNTER_BUFFERS: GLenum = GLenum(0x92D0);

pub const GL_MAX_FRAGMENT_BINDABLE_UNIFORMS_EXT: GLenum = GLenum(0x8DE3);

pub const GL_MAX_FRAGMENT_IMAGE_UNIFORMS: GLenum = GLenum(0x90CE);

///
/// * Group: [`GetPName`]
pub const GL_MAX_FRAGMENT_INPUT_COMPONENTS: GLenum = GLenum(0x9125);

pub const GL_MAX_FRAGMENT_INTERPOLATION_OFFSET: GLenum = GLenum(0x8E5C);

pub const GL_MAX_FRAGMENT_INTERPOLATION_OFFSET_NV: GLenum = GLenum(0x8E5C);

pub const GL_MAX_FRAGMENT_INTERPOLATION_OFFSET_OES: GLenum = GLenum(0x8E5C);

///
/// * Group: [`GetPName`]
pub const GL_MAX_FRAGMENT_LIGHTS_SGIX: GLenum = GLenum(0x8404);

pub const GL_MAX_FRAGMENT_PROGRAM_LOCAL_PARAMETERS_NV: GLenum = GLenum(0x8868);

///
/// * Group: [`GetPName`]
pub const GL_MAX_FRAGMENT_SHADER_STORAGE_BLOCKS: GLenum = GLenum(0x90DA);

///
/// * Group: [`GetPName`]
pub const GL_MAX_FRAGMENT_UNIFORM_BLOCKS: GLenum = GLenum(0x8A2D);

///
/// * Group: [`GetPName`]
pub const GL_MAX_FRAGMENT_UNIFORM_COMPONENTS: GLenum = GLenum(0x8B49);

pub const GL_MAX_FRAGMENT_UNIFORM_COMPONENTS_ARB: GLenum = GLenum(0x8B49);

///
/// * Group: [`GetPName`]
pub const GL_MAX_FRAGMENT_UNIFORM_VECTORS: GLenum = GLenum(0x8DFD);

///
/// * Group: [`GetPName`]
pub const GL_MAX_FRAMEBUFFER_HEIGHT: GLenum = GLenum(0x9316);

///
/// * Group: [`GetPName`]
pub const GL_MAX_FRAMEBUFFER_LAYERS: GLenum = GLenum(0x9317);

pub const GL_MAX_FRAMEBUFFER_LAYERS_EXT: GLenum = GLenum(0x9317);

pub const GL_MAX_FRAMEBUFFER_LAYERS_OES: GLenum = GLenum(0x9317);

///
/// * Group: [`GetPName`]
pub const GL_MAX_FRAMEBUFFER_SAMPLES: GLenum = GLenum(0x9318);

///
/// * Group: [`GetPName`]
pub const GL_MAX_FRAMEBUFFER_WIDTH: GLenum = GLenum(0x9315);

///
/// * Group: [`GetPName`]
pub const GL_MAX_FRAMEZOOM_FACTOR_SGIX: GLenum = GLenum(0x818D);

pub const GL_MAX_GENERAL_COMBINERS_NV: GLenum = GLenum(0x854D);

///
/// * Group: [`GetPName`]
pub const GL_MAX_GEOMETRY_ATOMIC_COUNTERS: GLenum = GLenum(0x92D5);

pub const GL_MAX_GEOMETRY_ATOMIC_COUNTERS_EXT: GLenum = GLenum(0x92D5);

pub const GL_MAX_GEOMETRY_ATOMIC_COUNTERS_OES: GLenum = GLenum(0x92D5);

pub const GL_MAX_GEOMETRY_ATOMIC_COUNTER_BUFFERS: GLenum = GLenum(0x92CF);

pub const GL_MAX_GEOMETRY_ATOMIC_COUNTER_BUFFERS_EXT: GLenum = GLenum(0x92CF);

pub const GL_MAX_GEOMETRY_ATOMIC_COUNTER_BUFFERS_OES: GLenum = GLenum(0x92CF);

pub const GL_MAX_GEOMETRY_BINDABLE_UNIFORMS_EXT: GLenum = GLenum(0x8DE4);

pub const GL_MAX_GEOMETRY_IMAGE_UNIFORMS: GLenum = GLenum(0x90CD);

pub const GL_MAX_GEOMETRY_IMAGE_UNIFORMS_EXT: GLenum = GLenum(0x90CD);

pub const GL_MAX_GEOMETRY_IMAGE_UNIFORMS_OES: GLenum = GLenum(0x90CD);

///
/// * Group: [`GetPName`]
pub const GL_MAX_GEOMETRY_INPUT_COMPONENTS: GLenum = GLenum(0x9123);

pub const GL_MAX_GEOMETRY_INPUT_COMPONENTS_EXT: GLenum = GLenum(0x9123);

pub const GL_MAX_GEOMETRY_INPUT_COMPONENTS_OES: GLenum = GLenum(0x9123);

///
/// * Group: [`GetPName`]
pub const GL_MAX_GEOMETRY_OUTPUT_COMPONENTS: GLenum = GLenum(0x9124);

pub const GL_MAX_GEOMETRY_OUTPUT_COMPONENTS_EXT: GLenum = GLenum(0x9124);

pub const GL_MAX_GEOMETRY_OUTPUT_COMPONENTS_OES: GLenum = GLenum(0x9124);

pub const GL_MAX_GEOMETRY_OUTPUT_VERTICES: GLenum = GLenum(0x8DE0);

pub const GL_MAX_GEOMETRY_OUTPUT_VERTICES_ARB: GLenum = GLenum(0x8DE0);

pub const GL_MAX_GEOMETRY_OUTPUT_VERTICES_EXT: GLenum = GLenum(0x8DE0);

pub const GL_MAX_GEOMETRY_OUTPUT_VERTICES_OES: GLenum = GLenum(0x8DE0);

pub const GL_MAX_GEOMETRY_PROGRAM_INVOCATIONS_NV: GLenum = GLenum(0x8E5A);

pub const GL_MAX_GEOMETRY_SHADER_INVOCATIONS: GLenum = GLenum(0x8E5A);

pub const GL_MAX_GEOMETRY_SHADER_INVOCATIONS_EXT: GLenum = GLenum(0x8E5A);

pub const GL_MAX_GEOMETRY_SHADER_INVOCATIONS_OES: GLenum = GLenum(0x8E5A);

///
/// * Group: [`GetPName`]
pub const GL_MAX_GEOMETRY_SHADER_STORAGE_BLOCKS: GLenum = GLenum(0x90D7);

pub const GL_MAX_GEOMETRY_SHADER_STORAGE_BLOCKS_EXT: GLenum = GLenum(0x90D7);

pub const GL_MAX_GEOMETRY_SHADER_STORAGE_BLOCKS_OES: GLenum = GLenum(0x90D7);

///
/// * Group: [`GetPName`]
pub const GL_MAX_GEOMETRY_TEXTURE_IMAGE_UNITS: GLenum = GLenum(0x8C29);

pub const GL_MAX_GEOMETRY_TEXTURE_IMAGE_UNITS_ARB: GLenum = GLenum(0x8C29);

pub const GL_MAX_GEOMETRY_TEXTURE_IMAGE_UNITS_EXT: GLenum = GLenum(0x8C29);

pub const GL_MAX_GEOMETRY_TEXTURE_IMAGE_UNITS_OES: GLenum = GLenum(0x8C29);

pub const GL_MAX_GEOMETRY_TOTAL_OUTPUT_COMPONENTS: GLenum = GLenum(0x8DE1);

pub const GL_MAX_GEOMETRY_TOTAL_OUTPUT_COMPONENTS_ARB: GLenum = GLenum(0x8DE1);

pub const GL_MAX_GEOMETRY_TOTAL_OUTPUT_COMPONENTS_EXT: GLenum = GLenum(0x8DE1);

pub const GL_MAX_GEOMETRY_TOTAL_OUTPUT_COMPONENTS_OES: GLenum = GLenum(0x8DE1);

///
/// * Group: [`GetPName`]
pub const GL_MAX_GEOMETRY_UNIFORM_BLOCKS: GLenum = GLenum(0x8A2C);

pub const GL_MAX_GEOMETRY_UNIFORM_BLOCKS_EXT: GLenum = GLenum(0x8A2C);

pub const GL_MAX_GEOMETRY_UNIFORM_BLOCKS_OES: GLenum = GLenum(0x8A2C);

///
/// * Group: [`GetPName`]
pub const GL_MAX_GEOMETRY_UNIFORM_COMPONENTS: GLenum = GLenum(0x8DDF);

pub const GL_MAX_GEOMETRY_UNIFORM_COMPONENTS_ARB: GLenum = GLenum(0x8DDF);

pub const GL_MAX_GEOMETRY_UNIFORM_COMPONENTS_EXT: GLenum = GLenum(0x8DDF);

pub const GL_MAX_GEOMETRY_UNIFORM_COMPONENTS_OES: GLenum = GLenum(0x8DDF);

pub const GL_MAX_GEOMETRY_VARYING_COMPONENTS_ARB: GLenum = GLenum(0x8DDD);

pub const GL_MAX_GEOMETRY_VARYING_COMPONENTS_EXT: GLenum = GLenum(0x8DDD);

///
/// * Group: [`InternalFormatPName`]
pub const GL_MAX_HEIGHT: GLenum = GLenum(0x827F);

pub const GL_MAX_IMAGE_SAMPLES: GLenum = GLenum(0x906D);

pub const GL_MAX_IMAGE_SAMPLES_EXT: GLenum = GLenum(0x906D);

pub const GL_MAX_IMAGE_UNITS: GLenum = GLenum(0x8F38);

pub const GL_MAX_IMAGE_UNITS_EXT: GLenum = GLenum(0x8F38);

///
/// * Group: [`GetPName`]
pub const GL_MAX_INTEGER_SAMPLES: GLenum = GLenum(0x9110);

///
/// * Group: [`GetPName`]
pub const GL_MAX_LABEL_LENGTH: GLenum = GLenum(0x82E8);

pub const GL_MAX_LABEL_LENGTH_KHR: GLenum = GLenum(0x82E8);

///
/// * Group: [`InternalFormatPName`]
pub const GL_MAX_LAYERS: GLenum = GLenum(0x8281);

pub const GL_MAX_LGPU_GPUS_NVX: GLenum = GLenum(0x92BA);

///
/// * Group: [`GetPName`]
pub const GL_MAX_LIGHTS: GLenum = GLenum(0x0D31);

///
/// * Group: [`GetPName`]
pub const GL_MAX_LIST_NESTING: GLenum = GLenum(0x0B31);

pub const GL_MAX_MAP_TESSELLATION_NV: GLenum = GLenum(0x86D6);

pub const GL_MAX_MATRIX_PALETTE_STACK_DEPTH_ARB: GLenum = GLenum(0x8841);

pub const GL_MAX_MESH_ATOMIC_COUNTERS_NV: GLenum = GLenum(0x8E65);

pub const GL_MAX_MESH_ATOMIC_COUNTER_BUFFERS_NV: GLenum = GLenum(0x8E64);

pub const GL_MAX_MESH_IMAGE_UNIFORMS_NV: GLenum = GLenum(0x8E62);

pub const GL_MAX_MESH_OUTPUT_PRIMITIVES_NV: GLenum = GLenum(0x9539);

pub const GL_MAX_MESH_OUTPUT_VERTICES_NV: GLenum = GLenum(0x9538);

pub const GL_MAX_MESH_SHADER_STORAGE_BLOCKS_NV: GLenum = GLenum(0x8E66);

pub const GL_MAX_MESH_TEXTURE_IMAGE_UNITS_NV: GLenum = GLenum(0x8E61);

pub const GL_MAX_MESH_TOTAL_MEMORY_SIZE_NV: GLenum = GLenum(0x9536);

pub const GL_MAX_MESH_UNIFORM_BLOCKS_NV: GLenum = GLenum(0x8E60);

pub const GL_MAX_MESH_UNIFORM_COMPONENTS_NV: GLenum = GLenum(0x8E63);

pub const GL_MAX_MESH_VIEWS_NV: GLenum = GLenum(0x9557);

pub const GL_MAX_MESH_WORK_GROUP_INVOCATIONS_NV: GLenum = GLenum(0x95A2);

pub const GL_MAX_MESH_WORK_GROUP_SIZE_NV: GLenum = GLenum(0x953B);

///
/// * Group: [`GetPName`]
pub const GL_MAX_MODELVIEW_STACK_DEPTH: GLenum = GLenum(0x0D36);

pub const GL_MAX_MULTISAMPLE_COVERAGE_MODES_NV: GLenum = GLenum(0x8E11);

pub const GL_MAX_MULTIVIEW_BUFFERS_EXT: GLenum = GLenum(0x90F2);

///
/// * Group: [`ProgramInterfacePName`]
pub const GL_MAX_NAME_LENGTH: GLenum = GLenum(0x92F6);

///
/// * Group: [`GetPName`]
pub const GL_MAX_NAME_STACK_DEPTH: GLenum = GLenum(0x0D37);

///
/// * Group: [`ProgramInterfacePName`]
pub const GL_MAX_NUM_ACTIVE_VARIABLES: GLenum = GLenum(0x92F7);

///
/// * Group: [`ProgramInterfacePName`]
pub const GL_MAX_NUM_COMPATIBLE_SUBROUTINES: GLenum = GLenum(0x92F8);

pub const GL_MAX_OPTIMIZED_VERTEX_SHADER_INSTRUCTIONS_EXT: GLenum = GLenum(0x87CA);

pub const GL_MAX_OPTIMIZED_VERTEX_SHADER_INVARIANTS_EXT: GLenum = GLenum(0x87CD);

pub const GL_MAX_OPTIMIZED_VERTEX_SHADER_LOCALS_EXT: GLenum = GLenum(0x87CE);

pub const GL_MAX_OPTIMIZED_VERTEX_SHADER_LOCAL_CONSTANTS_EXT: GLenum = GLenum(0x87CC);

pub const GL_MAX_OPTIMIZED_VERTEX_SHADER_VARIANTS_EXT: GLenum = GLenum(0x87CB);

pub const GL_MAX_PALETTE_MATRICES_ARB: GLenum = GLenum(0x8842);

pub const GL_MAX_PALETTE_MATRICES_OES: GLenum = GLenum(0x8842);

pub const GL_MAX_PATCH_VERTICES: GLenum = GLenum(0x8E7D);

pub const GL_MAX_PATCH_VERTICES_EXT: GLenum = GLenum(0x8E7D);

pub const GL_MAX_PATCH_VERTICES_OES: GLenum = GLenum(0x8E7D);

///
/// * Group: [`GetPName`]
pub const GL_MAX_PIXEL_MAP_TABLE: GLenum = GLenum(0x0D34);

pub const GL_MAX_PIXEL_TRANSFORM_2D_STACK_DEPTH_EXT: GLenum = GLenum(0x8337);

pub const GL_MAX_PN_TRIANGLES_TESSELATION_LEVEL_ATI: GLenum = GLenum(0x87F1);

pub const GL_MAX_PROGRAM_ADDRESS_REGISTERS_ARB: GLenum = GLenum(0x88B1);

pub const GL_MAX_PROGRAM_ALU_INSTRUCTIONS_ARB: GLenum = GLenum(0x880B);

pub const GL_MAX_PROGRAM_ATTRIBS_ARB: GLenum = GLenum(0x88AD);

pub const GL_MAX_PROGRAM_ATTRIB_COMPONENTS_NV: GLenum = GLenum(0x8908);

pub const GL_MAX_PROGRAM_CALL_DEPTH_NV: GLenum = GLenum(0x88F5);

pub const GL_MAX_PROGRAM_ENV_PARAMETERS_ARB: GLenum = GLenum(0x88B5);

pub const GL_MAX_PROGRAM_EXEC_INSTRUCTIONS_NV: GLenum = GLenum(0x88F4);

pub const GL_MAX_PROGRAM_GENERIC_ATTRIBS_NV: GLenum = GLenum(0x8DA5);

pub const GL_MAX_PROGRAM_GENERIC_RESULTS_NV: GLenum = GLenum(0x8DA6);

pub const GL_MAX_PROGRAM_IF_DEPTH_NV: GLenum = GLenum(0x88F6);

pub const GL_MAX_PROGRAM_INSTRUCTIONS_ARB: GLenum = GLenum(0x88A1);

pub const GL_MAX_PROGRAM_LOCAL_PARAMETERS_ARB: GLenum = GLenum(0x88B4);

pub const GL_MAX_PROGRAM_LOOP_COUNT_NV: GLenum = GLenum(0x88F8);

pub const GL_MAX_PROGRAM_LOOP_DEPTH_NV: GLenum = GLenum(0x88F7);

pub const GL_MAX_PROGRAM_MATRICES_ARB: GLenum = GLenum(0x862F);

pub const GL_MAX_PROGRAM_MATRIX_STACK_DEPTH_ARB: GLenum = GLenum(0x862E);

pub const GL_MAX_PROGRAM_NATIVE_ADDRESS_REGISTERS_ARB: GLenum = GLenum(0x88B3);

pub const GL_MAX_PROGRAM_NATIVE_ALU_INSTRUCTIONS_ARB: GLenum = GLenum(0x880E);

pub const GL_MAX_PROGRAM_NATIVE_ATTRIBS_ARB: GLenum = GLenum(0x88AF);

pub const GL_MAX_PROGRAM_NATIVE_INSTRUCTIONS_ARB: GLenum = GLenum(0x88A3);

pub const GL_MAX_PROGRAM_NATIVE_PARAMETERS_ARB: GLenum = GLenum(0x88AB);

pub const GL_MAX_PROGRAM_NATIVE_TEMPORARIES_ARB: GLenum = GLenum(0x88A7);

pub const GL_MAX_PROGRAM_NATIVE_TEX_INDIRECTIONS_ARB: GLenum = GLenum(0x8810);

pub const GL_MAX_PROGRAM_NATIVE_TEX_INSTRUCTIONS_ARB: GLenum = GLenum(0x880F);

pub const GL_MAX_PROGRAM_OUTPUT_VERTICES_NV: GLenum = GLenum(0x8C27);

pub const GL_MAX_PROGRAM_PARAMETERS_ARB: GLenum = GLenum(0x88A9);

pub const GL_MAX_PROGRAM_PARAMETER_BUFFER_BINDINGS_NV: GLenum = GLenum(0x8DA0);

pub const GL_MAX_PROGRAM_PARAMETER_BUFFER_SIZE_NV: GLenum = GLenum(0x8DA1);

pub const GL_MAX_PROGRAM_PATCH_ATTRIBS_NV: GLenum = GLenum(0x86D8);

pub const GL_MAX_PROGRAM_RESULT_COMPONENTS_NV: GLenum = GLenum(0x8909);

pub const GL_MAX_PROGRAM_SUBROUTINE_NUM_NV: GLenum = GLenum(0x8F45);

pub const GL_MAX_PROGRAM_SUBROUTINE_PARAMETERS_NV: GLenum = GLenum(0x8F44);

pub const GL_MAX_PROGRAM_TEMPORARIES_ARB: GLenum = GLenum(0x88A5);

///
/// * Group: [`GetPName`]
pub const GL_MAX_PROGRAM_TEXEL_OFFSET: GLenum = GLenum(0x8905);

pub const GL_MAX_PROGRAM_TEXEL_OFFSET_EXT: GLenum = GLenum(0x8905);

pub const GL_MAX_PROGRAM_TEXEL_OFFSET_NV: GLenum = GLenum(0x8905);

pub const GL_MAX_PROGRAM_TEXTURE_GATHER_COMPONENTS_ARB: GLenum = GLenum(0x8F9F);

pub const GL_MAX_PROGRAM_TEXTURE_GATHER_OFFSET: GLenum = GLenum(0x8E5F);

pub const GL_MAX_PROGRAM_TEXTURE_GATHER_OFFSET_ARB: GLenum = GLenum(0x8E5F);

pub const GL_MAX_PROGRAM_TEXTURE_GATHER_OFFSET_NV: GLenum = GLenum(0x8E5F);

pub const GL_MAX_PROGRAM_TEX_INDIRECTIONS_ARB: GLenum = GLenum(0x880D);

pub const GL_MAX_PROGRAM_TEX_INSTRUCTIONS_ARB: GLenum = GLenum(0x880C);

pub const GL_MAX_PROGRAM_TOTAL_OUTPUT_COMPONENTS_NV: GLenum = GLenum(0x8C28);

///
/// * Group: [`GetPName`]
pub const GL_MAX_PROJECTION_STACK_DEPTH: GLenum = GLenum(0x0D38);

pub const GL_MAX_RASTER_SAMPLES_EXT: GLenum = GLenum(0x9329);

pub const GL_MAX_RATIONAL_EVAL_ORDER_NV: GLenum = GLenum(0x86D7);

///
/// * Group: [`GetPName`]
pub const GL_MAX_RECTANGLE_TEXTURE_SIZE: GLenum = GLenum(0x84F8);

pub const GL_MAX_RECTANGLE_TEXTURE_SIZE_ARB: GLenum = GLenum(0x84F8);

pub const GL_MAX_RECTANGLE_TEXTURE_SIZE_NV: GLenum = GLenum(0x84F8);

///
/// * Group: [`GetPName`]
pub const GL_MAX_RENDERBUFFER_SIZE: GLenum = GLenum(0x84E8);

pub const GL_MAX_RENDERBUFFER_SIZE_EXT: GLenum = GLenum(0x84E8);

pub const GL_MAX_RENDERBUFFER_SIZE_OES: GLenum = GLenum(0x84E8);

pub const GL_MAX_SAMPLES: GLenum = GLenum(0x8D57);

pub const GL_MAX_SAMPLES_ANGLE: GLenum = GLenum(0x8D57);

pub const GL_MAX_SAMPLES_APPLE: GLenum = GLenum(0x8D57);

pub const GL_MAX_SAMPLES_EXT: GLenum = GLenum(0x8D57);

pub const GL_MAX_SAMPLES_IMG: GLenum = GLenum(0x9135);

pub const GL_MAX_SAMPLES_NV: GLenum = GLenum(0x8D57);

///
/// * Group: [`GetPName`]
pub const GL_MAX_SAMPLE_MASK_WORDS: GLenum = GLenum(0x8E59);

pub const GL_MAX_SAMPLE_MASK_WORDS_NV: GLenum = GLenum(0x8E59);

///
/// * Group: [`GetPName`]
pub const GL_MAX_SERVER_WAIT_TIMEOUT: GLenum = GLenum(0x9111);

pub const GL_MAX_SERVER_WAIT_TIMEOUT_APPLE: GLenum = GLenum(0x9111);

pub const GL_MAX_SHADER_BUFFER_ADDRESS_NV: GLenum = GLenum(0x8F35);

pub const GL_MAX_SHADER_COMBINED_LOCAL_STORAGE_FAST_SIZE_EXT: GLenum = GLenum(0x9650);

pub const GL_MAX_SHADER_COMBINED_LOCAL_STORAGE_SIZE_EXT: GLenum = GLenum(0x9651);

///
/// * Alias Of: [`GL_MAX_SHADER_COMPILER_THREADS_KHR`]
pub const GL_MAX_SHADER_COMPILER_THREADS_ARB: GLenum = GL_MAX_SHADER_COMPILER_THREADS_KHR;

pub const GL_MAX_SHADER_COMPILER_THREADS_KHR: GLenum = GLenum(0x91B0);

pub const GL_MAX_SHADER_PIXEL_LOCAL_STORAGE_FAST_SIZE_EXT: GLenum = GLenum(0x8F63);

pub const GL_MAX_SHADER_PIXEL_LOCAL_STORAGE_SIZE_EXT: GLenum = GLenum(0x8F67);

pub const GL_MAX_SHADER_STORAGE_BLOCK_SIZE: GLenum = GLenum(0x90DE);

///
/// * Group: [`GetPName`]
pub const GL_MAX_SHADER_STORAGE_BUFFER_BINDINGS: GLenum = GLenum(0x90DD);

pub const GL_MAX_SHADER_SUBSAMPLED_IMAGE_UNITS_QCOM: GLenum = GLenum(0x8FA1);

pub const GL_MAX_SHININESS_NV: GLenum = GLenum(0x8504);

pub const GL_MAX_SPARSE_3D_TEXTURE_SIZE_AMD: GLenum = GLenum(0x9199);

pub const GL_MAX_SPARSE_3D_TEXTURE_SIZE_ARB: GLenum = GLenum(0x9199);

pub const GL_MAX_SPARSE_3D_TEXTURE_SIZE_EXT: GLenum = GLenum(0x9199);

pub const GL_MAX_SPARSE_ARRAY_TEXTURE_LAYERS: GLenum = GLenum(0x919A);

pub const GL_MAX_SPARSE_ARRAY_TEXTURE_LAYERS_ARB: GLenum = GLenum(0x919A);

pub const GL_MAX_SPARSE_ARRAY_TEXTURE_LAYERS_EXT: GLenum = GLenum(0x919A);

pub const GL_MAX_SPARSE_TEXTURE_SIZE_AMD: GLenum = GLenum(0x9198);

pub const GL_MAX_SPARSE_TEXTURE_SIZE_ARB: GLenum = GLenum(0x9198);

pub const GL_MAX_SPARSE_TEXTURE_SIZE_EXT: GLenum = GLenum(0x9198);

pub const GL_MAX_SPOT_EXPONENT_NV: GLenum = GLenum(0x8505);

pub const GL_MAX_SUBPIXEL_PRECISION_BIAS_BITS_NV: GLenum = GLenum(0x9349);

pub const GL_MAX_SUBROUTINES: GLenum = GLenum(0x8DE7);

pub const GL_MAX_SUBROUTINE_UNIFORM_LOCATIONS: GLenum = GLenum(0x8DE8);

pub const GL_MAX_TASK_ATOMIC_COUNTERS_NV: GLenum = GLenum(0x8E6D);

pub const GL_MAX_TASK_ATOMIC_COUNTER_BUFFERS_NV: GLenum = GLenum(0x8E6C);

pub const GL_MAX_TASK_IMAGE_UNIFORMS_NV: GLenum = GLenum(0x8E6A);

pub const GL_MAX_TASK_OUTPUT_COUNT_NV: GLenum = GLenum(0x953A);

pub const GL_MAX_TASK_SHADER_STORAGE_BLOCKS_NV: GLenum = GLenum(0x8E6E);

pub const GL_MAX_TASK_TEXTURE_IMAGE_UNITS_NV: GLenum = GLenum(0x8E69);

pub const GL_MAX_TASK_TOTAL_MEMORY_SIZE_NV: GLenum = GLenum(0x9537);

pub const GL_MAX_TASK_UNIFORM_BLOCKS_NV: GLenum = GLenum(0x8E68);

pub const GL_MAX_TASK_UNIFORM_COMPONENTS_NV: GLenum = GLenum(0x8E6B);

pub const GL_MAX_TASK_WORK_GROUP_INVOCATIONS_NV: GLenum = GLenum(0x95A3);

pub const GL_MAX_TASK_WORK_GROUP_SIZE_NV: GLenum = GLenum(0x953C);

///
/// * Group: [`GetPName`]
pub const GL_MAX_TESS_CONTROL_ATOMIC_COUNTERS: GLenum = GLenum(0x92D3);

pub const GL_MAX_TESS_CONTROL_ATOMIC_COUNTERS_EXT: GLenum = GLenum(0x92D3);

pub const GL_MAX_TESS_CONTROL_ATOMIC_COUNTERS_OES: GLenum = GLenum(0x92D3);

pub const GL_MAX_TESS_CONTROL_ATOMIC_COUNTER_BUFFERS: GLenum = GLenum(0x92CD);

pub const GL_MAX_TESS_CONTROL_ATOMIC_COUNTER_BUFFERS_EXT: GLenum = GLenum(0x92CD);

pub const GL_MAX_TESS_CONTROL_ATOMIC_COUNTER_BUFFERS_OES: GLenum = GLenum(0x92CD);

pub const GL_MAX_TESS_CONTROL_IMAGE_UNIFORMS: GLenum = GLenum(0x90CB);

pub const GL_MAX_TESS_CONTROL_IMAGE_UNIFORMS_EXT: GLenum = GLenum(0x90CB);

pub const GL_MAX_TESS_CONTROL_IMAGE_UNIFORMS_OES: GLenum = GLenum(0x90CB);

pub const GL_MAX_TESS_CONTROL_INPUT_COMPONENTS: GLenum = GLenum(0x886C);

pub const GL_MAX_TESS_CONTROL_INPUT_COMPONENTS_EXT: GLenum = GLenum(0x886C);

pub const GL_MAX_TESS_CONTROL_INPUT_COMPONENTS_OES: GLenum = GLenum(0x886C);

pub const GL_MAX_TESS_CONTROL_OUTPUT_COMPONENTS: GLenum = GLenum(0x8E83);

pub const GL_MAX_TESS_CONTROL_OUTPUT_COMPONENTS_EXT: GLenum = GLenum(0x8E83);

pub const GL_MAX_TESS_CONTROL_OUTPUT_COMPONENTS_OES: GLenum = GLenum(0x8E83);

///
/// * Group: [`GetPName`]
pub const GL_MAX_TESS_CONTROL_SHADER_STORAGE_BLOCKS: GLenum = GLenum(0x90D8);

pub const GL_MAX_TESS_CONTROL_SHADER_STORAGE_BLOCKS_EXT: GLenum = GLenum(0x90D8);

pub const GL_MAX_TESS_CONTROL_SHADER_STORAGE_BLOCKS_OES: GLenum = GLenum(0x90D8);

pub const GL_MAX_TESS_CONTROL_TEXTURE_IMAGE_UNITS: GLenum = GLenum(0x8E81);

pub const GL_MAX_TESS_CONTROL_TEXTURE_IMAGE_UNITS_EXT: GLenum = GLenum(0x8E81);

pub const GL_MAX_TESS_CONTROL_TEXTURE_IMAGE_UNITS_OES: GLenum = GLenum(0x8E81);

pub const GL_MAX_TESS_CONTROL_TOTAL_OUTPUT_COMPONENTS: GLenum = GLenum(0x8E85);

pub const GL_MAX_TESS_CONTROL_TOTAL_OUTPUT_COMPONENTS_EXT: GLenum = GLenum(0x8E85);

pub const GL_MAX_TESS_CONTROL_TOTAL_OUTPUT_COMPONENTS_OES: GLenum = GLenum(0x8E85);

///
/// * Group: [`GetPName`]
pub const GL_MAX_TESS_CONTROL_UNIFORM_BLOCKS: GLenum = GLenum(0x8E89);

pub const GL_MAX_TESS_CONTROL_UNIFORM_BLOCKS_EXT: GLenum = GLenum(0x8E89);

pub const GL_MAX_TESS_CONTROL_UNIFORM_BLOCKS_OES: GLenum = GLenum(0x8E89);

pub const GL_MAX_TESS_CONTROL_UNIFORM_COMPONENTS: GLenum = GLenum(0x8E7F);

pub const GL_MAX_TESS_CONTROL_UNIFORM_COMPONENTS_EXT: GLenum = GLenum(0x8E7F);

pub const GL_MAX_TESS_CONTROL_UNIFORM_COMPONENTS_OES: GLenum = GLenum(0x8E7F);

///
/// * Group: [`GetPName`]
pub const GL_MAX_TESS_EVALUATION_ATOMIC_COUNTERS: GLenum = GLenum(0x92D4);

pub const GL_MAX_TESS_EVALUATION_ATOMIC_COUNTERS_EXT: GLenum = GLenum(0x92D4);

pub const GL_MAX_TESS_EVALUATION_ATOMIC_COUNTERS_OES: GLenum = GLenum(0x92D4);

pub const GL_MAX_TESS_EVALUATION_ATOMIC_COUNTER_BUFFERS: GLenum = GLenum(0x92CE);

pub const GL_MAX_TESS_EVALUATION_ATOMIC_COUNTER_BUFFERS_EXT: GLenum = GLenum(0x92CE);

pub const GL_MAX_TESS_EVALUATION_ATOMIC_COUNTER_BUFFERS_OES: GLenum = GLenum(0x92CE);

pub const GL_MAX_TESS_EVALUATION_IMAGE_UNIFORMS: GLenum = GLenum(0x90CC);

pub const GL_MAX_TESS_EVALUATION_IMAGE_UNIFORMS_EXT: GLenum = GLenum(0x90CC);

pub const GL_MAX_TESS_EVALUATION_IMAGE_UNIFORMS_OES: GLenum = GLenum(0x90CC);

pub const GL_MAX_TESS_EVALUATION_INPUT_COMPONENTS: GLenum = GLenum(0x886D);

pub const GL_MAX_TESS_EVALUATION_INPUT_COMPONENTS_EXT: GLenum = GLenum(0x886D);

pub const GL_MAX_TESS_EVALUATION_INPUT_COMPONENTS_OES: GLenum = GLenum(0x886D);

pub const GL_MAX_TESS_EVALUATION_OUTPUT_COMPONENTS: GLenum = GLenum(0x8E86);

pub const GL_MAX_TESS_EVALUATION_OUTPUT_COMPONENTS_EXT: GLenum = GLenum(0x8E86);

pub const GL_MAX_TESS_EVALUATION_OUTPUT_COMPONENTS_OES: GLenum = GLenum(0x8E86);

///
/// * Group: [`GetPName`]
pub const GL_MAX_TESS_EVALUATION_SHADER_STORAGE_BLOCKS: GLenum = GLenum(0x90D9);

pub const GL_MAX_TESS_EVALUATION_SHADER_STORAGE_BLOCKS_EXT: GLenum = GLenum(0x90D9);

pub const GL_MAX_TESS_EVALUATION_SHADER_STORAGE_BLOCKS_OES: GLenum = GLenum(0x90D9);

pub const GL_MAX_TESS_EVALUATION_TEXTURE_IMAGE_UNITS: GLenum = GLenum(0x8E82);

pub const GL_MAX_TESS_EVALUATION_TEXTURE_IMAGE_UNITS_EXT: GLenum = GLenum(0x8E82);

pub const GL_MAX_TESS_EVALUATION_TEXTURE_IMAGE_UNITS_OES: GLenum = GLenum(0x8E82);

///
/// * Group: [`GetPName`]
pub const GL_MAX_TESS_EVALUATION_UNIFORM_BLOCKS: GLenum = GLenum(0x8E8A);

pub const GL_MAX_TESS_EVALUATION_UNIFORM_BLOCKS_EXT: GLenum = GLenum(0x8E8A);

pub const GL_MAX_TESS_EVALUATION_UNIFORM_BLOCKS_OES: GLenum = GLenum(0x8E8A);

pub const GL_MAX_TESS_EVALUATION_UNIFORM_COMPONENTS: GLenum = GLenum(0x8E80);

pub const GL_MAX_TESS_EVALUATION_UNIFORM_COMPONENTS_EXT: GLenum = GLenum(0x8E80);

pub const GL_MAX_TESS_EVALUATION_UNIFORM_COMPONENTS_OES: GLenum = GLenum(0x8E80);

pub const GL_MAX_TESS_GEN_LEVEL: GLenum = GLenum(0x8E7E);

pub const GL_MAX_TESS_GEN_LEVEL_EXT: GLenum = GLenum(0x8E7E);

pub const GL_MAX_TESS_GEN_LEVEL_OES: GLenum = GLenum(0x8E7E);

pub const GL_MAX_TESS_PATCH_COMPONENTS: GLenum = GLenum(0x8E84);

pub const GL_MAX_TESS_PATCH_COMPONENTS_EXT: GLenum = GLenum(0x8E84);

pub const GL_MAX_TESS_PATCH_COMPONENTS_OES: GLenum = GLenum(0x8E84);

///
/// * Group: [`GetPName`]
pub const GL_MAX_TEXTURE_BUFFER_SIZE: GLenum = GLenum(0x8C2B);

pub const GL_MAX_TEXTURE_BUFFER_SIZE_ARB: GLenum = GLenum(0x8C2B);

pub const GL_MAX_TEXTURE_BUFFER_SIZE_EXT: GLenum = GLenum(0x8C2B);

pub const GL_MAX_TEXTURE_BUFFER_SIZE_OES: GLenum = GLenum(0x8C2B);

pub const GL_MAX_TEXTURE_COORDS: GLenum = GLenum(0x8871);

pub const GL_MAX_TEXTURE_COORDS_ARB: GLenum = GLenum(0x8871);

pub const GL_MAX_TEXTURE_COORDS_NV: GLenum = GLenum(0x8871);

///
/// * Group: [`GetPName`]
pub const GL_MAX_TEXTURE_IMAGE_UNITS: GLenum = GLenum(0x8872);

pub const GL_MAX_TEXTURE_IMAGE_UNITS_ARB: GLenum = GLenum(0x8872);

pub const GL_MAX_TEXTURE_IMAGE_UNITS_NV: GLenum = GLenum(0x8872);

///
/// * Group: [`GetPName`]
pub const GL_MAX_TEXTURE_LOD_BIAS: GLenum = GLenum(0x84FD);

pub const GL_MAX_TEXTURE_LOD_BIAS_EXT: GLenum = GLenum(0x84FD);

pub const GL_MAX_TEXTURE_MAX_ANISOTROPY: GLenum = GLenum(0x84FF);

///
/// * Alias Of: [`GL_MAX_TEXTURE_MAX_ANISOTROPY`]
pub const GL_MAX_TEXTURE_MAX_ANISOTROPY_EXT: GLenum = GL_MAX_TEXTURE_MAX_ANISOTROPY;

///
/// * Group: [`GetPName`]
pub const GL_MAX_TEXTURE_SIZE: GLenum = GLenum(0x0D33);

///
/// * Group: [`GetPName`]
pub const GL_MAX_TEXTURE_STACK_DEPTH: GLenum = GLenum(0x0D39);

pub const GL_MAX_TEXTURE_UNITS: GLenum = GLenum(0x84E2);

pub const GL_MAX_TEXTURE_UNITS_ARB: GLenum = GLenum(0x84E2);

///
/// * Group: [`GetPName`]
pub const GL_MAX_TIMELINE_SEMAPHORE_VALUE_DIFFERENCE_NV: GLenum = GLenum(0x95B6);

pub const GL_MAX_TRACK_MATRICES_NV: GLenum = GLenum(0x862F);

pub const GL_MAX_TRACK_MATRIX_STACK_DEPTH_NV: GLenum = GLenum(0x862E);

pub const GL_MAX_TRANSFORM_FEEDBACK_BUFFERS: GLenum = GLenum(0x8E70);

pub const GL_MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS: GLenum = GLenum(0x8C8A);

pub const GL_MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS_EXT: GLenum = GLenum(0x8C8A);

pub const GL_MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS_NV: GLenum = GLenum(0x8C8A);

pub const GL_MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS: GLenum = GLenum(0x8C8B);

pub const GL_MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS_EXT: GLenum = GLenum(0x8C8B);

pub const GL_MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS_NV: GLenum = GLenum(0x8C8B);

pub const GL_MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS: GLenum = GLenum(0x8C80);

pub const GL_MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS_EXT: GLenum = GLenum(0x8C80);

pub const GL_MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS_NV: GLenum = GLenum(0x8C80);

///
/// * Group: [`GetPName`]
pub const GL_MAX_UNIFORM_BLOCK_SIZE: GLenum = GLenum(0x8A30);

///
/// * Group: [`GetPName`]
pub const GL_MAX_UNIFORM_BUFFER_BINDINGS: GLenum = GLenum(0x8A2F);

///
/// * Group: [`GetPName`]
pub const GL_MAX_UNIFORM_LOCATIONS: GLenum = GLenum(0x826E);

///
/// * Group: [`GetPName`]
/// * Alias Of: [`GL_MAX_VARYING_FLOATS`]
pub const GL_MAX_VARYING_COMPONENTS: GLenum = GL_MAX_VARYING_FLOATS;

pub const GL_MAX_VARYING_COMPONENTS_EXT: GLenum = GLenum(0x8B4B);

///
/// * Group: [`GetPName`]
pub const GL_MAX_VARYING_FLOATS: GLenum = GLenum(0x8B4B);

pub const GL_MAX_VARYING_FLOATS_ARB: GLenum = GLenum(0x8B4B);

///
/// * Group: [`GetPName`]
pub const GL_MAX_VARYING_VECTORS: GLenum = GLenum(0x8DFC);

pub const GL_MAX_VERTEX_ARRAY_RANGE_ELEMENT_NV: GLenum = GLenum(0x8520);

///
/// * Group: [`GetPName`]
pub const GL_MAX_VERTEX_ATOMIC_COUNTERS: GLenum = GLenum(0x92D2);

pub const GL_MAX_VERTEX_ATOMIC_COUNTER_BUFFERS: GLenum = GLenum(0x92CC);

///
/// * Group: [`GetPName`]
pub const GL_MAX_VERTEX_ATTRIBS: GLenum = GLenum(0x8869);

pub const GL_MAX_VERTEX_ATTRIBS_ARB: GLenum = GLenum(0x8869);

///
/// * Group: [`GetPName`]
pub const GL_MAX_VERTEX_ATTRIB_BINDINGS: GLenum = GLenum(0x82DA);

///
/// * Group: [`GetPName`]
pub const GL_MAX_VERTEX_ATTRIB_RELATIVE_OFFSET: GLenum = GLenum(0x82D9);

pub const GL_MAX_VERTEX_ATTRIB_STRIDE: GLenum = GLenum(0x82E5);

pub const GL_MAX_VERTEX_BINDABLE_UNIFORMS_EXT: GLenum = GLenum(0x8DE2);

///
/// * Group: [`HintTarget`], [`HintTargetPGI`]
pub const GL_MAX_VERTEX_HINT_PGI: GLenum = GLenum(0x1A22D);

pub const GL_MAX_VERTEX_IMAGE_UNIFORMS: GLenum = GLenum(0x90CA);

///
/// * Group: [`GetPName`]
pub const GL_MAX_VERTEX_OUTPUT_COMPONENTS: GLenum = GLenum(0x9122);

pub const GL_MAX_VERTEX_SHADER_INSTRUCTIONS_EXT: GLenum = GLenum(0x87C5);

pub const GL_MAX_VERTEX_SHADER_INVARIANTS_EXT: GLenum = GLenum(0x87C7);

pub const GL_MAX_VERTEX_SHADER_LOCALS_EXT: GLenum = GLenum(0x87C9);

pub const GL_MAX_VERTEX_SHADER_LOCAL_CONSTANTS_EXT: GLenum = GLenum(0x87C8);

///
/// * Group: [`GetPName`]
pub const GL_MAX_VERTEX_SHADER_STORAGE_BLOCKS: GLenum = GLenum(0x90D6);

pub const GL_MAX_VERTEX_SHADER_VARIANTS_EXT: GLenum = GLenum(0x87C6);

pub const GL_MAX_VERTEX_STREAMS: GLenum = GLenum(0x8E71);

pub const GL_MAX_VERTEX_STREAMS_ATI: GLenum = GLenum(0x876B);

///
/// * Group: [`GetPName`]
pub const GL_MAX_VERTEX_TEXTURE_IMAGE_UNITS: GLenum = GLenum(0x8B4C);

pub const GL_MAX_VERTEX_TEXTURE_IMAGE_UNITS_ARB: GLenum = GLenum(0x8B4C);

///
/// * Group: [`GetPName`]
pub const GL_MAX_VERTEX_UNIFORM_BLOCKS: GLenum = GLenum(0x8A2B);

///
/// * Group: [`GetPName`]
pub const GL_MAX_VERTEX_UNIFORM_COMPONENTS: GLenum = GLenum(0x8B4A);

pub const GL_MAX_VERTEX_UNIFORM_COMPONENTS_ARB: GLenum = GLenum(0x8B4A);

///
/// * Group: [`GetPName`]
pub const GL_MAX_VERTEX_UNIFORM_VECTORS: GLenum = GLenum(0x8DFB);

pub const GL_MAX_VERTEX_UNITS_ARB: GLenum = GLenum(0x86A4);

pub const GL_MAX_VERTEX_UNITS_OES: GLenum = GLenum(0x86A4);

pub const GL_MAX_VERTEX_VARYING_COMPONENTS_ARB: GLenum = GLenum(0x8DDE);

pub const GL_MAX_VERTEX_VARYING_COMPONENTS_EXT: GLenum = GLenum(0x8DDE);

///
/// * Group: [`GetPName`]
pub const GL_MAX_VIEWPORTS: GLenum = GLenum(0x825B);

pub const GL_MAX_VIEWPORTS_NV: GLenum = GLenum(0x825B);

pub const GL_MAX_VIEWPORTS_OES: GLenum = GLenum(0x825B);

///
/// * Group: [`GetPName`]
pub const GL_MAX_VIEWPORT_DIMS: GLenum = GLenum(0x0D3A);

pub const GL_MAX_VIEWS_OVR: GLenum = GLenum(0x9631);

///
/// * Group: [`InternalFormatPName`]
pub const GL_MAX_WIDTH: GLenum = GLenum(0x827E);

pub const GL_MAX_WINDOW_RECTANGLES_EXT: GLenum = GLenum(0x8F14);

///
/// * Group: [`PrecisionType`]
pub const GL_MEDIUM_FLOAT: GLenum = GLenum(0x8DF1);

///
/// * Group: [`PrecisionType`]
pub const GL_MEDIUM_INT: GLenum = GLenum(0x8DF4);

pub const GL_MEMORY_ATTACHABLE_ALIGNMENT_NV: GLenum = GLenum(0x95A6);

pub const GL_MEMORY_ATTACHABLE_NV: GLenum = GLenum(0x95A8);

pub const GL_MEMORY_ATTACHABLE_SIZE_NV: GLenum = GLenum(0x95A7);

pub const GL_MESH_OUTPUT_PER_PRIMITIVE_GRANULARITY_NV: GLenum = GLenum(0x9543);

pub const GL_MESH_OUTPUT_PER_VERTEX_GRANULARITY_NV: GLenum = GLenum(0x92DF);

pub const GL_MESH_OUTPUT_TYPE_NV: GLenum = GLenum(0x957B);

pub const GL_MESH_PRIMITIVES_OUT_NV: GLenum = GLenum(0x957A);

///
/// * Group: [`UseProgramStageMask`]
pub const GL_MESH_SHADER_BIT_NV: GLbitfield = GLbitfield(0x00000040);

pub const GL_MESH_SHADER_NV: GLenum = GLenum(0x9559);

pub const GL_MESH_SUBROUTINE_NV: GLenum = GLenum(0x957C);

pub const GL_MESH_SUBROUTINE_UNIFORM_NV: GLenum = GLenum(0x957E);

pub const GL_MESH_VERTICES_OUT_NV: GLenum = GLenum(0x9579);

pub const GL_MESH_WORK_GROUP_SIZE_NV: GLenum = GLenum(0x953E);

///
/// * Group: [`BlendEquationModeEXT`]
pub const GL_MIN: GLenum = GLenum(0x8007);

///
/// * Group: [`MinmaxTarget`], [`MinmaxTargetEXT`]
pub const GL_MINMAX: GLenum = GLenum(0x802E);

///
/// * Group: [`MinmaxTargetEXT`], [`EnableCap`], [`GetPName`]
pub const GL_MINMAX_EXT: GLenum = GLenum(0x802E);

///
/// * Group: [`GetMinmaxParameterPNameEXT`]
pub const GL_MINMAX_FORMAT: GLenum = GLenum(0x802F);

///
/// * Group: [`GetMinmaxParameterPNameEXT`]
pub const GL_MINMAX_FORMAT_EXT: GLenum = GLenum(0x802F);

///
/// * Group: [`GetMinmaxParameterPNameEXT`]
pub const GL_MINMAX_SINK: GLenum = GLenum(0x8030);

///
/// * Group: [`GetMinmaxParameterPNameEXT`]
pub const GL_MINMAX_SINK_EXT: GLenum = GLenum(0x8030);

///
/// * Group: [`GetPName`]
pub const GL_MINOR_VERSION: GLenum = GLenum(0x821C);

pub const GL_MINUS_CLAMPED_NV: GLenum = GLenum(0x92B3);

pub const GL_MINUS_NV: GLenum = GLenum(0x929F);

///
/// * Group: [`BlendEquationModeEXT`]
pub const GL_MIN_EXT: GLenum = GLenum(0x8007);

pub const GL_MIN_FRAGMENT_INTERPOLATION_OFFSET: GLenum = GLenum(0x8E5B);

pub const GL_MIN_FRAGMENT_INTERPOLATION_OFFSET_NV: GLenum = GLenum(0x8E5B);

pub const GL_MIN_FRAGMENT_INTERPOLATION_OFFSET_OES: GLenum = GLenum(0x8E5B);

pub const GL_MIN_LOD_WARNING_AMD: GLenum = GLenum(0x919C);

///
/// * Group: [`GetPName`]
pub const GL_MIN_MAP_BUFFER_ALIGNMENT: GLenum = GLenum(0x90BC);

///
/// * Group: [`GetPName`]
pub const GL_MIN_PROGRAM_TEXEL_OFFSET: GLenum = GLenum(0x8904);

pub const GL_MIN_PROGRAM_TEXEL_OFFSET_EXT: GLenum = GLenum(0x8904);

pub const GL_MIN_PROGRAM_TEXEL_OFFSET_NV: GLenum = GLenum(0x8904);

pub const GL_MIN_PROGRAM_TEXTURE_GATHER_OFFSET: GLenum = GLenum(0x8E5E);

pub const GL_MIN_PROGRAM_TEXTURE_GATHER_OFFSET_ARB: GLenum = GLenum(0x8E5E);

pub const GL_MIN_PROGRAM_TEXTURE_GATHER_OFFSET_NV: GLenum = GLenum(0x8E5E);

pub const GL_MIN_SAMPLE_SHADING_VALUE: GLenum = GLenum(0x8C37);

pub const GL_MIN_SAMPLE_SHADING_VALUE_ARB: GLenum = GLenum(0x8C37);

pub const GL_MIN_SAMPLE_SHADING_VALUE_OES: GLenum = GLenum(0x8C37);

pub const GL_MIN_SPARSE_LEVEL_AMD: GLenum = GLenum(0x919B);

///
/// * Group: [`InternalFormatPName`]
pub const GL_MIPMAP: GLenum = GLenum(0x8293);

///
/// * Group: [`TextureWrapMode`]
pub const GL_MIRRORED_REPEAT: GLenum = GLenum(0x8370);

pub const GL_MIRRORED_REPEAT_ARB: GLenum = GLenum(0x8370);

pub const GL_MIRRORED_REPEAT_IBM: GLenum = GLenum(0x8370);

pub const GL_MIRRORED_REPEAT_OES: GLenum = GLenum(0x8370);

pub const GL_MIRROR_CLAMP_ATI: GLenum = GLenum(0x8742);

pub const GL_MIRROR_CLAMP_EXT: GLenum = GLenum(0x8742);

pub const GL_MIRROR_CLAMP_TO_BORDER_EXT: GLenum = GLenum(0x8912);

pub const GL_MIRROR_CLAMP_TO_EDGE: GLenum = GLenum(0x8743);

pub const GL_MIRROR_CLAMP_TO_EDGE_ATI: GLenum = GLenum(0x8743);

pub const GL_MIRROR_CLAMP_TO_EDGE_EXT: GLenum = GLenum(0x8743);

pub const GL_MITER_REVERT_NV: GLenum = GLenum(0x90A7);

pub const GL_MITER_TRUNCATE_NV: GLenum = GLenum(0x90A8);

pub const GL_MIXED_DEPTH_SAMPLES_SUPPORTED_NV: GLenum = GLenum(0x932F);

pub const GL_MIXED_STENCIL_SAMPLES_SUPPORTED_NV: GLenum = GLenum(0x9330);

///
/// * Group: [`MatrixMode`]
pub const GL_MODELVIEW: GLenum = GLenum(0x1700);

pub const GL_MODELVIEW0_ARB: GLenum = GLenum(0x1700);

///
/// * Group: [`MatrixMode`]
pub const GL_MODELVIEW0_EXT: GLenum = GLenum(0x1700);

///
/// * Group: [`GetPName`]
pub const GL_MODELVIEW0_MATRIX_EXT: GLenum = GLenum(0x0BA6);

///
/// * Group: [`GetPName`]
pub const GL_MODELVIEW0_STACK_DEPTH_EXT: GLenum = GLenum(0x0BA3);

pub const GL_MODELVIEW10_ARB: GLenum = GLenum(0x872A);

pub const GL_MODELVIEW11_ARB: GLenum = GLenum(0x872B);

pub const GL_MODELVIEW12_ARB: GLenum = GLenum(0x872C);

pub const GL_MODELVIEW13_ARB: GLenum = GLenum(0x872D);

pub const GL_MODELVIEW14_ARB: GLenum = GLenum(0x872E);

pub const GL_MODELVIEW15_ARB: GLenum = GLenum(0x872F);

pub const GL_MODELVIEW16_ARB: GLenum = GLenum(0x8730);

pub const GL_MODELVIEW17_ARB: GLenum = GLenum(0x8731);

pub const GL_MODELVIEW18_ARB: GLenum = GLenum(0x8732);

pub const GL_MODELVIEW19_ARB: GLenum = GLenum(0x8733);

pub const GL_MODELVIEW1_ARB: GLenum = GLenum(0x850A);

pub const GL_MODELVIEW1_EXT: GLenum = GLenum(0x850A);

pub const GL_MODELVIEW1_MATRIX_EXT: GLenum = GLenum(0x8506);

pub const GL_MODELVIEW1_STACK_DEPTH_EXT: GLenum = GLenum(0x8502);

pub const GL_MODELVIEW20_ARB: GLenum = GLenum(0x8734);

pub const GL_MODELVIEW21_ARB: GLenum = GLenum(0x8735);

pub const GL_MODELVIEW22_ARB: GLenum = GLenum(0x8736);

pub const GL_MODELVIEW23_ARB: GLenum = GLenum(0x8737);

pub const GL_MODELVIEW24_ARB: GLenum = GLenum(0x8738);

pub const GL_MODELVIEW25_ARB: GLenum = GLenum(0x8739);

pub const GL_MODELVIEW26_ARB: GLenum = GLenum(0x873A);

pub const GL_MODELVIEW27_ARB: GLenum = GLenum(0x873B);

pub const GL_MODELVIEW28_ARB: GLenum = GLenum(0x873C);

pub const GL_MODELVIEW29_ARB: GLenum = GLenum(0x873D);

pub const GL_MODELVIEW2_ARB: GLenum = GLenum(0x8722);

pub const GL_MODELVIEW30_ARB: GLenum = GLenum(0x873E);

pub const GL_MODELVIEW31_ARB: GLenum = GLenum(0x873F);

pub const GL_MODELVIEW3_ARB: GLenum = GLenum(0x8723);

pub const GL_MODELVIEW4_ARB: GLenum = GLenum(0x8724);

pub const GL_MODELVIEW5_ARB: GLenum = GLenum(0x8725);

pub const GL_MODELVIEW6_ARB: GLenum = GLenum(0x8726);

pub const GL_MODELVIEW7_ARB: GLenum = GLenum(0x8727);

pub const GL_MODELVIEW8_ARB: GLenum = GLenum(0x8728);

pub const GL_MODELVIEW9_ARB: GLenum = GLenum(0x8729);

///
/// * Group: [`GetPName`]
pub const GL_MODELVIEW_MATRIX: GLenum = GLenum(0x0BA6);

pub const GL_MODELVIEW_MATRIX_FLOAT_AS_INT_BITS_OES: GLenum = GLenum(0x898D);

pub const GL_MODELVIEW_PROJECTION_NV: GLenum = GLenum(0x8629);

///
/// * Group: [`GetPName`]
pub const GL_MODELVIEW_STACK_DEPTH: GLenum = GLenum(0x0BA3);

///
/// * Group: [`TextureEnvMode`], [`LightEnvModeSGIX`]
pub const GL_MODULATE: GLenum = GLenum(0x2100);

pub const GL_MODULATE_ADD_ATI: GLenum = GLenum(0x8744);

pub const GL_MODULATE_COLOR_IMG: GLenum = GLenum(0x8C04);

pub const GL_MODULATE_SIGNED_ADD_ATI: GLenum = GLenum(0x8745);

pub const GL_MODULATE_SUBTRACT_ATI: GLenum = GLenum(0x8746);

///
/// * Group: [`GetPName`]
pub const GL_MOTION_ESTIMATION_SEARCH_BLOCK_X_QCOM: GLenum = GLenum(0x8C90);

///
/// * Group: [`GetPName`]
pub const GL_MOTION_ESTIMATION_SEARCH_BLOCK_Y_QCOM: GLenum = GLenum(0x8C91);

pub const GL_MOVE_TO_CONTINUES_NV: GLenum = GLenum(0x90B6);

///
/// * Group: [`PathCoordType`]
pub const GL_MOVE_TO_NV: GLenum = GLenum(0x02);

pub const GL_MOVE_TO_RESETS_NV: GLenum = GLenum(0x90B5);

///
/// * Group: [`FragmentOpATI`]
pub const GL_MOV_ATI: GLenum = GLenum(0x8961);

///
/// * Group: [`AccumOp`]
pub const GL_MULT: GLenum = GLenum(0x0103);

pub const GL_MULTICAST_GPUS_NV: GLenum = GLenum(0x92BA);

pub const GL_MULTICAST_PROGRAMMABLE_SAMPLE_LOCATION_NV: GLenum = GLenum(0x9549);

pub const GL_MULTIPLY: GLenum = GLenum(0x9294);

pub const GL_MULTIPLY_KHR: GLenum = GLenum(0x9294);

pub const GL_MULTIPLY_NV: GLenum = GLenum(0x9294);

///
/// * Group: [`EnableCap`]
pub const GL_MULTISAMPLE: GLenum = GLenum(0x809D);

pub const GL_MULTISAMPLES_NV: GLenum = GLenum(0x9371);

pub const GL_MULTISAMPLE_3DFX: GLenum = GLenum(0x86B2);

pub const GL_MULTISAMPLE_ARB: GLenum = GLenum(0x809D);

///
/// * Group: [`AttribMask`]
pub const GL_MULTISAMPLE_BIT: GLbitfield = GLbitfield(0x20000000);

///
/// * Group: [`AttribMask`]
pub const GL_MULTISAMPLE_BIT_3DFX: GLbitfield = GLbitfield(0x20000000);

///
/// * Group: [`AttribMask`]
pub const GL_MULTISAMPLE_BIT_ARB: GLbitfield = GLbitfield(0x20000000);

///
/// * Group: [`AttribMask`]
pub const GL_MULTISAMPLE_BIT_EXT: GLbitfield = GLbitfield(0x20000000);

///
/// * Group: [`BufferBitQCOM`]
pub const GL_MULTISAMPLE_BUFFER_BIT0_QCOM: GLbitfield = GLbitfield(0x01000000);

///
/// * Group: [`BufferBitQCOM`]
pub const GL_MULTISAMPLE_BUFFER_BIT1_QCOM: GLbitfield = GLbitfield(0x02000000);

///
/// * Group: [`BufferBitQCOM`]
pub const GL_MULTISAMPLE_BUFFER_BIT2_QCOM: GLbitfield = GLbitfield(0x04000000);

///
/// * Group: [`BufferBitQCOM`]
pub const GL_MULTISAMPLE_BUFFER_BIT3_QCOM: GLbitfield = GLbitfield(0x08000000);

///
/// * Group: [`BufferBitQCOM`]
pub const GL_MULTISAMPLE_BUFFER_BIT4_QCOM: GLbitfield = GLbitfield(0x10000000);

///
/// * Group: [`BufferBitQCOM`]
pub const GL_MULTISAMPLE_BUFFER_BIT5_QCOM: GLbitfield = GLbitfield(0x20000000);

///
/// * Group: [`BufferBitQCOM`]
pub const GL_MULTISAMPLE_BUFFER_BIT6_QCOM: GLbitfield = GLbitfield(0x40000000);

///
/// * Group: [`BufferBitQCOM`]
pub const GL_MULTISAMPLE_BUFFER_BIT7_QCOM: GLbitfield = GLbitfield(0x80000000);

pub const GL_MULTISAMPLE_COVERAGE_MODES_NV: GLenum = GLenum(0x8E12);

pub const GL_MULTISAMPLE_EXT: GLenum = GLenum(0x809D);

///
/// * Group: [`HintTarget`]
pub const GL_MULTISAMPLE_FILTER_HINT_NV: GLenum = GLenum(0x8534);

pub const GL_MULTISAMPLE_LINE_WIDTH_GRANULARITY: GLenum = GLenum(0x9382);

pub const GL_MULTISAMPLE_LINE_WIDTH_GRANULARITY_ARB: GLenum = GLenum(0x9382);

pub const GL_MULTISAMPLE_LINE_WIDTH_RANGE: GLenum = GLenum(0x9381);

pub const GL_MULTISAMPLE_LINE_WIDTH_RANGE_ARB: GLenum = GLenum(0x9381);

pub const GL_MULTISAMPLE_RASTERIZATION_ALLOWED_EXT: GLenum = GLenum(0x932B);

///
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_MULTISAMPLE_SGIS: GLenum = GLenum(0x809D);

pub const GL_MULTIVIEW_EXT: GLenum = GLenum(0x90F1);

///
/// * Group: [`FragmentOpATI`]
pub const GL_MUL_ATI: GLenum = GLenum(0x8964);

///
/// * Group: [`VertexShaderParameterEXT`]
pub const GL_MVP_MATRIX_EXT: GLenum = GLenum(0x87E3);

///
/// * Group: [`InterleavedArrayFormat`]
pub const GL_N3F_V3F: GLenum = GLenum(0x2A25);

pub const GL_NAMED_STRING_LENGTH_ARB: GLenum = GLenum(0x8DE9);

pub const GL_NAMED_STRING_TYPE_ARB: GLenum = GLenum(0x8DEA);

///
/// * Group: [`ProgramResourceProperty`]
pub const GL_NAME_LENGTH: GLenum = GLenum(0x92F9);

///
/// * Group: [`GetPName`]
pub const GL_NAME_STACK_DEPTH: GLenum = GLenum(0x0D70);

///
/// * Group: [`LogicOp`]
pub const GL_NAND: GLenum = GLenum(0x150E);

///
/// * Group: [`HintTarget`]
pub const GL_NATIVE_GRAPHICS_BEGIN_HINT_PGI: GLenum = GLenum(0x1A203);

///
/// * Group: [`HintTarget`]
pub const GL_NATIVE_GRAPHICS_END_HINT_PGI: GLenum = GLenum(0x1A204);

pub const GL_NATIVE_GRAPHICS_HANDLE_PGI: GLenum = GLenum(0x1A202);

///
/// * Group: [`BlitFramebufferFilter`], [`TextureMagFilter`],
///   [`TextureMinFilter`]
pub const GL_NEAREST: GLenum = GLenum(0x2600);

///
/// * Group: [`TextureMinFilter`]
pub const GL_NEAREST_CLIPMAP_LINEAR_SGIX: GLenum = GLenum(0x844E);

///
/// * Group: [`TextureMinFilter`]
pub const GL_NEAREST_CLIPMAP_NEAREST_SGIX: GLenum = GLenum(0x844D);

///
/// * Group: [`TextureMinFilter`]
pub const GL_NEAREST_MIPMAP_LINEAR: GLenum = GLenum(0x2702);

///
/// * Group: [`TextureMinFilter`]
pub const GL_NEAREST_MIPMAP_NEAREST: GLenum = GLenum(0x2700);

///
/// * Group: [`FragmentShaderColorModMaskATI`]
pub const GL_NEGATE_BIT_ATI: GLbitfield = GLbitfield(0x00000004);

///
/// * Group: [`VertexShaderCoordOutEXT`]
pub const GL_NEGATIVE_ONE_EXT: GLenum = GLenum(0x87DF);

///
/// * Group: [`ClipControlDepth`]
pub const GL_NEGATIVE_ONE_TO_ONE: GLenum = GLenum(0x935E);

///
/// * Alias Of: [`GL_NEGATIVE_ONE_TO_ONE`]
pub const GL_NEGATIVE_ONE_TO_ONE_EXT: GLenum = GL_NEGATIVE_ONE_TO_ONE;

///
/// * Group: [`VertexShaderCoordOutEXT`]
pub const GL_NEGATIVE_W_EXT: GLenum = GLenum(0x87DC);

///
/// * Group: [`VertexShaderCoordOutEXT`]
pub const GL_NEGATIVE_X_EXT: GLenum = GLenum(0x87D9);

///
/// * Group: [`VertexShaderCoordOutEXT`]
pub const GL_NEGATIVE_Y_EXT: GLenum = GLenum(0x87DA);

///
/// * Group: [`VertexShaderCoordOutEXT`]
pub const GL_NEGATIVE_Z_EXT: GLenum = GLenum(0x87DB);

///
/// * Group: [`StencilFunction`], [`IndexFunctionEXT`], [`AlphaFunction`],
///   [`DepthFunction`]
pub const GL_NEVER: GLenum = GLenum(0x0200);

///
/// * Group: [`TransformFeedbackTokenNV`]
pub const GL_NEXT_BUFFER_NV: GLenum = GLenum(u32::MAX - 2);

pub const GL_NEXT_VIDEO_CAPTURE_BUFFER_STATUS_NV: GLenum = GLenum(0x9025);

///
/// * Group: [`HintMode`]
pub const GL_NICEST: GLenum = GLenum(0x1102);

///
/// * Group: [`SyncBehaviorFlags`], [`TextureCompareMode`], [`PathColorFormat`],
///   [`CombinerBiasNV`], [`CombinerScaleNV`], [`DrawBufferMode`],
///   [`PixelTexGenMode`], [`ReadBufferMode`], [`ColorBuffer`], [`PathGenMode`],
///   [`PathTransformType`], [`PathFontStyle`]
pub const GL_NONE: GLenum = GLenum(0);

///
/// * Group: [`ReadBufferMode`], [`DrawBufferMode`]
pub const GL_NONE_OES: GLenum = GLenum(0);

///
/// * Group: [`LogicOp`]
pub const GL_NOOP: GLenum = GLenum(0x1505);

///
/// * Group: [`CommandOpcodesNV`]
pub const GL_NOP_COMMAND_NV: GLenum = GLenum(0x0001);

///
/// * Group: [`LogicOp`]
pub const GL_NOR: GLenum = GLenum(0x1508);

///
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_NORMALIZE: GLenum = GLenum(0x0BA1);

///
/// * Group: [`ParameterRangeEXT`]
pub const GL_NORMALIZED_RANGE_EXT: GLenum = GLenum(0x87E0);

///
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_NORMAL_ARRAY: GLenum = GLenum(0x8075);

pub const GL_NORMAL_ARRAY_ADDRESS_NV: GLenum = GLenum(0x8F22);

pub const GL_NORMAL_ARRAY_BUFFER_BINDING: GLenum = GLenum(0x8897);

pub const GL_NORMAL_ARRAY_BUFFER_BINDING_ARB: GLenum = GLenum(0x8897);

///
/// * Group: [`GetPName`]
pub const GL_NORMAL_ARRAY_COUNT_EXT: GLenum = GLenum(0x8080);

pub const GL_NORMAL_ARRAY_EXT: GLenum = GLenum(0x8075);

pub const GL_NORMAL_ARRAY_LENGTH_NV: GLenum = GLenum(0x8F2C);

pub const GL_NORMAL_ARRAY_LIST_IBM: GLenum = GLenum(103071);

pub const GL_NORMAL_ARRAY_LIST_STRIDE_IBM: GLenum = GLenum(103081);

pub const GL_NORMAL_ARRAY_PARALLEL_POINTERS_INTEL: GLenum = GLenum(0x83F6);

///
/// * Group: [`GetPointervPName`]
pub const GL_NORMAL_ARRAY_POINTER: GLenum = GLenum(0x808F);

///
/// * Group: [`GetPointervPName`]
pub const GL_NORMAL_ARRAY_POINTER_EXT: GLenum = GLenum(0x808F);

///
/// * Group: [`GetPName`]
pub const GL_NORMAL_ARRAY_STRIDE: GLenum = GLenum(0x807F);

pub const GL_NORMAL_ARRAY_STRIDE_EXT: GLenum = GLenum(0x807F);

///
/// * Group: [`GetPName`]
pub const GL_NORMAL_ARRAY_TYPE: GLenum = GLenum(0x807E);

pub const GL_NORMAL_ARRAY_TYPE_EXT: GLenum = GLenum(0x807E);

///
/// * Group: [`VertexHintsMaskPGI`]
pub const GL_NORMAL_BIT_PGI: GLbitfield = GLbitfield(0x08000000);

pub const GL_NORMAL_MAP: GLenum = GLenum(0x8511);

pub const GL_NORMAL_MAP_ARB: GLenum = GLenum(0x8511);

pub const GL_NORMAL_MAP_EXT: GLenum = GLenum(0x8511);

pub const GL_NORMAL_MAP_NV: GLenum = GLenum(0x8511);

pub const GL_NORMAL_MAP_OES: GLenum = GLenum(0x8511);

///
/// * Group: [`StencilFunction`], [`IndexFunctionEXT`], [`AlphaFunction`],
///   [`DepthFunction`]
pub const GL_NOTEQUAL: GLenum = GLenum(0x0205);

///
/// * Group: [`GraphicsResetStatus`], [`ErrorCode`]
pub const GL_NO_ERROR: GLenum = GLenum(0);

pub const GL_NO_RESET_NOTIFICATION: GLenum = GLenum(0x8261);

pub const GL_NO_RESET_NOTIFICATION_ARB: GLenum = GLenum(0x8261);

pub const GL_NO_RESET_NOTIFICATION_EXT: GLenum = GLenum(0x8261);

pub const GL_NO_RESET_NOTIFICATION_KHR: GLenum = GLenum(0x8261);

///
/// * Group: [`ProgramResourceProperty`]
pub const GL_NUM_ACTIVE_VARIABLES: GLenum = GLenum(0x9304);

///
/// * Group: [`ProgramResourceProperty`], [`SubroutineParameterName`]
pub const GL_NUM_COMPATIBLE_SUBROUTINES: GLenum = GLenum(0x8E4A);

///
/// * Group: [`GetPName`]
pub const GL_NUM_COMPRESSED_TEXTURE_FORMATS: GLenum = GLenum(0x86A2);

pub const GL_NUM_COMPRESSED_TEXTURE_FORMATS_ARB: GLenum = GLenum(0x86A2);

///
/// * Group: [`GetPName`]
pub const GL_NUM_DEVICE_UUIDS_EXT: GLenum = GLenum(0x9596);

pub const GL_NUM_DOWNSAMPLE_SCALES_IMG: GLenum = GLenum(0x913D);

///
/// * Group: [`GetPName`]
pub const GL_NUM_EXTENSIONS: GLenum = GLenum(0x821D);

pub const GL_NUM_FILL_STREAMS_NV: GLenum = GLenum(0x8E29);

pub const GL_NUM_FRAGMENT_CONSTANTS_ATI: GLenum = GLenum(0x896F);

pub const GL_NUM_FRAGMENT_REGISTERS_ATI: GLenum = GLenum(0x896E);

pub const GL_NUM_GENERAL_COMBINERS_NV: GLenum = GLenum(0x854E);

pub const GL_NUM_INPUT_INTERPOLATOR_COMPONENTS_ATI: GLenum = GLenum(0x8973);

pub const GL_NUM_INSTRUCTIONS_PER_PASS_ATI: GLenum = GLenum(0x8971);

pub const GL_NUM_INSTRUCTIONS_TOTAL_ATI: GLenum = GLenum(0x8972);

pub const GL_NUM_LOOPBACK_COMPONENTS_ATI: GLenum = GLenum(0x8974);

pub const GL_NUM_PASSES_ATI: GLenum = GLenum(0x8970);

///
/// * Group: [`GetPName`]
pub const GL_NUM_PROGRAM_BINARY_FORMATS: GLenum = GLenum(0x87FE);

pub const GL_NUM_PROGRAM_BINARY_FORMATS_OES: GLenum = GLenum(0x87FE);

///
/// * Group: [`InternalFormatPName`]
pub const GL_NUM_SAMPLE_COUNTS: GLenum = GLenum(0x9380);

///
/// * Group: [`GetPName`]
pub const GL_NUM_SHADER_BINARY_FORMATS: GLenum = GLenum(0x8DF9);

pub const GL_NUM_SHADING_LANGUAGE_VERSIONS: GLenum = GLenum(0x82E9);

pub const GL_NUM_SPARSE_LEVELS_ARB: GLenum = GLenum(0x91AA);

pub const GL_NUM_SPARSE_LEVELS_EXT: GLenum = GLenum(0x91AA);

pub const GL_NUM_SPIR_V_EXTENSIONS: GLenum = GLenum(0x9554);

pub const GL_NUM_SUPPORTED_MULTISAMPLE_MODES_AMD: GLenum = GLenum(0x91B6);

pub const GL_NUM_TILING_TYPES_EXT: GLenum = GLenum(0x9582);

pub const GL_NUM_VIDEO_CAPTURE_STREAMS_NV: GLenum = GLenum(0x9024);

pub const GL_NUM_VIRTUAL_PAGE_SIZES_ARB: GLenum = GLenum(0x91A8);

pub const GL_NUM_VIRTUAL_PAGE_SIZES_EXT: GLenum = GLenum(0x91A8);

pub const GL_NUM_WINDOW_RECTANGLES_EXT: GLenum = GLenum(0x8F15);

pub const GL_OBJECT_ACTIVE_ATTRIBUTES_ARB: GLenum = GLenum(0x8B89);

pub const GL_OBJECT_ACTIVE_ATTRIBUTE_MAX_LENGTH_ARB: GLenum = GLenum(0x8B8A);

pub const GL_OBJECT_ACTIVE_UNIFORMS_ARB: GLenum = GLenum(0x8B86);

pub const GL_OBJECT_ACTIVE_UNIFORM_MAX_LENGTH_ARB: GLenum = GLenum(0x8B87);

pub const GL_OBJECT_ATTACHED_OBJECTS_ARB: GLenum = GLenum(0x8B85);

///
/// * Group: [`ArrayObjectPNameATI`]
pub const GL_OBJECT_BUFFER_SIZE_ATI: GLenum = GLenum(0x8764);

///
/// * Group: [`ArrayObjectPNameATI`]
pub const GL_OBJECT_BUFFER_USAGE_ATI: GLenum = GLenum(0x8765);

pub const GL_OBJECT_COMPILE_STATUS_ARB: GLenum = GLenum(0x8B81);

pub const GL_OBJECT_DELETE_STATUS_ARB: GLenum = GLenum(0x8B80);

///
/// * Group: [`TextureGenMode`]
pub const GL_OBJECT_DISTANCE_TO_LINE_SGIS: GLenum = GLenum(0x81F3);

///
/// * Group: [`TextureGenMode`]
pub const GL_OBJECT_DISTANCE_TO_POINT_SGIS: GLenum = GLenum(0x81F1);

pub const GL_OBJECT_INFO_LOG_LENGTH_ARB: GLenum = GLenum(0x8B84);

///
/// * Group: [`PathGenMode`], [`TextureGenMode`]
pub const GL_OBJECT_LINEAR: GLenum = GLenum(0x2401);

pub const GL_OBJECT_LINEAR_NV: GLenum = GLenum(0x2401);

///
/// * Group: [`TextureGenParameter`]
pub const GL_OBJECT_LINE_SGIS: GLenum = GLenum(0x81F7);

pub const GL_OBJECT_LINK_STATUS_ARB: GLenum = GLenum(0x8B82);

///
/// * Group: [`TextureGenParameter`]
pub const GL_OBJECT_PLANE: GLenum = GLenum(0x2501);

///
/// * Group: [`TextureGenParameter`]
pub const GL_OBJECT_POINT_SGIS: GLenum = GLenum(0x81F5);

pub const GL_OBJECT_SHADER_SOURCE_LENGTH_ARB: GLenum = GLenum(0x8B88);

pub const GL_OBJECT_SUBTYPE_ARB: GLenum = GLenum(0x8B4F);

///
/// * Group: [`SyncParameterName`]
pub const GL_OBJECT_TYPE: GLenum = GLenum(0x9112);

pub const GL_OBJECT_TYPE_APPLE: GLenum = GLenum(0x9112);

pub const GL_OBJECT_TYPE_ARB: GLenum = GLenum(0x8B4E);

pub const GL_OBJECT_VALIDATE_STATUS_ARB: GLenum = GLenum(0x8B83);

pub const GL_OCCLUSION_QUERY_EVENT_MASK_AMD: GLenum = GLenum(0x874F);

pub const GL_OCCLUSION_TEST_HP: GLenum = GLenum(0x8165);

pub const GL_OCCLUSION_TEST_RESULT_HP: GLenum = GLenum(0x8166);

///
/// * Group: [`ProgramResourceProperty`]
pub const GL_OFFSET: GLenum = GLenum(0x92FC);

pub const GL_OFFSET_HILO_PROJECTIVE_TEXTURE_2D_NV: GLenum = GLenum(0x8856);

pub const GL_OFFSET_HILO_PROJECTIVE_TEXTURE_RECTANGLE_NV: GLenum = GLenum(0x8857);

pub const GL_OFFSET_HILO_TEXTURE_2D_NV: GLenum = GLenum(0x8854);

pub const GL_OFFSET_HILO_TEXTURE_RECTANGLE_NV: GLenum = GLenum(0x8855);

pub const GL_OFFSET_PROJECTIVE_TEXTURE_2D_NV: GLenum = GLenum(0x8850);

pub const GL_OFFSET_PROJECTIVE_TEXTURE_2D_SCALE_NV: GLenum = GLenum(0x8851);

pub const GL_OFFSET_PROJECTIVE_TEXTURE_RECTANGLE_NV: GLenum = GLenum(0x8852);

pub const GL_OFFSET_PROJECTIVE_TEXTURE_RECTANGLE_SCALE_NV: GLenum = GLenum(0x8853);

///
/// * Alias Of: [`GL_OFFSET_TEXTURE_BIAS_NV`]
pub const GL_OFFSET_TEXTURE_2D_BIAS_NV: GLenum = GL_OFFSET_TEXTURE_BIAS_NV;

///
/// * Alias Of: [`GL_OFFSET_TEXTURE_MATRIX_NV`]
pub const GL_OFFSET_TEXTURE_2D_MATRIX_NV: GLenum = GL_OFFSET_TEXTURE_MATRIX_NV;

pub const GL_OFFSET_TEXTURE_2D_NV: GLenum = GLenum(0x86E8);

///
/// * Alias Of: [`GL_OFFSET_TEXTURE_SCALE_NV`]
pub const GL_OFFSET_TEXTURE_2D_SCALE_NV: GLenum = GL_OFFSET_TEXTURE_SCALE_NV;

pub const GL_OFFSET_TEXTURE_BIAS_NV: GLenum = GLenum(0x86E3);

pub const GL_OFFSET_TEXTURE_MATRIX_NV: GLenum = GLenum(0x86E1);

pub const GL_OFFSET_TEXTURE_RECTANGLE_NV: GLenum = GLenum(0x864C);

pub const GL_OFFSET_TEXTURE_RECTANGLE_SCALE_NV: GLenum = GLenum(0x864D);

pub const GL_OFFSET_TEXTURE_SCALE_NV: GLenum = GLenum(0x86E2);

///
/// * Group: [`TextureSwizzle`], [`BlendingFactor`]
pub const GL_ONE: GLenum = GLenum(1);

///
/// * Group: [`VertexShaderCoordOutEXT`]
pub const GL_ONE_EXT: GLenum = GLenum(0x87DE);

///
/// * Group: [`BlendingFactor`]
pub const GL_ONE_MINUS_CONSTANT_ALPHA: GLenum = GLenum(0x8004);

pub const GL_ONE_MINUS_CONSTANT_ALPHA_EXT: GLenum = GLenum(0x8004);

///
/// * Group: [`BlendingFactor`]
pub const GL_ONE_MINUS_CONSTANT_COLOR: GLenum = GLenum(0x8002);

pub const GL_ONE_MINUS_CONSTANT_COLOR_EXT: GLenum = GLenum(0x8002);

///
/// * Group: [`BlendingFactor`]
pub const GL_ONE_MINUS_DST_ALPHA: GLenum = GLenum(0x0305);

///
/// * Group: [`BlendingFactor`]
pub const GL_ONE_MINUS_DST_COLOR: GLenum = GLenum(0x0307);

///
/// * Group: [`BlendingFactor`]
pub const GL_ONE_MINUS_SRC1_ALPHA: GLenum = GLenum(0x88FB);

pub const GL_ONE_MINUS_SRC1_ALPHA_EXT: GLenum = GLenum(0x88FB);

///
/// * Group: [`BlendingFactor`]
pub const GL_ONE_MINUS_SRC1_COLOR: GLenum = GLenum(0x88FA);

pub const GL_ONE_MINUS_SRC1_COLOR_EXT: GLenum = GLenum(0x88FA);

///
/// * Group: [`BlendingFactor`]
pub const GL_ONE_MINUS_SRC_ALPHA: GLenum = GLenum(0x0303);

///
/// * Group: [`BlendingFactor`]
pub const GL_ONE_MINUS_SRC_COLOR: GLenum = GLenum(0x0301);

pub const GL_OPERAND0_ALPHA: GLenum = GLenum(0x8598);

pub const GL_OPERAND0_ALPHA_ARB: GLenum = GLenum(0x8598);

pub const GL_OPERAND0_ALPHA_EXT: GLenum = GLenum(0x8598);

pub const GL_OPERAND0_RGB: GLenum = GLenum(0x8590);

pub const GL_OPERAND0_RGB_ARB: GLenum = GLenum(0x8590);

pub const GL_OPERAND0_RGB_EXT: GLenum = GLenum(0x8590);

pub const GL_OPERAND1_ALPHA: GLenum = GLenum(0x8599);

pub const GL_OPERAND1_ALPHA_ARB: GLenum = GLenum(0x8599);

pub const GL_OPERAND1_ALPHA_EXT: GLenum = GLenum(0x8599);

pub const GL_OPERAND1_RGB: GLenum = GLenum(0x8591);

pub const GL_OPERAND1_RGB_ARB: GLenum = GLenum(0x8591);

pub const GL_OPERAND1_RGB_EXT: GLenum = GLenum(0x8591);

pub const GL_OPERAND2_ALPHA: GLenum = GLenum(0x859A);

pub const GL_OPERAND2_ALPHA_ARB: GLenum = GLenum(0x859A);

pub const GL_OPERAND2_ALPHA_EXT: GLenum = GLenum(0x859A);

pub const GL_OPERAND2_RGB: GLenum = GLenum(0x8592);

pub const GL_OPERAND2_RGB_ARB: GLenum = GLenum(0x8592);

pub const GL_OPERAND2_RGB_EXT: GLenum = GLenum(0x8592);

pub const GL_OPERAND3_ALPHA_NV: GLenum = GLenum(0x859B);

pub const GL_OPERAND3_RGB_NV: GLenum = GLenum(0x8593);

pub const GL_OPTIMAL_TILING_EXT: GLenum = GLenum(0x9584);

///
/// * Group: [`VertexShaderOpEXT`]
pub const GL_OP_ADD_EXT: GLenum = GLenum(0x8787);

///
/// * Group: [`VertexShaderOpEXT`]
pub const GL_OP_CLAMP_EXT: GLenum = GLenum(0x878E);

///
/// * Group: [`VertexShaderOpEXT`]
pub const GL_OP_CROSS_PRODUCT_EXT: GLenum = GLenum(0x8797);

///
/// * Group: [`VertexShaderOpEXT`]
pub const GL_OP_DOT3_EXT: GLenum = GLenum(0x8784);

///
/// * Group: [`VertexShaderOpEXT`]
pub const GL_OP_DOT4_EXT: GLenum = GLenum(0x8785);

///
/// * Group: [`VertexShaderOpEXT`]
pub const GL_OP_EXP_BASE_2_EXT: GLenum = GLenum(0x8791);

///
/// * Group: [`VertexShaderOpEXT`]
pub const GL_OP_FLOOR_EXT: GLenum = GLenum(0x878F);

///
/// * Group: [`VertexShaderOpEXT`]
pub const GL_OP_FRAC_EXT: GLenum = GLenum(0x8789);

///
/// * Group: [`VertexShaderOpEXT`]
pub const GL_OP_INDEX_EXT: GLenum = GLenum(0x8782);

///
/// * Group: [`VertexShaderOpEXT`]
pub const GL_OP_LOG_BASE_2_EXT: GLenum = GLenum(0x8792);

///
/// * Group: [`VertexShaderOpEXT`]
pub const GL_OP_MADD_EXT: GLenum = GLenum(0x8788);

///
/// * Group: [`VertexShaderOpEXT`]
pub const GL_OP_MAX_EXT: GLenum = GLenum(0x878A);

///
/// * Group: [`VertexShaderOpEXT`]
pub const GL_OP_MIN_EXT: GLenum = GLenum(0x878B);

///
/// * Group: [`VertexShaderOpEXT`]
pub const GL_OP_MOV_EXT: GLenum = GLenum(0x8799);

///
/// * Group: [`VertexShaderOpEXT`]
pub const GL_OP_MULTIPLY_MATRIX_EXT: GLenum = GLenum(0x8798);

///
/// * Group: [`VertexShaderOpEXT`]
pub const GL_OP_MUL_EXT: GLenum = GLenum(0x8786);

///
/// * Group: [`VertexShaderOpEXT`]
pub const GL_OP_NEGATE_EXT: GLenum = GLenum(0x8783);

///
/// * Group: [`VertexShaderOpEXT`]
pub const GL_OP_POWER_EXT: GLenum = GLenum(0x8793);

///
/// * Group: [`VertexShaderOpEXT`]
pub const GL_OP_RECIP_EXT: GLenum = GLenum(0x8794);

///
/// * Group: [`VertexShaderOpEXT`]
pub const GL_OP_RECIP_SQRT_EXT: GLenum = GLenum(0x8795);

///
/// * Group: [`VertexShaderOpEXT`]
pub const GL_OP_ROUND_EXT: GLenum = GLenum(0x8790);

///
/// * Group: [`VertexShaderOpEXT`]
pub const GL_OP_SET_GE_EXT: GLenum = GLenum(0x878C);

///
/// * Group: [`VertexShaderOpEXT`]
pub const GL_OP_SET_LT_EXT: GLenum = GLenum(0x878D);

///
/// * Group: [`VertexShaderOpEXT`]
pub const GL_OP_SUB_EXT: GLenum = GLenum(0x8796);

///
/// * Group: [`LogicOp`]
pub const GL_OR: GLenum = GLenum(0x1507);

///
/// * Group: [`MapQuery`], [`GetMapQuery`]
pub const GL_ORDER: GLenum = GLenum(0x0A01);

///
/// * Group: [`LogicOp`]
pub const GL_OR_INVERTED: GLenum = GLenum(0x150D);

///
/// * Group: [`LogicOp`]
pub const GL_OR_REVERSE: GLenum = GLenum(0x150B);

pub const GL_OUTPUT_COLOR0_EXT: GLenum = GLenum(0x879B);

pub const GL_OUTPUT_COLOR1_EXT: GLenum = GLenum(0x879C);

pub const GL_OUTPUT_FOG_EXT: GLenum = GLenum(0x87BD);

pub const GL_OUTPUT_TEXTURE_COORD0_EXT: GLenum = GLenum(0x879D);

pub const GL_OUTPUT_TEXTURE_COORD10_EXT: GLenum = GLenum(0x87A7);

pub const GL_OUTPUT_TEXTURE_COORD11_EXT: GLenum = GLenum(0x87A8);

pub const GL_OUTPUT_TEXTURE_COORD12_EXT: GLenum = GLenum(0x87A9);

pub const GL_OUTPUT_TEXTURE_COORD13_EXT: GLenum = GLenum(0x87AA);

pub const GL_OUTPUT_TEXTURE_COORD14_EXT: GLenum = GLenum(0x87AB);

pub const GL_OUTPUT_TEXTURE_COORD15_EXT: GLenum = GLenum(0x87AC);

pub const GL_OUTPUT_TEXTURE_COORD16_EXT: GLenum = GLenum(0x87AD);

pub const GL_OUTPUT_TEXTURE_COORD17_EXT: GLenum = GLenum(0x87AE);

pub const GL_OUTPUT_TEXTURE_COORD18_EXT: GLenum = GLenum(0x87AF);

pub const GL_OUTPUT_TEXTURE_COORD19_EXT: GLenum = GLenum(0x87B0);

pub const GL_OUTPUT_TEXTURE_COORD1_EXT: GLenum = GLenum(0x879E);

pub const GL_OUTPUT_TEXTURE_COORD20_EXT: GLenum = GLenum(0x87B1);

pub const GL_OUTPUT_TEXTURE_COORD21_EXT: GLenum = GLenum(0x87B2);

pub const GL_OUTPUT_TEXTURE_COORD22_EXT: GLenum = GLenum(0x87B3);

pub const GL_OUTPUT_TEXTURE_COORD23_EXT: GLenum = GLenum(0x87B4);

pub const GL_OUTPUT_TEXTURE_COORD24_EXT: GLenum = GLenum(0x87B5);

pub const GL_OUTPUT_TEXTURE_COORD25_EXT: GLenum = GLenum(0x87B6);

pub const GL_OUTPUT_TEXTURE_COORD26_EXT: GLenum = GLenum(0x87B7);

pub const GL_OUTPUT_TEXTURE_COORD27_EXT: GLenum = GLenum(0x87B8);

pub const GL_OUTPUT_TEXTURE_COORD28_EXT: GLenum = GLenum(0x87B9);

pub const GL_OUTPUT_TEXTURE_COORD29_EXT: GLenum = GLenum(0x87BA);

pub const GL_OUTPUT_TEXTURE_COORD2_EXT: GLenum = GLenum(0x879F);

pub const GL_OUTPUT_TEXTURE_COORD30_EXT: GLenum = GLenum(0x87BB);

pub const GL_OUTPUT_TEXTURE_COORD31_EXT: GLenum = GLenum(0x87BC);

pub const GL_OUTPUT_TEXTURE_COORD3_EXT: GLenum = GLenum(0x87A0);

pub const GL_OUTPUT_TEXTURE_COORD4_EXT: GLenum = GLenum(0x87A1);

pub const GL_OUTPUT_TEXTURE_COORD5_EXT: GLenum = GLenum(0x87A2);

pub const GL_OUTPUT_TEXTURE_COORD6_EXT: GLenum = GLenum(0x87A3);

pub const GL_OUTPUT_TEXTURE_COORD7_EXT: GLenum = GLenum(0x87A4);

pub const GL_OUTPUT_TEXTURE_COORD8_EXT: GLenum = GLenum(0x87A5);

pub const GL_OUTPUT_TEXTURE_COORD9_EXT: GLenum = GLenum(0x87A6);

pub const GL_OUTPUT_VERTEX_EXT: GLenum = GLenum(0x879A);

///
/// * Group: [`ErrorCode`]
pub const GL_OUT_OF_MEMORY: GLenum = GLenum(0x0505);

pub const GL_OVERLAY: GLenum = GLenum(0x9296);

pub const GL_OVERLAY_KHR: GLenum = GLenum(0x9296);

pub const GL_OVERLAY_NV: GLenum = GLenum(0x9296);

///
/// * Group: [`PixelStoreParameter`], [`GetPName`]
pub const GL_PACK_ALIGNMENT: GLenum = GLenum(0x0D05);

///
/// * Group: [`HintTarget`], [`GetPName`]
pub const GL_PACK_CMYK_HINT_EXT: GLenum = GLenum(0x800E);

pub const GL_PACK_COMPRESSED_BLOCK_DEPTH: GLenum = GLenum(0x912D);

pub const GL_PACK_COMPRESSED_BLOCK_HEIGHT: GLenum = GLenum(0x912C);

pub const GL_PACK_COMPRESSED_BLOCK_SIZE: GLenum = GLenum(0x912E);

pub const GL_PACK_COMPRESSED_BLOCK_WIDTH: GLenum = GLenum(0x912B);

pub const GL_PACK_COMPRESSED_SIZE_SGIX: GLenum = GLenum(0x831C);

///
/// * Group: [`PixelStoreParameter`], [`GetPName`]
pub const GL_PACK_IMAGE_DEPTH_SGIS: GLenum = GLenum(0x8131);

///
/// * Group: [`PixelStoreParameter`], [`GetPName`]
pub const GL_PACK_IMAGE_HEIGHT: GLenum = GLenum(0x806C);

///
/// * Group: [`PixelStoreParameter`], [`GetPName`]
pub const GL_PACK_IMAGE_HEIGHT_EXT: GLenum = GLenum(0x806C);

pub const GL_PACK_INVERT_MESA: GLenum = GLenum(0x8758);

///
/// * Group: [`PixelStoreParameter`], [`GetPName`]
pub const GL_PACK_LSB_FIRST: GLenum = GLenum(0x0D01);

pub const GL_PACK_MAX_COMPRESSED_SIZE_SGIX: GLenum = GLenum(0x831B);

///
/// * Group: [`PixelStoreParameter`]
pub const GL_PACK_RESAMPLE_OML: GLenum = GLenum(0x8984);

/// Formerly 0x842C in SGI specfile
/// * Group: [`PixelStoreParameter`], [`GetPName`]
pub const GL_PACK_RESAMPLE_SGIX: GLenum = GLenum(0x842E);

pub const GL_PACK_REVERSE_ROW_ORDER_ANGLE: GLenum = GLenum(0x93A4);

pub const GL_PACK_ROW_BYTES_APPLE: GLenum = GLenum(0x8A15);

///
/// * Group: [`PixelStoreParameter`], [`GetPName`]
pub const GL_PACK_ROW_LENGTH: GLenum = GLenum(0x0D02);

///
/// * Group: [`PixelStoreParameter`], [`GetPName`]
pub const GL_PACK_SKIP_IMAGES: GLenum = GLenum(0x806B);

///
/// * Group: [`PixelStoreParameter`], [`GetPName`]
pub const GL_PACK_SKIP_IMAGES_EXT: GLenum = GLenum(0x806B);

///
/// * Group: [`PixelStoreParameter`], [`GetPName`]
pub const GL_PACK_SKIP_PIXELS: GLenum = GLenum(0x0D04);

///
/// * Group: [`PixelStoreParameter`], [`GetPName`]
pub const GL_PACK_SKIP_ROWS: GLenum = GLenum(0x0D03);

///
/// * Group: [`PixelStoreParameter`], [`GetPName`]
pub const GL_PACK_SKIP_VOLUMES_SGIS: GLenum = GLenum(0x8130);

///
/// * Group: [`PixelStoreParameter`], [`GetPName`]
pub const GL_PACK_SUBSAMPLE_RATE_SGIX: GLenum = GLenum(0x85A0);

///
/// * Group: [`PixelStoreParameter`], [`GetPName`]
pub const GL_PACK_SWAP_BYTES: GLenum = GLenum(0x0D00);

pub const GL_PALETTE4_R5_G6_B5_OES: GLenum = GLenum(0x8B92);

pub const GL_PALETTE4_RGB5_A1_OES: GLenum = GLenum(0x8B94);

pub const GL_PALETTE4_RGB8_OES: GLenum = GLenum(0x8B90);

pub const GL_PALETTE4_RGBA4_OES: GLenum = GLenum(0x8B93);

pub const GL_PALETTE4_RGBA8_OES: GLenum = GLenum(0x8B91);

pub const GL_PALETTE8_R5_G6_B5_OES: GLenum = GLenum(0x8B97);

pub const GL_PALETTE8_RGB5_A1_OES: GLenum = GLenum(0x8B99);

pub const GL_PALETTE8_RGB8_OES: GLenum = GLenum(0x8B95);

pub const GL_PALETTE8_RGBA4_OES: GLenum = GLenum(0x8B98);

pub const GL_PALETTE8_RGBA8_OES: GLenum = GLenum(0x8B96);

pub const GL_PARALLEL_ARRAYS_INTEL: GLenum = GLenum(0x83F4);

///
/// * Group: [`BufferTargetARB`]
pub const GL_PARAMETER_BUFFER: GLenum = GLenum(0x80EE);

///
/// * Alias Of: [`GL_PARAMETER_BUFFER`]
pub const GL_PARAMETER_BUFFER_ARB: GLenum = GL_PARAMETER_BUFFER;

pub const GL_PARAMETER_BUFFER_BINDING: GLenum = GLenum(0x80EF);

///
/// * Alias Of: [`GL_PARAMETER_BUFFER_BINDING`]
pub const GL_PARAMETER_BUFFER_BINDING_ARB: GLenum = GL_PARAMETER_BUFFER_BINDING;

pub const GL_PARTIAL_SUCCESS_NV: GLenum = GLenum(0x902E);

pub const GL_PASS_THROUGH_NV: GLenum = GLenum(0x86E6);

///
/// * Group: [`FeedBackToken`]
pub const GL_PASS_THROUGH_TOKEN: GLenum = GLenum(0x0700);

///
/// * Group: [`PrimitiveType`]
pub const GL_PATCHES: GLenum = GLenum(0x000E);

///
/// * Group: [`PrimitiveType`]
pub const GL_PATCHES_EXT: GLenum = GLenum(0x000E);

pub const GL_PATCHES_OES: GLenum = GLenum(0x000E);

///
/// * Group: [`PatchParameterName`]
pub const GL_PATCH_DEFAULT_INNER_LEVEL: GLenum = GLenum(0x8E73);

pub const GL_PATCH_DEFAULT_INNER_LEVEL_EXT: GLenum = GLenum(0x8E73);

///
/// * Group: [`PatchParameterName`]
pub const GL_PATCH_DEFAULT_OUTER_LEVEL: GLenum = GLenum(0x8E74);

pub const GL_PATCH_DEFAULT_OUTER_LEVEL_EXT: GLenum = GLenum(0x8E74);

///
/// * Group: [`PatchParameterName`]
pub const GL_PATCH_VERTICES: GLenum = GLenum(0x8E72);

pub const GL_PATCH_VERTICES_EXT: GLenum = GLenum(0x8E72);

pub const GL_PATCH_VERTICES_OES: GLenum = GLenum(0x8E72);

///
/// * Group: [`PathParameter`]
pub const GL_PATH_CLIENT_LENGTH_NV: GLenum = GLenum(0x907F);

///
/// * Group: [`PathParameter`]
pub const GL_PATH_COMMAND_COUNT_NV: GLenum = GLenum(0x909D);

///
/// * Group: [`PathParameter`]
pub const GL_PATH_COMPUTED_LENGTH_NV: GLenum = GLenum(0x90A0);

///
/// * Group: [`PathParameter`]
pub const GL_PATH_COORD_COUNT_NV: GLenum = GLenum(0x909E);

pub const GL_PATH_COVER_DEPTH_FUNC_NV: GLenum = GLenum(0x90BF);

///
/// * Group: [`PathParameter`]
pub const GL_PATH_DASH_ARRAY_COUNT_NV: GLenum = GLenum(0x909F);

///
/// * Group: [`PathParameter`]
pub const GL_PATH_DASH_CAPS_NV: GLenum = GLenum(0x907B);

///
/// * Group: [`PathParameter`]
pub const GL_PATH_DASH_OFFSET_NV: GLenum = GLenum(0x907E);

///
/// * Group: [`PathParameter`]
pub const GL_PATH_DASH_OFFSET_RESET_NV: GLenum = GLenum(0x90B4);

///
/// * Group: [`PathParameter`]
pub const GL_PATH_END_CAPS_NV: GLenum = GLenum(0x9076);

pub const GL_PATH_ERROR_POSITION_NV: GLenum = GLenum(0x90AB);

///
/// * Group: [`PathParameter`]
pub const GL_PATH_FILL_BOUNDING_BOX_NV: GLenum = GLenum(0x90A1);

///
/// * Group: [`PathCoverMode`], [`PathParameter`]
pub const GL_PATH_FILL_COVER_MODE_NV: GLenum = GLenum(0x9082);

///
/// * Group: [`PathParameter`]
pub const GL_PATH_FILL_MASK_NV: GLenum = GLenum(0x9081);

///
/// * Group: [`PathParameter`], [`PathFillMode`]
pub const GL_PATH_FILL_MODE_NV: GLenum = GLenum(0x9080);

pub const GL_PATH_FOG_GEN_MODE_NV: GLenum = GLenum(0x90AC);

///
/// * Group: [`PathStringFormat`]
pub const GL_PATH_FORMAT_PS_NV: GLenum = GLenum(0x9071);

///
/// * Group: [`PathStringFormat`]
pub const GL_PATH_FORMAT_SVG_NV: GLenum = GLenum(0x9070);

pub const GL_PATH_GEN_COEFF_NV: GLenum = GLenum(0x90B1);

pub const GL_PATH_GEN_COLOR_FORMAT_NV: GLenum = GLenum(0x90B2);

pub const GL_PATH_GEN_COMPONENTS_NV: GLenum = GLenum(0x90B3);

pub const GL_PATH_GEN_MODE_NV: GLenum = GLenum(0x90B0);

///
/// * Group: [`PathParameter`]
pub const GL_PATH_INITIAL_DASH_CAP_NV: GLenum = GLenum(0x907C);

///
/// * Group: [`PathParameter`]
pub const GL_PATH_INITIAL_END_CAP_NV: GLenum = GLenum(0x9077);

///
/// * Group: [`PathParameter`]
pub const GL_PATH_JOIN_STYLE_NV: GLenum = GLenum(0x9079);

pub const GL_PATH_MAX_MODELVIEW_STACK_DEPTH_NV: GLenum = GLenum(0x0D36);

pub const GL_PATH_MAX_PROJECTION_STACK_DEPTH_NV: GLenum = GLenum(0x0D38);

///
/// * Group: [`PathParameter`]
pub const GL_PATH_MITER_LIMIT_NV: GLenum = GLenum(0x907A);

pub const GL_PATH_MODELVIEW_MATRIX_NV: GLenum = GLenum(0x0BA6);

pub const GL_PATH_MODELVIEW_NV: GLenum = GLenum(0x1700);

pub const GL_PATH_MODELVIEW_STACK_DEPTH_NV: GLenum = GLenum(0x0BA3);

///
/// * Group: [`PathGenMode`], [`PathParameter`]
pub const GL_PATH_OBJECT_BOUNDING_BOX_NV: GLenum = GLenum(0x908A);

pub const GL_PATH_PROJECTION_MATRIX_NV: GLenum = GLenum(0x0BA7);

pub const GL_PATH_PROJECTION_NV: GLenum = GLenum(0x1701);

pub const GL_PATH_PROJECTION_STACK_DEPTH_NV: GLenum = GLenum(0x0BA4);

pub const GL_PATH_STENCIL_DEPTH_OFFSET_FACTOR_NV: GLenum = GLenum(0x90BD);

pub const GL_PATH_STENCIL_DEPTH_OFFSET_UNITS_NV: GLenum = GLenum(0x90BE);

pub const GL_PATH_STENCIL_FUNC_NV: GLenum = GLenum(0x90B7);

pub const GL_PATH_STENCIL_REF_NV: GLenum = GLenum(0x90B8);

pub const GL_PATH_STENCIL_VALUE_MASK_NV: GLenum = GLenum(0x90B9);

///
/// * Group: [`PathParameter`]
pub const GL_PATH_STROKE_BOUNDING_BOX_NV: GLenum = GLenum(0x90A2);

///
/// * Group: [`PathParameter`]
pub const GL_PATH_STROKE_COVER_MODE_NV: GLenum = GLenum(0x9083);

///
/// * Group: [`PathParameter`]
pub const GL_PATH_STROKE_MASK_NV: GLenum = GLenum(0x9084);

///
/// * Group: [`PathParameter`]
pub const GL_PATH_STROKE_WIDTH_NV: GLenum = GLenum(0x9075);

///
/// * Group: [`PathParameter`]
pub const GL_PATH_TERMINAL_DASH_CAP_NV: GLenum = GLenum(0x907D);

///
/// * Group: [`PathParameter`]
pub const GL_PATH_TERMINAL_END_CAP_NV: GLenum = GLenum(0x9078);

pub const GL_PATH_TRANSPOSE_MODELVIEW_MATRIX_NV: GLenum = GLenum(0x84E3);

pub const GL_PATH_TRANSPOSE_PROJECTION_MATRIX_NV: GLenum = GLenum(0x84E4);

pub const GL_PERCENTAGE_AMD: GLenum = GLenum(0x8BC3);

pub const GL_PERFMON_GLOBAL_MODE_QCOM: GLenum = GLenum(0x8FA0);

pub const GL_PERFMON_RESULT_AMD: GLenum = GLenum(0x8BC6);

pub const GL_PERFMON_RESULT_AVAILABLE_AMD: GLenum = GLenum(0x8BC4);

pub const GL_PERFMON_RESULT_SIZE_AMD: GLenum = GLenum(0x8BC5);

pub const GL_PERFORMANCE_MONITOR_AMD: GLenum = GLenum(0x9152);

pub const GL_PERFQUERY_COUNTER_DATA_BOOL32_INTEL: GLenum = GLenum(0x94FC);

pub const GL_PERFQUERY_COUNTER_DATA_DOUBLE_INTEL: GLenum = GLenum(0x94FB);

pub const GL_PERFQUERY_COUNTER_DATA_FLOAT_INTEL: GLenum = GLenum(0x94FA);

pub const GL_PERFQUERY_COUNTER_DATA_UINT32_INTEL: GLenum = GLenum(0x94F8);

pub const GL_PERFQUERY_COUNTER_DATA_UINT64_INTEL: GLenum = GLenum(0x94F9);

pub const GL_PERFQUERY_COUNTER_DESC_LENGTH_MAX_INTEL: GLenum = GLenum(0x94FF);

pub const GL_PERFQUERY_COUNTER_DURATION_NORM_INTEL: GLenum = GLenum(0x94F1);

pub const GL_PERFQUERY_COUNTER_DURATION_RAW_INTEL: GLenum = GLenum(0x94F2);

pub const GL_PERFQUERY_COUNTER_EVENT_INTEL: GLenum = GLenum(0x94F0);

pub const GL_PERFQUERY_COUNTER_NAME_LENGTH_MAX_INTEL: GLenum = GLenum(0x94FE);

pub const GL_PERFQUERY_COUNTER_RAW_INTEL: GLenum = GLenum(0x94F4);

pub const GL_PERFQUERY_COUNTER_THROUGHPUT_INTEL: GLenum = GLenum(0x94F3);

pub const GL_PERFQUERY_COUNTER_TIMESTAMP_INTEL: GLenum = GLenum(0x94F5);

pub const GL_PERFQUERY_DONOT_FLUSH_INTEL: GLenum = GLenum(0x83F9);

pub const GL_PERFQUERY_FLUSH_INTEL: GLenum = GLenum(0x83FA);

///
/// * Group: [`PerformanceQueryCapsMaskINTEL`]
pub const GL_PERFQUERY_GLOBAL_CONTEXT_INTEL: GLbitfield = GLbitfield(0x00000001);

pub const GL_PERFQUERY_GPA_EXTENDED_COUNTERS_INTEL: GLenum = GLenum(0x9500);

pub const GL_PERFQUERY_QUERY_NAME_LENGTH_MAX_INTEL: GLenum = GLenum(0x94FD);

///
/// * Group: [`PerformanceQueryCapsMaskINTEL`]
pub const GL_PERFQUERY_SINGLE_CONTEXT_INTEL: GLbitfield = GLbitfield(0x00000000);

pub const GL_PERFQUERY_WAIT_INTEL: GLenum = GLenum(0x83FB);

///
/// * Group: [`HintTarget`], [`GetPName`]
pub const GL_PERSPECTIVE_CORRECTION_HINT: GLenum = GLenum(0x0C50);

///
/// * Group: [`TextureNormalModeEXT`]
pub const GL_PERTURB_EXT: GLenum = GLenum(0x85AE);

///
/// * Group: [`BufferStorageMask`]
pub const GL_PER_GPU_STORAGE_BIT_NV: GLbitfield = GLbitfield(0x0800);

pub const GL_PER_GPU_STORAGE_NV: GLenum = GLenum(0x9548);

pub const GL_PER_STAGE_CONSTANTS_NV: GLenum = GLenum(0x8535);

///
/// * Group: [`HintTarget`]
pub const GL_PHONG_HINT_WIN: GLenum = GLenum(0x80EB);

pub const GL_PHONG_WIN: GLenum = GLenum(0x80EA);

pub const GL_PINLIGHT_NV: GLenum = GLenum(0x92A8);

pub const GL_PIXELS_PER_SAMPLE_PATTERN_X_AMD: GLenum = GLenum(0x91AE);

pub const GL_PIXELS_PER_SAMPLE_PATTERN_Y_AMD: GLenum = GLenum(0x91AF);

///
/// * Group: [`MemoryBarrierMask`]
pub const GL_PIXEL_BUFFER_BARRIER_BIT: GLbitfield = GLbitfield(0x00000080);

///
/// * Group: [`MemoryBarrierMask`]
pub const GL_PIXEL_BUFFER_BARRIER_BIT_EXT: GLbitfield = GLbitfield(0x00000080);

pub const GL_PIXEL_COUNTER_BITS_NV: GLenum = GLenum(0x8864);

///
/// * Group: [`OcclusionQueryParameterNameNV`]
pub const GL_PIXEL_COUNT_AVAILABLE_NV: GLenum = GLenum(0x8867);

///
/// * Group: [`OcclusionQueryParameterNameNV`]
pub const GL_PIXEL_COUNT_NV: GLenum = GLenum(0x8866);

///
/// * Group: [`PixelTransformPNameEXT`]
pub const GL_PIXEL_CUBIC_WEIGHT_EXT: GLenum = GLenum(0x8333);

///
/// * Group: [`PixelTexGenParameterNameSGIS`]
pub const GL_PIXEL_FRAGMENT_ALPHA_SOURCE_SGIS: GLenum = GLenum(0x8355);

///
/// * Group: [`PixelTexGenParameterNameSGIS`]
pub const GL_PIXEL_FRAGMENT_RGB_SOURCE_SGIS: GLenum = GLenum(0x8354);

pub const GL_PIXEL_GROUP_COLOR_SGIS: GLenum = GLenum(0x8356);

///
/// * Group: [`PixelTransformPNameEXT`]
pub const GL_PIXEL_MAG_FILTER_EXT: GLenum = GLenum(0x8331);

///
/// * Group: [`PixelMap`], [`GetPixelMap`]
pub const GL_PIXEL_MAP_A_TO_A: GLenum = GLenum(0x0C79);

///
/// * Group: [`GetPName`]
pub const GL_PIXEL_MAP_A_TO_A_SIZE: GLenum = GLenum(0x0CB9);

///
/// * Group: [`PixelMap`], [`GetPixelMap`]
pub const GL_PIXEL_MAP_B_TO_B: GLenum = GLenum(0x0C78);

///
/// * Group: [`GetPName`]
pub const GL_PIXEL_MAP_B_TO_B_SIZE: GLenum = GLenum(0x0CB8);

///
/// * Group: [`PixelMap`], [`GetPixelMap`]
pub const GL_PIXEL_MAP_G_TO_G: GLenum = GLenum(0x0C77);

///
/// * Group: [`GetPName`]
pub const GL_PIXEL_MAP_G_TO_G_SIZE: GLenum = GLenum(0x0CB7);

///
/// * Group: [`PixelMap`], [`GetPixelMap`]
pub const GL_PIXEL_MAP_I_TO_A: GLenum = GLenum(0x0C75);

///
/// * Group: [`GetPName`]
pub const GL_PIXEL_MAP_I_TO_A_SIZE: GLenum = GLenum(0x0CB5);

///
/// * Group: [`PixelMap`], [`GetPixelMap`]
pub const GL_PIXEL_MAP_I_TO_B: GLenum = GLenum(0x0C74);

///
/// * Group: [`GetPName`]
pub const GL_PIXEL_MAP_I_TO_B_SIZE: GLenum = GLenum(0x0CB4);

///
/// * Group: [`PixelMap`], [`GetPixelMap`]
pub const GL_PIXEL_MAP_I_TO_G: GLenum = GLenum(0x0C73);

///
/// * Group: [`GetPName`]
pub const GL_PIXEL_MAP_I_TO_G_SIZE: GLenum = GLenum(0x0CB3);

///
/// * Group: [`PixelMap`], [`GetPixelMap`]
pub const GL_PIXEL_MAP_I_TO_I: GLenum = GLenum(0x0C70);

///
/// * Group: [`GetPName`]
pub const GL_PIXEL_MAP_I_TO_I_SIZE: GLenum = GLenum(0x0CB0);

///
/// * Group: [`PixelMap`], [`GetPixelMap`]
pub const GL_PIXEL_MAP_I_TO_R: GLenum = GLenum(0x0C72);

///
/// * Group: [`GetPName`]
pub const GL_PIXEL_MAP_I_TO_R_SIZE: GLenum = GLenum(0x0CB2);

///
/// * Group: [`PixelMap`], [`GetPixelMap`]
pub const GL_PIXEL_MAP_R_TO_R: GLenum = GLenum(0x0C76);

///
/// * Group: [`GetPName`]
pub const GL_PIXEL_MAP_R_TO_R_SIZE: GLenum = GLenum(0x0CB6);

///
/// * Group: [`PixelMap`], [`GetPixelMap`]
pub const GL_PIXEL_MAP_S_TO_S: GLenum = GLenum(0x0C71);

///
/// * Group: [`GetPName`]
pub const GL_PIXEL_MAP_S_TO_S_SIZE: GLenum = GLenum(0x0CB1);

///
/// * Group: [`PixelTransformPNameEXT`]
pub const GL_PIXEL_MIN_FILTER_EXT: GLenum = GLenum(0x8332);

///
/// * Group: [`AttribMask`]
pub const GL_PIXEL_MODE_BIT: GLbitfield = GLbitfield(0x00000020);

///
/// * Group: [`CopyBufferSubDataTarget`], [`BufferTargetARB`],
///   [`BufferStorageTarget`]
pub const GL_PIXEL_PACK_BUFFER: GLenum = GLenum(0x88EB);

pub const GL_PIXEL_PACK_BUFFER_ARB: GLenum = GLenum(0x88EB);

///
/// * Group: [`GetPName`]
pub const GL_PIXEL_PACK_BUFFER_BINDING: GLenum = GLenum(0x88ED);

pub const GL_PIXEL_PACK_BUFFER_BINDING_ARB: GLenum = GLenum(0x88ED);

pub const GL_PIXEL_PACK_BUFFER_BINDING_EXT: GLenum = GLenum(0x88ED);

pub const GL_PIXEL_PACK_BUFFER_BINDING_NV: GLenum = GLenum(0x88ED);

pub const GL_PIXEL_PACK_BUFFER_EXT: GLenum = GLenum(0x88EB);

pub const GL_PIXEL_PACK_BUFFER_NV: GLenum = GLenum(0x88EB);

///
/// * Group: [`PixelStoreSubsampleRate`]
pub const GL_PIXEL_SUBSAMPLE_2424_SGIX: GLenum = GLenum(0x85A3);

///
/// * Group: [`PixelStoreSubsampleRate`]
pub const GL_PIXEL_SUBSAMPLE_4242_SGIX: GLenum = GLenum(0x85A4);

///
/// * Group: [`PixelStoreSubsampleRate`]
pub const GL_PIXEL_SUBSAMPLE_4444_SGIX: GLenum = GLenum(0x85A2);

///
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_PIXEL_TEXTURE_SGIS: GLenum = GLenum(0x8353);

///
/// * Group: [`PixelTexGenMode`], [`PixelTexGenModeSGIX`]
pub const GL_PIXEL_TEX_GEN_ALPHA_LS_SGIX: GLenum = GLenum(0x8189);

///
/// * Group: [`PixelTexGenMode`], [`PixelTexGenModeSGIX`]
pub const GL_PIXEL_TEX_GEN_ALPHA_MS_SGIX: GLenum = GLenum(0x818A);

///
/// * Group: [`PixelTexGenMode`]
pub const GL_PIXEL_TEX_GEN_ALPHA_NO_REPLACE_SGIX: GLenum = GLenum(0x8188);

///
/// * Group: [`PixelTexGenMode`]
pub const GL_PIXEL_TEX_GEN_ALPHA_REPLACE_SGIX: GLenum = GLenum(0x8187);

///
/// * Group: [`GetPName`]
pub const GL_PIXEL_TEX_GEN_MODE_SGIX: GLenum = GLenum(0x832B);

///
/// * Group: [`TextureMinFilter`], [`PixelTexGenModeSGIX`], [`TextureMagFilter`]
pub const GL_PIXEL_TEX_GEN_Q_CEILING_SGIX: GLenum = GLenum(0x8184);

///
/// * Group: [`TextureMinFilter`], [`PixelTexGenModeSGIX`], [`TextureMagFilter`]
pub const GL_PIXEL_TEX_GEN_Q_FLOOR_SGIX: GLenum = GLenum(0x8186);

///
/// * Group: [`TextureMinFilter`], [`PixelTexGenModeSGIX`], [`TextureMagFilter`]
pub const GL_PIXEL_TEX_GEN_Q_ROUND_SGIX: GLenum = GLenum(0x8185);

///
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_PIXEL_TEX_GEN_SGIX: GLenum = GLenum(0x8139);

///
/// * Group: [`GetPName`]
pub const GL_PIXEL_TILE_BEST_ALIGNMENT_SGIX: GLenum = GLenum(0x813E);

///
/// * Group: [`GetPName`]
pub const GL_PIXEL_TILE_CACHE_INCREMENT_SGIX: GLenum = GLenum(0x813F);

///
/// * Group: [`PixelStoreParameter`], [`GetPName`]
pub const GL_PIXEL_TILE_CACHE_SIZE_SGIX: GLenum = GLenum(0x8145);

///
/// * Group: [`PixelStoreParameter`], [`GetPName`]
pub const GL_PIXEL_TILE_GRID_DEPTH_SGIX: GLenum = GLenum(0x8144);

///
/// * Group: [`PixelStoreParameter`], [`GetPName`]
pub const GL_PIXEL_TILE_GRID_HEIGHT_SGIX: GLenum = GLenum(0x8143);

///
/// * Group: [`PixelStoreParameter`], [`GetPName`]
pub const GL_PIXEL_TILE_GRID_WIDTH_SGIX: GLenum = GLenum(0x8142);

///
/// * Group: [`PixelStoreParameter`], [`GetPName`]
pub const GL_PIXEL_TILE_HEIGHT_SGIX: GLenum = GLenum(0x8141);

///
/// * Group: [`PixelStoreParameter`], [`GetPName`]
pub const GL_PIXEL_TILE_WIDTH_SGIX: GLenum = GLenum(0x8140);

///
/// * Group: [`PixelTransformTargetEXT`]
pub const GL_PIXEL_TRANSFORM_2D_EXT: GLenum = GLenum(0x8330);

pub const GL_PIXEL_TRANSFORM_2D_MATRIX_EXT: GLenum = GLenum(0x8338);

pub const GL_PIXEL_TRANSFORM_2D_STACK_DEPTH_EXT: GLenum = GLenum(0x8336);

///
/// * Group: [`CopyBufferSubDataTarget`], [`BufferTargetARB`],
///   [`BufferStorageTarget`]
pub const GL_PIXEL_UNPACK_BUFFER: GLenum = GLenum(0x88EC);

pub const GL_PIXEL_UNPACK_BUFFER_ARB: GLenum = GLenum(0x88EC);

///
/// * Group: [`GetPName`]
pub const GL_PIXEL_UNPACK_BUFFER_BINDING: GLenum = GLenum(0x88EF);

pub const GL_PIXEL_UNPACK_BUFFER_BINDING_ARB: GLenum = GLenum(0x88EF);

pub const GL_PIXEL_UNPACK_BUFFER_BINDING_EXT: GLenum = GLenum(0x88EF);

pub const GL_PIXEL_UNPACK_BUFFER_BINDING_NV: GLenum = GLenum(0x88EF);

pub const GL_PIXEL_UNPACK_BUFFER_EXT: GLenum = GLenum(0x88EC);

pub const GL_PIXEL_UNPACK_BUFFER_NV: GLenum = GLenum(0x88EC);

pub const GL_PLUS_CLAMPED_ALPHA_NV: GLenum = GLenum(0x92B2);

pub const GL_PLUS_CLAMPED_NV: GLenum = GLenum(0x92B1);

pub const GL_PLUS_DARKER_NV: GLenum = GLenum(0x9292);

pub const GL_PLUS_NV: GLenum = GLenum(0x9291);

pub const GL_PN_TRIANGLES_ATI: GLenum = GLenum(0x87F0);

///
/// * Group: [`PNTrianglesPNameATI`]
pub const GL_PN_TRIANGLES_NORMAL_MODE_ATI: GLenum = GLenum(0x87F3);

pub const GL_PN_TRIANGLES_NORMAL_MODE_LINEAR_ATI: GLenum = GLenum(0x87F7);

pub const GL_PN_TRIANGLES_NORMAL_MODE_QUADRATIC_ATI: GLenum = GLenum(0x87F8);

///
/// * Group: [`PNTrianglesPNameATI`]
pub const GL_PN_TRIANGLES_POINT_MODE_ATI: GLenum = GLenum(0x87F2);

pub const GL_PN_TRIANGLES_POINT_MODE_CUBIC_ATI: GLenum = GLenum(0x87F6);

pub const GL_PN_TRIANGLES_POINT_MODE_LINEAR_ATI: GLenum = GLenum(0x87F5);

///
/// * Group: [`PNTrianglesPNameATI`]
pub const GL_PN_TRIANGLES_TESSELATION_LEVEL_ATI: GLenum = GLenum(0x87F4);

///
/// * Group: [`PolygonMode`], [`MeshMode1`], [`MeshMode2`]
pub const GL_POINT: GLenum = GLenum(0x1B00);

///
/// * Group: [`PrimitiveType`]
pub const GL_POINTS: GLenum = GLenum(0x0000);

///
/// * Group: [`AttribMask`]
pub const GL_POINT_BIT: GLbitfield = GLbitfield(0x00000002);

///
/// * Group: [`PointParameterNameSGIS`]
pub const GL_POINT_DISTANCE_ATTENUATION: GLenum = GLenum(0x8129);

///
/// * Group: [`PointParameterNameSGIS`]
pub const GL_POINT_DISTANCE_ATTENUATION_ARB: GLenum = GLenum(0x8129);

///
/// * Group: [`PointParameterNameSGIS`], [`PointParameterNameARB`], [`GetPName`]
pub const GL_POINT_FADE_THRESHOLD_SIZE: GLenum = GLenum(0x8128);

///
/// * Group: [`PointParameterNameSGIS`]
pub const GL_POINT_FADE_THRESHOLD_SIZE_ARB: GLenum = GLenum(0x8128);

///
/// * Group: [`PointParameterNameSGIS`], [`PointParameterNameARB`]
pub const GL_POINT_FADE_THRESHOLD_SIZE_EXT: GLenum = GLenum(0x8128);

///
/// * Group: [`PointParameterNameSGIS`], [`GetPName`]
pub const GL_POINT_FADE_THRESHOLD_SIZE_SGIS: GLenum = GLenum(0x8128);

pub const GL_POINT_NV: GLenum = GLenum(0x1B00);

///
/// * Group: [`GetPName`]
pub const GL_POINT_SIZE: GLenum = GLenum(0x0B11);

pub const GL_POINT_SIZE_ARRAY_BUFFER_BINDING_OES: GLenum = GLenum(0x8B9F);

pub const GL_POINT_SIZE_ARRAY_OES: GLenum = GLenum(0x8B9C);

pub const GL_POINT_SIZE_ARRAY_POINTER_OES: GLenum = GLenum(0x898C);

pub const GL_POINT_SIZE_ARRAY_STRIDE_OES: GLenum = GLenum(0x898B);

pub const GL_POINT_SIZE_ARRAY_TYPE_OES: GLenum = GLenum(0x898A);

///
/// * Group: [`GetPName`]
pub const GL_POINT_SIZE_GRANULARITY: GLenum = GLenum(0x0B13);

///
/// * Group: [`PointParameterNameSGIS`]
pub const GL_POINT_SIZE_MAX: GLenum = GLenum(0x8127);

///
/// * Group: [`PointParameterNameSGIS`]
pub const GL_POINT_SIZE_MAX_ARB: GLenum = GLenum(0x8127);

///
/// * Group: [`PointParameterNameSGIS`], [`PointParameterNameARB`]
pub const GL_POINT_SIZE_MAX_EXT: GLenum = GLenum(0x8127);

///
/// * Group: [`PointParameterNameSGIS`], [`GetPName`]
pub const GL_POINT_SIZE_MAX_SGIS: GLenum = GLenum(0x8127);

///
/// * Group: [`PointParameterNameSGIS`]
pub const GL_POINT_SIZE_MIN: GLenum = GLenum(0x8126);

///
/// * Group: [`PointParameterNameSGIS`]
pub const GL_POINT_SIZE_MIN_ARB: GLenum = GLenum(0x8126);

///
/// * Group: [`PointParameterNameSGIS`], [`PointParameterNameARB`]
pub const GL_POINT_SIZE_MIN_EXT: GLenum = GLenum(0x8126);

///
/// * Group: [`PointParameterNameSGIS`], [`GetPName`]
pub const GL_POINT_SIZE_MIN_SGIS: GLenum = GLenum(0x8126);

///
/// * Group: [`GetPName`]
pub const GL_POINT_SIZE_RANGE: GLenum = GLenum(0x0B12);

///
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_POINT_SMOOTH: GLenum = GLenum(0x0B10);

///
/// * Group: [`HintTarget`], [`GetPName`]
pub const GL_POINT_SMOOTH_HINT: GLenum = GLenum(0x0C51);

pub const GL_POINT_SPRITE: GLenum = GLenum(0x8861);

pub const GL_POINT_SPRITE_ARB: GLenum = GLenum(0x8861);

pub const GL_POINT_SPRITE_COORD_ORIGIN: GLenum = GLenum(0x8CA0);

pub const GL_POINT_SPRITE_NV: GLenum = GLenum(0x8861);

pub const GL_POINT_SPRITE_OES: GLenum = GLenum(0x8861);

pub const GL_POINT_SPRITE_R_MODE_NV: GLenum = GLenum(0x8863);

///
/// * Group: [`FeedBackToken`]
pub const GL_POINT_TOKEN: GLenum = GLenum(0x0701);

///
/// * Group: [`PrimitiveType`]
pub const GL_POLYGON: GLenum = GLenum(0x0009);

///
/// * Group: [`AttribMask`]
pub const GL_POLYGON_BIT: GLbitfield = GLbitfield(0x00000008);

///
/// * Group: [`GetPName`]
pub const GL_POLYGON_MODE: GLenum = GLenum(0x0B40);

pub const GL_POLYGON_MODE_NV: GLenum = GLenum(0x0B40);

///
/// * Group: [`GetPName`]
pub const GL_POLYGON_OFFSET_BIAS_EXT: GLenum = GLenum(0x8039);

pub const GL_POLYGON_OFFSET_CLAMP: GLenum = GLenum(0x8E1B);

///
/// * Alias Of: [`GL_POLYGON_OFFSET_CLAMP`]
pub const GL_POLYGON_OFFSET_CLAMP_EXT: GLenum = GL_POLYGON_OFFSET_CLAMP;

///
/// * Group: [`CommandOpcodesNV`]
pub const GL_POLYGON_OFFSET_COMMAND_NV: GLenum = GLenum(0x000E);

pub const GL_POLYGON_OFFSET_EXT: GLenum = GLenum(0x8037);

///
/// * Group: [`GetPName`]
pub const GL_POLYGON_OFFSET_FACTOR: GLenum = GLenum(0x8038);

pub const GL_POLYGON_OFFSET_FACTOR_EXT: GLenum = GLenum(0x8038);

///
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_POLYGON_OFFSET_FILL: GLenum = GLenum(0x8037);

///
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_POLYGON_OFFSET_LINE: GLenum = GLenum(0x2A02);

pub const GL_POLYGON_OFFSET_LINE_NV: GLenum = GLenum(0x2A02);

///
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_POLYGON_OFFSET_POINT: GLenum = GLenum(0x2A01);

pub const GL_POLYGON_OFFSET_POINT_NV: GLenum = GLenum(0x2A01);

///
/// * Group: [`GetPName`]
pub const GL_POLYGON_OFFSET_UNITS: GLenum = GLenum(0x2A00);

///
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_POLYGON_SMOOTH: GLenum = GLenum(0x0B41);

///
/// * Group: [`HintTarget`], [`GetPName`]
pub const GL_POLYGON_SMOOTH_HINT: GLenum = GLenum(0x0C53);

///
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_POLYGON_STIPPLE: GLenum = GLenum(0x0B42);

///
/// * Group: [`AttribMask`]
pub const GL_POLYGON_STIPPLE_BIT: GLbitfield = GLbitfield(0x00000010);

///
/// * Group: [`FeedBackToken`]
pub const GL_POLYGON_TOKEN: GLenum = GLenum(0x0703);

///
/// * Group: [`LightParameter`], [`FragmentLightParameterSGIX`]
pub const GL_POSITION: GLenum = GLenum(0x1203);

///
/// * Group: [`PixelTransferParameter`]
pub const GL_POST_COLOR_MATRIX_ALPHA_BIAS: GLenum = GLenum(0x80BB);

///
/// * Group: [`PixelTransferParameter`], [`GetPName`]
pub const GL_POST_COLOR_MATRIX_ALPHA_BIAS_SGI: GLenum = GLenum(0x80BB);

///
/// * Group: [`PixelTransferParameter`]
pub const GL_POST_COLOR_MATRIX_ALPHA_SCALE: GLenum = GLenum(0x80B7);

///
/// * Group: [`PixelTransferParameter`], [`GetPName`]
pub const GL_POST_COLOR_MATRIX_ALPHA_SCALE_SGI: GLenum = GLenum(0x80B7);

///
/// * Group: [`PixelTransferParameter`]
pub const GL_POST_COLOR_MATRIX_BLUE_BIAS: GLenum = GLenum(0x80BA);

///
/// * Group: [`PixelTransferParameter`], [`GetPName`]
pub const GL_POST_COLOR_MATRIX_BLUE_BIAS_SGI: GLenum = GLenum(0x80BA);

///
/// * Group: [`PixelTransferParameter`]
pub const GL_POST_COLOR_MATRIX_BLUE_SCALE: GLenum = GLenum(0x80B6);

///
/// * Group: [`PixelTransferParameter`], [`GetPName`]
pub const GL_POST_COLOR_MATRIX_BLUE_SCALE_SGI: GLenum = GLenum(0x80B6);

///
/// * Group: [`ColorTableTarget`], [`ColorTableTargetSGI`], [`EnableCap`]
pub const GL_POST_COLOR_MATRIX_COLOR_TABLE: GLenum = GLenum(0x80D2);

///
/// * Group: [`GetPName`], [`ColorTableTargetSGI`], [`EnableCap`]
pub const GL_POST_COLOR_MATRIX_COLOR_TABLE_SGI: GLenum = GLenum(0x80D2);

///
/// * Group: [`PixelTransferParameter`]
pub const GL_POST_COLOR_MATRIX_GREEN_BIAS: GLenum = GLenum(0x80B9);

///
/// * Group: [`PixelTransferParameter`], [`GetPName`]
pub const GL_POST_COLOR_MATRIX_GREEN_BIAS_SGI: GLenum = GLenum(0x80B9);

///
/// * Group: [`PixelTransferParameter`]
pub const GL_POST_COLOR_MATRIX_GREEN_SCALE: GLenum = GLenum(0x80B5);

///
/// * Group: [`PixelTransferParameter`], [`GetPName`]
pub const GL_POST_COLOR_MATRIX_GREEN_SCALE_SGI: GLenum = GLenum(0x80B5);

///
/// * Group: [`PixelTransferParameter`]
pub const GL_POST_COLOR_MATRIX_RED_BIAS: GLenum = GLenum(0x80B8);

///
/// * Group: [`PixelTransferParameter`], [`GetPName`]
pub const GL_POST_COLOR_MATRIX_RED_BIAS_SGI: GLenum = GLenum(0x80B8);

///
/// * Group: [`PixelTransferParameter`]
pub const GL_POST_COLOR_MATRIX_RED_SCALE: GLenum = GLenum(0x80B4);

///
/// * Group: [`PixelTransferParameter`], [`GetPName`]
pub const GL_POST_COLOR_MATRIX_RED_SCALE_SGI: GLenum = GLenum(0x80B4);

///
/// * Group: [`PixelTransferParameter`]
pub const GL_POST_CONVOLUTION_ALPHA_BIAS: GLenum = GLenum(0x8023);

///
/// * Group: [`PixelTransferParameter`], [`GetPName`]
pub const GL_POST_CONVOLUTION_ALPHA_BIAS_EXT: GLenum = GLenum(0x8023);

///
/// * Group: [`PixelTransferParameter`]
pub const GL_POST_CONVOLUTION_ALPHA_SCALE: GLenum = GLenum(0x801F);

///
/// * Group: [`PixelTransferParameter`], [`GetPName`]
pub const GL_POST_CONVOLUTION_ALPHA_SCALE_EXT: GLenum = GLenum(0x801F);

///
/// * Group: [`PixelTransferParameter`]
pub const GL_POST_CONVOLUTION_BLUE_BIAS: GLenum = GLenum(0x8022);

///
/// * Group: [`PixelTransferParameter`], [`GetPName`]
pub const GL_POST_CONVOLUTION_BLUE_BIAS_EXT: GLenum = GLenum(0x8022);

///
/// * Group: [`PixelTransferParameter`]
pub const GL_POST_CONVOLUTION_BLUE_SCALE: GLenum = GLenum(0x801E);

///
/// * Group: [`PixelTransferParameter`], [`GetPName`]
pub const GL_POST_CONVOLUTION_BLUE_SCALE_EXT: GLenum = GLenum(0x801E);

///
/// * Group: [`ColorTableTarget`], [`ColorTableTargetSGI`], [`EnableCap`]
pub const GL_POST_CONVOLUTION_COLOR_TABLE: GLenum = GLenum(0x80D1);

///
/// * Group: [`GetPName`], [`ColorTableTargetSGI`], [`EnableCap`]
pub const GL_POST_CONVOLUTION_COLOR_TABLE_SGI: GLenum = GLenum(0x80D1);

///
/// * Group: [`PixelTransferParameter`]
pub const GL_POST_CONVOLUTION_GREEN_BIAS: GLenum = GLenum(0x8021);

///
/// * Group: [`PixelTransferParameter`], [`GetPName`]
pub const GL_POST_CONVOLUTION_GREEN_BIAS_EXT: GLenum = GLenum(0x8021);

///
/// * Group: [`PixelTransferParameter`]
pub const GL_POST_CONVOLUTION_GREEN_SCALE: GLenum = GLenum(0x801D);

///
/// * Group: [`PixelTransferParameter`], [`GetPName`]
pub const GL_POST_CONVOLUTION_GREEN_SCALE_EXT: GLenum = GLenum(0x801D);

///
/// * Group: [`PixelTransferParameter`]
pub const GL_POST_CONVOLUTION_RED_BIAS: GLenum = GLenum(0x8020);

///
/// * Group: [`PixelTransferParameter`], [`GetPName`]
pub const GL_POST_CONVOLUTION_RED_BIAS_EXT: GLenum = GLenum(0x8020);

///
/// * Group: [`PixelTransferParameter`]
pub const GL_POST_CONVOLUTION_RED_SCALE: GLenum = GLenum(0x801C);

///
/// * Group: [`PixelTransferParameter`], [`GetPName`]
pub const GL_POST_CONVOLUTION_RED_SCALE_EXT: GLenum = GLenum(0x801C);

pub const GL_POST_IMAGE_TRANSFORM_COLOR_TABLE_HP: GLenum = GLenum(0x8162);

///
/// * Group: [`GetPName`]
pub const GL_POST_TEXTURE_FILTER_BIAS_RANGE_SGIX: GLenum = GLenum(0x817B);

///
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_POST_TEXTURE_FILTER_BIAS_SGIX: GLenum = GLenum(0x8179);

///
/// * Group: [`GetPName`]
pub const GL_POST_TEXTURE_FILTER_SCALE_RANGE_SGIX: GLenum = GLenum(0x817C);

///
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_POST_TEXTURE_FILTER_SCALE_SGIX: GLenum = GLenum(0x817A);

///
/// * Group: [`HintTarget`]
pub const GL_PREFER_DOUBLEBUFFER_HINT_PGI: GLenum = GLenum(0x1A1F8);

pub const GL_PRESENT_DURATION_NV: GLenum = GLenum(0x8E2B);

pub const GL_PRESENT_TIME_NV: GLenum = GLenum(0x8E2A);

///
/// * Group: [`PreserveModeATI`]
pub const GL_PRESERVE_ATI: GLenum = GLenum(0x8762);

pub const GL_PREVIOUS: GLenum = GLenum(0x8578);

pub const GL_PREVIOUS_ARB: GLenum = GLenum(0x8578);

pub const GL_PREVIOUS_EXT: GLenum = GLenum(0x8578);

pub const GL_PREVIOUS_TEXTURE_INPUT_NV: GLenum = GLenum(0x86E4);

///
/// * Group: [`PathColor`]
pub const GL_PRIMARY_COLOR: GLenum = GLenum(0x8577);

pub const GL_PRIMARY_COLOR_ARB: GLenum = GLenum(0x8577);

pub const GL_PRIMARY_COLOR_EXT: GLenum = GLenum(0x8577);

///
/// * Group: [`PathColor`], [`CombinerRegisterNV`]
pub const GL_PRIMARY_COLOR_NV: GLenum = GLenum(0x852C);

///
/// * Group: [`QueryTarget`]
pub const GL_PRIMITIVES_GENERATED: GLenum = GLenum(0x8C87);

pub const GL_PRIMITIVES_GENERATED_EXT: GLenum = GLenum(0x8C87);

pub const GL_PRIMITIVES_GENERATED_NV: GLenum = GLenum(0x8C87);

pub const GL_PRIMITIVES_GENERATED_OES: GLenum = GLenum(0x8C87);

///
/// * Group: [`QueryTarget`]
pub const GL_PRIMITIVES_SUBMITTED: GLenum = GLenum(0x82EF);

///
/// * Alias Of: [`GL_PRIMITIVES_SUBMITTED`]
pub const GL_PRIMITIVES_SUBMITTED_ARB: GLenum = GL_PRIMITIVES_SUBMITTED;

pub const GL_PRIMITIVE_BOUNDING_BOX: GLenum = GLenum(0x92BE);

pub const GL_PRIMITIVE_BOUNDING_BOX_ARB: GLenum = GLenum(0x92BE);

pub const GL_PRIMITIVE_BOUNDING_BOX_EXT: GLenum = GLenum(0x92BE);

pub const GL_PRIMITIVE_BOUNDING_BOX_OES: GLenum = GLenum(0x92BE);

pub const GL_PRIMITIVE_ID_NV: GLenum = GLenum(0x8C7C);

///
/// * Group: [`EnableCap`]
pub const GL_PRIMITIVE_RESTART: GLenum = GLenum(0x8F9D);

///
/// * Group: [`EnableCap`]
pub const GL_PRIMITIVE_RESTART_FIXED_INDEX: GLenum = GLenum(0x8D69);

pub const GL_PRIMITIVE_RESTART_FOR_PATCHES_SUPPORTED: GLenum = GLenum(0x8221);

pub const GL_PRIMITIVE_RESTART_FOR_PATCHES_SUPPORTED_OES: GLenum = GLenum(0x8221);

///
/// * Group: [`GetPName`]
pub const GL_PRIMITIVE_RESTART_INDEX: GLenum = GLenum(0x8F9E);

pub const GL_PRIMITIVE_RESTART_INDEX_NV: GLenum = GLenum(0x8559);

pub const GL_PRIMITIVE_RESTART_NV: GLenum = GLenum(0x8558);

///
/// * Group: [`ObjectIdentifier`]
pub const GL_PROGRAM: GLenum = GLenum(0x82E2);

///
/// * Group: [`GetMultisamplePNameNV`]
pub const GL_PROGRAMMABLE_SAMPLE_LOCATION_ARB: GLenum = GLenum(0x9341);

pub const GL_PROGRAMMABLE_SAMPLE_LOCATION_NV: GLenum = GLenum(0x9341);

pub const GL_PROGRAMMABLE_SAMPLE_LOCATION_TABLE_SIZE_ARB: GLenum = GLenum(0x9340);

pub const GL_PROGRAMMABLE_SAMPLE_LOCATION_TABLE_SIZE_NV: GLenum = GLenum(0x9340);

pub const GL_PROGRAM_ADDRESS_REGISTERS_ARB: GLenum = GLenum(0x88B0);

pub const GL_PROGRAM_ALU_INSTRUCTIONS_ARB: GLenum = GLenum(0x8805);

pub const GL_PROGRAM_ATTRIBS_ARB: GLenum = GLenum(0x88AC);

pub const GL_PROGRAM_ATTRIB_COMPONENTS_NV: GLenum = GLenum(0x8906);

pub const GL_PROGRAM_BINARY_ANGLE: GLenum = GLenum(0x93A6);

///
/// * Group: [`GetPName`]
pub const GL_PROGRAM_BINARY_FORMATS: GLenum = GLenum(0x87FF);

pub const GL_PROGRAM_BINARY_FORMATS_OES: GLenum = GLenum(0x87FF);

pub const GL_PROGRAM_BINARY_FORMAT_MESA: GLenum = GLenum(0x875F);

///
/// * Group: [`ProgramPropertyARB`]
pub const GL_PROGRAM_BINARY_LENGTH: GLenum = GLenum(0x8741);

/// NOT an alias. Accidental reuse of GL_DOT3_RGBA_EXT
pub const GL_PROGRAM_BINARY_LENGTH_OES: GLenum = GLenum(0x8741);

///
/// * Group: [`ProgramParameterPName`], [`HintTarget`]
pub const GL_PROGRAM_BINARY_RETRIEVABLE_HINT: GLenum = GLenum(0x8257);

/// NOT an alias. Accidental reuse of GL_MAP2_VERTEX_ATTRIB7_4_NV
pub const GL_PROGRAM_BINDING_ARB: GLenum = GLenum(0x8677);

pub const GL_PROGRAM_ERROR_POSITION_ARB: GLenum = GLenum(0x864B);

pub const GL_PROGRAM_ERROR_POSITION_NV: GLenum = GLenum(0x864B);

pub const GL_PROGRAM_ERROR_STRING_ARB: GLenum = GLenum(0x8874);

pub const GL_PROGRAM_ERROR_STRING_NV: GLenum = GLenum(0x8874);

pub const GL_PROGRAM_FORMAT_ARB: GLenum = GLenum(0x8876);

///
/// * Group: [`ProgramFormat`]
pub const GL_PROGRAM_FORMAT_ASCII_ARB: GLenum = GLenum(0x8875);

///
/// * Group: [`ProgramInterface`]
pub const GL_PROGRAM_INPUT: GLenum = GLenum(0x92E3);

pub const GL_PROGRAM_INSTRUCTIONS_ARB: GLenum = GLenum(0x88A0);

pub const GL_PROGRAM_KHR: GLenum = GLenum(0x82E2);

pub const GL_PROGRAM_LENGTH_ARB: GLenum = GLenum(0x8627);

pub const GL_PROGRAM_LENGTH_NV: GLenum = GLenum(0x8627);

pub const GL_PROGRAM_MATRIX_EXT: GLenum = GLenum(0x8E2D);

pub const GL_PROGRAM_MATRIX_STACK_DEPTH_EXT: GLenum = GLenum(0x8E2F);

pub const GL_PROGRAM_NATIVE_ADDRESS_REGISTERS_ARB: GLenum = GLenum(0x88B2);

pub const GL_PROGRAM_NATIVE_ALU_INSTRUCTIONS_ARB: GLenum = GLenum(0x8808);

pub const GL_PROGRAM_NATIVE_ATTRIBS_ARB: GLenum = GLenum(0x88AE);

pub const GL_PROGRAM_NATIVE_INSTRUCTIONS_ARB: GLenum = GLenum(0x88A2);

pub const GL_PROGRAM_NATIVE_PARAMETERS_ARB: GLenum = GLenum(0x88AA);

pub const GL_PROGRAM_NATIVE_TEMPORARIES_ARB: GLenum = GLenum(0x88A6);

pub const GL_PROGRAM_NATIVE_TEX_INDIRECTIONS_ARB: GLenum = GLenum(0x880A);

pub const GL_PROGRAM_NATIVE_TEX_INSTRUCTIONS_ARB: GLenum = GLenum(0x8809);

pub const GL_PROGRAM_OBJECT_ARB: GLenum = GLenum(0x8B40);

pub const GL_PROGRAM_OBJECT_EXT: GLenum = GLenum(0x8B40);

///
/// * Group: [`ProgramInterface`]
pub const GL_PROGRAM_OUTPUT: GLenum = GLenum(0x92E4);

pub const GL_PROGRAM_PARAMETERS_ARB: GLenum = GLenum(0x88A8);

///
/// * Group: [`VertexAttribEnumNV`]
pub const GL_PROGRAM_PARAMETER_NV: GLenum = GLenum(0x8644);

///
/// * Group: [`ObjectIdentifier`]
pub const GL_PROGRAM_PIPELINE: GLenum = GLenum(0x82E4);

///
/// * Group: [`GetPName`]
pub const GL_PROGRAM_PIPELINE_BINDING: GLenum = GLenum(0x825A);

pub const GL_PROGRAM_PIPELINE_BINDING_EXT: GLenum = GLenum(0x825A);

pub const GL_PROGRAM_PIPELINE_KHR: GLenum = GLenum(0x82E4);

pub const GL_PROGRAM_PIPELINE_OBJECT_EXT: GLenum = GLenum(0x8A4F);

///
/// * Group: [`GetPName`], [`EnableCap`]
/// * Alias Of: [`GL_VERTEX_PROGRAM_POINT_SIZE`]
pub const GL_PROGRAM_POINT_SIZE: GLenum = GL_VERTEX_PROGRAM_POINT_SIZE;

pub const GL_PROGRAM_POINT_SIZE_ARB: GLenum = GLenum(0x8642);

pub const GL_PROGRAM_POINT_SIZE_EXT: GLenum = GLenum(0x8642);

pub const GL_PROGRAM_RESIDENT_NV: GLenum = GLenum(0x8647);

pub const GL_PROGRAM_RESULT_COMPONENTS_NV: GLenum = GLenum(0x8907);

///
/// * Group: [`ProgramParameterPName`]
pub const GL_PROGRAM_SEPARABLE: GLenum = GLenum(0x8258);

pub const GL_PROGRAM_SEPARABLE_EXT: GLenum = GLenum(0x8258);

///
/// * Group: [`ProgramStringProperty`]
pub const GL_PROGRAM_STRING_ARB: GLenum = GLenum(0x8628);

pub const GL_PROGRAM_STRING_NV: GLenum = GLenum(0x8628);

pub const GL_PROGRAM_TARGET_NV: GLenum = GLenum(0x8646);

pub const GL_PROGRAM_TEMPORARIES_ARB: GLenum = GLenum(0x88A4);

pub const GL_PROGRAM_TEX_INDIRECTIONS_ARB: GLenum = GLenum(0x8807);

pub const GL_PROGRAM_TEX_INSTRUCTIONS_ARB: GLenum = GLenum(0x8806);

pub const GL_PROGRAM_UNDER_NATIVE_LIMITS_ARB: GLenum = GLenum(0x88B6);

///
/// * Group: [`MatrixMode`]
pub const GL_PROJECTION: GLenum = GLenum(0x1701);

///
/// * Group: [`GetPName`]
pub const GL_PROJECTION_MATRIX: GLenum = GLenum(0x0BA7);

pub const GL_PROJECTION_MATRIX_FLOAT_AS_INT_BITS_OES: GLenum = GLenum(0x898E);

///
/// * Group: [`GetPName`]
pub const GL_PROJECTION_STACK_DEPTH: GLenum = GLenum(0x0BA4);

///
/// * Group: [`MemoryObjectParameterName`]
pub const GL_PROTECTED_MEMORY_OBJECT_EXT: GLenum = GLenum(0x959B);

///
/// * Group: [`GetPName`]
pub const GL_PROVOKING_VERTEX: GLenum = GLenum(0x8E4F);

pub const GL_PROVOKING_VERTEX_EXT: GLenum = GLenum(0x8E4F);

///
/// * Group: [`ColorTableTargetSGI`], [`ColorTableTarget`]
pub const GL_PROXY_COLOR_TABLE: GLenum = GLenum(0x80D3);

///
/// * Group: [`ColorTableTargetSGI`]
pub const GL_PROXY_COLOR_TABLE_SGI: GLenum = GLenum(0x80D3);

///
/// * Group: [`HistogramTarget`], [`HistogramTargetEXT`]
pub const GL_PROXY_HISTOGRAM: GLenum = GLenum(0x8025);

///
/// * Group: [`HistogramTargetEXT`]
pub const GL_PROXY_HISTOGRAM_EXT: GLenum = GLenum(0x8025);

///
/// * Group: [`ColorTableTargetSGI`], [`ColorTableTarget`]
pub const GL_PROXY_POST_COLOR_MATRIX_COLOR_TABLE: GLenum = GLenum(0x80D5);

///
/// * Group: [`ColorTableTargetSGI`]
pub const GL_PROXY_POST_COLOR_MATRIX_COLOR_TABLE_SGI: GLenum = GLenum(0x80D5);

///
/// * Group: [`ColorTableTargetSGI`], [`ColorTableTarget`]
pub const GL_PROXY_POST_CONVOLUTION_COLOR_TABLE: GLenum = GLenum(0x80D4);

///
/// * Group: [`ColorTableTargetSGI`]
pub const GL_PROXY_POST_CONVOLUTION_COLOR_TABLE_SGI: GLenum = GLenum(0x80D4);

pub const GL_PROXY_POST_IMAGE_TRANSFORM_COLOR_TABLE_HP: GLenum = GLenum(0x8163);

///
/// * Group: [`TextureTarget`]
pub const GL_PROXY_TEXTURE_1D: GLenum = GLenum(0x8063);

///
/// * Group: [`TextureTarget`]
pub const GL_PROXY_TEXTURE_1D_ARRAY: GLenum = GLenum(0x8C19);

///
/// * Group: [`TextureTarget`]
pub const GL_PROXY_TEXTURE_1D_ARRAY_EXT: GLenum = GLenum(0x8C19);

///
/// * Group: [`TextureTarget`]
pub const GL_PROXY_TEXTURE_1D_EXT: GLenum = GLenum(0x8063);

pub const GL_PROXY_TEXTURE_1D_STACK_MESAX: GLenum = GLenum(0x875B);

///
/// * Group: [`TextureTarget`]
pub const GL_PROXY_TEXTURE_2D: GLenum = GLenum(0x8064);

///
/// * Group: [`TextureTarget`]
pub const GL_PROXY_TEXTURE_2D_ARRAY: GLenum = GLenum(0x8C1B);

///
/// * Group: [`TextureTarget`]
pub const GL_PROXY_TEXTURE_2D_ARRAY_EXT: GLenum = GLenum(0x8C1B);

///
/// * Group: [`TextureTarget`]
pub const GL_PROXY_TEXTURE_2D_EXT: GLenum = GLenum(0x8064);

///
/// * Group: [`TextureTarget`]
pub const GL_PROXY_TEXTURE_2D_MULTISAMPLE: GLenum = GLenum(0x9101);

///
/// * Group: [`TextureTarget`]
pub const GL_PROXY_TEXTURE_2D_MULTISAMPLE_ARRAY: GLenum = GLenum(0x9103);

pub const GL_PROXY_TEXTURE_2D_STACK_MESAX: GLenum = GLenum(0x875C);

///
/// * Group: [`TextureTarget`]
pub const GL_PROXY_TEXTURE_3D: GLenum = GLenum(0x8070);

///
/// * Group: [`TextureTarget`]
pub const GL_PROXY_TEXTURE_3D_EXT: GLenum = GLenum(0x8070);

///
/// * Group: [`TextureTarget`]
pub const GL_PROXY_TEXTURE_4D_SGIS: GLenum = GLenum(0x8135);

///
/// * Group: [`ColorTableTargetSGI`]
pub const GL_PROXY_TEXTURE_COLOR_TABLE_SGI: GLenum = GLenum(0x80BD);

///
/// * Group: [`TextureTarget`]
pub const GL_PROXY_TEXTURE_CUBE_MAP: GLenum = GLenum(0x851B);

///
/// * Group: [`TextureTarget`]
pub const GL_PROXY_TEXTURE_CUBE_MAP_ARB: GLenum = GLenum(0x851B);

///
/// * Group: [`TextureTarget`]
pub const GL_PROXY_TEXTURE_CUBE_MAP_ARRAY: GLenum = GLenum(0x900B);

///
/// * Group: [`TextureTarget`]
pub const GL_PROXY_TEXTURE_CUBE_MAP_ARRAY_ARB: GLenum = GLenum(0x900B);

///
/// * Group: [`TextureTarget`]
pub const GL_PROXY_TEXTURE_CUBE_MAP_EXT: GLenum = GLenum(0x851B);

///
/// * Group: [`TextureTarget`]
pub const GL_PROXY_TEXTURE_RECTANGLE: GLenum = GLenum(0x84F7);

///
/// * Group: [`TextureTarget`]
pub const GL_PROXY_TEXTURE_RECTANGLE_ARB: GLenum = GLenum(0x84F7);

///
/// * Group: [`TextureTarget`]
pub const GL_PROXY_TEXTURE_RECTANGLE_NV: GLenum = GLenum(0x84F7);

pub const GL_PURGEABLE_APPLE: GLenum = GLenum(0x8A1D);

pub const GL_PURGED_CONTEXT_RESET_NV: GLenum = GLenum(0x92BB);

///
/// * Group: [`TextureCoordName`]
pub const GL_Q: GLenum = GLenum(0x2003);

///
/// * Group: [`LightParameter`], [`FragmentLightParameterSGIX`]
pub const GL_QUADRATIC_ATTENUATION: GLenum = GLenum(0x1209);

///
/// * Group: [`PathCoordType`]
pub const GL_QUADRATIC_CURVE_TO_NV: GLenum = GLenum(0x0A);

///
/// * Group: [`PrimitiveType`]
pub const GL_QUADS: GLenum = GLenum(0x0007);

///
/// * Group: [`PrimitiveType`]
pub const GL_QUADS_EXT: GLenum = GLenum(0x0007);

pub const GL_QUADS_FOLLOW_PROVOKING_VERTEX_CONVENTION: GLenum = GLenum(0x8E4C);

pub const GL_QUADS_FOLLOW_PROVOKING_VERTEX_CONVENTION_EXT: GLenum = GLenum(0x8E4C);

pub const GL_QUADS_OES: GLenum = GLenum(0x0007);

///
/// * Group: [`InternalFormat`]
pub const GL_QUAD_ALPHA4_SGIS: GLenum = GLenum(0x811E);

///
/// * Group: [`InternalFormat`]
pub const GL_QUAD_ALPHA8_SGIS: GLenum = GLenum(0x811F);

///
/// * Group: [`InternalFormat`]
pub const GL_QUAD_INTENSITY4_SGIS: GLenum = GLenum(0x8122);

///
/// * Group: [`InternalFormat`]
pub const GL_QUAD_INTENSITY8_SGIS: GLenum = GLenum(0x8123);

///
/// * Group: [`InternalFormat`]
pub const GL_QUAD_LUMINANCE4_SGIS: GLenum = GLenum(0x8120);

///
/// * Group: [`InternalFormat`]
pub const GL_QUAD_LUMINANCE8_SGIS: GLenum = GLenum(0x8121);

pub const GL_QUAD_MESH_SUN: GLenum = GLenum(0x8614);

///
/// * Group: [`PrimitiveType`]
pub const GL_QUAD_STRIP: GLenum = GLenum(0x0008);

///
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_QUAD_TEXTURE_SELECT_SGIS: GLenum = GLenum(0x8125);

///
/// * Group: [`FragmentShaderDestModMaskATI`]
pub const GL_QUARTER_BIT_ATI: GLbitfield = GLbitfield(0x00000010);

///
/// * Group: [`ObjectIdentifier`]
pub const GL_QUERY: GLenum = GLenum(0x82E3);

///
/// * Group: [`OcclusionQueryEventMaskAMD`]
pub const GL_QUERY_ALL_EVENT_BITS_AMD: GLbitfield = GLbitfield(0xFFFFFFFF);

///
/// * Group: [`CopyBufferSubDataTarget`], [`BufferTargetARB`],
///   [`BufferStorageTarget`]
pub const GL_QUERY_BUFFER: GLenum = GLenum(0x9192);

pub const GL_QUERY_BUFFER_AMD: GLenum = GLenum(0x9192);

///
/// * Group: [`MemoryBarrierMask`]
pub const GL_QUERY_BUFFER_BARRIER_BIT: GLbitfield = GLbitfield(0x00008000);

pub const GL_QUERY_BUFFER_BINDING: GLenum = GLenum(0x9193);

pub const GL_QUERY_BUFFER_BINDING_AMD: GLenum = GLenum(0x9193);

///
/// * Group: [`ConditionalRenderMode`]
pub const GL_QUERY_BY_REGION_NO_WAIT: GLenum = GLenum(0x8E16);

///
/// * Group: [`ConditionalRenderMode`]
pub const GL_QUERY_BY_REGION_NO_WAIT_INVERTED: GLenum = GLenum(0x8E1A);

pub const GL_QUERY_BY_REGION_NO_WAIT_NV: GLenum = GLenum(0x8E16);

///
/// * Group: [`ConditionalRenderMode`]
pub const GL_QUERY_BY_REGION_WAIT: GLenum = GLenum(0x8E15);

///
/// * Group: [`ConditionalRenderMode`]
pub const GL_QUERY_BY_REGION_WAIT_INVERTED: GLenum = GLenum(0x8E19);

pub const GL_QUERY_BY_REGION_WAIT_NV: GLenum = GLenum(0x8E15);

///
/// * Group: [`QueryParameterName`]
pub const GL_QUERY_COUNTER_BITS: GLenum = GLenum(0x8864);

pub const GL_QUERY_COUNTER_BITS_ARB: GLenum = GLenum(0x8864);

pub const GL_QUERY_COUNTER_BITS_EXT: GLenum = GLenum(0x8864);

///
/// * Group: [`OcclusionQueryEventMaskAMD`]
pub const GL_QUERY_DEPTH_BOUNDS_FAIL_EVENT_BIT_AMD: GLbitfield = GLbitfield(0x00000008);

///
/// * Group: [`OcclusionQueryEventMaskAMD`]
pub const GL_QUERY_DEPTH_FAIL_EVENT_BIT_AMD: GLbitfield = GLbitfield(0x00000002);

///
/// * Group: [`OcclusionQueryEventMaskAMD`]
pub const GL_QUERY_DEPTH_PASS_EVENT_BIT_AMD: GLbitfield = GLbitfield(0x00000001);

pub const GL_QUERY_KHR: GLenum = GLenum(0x82E3);

///
/// * Group: [`ConditionalRenderMode`]
pub const GL_QUERY_NO_WAIT: GLenum = GLenum(0x8E14);

///
/// * Group: [`ConditionalRenderMode`]
pub const GL_QUERY_NO_WAIT_INVERTED: GLenum = GLenum(0x8E18);

pub const GL_QUERY_NO_WAIT_NV: GLenum = GLenum(0x8E14);

pub const GL_QUERY_OBJECT_AMD: GLenum = GLenum(0x9153);

pub const GL_QUERY_OBJECT_EXT: GLenum = GLenum(0x9153);

pub const GL_QUERY_RESOURCE_BUFFEROBJECT_NV: GLenum = GLenum(0x9547);

pub const GL_QUERY_RESOURCE_MEMTYPE_VIDMEM_NV: GLenum = GLenum(0x9542);

pub const GL_QUERY_RESOURCE_RENDERBUFFER_NV: GLenum = GLenum(0x9546);

pub const GL_QUERY_RESOURCE_SYS_RESERVED_NV: GLenum = GLenum(0x9544);

pub const GL_QUERY_RESOURCE_TEXTURE_NV: GLenum = GLenum(0x9545);

pub const GL_QUERY_RESOURCE_TYPE_VIDMEM_ALLOC_NV: GLenum = GLenum(0x9540);

///
/// * Group: [`QueryObjectParameterName`]
pub const GL_QUERY_RESULT: GLenum = GLenum(0x8866);

pub const GL_QUERY_RESULT_ARB: GLenum = GLenum(0x8866);

///
/// * Group: [`QueryObjectParameterName`]
pub const GL_QUERY_RESULT_AVAILABLE: GLenum = GLenum(0x8867);

pub const GL_QUERY_RESULT_AVAILABLE_ARB: GLenum = GLenum(0x8867);

pub const GL_QUERY_RESULT_AVAILABLE_EXT: GLenum = GLenum(0x8867);

pub const GL_QUERY_RESULT_EXT: GLenum = GLenum(0x8866);

///
/// * Group: [`QueryObjectParameterName`]
pub const GL_QUERY_RESULT_NO_WAIT: GLenum = GLenum(0x9194);

pub const GL_QUERY_RESULT_NO_WAIT_AMD: GLenum = GLenum(0x9194);

///
/// * Group: [`OcclusionQueryEventMaskAMD`]
pub const GL_QUERY_STENCIL_FAIL_EVENT_BIT_AMD: GLbitfield = GLbitfield(0x00000004);

///
/// * Group: [`QueryObjectParameterName`]
pub const GL_QUERY_TARGET: GLenum = GLenum(0x82EA);

///
/// * Group: [`ConditionalRenderMode`]
pub const GL_QUERY_WAIT: GLenum = GLenum(0x8E13);

///
/// * Group: [`ConditionalRenderMode`]
pub const GL_QUERY_WAIT_INVERTED: GLenum = GLenum(0x8E17);

pub const GL_QUERY_WAIT_NV: GLenum = GLenum(0x8E13);

///
/// * Group: [`TextureCoordName`]
pub const GL_R: GLenum = GLenum(0x2002);

///
/// * Group: [`InternalFormat`]
pub const GL_R11F_G11F_B10F: GLenum = GLenum(0x8C3A);

///
/// * Group: [`InternalFormat`]
pub const GL_R11F_G11F_B10F_APPLE: GLenum = GLenum(0x8C3A);

///
/// * Group: [`InternalFormat`]
pub const GL_R11F_G11F_B10F_EXT: GLenum = GLenum(0x8C3A);

///
/// * Group: [`InternalFormat`]
pub const GL_R16: GLenum = GLenum(0x822A);

///
/// * Group: [`InternalFormat`]
pub const GL_R16F: GLenum = GLenum(0x822D);

///
/// * Group: [`InternalFormat`]
pub const GL_R16F_EXT: GLenum = GLenum(0x822D);

///
/// * Group: [`InternalFormat`]
pub const GL_R16I: GLenum = GLenum(0x8233);

///
/// * Group: [`InternalFormat`]
pub const GL_R16UI: GLenum = GLenum(0x8234);

///
/// * Group: [`InternalFormat`]
pub const GL_R16_EXT: GLenum = GLenum(0x822A);

///
/// * Group: [`InternalFormat`]
pub const GL_R16_SNORM: GLenum = GLenum(0x8F98);

///
/// * Group: [`InternalFormat`]
pub const GL_R16_SNORM_EXT: GLenum = GLenum(0x8F98);

pub const GL_R1UI_C3F_V3F_SUN: GLenum = GLenum(0x85C6);

pub const GL_R1UI_C4F_N3F_V3F_SUN: GLenum = GLenum(0x85C8);

pub const GL_R1UI_C4UB_V3F_SUN: GLenum = GLenum(0x85C5);

pub const GL_R1UI_N3F_V3F_SUN: GLenum = GLenum(0x85C7);

pub const GL_R1UI_T2F_C4F_N3F_V3F_SUN: GLenum = GLenum(0x85CB);

pub const GL_R1UI_T2F_N3F_V3F_SUN: GLenum = GLenum(0x85CA);

pub const GL_R1UI_T2F_V3F_SUN: GLenum = GLenum(0x85C9);

pub const GL_R1UI_V3F_SUN: GLenum = GLenum(0x85C4);

///
/// * Group: [`InternalFormat`]
pub const GL_R32F: GLenum = GLenum(0x822E);

///
/// * Group: [`InternalFormat`]
pub const GL_R32F_EXT: GLenum = GLenum(0x822E);

///
/// * Group: [`InternalFormat`]
pub const GL_R32I: GLenum = GLenum(0x8235);

///
/// * Group: [`InternalFormat`]
pub const GL_R32UI: GLenum = GLenum(0x8236);

///
/// * Group: [`InternalFormat`]
pub const GL_R3_G3_B2: GLenum = GLenum(0x2A10);

///
/// * Group: [`InternalFormat`]
pub const GL_R8: GLenum = GLenum(0x8229);

///
/// * Group: [`InternalFormat`]
pub const GL_R8I: GLenum = GLenum(0x8231);

///
/// * Group: [`InternalFormat`]
pub const GL_R8UI: GLenum = GLenum(0x8232);

///
/// * Group: [`InternalFormat`]
pub const GL_R8_EXT: GLenum = GLenum(0x8229);

///
/// * Group: [`InternalFormat`]
pub const GL_R8_SNORM: GLenum = GLenum(0x8F94);

///
/// * Group: [`EnableCap`]
pub const GL_RASTERIZER_DISCARD: GLenum = GLenum(0x8C89);

pub const GL_RASTERIZER_DISCARD_EXT: GLenum = GLenum(0x8C89);

pub const GL_RASTERIZER_DISCARD_NV: GLenum = GLenum(0x8C89);

pub const GL_RASTER_FIXED_SAMPLE_LOCATIONS_EXT: GLenum = GLenum(0x932A);

pub const GL_RASTER_MULTISAMPLE_EXT: GLenum = GLenum(0x9327);

pub const GL_RASTER_POSITION_UNCLIPPED_IBM: GLenum = GLenum(0x19262);

pub const GL_RASTER_SAMPLES_EXT: GLenum = GLenum(0x9328);

///
/// * Group: [`GetPName`]
pub const GL_READ_BUFFER: GLenum = GLenum(0x0C02);

///
/// * Group: [`GetPName`]
pub const GL_READ_BUFFER_EXT: GLenum = GLenum(0x0C02);

///
/// * Group: [`GetPName`]
pub const GL_READ_BUFFER_NV: GLenum = GLenum(0x0C02);

///
/// * Group: [`CheckFramebufferStatusTarget`], [`FramebufferTarget`]
pub const GL_READ_FRAMEBUFFER: GLenum = GLenum(0x8CA8);

pub const GL_READ_FRAMEBUFFER_ANGLE: GLenum = GLenum(0x8CA8);

pub const GL_READ_FRAMEBUFFER_APPLE: GLenum = GLenum(0x8CA8);

///
/// * Group: [`GetPName`]
pub const GL_READ_FRAMEBUFFER_BINDING: GLenum = GLenum(0x8CAA);

pub const GL_READ_FRAMEBUFFER_BINDING_ANGLE: GLenum = GLenum(0x8CAA);

pub const GL_READ_FRAMEBUFFER_BINDING_APPLE: GLenum = GLenum(0x8CAA);

pub const GL_READ_FRAMEBUFFER_BINDING_EXT: GLenum = GLenum(0x8CAA);

pub const GL_READ_FRAMEBUFFER_BINDING_NV: GLenum = GLenum(0x8CAA);

pub const GL_READ_FRAMEBUFFER_EXT: GLenum = GLenum(0x8CA8);

pub const GL_READ_FRAMEBUFFER_NV: GLenum = GLenum(0x8CA8);

///
/// * Group: [`BufferAccessARB`]
pub const GL_READ_ONLY: GLenum = GLenum(0x88B8);

pub const GL_READ_ONLY_ARB: GLenum = GLenum(0x88B8);

///
/// * Group: [`InternalFormatPName`]
pub const GL_READ_PIXELS: GLenum = GLenum(0x828C);

///
/// * Group: [`InternalFormatPName`]
pub const GL_READ_PIXELS_FORMAT: GLenum = GLenum(0x828D);

///
/// * Group: [`InternalFormatPName`]
pub const GL_READ_PIXELS_TYPE: GLenum = GLenum(0x828E);

pub const GL_READ_PIXEL_DATA_RANGE_LENGTH_NV: GLenum = GLenum(0x887B);

///
/// * Group: [`PixelDataRangeTargetNV`]
pub const GL_READ_PIXEL_DATA_RANGE_NV: GLenum = GLenum(0x8879);

pub const GL_READ_PIXEL_DATA_RANGE_POINTER_NV: GLenum = GLenum(0x887D);

///
/// * Group: [`BufferAccessARB`]
pub const GL_READ_WRITE: GLenum = GLenum(0x88BA);

pub const GL_READ_WRITE_ARB: GLenum = GLenum(0x88BA);

pub const GL_RECIP_ADD_SIGNED_ALPHA_IMG: GLenum = GLenum(0x8C05);

///
/// * Group: [`HintTarget`]
pub const GL_RECLAIM_MEMORY_HINT_PGI: GLenum = GLenum(0x1A1FE);

///
/// * Group: [`PathCoordType`]
pub const GL_RECT_NV: GLenum = GLenum(0xF6);

///
/// * Group: [`TextureSwizzle`], [`PixelFormat`], [`InternalFormat`]
pub const GL_RED: GLenum = GLenum(0x1903);

///
/// * Group: [`ConvolutionBorderModeEXT`]
pub const GL_REDUCE: GLenum = GLenum(0x8016);

///
/// * Group: [`ConvolutionBorderModeEXT`]
pub const GL_REDUCE_EXT: GLenum = GLenum(0x8016);

///
/// * Group: [`PixelTransferParameter`], [`GetPName`]
pub const GL_RED_BIAS: GLenum = GLenum(0x0D15);

///
/// * Group: [`GetPName`]
pub const GL_RED_BITS: GLenum = GLenum(0x0D52);

///
/// * Group: [`FragmentShaderDestMaskATI`]
pub const GL_RED_BIT_ATI: GLbitfield = GLbitfield(0x00000001);

///
/// * Group: [`InternalFormat`], [`PixelFormat`]
pub const GL_RED_EXT: GLenum = GLenum(0x1903);

///
/// * Group: [`PixelFormat`]
pub const GL_RED_INTEGER: GLenum = GLenum(0x8D94);

pub const GL_RED_INTEGER_EXT: GLenum = GLenum(0x8D94);

pub const GL_RED_MAX_CLAMP_INGR: GLenum = GLenum(0x8564);

pub const GL_RED_MIN_CLAMP_INGR: GLenum = GLenum(0x8560);

pub const GL_RED_NV: GLenum = GLenum(0x1903);

///
/// * Group: [`PixelTransferParameter`], [`GetPName`]
pub const GL_RED_SCALE: GLenum = GLenum(0x0D14);

pub const GL_RED_SNORM: GLenum = GLenum(0x8F90);

///
/// * Group: [`ProgramResourceProperty`]
pub const GL_REFERENCED_BY_COMPUTE_SHADER: GLenum = GLenum(0x930B);

///
/// * Group: [`ProgramResourceProperty`]
pub const GL_REFERENCED_BY_FRAGMENT_SHADER: GLenum = GLenum(0x930A);

///
/// * Group: [`ProgramResourceProperty`]
pub const GL_REFERENCED_BY_GEOMETRY_SHADER: GLenum = GLenum(0x9309);

pub const GL_REFERENCED_BY_GEOMETRY_SHADER_EXT: GLenum = GLenum(0x9309);

pub const GL_REFERENCED_BY_GEOMETRY_SHADER_OES: GLenum = GLenum(0x9309);

pub const GL_REFERENCED_BY_MESH_SHADER_NV: GLenum = GLenum(0x95A0);

pub const GL_REFERENCED_BY_TASK_SHADER_NV: GLenum = GLenum(0x95A1);

///
/// * Group: [`ProgramResourceProperty`]
pub const GL_REFERENCED_BY_TESS_CONTROL_SHADER: GLenum = GLenum(0x9307);

pub const GL_REFERENCED_BY_TESS_CONTROL_SHADER_EXT: GLenum = GLenum(0x9307);

pub const GL_REFERENCED_BY_TESS_CONTROL_SHADER_OES: GLenum = GLenum(0x9307);

///
/// * Group: [`ProgramResourceProperty`]
pub const GL_REFERENCED_BY_TESS_EVALUATION_SHADER: GLenum = GLenum(0x9308);

pub const GL_REFERENCED_BY_TESS_EVALUATION_SHADER_EXT: GLenum = GLenum(0x9308);

pub const GL_REFERENCED_BY_TESS_EVALUATION_SHADER_OES: GLenum = GLenum(0x9308);

///
/// * Group: [`ProgramResourceProperty`]
pub const GL_REFERENCED_BY_VERTEX_SHADER: GLenum = GLenum(0x9306);

///
/// * Group: [`GetPName`]
pub const GL_REFERENCE_PLANE_EQUATION_SGIX: GLenum = GLenum(0x817E);

///
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_REFERENCE_PLANE_SGIX: GLenum = GLenum(0x817D);

pub const GL_REFLECTION_MAP: GLenum = GLenum(0x8512);

pub const GL_REFLECTION_MAP_ARB: GLenum = GLenum(0x8512);

pub const GL_REFLECTION_MAP_EXT: GLenum = GLenum(0x8512);

pub const GL_REFLECTION_MAP_NV: GLenum = GLenum(0x8512);

pub const GL_REFLECTION_MAP_OES: GLenum = GLenum(0x8512);

pub const GL_REGISTER_COMBINERS_NV: GLenum = GLenum(0x8522);

pub const GL_REG_0_ATI: GLenum = GLenum(0x8921);

pub const GL_REG_10_ATI: GLenum = GLenum(0x892B);

pub const GL_REG_11_ATI: GLenum = GLenum(0x892C);

pub const GL_REG_12_ATI: GLenum = GLenum(0x892D);

pub const GL_REG_13_ATI: GLenum = GLenum(0x892E);

pub const GL_REG_14_ATI: GLenum = GLenum(0x892F);

pub const GL_REG_15_ATI: GLenum = GLenum(0x8930);

pub const GL_REG_16_ATI: GLenum = GLenum(0x8931);

pub const GL_REG_17_ATI: GLenum = GLenum(0x8932);

pub const GL_REG_18_ATI: GLenum = GLenum(0x8933);

pub const GL_REG_19_ATI: GLenum = GLenum(0x8934);

pub const GL_REG_1_ATI: GLenum = GLenum(0x8922);

pub const GL_REG_20_ATI: GLenum = GLenum(0x8935);

pub const GL_REG_21_ATI: GLenum = GLenum(0x8936);

pub const GL_REG_22_ATI: GLenum = GLenum(0x8937);

pub const GL_REG_23_ATI: GLenum = GLenum(0x8938);

pub const GL_REG_24_ATI: GLenum = GLenum(0x8939);

pub const GL_REG_25_ATI: GLenum = GLenum(0x893A);

pub const GL_REG_26_ATI: GLenum = GLenum(0x893B);

pub const GL_REG_27_ATI: GLenum = GLenum(0x893C);

pub const GL_REG_28_ATI: GLenum = GLenum(0x893D);

pub const GL_REG_29_ATI: GLenum = GLenum(0x893E);

pub const GL_REG_2_ATI: GLenum = GLenum(0x8923);

pub const GL_REG_30_ATI: GLenum = GLenum(0x893F);

pub const GL_REG_31_ATI: GLenum = GLenum(0x8940);

pub const GL_REG_3_ATI: GLenum = GLenum(0x8924);

pub const GL_REG_4_ATI: GLenum = GLenum(0x8925);

pub const GL_REG_5_ATI: GLenum = GLenum(0x8926);

pub const GL_REG_6_ATI: GLenum = GLenum(0x8927);

pub const GL_REG_7_ATI: GLenum = GLenum(0x8928);

pub const GL_REG_8_ATI: GLenum = GLenum(0x8929);

pub const GL_REG_9_ATI: GLenum = GLenum(0x892A);

///
/// * Group: [`PathCoordType`]
pub const GL_RELATIVE_ARC_TO_NV: GLenum = GLenum(0xFF);

///
/// * Group: [`PathCoordType`]
pub const GL_RELATIVE_CONIC_CURVE_TO_NV: GLenum = GLenum(0x1B);

///
/// * Group: [`PathCoordType`]
pub const GL_RELATIVE_CUBIC_CURVE_TO_NV: GLenum = GLenum(0x0D);

///
/// * Group: [`PathCoordType`]
pub const GL_RELATIVE_HORIZONTAL_LINE_TO_NV: GLenum = GLenum(0x07);

///
/// * Group: [`PathCoordType`]
pub const GL_RELATIVE_LARGE_CCW_ARC_TO_NV: GLenum = GLenum(0x17);

///
/// * Group: [`PathCoordType`]
pub const GL_RELATIVE_LARGE_CW_ARC_TO_NV: GLenum = GLenum(0x19);

///
/// * Group: [`PathCoordType`]
pub const GL_RELATIVE_LINE_TO_NV: GLenum = GLenum(0x05);

///
/// * Group: [`PathCoordType`]
pub const GL_RELATIVE_MOVE_TO_NV: GLenum = GLenum(0x03);

///
/// * Group: [`PathCoordType`]
pub const GL_RELATIVE_QUADRATIC_CURVE_TO_NV: GLenum = GLenum(0x0B);

///
/// * Group: [`PathCoordType`]
pub const GL_RELATIVE_RECT_NV: GLenum = GLenum(0xF7);

///
/// * Group: [`PathCoordType`]
pub const GL_RELATIVE_ROUNDED_RECT2_NV: GLenum = GLenum(0xEB);

///
/// * Group: [`PathCoordType`]
pub const GL_RELATIVE_ROUNDED_RECT4_NV: GLenum = GLenum(0xED);

///
/// * Group: [`PathCoordType`]
pub const GL_RELATIVE_ROUNDED_RECT8_NV: GLenum = GLenum(0xEF);

///
/// * Group: [`PathCoordType`]
pub const GL_RELATIVE_ROUNDED_RECT_NV: GLenum = GLenum(0xE9);

///
/// * Group: [`PathCoordType`]
pub const GL_RELATIVE_SMALL_CCW_ARC_TO_NV: GLenum = GLenum(0x13);

///
/// * Group: [`PathCoordType`]
pub const GL_RELATIVE_SMALL_CW_ARC_TO_NV: GLenum = GLenum(0x15);

///
/// * Group: [`PathCoordType`]
pub const GL_RELATIVE_SMOOTH_CUBIC_CURVE_TO_NV: GLenum = GLenum(0x11);

///
/// * Group: [`PathCoordType`]
pub const GL_RELATIVE_SMOOTH_QUADRATIC_CURVE_TO_NV: GLenum = GLenum(0x0F);

///
/// * Group: [`PathCoordType`]
pub const GL_RELATIVE_VERTICAL_LINE_TO_NV: GLenum = GLenum(0x09);

pub const GL_RELEASED_APPLE: GLenum = GLenum(0x8A19);

///
/// * Group: [`RenderingMode`]
pub const GL_RENDER: GLenum = GLenum(0x1C00);

///
/// * Group: [`ObjectIdentifier`], [`RenderbufferTarget`],
///   [`CopyImageSubDataTarget`]
pub const GL_RENDERBUFFER: GLenum = GLenum(0x8D41);

///
/// * Group: [`RenderbufferParameterName`]
pub const GL_RENDERBUFFER_ALPHA_SIZE: GLenum = GLenum(0x8D53);

///
/// * Group: [`RenderbufferParameterName`]
pub const GL_RENDERBUFFER_ALPHA_SIZE_EXT: GLenum = GLenum(0x8D53);

///
/// * Group: [`RenderbufferParameterName`]
pub const GL_RENDERBUFFER_ALPHA_SIZE_OES: GLenum = GLenum(0x8D53);

///
/// * Group: [`GetPName`]
pub const GL_RENDERBUFFER_BINDING: GLenum = GLenum(0x8CA7);

pub const GL_RENDERBUFFER_BINDING_ANGLE: GLenum = GLenum(0x8CA7);

pub const GL_RENDERBUFFER_BINDING_EXT: GLenum = GLenum(0x8CA7);

pub const GL_RENDERBUFFER_BINDING_OES: GLenum = GLenum(0x8CA7);

///
/// * Group: [`RenderbufferParameterName`]
pub const GL_RENDERBUFFER_BLUE_SIZE: GLenum = GLenum(0x8D52);

///
/// * Group: [`RenderbufferParameterName`]
pub const GL_RENDERBUFFER_BLUE_SIZE_EXT: GLenum = GLenum(0x8D52);

///
/// * Group: [`RenderbufferParameterName`]
pub const GL_RENDERBUFFER_BLUE_SIZE_OES: GLenum = GLenum(0x8D52);

///
/// * Group: [`RenderbufferParameterName`]
pub const GL_RENDERBUFFER_COLOR_SAMPLES_NV: GLenum = GLenum(0x8E10);

///
/// * Group: [`RenderbufferParameterName`]
pub const GL_RENDERBUFFER_COVERAGE_SAMPLES_NV: GLenum = GLenum(0x8CAB);

///
/// * Group: [`RenderbufferParameterName`]
pub const GL_RENDERBUFFER_DEPTH_SIZE: GLenum = GLenum(0x8D54);

///
/// * Group: [`RenderbufferParameterName`]
pub const GL_RENDERBUFFER_DEPTH_SIZE_EXT: GLenum = GLenum(0x8D54);

///
/// * Group: [`RenderbufferParameterName`]
pub const GL_RENDERBUFFER_DEPTH_SIZE_OES: GLenum = GLenum(0x8D54);

pub const GL_RENDERBUFFER_EXT: GLenum = GLenum(0x8D41);

pub const GL_RENDERBUFFER_FREE_MEMORY_ATI: GLenum = GLenum(0x87FD);

///
/// * Group: [`RenderbufferParameterName`]
pub const GL_RENDERBUFFER_GREEN_SIZE: GLenum = GLenum(0x8D51);

///
/// * Group: [`RenderbufferParameterName`]
pub const GL_RENDERBUFFER_GREEN_SIZE_EXT: GLenum = GLenum(0x8D51);

///
/// * Group: [`RenderbufferParameterName`]
pub const GL_RENDERBUFFER_GREEN_SIZE_OES: GLenum = GLenum(0x8D51);

///
/// * Group: [`RenderbufferParameterName`]
pub const GL_RENDERBUFFER_HEIGHT: GLenum = GLenum(0x8D43);

///
/// * Group: [`RenderbufferParameterName`]
pub const GL_RENDERBUFFER_HEIGHT_EXT: GLenum = GLenum(0x8D43);

///
/// * Group: [`RenderbufferParameterName`]
pub const GL_RENDERBUFFER_HEIGHT_OES: GLenum = GLenum(0x8D43);

///
/// * Group: [`RenderbufferParameterName`]
pub const GL_RENDERBUFFER_INTERNAL_FORMAT: GLenum = GLenum(0x8D44);

///
/// * Group: [`RenderbufferParameterName`]
pub const GL_RENDERBUFFER_INTERNAL_FORMAT_EXT: GLenum = GLenum(0x8D44);

///
/// * Group: [`RenderbufferParameterName`]
pub const GL_RENDERBUFFER_INTERNAL_FORMAT_OES: GLenum = GLenum(0x8D44);

///
/// * Group: [`RenderbufferTarget`]
pub const GL_RENDERBUFFER_OES: GLenum = GLenum(0x8D41);

///
/// * Group: [`RenderbufferParameterName`]
pub const GL_RENDERBUFFER_RED_SIZE: GLenum = GLenum(0x8D50);

///
/// * Group: [`RenderbufferParameterName`]
pub const GL_RENDERBUFFER_RED_SIZE_EXT: GLenum = GLenum(0x8D50);

///
/// * Group: [`RenderbufferParameterName`]
pub const GL_RENDERBUFFER_RED_SIZE_OES: GLenum = GLenum(0x8D50);

///
/// * Group: [`RenderbufferParameterName`]
pub const GL_RENDERBUFFER_SAMPLES: GLenum = GLenum(0x8CAB);

///
/// * Group: [`RenderbufferParameterName`]
pub const GL_RENDERBUFFER_SAMPLES_ANGLE: GLenum = GLenum(0x8CAB);

///
/// * Group: [`RenderbufferParameterName`]
pub const GL_RENDERBUFFER_SAMPLES_APPLE: GLenum = GLenum(0x8CAB);

///
/// * Group: [`RenderbufferParameterName`]
pub const GL_RENDERBUFFER_SAMPLES_EXT: GLenum = GLenum(0x8CAB);

///
/// * Group: [`RenderbufferParameterName`]
pub const GL_RENDERBUFFER_SAMPLES_IMG: GLenum = GLenum(0x9133);

///
/// * Group: [`RenderbufferParameterName`]
pub const GL_RENDERBUFFER_SAMPLES_NV: GLenum = GLenum(0x8CAB);

///
/// * Group: [`RenderbufferParameterName`]
pub const GL_RENDERBUFFER_STENCIL_SIZE: GLenum = GLenum(0x8D55);

///
/// * Group: [`RenderbufferParameterName`]
pub const GL_RENDERBUFFER_STENCIL_SIZE_EXT: GLenum = GLenum(0x8D55);

///
/// * Group: [`RenderbufferParameterName`]
pub const GL_RENDERBUFFER_STENCIL_SIZE_OES: GLenum = GLenum(0x8D55);

///
/// * Group: [`RenderbufferParameterName`]
pub const GL_RENDERBUFFER_STORAGE_SAMPLES_AMD: GLenum = GLenum(0x91B2);

///
/// * Group: [`RenderbufferParameterName`]
pub const GL_RENDERBUFFER_WIDTH: GLenum = GLenum(0x8D42);

///
/// * Group: [`RenderbufferParameterName`]
pub const GL_RENDERBUFFER_WIDTH_EXT: GLenum = GLenum(0x8D42);

///
/// * Group: [`RenderbufferParameterName`]
pub const GL_RENDERBUFFER_WIDTH_OES: GLenum = GLenum(0x8D42);

///
/// * Group: [`StringName`]
pub const GL_RENDERER: GLenum = GLenum(0x1F01);

pub const GL_RENDER_DIRECT_TO_FRAMEBUFFER_QCOM: GLenum = GLenum(0x8FB3);

pub const GL_RENDER_GPU_MASK_NV: GLenum = GLenum(0x9558);

///
/// * Group: [`GetPName`]
pub const GL_RENDER_MODE: GLenum = GLenum(0x0C40);

///
/// * Group: [`TextureWrapMode`]
pub const GL_REPEAT: GLenum = GLenum(0x2901);

///
/// * Group: [`StencilOp`], [`LightEnvModeSGIX`]
pub const GL_REPLACE: GLenum = GLenum(0x1E01);

pub const GL_REPLACEMENT_CODE_ARRAY_POINTER_SUN: GLenum = GLenum(0x85C3);

pub const GL_REPLACEMENT_CODE_ARRAY_STRIDE_SUN: GLenum = GLenum(0x85C2);

pub const GL_REPLACEMENT_CODE_ARRAY_SUN: GLenum = GLenum(0x85C0);

pub const GL_REPLACEMENT_CODE_ARRAY_TYPE_SUN: GLenum = GLenum(0x85C1);

pub const GL_REPLACEMENT_CODE_SUN: GLenum = GLenum(0x81D8);

///
/// * Group: [`TextureEnvMode`]
pub const GL_REPLACE_EXT: GLenum = GLenum(0x8062);

///
/// * Group: [`TriangleListSUN`]
pub const GL_REPLACE_MIDDLE_SUN: GLenum = GLenum(0x0002);

///
/// * Group: [`TriangleListSUN`]
pub const GL_REPLACE_OLDEST_SUN: GLenum = GLenum(0x0003);

pub const GL_REPLACE_VALUE_AMD: GLenum = GLenum(0x874B);

pub const GL_REPLICATE_BORDER: GLenum = GLenum(0x8153);

pub const GL_REPLICATE_BORDER_HP: GLenum = GLenum(0x8153);

pub const GL_REPRESENTATIVE_FRAGMENT_TEST_NV: GLenum = GLenum(0x937F);

pub const GL_REQUIRED_TEXTURE_IMAGE_UNITS_OES: GLenum = GLenum(0x8D68);

pub const GL_RESAMPLE_AVERAGE_OML: GLenum = GLenum(0x8988);

pub const GL_RESAMPLE_DECIMATE_OML: GLenum = GLenum(0x8989);

/// Formerly 0x8430 in SGI specfile
/// * Group: [`PixelStoreResampleMode`]
pub const GL_RESAMPLE_DECIMATE_SGIX: GLenum = GLenum(0x8430);

pub const GL_RESAMPLE_REPLICATE_OML: GLenum = GLenum(0x8986);

/// Formerly 0x842E in SGI specfile
/// * Group: [`PixelStoreResampleMode`]
pub const GL_RESAMPLE_REPLICATE_SGIX: GLenum = GLenum(0x8433);

pub const GL_RESAMPLE_ZERO_FILL_OML: GLenum = GLenum(0x8987);

/// Formerly 0x842F in SGI specfile
/// * Group: [`PixelStoreResampleMode`]
pub const GL_RESAMPLE_ZERO_FILL_SGIX: GLenum = GLenum(0x8434);

pub const GL_RESCALE_NORMAL: GLenum = GLenum(0x803A);

///
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_RESCALE_NORMAL_EXT: GLenum = GLenum(0x803A);

pub const GL_RESET_NOTIFICATION_STRATEGY: GLenum = GLenum(0x8256);

pub const GL_RESET_NOTIFICATION_STRATEGY_ARB: GLenum = GLenum(0x8256);

pub const GL_RESET_NOTIFICATION_STRATEGY_EXT: GLenum = GLenum(0x8256);

pub const GL_RESET_NOTIFICATION_STRATEGY_KHR: GLenum = GLenum(0x8256);

///
/// * Group: [`PathCoordType`]
pub const GL_RESTART_PATH_NV: GLenum = GLenum(0xF0);

///
/// * Group: [`TriangleListSUN`]
pub const GL_RESTART_SUN: GLenum = GLenum(0x0001);

pub const GL_RETAINED_APPLE: GLenum = GLenum(0x8A1B);

///
/// * Group: [`AccumOp`]
pub const GL_RETURN: GLenum = GLenum(0x0102);

///
/// * Group: [`InternalFormat`], [`PixelFormat`]
pub const GL_RG: GLenum = GLenum(0x8227);

///
/// * Group: [`InternalFormat`]
pub const GL_RG16: GLenum = GLenum(0x822C);

///
/// * Group: [`InternalFormat`]
pub const GL_RG16F: GLenum = GLenum(0x822F);

///
/// * Group: [`InternalFormat`]
pub const GL_RG16F_EXT: GLenum = GLenum(0x822F);

///
/// * Group: [`InternalFormat`]
pub const GL_RG16I: GLenum = GLenum(0x8239);

///
/// * Group: [`InternalFormat`]
pub const GL_RG16UI: GLenum = GLenum(0x823A);

///
/// * Group: [`InternalFormat`]
pub const GL_RG16_EXT: GLenum = GLenum(0x822C);

///
/// * Group: [`InternalFormat`]
pub const GL_RG16_SNORM: GLenum = GLenum(0x8F99);

///
/// * Group: [`InternalFormat`]
pub const GL_RG16_SNORM_EXT: GLenum = GLenum(0x8F99);

///
/// * Group: [`InternalFormat`]
pub const GL_RG32F: GLenum = GLenum(0x8230);

///
/// * Group: [`InternalFormat`]
pub const GL_RG32F_EXT: GLenum = GLenum(0x8230);

///
/// * Group: [`InternalFormat`]
pub const GL_RG32I: GLenum = GLenum(0x823B);

///
/// * Group: [`InternalFormat`]
pub const GL_RG32UI: GLenum = GLenum(0x823C);

///
/// * Group: [`InternalFormat`]
pub const GL_RG8: GLenum = GLenum(0x822B);

///
/// * Group: [`InternalFormat`]
pub const GL_RG8I: GLenum = GLenum(0x8237);

///
/// * Group: [`InternalFormat`]
pub const GL_RG8UI: GLenum = GLenum(0x8238);

///
/// * Group: [`InternalFormat`]
pub const GL_RG8_EXT: GLenum = GLenum(0x822B);

///
/// * Group: [`InternalFormat`]
pub const GL_RG8_SNORM: GLenum = GLenum(0x8F95);

///
/// * Group: [`PixelTexGenMode`], [`CombinerPortionNV`], [`PathColorFormat`],
///   [`CombinerComponentUsageNV`], [`PixelFormat`], [`InternalFormat`]
pub const GL_RGB: GLenum = GLenum(0x1907);

///
/// * Group: [`InternalFormat`]
pub const GL_RGB10: GLenum = GLenum(0x8052);

///
/// * Group: [`InternalFormat`]
pub const GL_RGB10_A2: GLenum = GLenum(0x8059);

///
/// * Group: [`InternalFormat`]
pub const GL_RGB10_A2UI: GLenum = GLenum(0x906F);

///
/// * Group: [`InternalFormat`]
pub const GL_RGB10_A2_EXT: GLenum = GLenum(0x8059);

///
/// * Group: [`InternalFormat`]
pub const GL_RGB10_EXT: GLenum = GLenum(0x8052);

///
/// * Group: [`InternalFormat`]
pub const GL_RGB12: GLenum = GLenum(0x8053);

///
/// * Group: [`InternalFormat`]
pub const GL_RGB12_EXT: GLenum = GLenum(0x8053);

///
/// * Group: [`InternalFormat`]
pub const GL_RGB16: GLenum = GLenum(0x8054);

///
/// * Group: [`InternalFormat`]
pub const GL_RGB16F: GLenum = GLenum(0x881B);

///
/// * Group: [`InternalFormat`]
pub const GL_RGB16F_ARB: GLenum = GLenum(0x881B);

///
/// * Group: [`InternalFormat`]
pub const GL_RGB16F_EXT: GLenum = GLenum(0x881B);

///
/// * Group: [`InternalFormat`]
pub const GL_RGB16I: GLenum = GLenum(0x8D89);

pub const GL_RGB16I_EXT: GLenum = GLenum(0x8D89);

///
/// * Group: [`InternalFormat`]
pub const GL_RGB16UI: GLenum = GLenum(0x8D77);

pub const GL_RGB16UI_EXT: GLenum = GLenum(0x8D77);

///
/// * Group: [`InternalFormat`]
pub const GL_RGB16_EXT: GLenum = GLenum(0x8054);

///
/// * Group: [`InternalFormat`]
pub const GL_RGB16_SNORM: GLenum = GLenum(0x8F9A);

///
/// * Group: [`InternalFormat`]
pub const GL_RGB16_SNORM_EXT: GLenum = GLenum(0x8F9A);

///
/// * Group: [`InternalFormat`]
pub const GL_RGB2_EXT: GLenum = GLenum(0x804E);

///
/// * Group: [`InternalFormat`]
pub const GL_RGB32F: GLenum = GLenum(0x8815);

pub const GL_RGB32F_ARB: GLenum = GLenum(0x8815);

pub const GL_RGB32F_EXT: GLenum = GLenum(0x8815);

///
/// * Group: [`InternalFormat`]
pub const GL_RGB32I: GLenum = GLenum(0x8D83);

pub const GL_RGB32I_EXT: GLenum = GLenum(0x8D83);

///
/// * Group: [`InternalFormat`]
pub const GL_RGB32UI: GLenum = GLenum(0x8D71);

pub const GL_RGB32UI_EXT: GLenum = GLenum(0x8D71);

///
/// * Group: [`InternalFormat`]
pub const GL_RGB4: GLenum = GLenum(0x804F);

///
/// * Group: [`InternalFormat`]
pub const GL_RGB4_EXT: GLenum = GLenum(0x804F);

pub const GL_RGB4_S3TC: GLenum = GLenum(0x83A1);

///
/// * Group: [`InternalFormat`]
pub const GL_RGB5: GLenum = GLenum(0x8050);

pub const GL_RGB565: GLenum = GLenum(0x8D62);

pub const GL_RGB565_OES: GLenum = GLenum(0x8D62);

///
/// * Group: [`InternalFormat`]
pub const GL_RGB5_A1: GLenum = GLenum(0x8057);

///
/// * Group: [`InternalFormat`]
pub const GL_RGB5_A1_EXT: GLenum = GLenum(0x8057);

///
/// * Group: [`InternalFormat`]
pub const GL_RGB5_A1_OES: GLenum = GLenum(0x8057);

///
/// * Group: [`InternalFormat`]
pub const GL_RGB5_EXT: GLenum = GLenum(0x8050);

///
/// * Group: [`InternalFormat`]
pub const GL_RGB8: GLenum = GLenum(0x8051);

///
/// * Group: [`InternalFormat`]
pub const GL_RGB8I: GLenum = GLenum(0x8D8F);

pub const GL_RGB8I_EXT: GLenum = GLenum(0x8D8F);

///
/// * Group: [`InternalFormat`]
pub const GL_RGB8UI: GLenum = GLenum(0x8D7D);

pub const GL_RGB8UI_EXT: GLenum = GLenum(0x8D7D);

///
/// * Group: [`InternalFormat`]
pub const GL_RGB8_EXT: GLenum = GLenum(0x8051);

///
/// * Group: [`InternalFormat`]
pub const GL_RGB8_OES: GLenum = GLenum(0x8051);

///
/// * Group: [`InternalFormat`]
pub const GL_RGB8_SNORM: GLenum = GLenum(0x8F96);

///
/// * Group: [`InternalFormat`]
pub const GL_RGB9_E5: GLenum = GLenum(0x8C3D);

///
/// * Group: [`InternalFormat`]
pub const GL_RGB9_E5_APPLE: GLenum = GLenum(0x8C3D);

///
/// * Group: [`InternalFormat`]
pub const GL_RGB9_E5_EXT: GLenum = GLenum(0x8C3D);

///
/// * Group: [`PixelTexGenMode`], [`PathColorFormat`], [`PixelFormat`],
///   [`InternalFormat`]
pub const GL_RGBA: GLenum = GLenum(0x1908);

///
/// * Group: [`InternalFormat`]
pub const GL_RGBA12: GLenum = GLenum(0x805A);

///
/// * Group: [`InternalFormat`]
pub const GL_RGBA12_EXT: GLenum = GLenum(0x805A);

///
/// * Group: [`InternalFormat`]
pub const GL_RGBA16: GLenum = GLenum(0x805B);

///
/// * Group: [`InternalFormat`]
pub const GL_RGBA16F: GLenum = GLenum(0x881A);

///
/// * Group: [`InternalFormat`]
pub const GL_RGBA16F_ARB: GLenum = GLenum(0x881A);

///
/// * Group: [`InternalFormat`]
pub const GL_RGBA16F_EXT: GLenum = GLenum(0x881A);

///
/// * Group: [`InternalFormat`]
pub const GL_RGBA16I: GLenum = GLenum(0x8D88);

pub const GL_RGBA16I_EXT: GLenum = GLenum(0x8D88);

///
/// * Group: [`InternalFormat`]
pub const GL_RGBA16UI: GLenum = GLenum(0x8D76);

pub const GL_RGBA16UI_EXT: GLenum = GLenum(0x8D76);

///
/// * Group: [`InternalFormat`]
pub const GL_RGBA16_EXT: GLenum = GLenum(0x805B);

pub const GL_RGBA16_SNORM: GLenum = GLenum(0x8F9B);

pub const GL_RGBA16_SNORM_EXT: GLenum = GLenum(0x8F9B);

pub const GL_RGBA2: GLenum = GLenum(0x8055);

pub const GL_RGBA2_EXT: GLenum = GLenum(0x8055);

///
/// * Group: [`InternalFormat`]
pub const GL_RGBA32F: GLenum = GLenum(0x8814);

///
/// * Group: [`InternalFormat`]
pub const GL_RGBA32F_ARB: GLenum = GLenum(0x8814);

///
/// * Group: [`InternalFormat`]
pub const GL_RGBA32F_EXT: GLenum = GLenum(0x8814);

///
/// * Group: [`InternalFormat`]
pub const GL_RGBA32I: GLenum = GLenum(0x8D82);

pub const GL_RGBA32I_EXT: GLenum = GLenum(0x8D82);

///
/// * Group: [`InternalFormat`]
pub const GL_RGBA32UI: GLenum = GLenum(0x8D70);

pub const GL_RGBA32UI_EXT: GLenum = GLenum(0x8D70);

///
/// * Group: [`InternalFormat`]
pub const GL_RGBA4: GLenum = GLenum(0x8056);

pub const GL_RGBA4_DXT5_S3TC: GLenum = GLenum(0x83A5);

///
/// * Group: [`InternalFormat`]
pub const GL_RGBA4_EXT: GLenum = GLenum(0x8056);

///
/// * Group: [`InternalFormat`]
pub const GL_RGBA4_OES: GLenum = GLenum(0x8056);

pub const GL_RGBA4_S3TC: GLenum = GLenum(0x83A3);

///
/// * Group: [`InternalFormat`]
pub const GL_RGBA8: GLenum = GLenum(0x8058);

///
/// * Group: [`InternalFormat`]
pub const GL_RGBA8I: GLenum = GLenum(0x8D8E);

pub const GL_RGBA8I_EXT: GLenum = GLenum(0x8D8E);

///
/// * Group: [`InternalFormat`]
pub const GL_RGBA8UI: GLenum = GLenum(0x8D7C);

pub const GL_RGBA8UI_EXT: GLenum = GLenum(0x8D7C);

///
/// * Group: [`InternalFormat`]
pub const GL_RGBA8_EXT: GLenum = GLenum(0x8058);

///
/// * Group: [`InternalFormat`]
pub const GL_RGBA8_OES: GLenum = GLenum(0x8058);

///
/// * Group: [`InternalFormat`]
pub const GL_RGBA8_SNORM: GLenum = GLenum(0x8F97);

pub const GL_RGBA_DXT5_S3TC: GLenum = GLenum(0x83A4);

pub const GL_RGBA_FLOAT16_APPLE: GLenum = GLenum(0x881A);

pub const GL_RGBA_FLOAT16_ATI: GLenum = GLenum(0x881A);

pub const GL_RGBA_FLOAT32_APPLE: GLenum = GLenum(0x8814);

pub const GL_RGBA_FLOAT32_ATI: GLenum = GLenum(0x8814);

pub const GL_RGBA_FLOAT_MODE_ARB: GLenum = GLenum(0x8820);

pub const GL_RGBA_FLOAT_MODE_ATI: GLenum = GLenum(0x8820);

///
/// * Group: [`PixelFormat`]
pub const GL_RGBA_INTEGER: GLenum = GLenum(0x8D99);

pub const GL_RGBA_INTEGER_EXT: GLenum = GLenum(0x8D99);

pub const GL_RGBA_INTEGER_MODE_EXT: GLenum = GLenum(0x8D9E);

///
/// * Group: [`GetPName`]
pub const GL_RGBA_MODE: GLenum = GLenum(0x0C31);

pub const GL_RGBA_S3TC: GLenum = GLenum(0x83A2);

pub const GL_RGBA_SIGNED_COMPONENTS_EXT: GLenum = GLenum(0x8C3C);

pub const GL_RGBA_SNORM: GLenum = GLenum(0x8F93);

pub const GL_RGBA_UNSIGNED_DOT_PRODUCT_MAPPING_NV: GLenum = GLenum(0x86D9);

pub const GL_RGB_422_APPLE: GLenum = GLenum(0x8A1F);

pub const GL_RGB_FLOAT16_APPLE: GLenum = GLenum(0x881B);

pub const GL_RGB_FLOAT16_ATI: GLenum = GLenum(0x881B);

pub const GL_RGB_FLOAT32_APPLE: GLenum = GLenum(0x8815);

pub const GL_RGB_FLOAT32_ATI: GLenum = GLenum(0x8815);

///
/// * Group: [`PixelFormat`]
pub const GL_RGB_INTEGER: GLenum = GLenum(0x8D98);

pub const GL_RGB_INTEGER_EXT: GLenum = GLenum(0x8D98);

pub const GL_RGB_RAW_422_APPLE: GLenum = GLenum(0x8A51);

pub const GL_RGB_S3TC: GLenum = GLenum(0x83A0);

pub const GL_RGB_SCALE: GLenum = GLenum(0x8573);

pub const GL_RGB_SCALE_ARB: GLenum = GLenum(0x8573);

pub const GL_RGB_SCALE_EXT: GLenum = GLenum(0x8573);

pub const GL_RGB_SNORM: GLenum = GLenum(0x8F92);

pub const GL_RG_EXT: GLenum = GLenum(0x8227);

///
/// * Group: [`PixelFormat`]
pub const GL_RG_INTEGER: GLenum = GLenum(0x8228);

pub const GL_RG_SNORM: GLenum = GLenum(0x8F91);

///
/// * Group: [`ColorBuffer`], [`DrawBufferMode`], [`ReadBufferMode`]
pub const GL_RIGHT: GLenum = GLenum(0x0407);

/// Reserved for future
pub const GL_ROBUST_GPU_TIMEOUT_MS_KHR: GLenum = GLenum(0x82FD);

///
/// * Group: [`PathCoordType`]
pub const GL_ROUNDED_RECT2_NV: GLenum = GLenum(0xEA);

///
/// * Group: [`PathCoordType`]
pub const GL_ROUNDED_RECT4_NV: GLenum = GLenum(0xEC);

///
/// * Group: [`PathCoordType`]
pub const GL_ROUNDED_RECT8_NV: GLenum = GLenum(0xEE);

///
/// * Group: [`PathCoordType`]
pub const GL_ROUNDED_RECT_NV: GLenum = GLenum(0xE8);

pub const GL_ROUND_NV: GLenum = GLenum(0x90A4);

///
/// * Group: [`TextureCoordName`]
pub const GL_S: GLenum = GLenum(0x2000);

///
/// * Group: [`ObjectIdentifier`]
pub const GL_SAMPLER: GLenum = GLenum(0x82E6);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_SAMPLER_1D: GLenum = GLenum(0x8B5D);

///
/// * Group: [`AttributeType`]
pub const GL_SAMPLER_1D_ARB: GLenum = GLenum(0x8B5D);

///
/// * Group: [`GlslTypeToken`], [`UniformType`]
pub const GL_SAMPLER_1D_ARRAY: GLenum = GLenum(0x8DC0);

pub const GL_SAMPLER_1D_ARRAY_EXT: GLenum = GLenum(0x8DC0);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_SAMPLER_1D_ARRAY_SHADOW: GLenum = GLenum(0x8DC3);

pub const GL_SAMPLER_1D_ARRAY_SHADOW_EXT: GLenum = GLenum(0x8DC3);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_SAMPLER_1D_SHADOW: GLenum = GLenum(0x8B61);

///
/// * Group: [`AttributeType`]
pub const GL_SAMPLER_1D_SHADOW_ARB: GLenum = GLenum(0x8B61);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_SAMPLER_2D: GLenum = GLenum(0x8B5E);

///
/// * Group: [`AttributeType`]
pub const GL_SAMPLER_2D_ARB: GLenum = GLenum(0x8B5E);

///
/// * Group: [`GlslTypeToken`], [`UniformType`]
pub const GL_SAMPLER_2D_ARRAY: GLenum = GLenum(0x8DC1);

pub const GL_SAMPLER_2D_ARRAY_EXT: GLenum = GLenum(0x8DC1);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_SAMPLER_2D_ARRAY_SHADOW: GLenum = GLenum(0x8DC4);

pub const GL_SAMPLER_2D_ARRAY_SHADOW_EXT: GLenum = GLenum(0x8DC4);

pub const GL_SAMPLER_2D_ARRAY_SHADOW_NV: GLenum = GLenum(0x8DC4);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_SAMPLER_2D_MULTISAMPLE: GLenum = GLenum(0x9108);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_SAMPLER_2D_MULTISAMPLE_ARRAY: GLenum = GLenum(0x910B);

pub const GL_SAMPLER_2D_MULTISAMPLE_ARRAY_OES: GLenum = GLenum(0x910B);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_SAMPLER_2D_RECT: GLenum = GLenum(0x8B63);

///
/// * Group: [`AttributeType`]
pub const GL_SAMPLER_2D_RECT_ARB: GLenum = GLenum(0x8B63);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_SAMPLER_2D_RECT_SHADOW: GLenum = GLenum(0x8B64);

///
/// * Group: [`AttributeType`]
pub const GL_SAMPLER_2D_RECT_SHADOW_ARB: GLenum = GLenum(0x8B64);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_SAMPLER_2D_SHADOW: GLenum = GLenum(0x8B62);

///
/// * Group: [`AttributeType`]
pub const GL_SAMPLER_2D_SHADOW_ARB: GLenum = GLenum(0x8B62);

///
/// * Group: [`AttributeType`]
pub const GL_SAMPLER_2D_SHADOW_EXT: GLenum = GLenum(0x8B62);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_SAMPLER_3D: GLenum = GLenum(0x8B5F);

///
/// * Group: [`AttributeType`]
pub const GL_SAMPLER_3D_ARB: GLenum = GLenum(0x8B5F);

///
/// * Group: [`AttributeType`]
pub const GL_SAMPLER_3D_OES: GLenum = GLenum(0x8B5F);

///
/// * Group: [`GetPName`]
pub const GL_SAMPLER_BINDING: GLenum = GLenum(0x8919);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_SAMPLER_BUFFER: GLenum = GLenum(0x8DC2);

pub const GL_SAMPLER_BUFFER_AMD: GLenum = GLenum(0x9001);

pub const GL_SAMPLER_BUFFER_EXT: GLenum = GLenum(0x8DC2);

pub const GL_SAMPLER_BUFFER_OES: GLenum = GLenum(0x8DC2);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_SAMPLER_CUBE: GLenum = GLenum(0x8B60);

///
/// * Group: [`AttributeType`]
pub const GL_SAMPLER_CUBE_ARB: GLenum = GLenum(0x8B60);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_SAMPLER_CUBE_MAP_ARRAY: GLenum = GLenum(0x900C);

pub const GL_SAMPLER_CUBE_MAP_ARRAY_ARB: GLenum = GLenum(0x900C);

pub const GL_SAMPLER_CUBE_MAP_ARRAY_EXT: GLenum = GLenum(0x900C);

pub const GL_SAMPLER_CUBE_MAP_ARRAY_OES: GLenum = GLenum(0x900C);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_SAMPLER_CUBE_MAP_ARRAY_SHADOW: GLenum = GLenum(0x900D);

pub const GL_SAMPLER_CUBE_MAP_ARRAY_SHADOW_ARB: GLenum = GLenum(0x900D);

pub const GL_SAMPLER_CUBE_MAP_ARRAY_SHADOW_EXT: GLenum = GLenum(0x900D);

pub const GL_SAMPLER_CUBE_MAP_ARRAY_SHADOW_OES: GLenum = GLenum(0x900D);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_SAMPLER_CUBE_SHADOW: GLenum = GLenum(0x8DC5);

pub const GL_SAMPLER_CUBE_SHADOW_EXT: GLenum = GLenum(0x8DC5);

pub const GL_SAMPLER_CUBE_SHADOW_NV: GLenum = GLenum(0x8DC5);

pub const GL_SAMPLER_EXTERNAL_2D_Y2Y_EXT: GLenum = GLenum(0x8BE7);

pub const GL_SAMPLER_EXTERNAL_OES: GLenum = GLenum(0x8D66);

pub const GL_SAMPLER_KHR: GLenum = GLenum(0x82E6);

pub const GL_SAMPLER_OBJECT_AMD: GLenum = GLenum(0x9155);

pub const GL_SAMPLER_RENDERBUFFER_NV: GLenum = GLenum(0x8E56);

///
/// * Group: [`GetFramebufferParameter`], [`GetPName`], [`InternalFormatPName`]
pub const GL_SAMPLES: GLenum = GLenum(0x80A9);

pub const GL_SAMPLES_3DFX: GLenum = GLenum(0x86B4);

pub const GL_SAMPLES_ARB: GLenum = GLenum(0x80A9);

pub const GL_SAMPLES_EXT: GLenum = GLenum(0x80A9);

///
/// * Group: [`QueryTarget`]
pub const GL_SAMPLES_PASSED: GLenum = GLenum(0x8914);

pub const GL_SAMPLES_PASSED_ARB: GLenum = GLenum(0x8914);

///
/// * Group: [`GetPName`]
pub const GL_SAMPLES_SGIS: GLenum = GLenum(0x80A9);

///
/// * Group: [`EnableCap`]
pub const GL_SAMPLE_ALPHA_TO_COVERAGE: GLenum = GLenum(0x809E);

pub const GL_SAMPLE_ALPHA_TO_COVERAGE_ARB: GLenum = GLenum(0x809E);

pub const GL_SAMPLE_ALPHA_TO_MASK_EXT: GLenum = GLenum(0x809E);

///
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_SAMPLE_ALPHA_TO_MASK_SGIS: GLenum = GLenum(0x809E);

///
/// * Group: [`EnableCap`]
pub const GL_SAMPLE_ALPHA_TO_ONE: GLenum = GLenum(0x809F);

pub const GL_SAMPLE_ALPHA_TO_ONE_ARB: GLenum = GLenum(0x809F);

pub const GL_SAMPLE_ALPHA_TO_ONE_EXT: GLenum = GLenum(0x809F);

///
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_SAMPLE_ALPHA_TO_ONE_SGIS: GLenum = GLenum(0x809F);

///
/// * Group: [`GetFramebufferParameter`], [`GetPName`]
pub const GL_SAMPLE_BUFFERS: GLenum = GLenum(0x80A8);

pub const GL_SAMPLE_BUFFERS_3DFX: GLenum = GLenum(0x86B3);

pub const GL_SAMPLE_BUFFERS_ARB: GLenum = GLenum(0x80A8);

pub const GL_SAMPLE_BUFFERS_EXT: GLenum = GLenum(0x80A8);

///
/// * Group: [`GetPName`]
pub const GL_SAMPLE_BUFFERS_SGIS: GLenum = GLenum(0x80A8);

///
/// * Group: [`EnableCap`]
pub const GL_SAMPLE_COVERAGE: GLenum = GLenum(0x80A0);

pub const GL_SAMPLE_COVERAGE_ARB: GLenum = GLenum(0x80A0);

///
/// * Group: [`GetPName`]
pub const GL_SAMPLE_COVERAGE_INVERT: GLenum = GLenum(0x80AB);

pub const GL_SAMPLE_COVERAGE_INVERT_ARB: GLenum = GLenum(0x80AB);

///
/// * Group: [`GetPName`]
pub const GL_SAMPLE_COVERAGE_VALUE: GLenum = GLenum(0x80AA);

pub const GL_SAMPLE_COVERAGE_VALUE_ARB: GLenum = GLenum(0x80AA);

///
/// * Group: [`GetMultisamplePNameNV`]
/// * Alias Of: [`GL_SAMPLE_POSITION`]
pub const GL_SAMPLE_LOCATION_ARB: GLenum = GL_SAMPLE_POSITION;

///
/// * Alias Of: [`GL_SAMPLE_POSITION_NV`]
pub const GL_SAMPLE_LOCATION_NV: GLenum = GL_SAMPLE_POSITION_NV;

pub const GL_SAMPLE_LOCATION_PIXEL_GRID_HEIGHT_ARB: GLenum = GLenum(0x933F);

pub const GL_SAMPLE_LOCATION_PIXEL_GRID_HEIGHT_NV: GLenum = GLenum(0x933F);

pub const GL_SAMPLE_LOCATION_PIXEL_GRID_WIDTH_ARB: GLenum = GLenum(0x933E);

pub const GL_SAMPLE_LOCATION_PIXEL_GRID_WIDTH_NV: GLenum = GLenum(0x933E);

pub const GL_SAMPLE_LOCATION_SUBPIXEL_BITS_ARB: GLenum = GLenum(0x933D);

pub const GL_SAMPLE_LOCATION_SUBPIXEL_BITS_NV: GLenum = GLenum(0x933D);

///
/// * Group: [`EnableCap`]
pub const GL_SAMPLE_MASK: GLenum = GLenum(0x8E51);

pub const GL_SAMPLE_MASK_EXT: GLenum = GLenum(0x80A0);

pub const GL_SAMPLE_MASK_INVERT_EXT: GLenum = GLenum(0x80AB);

///
/// * Group: [`GetPName`]
pub const GL_SAMPLE_MASK_INVERT_SGIS: GLenum = GLenum(0x80AB);

pub const GL_SAMPLE_MASK_NV: GLenum = GLenum(0x8E51);

///
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_SAMPLE_MASK_SGIS: GLenum = GLenum(0x80A0);

pub const GL_SAMPLE_MASK_VALUE: GLenum = GLenum(0x8E52);

pub const GL_SAMPLE_MASK_VALUE_EXT: GLenum = GLenum(0x80AA);

pub const GL_SAMPLE_MASK_VALUE_NV: GLenum = GLenum(0x8E52);

///
/// * Group: [`GetPName`]
pub const GL_SAMPLE_MASK_VALUE_SGIS: GLenum = GLenum(0x80AA);

pub const GL_SAMPLE_PATTERN_EXT: GLenum = GLenum(0x80AC);

///
/// * Group: [`GetPName`]
pub const GL_SAMPLE_PATTERN_SGIS: GLenum = GLenum(0x80AC);

///
/// * Group: [`GetMultisamplePNameNV`]
pub const GL_SAMPLE_POSITION: GLenum = GLenum(0x8E50);

pub const GL_SAMPLE_POSITION_NV: GLenum = GLenum(0x8E50);

///
/// * Group: [`EnableCap`]
pub const GL_SAMPLE_SHADING: GLenum = GLenum(0x8C36);

pub const GL_SAMPLE_SHADING_ARB: GLenum = GLenum(0x8C36);

pub const GL_SAMPLE_SHADING_OES: GLenum = GLenum(0x8C36);

///
/// * Group: [`FragmentShaderDestModMaskATI`]
pub const GL_SATURATE_BIT_ATI: GLbitfield = GLbitfield(0x00000040);

///
/// * Group: [`DataTypeEXT`]
pub const GL_SCALAR_EXT: GLenum = GLenum(0x87BE);

///
/// * Group: [`HintTarget`]
pub const GL_SCALEBIAS_HINT_SGIX: GLenum = GLenum(0x8322);

pub const GL_SCALED_RESOLVE_FASTEST_EXT: GLenum = GLenum(0x90BA);

pub const GL_SCALED_RESOLVE_NICEST_EXT: GLenum = GLenum(0x90BB);

///
/// * Group: [`CombinerScaleNV`]
pub const GL_SCALE_BY_FOUR_NV: GLenum = GLenum(0x853F);

///
/// * Group: [`CombinerScaleNV`]
pub const GL_SCALE_BY_ONE_HALF_NV: GLenum = GLenum(0x8540);

///
/// * Group: [`CombinerScaleNV`]
pub const GL_SCALE_BY_TWO_NV: GLenum = GLenum(0x853E);

///
/// * Group: [`AttribMask`]
pub const GL_SCISSOR_BIT: GLbitfield = GLbitfield(0x00080000);

///
/// * Group: [`GetPName`]
pub const GL_SCISSOR_BOX: GLenum = GLenum(0x0C10);

pub const GL_SCISSOR_BOX_EXCLUSIVE_NV: GLenum = GLenum(0x9556);

///
/// * Group: [`CommandOpcodesNV`]
pub const GL_SCISSOR_COMMAND_NV: GLenum = GLenum(0x0011);

///
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_SCISSOR_TEST: GLenum = GLenum(0x0C11);

pub const GL_SCISSOR_TEST_EXCLUSIVE_NV: GLenum = GLenum(0x9555);

pub const GL_SCREEN: GLenum = GLenum(0x9295);

pub const GL_SCREEN_COORDINATES_REND: GLenum = GLenum(0x8490);

pub const GL_SCREEN_KHR: GLenum = GLenum(0x9295);

pub const GL_SCREEN_NV: GLenum = GLenum(0x9295);

pub const GL_SECONDARY_COLOR_ARRAY: GLenum = GLenum(0x845E);

pub const GL_SECONDARY_COLOR_ARRAY_ADDRESS_NV: GLenum = GLenum(0x8F27);

pub const GL_SECONDARY_COLOR_ARRAY_BUFFER_BINDING: GLenum = GLenum(0x889C);

pub const GL_SECONDARY_COLOR_ARRAY_BUFFER_BINDING_ARB: GLenum = GLenum(0x889C);

pub const GL_SECONDARY_COLOR_ARRAY_EXT: GLenum = GLenum(0x845E);

pub const GL_SECONDARY_COLOR_ARRAY_LENGTH_NV: GLenum = GLenum(0x8F31);

pub const GL_SECONDARY_COLOR_ARRAY_LIST_IBM: GLenum = GLenum(103077);

pub const GL_SECONDARY_COLOR_ARRAY_LIST_STRIDE_IBM: GLenum = GLenum(103087);

pub const GL_SECONDARY_COLOR_ARRAY_POINTER: GLenum = GLenum(0x845D);

pub const GL_SECONDARY_COLOR_ARRAY_POINTER_EXT: GLenum = GLenum(0x845D);

pub const GL_SECONDARY_COLOR_ARRAY_SIZE: GLenum = GLenum(0x845A);

pub const GL_SECONDARY_COLOR_ARRAY_SIZE_EXT: GLenum = GLenum(0x845A);

pub const GL_SECONDARY_COLOR_ARRAY_STRIDE: GLenum = GLenum(0x845C);

pub const GL_SECONDARY_COLOR_ARRAY_STRIDE_EXT: GLenum = GLenum(0x845C);

pub const GL_SECONDARY_COLOR_ARRAY_TYPE: GLenum = GLenum(0x845B);

pub const GL_SECONDARY_COLOR_ARRAY_TYPE_EXT: GLenum = GLenum(0x845B);

///
/// * Group: [`PathColor`], [`CombinerRegisterNV`]
pub const GL_SECONDARY_COLOR_NV: GLenum = GLenum(0x852D);

pub const GL_SECONDARY_INTERPOLATOR_ATI: GLenum = GLenum(0x896D);

///
/// * Group: [`RenderingMode`]
pub const GL_SELECT: GLenum = GLenum(0x1C02);

///
/// * Group: [`GetPointervPName`]
pub const GL_SELECTION_BUFFER_POINTER: GLenum = GLenum(0x0DF3);

///
/// * Group: [`GetPName`]
pub const GL_SELECTION_BUFFER_SIZE: GLenum = GLenum(0x0DF4);

///
/// * Group: [`SemaphoreParameterName`]
pub const GL_SEMAPHORE_TYPE_BINARY_NV: GLenum = GLenum(0x95B4);

///
/// * Group: [`SemaphoreParameterName`]
pub const GL_SEMAPHORE_TYPE_NV: GLenum = GLenum(0x95B3);

///
/// * Group: [`SemaphoreParameterName`]
pub const GL_SEMAPHORE_TYPE_TIMELINE_NV: GLenum = GLenum(0x95B5);

///
/// * Group: [`SeparableTarget`], [`SeparableTargetEXT`]
pub const GL_SEPARABLE_2D: GLenum = GLenum(0x8012);

///
/// * Group: [`SeparableTargetEXT`], [`EnableCap`], [`GetPName`]
pub const GL_SEPARABLE_2D_EXT: GLenum = GLenum(0x8012);

///
/// * Group: [`TransformFeedbackBufferMode`]
pub const GL_SEPARATE_ATTRIBS: GLenum = GLenum(0x8C8D);

pub const GL_SEPARATE_ATTRIBS_EXT: GLenum = GLenum(0x8C8D);

pub const GL_SEPARATE_ATTRIBS_NV: GLenum = GLenum(0x8C8D);

///
/// * Group: [`LightModelColorControl`]
pub const GL_SEPARATE_SPECULAR_COLOR: GLenum = GLenum(0x81FA);

///
/// * Group: [`LightModelColorControl`]
pub const GL_SEPARATE_SPECULAR_COLOR_EXT: GLenum = GLenum(0x81FA);

///
/// * Group: [`LogicOp`]
pub const GL_SET: GLenum = GLenum(0x150F);

pub const GL_SET_AMD: GLenum = GLenum(0x874A);

///
/// * Group: [`ShaderBinaryFormat`]
pub const GL_SGX_BINARY_IMG: GLenum = GLenum(0x8C0A);

pub const GL_SGX_PROGRAM_BINARY_IMG: GLenum = GLenum(0x9130);

///
/// * Group: [`ObjectIdentifier`]
pub const GL_SHADER: GLenum = GLenum(0x82E1);

///
/// * Group: [`ShaderBinaryFormat`]
pub const GL_SHADER_BINARY_DMP: GLenum = GLenum(0x9250);

///
/// * Group: [`GetPName`]
pub const GL_SHADER_BINARY_FORMATS: GLenum = GLenum(0x8DF8);

///
/// * Group: [`ShaderBinaryFormat`]
pub const GL_SHADER_BINARY_FORMAT_SPIR_V: GLenum = GLenum(0x9551);

///
/// * Alias Of: [`GL_SHADER_BINARY_FORMAT_SPIR_V`]
pub const GL_SHADER_BINARY_FORMAT_SPIR_V_ARB: GLenum = GL_SHADER_BINARY_FORMAT_SPIR_V;

///
/// * Group: [`ShaderBinaryFormat`]
pub const GL_SHADER_BINARY_VIV: GLenum = GLenum(0x8FC4);

///
/// * Group: [`GetPName`]
pub const GL_SHADER_COMPILER: GLenum = GLenum(0x8DFA);

pub const GL_SHADER_CONSISTENT_NV: GLenum = GLenum(0x86DD);

///
/// * Group: [`MemoryBarrierMask`]
pub const GL_SHADER_GLOBAL_ACCESS_BARRIER_BIT_NV: GLbitfield = GLbitfield(0x00000010);

///
/// * Group: [`MemoryBarrierMask`]
pub const GL_SHADER_IMAGE_ACCESS_BARRIER_BIT: GLbitfield = GLbitfield(0x00000020);

///
/// * Group: [`MemoryBarrierMask`]
pub const GL_SHADER_IMAGE_ACCESS_BARRIER_BIT_EXT: GLbitfield = GLbitfield(0x00000020);

///
/// * Group: [`InternalFormatPName`]
pub const GL_SHADER_IMAGE_ATOMIC: GLenum = GLenum(0x82A6);

///
/// * Group: [`InternalFormatPName`]
pub const GL_SHADER_IMAGE_LOAD: GLenum = GLenum(0x82A4);

///
/// * Group: [`InternalFormatPName`]
pub const GL_SHADER_IMAGE_STORE: GLenum = GLenum(0x82A5);

pub const GL_SHADER_INCLUDE_ARB: GLenum = GLenum(0x8DAE);

pub const GL_SHADER_KHR: GLenum = GLenum(0x82E1);

pub const GL_SHADER_OBJECT_ARB: GLenum = GLenum(0x8B48);

pub const GL_SHADER_OBJECT_EXT: GLenum = GLenum(0x8B48);

pub const GL_SHADER_OPERATION_NV: GLenum = GLenum(0x86DF);

pub const GL_SHADER_PIXEL_LOCAL_STORAGE_EXT: GLenum = GLenum(0x8F64);

///
/// * Group: [`ShaderParameterName`]
pub const GL_SHADER_SOURCE_LENGTH: GLenum = GLenum(0x8B88);

///
/// * Group: [`MemoryBarrierMask`]
pub const GL_SHADER_STORAGE_BARRIER_BIT: GLbitfield = GLbitfield(0x00002000);

///
/// * Group: [`ProgramInterface`]
pub const GL_SHADER_STORAGE_BLOCK: GLenum = GLenum(0x92E6);

///
/// * Group: [`CopyBufferSubDataTarget`], [`BufferTargetARB`],
///   [`BufferStorageTarget`]
pub const GL_SHADER_STORAGE_BUFFER: GLenum = GLenum(0x90D2);

///
/// * Group: [`GetPName`]
pub const GL_SHADER_STORAGE_BUFFER_BINDING: GLenum = GLenum(0x90D3);

///
/// * Group: [`GetPName`]
pub const GL_SHADER_STORAGE_BUFFER_OFFSET_ALIGNMENT: GLenum = GLenum(0x90DF);

///
/// * Group: [`GetPName`]
pub const GL_SHADER_STORAGE_BUFFER_SIZE: GLenum = GLenum(0x90D5);

///
/// * Group: [`GetPName`]
pub const GL_SHADER_STORAGE_BUFFER_START: GLenum = GLenum(0x90D4);

///
/// * Group: [`ShaderParameterName`]
pub const GL_SHADER_TYPE: GLenum = GLenum(0x8B4F);

///
/// * Group: [`GetPName`]
pub const GL_SHADE_MODEL: GLenum = GLenum(0x0B54);

///
/// * Group: [`StringName`]
pub const GL_SHADING_LANGUAGE_VERSION: GLenum = GLenum(0x8B8C);

pub const GL_SHADING_LANGUAGE_VERSION_ARB: GLenum = GLenum(0x8B8C);

pub const GL_SHADING_RATE_16_INVOCATIONS_PER_PIXEL_NV: GLenum = GLenum(0x956F);

///
/// * Group: [`ShadingRateQCOM`]
pub const GL_SHADING_RATE_1X1_PIXELS_QCOM: GLenum = GLenum(0x96A6);

///
/// * Group: [`ShadingRateQCOM`]
pub const GL_SHADING_RATE_1X2_PIXELS_QCOM: GLenum = GLenum(0x96A7);

///
/// * Group: [`ShadingRateQCOM`]
pub const GL_SHADING_RATE_1X4_PIXELS_QCOM: GLenum = GLenum(0x96AA);

pub const GL_SHADING_RATE_1_INVOCATION_PER_1X2_PIXELS_NV: GLenum = GLenum(0x9566);

pub const GL_SHADING_RATE_1_INVOCATION_PER_2X1_PIXELS_NV: GLenum = GLenum(0x9567);

pub const GL_SHADING_RATE_1_INVOCATION_PER_2X2_PIXELS_NV: GLenum = GLenum(0x9568);

pub const GL_SHADING_RATE_1_INVOCATION_PER_2X4_PIXELS_NV: GLenum = GLenum(0x9569);

pub const GL_SHADING_RATE_1_INVOCATION_PER_4X2_PIXELS_NV: GLenum = GLenum(0x956A);

pub const GL_SHADING_RATE_1_INVOCATION_PER_4X4_PIXELS_NV: GLenum = GLenum(0x956B);

pub const GL_SHADING_RATE_1_INVOCATION_PER_PIXEL_NV: GLenum = GLenum(0x9565);

///
/// * Group: [`ShadingRateQCOM`]
pub const GL_SHADING_RATE_2X1_PIXELS_QCOM: GLenum = GLenum(0x96A8);

///
/// * Group: [`ShadingRateQCOM`]
pub const GL_SHADING_RATE_2X2_PIXELS_QCOM: GLenum = GLenum(0x96A9);

///
/// * Group: [`ShadingRateQCOM`]
pub const GL_SHADING_RATE_2X4_PIXELS_QCOM: GLenum = GLenum(0x96AD);

pub const GL_SHADING_RATE_2_INVOCATIONS_PER_PIXEL_NV: GLenum = GLenum(0x956C);

///
/// * Group: [`ShadingRateQCOM`]
pub const GL_SHADING_RATE_4X1_PIXELS_QCOM: GLenum = GLenum(0x96AB);

///
/// * Group: [`ShadingRateQCOM`]
pub const GL_SHADING_RATE_4X2_PIXELS_QCOM: GLenum = GLenum(0x96AC);

///
/// * Group: [`ShadingRateQCOM`]
pub const GL_SHADING_RATE_4X4_PIXELS_QCOM: GLenum = GLenum(0x96AE);

pub const GL_SHADING_RATE_4_INVOCATIONS_PER_PIXEL_NV: GLenum = GLenum(0x956D);

pub const GL_SHADING_RATE_8_INVOCATIONS_PER_PIXEL_NV: GLenum = GLenum(0x956E);

pub const GL_SHADING_RATE_IMAGE_BINDING_NV: GLenum = GLenum(0x955B);

pub const GL_SHADING_RATE_IMAGE_NV: GLenum = GLenum(0x9563);

///
/// * Group: [`GetPName`]
pub const GL_SHADING_RATE_IMAGE_PALETTE_COUNT_NV: GLenum = GLenum(0x95B2);

pub const GL_SHADING_RATE_IMAGE_PALETTE_SIZE_NV: GLenum = GLenum(0x955E);

///
/// * Group: [`EnableCap`], [`GetPName`]
pub const GL_SHADING_RATE_IMAGE_PER_PRIMITIVE_NV: GLenum = GLenum(0x95B1);

pub const GL_SHADING_RATE_IMAGE_TEXEL_HEIGHT_NV: GLenum = GLenum(0x955D);

pub const GL_SHADING_RATE_IMAGE_TEXEL_WIDTH_NV: GLenum = GLenum(0x955C);

pub const GL_SHADING_RATE_NO_INVOCATIONS_NV: GLenum = GLenum(0x9564);

///
/// * Group: [`EnableCap`]
pub const GL_SHADING_RATE_PRESERVE_ASPECT_RATIO_QCOM: GLenum = GLenum(0x96A5);

///
/// * Group: [`GetPName`]
pub const GL_SHADING_RATE_QCOM: GLenum = GLenum(0x96A4);

pub const GL_SHADING_RATE_SAMPLE_ORDER_DEFAULT_NV: GLenum = GLenum(0x95AE);

pub const GL_SHADING_RATE_SAMPLE_ORDER_PIXEL_MAJOR_NV: GLenum = GLenum(0x95AF);

pub const GL_SHADING_RATE_SAMPLE_ORDER_SAMPLE_MAJOR_NV: GLenum = GLenum(0x95B0);

///
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_SHADOW_AMBIENT_SGIX: GLenum = GLenum(0x80BF);

///
/// * Group: [`LightTexturePNameEXT`]
pub const GL_SHADOW_ATTENUATION_EXT: GLenum = GLenum(0x834E);

pub const GL_SHARED_EDGE_NV: GLenum = GLenum(0xC0);

///
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_SHARED_TEXTURE_PALETTE_EXT: GLenum = GLenum(0x81FB);

///
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_SHARPEN_TEXTURE_FUNC_POINTS_SGIS: GLenum = GLenum(0x80B0);

///
/// * Group: [`MaterialParameter`]
pub const GL_SHININESS: GLenum = GLenum(0x1601);

///
/// * Group: [`VertexAttribIType`], [`SecondaryColorPointerTypeIBM`],
///   [`WeightPointerTypeARB`], [`TangentPointerTypeEXT`],
///   [`BinormalPointerTypeEXT`], [`IndexPointerType`], [`ListNameType`],
///   [`NormalPointerType`], [`PixelType`], [`TexCoordPointerType`],
///   [`VertexPointerType`], [`VertexAttribType`], [`VertexAttribPointerType`]
pub const GL_SHORT: GLenum = GLenum(0x1402);

pub const GL_SIGNALED: GLenum = GLenum(0x9119);

pub const GL_SIGNALED_APPLE: GLenum = GLenum(0x9119);

pub const GL_SIGNED_ALPHA8_NV: GLenum = GLenum(0x8706);

pub const GL_SIGNED_ALPHA_NV: GLenum = GLenum(0x8705);

pub const GL_SIGNED_HILO16_NV: GLenum = GLenum(0x86FA);

pub const GL_SIGNED_HILO8_NV: GLenum = GLenum(0x885F);

pub const GL_SIGNED_HILO_NV: GLenum = GLenum(0x86F9);

///
/// * Group: [`CombinerMappingNV`]
pub const GL_SIGNED_IDENTITY_NV: GLenum = GLenum(0x853C);

pub const GL_SIGNED_INTENSITY8_NV: GLenum = GLenum(0x8708);

pub const GL_SIGNED_INTENSITY_NV: GLenum = GLenum(0x8707);

pub const GL_SIGNED_LUMINANCE8_ALPHA8_NV: GLenum = GLenum(0x8704);

pub const GL_SIGNED_LUMINANCE8_NV: GLenum = GLenum(0x8702);

pub const GL_SIGNED_LUMINANCE_ALPHA_NV: GLenum = GLenum(0x8703);

pub const GL_SIGNED_LUMINANCE_NV: GLenum = GLenum(0x8701);

///
/// * Group: [`CombinerMappingNV`]
pub const GL_SIGNED_NEGATE_NV: GLenum = GLenum(0x853D);

pub const GL_SIGNED_NORMALIZED: GLenum = GLenum(0x8F9C);

pub const GL_SIGNED_RGB8_NV: GLenum = GLenum(0x86FF);

pub const GL_SIGNED_RGB8_UNSIGNED_ALPHA8_NV: GLenum = GLenum(0x870D);

pub const GL_SIGNED_RGBA8_NV: GLenum = GLenum(0x86FC);

pub const GL_SIGNED_RGBA_NV: GLenum = GLenum(0x86FB);

pub const GL_SIGNED_RGB_NV: GLenum = GLenum(0x86FE);

pub const GL_SIGNED_RGB_UNSIGNED_ALPHA_NV: GLenum = GLenum(0x870C);

///
/// * Group: [`InternalFormatPName`]
pub const GL_SIMULTANEOUS_TEXTURE_AND_DEPTH_TEST: GLenum = GLenum(0x82AC);

///
/// * Group: [`InternalFormatPName`]
pub const GL_SIMULTANEOUS_TEXTURE_AND_DEPTH_WRITE: GLenum = GLenum(0x82AE);

///
/// * Group: [`InternalFormatPName`]
pub const GL_SIMULTANEOUS_TEXTURE_AND_STENCIL_TEST: GLenum = GLenum(0x82AD);

///
/// * Group: [`InternalFormatPName`]
pub const GL_SIMULTANEOUS_TEXTURE_AND_STENCIL_WRITE: GLenum = GLenum(0x82AF);

///
/// * Group: [`LightModelColorControl`]
pub const GL_SINGLE_COLOR: GLenum = GLenum(0x81F9);

///
/// * Group: [`LightModelColorControl`]
pub const GL_SINGLE_COLOR_EXT: GLenum = GLenum(0x81F9);

///
/// * Group: [`TransformFeedbackTokenNV`]
pub const GL_SKIP_COMPONENTS1_NV: GLenum = GLenum(u32::MAX - 6);

///
/// * Group: [`TransformFeedbackTokenNV`]
pub const GL_SKIP_COMPONENTS2_NV: GLenum = GLenum(u32::MAX - 5);

///
/// * Group: [`TransformFeedbackTokenNV`]
pub const GL_SKIP_COMPONENTS3_NV: GLenum = GLenum(u32::MAX - 4);

///
/// * Group: [`TransformFeedbackTokenNV`]
pub const GL_SKIP_COMPONENTS4_NV: GLenum = GLenum(u32::MAX - 3);

pub const GL_SKIP_DECODE_EXT: GLenum = GLenum(0x8A4A);

///
/// * Group: [`PathHandleMissingGlyphs`]
pub const GL_SKIP_MISSING_GLYPH_NV: GLenum = GLenum(0x90A9);

pub const GL_SLICE_ACCUM_SUN: GLenum = GLenum(0x85CC);

pub const GL_SLIM10U_SGIX: GLenum = GLenum(0x831E);

pub const GL_SLIM12S_SGIX: GLenum = GLenum(0x831F);

pub const GL_SLIM8U_SGIX: GLenum = GLenum(0x831D);

pub const GL_SLUMINANCE: GLenum = GLenum(0x8C46);

pub const GL_SLUMINANCE8: GLenum = GLenum(0x8C47);

pub const GL_SLUMINANCE8_ALPHA8: GLenum = GLenum(0x8C45);

pub const GL_SLUMINANCE8_ALPHA8_EXT: GLenum = GLenum(0x8C45);

pub const GL_SLUMINANCE8_ALPHA8_NV: GLenum = GLenum(0x8C45);

pub const GL_SLUMINANCE8_EXT: GLenum = GLenum(0x8C47);

pub const GL_SLUMINANCE8_NV: GLenum = GLenum(0x8C47);

pub const GL_SLUMINANCE_ALPHA: GLenum = GLenum(0x8C44);

pub const GL_SLUMINANCE_ALPHA_EXT: GLenum = GLenum(0x8C44);

pub const GL_SLUMINANCE_ALPHA_NV: GLenum = GLenum(0x8C44);

pub const GL_SLUMINANCE_EXT: GLenum = GLenum(0x8C46);

pub const GL_SLUMINANCE_NV: GLenum = GLenum(0x8C46);

///
/// * Group: [`PathCoordType`]
pub const GL_SMALL_CCW_ARC_TO_NV: GLenum = GLenum(0x12);

///
/// * Group: [`PathCoordType`]
pub const GL_SMALL_CW_ARC_TO_NV: GLenum = GLenum(0x14);

pub const GL_SMAPHS30_PROGRAM_BINARY_DMP: GLenum = GLenum(0x9251);

pub const GL_SMAPHS_PROGRAM_BINARY_DMP: GLenum = GLenum(0x9252);

///
/// * Group: [`ShadingModel`]
pub const GL_SMOOTH: GLenum = GLenum(0x1D01);

///
/// * Group: [`PathCoordType`]
pub const GL_SMOOTH_CUBIC_CURVE_TO_NV: GLenum = GLenum(0x10);

///
/// * Group: [`GetPName`]
/// * Alias Of: [`GL_LINE_WIDTH_GRANULARITY`]
pub const GL_SMOOTH_LINE_WIDTH_GRANULARITY: GLenum = GL_LINE_WIDTH_GRANULARITY;

///
/// * Group: [`GetPName`]
/// * Alias Of: [`GL_LINE_WIDTH_RANGE`]
pub const GL_SMOOTH_LINE_WIDTH_RANGE: GLenum = GL_LINE_WIDTH_RANGE;

///
/// * Group: [`GetPName`]
/// * Alias Of: [`GL_POINT_SIZE_GRANULARITY`]
pub const GL_SMOOTH_POINT_SIZE_GRANULARITY: GLenum = GL_POINT_SIZE_GRANULARITY;

///
/// * Group: [`GetPName`]
/// * Alias Of: [`GL_POINT_SIZE_RANGE`]
pub const GL_SMOOTH_POINT_SIZE_RANGE: GLenum = GL_POINT_SIZE_RANGE;

///
/// * Group: [`PathCoordType`]
pub const GL_SMOOTH_QUADRATIC_CURVE_TO_NV: GLenum = GLenum(0x0E);

pub const GL_SM_COUNT_NV: GLenum = GLenum(0x933B);

pub const GL_SOFTLIGHT: GLenum = GLenum(0x929C);

pub const GL_SOFTLIGHT_KHR: GLenum = GLenum(0x929C);

pub const GL_SOFTLIGHT_NV: GLenum = GLenum(0x929C);

pub const GL_SOURCE0_ALPHA: GLenum = GLenum(0x8588);

pub const GL_SOURCE0_ALPHA_ARB: GLenum = GLenum(0x8588);

pub const GL_SOURCE0_ALPHA_EXT: GLenum = GLenum(0x8588);

pub const GL_SOURCE0_RGB: GLenum = GLenum(0x8580);

pub const GL_SOURCE0_RGB_ARB: GLenum = GLenum(0x8580);

pub const GL_SOURCE0_RGB_EXT: GLenum = GLenum(0x8580);

pub const GL_SOURCE1_ALPHA: GLenum = GLenum(0x8589);

pub const GL_SOURCE1_ALPHA_ARB: GLenum = GLenum(0x8589);

pub const GL_SOURCE1_ALPHA_EXT: GLenum = GLenum(0x8589);

pub const GL_SOURCE1_RGB: GLenum = GLenum(0x8581);

pub const GL_SOURCE1_RGB_ARB: GLenum = GLenum(0x8581);

pub const GL_SOURCE1_RGB_EXT: GLenum = GLenum(0x8581);

pub const GL_SOURCE2_ALPHA: GLenum = GLenum(0x858A);

pub const GL_SOURCE2_ALPHA_ARB: GLenum = GLenum(0x858A);

pub const GL_SOURCE2_ALPHA_EXT: GLenum = GLenum(0x858A);

pub const GL_SOURCE2_RGB: GLenum = GLenum(0x8582);

pub const GL_SOURCE2_RGB_ARB: GLenum = GLenum(0x8582);

pub const GL_SOURCE2_RGB_EXT: GLenum = GLenum(0x8582);

pub const GL_SOURCE3_ALPHA_NV: GLenum = GLenum(0x858B);

pub const GL_SOURCE3_RGB_NV: GLenum = GLenum(0x8583);

///
/// * Group: [`CombinerRegisterNV`]
pub const GL_SPARE0_NV: GLenum = GLenum(0x852E);

pub const GL_SPARE0_PLUS_SECONDARY_COLOR_NV: GLenum = GLenum(0x8532);

///
/// * Group: [`CombinerRegisterNV`]
pub const GL_SPARE1_NV: GLenum = GLenum(0x852F);

pub const GL_SPARSE_BUFFER_PAGE_SIZE_ARB: GLenum = GLenum(0x82F8);

///
/// * Group: [`BufferStorageMask`]
pub const GL_SPARSE_STORAGE_BIT_ARB: GLbitfield = GLbitfield(0x0400);

pub const GL_SPARSE_TEXTURE_FULL_ARRAY_CUBE_MIPMAPS_ARB: GLenum = GLenum(0x91A9);

pub const GL_SPARSE_TEXTURE_FULL_ARRAY_CUBE_MIPMAPS_EXT: GLenum = GLenum(0x91A9);

///
/// * Group: [`MaterialParameter`], [`FragmentLightParameterSGIX`],
///   [`ColorMaterialParameter`]
pub const GL_SPECULAR: GLenum = GLenum(0x1202);

///
/// * Group: [`TextureGenMode`]
pub const GL_SPHERE_MAP: GLenum = GLenum(0x2402);

pub const GL_SPIR_V_BINARY: GLenum = GLenum(0x9552);

///
/// * Alias Of: [`GL_SPIR_V_BINARY`]
pub const GL_SPIR_V_BINARY_ARB: GLenum = GL_SPIR_V_BINARY;

pub const GL_SPIR_V_EXTENSIONS: GLenum = GLenum(0x9553);

///
/// * Group: [`LightParameter`], [`FragmentLightParameterSGIX`]
pub const GL_SPOT_CUTOFF: GLenum = GLenum(0x1206);

///
/// * Group: [`LightParameter`], [`FragmentLightParameterSGIX`]
pub const GL_SPOT_DIRECTION: GLenum = GLenum(0x1204);

///
/// * Group: [`LightParameter`], [`FragmentLightParameterSGIX`]
pub const GL_SPOT_EXPONENT: GLenum = GLenum(0x1205);

pub const GL_SPRITE_AXIAL_SGIX: GLenum = GLenum(0x814C);

///
/// * Group: [`GetPName`]
pub const GL_SPRITE_AXIS_SGIX: GLenum = GLenum(0x814A);

pub const GL_SPRITE_EYE_ALIGNED_SGIX: GLenum = GLenum(0x814E);

///
/// * Group: [`GetPName`], [`SpriteParameterNameSGIX`]
pub const GL_SPRITE_MODE_SGIX: GLenum = GLenum(0x8149);

pub const GL_SPRITE_OBJECT_ALIGNED_SGIX: GLenum = GLenum(0x814D);

///
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_SPRITE_SGIX: GLenum = GLenum(0x8148);

///
/// * Group: [`GetPName`]
pub const GL_SPRITE_TRANSLATION_SGIX: GLenum = GLenum(0x814B);

pub const GL_SQUARE_NV: GLenum = GLenum(0x90A3);

///
/// * Group: [`InternalFormat`]
pub const GL_SR8_EXT: GLenum = GLenum(0x8FBD);

///
/// * Alias Of: [`GL_SOURCE0_ALPHA`]
pub const GL_SRC0_ALPHA: GLenum = GL_SOURCE0_ALPHA;

///
/// * Alias Of: [`GL_SOURCE0_RGB`]
pub const GL_SRC0_RGB: GLenum = GL_SOURCE0_RGB;

///
/// * Group: [`BlendingFactor`]
/// * Alias Of: [`GL_SOURCE1_ALPHA`]
pub const GL_SRC1_ALPHA: GLenum = GL_SOURCE1_ALPHA;

pub const GL_SRC1_ALPHA_EXT: GLenum = GLenum(0x8589);

///
/// * Group: [`BlendingFactor`]
pub const GL_SRC1_COLOR: GLenum = GLenum(0x88F9);

pub const GL_SRC1_COLOR_EXT: GLenum = GLenum(0x88F9);

///
/// * Alias Of: [`GL_SOURCE1_RGB`]
pub const GL_SRC1_RGB: GLenum = GL_SOURCE1_RGB;

///
/// * Alias Of: [`GL_SOURCE2_ALPHA`]
pub const GL_SRC2_ALPHA: GLenum = GL_SOURCE2_ALPHA;

///
/// * Alias Of: [`GL_SOURCE2_RGB`]
pub const GL_SRC2_RGB: GLenum = GL_SOURCE2_RGB;

///
/// * Group: [`BlendingFactor`]
pub const GL_SRC_ALPHA: GLenum = GLenum(0x0302);

///
/// * Group: [`BlendingFactor`]
pub const GL_SRC_ALPHA_SATURATE: GLenum = GLenum(0x0308);

pub const GL_SRC_ALPHA_SATURATE_EXT: GLenum = GLenum(0x0308);

pub const GL_SRC_ATOP_NV: GLenum = GLenum(0x928E);

///
/// * Group: [`BlendingFactor`]
pub const GL_SRC_COLOR: GLenum = GLenum(0x0300);

pub const GL_SRC_IN_NV: GLenum = GLenum(0x928A);

pub const GL_SRC_NV: GLenum = GLenum(0x9286);

pub const GL_SRC_OUT_NV: GLenum = GLenum(0x928C);

pub const GL_SRC_OVER_NV: GLenum = GLenum(0x9288);

///
/// * Group: [`InternalFormat`]
pub const GL_SRG8_EXT: GLenum = GLenum(0x8FBE);

///
/// * Group: [`InternalFormat`]
pub const GL_SRGB: GLenum = GLenum(0x8C40);

///
/// * Group: [`InternalFormat`]
pub const GL_SRGB8: GLenum = GLenum(0x8C41);

///
/// * Group: [`InternalFormat`]
pub const GL_SRGB8_ALPHA8: GLenum = GLenum(0x8C43);

///
/// * Group: [`InternalFormat`]
pub const GL_SRGB8_ALPHA8_EXT: GLenum = GLenum(0x8C43);

///
/// * Group: [`InternalFormat`]
pub const GL_SRGB8_EXT: GLenum = GLenum(0x8C41);

///
/// * Group: [`InternalFormat`]
pub const GL_SRGB8_NV: GLenum = GLenum(0x8C41);

///
/// * Group: [`InternalFormat`]
pub const GL_SRGB_ALPHA: GLenum = GLenum(0x8C42);

///
/// * Group: [`InternalFormat`]
pub const GL_SRGB_ALPHA_EXT: GLenum = GLenum(0x8C42);

pub const GL_SRGB_DECODE_ARB: GLenum = GLenum(0x8299);

///
/// * Group: [`InternalFormat`]
pub const GL_SRGB_EXT: GLenum = GLenum(0x8C40);

///
/// * Group: [`InternalFormatPName`]
pub const GL_SRGB_READ: GLenum = GLenum(0x8297);

///
/// * Group: [`InternalFormatPName`]
pub const GL_SRGB_WRITE: GLenum = GLenum(0x8298);

///
/// * Group: [`ErrorCode`]
pub const GL_STACK_OVERFLOW: GLenum = GLenum(0x0503);

pub const GL_STACK_OVERFLOW_KHR: GLenum = GLenum(0x0503);

///
/// * Group: [`ErrorCode`]
pub const GL_STACK_UNDERFLOW: GLenum = GLenum(0x0504);

pub const GL_STACK_UNDERFLOW_KHR: GLenum = GLenum(0x0504);

pub const GL_STANDARD_FONT_FORMAT_NV: GLenum = GLenum(0x936C);

///
/// * Group: [`PathFontTarget`]
pub const GL_STANDARD_FONT_NAME_NV: GLenum = GLenum(0x9072);

pub const GL_STATE_RESTORE: GLenum = GLenum(0x8BDC);

///
/// * Group: [`ArrayObjectUsageATI`]
pub const GL_STATIC_ATI: GLenum = GLenum(0x8760);

///
/// * Group: [`VertexBufferObjectUsage`], [`BufferUsageARB`]
pub const GL_STATIC_COPY: GLenum = GLenum(0x88E6);

pub const GL_STATIC_COPY_ARB: GLenum = GLenum(0x88E6);

///
/// * Group: [`VertexBufferObjectUsage`], [`BufferUsageARB`]
pub const GL_STATIC_DRAW: GLenum = GLenum(0x88E4);

pub const GL_STATIC_DRAW_ARB: GLenum = GLenum(0x88E4);

///
/// * Group: [`VertexBufferObjectUsage`], [`BufferUsageARB`]
pub const GL_STATIC_READ: GLenum = GLenum(0x88E5);

pub const GL_STATIC_READ_ARB: GLenum = GLenum(0x88E5);

pub const GL_STATIC_VERTEX_ARRAY_IBM: GLenum = GLenum(103061);

///
/// * Group: [`Buffer`], [`PixelCopyType`], [`InvalidateFramebufferAttachment`]
pub const GL_STENCIL: GLenum = GLenum(0x1802);

///
/// * Group: [`FramebufferAttachment`]
pub const GL_STENCIL_ATTACHMENT: GLenum = GLenum(0x8D20);

///
/// * Group: [`InvalidateFramebufferAttachment`]
pub const GL_STENCIL_ATTACHMENT_EXT: GLenum = GLenum(0x8D20);

///
/// * Group: [`InvalidateFramebufferAttachment`]
pub const GL_STENCIL_ATTACHMENT_OES: GLenum = GLenum(0x8D20);

///
/// * Group: [`GetPName`]
pub const GL_STENCIL_BACK_FAIL: GLenum = GLenum(0x8801);

pub const GL_STENCIL_BACK_FAIL_ATI: GLenum = GLenum(0x8801);

///
/// * Group: [`GetPName`]
pub const GL_STENCIL_BACK_FUNC: GLenum = GLenum(0x8800);

pub const GL_STENCIL_BACK_FUNC_ATI: GLenum = GLenum(0x8800);

pub const GL_STENCIL_BACK_OP_VALUE_AMD: GLenum = GLenum(0x874D);

///
/// * Group: [`GetPName`]
pub const GL_STENCIL_BACK_PASS_DEPTH_FAIL: GLenum = GLenum(0x8802);

pub const GL_STENCIL_BACK_PASS_DEPTH_FAIL_ATI: GLenum = GLenum(0x8802);

///
/// * Group: [`GetPName`]
pub const GL_STENCIL_BACK_PASS_DEPTH_PASS: GLenum = GLenum(0x8803);

pub const GL_STENCIL_BACK_PASS_DEPTH_PASS_ATI: GLenum = GLenum(0x8803);

///
/// * Group: [`GetPName`]
pub const GL_STENCIL_BACK_REF: GLenum = GLenum(0x8CA3);

///
/// * Group: [`GetPName`]
pub const GL_STENCIL_BACK_VALUE_MASK: GLenum = GLenum(0x8CA4);

///
/// * Group: [`GetPName`]
pub const GL_STENCIL_BACK_WRITEMASK: GLenum = GLenum(0x8CA5);

///
/// * Group: [`GetPName`]
pub const GL_STENCIL_BITS: GLenum = GLenum(0x0D57);

///
/// * Group: [`ClearBufferMask`], [`AttribMask`]
pub const GL_STENCIL_BUFFER_BIT: GLbitfield = GLbitfield(0x00000400);

///
/// * Group: [`BufferBitQCOM`]
pub const GL_STENCIL_BUFFER_BIT0_QCOM: GLbitfield = GLbitfield(0x00010000);

///
/// * Group: [`BufferBitQCOM`]
pub const GL_STENCIL_BUFFER_BIT1_QCOM: GLbitfield = GLbitfield(0x00020000);

///
/// * Group: [`BufferBitQCOM`]
pub const GL_STENCIL_BUFFER_BIT2_QCOM: GLbitfield = GLbitfield(0x00040000);

///
/// * Group: [`BufferBitQCOM`]
pub const GL_STENCIL_BUFFER_BIT3_QCOM: GLbitfield = GLbitfield(0x00080000);

///
/// * Group: [`BufferBitQCOM`]
pub const GL_STENCIL_BUFFER_BIT4_QCOM: GLbitfield = GLbitfield(0x00100000);

///
/// * Group: [`BufferBitQCOM`]
pub const GL_STENCIL_BUFFER_BIT5_QCOM: GLbitfield = GLbitfield(0x00200000);

///
/// * Group: [`BufferBitQCOM`]
pub const GL_STENCIL_BUFFER_BIT6_QCOM: GLbitfield = GLbitfield(0x00400000);

///
/// * Group: [`BufferBitQCOM`]
pub const GL_STENCIL_BUFFER_BIT7_QCOM: GLbitfield = GLbitfield(0x00800000);

pub const GL_STENCIL_CLEAR_TAG_VALUE_EXT: GLenum = GLenum(0x88F3);

///
/// * Group: [`GetPName`]
pub const GL_STENCIL_CLEAR_VALUE: GLenum = GLenum(0x0B91);

pub const GL_STENCIL_COMPONENTS: GLenum = GLenum(0x8285);

///
/// * Group: [`PixelCopyType`]
pub const GL_STENCIL_EXT: GLenum = GLenum(0x1802);

///
/// * Group: [`GetPName`]
pub const GL_STENCIL_FAIL: GLenum = GLenum(0x0B94);

///
/// * Group: [`GetPName`]
pub const GL_STENCIL_FUNC: GLenum = GLenum(0x0B92);

///
/// * Group: [`InternalFormat`], [`PixelFormat`]
pub const GL_STENCIL_INDEX: GLenum = GLenum(0x1901);

///
/// * Group: [`InternalFormat`]
pub const GL_STENCIL_INDEX1: GLenum = GLenum(0x8D46);

///
/// * Group: [`InternalFormat`]
pub const GL_STENCIL_INDEX16: GLenum = GLenum(0x8D49);

///
/// * Group: [`InternalFormat`]
pub const GL_STENCIL_INDEX16_EXT: GLenum = GLenum(0x8D49);

///
/// * Group: [`InternalFormat`]
pub const GL_STENCIL_INDEX1_EXT: GLenum = GLenum(0x8D46);

///
/// * Group: [`InternalFormat`]
pub const GL_STENCIL_INDEX1_OES: GLenum = GLenum(0x8D46);

///
/// * Group: [`InternalFormat`]
pub const GL_STENCIL_INDEX4: GLenum = GLenum(0x8D47);

///
/// * Group: [`InternalFormat`]
pub const GL_STENCIL_INDEX4_EXT: GLenum = GLenum(0x8D47);

///
/// * Group: [`InternalFormat`]
pub const GL_STENCIL_INDEX4_OES: GLenum = GLenum(0x8D47);

///
/// * Group: [`InternalFormat`]
pub const GL_STENCIL_INDEX8: GLenum = GLenum(0x8D48);

///
/// * Group: [`InternalFormat`]
pub const GL_STENCIL_INDEX8_EXT: GLenum = GLenum(0x8D48);

///
/// * Group: [`InternalFormat`]
pub const GL_STENCIL_INDEX8_OES: GLenum = GLenum(0x8D48);

///
/// * Group: [`InternalFormat`]
pub const GL_STENCIL_INDEX_OES: GLenum = GLenum(0x1901);

pub const GL_STENCIL_OP_VALUE_AMD: GLenum = GLenum(0x874C);

///
/// * Group: [`GetPName`]
pub const GL_STENCIL_PASS_DEPTH_FAIL: GLenum = GLenum(0x0B95);

///
/// * Group: [`GetPName`]
pub const GL_STENCIL_PASS_DEPTH_PASS: GLenum = GLenum(0x0B96);

///
/// * Group: [`GetPName`]
pub const GL_STENCIL_REF: GLenum = GLenum(0x0B97);

///
/// * Group: [`CommandOpcodesNV`]
pub const GL_STENCIL_REF_COMMAND_NV: GLenum = GLenum(0x000C);

///
/// * Group: [`InternalFormatPName`]
pub const GL_STENCIL_RENDERABLE: GLenum = GLenum(0x8288);

pub const GL_STENCIL_SAMPLES_NV: GLenum = GLenum(0x932E);

pub const GL_STENCIL_TAG_BITS_EXT: GLenum = GLenum(0x88F2);

///
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_STENCIL_TEST: GLenum = GLenum(0x0B90);

pub const GL_STENCIL_TEST_TWO_SIDE_EXT: GLenum = GLenum(0x8910);

///
/// * Group: [`GetPName`]
pub const GL_STENCIL_VALUE_MASK: GLenum = GLenum(0x0B93);

///
/// * Group: [`GetPName`]
pub const GL_STENCIL_WRITEMASK: GLenum = GLenum(0x0B98);

///
/// * Group: [`GetFramebufferParameter`], [`GetPName`]
pub const GL_STEREO: GLenum = GLenum(0x0C33);

///
/// * Group: [`VertexArrayPNameAPPLE`]
pub const GL_STORAGE_CACHED_APPLE: GLenum = GLenum(0x85BE);

///
/// * Group: [`VertexArrayPNameAPPLE`]
pub const GL_STORAGE_CLIENT_APPLE: GLenum = GLenum(0x85B4);

pub const GL_STORAGE_PRIVATE_APPLE: GLenum = GLenum(0x85BD);

///
/// * Group: [`VertexArrayPNameAPPLE`]
pub const GL_STORAGE_SHARED_APPLE: GLenum = GLenum(0x85BF);

///
/// * Group: [`VertexBufferObjectUsage`], [`BufferUsageARB`]
pub const GL_STREAM_COPY: GLenum = GLenum(0x88E2);

pub const GL_STREAM_COPY_ARB: GLenum = GLenum(0x88E2);

///
/// * Group: [`VertexBufferObjectUsage`], [`BufferUsageARB`]
pub const GL_STREAM_DRAW: GLenum = GLenum(0x88E0);

pub const GL_STREAM_DRAW_ARB: GLenum = GLenum(0x88E0);

pub const GL_STREAM_RASTERIZATION_AMD: GLenum = GLenum(0x91A0);

///
/// * Group: [`VertexBufferObjectUsage`], [`BufferUsageARB`]
pub const GL_STREAM_READ: GLenum = GLenum(0x88E1);

pub const GL_STREAM_READ_ARB: GLenum = GLenum(0x88E1);

///
/// * Group: [`HintTarget`]
pub const GL_STRICT_DEPTHFUNC_HINT_PGI: GLenum = GLenum(0x1A216);

///
/// * Group: [`HintTarget`]
pub const GL_STRICT_LIGHTING_HINT_PGI: GLenum = GLenum(0x1A217);

///
/// * Group: [`HintTarget`]
pub const GL_STRICT_SCISSOR_HINT_PGI: GLenum = GLenum(0x1A218);

///
/// * Group: [`SubgroupSupportedFeatures`]
pub const GL_SUBGROUP_FEATURE_ARITHMETIC_BIT_KHR: GLbitfield = GLbitfield(0x00000004);

///
/// * Group: [`SubgroupSupportedFeatures`]
pub const GL_SUBGROUP_FEATURE_BALLOT_BIT_KHR: GLbitfield = GLbitfield(0x00000008);

///
/// * Group: [`SubgroupSupportedFeatures`]
pub const GL_SUBGROUP_FEATURE_BASIC_BIT_KHR: GLbitfield = GLbitfield(0x00000001);

///
/// * Group: [`SubgroupSupportedFeatures`]
pub const GL_SUBGROUP_FEATURE_CLUSTERED_BIT_KHR: GLbitfield = GLbitfield(0x00000040);

///
/// * Group: [`SubgroupSupportedFeatures`]
pub const GL_SUBGROUP_FEATURE_PARTITIONED_BIT_NV: GLbitfield = GLbitfield(0x00000100);

///
/// * Group: [`SubgroupSupportedFeatures`]
pub const GL_SUBGROUP_FEATURE_QUAD_BIT_KHR: GLbitfield = GLbitfield(0x00000080);

///
/// * Group: [`SubgroupSupportedFeatures`]
pub const GL_SUBGROUP_FEATURE_SHUFFLE_BIT_KHR: GLbitfield = GLbitfield(0x00000010);

///
/// * Group: [`SubgroupSupportedFeatures`]
pub const GL_SUBGROUP_FEATURE_SHUFFLE_RELATIVE_BIT_KHR: GLbitfield = GLbitfield(0x00000020);

///
/// * Group: [`SubgroupSupportedFeatures`]
pub const GL_SUBGROUP_FEATURE_VOTE_BIT_KHR: GLbitfield = GLbitfield(0x00000002);

pub const GL_SUBGROUP_QUAD_ALL_STAGES_KHR: GLenum = GLenum(0x9535);

pub const GL_SUBGROUP_SIZE_KHR: GLenum = GLenum(0x9532);

pub const GL_SUBGROUP_SUPPORTED_FEATURES_KHR: GLenum = GLenum(0x9534);

pub const GL_SUBGROUP_SUPPORTED_STAGES_KHR: GLenum = GLenum(0x9533);

///
/// * Group: [`GetPName`]
pub const GL_SUBPIXEL_BITS: GLenum = GLenum(0x0D50);

pub const GL_SUBPIXEL_PRECISION_BIAS_X_BITS_NV: GLenum = GLenum(0x9347);

pub const GL_SUBPIXEL_PRECISION_BIAS_Y_BITS_NV: GLenum = GLenum(0x9348);

pub const GL_SUBSAMPLE_DISTANCE_AMD: GLenum = GLenum(0x883F);

pub const GL_SUBTRACT: GLenum = GLenum(0x84E7);

pub const GL_SUBTRACT_ARB: GLenum = GLenum(0x84E7);

///
/// * Group: [`FragmentOpATI`]
pub const GL_SUB_ATI: GLenum = GLenum(0x8965);

pub const GL_SUCCESS_NV: GLenum = GLenum(0x902F);

pub const GL_SUPERSAMPLE_SCALE_X_NV: GLenum = GLenum(0x9372);

pub const GL_SUPERSAMPLE_SCALE_Y_NV: GLenum = GLenum(0x9373);

pub const GL_SUPPORTED_MULTISAMPLE_MODES_AMD: GLenum = GLenum(0x91B7);

pub const GL_SURFACE_MAPPED_NV: GLenum = GLenum(0x8700);

pub const GL_SURFACE_REGISTERED_NV: GLenum = GLenum(0x86FD);

pub const GL_SURFACE_STATE_NV: GLenum = GLenum(0x86EB);

///
/// * Group: [`SwizzleOpATI`]
pub const GL_SWIZZLE_STQ_ATI: GLenum = GLenum(0x8977);

///
/// * Group: [`SwizzleOpATI`]
pub const GL_SWIZZLE_STQ_DQ_ATI: GLenum = GLenum(0x8979);

pub const GL_SWIZZLE_STRQ_ATI: GLenum = GLenum(0x897A);

pub const GL_SWIZZLE_STRQ_DQ_ATI: GLenum = GLenum(0x897B);

///
/// * Group: [`SwizzleOpATI`]
pub const GL_SWIZZLE_STR_ATI: GLenum = GLenum(0x8976);

///
/// * Group: [`SwizzleOpATI`]
pub const GL_SWIZZLE_STR_DR_ATI: GLenum = GLenum(0x8978);

pub const GL_SYNC_CL_EVENT_ARB: GLenum = GLenum(0x8240);

pub const GL_SYNC_CL_EVENT_COMPLETE_ARB: GLenum = GLenum(0x8241);

///
/// * Group: [`SyncParameterName`]
pub const GL_SYNC_CONDITION: GLenum = GLenum(0x9113);

pub const GL_SYNC_CONDITION_APPLE: GLenum = GLenum(0x9113);

pub const GL_SYNC_FENCE: GLenum = GLenum(0x9116);

pub const GL_SYNC_FENCE_APPLE: GLenum = GLenum(0x9116);

///
/// * Group: [`SyncParameterName`]
pub const GL_SYNC_FLAGS: GLenum = GLenum(0x9115);

pub const GL_SYNC_FLAGS_APPLE: GLenum = GLenum(0x9115);

///
/// * Group: [`SyncObjectMask`]
pub const GL_SYNC_FLUSH_COMMANDS_BIT: GLbitfield = GLbitfield(0x00000001);

///
/// * Group: [`SyncObjectMask`]
pub const GL_SYNC_FLUSH_COMMANDS_BIT_APPLE: GLbitfield = GLbitfield(0x00000001);

///
/// * Group: [`SyncCondition`]
pub const GL_SYNC_GPU_COMMANDS_COMPLETE: GLenum = GLenum(0x9117);

pub const GL_SYNC_GPU_COMMANDS_COMPLETE_APPLE: GLenum = GLenum(0x9117);

pub const GL_SYNC_OBJECT_APPLE: GLenum = GLenum(0x8A53);

///
/// * Group: [`SyncParameterName`]
pub const GL_SYNC_STATUS: GLenum = GLenum(0x9114);

pub const GL_SYNC_STATUS_APPLE: GLenum = GLenum(0x9114);

pub const GL_SYNC_X11_FENCE_EXT: GLenum = GLenum(0x90E1);

///
/// * Group: [`PathFontTarget`]
pub const GL_SYSTEM_FONT_NAME_NV: GLenum = GLenum(0x9073);

///
/// * Group: [`TextureCoordName`]
pub const GL_T: GLenum = GLenum(0x2001);

///
/// * Group: [`InterleavedArrayFormat`]
pub const GL_T2F_C3F_V3F: GLenum = GLenum(0x2A2A);

///
/// * Group: [`InterleavedArrayFormat`]
pub const GL_T2F_C4F_N3F_V3F: GLenum = GLenum(0x2A2C);

///
/// * Group: [`InterleavedArrayFormat`]
pub const GL_T2F_C4UB_V3F: GLenum = GLenum(0x2A29);

pub const GL_T2F_IUI_N3F_V2F_EXT: GLenum = GLenum(0x81B3);

pub const GL_T2F_IUI_N3F_V3F_EXT: GLenum = GLenum(0x81B4);

pub const GL_T2F_IUI_V2F_EXT: GLenum = GLenum(0x81B1);

pub const GL_T2F_IUI_V3F_EXT: GLenum = GLenum(0x81B2);

///
/// * Group: [`InterleavedArrayFormat`]
pub const GL_T2F_N3F_V3F: GLenum = GLenum(0x2A2B);

///
/// * Group: [`InterleavedArrayFormat`]
pub const GL_T2F_V3F: GLenum = GLenum(0x2A27);

///
/// * Group: [`InterleavedArrayFormat`]
pub const GL_T4F_C4F_N3F_V4F: GLenum = GLenum(0x2A2D);

///
/// * Group: [`InterleavedArrayFormat`]
pub const GL_T4F_V4F: GLenum = GLenum(0x2A28);

///
/// * Group: [`ErrorCode`]
pub const GL_TABLE_TOO_LARGE: GLenum = GLenum(0x8031);

///
/// * Group: [`ErrorCode`]
pub const GL_TABLE_TOO_LARGE_EXT: GLenum = GLenum(0x8031);

pub const GL_TANGENT_ARRAY_EXT: GLenum = GLenum(0x8439);

pub const GL_TANGENT_ARRAY_POINTER_EXT: GLenum = GLenum(0x8442);

pub const GL_TANGENT_ARRAY_STRIDE_EXT: GLenum = GLenum(0x843F);

pub const GL_TANGENT_ARRAY_TYPE_EXT: GLenum = GLenum(0x843E);

///
/// * Group: [`UseProgramStageMask`]
pub const GL_TASK_SHADER_BIT_NV: GLbitfield = GLbitfield(0x00000080);

pub const GL_TASK_SHADER_NV: GLenum = GLenum(0x955A);

pub const GL_TASK_SUBROUTINE_NV: GLenum = GLenum(0x957D);

pub const GL_TASK_SUBROUTINE_UNIFORM_NV: GLenum = GLenum(0x957F);

pub const GL_TASK_WORK_GROUP_SIZE_NV: GLenum = GLenum(0x953F);

///
/// * Group: [`CommandOpcodesNV`]
pub const GL_TERMINATE_SEQUENCE_COMMAND_NV: GLenum = GLenum(0x0000);

pub const GL_TESSELLATION_FACTOR_AMD: GLenum = GLenum(0x9005);

pub const GL_TESSELLATION_MODE_AMD: GLenum = GLenum(0x9004);

pub const GL_TESS_CONTROL_OUTPUT_VERTICES: GLenum = GLenum(0x8E75);

pub const GL_TESS_CONTROL_OUTPUT_VERTICES_EXT: GLenum = GLenum(0x8E75);

pub const GL_TESS_CONTROL_OUTPUT_VERTICES_OES: GLenum = GLenum(0x8E75);

///
/// * Group: [`ProgramTarget`]
pub const GL_TESS_CONTROL_PROGRAM_NV: GLenum = GLenum(0x891E);

pub const GL_TESS_CONTROL_PROGRAM_PARAMETER_BUFFER_NV: GLenum = GLenum(0x8C74);

///
/// * Group: [`PipelineParameterName`], [`ShaderType`]
pub const GL_TESS_CONTROL_SHADER: GLenum = GLenum(0x8E88);

///
/// * Group: [`UseProgramStageMask`]
pub const GL_TESS_CONTROL_SHADER_BIT: GLbitfield = GLbitfield(0x00000008);

///
/// * Group: [`UseProgramStageMask`]
pub const GL_TESS_CONTROL_SHADER_BIT_EXT: GLbitfield = GLbitfield(0x00000008);

///
/// * Group: [`UseProgramStageMask`]
pub const GL_TESS_CONTROL_SHADER_BIT_OES: GLbitfield = GLbitfield(0x00000008);

pub const GL_TESS_CONTROL_SHADER_EXT: GLenum = GLenum(0x8E88);

pub const GL_TESS_CONTROL_SHADER_OES: GLenum = GLenum(0x8E88);

pub const GL_TESS_CONTROL_SHADER_PATCHES: GLenum = GLenum(0x82F1);

///
/// * Alias Of: [`GL_TESS_CONTROL_SHADER_PATCHES`]
pub const GL_TESS_CONTROL_SHADER_PATCHES_ARB: GLenum = GL_TESS_CONTROL_SHADER_PATCHES;

///
/// * Group: [`ProgramInterface`]
pub const GL_TESS_CONTROL_SUBROUTINE: GLenum = GLenum(0x92E9);

///
/// * Group: [`ProgramInterface`]
pub const GL_TESS_CONTROL_SUBROUTINE_UNIFORM: GLenum = GLenum(0x92EF);

///
/// * Group: [`InternalFormatPName`]
pub const GL_TESS_CONTROL_TEXTURE: GLenum = GLenum(0x829C);

///
/// * Group: [`ProgramTarget`]
pub const GL_TESS_EVALUATION_PROGRAM_NV: GLenum = GLenum(0x891F);

pub const GL_TESS_EVALUATION_PROGRAM_PARAMETER_BUFFER_NV: GLenum = GLenum(0x8C75);

///
/// * Group: [`PipelineParameterName`], [`ShaderType`]
pub const GL_TESS_EVALUATION_SHADER: GLenum = GLenum(0x8E87);

///
/// * Group: [`UseProgramStageMask`]
pub const GL_TESS_EVALUATION_SHADER_BIT: GLbitfield = GLbitfield(0x00000010);

///
/// * Group: [`UseProgramStageMask`]
pub const GL_TESS_EVALUATION_SHADER_BIT_EXT: GLbitfield = GLbitfield(0x00000010);

///
/// * Group: [`UseProgramStageMask`]
pub const GL_TESS_EVALUATION_SHADER_BIT_OES: GLbitfield = GLbitfield(0x00000010);

pub const GL_TESS_EVALUATION_SHADER_EXT: GLenum = GLenum(0x8E87);

pub const GL_TESS_EVALUATION_SHADER_INVOCATIONS: GLenum = GLenum(0x82F2);

///
/// * Alias Of: [`GL_TESS_EVALUATION_SHADER_INVOCATIONS`]
pub const GL_TESS_EVALUATION_SHADER_INVOCATIONS_ARB: GLenum = GL_TESS_EVALUATION_SHADER_INVOCATIONS;

pub const GL_TESS_EVALUATION_SHADER_OES: GLenum = GLenum(0x8E87);

///
/// * Group: [`ProgramInterface`]
pub const GL_TESS_EVALUATION_SUBROUTINE: GLenum = GLenum(0x92EA);

///
/// * Group: [`ProgramInterface`]
pub const GL_TESS_EVALUATION_SUBROUTINE_UNIFORM: GLenum = GLenum(0x92F0);

///
/// * Group: [`InternalFormatPName`]
pub const GL_TESS_EVALUATION_TEXTURE: GLenum = GLenum(0x829D);

pub const GL_TESS_GEN_MODE: GLenum = GLenum(0x8E76);

pub const GL_TESS_GEN_MODE_EXT: GLenum = GLenum(0x8E76);

pub const GL_TESS_GEN_MODE_OES: GLenum = GLenum(0x8E76);

pub const GL_TESS_GEN_POINT_MODE: GLenum = GLenum(0x8E79);

pub const GL_TESS_GEN_POINT_MODE_EXT: GLenum = GLenum(0x8E79);

pub const GL_TESS_GEN_POINT_MODE_OES: GLenum = GLenum(0x8E79);

pub const GL_TESS_GEN_SPACING: GLenum = GLenum(0x8E77);

pub const GL_TESS_GEN_SPACING_EXT: GLenum = GLenum(0x8E77);

pub const GL_TESS_GEN_SPACING_OES: GLenum = GLenum(0x8E77);

pub const GL_TESS_GEN_VERTEX_ORDER: GLenum = GLenum(0x8E78);

pub const GL_TESS_GEN_VERTEX_ORDER_EXT: GLenum = GLenum(0x8E78);

pub const GL_TESS_GEN_VERTEX_ORDER_OES: GLenum = GLenum(0x8E78);

///
/// * Group: [`VertexHintsMaskPGI`]
pub const GL_TEXCOORD1_BIT_PGI: GLbitfield = GLbitfield(0x10000000);

///
/// * Group: [`VertexHintsMaskPGI`]
pub const GL_TEXCOORD2_BIT_PGI: GLbitfield = GLbitfield(0x20000000);

///
/// * Group: [`VertexHintsMaskPGI`]
pub const GL_TEXCOORD3_BIT_PGI: GLbitfield = GLbitfield(0x40000000);

///
/// * Group: [`VertexHintsMaskPGI`]
pub const GL_TEXCOORD4_BIT_PGI: GLbitfield = GLbitfield(0x80000000);

///
/// * Group: [`ObjectIdentifier`], [`MatrixMode`]
pub const GL_TEXTURE: GLenum = GLenum(0x1702);

///
/// * Group: [`TextureUnit`]
pub const GL_TEXTURE0: GLenum = GLenum(0x84C0);

///
/// * Group: [`CombinerRegisterNV`]
pub const GL_TEXTURE0_ARB: GLenum = GLenum(0x84C0);

///
/// * Group: [`TextureUnit`]
pub const GL_TEXTURE1: GLenum = GLenum(0x84C1);

///
/// * Group: [`TextureUnit`]
pub const GL_TEXTURE10: GLenum = GLenum(0x84CA);

pub const GL_TEXTURE10_ARB: GLenum = GLenum(0x84CA);

///
/// * Group: [`TextureUnit`]
pub const GL_TEXTURE11: GLenum = GLenum(0x84CB);

pub const GL_TEXTURE11_ARB: GLenum = GLenum(0x84CB);

///
/// * Group: [`TextureUnit`]
pub const GL_TEXTURE12: GLenum = GLenum(0x84CC);

pub const GL_TEXTURE12_ARB: GLenum = GLenum(0x84CC);

///
/// * Group: [`TextureUnit`]
pub const GL_TEXTURE13: GLenum = GLenum(0x84CD);

pub const GL_TEXTURE13_ARB: GLenum = GLenum(0x84CD);

///
/// * Group: [`TextureUnit`]
pub const GL_TEXTURE14: GLenum = GLenum(0x84CE);

pub const GL_TEXTURE14_ARB: GLenum = GLenum(0x84CE);

///
/// * Group: [`TextureUnit`]
pub const GL_TEXTURE15: GLenum = GLenum(0x84CF);

pub const GL_TEXTURE15_ARB: GLenum = GLenum(0x84CF);

///
/// * Group: [`TextureUnit`]
pub const GL_TEXTURE16: GLenum = GLenum(0x84D0);

pub const GL_TEXTURE16_ARB: GLenum = GLenum(0x84D0);

///
/// * Group: [`TextureUnit`]
pub const GL_TEXTURE17: GLenum = GLenum(0x84D1);

pub const GL_TEXTURE17_ARB: GLenum = GLenum(0x84D1);

///
/// * Group: [`TextureUnit`]
pub const GL_TEXTURE18: GLenum = GLenum(0x84D2);

pub const GL_TEXTURE18_ARB: GLenum = GLenum(0x84D2);

///
/// * Group: [`TextureUnit`]
pub const GL_TEXTURE19: GLenum = GLenum(0x84D3);

pub const GL_TEXTURE19_ARB: GLenum = GLenum(0x84D3);

///
/// * Group: [`CombinerRegisterNV`]
pub const GL_TEXTURE1_ARB: GLenum = GLenum(0x84C1);

///
/// * Group: [`TextureUnit`]
pub const GL_TEXTURE2: GLenum = GLenum(0x84C2);

///
/// * Group: [`TextureUnit`]
pub const GL_TEXTURE20: GLenum = GLenum(0x84D4);

pub const GL_TEXTURE20_ARB: GLenum = GLenum(0x84D4);

///
/// * Group: [`TextureUnit`]
pub const GL_TEXTURE21: GLenum = GLenum(0x84D5);

pub const GL_TEXTURE21_ARB: GLenum = GLenum(0x84D5);

///
/// * Group: [`TextureUnit`]
pub const GL_TEXTURE22: GLenum = GLenum(0x84D6);

pub const GL_TEXTURE22_ARB: GLenum = GLenum(0x84D6);

///
/// * Group: [`TextureUnit`]
pub const GL_TEXTURE23: GLenum = GLenum(0x84D7);

pub const GL_TEXTURE23_ARB: GLenum = GLenum(0x84D7);

///
/// * Group: [`TextureUnit`]
pub const GL_TEXTURE24: GLenum = GLenum(0x84D8);

pub const GL_TEXTURE24_ARB: GLenum = GLenum(0x84D8);

///
/// * Group: [`TextureUnit`]
pub const GL_TEXTURE25: GLenum = GLenum(0x84D9);

pub const GL_TEXTURE25_ARB: GLenum = GLenum(0x84D9);

///
/// * Group: [`TextureUnit`]
pub const GL_TEXTURE26: GLenum = GLenum(0x84DA);

pub const GL_TEXTURE26_ARB: GLenum = GLenum(0x84DA);

///
/// * Group: [`TextureUnit`]
pub const GL_TEXTURE27: GLenum = GLenum(0x84DB);

pub const GL_TEXTURE27_ARB: GLenum = GLenum(0x84DB);

///
/// * Group: [`TextureUnit`]
pub const GL_TEXTURE28: GLenum = GLenum(0x84DC);

pub const GL_TEXTURE28_ARB: GLenum = GLenum(0x84DC);

///
/// * Group: [`TextureUnit`]
pub const GL_TEXTURE29: GLenum = GLenum(0x84DD);

pub const GL_TEXTURE29_ARB: GLenum = GLenum(0x84DD);

pub const GL_TEXTURE2_ARB: GLenum = GLenum(0x84C2);

///
/// * Group: [`TextureUnit`]
pub const GL_TEXTURE3: GLenum = GLenum(0x84C3);

///
/// * Group: [`TextureUnit`]
pub const GL_TEXTURE30: GLenum = GLenum(0x84DE);

pub const GL_TEXTURE30_ARB: GLenum = GLenum(0x84DE);

///
/// * Group: [`TextureUnit`]
pub const GL_TEXTURE31: GLenum = GLenum(0x84DF);

pub const GL_TEXTURE31_ARB: GLenum = GLenum(0x84DF);

pub const GL_TEXTURE3_ARB: GLenum = GLenum(0x84C3);

///
/// * Group: [`TextureUnit`]
pub const GL_TEXTURE4: GLenum = GLenum(0x84C4);

pub const GL_TEXTURE4_ARB: GLenum = GLenum(0x84C4);

///
/// * Group: [`TextureUnit`]
pub const GL_TEXTURE5: GLenum = GLenum(0x84C5);

pub const GL_TEXTURE5_ARB: GLenum = GLenum(0x84C5);

///
/// * Group: [`TextureUnit`]
pub const GL_TEXTURE6: GLenum = GLenum(0x84C6);

pub const GL_TEXTURE6_ARB: GLenum = GLenum(0x84C6);

///
/// * Group: [`TextureUnit`]
pub const GL_TEXTURE7: GLenum = GLenum(0x84C7);

pub const GL_TEXTURE7_ARB: GLenum = GLenum(0x84C7);

///
/// * Group: [`TextureUnit`]
pub const GL_TEXTURE8: GLenum = GLenum(0x84C8);

pub const GL_TEXTURE8_ARB: GLenum = GLenum(0x84C8);

///
/// * Group: [`TextureUnit`]
pub const GL_TEXTURE9: GLenum = GLenum(0x84C9);

pub const GL_TEXTURE9_ARB: GLenum = GLenum(0x84C9);

///
/// * Group: [`CopyImageSubDataTarget`], [`EnableCap`], [`GetPName`],
///   [`TextureTarget`]
pub const GL_TEXTURE_1D: GLenum = GLenum(0x0DE0);

///
/// * Group: [`CopyImageSubDataTarget`], [`TextureTarget`]
pub const GL_TEXTURE_1D_ARRAY: GLenum = GLenum(0x8C18);

pub const GL_TEXTURE_1D_ARRAY_EXT: GLenum = GLenum(0x8C18);

pub const GL_TEXTURE_1D_BINDING_EXT: GLenum = GLenum(0x8068);

pub const GL_TEXTURE_1D_STACK_BINDING_MESAX: GLenum = GLenum(0x875D);

pub const GL_TEXTURE_1D_STACK_MESAX: GLenum = GLenum(0x8759);

///
/// * Group: [`CopyImageSubDataTarget`], [`EnableCap`], [`GetPName`],
///   [`TextureTarget`]
pub const GL_TEXTURE_2D: GLenum = GLenum(0x0DE1);

///
/// * Group: [`CopyImageSubDataTarget`], [`TextureTarget`]
pub const GL_TEXTURE_2D_ARRAY: GLenum = GLenum(0x8C1A);

pub const GL_TEXTURE_2D_ARRAY_EXT: GLenum = GLenum(0x8C1A);

pub const GL_TEXTURE_2D_BINDING_EXT: GLenum = GLenum(0x8069);

///
/// * Group: [`CopyImageSubDataTarget`], [`TextureTarget`]
pub const GL_TEXTURE_2D_MULTISAMPLE: GLenum = GLenum(0x9100);

///
/// * Group: [`CopyImageSubDataTarget`], [`TextureTarget`]
pub const GL_TEXTURE_2D_MULTISAMPLE_ARRAY: GLenum = GLenum(0x9102);

pub const GL_TEXTURE_2D_MULTISAMPLE_ARRAY_OES: GLenum = GLenum(0x9102);

pub const GL_TEXTURE_2D_STACK_BINDING_MESAX: GLenum = GLenum(0x875E);

pub const GL_TEXTURE_2D_STACK_MESAX: GLenum = GLenum(0x875A);

///
/// * Group: [`CopyImageSubDataTarget`], [`TextureTarget`]
pub const GL_TEXTURE_3D: GLenum = GLenum(0x806F);

///
/// * Group: [`GetPName`]
pub const GL_TEXTURE_3D_BINDING_EXT: GLenum = GLenum(0x806A);

pub const GL_TEXTURE_3D_BINDING_OES: GLenum = GLenum(0x806A);

///
/// * Group: [`TextureTarget`], [`EnableCap`], [`GetPName`]
pub const GL_TEXTURE_3D_EXT: GLenum = GLenum(0x806F);

///
/// * Group: [`TextureTarget`]
pub const GL_TEXTURE_3D_OES: GLenum = GLenum(0x806F);

///
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_TEXTURE_4DSIZE_SGIS: GLenum = GLenum(0x8136);

///
/// * Group: [`GetPName`]
pub const GL_TEXTURE_4D_BINDING_SGIS: GLenum = GLenum(0x814F);

///
/// * Group: [`TextureTarget`], [`EnableCap`], [`GetPName`]
pub const GL_TEXTURE_4D_SGIS: GLenum = GLenum(0x8134);

pub const GL_TEXTURE_ALPHA_MODULATE_IMG: GLenum = GLenum(0x8C06);

///
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_TEXTURE_ALPHA_SIZE: GLenum = GLenum(0x805F);

pub const GL_TEXTURE_ALPHA_SIZE_EXT: GLenum = GLenum(0x805F);

pub const GL_TEXTURE_ALPHA_TYPE: GLenum = GLenum(0x8C13);

pub const GL_TEXTURE_ALPHA_TYPE_ARB: GLenum = GLenum(0x8C13);

pub const GL_TEXTURE_APPLICATION_MODE_EXT: GLenum = GLenum(0x834F);

pub const GL_TEXTURE_ASTC_DECODE_PRECISION_EXT: GLenum = GLenum(0x8F69);

///
/// * Group: [`TextureParameterName`]
pub const GL_TEXTURE_BASE_LEVEL: GLenum = GLenum(0x813C);

///
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_TEXTURE_BASE_LEVEL_SGIS: GLenum = GLenum(0x813C);

///
/// * Group: [`GetPName`]
pub const GL_TEXTURE_BINDING_1D: GLenum = GLenum(0x8068);

///
/// * Group: [`GetPName`]
pub const GL_TEXTURE_BINDING_1D_ARRAY: GLenum = GLenum(0x8C1C);

pub const GL_TEXTURE_BINDING_1D_ARRAY_EXT: GLenum = GLenum(0x8C1C);

///
/// * Group: [`GetPName`]
pub const GL_TEXTURE_BINDING_2D: GLenum = GLenum(0x8069);

///
/// * Group: [`GetPName`]
pub const GL_TEXTURE_BINDING_2D_ARRAY: GLenum = GLenum(0x8C1D);

pub const GL_TEXTURE_BINDING_2D_ARRAY_EXT: GLenum = GLenum(0x8C1D);

///
/// * Group: [`GetPName`]
pub const GL_TEXTURE_BINDING_2D_MULTISAMPLE: GLenum = GLenum(0x9104);

///
/// * Group: [`GetPName`]
pub const GL_TEXTURE_BINDING_2D_MULTISAMPLE_ARRAY: GLenum = GLenum(0x9105);

pub const GL_TEXTURE_BINDING_2D_MULTISAMPLE_ARRAY_OES: GLenum = GLenum(0x9105);

///
/// * Group: [`GetPName`]
pub const GL_TEXTURE_BINDING_3D: GLenum = GLenum(0x806A);

pub const GL_TEXTURE_BINDING_3D_OES: GLenum = GLenum(0x806A);

///
/// * Group: [`GetPName`]
pub const GL_TEXTURE_BINDING_BUFFER: GLenum = GLenum(0x8C2C);

pub const GL_TEXTURE_BINDING_BUFFER_ARB: GLenum = GLenum(0x8C2C);

pub const GL_TEXTURE_BINDING_BUFFER_EXT: GLenum = GLenum(0x8C2C);

pub const GL_TEXTURE_BINDING_BUFFER_OES: GLenum = GLenum(0x8C2C);

///
/// * Group: [`GetPName`]
pub const GL_TEXTURE_BINDING_CUBE_MAP: GLenum = GLenum(0x8514);

pub const GL_TEXTURE_BINDING_CUBE_MAP_ARB: GLenum = GLenum(0x8514);

pub const GL_TEXTURE_BINDING_CUBE_MAP_ARRAY: GLenum = GLenum(0x900A);

pub const GL_TEXTURE_BINDING_CUBE_MAP_ARRAY_ARB: GLenum = GLenum(0x900A);

pub const GL_TEXTURE_BINDING_CUBE_MAP_ARRAY_EXT: GLenum = GLenum(0x900A);

pub const GL_TEXTURE_BINDING_CUBE_MAP_ARRAY_OES: GLenum = GLenum(0x900A);

pub const GL_TEXTURE_BINDING_CUBE_MAP_EXT: GLenum = GLenum(0x8514);

pub const GL_TEXTURE_BINDING_CUBE_MAP_OES: GLenum = GLenum(0x8514);

pub const GL_TEXTURE_BINDING_EXTERNAL_OES: GLenum = GLenum(0x8D67);

///
/// * Group: [`GetPName`]
pub const GL_TEXTURE_BINDING_RECTANGLE: GLenum = GLenum(0x84F6);

pub const GL_TEXTURE_BINDING_RECTANGLE_ARB: GLenum = GLenum(0x84F6);

pub const GL_TEXTURE_BINDING_RECTANGLE_NV: GLenum = GLenum(0x84F6);

pub const GL_TEXTURE_BINDING_RENDERBUFFER_NV: GLenum = GLenum(0x8E53);

///
/// * Group: [`AttribMask`]
pub const GL_TEXTURE_BIT: GLbitfield = GLbitfield(0x00040000);

///
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_TEXTURE_BLUE_SIZE: GLenum = GLenum(0x805E);

pub const GL_TEXTURE_BLUE_SIZE_EXT: GLenum = GLenum(0x805E);

pub const GL_TEXTURE_BLUE_TYPE: GLenum = GLenum(0x8C12);

pub const GL_TEXTURE_BLUE_TYPE_ARB: GLenum = GLenum(0x8C12);

///
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_TEXTURE_BORDER: GLenum = GLenum(0x1005);

///
/// * Group: [`SamplerParameterF`], [`GetTextureParameter`],
///   [`TextureParameterName`]
pub const GL_TEXTURE_BORDER_COLOR: GLenum = GLenum(0x1004);

pub const GL_TEXTURE_BORDER_COLOR_EXT: GLenum = GLenum(0x1004);

///
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_TEXTURE_BORDER_COLOR_NV: GLenum = GLenum(0x1004);

pub const GL_TEXTURE_BORDER_COLOR_OES: GLenum = GLenum(0x1004);

pub const GL_TEXTURE_BORDER_VALUES_NV: GLenum = GLenum(0x871A);

///
/// * Group: [`TextureTarget`], [`CopyBufferSubDataTarget`],
///   [`BufferTargetARB`], [`BufferStorageTarget`]
pub const GL_TEXTURE_BUFFER: GLenum = GLenum(0x8C2A);

pub const GL_TEXTURE_BUFFER_ARB: GLenum = GLenum(0x8C2A);

/// Equivalent to GL_TEXTURE_BUFFER_ARB query, but named more consistently
pub const GL_TEXTURE_BUFFER_BINDING: GLenum = GLenum(0x8C2A);

pub const GL_TEXTURE_BUFFER_BINDING_EXT: GLenum = GLenum(0x8C2A);

pub const GL_TEXTURE_BUFFER_BINDING_OES: GLenum = GLenum(0x8C2A);

pub const GL_TEXTURE_BUFFER_DATA_STORE_BINDING: GLenum = GLenum(0x8C2D);

pub const GL_TEXTURE_BUFFER_DATA_STORE_BINDING_ARB: GLenum = GLenum(0x8C2D);

pub const GL_TEXTURE_BUFFER_DATA_STORE_BINDING_EXT: GLenum = GLenum(0x8C2D);

pub const GL_TEXTURE_BUFFER_DATA_STORE_BINDING_OES: GLenum = GLenum(0x8C2D);

pub const GL_TEXTURE_BUFFER_EXT: GLenum = GLenum(0x8C2A);

pub const GL_TEXTURE_BUFFER_FORMAT_ARB: GLenum = GLenum(0x8C2E);

pub const GL_TEXTURE_BUFFER_FORMAT_EXT: GLenum = GLenum(0x8C2E);

pub const GL_TEXTURE_BUFFER_OES: GLenum = GLenum(0x8C2A);

pub const GL_TEXTURE_BUFFER_OFFSET: GLenum = GLenum(0x919D);

///
/// * Group: [`GetPName`]
pub const GL_TEXTURE_BUFFER_OFFSET_ALIGNMENT: GLenum = GLenum(0x919F);

pub const GL_TEXTURE_BUFFER_OFFSET_ALIGNMENT_EXT: GLenum = GLenum(0x919F);

pub const GL_TEXTURE_BUFFER_OFFSET_ALIGNMENT_OES: GLenum = GLenum(0x919F);

pub const GL_TEXTURE_BUFFER_OFFSET_EXT: GLenum = GLenum(0x919D);

pub const GL_TEXTURE_BUFFER_OFFSET_OES: GLenum = GLenum(0x919D);

pub const GL_TEXTURE_BUFFER_SIZE: GLenum = GLenum(0x919E);

pub const GL_TEXTURE_BUFFER_SIZE_EXT: GLenum = GLenum(0x919E);

pub const GL_TEXTURE_BUFFER_SIZE_OES: GLenum = GLenum(0x919E);

///
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_TEXTURE_CLIPMAP_CENTER_SGIX: GLenum = GLenum(0x8171);

///
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_TEXTURE_CLIPMAP_DEPTH_SGIX: GLenum = GLenum(0x8176);

///
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_TEXTURE_CLIPMAP_FRAME_SGIX: GLenum = GLenum(0x8172);

///
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_TEXTURE_CLIPMAP_LOD_OFFSET_SGIX: GLenum = GLenum(0x8175);

///
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_TEXTURE_CLIPMAP_OFFSET_SGIX: GLenum = GLenum(0x8173);

///
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_TEXTURE_CLIPMAP_VIRTUAL_DEPTH_SGIX: GLenum = GLenum(0x8174);

pub const GL_TEXTURE_COLOR_SAMPLES_NV: GLenum = GLenum(0x9046);

///
/// * Group: [`GetPName`], [`ColorTableTargetSGI`], [`EnableCap`]
pub const GL_TEXTURE_COLOR_TABLE_SGI: GLenum = GLenum(0x80BC);

pub const GL_TEXTURE_COLOR_WRITEMASK_SGIS: GLenum = GLenum(0x81EF);

pub const GL_TEXTURE_COMPARE_FAIL_VALUE_ARB: GLenum = GLenum(0x80BF);

///
/// * Group: [`SamplerParameterI`], [`TextureParameterName`]
pub const GL_TEXTURE_COMPARE_FUNC: GLenum = GLenum(0x884D);

pub const GL_TEXTURE_COMPARE_FUNC_ARB: GLenum = GLenum(0x884D);

pub const GL_TEXTURE_COMPARE_FUNC_EXT: GLenum = GLenum(0x884D);

///
/// * Group: [`SamplerParameterI`], [`TextureParameterName`]
pub const GL_TEXTURE_COMPARE_MODE: GLenum = GLenum(0x884C);

pub const GL_TEXTURE_COMPARE_MODE_ARB: GLenum = GLenum(0x884C);

pub const GL_TEXTURE_COMPARE_MODE_EXT: GLenum = GLenum(0x884C);

///
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_TEXTURE_COMPARE_OPERATOR_SGIX: GLenum = GLenum(0x819B);

///
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_TEXTURE_COMPARE_SGIX: GLenum = GLenum(0x819A);

///
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_TEXTURE_COMPONENTS: GLenum = GLenum(0x1003);

///
/// * Group: [`InternalFormatPName`]
pub const GL_TEXTURE_COMPRESSED: GLenum = GLenum(0x86A1);

pub const GL_TEXTURE_COMPRESSED_ARB: GLenum = GLenum(0x86A1);

///
/// * Group: [`InternalFormatPName`]
pub const GL_TEXTURE_COMPRESSED_BLOCK_HEIGHT: GLenum = GLenum(0x82B2);

///
/// * Group: [`InternalFormatPName`]
pub const GL_TEXTURE_COMPRESSED_BLOCK_SIZE: GLenum = GLenum(0x82B3);

///
/// * Group: [`InternalFormatPName`]
pub const GL_TEXTURE_COMPRESSED_BLOCK_WIDTH: GLenum = GLenum(0x82B1);

pub const GL_TEXTURE_COMPRESSED_IMAGE_SIZE: GLenum = GLenum(0x86A0);

pub const GL_TEXTURE_COMPRESSED_IMAGE_SIZE_ARB: GLenum = GLenum(0x86A0);

///
/// * Group: [`HintTarget`], [`GetPName`]
pub const GL_TEXTURE_COMPRESSION_HINT: GLenum = GLenum(0x84EF);

///
/// * Group: [`HintTarget`]
pub const GL_TEXTURE_COMPRESSION_HINT_ARB: GLenum = GLenum(0x84EF);

pub const GL_TEXTURE_CONSTANT_DATA_SUNX: GLenum = GLenum(0x81D6);

///
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_TEXTURE_COORD_ARRAY: GLenum = GLenum(0x8078);

pub const GL_TEXTURE_COORD_ARRAY_ADDRESS_NV: GLenum = GLenum(0x8F25);

pub const GL_TEXTURE_COORD_ARRAY_BUFFER_BINDING: GLenum = GLenum(0x889A);

pub const GL_TEXTURE_COORD_ARRAY_BUFFER_BINDING_ARB: GLenum = GLenum(0x889A);

///
/// * Group: [`GetPName`]
pub const GL_TEXTURE_COORD_ARRAY_COUNT_EXT: GLenum = GLenum(0x808B);

pub const GL_TEXTURE_COORD_ARRAY_EXT: GLenum = GLenum(0x8078);

pub const GL_TEXTURE_COORD_ARRAY_LENGTH_NV: GLenum = GLenum(0x8F2F);

pub const GL_TEXTURE_COORD_ARRAY_LIST_IBM: GLenum = GLenum(103074);

pub const GL_TEXTURE_COORD_ARRAY_LIST_STRIDE_IBM: GLenum = GLenum(103084);

pub const GL_TEXTURE_COORD_ARRAY_PARALLEL_POINTERS_INTEL: GLenum = GLenum(0x83F8);

///
/// * Group: [`GetPointervPName`]
pub const GL_TEXTURE_COORD_ARRAY_POINTER: GLenum = GLenum(0x8092);

///
/// * Group: [`GetPointervPName`]
pub const GL_TEXTURE_COORD_ARRAY_POINTER_EXT: GLenum = GLenum(0x8092);

///
/// * Group: [`GetPName`]
pub const GL_TEXTURE_COORD_ARRAY_SIZE: GLenum = GLenum(0x8088);

pub const GL_TEXTURE_COORD_ARRAY_SIZE_EXT: GLenum = GLenum(0x8088);

///
/// * Group: [`GetPName`]
pub const GL_TEXTURE_COORD_ARRAY_STRIDE: GLenum = GLenum(0x808A);

pub const GL_TEXTURE_COORD_ARRAY_STRIDE_EXT: GLenum = GLenum(0x808A);

///
/// * Group: [`GetPName`]
pub const GL_TEXTURE_COORD_ARRAY_TYPE: GLenum = GLenum(0x8089);

pub const GL_TEXTURE_COORD_ARRAY_TYPE_EXT: GLenum = GLenum(0x8089);

pub const GL_TEXTURE_COORD_NV: GLenum = GLenum(0x8C79);

pub const GL_TEXTURE_COVERAGE_SAMPLES_NV: GLenum = GLenum(0x9045);

pub const GL_TEXTURE_CROP_RECT_OES: GLenum = GLenum(0x8B9D);

///
/// * Group: [`CopyImageSubDataTarget`], [`TextureTarget`]
pub const GL_TEXTURE_CUBE_MAP: GLenum = GLenum(0x8513);

pub const GL_TEXTURE_CUBE_MAP_ARB: GLenum = GLenum(0x8513);

///
/// * Group: [`CopyImageSubDataTarget`], [`TextureTarget`]
pub const GL_TEXTURE_CUBE_MAP_ARRAY: GLenum = GLenum(0x9009);

///
/// * Group: [`TextureTarget`]
pub const GL_TEXTURE_CUBE_MAP_ARRAY_ARB: GLenum = GLenum(0x9009);

///
/// * Group: [`TextureTarget`]
pub const GL_TEXTURE_CUBE_MAP_ARRAY_EXT: GLenum = GLenum(0x9009);

///
/// * Group: [`TextureTarget`]
pub const GL_TEXTURE_CUBE_MAP_ARRAY_OES: GLenum = GLenum(0x9009);

pub const GL_TEXTURE_CUBE_MAP_EXT: GLenum = GLenum(0x8513);

///
/// * Group: [`TextureTarget`]
pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_X: GLenum = GLenum(0x8516);

pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_X_ARB: GLenum = GLenum(0x8516);

pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_X_EXT: GLenum = GLenum(0x8516);

pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_X_OES: GLenum = GLenum(0x8516);

///
/// * Group: [`TextureTarget`]
pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_Y: GLenum = GLenum(0x8518);

pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_Y_ARB: GLenum = GLenum(0x8518);

pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_Y_EXT: GLenum = GLenum(0x8518);

pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_Y_OES: GLenum = GLenum(0x8518);

///
/// * Group: [`TextureTarget`]
pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_Z: GLenum = GLenum(0x851A);

pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_Z_ARB: GLenum = GLenum(0x851A);

pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_Z_EXT: GLenum = GLenum(0x851A);

pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_Z_OES: GLenum = GLenum(0x851A);

pub const GL_TEXTURE_CUBE_MAP_OES: GLenum = GLenum(0x8513);

///
/// * Group: [`TextureTarget`]
pub const GL_TEXTURE_CUBE_MAP_POSITIVE_X: GLenum = GLenum(0x8515);

pub const GL_TEXTURE_CUBE_MAP_POSITIVE_X_ARB: GLenum = GLenum(0x8515);

pub const GL_TEXTURE_CUBE_MAP_POSITIVE_X_EXT: GLenum = GLenum(0x8515);

pub const GL_TEXTURE_CUBE_MAP_POSITIVE_X_OES: GLenum = GLenum(0x8515);

///
/// * Group: [`TextureTarget`]
pub const GL_TEXTURE_CUBE_MAP_POSITIVE_Y: GLenum = GLenum(0x8517);

pub const GL_TEXTURE_CUBE_MAP_POSITIVE_Y_ARB: GLenum = GLenum(0x8517);

pub const GL_TEXTURE_CUBE_MAP_POSITIVE_Y_EXT: GLenum = GLenum(0x8517);

pub const GL_TEXTURE_CUBE_MAP_POSITIVE_Y_OES: GLenum = GLenum(0x8517);

///
/// * Group: [`TextureTarget`]
pub const GL_TEXTURE_CUBE_MAP_POSITIVE_Z: GLenum = GLenum(0x8519);

pub const GL_TEXTURE_CUBE_MAP_POSITIVE_Z_ARB: GLenum = GLenum(0x8519);

pub const GL_TEXTURE_CUBE_MAP_POSITIVE_Z_EXT: GLenum = GLenum(0x8519);

pub const GL_TEXTURE_CUBE_MAP_POSITIVE_Z_OES: GLenum = GLenum(0x8519);

///
/// * Group: [`EnableCap`]
pub const GL_TEXTURE_CUBE_MAP_SEAMLESS: GLenum = GLenum(0x884F);

///
/// * Group: [`FfdMaskSGIX`]
pub const GL_TEXTURE_DEFORMATION_BIT_SGIX: GLbitfield = GLbitfield(0x00000001);

///
/// * Group: [`MapTarget`], [`FfdTargetSGIX`]
pub const GL_TEXTURE_DEFORMATION_SGIX: GLenum = GLenum(0x8195);

pub const GL_TEXTURE_DEPTH: GLenum = GLenum(0x8071);

///
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_TEXTURE_DEPTH_EXT: GLenum = GLenum(0x8071);

pub const GL_TEXTURE_DEPTH_QCOM: GLenum = GLenum(0x8BD4);

pub const GL_TEXTURE_DEPTH_SIZE: GLenum = GLenum(0x884A);

pub const GL_TEXTURE_DEPTH_SIZE_ARB: GLenum = GLenum(0x884A);

pub const GL_TEXTURE_DEPTH_TYPE: GLenum = GLenum(0x8C16);

pub const GL_TEXTURE_DEPTH_TYPE_ARB: GLenum = GLenum(0x8C16);

pub const GL_TEXTURE_DS_SIZE_NV: GLenum = GLenum(0x871D);

pub const GL_TEXTURE_DT_SIZE_NV: GLenum = GLenum(0x871E);

///
/// * Group: [`TextureEnvTarget`]
pub const GL_TEXTURE_ENV: GLenum = GLenum(0x2300);

///
/// * Group: [`TextureEnvMode`]
pub const GL_TEXTURE_ENV_BIAS_SGIX: GLenum = GLenum(0x80BE);

///
/// * Group: [`TextureEnvParameter`]
pub const GL_TEXTURE_ENV_COLOR: GLenum = GLenum(0x2201);

///
/// * Group: [`TextureEnvParameter`]
pub const GL_TEXTURE_ENV_MODE: GLenum = GLenum(0x2200);

pub const GL_TEXTURE_EXTERNAL_OES: GLenum = GLenum(0x8D65);

///
/// * Group: [`MemoryBarrierMask`]
pub const GL_TEXTURE_FETCH_BARRIER_BIT: GLbitfield = GLbitfield(0x00000008);

///
/// * Group: [`MemoryBarrierMask`]
pub const GL_TEXTURE_FETCH_BARRIER_BIT_EXT: GLbitfield = GLbitfield(0x00000008);

///
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_TEXTURE_FILTER4_SIZE_SGIS: GLenum = GLenum(0x8147);

pub const GL_TEXTURE_FILTER_CONTROL: GLenum = GLenum(0x8500);

pub const GL_TEXTURE_FILTER_CONTROL_EXT: GLenum = GLenum(0x8500);

pub const GL_TEXTURE_FIXED_SAMPLE_LOCATIONS: GLenum = GLenum(0x9107);

pub const GL_TEXTURE_FLOAT_COMPONENTS_NV: GLenum = GLenum(0x888C);

pub const GL_TEXTURE_FORMAT_QCOM: GLenum = GLenum(0x8BD6);

pub const GL_TEXTURE_FORMAT_SRGB_OVERRIDE_EXT: GLenum = GLenum(0x8FBF);

///
/// * Group: [`TextureParameterName`]
pub const GL_TEXTURE_FOVEATED_CUTOFF_DENSITY_QCOM: GLenum = GLenum(0x96A0);

pub const GL_TEXTURE_FOVEATED_FEATURE_BITS_QCOM: GLenum = GLenum(0x8BFB);

pub const GL_TEXTURE_FOVEATED_FEATURE_QUERY_QCOM: GLenum = GLenum(0x8BFD);

pub const GL_TEXTURE_FOVEATED_MIN_PIXEL_DENSITY_QCOM: GLenum = GLenum(0x8BFC);

pub const GL_TEXTURE_FOVEATED_NUM_FOCAL_POINTS_QUERY_QCOM: GLenum = GLenum(0x8BFE);

pub const GL_TEXTURE_FREE_MEMORY_ATI: GLenum = GLenum(0x87FC);

///
/// * Group: [`InternalFormatPName`]
pub const GL_TEXTURE_GATHER: GLenum = GLenum(0x82A2);

///
/// * Group: [`InternalFormatPName`]
pub const GL_TEXTURE_GATHER_SHADOW: GLenum = GLenum(0x82A3);

///
/// * Group: [`TextureGenParameter`]
pub const GL_TEXTURE_GEN_MODE: GLenum = GLenum(0x2500);

pub const GL_TEXTURE_GEN_MODE_OES: GLenum = GLenum(0x2500);

///
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_TEXTURE_GEN_Q: GLenum = GLenum(0x0C63);

///
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_TEXTURE_GEN_R: GLenum = GLenum(0x0C62);

///
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_TEXTURE_GEN_S: GLenum = GLenum(0x0C60);

pub const GL_TEXTURE_GEN_STR_OES: GLenum = GLenum(0x8D60);

///
/// * Group: [`GetPName`], [`EnableCap`]
pub const GL_TEXTURE_GEN_T: GLenum = GLenum(0x0C61);

///
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_TEXTURE_GEQUAL_R_SGIX: GLenum = GLenum(0x819D);

///
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_TEXTURE_GREEN_SIZE: GLenum = GLenum(0x805D);

pub const GL_TEXTURE_GREEN_SIZE_EXT: GLenum = GLenum(0x805D);

pub const GL_TEXTURE_GREEN_TYPE: GLenum = GLenum(0x8C11);

pub const GL_TEXTURE_GREEN_TYPE_ARB: GLenum = GLenum(0x8C11);

///
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_TEXTURE_HEIGHT: GLenum = GLenum(0x1001);

pub const GL_TEXTURE_HEIGHT_QCOM: GLenum = GLenum(0x8BD3);

pub const GL_TEXTURE_HI_SIZE_NV: GLenum = GLenum(0x871B);

///
/// * Group: [`InternalFormatPName`]
pub const GL_TEXTURE_IMAGE_FORMAT: GLenum = GLenum(0x828F);

///
/// * Group: [`InternalFormatPName`]
pub const GL_TEXTURE_IMAGE_TYPE: GLenum = GLenum(0x8290);

pub const GL_TEXTURE_IMAGE_VALID_QCOM: GLenum = GLenum(0x8BD8);

pub const GL_TEXTURE_IMMUTABLE_FORMAT: GLenum = GLenum(0x912F);

pub const GL_TEXTURE_IMMUTABLE_FORMAT_EXT: GLenum = GLenum(0x912F);

pub const GL_TEXTURE_IMMUTABLE_LEVELS: GLenum = GLenum(0x82DF);

pub const GL_TEXTURE_INDEX_SIZE_EXT: GLenum = GLenum(0x80ED);

///
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_TEXTURE_INTENSITY_SIZE: GLenum = GLenum(0x8061);

pub const GL_TEXTURE_INTENSITY_SIZE_EXT: GLenum = GLenum(0x8061);

pub const GL_TEXTURE_INTENSITY_TYPE: GLenum = GLenum(0x8C15);

pub const GL_TEXTURE_INTENSITY_TYPE_ARB: GLenum = GLenum(0x8C15);

///
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_TEXTURE_INTERNAL_FORMAT: GLenum = GLenum(0x1003);

pub const GL_TEXTURE_INTERNAL_FORMAT_QCOM: GLenum = GLenum(0x8BD5);

///
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_TEXTURE_LEQUAL_R_SGIX: GLenum = GLenum(0x819C);

pub const GL_TEXTURE_LIGHTING_MODE_HP: GLenum = GLenum(0x8167);

pub const GL_TEXTURE_LIGHT_EXT: GLenum = GLenum(0x8350);

///
/// * Group: [`TextureParameterName`], [`SamplerParameterF`]
pub const GL_TEXTURE_LOD_BIAS: GLenum = GLenum(0x8501);

pub const GL_TEXTURE_LOD_BIAS_EXT: GLenum = GLenum(0x8501);

///
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_TEXTURE_LOD_BIAS_R_SGIX: GLenum = GLenum(0x8190);

///
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_TEXTURE_LOD_BIAS_S_SGIX: GLenum = GLenum(0x818E);

///
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_TEXTURE_LOD_BIAS_T_SGIX: GLenum = GLenum(0x818F);

pub const GL_TEXTURE_LO_SIZE_NV: GLenum = GLenum(0x871C);

///
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_TEXTURE_LUMINANCE_SIZE: GLenum = GLenum(0x8060);

pub const GL_TEXTURE_LUMINANCE_SIZE_EXT: GLenum = GLenum(0x8060);

pub const GL_TEXTURE_LUMINANCE_TYPE: GLenum = GLenum(0x8C14);

pub const GL_TEXTURE_LUMINANCE_TYPE_ARB: GLenum = GLenum(0x8C14);

///
/// * Group: [`SamplerParameterI`], [`GetTextureParameter`],
///   [`TextureParameterName`]
pub const GL_TEXTURE_MAG_FILTER: GLenum = GLenum(0x2800);

pub const GL_TEXTURE_MAG_SIZE_NV: GLenum = GLenum(0x871F);

pub const GL_TEXTURE_MATERIAL_FACE_EXT: GLenum = GLenum(0x8351);

pub const GL_TEXTURE_MATERIAL_PARAMETER_EXT: GLenum = GLenum(0x8352);

///
/// * Group: [`GetPName`], [`VertexShaderTextureUnitParameter`]
pub const GL_TEXTURE_MATRIX: GLenum = GLenum(0x0BA8);

pub const GL_TEXTURE_MATRIX_FLOAT_AS_INT_BITS_OES: GLenum = GLenum(0x898F);

///
/// * Group: [`SamplerParameterF`]
pub const GL_TEXTURE_MAX_ANISOTROPY: GLenum = GLenum(0x84FE);

///
/// * Alias Of: [`GL_TEXTURE_MAX_ANISOTROPY`]
pub const GL_TEXTURE_MAX_ANISOTROPY_EXT: GLenum = GL_TEXTURE_MAX_ANISOTROPY;

///
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_TEXTURE_MAX_CLAMP_R_SGIX: GLenum = GLenum(0x836B);

///
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_TEXTURE_MAX_CLAMP_S_SGIX: GLenum = GLenum(0x8369);

///
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_TEXTURE_MAX_CLAMP_T_SGIX: GLenum = GLenum(0x836A);

///
/// * Group: [`TextureParameterName`]
pub const GL_TEXTURE_MAX_LEVEL: GLenum = GLenum(0x813D);

pub const GL_TEXTURE_MAX_LEVEL_APPLE: GLenum = GLenum(0x813D);

///
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_TEXTURE_MAX_LEVEL_SGIS: GLenum = GLenum(0x813D);

///
/// * Group: [`SamplerParameterF`], [`TextureParameterName`]
pub const GL_TEXTURE_MAX_LOD: GLenum = GLenum(0x813B);

///
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_TEXTURE_MAX_LOD_SGIS: GLenum = GLenum(0x813B);

pub const GL_TEXTURE_MEMORY_LAYOUT_INTEL: GLenum = GLenum(0x83FF);

///
/// * Group: [`SamplerParameterI`], [`GetTextureParameter`],
///   [`TextureParameterName`]
pub const GL_TEXTURE_MIN_FILTER: GLenum = GLenum(0x2801);

///
/// * Group: [`SamplerParameterF`], [`TextureParameterName`]
pub const GL_TEXTURE_MIN_LOD: GLenum = GLenum(0x813A);

///
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_TEXTURE_MIN_LOD_SGIS: GLenum = GLenum(0x813A);

///
/// * Group: [`HintTarget`]
pub const GL_TEXTURE_MULTI_BUFFER_HINT_SGIX: GLenum = GLenum(0x812E);

pub const GL_TEXTURE_NORMAL_EXT: GLenum = GLenum(0x85AF);

pub const GL_TEXTURE_NUM_LEVELS_QCOM: GLenum = GLenum(0x8BD9);

pub const GL_TEXTURE_OBJECT_VALID_QCOM: GLenum = GLenum(0x8BDB);

pub const GL_TEXTURE_POST_SPECULAR_HP: GLenum = GLenum(0x8168);

pub const GL_TEXTURE_PRE_SPECULAR_HP: GLenum = GLenum(0x8169);

///
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_TEXTURE_PRIORITY: GLenum = GLenum(0x8066);

///
/// * Group: [`TextureParameterName`]
pub const GL_TEXTURE_PRIORITY_EXT: GLenum = GLenum(0x8066);

pub const GL_TEXTURE_PROTECTED_EXT: GLenum = GLenum(0x8BFA);

pub const GL_TEXTURE_RANGE_LENGTH_APPLE: GLenum = GLenum(0x85B7);

pub const GL_TEXTURE_RANGE_POINTER_APPLE: GLenum = GLenum(0x85B8);

///
/// * Group: [`CopyImageSubDataTarget`], [`TextureTarget`]
pub const GL_TEXTURE_RECTANGLE: GLenum = GLenum(0x84F5);

pub const GL_TEXTURE_RECTANGLE_ARB: GLenum = GLenum(0x84F5);

pub const GL_TEXTURE_RECTANGLE_NV: GLenum = GLenum(0x84F5);

pub const GL_TEXTURE_REDUCTION_MODE_ARB: GLenum = GLenum(0x9366);

///
/// * Alias Of: [`GL_TEXTURE_REDUCTION_MODE_ARB`]
pub const GL_TEXTURE_REDUCTION_MODE_EXT: GLenum = GL_TEXTURE_REDUCTION_MODE_ARB;

///
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_TEXTURE_RED_SIZE: GLenum = GLenum(0x805C);

pub const GL_TEXTURE_RED_SIZE_EXT: GLenum = GLenum(0x805C);

pub const GL_TEXTURE_RED_TYPE: GLenum = GLenum(0x8C10);

pub const GL_TEXTURE_RED_TYPE_ARB: GLenum = GLenum(0x8C10);

pub const GL_TEXTURE_RENDERBUFFER_DATA_STORE_BINDING_NV: GLenum = GLenum(0x8E54);

pub const GL_TEXTURE_RENDERBUFFER_NV: GLenum = GLenum(0x8E55);

///
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_TEXTURE_RESIDENT: GLenum = GLenum(0x8067);

pub const GL_TEXTURE_RESIDENT_EXT: GLenum = GLenum(0x8067);

pub const GL_TEXTURE_SAMPLES: GLenum = GLenum(0x9106);

pub const GL_TEXTURE_SAMPLES_IMG: GLenum = GLenum(0x9136);

pub const GL_TEXTURE_SHADER_NV: GLenum = GLenum(0x86DE);

///
/// * Group: [`InternalFormatPName`]
pub const GL_TEXTURE_SHADOW: GLenum = GLenum(0x82A1);

pub const GL_TEXTURE_SHARED_SIZE: GLenum = GLenum(0x8C3F);

pub const GL_TEXTURE_SHARED_SIZE_EXT: GLenum = GLenum(0x8C3F);

pub const GL_TEXTURE_SPARSE_ARB: GLenum = GLenum(0x91A6);

pub const GL_TEXTURE_SPARSE_EXT: GLenum = GLenum(0x91A6);

pub const GL_TEXTURE_SRGB_DECODE_EXT: GLenum = GLenum(0x8A48);

///
/// * Group: [`GetPName`]
pub const GL_TEXTURE_STACK_DEPTH: GLenum = GLenum(0x0BA5);

pub const GL_TEXTURE_STENCIL_SIZE: GLenum = GLenum(0x88F1);

pub const GL_TEXTURE_STENCIL_SIZE_EXT: GLenum = GLenum(0x88F1);

///
/// * Group: [`HintTarget`]
pub const GL_TEXTURE_STORAGE_HINT_APPLE: GLenum = GLenum(0x85BC);

///
/// * Group: [`TextureStorageMaskAMD`]
pub const GL_TEXTURE_STORAGE_SPARSE_BIT_AMD: GLbitfield = GLbitfield(0x00000001);

///
/// * Group: [`TextureParameterName`]
pub const GL_TEXTURE_SWIZZLE_A: GLenum = GLenum(0x8E45);

pub const GL_TEXTURE_SWIZZLE_A_EXT: GLenum = GLenum(0x8E45);

///
/// * Group: [`TextureParameterName`]
pub const GL_TEXTURE_SWIZZLE_B: GLenum = GLenum(0x8E44);

pub const GL_TEXTURE_SWIZZLE_B_EXT: GLenum = GLenum(0x8E44);

///
/// * Group: [`TextureParameterName`]
pub const GL_TEXTURE_SWIZZLE_G: GLenum = GLenum(0x8E43);

pub const GL_TEXTURE_SWIZZLE_G_EXT: GLenum = GLenum(0x8E43);

///
/// * Group: [`TextureParameterName`]
pub const GL_TEXTURE_SWIZZLE_R: GLenum = GLenum(0x8E42);

///
/// * Group: [`TextureParameterName`]
pub const GL_TEXTURE_SWIZZLE_RGBA: GLenum = GLenum(0x8E46);

pub const GL_TEXTURE_SWIZZLE_RGBA_EXT: GLenum = GLenum(0x8E46);

pub const GL_TEXTURE_SWIZZLE_R_EXT: GLenum = GLenum(0x8E42);

pub const GL_TEXTURE_TARGET: GLenum = GLenum(0x1006);

pub const GL_TEXTURE_TARGET_QCOM: GLenum = GLenum(0x8BDA);

///
/// * Group: [`TextureParameterName`]
pub const GL_TEXTURE_TILING_EXT: GLenum = GLenum(0x9580);

///
/// * Group: [`ErrorCode`]
pub const GL_TEXTURE_TOO_LARGE_EXT: GLenum = GLenum(0x8065);

pub const GL_TEXTURE_TYPE_QCOM: GLenum = GLenum(0x8BD7);

///
/// * Group: [`SamplerParameterF`], [`SamplerParameterI`],
///   [`GetTextureParameter`], [`TextureParameterName`]
pub const GL_TEXTURE_UNNORMALIZED_COORDINATES_ARM: GLenum = GLenum(0x8F6A);

pub const GL_TEXTURE_UNSIGNED_REMAP_MODE_NV: GLenum = GLenum(0x888F);

///
/// * Group: [`MemoryBarrierMask`]
pub const GL_TEXTURE_UPDATE_BARRIER_BIT: GLbitfield = GLbitfield(0x00000100);

///
/// * Group: [`MemoryBarrierMask`]
pub const GL_TEXTURE_UPDATE_BARRIER_BIT_EXT: GLbitfield = GLbitfield(0x00000100);

pub const GL_TEXTURE_USAGE_ANGLE: GLenum = GLenum(0x93A2);

///
/// * Group: [`InternalFormatPName`]
pub const GL_TEXTURE_VIEW: GLenum = GLenum(0x82B5);

pub const GL_TEXTURE_VIEW_MIN_LAYER: GLenum = GLenum(0x82DD);

pub const GL_TEXTURE_VIEW_MIN_LAYER_EXT: GLenum = GLenum(0x82DD);

pub const GL_TEXTURE_VIEW_MIN_LAYER_OES: GLenum = GLenum(0x82DD);

pub const GL_TEXTURE_VIEW_MIN_LEVEL: GLenum = GLenum(0x82DB);

pub const GL_TEXTURE_VIEW_MIN_LEVEL_EXT: GLenum = GLenum(0x82DB);

pub const GL_TEXTURE_VIEW_MIN_LEVEL_OES: GLenum = GLenum(0x82DB);

pub const GL_TEXTURE_VIEW_NUM_LAYERS: GLenum = GLenum(0x82DE);

pub const GL_TEXTURE_VIEW_NUM_LAYERS_EXT: GLenum = GLenum(0x82DE);

pub const GL_TEXTURE_VIEW_NUM_LAYERS_OES: GLenum = GLenum(0x82DE);

pub const GL_TEXTURE_VIEW_NUM_LEVELS: GLenum = GLenum(0x82DC);

pub const GL_TEXTURE_VIEW_NUM_LEVELS_EXT: GLenum = GLenum(0x82DC);

pub const GL_TEXTURE_VIEW_NUM_LEVELS_OES: GLenum = GLenum(0x82DC);

///
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_TEXTURE_WIDTH: GLenum = GLenum(0x1000);

pub const GL_TEXTURE_WIDTH_QCOM: GLenum = GLenum(0x8BD2);

///
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_TEXTURE_WRAP_Q_SGIS: GLenum = GLenum(0x8137);

///
/// * Group: [`SamplerParameterI`], [`TextureParameterName`]
pub const GL_TEXTURE_WRAP_R: GLenum = GLenum(0x8072);

///
/// * Group: [`TextureParameterName`], [`GetTextureParameter`]
pub const GL_TEXTURE_WRAP_R_EXT: GLenum = GLenum(0x8072);

///
/// * Group: [`TextureParameterName`]
pub const GL_TEXTURE_WRAP_R_OES: GLenum = GLenum(0x8072);

///
/// * Group: [`SamplerParameterI`], [`GetTextureParameter`],
///   [`TextureParameterName`]
pub const GL_TEXTURE_WRAP_S: GLenum = GLenum(0x2802);

///
/// * Group: [`SamplerParameterI`], [`GetTextureParameter`],
///   [`TextureParameterName`]
pub const GL_TEXTURE_WRAP_T: GLenum = GLenum(0x2803);

///
/// * Group: [`ProgramTarget`]
pub const GL_TEXT_FRAGMENT_SHADER_ATI: GLenum = GLenum(0x8200);

pub const GL_TILE_RASTER_ORDER_FIXED_MESA: GLenum = GLenum(0x8BB8);

pub const GL_TILE_RASTER_ORDER_INCREASING_X_MESA: GLenum = GLenum(0x8BB9);

pub const GL_TILE_RASTER_ORDER_INCREASING_Y_MESA: GLenum = GLenum(0x8BBA);

pub const GL_TILING_TYPES_EXT: GLenum = GLenum(0x9583);

///
/// * Group: [`SemaphoreParameterName`]
pub const GL_TIMELINE_SEMAPHORE_VALUE_NV: GLenum = GLenum(0x9595);

///
/// * Group: [`SyncStatus`]
pub const GL_TIMEOUT_EXPIRED: GLenum = GLenum(0x911B);

pub const GL_TIMEOUT_EXPIRED_APPLE: GLenum = GLenum(0x911B);

/// Tagged as uint64
pub const GL_TIMEOUT_IGNORED: u64 = 0xFFFFFFFFFFFFFFFF;

/// Tagged as uint64
pub const GL_TIMEOUT_IGNORED_APPLE: u64 = 0xFFFFFFFFFFFFFFFF;

///
/// * Group: [`QueryCounterTarget`], [`GetPName`]
pub const GL_TIMESTAMP: GLenum = GLenum(0x8E28);

pub const GL_TIMESTAMP_EXT: GLenum = GLenum(0x8E28);

///
/// * Group: [`QueryTarget`]
pub const GL_TIME_ELAPSED: GLenum = GLenum(0x88BF);

pub const GL_TIME_ELAPSED_EXT: GLenum = GLenum(0x88BF);

///
/// * Group: [`ProgramResourceProperty`]
pub const GL_TOP_LEVEL_ARRAY_SIZE: GLenum = GLenum(0x930C);

///
/// * Group: [`ProgramResourceProperty`]
pub const GL_TOP_LEVEL_ARRAY_STRIDE: GLenum = GLenum(0x930D);

///
/// * Group: [`TraceMaskMESA`]
pub const GL_TRACE_ALL_BITS_MESA: GLbitfield = GLbitfield(0xFFFF);

///
/// * Group: [`TraceMaskMESA`]
pub const GL_TRACE_ARRAYS_BIT_MESA: GLbitfield = GLbitfield(0x0004);

///
/// * Group: [`TraceMaskMESA`]
pub const GL_TRACE_ERRORS_BIT_MESA: GLbitfield = GLbitfield(0x0020);

pub const GL_TRACE_MASK_MESA: GLenum = GLenum(0x8755);

pub const GL_TRACE_NAME_MESA: GLenum = GLenum(0x8756);

///
/// * Group: [`TraceMaskMESA`]
pub const GL_TRACE_OPERATIONS_BIT_MESA: GLbitfield = GLbitfield(0x0001);

///
/// * Group: [`TraceMaskMESA`]
pub const GL_TRACE_PIXELS_BIT_MESA: GLbitfield = GLbitfield(0x0010);

///
/// * Group: [`TraceMaskMESA`]
pub const GL_TRACE_PRIMITIVES_BIT_MESA: GLbitfield = GLbitfield(0x0002);

///
/// * Group: [`TraceMaskMESA`]
pub const GL_TRACE_TEXTURES_BIT_MESA: GLbitfield = GLbitfield(0x0008);

pub const GL_TRACK_MATRIX_NV: GLenum = GLenum(0x8648);

pub const GL_TRACK_MATRIX_TRANSFORM_NV: GLenum = GLenum(0x8649);

///
/// * Group: [`AttribMask`]
pub const GL_TRANSFORM_BIT: GLbitfield = GLbitfield(0x00001000);

///
/// * Group: [`ObjectIdentifier`], [`BindTransformFeedbackTarget`]
pub const GL_TRANSFORM_FEEDBACK: GLenum = GLenum(0x8E22);

///
/// * Group: [`TransformFeedbackPName`]
/// * Alias Of: [`GL_TRANSFORM_FEEDBACK_BUFFER_ACTIVE`]
pub const GL_TRANSFORM_FEEDBACK_ACTIVE: GLenum = GL_TRANSFORM_FEEDBACK_BUFFER_ACTIVE;

pub const GL_TRANSFORM_FEEDBACK_ATTRIBS_NV: GLenum = GLenum(0x8C7E);

///
/// * Group: [`MemoryBarrierMask`]
pub const GL_TRANSFORM_FEEDBACK_BARRIER_BIT: GLbitfield = GLbitfield(0x00000800);

///
/// * Group: [`MemoryBarrierMask`]
pub const GL_TRANSFORM_FEEDBACK_BARRIER_BIT_EXT: GLbitfield = GLbitfield(0x00000800);

pub const GL_TRANSFORM_FEEDBACK_BINDING: GLenum = GLenum(0x8E25);

pub const GL_TRANSFORM_FEEDBACK_BINDING_NV: GLenum = GLenum(0x8E25);

///
/// * Group: [`ProgramInterface`], [`BufferTargetARB`], [`BufferStorageTarget`],
///   [`CopyBufferSubDataTarget`]
pub const GL_TRANSFORM_FEEDBACK_BUFFER: GLenum = GLenum(0x8C8E);

pub const GL_TRANSFORM_FEEDBACK_BUFFER_ACTIVE: GLenum = GLenum(0x8E24);

pub const GL_TRANSFORM_FEEDBACK_BUFFER_ACTIVE_NV: GLenum = GLenum(0x8E24);

///
/// * Group: [`TransformFeedbackPName`], [`GetPName`]
pub const GL_TRANSFORM_FEEDBACK_BUFFER_BINDING: GLenum = GLenum(0x8C8F);

pub const GL_TRANSFORM_FEEDBACK_BUFFER_BINDING_EXT: GLenum = GLenum(0x8C8F);

pub const GL_TRANSFORM_FEEDBACK_BUFFER_BINDING_NV: GLenum = GLenum(0x8C8F);

pub const GL_TRANSFORM_FEEDBACK_BUFFER_EXT: GLenum = GLenum(0x8C8E);

///
/// * Group: [`ProgramResourceProperty`]
pub const GL_TRANSFORM_FEEDBACK_BUFFER_INDEX: GLenum = GLenum(0x934B);

///
/// * Group: [`ProgramPropertyARB`]
pub const GL_TRANSFORM_FEEDBACK_BUFFER_MODE: GLenum = GLenum(0x8C7F);

pub const GL_TRANSFORM_FEEDBACK_BUFFER_MODE_EXT: GLenum = GLenum(0x8C7F);

pub const GL_TRANSFORM_FEEDBACK_BUFFER_MODE_NV: GLenum = GLenum(0x8C7F);

pub const GL_TRANSFORM_FEEDBACK_BUFFER_NV: GLenum = GLenum(0x8C8E);

pub const GL_TRANSFORM_FEEDBACK_BUFFER_PAUSED: GLenum = GLenum(0x8E23);

pub const GL_TRANSFORM_FEEDBACK_BUFFER_PAUSED_NV: GLenum = GLenum(0x8E23);

///
/// * Group: [`TransformFeedbackPName`], [`GetPName`]
pub const GL_TRANSFORM_FEEDBACK_BUFFER_SIZE: GLenum = GLenum(0x8C85);

pub const GL_TRANSFORM_FEEDBACK_BUFFER_SIZE_EXT: GLenum = GLenum(0x8C85);

pub const GL_TRANSFORM_FEEDBACK_BUFFER_SIZE_NV: GLenum = GLenum(0x8C85);

///
/// * Group: [`TransformFeedbackPName`], [`GetPName`]
pub const GL_TRANSFORM_FEEDBACK_BUFFER_START: GLenum = GLenum(0x8C84);

pub const GL_TRANSFORM_FEEDBACK_BUFFER_START_EXT: GLenum = GLenum(0x8C84);

pub const GL_TRANSFORM_FEEDBACK_BUFFER_START_NV: GLenum = GLenum(0x8C84);

///
/// * Group: [`ProgramResourceProperty`]
pub const GL_TRANSFORM_FEEDBACK_BUFFER_STRIDE: GLenum = GLenum(0x934C);

pub const GL_TRANSFORM_FEEDBACK_NV: GLenum = GLenum(0x8E22);

///
/// * Group: [`QueryTarget`]
pub const GL_TRANSFORM_FEEDBACK_OVERFLOW: GLenum = GLenum(0x82EC);

///
/// * Alias Of: [`GL_TRANSFORM_FEEDBACK_OVERFLOW`]
pub const GL_TRANSFORM_FEEDBACK_OVERFLOW_ARB: GLenum = GL_TRANSFORM_FEEDBACK_OVERFLOW;

///
/// * Group: [`TransformFeedbackPName`]
/// * Alias Of: [`GL_TRANSFORM_FEEDBACK_BUFFER_PAUSED`]
pub const GL_TRANSFORM_FEEDBACK_PAUSED: GLenum = GL_TRANSFORM_FEEDBACK_BUFFER_PAUSED;

///
/// * Group: [`QueryTarget`]
pub const GL_TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN: GLenum = GLenum(0x8C88);

pub const GL_TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN_EXT: GLenum = GLenum(0x8C88);

pub const GL_TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN_NV: GLenum = GLenum(0x8C88);

pub const GL_TRANSFORM_FEEDBACK_RECORD_NV: GLenum = GLenum(0x8C86);

pub const GL_TRANSFORM_FEEDBACK_STREAM_OVERFLOW: GLenum = GLenum(0x82ED);

///
/// * Alias Of: [`GL_TRANSFORM_FEEDBACK_STREAM_OVERFLOW`]
pub const GL_TRANSFORM_FEEDBACK_STREAM_OVERFLOW_ARB: GLenum = GL_TRANSFORM_FEEDBACK_STREAM_OVERFLOW;

///
/// * Group: [`ProgramInterface`]
pub const GL_TRANSFORM_FEEDBACK_VARYING: GLenum = GLenum(0x92F4);

///
/// * Group: [`ProgramPropertyARB`]
pub const GL_TRANSFORM_FEEDBACK_VARYINGS: GLenum = GLenum(0x8C83);

pub const GL_TRANSFORM_FEEDBACK_VARYINGS_EXT: GLenum = GLenum(0x8C83);

pub const GL_TRANSFORM_FEEDBACK_VARYINGS_NV: GLenum = GLenum(0x8C83);

///
/// * Group: [`ProgramPropertyARB`]
pub const GL_TRANSFORM_FEEDBACK_VARYING_MAX_LENGTH: GLenum = GLenum(0x8C76);

pub const GL_TRANSFORM_FEEDBACK_VARYING_MAX_LENGTH_EXT: GLenum = GLenum(0x8C76);

///
/// * Group: [`HintTarget`]
pub const GL_TRANSFORM_HINT_APPLE: GLenum = GLenum(0x85B1);

pub const GL_TRANSLATED_SHADER_SOURCE_LENGTH_ANGLE: GLenum = GLenum(0x93A0);

///
/// * Group: [`PathTransformType`]
pub const GL_TRANSLATE_2D_NV: GLenum = GLenum(0x9090);

///
/// * Group: [`PathTransformType`]
pub const GL_TRANSLATE_3D_NV: GLenum = GLenum(0x9091);

///
/// * Group: [`PathTransformType`]
pub const GL_TRANSLATE_X_NV: GLenum = GLenum(0x908E);

///
/// * Group: [`PathTransformType`]
pub const GL_TRANSLATE_Y_NV: GLenum = GLenum(0x908F);

///
/// * Group: [`PathTransformType`]
pub const GL_TRANSPOSE_AFFINE_2D_NV: GLenum = GLenum(0x9096);

///
/// * Group: [`PathTransformType`]
pub const GL_TRANSPOSE_AFFINE_3D_NV: GLenum = GLenum(0x9098);

pub const GL_TRANSPOSE_COLOR_MATRIX: GLenum = GLenum(0x84E6);

pub const GL_TRANSPOSE_COLOR_MATRIX_ARB: GLenum = GLenum(0x84E6);

pub const GL_TRANSPOSE_CURRENT_MATRIX_ARB: GLenum = GLenum(0x88B7);

pub const GL_TRANSPOSE_MODELVIEW_MATRIX: GLenum = GLenum(0x84E3);

pub const GL_TRANSPOSE_MODELVIEW_MATRIX_ARB: GLenum = GLenum(0x84E3);

pub const GL_TRANSPOSE_NV: GLenum = GLenum(0x862C);

pub const GL_TRANSPOSE_PROGRAM_MATRIX_EXT: GLenum = GLenum(0x8E2E);

pub const GL_TRANSPOSE_PROJECTION_MATRIX: GLenum = GLenum(0x84E4);

pub const GL_TRANSPOSE_PROJECTION_MATRIX_ARB: GLenum = GLenum(0x84E4);

pub const GL_TRANSPOSE_TEXTURE_MATRIX: GLenum = GLenum(0x84E5);

pub const GL_TRANSPOSE_TEXTURE_MATRIX_ARB: GLenum = GLenum(0x84E5);

///
/// * Group: [`PrimitiveType`]
pub const GL_TRIANGLES: GLenum = GLenum(0x0004);

///
/// * Group: [`PrimitiveType`]
pub const GL_TRIANGLES_ADJACENCY: GLenum = GLenum(0x000C);

///
/// * Group: [`PrimitiveType`]
pub const GL_TRIANGLES_ADJACENCY_ARB: GLenum = GLenum(0x000C);

///
/// * Group: [`PrimitiveType`]
pub const GL_TRIANGLES_ADJACENCY_EXT: GLenum = GLenum(0x000C);

pub const GL_TRIANGLES_ADJACENCY_OES: GLenum = GLenum(0x000C);

///
/// * Group: [`PrimitiveType`]
pub const GL_TRIANGLE_FAN: GLenum = GLenum(0x0006);

pub const GL_TRIANGLE_LIST_SUN: GLenum = GLenum(0x81D7);

pub const GL_TRIANGLE_MESH_SUN: GLenum = GLenum(0x8615);

///
/// * Group: [`PrimitiveType`]
pub const GL_TRIANGLE_STRIP: GLenum = GLenum(0x0005);

///
/// * Group: [`PrimitiveType`]
pub const GL_TRIANGLE_STRIP_ADJACENCY: GLenum = GLenum(0x000D);

///
/// * Group: [`PrimitiveType`]
pub const GL_TRIANGLE_STRIP_ADJACENCY_ARB: GLenum = GLenum(0x000D);

///
/// * Group: [`PrimitiveType`]
pub const GL_TRIANGLE_STRIP_ADJACENCY_EXT: GLenum = GLenum(0x000D);

pub const GL_TRIANGLE_STRIP_ADJACENCY_OES: GLenum = GLenum(0x000D);

pub const GL_TRIANGULAR_NV: GLenum = GLenum(0x90A5);

///
/// * Group: [`Boolean`], [`VertexShaderWriteMaskEXT`], [`ClampColorModeARB`]
pub const GL_TRUE: GLenum = GLenum(1);

///
/// * Group: [`ProgramResourceProperty`]
pub const GL_TYPE: GLenum = GLenum(0x92FA);

pub const GL_UNCORRELATED_NV: GLenum = GLenum(0x9282);

pub const GL_UNDEFINED_APPLE: GLenum = GLenum(0x8A1C);

pub const GL_UNDEFINED_VERTEX: GLenum = GLenum(0x8260);

pub const GL_UNDEFINED_VERTEX_EXT: GLenum = GLenum(0x8260);

pub const GL_UNDEFINED_VERTEX_OES: GLenum = GLenum(0x8260);

///
/// * Group: [`ProgramResourceProperty`], [`ProgramInterface`]
pub const GL_UNIFORM: GLenum = GLenum(0x92E1);

///
/// * Group: [`CommandOpcodesNV`]
pub const GL_UNIFORM_ADDRESS_COMMAND_NV: GLenum = GLenum(0x000A);

///
/// * Group: [`UniformPName`]
pub const GL_UNIFORM_ARRAY_STRIDE: GLenum = GLenum(0x8A3C);

///
/// * Group: [`UniformPName`]
pub const GL_UNIFORM_ATOMIC_COUNTER_BUFFER_INDEX: GLenum = GLenum(0x92DA);

///
/// * Group: [`MemoryBarrierMask`]
pub const GL_UNIFORM_BARRIER_BIT: GLbitfield = GLbitfield(0x00000004);

///
/// * Group: [`MemoryBarrierMask`]
pub const GL_UNIFORM_BARRIER_BIT_EXT: GLbitfield = GLbitfield(0x00000004);

///
/// * Group: [`ProgramInterface`]
pub const GL_UNIFORM_BLOCK: GLenum = GLenum(0x92E2);

///
/// * Group: [`UniformBlockPName`]
pub const GL_UNIFORM_BLOCK_ACTIVE_UNIFORMS: GLenum = GLenum(0x8A42);

///
/// * Group: [`UniformBlockPName`]
pub const GL_UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES: GLenum = GLenum(0x8A43);

///
/// * Group: [`UniformBlockPName`]
pub const GL_UNIFORM_BLOCK_BINDING: GLenum = GLenum(0x8A3F);

///
/// * Group: [`UniformBlockPName`]
pub const GL_UNIFORM_BLOCK_DATA_SIZE: GLenum = GLenum(0x8A40);

///
/// * Group: [`UniformPName`]
pub const GL_UNIFORM_BLOCK_INDEX: GLenum = GLenum(0x8A3A);

///
/// * Group: [`UniformBlockPName`]
pub const GL_UNIFORM_BLOCK_NAME_LENGTH: GLenum = GLenum(0x8A41);

///
/// * Group: [`UniformBlockPName`]
pub const GL_UNIFORM_BLOCK_REFERENCED_BY_COMPUTE_SHADER: GLenum = GLenum(0x90EC);

///
/// * Group: [`UniformBlockPName`]
pub const GL_UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER: GLenum = GLenum(0x8A46);

///
/// * Group: [`UniformBlockPName`]
pub const GL_UNIFORM_BLOCK_REFERENCED_BY_GEOMETRY_SHADER: GLenum = GLenum(0x8A45);

pub const GL_UNIFORM_BLOCK_REFERENCED_BY_MESH_SHADER_NV: GLenum = GLenum(0x959C);

pub const GL_UNIFORM_BLOCK_REFERENCED_BY_TASK_SHADER_NV: GLenum = GLenum(0x959D);

///
/// * Group: [`UniformBlockPName`]
pub const GL_UNIFORM_BLOCK_REFERENCED_BY_TESS_CONTROL_SHADER: GLenum = GLenum(0x84F0);

///
/// * Group: [`UniformBlockPName`]
pub const GL_UNIFORM_BLOCK_REFERENCED_BY_TESS_EVALUATION_SHADER: GLenum = GLenum(0x84F1);

///
/// * Group: [`UniformBlockPName`]
pub const GL_UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER: GLenum = GLenum(0x8A44);

///
/// * Group: [`CopyBufferSubDataTarget`], [`BufferTargetARB`],
///   [`BufferStorageTarget`]
pub const GL_UNIFORM_BUFFER: GLenum = GLenum(0x8A11);

pub const GL_UNIFORM_BUFFER_ADDRESS_NV: GLenum = GLenum(0x936F);

///
/// * Group: [`GetPName`]
pub const GL_UNIFORM_BUFFER_BINDING: GLenum = GLenum(0x8A28);

pub const GL_UNIFORM_BUFFER_BINDING_EXT: GLenum = GLenum(0x8DEF);

pub const GL_UNIFORM_BUFFER_EXT: GLenum = GLenum(0x8DEE);

pub const GL_UNIFORM_BUFFER_LENGTH_NV: GLenum = GLenum(0x9370);

///
/// * Group: [`GetPName`]
pub const GL_UNIFORM_BUFFER_OFFSET_ALIGNMENT: GLenum = GLenum(0x8A34);

///
/// * Group: [`GetPName`]
pub const GL_UNIFORM_BUFFER_SIZE: GLenum = GLenum(0x8A2A);

///
/// * Group: [`GetPName`]
pub const GL_UNIFORM_BUFFER_START: GLenum = GLenum(0x8A29);

pub const GL_UNIFORM_BUFFER_UNIFIED_NV: GLenum = GLenum(0x936E);

///
/// * Group: [`UniformPName`]
pub const GL_UNIFORM_IS_ROW_MAJOR: GLenum = GLenum(0x8A3E);

///
/// * Group: [`UniformPName`]
pub const GL_UNIFORM_MATRIX_STRIDE: GLenum = GLenum(0x8A3D);

///
/// * Group: [`SubroutineParameterName`], [`UniformPName`]
pub const GL_UNIFORM_NAME_LENGTH: GLenum = GLenum(0x8A39);

///
/// * Group: [`UniformPName`]
pub const GL_UNIFORM_OFFSET: GLenum = GLenum(0x8A3B);

///
/// * Group: [`SubroutineParameterName`], [`UniformPName`]
pub const GL_UNIFORM_SIZE: GLenum = GLenum(0x8A38);

///
/// * Group: [`UniformPName`]
pub const GL_UNIFORM_TYPE: GLenum = GLenum(0x8A37);

///
/// * Group: [`GraphicsResetStatus`]
pub const GL_UNKNOWN_CONTEXT_RESET: GLenum = GLenum(0x8255);

pub const GL_UNKNOWN_CONTEXT_RESET_ARB: GLenum = GLenum(0x8255);

pub const GL_UNKNOWN_CONTEXT_RESET_EXT: GLenum = GLenum(0x8255);

pub const GL_UNKNOWN_CONTEXT_RESET_KHR: GLenum = GLenum(0x8255);

///
/// * Group: [`PixelStoreParameter`], [`GetPName`]
pub const GL_UNPACK_ALIGNMENT: GLenum = GLenum(0x0CF5);

pub const GL_UNPACK_CLIENT_STORAGE_APPLE: GLenum = GLenum(0x85B2);

///
/// * Group: [`HintTarget`], [`GetPName`]
pub const GL_UNPACK_CMYK_HINT_EXT: GLenum = GLenum(0x800F);

pub const GL_UNPACK_COLORSPACE_CONVERSION_WEBGL: GLenum = GLenum(0x9243);

pub const GL_UNPACK_COMPRESSED_BLOCK_DEPTH: GLenum = GLenum(0x9129);

pub const GL_UNPACK_COMPRESSED_BLOCK_HEIGHT: GLenum = GLenum(0x9128);

pub const GL_UNPACK_COMPRESSED_BLOCK_SIZE: GLenum = GLenum(0x912A);

pub const GL_UNPACK_COMPRESSED_BLOCK_WIDTH: GLenum = GLenum(0x9127);

pub const GL_UNPACK_COMPRESSED_SIZE_SGIX: GLenum = GLenum(0x831A);

pub const GL_UNPACK_CONSTANT_DATA_SUNX: GLenum = GLenum(0x81D5);

pub const GL_UNPACK_FLIP_Y_WEBGL: GLenum = GLenum(0x9240);

///
/// * Group: [`PixelStoreParameter`], [`GetPName`]
pub const GL_UNPACK_IMAGE_DEPTH_SGIS: GLenum = GLenum(0x8133);

///
/// * Group: [`PixelStoreParameter`], [`GetPName`]
pub const GL_UNPACK_IMAGE_HEIGHT: GLenum = GLenum(0x806E);

///
/// * Group: [`PixelStoreParameter`], [`GetPName`]
pub const GL_UNPACK_IMAGE_HEIGHT_EXT: GLenum = GLenum(0x806E);

///
/// * Group: [`PixelStoreParameter`], [`GetPName`]
pub const GL_UNPACK_LSB_FIRST: GLenum = GLenum(0x0CF1);

pub const GL_UNPACK_PREMULTIPLY_ALPHA_WEBGL: GLenum = GLenum(0x9241);

///
/// * Group: [`PixelStoreParameter`]
pub const GL_UNPACK_RESAMPLE_OML: GLenum = GLenum(0x8985);

/// Formerly 0x842D in SGI specfile
/// * Group: [`PixelStoreParameter`], [`GetPName`]
pub const GL_UNPACK_RESAMPLE_SGIX: GLenum = GLenum(0x842F);

pub const GL_UNPACK_ROW_BYTES_APPLE: GLenum = GLenum(0x8A16);

///
/// * Group: [`PixelStoreParameter`], [`GetPName`]
pub const GL_UNPACK_ROW_LENGTH: GLenum = GLenum(0x0CF2);

///
/// * Group: [`PixelStoreParameter`]
pub const GL_UNPACK_ROW_LENGTH_EXT: GLenum = GLenum(0x0CF2);

///
/// * Group: [`PixelStoreParameter`], [`GetPName`]
pub const GL_UNPACK_SKIP_IMAGES: GLenum = GLenum(0x806D);

///
/// * Group: [`PixelStoreParameter`], [`GetPName`]
pub const GL_UNPACK_SKIP_IMAGES_EXT: GLenum = GLenum(0x806D);

///
/// * Group: [`PixelStoreParameter`], [`GetPName`]
pub const GL_UNPACK_SKIP_PIXELS: GLenum = GLenum(0x0CF4);

///
/// * Group: [`PixelStoreParameter`]
pub const GL_UNPACK_SKIP_PIXELS_EXT: GLenum = GLenum(0x0CF4);

///
/// * Group: [`PixelStoreParameter`], [`GetPName`]
pub const GL_UNPACK_SKIP_ROWS: GLenum = GLenum(0x0CF3);

///
/// * Group: [`PixelStoreParameter`]
pub const GL_UNPACK_SKIP_ROWS_EXT: GLenum = GLenum(0x0CF3);

///
/// * Group: [`PixelStoreParameter`], [`GetPName`]
pub const GL_UNPACK_SKIP_VOLUMES_SGIS: GLenum = GLenum(0x8132);

///
/// * Group: [`PixelStoreParameter`], [`GetPName`]
pub const GL_UNPACK_SUBSAMPLE_RATE_SGIX: GLenum = GLenum(0x85A1);

///
/// * Group: [`PixelStoreParameter`], [`GetPName`]
pub const GL_UNPACK_SWAP_BYTES: GLenum = GLenum(0x0CF0);

pub const GL_UNSIGNALED: GLenum = GLenum(0x9118);

pub const GL_UNSIGNALED_APPLE: GLenum = GLenum(0x9118);

///
/// * Group: [`VertexAttribIType`], [`ScalarType`], [`ReplacementCodeTypeSUN`],
///   [`ElementPointerTypeATI`], [`MatrixIndexPointerTypeARB`],
///   [`WeightPointerTypeARB`], [`ColorPointerType`], [`DrawElementsType`],
///   [`ListNameType`], [`PixelType`], [`VertexAttribType`],
///   [`VertexAttribPointerType`]
pub const GL_UNSIGNED_BYTE: GLenum = GLenum(0x1401);

pub const GL_UNSIGNED_BYTE_2_3_3_REV: GLenum = GLenum(0x8362);

pub const GL_UNSIGNED_BYTE_2_3_3_REV_EXT: GLenum = GLenum(0x8362);

///
/// * Group: [`PixelType`]
pub const GL_UNSIGNED_BYTE_3_3_2: GLenum = GLenum(0x8032);

///
/// * Group: [`PixelType`]
pub const GL_UNSIGNED_BYTE_3_3_2_EXT: GLenum = GLenum(0x8032);

///
/// * Group: [`CombinerMappingNV`]
pub const GL_UNSIGNED_IDENTITY_NV: GLenum = GLenum(0x8536);

///
/// * Group: [`VertexAttribIType`], [`ScalarType`], [`ReplacementCodeTypeSUN`],
///   [`ElementPointerTypeATI`], [`MatrixIndexPointerTypeARB`],
///   [`WeightPointerTypeARB`], [`ColorPointerType`], [`DrawElementsType`],
///   [`ListNameType`], [`PixelFormat`], [`PixelType`], [`VertexAttribType`],
///   [`AttributeType`], [`UniformType`], [`VertexAttribPointerType`],
///   [`GlslTypeToken`]
pub const GL_UNSIGNED_INT: GLenum = GLenum(0x1405);

pub const GL_UNSIGNED_INT16_NV: GLenum = GLenum(0x8FF0);

pub const GL_UNSIGNED_INT16_VEC2_NV: GLenum = GLenum(0x8FF1);

pub const GL_UNSIGNED_INT16_VEC3_NV: GLenum = GLenum(0x8FF2);

pub const GL_UNSIGNED_INT16_VEC4_NV: GLenum = GLenum(0x8FF3);

pub const GL_UNSIGNED_INT64_AMD: GLenum = GLenum(0x8BC2);

///
/// * Group: [`VertexAttribPointerType`], [`AttributeType`]
pub const GL_UNSIGNED_INT64_ARB: GLenum = GLenum(0x140F);

///
/// * Group: [`VertexAttribPointerType`], [`AttributeType`]
pub const GL_UNSIGNED_INT64_NV: GLenum = GLenum(0x140F);

///
/// * Group: [`AttributeType`]
pub const GL_UNSIGNED_INT64_VEC2_ARB: GLenum = GLenum(0x8FF5);

pub const GL_UNSIGNED_INT64_VEC2_NV: GLenum = GLenum(0x8FF5);

///
/// * Group: [`AttributeType`]
pub const GL_UNSIGNED_INT64_VEC3_ARB: GLenum = GLenum(0x8FF6);

pub const GL_UNSIGNED_INT64_VEC3_NV: GLenum = GLenum(0x8FF6);

///
/// * Group: [`AttributeType`]
pub const GL_UNSIGNED_INT64_VEC4_ARB: GLenum = GLenum(0x8FF7);

pub const GL_UNSIGNED_INT64_VEC4_NV: GLenum = GLenum(0x8FF7);

pub const GL_UNSIGNED_INT8_NV: GLenum = GLenum(0x8FEC);

pub const GL_UNSIGNED_INT8_VEC2_NV: GLenum = GLenum(0x8FED);

pub const GL_UNSIGNED_INT8_VEC3_NV: GLenum = GLenum(0x8FEE);

pub const GL_UNSIGNED_INT8_VEC4_NV: GLenum = GLenum(0x8FEF);

///
/// * Group: [`VertexAttribPointerType`], [`VertexAttribType`]
pub const GL_UNSIGNED_INT_10F_11F_11F_REV: GLenum = GLenum(0x8C3B);

pub const GL_UNSIGNED_INT_10F_11F_11F_REV_APPLE: GLenum = GLenum(0x8C3B);

pub const GL_UNSIGNED_INT_10F_11F_11F_REV_EXT: GLenum = GLenum(0x8C3B);

///
/// * Group: [`PixelType`]
pub const GL_UNSIGNED_INT_10_10_10_2: GLenum = GLenum(0x8036);

///
/// * Group: [`PixelType`]
pub const GL_UNSIGNED_INT_10_10_10_2_EXT: GLenum = GLenum(0x8036);

pub const GL_UNSIGNED_INT_10_10_10_2_OES: GLenum = GLenum(0x8DF6);

pub const GL_UNSIGNED_INT_24_8: GLenum = GLenum(0x84FA);

pub const GL_UNSIGNED_INT_24_8_EXT: GLenum = GLenum(0x84FA);

pub const GL_UNSIGNED_INT_24_8_MESA: GLenum = GLenum(0x8751);

pub const GL_UNSIGNED_INT_24_8_NV: GLenum = GLenum(0x84FA);

pub const GL_UNSIGNED_INT_24_8_OES: GLenum = GLenum(0x84FA);

///
/// * Group: [`VertexAttribPointerType`], [`VertexAttribType`]
pub const GL_UNSIGNED_INT_2_10_10_10_REV: GLenum = GLenum(0x8368);

pub const GL_UNSIGNED_INT_2_10_10_10_REV_EXT: GLenum = GLenum(0x8368);

pub const GL_UNSIGNED_INT_5_9_9_9_REV: GLenum = GLenum(0x8C3E);

pub const GL_UNSIGNED_INT_5_9_9_9_REV_APPLE: GLenum = GLenum(0x8C3E);

pub const GL_UNSIGNED_INT_5_9_9_9_REV_EXT: GLenum = GLenum(0x8C3E);

pub const GL_UNSIGNED_INT_8_24_REV_MESA: GLenum = GLenum(0x8752);

///
/// * Group: [`PixelType`]
pub const GL_UNSIGNED_INT_8_8_8_8: GLenum = GLenum(0x8035);

///
/// * Group: [`PixelType`]
pub const GL_UNSIGNED_INT_8_8_8_8_EXT: GLenum = GLenum(0x8035);

pub const GL_UNSIGNED_INT_8_8_8_8_REV: GLenum = GLenum(0x8367);

pub const GL_UNSIGNED_INT_8_8_8_8_REV_EXT: GLenum = GLenum(0x8367);

pub const GL_UNSIGNED_INT_8_8_S8_S8_REV_NV: GLenum = GLenum(0x86DB);

///
/// * Group: [`GlslTypeToken`]
pub const GL_UNSIGNED_INT_ATOMIC_COUNTER: GLenum = GLenum(0x92DB);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`]
pub const GL_UNSIGNED_INT_IMAGE_1D: GLenum = GLenum(0x9062);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`]
pub const GL_UNSIGNED_INT_IMAGE_1D_ARRAY: GLenum = GLenum(0x9068);

pub const GL_UNSIGNED_INT_IMAGE_1D_ARRAY_EXT: GLenum = GLenum(0x9068);

pub const GL_UNSIGNED_INT_IMAGE_1D_EXT: GLenum = GLenum(0x9062);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`]
pub const GL_UNSIGNED_INT_IMAGE_2D: GLenum = GLenum(0x9063);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`]
pub const GL_UNSIGNED_INT_IMAGE_2D_ARRAY: GLenum = GLenum(0x9069);

pub const GL_UNSIGNED_INT_IMAGE_2D_ARRAY_EXT: GLenum = GLenum(0x9069);

pub const GL_UNSIGNED_INT_IMAGE_2D_EXT: GLenum = GLenum(0x9063);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`]
pub const GL_UNSIGNED_INT_IMAGE_2D_MULTISAMPLE: GLenum = GLenum(0x906B);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`]
pub const GL_UNSIGNED_INT_IMAGE_2D_MULTISAMPLE_ARRAY: GLenum = GLenum(0x906C);

pub const GL_UNSIGNED_INT_IMAGE_2D_MULTISAMPLE_ARRAY_EXT: GLenum = GLenum(0x906C);

pub const GL_UNSIGNED_INT_IMAGE_2D_MULTISAMPLE_EXT: GLenum = GLenum(0x906B);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`]
pub const GL_UNSIGNED_INT_IMAGE_2D_RECT: GLenum = GLenum(0x9065);

pub const GL_UNSIGNED_INT_IMAGE_2D_RECT_EXT: GLenum = GLenum(0x9065);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`]
pub const GL_UNSIGNED_INT_IMAGE_3D: GLenum = GLenum(0x9064);

pub const GL_UNSIGNED_INT_IMAGE_3D_EXT: GLenum = GLenum(0x9064);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`]
pub const GL_UNSIGNED_INT_IMAGE_BUFFER: GLenum = GLenum(0x9067);

pub const GL_UNSIGNED_INT_IMAGE_BUFFER_EXT: GLenum = GLenum(0x9067);

pub const GL_UNSIGNED_INT_IMAGE_BUFFER_OES: GLenum = GLenum(0x9067);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`]
pub const GL_UNSIGNED_INT_IMAGE_CUBE: GLenum = GLenum(0x9066);

pub const GL_UNSIGNED_INT_IMAGE_CUBE_EXT: GLenum = GLenum(0x9066);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`]
pub const GL_UNSIGNED_INT_IMAGE_CUBE_MAP_ARRAY: GLenum = GLenum(0x906A);

pub const GL_UNSIGNED_INT_IMAGE_CUBE_MAP_ARRAY_EXT: GLenum = GLenum(0x906A);

pub const GL_UNSIGNED_INT_IMAGE_CUBE_MAP_ARRAY_OES: GLenum = GLenum(0x906A);

pub const GL_UNSIGNED_INT_S8_S8_8_8_NV: GLenum = GLenum(0x86DA);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_UNSIGNED_INT_SAMPLER_1D: GLenum = GLenum(0x8DD1);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_UNSIGNED_INT_SAMPLER_1D_ARRAY: GLenum = GLenum(0x8DD6);

pub const GL_UNSIGNED_INT_SAMPLER_1D_ARRAY_EXT: GLenum = GLenum(0x8DD6);

pub const GL_UNSIGNED_INT_SAMPLER_1D_EXT: GLenum = GLenum(0x8DD1);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_UNSIGNED_INT_SAMPLER_2D: GLenum = GLenum(0x8DD2);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_UNSIGNED_INT_SAMPLER_2D_ARRAY: GLenum = GLenum(0x8DD7);

pub const GL_UNSIGNED_INT_SAMPLER_2D_ARRAY_EXT: GLenum = GLenum(0x8DD7);

pub const GL_UNSIGNED_INT_SAMPLER_2D_EXT: GLenum = GLenum(0x8DD2);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE: GLenum = GLenum(0x910A);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE_ARRAY: GLenum = GLenum(0x910D);

pub const GL_UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE_ARRAY_OES: GLenum = GLenum(0x910D);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_UNSIGNED_INT_SAMPLER_2D_RECT: GLenum = GLenum(0x8DD5);

pub const GL_UNSIGNED_INT_SAMPLER_2D_RECT_EXT: GLenum = GLenum(0x8DD5);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_UNSIGNED_INT_SAMPLER_3D: GLenum = GLenum(0x8DD3);

pub const GL_UNSIGNED_INT_SAMPLER_3D_EXT: GLenum = GLenum(0x8DD3);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_UNSIGNED_INT_SAMPLER_BUFFER: GLenum = GLenum(0x8DD8);

pub const GL_UNSIGNED_INT_SAMPLER_BUFFER_AMD: GLenum = GLenum(0x9003);

pub const GL_UNSIGNED_INT_SAMPLER_BUFFER_EXT: GLenum = GLenum(0x8DD8);

pub const GL_UNSIGNED_INT_SAMPLER_BUFFER_OES: GLenum = GLenum(0x8DD8);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_UNSIGNED_INT_SAMPLER_CUBE: GLenum = GLenum(0x8DD4);

pub const GL_UNSIGNED_INT_SAMPLER_CUBE_EXT: GLenum = GLenum(0x8DD4);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_UNSIGNED_INT_SAMPLER_CUBE_MAP_ARRAY: GLenum = GLenum(0x900F);

pub const GL_UNSIGNED_INT_SAMPLER_CUBE_MAP_ARRAY_ARB: GLenum = GLenum(0x900F);

pub const GL_UNSIGNED_INT_SAMPLER_CUBE_MAP_ARRAY_EXT: GLenum = GLenum(0x900F);

pub const GL_UNSIGNED_INT_SAMPLER_CUBE_MAP_ARRAY_OES: GLenum = GLenum(0x900F);

pub const GL_UNSIGNED_INT_SAMPLER_RENDERBUFFER_NV: GLenum = GLenum(0x8E58);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_UNSIGNED_INT_VEC2: GLenum = GLenum(0x8DC6);

pub const GL_UNSIGNED_INT_VEC2_EXT: GLenum = GLenum(0x8DC6);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_UNSIGNED_INT_VEC3: GLenum = GLenum(0x8DC7);

pub const GL_UNSIGNED_INT_VEC3_EXT: GLenum = GLenum(0x8DC7);

///
/// * Group: [`GlslTypeToken`], [`AttributeType`], [`UniformType`]
pub const GL_UNSIGNED_INT_VEC4: GLenum = GLenum(0x8DC8);

pub const GL_UNSIGNED_INT_VEC4_EXT: GLenum = GLenum(0x8DC8);

///
/// * Group: [`CombinerMappingNV`]
pub const GL_UNSIGNED_INVERT_NV: GLenum = GLenum(0x8537);

pub const GL_UNSIGNED_NORMALIZED: GLenum = GLenum(0x8C17);

pub const GL_UNSIGNED_NORMALIZED_ARB: GLenum = GLenum(0x8C17);

pub const GL_UNSIGNED_NORMALIZED_EXT: GLenum = GLenum(0x8C17);

///
/// * Group: [`VertexAttribIType`], [`ScalarType`], [`ReplacementCodeTypeSUN`],
///   [`ElementPointerTypeATI`], [`MatrixIndexPointerTypeARB`],
///   [`WeightPointerTypeARB`], [`ColorPointerType`], [`DrawElementsType`],
///   [`ListNameType`], [`PixelFormat`], [`PixelType`], [`VertexAttribType`],
///   [`VertexAttribPointerType`]
pub const GL_UNSIGNED_SHORT: GLenum = GLenum(0x1403);

pub const GL_UNSIGNED_SHORT_15_1_MESA: GLenum = GLenum(0x8753);

pub const GL_UNSIGNED_SHORT_1_15_REV_MESA: GLenum = GLenum(0x8754);

pub const GL_UNSIGNED_SHORT_1_5_5_5_REV: GLenum = GLenum(0x8366);

pub const GL_UNSIGNED_SHORT_1_5_5_5_REV_EXT: GLenum = GLenum(0x8366);

///
/// * Group: [`PixelType`]
pub const GL_UNSIGNED_SHORT_4_4_4_4: GLenum = GLenum(0x8033);

///
/// * Group: [`PixelType`]
pub const GL_UNSIGNED_SHORT_4_4_4_4_EXT: GLenum = GLenum(0x8033);

pub const GL_UNSIGNED_SHORT_4_4_4_4_REV: GLenum = GLenum(0x8365);

pub const GL_UNSIGNED_SHORT_4_4_4_4_REV_EXT: GLenum = GLenum(0x8365);

pub const GL_UNSIGNED_SHORT_4_4_4_4_REV_IMG: GLenum = GLenum(0x8365);

///
/// * Group: [`PixelType`]
pub const GL_UNSIGNED_SHORT_5_5_5_1: GLenum = GLenum(0x8034);

///
/// * Group: [`PixelType`]
pub const GL_UNSIGNED_SHORT_5_5_5_1_EXT: GLenum = GLenum(0x8034);

pub const GL_UNSIGNED_SHORT_5_6_5: GLenum = GLenum(0x8363);

pub const GL_UNSIGNED_SHORT_5_6_5_EXT: GLenum = GLenum(0x8363);

pub const GL_UNSIGNED_SHORT_5_6_5_REV: GLenum = GLenum(0x8364);

pub const GL_UNSIGNED_SHORT_5_6_5_REV_EXT: GLenum = GLenum(0x8364);

pub const GL_UNSIGNED_SHORT_8_8_APPLE: GLenum = GLenum(0x85BA);

pub const GL_UNSIGNED_SHORT_8_8_MESA: GLenum = GLenum(0x85BA);

pub const GL_UNSIGNED_SHORT_8_8_REV_APPLE: GLenum = GLenum(0x85BB);

pub const GL_UNSIGNED_SHORT_8_8_REV_MESA: GLenum = GLenum(0x85BB);

pub const GL_UPLOAD_GPU_MASK_NVX: GLenum = GLenum(0x954A);

///
/// * Group: [`ClipControlOrigin`]
pub const GL_UPPER_LEFT: GLenum = GLenum(0x8CA2);

///
/// * Alias Of: [`GL_UPPER_LEFT`]
pub const GL_UPPER_LEFT_EXT: GLenum = GL_UPPER_LEFT;

///
/// * Group: [`PathHandleMissingGlyphs`]
pub const GL_USE_MISSING_GLYPH_NV: GLenum = GLenum(0x90AA);

///
/// * Group: [`PathElementType`]
pub const GL_UTF16_NV: GLenum = GLenum(0x909B);

///
/// * Group: [`PathElementType`]
pub const GL_UTF8_NV: GLenum = GLenum(0x909A);

pub const GL_UUID_SIZE_EXT: GLenum = GLenum(16);

///
/// * Group: [`InterleavedArrayFormat`]
pub const GL_V2F: GLenum = GLenum(0x2A20);

///
/// * Group: [`InterleavedArrayFormat`]
pub const GL_V3F: GLenum = GLenum(0x2A21);

pub const GL_VALIDATE_SHADER_BINARY_QCOM: GLenum = GLenum(0x96A3);

///
/// * Group: [`ProgramPropertyARB`]
pub const GL_VALIDATE_STATUS: GLenum = GLenum(0x8B83);

///
/// * Group: [`CombinerVariableNV`]
pub const GL_VARIABLE_A_NV: GLenum = GLenum(0x8523);

///
/// * Group: [`CombinerVariableNV`]
pub const GL_VARIABLE_B_NV: GLenum = GLenum(0x8524);

///
/// * Group: [`CombinerVariableNV`]
pub const GL_VARIABLE_C_NV: GLenum = GLenum(0x8525);

///
/// * Group: [`CombinerVariableNV`]
pub const GL_VARIABLE_D_NV: GLenum = GLenum(0x8526);

///
/// * Group: [`CombinerVariableNV`]
pub const GL_VARIABLE_E_NV: GLenum = GLenum(0x8527);

///
/// * Group: [`CombinerVariableNV`]
pub const GL_VARIABLE_F_NV: GLenum = GLenum(0x8528);

///
/// * Group: [`CombinerVariableNV`]
pub const GL_VARIABLE_G_NV: GLenum = GLenum(0x8529);

///
/// * Group: [`VariantCapEXT`]
pub const GL_VARIANT_ARRAY_EXT: GLenum = GLenum(0x87E8);

pub const GL_VARIANT_ARRAY_POINTER_EXT: GLenum = GLenum(0x87E9);

///
/// * Group: [`GetVariantValueEXT`]
pub const GL_VARIANT_ARRAY_STRIDE_EXT: GLenum = GLenum(0x87E6);

///
/// * Group: [`GetVariantValueEXT`]
pub const GL_VARIANT_ARRAY_TYPE_EXT: GLenum = GLenum(0x87E7);

///
/// * Group: [`GetVariantValueEXT`]
pub const GL_VARIANT_DATATYPE_EXT: GLenum = GLenum(0x87E5);

///
/// * Group: [`VertexShaderStorageTypeEXT`]
pub const GL_VARIANT_EXT: GLenum = GLenum(0x87C1);

///
/// * Group: [`GetVariantValueEXT`]
pub const GL_VARIANT_VALUE_EXT: GLenum = GLenum(0x87E4);

pub const GL_VBO_FREE_MEMORY_ATI: GLenum = GLenum(0x87FB);

///
/// * Group: [`DataTypeEXT`]
pub const GL_VECTOR_EXT: GLenum = GLenum(0x87BF);

///
/// * Group: [`StringName`]
pub const GL_VENDOR: GLenum = GLenum(0x1F00);

///
/// * Group: [`StringName`]
pub const GL_VERSION: GLenum = GLenum(0x1F02);

/// Not an API enum. API definition macro for ES 1.0/1.1 headers
pub const GL_VERSION_ES_CL_1_0: GLenum = GLenum(1);

/// Not an API enum. API definition macro for ES 1.0/1.1 headers
pub const GL_VERSION_ES_CL_1_1: GLenum = GLenum(1);

/// Not an API enum. API definition macro for ES 1.0/1.1 headers
pub const GL_VERSION_ES_CM_1_1: GLenum = GLenum(1);

///
/// * Group: [`VertexHintsMaskPGI`]
pub const GL_VERTEX23_BIT_PGI: GLbitfield = GLbitfield(0x00000004);

///
/// * Group: [`VertexHintsMaskPGI`]
pub const GL_VERTEX4_BIT_PGI: GLbitfield = GLbitfield(0x00000008);

///
/// * Group: [`ObjectIdentifier`], [`EnableCap`], [`GetPName`]
pub const GL_VERTEX_ARRAY: GLenum = GLenum(0x8074);

pub const GL_VERTEX_ARRAY_ADDRESS_NV: GLenum = GLenum(0x8F21);

///
/// * Group: [`GetPName`]
pub const GL_VERTEX_ARRAY_BINDING: GLenum = GLenum(0x85B5);

pub const GL_VERTEX_ARRAY_BINDING_APPLE: GLenum = GLenum(0x85B5);

pub const GL_VERTEX_ARRAY_BINDING_OES: GLenum = GLenum(0x85B5);

pub const GL_VERTEX_ARRAY_BUFFER_BINDING: GLenum = GLenum(0x8896);

pub const GL_VERTEX_ARRAY_BUFFER_BINDING_ARB: GLenum = GLenum(0x8896);

///
/// * Group: [`GetPName`]
pub const GL_VERTEX_ARRAY_COUNT_EXT: GLenum = GLenum(0x807D);

pub const GL_VERTEX_ARRAY_EXT: GLenum = GLenum(0x8074);

pub const GL_VERTEX_ARRAY_KHR: GLenum = GLenum(0x8074);

pub const GL_VERTEX_ARRAY_LENGTH_NV: GLenum = GLenum(0x8F2B);

pub const GL_VERTEX_ARRAY_LIST_IBM: GLenum = GLenum(103070);

pub const GL_VERTEX_ARRAY_LIST_STRIDE_IBM: GLenum = GLenum(103080);

pub const GL_VERTEX_ARRAY_OBJECT_AMD: GLenum = GLenum(0x9154);

pub const GL_VERTEX_ARRAY_OBJECT_EXT: GLenum = GLenum(0x9154);

pub const GL_VERTEX_ARRAY_PARALLEL_POINTERS_INTEL: GLenum = GLenum(0x83F5);

///
/// * Group: [`GetPointervPName`]
pub const GL_VERTEX_ARRAY_POINTER: GLenum = GLenum(0x808E);

///
/// * Group: [`GetPointervPName`]
pub const GL_VERTEX_ARRAY_POINTER_EXT: GLenum = GLenum(0x808E);

pub const GL_VERTEX_ARRAY_RANGE_APPLE: GLenum = GLenum(0x851D);

pub const GL_VERTEX_ARRAY_RANGE_LENGTH_APPLE: GLenum = GLenum(0x851E);

pub const GL_VERTEX_ARRAY_RANGE_LENGTH_NV: GLenum = GLenum(0x851E);

pub const GL_VERTEX_ARRAY_RANGE_NV: GLenum = GLenum(0x851D);

pub const GL_VERTEX_ARRAY_RANGE_POINTER_APPLE: GLenum = GLenum(0x8521);

pub const GL_VERTEX_ARRAY_RANGE_POINTER_NV: GLenum = GLenum(0x8521);

pub const GL_VERTEX_ARRAY_RANGE_VALID_NV: GLenum = GLenum(0x851F);

pub const GL_VERTEX_ARRAY_RANGE_WITHOUT_FLUSH_NV: GLenum = GLenum(0x8533);

///
/// * Group: [`GetPName`]
pub const GL_VERTEX_ARRAY_SIZE: GLenum = GLenum(0x807A);

pub const GL_VERTEX_ARRAY_SIZE_EXT: GLenum = GLenum(0x807A);

///
/// * Group: [`HintTarget`]
pub const GL_VERTEX_ARRAY_STORAGE_HINT_APPLE: GLenum = GLenum(0x851F);

///
/// * Group: [`GetPName`]
pub const GL_VERTEX_ARRAY_STRIDE: GLenum = GLenum(0x807C);

pub const GL_VERTEX_ARRAY_STRIDE_EXT: GLenum = GLenum(0x807C);

///
/// * Group: [`GetPName`]
pub const GL_VERTEX_ARRAY_TYPE: GLenum = GLenum(0x807B);

pub const GL_VERTEX_ARRAY_TYPE_EXT: GLenum = GLenum(0x807B);

pub const GL_VERTEX_ATTRIB_ARRAY0_NV: GLenum = GLenum(0x8650);

pub const GL_VERTEX_ATTRIB_ARRAY10_NV: GLenum = GLenum(0x865A);

pub const GL_VERTEX_ATTRIB_ARRAY11_NV: GLenum = GLenum(0x865B);

pub const GL_VERTEX_ATTRIB_ARRAY12_NV: GLenum = GLenum(0x865C);

pub const GL_VERTEX_ATTRIB_ARRAY13_NV: GLenum = GLenum(0x865D);

pub const GL_VERTEX_ATTRIB_ARRAY14_NV: GLenum = GLenum(0x865E);

pub const GL_VERTEX_ATTRIB_ARRAY15_NV: GLenum = GLenum(0x865F);

pub const GL_VERTEX_ATTRIB_ARRAY1_NV: GLenum = GLenum(0x8651);

pub const GL_VERTEX_ATTRIB_ARRAY2_NV: GLenum = GLenum(0x8652);

pub const GL_VERTEX_ATTRIB_ARRAY3_NV: GLenum = GLenum(0x8653);

pub const GL_VERTEX_ATTRIB_ARRAY4_NV: GLenum = GLenum(0x8654);

pub const GL_VERTEX_ATTRIB_ARRAY5_NV: GLenum = GLenum(0x8655);

pub const GL_VERTEX_ATTRIB_ARRAY6_NV: GLenum = GLenum(0x8656);

pub const GL_VERTEX_ATTRIB_ARRAY7_NV: GLenum = GLenum(0x8657);

pub const GL_VERTEX_ATTRIB_ARRAY8_NV: GLenum = GLenum(0x8658);

pub const GL_VERTEX_ATTRIB_ARRAY9_NV: GLenum = GLenum(0x8659);

pub const GL_VERTEX_ATTRIB_ARRAY_ADDRESS_NV: GLenum = GLenum(0x8F20);

///
/// * Group: [`MemoryBarrierMask`]
pub const GL_VERTEX_ATTRIB_ARRAY_BARRIER_BIT: GLbitfield = GLbitfield(0x00000001);

///
/// * Group: [`MemoryBarrierMask`]
pub const GL_VERTEX_ATTRIB_ARRAY_BARRIER_BIT_EXT: GLbitfield = GLbitfield(0x00000001);

///
/// * Group: [`VertexAttribEnum`], [`VertexAttribPropertyARB`]
pub const GL_VERTEX_ATTRIB_ARRAY_BUFFER_BINDING: GLenum = GLenum(0x889F);

pub const GL_VERTEX_ATTRIB_ARRAY_BUFFER_BINDING_ARB: GLenum = GLenum(0x889F);

///
/// * Group: [`VertexAttribEnum`], [`VertexAttribPropertyARB`],
///   [`VertexArrayPName`]
pub const GL_VERTEX_ATTRIB_ARRAY_DIVISOR: GLenum = GLenum(0x88FE);

pub const GL_VERTEX_ATTRIB_ARRAY_DIVISOR_ANGLE: GLenum = GLenum(0x88FE);

pub const GL_VERTEX_ATTRIB_ARRAY_DIVISOR_ARB: GLenum = GLenum(0x88FE);

pub const GL_VERTEX_ATTRIB_ARRAY_DIVISOR_EXT: GLenum = GLenum(0x88FE);

pub const GL_VERTEX_ATTRIB_ARRAY_DIVISOR_NV: GLenum = GLenum(0x88FE);

///
/// * Group: [`VertexAttribEnum`], [`VertexAttribPropertyARB`],
///   [`VertexArrayPName`]
pub const GL_VERTEX_ATTRIB_ARRAY_ENABLED: GLenum = GLenum(0x8622);

pub const GL_VERTEX_ATTRIB_ARRAY_ENABLED_ARB: GLenum = GLenum(0x8622);

///
/// * Group: [`VertexAttribEnum`], [`VertexAttribPropertyARB`],
///   [`VertexArrayPName`]
pub const GL_VERTEX_ATTRIB_ARRAY_INTEGER: GLenum = GLenum(0x88FD);

///
/// * Group: [`VertexAttribPropertyARB`]
pub const GL_VERTEX_ATTRIB_ARRAY_INTEGER_EXT: GLenum = GLenum(0x88FD);

pub const GL_VERTEX_ATTRIB_ARRAY_INTEGER_NV: GLenum = GLenum(0x88FD);

pub const GL_VERTEX_ATTRIB_ARRAY_LENGTH_NV: GLenum = GLenum(0x8F2A);

///
/// * Group: [`VertexArrayPName`], [`VertexAttribPropertyARB`]
pub const GL_VERTEX_ATTRIB_ARRAY_LONG: GLenum = GLenum(0x874E);

///
/// * Group: [`VertexAttribEnum`], [`VertexAttribPropertyARB`],
///   [`VertexArrayPName`]
pub const GL_VERTEX_ATTRIB_ARRAY_NORMALIZED: GLenum = GLenum(0x886A);

pub const GL_VERTEX_ATTRIB_ARRAY_NORMALIZED_ARB: GLenum = GLenum(0x886A);

///
/// * Group: [`VertexAttribPointerPropertyARB`]
pub const GL_VERTEX_ATTRIB_ARRAY_POINTER: GLenum = GLenum(0x8645);

///
/// * Group: [`VertexAttribPointerPropertyARB`]
pub const GL_VERTEX_ATTRIB_ARRAY_POINTER_ARB: GLenum = GLenum(0x8645);

///
/// * Group: [`VertexAttribEnum`], [`VertexAttribPropertyARB`],
///   [`VertexArrayPName`]
pub const GL_VERTEX_ATTRIB_ARRAY_SIZE: GLenum = GLenum(0x8623);

pub const GL_VERTEX_ATTRIB_ARRAY_SIZE_ARB: GLenum = GLenum(0x8623);

///
/// * Group: [`VertexAttribEnum`], [`VertexAttribPropertyARB`],
///   [`VertexArrayPName`]
pub const GL_VERTEX_ATTRIB_ARRAY_STRIDE: GLenum = GLenum(0x8624);

pub const GL_VERTEX_ATTRIB_ARRAY_STRIDE_ARB: GLenum = GLenum(0x8624);

///
/// * Group: [`VertexAttribEnum`], [`VertexAttribPropertyARB`],
///   [`VertexArrayPName`]
pub const GL_VERTEX_ATTRIB_ARRAY_TYPE: GLenum = GLenum(0x8625);

pub const GL_VERTEX_ATTRIB_ARRAY_TYPE_ARB: GLenum = GLenum(0x8625);

pub const GL_VERTEX_ATTRIB_ARRAY_UNIFIED_NV: GLenum = GLenum(0x8F1E);

///
/// * Group: [`VertexAttribPropertyARB`]
pub const GL_VERTEX_ATTRIB_BINDING: GLenum = GLenum(0x82D4);

pub const GL_VERTEX_ATTRIB_MAP1_APPLE: GLenum = GLenum(0x8A00);

pub const GL_VERTEX_ATTRIB_MAP1_COEFF_APPLE: GLenum = GLenum(0x8A03);

pub const GL_VERTEX_ATTRIB_MAP1_DOMAIN_APPLE: GLenum = GLenum(0x8A05);

pub const GL_VERTEX_ATTRIB_MAP1_ORDER_APPLE: GLenum = GLenum(0x8A04);

pub const GL_VERTEX_ATTRIB_MAP1_SIZE_APPLE: GLenum = GLenum(0x8A02);

pub const GL_VERTEX_ATTRIB_MAP2_APPLE: GLenum = GLenum(0x8A01);

pub const GL_VERTEX_ATTRIB_MAP2_COEFF_APPLE: GLenum = GLenum(0x8A07);

pub const GL_VERTEX_ATTRIB_MAP2_DOMAIN_APPLE: GLenum = GLenum(0x8A09);

pub const GL_VERTEX_ATTRIB_MAP2_ORDER_APPLE: GLenum = GLenum(0x8A08);

pub const GL_VERTEX_ATTRIB_MAP2_SIZE_APPLE: GLenum = GLenum(0x8A06);

///
/// * Group: [`VertexArrayPName`], [`VertexAttribPropertyARB`]
pub const GL_VERTEX_ATTRIB_RELATIVE_OFFSET: GLenum = GLenum(0x82D5);

pub const GL_VERTEX_BINDING_BUFFER: GLenum = GLenum(0x8F4F);

///
/// * Group: [`GetPName`]
pub const GL_VERTEX_BINDING_DIVISOR: GLenum = GLenum(0x82D6);

///
/// * Group: [`GetPName`]
pub const GL_VERTEX_BINDING_OFFSET: GLenum = GLenum(0x82D7);

///
/// * Group: [`GetPName`]
pub const GL_VERTEX_BINDING_STRIDE: GLenum = GLenum(0x82D8);

pub const GL_VERTEX_BLEND_ARB: GLenum = GLenum(0x86A7);

///
/// * Group: [`HintTarget`], [`HintTargetPGI`]
pub const GL_VERTEX_CONSISTENT_HINT_PGI: GLenum = GLenum(0x1A22B);

///
/// * Group: [`HintTarget`], [`HintTargetPGI`]
pub const GL_VERTEX_DATA_HINT_PGI: GLenum = GLenum(0x1A22A);

pub const GL_VERTEX_ELEMENT_SWIZZLE_AMD: GLenum = GLenum(0x91A4);

pub const GL_VERTEX_ID_NV: GLenum = GLenum(0x8C7B);

pub const GL_VERTEX_ID_SWIZZLE_AMD: GLenum = GLenum(0x91A5);

///
/// * Group: [`HintTarget`], [`GetPName`]
pub const GL_VERTEX_PRECLIP_HINT_SGIX: GLenum = GLenum(0x83EF);

///
/// * Group: [`HintTarget`], [`GetPName`]
pub const GL_VERTEX_PRECLIP_SGIX: GLenum = GLenum(0x83EE);

///
/// * Group: [`ProgramTarget`]
pub const GL_VERTEX_PROGRAM_ARB: GLenum = GLenum(0x8620);

pub const GL_VERTEX_PROGRAM_BINDING_NV: GLenum = GLenum(0x864A);

pub const GL_VERTEX_PROGRAM_CALLBACK_DATA_MESA: GLenum = GLenum(0x8BB7);

pub const GL_VERTEX_PROGRAM_CALLBACK_FUNC_MESA: GLenum = GLenum(0x8BB6);

pub const GL_VERTEX_PROGRAM_CALLBACK_MESA: GLenum = GLenum(0x8BB5);

pub const GL_VERTEX_PROGRAM_NV: GLenum = GLenum(0x8620);

pub const GL_VERTEX_PROGRAM_PARAMETER_BUFFER_NV: GLenum = GLenum(0x8DA2);

pub const GL_VERTEX_PROGRAM_POINT_SIZE: GLenum = GLenum(0x8642);

pub const GL_VERTEX_PROGRAM_POINT_SIZE_ARB: GLenum = GLenum(0x8642);

pub const GL_VERTEX_PROGRAM_POINT_SIZE_NV: GLenum = GLenum(0x8642);

pub const GL_VERTEX_PROGRAM_POSITION_MESA: GLenum = GLenum(0x8BB4);

pub const GL_VERTEX_PROGRAM_TWO_SIDE: GLenum = GLenum(0x8643);

pub const GL_VERTEX_PROGRAM_TWO_SIDE_ARB: GLenum = GLenum(0x8643);

pub const GL_VERTEX_PROGRAM_TWO_SIDE_NV: GLenum = GLenum(0x8643);

///
/// * Group: [`PipelineParameterName`], [`ShaderType`]
pub const GL_VERTEX_SHADER: GLenum = GLenum(0x8B31);

///
/// * Group: [`ShaderType`]
pub const GL_VERTEX_SHADER_ARB: GLenum = GLenum(0x8B31);

pub const GL_VERTEX_SHADER_BINDING_EXT: GLenum = GLenum(0x8781);

///
/// * Group: [`UseProgramStageMask`]
pub const GL_VERTEX_SHADER_BIT: GLbitfield = GLbitfield(0x00000001);

///
/// * Group: [`UseProgramStageMask`]
pub const GL_VERTEX_SHADER_BIT_EXT: GLbitfield = GLbitfield(0x00000001);

pub const GL_VERTEX_SHADER_EXT: GLenum = GLenum(0x8780);

pub const GL_VERTEX_SHADER_INSTRUCTIONS_EXT: GLenum = GLenum(0x87CF);

pub const GL_VERTEX_SHADER_INVARIANTS_EXT: GLenum = GLenum(0x87D1);

///
/// * Group: [`QueryTarget`]
pub const GL_VERTEX_SHADER_INVOCATIONS: GLenum = GLenum(0x82F0);

///
/// * Alias Of: [`GL_VERTEX_SHADER_INVOCATIONS`]
pub const GL_VERTEX_SHADER_INVOCATIONS_ARB: GLenum = GL_VERTEX_SHADER_INVOCATIONS;

pub const GL_VERTEX_SHADER_LOCALS_EXT: GLenum = GLenum(0x87D3);

pub const GL_VERTEX_SHADER_LOCAL_CONSTANTS_EXT: GLenum = GLenum(0x87D2);

pub const GL_VERTEX_SHADER_OPTIMIZED_EXT: GLenum = GLenum(0x87D4);

pub const GL_VERTEX_SHADER_VARIANTS_EXT: GLenum = GLenum(0x87D0);

pub const GL_VERTEX_SOURCE_ATI: GLenum = GLenum(0x8774);

pub const GL_VERTEX_STATE_PROGRAM_NV: GLenum = GLenum(0x8621);

///
/// * Group: [`VertexStreamATI`]
pub const GL_VERTEX_STREAM0_ATI: GLenum = GLenum(0x876C);

///
/// * Group: [`VertexStreamATI`]
pub const GL_VERTEX_STREAM1_ATI: GLenum = GLenum(0x876D);

///
/// * Group: [`VertexStreamATI`]
pub const GL_VERTEX_STREAM2_ATI: GLenum = GLenum(0x876E);

///
/// * Group: [`VertexStreamATI`]
pub const GL_VERTEX_STREAM3_ATI: GLenum = GLenum(0x876F);

///
/// * Group: [`VertexStreamATI`]
pub const GL_VERTEX_STREAM4_ATI: GLenum = GLenum(0x8770);

///
/// * Group: [`VertexStreamATI`]
pub const GL_VERTEX_STREAM5_ATI: GLenum = GLenum(0x8771);

///
/// * Group: [`VertexStreamATI`]
pub const GL_VERTEX_STREAM6_ATI: GLenum = GLenum(0x8772);

///
/// * Group: [`VertexStreamATI`]
pub const GL_VERTEX_STREAM7_ATI: GLenum = GLenum(0x8773);

///
/// * Group: [`ProgramInterface`]
pub const GL_VERTEX_SUBROUTINE: GLenum = GLenum(0x92E8);

///
/// * Group: [`ProgramInterface`]
pub const GL_VERTEX_SUBROUTINE_UNIFORM: GLenum = GLenum(0x92EE);

///
/// * Group: [`InternalFormatPName`]
pub const GL_VERTEX_TEXTURE: GLenum = GLenum(0x829B);

pub const GL_VERTEX_WEIGHTING_EXT: GLenum = GLenum(0x8509);

pub const GL_VERTEX_WEIGHT_ARRAY_EXT: GLenum = GLenum(0x850C);

pub const GL_VERTEX_WEIGHT_ARRAY_POINTER_EXT: GLenum = GLenum(0x8510);

pub const GL_VERTEX_WEIGHT_ARRAY_SIZE_EXT: GLenum = GLenum(0x850D);

pub const GL_VERTEX_WEIGHT_ARRAY_STRIDE_EXT: GLenum = GLenum(0x850F);

pub const GL_VERTEX_WEIGHT_ARRAY_TYPE_EXT: GLenum = GLenum(0x850E);

///
/// * Group: [`PathCoordType`]
pub const GL_VERTICAL_LINE_TO_NV: GLenum = GLenum(0x08);

///
/// * Group: [`QueryTarget`]
pub const GL_VERTICES_SUBMITTED: GLenum = GLenum(0x82EE);

///
/// * Alias Of: [`GL_VERTICES_SUBMITTED`]
pub const GL_VERTICES_SUBMITTED_ARB: GLenum = GL_VERTICES_SUBMITTED;

pub const GL_VIBRANCE_BIAS_NV: GLenum = GLenum(0x8719);

pub const GL_VIBRANCE_SCALE_NV: GLenum = GLenum(0x8713);

pub const GL_VIDEO_BUFFER_BINDING_NV: GLenum = GLenum(0x9021);

pub const GL_VIDEO_BUFFER_INTERNAL_FORMAT_NV: GLenum = GLenum(0x902D);

pub const GL_VIDEO_BUFFER_NV: GLenum = GLenum(0x9020);

pub const GL_VIDEO_BUFFER_PITCH_NV: GLenum = GLenum(0x9028);

pub const GL_VIDEO_CAPTURE_FIELD_LOWER_HEIGHT_NV: GLenum = GLenum(0x903B);

pub const GL_VIDEO_CAPTURE_FIELD_UPPER_HEIGHT_NV: GLenum = GLenum(0x903A);

pub const GL_VIDEO_CAPTURE_FRAME_HEIGHT_NV: GLenum = GLenum(0x9039);

pub const GL_VIDEO_CAPTURE_FRAME_WIDTH_NV: GLenum = GLenum(0x9038);

pub const GL_VIDEO_CAPTURE_SURFACE_ORIGIN_NV: GLenum = GLenum(0x903C);

pub const GL_VIDEO_CAPTURE_TO_422_SUPPORTED_NV: GLenum = GLenum(0x9026);

pub const GL_VIDEO_COLOR_CONVERSION_MATRIX_NV: GLenum = GLenum(0x9029);

pub const GL_VIDEO_COLOR_CONVERSION_MAX_NV: GLenum = GLenum(0x902A);

pub const GL_VIDEO_COLOR_CONVERSION_MIN_NV: GLenum = GLenum(0x902B);

pub const GL_VIDEO_COLOR_CONVERSION_OFFSET_NV: GLenum = GLenum(0x902C);

///
/// * Group: [`GetPName`]
pub const GL_VIEWPORT: GLenum = GLenum(0x0BA2);

///
/// * Group: [`AttribMask`]
pub const GL_VIEWPORT_BIT: GLbitfield = GLbitfield(0x00000800);

///
/// * Group: [`GetPName`]
pub const GL_VIEWPORT_BOUNDS_RANGE: GLenum = GLenum(0x825D);

pub const GL_VIEWPORT_BOUNDS_RANGE_EXT: GLenum = GLenum(0x825D);

pub const GL_VIEWPORT_BOUNDS_RANGE_NV: GLenum = GLenum(0x825D);

pub const GL_VIEWPORT_BOUNDS_RANGE_OES: GLenum = GLenum(0x825D);

///
/// * Group: [`CommandOpcodesNV`]
pub const GL_VIEWPORT_COMMAND_NV: GLenum = GLenum(0x0010);

///
/// * Group: [`GetPName`]
pub const GL_VIEWPORT_INDEX_PROVOKING_VERTEX: GLenum = GLenum(0x825F);

pub const GL_VIEWPORT_INDEX_PROVOKING_VERTEX_EXT: GLenum = GLenum(0x825F);

pub const GL_VIEWPORT_INDEX_PROVOKING_VERTEX_NV: GLenum = GLenum(0x825F);

pub const GL_VIEWPORT_INDEX_PROVOKING_VERTEX_OES: GLenum = GLenum(0x825F);

pub const GL_VIEWPORT_POSITION_W_SCALE_NV: GLenum = GLenum(0x937C);

pub const GL_VIEWPORT_POSITION_W_SCALE_X_COEFF_NV: GLenum = GLenum(0x937D);

pub const GL_VIEWPORT_POSITION_W_SCALE_Y_COEFF_NV: GLenum = GLenum(0x937E);

///
/// * Group: [`GetPName`]
pub const GL_VIEWPORT_SUBPIXEL_BITS: GLenum = GLenum(0x825C);

pub const GL_VIEWPORT_SUBPIXEL_BITS_EXT: GLenum = GLenum(0x825C);

pub const GL_VIEWPORT_SUBPIXEL_BITS_NV: GLenum = GLenum(0x825C);

pub const GL_VIEWPORT_SUBPIXEL_BITS_OES: GLenum = GLenum(0x825C);

pub const GL_VIEWPORT_SWIZZLE_NEGATIVE_W_NV: GLenum = GLenum(0x9357);

pub const GL_VIEWPORT_SWIZZLE_NEGATIVE_X_NV: GLenum = GLenum(0x9351);

pub const GL_VIEWPORT_SWIZZLE_NEGATIVE_Y_NV: GLenum = GLenum(0x9353);

pub const GL_VIEWPORT_SWIZZLE_NEGATIVE_Z_NV: GLenum = GLenum(0x9355);

pub const GL_VIEWPORT_SWIZZLE_POSITIVE_W_NV: GLenum = GLenum(0x9356);

pub const GL_VIEWPORT_SWIZZLE_POSITIVE_X_NV: GLenum = GLenum(0x9350);

pub const GL_VIEWPORT_SWIZZLE_POSITIVE_Y_NV: GLenum = GLenum(0x9352);

pub const GL_VIEWPORT_SWIZZLE_POSITIVE_Z_NV: GLenum = GLenum(0x9354);

pub const GL_VIEWPORT_SWIZZLE_W_NV: GLenum = GLenum(0x935B);

pub const GL_VIEWPORT_SWIZZLE_X_NV: GLenum = GLenum(0x9358);

pub const GL_VIEWPORT_SWIZZLE_Y_NV: GLenum = GLenum(0x9359);

pub const GL_VIEWPORT_SWIZZLE_Z_NV: GLenum = GLenum(0x935A);

pub const GL_VIEW_CLASS_128_BITS: GLenum = GLenum(0x82C4);

pub const GL_VIEW_CLASS_16_BITS: GLenum = GLenum(0x82CA);

pub const GL_VIEW_CLASS_24_BITS: GLenum = GLenum(0x82C9);

pub const GL_VIEW_CLASS_32_BITS: GLenum = GLenum(0x82C8);

pub const GL_VIEW_CLASS_48_BITS: GLenum = GLenum(0x82C7);

pub const GL_VIEW_CLASS_64_BITS: GLenum = GLenum(0x82C6);

pub const GL_VIEW_CLASS_8_BITS: GLenum = GLenum(0x82CB);

pub const GL_VIEW_CLASS_96_BITS: GLenum = GLenum(0x82C5);

pub const GL_VIEW_CLASS_ASTC_10x10_RGBA: GLenum = GLenum(0x9393);

pub const GL_VIEW_CLASS_ASTC_10x5_RGBA: GLenum = GLenum(0x9390);

pub const GL_VIEW_CLASS_ASTC_10x6_RGBA: GLenum = GLenum(0x9391);

pub const GL_VIEW_CLASS_ASTC_10x8_RGBA: GLenum = GLenum(0x9392);

pub const GL_VIEW_CLASS_ASTC_12x10_RGBA: GLenum = GLenum(0x9394);

pub const GL_VIEW_CLASS_ASTC_12x12_RGBA: GLenum = GLenum(0x9395);

pub const GL_VIEW_CLASS_ASTC_4x4_RGBA: GLenum = GLenum(0x9388);

pub const GL_VIEW_CLASS_ASTC_5x4_RGBA: GLenum = GLenum(0x9389);

pub const GL_VIEW_CLASS_ASTC_5x5_RGBA: GLenum = GLenum(0x938A);

pub const GL_VIEW_CLASS_ASTC_6x5_RGBA: GLenum = GLenum(0x938B);

pub const GL_VIEW_CLASS_ASTC_6x6_RGBA: GLenum = GLenum(0x938C);

pub const GL_VIEW_CLASS_ASTC_8x5_RGBA: GLenum = GLenum(0x938D);

pub const GL_VIEW_CLASS_ASTC_8x6_RGBA: GLenum = GLenum(0x938E);

pub const GL_VIEW_CLASS_ASTC_8x8_RGBA: GLenum = GLenum(0x938F);

pub const GL_VIEW_CLASS_BPTC_FLOAT: GLenum = GLenum(0x82D3);

pub const GL_VIEW_CLASS_BPTC_UNORM: GLenum = GLenum(0x82D2);

pub const GL_VIEW_CLASS_EAC_R11: GLenum = GLenum(0x9383);

pub const GL_VIEW_CLASS_EAC_RG11: GLenum = GLenum(0x9384);

pub const GL_VIEW_CLASS_ETC2_EAC_RGBA: GLenum = GLenum(0x9387);

pub const GL_VIEW_CLASS_ETC2_RGB: GLenum = GLenum(0x9385);

pub const GL_VIEW_CLASS_ETC2_RGBA: GLenum = GLenum(0x9386);

pub const GL_VIEW_CLASS_RGTC1_RED: GLenum = GLenum(0x82D0);

pub const GL_VIEW_CLASS_RGTC2_RG: GLenum = GLenum(0x82D1);

pub const GL_VIEW_CLASS_S3TC_DXT1_RGB: GLenum = GLenum(0x82CC);

pub const GL_VIEW_CLASS_S3TC_DXT1_RGBA: GLenum = GLenum(0x82CD);

pub const GL_VIEW_CLASS_S3TC_DXT3_RGBA: GLenum = GLenum(0x82CE);

pub const GL_VIEW_CLASS_S3TC_DXT5_RGBA: GLenum = GLenum(0x82CF);

///
/// * Group: [`InternalFormatPName`]
pub const GL_VIEW_COMPATIBILITY_CLASS: GLenum = GLenum(0x82B6);

pub const GL_VIRTUAL_PAGE_SIZE_INDEX_ARB: GLenum = GLenum(0x91A7);

pub const GL_VIRTUAL_PAGE_SIZE_INDEX_EXT: GLenum = GLenum(0x91A7);

pub const GL_VIRTUAL_PAGE_SIZE_X_AMD: GLenum = GLenum(0x9195);

pub const GL_VIRTUAL_PAGE_SIZE_X_ARB: GLenum = GLenum(0x9195);

pub const GL_VIRTUAL_PAGE_SIZE_X_EXT: GLenum = GLenum(0x9195);

pub const GL_VIRTUAL_PAGE_SIZE_Y_AMD: GLenum = GLenum(0x9196);

pub const GL_VIRTUAL_PAGE_SIZE_Y_ARB: GLenum = GLenum(0x9196);

pub const GL_VIRTUAL_PAGE_SIZE_Y_EXT: GLenum = GLenum(0x9196);

pub const GL_VIRTUAL_PAGE_SIZE_Z_AMD: GLenum = GLenum(0x9197);

pub const GL_VIRTUAL_PAGE_SIZE_Z_ARB: GLenum = GLenum(0x9197);

pub const GL_VIRTUAL_PAGE_SIZE_Z_EXT: GLenum = GLenum(0x9197);

pub const GL_VIVIDLIGHT_NV: GLenum = GLenum(0x92A6);

pub const GL_VOLATILE_APPLE: GLenum = GLenum(0x8A1A);

///
/// * Group: [`SyncStatus`]
pub const GL_WAIT_FAILED: GLenum = GLenum(0x911D);

pub const GL_WAIT_FAILED_APPLE: GLenum = GLenum(0x911D);

pub const GL_WARPS_PER_SM_NV: GLenum = GLenum(0x933A);

pub const GL_WARP_SIZE_NV: GLenum = GLenum(0x9339);

pub const GL_WEIGHTED_AVERAGE_ARB: GLenum = GLenum(0x9367);

///
/// * Alias Of: [`GL_WEIGHTED_AVERAGE_ARB`]
pub const GL_WEIGHTED_AVERAGE_EXT: GLenum = GL_WEIGHTED_AVERAGE_ARB;

pub const GL_WEIGHT_ARRAY_ARB: GLenum = GLenum(0x86AD);

pub const GL_WEIGHT_ARRAY_BUFFER_BINDING: GLenum = GLenum(0x889E);

pub const GL_WEIGHT_ARRAY_BUFFER_BINDING_ARB: GLenum = GLenum(0x889E);

pub const GL_WEIGHT_ARRAY_BUFFER_BINDING_OES: GLenum = GLenum(0x889E);

pub const GL_WEIGHT_ARRAY_OES: GLenum = GLenum(0x86AD);

pub const GL_WEIGHT_ARRAY_POINTER_ARB: GLenum = GLenum(0x86AC);

pub const GL_WEIGHT_ARRAY_POINTER_OES: GLenum = GLenum(0x86AC);

pub const GL_WEIGHT_ARRAY_SIZE_ARB: GLenum = GLenum(0x86AB);

pub const GL_WEIGHT_ARRAY_SIZE_OES: GLenum = GLenum(0x86AB);

pub const GL_WEIGHT_ARRAY_STRIDE_ARB: GLenum = GLenum(0x86AA);

pub const GL_WEIGHT_ARRAY_STRIDE_OES: GLenum = GLenum(0x86AA);

pub const GL_WEIGHT_ARRAY_TYPE_ARB: GLenum = GLenum(0x86A9);

pub const GL_WEIGHT_ARRAY_TYPE_OES: GLenum = GLenum(0x86A9);

pub const GL_WEIGHT_SUM_UNITY_ARB: GLenum = GLenum(0x86A6);

///
/// * Group: [`HintTarget`]
pub const GL_WIDE_LINE_HINT_PGI: GLenum = GLenum(0x1A222);

pub const GL_WINDOW_RECTANGLE_EXT: GLenum = GLenum(0x8F12);

pub const GL_WINDOW_RECTANGLE_MODE_EXT: GLenum = GLenum(0x8F13);

pub const GL_WRAP_BORDER_SUN: GLenum = GLenum(0x81D4);

pub const GL_WRITEONLY_RENDERING_QCOM: GLenum = GLenum(0x8823);

pub const GL_WRITE_DISCARD_NV: GLenum = GLenum(0x88BE);

///
/// * Group: [`BufferAccessARB`]
pub const GL_WRITE_ONLY: GLenum = GLenum(0x88B9);

pub const GL_WRITE_ONLY_ARB: GLenum = GLenum(0x88B9);

pub const GL_WRITE_ONLY_OES: GLenum = GLenum(0x88B9);

pub const GL_WRITE_PIXEL_DATA_RANGE_LENGTH_NV: GLenum = GLenum(0x887A);

///
/// * Group: [`PixelDataRangeTargetNV`]
pub const GL_WRITE_PIXEL_DATA_RANGE_NV: GLenum = GLenum(0x8878);

pub const GL_WRITE_PIXEL_DATA_RANGE_POINTER_NV: GLenum = GLenum(0x887C);

///
/// * Group: [`VertexShaderCoordOutEXT`]
pub const GL_W_EXT: GLenum = GLenum(0x87D8);

///
/// * Group: [`LogicOp`]
pub const GL_XOR: GLenum = GLenum(0x1506);

pub const GL_XOR_NV: GLenum = GLenum(0x1506);

///
/// * Group: [`VertexShaderCoordOutEXT`]
pub const GL_X_EXT: GLenum = GLenum(0x87D5);

pub const GL_YCBAYCR8A_4224_NV: GLenum = GLenum(0x9032);

pub const GL_YCBCR_422_APPLE: GLenum = GLenum(0x85B9);

pub const GL_YCBCR_MESA: GLenum = GLenum(0x8757);

pub const GL_YCBYCR8_422_NV: GLenum = GLenum(0x9031);

pub const GL_YCRCBA_SGIX: GLenum = GLenum(0x8319);

///
/// * Group: [`PixelFormat`]
pub const GL_YCRCB_422_SGIX: GLenum = GLenum(0x81BB);

///
/// * Group: [`PixelFormat`]
pub const GL_YCRCB_444_SGIX: GLenum = GLenum(0x81BC);

pub const GL_YCRCB_SGIX: GLenum = GLenum(0x8318);

///
/// * Group: [`VertexShaderCoordOutEXT`]
pub const GL_Y_EXT: GLenum = GLenum(0x87D6);

/// NOT an alias. Accidental reuse of GL_DOT3_RGB_EXT
pub const GL_Z400_BINARY_AMD: GLenum = GLenum(0x8740);

pub const GL_Z4Y12Z4CB12Z4A12Z4Y12Z4CR12Z4A12_4224_NV: GLenum = GLenum(0x9036);

pub const GL_Z4Y12Z4CB12Z4CR12_444_NV: GLenum = GLenum(0x9037);

pub const GL_Z4Y12Z4CB12Z4Y12Z4CR12_422_NV: GLenum = GLenum(0x9035);

pub const GL_Z6Y10Z6CB10Z6A10Z6Y10Z6CR10Z6A10_4224_NV: GLenum = GLenum(0x9034);

pub const GL_Z6Y10Z6CB10Z6Y10Z6CR10_422_NV: GLenum = GLenum(0x9033);

///
/// * Group: [`TextureSwizzle`], [`StencilOp`], [`BlendingFactor`]
pub const GL_ZERO: GLenum = GLenum(0);

///
/// * Group: [`VertexShaderCoordOutEXT`]
pub const GL_ZERO_EXT: GLenum = GLenum(0x87DD);

///
/// * Group: [`ClipControlDepth`]
pub const GL_ZERO_TO_ONE: GLenum = GLenum(0x935F);

///
/// * Alias Of: [`GL_ZERO_TO_ONE`]
pub const GL_ZERO_TO_ONE_EXT: GLenum = GL_ZERO_TO_ONE;

///
/// * Group: [`GetPName`]
pub const GL_ZOOM_X: GLenum = GLenum(0x0D16);

///
/// * Group: [`GetPName`]
pub const GL_ZOOM_Y: GLenum = GLenum(0x0D17);

///
/// * Group: [`VertexShaderCoordOutEXT`]
pub const GL_Z_EXT: GLenum = GLenum(0x87D7);
