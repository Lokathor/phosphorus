use super::*;

pub const GL_3DFX_tbuffer_COMMANDS: &[&str] = &["glTbufferMask3DFX"];

pub const GL_AMD_debug_output_COMMANDS: &[&str] = &["glDebugMessageCallbackAMD", "glDebugMessageEnableAMD", "glDebugMessageInsertAMD", "glGetDebugMessageLogAMD"];

pub const GL_AMD_draw_buffers_blend_COMMANDS: &[&str] = &["glBlendEquationIndexedAMD", "glBlendEquationSeparateIndexedAMD", "glBlendFuncIndexedAMD", "glBlendFuncSeparateIndexedAMD"];

pub const GL_AMD_framebuffer_multisample_advanced_COMMANDS: &[&str] = &["glNamedRenderbufferStorageMultisampleAdvancedAMD", "glRenderbufferStorageMultisampleAdvancedAMD"];

pub const GL_AMD_framebuffer_sample_positions_COMMANDS: &[&str] = &["glFramebufferSamplePositionsfvAMD", "glGetFramebufferParameterfvAMD", "glGetNamedFramebufferParameterfvAMD", "glNamedFramebufferSamplePositionsfvAMD"];

pub const GL_AMD_gpu_shader_int64_COMMANDS: &[&str] = &[
  "glGetUniformi64vNV",
  "glGetUniformui64vNV",
  "glProgramUniform1i64NV",
  "glProgramUniform1i64vNV",
  "glProgramUniform1ui64NV",
  "glProgramUniform1ui64vNV",
  "glProgramUniform2i64NV",
  "glProgramUniform2i64vNV",
  "glProgramUniform2ui64NV",
  "glProgramUniform2ui64vNV",
  "glProgramUniform3i64NV",
  "glProgramUniform3i64vNV",
  "glProgramUniform3ui64NV",
  "glProgramUniform3ui64vNV",
  "glProgramUniform4i64NV",
  "glProgramUniform4i64vNV",
  "glProgramUniform4ui64NV",
  "glProgramUniform4ui64vNV",
  "glUniform1i64NV",
  "glUniform1i64vNV",
  "glUniform1ui64NV",
  "glUniform1ui64vNV",
  "glUniform2i64NV",
  "glUniform2i64vNV",
  "glUniform2ui64NV",
  "glUniform2ui64vNV",
  "glUniform3i64NV",
  "glUniform3i64vNV",
  "glUniform3ui64NV",
  "glUniform3ui64vNV",
  "glUniform4i64NV",
  "glUniform4i64vNV",
  "glUniform4ui64NV",
  "glUniform4ui64vNV",
];

pub const GL_AMD_interleaved_elements_COMMANDS: &[&str] = &["glVertexAttribParameteriAMD"];

pub const GL_AMD_multi_draw_indirect_COMMANDS: &[&str] = &["glMultiDrawArraysIndirectAMD", "glMultiDrawElementsIndirectAMD"];

pub const GL_AMD_name_gen_delete_COMMANDS: &[&str] = &["glDeleteNamesAMD", "glGenNamesAMD", "glIsNameAMD"];

pub const GL_AMD_occlusion_query_event_COMMANDS: &[&str] = &["glQueryObjectParameteruiAMD"];

pub const GL_AMD_performance_monitor_COMMANDS: &[&str] = &["glBeginPerfMonitorAMD", "glDeletePerfMonitorsAMD", "glEndPerfMonitorAMD", "glGenPerfMonitorsAMD", "glGetPerfMonitorCounterDataAMD", "glGetPerfMonitorCounterInfoAMD", "glGetPerfMonitorCounterStringAMD", "glGetPerfMonitorCountersAMD", "glGetPerfMonitorGroupStringAMD", "glGetPerfMonitorGroupsAMD", "glSelectPerfMonitorCountersAMD"];

pub const GL_AMD_sample_positions_COMMANDS: &[&str] = &["glSetMultisamplefvAMD"];

pub const GL_AMD_sparse_texture_COMMANDS: &[&str] = &["glTexStorageSparseAMD", "glTextureStorageSparseAMD"];

pub const GL_AMD_stencil_operation_extended_COMMANDS: &[&str] = &["glStencilOpValueAMD"];

pub const GL_AMD_vertex_shader_tessellator_COMMANDS: &[&str] = &["glTessellationFactorAMD", "glTessellationModeAMD"];

pub const GL_ANGLE_framebuffer_blit_COMMANDS: &[&str] = &["glBlitFramebufferANGLE"];

pub const GL_ANGLE_framebuffer_multisample_COMMANDS: &[&str] = &["glRenderbufferStorageMultisampleANGLE"];

pub const GL_ANGLE_instanced_arrays_COMMANDS: &[&str] = &["glDrawArraysInstancedANGLE", "glDrawElementsInstancedANGLE", "glVertexAttribDivisorANGLE"];

pub const GL_ANGLE_translated_shader_source_COMMANDS: &[&str] = &["glGetTranslatedShaderSourceANGLE"];

pub const GL_APPLE_copy_texture_levels_COMMANDS: &[&str] = &["glCopyTextureLevelsAPPLE"];

pub const GL_APPLE_element_array_COMMANDS: &[&str] = &["glDrawElementArrayAPPLE", "glDrawRangeElementArrayAPPLE", "glElementPointerAPPLE", "glMultiDrawElementArrayAPPLE", "glMultiDrawRangeElementArrayAPPLE"];

pub const GL_APPLE_fence_COMMANDS: &[&str] = &["glDeleteFencesAPPLE", "glFinishFenceAPPLE", "glFinishObjectAPPLE", "glGenFencesAPPLE", "glIsFenceAPPLE", "glSetFenceAPPLE", "glTestFenceAPPLE", "glTestObjectAPPLE"];

pub const GL_APPLE_flush_buffer_range_COMMANDS: &[&str] = &["glBufferParameteriAPPLE", "glFlushMappedBufferRangeAPPLE"];

pub const GL_APPLE_framebuffer_multisample_COMMANDS: &[&str] = &["glRenderbufferStorageMultisampleAPPLE", "glResolveMultisampleFramebufferAPPLE"];

pub const GL_APPLE_object_purgeable_COMMANDS: &[&str] = &["glGetObjectParameterivAPPLE", "glObjectPurgeableAPPLE", "glObjectUnpurgeableAPPLE"];

pub const GL_APPLE_sync_COMMANDS: &[&str] = &["glClientWaitSyncAPPLE", "glDeleteSyncAPPLE", "glFenceSyncAPPLE", "glGetInteger64vAPPLE", "glGetSyncivAPPLE", "glIsSyncAPPLE", "glWaitSyncAPPLE"];

pub const GL_APPLE_texture_range_COMMANDS: &[&str] = &["glGetTexParameterPointervAPPLE", "glTextureRangeAPPLE"];

pub const GL_APPLE_vertex_array_object_COMMANDS: &[&str] = &["glBindVertexArrayAPPLE", "glDeleteVertexArraysAPPLE", "glGenVertexArraysAPPLE", "glIsVertexArrayAPPLE"];

pub const GL_APPLE_vertex_array_range_COMMANDS: &[&str] = &["glFlushVertexArrayRangeAPPLE", "glVertexArrayParameteriAPPLE", "glVertexArrayRangeAPPLE"];

pub const GL_APPLE_vertex_program_evaluators_COMMANDS: &[&str] = &["glDisableVertexAttribAPPLE", "glEnableVertexAttribAPPLE", "glIsVertexAttribEnabledAPPLE", "glMapVertexAttrib1dAPPLE", "glMapVertexAttrib1fAPPLE", "glMapVertexAttrib2dAPPLE", "glMapVertexAttrib2fAPPLE"];

pub const GL_ARB_ES2_compatibility_COMMANDS: &[&str] = &["glClearDepthf", "glDepthRangef", "glGetShaderPrecisionFormat", "glReleaseShaderCompiler", "glShaderBinary"];

pub const GL_ARB_ES3_1_compatibility_COMMANDS: &[&str] = &["glMemoryBarrierByRegion"];

pub const GL_ARB_ES3_2_compatibility_COMMANDS: &[&str] = &["glPrimitiveBoundingBoxARB"];

pub const GL_ARB_base_instance_COMMANDS: &[&str] = &["glDrawArraysInstancedBaseInstance", "glDrawElementsInstancedBaseInstance", "glDrawElementsInstancedBaseVertexBaseInstance"];

pub const GL_ARB_bindless_texture_COMMANDS: &[&str] = &["glGetImageHandleARB", "glGetTextureHandleARB", "glGetTextureSamplerHandleARB", "glGetVertexAttribLui64vARB", "glIsImageHandleResidentARB", "glIsTextureHandleResidentARB", "glMakeImageHandleNonResidentARB", "glMakeImageHandleResidentARB", "glMakeTextureHandleNonResidentARB", "glMakeTextureHandleResidentARB", "glProgramUniformHandleui64ARB", "glProgramUniformHandleui64vARB", "glUniformHandleui64ARB", "glUniformHandleui64vARB", "glVertexAttribL1ui64ARB", "glVertexAttribL1ui64vARB"];

pub const GL_ARB_blend_func_extended_COMMANDS: &[&str] = &["glBindFragDataLocationIndexed", "glGetFragDataIndex"];

pub const GL_ARB_buffer_storage_COMMANDS: &[&str] = &["glBufferStorage"];

pub const GL_ARB_cl_event_COMMANDS: &[&str] = &["glCreateSyncFromCLeventARB"];

pub const GL_ARB_clear_buffer_object_COMMANDS: &[&str] = &["glClearBufferData", "glClearBufferSubData"];

pub const GL_ARB_clear_texture_COMMANDS: &[&str] = &["glClearTexImage", "glClearTexSubImage"];

pub const GL_ARB_clip_control_COMMANDS: &[&str] = &["glClipControl"];

pub const GL_ARB_color_buffer_float_COMMANDS: &[&str] = &["glClampColorARB"];

pub const GL_ARB_compute_shader_COMMANDS: &[&str] = &["glDispatchCompute", "glDispatchComputeIndirect"];

pub const GL_ARB_compute_variable_group_size_COMMANDS: &[&str] = &["glDispatchComputeGroupSizeARB"];

pub const GL_ARB_copy_buffer_COMMANDS: &[&str] = &["glCopyBufferSubData"];

pub const GL_ARB_copy_image_COMMANDS: &[&str] = &["glCopyImageSubData"];

pub const GL_ARB_debug_output_COMMANDS: &[&str] = &["glDebugMessageCallbackARB", "glDebugMessageControlARB", "glDebugMessageInsertARB", "glGetDebugMessageLogARB"];

pub const GL_ARB_direct_state_access_COMMANDS: &[&str] = &[
  "glBindTextureUnit",
  "glBlitNamedFramebuffer",
  "glCheckNamedFramebufferStatus",
  "glClearNamedBufferData",
  "glClearNamedBufferSubData",
  "glClearNamedFramebufferfi",
  "glClearNamedFramebufferfv",
  "glClearNamedFramebufferiv",
  "glClearNamedFramebufferuiv",
  "glCompressedTextureSubImage1D",
  "glCompressedTextureSubImage2D",
  "glCompressedTextureSubImage3D",
  "glCopyNamedBufferSubData",
  "glCopyTextureSubImage1D",
  "glCopyTextureSubImage2D",
  "glCopyTextureSubImage3D",
  "glCreateBuffers",
  "glCreateFramebuffers",
  "glCreateProgramPipelines",
  "glCreateQueries",
  "glCreateRenderbuffers",
  "glCreateSamplers",
  "glCreateTextures",
  "glCreateTransformFeedbacks",
  "glCreateVertexArrays",
  "glDisableVertexArrayAttrib",
  "glEnableVertexArrayAttrib",
  "glFlushMappedNamedBufferRange",
  "glGenerateTextureMipmap",
  "glGetCompressedTextureImage",
  "glGetNamedBufferParameteri64v",
  "glGetNamedBufferParameteriv",
  "glGetNamedBufferPointerv",
  "glGetNamedBufferSubData",
  "glGetNamedFramebufferAttachmentParameteriv",
  "glGetNamedFramebufferParameteriv",
  "glGetNamedRenderbufferParameteriv",
  "glGetQueryBufferObjecti64v",
  "glGetQueryBufferObjectiv",
  "glGetQueryBufferObjectui64v",
  "glGetQueryBufferObjectuiv",
  "glGetTextureImage",
  "glGetTextureLevelParameterfv",
  "glGetTextureLevelParameteriv",
  "glGetTextureParameterIiv",
  "glGetTextureParameterIuiv",
  "glGetTextureParameterfv",
  "glGetTextureParameteriv",
  "glGetTransformFeedbacki64_v",
  "glGetTransformFeedbacki_v",
  "glGetTransformFeedbackiv",
  "glGetVertexArrayIndexed64iv",
  "glGetVertexArrayIndexediv",
  "glGetVertexArrayiv",
  "glInvalidateNamedFramebufferData",
  "glInvalidateNamedFramebufferSubData",
  "glMapNamedBuffer",
  "glMapNamedBufferRange",
  "glNamedBufferData",
  "glNamedBufferStorage",
  "glNamedBufferSubData",
  "glNamedFramebufferDrawBuffer",
  "glNamedFramebufferDrawBuffers",
  "glNamedFramebufferParameteri",
  "glNamedFramebufferReadBuffer",
  "glNamedFramebufferRenderbuffer",
  "glNamedFramebufferTexture",
  "glNamedFramebufferTextureLayer",
  "glNamedRenderbufferStorage",
  "glNamedRenderbufferStorageMultisample",
  "glTextureBuffer",
  "glTextureBufferRange",
  "glTextureParameterIiv",
  "glTextureParameterIuiv",
  "glTextureParameterf",
  "glTextureParameterfv",
  "glTextureParameteri",
  "glTextureParameteriv",
  "glTextureStorage1D",
  "glTextureStorage2D",
  "glTextureStorage2DMultisample",
  "glTextureStorage3D",
  "glTextureStorage3DMultisample",
  "glTextureSubImage1D",
  "glTextureSubImage2D",
  "glTextureSubImage3D",
  "glTransformFeedbackBufferBase",
  "glTransformFeedbackBufferRange",
  "glUnmapNamedBuffer",
  "glVertexArrayAttribBinding",
  "glVertexArrayAttribFormat",
  "glVertexArrayAttribIFormat",
  "glVertexArrayAttribLFormat",
  "glVertexArrayBindingDivisor",
  "glVertexArrayElementBuffer",
  "glVertexArrayVertexBuffer",
  "glVertexArrayVertexBuffers",
];

pub const GL_ARB_draw_buffers_COMMANDS: &[&str] = &["glDrawBuffersARB"];

pub const GL_ARB_draw_buffers_blend_COMMANDS: &[&str] = &["glBlendEquationSeparateiARB", "glBlendEquationiARB", "glBlendFuncSeparateiARB", "glBlendFunciARB"];

pub const GL_ARB_draw_elements_base_vertex_COMMANDS: &[&str] = &["glDrawElementsBaseVertex", "glDrawElementsInstancedBaseVertex", "glDrawRangeElementsBaseVertex", "glMultiDrawElementsBaseVertex"];

pub const GL_ARB_draw_indirect_COMMANDS: &[&str] = &["glDrawArraysIndirect", "glDrawElementsIndirect"];

pub const GL_ARB_draw_instanced_COMMANDS: &[&str] = &["glDrawArraysInstancedARB", "glDrawElementsInstancedARB"];

pub const GL_ARB_fragment_program_COMMANDS: &[&str] = &["glBindProgramARB", "glDeleteProgramsARB", "glGenProgramsARB", "glGetProgramEnvParameterdvARB", "glGetProgramEnvParameterfvARB", "glGetProgramLocalParameterdvARB", "glGetProgramLocalParameterfvARB", "glGetProgramStringARB", "glGetProgramivARB", "glIsProgramARB", "glProgramEnvParameter4dARB", "glProgramEnvParameter4dvARB", "glProgramEnvParameter4fARB", "glProgramEnvParameter4fvARB", "glProgramLocalParameter4dARB", "glProgramLocalParameter4dvARB", "glProgramLocalParameter4fARB", "glProgramLocalParameter4fvARB", "glProgramStringARB"];

pub const GL_ARB_framebuffer_no_attachments_COMMANDS: &[&str] = &["glFramebufferParameteri", "glGetFramebufferParameteriv"];

pub const GL_ARB_framebuffer_object_COMMANDS: &[&str] = &["glBindFramebuffer", "glBindRenderbuffer", "glBlitFramebuffer", "glCheckFramebufferStatus", "glDeleteFramebuffers", "glDeleteRenderbuffers", "glFramebufferRenderbuffer", "glFramebufferTexture1D", "glFramebufferTexture2D", "glFramebufferTexture3D", "glFramebufferTextureLayer", "glGenFramebuffers", "glGenRenderbuffers", "glGenerateMipmap", "glGetFramebufferAttachmentParameteriv", "glGetRenderbufferParameteriv", "glIsFramebuffer", "glIsRenderbuffer", "glRenderbufferStorage", "glRenderbufferStorageMultisample"];

pub const GL_ARB_geometry_shader4_COMMANDS: &[&str] = &["glFramebufferTextureARB", "glFramebufferTextureFaceARB", "glFramebufferTextureLayerARB", "glProgramParameteriARB"];

