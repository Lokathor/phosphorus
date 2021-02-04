pub const GL_3DFX_tbuffer_COMMANDS: &[&str] = &["glTbufferMask3DFX"];

pub const GL_AMD_debug_output_COMMANDS: &[&str] = &["glDebugMessageEnableAMD", "glDebugMessageInsertAMD", "glDebugMessageCallbackAMD", "glGetDebugMessageLogAMD"];

pub const GL_AMD_draw_buffers_blend_COMMANDS: &[&str] = &["glBlendFuncIndexedAMD", "glBlendFuncSeparateIndexedAMD", "glBlendEquationIndexedAMD", "glBlendEquationSeparateIndexedAMD"];

pub const GL_AMD_framebuffer_multisample_advanced_COMMANDS: &[&str] = &["glRenderbufferStorageMultisampleAdvancedAMD", "glNamedRenderbufferStorageMultisampleAdvancedAMD"];

pub const GL_AMD_framebuffer_sample_positions_COMMANDS: &[&str] = &["glFramebufferSamplePositionsfvAMD", "glNamedFramebufferSamplePositionsfvAMD", "glGetFramebufferParameterfvAMD", "glGetNamedFramebufferParameterfvAMD"];

pub const GL_AMD_gpu_shader_int64_COMMANDS: &[&str] = &[
  "glUniform1i64NV",
  "glUniform2i64NV",
  "glUniform3i64NV",
  "glUniform4i64NV",
  "glUniform1i64vNV",
  "glUniform2i64vNV",
  "glUniform3i64vNV",
  "glUniform4i64vNV",
  "glUniform1ui64NV",
  "glUniform2ui64NV",
  "glUniform3ui64NV",
  "glUniform4ui64NV",
  "glUniform1ui64vNV",
  "glUniform2ui64vNV",
  "glUniform3ui64vNV",
  "glUniform4ui64vNV",
  "glGetUniformi64vNV",
  "glGetUniformui64vNV",
  "glProgramUniform1i64NV",
  "glProgramUniform2i64NV",
  "glProgramUniform3i64NV",
  "glProgramUniform4i64NV",
  "glProgramUniform1i64vNV",
  "glProgramUniform2i64vNV",
  "glProgramUniform3i64vNV",
  "glProgramUniform4i64vNV",
  "glProgramUniform1ui64NV",
  "glProgramUniform2ui64NV",
  "glProgramUniform3ui64NV",
  "glProgramUniform4ui64NV",
  "glProgramUniform1ui64vNV",
  "glProgramUniform2ui64vNV",
  "glProgramUniform3ui64vNV",
  "glProgramUniform4ui64vNV",
];

pub const GL_AMD_interleaved_elements_COMMANDS: &[&str] = &["glVertexAttribParameteriAMD"];

pub const GL_AMD_multi_draw_indirect_COMMANDS: &[&str] = &["glMultiDrawArraysIndirectAMD", "glMultiDrawElementsIndirectAMD"];

pub const GL_AMD_name_gen_delete_COMMANDS: &[&str] = &["glGenNamesAMD", "glDeleteNamesAMD", "glIsNameAMD"];

pub const GL_AMD_occlusion_query_event_COMMANDS: &[&str] = &["glQueryObjectParameteruiAMD"];

pub const GL_AMD_performance_monitor_COMMANDS: &[&str] = &["glGetPerfMonitorGroupsAMD", "glGetPerfMonitorCountersAMD", "glGetPerfMonitorGroupStringAMD", "glGetPerfMonitorCounterStringAMD", "glGetPerfMonitorCounterInfoAMD", "glGenPerfMonitorsAMD", "glDeletePerfMonitorsAMD", "glSelectPerfMonitorCountersAMD", "glBeginPerfMonitorAMD", "glEndPerfMonitorAMD", "glGetPerfMonitorCounterDataAMD"];

pub const GL_AMD_sample_positions_COMMANDS: &[&str] = &["glSetMultisamplefvAMD"];

pub const GL_AMD_sparse_texture_COMMANDS: &[&str] = &["glTexStorageSparseAMD", "glTextureStorageSparseAMD"];

pub const GL_AMD_stencil_operation_extended_COMMANDS: &[&str] = &["glStencilOpValueAMD"];

pub const GL_AMD_vertex_shader_tessellator_COMMANDS: &[&str] = &["glTessellationFactorAMD", "glTessellationModeAMD"];

pub const GL_ANGLE_framebuffer_blit_COMMANDS: &[&str] = &["glBlitFramebufferANGLE"];

pub const GL_ANGLE_framebuffer_multisample_COMMANDS: &[&str] = &["glRenderbufferStorageMultisampleANGLE"];

pub const GL_ANGLE_instanced_arrays_COMMANDS: &[&str] = &["glDrawArraysInstancedANGLE", "glDrawElementsInstancedANGLE", "glVertexAttribDivisorANGLE"];

pub const GL_ANGLE_translated_shader_source_COMMANDS: &[&str] = &["glGetTranslatedShaderSourceANGLE"];

pub const GL_APPLE_copy_texture_levels_COMMANDS: &[&str] = &["glCopyTextureLevelsAPPLE"];

pub const GL_APPLE_element_array_COMMANDS: &[&str] = &["glElementPointerAPPLE", "glDrawElementArrayAPPLE", "glDrawRangeElementArrayAPPLE", "glMultiDrawElementArrayAPPLE", "glMultiDrawRangeElementArrayAPPLE"];

pub const GL_APPLE_fence_COMMANDS: &[&str] = &["glGenFencesAPPLE", "glDeleteFencesAPPLE", "glSetFenceAPPLE", "glIsFenceAPPLE", "glTestFenceAPPLE", "glFinishFenceAPPLE", "glTestObjectAPPLE", "glFinishObjectAPPLE"];

pub const GL_APPLE_flush_buffer_range_COMMANDS: &[&str] = &["glBufferParameteriAPPLE", "glFlushMappedBufferRangeAPPLE"];

pub const GL_APPLE_framebuffer_multisample_COMMANDS: &[&str] = &["glRenderbufferStorageMultisampleAPPLE", "glResolveMultisampleFramebufferAPPLE"];

pub const GL_APPLE_object_purgeable_COMMANDS: &[&str] = &["glObjectPurgeableAPPLE", "glObjectUnpurgeableAPPLE", "glGetObjectParameterivAPPLE"];

pub const GL_APPLE_sync_COMMANDS: &[&str] = &["glFenceSyncAPPLE", "glIsSyncAPPLE", "glDeleteSyncAPPLE", "glClientWaitSyncAPPLE", "glWaitSyncAPPLE", "glGetInteger64vAPPLE", "glGetSyncivAPPLE"];

pub const GL_APPLE_texture_range_COMMANDS: &[&str] = &["glTextureRangeAPPLE", "glGetTexParameterPointervAPPLE"];

pub const GL_APPLE_vertex_array_object_COMMANDS: &[&str] = &["glBindVertexArrayAPPLE", "glDeleteVertexArraysAPPLE", "glGenVertexArraysAPPLE", "glIsVertexArrayAPPLE"];

pub const GL_APPLE_vertex_array_range_COMMANDS: &[&str] = &["glVertexArrayRangeAPPLE", "glFlushVertexArrayRangeAPPLE", "glVertexArrayParameteriAPPLE"];

pub const GL_APPLE_vertex_program_evaluators_COMMANDS: &[&str] = &["glEnableVertexAttribAPPLE", "glDisableVertexAttribAPPLE", "glIsVertexAttribEnabledAPPLE", "glMapVertexAttrib1dAPPLE", "glMapVertexAttrib1fAPPLE", "glMapVertexAttrib2dAPPLE", "glMapVertexAttrib2fAPPLE"];

pub const GL_ARB_ES2_compatibility_COMMANDS: &[&str] = &["glReleaseShaderCompiler", "glShaderBinary", "glGetShaderPrecisionFormat", "glDepthRangef", "glClearDepthf"];

pub const GL_ARB_ES3_1_compatibility_COMMANDS: &[&str] = &["glMemoryBarrierByRegion"];

pub const GL_ARB_ES3_2_compatibility_COMMANDS: &[&str] = &["glPrimitiveBoundingBoxARB"];

pub const GL_ARB_base_instance_COMMANDS: &[&str] = &["glDrawArraysInstancedBaseInstance", "glDrawElementsInstancedBaseInstance", "glDrawElementsInstancedBaseVertexBaseInstance"];

pub const GL_ARB_bindless_texture_COMMANDS: &[&str] = &["glGetTextureHandleARB", "glGetTextureSamplerHandleARB", "glMakeTextureHandleResidentARB", "glMakeTextureHandleNonResidentARB", "glGetImageHandleARB", "glMakeImageHandleResidentARB", "glMakeImageHandleNonResidentARB", "glUniformHandleui64ARB", "glUniformHandleui64vARB", "glProgramUniformHandleui64ARB", "glProgramUniformHandleui64vARB", "glIsTextureHandleResidentARB", "glIsImageHandleResidentARB", "glVertexAttribL1ui64ARB", "glVertexAttribL1ui64vARB", "glGetVertexAttribLui64vARB"];

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

pub const GL_ARB_debug_output_COMMANDS: &[&str] = &["glDebugMessageControlARB", "glDebugMessageInsertARB", "glDebugMessageCallbackARB", "glGetDebugMessageLogARB"];

pub const GL_ARB_direct_state_access_COMMANDS: &[&str] = &[
  "glCreateTransformFeedbacks",
  "glTransformFeedbackBufferBase",
  "glTransformFeedbackBufferRange",
  "glGetTransformFeedbackiv",
  "glGetTransformFeedbacki_v",
  "glGetTransformFeedbacki64_v",
  "glCreateBuffers",
  "glNamedBufferStorage",
  "glNamedBufferData",
  "glNamedBufferSubData",
  "glCopyNamedBufferSubData",
  "glClearNamedBufferData",
  "glClearNamedBufferSubData",
  "glMapNamedBuffer",
  "glMapNamedBufferRange",
  "glUnmapNamedBuffer",
  "glFlushMappedNamedBufferRange",
  "glGetNamedBufferParameteriv",
  "glGetNamedBufferParameteri64v",
  "glGetNamedBufferPointerv",
  "glGetNamedBufferSubData",
  "glCreateFramebuffers",
  "glNamedFramebufferRenderbuffer",
  "glNamedFramebufferParameteri",
  "glNamedFramebufferTexture",
  "glNamedFramebufferTextureLayer",
  "glNamedFramebufferDrawBuffer",
  "glNamedFramebufferDrawBuffers",
  "glNamedFramebufferReadBuffer",
  "glInvalidateNamedFramebufferData",
  "glInvalidateNamedFramebufferSubData",
  "glClearNamedFramebufferiv",
  "glClearNamedFramebufferuiv",
  "glClearNamedFramebufferfv",
  "glClearNamedFramebufferfi",
  "glBlitNamedFramebuffer",
  "glCheckNamedFramebufferStatus",
  "glGetNamedFramebufferParameteriv",
  "glGetNamedFramebufferAttachmentParameteriv",
  "glCreateRenderbuffers",
  "glNamedRenderbufferStorage",
  "glNamedRenderbufferStorageMultisample",
  "glGetNamedRenderbufferParameteriv",
  "glCreateTextures",
  "glTextureBuffer",
  "glTextureBufferRange",
  "glTextureStorage1D",
  "glTextureStorage2D",
  "glTextureStorage3D",
  "glTextureStorage2DMultisample",
  "glTextureStorage3DMultisample",
  "glTextureSubImage1D",
  "glTextureSubImage2D",
  "glTextureSubImage3D",
  "glCompressedTextureSubImage1D",
  "glCompressedTextureSubImage2D",
  "glCompressedTextureSubImage3D",
  "glCopyTextureSubImage1D",
  "glCopyTextureSubImage2D",
  "glCopyTextureSubImage3D",
  "glTextureParameterf",
  "glTextureParameterfv",
  "glTextureParameteri",
  "glTextureParameterIiv",
  "glTextureParameterIuiv",
  "glTextureParameteriv",
  "glGenerateTextureMipmap",
  "glBindTextureUnit",
  "glGetTextureImage",
  "glGetCompressedTextureImage",
  "glGetTextureLevelParameterfv",
  "glGetTextureLevelParameteriv",
  "glGetTextureParameterfv",
  "glGetTextureParameterIiv",
  "glGetTextureParameterIuiv",
  "glGetTextureParameteriv",
  "glCreateVertexArrays",
  "glDisableVertexArrayAttrib",
  "glEnableVertexArrayAttrib",
  "glVertexArrayElementBuffer",
  "glVertexArrayVertexBuffer",
  "glVertexArrayVertexBuffers",
  "glVertexArrayAttribBinding",
  "glVertexArrayAttribFormat",
  "glVertexArrayAttribIFormat",
  "glVertexArrayAttribLFormat",
  "glVertexArrayBindingDivisor",
  "glGetVertexArrayiv",
  "glGetVertexArrayIndexediv",
  "glGetVertexArrayIndexed64iv",
  "glCreateSamplers",
  "glCreateProgramPipelines",
  "glCreateQueries",
  "glGetQueryBufferObjecti64v",
  "glGetQueryBufferObjectiv",
  "glGetQueryBufferObjectui64v",
  "glGetQueryBufferObjectuiv",
];

pub const GL_ARB_draw_buffers_COMMANDS: &[&str] = &["glDrawBuffersARB"];

pub const GL_ARB_draw_buffers_blend_COMMANDS: &[&str] = &["glBlendEquationiARB", "glBlendEquationSeparateiARB", "glBlendFunciARB", "glBlendFuncSeparateiARB"];

pub const GL_ARB_draw_elements_base_vertex_COMMANDS: &[&str] = &["glDrawElementsBaseVertex", "glDrawRangeElementsBaseVertex", "glDrawElementsInstancedBaseVertex", "glMultiDrawElementsBaseVertex"];

pub const GL_ARB_draw_indirect_COMMANDS: &[&str] = &["glDrawArraysIndirect", "glDrawElementsIndirect"];

pub const GL_ARB_draw_instanced_COMMANDS: &[&str] = &["glDrawArraysInstancedARB", "glDrawElementsInstancedARB"];

pub const GL_ARB_fragment_program_COMMANDS: &[&str] = &["glProgramStringARB", "glBindProgramARB", "glDeleteProgramsARB", "glGenProgramsARB", "glProgramEnvParameter4dARB", "glProgramEnvParameter4dvARB", "glProgramEnvParameter4fARB", "glProgramEnvParameter4fvARB", "glProgramLocalParameter4dARB", "glProgramLocalParameter4dvARB", "glProgramLocalParameter4fARB", "glProgramLocalParameter4fvARB", "glGetProgramEnvParameterdvARB", "glGetProgramEnvParameterfvARB", "glGetProgramLocalParameterdvARB", "glGetProgramLocalParameterfvARB", "glGetProgramivARB", "glGetProgramStringARB", "glIsProgramARB"];

pub const GL_ARB_framebuffer_no_attachments_COMMANDS: &[&str] = &["glFramebufferParameteri", "glGetFramebufferParameteriv"];

pub const GL_ARB_framebuffer_object_COMMANDS: &[&str] = &["glIsRenderbuffer", "glBindRenderbuffer", "glDeleteRenderbuffers", "glGenRenderbuffers", "glRenderbufferStorage", "glGetRenderbufferParameteriv", "glIsFramebuffer", "glBindFramebuffer", "glDeleteFramebuffers", "glGenFramebuffers", "glCheckFramebufferStatus", "glFramebufferTexture1D", "glFramebufferTexture2D", "glFramebufferTexture3D", "glFramebufferRenderbuffer", "glGetFramebufferAttachmentParameteriv", "glGenerateMipmap", "glBlitFramebuffer", "glRenderbufferStorageMultisample", "glFramebufferTextureLayer"];

pub const GL_ARB_geometry_shader4_COMMANDS: &[&str] = &["glProgramParameteriARB", "glFramebufferTextureARB", "glFramebufferTextureLayerARB", "glFramebufferTextureFaceARB"];

pub const GL_ARB_get_program_binary_COMMANDS: &[&str] = &["glGetProgramBinary", "glProgramBinary", "glProgramParameteri"];

pub const GL_ARB_get_texture_sub_image_COMMANDS: &[&str] = &["glGetTextureSubImage", "glGetCompressedTextureSubImage"];

