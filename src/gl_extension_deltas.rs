pub struct GlExtensionDelta {
  pub name: &'static str,
  //
  pub enums_added: &'static [&'static str],
  pub commands_added: &'static [&'static str],
}

// GL_KHR_debug is needed for OpenGL that is pre-4.3

/// The `GL_KHR_debug` extension, for OpenGL.
///
/// Incorrect for other variants of GL!
#[doc(alias = "GL_KHR_debug")]
pub const GL_KHR_debug_GL: GlExtensionDelta = GlExtensionDelta {
  name: "GL_KHR_debug",
  enums_added: &[
    "GL_BUFFER",
    "GL_CONTEXT_FLAG_DEBUG_BIT",
    "GL_DEBUG_CALLBACK_FUNCTION",
    "GL_DEBUG_CALLBACK_USER_PARAM",
    "GL_DEBUG_GROUP_STACK_DEPTH",
    "GL_DEBUG_LOGGED_MESSAGES",
    "GL_DEBUG_NEXT_LOGGED_MESSAGE_LENGTH",
    "GL_DEBUG_OUTPUT",
    "GL_DEBUG_OUTPUT_SYNCHRONOUS",
    "GL_DEBUG_SEVERITY_HIGH",
    "GL_DEBUG_SEVERITY_LOW",
    "GL_DEBUG_SEVERITY_MEDIUM",
    "GL_DEBUG_SEVERITY_NOTIFICATION",
    "GL_DEBUG_SOURCE_API",
    "GL_DEBUG_SOURCE_APPLICATION",
    "GL_DEBUG_SOURCE_OTHER",
    "GL_DEBUG_SOURCE_SHADER_COMPILER",
    "GL_DEBUG_SOURCE_THIRD_PARTY",
    "GL_DEBUG_SOURCE_WINDOW_SYSTEM",
    "GL_DEBUG_TYPE_DEPRECATED_BEHAVIOR",
    "GL_DEBUG_TYPE_ERROR",
    "GL_DEBUG_TYPE_MARKER",
    "GL_DEBUG_TYPE_OTHER",
    "GL_DEBUG_TYPE_PERFORMANCE",
    "GL_DEBUG_TYPE_POP_GROUP",
    "GL_DEBUG_TYPE_PORTABILITY",
    "GL_DEBUG_TYPE_PUSH_GROUP",
    "GL_DEBUG_TYPE_UNDEFINED_BEHAVIOR",
    "GL_MAX_DEBUG_GROUP_STACK_DEPTH",
    "GL_MAX_DEBUG_LOGGED_MESSAGES",
    "GL_MAX_DEBUG_MESSAGE_LENGTH",
    "GL_MAX_LABEL_LENGTH",
    "GL_PROGRAM",
    "GL_PROGRAM_PIPELINE",
    "GL_QUERY",
    "GL_SAMPLER",
    "GL_SHADER",
    "GL_STACK_OVERFLOW",
    "GL_STACK_UNDERFLOW",
    "GL_VERTEX_ARRAY",
  ],
  commands_added: &["glDebugMessageCallback", "glDebugMessageControl", "glDebugMessageInsert", "glGetDebugMessageLog", "glGetObjectLabel", "glGetObjectPtrLabel", "glGetPointerv", "glObjectLabel", "glObjectPtrLabel", "glPopDebugGroup", "glPushDebugGroup"],
};