pub const GL_ARB_get_program_binary_COMMANDS: &[&str] = &["glGetProgramBinary", "glProgramBinary", "glProgramParameteri"];

pub const GL_ARB_get_texture_sub_image_COMMANDS: &[&str] = &["glGetCompressedTextureSubImage", "glGetTextureSubImage"];

pub const GL_ARB_gl_spirv_COMMANDS: &[&str] = &["glSpecializeShaderARB"];

pub const GL_ARB_gpu_shader_fp64_COMMANDS: &[&str] = &["glGetUniformdv", "glUniform1d", "glUniform1dv", "glUniform2d", "glUniform2dv", "glUniform3d", "glUniform3dv", "glUniform4d", "glUniform4dv", "glUniformMatrix2dv", "glUniformMatrix2x3dv", "glUniformMatrix2x4dv", "glUniformMatrix3dv", "glUniformMatrix3x2dv", "glUniformMatrix3x4dv", "glUniformMatrix4dv", "glUniformMatrix4x2dv", "glUniformMatrix4x3dv"];

pub const GL_ARB_gpu_shader_int64_COMMANDS: &[&str] = &[
  "glGetUniformi64vARB",
  "glGetUniformui64vARB",
  "glGetnUniformi64vARB",
  "glGetnUniformui64vARB",
  "glProgramUniform1i64ARB",
  "glProgramUniform1i64vARB",
  "glProgramUniform1ui64ARB",
  "glProgramUniform1ui64vARB",
  "glProgramUniform2i64ARB",
  "glProgramUniform2i64vARB",
  "glProgramUniform2ui64ARB",
  "glProgramUniform2ui64vARB",
  "glProgramUniform3i64ARB",
  "glProgramUniform3i64vARB",
  "glProgramUniform3ui64ARB",
  "glProgramUniform3ui64vARB",
  "glProgramUniform4i64ARB",
  "glProgramUniform4i64vARB",
  "glProgramUniform4ui64ARB",
  "glProgramUniform4ui64vARB",
  "glUniform1i64ARB",
  "glUniform1i64vARB",
  "glUniform1ui64ARB",
  "glUniform1ui64vARB",
  "glUniform2i64ARB",
  "glUniform2i64vARB",
  "glUniform2ui64ARB",
  "glUniform2ui64vARB",
  "glUniform3i64ARB",
  "glUniform3i64vARB",
  "glUniform3ui64ARB",
  "glUniform3ui64vARB",
  "glUniform4i64ARB",
  "glUniform4i64vARB",
  "glUniform4ui64ARB",
  "glUniform4ui64vARB",
];

pub const GL_ARB_imaging_COMMANDS: &[&str] = &[
  "glBlendColor",
  "glBlendEquation",
  "glColorSubTable",
  "glColorTable",
  "glColorTableParameterfv",
  "glColorTableParameteriv",
  "glConvolutionFilter1D",
  "glConvolutionFilter2D",
  "glConvolutionParameterf",
  "glConvolutionParameterfv",
  "glConvolutionParameteri",
  "glConvolutionParameteriv",
  "glCopyColorSubTable",
  "glCopyColorTable",
  "glCopyConvolutionFilter1D",
  "glCopyConvolutionFilter2D",
  "glGetColorTable",
  "glGetColorTableParameterfv",
  "glGetColorTableParameteriv",
  "glGetConvolutionFilter",
  "glGetConvolutionParameterfv",
  "glGetConvolutionParameteriv",
  "glGetHistogram",
  "glGetHistogramParameterfv",
  "glGetHistogramParameteriv",
  "glGetMinmax",
  "glGetMinmaxParameterfv",
  "glGetMinmaxParameteriv",
  "glGetSeparableFilter",
  "glHistogram",
  "glMinmax",
  "glResetHistogram",
  "glResetMinmax",
  "glSeparableFilter2D",
];

pub const GL_ARB_indirect_parameters_COMMANDS: &[&str] = &["glMultiDrawArraysIndirectCountARB", "glMultiDrawElementsIndirectCountARB"];

pub const GL_ARB_instanced_arrays_COMMANDS: &[&str] = &["glVertexAttribDivisorARB"];

pub const GL_ARB_internalformat_query_COMMANDS: &[&str] = &["glGetInternalformativ"];

pub const GL_ARB_internalformat_query2_COMMANDS: &[&str] = &["glGetInternalformati64v"];

pub const GL_ARB_invalidate_subdata_COMMANDS: &[&str] = &["glInvalidateBufferData", "glInvalidateBufferSubData", "glInvalidateFramebuffer", "glInvalidateSubFramebuffer", "glInvalidateTexImage", "glInvalidateTexSubImage"];

pub const GL_ARB_map_buffer_range_COMMANDS: &[&str] = &["glFlushMappedBufferRange", "glMapBufferRange"];

pub const GL_ARB_matrix_palette_COMMANDS: &[&str] = &["glCurrentPaletteMatrixARB", "glMatrixIndexPointerARB", "glMatrixIndexubvARB", "glMatrixIndexuivARB", "glMatrixIndexusvARB"];

pub const GL_ARB_multi_bind_COMMANDS: &[&str] = &["glBindBuffersBase", "glBindBuffersRange", "glBindImageTextures", "glBindSamplers", "glBindTextures", "glBindVertexBuffers"];

pub const GL_ARB_multi_draw_indirect_COMMANDS: &[&str] = &["glMultiDrawArraysIndirect", "glMultiDrawElementsIndirect"];

pub const GL_ARB_multisample_COMMANDS: &[&str] = &["glSampleCoverageARB"];

pub const GL_ARB_multitexture_COMMANDS: &[&str] = &[
  "glActiveTextureARB",
  "glClientActiveTextureARB",
  "glMultiTexCoord1dARB",
  "glMultiTexCoord1dvARB",
  "glMultiTexCoord1fARB",
  "glMultiTexCoord1fvARB",
  "glMultiTexCoord1iARB",
  "glMultiTexCoord1ivARB",
  "glMultiTexCoord1sARB",
  "glMultiTexCoord1svARB",
  "glMultiTexCoord2dARB",
  "glMultiTexCoord2dvARB",
  "glMultiTexCoord2fARB",
  "glMultiTexCoord2fvARB",
  "glMultiTexCoord2iARB",
  "glMultiTexCoord2ivARB",
  "glMultiTexCoord2sARB",
  "glMultiTexCoord2svARB",
  "glMultiTexCoord3dARB",
  "glMultiTexCoord3dvARB",
  "glMultiTexCoord3fARB",
  "glMultiTexCoord3fvARB",
  "glMultiTexCoord3iARB",
  "glMultiTexCoord3ivARB",
  "glMultiTexCoord3sARB",
  "glMultiTexCoord3svARB",
  "glMultiTexCoord4dARB",
  "glMultiTexCoord4dvARB",
  "glMultiTexCoord4fARB",
  "glMultiTexCoord4fvARB",
  "glMultiTexCoord4iARB",
  "glMultiTexCoord4ivARB",
  "glMultiTexCoord4sARB",
  "glMultiTexCoord4svARB",
];

pub const GL_ARB_occlusion_query_COMMANDS: &[&str] = &["glBeginQueryARB", "glDeleteQueriesARB", "glEndQueryARB", "glGenQueriesARB", "glGetQueryObjectivARB", "glGetQueryObjectuivARB", "glGetQueryivARB", "glIsQueryARB"];

pub const GL_ARB_parallel_shader_compile_COMMANDS: &[&str] = &["glMaxShaderCompilerThreadsARB"];

pub const GL_ARB_point_parameters_COMMANDS: &[&str] = &["glPointParameterfARB", "glPointParameterfvARB"];

pub const GL_ARB_polygon_offset_clamp_COMMANDS: &[&str] = &["glPolygonOffsetClamp"];

pub const GL_ARB_program_interface_query_COMMANDS: &[&str] = &["glGetProgramInterfaceiv", "glGetProgramResourceIndex", "glGetProgramResourceLocation", "glGetProgramResourceLocationIndex", "glGetProgramResourceName", "glGetProgramResourceiv"];

pub const GL_ARB_provoking_vertex_COMMANDS: &[&str] = &["glProvokingVertex"];

pub const GL_ARB_robustness_COMMANDS: &[&str] = &["glGetGraphicsResetStatusARB", "glGetnColorTableARB", "glGetnCompressedTexImageARB", "glGetnConvolutionFilterARB", "glGetnHistogramARB", "glGetnMapdvARB", "glGetnMapfvARB", "glGetnMapivARB", "glGetnMinmaxARB", "glGetnPixelMapfvARB", "glGetnPixelMapuivARB", "glGetnPixelMapusvARB", "glGetnPolygonStippleARB", "glGetnSeparableFilterARB", "glGetnTexImageARB", "glGetnUniformdvARB", "glGetnUniformfvARB", "glGetnUniformivARB", "glGetnUniformuivARB", "glReadnPixelsARB"];

pub const GL_ARB_sample_locations_COMMANDS: &[&str] = &["glEvaluateDepthValuesARB", "glFramebufferSampleLocationsfvARB", "glNamedFramebufferSampleLocationsfvARB"];

pub const GL_ARB_sample_shading_COMMANDS: &[&str] = &["glMinSampleShadingARB"];

pub const GL_ARB_sampler_objects_COMMANDS: &[&str] = &["glBindSampler", "glDeleteSamplers", "glGenSamplers", "glGetSamplerParameterIiv", "glGetSamplerParameterIuiv", "glGetSamplerParameterfv", "glGetSamplerParameteriv", "glIsSampler", "glSamplerParameterIiv", "glSamplerParameterIuiv", "glSamplerParameterf", "glSamplerParameterfv", "glSamplerParameteri", "glSamplerParameteriv"];

pub const GL_ARB_separate_shader_objects_COMMANDS: &[&str] = &[
  "glActiveShaderProgram",
  "glBindProgramPipeline",
  "glCreateShaderProgramv",
  "glDeleteProgramPipelines",
  "glGenProgramPipelines",
  "glGetProgramPipelineInfoLog",
  "glGetProgramPipelineiv",
  "glIsProgramPipeline",
  "glProgramParameteri",
  "glProgramUniform1d",
  "glProgramUniform1dv",
  "glProgramUniform1f",
  "glProgramUniform1fv",
  "glProgramUniform1i",
  "glProgramUniform1iv",
  "glProgramUniform1ui",
  "glProgramUniform1uiv",
  "glProgramUniform2d",
  "glProgramUniform2dv",
  "glProgramUniform2f",
  "glProgramUniform2fv",
  "glProgramUniform2i",
  "glProgramUniform2iv",
  "glProgramUniform2ui",
  "glProgramUniform2uiv",
  "glProgramUniform3d",
  "glProgramUniform3dv",
  "glProgramUniform3f",
  "glProgramUniform3fv",
  "glProgramUniform3i",
  "glProgramUniform3iv",
  "glProgramUniform3ui",
  "glProgramUniform3uiv",
  "glProgramUniform4d",
  "glProgramUniform4dv",
  "glProgramUniform4f",
  "glProgramUniform4fv",
  "glProgramUniform4i",
  "glProgramUniform4iv",
  "glProgramUniform4ui",
  "glProgramUniform4uiv",
  "glProgramUniformMatrix2dv",
  "glProgramUniformMatrix2fv",
  "glProgramUniformMatrix2x3dv",
  "glProgramUniformMatrix2x3fv",
  "glProgramUniformMatrix2x4dv",
  "glProgramUniformMatrix2x4fv",
  "glProgramUniformMatrix3dv",
  "glProgramUniformMatrix3fv",
  "glProgramUniformMatrix3x2dv",
  "glProgramUniformMatrix3x2fv",
  "glProgramUniformMatrix3x4dv",
  "glProgramUniformMatrix3x4fv",
  "glProgramUniformMatrix4dv",
  "glProgramUniformMatrix4fv",
  "glProgramUniformMatrix4x2dv",
  "glProgramUniformMatrix4x2fv",
  "glProgramUniformMatrix4x3dv",
  "glProgramUniformMatrix4x3fv",
  "glUseProgramStages",
  "glValidateProgramPipeline",
];

pub const GL_ARB_shader_atomic_counters_COMMANDS: &[&str] = &["glGetActiveAtomicCounterBufferiv"];

pub const GL_ARB_shader_image_load_store_COMMANDS: &[&str] = &["glBindImageTexture", "glMemoryBarrier"];

pub const GL_ARB_shader_objects_COMMANDS: &[&str] = &[
  "glAttachObjectARB",
  "glCompileShaderARB",
  "glCreateProgramObjectARB",
  "glCreateShaderObjectARB",
  "glDeleteObjectARB",
  "glDetachObjectARB",
  "glGetActiveUniformARB",
  "glGetAttachedObjectsARB",
  "glGetHandleARB",
  "glGetInfoLogARB",
  "glGetObjectParameterfvARB",
  "glGetObjectParameterivARB",
  "glGetShaderSourceARB",
  "glGetUniformLocationARB",
  "glGetUniformfvARB",
  "glGetUniformivARB",
  "glLinkProgramARB",
  "glShaderSourceARB",
  "glUniform1fARB",
  "glUniform1fvARB",
  "glUniform1iARB",
  "glUniform1ivARB",
  "glUniform2fARB",
  "glUniform2fvARB",
  "glUniform2iARB",
  "glUniform2ivARB",
  "glUniform3fARB",
  "glUniform3fvARB",
  "glUniform3iARB",
  "glUniform3ivARB",
  "glUniform4fARB",
  "glUniform4fvARB",
  "glUniform4iARB",
  "glUniform4ivARB",
  "glUniformMatrix2fvARB",
  "glUniformMatrix3fvARB",
  "glUniformMatrix4fvARB",
  "glUseProgramObjectARB",
  "glValidateProgramARB",
];

pub const GL_ARB_shader_storage_buffer_object_COMMANDS: &[&str] = &["glShaderStorageBlockBinding"];

pub const GL_ARB_shader_subroutine_COMMANDS: &[&str] = &["glGetActiveSubroutineName", "glGetActiveSubroutineUniformName", "glGetActiveSubroutineUniformiv", "glGetProgramStageiv", "glGetSubroutineIndex", "glGetSubroutineUniformLocation", "glGetUniformSubroutineuiv", "glUniformSubroutinesuiv"];

pub const GL_ARB_shading_language_include_COMMANDS: &[&str] = &["glCompileShaderIncludeARB", "glDeleteNamedStringARB", "glGetNamedStringARB", "glGetNamedStringivARB", "glIsNamedStringARB", "glNamedStringARB"];

pub const GL_ARB_sparse_buffer_COMMANDS: &[&str] = &["glBufferPageCommitmentARB", "glNamedBufferPageCommitmentARB", "glNamedBufferPageCommitmentEXT"];

pub const GL_ARB_sparse_texture_COMMANDS: &[&str] = &["glTexPageCommitmentARB"];

pub const GL_ARB_sync_COMMANDS: &[&str] = &["glClientWaitSync", "glDeleteSync", "glFenceSync", "glGetInteger64v", "glGetSynciv", "glIsSync", "glWaitSync"];

pub const GL_ARB_tessellation_shader_COMMANDS: &[&str] = &["glPatchParameterfv", "glPatchParameteri"];

pub const GL_ARB_texture_barrier_COMMANDS: &[&str] = &["glTextureBarrier"];

pub const GL_ARB_texture_buffer_object_COMMANDS: &[&str] = &["glTexBufferARB"];

pub const GL_ARB_texture_buffer_range_COMMANDS: &[&str] = &["glTexBufferRange"];

pub const GL_ARB_texture_compression_COMMANDS: &[&str] = &["glCompressedTexImage1DARB", "glCompressedTexImage2DARB", "glCompressedTexImage3DARB", "glCompressedTexSubImage1DARB", "glCompressedTexSubImage2DARB", "glCompressedTexSubImage3DARB", "glGetCompressedTexImageARB"];

pub const GL_ARB_texture_multisample_COMMANDS: &[&str] = &["glGetMultisamplefv", "glSampleMaski", "glTexImage2DMultisample", "glTexImage3DMultisample"];

pub const GL_ARB_texture_storage_COMMANDS: &[&str] = &["glTexStorage1D", "glTexStorage2D", "glTexStorage3D"];

pub const GL_ARB_texture_storage_multisample_COMMANDS: &[&str] = &["glTexStorage2DMultisample", "glTexStorage3DMultisample"];

pub const GL_ARB_texture_view_COMMANDS: &[&str] = &["glTextureView"];

pub const GL_ARB_timer_query_COMMANDS: &[&str] = &["glGetQueryObjecti64v", "glGetQueryObjectui64v", "glQueryCounter"];

pub const GL_ARB_transform_feedback2_COMMANDS: &[&str] = &["glBindTransformFeedback", "glDeleteTransformFeedbacks", "glDrawTransformFeedback", "glGenTransformFeedbacks", "glIsTransformFeedback", "glPauseTransformFeedback", "glResumeTransformFeedback"];

pub const GL_ARB_transform_feedback3_COMMANDS: &[&str] = &["glBeginQueryIndexed", "glDrawTransformFeedbackStream", "glEndQueryIndexed", "glGetQueryIndexediv"];

pub const GL_ARB_transform_feedback_instanced_COMMANDS: &[&str] = &["glDrawTransformFeedbackInstanced", "glDrawTransformFeedbackStreamInstanced"];