pub const GL_ARB_gl_spirv_COMMANDS: &[&str] = &["glSpecializeShaderARB"];

pub const GL_ARB_gpu_shader_fp64_COMMANDS: &[&str] = &["glUniform1d", "glUniform2d", "glUniform3d", "glUniform4d", "glUniform1dv", "glUniform2dv", "glUniform3dv", "glUniform4dv", "glUniformMatrix2dv", "glUniformMatrix3dv", "glUniformMatrix4dv", "glUniformMatrix2x3dv", "glUniformMatrix2x4dv", "glUniformMatrix3x2dv", "glUniformMatrix3x4dv", "glUniformMatrix4x2dv", "glUniformMatrix4x3dv", "glGetUniformdv"];

pub const GL_ARB_gpu_shader_int64_COMMANDS: &[&str] = &[
  "glUniform1i64ARB",
  "glUniform2i64ARB",
  "glUniform3i64ARB",
  "glUniform4i64ARB",
  "glUniform1i64vARB",
  "glUniform2i64vARB",
  "glUniform3i64vARB",
  "glUniform4i64vARB",
  "glUniform1ui64ARB",
  "glUniform2ui64ARB",
  "glUniform3ui64ARB",
  "glUniform4ui64ARB",
  "glUniform1ui64vARB",
  "glUniform2ui64vARB",
  "glUniform3ui64vARB",
  "glUniform4ui64vARB",
  "glGetUniformi64vARB",
  "glGetUniformui64vARB",
  "glGetnUniformi64vARB",
  "glGetnUniformui64vARB",
  "glProgramUniform1i64ARB",
  "glProgramUniform2i64ARB",
  "glProgramUniform3i64ARB",
  "glProgramUniform4i64ARB",
  "glProgramUniform1i64vARB",
  "glProgramUniform2i64vARB",
  "glProgramUniform3i64vARB",
  "glProgramUniform4i64vARB",
  "glProgramUniform1ui64ARB",
  "glProgramUniform2ui64ARB",
  "glProgramUniform3ui64ARB",
  "glProgramUniform4ui64ARB",
  "glProgramUniform1ui64vARB",
  "glProgramUniform2ui64vARB",
  "glProgramUniform3ui64vARB",
  "glProgramUniform4ui64vARB",
];

pub const GL_ARB_imaging_COMMANDS: &[&str] = &[
  "glBlendColor",
  "glBlendEquation",
  "glColorTable",
  "glColorTableParameterfv",
  "glColorTableParameteriv",
  "glCopyColorTable",
  "glGetColorTable",
  "glGetColorTableParameterfv",
  "glGetColorTableParameteriv",
  "glColorSubTable",
  "glCopyColorSubTable",
  "glConvolutionFilter1D",
  "glConvolutionFilter2D",
  "glConvolutionParameterf",
  "glConvolutionParameterfv",
  "glConvolutionParameteri",
  "glConvolutionParameteriv",
  "glCopyConvolutionFilter1D",
  "glCopyConvolutionFilter2D",
  "glGetConvolutionFilter",
  "glGetConvolutionParameterfv",
  "glGetConvolutionParameteriv",
  "glGetSeparableFilter",
  "glSeparableFilter2D",
  "glGetHistogram",
  "glGetHistogramParameterfv",
  "glGetHistogramParameteriv",
  "glGetMinmax",
  "glGetMinmaxParameterfv",
  "glGetMinmaxParameteriv",
  "glHistogram",
  "glMinmax",
  "glResetHistogram",
  "glResetMinmax",
];

pub const GL_ARB_indirect_parameters_COMMANDS: &[&str] = &["glMultiDrawArraysIndirectCountARB", "glMultiDrawElementsIndirectCountARB"];

pub const GL_ARB_instanced_arrays_COMMANDS: &[&str] = &["glVertexAttribDivisorARB"];

pub const GL_ARB_internalformat_query_COMMANDS: &[&str] = &["glGetInternalformativ"];

pub const GL_ARB_internalformat_query2_COMMANDS: &[&str] = &["glGetInternalformati64v"];

pub const GL_ARB_invalidate_subdata_COMMANDS: &[&str] = &["glInvalidateTexSubImage", "glInvalidateTexImage", "glInvalidateBufferSubData", "glInvalidateBufferData", "glInvalidateFramebuffer", "glInvalidateSubFramebuffer"];

pub const GL_ARB_map_buffer_range_COMMANDS: &[&str] = &["glMapBufferRange", "glFlushMappedBufferRange"];

pub const GL_ARB_matrix_palette_COMMANDS: &[&str] = &["glCurrentPaletteMatrixARB", "glMatrixIndexubvARB", "glMatrixIndexusvARB", "glMatrixIndexuivARB", "glMatrixIndexPointerARB"];

pub const GL_ARB_multi_bind_COMMANDS: &[&str] = &["glBindBuffersBase", "glBindBuffersRange", "glBindTextures", "glBindSamplers", "glBindImageTextures", "glBindVertexBuffers"];

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

pub const GL_ARB_occlusion_query_COMMANDS: &[&str] = &["glGenQueriesARB", "glDeleteQueriesARB", "glIsQueryARB", "glBeginQueryARB", "glEndQueryARB", "glGetQueryivARB", "glGetQueryObjectivARB", "glGetQueryObjectuivARB"];

pub const GL_ARB_parallel_shader_compile_COMMANDS: &[&str] = &["glMaxShaderCompilerThreadsARB"];

pub const GL_ARB_point_parameters_COMMANDS: &[&str] = &["glPointParameterfARB", "glPointParameterfvARB"];

pub const GL_ARB_polygon_offset_clamp_COMMANDS: &[&str] = &["glPolygonOffsetClamp"];

pub const GL_ARB_program_interface_query_COMMANDS: &[&str] = &["glGetProgramInterfaceiv", "glGetProgramResourceIndex", "glGetProgramResourceName", "glGetProgramResourceiv", "glGetProgramResourceLocation", "glGetProgramResourceLocationIndex"];

pub const GL_ARB_provoking_vertex_COMMANDS: &[&str] = &["glProvokingVertex"];

pub const GL_ARB_robustness_COMMANDS: &[&str] = &["glGetGraphicsResetStatusARB", "glGetnTexImageARB", "glReadnPixelsARB", "glGetnCompressedTexImageARB", "glGetnUniformfvARB", "glGetnUniformivARB", "glGetnUniformuivARB", "glGetnUniformdvARB", "glGetnMapdvARB", "glGetnMapfvARB", "glGetnMapivARB", "glGetnPixelMapfvARB", "glGetnPixelMapuivARB", "glGetnPixelMapusvARB", "glGetnPolygonStippleARB", "glGetnColorTableARB", "glGetnConvolutionFilterARB", "glGetnSeparableFilterARB", "glGetnHistogramARB", "glGetnMinmaxARB"];

pub const GL_ARB_sample_locations_COMMANDS: &[&str] = &["glFramebufferSampleLocationsfvARB", "glNamedFramebufferSampleLocationsfvARB", "glEvaluateDepthValuesARB"];

pub const GL_ARB_sample_shading_COMMANDS: &[&str] = &["glMinSampleShadingARB"];

pub const GL_ARB_sampler_objects_COMMANDS: &[&str] = &["glGenSamplers", "glDeleteSamplers", "glIsSampler", "glBindSampler", "glSamplerParameteri", "glSamplerParameteriv", "glSamplerParameterf", "glSamplerParameterfv", "glSamplerParameterIiv", "glSamplerParameterIuiv", "glGetSamplerParameteriv", "glGetSamplerParameterIiv", "glGetSamplerParameterfv", "glGetSamplerParameterIuiv"];

pub const GL_ARB_separate_shader_objects_COMMANDS: &[&str] = &[
  "glUseProgramStages",
  "glActiveShaderProgram",
  "glCreateShaderProgramv",
  "glBindProgramPipeline",
  "glDeleteProgramPipelines",
  "glGenProgramPipelines",
  "glIsProgramPipeline",
  "glGetProgramPipelineiv",
  "glProgramParameteri",
  "glProgramUniform1i",
  "glProgramUniform1iv",
  "glProgramUniform1f",
  "glProgramUniform1fv",
  "glProgramUniform1d",
  "glProgramUniform1dv",
  "glProgramUniform1ui",
  "glProgramUniform1uiv",
  "glProgramUniform2i",
  "glProgramUniform2iv",
  "glProgramUniform2f",
  "glProgramUniform2fv",
  "glProgramUniform2d",
  "glProgramUniform2dv",
  "glProgramUniform2ui",
  "glProgramUniform2uiv",
  "glProgramUniform3i",
  "glProgramUniform3iv",
  "glProgramUniform3f",
  "glProgramUniform3fv",
  "glProgramUniform3d",
  "glProgramUniform3dv",
  "glProgramUniform3ui",
  "glProgramUniform3uiv",
  "glProgramUniform4i",
  "glProgramUniform4iv",
  "glProgramUniform4f",
  "glProgramUniform4fv",
  "glProgramUniform4d",
  "glProgramUniform4dv",
  "glProgramUniform4ui",
  "glProgramUniform4uiv",
  "glProgramUniformMatrix2fv",
  "glProgramUniformMatrix3fv",
  "glProgramUniformMatrix4fv",
  "glProgramUniformMatrix2dv",
  "glProgramUniformMatrix3dv",
  "glProgramUniformMatrix4dv",
  "glProgramUniformMatrix2x3fv",
  "glProgramUniformMatrix3x2fv",
  "glProgramUniformMatrix2x4fv",
  "glProgramUniformMatrix4x2fv",
  "glProgramUniformMatrix3x4fv",
  "glProgramUniformMatrix4x3fv",
  "glProgramUniformMatrix2x3dv",
  "glProgramUniformMatrix3x2dv",
  "glProgramUniformMatrix2x4dv",
  "glProgramUniformMatrix4x2dv",
  "glProgramUniformMatrix3x4dv",
  "glProgramUniformMatrix4x3dv",
  "glValidateProgramPipeline",
  "glGetProgramPipelineInfoLog",
];

pub const GL_ARB_shader_atomic_counters_COMMANDS: &[&str] = &["glGetActiveAtomicCounterBufferiv"];

pub const GL_ARB_shader_image_load_store_COMMANDS: &[&str] = &["glBindImageTexture", "glMemoryBarrier"];

pub const GL_ARB_shader_objects_COMMANDS: &[&str] = &[
  "glDeleteObjectARB",
  "glGetHandleARB",
  "glDetachObjectARB",
  "glCreateShaderObjectARB",
  "glShaderSourceARB",
  "glCompileShaderARB",
  "glCreateProgramObjectARB",
  "glAttachObjectARB",
  "glLinkProgramARB",
  "glUseProgramObjectARB",
  "glValidateProgramARB",
  "glUniform1fARB",
  "glUniform2fARB",
  "glUniform3fARB",
  "glUniform4fARB",
  "glUniform1iARB",
  "glUniform2iARB",
  "glUniform3iARB",
  "glUniform4iARB",
  "glUniform1fvARB",
  "glUniform2fvARB",
  "glUniform3fvARB",
  "glUniform4fvARB",
  "glUniform1ivARB",
  "glUniform2ivARB",
  "glUniform3ivARB",
  "glUniform4ivARB",
  "glUniformMatrix2fvARB",
  "glUniformMatrix3fvARB",
  "glUniformMatrix4fvARB",
  "glGetObjectParameterfvARB",
  "glGetObjectParameterivARB",
  "glGetInfoLogARB",
  "glGetAttachedObjectsARB",
  "glGetUniformLocationARB",
  "glGetActiveUniformARB",
  "glGetUniformfvARB",
  "glGetUniformivARB",
  "glGetShaderSourceARB",
];

pub const GL_ARB_shader_storage_buffer_object_COMMANDS: &[&str] = &["glShaderStorageBlockBinding"];

pub const GL_ARB_shader_subroutine_COMMANDS: &[&str] = &["glGetSubroutineUniformLocation", "glGetSubroutineIndex", "glGetActiveSubroutineUniformiv", "glGetActiveSubroutineUniformName", "glGetActiveSubroutineName", "glUniformSubroutinesuiv", "glGetUniformSubroutineuiv", "glGetProgramStageiv"];

pub const GL_ARB_shading_language_include_COMMANDS: &[&str] = &["glNamedStringARB", "glDeleteNamedStringARB", "glCompileShaderIncludeARB", "glIsNamedStringARB", "glGetNamedStringARB", "glGetNamedStringivARB"];

pub const GL_ARB_sparse_buffer_COMMANDS: &[&str] = &["glBufferPageCommitmentARB", "glNamedBufferPageCommitmentEXT", "glNamedBufferPageCommitmentARB"];

pub const GL_ARB_sparse_texture_COMMANDS: &[&str] = &["glTexPageCommitmentARB"];

pub const GL_ARB_sync_COMMANDS: &[&str] = &["glFenceSync", "glIsSync", "glDeleteSync", "glClientWaitSync", "glWaitSync", "glGetInteger64v", "glGetSynciv"];

pub const GL_ARB_tessellation_shader_COMMANDS: &[&str] = &["glPatchParameteri", "glPatchParameterfv"];

pub const GL_ARB_texture_barrier_COMMANDS: &[&str] = &["glTextureBarrier"];

pub const GL_ARB_texture_buffer_object_COMMANDS: &[&str] = &["glTexBufferARB"];

pub const GL_ARB_texture_buffer_range_COMMANDS: &[&str] = &["glTexBufferRange"];

pub const GL_ARB_texture_compression_COMMANDS: &[&str] = &["glCompressedTexImage3DARB", "glCompressedTexImage2DARB", "glCompressedTexImage1DARB", "glCompressedTexSubImage3DARB", "glCompressedTexSubImage2DARB", "glCompressedTexSubImage1DARB", "glGetCompressedTexImageARB"];

pub const GL_ARB_texture_multisample_COMMANDS: &[&str] = &["glTexImage2DMultisample", "glTexImage3DMultisample", "glGetMultisamplefv", "glSampleMaski"];

pub const GL_ARB_texture_storage_COMMANDS: &[&str] = &["glTexStorage1D", "glTexStorage2D", "glTexStorage3D"];

pub const GL_ARB_texture_storage_multisample_COMMANDS: &[&str] = &["glTexStorage2DMultisample", "glTexStorage3DMultisample"];

pub const GL_ARB_texture_view_COMMANDS: &[&str] = &["glTextureView"];

pub const GL_ARB_timer_query_COMMANDS: &[&str] = &["glQueryCounter", "glGetQueryObjecti64v", "glGetQueryObjectui64v"];

pub const GL_ARB_transform_feedback2_COMMANDS: &[&str] = &["glBindTransformFeedback", "glDeleteTransformFeedbacks", "glGenTransformFeedbacks", "glIsTransformFeedback", "glPauseTransformFeedback", "glResumeTransformFeedback", "glDrawTransformFeedback"];

pub const GL_ARB_transform_feedback3_COMMANDS: &[&str] = &["glDrawTransformFeedbackStream", "glBeginQueryIndexed", "glEndQueryIndexed", "glGetQueryIndexediv"];

pub const GL_ARB_transform_feedback_instanced_COMMANDS: &[&str] = &["glDrawTransformFeedbackInstanced", "glDrawTransformFeedbackStreamInstanced"];

pub const GL_ARB_transpose_matrix_COMMANDS: &[&str] = &["glLoadTransposeMatrixfARB", "glLoadTransposeMatrixdARB", "glMultTransposeMatrixfARB", "glMultTransposeMatrixdARB"];

pub const GL_ARB_uniform_buffer_object_COMMANDS: &[&str] = &["glGetUniformIndices", "glGetActiveUniformsiv", "glGetActiveUniformName", "glGetUniformBlockIndex", "glGetActiveUniformBlockiv", "glGetActiveUniformBlockName", "glUniformBlockBinding", "glBindBufferRange", "glBindBufferBase", "glGetIntegeri_v"];

pub const GL_ARB_vertex_array_object_COMMANDS: &[&str] = &["glBindVertexArray", "glDeleteVertexArrays", "glGenVertexArrays", "glIsVertexArray"];

pub const GL_ARB_vertex_attrib_64bit_COMMANDS: &[&str] = &["glVertexAttribL1d", "glVertexAttribL2d", "glVertexAttribL3d", "glVertexAttribL4d", "glVertexAttribL1dv", "glVertexAttribL2dv", "glVertexAttribL3dv", "glVertexAttribL4dv", "glVertexAttribLPointer", "glGetVertexAttribLdv"];