/// The `GL_KHR_debug` extension, for OpenGL ES 1.
///
/// Incorrect for other versions of GL.
#[doc(alias = "GL_KHR_debug")]
pub const GL_KHR_debug_ES1: GlExtensionDelta = GlExtensionDelta {
  name: "GL_KHR_debug",
  enums_added: &[
    "GL_BUFFER_KHR",
    "GL_CONTEXT_FLAG_DEBUG_BIT_KHR",
    "GL_DEBUG_CALLBACK_FUNCTION_KHR",
    "GL_DEBUG_CALLBACK_USER_PARAM_KHR",
    "GL_DEBUG_GROUP_STACK_DEPTH_KHR",
    "GL_DEBUG_LOGGED_MESSAGES_KHR",
    "GL_DEBUG_NEXT_LOGGED_MESSAGE_LENGTH_KHR",
    "GL_DEBUG_OUTPUT_KHR",
    "GL_DEBUG_OUTPUT_SYNCHRONOUS_KHR",
    "GL_DEBUG_SEVERITY_HIGH_KHR",
    "GL_DEBUG_SEVERITY_LOW_KHR",
    "GL_DEBUG_SEVERITY_MEDIUM_KHR",
    "GL_DEBUG_SEVERITY_NOTIFICATION_KHR",
    "GL_DEBUG_SOURCE_API_KHR",
    "GL_DEBUG_SOURCE_APPLICATION_KHR",
    "GL_DEBUG_SOURCE_OTHER_KHR",
    "GL_DEBUG_SOURCE_SHADER_COMPILER_KHR",
    "GL_DEBUG_SOURCE_THIRD_PARTY_KHR",
    "GL_DEBUG_SOURCE_WINDOW_SYSTEM_KHR",
    "GL_DEBUG_TYPE_DEPRECATED_BEHAVIOR_KHR",
    "GL_DEBUG_TYPE_ERROR_KHR",
    "GL_DEBUG_TYPE_MARKER_KHR",
    "GL_DEBUG_TYPE_OTHER_KHR",
    "GL_DEBUG_TYPE_PERFORMANCE_KHR",
    "GL_DEBUG_TYPE_POP_GROUP_KHR",
    "GL_DEBUG_TYPE_PORTABILITY_KHR",
    "GL_DEBUG_TYPE_PUSH_GROUP_KHR",
    "GL_DEBUG_TYPE_UNDEFINED_BEHAVIOR_KHR",
    "GL_MAX_DEBUG_GROUP_STACK_DEPTH_KHR",
    "GL_MAX_DEBUG_LOGGED_MESSAGES_KHR",
    "GL_MAX_DEBUG_MESSAGE_LENGTH_KHR",
    "GL_MAX_LABEL_LENGTH_KHR",
    "GL_PROGRAM_KHR",
    "GL_PROGRAM_PIPELINE_KHR",
    "GL_QUERY_KHR",
    "GL_SAMPLER_KHR",
    "GL_SHADER_KHR",
    "GL_STACK_OVERFLOW_KHR",
    "GL_STACK_UNDERFLOW_KHR",
    "GL_VERTEX_ARRAY_KHR",
  ],
  commands_added: &["glDebugMessageCallbackKHR", "glDebugMessageControlKHR", "glDebugMessageInsertKHR", "glGetDebugMessageLogKHR", "glGetObjectLabelKHR", "glGetObjectPtrLabelKHR", "glGetPointervKHR", "glObjectLabelKHR", "glObjectPtrLabelKHR", "glPopDebugGroupKHR", "glPushDebugGroupKHR"],
};

/// The `GL_KHR_debug` extension, for OpenGL ES 2.
///
/// Incorrect for other versions of GL.
#[doc(alias = "GL_KHR_debug")]
pub const GL_KHR_debug_ES2: GlExtensionDelta = GlExtensionDelta {
  name: "GL_KHR_debug",
  enums_added: &[
    "GL_BUFFER_KHR",
    "GL_CONTEXT_FLAG_DEBUG_BIT_KHR",
    "GL_DEBUG_CALLBACK_FUNCTION_KHR",
    "GL_DEBUG_CALLBACK_USER_PARAM_KHR",
    "GL_DEBUG_GROUP_STACK_DEPTH_KHR",
    "GL_DEBUG_LOGGED_MESSAGES_KHR",
    "GL_DEBUG_NEXT_LOGGED_MESSAGE_LENGTH_KHR",
    "GL_DEBUG_OUTPUT_KHR",
    "GL_DEBUG_OUTPUT_SYNCHRONOUS_KHR",
    "GL_DEBUG_SEVERITY_HIGH_KHR",
    "GL_DEBUG_SEVERITY_LOW_KHR",
    "GL_DEBUG_SEVERITY_MEDIUM_KHR",
    "GL_DEBUG_SEVERITY_NOTIFICATION_KHR",
    "GL_DEBUG_SOURCE_API_KHR",
    "GL_DEBUG_SOURCE_APPLICATION_KHR",
    "GL_DEBUG_SOURCE_OTHER_KHR",
    "GL_DEBUG_SOURCE_SHADER_COMPILER_KHR",
    "GL_DEBUG_SOURCE_THIRD_PARTY_KHR",
    "GL_DEBUG_SOURCE_WINDOW_SYSTEM_KHR",
    "GL_DEBUG_TYPE_DEPRECATED_BEHAVIOR_KHR",
    "GL_DEBUG_TYPE_ERROR_KHR",
    "GL_DEBUG_TYPE_MARKER_KHR",
    "GL_DEBUG_TYPE_OTHER_KHR",
    "GL_DEBUG_TYPE_PERFORMANCE_KHR",
    "GL_DEBUG_TYPE_POP_GROUP_KHR",
    "GL_DEBUG_TYPE_PORTABILITY_KHR",
    "GL_DEBUG_TYPE_PUSH_GROUP_KHR",
    "GL_DEBUG_TYPE_UNDEFINED_BEHAVIOR_KHR",
    "GL_MAX_DEBUG_GROUP_STACK_DEPTH_KHR",
    "GL_MAX_DEBUG_LOGGED_MESSAGES_KHR",
    "GL_MAX_DEBUG_MESSAGE_LENGTH_KHR",
    "GL_MAX_LABEL_LENGTH_KHR",
    "GL_PROGRAM_KHR",
    "GL_PROGRAM_PIPELINE_KHR",
    "GL_QUERY_KHR",
    "GL_SAMPLER_KHR",
    "GL_SHADER_KHR",
    "GL_STACK_OVERFLOW_KHR",
    "GL_STACK_UNDERFLOW_KHR",
    "GL_VERTEX_ARRAY_KHR",
  ],
  commands_added: &["glDebugMessageCallbackKHR", "glDebugMessageControlKHR", "glDebugMessageInsertKHR", "glGetDebugMessageLogKHR", "glGetObjectLabelKHR", "glGetObjectPtrLabelKHR", "glGetPointervKHR", "glObjectLabelKHR", "glObjectPtrLabelKHR", "glPopDebugGroupKHR", "glPushDebugGroupKHR"],
};