pub const GL_ARB_transpose_matrix_COMMANDS: &[&str] = &["glLoadTransposeMatrixdARB", "glLoadTransposeMatrixfARB", "glMultTransposeMatrixdARB", "glMultTransposeMatrixfARB"];

pub const GL_ARB_uniform_buffer_object_COMMANDS: &[&str] = &["glBindBufferBase", "glBindBufferRange", "glGetActiveUniformBlockName", "glGetActiveUniformBlockiv", "glGetActiveUniformName", "glGetActiveUniformsiv", "glGetIntegeri_v", "glGetUniformBlockIndex", "glGetUniformIndices", "glUniformBlockBinding"];

pub const GL_ARB_vertex_array_object_COMMANDS: &[&str] = &["glBindVertexArray", "glDeleteVertexArrays", "glGenVertexArrays", "glIsVertexArray"];

pub const GL_ARB_vertex_attrib_64bit_COMMANDS: &[&str] = &["glGetVertexAttribLdv", "glVertexAttribL1d", "glVertexAttribL1dv", "glVertexAttribL2d", "glVertexAttribL2dv", "glVertexAttribL3d", "glVertexAttribL3dv", "glVertexAttribL4d", "glVertexAttribL4dv", "glVertexAttribLPointer"];

pub const GL_ARB_vertex_attrib_binding_COMMANDS: &[&str] = &["glBindVertexBuffer", "glVertexAttribBinding", "glVertexAttribFormat", "glVertexAttribIFormat", "glVertexAttribLFormat", "glVertexBindingDivisor"];

pub const GL_ARB_vertex_blend_COMMANDS: &[&str] = &["glVertexBlendARB", "glWeightPointerARB", "glWeightbvARB", "glWeightdvARB", "glWeightfvARB", "glWeightivARB", "glWeightsvARB", "glWeightubvARB", "glWeightuivARB", "glWeightusvARB"];

pub const GL_ARB_vertex_buffer_object_COMMANDS: &[&str] = &["glBindBufferARB", "glBufferDataARB", "glBufferSubDataARB", "glDeleteBuffersARB", "glGenBuffersARB", "glGetBufferParameterivARB", "glGetBufferPointervARB", "glGetBufferSubDataARB", "glIsBufferARB", "glMapBufferARB", "glUnmapBufferARB"];

pub const GL_ARB_vertex_program_COMMANDS: &[&str] = &[
  "glBindProgramARB",
  "glDeleteProgramsARB",
  "glDisableVertexAttribArrayARB",
  "glEnableVertexAttribArrayARB",
  "glGenProgramsARB",
  "glGetProgramEnvParameterdvARB",
  "glGetProgramEnvParameterfvARB",
  "glGetProgramLocalParameterdvARB",
  "glGetProgramLocalParameterfvARB",
  "glGetProgramStringARB",
  "glGetProgramivARB",
  "glGetVertexAttribPointervARB",
  "glGetVertexAttribdvARB",
  "glGetVertexAttribfvARB",
  "glGetVertexAttribivARB",
  "glIsProgramARB",
  "glProgramEnvParameter4dARB",
  "glProgramEnvParameter4dvARB",
  "glProgramEnvParameter4fARB",
  "glProgramEnvParameter4fvARB",
  "glProgramLocalParameter4dARB",
  "glProgramLocalParameter4dvARB",
  "glProgramLocalParameter4fARB",
  "glProgramLocalParameter4fvARB",
  "glProgramStringARB",
  "glVertexAttrib1dARB",
  "glVertexAttrib1dvARB",
  "glVertexAttrib1fARB",
  "glVertexAttrib1fvARB",
  "glVertexAttrib1sARB",
  "glVertexAttrib1svARB",
  "glVertexAttrib2dARB",
  "glVertexAttrib2dvARB",
  "glVertexAttrib2fARB",
  "glVertexAttrib2fvARB",
  "glVertexAttrib2sARB",
  "glVertexAttrib2svARB",
  "glVertexAttrib3dARB",
  "glVertexAttrib3dvARB",
  "glVertexAttrib3fARB",
  "glVertexAttrib3fvARB",
  "glVertexAttrib3sARB",
  "glVertexAttrib3svARB",
  "glVertexAttrib4NbvARB",
  "glVertexAttrib4NivARB",
  "glVertexAttrib4NsvARB",
  "glVertexAttrib4NubARB",
  "glVertexAttrib4NubvARB",
  "glVertexAttrib4NuivARB",
  "glVertexAttrib4NusvARB",
  "glVertexAttrib4bvARB",
  "glVertexAttrib4dARB",
  "glVertexAttrib4dvARB",
  "glVertexAttrib4fARB",
  "glVertexAttrib4fvARB",
  "glVertexAttrib4ivARB",
  "glVertexAttrib4sARB",
  "glVertexAttrib4svARB",
  "glVertexAttrib4ubvARB",
  "glVertexAttrib4uivARB",
  "glVertexAttrib4usvARB",
  "glVertexAttribPointerARB",
];

pub const GL_ARB_vertex_shader_COMMANDS: &[&str] = &[
  "glBindAttribLocationARB",
  "glDisableVertexAttribArrayARB",
  "glEnableVertexAttribArrayARB",
  "glGetActiveAttribARB",
  "glGetAttribLocationARB",
  "glGetVertexAttribPointervARB",
  "glGetVertexAttribdvARB",
  "glGetVertexAttribfvARB",
  "glGetVertexAttribivARB",
  "glVertexAttrib1dARB",
  "glVertexAttrib1dvARB",
  "glVertexAttrib1fARB",
  "glVertexAttrib1fvARB",
  "glVertexAttrib1sARB",
  "glVertexAttrib1svARB",
  "glVertexAttrib2dARB",
  "glVertexAttrib2dvARB",
  "glVertexAttrib2fARB",
  "glVertexAttrib2fvARB",
  "glVertexAttrib2sARB",
  "glVertexAttrib2svARB",
  "glVertexAttrib3dARB",
  "glVertexAttrib3dvARB",
  "glVertexAttrib3fARB",
  "glVertexAttrib3fvARB",
  "glVertexAttrib3sARB",
  "glVertexAttrib3svARB",
  "glVertexAttrib4NbvARB",
  "glVertexAttrib4NivARB",
  "glVertexAttrib4NsvARB",
  "glVertexAttrib4NubARB",
  "glVertexAttrib4NubvARB",
  "glVertexAttrib4NuivARB",
  "glVertexAttrib4NusvARB",
  "glVertexAttrib4bvARB",
  "glVertexAttrib4dARB",
  "glVertexAttrib4dvARB",
  "glVertexAttrib4fARB",
  "glVertexAttrib4fvARB",
  "glVertexAttrib4ivARB",
  "glVertexAttrib4sARB",
  "glVertexAttrib4svARB",
  "glVertexAttrib4ubvARB",
  "glVertexAttrib4uivARB",
  "glVertexAttrib4usvARB",
  "glVertexAttribPointerARB",
];

pub const GL_ARB_vertex_type_2_10_10_10_rev_COMMANDS: &[&str] =
  &["glColorP3ui", "glColorP3uiv", "glColorP4ui", "glColorP4uiv", "glMultiTexCoordP1ui", "glMultiTexCoordP1uiv", "glMultiTexCoordP2ui", "glMultiTexCoordP2uiv", "glMultiTexCoordP3ui", "glMultiTexCoordP3uiv", "glMultiTexCoordP4ui", "glMultiTexCoordP4uiv", "glNormalP3ui", "glNormalP3uiv", "glSecondaryColorP3ui", "glSecondaryColorP3uiv", "glTexCoordP1ui", "glTexCoordP1uiv", "glTexCoordP2ui", "glTexCoordP2uiv", "glTexCoordP3ui", "glTexCoordP3uiv", "glTexCoordP4ui", "glTexCoordP4uiv", "glVertexAttribP1ui", "glVertexAttribP1uiv", "glVertexAttribP2ui", "glVertexAttribP2uiv", "glVertexAttribP3ui", "glVertexAttribP3uiv", "glVertexAttribP4ui", "glVertexAttribP4uiv", "glVertexP2ui", "glVertexP2uiv", "glVertexP3ui", "glVertexP3uiv", "glVertexP4ui", "glVertexP4uiv"];

pub const GL_ARB_viewport_array_COMMANDS: &[&str] = &["glDepthRangeArraydvNV", "glDepthRangeArrayv", "glDepthRangeIndexed", "glDepthRangeIndexeddNV", "glGetDoublei_v", "glGetFloati_v", "glScissorArrayv", "glScissorIndexed", "glScissorIndexedv", "glViewportArrayv", "glViewportIndexedf", "glViewportIndexedfv"];

pub const GL_ARB_window_pos_COMMANDS: &[&str] = &["glWindowPos2dARB", "glWindowPos2dvARB", "glWindowPos2fARB", "glWindowPos2fvARB", "glWindowPos2iARB", "glWindowPos2ivARB", "glWindowPos2sARB", "glWindowPos2svARB", "glWindowPos3dARB", "glWindowPos3dvARB", "glWindowPos3fARB", "glWindowPos3fvARB", "glWindowPos3iARB", "glWindowPos3ivARB", "glWindowPos3sARB", "glWindowPos3svARB"];

pub const GL_ATI_draw_buffers_COMMANDS: &[&str] = &["glDrawBuffersATI"];

pub const GL_ATI_element_array_COMMANDS: &[&str] = &["glDrawElementArrayATI", "glDrawRangeElementArrayATI", "glElementPointerATI"];

pub const GL_ATI_envmap_bumpmap_COMMANDS: &[&str] = &["glGetTexBumpParameterfvATI", "glGetTexBumpParameterivATI", "glTexBumpParameterfvATI", "glTexBumpParameterivATI"];

pub const GL_ATI_fragment_shader_COMMANDS: &[&str] = &["glAlphaFragmentOp1ATI", "glAlphaFragmentOp2ATI", "glAlphaFragmentOp3ATI", "glBeginFragmentShaderATI", "glBindFragmentShaderATI", "glColorFragmentOp1ATI", "glColorFragmentOp2ATI", "glColorFragmentOp3ATI", "glDeleteFragmentShaderATI", "glEndFragmentShaderATI", "glGenFragmentShadersATI", "glPassTexCoordATI", "glSampleMapATI", "glSetFragmentShaderConstantATI"];

pub const GL_ATI_map_object_buffer_COMMANDS: &[&str] = &["glMapObjectBufferATI", "glUnmapObjectBufferATI"];

pub const GL_ATI_pn_triangles_COMMANDS: &[&str] = &["glPNTrianglesfATI", "glPNTrianglesiATI"];

pub const GL_ATI_separate_stencil_COMMANDS: &[&str] = &["glStencilFuncSeparateATI", "glStencilOpSeparateATI"];

pub const GL_ATI_vertex_array_object_COMMANDS: &[&str] = &["glArrayObjectATI", "glFreeObjectBufferATI", "glGetArrayObjectfvATI", "glGetArrayObjectivATI", "glGetObjectBufferfvATI", "glGetObjectBufferivATI", "glGetVariantArrayObjectfvATI", "glGetVariantArrayObjectivATI", "glIsObjectBufferATI", "glNewObjectBufferATI", "glUpdateObjectBufferATI", "glVariantArrayObjectATI"];

pub const GL_ATI_vertex_attrib_array_object_COMMANDS: &[&str] = &["glGetVertexAttribArrayObjectfvATI", "glGetVertexAttribArrayObjectivATI", "glVertexAttribArrayObjectATI"];

pub const GL_ATI_vertex_streams_COMMANDS: &[&str] = &[
  "glClientActiveVertexStreamATI",
  "glNormalStream3bATI",
  "glNormalStream3bvATI",
  "glNormalStream3dATI",
  "glNormalStream3dvATI",
  "glNormalStream3fATI",
  "glNormalStream3fvATI",
  "glNormalStream3iATI",
  "glNormalStream3ivATI",
  "glNormalStream3sATI",
  "glNormalStream3svATI",
  "glVertexBlendEnvfATI",
  "glVertexBlendEnviATI",
  "glVertexStream1dATI",
  "glVertexStream1dvATI",
  "glVertexStream1fATI",
  "glVertexStream1fvATI",
  "glVertexStream1iATI",
  "glVertexStream1ivATI",
  "glVertexStream1sATI",
  "glVertexStream1svATI",
  "glVertexStream2dATI",
  "glVertexStream2dvATI",
  "glVertexStream2fATI",
  "glVertexStream2fvATI",
  "glVertexStream2iATI",
  "glVertexStream2ivATI",
  "glVertexStream2sATI",
  "glVertexStream2svATI",
  "glVertexStream3dATI",
  "glVertexStream3dvATI",
  "glVertexStream3fATI",
  "glVertexStream3fvATI",
  "glVertexStream3iATI",
  "glVertexStream3ivATI",
  "glVertexStream3sATI",
  "glVertexStream3svATI",
  "glVertexStream4dATI",
  "glVertexStream4dvATI",
  "glVertexStream4fATI",
  "glVertexStream4fvATI",
  "glVertexStream4iATI",
  "glVertexStream4ivATI",
  "glVertexStream4sATI",
  "glVertexStream4svATI",
];

pub const GL_EXT_EGL_image_storage_COMMANDS: &[&str] = &["glEGLImageTargetTexStorageEXT", "glEGLImageTargetTextureStorageEXT"];

pub const GL_EXT_base_instance_COMMANDS: &[&str] = &["glDrawArraysInstancedBaseInstanceEXT", "glDrawElementsInstancedBaseInstanceEXT", "glDrawElementsInstancedBaseVertexBaseInstanceEXT"];

pub const GL_EXT_bindable_uniform_COMMANDS: &[&str] = &["glGetUniformBufferSizeEXT", "glGetUniformOffsetEXT", "glUniformBufferEXT"];

pub const GL_EXT_blend_color_COMMANDS: &[&str] = &["glBlendColorEXT"];

pub const GL_EXT_blend_equation_separate_COMMANDS: &[&str] = &["glBlendEquationSeparateEXT"];

pub const GL_EXT_blend_func_extended_COMMANDS: &[&str] = &["glBindFragDataLocationEXT", "glBindFragDataLocationIndexedEXT", "glGetFragDataIndexEXT", "glGetProgramResourceLocationIndexEXT"];

pub const GL_EXT_blend_func_separate_COMMANDS: &[&str] = &["glBlendFuncSeparateEXT"];

pub const GL_EXT_blend_minmax_COMMANDS: &[&str] = &["glBlendEquationEXT"];

pub const GL_EXT_buffer_storage_COMMANDS: &[&str] = &["glBufferStorageEXT"];

pub const GL_EXT_clear_texture_COMMANDS: &[&str] = &["glClearTexImageEXT", "glClearTexSubImageEXT"];

pub const GL_EXT_clip_control_COMMANDS: &[&str] = &["glClipControlEXT"];

pub const GL_EXT_color_subtable_COMMANDS: &[&str] = &["glColorSubTableEXT", "glCopyColorSubTableEXT"];

pub const GL_EXT_compiled_vertex_array_COMMANDS: &[&str] = &["glLockArraysEXT", "glUnlockArraysEXT"];

pub const GL_EXT_convolution_COMMANDS: &[&str] = &["glConvolutionFilter1DEXT", "glConvolutionFilter2DEXT", "glConvolutionParameterfEXT", "glConvolutionParameterfvEXT", "glConvolutionParameteriEXT", "glConvolutionParameterivEXT", "glCopyConvolutionFilter1DEXT", "glCopyConvolutionFilter2DEXT", "glGetConvolutionFilterEXT", "glGetConvolutionParameterfvEXT", "glGetConvolutionParameterivEXT", "glGetSeparableFilterEXT", "glSeparableFilter2DEXT"];

pub const GL_EXT_coordinate_frame_COMMANDS: &[&str] = &["glBinormal3bEXT", "glBinormal3bvEXT", "glBinormal3dEXT", "glBinormal3dvEXT", "glBinormal3fEXT", "glBinormal3fvEXT", "glBinormal3iEXT", "glBinormal3ivEXT", "glBinormal3sEXT", "glBinormal3svEXT", "glBinormalPointerEXT", "glTangent3bEXT", "glTangent3bvEXT", "glTangent3dEXT", "glTangent3dvEXT", "glTangent3fEXT", "glTangent3fvEXT", "glTangent3iEXT", "glTangent3ivEXT", "glTangent3sEXT", "glTangent3svEXT", "glTangentPointerEXT"];

pub const GL_EXT_copy_image_COMMANDS: &[&str] = &["glCopyImageSubDataEXT"];

pub const GL_EXT_copy_texture_COMMANDS: &[&str] = &["glCopyTexImage1DEXT", "glCopyTexImage2DEXT", "glCopyTexSubImage1DEXT", "glCopyTexSubImage2DEXT", "glCopyTexSubImage3DEXT"];

pub const GL_EXT_cull_vertex_COMMANDS: &[&str] = &["glCullParameterdvEXT", "glCullParameterfvEXT"];

pub const GL_EXT_debug_label_COMMANDS: &[&str] = &["glGetObjectLabelEXT", "glLabelObjectEXT"];

pub const GL_EXT_debug_marker_COMMANDS: &[&str] = &["glInsertEventMarkerEXT", "glPopGroupMarkerEXT", "glPushGroupMarkerEXT"];

pub const GL_EXT_depth_bounds_test_COMMANDS: &[&str] = &["glDepthBoundsEXT"];