pub const GL_ARB_vertex_attrib_binding_COMMANDS: &[&str] = &["glBindVertexBuffer", "glVertexAttribFormat", "glVertexAttribIFormat", "glVertexAttribLFormat", "glVertexAttribBinding", "glVertexBindingDivisor"];

pub const GL_ARB_vertex_blend_COMMANDS: &[&str] = &["glWeightbvARB", "glWeightsvARB", "glWeightivARB", "glWeightfvARB", "glWeightdvARB", "glWeightubvARB", "glWeightusvARB", "glWeightuivARB", "glWeightPointerARB", "glVertexBlendARB"];

pub const GL_ARB_vertex_buffer_object_COMMANDS: &[&str] = &["glBindBufferARB", "glDeleteBuffersARB", "glGenBuffersARB", "glIsBufferARB", "glBufferDataARB", "glBufferSubDataARB", "glGetBufferSubDataARB", "glMapBufferARB", "glUnmapBufferARB", "glGetBufferParameterivARB", "glGetBufferPointervARB"];

pub const GL_ARB_vertex_program_COMMANDS: &[&str] = &[
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
  "glEnableVertexAttribArrayARB",
  "glDisableVertexAttribArrayARB",
  "glProgramStringARB",
  "glBindProgramARB",
  "glDeleteProgramsARB",
  "glGenProgramsARB",
  "glProgramEnvParameter4dARB",
  "glProgramEnvParameter4dvARB",
  "glProgramEnvParameter4fARB",
  "glProgramEnvParameter4fvARB",
  "glProgramLocalParameter4dARB",
  "glProgramLocalParameter4dvARB",
  "glProgramLocalParameter4fARB",
  "glProgramLocalParameter4fvARB",
  "glGetProgramEnvParameterdvARB",
  "glGetProgramEnvParameterfvARB",
  "glGetProgramLocalParameterdvARB",
  "glGetProgramLocalParameterfvARB",
  "glGetProgramivARB",
  "glGetProgramStringARB",
  "glGetVertexAttribdvARB",
  "glGetVertexAttribfvARB",
  "glGetVertexAttribivARB",
  "glGetVertexAttribPointervARB",
  "glIsProgramARB",
];

pub const GL_ARB_vertex_shader_COMMANDS: &[&str] = &[
  "glVertexAttrib1fARB",
  "glVertexAttrib1sARB",
  "glVertexAttrib1dARB",
  "glVertexAttrib2fARB",
  "glVertexAttrib2sARB",
  "glVertexAttrib2dARB",
  "glVertexAttrib3fARB",
  "glVertexAttrib3sARB",
  "glVertexAttrib3dARB",
  "glVertexAttrib4fARB",
  "glVertexAttrib4sARB",
  "glVertexAttrib4dARB",
  "glVertexAttrib4NubARB",
  "glVertexAttrib1fvARB",
  "glVertexAttrib1svARB",
  "glVertexAttrib1dvARB",
  "glVertexAttrib2fvARB",
  "glVertexAttrib2svARB",
  "glVertexAttrib2dvARB",
  "glVertexAttrib3fvARB",
  "glVertexAttrib3svARB",
  "glVertexAttrib3dvARB",
  "glVertexAttrib4fvARB",
  "glVertexAttrib4svARB",
  "glVertexAttrib4dvARB",
  "glVertexAttrib4ivARB",
  "glVertexAttrib4bvARB",
  "glVertexAttrib4ubvARB",
  "glVertexAttrib4usvARB",
  "glVertexAttrib4uivARB",
  "glVertexAttrib4NbvARB",
  "glVertexAttrib4NsvARB",
  "glVertexAttrib4NivARB",
  "glVertexAttrib4NubvARB",
  "glVertexAttrib4NusvARB",
  "glVertexAttrib4NuivARB",
  "glVertexAttribPointerARB",
  "glEnableVertexAttribArrayARB",
  "glDisableVertexAttribArrayARB",
  "glBindAttribLocationARB",
  "glGetActiveAttribARB",
  "glGetAttribLocationARB",
  "glGetVertexAttribdvARB",
  "glGetVertexAttribfvARB",
  "glGetVertexAttribivARB",
  "glGetVertexAttribPointervARB",
];

pub const GL_ARB_vertex_type_2_10_10_10_rev_COMMANDS: &[&str] =
  &["glVertexAttribP1ui", "glVertexAttribP1uiv", "glVertexAttribP2ui", "glVertexAttribP2uiv", "glVertexAttribP3ui", "glVertexAttribP3uiv", "glVertexAttribP4ui", "glVertexAttribP4uiv", "glVertexP2ui", "glVertexP2uiv", "glVertexP3ui", "glVertexP3uiv", "glVertexP4ui", "glVertexP4uiv", "glTexCoordP1ui", "glTexCoordP1uiv", "glTexCoordP2ui", "glTexCoordP2uiv", "glTexCoordP3ui", "glTexCoordP3uiv", "glTexCoordP4ui", "glTexCoordP4uiv", "glMultiTexCoordP1ui", "glMultiTexCoordP1uiv", "glMultiTexCoordP2ui", "glMultiTexCoordP2uiv", "glMultiTexCoordP3ui", "glMultiTexCoordP3uiv", "glMultiTexCoordP4ui", "glMultiTexCoordP4uiv", "glNormalP3ui", "glNormalP3uiv", "glColorP3ui", "glColorP3uiv", "glColorP4ui", "glColorP4uiv", "glSecondaryColorP3ui", "glSecondaryColorP3uiv"];

pub const GL_ARB_viewport_array_COMMANDS: &[&str] = &["glViewportArrayv", "glViewportIndexedf", "glViewportIndexedfv", "glScissorArrayv", "glScissorIndexed", "glScissorIndexedv", "glDepthRangeArrayv", "glDepthRangeIndexed", "glGetFloati_v", "glGetDoublei_v", "glDepthRangeArraydvNV", "glDepthRangeIndexeddNV"];

pub const GL_ARB_window_pos_COMMANDS: &[&str] = &["glWindowPos2dARB", "glWindowPos2dvARB", "glWindowPos2fARB", "glWindowPos2fvARB", "glWindowPos2iARB", "glWindowPos2ivARB", "glWindowPos2sARB", "glWindowPos2svARB", "glWindowPos3dARB", "glWindowPos3dvARB", "glWindowPos3fARB", "glWindowPos3fvARB", "glWindowPos3iARB", "glWindowPos3ivARB", "glWindowPos3sARB", "glWindowPos3svARB"];

pub const GL_ATI_draw_buffers_COMMANDS: &[&str] = &["glDrawBuffersATI"];

pub const GL_ATI_element_array_COMMANDS: &[&str] = &["glElementPointerATI", "glDrawElementArrayATI", "glDrawRangeElementArrayATI"];

pub const GL_ATI_envmap_bumpmap_COMMANDS: &[&str] = &["glTexBumpParameterivATI", "glTexBumpParameterfvATI", "glGetTexBumpParameterivATI", "glGetTexBumpParameterfvATI"];

pub const GL_ATI_fragment_shader_COMMANDS: &[&str] = &["glGenFragmentShadersATI", "glBindFragmentShaderATI", "glDeleteFragmentShaderATI", "glBeginFragmentShaderATI", "glEndFragmentShaderATI", "glPassTexCoordATI", "glSampleMapATI", "glColorFragmentOp1ATI", "glColorFragmentOp2ATI", "glColorFragmentOp3ATI", "glAlphaFragmentOp1ATI", "glAlphaFragmentOp2ATI", "glAlphaFragmentOp3ATI", "glSetFragmentShaderConstantATI"];

pub const GL_ATI_map_object_buffer_COMMANDS: &[&str] = &["glMapObjectBufferATI", "glUnmapObjectBufferATI"];

pub const GL_ATI_pn_triangles_COMMANDS: &[&str] = &["glPNTrianglesiATI", "glPNTrianglesfATI"];

pub const GL_ATI_separate_stencil_COMMANDS: &[&str] = &["glStencilOpSeparateATI", "glStencilFuncSeparateATI"];

pub const GL_ATI_vertex_array_object_COMMANDS: &[&str] = &["glNewObjectBufferATI", "glIsObjectBufferATI", "glUpdateObjectBufferATI", "glGetObjectBufferfvATI", "glGetObjectBufferivATI", "glFreeObjectBufferATI", "glArrayObjectATI", "glGetArrayObjectfvATI", "glGetArrayObjectivATI", "glVariantArrayObjectATI", "glGetVariantArrayObjectfvATI", "glGetVariantArrayObjectivATI"];

pub const GL_ATI_vertex_attrib_array_object_COMMANDS: &[&str] = &["glVertexAttribArrayObjectATI", "glGetVertexAttribArrayObjectfvATI", "glGetVertexAttribArrayObjectivATI"];

pub const GL_ATI_vertex_streams_COMMANDS: &[&str] = &[
  "glVertexStream1sATI",
  "glVertexStream1svATI",
  "glVertexStream1iATI",
  "glVertexStream1ivATI",
  "glVertexStream1fATI",
  "glVertexStream1fvATI",
  "glVertexStream1dATI",
  "glVertexStream1dvATI",
  "glVertexStream2sATI",
  "glVertexStream2svATI",
  "glVertexStream2iATI",
  "glVertexStream2ivATI",
  "glVertexStream2fATI",
  "glVertexStream2fvATI",
  "glVertexStream2dATI",
  "glVertexStream2dvATI",
  "glVertexStream3sATI",
  "glVertexStream3svATI",
  "glVertexStream3iATI",
  "glVertexStream3ivATI",
  "glVertexStream3fATI",
  "glVertexStream3fvATI",
  "glVertexStream3dATI",
  "glVertexStream3dvATI",
  "glVertexStream4sATI",
  "glVertexStream4svATI",
  "glVertexStream4iATI",
  "glVertexStream4ivATI",
  "glVertexStream4fATI",
  "glVertexStream4fvATI",
  "glVertexStream4dATI",
  "glVertexStream4dvATI",
  "glNormalStream3bATI",
  "glNormalStream3bvATI",
  "glNormalStream3sATI",
  "glNormalStream3svATI",
  "glNormalStream3iATI",
  "glNormalStream3ivATI",
  "glNormalStream3fATI",
  "glNormalStream3fvATI",
  "glNormalStream3dATI",
  "glNormalStream3dvATI",
  "glClientActiveVertexStreamATI",
  "glVertexBlendEnviATI",
  "glVertexBlendEnvfATI",
];

pub const GL_EXT_EGL_image_storage_COMMANDS: &[&str] = &["glEGLImageTargetTexStorageEXT", "glEGLImageTargetTextureStorageEXT"];

pub const GL_EXT_base_instance_COMMANDS: &[&str] = &["glDrawArraysInstancedBaseInstanceEXT", "glDrawElementsInstancedBaseInstanceEXT", "glDrawElementsInstancedBaseVertexBaseInstanceEXT"];

pub const GL_EXT_bindable_uniform_COMMANDS: &[&str] = &["glUniformBufferEXT", "glGetUniformBufferSizeEXT", "glGetUniformOffsetEXT"];

pub const GL_EXT_blend_color_COMMANDS: &[&str] = &["glBlendColorEXT"];

pub const GL_EXT_blend_equation_separate_COMMANDS: &[&str] = &["glBlendEquationSeparateEXT"];

pub const GL_EXT_blend_func_extended_COMMANDS: &[&str] = &["glBindFragDataLocationIndexedEXT", "glBindFragDataLocationEXT", "glGetProgramResourceLocationIndexEXT", "glGetFragDataIndexEXT"];

pub const GL_EXT_blend_func_separate_COMMANDS: &[&str] = &["glBlendFuncSeparateEXT"];

pub const GL_EXT_blend_minmax_COMMANDS: &[&str] = &["glBlendEquationEXT"];

pub const GL_EXT_buffer_storage_COMMANDS: &[&str] = &["glBufferStorageEXT"];

pub const GL_EXT_clear_texture_COMMANDS: &[&str] = &["glClearTexImageEXT", "glClearTexSubImageEXT"];

pub const GL_EXT_clip_control_COMMANDS: &[&str] = &["glClipControlEXT"];

pub const GL_EXT_color_subtable_COMMANDS: &[&str] = &["glColorSubTableEXT", "glCopyColorSubTableEXT"];

pub const GL_EXT_compiled_vertex_array_COMMANDS: &[&str] = &["glLockArraysEXT", "glUnlockArraysEXT"];

pub const GL_EXT_convolution_COMMANDS: &[&str] = &["glConvolutionFilter1DEXT", "glConvolutionFilter2DEXT", "glConvolutionParameterfEXT", "glConvolutionParameterfvEXT", "glConvolutionParameteriEXT", "glConvolutionParameterivEXT", "glCopyConvolutionFilter1DEXT", "glCopyConvolutionFilter2DEXT", "glGetConvolutionFilterEXT", "glGetConvolutionParameterfvEXT", "glGetConvolutionParameterivEXT", "glGetSeparableFilterEXT", "glSeparableFilter2DEXT"];

pub const GL_EXT_coordinate_frame_COMMANDS: &[&str] = &["glTangent3bEXT", "glTangent3bvEXT", "glTangent3dEXT", "glTangent3dvEXT", "glTangent3fEXT", "glTangent3fvEXT", "glTangent3iEXT", "glTangent3ivEXT", "glTangent3sEXT", "glTangent3svEXT", "glBinormal3bEXT", "glBinormal3bvEXT", "glBinormal3dEXT", "glBinormal3dvEXT", "glBinormal3fEXT", "glBinormal3fvEXT", "glBinormal3iEXT", "glBinormal3ivEXT", "glBinormal3sEXT", "glBinormal3svEXT", "glTangentPointerEXT", "glBinormalPointerEXT"];

pub const GL_EXT_copy_image_COMMANDS: &[&str] = &["glCopyImageSubDataEXT"];

pub const GL_EXT_copy_texture_COMMANDS: &[&str] = &["glCopyTexImage1DEXT", "glCopyTexImage2DEXT", "glCopyTexSubImage1DEXT", "glCopyTexSubImage2DEXT", "glCopyTexSubImage3DEXT"];

pub const GL_EXT_cull_vertex_COMMANDS: &[&str] = &["glCullParameterdvEXT", "glCullParameterfvEXT"];

pub const GL_EXT_debug_label_COMMANDS: &[&str] = &["glLabelObjectEXT", "glGetObjectLabelEXT"];

pub const GL_EXT_debug_marker_COMMANDS: &[&str] = &["glInsertEventMarkerEXT", "glPushGroupMarkerEXT", "glPopGroupMarkerEXT"];

pub const GL_EXT_depth_bounds_test_COMMANDS: &[&str] = &["glDepthBoundsEXT"];