// GL_ARB_indirect_parameters is needed for OpenGL that is pre-4.6

pub const GL_ARB_indirect_parameters: GlExtensionDelta = GlExtensionDelta { name: "GL_ARB_indirect_parameters", enums_added: &["GL_PARAMETER_BUFFER_ARB", "GL_PARAMETER_BUFFER_BINDING_ARB"], commands_added: &["glMultiDrawArraysIndirectCountARB", "glMultiDrawElementsIndirectCountARB"] };

// This stuff never became core

pub const GL_ARB_texture_filter_anisotropic: GlExtensionDelta = GlExtensionDelta { name: "GL_ARB_texture_filter_anisotropic", enums_added: &["GL_TEXTURE_MAX_ANISOTROPY", "GL_MAX_TEXTURE_MAX_ANISOTROPY"], commands_added: &[] };

pub const GL_ARB_bindless_texture: GlExtensionDelta = GlExtensionDelta { name: "GL_ARB_bindless_texture", enums_added: &["GL_UNSIGNED_INT64_ARB"], commands_added: &["glGetTextureHandleARB", "glGetTextureSamplerHandleARB", "glMakeTextureHandleResidentARB", "glMakeTextureHandleNonResidentARB", "glGetImageHandleARB", "glMakeImageHandleResidentARB", "glMakeImageHandleNonResidentARB", "glUniformHandleui64ARB", "glUniformHandleui64vARB", "glProgramUniformHandleui64ARB", "glProgramUniformHandleui64vARB", "glIsTextureHandleResidentARB", "glIsImageHandleResidentARB", "glVertexAttribL1ui64ARB", "glVertexAttribL1ui64vARB", "glGetVertexAttribLui64vARB"] };

pub const GL_ARB_sparse_texture: GlExtensionDelta = GlExtensionDelta { name: "GL_ARB_sparse_texture", enums_added: &["GL_TEXTURE_SPARSE_ARB", "GL_VIRTUAL_PAGE_SIZE_INDEX_ARB", "GL_NUM_SPARSE_LEVELS_ARB", "GL_NUM_VIRTUAL_PAGE_SIZES_ARB", "GL_VIRTUAL_PAGE_SIZE_X_ARB", "GL_VIRTUAL_PAGE_SIZE_Y_ARB", "GL_VIRTUAL_PAGE_SIZE_Z_ARB", "GL_MAX_SPARSE_TEXTURE_SIZE_ARB", "GL_MAX_SPARSE_3D_TEXTURE_SIZE_ARB", "GL_MAX_SPARSE_ARRAY_TEXTURE_LAYERS_ARB", "GL_SPARSE_TEXTURE_FULL_ARRAY_CUBE_MIPMAPS_ARB"], commands_added: &["glTexPageCommitmentARB"] };

pub const GL_ARB_pipeline_statistics_query: GlExtensionDelta = GlExtensionDelta { name: "GL_ARB_pipeline_statistics_query", enums_added: &["GL_VERTICES_SUBMITTED_ARB", "GL_PRIMITIVES_SUBMITTED_ARB", "GL_VERTEX_SHADER_INVOCATIONS_ARB", "GL_TESS_CONTROL_SHADER_PATCHES_ARB", "GL_TESS_EVALUATION_SHADER_INVOCATIONS_ARB", "GL_GEOMETRY_SHADER_INVOCATIONS", "GL_GEOMETRY_SHADER_PRIMITIVES_EMITTED_ARB", "GL_FRAGMENT_SHADER_INVOCATIONS_ARB", "GL_COMPUTE_SHADER_INVOCATIONS_ARB", "GL_CLIPPING_INPUT_PRIMITIVES_ARB", "GL_CLIPPING_OUTPUT_PRIMITIVES_ARB"], commands_added: &[] };