pub const GL_EXT_direct_state_access_COMMANDS: &[&str] = &[
  "glBindMultiTextureEXT",
  "glCheckNamedFramebufferStatusEXT",
  "glClearNamedBufferDataEXT",
  "glClearNamedBufferSubDataEXT",
  "glClientAttribDefaultEXT",
  "glCompressedMultiTexImage1DEXT",
  "glCompressedMultiTexImage2DEXT",
  "glCompressedMultiTexImage3DEXT",
  "glCompressedMultiTexSubImage1DEXT",
  "glCompressedMultiTexSubImage2DEXT",
  "glCompressedMultiTexSubImage3DEXT",
  "glCompressedTextureImage1DEXT",
  "glCompressedTextureImage2DEXT",
  "glCompressedTextureImage3DEXT",
  "glCompressedTextureSubImage1DEXT",
  "glCompressedTextureSubImage2DEXT",
  "glCompressedTextureSubImage3DEXT",
  "glCopyMultiTexImage1DEXT",
  "glCopyMultiTexImage2DEXT",
  "glCopyMultiTexSubImage1DEXT",
  "glCopyMultiTexSubImage2DEXT",
  "glCopyMultiTexSubImage3DEXT",
  "glCopyTextureImage1DEXT",
  "glCopyTextureImage2DEXT",
  "glCopyTextureSubImage1DEXT",
  "glCopyTextureSubImage2DEXT",
  "glCopyTextureSubImage3DEXT",
  "glDisableClientStateIndexedEXT",
  "glDisableClientStateiEXT",
  "glDisableIndexedEXT",
  "glDisableVertexArrayAttribEXT",
  "glDisableVertexArrayEXT",
  "glEnableClientStateIndexedEXT",
  "glEnableClientStateiEXT",
  "glEnableIndexedEXT",
  "glEnableVertexArrayAttribEXT",
  "glEnableVertexArrayEXT",
  "glFlushMappedNamedBufferRangeEXT",
  "glFramebufferDrawBufferEXT",
  "glFramebufferDrawBuffersEXT",
  "glFramebufferReadBufferEXT",
  "glGenerateMultiTexMipmapEXT",
  "glGenerateTextureMipmapEXT",
  "glGetBooleanIndexedvEXT",
  "glGetCompressedMultiTexImageEXT",
  "glGetCompressedTextureImageEXT",
  "glGetDoubleIndexedvEXT",
  "glGetDoublei_vEXT",
  "glGetFloatIndexedvEXT",
  "glGetFloati_vEXT",
  "glGetFramebufferParameterivEXT",
  "glGetIntegerIndexedvEXT",
  "glGetMultiTexEnvfvEXT",
  "glGetMultiTexEnvivEXT",
  "glGetMultiTexGendvEXT",
  "glGetMultiTexGenfvEXT",
  "glGetMultiTexGenivEXT",
  "glGetMultiTexImageEXT",
  "glGetMultiTexLevelParameterfvEXT",
  "glGetMultiTexLevelParameterivEXT",
  "glGetMultiTexParameterIivEXT",
  "glGetMultiTexParameterIuivEXT",
  "glGetMultiTexParameterfvEXT",
  "glGetMultiTexParameterivEXT",
  "glGetNamedBufferParameterivEXT",
  "glGetNamedBufferPointervEXT",
  "glGetNamedBufferSubDataEXT",
  "glGetNamedFramebufferAttachmentParameterivEXT",
  "glGetNamedFramebufferParameterivEXT",
  "glGetNamedProgramLocalParameterIivEXT",
  "glGetNamedProgramLocalParameterIuivEXT",
  "glGetNamedProgramLocalParameterdvEXT",
  "glGetNamedProgramLocalParameterfvEXT",
  "glGetNamedProgramStringEXT",
  "glGetNamedProgramivEXT",
  "glGetNamedRenderbufferParameterivEXT",
  "glGetPointerIndexedvEXT",
  "glGetPointeri_vEXT",
  "glGetTextureImageEXT",
  "glGetTextureLevelParameterfvEXT",
  "glGetTextureLevelParameterivEXT",
  "glGetTextureParameterIivEXT",
  "glGetTextureParameterIuivEXT",
  "glGetTextureParameterfvEXT",
  "glGetTextureParameterivEXT",
  "glGetVertexArrayIntegeri_vEXT",
  "glGetVertexArrayIntegervEXT",
  "glGetVertexArrayPointeri_vEXT",
  "glGetVertexArrayPointervEXT",
  "glIsEnabledIndexedEXT",
  "glMapNamedBufferEXT",
  "glMapNamedBufferRangeEXT",
  "glMatrixFrustumEXT",
  "glMatrixLoadIdentityEXT",
  "glMatrixLoadTransposedEXT",
  "glMatrixLoadTransposefEXT",
  "glMatrixLoaddEXT",
  "glMatrixLoadfEXT",
  "glMatrixMultTransposedEXT",
  "glMatrixMultTransposefEXT",
  "glMatrixMultdEXT",
  "glMatrixMultfEXT",
  "glMatrixOrthoEXT",
  "glMatrixPopEXT",
  "glMatrixPushEXT",
  "glMatrixRotatedEXT",
  "glMatrixRotatefEXT",
  "glMatrixScaledEXT",
  "glMatrixScalefEXT",
  "glMatrixTranslatedEXT",
  "glMatrixTranslatefEXT",
  "glMultiTexBufferEXT",
  "glMultiTexCoordPointerEXT",
  "glMultiTexEnvfEXT",
  "glMultiTexEnvfvEXT",
  "glMultiTexEnviEXT",
  "glMultiTexEnvivEXT",
  "glMultiTexGendEXT",
  "glMultiTexGendvEXT",
  "glMultiTexGenfEXT",
  "glMultiTexGenfvEXT",
  "glMultiTexGeniEXT",
  "glMultiTexGenivEXT",
  "glMultiTexImage1DEXT",
  "glMultiTexImage2DEXT",
  "glMultiTexImage3DEXT",
  "glMultiTexParameterIivEXT",
  "glMultiTexParameterIuivEXT",
  "glMultiTexParameterfEXT",
  "glMultiTexParameterfvEXT",
  "glMultiTexParameteriEXT",
  "glMultiTexParameterivEXT",
  "glMultiTexRenderbufferEXT",
  "glMultiTexSubImage1DEXT",
  "glMultiTexSubImage2DEXT",
  "glMultiTexSubImage3DEXT",
  "glNamedBufferDataEXT",
  "glNamedBufferStorageEXT",
  "glNamedBufferSubDataEXT",
  "glNamedCopyBufferSubDataEXT",
  "glNamedFramebufferParameteriEXT",
  "glNamedFramebufferRenderbufferEXT",
  "glNamedFramebufferTexture1DEXT",
  "glNamedFramebufferTexture2DEXT",
  "glNamedFramebufferTexture3DEXT",
  "glNamedFramebufferTextureEXT",
  "glNamedFramebufferTextureFaceEXT",
  "glNamedFramebufferTextureLayerEXT",
  "glNamedProgramLocalParameter4dEXT",
  "glNamedProgramLocalParameter4dvEXT",
  "glNamedProgramLocalParameter4fEXT",
  "glNamedProgramLocalParameter4fvEXT",
  "glNamedProgramLocalParameterI4iEXT",
  "glNamedProgramLocalParameterI4ivEXT",
  "glNamedProgramLocalParameterI4uiEXT",
  "glNamedProgramLocalParameterI4uivEXT",
  "glNamedProgramLocalParameters4fvEXT",
  "glNamedProgramLocalParametersI4ivEXT",
  "glNamedProgramLocalParametersI4uivEXT",
  "glNamedProgramStringEXT",
  "glNamedRenderbufferStorageEXT",
  "glNamedRenderbufferStorageMultisampleCoverageEXT",
  "glNamedRenderbufferStorageMultisampleEXT",
  "glProgramUniform1dEXT",
  "glProgramUniform1dvEXT",
  "glProgramUniform1fEXT",
  "glProgramUniform1fvEXT",
  "glProgramUniform1iEXT",
  "glProgramUniform1ivEXT",
  "glProgramUniform1uiEXT",
  "glProgramUniform1uivEXT",
  "glProgramUniform2dEXT",
  "glProgramUniform2dvEXT",
  "glProgramUniform2fEXT",
  "glProgramUniform2fvEXT",
  "glProgramUniform2iEXT",
  "glProgramUniform2ivEXT",
  "glProgramUniform2uiEXT",
  "glProgramUniform2uivEXT",
  "glProgramUniform3dEXT",
  "glProgramUniform3dvEXT",
  "glProgramUniform3fEXT",
  "glProgramUniform3fvEXT",
  "glProgramUniform3iEXT",
  "glProgramUniform3ivEXT",
  "glProgramUniform3uiEXT",
  "glProgramUniform3uivEXT",
  "glProgramUniform4dEXT",
  "glProgramUniform4dvEXT",
  "glProgramUniform4fEXT",
  "glProgramUniform4fvEXT",
  "glProgramUniform4iEXT",
  "glProgramUniform4ivEXT",
  "glProgramUniform4uiEXT",
  "glProgramUniform4uivEXT",
  "glProgramUniformMatrix2dvEXT",
  "glProgramUniformMatrix2fvEXT",
  "glProgramUniformMatrix2x3dvEXT",
  "glProgramUniformMatrix2x3fvEXT",
  "glProgramUniformMatrix2x4dvEXT",
  "glProgramUniformMatrix2x4fvEXT",
  "glProgramUniformMatrix3dvEXT",
  "glProgramUniformMatrix3fvEXT",
  "glProgramUniformMatrix3x2dvEXT",
  "glProgramUniformMatrix3x2fvEXT",
  "glProgramUniformMatrix3x4dvEXT",
  "glProgramUniformMatrix3x4fvEXT",
  "glProgramUniformMatrix4dvEXT",
  "glProgramUniformMatrix4fvEXT",
  "glProgramUniformMatrix4x2dvEXT",
  "glProgramUniformMatrix4x2fvEXT",
  "glProgramUniformMatrix4x3dvEXT",
  "glProgramUniformMatrix4x3fvEXT",
  "glPushClientAttribDefaultEXT",
  "glTextureBufferEXT",
  "glTextureBufferRangeEXT",
  "glTextureImage1DEXT",
  "glTextureImage2DEXT",
  "glTextureImage3DEXT",
  "glTexturePageCommitmentEXT",
  "glTextureParameterIivEXT",
  "glTextureParameterIuivEXT",
  "glTextureParameterfEXT",
  "glTextureParameterfvEXT",
  "glTextureParameteriEXT",
  "glTextureParameterivEXT",
  "glTextureRenderbufferEXT",
  "glTextureStorage1DEXT",
  "glTextureStorage2DEXT",
  "glTextureStorage2DMultisampleEXT",
  "glTextureStorage3DEXT",
  "glTextureStorage3DMultisampleEXT",
  "glTextureSubImage1DEXT",
  "glTextureSubImage2DEXT",
  "glTextureSubImage3DEXT",
  "glUnmapNamedBufferEXT",
  "glVertexArrayBindVertexBufferEXT",
  "glVertexArrayColorOffsetEXT",
  "glVertexArrayEdgeFlagOffsetEXT",
  "glVertexArrayFogCoordOffsetEXT",
  "glVertexArrayIndexOffsetEXT",
  "glVertexArrayMultiTexCoordOffsetEXT",
  "glVertexArrayNormalOffsetEXT",
  "glVertexArraySecondaryColorOffsetEXT",
  "glVertexArrayTexCoordOffsetEXT",
  "glVertexArrayVertexAttribBindingEXT",
  "glVertexArrayVertexAttribDivisorEXT",
  "glVertexArrayVertexAttribFormatEXT",
  "glVertexArrayVertexAttribIFormatEXT",
  "glVertexArrayVertexAttribIOffsetEXT",
  "glVertexArrayVertexAttribLFormatEXT",
  "glVertexArrayVertexAttribLOffsetEXT",
  "glVertexArrayVertexAttribOffsetEXT",
  "glVertexArrayVertexBindingDivisorEXT",
  "glVertexArrayVertexOffsetEXT",
];

pub const GL_EXT_discard_framebuffer_COMMANDS: &[&str] = &["glDiscardFramebufferEXT"];

pub const GL_EXT_disjoint_timer_query_COMMANDS: &[&str] = &["glBeginQueryEXT", "glDeleteQueriesEXT", "glEndQueryEXT", "glGenQueriesEXT", "glGetInteger64vEXT", "glGetQueryObjecti64vEXT", "glGetQueryObjectivEXT", "glGetQueryObjectui64vEXT", "glGetQueryObjectuivEXT", "glGetQueryivEXT", "glIsQueryEXT", "glQueryCounterEXT"];

pub const GL_EXT_draw_buffers_COMMANDS: &[&str] = &["glDrawBuffersEXT"];

pub const GL_EXT_draw_buffers2_COMMANDS: &[&str] = &["glColorMaskIndexedEXT", "glDisableIndexedEXT", "glEnableIndexedEXT", "glGetBooleanIndexedvEXT", "glGetIntegerIndexedvEXT", "glIsEnabledIndexedEXT"];

pub const GL_EXT_draw_buffers_indexed_COMMANDS: &[&str] = &["glBlendEquationSeparateiEXT", "glBlendEquationiEXT", "glBlendFuncSeparateiEXT", "glBlendFunciEXT", "glColorMaskiEXT", "glDisableiEXT", "glEnableiEXT", "glIsEnablediEXT"];

pub const GL_EXT_draw_elements_base_vertex_COMMANDS: &[&str] = &["glDrawElementsBaseVertexEXT", "glDrawElementsInstancedBaseVertexEXT", "glDrawRangeElementsBaseVertexEXT", "glMultiDrawElementsBaseVertexEXT"];

pub const GL_EXT_draw_instanced_COMMANDS: &[&str] = &["glDrawArraysInstancedEXT", "glDrawElementsInstancedEXT"];

pub const GL_EXT_draw_range_elements_COMMANDS: &[&str] = &["glDrawRangeElementsEXT"];

pub const GL_EXT_draw_transform_feedback_COMMANDS: &[&str] = &["glDrawTransformFeedbackEXT", "glDrawTransformFeedbackInstancedEXT"];

pub const GL_EXT_external_buffer_COMMANDS: &[&str] = &["glBufferStorageExternalEXT", "glNamedBufferStorageExternalEXT"];

pub const GL_EXT_fog_coord_COMMANDS: &[&str] = &["glFogCoordPointerEXT", "glFogCoorddEXT", "glFogCoorddvEXT", "glFogCoordfEXT", "glFogCoordfvEXT"];

pub const GL_EXT_framebuffer_blit_COMMANDS: &[&str] = &["glBlitFramebufferEXT"];

pub const GL_EXT_framebuffer_multisample_COMMANDS: &[&str] = &["glRenderbufferStorageMultisampleEXT"];

pub const GL_EXT_framebuffer_object_COMMANDS: &[&str] = &["glBindFramebufferEXT", "glBindRenderbufferEXT", "glCheckFramebufferStatusEXT", "glDeleteFramebuffersEXT", "glDeleteRenderbuffersEXT", "glFramebufferRenderbufferEXT", "glFramebufferTexture1DEXT", "glFramebufferTexture2DEXT", "glFramebufferTexture3DEXT", "glGenFramebuffersEXT", "glGenRenderbuffersEXT", "glGenerateMipmapEXT", "glGetFramebufferAttachmentParameterivEXT", "glGetRenderbufferParameterivEXT", "glIsFramebufferEXT", "glIsRenderbufferEXT", "glRenderbufferStorageEXT"];

pub const GL_EXT_geometry_shader_COMMANDS: &[&str] = &["glFramebufferTextureEXT"];

pub const GL_EXT_geometry_shader4_COMMANDS: &[&str] = &["glProgramParameteriEXT"];

pub const GL_EXT_gpu_program_parameters_COMMANDS: &[&str] = &["glProgramEnvParameters4fvEXT", "glProgramLocalParameters4fvEXT"];

pub const GL_EXT_gpu_shader4_COMMANDS: &[&str] = &["glBindFragDataLocationEXT", "glGetFragDataLocationEXT", "glGetUniformuivEXT", "glUniform1uiEXT", "glUniform1uivEXT", "glUniform2uiEXT", "glUniform2uivEXT", "glUniform3uiEXT", "glUniform3uivEXT", "glUniform4uiEXT", "glUniform4uivEXT"];

pub const GL_EXT_histogram_COMMANDS: &[&str] = &["glGetHistogramEXT", "glGetHistogramParameterfvEXT", "glGetHistogramParameterivEXT", "glGetMinmaxEXT", "glGetMinmaxParameterfvEXT", "glGetMinmaxParameterivEXT", "glHistogramEXT", "glMinmaxEXT", "glResetHistogramEXT", "glResetMinmaxEXT"];

pub const GL_EXT_index_func_COMMANDS: &[&str] = &["glIndexFuncEXT"];

pub const GL_EXT_index_material_COMMANDS: &[&str] = &["glIndexMaterialEXT"];

pub const GL_EXT_instanced_arrays_COMMANDS: &[&str] = &["glDrawArraysInstancedEXT", "glDrawElementsInstancedEXT", "glVertexAttribDivisorEXT"];