pub const GL_EXT_direct_state_access_COMMANDS: &[&str] = &[
  "glMatrixLoadfEXT",
  "glMatrixLoaddEXT",
  "glMatrixMultfEXT",
  "glMatrixMultdEXT",
  "glMatrixLoadIdentityEXT",
  "glMatrixRotatefEXT",
  "glMatrixRotatedEXT",
  "glMatrixScalefEXT",
  "glMatrixScaledEXT",
  "glMatrixTranslatefEXT",
  "glMatrixTranslatedEXT",
  "glMatrixFrustumEXT",
  "glMatrixOrthoEXT",
  "glMatrixPopEXT",
  "glMatrixPushEXT",
  "glClientAttribDefaultEXT",
  "glPushClientAttribDefaultEXT",
  "glTextureParameterfEXT",
  "glTextureParameterfvEXT",
  "glTextureParameteriEXT",
  "glTextureParameterivEXT",
  "glTextureImage1DEXT",
  "glTextureImage2DEXT",
  "glTextureSubImage1DEXT",
  "glTextureSubImage2DEXT",
  "glCopyTextureImage1DEXT",
  "glCopyTextureImage2DEXT",
  "glCopyTextureSubImage1DEXT",
  "glCopyTextureSubImage2DEXT",
  "glGetTextureImageEXT",
  "glGetTextureParameterfvEXT",
  "glGetTextureParameterivEXT",
  "glGetTextureLevelParameterfvEXT",
  "glGetTextureLevelParameterivEXT",
  "glTextureImage3DEXT",
  "glTextureSubImage3DEXT",
  "glCopyTextureSubImage3DEXT",
  "glBindMultiTextureEXT",
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
  "glGetMultiTexEnvfvEXT",
  "glGetMultiTexEnvivEXT",
  "glGetMultiTexGendvEXT",
  "glGetMultiTexGenfvEXT",
  "glGetMultiTexGenivEXT",
  "glMultiTexParameteriEXT",
  "glMultiTexParameterivEXT",
  "glMultiTexParameterfEXT",
  "glMultiTexParameterfvEXT",
  "glMultiTexImage1DEXT",
  "glMultiTexImage2DEXT",
  "glMultiTexSubImage1DEXT",
  "glMultiTexSubImage2DEXT",
  "glCopyMultiTexImage1DEXT",
  "glCopyMultiTexImage2DEXT",
  "glCopyMultiTexSubImage1DEXT",
  "glCopyMultiTexSubImage2DEXT",
  "glGetMultiTexImageEXT",
  "glGetMultiTexParameterfvEXT",
  "glGetMultiTexParameterivEXT",
  "glGetMultiTexLevelParameterfvEXT",
  "glGetMultiTexLevelParameterivEXT",
  "glMultiTexImage3DEXT",
  "glMultiTexSubImage3DEXT",
  "glCopyMultiTexSubImage3DEXT",
  "glEnableClientStateIndexedEXT",
  "glDisableClientStateIndexedEXT",
  "glGetFloatIndexedvEXT",
  "glGetDoubleIndexedvEXT",
  "glGetPointerIndexedvEXT",
  "glEnableIndexedEXT",
  "glDisableIndexedEXT",
  "glIsEnabledIndexedEXT",
  "glGetIntegerIndexedvEXT",
  "glGetBooleanIndexedvEXT",
  "glCompressedTextureImage3DEXT",
  "glCompressedTextureImage2DEXT",
  "glCompressedTextureImage1DEXT",
  "glCompressedTextureSubImage3DEXT",
  "glCompressedTextureSubImage2DEXT",
  "glCompressedTextureSubImage1DEXT",
  "glGetCompressedTextureImageEXT",
  "glCompressedMultiTexImage3DEXT",
  "glCompressedMultiTexImage2DEXT",
  "glCompressedMultiTexImage1DEXT",
  "glCompressedMultiTexSubImage3DEXT",
  "glCompressedMultiTexSubImage2DEXT",
  "glCompressedMultiTexSubImage1DEXT",
  "glGetCompressedMultiTexImageEXT",
  "glMatrixLoadTransposefEXT",
  "glMatrixLoadTransposedEXT",
  "glMatrixMultTransposefEXT",
  "glMatrixMultTransposedEXT",
  "glNamedBufferDataEXT",
  "glNamedBufferSubDataEXT",
  "glMapNamedBufferEXT",
  "glUnmapNamedBufferEXT",
  "glGetNamedBufferParameterivEXT",
  "glGetNamedBufferPointervEXT",
  "glGetNamedBufferSubDataEXT",
  "glProgramUniform1fEXT",
  "glProgramUniform2fEXT",
  "glProgramUniform3fEXT",
  "glProgramUniform4fEXT",
  "glProgramUniform1iEXT",
  "glProgramUniform2iEXT",
  "glProgramUniform3iEXT",
  "glProgramUniform4iEXT",
  "glProgramUniform1fvEXT",
  "glProgramUniform2fvEXT",
  "glProgramUniform3fvEXT",
  "glProgramUniform4fvEXT",
  "glProgramUniform1ivEXT",
  "glProgramUniform2ivEXT",
  "glProgramUniform3ivEXT",
  "glProgramUniform4ivEXT",
  "glProgramUniformMatrix2fvEXT",
  "glProgramUniformMatrix3fvEXT",
  "glProgramUniformMatrix4fvEXT",
  "glProgramUniformMatrix2x3fvEXT",
  "glProgramUniformMatrix3x2fvEXT",
  "glProgramUniformMatrix2x4fvEXT",
  "glProgramUniformMatrix4x2fvEXT",
  "glProgramUniformMatrix3x4fvEXT",
  "glProgramUniformMatrix4x3fvEXT",
  "glTextureBufferEXT",
  "glMultiTexBufferEXT",
  "glTextureParameterIivEXT",
  "glTextureParameterIuivEXT",
  "glGetTextureParameterIivEXT",
  "glGetTextureParameterIuivEXT",
  "glMultiTexParameterIivEXT",
  "glMultiTexParameterIuivEXT",
  "glGetMultiTexParameterIivEXT",
  "glGetMultiTexParameterIuivEXT",
  "glProgramUniform1uiEXT",
  "glProgramUniform2uiEXT",
  "glProgramUniform3uiEXT",
  "glProgramUniform4uiEXT",
  "glProgramUniform1uivEXT",
  "glProgramUniform2uivEXT",
  "glProgramUniform3uivEXT",
  "glProgramUniform4uivEXT",
  "glNamedProgramLocalParameters4fvEXT",
  "glNamedProgramLocalParameterI4iEXT",
  "glNamedProgramLocalParameterI4ivEXT",
  "glNamedProgramLocalParametersI4ivEXT",
  "glNamedProgramLocalParameterI4uiEXT",
  "glNamedProgramLocalParameterI4uivEXT",
  "glNamedProgramLocalParametersI4uivEXT",
  "glGetNamedProgramLocalParameterIivEXT",
  "glGetNamedProgramLocalParameterIuivEXT",
  "glEnableClientStateiEXT",
  "glDisableClientStateiEXT",
  "glGetFloati_vEXT",
  "glGetDoublei_vEXT",
  "glGetPointeri_vEXT",
  "glNamedProgramStringEXT",
  "glNamedProgramLocalParameter4dEXT",
  "glNamedProgramLocalParameter4dvEXT",
  "glNamedProgramLocalParameter4fEXT",
  "glNamedProgramLocalParameter4fvEXT",
  "glGetNamedProgramLocalParameterdvEXT",
  "glGetNamedProgramLocalParameterfvEXT",
  "glGetNamedProgramivEXT",
  "glGetNamedProgramStringEXT",
  "glNamedRenderbufferStorageEXT",
  "glGetNamedRenderbufferParameterivEXT",
  "glNamedRenderbufferStorageMultisampleEXT",
  "glNamedRenderbufferStorageMultisampleCoverageEXT",
  "glCheckNamedFramebufferStatusEXT",
  "glNamedFramebufferTexture1DEXT",
  "glNamedFramebufferTexture2DEXT",
  "glNamedFramebufferTexture3DEXT",
  "glNamedFramebufferRenderbufferEXT",
  "glGetNamedFramebufferAttachmentParameterivEXT",
  "glGenerateTextureMipmapEXT",
  "glGenerateMultiTexMipmapEXT",
  "glFramebufferDrawBufferEXT",
  "glFramebufferDrawBuffersEXT",
  "glFramebufferReadBufferEXT",
  "glGetFramebufferParameterivEXT",
  "glNamedCopyBufferSubDataEXT",
  "glNamedFramebufferTextureEXT",
  "glNamedFramebufferTextureLayerEXT",
  "glNamedFramebufferTextureFaceEXT",
  "glTextureRenderbufferEXT",
  "glMultiTexRenderbufferEXT",
  "glVertexArrayVertexOffsetEXT",
  "glVertexArrayColorOffsetEXT",
  "glVertexArrayEdgeFlagOffsetEXT",
  "glVertexArrayIndexOffsetEXT",
  "glVertexArrayNormalOffsetEXT",
  "glVertexArrayTexCoordOffsetEXT",
  "glVertexArrayMultiTexCoordOffsetEXT",
  "glVertexArrayFogCoordOffsetEXT",
  "glVertexArraySecondaryColorOffsetEXT",
  "glVertexArrayVertexAttribOffsetEXT",
  "glVertexArrayVertexAttribIOffsetEXT",
  "glEnableVertexArrayEXT",
  "glDisableVertexArrayEXT",
  "glEnableVertexArrayAttribEXT",
  "glDisableVertexArrayAttribEXT",
  "glGetVertexArrayIntegervEXT",
  "glGetVertexArrayPointervEXT",
  "glGetVertexArrayIntegeri_vEXT",
  "glGetVertexArrayPointeri_vEXT",
  "glMapNamedBufferRangeEXT",
  "glFlushMappedNamedBufferRangeEXT",
  "glNamedBufferStorageEXT",
  "glClearNamedBufferDataEXT",
  "glClearNamedBufferSubDataEXT",
  "glNamedFramebufferParameteriEXT",
  "glGetNamedFramebufferParameterivEXT",
  "glProgramUniform1dEXT",
  "glProgramUniform2dEXT",
  "glProgramUniform3dEXT",
  "glProgramUniform4dEXT",
  "glProgramUniform1dvEXT",
  "glProgramUniform2dvEXT",
  "glProgramUniform3dvEXT",
  "glProgramUniform4dvEXT",
  "glProgramUniformMatrix2dvEXT",
  "glProgramUniformMatrix3dvEXT",
  "glProgramUniformMatrix4dvEXT",
  "glProgramUniformMatrix2x3dvEXT",
  "glProgramUniformMatrix2x4dvEXT",
  "glProgramUniformMatrix3x2dvEXT",
  "glProgramUniformMatrix3x4dvEXT",
  "glProgramUniformMatrix4x2dvEXT",
  "glProgramUniformMatrix4x3dvEXT",
  "glTextureBufferRangeEXT",
  "glTextureStorage1DEXT",
  "glTextureStorage2DEXT",
  "glTextureStorage3DEXT",
  "glTextureStorage2DMultisampleEXT",
  "glTextureStorage3DMultisampleEXT",
  "glVertexArrayBindVertexBufferEXT",
  "glVertexArrayVertexAttribFormatEXT",
  "glVertexArrayVertexAttribIFormatEXT",
  "glVertexArrayVertexAttribLFormatEXT",
  "glVertexArrayVertexAttribBindingEXT",
  "glVertexArrayVertexBindingDivisorEXT",
  "glVertexArrayVertexAttribLOffsetEXT",
  "glTexturePageCommitmentEXT",
  "glVertexArrayVertexAttribDivisorEXT",
];

pub const GL_EXT_discard_framebuffer_COMMANDS: &[&str] = &["glDiscardFramebufferEXT"];

pub const GL_EXT_disjoint_timer_query_COMMANDS: &[&str] = &["glGenQueriesEXT", "glDeleteQueriesEXT", "glIsQueryEXT", "glBeginQueryEXT", "glEndQueryEXT", "glQueryCounterEXT", "glGetQueryivEXT", "glGetQueryObjectivEXT", "glGetQueryObjectuivEXT", "glGetQueryObjecti64vEXT", "glGetQueryObjectui64vEXT", "glGetInteger64vEXT"];

pub const GL_EXT_draw_buffers_COMMANDS: &[&str] = &["glDrawBuffersEXT"];

pub const GL_EXT_draw_buffers2_COMMANDS: &[&str] = &["glColorMaskIndexedEXT", "glGetBooleanIndexedvEXT", "glGetIntegerIndexedvEXT", "glEnableIndexedEXT", "glDisableIndexedEXT", "glIsEnabledIndexedEXT"];

pub const GL_EXT_draw_buffers_indexed_COMMANDS: &[&str] = &["glEnableiEXT", "glDisableiEXT", "glBlendEquationiEXT", "glBlendEquationSeparateiEXT", "glBlendFunciEXT", "glBlendFuncSeparateiEXT", "glColorMaskiEXT", "glIsEnablediEXT"];

pub const GL_EXT_draw_elements_base_vertex_COMMANDS: &[&str] = &["glDrawElementsBaseVertexEXT", "glDrawRangeElementsBaseVertexEXT", "glDrawElementsInstancedBaseVertexEXT", "glMultiDrawElementsBaseVertexEXT"];

pub const GL_EXT_draw_instanced_COMMANDS: &[&str] = &["glDrawArraysInstancedEXT", "glDrawElementsInstancedEXT"];

pub const GL_EXT_draw_range_elements_COMMANDS: &[&str] = &["glDrawRangeElementsEXT"];

pub const GL_EXT_draw_transform_feedback_COMMANDS: &[&str] = &["glDrawTransformFeedbackEXT", "glDrawTransformFeedbackInstancedEXT"];

pub const GL_EXT_external_buffer_COMMANDS: &[&str] = &["glBufferStorageExternalEXT", "glNamedBufferStorageExternalEXT"];

pub const GL_EXT_fog_coord_COMMANDS: &[&str] = &["glFogCoordfEXT", "glFogCoordfvEXT", "glFogCoorddEXT", "glFogCoorddvEXT", "glFogCoordPointerEXT"];

pub const GL_EXT_framebuffer_blit_COMMANDS: &[&str] = &["glBlitFramebufferEXT"];

pub const GL_EXT_framebuffer_multisample_COMMANDS: &[&str] = &["glRenderbufferStorageMultisampleEXT"];

pub const GL_EXT_framebuffer_object_COMMANDS: &[&str] = &["glIsRenderbufferEXT", "glBindRenderbufferEXT", "glDeleteRenderbuffersEXT", "glGenRenderbuffersEXT", "glRenderbufferStorageEXT", "glGetRenderbufferParameterivEXT", "glIsFramebufferEXT", "glBindFramebufferEXT", "glDeleteFramebuffersEXT", "glGenFramebuffersEXT", "glCheckFramebufferStatusEXT", "glFramebufferTexture1DEXT", "glFramebufferTexture2DEXT", "glFramebufferTexture3DEXT", "glFramebufferRenderbufferEXT", "glGetFramebufferAttachmentParameterivEXT", "glGenerateMipmapEXT"];

pub const GL_EXT_geometry_shader_COMMANDS: &[&str] = &["glFramebufferTextureEXT"];

pub const GL_EXT_geometry_shader4_COMMANDS: &[&str] = &["glProgramParameteriEXT"];

pub const GL_EXT_gpu_program_parameters_COMMANDS: &[&str] = &["glProgramEnvParameters4fvEXT", "glProgramLocalParameters4fvEXT"];

pub const GL_EXT_gpu_shader4_COMMANDS: &[&str] = &["glGetUniformuivEXT", "glBindFragDataLocationEXT", "glGetFragDataLocationEXT", "glUniform1uiEXT", "glUniform2uiEXT", "glUniform3uiEXT", "glUniform4uiEXT", "glUniform1uivEXT", "glUniform2uivEXT", "glUniform3uivEXT", "glUniform4uivEXT"];

pub const GL_EXT_histogram_COMMANDS: &[&str] = &["glGetHistogramEXT", "glGetHistogramParameterfvEXT", "glGetHistogramParameterivEXT", "glGetMinmaxEXT", "glGetMinmaxParameterfvEXT", "glGetMinmaxParameterivEXT", "glHistogramEXT", "glMinmaxEXT", "glResetHistogramEXT", "glResetMinmaxEXT"];

pub const GL_EXT_index_func_COMMANDS: &[&str] = &["glIndexFuncEXT"];

pub const GL_EXT_index_material_COMMANDS: &[&str] = &["glIndexMaterialEXT"];

pub const GL_EXT_instanced_arrays_COMMANDS: &[&str] = &["glDrawArraysInstancedEXT", "glDrawElementsInstancedEXT", "glVertexAttribDivisorEXT"];

pub const GL_EXT_light_texture_COMMANDS: &[&str] = &["glApplyTextureEXT", "glTextureLightEXT", "glTextureMaterialEXT"];

pub const GL_EXT_map_buffer_range_COMMANDS: &[&str] = &["glMapBufferRangeEXT", "glFlushMappedBufferRangeEXT"];

pub const GL_EXT_memory_object_COMMANDS: &[&str] = &["glGetUnsignedBytevEXT", "glGetUnsignedBytei_vEXT", "glDeleteMemoryObjectsEXT", "glIsMemoryObjectEXT", "glCreateMemoryObjectsEXT", "glMemoryObjectParameterivEXT", "glGetMemoryObjectParameterivEXT", "glTexStorageMem2DEXT", "glTexStorageMem2DMultisampleEXT", "glTexStorageMem3DEXT", "glTexStorageMem3DMultisampleEXT", "glBufferStorageMemEXT", "glTextureStorageMem2DEXT", "glTextureStorageMem2DMultisampleEXT", "glTextureStorageMem3DEXT", "glTextureStorageMem3DMultisampleEXT", "glNamedBufferStorageMemEXT", "glTexStorageMem1DEXT", "glTextureStorageMem1DEXT"];