pub const GL_EXT_light_texture_COMMANDS: &[&str] = &["glApplyTextureEXT", "glTextureLightEXT", "glTextureMaterialEXT"];

pub const GL_EXT_map_buffer_range_COMMANDS: &[&str] = &["glFlushMappedBufferRangeEXT", "glMapBufferRangeEXT"];

pub const GL_EXT_memory_object_COMMANDS: &[&str] = &["glBufferStorageMemEXT", "glCreateMemoryObjectsEXT", "glDeleteMemoryObjectsEXT", "glGetMemoryObjectParameterivEXT", "glGetUnsignedBytei_vEXT", "glGetUnsignedBytevEXT", "glIsMemoryObjectEXT", "glMemoryObjectParameterivEXT", "glNamedBufferStorageMemEXT", "glTexStorageMem1DEXT", "glTexStorageMem2DEXT", "glTexStorageMem2DMultisampleEXT", "glTexStorageMem3DEXT", "glTexStorageMem3DMultisampleEXT", "glTextureStorageMem1DEXT", "glTextureStorageMem2DEXT", "glTextureStorageMem2DMultisampleEXT", "glTextureStorageMem3DEXT", "glTextureStorageMem3DMultisampleEXT"];

pub const GL_EXT_memory_object_fd_COMMANDS: &[&str] = &["glImportMemoryFdEXT"];

pub const GL_EXT_memory_object_win32_COMMANDS: &[&str] = &["glImportMemoryWin32HandleEXT", "glImportMemoryWin32NameEXT"];

pub const GL_EXT_multi_draw_arrays_COMMANDS: &[&str] = &["glMultiDrawArraysEXT", "glMultiDrawElementsEXT"];

pub const GL_EXT_multi_draw_indirect_COMMANDS: &[&str] = &["glMultiDrawArraysIndirectEXT", "glMultiDrawElementsIndirectEXT"];

pub const GL_EXT_multisample_COMMANDS: &[&str] = &["glSampleMaskEXT", "glSamplePatternEXT"];

pub const GL_EXT_multisampled_render_to_texture_COMMANDS: &[&str] = &["glFramebufferTexture2DMultisampleEXT", "glRenderbufferStorageMultisampleEXT"];

pub const GL_EXT_multiview_draw_buffers_COMMANDS: &[&str] = &["glDrawBuffersIndexedEXT", "glGetIntegeri_vEXT", "glReadBufferIndexedEXT"];

pub const GL_EXT_occlusion_query_boolean_COMMANDS: &[&str] = &["glBeginQueryEXT", "glDeleteQueriesEXT", "glEndQueryEXT", "glGenQueriesEXT", "glGetQueryObjectuivEXT", "glGetQueryivEXT", "glIsQueryEXT"];

pub const GL_EXT_paletted_texture_COMMANDS: &[&str] = &["glColorTableEXT", "glGetColorTableEXT", "glGetColorTableParameterfvEXT", "glGetColorTableParameterivEXT"];

pub const GL_EXT_pixel_transform_COMMANDS: &[&str] = &["glGetPixelTransformParameterfvEXT", "glGetPixelTransformParameterivEXT", "glPixelTransformParameterfEXT", "glPixelTransformParameterfvEXT", "glPixelTransformParameteriEXT", "glPixelTransformParameterivEXT"];

pub const GL_EXT_point_parameters_COMMANDS: &[&str] = &["glPointParameterfEXT", "glPointParameterfvEXT"];

pub const GL_EXT_polygon_offset_COMMANDS: &[&str] = &["glPolygonOffsetEXT"];

pub const GL_EXT_polygon_offset_clamp_COMMANDS: &[&str] = &["glPolygonOffsetClampEXT"];

pub const GL_EXT_primitive_bounding_box_COMMANDS: &[&str] = &["glPrimitiveBoundingBoxEXT"];

pub const GL_EXT_provoking_vertex_COMMANDS: &[&str] = &["glProvokingVertexEXT"];

pub const GL_EXT_raster_multisample_COMMANDS: &[&str] = &["glRasterSamplesEXT"];

pub const GL_EXT_robustness_COMMANDS: &[&str] = &["glGetGraphicsResetStatusEXT", "glGetnUniformfvEXT", "glGetnUniformivEXT", "glReadnPixelsEXT"];

pub const GL_EXT_secondary_color_COMMANDS: &[&str] = &["glSecondaryColor3bEXT", "glSecondaryColor3bvEXT", "glSecondaryColor3dEXT", "glSecondaryColor3dvEXT", "glSecondaryColor3fEXT", "glSecondaryColor3fvEXT", "glSecondaryColor3iEXT", "glSecondaryColor3ivEXT", "glSecondaryColor3sEXT", "glSecondaryColor3svEXT", "glSecondaryColor3ubEXT", "glSecondaryColor3ubvEXT", "glSecondaryColor3uiEXT", "glSecondaryColor3uivEXT", "glSecondaryColor3usEXT", "glSecondaryColor3usvEXT", "glSecondaryColorPointerEXT"];

pub const GL_EXT_semaphore_COMMANDS: &[&str] = &["glDeleteSemaphoresEXT", "glGenSemaphoresEXT", "glGetSemaphoreParameterui64vEXT", "glGetUnsignedBytei_vEXT", "glGetUnsignedBytevEXT", "glIsSemaphoreEXT", "glSemaphoreParameterui64vEXT", "glSignalSemaphoreEXT", "glWaitSemaphoreEXT"];

pub const GL_EXT_semaphore_fd_COMMANDS: &[&str] = &["glImportSemaphoreFdEXT"];

pub const GL_EXT_semaphore_win32_COMMANDS: &[&str] = &["glImportSemaphoreWin32HandleEXT", "glImportSemaphoreWin32NameEXT"];

pub const GL_EXT_separate_shader_objects_COMMANDS: &[&str] = &[
  "glActiveProgramEXT",
  "glActiveShaderProgramEXT",
  "glBindProgramPipelineEXT",
  "glCreateShaderProgramEXT",
  "glCreateShaderProgramvEXT",
  "glDeleteProgramPipelinesEXT",
  "glGenProgramPipelinesEXT",
  "glGetProgramPipelineInfoLogEXT",
  "glGetProgramPipelineivEXT",
  "glIsProgramPipelineEXT",
  "glProgramParameteriEXT",
  "glProgramUniform1fEXT",
  "glProgramUniform1fvEXT",
  "glProgramUniform1iEXT",
  "glProgramUniform1ivEXT",
  "glProgramUniform1uiEXT",
  "glProgramUniform1uivEXT",
  "glProgramUniform2fEXT",
  "glProgramUniform2fvEXT",
  "glProgramUniform2iEXT",
  "glProgramUniform2ivEXT",
  "glProgramUniform2uiEXT",
  "glProgramUniform2uivEXT",
  "glProgramUniform3fEXT",
  "glProgramUniform3fvEXT",
  "glProgramUniform3iEXT",
  "glProgramUniform3ivEXT",
  "glProgramUniform3uiEXT",
  "glProgramUniform3uivEXT",
  "glProgramUniform4fEXT",
  "glProgramUniform4fvEXT",
  "glProgramUniform4iEXT",
  "glProgramUniform4ivEXT",
  "glProgramUniform4uiEXT",
  "glProgramUniform4uivEXT",
  "glProgramUniformMatrix2fvEXT",
  "glProgramUniformMatrix2x3fvEXT",
  "glProgramUniformMatrix2x4fvEXT",
  "glProgramUniformMatrix3fvEXT",
  "glProgramUniformMatrix3x2fvEXT",
  "glProgramUniformMatrix3x4fvEXT",
  "glProgramUniformMatrix4fvEXT",
  "glProgramUniformMatrix4x2fvEXT",
  "glProgramUniformMatrix4x3fvEXT",
  "glUseProgramStagesEXT",
  "glUseShaderProgramEXT",
  "glValidateProgramPipelineEXT",
];

pub const GL_EXT_shader_framebuffer_fetch_non_coherent_COMMANDS: &[&str] = &["glFramebufferFetchBarrierEXT"];

pub const GL_EXT_shader_image_load_store_COMMANDS: &[&str] = &["glBindImageTextureEXT", "glMemoryBarrierEXT"];

pub const GL_EXT_shader_pixel_local_storage2_COMMANDS: &[&str] = &["glClearPixelLocalStorageuiEXT", "glFramebufferPixelLocalStorageSizeEXT", "glGetFramebufferPixelLocalStorageSizeEXT"];

pub const GL_EXT_sparse_texture_COMMANDS: &[&str] = &["glTexPageCommitmentEXT"];

pub const GL_EXT_stencil_clear_tag_COMMANDS: &[&str] = &["glStencilClearTagEXT"];

pub const GL_EXT_stencil_two_side_COMMANDS: &[&str] = &["glActiveStencilFaceEXT"];

pub const GL_EXT_subtexture_COMMANDS: &[&str] = &["glTexSubImage1DEXT", "glTexSubImage2DEXT"];

pub const GL_EXT_tessellation_shader_COMMANDS: &[&str] = &["glPatchParameteriEXT"];

pub const GL_EXT_texture3D_COMMANDS: &[&str] = &["glTexImage3DEXT", "glTexSubImage3DEXT"];

pub const GL_EXT_texture_array_COMMANDS: &[&str] = &["glFramebufferTextureLayerEXT"];

pub const GL_EXT_texture_border_clamp_COMMANDS: &[&str] = &["glGetSamplerParameterIivEXT", "glGetSamplerParameterIuivEXT", "glGetTexParameterIivEXT", "glGetTexParameterIuivEXT", "glSamplerParameterIivEXT", "glSamplerParameterIuivEXT", "glTexParameterIivEXT", "glTexParameterIuivEXT"];

pub const GL_EXT_texture_buffer_COMMANDS: &[&str] = &["glTexBufferEXT", "glTexBufferRangeEXT"];

pub const GL_EXT_texture_buffer_object_COMMANDS: &[&str] = &["glTexBufferEXT"];

pub const GL_EXT_texture_integer_COMMANDS: &[&str] = &["glClearColorIiEXT", "glClearColorIuiEXT", "glGetTexParameterIivEXT", "glGetTexParameterIuivEXT", "glTexParameterIivEXT", "glTexParameterIuivEXT"];

pub const GL_EXT_texture_object_COMMANDS: &[&str] = &["glAreTexturesResidentEXT", "glBindTextureEXT", "glDeleteTexturesEXT", "glGenTexturesEXT", "glIsTextureEXT", "glPrioritizeTexturesEXT"];

pub const GL_EXT_texture_perturb_normal_COMMANDS: &[&str] = &["glTextureNormalEXT"];

pub const GL_EXT_texture_storage_COMMANDS: &[&str] = &["glTexStorage1DEXT", "glTexStorage2DEXT", "glTexStorage3DEXT", "glTextureStorage1DEXT", "glTextureStorage2DEXT", "glTextureStorage3DEXT"];

pub const GL_EXT_texture_view_COMMANDS: &[&str] = &["glTextureViewEXT"];

pub const GL_EXT_timer_query_COMMANDS: &[&str] = &["glGetQueryObjecti64vEXT", "glGetQueryObjectui64vEXT"];

pub const GL_EXT_transform_feedback_COMMANDS: &[&str] = &["glBeginTransformFeedbackEXT", "glBindBufferBaseEXT", "glBindBufferOffsetEXT", "glBindBufferRangeEXT", "glEndTransformFeedbackEXT", "glGetTransformFeedbackVaryingEXT", "glTransformFeedbackVaryingsEXT"];

pub const GL_EXT_vertex_array_COMMANDS: &[&str] = &["glArrayElementEXT", "glColorPointerEXT", "glDrawArraysEXT", "glEdgeFlagPointerEXT", "glGetPointervEXT", "glIndexPointerEXT", "glNormalPointerEXT", "glTexCoordPointerEXT", "glVertexPointerEXT"];

pub const GL_EXT_vertex_attrib_64bit_COMMANDS: &[&str] = &["glGetVertexAttribLdvEXT", "glVertexAttribL1dEXT", "glVertexAttribL1dvEXT", "glVertexAttribL2dEXT", "glVertexAttribL2dvEXT", "glVertexAttribL3dEXT", "glVertexAttribL3dvEXT", "glVertexAttribL4dEXT", "glVertexAttribL4dvEXT", "glVertexAttribLPointerEXT"];

pub const GL_EXT_vertex_shader_COMMANDS: &[&str] = &[
  "glBeginVertexShaderEXT",
  "glBindLightParameterEXT",
  "glBindMaterialParameterEXT",
  "glBindParameterEXT",
  "glBindTexGenParameterEXT",
  "glBindTextureUnitParameterEXT",
  "glBindVertexShaderEXT",
  "glDeleteVertexShaderEXT",
  "glDisableVariantClientStateEXT",
  "glEnableVariantClientStateEXT",
  "glEndVertexShaderEXT",
  "glExtractComponentEXT",
  "glGenSymbolsEXT",
  "glGenVertexShadersEXT",
  "glGetInvariantBooleanvEXT",
  "glGetInvariantFloatvEXT",
  "glGetInvariantIntegervEXT",
  "glGetLocalConstantBooleanvEXT",
  "glGetLocalConstantFloatvEXT",
  "glGetLocalConstantIntegervEXT",
  "glGetVariantBooleanvEXT",
  "glGetVariantFloatvEXT",
  "glGetVariantIntegervEXT",
  "glGetVariantPointervEXT",
  "glInsertComponentEXT",
  "glIsVariantEnabledEXT",
  "glSetInvariantEXT",
  "glSetLocalConstantEXT",
  "glShaderOp1EXT",
  "glShaderOp2EXT",
  "glShaderOp3EXT",
  "glSwizzleEXT",
  "glVariantPointerEXT",
  "glVariantbvEXT",
  "glVariantdvEXT",
  "glVariantfvEXT",
  "glVariantivEXT",
  "glVariantsvEXT",
  "glVariantubvEXT",
  "glVariantuivEXT",
  "glVariantusvEXT",
  "glWriteMaskEXT",
];

pub const GL_EXT_vertex_weighting_COMMANDS: &[&str] = &["glVertexWeightPointerEXT", "glVertexWeightfEXT", "glVertexWeightfvEXT"];

pub const GL_EXT_win32_keyed_mutex_COMMANDS: &[&str] = &["glAcquireKeyedMutexWin32EXT", "glReleaseKeyedMutexWin32EXT"];

pub const GL_EXT_window_rectangles_COMMANDS: &[&str] = &["glWindowRectanglesEXT"];

pub const GL_EXT_x11_sync_object_COMMANDS: &[&str] = &["glImportSyncEXT"];

pub const GL_GREMEDY_frame_terminator_COMMANDS: &[&str] = &["glFrameTerminatorGREMEDY"];

pub const GL_GREMEDY_string_marker_COMMANDS: &[&str] = &["glStringMarkerGREMEDY"];

pub const GL_HP_image_transform_COMMANDS: &[&str] = &["glGetImageTransformParameterfvHP", "glGetImageTransformParameterivHP", "glImageTransformParameterfHP", "glImageTransformParameterfvHP", "glImageTransformParameteriHP", "glImageTransformParameterivHP"];

pub const GL_IBM_multimode_draw_arrays_COMMANDS: &[&str] = &["glMultiModeDrawArraysIBM", "glMultiModeDrawElementsIBM"];

pub const GL_IBM_static_data_COMMANDS: &[&str] = &["glFlushStaticDataIBM"];

pub const GL_IBM_vertex_array_lists_COMMANDS: &[&str] = &["glColorPointerListIBM", "glEdgeFlagPointerListIBM", "glFogCoordPointerListIBM", "glIndexPointerListIBM", "glNormalPointerListIBM", "glSecondaryColorPointerListIBM", "glTexCoordPointerListIBM", "glVertexPointerListIBM"];

pub const GL_IMG_bindless_texture_COMMANDS: &[&str] = &["glGetTextureHandleIMG", "glGetTextureSamplerHandleIMG", "glProgramUniformHandleui64IMG", "glProgramUniformHandleui64vIMG", "glUniformHandleui64IMG", "glUniformHandleui64vIMG"];

pub const GL_IMG_framebuffer_downsample_COMMANDS: &[&str] = &["glFramebufferTexture2DDownsampleIMG", "glFramebufferTextureLayerDownsampleIMG"];

pub const GL_IMG_multisampled_render_to_texture_COMMANDS: &[&str] = &["glFramebufferTexture2DMultisampleIMG", "glRenderbufferStorageMultisampleIMG"];

pub const GL_IMG_user_clip_plane_COMMANDS: &[&str] = &["glClipPlanefIMG", "glClipPlanexIMG"];

pub const GL_INGR_blend_func_separate_COMMANDS: &[&str] = &["glBlendFuncSeparateINGR"];

pub const GL_INTEL_framebuffer_CMAA_COMMANDS: &[&str] = &["glApplyFramebufferAttachmentCMAAINTEL"];

pub const GL_INTEL_map_texture_COMMANDS: &[&str] = &["glMapTexture2DINTEL", "glSyncTextureINTEL", "glUnmapTexture2DINTEL"];

pub const GL_INTEL_parallel_arrays_COMMANDS: &[&str] = &["glColorPointervINTEL", "glNormalPointervINTEL", "glTexCoordPointervINTEL", "glVertexPointervINTEL"];

pub const GL_INTEL_performance_query_COMMANDS: &[&str] = &["glBeginPerfQueryINTEL", "glCreatePerfQueryINTEL", "glDeletePerfQueryINTEL", "glEndPerfQueryINTEL", "glGetFirstPerfQueryIdINTEL", "glGetNextPerfQueryIdINTEL", "glGetPerfCounterInfoINTEL", "glGetPerfQueryDataINTEL", "glGetPerfQueryIdByNameINTEL", "glGetPerfQueryInfoINTEL"];

pub const GL_KHR_blend_equation_advanced_COMMANDS: &[&str] = &["glBlendBarrierKHR"];

pub const GL_KHR_debug_COMMANDS: &[&str] = &["glDebugMessageCallback", "glDebugMessageCallbackKHR", "glDebugMessageControl", "glDebugMessageControlKHR", "glDebugMessageInsert", "glDebugMessageInsertKHR", "glGetDebugMessageLog", "glGetDebugMessageLogKHR", "glGetObjectLabel", "glGetObjectLabelKHR", "glGetObjectPtrLabel", "glGetObjectPtrLabelKHR", "glGetPointerv", "glGetPointervKHR", "glObjectLabel", "glObjectLabelKHR", "glObjectPtrLabel", "glObjectPtrLabelKHR", "glPopDebugGroup", "glPopDebugGroupKHR", "glPushDebugGroup", "glPushDebugGroupKHR"];

pub const GL_KHR_parallel_shader_compile_COMMANDS: &[&str] = &["glMaxShaderCompilerThreadsKHR"];

pub const GL_KHR_robustness_COMMANDS: &[&str] = &["glGetGraphicsResetStatus", "glGetGraphicsResetStatusKHR", "glGetnUniformfv", "glGetnUniformfvKHR", "glGetnUniformiv", "glGetnUniformivKHR", "glGetnUniformuiv", "glGetnUniformuivKHR", "glReadnPixels", "glReadnPixelsKHR"];

pub const GL_MESA_framebuffer_flip_y_COMMANDS: &[&str] = &["glFramebufferParameteriMESA", "glGetFramebufferParameterivMESA"];

pub const GL_MESA_resize_buffers_COMMANDS: &[&str] = &["glResizeBuffersMESA"];

pub const GL_MESA_window_pos_COMMANDS: &[&str] = &["glWindowPos2dMESA", "glWindowPos2dvMESA", "glWindowPos2fMESA", "glWindowPos2fvMESA", "glWindowPos2iMESA", "glWindowPos2ivMESA", "glWindowPos2sMESA", "glWindowPos2svMESA", "glWindowPos3dMESA", "glWindowPos3dvMESA", "glWindowPos3fMESA", "glWindowPos3fvMESA", "glWindowPos3iMESA", "glWindowPos3ivMESA", "glWindowPos3sMESA", "glWindowPos3svMESA", "glWindowPos4dMESA", "glWindowPos4dvMESA", "glWindowPos4fMESA", "glWindowPos4fvMESA", "glWindowPos4iMESA", "glWindowPos4ivMESA", "glWindowPos4sMESA", "glWindowPos4svMESA"];

pub const GL_NVX_conditional_render_COMMANDS: &[&str] = &["glBeginConditionalRenderNVX", "glEndConditionalRenderNVX"];

pub const GL_NVX_gpu_multicast2_COMMANDS: &[&str] = &["glAsyncCopyBufferSubDataNVX", "glAsyncCopyImageSubDataNVX", "glMulticastScissorArrayvNVX", "glMulticastViewportArrayvNVX", "glMulticastViewportPositionWScaleNVX", "glUploadGpuMaskNVX"];

pub const GL_NVX_linked_gpu_multicast_COMMANDS: &[&str] = &["glLGPUCopyImageSubDataNVX", "glLGPUInterlockNVX", "glLGPUNamedBufferSubDataNVX"];

pub const GL_NVX_progress_fence_COMMANDS: &[&str] = &["glClientWaitSemaphoreui64NVX", "glCreateProgressFenceNVX", "glSignalSemaphoreui64NVX", "glWaitSemaphoreui64NVX"];

pub const GL_NV_alpha_to_coverage_dither_control_COMMANDS: &[&str] = &["glAlphaToCoverageDitherControlNV"];

pub const GL_NV_bindless_multi_draw_indirect_COMMANDS: &[&str] = &["glMultiDrawArraysIndirectBindlessNV", "glMultiDrawElementsIndirectBindlessNV"];

pub const GL_NV_bindless_multi_draw_indirect_count_COMMANDS: &[&str] = &["glMultiDrawArraysIndirectBindlessCountNV", "glMultiDrawElementsIndirectBindlessCountNV"];

pub const GL_NV_bindless_texture_COMMANDS: &[&str] = &["glGetImageHandleNV", "glGetTextureHandleNV", "glGetTextureSamplerHandleNV", "glIsImageHandleResidentNV", "glIsTextureHandleResidentNV", "glMakeImageHandleNonResidentNV", "glMakeImageHandleResidentNV", "glMakeTextureHandleNonResidentNV", "glMakeTextureHandleResidentNV", "glProgramUniformHandleui64NV", "glProgramUniformHandleui64vNV", "glUniformHandleui64NV", "glUniformHandleui64vNV"];

pub const GL_NV_blend_equation_advanced_COMMANDS: &[&str] = &["glBlendBarrierNV", "glBlendParameteriNV"];

pub const GL_NV_clip_space_w_scaling_COMMANDS: &[&str] = &["glViewportPositionWScaleNV"];

pub const GL_NV_command_list_COMMANDS: &[&str] = &["glCallCommandListNV", "glCommandListSegmentsNV", "glCompileCommandListNV", "glCreateCommandListsNV", "glCreateStatesNV", "glDeleteCommandListsNV", "glDeleteStatesNV", "glDrawCommandsAddressNV", "glDrawCommandsNV", "glDrawCommandsStatesAddressNV", "glDrawCommandsStatesNV", "glGetCommandHeaderNV", "glGetStageIndexNV", "glIsCommandListNV", "glIsStateNV", "glListDrawCommandsStatesClientNV", "glStateCaptureNV"];

pub const GL_NV_conditional_render_COMMANDS: &[&str] = &["glBeginConditionalRenderNV", "glEndConditionalRenderNV"];

pub const GL_NV_conservative_raster_COMMANDS: &[&str] = &["glSubpixelPrecisionBiasNV"];

pub const GL_NV_conservative_raster_dilate_COMMANDS: &[&str] = &["glConservativeRasterParameterfNV"];

pub const GL_NV_conservative_raster_pre_snap_triangles_COMMANDS: &[&str] = &["glConservativeRasterParameteriNV"];

pub const GL_NV_copy_buffer_COMMANDS: &[&str] = &["glCopyBufferSubDataNV"];

pub const GL_NV_copy_image_COMMANDS: &[&str] = &["glCopyImageSubDataNV"];

pub const GL_NV_coverage_sample_COMMANDS: &[&str] = &["glCoverageMaskNV", "glCoverageOperationNV"];

pub const GL_NV_depth_buffer_float_COMMANDS: &[&str] = &["glClearDepthdNV", "glDepthBoundsdNV", "glDepthRangedNV"];

pub const GL_NV_draw_buffers_COMMANDS: &[&str] = &["glDrawBuffersNV"];

pub const GL_NV_draw_instanced_COMMANDS: &[&str] = &["glDrawArraysInstancedNV", "glDrawElementsInstancedNV"];

pub const GL_NV_draw_texture_COMMANDS: &[&str] = &["glDrawTextureNV"];

pub const GL_NV_draw_vulkan_image_COMMANDS: &[&str] = &["glDrawVkImageNV", "glGetVkProcAddrNV", "glSignalVkFenceNV", "glSignalVkSemaphoreNV", "glWaitVkSemaphoreNV"];

pub const GL_NV_evaluators_COMMANDS: &[&str] = &["glEvalMapsNV", "glGetMapAttribParameterfvNV", "glGetMapAttribParameterivNV", "glGetMapControlPointsNV", "glGetMapParameterfvNV", "glGetMapParameterivNV", "glMapControlPointsNV", "glMapParameterfvNV", "glMapParameterivNV"];

pub const GL_NV_explicit_multisample_COMMANDS: &[&str] = &["glGetMultisamplefvNV", "glSampleMaskIndexedNV", "glTexRenderbufferNV"];

pub const GL_NV_fence_COMMANDS: &[&str] = &["glDeleteFencesNV", "glFinishFenceNV", "glGenFencesNV", "glGetFenceivNV", "glIsFenceNV", "glSetFenceNV", "glTestFenceNV"];

pub const GL_NV_fragment_coverage_to_color_COMMANDS: &[&str] = &["glFragmentCoverageColorNV"];

pub const GL_NV_fragment_program_COMMANDS: &[&str] = &["glGetProgramNamedParameterdvNV", "glGetProgramNamedParameterfvNV", "glProgramNamedParameter4dNV", "glProgramNamedParameter4dvNV", "glProgramNamedParameter4fNV", "glProgramNamedParameter4fvNV"];

pub const GL_NV_framebuffer_blit_COMMANDS: &[&str] = &["glBlitFramebufferNV"];

pub const GL_NV_framebuffer_mixed_samples_COMMANDS: &[&str] = &["glCoverageModulationNV", "glCoverageModulationTableNV", "glGetCoverageModulationTableNV", "glRasterSamplesEXT"];

pub const GL_NV_framebuffer_multisample_COMMANDS: &[&str] = &["glRenderbufferStorageMultisampleNV"];

pub const GL_NV_framebuffer_multisample_coverage_COMMANDS: &[&str] = &["glRenderbufferStorageMultisampleCoverageNV"];

pub const GL_NV_geometry_program4_COMMANDS: &[&str] = &["glFramebufferTextureEXT", "glFramebufferTextureFaceEXT", "glFramebufferTextureLayerEXT", "glProgramVertexLimitNV"];

pub const GL_NV_gpu_multicast_COMMANDS: &[&str] = &["glMulticastBarrierNV", "glMulticastBlitFramebufferNV", "glMulticastBufferSubDataNV", "glMulticastCopyBufferSubDataNV", "glMulticastCopyImageSubDataNV", "glMulticastFramebufferSampleLocationsfvNV", "glMulticastGetQueryObjecti64vNV", "glMulticastGetQueryObjectivNV", "glMulticastGetQueryObjectui64vNV", "glMulticastGetQueryObjectuivNV", "glMulticastWaitSyncNV", "glRenderGpuMaskNV"];

pub const GL_NV_gpu_program4_COMMANDS: &[&str] = &["glGetProgramEnvParameterIivNV", "glGetProgramEnvParameterIuivNV", "glGetProgramLocalParameterIivNV", "glGetProgramLocalParameterIuivNV", "glProgramEnvParameterI4iNV", "glProgramEnvParameterI4ivNV", "glProgramEnvParameterI4uiNV", "glProgramEnvParameterI4uivNV", "glProgramEnvParametersI4ivNV", "glProgramEnvParametersI4uivNV", "glProgramLocalParameterI4iNV", "glProgramLocalParameterI4ivNV", "glProgramLocalParameterI4uiNV", "glProgramLocalParameterI4uivNV", "glProgramLocalParametersI4ivNV", "glProgramLocalParametersI4uivNV"];

pub const GL_NV_gpu_program5_COMMANDS: &[&str] = &["glGetProgramSubroutineParameteruivNV", "glProgramSubroutineParametersuivNV"];

pub const GL_NV_gpu_shader5_COMMANDS: &[&str] =
  &["glGetUniformi64vNV", "glProgramUniform1i64NV", "glProgramUniform1i64vNV", "glProgramUniform1ui64NV", "glProgramUniform1ui64vNV", "glProgramUniform2i64NV", "glProgramUniform2i64vNV", "glProgramUniform2ui64NV", "glProgramUniform2ui64vNV", "glProgramUniform3i64NV", "glProgramUniform3i64vNV", "glProgramUniform3ui64NV", "glProgramUniform3ui64vNV", "glProgramUniform4i64NV", "glProgramUniform4i64vNV", "glProgramUniform4ui64NV", "glProgramUniform4ui64vNV", "glUniform1i64NV", "glUniform1i64vNV", "glUniform1ui64NV", "glUniform1ui64vNV", "glUniform2i64NV", "glUniform2i64vNV", "glUniform2ui64NV", "glUniform2ui64vNV", "glUniform3i64NV", "glUniform3i64vNV", "glUniform3ui64NV", "glUniform3ui64vNV", "glUniform4i64NV", "glUniform4i64vNV", "glUniform4ui64NV", "glUniform4ui64vNV"];

pub const GL_NV_half_float_COMMANDS: &[&str] = &[
  "glColor3hNV",
  "glColor3hvNV",
  "glColor4hNV",
  "glColor4hvNV",
  "glFogCoordhNV",
  "glFogCoordhvNV",
  "glMultiTexCoord1hNV",
  "glMultiTexCoord1hvNV",
  "glMultiTexCoord2hNV",
  "glMultiTexCoord2hvNV",
  "glMultiTexCoord3hNV",
  "glMultiTexCoord3hvNV",
  "glMultiTexCoord4hNV",
  "glMultiTexCoord4hvNV",
  "glNormal3hNV",
  "glNormal3hvNV",
  "glSecondaryColor3hNV",
  "glSecondaryColor3hvNV",
  "glTexCoord1hNV",
  "glTexCoord1hvNV",
  "glTexCoord2hNV",
  "glTexCoord2hvNV",
  "glTexCoord3hNV",
  "glTexCoord3hvNV",
  "glTexCoord4hNV",
  "glTexCoord4hvNV",
  "glVertex2hNV",
  "glVertex2hvNV",
  "glVertex3hNV",
  "glVertex3hvNV",
  "glVertex4hNV",
  "glVertex4hvNV",
  "glVertexAttrib1hNV",
  "glVertexAttrib1hvNV",
  "glVertexAttrib2hNV",
  "glVertexAttrib2hvNV",
  "glVertexAttrib3hNV",
  "glVertexAttrib3hvNV",
  "glVertexAttrib4hNV",
  "glVertexAttrib4hvNV",
  "glVertexAttribs1hvNV",
  "glVertexAttribs2hvNV",
  "glVertexAttribs3hvNV",
  "glVertexAttribs4hvNV",
  "glVertexWeighthNV",
  "glVertexWeighthvNV",
];

pub const GL_NV_instanced_arrays_COMMANDS: &[&str] = &["glVertexAttribDivisorNV"];

pub const GL_NV_internalformat_sample_query_COMMANDS: &[&str] = &["glGetInternalformatSampleivNV"];

pub const GL_NV_memory_attachment_COMMANDS: &[&str] = &["glBufferAttachMemoryNV", "glGetMemoryObjectDetachedResourcesuivNV", "glNamedBufferAttachMemoryNV", "glResetMemoryObjectParameterNV", "glTexAttachMemoryNV", "glTextureAttachMemoryNV"];

pub const GL_NV_memory_object_sparse_COMMANDS: &[&str] = &["glBufferPageCommitmentMemNV", "glNamedBufferPageCommitmentMemNV", "glTexPageCommitmentMemNV", "glTexturePageCommitmentMemNV"];

pub const GL_NV_mesh_shader_COMMANDS: &[&str] = &["glDrawMeshTasksIndirectNV", "glDrawMeshTasksNV", "glMultiDrawMeshTasksIndirectCountNV", "glMultiDrawMeshTasksIndirectNV"];

pub const GL_NV_non_square_matrices_COMMANDS: &[&str] = &["glUniformMatrix2x3fvNV", "glUniformMatrix2x4fvNV", "glUniformMatrix3x2fvNV", "glUniformMatrix3x4fvNV", "glUniformMatrix4x2fvNV", "glUniformMatrix4x3fvNV"];

pub const GL_NV_occlusion_query_COMMANDS: &[&str] = &["glBeginOcclusionQueryNV", "glDeleteOcclusionQueriesNV", "glEndOcclusionQueryNV", "glGenOcclusionQueriesNV", "glGetOcclusionQueryivNV", "glGetOcclusionQueryuivNV", "glIsOcclusionQueryNV"];

pub const GL_NV_parameter_buffer_object_COMMANDS: &[&str] = &["glProgramBufferParametersIivNV", "glProgramBufferParametersIuivNV", "glProgramBufferParametersfvNV"];