pub const GL_EXT_memory_object_fd_COMMANDS: &[&str] = &["glImportMemoryFdEXT"];

pub const GL_EXT_memory_object_win32_COMMANDS: &[&str] = &["glImportMemoryWin32HandleEXT", "glImportMemoryWin32NameEXT"];

pub const GL_EXT_multi_draw_arrays_COMMANDS: &[&str] = &["glMultiDrawArraysEXT", "glMultiDrawElementsEXT"];

pub const GL_EXT_multi_draw_indirect_COMMANDS: &[&str] = &["glMultiDrawArraysIndirectEXT", "glMultiDrawElementsIndirectEXT"];

pub const GL_EXT_multisample_COMMANDS: &[&str] = &["glSampleMaskEXT", "glSamplePatternEXT"];

pub const GL_EXT_multisampled_render_to_texture_COMMANDS: &[&str] = &["glRenderbufferStorageMultisampleEXT", "glFramebufferTexture2DMultisampleEXT"];

pub const GL_EXT_multiview_draw_buffers_COMMANDS: &[&str] = &["glReadBufferIndexedEXT", "glDrawBuffersIndexedEXT", "glGetIntegeri_vEXT"];

pub const GL_EXT_occlusion_query_boolean_COMMANDS: &[&str] = &["glGenQueriesEXT", "glDeleteQueriesEXT", "glIsQueryEXT", "glBeginQueryEXT", "glEndQueryEXT", "glGetQueryivEXT", "glGetQueryObjectuivEXT"];

pub const GL_EXT_paletted_texture_COMMANDS: &[&str] = &["glColorTableEXT", "glGetColorTableEXT", "glGetColorTableParameterivEXT", "glGetColorTableParameterfvEXT"];

pub const GL_EXT_pixel_transform_COMMANDS: &[&str] = &["glPixelTransformParameteriEXT", "glPixelTransformParameterfEXT", "glPixelTransformParameterivEXT", "glPixelTransformParameterfvEXT", "glGetPixelTransformParameterivEXT", "glGetPixelTransformParameterfvEXT"];

pub const GL_EXT_point_parameters_COMMANDS: &[&str] = &["glPointParameterfEXT", "glPointParameterfvEXT"];

pub const GL_EXT_polygon_offset_COMMANDS: &[&str] = &["glPolygonOffsetEXT"];

pub const GL_EXT_polygon_offset_clamp_COMMANDS: &[&str] = &["glPolygonOffsetClampEXT"];

pub const GL_EXT_primitive_bounding_box_COMMANDS: &[&str] = &["glPrimitiveBoundingBoxEXT"];

pub const GL_EXT_provoking_vertex_COMMANDS: &[&str] = &["glProvokingVertexEXT"];

pub const GL_EXT_raster_multisample_COMMANDS: &[&str] = &["glRasterSamplesEXT"];

pub const GL_EXT_robustness_COMMANDS: &[&str] = &["glGetGraphicsResetStatusEXT", "glReadnPixelsEXT", "glGetnUniformfvEXT", "glGetnUniformivEXT"];

pub const GL_EXT_secondary_color_COMMANDS: &[&str] = &["glSecondaryColor3bEXT", "glSecondaryColor3bvEXT", "glSecondaryColor3dEXT", "glSecondaryColor3dvEXT", "glSecondaryColor3fEXT", "glSecondaryColor3fvEXT", "glSecondaryColor3iEXT", "glSecondaryColor3ivEXT", "glSecondaryColor3sEXT", "glSecondaryColor3svEXT", "glSecondaryColor3ubEXT", "glSecondaryColor3ubvEXT", "glSecondaryColor3uiEXT", "glSecondaryColor3uivEXT", "glSecondaryColor3usEXT", "glSecondaryColor3usvEXT", "glSecondaryColorPointerEXT"];

pub const GL_EXT_semaphore_COMMANDS: &[&str] = &["glGetUnsignedBytevEXT", "glGetUnsignedBytei_vEXT", "glGenSemaphoresEXT", "glDeleteSemaphoresEXT", "glIsSemaphoreEXT", "glSemaphoreParameterui64vEXT", "glGetSemaphoreParameterui64vEXT", "glWaitSemaphoreEXT", "glSignalSemaphoreEXT"];

pub const GL_EXT_semaphore_fd_COMMANDS: &[&str] = &["glImportSemaphoreFdEXT"];

pub const GL_EXT_semaphore_win32_COMMANDS: &[&str] = &["glImportSemaphoreWin32HandleEXT", "glImportSemaphoreWin32NameEXT"];

pub const GL_EXT_separate_shader_objects_COMMANDS: &[&str] = &[
  "glUseShaderProgramEXT",
  "glActiveProgramEXT",
  "glCreateShaderProgramEXT",
  "glActiveShaderProgramEXT",
  "glBindProgramPipelineEXT",
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
  "glProgramUniform2fEXT",
  "glProgramUniform2fvEXT",
  "glProgramUniform2iEXT",
  "glProgramUniform2ivEXT",
  "glProgramUniform3fEXT",
  "glProgramUniform3fvEXT",
  "glProgramUniform3iEXT",
  "glProgramUniform3ivEXT",
  "glProgramUniform4fEXT",
  "glProgramUniform4fvEXT",
  "glProgramUniform4iEXT",
  "glProgramUniform4ivEXT",
  "glProgramUniformMatrix2fvEXT",
  "glProgramUniformMatrix3fvEXT",
  "glProgramUniformMatrix4fvEXT",
  "glUseProgramStagesEXT",
  "glValidateProgramPipelineEXT",
  "glProgramUniform1uiEXT",
  "glProgramUniform2uiEXT",
  "glProgramUniform3uiEXT",
  "glProgramUniform4uiEXT",
  "glProgramUniform1uivEXT",
  "glProgramUniform2uivEXT",
  "glProgramUniform3uivEXT",
  "glProgramUniform4uivEXT",
  "glProgramUniformMatrix2x3fvEXT",
  "glProgramUniformMatrix3x2fvEXT",
  "glProgramUniformMatrix2x4fvEXT",
  "glProgramUniformMatrix4x2fvEXT",
  "glProgramUniformMatrix3x4fvEXT",
  "glProgramUniformMatrix4x3fvEXT",
];

pub const GL_EXT_shader_framebuffer_fetch_non_coherent_COMMANDS: &[&str] = &["glFramebufferFetchBarrierEXT"];

pub const GL_EXT_shader_image_load_store_COMMANDS: &[&str] = &["glBindImageTextureEXT", "glMemoryBarrierEXT"];

pub const GL_EXT_shader_pixel_local_storage2_COMMANDS: &[&str] = &["glFramebufferPixelLocalStorageSizeEXT", "glGetFramebufferPixelLocalStorageSizeEXT", "glClearPixelLocalStorageuiEXT"];

pub const GL_EXT_sparse_texture_COMMANDS: &[&str] = &["glTexPageCommitmentEXT"];

pub const GL_EXT_stencil_clear_tag_COMMANDS: &[&str] = &["glStencilClearTagEXT"];

pub const GL_EXT_stencil_two_side_COMMANDS: &[&str] = &["glActiveStencilFaceEXT"];

pub const GL_EXT_subtexture_COMMANDS: &[&str] = &["glTexSubImage1DEXT", "glTexSubImage2DEXT"];

pub const GL_EXT_tessellation_shader_COMMANDS: &[&str] = &["glPatchParameteriEXT"];

pub const GL_EXT_texture3D_COMMANDS: &[&str] = &["glTexImage3DEXT", "glTexSubImage3DEXT"];

pub const GL_EXT_texture_array_COMMANDS: &[&str] = &["glFramebufferTextureLayerEXT"];

pub const GL_EXT_texture_border_clamp_COMMANDS: &[&str] = &["glTexParameterIivEXT", "glTexParameterIuivEXT", "glGetTexParameterIivEXT", "glGetTexParameterIuivEXT", "glSamplerParameterIivEXT", "glSamplerParameterIuivEXT", "glGetSamplerParameterIivEXT", "glGetSamplerParameterIuivEXT"];

pub const GL_EXT_texture_buffer_COMMANDS: &[&str] = &["glTexBufferEXT", "glTexBufferRangeEXT"];

pub const GL_EXT_texture_buffer_object_COMMANDS: &[&str] = &["glTexBufferEXT"];

pub const GL_EXT_texture_integer_COMMANDS: &[&str] = &["glTexParameterIivEXT", "glTexParameterIuivEXT", "glGetTexParameterIivEXT", "glGetTexParameterIuivEXT", "glClearColorIiEXT", "glClearColorIuiEXT"];

pub const GL_EXT_texture_object_COMMANDS: &[&str] = &["glAreTexturesResidentEXT", "glBindTextureEXT", "glDeleteTexturesEXT", "glGenTexturesEXT", "glIsTextureEXT", "glPrioritizeTexturesEXT"];

pub const GL_EXT_texture_perturb_normal_COMMANDS: &[&str] = &["glTextureNormalEXT"];

pub const GL_EXT_texture_storage_COMMANDS: &[&str] = &["glTexStorage1DEXT", "glTexStorage2DEXT", "glTexStorage3DEXT", "glTextureStorage1DEXT", "glTextureStorage2DEXT", "glTextureStorage3DEXT"];

pub const GL_EXT_texture_view_COMMANDS: &[&str] = &["glTextureViewEXT"];

pub const GL_EXT_timer_query_COMMANDS: &[&str] = &["glGetQueryObjecti64vEXT", "glGetQueryObjectui64vEXT"];

pub const GL_EXT_transform_feedback_COMMANDS: &[&str] = &["glBeginTransformFeedbackEXT", "glEndTransformFeedbackEXT", "glBindBufferRangeEXT", "glBindBufferOffsetEXT", "glBindBufferBaseEXT", "glTransformFeedbackVaryingsEXT", "glGetTransformFeedbackVaryingEXT"];

pub const GL_EXT_vertex_array_COMMANDS: &[&str] = &["glArrayElementEXT", "glColorPointerEXT", "glDrawArraysEXT", "glEdgeFlagPointerEXT", "glGetPointervEXT", "glIndexPointerEXT", "glNormalPointerEXT", "glTexCoordPointerEXT", "glVertexPointerEXT"];

pub const GL_EXT_vertex_attrib_64bit_COMMANDS: &[&str] = &["glVertexAttribL1dEXT", "glVertexAttribL2dEXT", "glVertexAttribL3dEXT", "glVertexAttribL4dEXT", "glVertexAttribL1dvEXT", "glVertexAttribL2dvEXT", "glVertexAttribL3dvEXT", "glVertexAttribL4dvEXT", "glVertexAttribLPointerEXT", "glGetVertexAttribLdvEXT"];

pub const GL_EXT_vertex_shader_COMMANDS: &[&str] = &[
  "glBeginVertexShaderEXT",
  "glEndVertexShaderEXT",
  "glBindVertexShaderEXT",
  "glGenVertexShadersEXT",
  "glDeleteVertexShaderEXT",
  "glShaderOp1EXT",
  "glShaderOp2EXT",
  "glShaderOp3EXT",
  "glSwizzleEXT",
  "glWriteMaskEXT",
  "glInsertComponentEXT",
  "glExtractComponentEXT",
  "glGenSymbolsEXT",
  "glSetInvariantEXT",
  "glSetLocalConstantEXT",
  "glVariantbvEXT",
  "glVariantsvEXT",
  "glVariantivEXT",
  "glVariantfvEXT",
  "glVariantdvEXT",
  "glVariantubvEXT",
  "glVariantusvEXT",
  "glVariantuivEXT",
  "glVariantPointerEXT",
  "glEnableVariantClientStateEXT",
  "glDisableVariantClientStateEXT",
  "glBindLightParameterEXT",
  "glBindMaterialParameterEXT",
  "glBindTexGenParameterEXT",
  "glBindTextureUnitParameterEXT",
  "glBindParameterEXT",
  "glIsVariantEnabledEXT",
  "glGetVariantBooleanvEXT",
  "glGetVariantIntegervEXT",
  "glGetVariantFloatvEXT",
  "glGetVariantPointervEXT",
  "glGetInvariantBooleanvEXT",
  "glGetInvariantIntegervEXT",
  "glGetInvariantFloatvEXT",
  "glGetLocalConstantBooleanvEXT",
  "glGetLocalConstantIntegervEXT",
  "glGetLocalConstantFloatvEXT",
];

pub const GL_EXT_vertex_weighting_COMMANDS: &[&str] = &["glVertexWeightfEXT", "glVertexWeightfvEXT", "glVertexWeightPointerEXT"];

pub const GL_EXT_win32_keyed_mutex_COMMANDS: &[&str] = &["glAcquireKeyedMutexWin32EXT", "glReleaseKeyedMutexWin32EXT"];

pub const GL_EXT_window_rectangles_COMMANDS: &[&str] = &["glWindowRectanglesEXT"];

pub const GL_EXT_x11_sync_object_COMMANDS: &[&str] = &["glImportSyncEXT"];

pub const GL_GREMEDY_frame_terminator_COMMANDS: &[&str] = &["glFrameTerminatorGREMEDY"];

pub const GL_GREMEDY_string_marker_COMMANDS: &[&str] = &["glStringMarkerGREMEDY"];

pub const GL_HP_image_transform_COMMANDS: &[&str] = &["glImageTransformParameteriHP", "glImageTransformParameterfHP", "glImageTransformParameterivHP", "glImageTransformParameterfvHP", "glGetImageTransformParameterivHP", "glGetImageTransformParameterfvHP"];

pub const GL_IBM_multimode_draw_arrays_COMMANDS: &[&str] = &["glMultiModeDrawArraysIBM", "glMultiModeDrawElementsIBM"];

pub const GL_IBM_static_data_COMMANDS: &[&str] = &["glFlushStaticDataIBM"];

pub const GL_IBM_vertex_array_lists_COMMANDS: &[&str] = &["glColorPointerListIBM", "glSecondaryColorPointerListIBM", "glEdgeFlagPointerListIBM", "glFogCoordPointerListIBM", "glIndexPointerListIBM", "glNormalPointerListIBM", "glTexCoordPointerListIBM", "glVertexPointerListIBM"];

pub const GL_IMG_bindless_texture_COMMANDS: &[&str] = &["glGetTextureHandleIMG", "glGetTextureSamplerHandleIMG", "glUniformHandleui64IMG", "glUniformHandleui64vIMG", "glProgramUniformHandleui64IMG", "glProgramUniformHandleui64vIMG"];

pub const GL_IMG_framebuffer_downsample_COMMANDS: &[&str] = &["glFramebufferTexture2DDownsampleIMG", "glFramebufferTextureLayerDownsampleIMG"];

pub const GL_IMG_multisampled_render_to_texture_COMMANDS: &[&str] = &["glRenderbufferStorageMultisampleIMG", "glFramebufferTexture2DMultisampleIMG"];

pub const GL_IMG_user_clip_plane_COMMANDS: &[&str] = &["glClipPlanefIMG", "glClipPlanexIMG"];

pub const GL_INGR_blend_func_separate_COMMANDS: &[&str] = &["glBlendFuncSeparateINGR"];

pub const GL_INTEL_framebuffer_CMAA_COMMANDS: &[&str] = &["glApplyFramebufferAttachmentCMAAINTEL"];

pub const GL_INTEL_map_texture_COMMANDS: &[&str] = &["glSyncTextureINTEL", "glUnmapTexture2DINTEL", "glMapTexture2DINTEL"];

pub const GL_INTEL_parallel_arrays_COMMANDS: &[&str] = &["glVertexPointervINTEL", "glNormalPointervINTEL", "glColorPointervINTEL", "glTexCoordPointervINTEL"];

pub const GL_INTEL_performance_query_COMMANDS: &[&str] = &["glBeginPerfQueryINTEL", "glCreatePerfQueryINTEL", "glDeletePerfQueryINTEL", "glEndPerfQueryINTEL", "glGetFirstPerfQueryIdINTEL", "glGetNextPerfQueryIdINTEL", "glGetPerfCounterInfoINTEL", "glGetPerfQueryDataINTEL", "glGetPerfQueryIdByNameINTEL", "glGetPerfQueryInfoINTEL"];

pub const GL_KHR_blend_equation_advanced_COMMANDS: &[&str] = &["glBlendBarrierKHR"];

pub const GL_KHR_debug_COMMANDS: &[&str] = &["glDebugMessageControl", "glDebugMessageInsert", "glDebugMessageCallback", "glGetDebugMessageLog", "glPushDebugGroup", "glPopDebugGroup", "glObjectLabel", "glGetObjectLabel", "glObjectPtrLabel", "glGetObjectPtrLabel", "glGetPointerv", "glDebugMessageControlKHR", "glDebugMessageInsertKHR", "glDebugMessageCallbackKHR", "glGetDebugMessageLogKHR", "glPushDebugGroupKHR", "glPopDebugGroupKHR", "glObjectLabelKHR", "glGetObjectLabelKHR", "glObjectPtrLabelKHR", "glGetObjectPtrLabelKHR", "glGetPointervKHR"];

pub const GL_KHR_parallel_shader_compile_COMMANDS: &[&str] = &["glMaxShaderCompilerThreadsKHR"];

pub const GL_KHR_robustness_COMMANDS: &[&str] = &["glGetGraphicsResetStatus", "glReadnPixels", "glGetnUniformfv", "glGetnUniformiv", "glGetnUniformuiv", "glGetGraphicsResetStatusKHR", "glReadnPixelsKHR", "glGetnUniformfvKHR", "glGetnUniformivKHR", "glGetnUniformuivKHR"];

pub const GL_MESA_framebuffer_flip_y_COMMANDS: &[&str] = &["glFramebufferParameteriMESA", "glGetFramebufferParameterivMESA"];

pub const GL_MESA_resize_buffers_COMMANDS: &[&str] = &["glResizeBuffersMESA"];

pub const GL_MESA_window_pos_COMMANDS: &[&str] = &["glWindowPos2dMESA", "glWindowPos2dvMESA", "glWindowPos2fMESA", "glWindowPos2fvMESA", "glWindowPos2iMESA", "glWindowPos2ivMESA", "glWindowPos2sMESA", "glWindowPos2svMESA", "glWindowPos3dMESA", "glWindowPos3dvMESA", "glWindowPos3fMESA", "glWindowPos3fvMESA", "glWindowPos3iMESA", "glWindowPos3ivMESA", "glWindowPos3sMESA", "glWindowPos3svMESA", "glWindowPos4dMESA", "glWindowPos4dvMESA", "glWindowPos4fMESA", "glWindowPos4fvMESA", "glWindowPos4iMESA", "glWindowPos4ivMESA", "glWindowPos4sMESA", "glWindowPos4svMESA"];

pub const GL_NVX_conditional_render_COMMANDS: &[&str] = &["glBeginConditionalRenderNVX", "glEndConditionalRenderNVX"];

pub const GL_NVX_gpu_multicast2_COMMANDS: &[&str] = &["glUploadGpuMaskNVX", "glMulticastViewportArrayvNVX", "glMulticastViewportPositionWScaleNVX", "glMulticastScissorArrayvNVX", "glAsyncCopyBufferSubDataNVX", "glAsyncCopyImageSubDataNVX"];

pub const GL_NVX_linked_gpu_multicast_COMMANDS: &[&str] = &["glLGPUNamedBufferSubDataNVX", "glLGPUCopyImageSubDataNVX", "glLGPUInterlockNVX"];

pub const GL_NVX_progress_fence_COMMANDS: &[&str] = &["glCreateProgressFenceNVX", "glSignalSemaphoreui64NVX", "glWaitSemaphoreui64NVX", "glClientWaitSemaphoreui64NVX"];

pub const GL_NV_alpha_to_coverage_dither_control_COMMANDS: &[&str] = &["glAlphaToCoverageDitherControlNV"];

pub const GL_NV_bindless_multi_draw_indirect_COMMANDS: &[&str] = &["glMultiDrawArraysIndirectBindlessNV", "glMultiDrawElementsIndirectBindlessNV"];

pub const GL_NV_bindless_multi_draw_indirect_count_COMMANDS: &[&str] = &["glMultiDrawArraysIndirectBindlessCountNV", "glMultiDrawElementsIndirectBindlessCountNV"];

pub const GL_NV_bindless_texture_COMMANDS: &[&str] = &["glGetTextureHandleNV", "glGetTextureSamplerHandleNV", "glMakeTextureHandleResidentNV", "glMakeTextureHandleNonResidentNV", "glGetImageHandleNV", "glMakeImageHandleResidentNV", "glMakeImageHandleNonResidentNV", "glUniformHandleui64NV", "glUniformHandleui64vNV", "glProgramUniformHandleui64NV", "glProgramUniformHandleui64vNV", "glIsTextureHandleResidentNV", "glIsImageHandleResidentNV"];

pub const GL_NV_blend_equation_advanced_COMMANDS: &[&str] = &["glBlendParameteriNV", "glBlendBarrierNV"];

pub const GL_NV_clip_space_w_scaling_COMMANDS: &[&str] = &["glViewportPositionWScaleNV"];

pub const GL_NV_command_list_COMMANDS: &[&str] = &["glCreateStatesNV", "glDeleteStatesNV", "glIsStateNV", "glStateCaptureNV", "glGetCommandHeaderNV", "glGetStageIndexNV", "glDrawCommandsNV", "glDrawCommandsAddressNV", "glDrawCommandsStatesNV", "glDrawCommandsStatesAddressNV", "glCreateCommandListsNV", "glDeleteCommandListsNV", "glIsCommandListNV", "glListDrawCommandsStatesClientNV", "glCommandListSegmentsNV", "glCompileCommandListNV", "glCallCommandListNV"];

pub const GL_NV_conditional_render_COMMANDS: &[&str] = &["glBeginConditionalRenderNV", "glEndConditionalRenderNV"];

pub const GL_NV_conservative_raster_COMMANDS: &[&str] = &["glSubpixelPrecisionBiasNV"];

pub const GL_NV_conservative_raster_dilate_COMMANDS: &[&str] = &["glConservativeRasterParameterfNV"];

pub const GL_NV_conservative_raster_pre_snap_triangles_COMMANDS: &[&str] = &["glConservativeRasterParameteriNV"];

pub const GL_NV_copy_buffer_COMMANDS: &[&str] = &["glCopyBufferSubDataNV"];

pub const GL_NV_copy_image_COMMANDS: &[&str] = &["glCopyImageSubDataNV"];

pub const GL_NV_coverage_sample_COMMANDS: &[&str] = &["glCoverageMaskNV", "glCoverageOperationNV"];

pub const GL_NV_depth_buffer_float_COMMANDS: &[&str] = &["glDepthRangedNV", "glClearDepthdNV", "glDepthBoundsdNV"];

pub const GL_NV_draw_buffers_COMMANDS: &[&str] = &["glDrawBuffersNV"];

pub const GL_NV_draw_instanced_COMMANDS: &[&str] = &["glDrawArraysInstancedNV", "glDrawElementsInstancedNV"];

pub const GL_NV_draw_texture_COMMANDS: &[&str] = &["glDrawTextureNV"];

pub const GL_NV_draw_vulkan_image_COMMANDS: &[&str] = &["glDrawVkImageNV", "glGetVkProcAddrNV", "glWaitVkSemaphoreNV", "glSignalVkSemaphoreNV", "glSignalVkFenceNV"];

pub const GL_NV_evaluators_COMMANDS: &[&str] = &["glMapControlPointsNV", "glMapParameterivNV", "glMapParameterfvNV", "glGetMapControlPointsNV", "glGetMapParameterivNV", "glGetMapParameterfvNV", "glGetMapAttribParameterivNV", "glGetMapAttribParameterfvNV", "glEvalMapsNV"];

pub const GL_NV_explicit_multisample_COMMANDS: &[&str] = &["glGetMultisamplefvNV", "glSampleMaskIndexedNV", "glTexRenderbufferNV"];

pub const GL_NV_fence_COMMANDS: &[&str] = &["glDeleteFencesNV", "glGenFencesNV", "glIsFenceNV", "glTestFenceNV", "glGetFenceivNV", "glFinishFenceNV", "glSetFenceNV"];

pub const GL_NV_fragment_coverage_to_color_COMMANDS: &[&str] = &["glFragmentCoverageColorNV"];

pub const GL_NV_fragment_program_COMMANDS: &[&str] = &["glProgramNamedParameter4fNV", "glProgramNamedParameter4fvNV", "glProgramNamedParameter4dNV", "glProgramNamedParameter4dvNV", "glGetProgramNamedParameterfvNV", "glGetProgramNamedParameterdvNV"];

pub const GL_NV_framebuffer_blit_COMMANDS: &[&str] = &["glBlitFramebufferNV"];

pub const GL_NV_framebuffer_mixed_samples_COMMANDS: &[&str] = &["glRasterSamplesEXT", "glCoverageModulationTableNV", "glGetCoverageModulationTableNV", "glCoverageModulationNV"];

pub const GL_NV_framebuffer_multisample_COMMANDS: &[&str] = &["glRenderbufferStorageMultisampleNV"];

pub const GL_NV_framebuffer_multisample_coverage_COMMANDS: &[&str] = &["glRenderbufferStorageMultisampleCoverageNV"];

pub const GL_NV_geometry_program4_COMMANDS: &[&str] = &["glProgramVertexLimitNV", "glFramebufferTextureEXT", "glFramebufferTextureLayerEXT", "glFramebufferTextureFaceEXT"];

pub const GL_NV_gpu_multicast_COMMANDS: &[&str] = &["glRenderGpuMaskNV", "glMulticastBufferSubDataNV", "glMulticastCopyBufferSubDataNV", "glMulticastCopyImageSubDataNV", "glMulticastBlitFramebufferNV", "glMulticastFramebufferSampleLocationsfvNV", "glMulticastBarrierNV", "glMulticastWaitSyncNV", "glMulticastGetQueryObjectivNV", "glMulticastGetQueryObjectuivNV", "glMulticastGetQueryObjecti64vNV", "glMulticastGetQueryObjectui64vNV"];

pub const GL_NV_gpu_program4_COMMANDS: &[&str] = &["glProgramLocalParameterI4iNV", "glProgramLocalParameterI4ivNV", "glProgramLocalParametersI4ivNV", "glProgramLocalParameterI4uiNV", "glProgramLocalParameterI4uivNV", "glProgramLocalParametersI4uivNV", "glProgramEnvParameterI4iNV", "glProgramEnvParameterI4ivNV", "glProgramEnvParametersI4ivNV", "glProgramEnvParameterI4uiNV", "glProgramEnvParameterI4uivNV", "glProgramEnvParametersI4uivNV", "glGetProgramLocalParameterIivNV", "glGetProgramLocalParameterIuivNV", "glGetProgramEnvParameterIivNV", "glGetProgramEnvParameterIuivNV"];

pub const GL_NV_gpu_program5_COMMANDS: &[&str] = &["glProgramSubroutineParametersuivNV", "glGetProgramSubroutineParameteruivNV"];

pub const GL_NV_gpu_shader5_COMMANDS: &[&str] =
  &["glUniform1i64NV", "glUniform2i64NV", "glUniform3i64NV", "glUniform4i64NV", "glUniform1i64vNV", "glUniform2i64vNV", "glUniform3i64vNV", "glUniform4i64vNV", "glUniform1ui64NV", "glUniform2ui64NV", "glUniform3ui64NV", "glUniform4ui64NV", "glUniform1ui64vNV", "glUniform2ui64vNV", "glUniform3ui64vNV", "glUniform4ui64vNV", "glGetUniformi64vNV", "glProgramUniform1i64NV", "glProgramUniform2i64NV", "glProgramUniform3i64NV", "glProgramUniform4i64NV", "glProgramUniform1i64vNV", "glProgramUniform2i64vNV", "glProgramUniform3i64vNV", "glProgramUniform4i64vNV", "glProgramUniform1ui64NV", "glProgramUniform2ui64NV", "glProgramUniform3ui64NV", "glProgramUniform4ui64NV", "glProgramUniform1ui64vNV", "glProgramUniform2ui64vNV", "glProgramUniform3ui64vNV", "glProgramUniform4ui64vNV"];

pub const GL_NV_half_float_COMMANDS: &[&str] = &[
  "glVertex2hNV",
  "glVertex2hvNV",
  "glVertex3hNV",
  "glVertex3hvNV",
  "glVertex4hNV",
  "glVertex4hvNV",
  "glNormal3hNV",
  "glNormal3hvNV",
  "glColor3hNV",
  "glColor3hvNV",
  "glColor4hNV",
  "glColor4hvNV",
  "glTexCoord1hNV",
  "glTexCoord1hvNV",
  "glTexCoord2hNV",
  "glTexCoord2hvNV",
  "glTexCoord3hNV",
  "glTexCoord3hvNV",
  "glTexCoord4hNV",
  "glTexCoord4hvNV",
  "glMultiTexCoord1hNV",
  "glMultiTexCoord1hvNV",
  "glMultiTexCoord2hNV",
  "glMultiTexCoord2hvNV",
  "glMultiTexCoord3hNV",
  "glMultiTexCoord3hvNV",
  "glMultiTexCoord4hNV",
  "glMultiTexCoord4hvNV",
  "glFogCoordhNV",
  "glFogCoordhvNV",
  "glSecondaryColor3hNV",
  "glSecondaryColor3hvNV",
  "glVertexWeighthNV",
  "glVertexWeighthvNV",
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
];

pub const GL_NV_instanced_arrays_COMMANDS: &[&str] = &["glVertexAttribDivisorNV"];

pub const GL_NV_internalformat_sample_query_COMMANDS: &[&str] = &["glGetInternalformatSampleivNV"];

pub const GL_NV_memory_attachment_COMMANDS: &[&str] = &["glGetMemoryObjectDetachedResourcesuivNV", "glResetMemoryObjectParameterNV", "glTexAttachMemoryNV", "glBufferAttachMemoryNV", "glTextureAttachMemoryNV", "glNamedBufferAttachMemoryNV"];

pub const GL_NV_memory_object_sparse_COMMANDS: &[&str] = &["glBufferPageCommitmentMemNV", "glTexPageCommitmentMemNV", "glNamedBufferPageCommitmentMemNV", "glTexturePageCommitmentMemNV"];

pub const GL_NV_mesh_shader_COMMANDS: &[&str] = &["glDrawMeshTasksNV", "glDrawMeshTasksIndirectNV", "glMultiDrawMeshTasksIndirectNV", "glMultiDrawMeshTasksIndirectCountNV"];

pub const GL_NV_non_square_matrices_COMMANDS: &[&str] = &["glUniformMatrix2x3fvNV", "glUniformMatrix3x2fvNV", "glUniformMatrix2x4fvNV", "glUniformMatrix4x2fvNV", "glUniformMatrix3x4fvNV", "glUniformMatrix4x3fvNV"];

pub const GL_NV_occlusion_query_COMMANDS: &[&str] = &["glGenOcclusionQueriesNV", "glDeleteOcclusionQueriesNV", "glIsOcclusionQueryNV", "glBeginOcclusionQueryNV", "glEndOcclusionQueryNV", "glGetOcclusionQueryivNV", "glGetOcclusionQueryuivNV"];

pub const GL_NV_parameter_buffer_object_COMMANDS: &[&str] = &["glProgramBufferParametersfvNV", "glProgramBufferParametersIivNV", "glProgramBufferParametersIuivNV"];