pub const GL_NV_path_rendering_COMMANDS: &[&str] = &[
  "glCopyPathNV",
  "glCoverFillPathInstancedNV",
  "glCoverFillPathNV",
  "glCoverStrokePathInstancedNV",
  "glCoverStrokePathNV",
  "glDeletePathsNV",
  "glGenPathsNV",
  "glGetPathColorGenfvNV",
  "glGetPathColorGenivNV",
  "glGetPathCommandsNV",
  "glGetPathCoordsNV",
  "glGetPathDashArrayNV",
  "glGetPathLengthNV",
  "glGetPathMetricRangeNV",
  "glGetPathMetricsNV",
  "glGetPathParameterfvNV",
  "glGetPathParameterivNV",
  "glGetPathSpacingNV",
  "glGetPathTexGenfvNV",
  "glGetPathTexGenivNV",
  "glGetProgramResourcefvNV",
  "glInterpolatePathsNV",
  "glIsPathNV",
  "glIsPointInFillPathNV",
  "glIsPointInStrokePathNV",
  "glMatrixFrustumEXT",
  "glMatrixLoad3x2fNV",
  "glMatrixLoad3x3fNV",
  "glMatrixLoadIdentityEXT",
  "glMatrixLoadTranspose3x3fNV",
  "glMatrixLoadTransposedEXT",
  "glMatrixLoadTransposefEXT",
  "glMatrixLoaddEXT",
  "glMatrixLoadfEXT",
  "glMatrixMult3x2fNV",
  "glMatrixMult3x3fNV",
  "glMatrixMultTranspose3x3fNV",
  "glMatrixMultTransposedEXT",
  "glMatrixMultTransposefEXT",
  "glMatrixMultdEXT",
  "glMatrixMultfEXT",
  "glMatrixOrthoEXT",
  "glMatrixPopEXT",
  "glMatrixPushEXT",
  "glMatrixRotatedEXT",
  "glMatrixRotatefEXT",
  "glMatrixScaledEXT",
  "glMatrixScalefEXT",
  "glMatrixTranslatedEXT",
  "glMatrixTranslatefEXT",
  "glPathColorGenNV",
  "glPathCommandsNV",
  "glPathCoordsNV",
  "glPathCoverDepthFuncNV",
  "glPathDashArrayNV",
  "glPathFogGenNV",
  "glPathGlyphIndexArrayNV",
  "glPathGlyphIndexRangeNV",
  "glPathGlyphRangeNV",
  "glPathGlyphsNV",
  "glPathMemoryGlyphIndexArrayNV",
  "glPathParameterfNV",
  "glPathParameterfvNV",
  "glPathParameteriNV",
  "glPathParameterivNV",
  "glPathStencilDepthOffsetNV",
  "glPathStencilFuncNV",
  "glPathStringNV",
  "glPathSubCommandsNV",
  "glPathSubCoordsNV",
  "glPathTexGenNV",
  "glPointAlongPathNV",
  "glProgramPathFragmentInputGenNV",
  "glStencilFillPathInstancedNV",
  "glStencilFillPathNV",
  "glStencilStrokePathInstancedNV",
  "glStencilStrokePathNV",
  "glStencilThenCoverFillPathInstancedNV",
  "glStencilThenCoverFillPathNV",
  "glStencilThenCoverStrokePathInstancedNV",
  "glStencilThenCoverStrokePathNV",
  "glTransformPathNV",
  "glWeightPathsNV",
];

pub const GL_NV_pixel_data_range_COMMANDS: &[&str] = &["glFlushPixelDataRangeNV", "glPixelDataRangeNV"];

pub const GL_NV_point_sprite_COMMANDS: &[&str] = &["glPointParameteriNV", "glPointParameterivNV"];

pub const GL_NV_polygon_mode_COMMANDS: &[&str] = &["glPolygonModeNV"];

pub const GL_NV_present_video_COMMANDS: &[&str] = &["glGetVideoi64vNV", "glGetVideoivNV", "glGetVideoui64vNV", "glGetVideouivNV", "glPresentFrameDualFillNV", "glPresentFrameKeyedNV"];

pub const GL_NV_primitive_restart_COMMANDS: &[&str] = &["glPrimitiveRestartIndexNV", "glPrimitiveRestartNV"];

pub const GL_NV_query_resource_COMMANDS: &[&str] = &["glQueryResourceNV"];

pub const GL_NV_query_resource_tag_COMMANDS: &[&str] = &["glDeleteQueryResourceTagNV", "glGenQueryResourceTagNV", "glQueryResourceTagNV"];

pub const GL_NV_read_buffer_COMMANDS: &[&str] = &["glReadBufferNV"];

pub const GL_NV_register_combiners_COMMANDS: &[&str] = &["glCombinerInputNV", "glCombinerOutputNV", "glCombinerParameterfNV", "glCombinerParameterfvNV", "glCombinerParameteriNV", "glCombinerParameterivNV", "glFinalCombinerInputNV", "glGetCombinerInputParameterfvNV", "glGetCombinerInputParameterivNV", "glGetCombinerOutputParameterfvNV", "glGetCombinerOutputParameterivNV", "glGetFinalCombinerInputParameterfvNV", "glGetFinalCombinerInputParameterivNV"];

pub const GL_NV_register_combiners2_COMMANDS: &[&str] = &["glCombinerStageParameterfvNV", "glGetCombinerStageParameterfvNV"];

pub const GL_NV_sample_locations_COMMANDS: &[&str] = &["glFramebufferSampleLocationsfvNV", "glNamedFramebufferSampleLocationsfvNV", "glResolveDepthValuesNV"];

pub const GL_NV_scissor_exclusive_COMMANDS: &[&str] = &["glScissorExclusiveArrayvNV", "glScissorExclusiveNV"];

pub const GL_NV_shader_buffer_load_COMMANDS: &[&str] = &["glGetBufferParameterui64vNV", "glGetIntegerui64vNV", "glGetNamedBufferParameterui64vNV", "glGetUniformui64vNV", "glIsBufferResidentNV", "glIsNamedBufferResidentNV", "glMakeBufferNonResidentNV", "glMakeBufferResidentNV", "glMakeNamedBufferNonResidentNV", "glMakeNamedBufferResidentNV", "glProgramUniformui64NV", "glProgramUniformui64vNV", "glUniformui64NV", "glUniformui64vNV"];

pub const GL_NV_shading_rate_image_COMMANDS: &[&str] = &["glBindShadingRateImageNV", "glGetShadingRateImagePaletteNV", "glGetShadingRateSampleLocationivNV", "glShadingRateImageBarrierNV", "glShadingRateImagePaletteNV", "glShadingRateSampleOrderCustomNV", "glShadingRateSampleOrderNV"];

pub const GL_NV_texture_barrier_COMMANDS: &[&str] = &["glTextureBarrierNV"];

pub const GL_NV_texture_multisample_COMMANDS: &[&str] = &["glTexImage2DMultisampleCoverageNV", "glTexImage3DMultisampleCoverageNV", "glTextureImage2DMultisampleCoverageNV", "glTextureImage2DMultisampleNV", "glTextureImage3DMultisampleCoverageNV", "glTextureImage3DMultisampleNV"];

pub const GL_NV_timeline_semaphore_COMMANDS: &[&str] = &["glCreateSemaphoresNV", "glGetSemaphoreParameterivNV", "glSemaphoreParameterivNV"];

pub const GL_NV_transform_feedback_COMMANDS: &[&str] = &["glActiveVaryingNV", "glBeginTransformFeedbackNV", "glBindBufferBaseNV", "glBindBufferOffsetNV", "glBindBufferRangeNV", "glEndTransformFeedbackNV", "glGetActiveVaryingNV", "glGetTransformFeedbackVaryingNV", "glGetVaryingLocationNV", "glTransformFeedbackAttribsNV", "glTransformFeedbackStreamAttribsNV", "glTransformFeedbackVaryingsNV"];

pub const GL_NV_transform_feedback2_COMMANDS: &[&str] = &["glBindTransformFeedbackNV", "glDeleteTransformFeedbacksNV", "glDrawTransformFeedbackNV", "glGenTransformFeedbacksNV", "glIsTransformFeedbackNV", "glPauseTransformFeedbackNV", "glResumeTransformFeedbackNV"];

pub const GL_NV_vdpau_interop_COMMANDS: &[&str] = &["glVDPAUFiniNV", "glVDPAUGetSurfaceivNV", "glVDPAUInitNV", "glVDPAUIsSurfaceNV", "glVDPAUMapSurfacesNV", "glVDPAURegisterOutputSurfaceNV", "glVDPAURegisterVideoSurfaceNV", "glVDPAUSurfaceAccessNV", "glVDPAUUnmapSurfacesNV", "glVDPAUUnregisterSurfaceNV"];

pub const GL_NV_vdpau_interop2_COMMANDS: &[&str] = &["glVDPAURegisterVideoSurfaceWithPictureStructureNV"];

pub const GL_NV_vertex_array_range_COMMANDS: &[&str] = &["glFlushVertexArrayRangeNV", "glVertexArrayRangeNV"];

pub const GL_NV_vertex_attrib_integer_64bit_COMMANDS: &[&str] = &["glGetVertexAttribLi64vNV", "glGetVertexAttribLui64vNV", "glVertexAttribL1i64NV", "glVertexAttribL1i64vNV", "glVertexAttribL1ui64NV", "glVertexAttribL1ui64vNV", "glVertexAttribL2i64NV", "glVertexAttribL2i64vNV", "glVertexAttribL2ui64NV", "glVertexAttribL2ui64vNV", "glVertexAttribL3i64NV", "glVertexAttribL3i64vNV", "glVertexAttribL3ui64NV", "glVertexAttribL3ui64vNV", "glVertexAttribL4i64NV", "glVertexAttribL4i64vNV", "glVertexAttribL4ui64NV", "glVertexAttribL4ui64vNV", "glVertexAttribLFormatNV"];

pub const GL_NV_vertex_buffer_unified_memory_COMMANDS: &[&str] = &["glBufferAddressRangeNV", "glColorFormatNV", "glEdgeFlagFormatNV", "glFogCoordFormatNV", "glGetIntegerui64i_vNV", "glIndexFormatNV", "glNormalFormatNV", "glSecondaryColorFormatNV", "glTexCoordFormatNV", "glVertexAttribFormatNV", "glVertexAttribIFormatNV", "glVertexFormatNV"];

pub const GL_NV_vertex_program_COMMANDS: &[&str] = &[
  "glAreProgramsResidentNV",
  "glBindProgramNV",
  "glDeleteProgramsNV",
  "glExecuteProgramNV",
  "glGenProgramsNV",
  "glGetProgramParameterdvNV",
  "glGetProgramParameterfvNV",
  "glGetProgramStringNV",
  "glGetProgramivNV",
  "glGetTrackMatrixivNV",
  "glGetVertexAttribPointervNV",
  "glGetVertexAttribdvNV",
  "glGetVertexAttribfvNV",
  "glGetVertexAttribivNV",
  "glIsProgramNV",
  "glLoadProgramNV",
  "glProgramParameter4dNV",
  "glProgramParameter4dvNV",
  "glProgramParameter4fNV",
  "glProgramParameter4fvNV",
  "glProgramParameters4dvNV",
  "glProgramParameters4fvNV",
  "glRequestResidentProgramsNV",
  "glTrackMatrixNV",
  "glVertexAttrib1dNV",
  "glVertexAttrib1dvNV",
  "glVertexAttrib1fNV",
  "glVertexAttrib1fvNV",
  "glVertexAttrib1sNV",
  "glVertexAttrib1svNV",
  "glVertexAttrib2dNV",
  "glVertexAttrib2dvNV",
  "glVertexAttrib2fNV",
  "glVertexAttrib2fvNV",
  "glVertexAttrib2sNV",
  "glVertexAttrib2svNV",
  "glVertexAttrib3dNV",
  "glVertexAttrib3dvNV",
  "glVertexAttrib3fNV",
  "glVertexAttrib3fvNV",
  "glVertexAttrib3sNV",
  "glVertexAttrib3svNV",
  "glVertexAttrib4dNV",
  "glVertexAttrib4dvNV",
  "glVertexAttrib4fNV",
  "glVertexAttrib4fvNV",
  "glVertexAttrib4sNV",
  "glVertexAttrib4svNV",
  "glVertexAttrib4ubNV",
  "glVertexAttrib4ubvNV",
  "glVertexAttribPointerNV",
  "glVertexAttribs1dvNV",
  "glVertexAttribs1fvNV",
  "glVertexAttribs1svNV",
  "glVertexAttribs2dvNV",
  "glVertexAttribs2fvNV",
  "glVertexAttribs2svNV",
  "glVertexAttribs3dvNV",
  "glVertexAttribs3fvNV",
  "glVertexAttribs3svNV",
  "glVertexAttribs4dvNV",
  "glVertexAttribs4fvNV",
  "glVertexAttribs4svNV",
  "glVertexAttribs4ubvNV",
];

pub const GL_NV_vertex_program4_COMMANDS: &[&str] = &["glGetVertexAttribIivEXT", "glGetVertexAttribIuivEXT", "glVertexAttribI1iEXT", "glVertexAttribI1ivEXT", "glVertexAttribI1uiEXT", "glVertexAttribI1uivEXT", "glVertexAttribI2iEXT", "glVertexAttribI2ivEXT", "glVertexAttribI2uiEXT", "glVertexAttribI2uivEXT", "glVertexAttribI3iEXT", "glVertexAttribI3ivEXT", "glVertexAttribI3uiEXT", "glVertexAttribI3uivEXT", "glVertexAttribI4bvEXT", "glVertexAttribI4iEXT", "glVertexAttribI4ivEXT", "glVertexAttribI4svEXT", "glVertexAttribI4ubvEXT", "glVertexAttribI4uiEXT", "glVertexAttribI4uivEXT", "glVertexAttribI4usvEXT", "glVertexAttribIPointerEXT"];

pub const GL_NV_video_capture_COMMANDS: &[&str] = &["glBeginVideoCaptureNV", "glBindVideoCaptureStreamBufferNV", "glBindVideoCaptureStreamTextureNV", "glEndVideoCaptureNV", "glGetVideoCaptureStreamdvNV", "glGetVideoCaptureStreamfvNV", "glGetVideoCaptureStreamivNV", "glGetVideoCaptureivNV", "glVideoCaptureNV", "glVideoCaptureStreamParameterdvNV", "glVideoCaptureStreamParameterfvNV", "glVideoCaptureStreamParameterivNV"];

pub const GL_NV_viewport_array_COMMANDS: &[&str] = &["glDepthRangeArrayfvNV", "glDepthRangeIndexedfNV", "glDisableiNV", "glEnableiNV", "glGetFloati_vNV", "glIsEnablediNV", "glScissorArrayvNV", "glScissorIndexedNV", "glScissorIndexedvNV", "glViewportArrayvNV", "glViewportIndexedfNV", "glViewportIndexedfvNV"];

pub const GL_NV_viewport_swizzle_COMMANDS: &[&str] = &["glViewportSwizzleNV"];

pub const GL_OES_EGL_image_COMMANDS: &[&str] = &["glEGLImageTargetRenderbufferStorageOES", "glEGLImageTargetTexture2DOES"];

pub const GL_OES_blend_equation_separate_COMMANDS: &[&str] = &["glBlendEquationSeparateOES"];

pub const GL_OES_blend_func_separate_COMMANDS: &[&str] = &["glBlendFuncSeparateOES"];

pub const GL_OES_blend_subtract_COMMANDS: &[&str] = &["glBlendEquationOES"];

pub const GL_OES_byte_coordinates_COMMANDS: &[&str] = &["glMultiTexCoord1bOES", "glMultiTexCoord1bvOES", "glMultiTexCoord2bOES", "glMultiTexCoord2bvOES", "glMultiTexCoord3bOES", "glMultiTexCoord3bvOES", "glMultiTexCoord4bOES", "glMultiTexCoord4bvOES", "glTexCoord1bOES", "glTexCoord1bvOES", "glTexCoord2bOES", "glTexCoord2bvOES", "glTexCoord3bOES", "glTexCoord3bvOES", "glTexCoord4bOES", "glTexCoord4bvOES", "glVertex2bOES", "glVertex2bvOES", "glVertex3bOES", "glVertex3bvOES", "glVertex4bOES", "glVertex4bvOES"];

pub const GL_OES_copy_image_COMMANDS: &[&str] = &["glCopyImageSubDataOES"];

pub const GL_OES_draw_buffers_indexed_COMMANDS: &[&str] = &["glBlendEquationSeparateiOES", "glBlendEquationiOES", "glBlendFuncSeparateiOES", "glBlendFunciOES", "glColorMaskiOES", "glDisableiOES", "glEnableiOES", "glIsEnablediOES"];

pub const GL_OES_draw_elements_base_vertex_COMMANDS: &[&str] = &["glDrawElementsBaseVertexOES", "glDrawElementsInstancedBaseVertexOES", "glDrawRangeElementsBaseVertexOES", "glMultiDrawElementsBaseVertexEXT"];

pub const GL_OES_draw_texture_COMMANDS: &[&str] = &["glDrawTexfOES", "glDrawTexfvOES", "glDrawTexiOES", "glDrawTexivOES", "glDrawTexsOES", "glDrawTexsvOES", "glDrawTexxOES", "glDrawTexxvOES"];