pub const GL_NV_path_rendering_COMMANDS: &[&str] = &[
  "glGenPathsNV",
  "glDeletePathsNV",
  "glIsPathNV",
  "glPathCommandsNV",
  "glPathCoordsNV",
  "glPathSubCommandsNV",
  "glPathSubCoordsNV",
  "glPathStringNV",
  "glPathGlyphsNV",
  "glPathGlyphRangeNV",
  "glWeightPathsNV",
  "glCopyPathNV",
  "glInterpolatePathsNV",
  "glTransformPathNV",
  "glPathParameterivNV",
  "glPathParameteriNV",
  "glPathParameterfvNV",
  "glPathParameterfNV",
  "glPathDashArrayNV",
  "glPathStencilFuncNV",
  "glPathStencilDepthOffsetNV",
  "glStencilFillPathNV",
  "glStencilStrokePathNV",
  "glStencilFillPathInstancedNV",
  "glStencilStrokePathInstancedNV",
  "glPathCoverDepthFuncNV",
  "glCoverFillPathNV",
  "glCoverStrokePathNV",
  "glCoverFillPathInstancedNV",
  "glCoverStrokePathInstancedNV",
  "glGetPathParameterivNV",
  "glGetPathParameterfvNV",
  "glGetPathCommandsNV",
  "glGetPathCoordsNV",
  "glGetPathDashArrayNV",
  "glGetPathMetricsNV",
  "glGetPathMetricRangeNV",
  "glGetPathSpacingNV",
  "glIsPointInFillPathNV",
  "glIsPointInStrokePathNV",
  "glGetPathLengthNV",
  "glPointAlongPathNV",
  "glMatrixLoad3x2fNV",
  "glMatrixLoad3x3fNV",
  "glMatrixLoadTranspose3x3fNV",
  "glMatrixMult3x2fNV",
  "glMatrixMult3x3fNV",
  "glMatrixMultTranspose3x3fNV",
  "glStencilThenCoverFillPathNV",
  "glStencilThenCoverStrokePathNV",
  "glStencilThenCoverFillPathInstancedNV",
  "glStencilThenCoverStrokePathInstancedNV",
  "glPathGlyphIndexRangeNV",
  "glPathGlyphIndexArrayNV",
  "glPathMemoryGlyphIndexArrayNV",
  "glProgramPathFragmentInputGenNV",
  "glGetProgramResourcefvNV",
  "glPathColorGenNV",
  "glPathTexGenNV",
  "glPathFogGenNV",
  "glGetPathColorGenivNV",
  "glGetPathColorGenfvNV",
  "glGetPathTexGenivNV",
  "glGetPathTexGenfvNV",
  "glMatrixFrustumEXT",
  "glMatrixLoadIdentityEXT",
  "glMatrixLoadTransposefEXT",
  "glMatrixLoadTransposedEXT",
  "glMatrixLoadfEXT",
  "glMatrixLoaddEXT",
  "glMatrixMultTransposefEXT",
  "glMatrixMultTransposedEXT",
  "glMatrixMultfEXT",
  "glMatrixMultdEXT",
  "glMatrixOrthoEXT",
  "glMatrixPopEXT",
  "glMatrixPushEXT",
  "glMatrixRotatefEXT",
  "glMatrixRotatedEXT",
  "glMatrixScalefEXT",
  "glMatrixScaledEXT",
  "glMatrixTranslatefEXT",
  "glMatrixTranslatedEXT",
];

pub const GL_NV_pixel_data_range_COMMANDS: &[&str] = &["glPixelDataRangeNV", "glFlushPixelDataRangeNV"];

pub const GL_NV_point_sprite_COMMANDS: &[&str] = &["glPointParameteriNV", "glPointParameterivNV"];

pub const GL_NV_polygon_mode_COMMANDS: &[&str] = &["glPolygonModeNV"];

pub const GL_NV_present_video_COMMANDS: &[&str] = &["glPresentFrameKeyedNV", "glPresentFrameDualFillNV", "glGetVideoivNV", "glGetVideouivNV", "glGetVideoi64vNV", "glGetVideoui64vNV"];

pub const GL_NV_primitive_restart_COMMANDS: &[&str] = &["glPrimitiveRestartNV", "glPrimitiveRestartIndexNV"];

pub const GL_NV_query_resource_COMMANDS: &[&str] = &["glQueryResourceNV"];

pub const GL_NV_query_resource_tag_COMMANDS: &[&str] = &["glGenQueryResourceTagNV", "glDeleteQueryResourceTagNV", "glQueryResourceTagNV"];

pub const GL_NV_read_buffer_COMMANDS: &[&str] = &["glReadBufferNV"];

pub const GL_NV_register_combiners_COMMANDS: &[&str] = &["glCombinerParameterfvNV", "glCombinerParameterfNV", "glCombinerParameterivNV", "glCombinerParameteriNV", "glCombinerInputNV", "glCombinerOutputNV", "glFinalCombinerInputNV", "glGetCombinerInputParameterfvNV", "glGetCombinerInputParameterivNV", "glGetCombinerOutputParameterfvNV", "glGetCombinerOutputParameterivNV", "glGetFinalCombinerInputParameterfvNV", "glGetFinalCombinerInputParameterivNV"];

pub const GL_NV_register_combiners2_COMMANDS: &[&str] = &["glCombinerStageParameterfvNV", "glGetCombinerStageParameterfvNV"];

pub const GL_NV_sample_locations_COMMANDS: &[&str] = &["glFramebufferSampleLocationsfvNV", "glNamedFramebufferSampleLocationsfvNV", "glResolveDepthValuesNV"];

pub const GL_NV_scissor_exclusive_COMMANDS: &[&str] = &["glScissorExclusiveNV", "glScissorExclusiveArrayvNV"];

pub const GL_NV_shader_buffer_load_COMMANDS: &[&str] = &["glMakeBufferResidentNV", "glMakeBufferNonResidentNV", "glIsBufferResidentNV", "glMakeNamedBufferResidentNV", "glMakeNamedBufferNonResidentNV", "glIsNamedBufferResidentNV", "glGetBufferParameterui64vNV", "glGetNamedBufferParameterui64vNV", "glGetIntegerui64vNV", "glUniformui64NV", "glUniformui64vNV", "glGetUniformui64vNV", "glProgramUniformui64NV", "glProgramUniformui64vNV"];

pub const GL_NV_shading_rate_image_COMMANDS: &[&str] = &["glBindShadingRateImageNV", "glGetShadingRateImagePaletteNV", "glGetShadingRateSampleLocationivNV", "glShadingRateImageBarrierNV", "glShadingRateImagePaletteNV", "glShadingRateSampleOrderNV", "glShadingRateSampleOrderCustomNV"];

pub const GL_NV_texture_barrier_COMMANDS: &[&str] = &["glTextureBarrierNV"];

pub const GL_NV_texture_multisample_COMMANDS: &[&str] = &["glTexImage2DMultisampleCoverageNV", "glTexImage3DMultisampleCoverageNV", "glTextureImage2DMultisampleNV", "glTextureImage3DMultisampleNV", "glTextureImage2DMultisampleCoverageNV", "glTextureImage3DMultisampleCoverageNV"];

pub const GL_NV_timeline_semaphore_COMMANDS: &[&str] = &["glCreateSemaphoresNV", "glSemaphoreParameterivNV", "glGetSemaphoreParameterivNV"];

pub const GL_NV_transform_feedback_COMMANDS: &[&str] = &["glBeginTransformFeedbackNV", "glEndTransformFeedbackNV", "glTransformFeedbackAttribsNV", "glBindBufferRangeNV", "glBindBufferOffsetNV", "glBindBufferBaseNV", "glTransformFeedbackVaryingsNV", "glActiveVaryingNV", "glGetVaryingLocationNV", "glGetActiveVaryingNV", "glGetTransformFeedbackVaryingNV", "glTransformFeedbackStreamAttribsNV"];

pub const GL_NV_transform_feedback2_COMMANDS: &[&str] = &["glBindTransformFeedbackNV", "glDeleteTransformFeedbacksNV", "glGenTransformFeedbacksNV", "glIsTransformFeedbackNV", "glPauseTransformFeedbackNV", "glResumeTransformFeedbackNV", "glDrawTransformFeedbackNV"];

pub const GL_NV_vdpau_interop_COMMANDS: &[&str] = &["glVDPAUInitNV", "glVDPAUFiniNV", "glVDPAURegisterVideoSurfaceNV", "glVDPAURegisterOutputSurfaceNV", "glVDPAUIsSurfaceNV", "glVDPAUUnregisterSurfaceNV", "glVDPAUGetSurfaceivNV", "glVDPAUSurfaceAccessNV", "glVDPAUMapSurfacesNV", "glVDPAUUnmapSurfacesNV"];

pub const GL_NV_vdpau_interop2_COMMANDS: &[&str] = &["glVDPAURegisterVideoSurfaceWithPictureStructureNV"];

pub const GL_NV_vertex_array_range_COMMANDS: &[&str] = &["glFlushVertexArrayRangeNV", "glVertexArrayRangeNV"];

pub const GL_NV_vertex_attrib_integer_64bit_COMMANDS: &[&str] = &["glVertexAttribL1i64NV", "glVertexAttribL2i64NV", "glVertexAttribL3i64NV", "glVertexAttribL4i64NV", "glVertexAttribL1i64vNV", "glVertexAttribL2i64vNV", "glVertexAttribL3i64vNV", "glVertexAttribL4i64vNV", "glVertexAttribL1ui64NV", "glVertexAttribL2ui64NV", "glVertexAttribL3ui64NV", "glVertexAttribL4ui64NV", "glVertexAttribL1ui64vNV", "glVertexAttribL2ui64vNV", "glVertexAttribL3ui64vNV", "glVertexAttribL4ui64vNV", "glGetVertexAttribLi64vNV", "glGetVertexAttribLui64vNV", "glVertexAttribLFormatNV"];

pub const GL_NV_vertex_buffer_unified_memory_COMMANDS: &[&str] = &["glBufferAddressRangeNV", "glVertexFormatNV", "glNormalFormatNV", "glColorFormatNV", "glIndexFormatNV", "glTexCoordFormatNV", "glEdgeFlagFormatNV", "glSecondaryColorFormatNV", "glFogCoordFormatNV", "glVertexAttribFormatNV", "glVertexAttribIFormatNV", "glGetIntegerui64i_vNV"];