pub const GL_OES_fixed_point_COMMANDS: &[&str] = &[
  "glAccumxOES",
  "glAlphaFuncxOES",
  "glBitmapxOES",
  "glBlendColorxOES",
  "glClearAccumxOES",
  "glClearColorxOES",
  "glClearDepthxOES",
  "glClipPlanexOES",
  "glColor3xOES",
  "glColor3xvOES",
  "glColor4xOES",
  "glColor4xvOES",
  "glConvolutionParameterxOES",
  "glConvolutionParameterxvOES",
  "glDepthRangexOES",
  "glEvalCoord1xOES",
  "glEvalCoord1xvOES",
  "glEvalCoord2xOES",
  "glEvalCoord2xvOES",
  "glFeedbackBufferxOES",
  "glFogxOES",
  "glFogxvOES",
  "glFrustumxOES",
  "glGetClipPlanexOES",
  "glGetConvolutionParameterxvOES",
  "glGetFixedvOES",
  "glGetHistogramParameterxvOES",
  "glGetLightxOES",
  "glGetLightxvOES",
  "glGetMapxvOES",
  "glGetMaterialxOES",
  "glGetMaterialxvOES",
  "glGetPixelMapxv",
  "glGetTexEnvxvOES",
  "glGetTexGenxvOES",
  "glGetTexLevelParameterxvOES",
  "glGetTexParameterxvOES",
  "glIndexxOES",
  "glIndexxvOES",
  "glLightModelxOES",
  "glLightModelxvOES",
  "glLightxOES",
  "glLightxvOES",
  "glLineWidthxOES",
  "glLoadMatrixxOES",
  "glLoadTransposeMatrixxOES",
  "glMap1xOES",
  "glMap2xOES",
  "glMapGrid1xOES",
  "glMapGrid2xOES",
  "glMaterialxOES",
  "glMaterialxvOES",
  "glMultMatrixxOES",
  "glMultTransposeMatrixxOES",
  "glMultiTexCoord1xOES",
  "glMultiTexCoord1xvOES",
  "glMultiTexCoord2xOES",
  "glMultiTexCoord2xvOES",
  "glMultiTexCoord3xOES",
  "glMultiTexCoord3xvOES",
  "glMultiTexCoord4xOES",
  "glMultiTexCoord4xvOES",
  "glNormal3xOES",
  "glNormal3xvOES",
  "glOrthoxOES",
  "glPassThroughxOES",
  "glPixelMapx",
  "glPixelStorex",
  "glPixelTransferxOES",
  "glPixelZoomxOES",
  "glPointParameterxOES",
  "glPointParameterxvOES",
  "glPointSizexOES",
  "glPolygonOffsetxOES",
  "glPrioritizeTexturesxOES",
  "glRasterPos2xOES",
  "glRasterPos2xvOES",
  "glRasterPos3xOES",
  "glRasterPos3xvOES",
  "glRasterPos4xOES",
  "glRasterPos4xvOES",
  "glRectxOES",
  "glRectxvOES",
  "glRotatexOES",
  "glSampleCoveragexOES",
  "glScalexOES",
  "glTexCoord1xOES",
  "glTexCoord1xvOES",
  "glTexCoord2xOES",
  "glTexCoord2xvOES",
  "glTexCoord3xOES",
  "glTexCoord3xvOES",
  "glTexCoord4xOES",
  "glTexCoord4xvOES",
  "glTexEnvxOES",
  "glTexEnvxvOES",
  "glTexGenxOES",
  "glTexGenxvOES",
  "glTexParameterxOES",
  "glTexParameterxvOES",
  "glTranslatexOES",
  "glVertex2xOES",
  "glVertex2xvOES",
  "glVertex3xOES",
  "glVertex3xvOES",
  "glVertex4xOES",
  "glVertex4xvOES",
];

pub const GL_OES_framebuffer_object_COMMANDS: &[&str] = &["glBindFramebufferOES", "glBindRenderbufferOES", "glCheckFramebufferStatusOES", "glDeleteFramebuffersOES", "glDeleteRenderbuffersOES", "glFramebufferRenderbufferOES", "glFramebufferTexture2DOES", "glGenFramebuffersOES", "glGenRenderbuffersOES", "glGenerateMipmapOES", "glGetFramebufferAttachmentParameterivOES", "glGetRenderbufferParameterivOES", "glIsFramebufferOES", "glIsRenderbufferOES", "glRenderbufferStorageOES"];

pub const GL_OES_geometry_shader_COMMANDS: &[&str] = &["glFramebufferTextureOES"];

pub const GL_OES_get_program_binary_COMMANDS: &[&str] = &["glGetProgramBinaryOES", "glProgramBinaryOES"];

pub const GL_OES_mapbuffer_COMMANDS: &[&str] = &["glGetBufferPointervOES", "glMapBufferOES", "glUnmapBufferOES"];

pub const GL_OES_matrix_palette_COMMANDS: &[&str] = &["glCurrentPaletteMatrixOES", "glLoadPaletteFromModelViewMatrixOES", "glMatrixIndexPointerOES", "glWeightPointerOES"];

pub const GL_OES_point_size_array_COMMANDS: &[&str] = &["glPointSizePointerOES"];

pub const GL_OES_primitive_bounding_box_COMMANDS: &[&str] = &["glPrimitiveBoundingBoxOES"];

pub const GL_OES_query_matrix_COMMANDS: &[&str] = &["glQueryMatrixxOES"];

pub const GL_OES_sample_shading_COMMANDS: &[&str] = &["glMinSampleShadingOES"];

pub const GL_OES_single_precision_COMMANDS: &[&str] = &["glClearDepthfOES", "glClipPlanefOES", "glDepthRangefOES", "glFrustumfOES", "glGetClipPlanefOES", "glOrthofOES"];

pub const GL_OES_tessellation_shader_COMMANDS: &[&str] = &["glPatchParameteriOES"];

pub const GL_OES_texture_3D_COMMANDS: &[&str] = &["glCompressedTexImage3DOES", "glCompressedTexSubImage3DOES", "glCopyTexSubImage3DOES", "glFramebufferTexture3DOES", "glTexImage3DOES", "glTexSubImage3DOES"];

pub const GL_OES_texture_border_clamp_COMMANDS: &[&str] = &["glGetSamplerParameterIivOES", "glGetSamplerParameterIuivOES", "glGetTexParameterIivOES", "glGetTexParameterIuivOES", "glSamplerParameterIivOES", "glSamplerParameterIuivOES", "glTexParameterIivOES", "glTexParameterIuivOES"];

pub const GL_OES_texture_buffer_COMMANDS: &[&str] = &["glTexBufferOES", "glTexBufferRangeOES"];

pub const GL_OES_texture_cube_map_COMMANDS: &[&str] = &["glGetTexGenfvOES", "glGetTexGenivOES", "glGetTexGenxvOES", "glTexGenfOES", "glTexGenfvOES", "glTexGeniOES", "glTexGenivOES", "glTexGenxOES", "glTexGenxvOES"];

pub const GL_OES_texture_storage_multisample_2d_array_COMMANDS: &[&str] = &["glTexStorage3DMultisampleOES"];

pub const GL_OES_texture_view_COMMANDS: &[&str] = &["glTextureViewOES"];

pub const GL_OES_vertex_array_object_COMMANDS: &[&str] = &["glBindVertexArrayOES", "glDeleteVertexArraysOES", "glGenVertexArraysOES", "glIsVertexArrayOES"];

pub const GL_OES_viewport_array_COMMANDS: &[&str] = &["glDepthRangeArrayfvOES", "glDepthRangeIndexedfOES", "glDisableiOES", "glEnableiOES", "glGetFloati_vOES", "glIsEnablediOES", "glScissorArrayvOES", "glScissorIndexedOES", "glScissorIndexedvOES", "glViewportArrayvOES", "glViewportIndexedfOES", "glViewportIndexedfvOES"];

pub const GL_OVR_multiview_COMMANDS: &[&str] = &["glFramebufferTextureMultiviewOVR"];

pub const GL_OVR_multiview_multisampled_render_to_texture_COMMANDS: &[&str] = &["glFramebufferTextureMultisampleMultiviewOVR"];

pub const GL_PGI_misc_hints_COMMANDS: &[&str] = &["glHintPGI"];

pub const GL_QCOM_alpha_test_COMMANDS: &[&str] = &["glAlphaFuncQCOM"];

pub const GL_QCOM_driver_control_COMMANDS: &[&str] = &["glDisableDriverControlQCOM", "glEnableDriverControlQCOM", "glGetDriverControlStringQCOM", "glGetDriverControlsQCOM"];

pub const GL_QCOM_extended_get_COMMANDS: &[&str] = &["glExtGetBufferPointervQCOM", "glExtGetBuffersQCOM", "glExtGetFramebuffersQCOM", "glExtGetRenderbuffersQCOM", "glExtGetTexLevelParameterivQCOM", "glExtGetTexSubImageQCOM", "glExtGetTexturesQCOM", "glExtTexObjectStateOverrideiQCOM"];

pub const GL_QCOM_extended_get2_COMMANDS: &[&str] = &["glExtGetProgramBinarySourceQCOM", "glExtGetProgramsQCOM", "glExtGetShadersQCOM", "glExtIsProgramBinaryQCOM"];

pub const GL_QCOM_frame_extrapolation_COMMANDS: &[&str] = &["glExtrapolateTex2DQCOM"];

pub const GL_QCOM_framebuffer_foveated_COMMANDS: &[&str] = &["glFramebufferFoveationConfigQCOM", "glFramebufferFoveationParametersQCOM"];

pub const GL_QCOM_motion_estimation_COMMANDS: &[&str] = &["glTexEstimateMotionQCOM", "glTexEstimateMotionRegionsQCOM"];

pub const GL_QCOM_shader_framebuffer_fetch_noncoherent_COMMANDS: &[&str] = &["glFramebufferFetchBarrierQCOM"];

pub const GL_QCOM_shading_rate_COMMANDS: &[&str] = &["glShadingRateQCOM"];

pub const GL_QCOM_texture_foveated_COMMANDS: &[&str] = &["glTextureFoveationParametersQCOM"];

pub const GL_QCOM_tiled_rendering_COMMANDS: &[&str] = &["glEndTilingQCOM", "glStartTilingQCOM"];

pub const GL_SGIS_detail_texture_COMMANDS: &[&str] = &["glDetailTexFuncSGIS", "glGetDetailTexFuncSGIS"];

pub const GL_SGIS_fog_function_COMMANDS: &[&str] = &["glFogFuncSGIS", "glGetFogFuncSGIS"];

pub const GL_SGIS_multisample_COMMANDS: &[&str] = &["glSampleMaskSGIS", "glSamplePatternSGIS"];

pub const GL_SGIS_pixel_texture_COMMANDS: &[&str] = &["glGetPixelTexGenParameterfvSGIS", "glGetPixelTexGenParameterivSGIS", "glPixelTexGenParameterfSGIS", "glPixelTexGenParameterfvSGIS", "glPixelTexGenParameteriSGIS", "glPixelTexGenParameterivSGIS"];

pub const GL_SGIS_point_parameters_COMMANDS: &[&str] = &["glPointParameterfSGIS", "glPointParameterfvSGIS"];

pub const GL_SGIS_sharpen_texture_COMMANDS: &[&str] = &["glGetSharpenTexFuncSGIS", "glSharpenTexFuncSGIS"];

pub const GL_SGIS_texture4D_COMMANDS: &[&str] = &["glTexImage4DSGIS", "glTexSubImage4DSGIS"];

pub const GL_SGIS_texture_color_mask_COMMANDS: &[&str] = &["glTextureColorMaskSGIS"];

pub const GL_SGIS_texture_filter4_COMMANDS: &[&str] = &["glGetTexFilterFuncSGIS", "glTexFilterFuncSGIS"];

pub const GL_SGIX_async_COMMANDS: &[&str] = &["glAsyncMarkerSGIX", "glDeleteAsyncMarkersSGIX", "glFinishAsyncSGIX", "glGenAsyncMarkersSGIX", "glIsAsyncMarkerSGIX", "glPollAsyncSGIX"];

pub const GL_SGIX_flush_raster_COMMANDS: &[&str] = &["glFlushRasterSGIX"];

pub const GL_SGIX_fragment_lighting_COMMANDS: &[&str] = &["glFragmentColorMaterialSGIX", "glFragmentLightModelfSGIX", "glFragmentLightModelfvSGIX", "glFragmentLightModeliSGIX", "glFragmentLightModelivSGIX", "glFragmentLightfSGIX", "glFragmentLightfvSGIX", "glFragmentLightiSGIX", "glFragmentLightivSGIX", "glFragmentMaterialfSGIX", "glFragmentMaterialfvSGIX", "glFragmentMaterialiSGIX", "glFragmentMaterialivSGIX", "glGetFragmentLightfvSGIX", "glGetFragmentLightivSGIX", "glGetFragmentMaterialfvSGIX", "glGetFragmentMaterialivSGIX", "glLightEnviSGIX"];

pub const GL_SGIX_framezoom_COMMANDS: &[&str] = &["glFrameZoomSGIX"];

pub const GL_SGIX_igloo_interface_COMMANDS: &[&str] = &["glIglooInterfaceSGIX"];

pub const GL_SGIX_instruments_COMMANDS: &[&str] = &["glGetInstrumentsSGIX", "glInstrumentsBufferSGIX", "glPollInstrumentsSGIX", "glReadInstrumentsSGIX", "glStartInstrumentsSGIX", "glStopInstrumentsSGIX"];

pub const GL_SGIX_list_priority_COMMANDS: &[&str] = &["glGetListParameterfvSGIX", "glGetListParameterivSGIX", "glListParameterfSGIX", "glListParameterfvSGIX", "glListParameteriSGIX", "glListParameterivSGIX"];

pub const GL_SGIX_pixel_texture_COMMANDS: &[&str] = &["glPixelTexGenSGIX"];

pub const GL_SGIX_polynomial_ffd_COMMANDS: &[&str] = &["glDeformSGIX", "glDeformationMap3dSGIX", "glDeformationMap3fSGIX", "glLoadIdentityDeformationMapSGIX"];

pub const GL_SGIX_reference_plane_COMMANDS: &[&str] = &["glReferencePlaneSGIX"];

pub const GL_SGIX_sprite_COMMANDS: &[&str] = &["glSpriteParameterfSGIX", "glSpriteParameterfvSGIX", "glSpriteParameteriSGIX", "glSpriteParameterivSGIX"];

pub const GL_SGIX_tag_sample_buffer_COMMANDS: &[&str] = &["glTagSampleBufferSGIX"];

pub const GL_SGI_color_table_COMMANDS: &[&str] = &["glColorTableParameterfvSGI", "glColorTableParameterivSGI", "glColorTableSGI", "glCopyColorTableSGI", "glGetColorTableParameterfvSGI", "glGetColorTableParameterivSGI", "glGetColorTableSGI"];

pub const GL_SUNX_constant_data_COMMANDS: &[&str] = &["glFinishTextureSUNX"];

pub const GL_SUN_global_alpha_COMMANDS: &[&str] = &["glGlobalAlphaFactorbSUN", "glGlobalAlphaFactordSUN", "glGlobalAlphaFactorfSUN", "glGlobalAlphaFactoriSUN", "glGlobalAlphaFactorsSUN", "glGlobalAlphaFactorubSUN", "glGlobalAlphaFactoruiSUN", "glGlobalAlphaFactorusSUN"];

pub const GL_SUN_mesh_array_COMMANDS: &[&str] = &["glDrawMeshArraysSUN"];

pub const GL_SUN_triangle_list_COMMANDS: &[&str] = &["glReplacementCodePointerSUN", "glReplacementCodeubSUN", "glReplacementCodeubvSUN", "glReplacementCodeuiSUN", "glReplacementCodeuivSUN", "glReplacementCodeusSUN", "glReplacementCodeusvSUN"];

pub const GL_SUN_vertex_COMMANDS: &[&str] = &[
  "glColor3fVertex3fSUN",
  "glColor3fVertex3fvSUN",
  "glColor4fNormal3fVertex3fSUN",
  "glColor4fNormal3fVertex3fvSUN",
  "glColor4ubVertex2fSUN",
  "glColor4ubVertex2fvSUN",
  "glColor4ubVertex3fSUN",
  "glColor4ubVertex3fvSUN",
  "glNormal3fVertex3fSUN",
  "glNormal3fVertex3fvSUN",
  "glReplacementCodeuiColor3fVertex3fSUN",
  "glReplacementCodeuiColor3fVertex3fvSUN",
  "glReplacementCodeuiColor4fNormal3fVertex3fSUN",
  "glReplacementCodeuiColor4fNormal3fVertex3fvSUN",
  "glReplacementCodeuiColor4ubVertex3fSUN",
  "glReplacementCodeuiColor4ubVertex3fvSUN",
  "glReplacementCodeuiNormal3fVertex3fSUN",
  "glReplacementCodeuiNormal3fVertex3fvSUN",
  "glReplacementCodeuiTexCoord2fColor4fNormal3fVertex3fSUN",
  "glReplacementCodeuiTexCoord2fColor4fNormal3fVertex3fvSUN",
  "glReplacementCodeuiTexCoord2fNormal3fVertex3fSUN",
  "glReplacementCodeuiTexCoord2fNormal3fVertex3fvSUN",
  "glReplacementCodeuiTexCoord2fVertex3fSUN",
  "glReplacementCodeuiTexCoord2fVertex3fvSUN",
  "glReplacementCodeuiVertex3fSUN",
  "glReplacementCodeuiVertex3fvSUN",
  "glTexCoord2fColor3fVertex3fSUN",
  "glTexCoord2fColor3fVertex3fvSUN",
  "glTexCoord2fColor4fNormal3fVertex3fSUN",
  "glTexCoord2fColor4fNormal3fVertex3fvSUN",
  "glTexCoord2fColor4ubVertex3fSUN",
  "glTexCoord2fColor4ubVertex3fvSUN",
  "glTexCoord2fNormal3fVertex3fSUN",
  "glTexCoord2fNormal3fVertex3fvSUN",
  "glTexCoord2fVertex3fSUN",
  "glTexCoord2fVertex3fvSUN",
  "glTexCoord4fColor4fNormal3fVertex4fSUN",
  "glTexCoord4fColor4fNormal3fVertex4fvSUN",
  "glTexCoord4fVertex4fSUN",
  "glTexCoord4fVertex4fvSUN",
];