pub const GL_NV_vertex_program_COMMANDS: &[&str] = &[
  "glAreProgramsResidentNV",
  "glBindProgramNV",
  "glDeleteProgramsNV",
  "glExecuteProgramNV",
  "glGenProgramsNV",
  "glGetProgramParameterdvNV",
  "glGetProgramParameterfvNV",
  "glGetProgramivNV",
  "glGetProgramStringNV",
  "glGetTrackMatrixivNV",
  "glGetVertexAttribdvNV",
  "glGetVertexAttribfvNV",
  "glGetVertexAttribivNV",
  "glGetVertexAttribPointervNV",
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
  "glVertexAttribPointerNV",
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

pub const GL_NV_vertex_program4_COMMANDS: &[&str] = &["glVertexAttribI1iEXT", "glVertexAttribI2iEXT", "glVertexAttribI3iEXT", "glVertexAttribI4iEXT", "glVertexAttribI1uiEXT", "glVertexAttribI2uiEXT", "glVertexAttribI3uiEXT", "glVertexAttribI4uiEXT", "glVertexAttribI1ivEXT", "glVertexAttribI2ivEXT", "glVertexAttribI3ivEXT", "glVertexAttribI4ivEXT", "glVertexAttribI1uivEXT", "glVertexAttribI2uivEXT", "glVertexAttribI3uivEXT", "glVertexAttribI4uivEXT", "glVertexAttribI4bvEXT", "glVertexAttribI4svEXT", "glVertexAttribI4ubvEXT", "glVertexAttribI4usvEXT", "glVertexAttribIPointerEXT", "glGetVertexAttribIivEXT", "glGetVertexAttribIuivEXT"];

pub const GL_NV_video_capture_COMMANDS: &[&str] = &["glBeginVideoCaptureNV", "glBindVideoCaptureStreamBufferNV", "glBindVideoCaptureStreamTextureNV", "glEndVideoCaptureNV", "glGetVideoCaptureivNV", "glGetVideoCaptureStreamivNV", "glGetVideoCaptureStreamfvNV", "glGetVideoCaptureStreamdvNV", "glVideoCaptureNV", "glVideoCaptureStreamParameterivNV", "glVideoCaptureStreamParameterfvNV", "glVideoCaptureStreamParameterdvNV"];

pub const GL_NV_viewport_array_COMMANDS: &[&str] = &["glViewportArrayvNV", "glViewportIndexedfNV", "glViewportIndexedfvNV", "glScissorArrayvNV", "glScissorIndexedNV", "glScissorIndexedvNV", "glDepthRangeArrayfvNV", "glDepthRangeIndexedfNV", "glGetFloati_vNV", "glEnableiNV", "glDisableiNV", "glIsEnablediNV"];

pub const GL_NV_viewport_swizzle_COMMANDS: &[&str] = &["glViewportSwizzleNV"];

pub const GL_OES_EGL_image_COMMANDS: &[&str] = &["glEGLImageTargetTexture2DOES", "glEGLImageTargetRenderbufferStorageOES"];

pub const GL_OES_blend_equation_separate_COMMANDS: &[&str] = &["glBlendEquationSeparateOES"];

pub const GL_OES_blend_func_separate_COMMANDS: &[&str] = &["glBlendFuncSeparateOES"];

pub const GL_OES_blend_subtract_COMMANDS: &[&str] = &["glBlendEquationOES"];

pub const GL_OES_byte_coordinates_COMMANDS: &[&str] = &["glMultiTexCoord1bOES", "glMultiTexCoord1bvOES", "glMultiTexCoord2bOES", "glMultiTexCoord2bvOES", "glMultiTexCoord3bOES", "glMultiTexCoord3bvOES", "glMultiTexCoord4bOES", "glMultiTexCoord4bvOES", "glTexCoord1bOES", "glTexCoord1bvOES", "glTexCoord2bOES", "glTexCoord2bvOES", "glTexCoord3bOES", "glTexCoord3bvOES", "glTexCoord4bOES", "glTexCoord4bvOES", "glVertex2bOES", "glVertex2bvOES", "glVertex3bOES", "glVertex3bvOES", "glVertex4bOES", "glVertex4bvOES"];

pub const GL_OES_copy_image_COMMANDS: &[&str] = &["glCopyImageSubDataOES"];

pub const GL_OES_draw_buffers_indexed_COMMANDS: &[&str] = &["glEnableiOES", "glDisableiOES", "glBlendEquationiOES", "glBlendEquationSeparateiOES", "glBlendFunciOES", "glBlendFuncSeparateiOES", "glColorMaskiOES", "glIsEnablediOES"];

pub const GL_OES_draw_elements_base_vertex_COMMANDS: &[&str] = &["glDrawElementsBaseVertexOES", "glDrawRangeElementsBaseVertexOES", "glDrawElementsInstancedBaseVertexOES", "glMultiDrawElementsBaseVertexEXT"];

pub const GL_OES_draw_texture_COMMANDS: &[&str] = &["glDrawTexsOES", "glDrawTexiOES", "glDrawTexxOES", "glDrawTexsvOES", "glDrawTexivOES", "glDrawTexxvOES", "glDrawTexfOES", "glDrawTexfvOES"];

pub const GL_OES_fixed_point_COMMANDS: &[&str] = &[
  "glAlphaFuncxOES",
  "glClearColorxOES",
  "glClearDepthxOES",
  "glClipPlanexOES",
  "glColor4xOES",
  "glDepthRangexOES",
  "glFogxOES",
  "glFogxvOES",
  "glFrustumxOES",
  "glGetClipPlanexOES",
  "glGetFixedvOES",
  "glGetTexEnvxvOES",
  "glGetTexParameterxvOES",
  "glLightModelxOES",
  "glLightModelxvOES",
  "glLightxOES",
  "glLightxvOES",
  "glLineWidthxOES",
  "glLoadMatrixxOES",
  "glMaterialxOES",
  "glMaterialxvOES",
  "glMultMatrixxOES",
  "glMultiTexCoord4xOES",
  "glNormal3xOES",
  "glOrthoxOES",
  "glPointParameterxvOES",
  "glPointSizexOES",
  "glPolygonOffsetxOES",
  "glRotatexOES",
  "glScalexOES",
  "glTexEnvxOES",
  "glTexEnvxvOES",
  "glTexParameterxOES",
  "glTexParameterxvOES",
  "glTranslatexOES",
  "glGetLightxvOES",
  "glGetMaterialxvOES",
  "glPointParameterxOES",
  "glSampleCoveragexOES",
  "glAccumxOES",
  "glBitmapxOES",
  "glBlendColorxOES",
  "glClearAccumxOES",
  "glColor3xOES",
  "glColor3xvOES",
  "glColor4xvOES",
  "glConvolutionParameterxOES",
  "glConvolutionParameterxvOES",
  "glEvalCoord1xOES",
  "glEvalCoord1xvOES",
  "glEvalCoord2xOES",
  "glEvalCoord2xvOES",
  "glFeedbackBufferxOES",
  "glGetConvolutionParameterxvOES",
  "glGetHistogramParameterxvOES",
  "glGetLightxOES",
  "glGetMapxvOES",
  "glGetMaterialxOES",
  "glGetPixelMapxv",
  "glGetTexGenxvOES",
  "glGetTexLevelParameterxvOES",
  "glIndexxOES",
  "glIndexxvOES",
  "glLoadTransposeMatrixxOES",
  "glMap1xOES",
  "glMap2xOES",
  "glMapGrid1xOES",
  "glMapGrid2xOES",
  "glMultTransposeMatrixxOES",
  "glMultiTexCoord1xOES",
  "glMultiTexCoord1xvOES",
  "glMultiTexCoord2xOES",
  "glMultiTexCoord2xvOES",
  "glMultiTexCoord3xOES",
  "glMultiTexCoord3xvOES",
  "glMultiTexCoord4xvOES",
  "glNormal3xvOES",
  "glPassThroughxOES",
  "glPixelMapx",
  "glPixelStorex",
  "glPixelTransferxOES",
  "glPixelZoomxOES",
  "glPrioritizeTexturesxOES",
  "glRasterPos2xOES",
  "glRasterPos2xvOES",
  "glRasterPos3xOES",
  "glRasterPos3xvOES",
  "glRasterPos4xOES",
  "glRasterPos4xvOES",
  "glRectxOES",
  "glRectxvOES",
  "glTexCoord1xOES",
  "glTexCoord1xvOES",
  "glTexCoord2xOES",
  "glTexCoord2xvOES",
  "glTexCoord3xOES",
  "glTexCoord3xvOES",
  "glTexCoord4xOES",
  "glTexCoord4xvOES",
  "glTexGenxOES",
  "glTexGenxvOES",
  "glVertex2xOES",
  "glVertex2xvOES",
  "glVertex3xOES",
  "glVertex3xvOES",
  "glVertex4xOES",
  "glVertex4xvOES",
];

pub const GL_OES_framebuffer_object_COMMANDS: &[&str] = &["glIsRenderbufferOES", "glBindRenderbufferOES", "glDeleteRenderbuffersOES", "glGenRenderbuffersOES", "glRenderbufferStorageOES", "glGetRenderbufferParameterivOES", "glIsFramebufferOES", "glBindFramebufferOES", "glDeleteFramebuffersOES", "glGenFramebuffersOES", "glCheckFramebufferStatusOES", "glFramebufferRenderbufferOES", "glFramebufferTexture2DOES", "glGetFramebufferAttachmentParameterivOES", "glGenerateMipmapOES"];

pub const GL_OES_geometry_shader_COMMANDS: &[&str] = &["glFramebufferTextureOES"];

pub const GL_OES_get_program_binary_COMMANDS: &[&str] = &["glGetProgramBinaryOES", "glProgramBinaryOES"];

pub const GL_OES_mapbuffer_COMMANDS: &[&str] = &["glMapBufferOES", "glUnmapBufferOES", "glGetBufferPointervOES"];

pub const GL_OES_matrix_palette_COMMANDS: &[&str] = &["glCurrentPaletteMatrixOES", "glLoadPaletteFromModelViewMatrixOES", "glMatrixIndexPointerOES", "glWeightPointerOES"];

pub const GL_OES_point_size_array_COMMANDS: &[&str] = &["glPointSizePointerOES"];

pub const GL_OES_primitive_bounding_box_COMMANDS: &[&str] = &["glPrimitiveBoundingBoxOES"];

pub const GL_OES_query_matrix_COMMANDS: &[&str] = &["glQueryMatrixxOES"];

pub const GL_OES_sample_shading_COMMANDS: &[&str] = &["glMinSampleShadingOES"];

pub const GL_OES_single_precision_COMMANDS: &[&str] = &["glClearDepthfOES", "glClipPlanefOES", "glDepthRangefOES", "glFrustumfOES", "glGetClipPlanefOES", "glOrthofOES"];

pub const GL_OES_tessellation_shader_COMMANDS: &[&str] = &["glPatchParameteriOES"];

pub const GL_OES_texture_3D_COMMANDS: &[&str] = &["glTexImage3DOES", "glTexSubImage3DOES", "glCopyTexSubImage3DOES", "glCompressedTexImage3DOES", "glCompressedTexSubImage3DOES", "glFramebufferTexture3DOES"];

pub const GL_OES_texture_border_clamp_COMMANDS: &[&str] = &["glTexParameterIivOES", "glTexParameterIuivOES", "glGetTexParameterIivOES", "glGetTexParameterIuivOES", "glSamplerParameterIivOES", "glSamplerParameterIuivOES", "glGetSamplerParameterIivOES", "glGetSamplerParameterIuivOES"];

pub const GL_OES_texture_buffer_COMMANDS: &[&str] = &["glTexBufferOES", "glTexBufferRangeOES"];

pub const GL_OES_texture_cube_map_COMMANDS: &[&str] = &["glTexGenfOES", "glTexGenfvOES", "glTexGeniOES", "glTexGenivOES", "glTexGenxOES", "glTexGenxvOES", "glGetTexGenfvOES", "glGetTexGenivOES", "glGetTexGenxvOES"];

pub const GL_OES_texture_storage_multisample_2d_array_COMMANDS: &[&str] = &["glTexStorage3DMultisampleOES"];

pub const GL_OES_texture_view_COMMANDS: &[&str] = &["glTextureViewOES"];

pub const GL_OES_vertex_array_object_COMMANDS: &[&str] = &["glBindVertexArrayOES", "glDeleteVertexArraysOES", "glGenVertexArraysOES", "glIsVertexArrayOES"];

pub const GL_OES_viewport_array_COMMANDS: &[&str] = &["glViewportArrayvOES", "glViewportIndexedfOES", "glViewportIndexedfvOES", "glScissorArrayvOES", "glScissorIndexedOES", "glScissorIndexedvOES", "glDepthRangeArrayfvOES", "glDepthRangeIndexedfOES", "glGetFloati_vOES", "glEnableiOES", "glDisableiOES", "glIsEnablediOES"];

pub const GL_OVR_multiview_COMMANDS: &[&str] = &["glFramebufferTextureMultiviewOVR"];

pub const GL_OVR_multiview_multisampled_render_to_texture_COMMANDS: &[&str] = &["glFramebufferTextureMultisampleMultiviewOVR"];

pub const GL_PGI_misc_hints_COMMANDS: &[&str] = &["glHintPGI"];

pub const GL_QCOM_alpha_test_COMMANDS: &[&str] = &["glAlphaFuncQCOM"];

pub const GL_QCOM_driver_control_COMMANDS: &[&str] = &["glGetDriverControlsQCOM", "glGetDriverControlStringQCOM", "glEnableDriverControlQCOM", "glDisableDriverControlQCOM"];

pub const GL_QCOM_extended_get_COMMANDS: &[&str] = &["glExtGetTexturesQCOM", "glExtGetBuffersQCOM", "glExtGetRenderbuffersQCOM", "glExtGetFramebuffersQCOM", "glExtGetTexLevelParameterivQCOM", "glExtTexObjectStateOverrideiQCOM", "glExtGetTexSubImageQCOM", "glExtGetBufferPointervQCOM"];

pub const GL_QCOM_extended_get2_COMMANDS: &[&str] = &["glExtGetShadersQCOM", "glExtGetProgramsQCOM", "glExtIsProgramBinaryQCOM", "glExtGetProgramBinarySourceQCOM"];

pub const GL_QCOM_frame_extrapolation_COMMANDS: &[&str] = &["glExtrapolateTex2DQCOM"];

pub const GL_QCOM_framebuffer_foveated_COMMANDS: &[&str] = &["glFramebufferFoveationConfigQCOM", "glFramebufferFoveationParametersQCOM"];

pub const GL_QCOM_motion_estimation_COMMANDS: &[&str] = &["glTexEstimateMotionQCOM", "glTexEstimateMotionRegionsQCOM"];

pub const GL_QCOM_shader_framebuffer_fetch_noncoherent_COMMANDS: &[&str] = &["glFramebufferFetchBarrierQCOM"];

pub const GL_QCOM_shading_rate_COMMANDS: &[&str] = &["glShadingRateQCOM"];

pub const GL_QCOM_texture_foveated_COMMANDS: &[&str] = &["glTextureFoveationParametersQCOM"];

pub const GL_QCOM_tiled_rendering_COMMANDS: &[&str] = &["glStartTilingQCOM", "glEndTilingQCOM"];

pub const GL_SGIS_detail_texture_COMMANDS: &[&str] = &["glDetailTexFuncSGIS", "glGetDetailTexFuncSGIS"];

pub const GL_SGIS_fog_function_COMMANDS: &[&str] = &["glFogFuncSGIS", "glGetFogFuncSGIS"];

pub const GL_SGIS_multisample_COMMANDS: &[&str] = &["glSampleMaskSGIS", "glSamplePatternSGIS"];

pub const GL_SGIS_pixel_texture_COMMANDS: &[&str] = &["glPixelTexGenParameteriSGIS", "glPixelTexGenParameterivSGIS", "glPixelTexGenParameterfSGIS", "glPixelTexGenParameterfvSGIS", "glGetPixelTexGenParameterivSGIS", "glGetPixelTexGenParameterfvSGIS"];

pub const GL_SGIS_point_parameters_COMMANDS: &[&str] = &["glPointParameterfSGIS", "glPointParameterfvSGIS"];

pub const GL_SGIS_sharpen_texture_COMMANDS: &[&str] = &["glSharpenTexFuncSGIS", "glGetSharpenTexFuncSGIS"];

pub const GL_SGIS_texture4D_COMMANDS: &[&str] = &["glTexImage4DSGIS", "glTexSubImage4DSGIS"];

pub const GL_SGIS_texture_color_mask_COMMANDS: &[&str] = &["glTextureColorMaskSGIS"];

pub const GL_SGIS_texture_filter4_COMMANDS: &[&str] = &["glGetTexFilterFuncSGIS", "glTexFilterFuncSGIS"];

pub const GL_SGIX_async_COMMANDS: &[&str] = &["glAsyncMarkerSGIX", "glFinishAsyncSGIX", "glPollAsyncSGIX", "glGenAsyncMarkersSGIX", "glDeleteAsyncMarkersSGIX", "glIsAsyncMarkerSGIX"];

pub const GL_SGIX_flush_raster_COMMANDS: &[&str] = &["glFlushRasterSGIX"];

pub const GL_SGIX_fragment_lighting_COMMANDS: &[&str] = &["glFragmentColorMaterialSGIX", "glFragmentLightfSGIX", "glFragmentLightfvSGIX", "glFragmentLightiSGIX", "glFragmentLightivSGIX", "glFragmentLightModelfSGIX", "glFragmentLightModelfvSGIX", "glFragmentLightModeliSGIX", "glFragmentLightModelivSGIX", "glFragmentMaterialfSGIX", "glFragmentMaterialfvSGIX", "glFragmentMaterialiSGIX", "glFragmentMaterialivSGIX", "glGetFragmentLightfvSGIX", "glGetFragmentLightivSGIX", "glGetFragmentMaterialfvSGIX", "glGetFragmentMaterialivSGIX", "glLightEnviSGIX"];

pub const GL_SGIX_framezoom_COMMANDS: &[&str] = &["glFrameZoomSGIX"];

pub const GL_SGIX_igloo_interface_COMMANDS: &[&str] = &["glIglooInterfaceSGIX"];

pub const GL_SGIX_instruments_COMMANDS: &[&str] = &["glGetInstrumentsSGIX", "glInstrumentsBufferSGIX", "glPollInstrumentsSGIX", "glReadInstrumentsSGIX", "glStartInstrumentsSGIX", "glStopInstrumentsSGIX"];

pub const GL_SGIX_list_priority_COMMANDS: &[&str] = &["glGetListParameterfvSGIX", "glGetListParameterivSGIX", "glListParameterfSGIX", "glListParameterfvSGIX", "glListParameteriSGIX", "glListParameterivSGIX"];

pub const GL_SGIX_pixel_texture_COMMANDS: &[&str] = &["glPixelTexGenSGIX"];

pub const GL_SGIX_polynomial_ffd_COMMANDS: &[&str] = &["glDeformationMap3dSGIX", "glDeformationMap3fSGIX", "glDeformSGIX", "glLoadIdentityDeformationMapSGIX"];

pub const GL_SGIX_reference_plane_COMMANDS: &[&str] = &["glReferencePlaneSGIX"];

pub const GL_SGIX_sprite_COMMANDS: &[&str] = &["glSpriteParameterfSGIX", "glSpriteParameterfvSGIX", "glSpriteParameteriSGIX", "glSpriteParameterivSGIX"];

pub const GL_SGIX_tag_sample_buffer_COMMANDS: &[&str] = &["glTagSampleBufferSGIX"];

pub const GL_SGI_color_table_COMMANDS: &[&str] = &["glColorTableSGI", "glColorTableParameterfvSGI", "glColorTableParameterivSGI", "glCopyColorTableSGI", "glGetColorTableSGI", "glGetColorTableParameterfvSGI", "glGetColorTableParameterivSGI"];

pub const GL_SUNX_constant_data_COMMANDS: &[&str] = &["glFinishTextureSUNX"];

pub const GL_SUN_global_alpha_COMMANDS: &[&str] = &["glGlobalAlphaFactorbSUN", "glGlobalAlphaFactorsSUN", "glGlobalAlphaFactoriSUN", "glGlobalAlphaFactorfSUN", "glGlobalAlphaFactordSUN", "glGlobalAlphaFactorubSUN", "glGlobalAlphaFactorusSUN", "glGlobalAlphaFactoruiSUN"];

pub const GL_SUN_mesh_array_COMMANDS: &[&str] = &["glDrawMeshArraysSUN"];

pub const GL_SUN_triangle_list_COMMANDS: &[&str] = &["glReplacementCodeuiSUN", "glReplacementCodeusSUN", "glReplacementCodeubSUN", "glReplacementCodeuivSUN", "glReplacementCodeusvSUN", "glReplacementCodeubvSUN", "glReplacementCodePointerSUN"];

pub const GL_SUN_vertex_COMMANDS: &[&str] = &[
  "glColor4ubVertex2fSUN",
  "glColor4ubVertex2fvSUN",
  "glColor4ubVertex3fSUN",
  "glColor4ubVertex3fvSUN",
  "glColor3fVertex3fSUN",
  "glColor3fVertex3fvSUN",
  "glNormal3fVertex3fSUN",
  "glNormal3fVertex3fvSUN",
  "glColor4fNormal3fVertex3fSUN",
  "glColor4fNormal3fVertex3fvSUN",
  "glTexCoord2fVertex3fSUN",
  "glTexCoord2fVertex3fvSUN",
  "glTexCoord4fVertex4fSUN",
  "glTexCoord4fVertex4fvSUN",
  "glTexCoord2fColor4ubVertex3fSUN",
  "glTexCoord2fColor4ubVertex3fvSUN",
  "glTexCoord2fColor3fVertex3fSUN",
  "glTexCoord2fColor3fVertex3fvSUN",
  "glTexCoord2fNormal3fVertex3fSUN",
  "glTexCoord2fNormal3fVertex3fvSUN",
  "glTexCoord2fColor4fNormal3fVertex3fSUN",
  "glTexCoord2fColor4fNormal3fVertex3fvSUN",
  "glTexCoord4fColor4fNormal3fVertex4fSUN",
  "glTexCoord4fColor4fNormal3fVertex4fvSUN",
  "glReplacementCodeuiVertex3fSUN",
  "glReplacementCodeuiVertex3fvSUN",
  "glReplacementCodeuiColor4ubVertex3fSUN",
  "glReplacementCodeuiColor4ubVertex3fvSUN",
  "glReplacementCodeuiColor3fVertex3fSUN",
  "glReplacementCodeuiColor3fVertex3fvSUN",
  "glReplacementCodeuiNormal3fVertex3fSUN",
  "glReplacementCodeuiNormal3fVertex3fvSUN",
  "glReplacementCodeuiColor4fNormal3fVertex3fSUN",
  "glReplacementCodeuiColor4fNormal3fVertex3fvSUN",
  "glReplacementCodeuiTexCoord2fVertex3fSUN",
  "glReplacementCodeuiTexCoord2fVertex3fvSUN",
  "glReplacementCodeuiTexCoord2fNormal3fVertex3fSUN",
  "glReplacementCodeuiTexCoord2fNormal3fVertex3fvSUN",
  "glReplacementCodeuiTexCoord2fColor4fNormal3fVertex3fSUN",
  "glReplacementCodeuiTexCoord2fColor4fNormal3fVertex3fvSUN",
];
