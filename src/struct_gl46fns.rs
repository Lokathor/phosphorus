use super::*;

// Note(Lokathor): _p for ptr, _t for type

#[repr(C)]
pub struct GlFns46 {
  glActiveShaderProgram_p: glActiveShaderProgram_t,
  glActiveTexture_p: glActiveTexture_t,
  glAttachShader_p: glAttachShader_t,
  glBeginConditionalRender_p: glBeginConditionalRender_t,
  glBeginQuery_p: glBeginQuery_t,
  glBeginQueryIndexed_p: glBeginQueryIndexed_t,
  glBeginTransformFeedback_p: glBeginTransformFeedback_t,
  glBindAttribLocation_p: glBindAttribLocation_t,
  glBindBuffer_p: glBindBuffer_t,
  glBindBufferBase_p: glBindBufferBase_t,
  glBindBufferRange_p: glBindBufferRange_t,
  glBindBuffersBase_p: glBindBuffersBase_t,
  glBindBuffersRange_p: glBindBuffersRange_t,
  glBindFragDataLocation_p: glBindFragDataLocation_t,
  glBindFragDataLocationIndexed_p: glBindFragDataLocationIndexed_t,
  glBindFramebuffer_p: glBindFramebuffer_t,
  glBindImageTexture_p: glBindImageTexture_t,
  glBindImageTextures_p: glBindImageTextures_t,
  glBindProgramPipeline_p: glBindProgramPipeline_t,
  glBindRenderbuffer_p: glBindRenderbuffer_t,
  glBindSampler_p: glBindSampler_t,
  glBindSamplers_p: glBindSamplers_t,
  glBindTexture_p: glBindTexture_t,
  glBindTextureUnit_p: glBindTextureUnit_t,
  glBindTextures_p: glBindTextures_t,
  glBindTransformFeedback_p: glBindTransformFeedback_t,
  glBindVertexArray_p: glBindVertexArray_t,
  glBindVertexBuffer_p: glBindVertexBuffer_t,
  glBindVertexBuffers_p: glBindVertexBuffers_t,
  glBlendColor_p: glBlendColor_t,
  glBlendEquation_p: glBlendEquation_t,
  glBlendEquationSeparate_p: glBlendEquationSeparate_t,
  glBlendEquationSeparatei_p: glBlendEquationSeparatei_t,
  glBlendEquationi_p: glBlendEquationi_t,
  glBlendFunc_p: glBlendFunc_t,
  glBlendFuncSeparate_p: glBlendFuncSeparate_t,
  glBlendFuncSeparatei_p: glBlendFuncSeparatei_t,
  glBlendFunci_p: glBlendFunci_t,
  glBlitFramebuffer_p: glBlitFramebuffer_t,
  glBlitNamedFramebuffer_p: glBlitNamedFramebuffer_t,
  glBufferData_p: glBufferData_t,
  glBufferStorage_p: glBufferStorage_t,
  glBufferSubData_p: glBufferSubData_t,
  glCheckFramebufferStatus_p: glCheckFramebufferStatus_t,
  glCheckNamedFramebufferStatus_p: glCheckNamedFramebufferStatus_t,
  glClampColor_p: glClampColor_t,
  glClear_p: glClear_t,
  glClearBufferData_p: glClearBufferData_t,
  glClearBufferSubData_p: glClearBufferSubData_t,
  glClearBufferfi_p: glClearBufferfi_t,
  glClearBufferfv_p: glClearBufferfv_t,
  glClearBufferiv_p: glClearBufferiv_t,
  glClearBufferuiv_p: glClearBufferuiv_t,
  glClearColor_p: glClearColor_t,
  glClearDepth_p: glClearDepth_t,
  glClearDepthf_p: glClearDepthf_t,
  glClearNamedBufferData_p: glClearNamedBufferData_t,
  glClearNamedBufferSubData_p: glClearNamedBufferSubData_t,
  glClearNamedFramebufferfi_p: glClearNamedFramebufferfi_t,
  glClearNamedFramebufferfv_p: glClearNamedFramebufferfv_t,
  glClearNamedFramebufferiv_p: glClearNamedFramebufferiv_t,
  glClearNamedFramebufferuiv_p: glClearNamedFramebufferuiv_t,
  glClearStencil_p: glClearStencil_t,
  glClearTexImage_p: glClearTexImage_t,
  glClearTexSubImage_p: glClearTexSubImage_t,
  glClientWaitSync_p: glClientWaitSync_t,
  glClipControl_p: glClipControl_t,
  glColorMask_p: glColorMask_t,
  glColorMaski_p: glColorMaski_t,
  glColorP3ui_p: glColorP3ui_t,
  glColorP3uiv_p: glColorP3uiv_t,
  glColorP4ui_p: glColorP4ui_t,
  glColorP4uiv_p: glColorP4uiv_t,
  glCompileShader_p: glCompileShader_t,
  glCompressedTexImage1D_p: glCompressedTexImage1D_t,
  glCompressedTexImage2D_p: glCompressedTexImage2D_t,
  glCompressedTexImage3D_p: glCompressedTexImage3D_t,
  glCompressedTexSubImage1D_p: glCompressedTexSubImage1D_t,
  glCompressedTexSubImage2D_p: glCompressedTexSubImage2D_t,
  glCompressedTexSubImage3D_p: glCompressedTexSubImage3D_t,
  glCompressedTextureSubImage1D_p: glCompressedTextureSubImage1D_t,
  glCompressedTextureSubImage2D_p: glCompressedTextureSubImage2D_t,
  glCompressedTextureSubImage3D_p: glCompressedTextureSubImage3D_t,
  glCopyBufferSubData_p: glCopyBufferSubData_t,
  glCopyImageSubData_p: glCopyImageSubData_t,
  glCopyNamedBufferSubData_p: glCopyNamedBufferSubData_t,
  glCopyTexImage1D_p: glCopyTexImage1D_t,
  glCopyTexImage2D_p: glCopyTexImage2D_t,
  glCopyTexSubImage1D_p: glCopyTexSubImage1D_t,
  glCopyTexSubImage2D_p: glCopyTexSubImage2D_t,
  glCopyTexSubImage3D_p: glCopyTexSubImage3D_t,
  glCopyTextureSubImage1D_p: glCopyTextureSubImage1D_t,
  glCopyTextureSubImage2D_p: glCopyTextureSubImage2D_t,
  glCopyTextureSubImage3D_p: glCopyTextureSubImage3D_t,
  glCreateBuffers_p: glCreateBuffers_t,
  glCreateFramebuffers_p: glCreateFramebuffers_t,
  glCreateProgram_p: glCreateProgram_t,
  glCreateProgramPipelines_p: glCreateProgramPipelines_t,
  glCreateQueries_p: glCreateQueries_t,
  glCreateRenderbuffers_p: glCreateRenderbuffers_t,
  glCreateSamplers_p: glCreateSamplers_t,
  glCreateShader_p: glCreateShader_t,
  glCreateShaderProgramv_p: glCreateShaderProgramv_t,
  glCreateTextures_p: glCreateTextures_t,
  glCreateTransformFeedbacks_p: glCreateTransformFeedbacks_t,
  glCreateVertexArrays_p: glCreateVertexArrays_t,
  glCullFace_p: glCullFace_t,
  glDebugMessageCallback_p: glDebugMessageCallback_t,
  glDebugMessageControl_p: glDebugMessageControl_t,
  glDebugMessageInsert_p: glDebugMessageInsert_t,
  glDeleteBuffers_p: glDeleteBuffers_t,
  glDeleteFramebuffers_p: glDeleteFramebuffers_t,
  glDeleteProgram_p: glDeleteProgram_t,
  glDeleteProgramPipelines_p: glDeleteProgramPipelines_t,
  glDeleteQueries_p: glDeleteQueries_t,
  glDeleteRenderbuffers_p: glDeleteRenderbuffers_t,
  glDeleteSamplers_p: glDeleteSamplers_t,
  glDeleteShader_p: glDeleteShader_t,
  glDeleteSync_p: glDeleteSync_t,
  glDeleteTextures_p: glDeleteTextures_t,
  glDeleteTransformFeedbacks_p: glDeleteTransformFeedbacks_t,
  glDeleteVertexArrays_p: glDeleteVertexArrays_t,
  glDepthFunc_p: glDepthFunc_t,
  glDepthMask_p: glDepthMask_t,
  glDepthRange_p: glDepthRange_t,
  glDepthRangeArrayv_p: glDepthRangeArrayv_t,
  glDepthRangeIndexed_p: glDepthRangeIndexed_t,
  glDepthRangef_p: glDepthRangef_t,
  glDetachShader_p: glDetachShader_t,
  glDisable_p: glDisable_t,
  glDisableVertexArrayAttrib_p: glDisableVertexArrayAttrib_t,
  glDisableVertexAttribArray_p: glDisableVertexAttribArray_t,
  glDisablei_p: glDisablei_t,
  glDispatchCompute_p: glDispatchCompute_t,
  glDispatchComputeIndirect_p: glDispatchComputeIndirect_t,
  glDrawArrays_p: glDrawArrays_t,
  glDrawArraysIndirect_p: glDrawArraysIndirect_t,
  glDrawArraysInstanced_p: glDrawArraysInstanced_t,
  glDrawArraysInstancedBaseInstance_p: glDrawArraysInstancedBaseInstance_t,
  glDrawBuffer_p: glDrawBuffer_t,
  glDrawBuffers_p: glDrawBuffers_t,
  glDrawElements_p: glDrawElements_t,
  glDrawElementsBaseVertex_p: glDrawElementsBaseVertex_t,
  glDrawElementsIndirect_p: glDrawElementsIndirect_t,
  glDrawElementsInstanced_p: glDrawElementsInstanced_t,
  glDrawElementsInstancedBaseInstance_p: glDrawElementsInstancedBaseInstance_t,
  glDrawElementsInstancedBaseVertex_p: glDrawElementsInstancedBaseVertex_t,
  glDrawElementsInstancedBaseVertexBaseInstance_p: glDrawElementsInstancedBaseVertexBaseInstance_t,
  glDrawRangeElements_p: glDrawRangeElements_t,
  glDrawRangeElementsBaseVertex_p: glDrawRangeElementsBaseVertex_t,
  glDrawTransformFeedback_p: glDrawTransformFeedback_t,
  glDrawTransformFeedbackInstanced_p: glDrawTransformFeedbackInstanced_t,
  glDrawTransformFeedbackStream_p: glDrawTransformFeedbackStream_t,
  glDrawTransformFeedbackStreamInstanced_p: glDrawTransformFeedbackStreamInstanced_t,
  glEnable_p: glEnable_t,
  glEnableVertexArrayAttrib_p: glEnableVertexArrayAttrib_t,
  glEnableVertexAttribArray_p: glEnableVertexAttribArray_t,
  glEnablei_p: glEnablei_t,
  glEndConditionalRender_p: glEndConditionalRender_t,
  glEndQuery_p: glEndQuery_t,
  glEndQueryIndexed_p: glEndQueryIndexed_t,
  glEndTransformFeedback_p: glEndTransformFeedback_t,
  glFenceSync_p: glFenceSync_t,
  glFinish_p: glFinish_t,
  glFlush_p: glFlush_t,
  glFlushMappedBufferRange_p: glFlushMappedBufferRange_t,
  glFlushMappedNamedBufferRange_p: glFlushMappedNamedBufferRange_t,
  glFramebufferParameteri_p: glFramebufferParameteri_t,
  glFramebufferRenderbuffer_p: glFramebufferRenderbuffer_t,
  glFramebufferTexture_p: glFramebufferTexture_t,
  glFramebufferTexture1D_p: glFramebufferTexture1D_t,
  glFramebufferTexture2D_p: glFramebufferTexture2D_t,
  glFramebufferTexture3D_p: glFramebufferTexture3D_t,
  glFramebufferTextureLayer_p: glFramebufferTextureLayer_t,
  glFrontFace_p: glFrontFace_t,
  glGenBuffers_p: glGenBuffers_t,
  glGenFramebuffers_p: glGenFramebuffers_t,
  glGenProgramPipelines_p: glGenProgramPipelines_t,
  glGenQueries_p: glGenQueries_t,
  glGenRenderbuffers_p: glGenRenderbuffers_t,
  glGenSamplers_p: glGenSamplers_t,
  glGenTextures_p: glGenTextures_t,
  glGenTransformFeedbacks_p: glGenTransformFeedbacks_t,
  glGenVertexArrays_p: glGenVertexArrays_t,
  glGenerateMipmap_p: glGenerateMipmap_t,
  glGenerateTextureMipmap_p: glGenerateTextureMipmap_t,
  glGetActiveAtomicCounterBufferiv_p: glGetActiveAtomicCounterBufferiv_t,
  glGetActiveAttrib_p: glGetActiveAttrib_t,
  glGetActiveSubroutineName_p: glGetActiveSubroutineName_t,
  glGetActiveSubroutineUniformName_p: glGetActiveSubroutineUniformName_t,
  glGetActiveSubroutineUniformiv_p: glGetActiveSubroutineUniformiv_t,
  glGetActiveUniform_p: glGetActiveUniform_t,
  glGetActiveUniformBlockName_p: glGetActiveUniformBlockName_t,
  glGetActiveUniformBlockiv_p: glGetActiveUniformBlockiv_t,
  glGetActiveUniformName_p: glGetActiveUniformName_t,
  glGetActiveUniformsiv_p: glGetActiveUniformsiv_t,
  glGetAttachedShaders_p: glGetAttachedShaders_t,
  glGetAttribLocation_p: glGetAttribLocation_t,
  glGetBooleani_v_p: glGetBooleani_v_t,
  glGetBooleanv_p: glGetBooleanv_t,
  glGetBufferParameteri64v_p: glGetBufferParameteri64v_t,
  glGetBufferParameteriv_p: glGetBufferParameteriv_t,
  glGetBufferPointerv_p: glGetBufferPointerv_t,
  glGetBufferSubData_p: glGetBufferSubData_t,
  glGetCompressedTexImage_p: glGetCompressedTexImage_t,
  glGetCompressedTextureImage_p: glGetCompressedTextureImage_t,
  glGetCompressedTextureSubImage_p: glGetCompressedTextureSubImage_t,
  glGetDebugMessageLog_p: glGetDebugMessageLog_t,
  glGetDoublei_v_p: glGetDoublei_v_t,
  glGetDoublev_p: glGetDoublev_t,
  glGetError_p: glGetError_t,
  glGetFloati_v_p: glGetFloati_v_t,
  glGetFloatv_p: glGetFloatv_t,
  glGetFragDataIndex_p: glGetFragDataIndex_t,
  glGetFragDataLocation_p: glGetFragDataLocation_t,
  glGetFramebufferAttachmentParameteriv_p: glGetFramebufferAttachmentParameteriv_t,
  glGetFramebufferParameteriv_p: glGetFramebufferParameteriv_t,
  glGetGraphicsResetStatus_p: glGetGraphicsResetStatus_t,
  glGetInteger64i_v_p: glGetInteger64i_v_t,
  glGetInteger64v_p: glGetInteger64v_t,
  glGetIntegeri_v_p: glGetIntegeri_v_t,
  glGetIntegerv_p: glGetIntegerv_t,
  glGetInternalformati64v_p: glGetInternalformati64v_t,
  glGetInternalformativ_p: glGetInternalformativ_t,
  glGetMultisamplefv_p: glGetMultisamplefv_t,
  glGetNamedBufferParameteri64v_p: glGetNamedBufferParameteri64v_t,
  glGetNamedBufferParameteriv_p: glGetNamedBufferParameteriv_t,
  glGetNamedBufferPointerv_p: glGetNamedBufferPointerv_t,
  glGetNamedBufferSubData_p: glGetNamedBufferSubData_t,
  glGetNamedFramebufferAttachmentParameteriv_p: glGetNamedFramebufferAttachmentParameteriv_t,
  glGetNamedFramebufferParameteriv_p: glGetNamedFramebufferParameteriv_t,
  glGetNamedRenderbufferParameteriv_p: glGetNamedRenderbufferParameteriv_t,
  glGetObjectLabel_p: glGetObjectLabel_t,
  glGetObjectPtrLabel_p: glGetObjectPtrLabel_t,
  glGetPointerv_p: glGetPointerv_t,
  glGetProgramBinary_p: glGetProgramBinary_t,
  glGetProgramInfoLog_p: glGetProgramInfoLog_t,
  glGetProgramInterfaceiv_p: glGetProgramInterfaceiv_t,
  glGetProgramPipelineInfoLog_p: glGetProgramPipelineInfoLog_t,
  glGetProgramPipelineiv_p: glGetProgramPipelineiv_t,
  glGetProgramResourceIndex_p: glGetProgramResourceIndex_t,
  glGetProgramResourceLocation_p: glGetProgramResourceLocation_t,
  glGetProgramResourceLocationIndex_p: glGetProgramResourceLocationIndex_t,
  glGetProgramResourceName_p: glGetProgramResourceName_t,
  glGetProgramResourceiv_p: glGetProgramResourceiv_t,
  glGetProgramStageiv_p: glGetProgramStageiv_t,
  glGetProgramiv_p: glGetProgramiv_t,
  glGetQueryBufferObjecti64v_p: glGetQueryBufferObjecti64v_t,
  glGetQueryBufferObjectiv_p: glGetQueryBufferObjectiv_t,
  glGetQueryBufferObjectui64v_p: glGetQueryBufferObjectui64v_t,
  glGetQueryBufferObjectuiv_p: glGetQueryBufferObjectuiv_t,
  glGetQueryIndexediv_p: glGetQueryIndexediv_t,
  glGetQueryObjecti64v_p: glGetQueryObjecti64v_t,
  glGetQueryObjectiv_p: glGetQueryObjectiv_t,
  glGetQueryObjectui64v_p: glGetQueryObjectui64v_t,
  glGetQueryObjectuiv_p: glGetQueryObjectuiv_t,
  glGetQueryiv_p: glGetQueryiv_t,
  glGetRenderbufferParameteriv_p: glGetRenderbufferParameteriv_t,
  glGetSamplerParameterIiv_p: glGetSamplerParameterIiv_t,
  glGetSamplerParameterIuiv_p: glGetSamplerParameterIuiv_t,
  glGetSamplerParameterfv_p: glGetSamplerParameterfv_t,
  glGetSamplerParameteriv_p: glGetSamplerParameteriv_t,
  glGetShaderInfoLog_p: glGetShaderInfoLog_t,
  glGetShaderPrecisionFormat_p: glGetShaderPrecisionFormat_t,
  glGetShaderSource_p: glGetShaderSource_t,
  glGetShaderiv_p: glGetShaderiv_t,
  glGetString_p: glGetString_t,
  glGetStringi_p: glGetStringi_t,
  glGetSubroutineIndex_p: glGetSubroutineIndex_t,
  glGetSubroutineUniformLocation_p: glGetSubroutineUniformLocation_t,
  glGetSynciv_p: glGetSynciv_t,
  glGetTexImage_p: glGetTexImage_t,
  glGetTexLevelParameterfv_p: glGetTexLevelParameterfv_t,
  glGetTexLevelParameteriv_p: glGetTexLevelParameteriv_t,
  glGetTexParameterIiv_p: glGetTexParameterIiv_t,
  glGetTexParameterIuiv_p: glGetTexParameterIuiv_t,
  glGetTexParameterfv_p: glGetTexParameterfv_t,
  glGetTexParameteriv_p: glGetTexParameteriv_t,
  glGetTextureImage_p: glGetTextureImage_t,
  glGetTextureLevelParameterfv_p: glGetTextureLevelParameterfv_t,
  glGetTextureLevelParameteriv_p: glGetTextureLevelParameteriv_t,
  glGetTextureParameterIiv_p: glGetTextureParameterIiv_t,
  glGetTextureParameterIuiv_p: glGetTextureParameterIuiv_t,
  glGetTextureParameterfv_p: glGetTextureParameterfv_t,
  glGetTextureParameteriv_p: glGetTextureParameteriv_t,
  glGetTextureSubImage_p: glGetTextureSubImage_t,
  glGetTransformFeedbackVarying_p: glGetTransformFeedbackVarying_t,
  glGetTransformFeedbacki64_v_p: glGetTransformFeedbacki64_v_t,
  glGetTransformFeedbacki_v_p: glGetTransformFeedbacki_v_t,
  glGetTransformFeedbackiv_p: glGetTransformFeedbackiv_t,
  glGetUniformBlockIndex_p: glGetUniformBlockIndex_t,
  glGetUniformIndices_p: glGetUniformIndices_t,
  glGetUniformLocation_p: glGetUniformLocation_t,
  glGetUniformSubroutineuiv_p: glGetUniformSubroutineuiv_t,
  glGetUniformdv_p: glGetUniformdv_t,
  glGetUniformfv_p: glGetUniformfv_t,
  glGetUniformiv_p: glGetUniformiv_t,
  glGetUniformuiv_p: glGetUniformuiv_t,
  glGetVertexArrayIndexed64iv_p: glGetVertexArrayIndexed64iv_t,
  glGetVertexArrayIndexediv_p: glGetVertexArrayIndexediv_t,
  glGetVertexArrayiv_p: glGetVertexArrayiv_t,
  glGetVertexAttribIiv_p: glGetVertexAttribIiv_t,
  glGetVertexAttribIuiv_p: glGetVertexAttribIuiv_t,
  glGetVertexAttribLdv_p: glGetVertexAttribLdv_t,
  glGetVertexAttribPointerv_p: glGetVertexAttribPointerv_t,
  glGetVertexAttribdv_p: glGetVertexAttribdv_t,
  glGetVertexAttribfv_p: glGetVertexAttribfv_t,
  glGetVertexAttribiv_p: glGetVertexAttribiv_t,
  glGetnColorTable_p: glGetnColorTable_t,
  glGetnCompressedTexImage_p: glGetnCompressedTexImage_t,
  glGetnConvolutionFilter_p: glGetnConvolutionFilter_t,
  glGetnHistogram_p: glGetnHistogram_t,
  glGetnMapdv_p: glGetnMapdv_t,
  glGetnMapfv_p: glGetnMapfv_t,
  glGetnMapiv_p: glGetnMapiv_t,
  glGetnMinmax_p: glGetnMinmax_t,
  glGetnPixelMapfv_p: glGetnPixelMapfv_t,
  glGetnPixelMapuiv_p: glGetnPixelMapuiv_t,
  glGetnPixelMapusv_p: glGetnPixelMapusv_t,
  glGetnPolygonStipple_p: glGetnPolygonStipple_t,
  glGetnSeparableFilter_p: glGetnSeparableFilter_t,
  glGetnTexImage_p: glGetnTexImage_t,
  glGetnUniformdv_p: glGetnUniformdv_t,
  glGetnUniformfv_p: glGetnUniformfv_t,
  glGetnUniformiv_p: glGetnUniformiv_t,
  glGetnUniformuiv_p: glGetnUniformuiv_t,
  glHint_p: glHint_t,
  glInvalidateBufferData_p: glInvalidateBufferData_t,
  glInvalidateBufferSubData_p: glInvalidateBufferSubData_t,
  glInvalidateFramebuffer_p: glInvalidateFramebuffer_t,
  glInvalidateNamedFramebufferData_p: glInvalidateNamedFramebufferData_t,
  glInvalidateNamedFramebufferSubData_p: glInvalidateNamedFramebufferSubData_t,
  glInvalidateSubFramebuffer_p: glInvalidateSubFramebuffer_t,
  glInvalidateTexImage_p: glInvalidateTexImage_t,
  glInvalidateTexSubImage_p: glInvalidateTexSubImage_t,
  glIsBuffer_p: glIsBuffer_t,
  glIsEnabled_p: glIsEnabled_t,
  glIsEnabledi_p: glIsEnabledi_t,
  glIsFramebuffer_p: glIsFramebuffer_t,
  glIsProgram_p: glIsProgram_t,
  glIsProgramPipeline_p: glIsProgramPipeline_t,
  glIsQuery_p: glIsQuery_t,
  glIsRenderbuffer_p: glIsRenderbuffer_t,
  glIsSampler_p: glIsSampler_t,
  glIsShader_p: glIsShader_t,
  glIsSync_p: glIsSync_t,
  glIsTexture_p: glIsTexture_t,
  glIsTransformFeedback_p: glIsTransformFeedback_t,
  glIsVertexArray_p: glIsVertexArray_t,
  glLineWidth_p: glLineWidth_t,
  glLinkProgram_p: glLinkProgram_t,
  glLogicOp_p: glLogicOp_t,
  glMapBuffer_p: glMapBuffer_t,
  glMapBufferRange_p: glMapBufferRange_t,
  glMapNamedBuffer_p: glMapNamedBuffer_t,
  glMapNamedBufferRange_p: glMapNamedBufferRange_t,
  glMemoryBarrier_p: glMemoryBarrier_t,
  glMemoryBarrierByRegion_p: glMemoryBarrierByRegion_t,
  glMinSampleShading_p: glMinSampleShading_t,
  glMultiDrawArrays_p: glMultiDrawArrays_t,
  glMultiDrawArraysIndirect_p: glMultiDrawArraysIndirect_t,
  glMultiDrawArraysIndirectCount_p: glMultiDrawArraysIndirectCount_t,
  glMultiDrawElements_p: glMultiDrawElements_t,
  glMultiDrawElementsBaseVertex_p: glMultiDrawElementsBaseVertex_t,
  glMultiDrawElementsIndirect_p: glMultiDrawElementsIndirect_t,
  glMultiDrawElementsIndirectCount_p: glMultiDrawElementsIndirectCount_t,
  glMultiTexCoordP1ui_p: glMultiTexCoordP1ui_t,
  glMultiTexCoordP1uiv_p: glMultiTexCoordP1uiv_t,
  glMultiTexCoordP2ui_p: glMultiTexCoordP2ui_t,
  glMultiTexCoordP2uiv_p: glMultiTexCoordP2uiv_t,
  glMultiTexCoordP3ui_p: glMultiTexCoordP3ui_t,
  glMultiTexCoordP3uiv_p: glMultiTexCoordP3uiv_t,
  glMultiTexCoordP4ui_p: glMultiTexCoordP4ui_t,
  glMultiTexCoordP4uiv_p: glMultiTexCoordP4uiv_t,
  glNamedBufferData_p: glNamedBufferData_t,
  glNamedBufferStorage_p: glNamedBufferStorage_t,
  glNamedBufferSubData_p: glNamedBufferSubData_t,
  glNamedFramebufferDrawBuffer_p: glNamedFramebufferDrawBuffer_t,
  glNamedFramebufferDrawBuffers_p: glNamedFramebufferDrawBuffers_t,
  glNamedFramebufferParameteri_p: glNamedFramebufferParameteri_t,
  glNamedFramebufferReadBuffer_p: glNamedFramebufferReadBuffer_t,
  glNamedFramebufferRenderbuffer_p: glNamedFramebufferRenderbuffer_t,
  glNamedFramebufferTexture_p: glNamedFramebufferTexture_t,
  glNamedFramebufferTextureLayer_p: glNamedFramebufferTextureLayer_t,
  glNamedRenderbufferStorage_p: glNamedRenderbufferStorage_t,
  glNamedRenderbufferStorageMultisample_p: glNamedRenderbufferStorageMultisample_t,
  glNormalP3ui_p: glNormalP3ui_t,
  glNormalP3uiv_p: glNormalP3uiv_t,
  glObjectLabel_p: glObjectLabel_t,
  glObjectPtrLabel_p: glObjectPtrLabel_t,
  glPatchParameterfv_p: glPatchParameterfv_t,
  glPatchParameteri_p: glPatchParameteri_t,
  glPauseTransformFeedback_p: glPauseTransformFeedback_t,
  glPixelStoref_p: glPixelStoref_t,
  glPixelStorei_p: glPixelStorei_t,
  glPointParameterf_p: glPointParameterf_t,
  glPointParameterfv_p: glPointParameterfv_t,
  glPointParameteri_p: glPointParameteri_t,
  glPointParameteriv_p: glPointParameteriv_t,
  glPointSize_p: glPointSize_t,
  glPolygonMode_p: glPolygonMode_t,
  glPolygonOffset_p: glPolygonOffset_t,
  glPolygonOffsetClamp_p: glPolygonOffsetClamp_t,
  glPopDebugGroup_p: glPopDebugGroup_t,
  glPrimitiveRestartIndex_p: glPrimitiveRestartIndex_t,
  glProgramBinary_p: glProgramBinary_t,
  glProgramParameteri_p: glProgramParameteri_t,
  glProgramUniform1d_p: glProgramUniform1d_t,
  glProgramUniform1dv_p: glProgramUniform1dv_t,
  glProgramUniform1f_p: glProgramUniform1f_t,
  glProgramUniform1fv_p: glProgramUniform1fv_t,
  glProgramUniform1i_p: glProgramUniform1i_t,
  glProgramUniform1iv_p: glProgramUniform1iv_t,
  glProgramUniform1ui_p: glProgramUniform1ui_t,
  glProgramUniform1uiv_p: glProgramUniform1uiv_t,
  glProgramUniform2d_p: glProgramUniform2d_t,
  glProgramUniform2dv_p: glProgramUniform2dv_t,
  glProgramUniform2f_p: glProgramUniform2f_t,
  glProgramUniform2fv_p: glProgramUniform2fv_t,
  glProgramUniform2i_p: glProgramUniform2i_t,
  glProgramUniform2iv_p: glProgramUniform2iv_t,
  glProgramUniform2ui_p: glProgramUniform2ui_t,
  glProgramUniform2uiv_p: glProgramUniform2uiv_t,
  glProgramUniform3d_p: glProgramUniform3d_t,
  glProgramUniform3dv_p: glProgramUniform3dv_t,
  glProgramUniform3f_p: glProgramUniform3f_t,
  glProgramUniform3fv_p: glProgramUniform3fv_t,
  glProgramUniform3i_p: glProgramUniform3i_t,
  glProgramUniform3iv_p: glProgramUniform3iv_t,
  glProgramUniform3ui_p: glProgramUniform3ui_t,
  glProgramUniform3uiv_p: glProgramUniform3uiv_t,
  glProgramUniform4d_p: glProgramUniform4d_t,
  glProgramUniform4dv_p: glProgramUniform4dv_t,
  glProgramUniform4f_p: glProgramUniform4f_t,
  glProgramUniform4fv_p: glProgramUniform4fv_t,
  glProgramUniform4i_p: glProgramUniform4i_t,
  glProgramUniform4iv_p: glProgramUniform4iv_t,
  glProgramUniform4ui_p: glProgramUniform4ui_t,
  glProgramUniform4uiv_p: glProgramUniform4uiv_t,
  glProgramUniformMatrix2dv_p: glProgramUniformMatrix2dv_t,
  glProgramUniformMatrix2fv_p: glProgramUniformMatrix2fv_t,
  glProgramUniformMatrix2x3dv_p: glProgramUniformMatrix2x3dv_t,
  glProgramUniformMatrix2x3fv_p: glProgramUniformMatrix2x3fv_t,
  glProgramUniformMatrix2x4dv_p: glProgramUniformMatrix2x4dv_t,
  glProgramUniformMatrix2x4fv_p: glProgramUniformMatrix2x4fv_t,
  glProgramUniformMatrix3dv_p: glProgramUniformMatrix3dv_t,
  glProgramUniformMatrix3fv_p: glProgramUniformMatrix3fv_t,
  glProgramUniformMatrix3x2dv_p: glProgramUniformMatrix3x2dv_t,
  glProgramUniformMatrix3x2fv_p: glProgramUniformMatrix3x2fv_t,
  glProgramUniformMatrix3x4dv_p: glProgramUniformMatrix3x4dv_t,
  glProgramUniformMatrix3x4fv_p: glProgramUniformMatrix3x4fv_t,
  glProgramUniformMatrix4dv_p: glProgramUniformMatrix4dv_t,
  glProgramUniformMatrix4fv_p: glProgramUniformMatrix4fv_t,
  glProgramUniformMatrix4x2dv_p: glProgramUniformMatrix4x2dv_t,
  glProgramUniformMatrix4x2fv_p: glProgramUniformMatrix4x2fv_t,
  glProgramUniformMatrix4x3dv_p: glProgramUniformMatrix4x3dv_t,
  glProgramUniformMatrix4x3fv_p: glProgramUniformMatrix4x3fv_t,
  glProvokingVertex_p: glProvokingVertex_t,
  glPushDebugGroup_p: glPushDebugGroup_t,
  glQueryCounter_p: glQueryCounter_t,
  glReadBuffer_p: glReadBuffer_t,
  glReadPixels_p: glReadPixels_t,
  glReadnPixels_p: glReadnPixels_t,
  glReleaseShaderCompiler_p: glReleaseShaderCompiler_t,
  glRenderbufferStorage_p: glRenderbufferStorage_t,
  glRenderbufferStorageMultisample_p: glRenderbufferStorageMultisample_t,
  glResumeTransformFeedback_p: glResumeTransformFeedback_t,
  glSampleCoverage_p: glSampleCoverage_t,
  glSampleMaski_p: glSampleMaski_t,
  glSamplerParameterIiv_p: glSamplerParameterIiv_t,
  glSamplerParameterIuiv_p: glSamplerParameterIuiv_t,
  glSamplerParameterf_p: glSamplerParameterf_t,
  glSamplerParameterfv_p: glSamplerParameterfv_t,
  glSamplerParameteri_p: glSamplerParameteri_t,
  glSamplerParameteriv_p: glSamplerParameteriv_t,
  glScissor_p: glScissor_t,
  glScissorArrayv_p: glScissorArrayv_t,
  glScissorIndexed_p: glScissorIndexed_t,
  glScissorIndexedv_p: glScissorIndexedv_t,
  glSecondaryColorP3ui_p: glSecondaryColorP3ui_t,
  glSecondaryColorP3uiv_p: glSecondaryColorP3uiv_t,
  glShaderBinary_p: glShaderBinary_t,
  glShaderSource_p: glShaderSource_t,
  glShaderStorageBlockBinding_p: glShaderStorageBlockBinding_t,
  glSpecializeShader_p: glSpecializeShader_t,
  glStencilFunc_p: glStencilFunc_t,
  glStencilFuncSeparate_p: glStencilFuncSeparate_t,
  glStencilMask_p: glStencilMask_t,
  glStencilMaskSeparate_p: glStencilMaskSeparate_t,
  glStencilOp_p: glStencilOp_t,
  glStencilOpSeparate_p: glStencilOpSeparate_t,
  glTexBuffer_p: glTexBuffer_t,
  glTexBufferRange_p: glTexBufferRange_t,
  glTexCoordP1ui_p: glTexCoordP1ui_t,
  glTexCoordP1uiv_p: glTexCoordP1uiv_t,
  glTexCoordP2ui_p: glTexCoordP2ui_t,
  glTexCoordP2uiv_p: glTexCoordP2uiv_t,
  glTexCoordP3ui_p: glTexCoordP3ui_t,
  glTexCoordP3uiv_p: glTexCoordP3uiv_t,
  glTexCoordP4ui_p: glTexCoordP4ui_t,
  glTexCoordP4uiv_p: glTexCoordP4uiv_t,
  glTexImage1D_p: glTexImage1D_t,
  glTexImage2D_p: glTexImage2D_t,
  glTexImage2DMultisample_p: glTexImage2DMultisample_t,
  glTexImage3D_p: glTexImage3D_t,
  glTexImage3DMultisample_p: glTexImage3DMultisample_t,
  glTexParameterIiv_p: glTexParameterIiv_t,
  glTexParameterIuiv_p: glTexParameterIuiv_t,
  glTexParameterf_p: glTexParameterf_t,
  glTexParameterfv_p: glTexParameterfv_t,
  glTexParameteri_p: glTexParameteri_t,
  glTexParameteriv_p: glTexParameteriv_t,
  glTexStorage1D_p: glTexStorage1D_t,
  glTexStorage2D_p: glTexStorage2D_t,
  glTexStorage2DMultisample_p: glTexStorage2DMultisample_t,
  glTexStorage3D_p: glTexStorage3D_t,
  glTexStorage3DMultisample_p: glTexStorage3DMultisample_t,
  glTexSubImage1D_p: glTexSubImage1D_t,
  glTexSubImage2D_p: glTexSubImage2D_t,
  glTexSubImage3D_p: glTexSubImage3D_t,
  glTextureBarrier_p: glTextureBarrier_t,
  glTextureBuffer_p: glTextureBuffer_t,
  glTextureBufferRange_p: glTextureBufferRange_t,
  glTextureParameterIiv_p: glTextureParameterIiv_t,
  glTextureParameterIuiv_p: glTextureParameterIuiv_t,
  glTextureParameterf_p: glTextureParameterf_t,
  glTextureParameterfv_p: glTextureParameterfv_t,
  glTextureParameteri_p: glTextureParameteri_t,
  glTextureParameteriv_p: glTextureParameteriv_t,
  glTextureStorage1D_p: glTextureStorage1D_t,
  glTextureStorage2D_p: glTextureStorage2D_t,
  glTextureStorage2DMultisample_p: glTextureStorage2DMultisample_t,
  glTextureStorage3D_p: glTextureStorage3D_t,
  glTextureStorage3DMultisample_p: glTextureStorage3DMultisample_t,
  glTextureSubImage1D_p: glTextureSubImage1D_t,
  glTextureSubImage2D_p: glTextureSubImage2D_t,
  glTextureSubImage3D_p: glTextureSubImage3D_t,
  glTextureView_p: glTextureView_t,
  glTransformFeedbackBufferBase_p: glTransformFeedbackBufferBase_t,
  glTransformFeedbackBufferRange_p: glTransformFeedbackBufferRange_t,
  glTransformFeedbackVaryings_p: glTransformFeedbackVaryings_t,
  glUniform1d_p: glUniform1d_t,
  glUniform1dv_p: glUniform1dv_t,
  glUniform1f_p: glUniform1f_t,
  glUniform1fv_p: glUniform1fv_t,
  glUniform1i_p: glUniform1i_t,
  glUniform1iv_p: glUniform1iv_t,
  glUniform1ui_p: glUniform1ui_t,
  glUniform1uiv_p: glUniform1uiv_t,
  glUniform2d_p: glUniform2d_t,
  glUniform2dv_p: glUniform2dv_t,
  glUniform2f_p: glUniform2f_t,
  glUniform2fv_p: glUniform2fv_t,
  glUniform2i_p: glUniform2i_t,
  glUniform2iv_p: glUniform2iv_t,
  glUniform2ui_p: glUniform2ui_t,
  glUniform2uiv_p: glUniform2uiv_t,
  glUniform3d_p: glUniform3d_t,
  glUniform3dv_p: glUniform3dv_t,
  glUniform3f_p: glUniform3f_t,
  glUniform3fv_p: glUniform3fv_t,
  glUniform3i_p: glUniform3i_t,
  glUniform3iv_p: glUniform3iv_t,
  glUniform3ui_p: glUniform3ui_t,
  glUniform3uiv_p: glUniform3uiv_t,
  glUniform4d_p: glUniform4d_t,
  glUniform4dv_p: glUniform4dv_t,
  glUniform4f_p: glUniform4f_t,
  glUniform4fv_p: glUniform4fv_t,
  glUniform4i_p: glUniform4i_t,
  glUniform4iv_p: glUniform4iv_t,
  glUniform4ui_p: glUniform4ui_t,
  glUniform4uiv_p: glUniform4uiv_t,
  glUniformBlockBinding_p: glUniformBlockBinding_t,
  glUniformMatrix2dv_p: glUniformMatrix2dv_t,
  glUniformMatrix2fv_p: glUniformMatrix2fv_t,
  glUniformMatrix2x3dv_p: glUniformMatrix2x3dv_t,
  glUniformMatrix2x3fv_p: glUniformMatrix2x3fv_t,
  glUniformMatrix2x4dv_p: glUniformMatrix2x4dv_t,
  glUniformMatrix2x4fv_p: glUniformMatrix2x4fv_t,
  glUniformMatrix3dv_p: glUniformMatrix3dv_t,
  glUniformMatrix3fv_p: glUniformMatrix3fv_t,
  glUniformMatrix3x2dv_p: glUniformMatrix3x2dv_t,
  glUniformMatrix3x2fv_p: glUniformMatrix3x2fv_t,
  glUniformMatrix3x4dv_p: glUniformMatrix3x4dv_t,
  glUniformMatrix3x4fv_p: glUniformMatrix3x4fv_t,
  glUniformMatrix4dv_p: glUniformMatrix4dv_t,
  glUniformMatrix4fv_p: glUniformMatrix4fv_t,
  glUniformMatrix4x2dv_p: glUniformMatrix4x2dv_t,
  glUniformMatrix4x2fv_p: glUniformMatrix4x2fv_t,
  glUniformMatrix4x3dv_p: glUniformMatrix4x3dv_t,
  glUniformMatrix4x3fv_p: glUniformMatrix4x3fv_t,
  glUniformSubroutinesuiv_p: glUniformSubroutinesuiv_t,
  glUnmapBuffer_p: glUnmapBuffer_t,
  glUnmapNamedBuffer_p: glUnmapNamedBuffer_t,
  glUseProgram_p: glUseProgram_t,
  glUseProgramStages_p: glUseProgramStages_t,
  glValidateProgram_p: glValidateProgram_t,
  glValidateProgramPipeline_p: glValidateProgramPipeline_t,
  glVertexArrayAttribBinding_p: glVertexArrayAttribBinding_t,
  glVertexArrayAttribFormat_p: glVertexArrayAttribFormat_t,
  glVertexArrayAttribIFormat_p: glVertexArrayAttribIFormat_t,
  glVertexArrayAttribLFormat_p: glVertexArrayAttribLFormat_t,
  glVertexArrayBindingDivisor_p: glVertexArrayBindingDivisor_t,
  glVertexArrayElementBuffer_p: glVertexArrayElementBuffer_t,
  glVertexArrayVertexBuffer_p: glVertexArrayVertexBuffer_t,
  glVertexArrayVertexBuffers_p: glVertexArrayVertexBuffers_t,
  glVertexAttrib1d_p: glVertexAttrib1d_t,
  glVertexAttrib1dv_p: glVertexAttrib1dv_t,
  glVertexAttrib1f_p: glVertexAttrib1f_t,
  glVertexAttrib1fv_p: glVertexAttrib1fv_t,
  glVertexAttrib1s_p: glVertexAttrib1s_t,
  glVertexAttrib1sv_p: glVertexAttrib1sv_t,
  glVertexAttrib2d_p: glVertexAttrib2d_t,
  glVertexAttrib2dv_p: glVertexAttrib2dv_t,
  glVertexAttrib2f_p: glVertexAttrib2f_t,
  glVertexAttrib2fv_p: glVertexAttrib2fv_t,
  glVertexAttrib2s_p: glVertexAttrib2s_t,
  glVertexAttrib2sv_p: glVertexAttrib2sv_t,
  glVertexAttrib3d_p: glVertexAttrib3d_t,
  glVertexAttrib3dv_p: glVertexAttrib3dv_t,
  glVertexAttrib3f_p: glVertexAttrib3f_t,
  glVertexAttrib3fv_p: glVertexAttrib3fv_t,
  glVertexAttrib3s_p: glVertexAttrib3s_t,
  glVertexAttrib3sv_p: glVertexAttrib3sv_t,
  glVertexAttrib4Nbv_p: glVertexAttrib4Nbv_t,
  glVertexAttrib4Niv_p: glVertexAttrib4Niv_t,
  glVertexAttrib4Nsv_p: glVertexAttrib4Nsv_t,
  glVertexAttrib4Nub_p: glVertexAttrib4Nub_t,
  glVertexAttrib4Nubv_p: glVertexAttrib4Nubv_t,
  glVertexAttrib4Nuiv_p: glVertexAttrib4Nuiv_t,
  glVertexAttrib4Nusv_p: glVertexAttrib4Nusv_t,
  glVertexAttrib4bv_p: glVertexAttrib4bv_t,
  glVertexAttrib4d_p: glVertexAttrib4d_t,
  glVertexAttrib4dv_p: glVertexAttrib4dv_t,
  glVertexAttrib4f_p: glVertexAttrib4f_t,
  glVertexAttrib4fv_p: glVertexAttrib4fv_t,
  glVertexAttrib4iv_p: glVertexAttrib4iv_t,
  glVertexAttrib4s_p: glVertexAttrib4s_t,
  glVertexAttrib4sv_p: glVertexAttrib4sv_t,
  glVertexAttrib4ubv_p: glVertexAttrib4ubv_t,
  glVertexAttrib4uiv_p: glVertexAttrib4uiv_t,
  glVertexAttrib4usv_p: glVertexAttrib4usv_t,
  glVertexAttribBinding_p: glVertexAttribBinding_t,
  glVertexAttribDivisor_p: glVertexAttribDivisor_t,
  glVertexAttribFormat_p: glVertexAttribFormat_t,
  glVertexAttribI1i_p: glVertexAttribI1i_t,
  glVertexAttribI1iv_p: glVertexAttribI1iv_t,
  glVertexAttribI1ui_p: glVertexAttribI1ui_t,
  glVertexAttribI1uiv_p: glVertexAttribI1uiv_t,
  glVertexAttribI2i_p: glVertexAttribI2i_t,
  glVertexAttribI2iv_p: glVertexAttribI2iv_t,
  glVertexAttribI2ui_p: glVertexAttribI2ui_t,
  glVertexAttribI2uiv_p: glVertexAttribI2uiv_t,
  glVertexAttribI3i_p: glVertexAttribI3i_t,
  glVertexAttribI3iv_p: glVertexAttribI3iv_t,
  glVertexAttribI3ui_p: glVertexAttribI3ui_t,
  glVertexAttribI3uiv_p: glVertexAttribI3uiv_t,
  glVertexAttribI4bv_p: glVertexAttribI4bv_t,
  glVertexAttribI4i_p: glVertexAttribI4i_t,
  glVertexAttribI4iv_p: glVertexAttribI4iv_t,
  glVertexAttribI4sv_p: glVertexAttribI4sv_t,
  glVertexAttribI4ubv_p: glVertexAttribI4ubv_t,
  glVertexAttribI4ui_p: glVertexAttribI4ui_t,
  glVertexAttribI4uiv_p: glVertexAttribI4uiv_t,
  glVertexAttribI4usv_p: glVertexAttribI4usv_t,
  glVertexAttribIFormat_p: glVertexAttribIFormat_t,
  glVertexAttribIPointer_p: glVertexAttribIPointer_t,
  glVertexAttribL1d_p: glVertexAttribL1d_t,
  glVertexAttribL1dv_p: glVertexAttribL1dv_t,
  glVertexAttribL2d_p: glVertexAttribL2d_t,
  glVertexAttribL2dv_p: glVertexAttribL2dv_t,
  glVertexAttribL3d_p: glVertexAttribL3d_t,
  glVertexAttribL3dv_p: glVertexAttribL3dv_t,
  glVertexAttribL4d_p: glVertexAttribL4d_t,
  glVertexAttribL4dv_p: glVertexAttribL4dv_t,
  glVertexAttribLFormat_p: glVertexAttribLFormat_t,
  glVertexAttribLPointer_p: glVertexAttribLPointer_t,
  glVertexAttribP1ui_p: glVertexAttribP1ui_t,
  glVertexAttribP1uiv_p: glVertexAttribP1uiv_t,
  glVertexAttribP2ui_p: glVertexAttribP2ui_t,
  glVertexAttribP2uiv_p: glVertexAttribP2uiv_t,
  glVertexAttribP3ui_p: glVertexAttribP3ui_t,
  glVertexAttribP3uiv_p: glVertexAttribP3uiv_t,
  glVertexAttribP4ui_p: glVertexAttribP4ui_t,
  glVertexAttribP4uiv_p: glVertexAttribP4uiv_t,
  glVertexAttribPointer_p: glVertexAttribPointer_t,
  glVertexBindingDivisor_p: glVertexBindingDivisor_t,
  glVertexP2ui_p: glVertexP2ui_t,
  glVertexP2uiv_p: glVertexP2uiv_t,
  glVertexP3ui_p: glVertexP3ui_t,
  glVertexP3uiv_p: glVertexP3uiv_t,
  glVertexP4ui_p: glVertexP4ui_t,
  glVertexP4uiv_p: glVertexP4uiv_t,
  glViewport_p: glViewport_t,
  glViewportArrayv_p: glViewportArrayv_t,
  glViewportIndexedf_p: glViewportIndexedf_t,
  glViewportIndexedfv_p: glViewportIndexedfv_t,
  glWaitSync_p: glWaitSync_t,
  glDebugMessageCallbackARB_p: Option<glDebugMessageCallbackARB_t>,
  glDebugMessageControlARB_p: Option<glDebugMessageControlARB_t>,
  glDebugMessageInsertARB_p: Option<glDebugMessageInsertARB_t>,
  glGetDebugMessageLogARB_p: Option<glGetDebugMessageLogARB_t>,
  glDebugMessageCallbackKHR_p: Option<glDebugMessageCallbackKHR_t>,
  glDebugMessageControlKHR_p: Option<glDebugMessageControlKHR_t>,
  glDebugMessageInsertKHR_p: Option<glDebugMessageInsertKHR_t>,
  glGetDebugMessageLogKHR_p: Option<glGetDebugMessageLogKHR_t>,
  glGetObjectLabelKHR_p: Option<glGetObjectLabelKHR_t>,
  glGetObjectPtrLabelKHR_p: Option<glGetObjectPtrLabelKHR_t>,
  glGetPointervKHR_p: Option<glGetPointervKHR_t>,
  glObjectLabelKHR_p: Option<glObjectLabelKHR_t>,
  glObjectPtrLabelKHR_p: Option<glObjectPtrLabelKHR_t>,
  glPopDebugGroupKHR_p: Option<glPopDebugGroupKHR_t>,
  glPushDebugGroupKHR_p: Option<glPushDebugGroupKHR_t>,
}

impl GlFns46 {
  fn ptr_filter(p: *const c_void) -> Option<core::ptr::NonNull<c_void>> {
    match p as usize {
      // Note(Lokathor): wgl is known to sometimes give phony non-null pointer values.
      0 | 1 | 2 | 3 | usize::MAX => None,
      _ => unsafe { core::mem::transmute(p) },
    }
  }
  #[cold]
  #[inline(never)]
  #[cfg_attr(feature = "track_caller", track_caller)]
  fn not_loaded(name: &str) -> ! {
    panic!("Function Not Loaded: {}", name);
  }

  /// Loads all GL functions from the loader given.
  ///
  /// ## Failure
  /// This fails if any non-nullable function does not load.
  /// The error value will be the name of the first non-nullable function that
  /// doesn't load.
  ///
  /// ## Safety
  /// * The "Get Proc Address" function you provide will always be given a
  ///   pointer to the start of a null-terminated string containing the name of
  ///   a GL function to load.
  /// * The "Get Proc Address" function given must always return accurate
  ///   function pointer values, or null on failure.
  pub unsafe fn load_from<F: Fn(*const u8) -> *const c_void>(f: F) -> Result<Self, &'static str> {
    use core::mem::transmute;
    type nn_cv = core::ptr::NonNull<c_void>;
    // non-nullable loads
    let glActiveShaderProgram_p = transmute::<nn_cv, glActiveShaderProgram_t>(Self::ptr_filter(f(b"glActiveShaderProgram\0".as_ptr())).ok_or("glActiveShaderProgram")?);
    let glActiveTexture_p = transmute::<nn_cv, glActiveTexture_t>(Self::ptr_filter(f(b"glActiveTexture\0".as_ptr())).ok_or("glActiveTexture")?);
    let glAttachShader_p = transmute::<nn_cv, glAttachShader_t>(Self::ptr_filter(f(b"glAttachShader\0".as_ptr())).ok_or("glAttachShader")?);
    let glBeginConditionalRender_p = transmute::<nn_cv, glBeginConditionalRender_t>(Self::ptr_filter(f(b"glBeginConditionalRender\0".as_ptr())).ok_or("glBeginConditionalRender")?);
    let glBeginQuery_p = transmute::<nn_cv, glBeginQuery_t>(Self::ptr_filter(f(b"glBeginQuery\0".as_ptr())).ok_or("glBeginQuery")?);
    let glBeginQueryIndexed_p = transmute::<nn_cv, glBeginQueryIndexed_t>(Self::ptr_filter(f(b"glBeginQueryIndexed\0".as_ptr())).ok_or("glBeginQueryIndexed")?);
    let glBeginTransformFeedback_p = transmute::<nn_cv, glBeginTransformFeedback_t>(Self::ptr_filter(f(b"glBeginTransformFeedback\0".as_ptr())).ok_or("glBeginTransformFeedback")?);
    let glBindAttribLocation_p = transmute::<nn_cv, glBindAttribLocation_t>(Self::ptr_filter(f(b"glBindAttribLocation\0".as_ptr())).ok_or("glBindAttribLocation")?);
    let glBindBuffer_p = transmute::<nn_cv, glBindBuffer_t>(Self::ptr_filter(f(b"glBindBuffer\0".as_ptr())).ok_or("glBindBuffer")?);
    let glBindBufferBase_p = transmute::<nn_cv, glBindBufferBase_t>(Self::ptr_filter(f(b"glBindBufferBase\0".as_ptr())).ok_or("glBindBufferBase")?);
    let glBindBufferRange_p = transmute::<nn_cv, glBindBufferRange_t>(Self::ptr_filter(f(b"glBindBufferRange\0".as_ptr())).ok_or("glBindBufferRange")?);
    let glBindBuffersBase_p = transmute::<nn_cv, glBindBuffersBase_t>(Self::ptr_filter(f(b"glBindBuffersBase\0".as_ptr())).ok_or("glBindBuffersBase")?);
    let glBindBuffersRange_p = transmute::<nn_cv, glBindBuffersRange_t>(Self::ptr_filter(f(b"glBindBuffersRange\0".as_ptr())).ok_or("glBindBuffersRange")?);
    let glBindFragDataLocation_p = transmute::<nn_cv, glBindFragDataLocation_t>(Self::ptr_filter(f(b"glBindFragDataLocation\0".as_ptr())).ok_or("glBindFragDataLocation")?);
    let glBindFragDataLocationIndexed_p = transmute::<nn_cv, glBindFragDataLocationIndexed_t>(Self::ptr_filter(f(b"glBindFragDataLocationIndexed\0".as_ptr())).ok_or("glBindFragDataLocationIndexed")?);
    let glBindFramebuffer_p = transmute::<nn_cv, glBindFramebuffer_t>(Self::ptr_filter(f(b"glBindFramebuffer\0".as_ptr())).ok_or("glBindFramebuffer")?);
    let glBindImageTexture_p = transmute::<nn_cv, glBindImageTexture_t>(Self::ptr_filter(f(b"glBindImageTexture\0".as_ptr())).ok_or("glBindImageTexture")?);
    let glBindImageTextures_p = transmute::<nn_cv, glBindImageTextures_t>(Self::ptr_filter(f(b"glBindImageTextures\0".as_ptr())).ok_or("glBindImageTextures")?);
    let glBindProgramPipeline_p = transmute::<nn_cv, glBindProgramPipeline_t>(Self::ptr_filter(f(b"glBindProgramPipeline\0".as_ptr())).ok_or("glBindProgramPipeline")?);
    let glBindRenderbuffer_p = transmute::<nn_cv, glBindRenderbuffer_t>(Self::ptr_filter(f(b"glBindRenderbuffer\0".as_ptr())).ok_or("glBindRenderbuffer")?);
    let glBindSampler_p = transmute::<nn_cv, glBindSampler_t>(Self::ptr_filter(f(b"glBindSampler\0".as_ptr())).ok_or("glBindSampler")?);
    let glBindSamplers_p = transmute::<nn_cv, glBindSamplers_t>(Self::ptr_filter(f(b"glBindSamplers\0".as_ptr())).ok_or("glBindSamplers")?);
    let glBindTexture_p = transmute::<nn_cv, glBindTexture_t>(Self::ptr_filter(f(b"glBindTexture\0".as_ptr())).ok_or("glBindTexture")?);
    let glBindTextureUnit_p = transmute::<nn_cv, glBindTextureUnit_t>(Self::ptr_filter(f(b"glBindTextureUnit\0".as_ptr())).ok_or("glBindTextureUnit")?);
    let glBindTextures_p = transmute::<nn_cv, glBindTextures_t>(Self::ptr_filter(f(b"glBindTextures\0".as_ptr())).ok_or("glBindTextures")?);
    let glBindTransformFeedback_p = transmute::<nn_cv, glBindTransformFeedback_t>(Self::ptr_filter(f(b"glBindTransformFeedback\0".as_ptr())).ok_or("glBindTransformFeedback")?);
    let glBindVertexArray_p = transmute::<nn_cv, glBindVertexArray_t>(Self::ptr_filter(f(b"glBindVertexArray\0".as_ptr())).ok_or("glBindVertexArray")?);
    let glBindVertexBuffer_p = transmute::<nn_cv, glBindVertexBuffer_t>(Self::ptr_filter(f(b"glBindVertexBuffer\0".as_ptr())).ok_or("glBindVertexBuffer")?);
    let glBindVertexBuffers_p = transmute::<nn_cv, glBindVertexBuffers_t>(Self::ptr_filter(f(b"glBindVertexBuffers\0".as_ptr())).ok_or("glBindVertexBuffers")?);
    let glBlendColor_p = transmute::<nn_cv, glBlendColor_t>(Self::ptr_filter(f(b"glBlendColor\0".as_ptr())).ok_or("glBlendColor")?);
    let glBlendEquation_p = transmute::<nn_cv, glBlendEquation_t>(Self::ptr_filter(f(b"glBlendEquation\0".as_ptr())).ok_or("glBlendEquation")?);
    let glBlendEquationSeparate_p = transmute::<nn_cv, glBlendEquationSeparate_t>(Self::ptr_filter(f(b"glBlendEquationSeparate\0".as_ptr())).ok_or("glBlendEquationSeparate")?);
    let glBlendEquationSeparatei_p = transmute::<nn_cv, glBlendEquationSeparatei_t>(Self::ptr_filter(f(b"glBlendEquationSeparatei\0".as_ptr())).ok_or("glBlendEquationSeparatei")?);
    let glBlendEquationi_p = transmute::<nn_cv, glBlendEquationi_t>(Self::ptr_filter(f(b"glBlendEquationi\0".as_ptr())).ok_or("glBlendEquationi")?);
    let glBlendFunc_p = transmute::<nn_cv, glBlendFunc_t>(Self::ptr_filter(f(b"glBlendFunc\0".as_ptr())).ok_or("glBlendFunc")?);
    let glBlendFuncSeparate_p = transmute::<nn_cv, glBlendFuncSeparate_t>(Self::ptr_filter(f(b"glBlendFuncSeparate\0".as_ptr())).ok_or("glBlendFuncSeparate")?);
    let glBlendFuncSeparatei_p = transmute::<nn_cv, glBlendFuncSeparatei_t>(Self::ptr_filter(f(b"glBlendFuncSeparatei\0".as_ptr())).ok_or("glBlendFuncSeparatei")?);
    let glBlendFunci_p = transmute::<nn_cv, glBlendFunci_t>(Self::ptr_filter(f(b"glBlendFunci\0".as_ptr())).ok_or("glBlendFunci")?);
    let glBlitFramebuffer_p = transmute::<nn_cv, glBlitFramebuffer_t>(Self::ptr_filter(f(b"glBlitFramebuffer\0".as_ptr())).ok_or("glBlitFramebuffer")?);
    let glBlitNamedFramebuffer_p = transmute::<nn_cv, glBlitNamedFramebuffer_t>(Self::ptr_filter(f(b"glBlitNamedFramebuffer\0".as_ptr())).ok_or("glBlitNamedFramebuffer")?);
    let glBufferData_p = transmute::<nn_cv, glBufferData_t>(Self::ptr_filter(f(b"glBufferData\0".as_ptr())).ok_or("glBufferData")?);
    let glBufferStorage_p = transmute::<nn_cv, glBufferStorage_t>(Self::ptr_filter(f(b"glBufferStorage\0".as_ptr())).ok_or("glBufferStorage")?);
    let glBufferSubData_p = transmute::<nn_cv, glBufferSubData_t>(Self::ptr_filter(f(b"glBufferSubData\0".as_ptr())).ok_or("glBufferSubData")?);
    let glCheckFramebufferStatus_p = transmute::<nn_cv, glCheckFramebufferStatus_t>(Self::ptr_filter(f(b"glCheckFramebufferStatus\0".as_ptr())).ok_or("glCheckFramebufferStatus")?);
    let glCheckNamedFramebufferStatus_p = transmute::<nn_cv, glCheckNamedFramebufferStatus_t>(Self::ptr_filter(f(b"glCheckNamedFramebufferStatus\0".as_ptr())).ok_or("glCheckNamedFramebufferStatus")?);
    let glClampColor_p = transmute::<nn_cv, glClampColor_t>(Self::ptr_filter(f(b"glClampColor\0".as_ptr())).ok_or("glClampColor")?);
    let glClear_p = transmute::<nn_cv, glClear_t>(Self::ptr_filter(f(b"glClear\0".as_ptr())).ok_or("glClear")?);
    let glClearBufferData_p = transmute::<nn_cv, glClearBufferData_t>(Self::ptr_filter(f(b"glClearBufferData\0".as_ptr())).ok_or("glClearBufferData")?);
    let glClearBufferSubData_p = transmute::<nn_cv, glClearBufferSubData_t>(Self::ptr_filter(f(b"glClearBufferSubData\0".as_ptr())).ok_or("glClearBufferSubData")?);
    let glClearBufferfi_p = transmute::<nn_cv, glClearBufferfi_t>(Self::ptr_filter(f(b"glClearBufferfi\0".as_ptr())).ok_or("glClearBufferfi")?);
    let glClearBufferfv_p = transmute::<nn_cv, glClearBufferfv_t>(Self::ptr_filter(f(b"glClearBufferfv\0".as_ptr())).ok_or("glClearBufferfv")?);
    let glClearBufferiv_p = transmute::<nn_cv, glClearBufferiv_t>(Self::ptr_filter(f(b"glClearBufferiv\0".as_ptr())).ok_or("glClearBufferiv")?);
    let glClearBufferuiv_p = transmute::<nn_cv, glClearBufferuiv_t>(Self::ptr_filter(f(b"glClearBufferuiv\0".as_ptr())).ok_or("glClearBufferuiv")?);
    let glClearColor_p = transmute::<nn_cv, glClearColor_t>(Self::ptr_filter(f(b"glClearColor\0".as_ptr())).ok_or("glClearColor")?);
    let glClearDepth_p = transmute::<nn_cv, glClearDepth_t>(Self::ptr_filter(f(b"glClearDepth\0".as_ptr())).ok_or("glClearDepth")?);
    let glClearDepthf_p = transmute::<nn_cv, glClearDepthf_t>(Self::ptr_filter(f(b"glClearDepthf\0".as_ptr())).ok_or("glClearDepthf")?);
    let glClearNamedBufferData_p = transmute::<nn_cv, glClearNamedBufferData_t>(Self::ptr_filter(f(b"glClearNamedBufferData\0".as_ptr())).ok_or("glClearNamedBufferData")?);
    let glClearNamedBufferSubData_p = transmute::<nn_cv, glClearNamedBufferSubData_t>(Self::ptr_filter(f(b"glClearNamedBufferSubData\0".as_ptr())).ok_or("glClearNamedBufferSubData")?);
    let glClearNamedFramebufferfi_p = transmute::<nn_cv, glClearNamedFramebufferfi_t>(Self::ptr_filter(f(b"glClearNamedFramebufferfi\0".as_ptr())).ok_or("glClearNamedFramebufferfi")?);
    let glClearNamedFramebufferfv_p = transmute::<nn_cv, glClearNamedFramebufferfv_t>(Self::ptr_filter(f(b"glClearNamedFramebufferfv\0".as_ptr())).ok_or("glClearNamedFramebufferfv")?);
    let glClearNamedFramebufferiv_p = transmute::<nn_cv, glClearNamedFramebufferiv_t>(Self::ptr_filter(f(b"glClearNamedFramebufferiv\0".as_ptr())).ok_or("glClearNamedFramebufferiv")?);
    let glClearNamedFramebufferuiv_p = transmute::<nn_cv, glClearNamedFramebufferuiv_t>(Self::ptr_filter(f(b"glClearNamedFramebufferuiv\0".as_ptr())).ok_or("glClearNamedFramebufferuiv")?);
    let glClearStencil_p = transmute::<nn_cv, glClearStencil_t>(Self::ptr_filter(f(b"glClearStencil\0".as_ptr())).ok_or("glClearStencil")?);
    let glClearTexImage_p = transmute::<nn_cv, glClearTexImage_t>(Self::ptr_filter(f(b"glClearTexImage\0".as_ptr())).ok_or("glClearTexImage")?);
    let glClearTexSubImage_p = transmute::<nn_cv, glClearTexSubImage_t>(Self::ptr_filter(f(b"glClearTexSubImage\0".as_ptr())).ok_or("glClearTexSubImage")?);
    let glClientWaitSync_p = transmute::<nn_cv, glClientWaitSync_t>(Self::ptr_filter(f(b"glClientWaitSync\0".as_ptr())).ok_or("glClientWaitSync")?);
    let glClipControl_p = transmute::<nn_cv, glClipControl_t>(Self::ptr_filter(f(b"glClipControl\0".as_ptr())).ok_or("glClipControl")?);
    let glColorMask_p = transmute::<nn_cv, glColorMask_t>(Self::ptr_filter(f(b"glColorMask\0".as_ptr())).ok_or("glColorMask")?);
    let glColorMaski_p = transmute::<nn_cv, glColorMaski_t>(Self::ptr_filter(f(b"glColorMaski\0".as_ptr())).ok_or("glColorMaski")?);
    let glColorP3ui_p = transmute::<nn_cv, glColorP3ui_t>(Self::ptr_filter(f(b"glColorP3ui\0".as_ptr())).ok_or("glColorP3ui")?);
    let glColorP3uiv_p = transmute::<nn_cv, glColorP3uiv_t>(Self::ptr_filter(f(b"glColorP3uiv\0".as_ptr())).ok_or("glColorP3uiv")?);
    let glColorP4ui_p = transmute::<nn_cv, glColorP4ui_t>(Self::ptr_filter(f(b"glColorP4ui\0".as_ptr())).ok_or("glColorP4ui")?);
    let glColorP4uiv_p = transmute::<nn_cv, glColorP4uiv_t>(Self::ptr_filter(f(b"glColorP4uiv\0".as_ptr())).ok_or("glColorP4uiv")?);
    let glCompileShader_p = transmute::<nn_cv, glCompileShader_t>(Self::ptr_filter(f(b"glCompileShader\0".as_ptr())).ok_or("glCompileShader")?);
    let glCompressedTexImage1D_p = transmute::<nn_cv, glCompressedTexImage1D_t>(Self::ptr_filter(f(b"glCompressedTexImage1D\0".as_ptr())).ok_or("glCompressedTexImage1D")?);
    let glCompressedTexImage2D_p = transmute::<nn_cv, glCompressedTexImage2D_t>(Self::ptr_filter(f(b"glCompressedTexImage2D\0".as_ptr())).ok_or("glCompressedTexImage2D")?);
    let glCompressedTexImage3D_p = transmute::<nn_cv, glCompressedTexImage3D_t>(Self::ptr_filter(f(b"glCompressedTexImage3D\0".as_ptr())).ok_or("glCompressedTexImage3D")?);
    let glCompressedTexSubImage1D_p = transmute::<nn_cv, glCompressedTexSubImage1D_t>(Self::ptr_filter(f(b"glCompressedTexSubImage1D\0".as_ptr())).ok_or("glCompressedTexSubImage1D")?);
    let glCompressedTexSubImage2D_p = transmute::<nn_cv, glCompressedTexSubImage2D_t>(Self::ptr_filter(f(b"glCompressedTexSubImage2D\0".as_ptr())).ok_or("glCompressedTexSubImage2D")?);
    let glCompressedTexSubImage3D_p = transmute::<nn_cv, glCompressedTexSubImage3D_t>(Self::ptr_filter(f(b"glCompressedTexSubImage3D\0".as_ptr())).ok_or("glCompressedTexSubImage3D")?);
    let glCompressedTextureSubImage1D_p = transmute::<nn_cv, glCompressedTextureSubImage1D_t>(Self::ptr_filter(f(b"glCompressedTextureSubImage1D\0".as_ptr())).ok_or("glCompressedTextureSubImage1D")?);
    let glCompressedTextureSubImage2D_p = transmute::<nn_cv, glCompressedTextureSubImage2D_t>(Self::ptr_filter(f(b"glCompressedTextureSubImage2D\0".as_ptr())).ok_or("glCompressedTextureSubImage2D")?);
    let glCompressedTextureSubImage3D_p = transmute::<nn_cv, glCompressedTextureSubImage3D_t>(Self::ptr_filter(f(b"glCompressedTextureSubImage3D\0".as_ptr())).ok_or("glCompressedTextureSubImage3D")?);
    let glCopyBufferSubData_p = transmute::<nn_cv, glCopyBufferSubData_t>(Self::ptr_filter(f(b"glCopyBufferSubData\0".as_ptr())).ok_or("glCopyBufferSubData")?);
    let glCopyImageSubData_p = transmute::<nn_cv, glCopyImageSubData_t>(Self::ptr_filter(f(b"glCopyImageSubData\0".as_ptr())).ok_or("glCopyImageSubData")?);
    let glCopyNamedBufferSubData_p = transmute::<nn_cv, glCopyNamedBufferSubData_t>(Self::ptr_filter(f(b"glCopyNamedBufferSubData\0".as_ptr())).ok_or("glCopyNamedBufferSubData")?);
    let glCopyTexImage1D_p = transmute::<nn_cv, glCopyTexImage1D_t>(Self::ptr_filter(f(b"glCopyTexImage1D\0".as_ptr())).ok_or("glCopyTexImage1D")?);
    let glCopyTexImage2D_p = transmute::<nn_cv, glCopyTexImage2D_t>(Self::ptr_filter(f(b"glCopyTexImage2D\0".as_ptr())).ok_or("glCopyTexImage2D")?);
    let glCopyTexSubImage1D_p = transmute::<nn_cv, glCopyTexSubImage1D_t>(Self::ptr_filter(f(b"glCopyTexSubImage1D\0".as_ptr())).ok_or("glCopyTexSubImage1D")?);
    let glCopyTexSubImage2D_p = transmute::<nn_cv, glCopyTexSubImage2D_t>(Self::ptr_filter(f(b"glCopyTexSubImage2D\0".as_ptr())).ok_or("glCopyTexSubImage2D")?);
    let glCopyTexSubImage3D_p = transmute::<nn_cv, glCopyTexSubImage3D_t>(Self::ptr_filter(f(b"glCopyTexSubImage3D\0".as_ptr())).ok_or("glCopyTexSubImage3D")?);
    let glCopyTextureSubImage1D_p = transmute::<nn_cv, glCopyTextureSubImage1D_t>(Self::ptr_filter(f(b"glCopyTextureSubImage1D\0".as_ptr())).ok_or("glCopyTextureSubImage1D")?);
    let glCopyTextureSubImage2D_p = transmute::<nn_cv, glCopyTextureSubImage2D_t>(Self::ptr_filter(f(b"glCopyTextureSubImage2D\0".as_ptr())).ok_or("glCopyTextureSubImage2D")?);
    let glCopyTextureSubImage3D_p = transmute::<nn_cv, glCopyTextureSubImage3D_t>(Self::ptr_filter(f(b"glCopyTextureSubImage3D\0".as_ptr())).ok_or("glCopyTextureSubImage3D")?);
    let glCreateBuffers_p = transmute::<nn_cv, glCreateBuffers_t>(Self::ptr_filter(f(b"glCreateBuffers\0".as_ptr())).ok_or("glCreateBuffers")?);
    let glCreateFramebuffers_p = transmute::<nn_cv, glCreateFramebuffers_t>(Self::ptr_filter(f(b"glCreateFramebuffers\0".as_ptr())).ok_or("glCreateFramebuffers")?);
    let glCreateProgram_p = transmute::<nn_cv, glCreateProgram_t>(Self::ptr_filter(f(b"glCreateProgram\0".as_ptr())).ok_or("glCreateProgram")?);
    let glCreateProgramPipelines_p = transmute::<nn_cv, glCreateProgramPipelines_t>(Self::ptr_filter(f(b"glCreateProgramPipelines\0".as_ptr())).ok_or("glCreateProgramPipelines")?);
    let glCreateQueries_p = transmute::<nn_cv, glCreateQueries_t>(Self::ptr_filter(f(b"glCreateQueries\0".as_ptr())).ok_or("glCreateQueries")?);
    let glCreateRenderbuffers_p = transmute::<nn_cv, glCreateRenderbuffers_t>(Self::ptr_filter(f(b"glCreateRenderbuffers\0".as_ptr())).ok_or("glCreateRenderbuffers")?);
    let glCreateSamplers_p = transmute::<nn_cv, glCreateSamplers_t>(Self::ptr_filter(f(b"glCreateSamplers\0".as_ptr())).ok_or("glCreateSamplers")?);
    let glCreateShader_p = transmute::<nn_cv, glCreateShader_t>(Self::ptr_filter(f(b"glCreateShader\0".as_ptr())).ok_or("glCreateShader")?);
    let glCreateShaderProgramv_p = transmute::<nn_cv, glCreateShaderProgramv_t>(Self::ptr_filter(f(b"glCreateShaderProgramv\0".as_ptr())).ok_or("glCreateShaderProgramv")?);
    let glCreateTextures_p = transmute::<nn_cv, glCreateTextures_t>(Self::ptr_filter(f(b"glCreateTextures\0".as_ptr())).ok_or("glCreateTextures")?);
    let glCreateTransformFeedbacks_p = transmute::<nn_cv, glCreateTransformFeedbacks_t>(Self::ptr_filter(f(b"glCreateTransformFeedbacks\0".as_ptr())).ok_or("glCreateTransformFeedbacks")?);
    let glCreateVertexArrays_p = transmute::<nn_cv, glCreateVertexArrays_t>(Self::ptr_filter(f(b"glCreateVertexArrays\0".as_ptr())).ok_or("glCreateVertexArrays")?);
    let glCullFace_p = transmute::<nn_cv, glCullFace_t>(Self::ptr_filter(f(b"glCullFace\0".as_ptr())).ok_or("glCullFace")?);
    let glDebugMessageCallback_p = transmute::<nn_cv, glDebugMessageCallback_t>(Self::ptr_filter(f(b"glDebugMessageCallback\0".as_ptr())).ok_or("glDebugMessageCallback")?);
    let glDebugMessageControl_p = transmute::<nn_cv, glDebugMessageControl_t>(Self::ptr_filter(f(b"glDebugMessageControl\0".as_ptr())).ok_or("glDebugMessageControl")?);
    let glDebugMessageInsert_p = transmute::<nn_cv, glDebugMessageInsert_t>(Self::ptr_filter(f(b"glDebugMessageInsert\0".as_ptr())).ok_or("glDebugMessageInsert")?);
    let glDeleteBuffers_p = transmute::<nn_cv, glDeleteBuffers_t>(Self::ptr_filter(f(b"glDeleteBuffers\0".as_ptr())).ok_or("glDeleteBuffers")?);
    let glDeleteFramebuffers_p = transmute::<nn_cv, glDeleteFramebuffers_t>(Self::ptr_filter(f(b"glDeleteFramebuffers\0".as_ptr())).ok_or("glDeleteFramebuffers")?);
    let glDeleteProgram_p = transmute::<nn_cv, glDeleteProgram_t>(Self::ptr_filter(f(b"glDeleteProgram\0".as_ptr())).ok_or("glDeleteProgram")?);
    let glDeleteProgramPipelines_p = transmute::<nn_cv, glDeleteProgramPipelines_t>(Self::ptr_filter(f(b"glDeleteProgramPipelines\0".as_ptr())).ok_or("glDeleteProgramPipelines")?);
    let glDeleteQueries_p = transmute::<nn_cv, glDeleteQueries_t>(Self::ptr_filter(f(b"glDeleteQueries\0".as_ptr())).ok_or("glDeleteQueries")?);
    let glDeleteRenderbuffers_p = transmute::<nn_cv, glDeleteRenderbuffers_t>(Self::ptr_filter(f(b"glDeleteRenderbuffers\0".as_ptr())).ok_or("glDeleteRenderbuffers")?);
    let glDeleteSamplers_p = transmute::<nn_cv, glDeleteSamplers_t>(Self::ptr_filter(f(b"glDeleteSamplers\0".as_ptr())).ok_or("glDeleteSamplers")?);
    let glDeleteShader_p = transmute::<nn_cv, glDeleteShader_t>(Self::ptr_filter(f(b"glDeleteShader\0".as_ptr())).ok_or("glDeleteShader")?);
    let glDeleteSync_p = transmute::<nn_cv, glDeleteSync_t>(Self::ptr_filter(f(b"glDeleteSync\0".as_ptr())).ok_or("glDeleteSync")?);
    let glDeleteTextures_p = transmute::<nn_cv, glDeleteTextures_t>(Self::ptr_filter(f(b"glDeleteTextures\0".as_ptr())).ok_or("glDeleteTextures")?);
    let glDeleteTransformFeedbacks_p = transmute::<nn_cv, glDeleteTransformFeedbacks_t>(Self::ptr_filter(f(b"glDeleteTransformFeedbacks\0".as_ptr())).ok_or("glDeleteTransformFeedbacks")?);
    let glDeleteVertexArrays_p = transmute::<nn_cv, glDeleteVertexArrays_t>(Self::ptr_filter(f(b"glDeleteVertexArrays\0".as_ptr())).ok_or("glDeleteVertexArrays")?);
    let glDepthFunc_p = transmute::<nn_cv, glDepthFunc_t>(Self::ptr_filter(f(b"glDepthFunc\0".as_ptr())).ok_or("glDepthFunc")?);
    let glDepthMask_p = transmute::<nn_cv, glDepthMask_t>(Self::ptr_filter(f(b"glDepthMask\0".as_ptr())).ok_or("glDepthMask")?);
    let glDepthRange_p = transmute::<nn_cv, glDepthRange_t>(Self::ptr_filter(f(b"glDepthRange\0".as_ptr())).ok_or("glDepthRange")?);
    let glDepthRangeArrayv_p = transmute::<nn_cv, glDepthRangeArrayv_t>(Self::ptr_filter(f(b"glDepthRangeArrayv\0".as_ptr())).ok_or("glDepthRangeArrayv")?);
    let glDepthRangeIndexed_p = transmute::<nn_cv, glDepthRangeIndexed_t>(Self::ptr_filter(f(b"glDepthRangeIndexed\0".as_ptr())).ok_or("glDepthRangeIndexed")?);
    let glDepthRangef_p = transmute::<nn_cv, glDepthRangef_t>(Self::ptr_filter(f(b"glDepthRangef\0".as_ptr())).ok_or("glDepthRangef")?);
    let glDetachShader_p = transmute::<nn_cv, glDetachShader_t>(Self::ptr_filter(f(b"glDetachShader\0".as_ptr())).ok_or("glDetachShader")?);
    let glDisable_p = transmute::<nn_cv, glDisable_t>(Self::ptr_filter(f(b"glDisable\0".as_ptr())).ok_or("glDisable")?);
    let glDisableVertexArrayAttrib_p = transmute::<nn_cv, glDisableVertexArrayAttrib_t>(Self::ptr_filter(f(b"glDisableVertexArrayAttrib\0".as_ptr())).ok_or("glDisableVertexArrayAttrib")?);
    let glDisableVertexAttribArray_p = transmute::<nn_cv, glDisableVertexAttribArray_t>(Self::ptr_filter(f(b"glDisableVertexAttribArray\0".as_ptr())).ok_or("glDisableVertexAttribArray")?);
    let glDisablei_p = transmute::<nn_cv, glDisablei_t>(Self::ptr_filter(f(b"glDisablei\0".as_ptr())).ok_or("glDisablei")?);
    let glDispatchCompute_p = transmute::<nn_cv, glDispatchCompute_t>(Self::ptr_filter(f(b"glDispatchCompute\0".as_ptr())).ok_or("glDispatchCompute")?);
    let glDispatchComputeIndirect_p = transmute::<nn_cv, glDispatchComputeIndirect_t>(Self::ptr_filter(f(b"glDispatchComputeIndirect\0".as_ptr())).ok_or("glDispatchComputeIndirect")?);
    let glDrawArrays_p = transmute::<nn_cv, glDrawArrays_t>(Self::ptr_filter(f(b"glDrawArrays\0".as_ptr())).ok_or("glDrawArrays")?);
    let glDrawArraysIndirect_p = transmute::<nn_cv, glDrawArraysIndirect_t>(Self::ptr_filter(f(b"glDrawArraysIndirect\0".as_ptr())).ok_or("glDrawArraysIndirect")?);
    let glDrawArraysInstanced_p = transmute::<nn_cv, glDrawArraysInstanced_t>(Self::ptr_filter(f(b"glDrawArraysInstanced\0".as_ptr())).ok_or("glDrawArraysInstanced")?);
    let glDrawArraysInstancedBaseInstance_p = transmute::<nn_cv, glDrawArraysInstancedBaseInstance_t>(Self::ptr_filter(f(b"glDrawArraysInstancedBaseInstance\0".as_ptr())).ok_or("glDrawArraysInstancedBaseInstance")?);
    let glDrawBuffer_p = transmute::<nn_cv, glDrawBuffer_t>(Self::ptr_filter(f(b"glDrawBuffer\0".as_ptr())).ok_or("glDrawBuffer")?);
    let glDrawBuffers_p = transmute::<nn_cv, glDrawBuffers_t>(Self::ptr_filter(f(b"glDrawBuffers\0".as_ptr())).ok_or("glDrawBuffers")?);
    let glDrawElements_p = transmute::<nn_cv, glDrawElements_t>(Self::ptr_filter(f(b"glDrawElements\0".as_ptr())).ok_or("glDrawElements")?);
    let glDrawElementsBaseVertex_p = transmute::<nn_cv, glDrawElementsBaseVertex_t>(Self::ptr_filter(f(b"glDrawElementsBaseVertex\0".as_ptr())).ok_or("glDrawElementsBaseVertex")?);
    let glDrawElementsIndirect_p = transmute::<nn_cv, glDrawElementsIndirect_t>(Self::ptr_filter(f(b"glDrawElementsIndirect\0".as_ptr())).ok_or("glDrawElementsIndirect")?);
    let glDrawElementsInstanced_p = transmute::<nn_cv, glDrawElementsInstanced_t>(Self::ptr_filter(f(b"glDrawElementsInstanced\0".as_ptr())).ok_or("glDrawElementsInstanced")?);
    let glDrawElementsInstancedBaseInstance_p = transmute::<nn_cv, glDrawElementsInstancedBaseInstance_t>(Self::ptr_filter(f(b"glDrawElementsInstancedBaseInstance\0".as_ptr())).ok_or("glDrawElementsInstancedBaseInstance")?);
    let glDrawElementsInstancedBaseVertex_p = transmute::<nn_cv, glDrawElementsInstancedBaseVertex_t>(Self::ptr_filter(f(b"glDrawElementsInstancedBaseVertex\0".as_ptr())).ok_or("glDrawElementsInstancedBaseVertex")?);
    let glDrawElementsInstancedBaseVertexBaseInstance_p = transmute::<nn_cv, glDrawElementsInstancedBaseVertexBaseInstance_t>(Self::ptr_filter(f(b"glDrawElementsInstancedBaseVertexBaseInstance\0".as_ptr())).ok_or("glDrawElementsInstancedBaseVertexBaseInstance")?);
    let glDrawRangeElements_p = transmute::<nn_cv, glDrawRangeElements_t>(Self::ptr_filter(f(b"glDrawRangeElements\0".as_ptr())).ok_or("glDrawRangeElements")?);
    let glDrawRangeElementsBaseVertex_p = transmute::<nn_cv, glDrawRangeElementsBaseVertex_t>(Self::ptr_filter(f(b"glDrawRangeElementsBaseVertex\0".as_ptr())).ok_or("glDrawRangeElementsBaseVertex")?);
    let glDrawTransformFeedback_p = transmute::<nn_cv, glDrawTransformFeedback_t>(Self::ptr_filter(f(b"glDrawTransformFeedback\0".as_ptr())).ok_or("glDrawTransformFeedback")?);
    let glDrawTransformFeedbackInstanced_p = transmute::<nn_cv, glDrawTransformFeedbackInstanced_t>(Self::ptr_filter(f(b"glDrawTransformFeedbackInstanced\0".as_ptr())).ok_or("glDrawTransformFeedbackInstanced")?);
    let glDrawTransformFeedbackStream_p = transmute::<nn_cv, glDrawTransformFeedbackStream_t>(Self::ptr_filter(f(b"glDrawTransformFeedbackStream\0".as_ptr())).ok_or("glDrawTransformFeedbackStream")?);
    let glDrawTransformFeedbackStreamInstanced_p = transmute::<nn_cv, glDrawTransformFeedbackStreamInstanced_t>(Self::ptr_filter(f(b"glDrawTransformFeedbackStreamInstanced\0".as_ptr())).ok_or("glDrawTransformFeedbackStreamInstanced")?);
    let glEnable_p = transmute::<nn_cv, glEnable_t>(Self::ptr_filter(f(b"glEnable\0".as_ptr())).ok_or("glEnable")?);
    let glEnableVertexArrayAttrib_p = transmute::<nn_cv, glEnableVertexArrayAttrib_t>(Self::ptr_filter(f(b"glEnableVertexArrayAttrib\0".as_ptr())).ok_or("glEnableVertexArrayAttrib")?);
    let glEnableVertexAttribArray_p = transmute::<nn_cv, glEnableVertexAttribArray_t>(Self::ptr_filter(f(b"glEnableVertexAttribArray\0".as_ptr())).ok_or("glEnableVertexAttribArray")?);
    let glEnablei_p = transmute::<nn_cv, glEnablei_t>(Self::ptr_filter(f(b"glEnablei\0".as_ptr())).ok_or("glEnablei")?);
    let glEndConditionalRender_p = transmute::<nn_cv, glEndConditionalRender_t>(Self::ptr_filter(f(b"glEndConditionalRender\0".as_ptr())).ok_or("glEndConditionalRender")?);
    let glEndQuery_p = transmute::<nn_cv, glEndQuery_t>(Self::ptr_filter(f(b"glEndQuery\0".as_ptr())).ok_or("glEndQuery")?);
    let glEndQueryIndexed_p = transmute::<nn_cv, glEndQueryIndexed_t>(Self::ptr_filter(f(b"glEndQueryIndexed\0".as_ptr())).ok_or("glEndQueryIndexed")?);
    let glEndTransformFeedback_p = transmute::<nn_cv, glEndTransformFeedback_t>(Self::ptr_filter(f(b"glEndTransformFeedback\0".as_ptr())).ok_or("glEndTransformFeedback")?);
    let glFenceSync_p = transmute::<nn_cv, glFenceSync_t>(Self::ptr_filter(f(b"glFenceSync\0".as_ptr())).ok_or("glFenceSync")?);
    let glFinish_p = transmute::<nn_cv, glFinish_t>(Self::ptr_filter(f(b"glFinish\0".as_ptr())).ok_or("glFinish")?);
    let glFlush_p = transmute::<nn_cv, glFlush_t>(Self::ptr_filter(f(b"glFlush\0".as_ptr())).ok_or("glFlush")?);
    let glFlushMappedBufferRange_p = transmute::<nn_cv, glFlushMappedBufferRange_t>(Self::ptr_filter(f(b"glFlushMappedBufferRange\0".as_ptr())).ok_or("glFlushMappedBufferRange")?);
    let glFlushMappedNamedBufferRange_p = transmute::<nn_cv, glFlushMappedNamedBufferRange_t>(Self::ptr_filter(f(b"glFlushMappedNamedBufferRange\0".as_ptr())).ok_or("glFlushMappedNamedBufferRange")?);
    let glFramebufferParameteri_p = transmute::<nn_cv, glFramebufferParameteri_t>(Self::ptr_filter(f(b"glFramebufferParameteri\0".as_ptr())).ok_or("glFramebufferParameteri")?);
    let glFramebufferRenderbuffer_p = transmute::<nn_cv, glFramebufferRenderbuffer_t>(Self::ptr_filter(f(b"glFramebufferRenderbuffer\0".as_ptr())).ok_or("glFramebufferRenderbuffer")?);
    let glFramebufferTexture_p = transmute::<nn_cv, glFramebufferTexture_t>(Self::ptr_filter(f(b"glFramebufferTexture\0".as_ptr())).ok_or("glFramebufferTexture")?);
    let glFramebufferTexture1D_p = transmute::<nn_cv, glFramebufferTexture1D_t>(Self::ptr_filter(f(b"glFramebufferTexture1D\0".as_ptr())).ok_or("glFramebufferTexture1D")?);
    let glFramebufferTexture2D_p = transmute::<nn_cv, glFramebufferTexture2D_t>(Self::ptr_filter(f(b"glFramebufferTexture2D\0".as_ptr())).ok_or("glFramebufferTexture2D")?);
    let glFramebufferTexture3D_p = transmute::<nn_cv, glFramebufferTexture3D_t>(Self::ptr_filter(f(b"glFramebufferTexture3D\0".as_ptr())).ok_or("glFramebufferTexture3D")?);
    let glFramebufferTextureLayer_p = transmute::<nn_cv, glFramebufferTextureLayer_t>(Self::ptr_filter(f(b"glFramebufferTextureLayer\0".as_ptr())).ok_or("glFramebufferTextureLayer")?);
    let glFrontFace_p = transmute::<nn_cv, glFrontFace_t>(Self::ptr_filter(f(b"glFrontFace\0".as_ptr())).ok_or("glFrontFace")?);
    let glGenBuffers_p = transmute::<nn_cv, glGenBuffers_t>(Self::ptr_filter(f(b"glGenBuffers\0".as_ptr())).ok_or("glGenBuffers")?);
    let glGenFramebuffers_p = transmute::<nn_cv, glGenFramebuffers_t>(Self::ptr_filter(f(b"glGenFramebuffers\0".as_ptr())).ok_or("glGenFramebuffers")?);
    let glGenProgramPipelines_p = transmute::<nn_cv, glGenProgramPipelines_t>(Self::ptr_filter(f(b"glGenProgramPipelines\0".as_ptr())).ok_or("glGenProgramPipelines")?);
    let glGenQueries_p = transmute::<nn_cv, glGenQueries_t>(Self::ptr_filter(f(b"glGenQueries\0".as_ptr())).ok_or("glGenQueries")?);
    let glGenRenderbuffers_p = transmute::<nn_cv, glGenRenderbuffers_t>(Self::ptr_filter(f(b"glGenRenderbuffers\0".as_ptr())).ok_or("glGenRenderbuffers")?);
    let glGenSamplers_p = transmute::<nn_cv, glGenSamplers_t>(Self::ptr_filter(f(b"glGenSamplers\0".as_ptr())).ok_or("glGenSamplers")?);
    let glGenTextures_p = transmute::<nn_cv, glGenTextures_t>(Self::ptr_filter(f(b"glGenTextures\0".as_ptr())).ok_or("glGenTextures")?);
    let glGenTransformFeedbacks_p = transmute::<nn_cv, glGenTransformFeedbacks_t>(Self::ptr_filter(f(b"glGenTransformFeedbacks\0".as_ptr())).ok_or("glGenTransformFeedbacks")?);
    let glGenVertexArrays_p = transmute::<nn_cv, glGenVertexArrays_t>(Self::ptr_filter(f(b"glGenVertexArrays\0".as_ptr())).ok_or("glGenVertexArrays")?);
    let glGenerateMipmap_p = transmute::<nn_cv, glGenerateMipmap_t>(Self::ptr_filter(f(b"glGenerateMipmap\0".as_ptr())).ok_or("glGenerateMipmap")?);
    let glGenerateTextureMipmap_p = transmute::<nn_cv, glGenerateTextureMipmap_t>(Self::ptr_filter(f(b"glGenerateTextureMipmap\0".as_ptr())).ok_or("glGenerateTextureMipmap")?);
    let glGetActiveAtomicCounterBufferiv_p = transmute::<nn_cv, glGetActiveAtomicCounterBufferiv_t>(Self::ptr_filter(f(b"glGetActiveAtomicCounterBufferiv\0".as_ptr())).ok_or("glGetActiveAtomicCounterBufferiv")?);
    let glGetActiveAttrib_p = transmute::<nn_cv, glGetActiveAttrib_t>(Self::ptr_filter(f(b"glGetActiveAttrib\0".as_ptr())).ok_or("glGetActiveAttrib")?);
    let glGetActiveSubroutineName_p = transmute::<nn_cv, glGetActiveSubroutineName_t>(Self::ptr_filter(f(b"glGetActiveSubroutineName\0".as_ptr())).ok_or("glGetActiveSubroutineName")?);
    let glGetActiveSubroutineUniformName_p = transmute::<nn_cv, glGetActiveSubroutineUniformName_t>(Self::ptr_filter(f(b"glGetActiveSubroutineUniformName\0".as_ptr())).ok_or("glGetActiveSubroutineUniformName")?);
    let glGetActiveSubroutineUniformiv_p = transmute::<nn_cv, glGetActiveSubroutineUniformiv_t>(Self::ptr_filter(f(b"glGetActiveSubroutineUniformiv\0".as_ptr())).ok_or("glGetActiveSubroutineUniformiv")?);
    let glGetActiveUniform_p = transmute::<nn_cv, glGetActiveUniform_t>(Self::ptr_filter(f(b"glGetActiveUniform\0".as_ptr())).ok_or("glGetActiveUniform")?);
    let glGetActiveUniformBlockName_p = transmute::<nn_cv, glGetActiveUniformBlockName_t>(Self::ptr_filter(f(b"glGetActiveUniformBlockName\0".as_ptr())).ok_or("glGetActiveUniformBlockName")?);
    let glGetActiveUniformBlockiv_p = transmute::<nn_cv, glGetActiveUniformBlockiv_t>(Self::ptr_filter(f(b"glGetActiveUniformBlockiv\0".as_ptr())).ok_or("glGetActiveUniformBlockiv")?);
    let glGetActiveUniformName_p = transmute::<nn_cv, glGetActiveUniformName_t>(Self::ptr_filter(f(b"glGetActiveUniformName\0".as_ptr())).ok_or("glGetActiveUniformName")?);
    let glGetActiveUniformsiv_p = transmute::<nn_cv, glGetActiveUniformsiv_t>(Self::ptr_filter(f(b"glGetActiveUniformsiv\0".as_ptr())).ok_or("glGetActiveUniformsiv")?);
    let glGetAttachedShaders_p = transmute::<nn_cv, glGetAttachedShaders_t>(Self::ptr_filter(f(b"glGetAttachedShaders\0".as_ptr())).ok_or("glGetAttachedShaders")?);
    let glGetAttribLocation_p = transmute::<nn_cv, glGetAttribLocation_t>(Self::ptr_filter(f(b"glGetAttribLocation\0".as_ptr())).ok_or("glGetAttribLocation")?);
    let glGetBooleani_v_p = transmute::<nn_cv, glGetBooleani_v_t>(Self::ptr_filter(f(b"glGetBooleani_v\0".as_ptr())).ok_or("glGetBooleani_v")?);
    let glGetBooleanv_p = transmute::<nn_cv, glGetBooleanv_t>(Self::ptr_filter(f(b"glGetBooleanv\0".as_ptr())).ok_or("glGetBooleanv")?);
    let glGetBufferParameteri64v_p = transmute::<nn_cv, glGetBufferParameteri64v_t>(Self::ptr_filter(f(b"glGetBufferParameteri64v\0".as_ptr())).ok_or("glGetBufferParameteri64v")?);
    let glGetBufferParameteriv_p = transmute::<nn_cv, glGetBufferParameteriv_t>(Self::ptr_filter(f(b"glGetBufferParameteriv\0".as_ptr())).ok_or("glGetBufferParameteriv")?);
    let glGetBufferPointerv_p = transmute::<nn_cv, glGetBufferPointerv_t>(Self::ptr_filter(f(b"glGetBufferPointerv\0".as_ptr())).ok_or("glGetBufferPointerv")?);
    let glGetBufferSubData_p = transmute::<nn_cv, glGetBufferSubData_t>(Self::ptr_filter(f(b"glGetBufferSubData\0".as_ptr())).ok_or("glGetBufferSubData")?);
    let glGetCompressedTexImage_p = transmute::<nn_cv, glGetCompressedTexImage_t>(Self::ptr_filter(f(b"glGetCompressedTexImage\0".as_ptr())).ok_or("glGetCompressedTexImage")?);
    let glGetCompressedTextureImage_p = transmute::<nn_cv, glGetCompressedTextureImage_t>(Self::ptr_filter(f(b"glGetCompressedTextureImage\0".as_ptr())).ok_or("glGetCompressedTextureImage")?);
    let glGetCompressedTextureSubImage_p = transmute::<nn_cv, glGetCompressedTextureSubImage_t>(Self::ptr_filter(f(b"glGetCompressedTextureSubImage\0".as_ptr())).ok_or("glGetCompressedTextureSubImage")?);
    let glGetDebugMessageLog_p = transmute::<nn_cv, glGetDebugMessageLog_t>(Self::ptr_filter(f(b"glGetDebugMessageLog\0".as_ptr())).ok_or("glGetDebugMessageLog")?);
    let glGetDoublei_v_p = transmute::<nn_cv, glGetDoublei_v_t>(Self::ptr_filter(f(b"glGetDoublei_v\0".as_ptr())).ok_or("glGetDoublei_v")?);
    let glGetDoublev_p = transmute::<nn_cv, glGetDoublev_t>(Self::ptr_filter(f(b"glGetDoublev\0".as_ptr())).ok_or("glGetDoublev")?);
    let glGetError_p = transmute::<nn_cv, glGetError_t>(Self::ptr_filter(f(b"glGetError\0".as_ptr())).ok_or("glGetError")?);
    let glGetFloati_v_p = transmute::<nn_cv, glGetFloati_v_t>(Self::ptr_filter(f(b"glGetFloati_v\0".as_ptr())).ok_or("glGetFloati_v")?);
    let glGetFloatv_p = transmute::<nn_cv, glGetFloatv_t>(Self::ptr_filter(f(b"glGetFloatv\0".as_ptr())).ok_or("glGetFloatv")?);
    let glGetFragDataIndex_p = transmute::<nn_cv, glGetFragDataIndex_t>(Self::ptr_filter(f(b"glGetFragDataIndex\0".as_ptr())).ok_or("glGetFragDataIndex")?);
    let glGetFragDataLocation_p = transmute::<nn_cv, glGetFragDataLocation_t>(Self::ptr_filter(f(b"glGetFragDataLocation\0".as_ptr())).ok_or("glGetFragDataLocation")?);
    let glGetFramebufferAttachmentParameteriv_p = transmute::<nn_cv, glGetFramebufferAttachmentParameteriv_t>(Self::ptr_filter(f(b"glGetFramebufferAttachmentParameteriv\0".as_ptr())).ok_or("glGetFramebufferAttachmentParameteriv")?);
    let glGetFramebufferParameteriv_p = transmute::<nn_cv, glGetFramebufferParameteriv_t>(Self::ptr_filter(f(b"glGetFramebufferParameteriv\0".as_ptr())).ok_or("glGetFramebufferParameteriv")?);
    let glGetGraphicsResetStatus_p = transmute::<nn_cv, glGetGraphicsResetStatus_t>(Self::ptr_filter(f(b"glGetGraphicsResetStatus\0".as_ptr())).ok_or("glGetGraphicsResetStatus")?);
    let glGetInteger64i_v_p = transmute::<nn_cv, glGetInteger64i_v_t>(Self::ptr_filter(f(b"glGetInteger64i_v\0".as_ptr())).ok_or("glGetInteger64i_v")?);
    let glGetInteger64v_p = transmute::<nn_cv, glGetInteger64v_t>(Self::ptr_filter(f(b"glGetInteger64v\0".as_ptr())).ok_or("glGetInteger64v")?);
    let glGetIntegeri_v_p = transmute::<nn_cv, glGetIntegeri_v_t>(Self::ptr_filter(f(b"glGetIntegeri_v\0".as_ptr())).ok_or("glGetIntegeri_v")?);
    let glGetIntegerv_p = transmute::<nn_cv, glGetIntegerv_t>(Self::ptr_filter(f(b"glGetIntegerv\0".as_ptr())).ok_or("glGetIntegerv")?);
    let glGetInternalformati64v_p = transmute::<nn_cv, glGetInternalformati64v_t>(Self::ptr_filter(f(b"glGetInternalformati64v\0".as_ptr())).ok_or("glGetInternalformati64v")?);
    let glGetInternalformativ_p = transmute::<nn_cv, glGetInternalformativ_t>(Self::ptr_filter(f(b"glGetInternalformativ\0".as_ptr())).ok_or("glGetInternalformativ")?);
    let glGetMultisamplefv_p = transmute::<nn_cv, glGetMultisamplefv_t>(Self::ptr_filter(f(b"glGetMultisamplefv\0".as_ptr())).ok_or("glGetMultisamplefv")?);
    let glGetNamedBufferParameteri64v_p = transmute::<nn_cv, glGetNamedBufferParameteri64v_t>(Self::ptr_filter(f(b"glGetNamedBufferParameteri64v\0".as_ptr())).ok_or("glGetNamedBufferParameteri64v")?);
    let glGetNamedBufferParameteriv_p = transmute::<nn_cv, glGetNamedBufferParameteriv_t>(Self::ptr_filter(f(b"glGetNamedBufferParameteriv\0".as_ptr())).ok_or("glGetNamedBufferParameteriv")?);
    let glGetNamedBufferPointerv_p = transmute::<nn_cv, glGetNamedBufferPointerv_t>(Self::ptr_filter(f(b"glGetNamedBufferPointerv\0".as_ptr())).ok_or("glGetNamedBufferPointerv")?);
    let glGetNamedBufferSubData_p = transmute::<nn_cv, glGetNamedBufferSubData_t>(Self::ptr_filter(f(b"glGetNamedBufferSubData\0".as_ptr())).ok_or("glGetNamedBufferSubData")?);
    let glGetNamedFramebufferAttachmentParameteriv_p = transmute::<nn_cv, glGetNamedFramebufferAttachmentParameteriv_t>(Self::ptr_filter(f(b"glGetNamedFramebufferAttachmentParameteriv\0".as_ptr())).ok_or("glGetNamedFramebufferAttachmentParameteriv")?);
    let glGetNamedFramebufferParameteriv_p = transmute::<nn_cv, glGetNamedFramebufferParameteriv_t>(Self::ptr_filter(f(b"glGetNamedFramebufferParameteriv\0".as_ptr())).ok_or("glGetNamedFramebufferParameteriv")?);
    let glGetNamedRenderbufferParameteriv_p = transmute::<nn_cv, glGetNamedRenderbufferParameteriv_t>(Self::ptr_filter(f(b"glGetNamedRenderbufferParameteriv\0".as_ptr())).ok_or("glGetNamedRenderbufferParameteriv")?);
    let glGetObjectLabel_p = transmute::<nn_cv, glGetObjectLabel_t>(Self::ptr_filter(f(b"glGetObjectLabel\0".as_ptr())).ok_or("glGetObjectLabel")?);
    let glGetObjectPtrLabel_p = transmute::<nn_cv, glGetObjectPtrLabel_t>(Self::ptr_filter(f(b"glGetObjectPtrLabel\0".as_ptr())).ok_or("glGetObjectPtrLabel")?);
    let glGetPointerv_p = transmute::<nn_cv, glGetPointerv_t>(Self::ptr_filter(f(b"glGetPointerv\0".as_ptr())).ok_or("glGetPointerv")?);
    let glGetProgramBinary_p = transmute::<nn_cv, glGetProgramBinary_t>(Self::ptr_filter(f(b"glGetProgramBinary\0".as_ptr())).ok_or("glGetProgramBinary")?);
    let glGetProgramInfoLog_p = transmute::<nn_cv, glGetProgramInfoLog_t>(Self::ptr_filter(f(b"glGetProgramInfoLog\0".as_ptr())).ok_or("glGetProgramInfoLog")?);
    let glGetProgramInterfaceiv_p = transmute::<nn_cv, glGetProgramInterfaceiv_t>(Self::ptr_filter(f(b"glGetProgramInterfaceiv\0".as_ptr())).ok_or("glGetProgramInterfaceiv")?);
    let glGetProgramPipelineInfoLog_p = transmute::<nn_cv, glGetProgramPipelineInfoLog_t>(Self::ptr_filter(f(b"glGetProgramPipelineInfoLog\0".as_ptr())).ok_or("glGetProgramPipelineInfoLog")?);
    let glGetProgramPipelineiv_p = transmute::<nn_cv, glGetProgramPipelineiv_t>(Self::ptr_filter(f(b"glGetProgramPipelineiv\0".as_ptr())).ok_or("glGetProgramPipelineiv")?);
    let glGetProgramResourceIndex_p = transmute::<nn_cv, glGetProgramResourceIndex_t>(Self::ptr_filter(f(b"glGetProgramResourceIndex\0".as_ptr())).ok_or("glGetProgramResourceIndex")?);
    let glGetProgramResourceLocation_p = transmute::<nn_cv, glGetProgramResourceLocation_t>(Self::ptr_filter(f(b"glGetProgramResourceLocation\0".as_ptr())).ok_or("glGetProgramResourceLocation")?);
    let glGetProgramResourceLocationIndex_p = transmute::<nn_cv, glGetProgramResourceLocationIndex_t>(Self::ptr_filter(f(b"glGetProgramResourceLocationIndex\0".as_ptr())).ok_or("glGetProgramResourceLocationIndex")?);
    let glGetProgramResourceName_p = transmute::<nn_cv, glGetProgramResourceName_t>(Self::ptr_filter(f(b"glGetProgramResourceName\0".as_ptr())).ok_or("glGetProgramResourceName")?);
    let glGetProgramResourceiv_p = transmute::<nn_cv, glGetProgramResourceiv_t>(Self::ptr_filter(f(b"glGetProgramResourceiv\0".as_ptr())).ok_or("glGetProgramResourceiv")?);
    let glGetProgramStageiv_p = transmute::<nn_cv, glGetProgramStageiv_t>(Self::ptr_filter(f(b"glGetProgramStageiv\0".as_ptr())).ok_or("glGetProgramStageiv")?);
    let glGetProgramiv_p = transmute::<nn_cv, glGetProgramiv_t>(Self::ptr_filter(f(b"glGetProgramiv\0".as_ptr())).ok_or("glGetProgramiv")?);
    let glGetQueryBufferObjecti64v_p = transmute::<nn_cv, glGetQueryBufferObjecti64v_t>(Self::ptr_filter(f(b"glGetQueryBufferObjecti64v\0".as_ptr())).ok_or("glGetQueryBufferObjecti64v")?);
    let glGetQueryBufferObjectiv_p = transmute::<nn_cv, glGetQueryBufferObjectiv_t>(Self::ptr_filter(f(b"glGetQueryBufferObjectiv\0".as_ptr())).ok_or("glGetQueryBufferObjectiv")?);
    let glGetQueryBufferObjectui64v_p = transmute::<nn_cv, glGetQueryBufferObjectui64v_t>(Self::ptr_filter(f(b"glGetQueryBufferObjectui64v\0".as_ptr())).ok_or("glGetQueryBufferObjectui64v")?);
    let glGetQueryBufferObjectuiv_p = transmute::<nn_cv, glGetQueryBufferObjectuiv_t>(Self::ptr_filter(f(b"glGetQueryBufferObjectuiv\0".as_ptr())).ok_or("glGetQueryBufferObjectuiv")?);
    let glGetQueryIndexediv_p = transmute::<nn_cv, glGetQueryIndexediv_t>(Self::ptr_filter(f(b"glGetQueryIndexediv\0".as_ptr())).ok_or("glGetQueryIndexediv")?);
    let glGetQueryObjecti64v_p = transmute::<nn_cv, glGetQueryObjecti64v_t>(Self::ptr_filter(f(b"glGetQueryObjecti64v\0".as_ptr())).ok_or("glGetQueryObjecti64v")?);
    let glGetQueryObjectiv_p = transmute::<nn_cv, glGetQueryObjectiv_t>(Self::ptr_filter(f(b"glGetQueryObjectiv\0".as_ptr())).ok_or("glGetQueryObjectiv")?);
    let glGetQueryObjectui64v_p = transmute::<nn_cv, glGetQueryObjectui64v_t>(Self::ptr_filter(f(b"glGetQueryObjectui64v\0".as_ptr())).ok_or("glGetQueryObjectui64v")?);
    let glGetQueryObjectuiv_p = transmute::<nn_cv, glGetQueryObjectuiv_t>(Self::ptr_filter(f(b"glGetQueryObjectuiv\0".as_ptr())).ok_or("glGetQueryObjectuiv")?);
    let glGetQueryiv_p = transmute::<nn_cv, glGetQueryiv_t>(Self::ptr_filter(f(b"glGetQueryiv\0".as_ptr())).ok_or("glGetQueryiv")?);
    let glGetRenderbufferParameteriv_p = transmute::<nn_cv, glGetRenderbufferParameteriv_t>(Self::ptr_filter(f(b"glGetRenderbufferParameteriv\0".as_ptr())).ok_or("glGetRenderbufferParameteriv")?);
    let glGetSamplerParameterIiv_p = transmute::<nn_cv, glGetSamplerParameterIiv_t>(Self::ptr_filter(f(b"glGetSamplerParameterIiv\0".as_ptr())).ok_or("glGetSamplerParameterIiv")?);
    let glGetSamplerParameterIuiv_p = transmute::<nn_cv, glGetSamplerParameterIuiv_t>(Self::ptr_filter(f(b"glGetSamplerParameterIuiv\0".as_ptr())).ok_or("glGetSamplerParameterIuiv")?);
    let glGetSamplerParameterfv_p = transmute::<nn_cv, glGetSamplerParameterfv_t>(Self::ptr_filter(f(b"glGetSamplerParameterfv\0".as_ptr())).ok_or("glGetSamplerParameterfv")?);
    let glGetSamplerParameteriv_p = transmute::<nn_cv, glGetSamplerParameteriv_t>(Self::ptr_filter(f(b"glGetSamplerParameteriv\0".as_ptr())).ok_or("glGetSamplerParameteriv")?);
    let glGetShaderInfoLog_p = transmute::<nn_cv, glGetShaderInfoLog_t>(Self::ptr_filter(f(b"glGetShaderInfoLog\0".as_ptr())).ok_or("glGetShaderInfoLog")?);
    let glGetShaderPrecisionFormat_p = transmute::<nn_cv, glGetShaderPrecisionFormat_t>(Self::ptr_filter(f(b"glGetShaderPrecisionFormat\0".as_ptr())).ok_or("glGetShaderPrecisionFormat")?);
    let glGetShaderSource_p = transmute::<nn_cv, glGetShaderSource_t>(Self::ptr_filter(f(b"glGetShaderSource\0".as_ptr())).ok_or("glGetShaderSource")?);
    let glGetShaderiv_p = transmute::<nn_cv, glGetShaderiv_t>(Self::ptr_filter(f(b"glGetShaderiv\0".as_ptr())).ok_or("glGetShaderiv")?);
    let glGetString_p = transmute::<nn_cv, glGetString_t>(Self::ptr_filter(f(b"glGetString\0".as_ptr())).ok_or("glGetString")?);
    let glGetStringi_p = transmute::<nn_cv, glGetStringi_t>(Self::ptr_filter(f(b"glGetStringi\0".as_ptr())).ok_or("glGetStringi")?);
    let glGetSubroutineIndex_p = transmute::<nn_cv, glGetSubroutineIndex_t>(Self::ptr_filter(f(b"glGetSubroutineIndex\0".as_ptr())).ok_or("glGetSubroutineIndex")?);
    let glGetSubroutineUniformLocation_p = transmute::<nn_cv, glGetSubroutineUniformLocation_t>(Self::ptr_filter(f(b"glGetSubroutineUniformLocation\0".as_ptr())).ok_or("glGetSubroutineUniformLocation")?);
    let glGetSynciv_p = transmute::<nn_cv, glGetSynciv_t>(Self::ptr_filter(f(b"glGetSynciv\0".as_ptr())).ok_or("glGetSynciv")?);
    let glGetTexImage_p = transmute::<nn_cv, glGetTexImage_t>(Self::ptr_filter(f(b"glGetTexImage\0".as_ptr())).ok_or("glGetTexImage")?);
    let glGetTexLevelParameterfv_p = transmute::<nn_cv, glGetTexLevelParameterfv_t>(Self::ptr_filter(f(b"glGetTexLevelParameterfv\0".as_ptr())).ok_or("glGetTexLevelParameterfv")?);
    let glGetTexLevelParameteriv_p = transmute::<nn_cv, glGetTexLevelParameteriv_t>(Self::ptr_filter(f(b"glGetTexLevelParameteriv\0".as_ptr())).ok_or("glGetTexLevelParameteriv")?);
    let glGetTexParameterIiv_p = transmute::<nn_cv, glGetTexParameterIiv_t>(Self::ptr_filter(f(b"glGetTexParameterIiv\0".as_ptr())).ok_or("glGetTexParameterIiv")?);
    let glGetTexParameterIuiv_p = transmute::<nn_cv, glGetTexParameterIuiv_t>(Self::ptr_filter(f(b"glGetTexParameterIuiv\0".as_ptr())).ok_or("glGetTexParameterIuiv")?);
    let glGetTexParameterfv_p = transmute::<nn_cv, glGetTexParameterfv_t>(Self::ptr_filter(f(b"glGetTexParameterfv\0".as_ptr())).ok_or("glGetTexParameterfv")?);
    let glGetTexParameteriv_p = transmute::<nn_cv, glGetTexParameteriv_t>(Self::ptr_filter(f(b"glGetTexParameteriv\0".as_ptr())).ok_or("glGetTexParameteriv")?);
    let glGetTextureImage_p = transmute::<nn_cv, glGetTextureImage_t>(Self::ptr_filter(f(b"glGetTextureImage\0".as_ptr())).ok_or("glGetTextureImage")?);
    let glGetTextureLevelParameterfv_p = transmute::<nn_cv, glGetTextureLevelParameterfv_t>(Self::ptr_filter(f(b"glGetTextureLevelParameterfv\0".as_ptr())).ok_or("glGetTextureLevelParameterfv")?);
    let glGetTextureLevelParameteriv_p = transmute::<nn_cv, glGetTextureLevelParameteriv_t>(Self::ptr_filter(f(b"glGetTextureLevelParameteriv\0".as_ptr())).ok_or("glGetTextureLevelParameteriv")?);
    let glGetTextureParameterIiv_p = transmute::<nn_cv, glGetTextureParameterIiv_t>(Self::ptr_filter(f(b"glGetTextureParameterIiv\0".as_ptr())).ok_or("glGetTextureParameterIiv")?);
    let glGetTextureParameterIuiv_p = transmute::<nn_cv, glGetTextureParameterIuiv_t>(Self::ptr_filter(f(b"glGetTextureParameterIuiv\0".as_ptr())).ok_or("glGetTextureParameterIuiv")?);
    let glGetTextureParameterfv_p = transmute::<nn_cv, glGetTextureParameterfv_t>(Self::ptr_filter(f(b"glGetTextureParameterfv\0".as_ptr())).ok_or("glGetTextureParameterfv")?);
    let glGetTextureParameteriv_p = transmute::<nn_cv, glGetTextureParameteriv_t>(Self::ptr_filter(f(b"glGetTextureParameteriv\0".as_ptr())).ok_or("glGetTextureParameteriv")?);
    let glGetTextureSubImage_p = transmute::<nn_cv, glGetTextureSubImage_t>(Self::ptr_filter(f(b"glGetTextureSubImage\0".as_ptr())).ok_or("glGetTextureSubImage")?);
    let glGetTransformFeedbackVarying_p = transmute::<nn_cv, glGetTransformFeedbackVarying_t>(Self::ptr_filter(f(b"glGetTransformFeedbackVarying\0".as_ptr())).ok_or("glGetTransformFeedbackVarying")?);
    let glGetTransformFeedbacki64_v_p = transmute::<nn_cv, glGetTransformFeedbacki64_v_t>(Self::ptr_filter(f(b"glGetTransformFeedbacki64_v\0".as_ptr())).ok_or("glGetTransformFeedbacki64_v")?);
    let glGetTransformFeedbacki_v_p = transmute::<nn_cv, glGetTransformFeedbacki_v_t>(Self::ptr_filter(f(b"glGetTransformFeedbacki_v\0".as_ptr())).ok_or("glGetTransformFeedbacki_v")?);
    let glGetTransformFeedbackiv_p = transmute::<nn_cv, glGetTransformFeedbackiv_t>(Self::ptr_filter(f(b"glGetTransformFeedbackiv\0".as_ptr())).ok_or("glGetTransformFeedbackiv")?);
    let glGetUniformBlockIndex_p = transmute::<nn_cv, glGetUniformBlockIndex_t>(Self::ptr_filter(f(b"glGetUniformBlockIndex\0".as_ptr())).ok_or("glGetUniformBlockIndex")?);
    let glGetUniformIndices_p = transmute::<nn_cv, glGetUniformIndices_t>(Self::ptr_filter(f(b"glGetUniformIndices\0".as_ptr())).ok_or("glGetUniformIndices")?);
    let glGetUniformLocation_p = transmute::<nn_cv, glGetUniformLocation_t>(Self::ptr_filter(f(b"glGetUniformLocation\0".as_ptr())).ok_or("glGetUniformLocation")?);
    let glGetUniformSubroutineuiv_p = transmute::<nn_cv, glGetUniformSubroutineuiv_t>(Self::ptr_filter(f(b"glGetUniformSubroutineuiv\0".as_ptr())).ok_or("glGetUniformSubroutineuiv")?);
    let glGetUniformdv_p = transmute::<nn_cv, glGetUniformdv_t>(Self::ptr_filter(f(b"glGetUniformdv\0".as_ptr())).ok_or("glGetUniformdv")?);
    let glGetUniformfv_p = transmute::<nn_cv, glGetUniformfv_t>(Self::ptr_filter(f(b"glGetUniformfv\0".as_ptr())).ok_or("glGetUniformfv")?);
    let glGetUniformiv_p = transmute::<nn_cv, glGetUniformiv_t>(Self::ptr_filter(f(b"glGetUniformiv\0".as_ptr())).ok_or("glGetUniformiv")?);
    let glGetUniformuiv_p = transmute::<nn_cv, glGetUniformuiv_t>(Self::ptr_filter(f(b"glGetUniformuiv\0".as_ptr())).ok_or("glGetUniformuiv")?);
    let glGetVertexArrayIndexed64iv_p = transmute::<nn_cv, glGetVertexArrayIndexed64iv_t>(Self::ptr_filter(f(b"glGetVertexArrayIndexed64iv\0".as_ptr())).ok_or("glGetVertexArrayIndexed64iv")?);
    let glGetVertexArrayIndexediv_p = transmute::<nn_cv, glGetVertexArrayIndexediv_t>(Self::ptr_filter(f(b"glGetVertexArrayIndexediv\0".as_ptr())).ok_or("glGetVertexArrayIndexediv")?);
    let glGetVertexArrayiv_p = transmute::<nn_cv, glGetVertexArrayiv_t>(Self::ptr_filter(f(b"glGetVertexArrayiv\0".as_ptr())).ok_or("glGetVertexArrayiv")?);
    let glGetVertexAttribIiv_p = transmute::<nn_cv, glGetVertexAttribIiv_t>(Self::ptr_filter(f(b"glGetVertexAttribIiv\0".as_ptr())).ok_or("glGetVertexAttribIiv")?);
    let glGetVertexAttribIuiv_p = transmute::<nn_cv, glGetVertexAttribIuiv_t>(Self::ptr_filter(f(b"glGetVertexAttribIuiv\0".as_ptr())).ok_or("glGetVertexAttribIuiv")?);
    let glGetVertexAttribLdv_p = transmute::<nn_cv, glGetVertexAttribLdv_t>(Self::ptr_filter(f(b"glGetVertexAttribLdv\0".as_ptr())).ok_or("glGetVertexAttribLdv")?);
    let glGetVertexAttribPointerv_p = transmute::<nn_cv, glGetVertexAttribPointerv_t>(Self::ptr_filter(f(b"glGetVertexAttribPointerv\0".as_ptr())).ok_or("glGetVertexAttribPointerv")?);
    let glGetVertexAttribdv_p = transmute::<nn_cv, glGetVertexAttribdv_t>(Self::ptr_filter(f(b"glGetVertexAttribdv\0".as_ptr())).ok_or("glGetVertexAttribdv")?);
    let glGetVertexAttribfv_p = transmute::<nn_cv, glGetVertexAttribfv_t>(Self::ptr_filter(f(b"glGetVertexAttribfv\0".as_ptr())).ok_or("glGetVertexAttribfv")?);
    let glGetVertexAttribiv_p = transmute::<nn_cv, glGetVertexAttribiv_t>(Self::ptr_filter(f(b"glGetVertexAttribiv\0".as_ptr())).ok_or("glGetVertexAttribiv")?);
    let glGetnColorTable_p = transmute::<nn_cv, glGetnColorTable_t>(Self::ptr_filter(f(b"glGetnColorTable\0".as_ptr())).ok_or("glGetnColorTable")?);
    let glGetnCompressedTexImage_p = transmute::<nn_cv, glGetnCompressedTexImage_t>(Self::ptr_filter(f(b"glGetnCompressedTexImage\0".as_ptr())).ok_or("glGetnCompressedTexImage")?);
    let glGetnConvolutionFilter_p = transmute::<nn_cv, glGetnConvolutionFilter_t>(Self::ptr_filter(f(b"glGetnConvolutionFilter\0".as_ptr())).ok_or("glGetnConvolutionFilter")?);
    let glGetnHistogram_p = transmute::<nn_cv, glGetnHistogram_t>(Self::ptr_filter(f(b"glGetnHistogram\0".as_ptr())).ok_or("glGetnHistogram")?);
    let glGetnMapdv_p = transmute::<nn_cv, glGetnMapdv_t>(Self::ptr_filter(f(b"glGetnMapdv\0".as_ptr())).ok_or("glGetnMapdv")?);
    let glGetnMapfv_p = transmute::<nn_cv, glGetnMapfv_t>(Self::ptr_filter(f(b"glGetnMapfv\0".as_ptr())).ok_or("glGetnMapfv")?);
    let glGetnMapiv_p = transmute::<nn_cv, glGetnMapiv_t>(Self::ptr_filter(f(b"glGetnMapiv\0".as_ptr())).ok_or("glGetnMapiv")?);
    let glGetnMinmax_p = transmute::<nn_cv, glGetnMinmax_t>(Self::ptr_filter(f(b"glGetnMinmax\0".as_ptr())).ok_or("glGetnMinmax")?);
    let glGetnPixelMapfv_p = transmute::<nn_cv, glGetnPixelMapfv_t>(Self::ptr_filter(f(b"glGetnPixelMapfv\0".as_ptr())).ok_or("glGetnPixelMapfv")?);
    let glGetnPixelMapuiv_p = transmute::<nn_cv, glGetnPixelMapuiv_t>(Self::ptr_filter(f(b"glGetnPixelMapuiv\0".as_ptr())).ok_or("glGetnPixelMapuiv")?);
    let glGetnPixelMapusv_p = transmute::<nn_cv, glGetnPixelMapusv_t>(Self::ptr_filter(f(b"glGetnPixelMapusv\0".as_ptr())).ok_or("glGetnPixelMapusv")?);
    let glGetnPolygonStipple_p = transmute::<nn_cv, glGetnPolygonStipple_t>(Self::ptr_filter(f(b"glGetnPolygonStipple\0".as_ptr())).ok_or("glGetnPolygonStipple")?);
    let glGetnSeparableFilter_p = transmute::<nn_cv, glGetnSeparableFilter_t>(Self::ptr_filter(f(b"glGetnSeparableFilter\0".as_ptr())).ok_or("glGetnSeparableFilter")?);
    let glGetnTexImage_p = transmute::<nn_cv, glGetnTexImage_t>(Self::ptr_filter(f(b"glGetnTexImage\0".as_ptr())).ok_or("glGetnTexImage")?);
    let glGetnUniformdv_p = transmute::<nn_cv, glGetnUniformdv_t>(Self::ptr_filter(f(b"glGetnUniformdv\0".as_ptr())).ok_or("glGetnUniformdv")?);
    let glGetnUniformfv_p = transmute::<nn_cv, glGetnUniformfv_t>(Self::ptr_filter(f(b"glGetnUniformfv\0".as_ptr())).ok_or("glGetnUniformfv")?);
    let glGetnUniformiv_p = transmute::<nn_cv, glGetnUniformiv_t>(Self::ptr_filter(f(b"glGetnUniformiv\0".as_ptr())).ok_or("glGetnUniformiv")?);
    let glGetnUniformuiv_p = transmute::<nn_cv, glGetnUniformuiv_t>(Self::ptr_filter(f(b"glGetnUniformuiv\0".as_ptr())).ok_or("glGetnUniformuiv")?);
    let glHint_p = transmute::<nn_cv, glHint_t>(Self::ptr_filter(f(b"glHint\0".as_ptr())).ok_or("glHint")?);
    let glInvalidateBufferData_p = transmute::<nn_cv, glInvalidateBufferData_t>(Self::ptr_filter(f(b"glInvalidateBufferData\0".as_ptr())).ok_or("glInvalidateBufferData")?);
    let glInvalidateBufferSubData_p = transmute::<nn_cv, glInvalidateBufferSubData_t>(Self::ptr_filter(f(b"glInvalidateBufferSubData\0".as_ptr())).ok_or("glInvalidateBufferSubData")?);
    let glInvalidateFramebuffer_p = transmute::<nn_cv, glInvalidateFramebuffer_t>(Self::ptr_filter(f(b"glInvalidateFramebuffer\0".as_ptr())).ok_or("glInvalidateFramebuffer")?);
    let glInvalidateNamedFramebufferData_p = transmute::<nn_cv, glInvalidateNamedFramebufferData_t>(Self::ptr_filter(f(b"glInvalidateNamedFramebufferData\0".as_ptr())).ok_or("glInvalidateNamedFramebufferData")?);
    let glInvalidateNamedFramebufferSubData_p = transmute::<nn_cv, glInvalidateNamedFramebufferSubData_t>(Self::ptr_filter(f(b"glInvalidateNamedFramebufferSubData\0".as_ptr())).ok_or("glInvalidateNamedFramebufferSubData")?);
    let glInvalidateSubFramebuffer_p = transmute::<nn_cv, glInvalidateSubFramebuffer_t>(Self::ptr_filter(f(b"glInvalidateSubFramebuffer\0".as_ptr())).ok_or("glInvalidateSubFramebuffer")?);
    let glInvalidateTexImage_p = transmute::<nn_cv, glInvalidateTexImage_t>(Self::ptr_filter(f(b"glInvalidateTexImage\0".as_ptr())).ok_or("glInvalidateTexImage")?);
    let glInvalidateTexSubImage_p = transmute::<nn_cv, glInvalidateTexSubImage_t>(Self::ptr_filter(f(b"glInvalidateTexSubImage\0".as_ptr())).ok_or("glInvalidateTexSubImage")?);
    let glIsBuffer_p = transmute::<nn_cv, glIsBuffer_t>(Self::ptr_filter(f(b"glIsBuffer\0".as_ptr())).ok_or("glIsBuffer")?);
    let glIsEnabled_p = transmute::<nn_cv, glIsEnabled_t>(Self::ptr_filter(f(b"glIsEnabled\0".as_ptr())).ok_or("glIsEnabled")?);
    let glIsEnabledi_p = transmute::<nn_cv, glIsEnabledi_t>(Self::ptr_filter(f(b"glIsEnabledi\0".as_ptr())).ok_or("glIsEnabledi")?);
    let glIsFramebuffer_p = transmute::<nn_cv, glIsFramebuffer_t>(Self::ptr_filter(f(b"glIsFramebuffer\0".as_ptr())).ok_or("glIsFramebuffer")?);
    let glIsProgram_p = transmute::<nn_cv, glIsProgram_t>(Self::ptr_filter(f(b"glIsProgram\0".as_ptr())).ok_or("glIsProgram")?);
    let glIsProgramPipeline_p = transmute::<nn_cv, glIsProgramPipeline_t>(Self::ptr_filter(f(b"glIsProgramPipeline\0".as_ptr())).ok_or("glIsProgramPipeline")?);
    let glIsQuery_p = transmute::<nn_cv, glIsQuery_t>(Self::ptr_filter(f(b"glIsQuery\0".as_ptr())).ok_or("glIsQuery")?);
    let glIsRenderbuffer_p = transmute::<nn_cv, glIsRenderbuffer_t>(Self::ptr_filter(f(b"glIsRenderbuffer\0".as_ptr())).ok_or("glIsRenderbuffer")?);
    let glIsSampler_p = transmute::<nn_cv, glIsSampler_t>(Self::ptr_filter(f(b"glIsSampler\0".as_ptr())).ok_or("glIsSampler")?);
    let glIsShader_p = transmute::<nn_cv, glIsShader_t>(Self::ptr_filter(f(b"glIsShader\0".as_ptr())).ok_or("glIsShader")?);
    let glIsSync_p = transmute::<nn_cv, glIsSync_t>(Self::ptr_filter(f(b"glIsSync\0".as_ptr())).ok_or("glIsSync")?);
    let glIsTexture_p = transmute::<nn_cv, glIsTexture_t>(Self::ptr_filter(f(b"glIsTexture\0".as_ptr())).ok_or("glIsTexture")?);
    let glIsTransformFeedback_p = transmute::<nn_cv, glIsTransformFeedback_t>(Self::ptr_filter(f(b"glIsTransformFeedback\0".as_ptr())).ok_or("glIsTransformFeedback")?);
    let glIsVertexArray_p = transmute::<nn_cv, glIsVertexArray_t>(Self::ptr_filter(f(b"glIsVertexArray\0".as_ptr())).ok_or("glIsVertexArray")?);
    let glLineWidth_p = transmute::<nn_cv, glLineWidth_t>(Self::ptr_filter(f(b"glLineWidth\0".as_ptr())).ok_or("glLineWidth")?);
    let glLinkProgram_p = transmute::<nn_cv, glLinkProgram_t>(Self::ptr_filter(f(b"glLinkProgram\0".as_ptr())).ok_or("glLinkProgram")?);
    let glLogicOp_p = transmute::<nn_cv, glLogicOp_t>(Self::ptr_filter(f(b"glLogicOp\0".as_ptr())).ok_or("glLogicOp")?);
    let glMapBuffer_p = transmute::<nn_cv, glMapBuffer_t>(Self::ptr_filter(f(b"glMapBuffer\0".as_ptr())).ok_or("glMapBuffer")?);
    let glMapBufferRange_p = transmute::<nn_cv, glMapBufferRange_t>(Self::ptr_filter(f(b"glMapBufferRange\0".as_ptr())).ok_or("glMapBufferRange")?);
    let glMapNamedBuffer_p = transmute::<nn_cv, glMapNamedBuffer_t>(Self::ptr_filter(f(b"glMapNamedBuffer\0".as_ptr())).ok_or("glMapNamedBuffer")?);
    let glMapNamedBufferRange_p = transmute::<nn_cv, glMapNamedBufferRange_t>(Self::ptr_filter(f(b"glMapNamedBufferRange\0".as_ptr())).ok_or("glMapNamedBufferRange")?);
    let glMemoryBarrier_p = transmute::<nn_cv, glMemoryBarrier_t>(Self::ptr_filter(f(b"glMemoryBarrier\0".as_ptr())).ok_or("glMemoryBarrier")?);
    let glMemoryBarrierByRegion_p = transmute::<nn_cv, glMemoryBarrierByRegion_t>(Self::ptr_filter(f(b"glMemoryBarrierByRegion\0".as_ptr())).ok_or("glMemoryBarrierByRegion")?);
    let glMinSampleShading_p = transmute::<nn_cv, glMinSampleShading_t>(Self::ptr_filter(f(b"glMinSampleShading\0".as_ptr())).ok_or("glMinSampleShading")?);
    let glMultiDrawArrays_p = transmute::<nn_cv, glMultiDrawArrays_t>(Self::ptr_filter(f(b"glMultiDrawArrays\0".as_ptr())).ok_or("glMultiDrawArrays")?);
    let glMultiDrawArraysIndirect_p = transmute::<nn_cv, glMultiDrawArraysIndirect_t>(Self::ptr_filter(f(b"glMultiDrawArraysIndirect\0".as_ptr())).ok_or("glMultiDrawArraysIndirect")?);
    let glMultiDrawArraysIndirectCount_p = transmute::<nn_cv, glMultiDrawArraysIndirectCount_t>(Self::ptr_filter(f(b"glMultiDrawArraysIndirectCount\0".as_ptr())).ok_or("glMultiDrawArraysIndirectCount")?);
    let glMultiDrawElements_p = transmute::<nn_cv, glMultiDrawElements_t>(Self::ptr_filter(f(b"glMultiDrawElements\0".as_ptr())).ok_or("glMultiDrawElements")?);
    let glMultiDrawElementsBaseVertex_p = transmute::<nn_cv, glMultiDrawElementsBaseVertex_t>(Self::ptr_filter(f(b"glMultiDrawElementsBaseVertex\0".as_ptr())).ok_or("glMultiDrawElementsBaseVertex")?);
    let glMultiDrawElementsIndirect_p = transmute::<nn_cv, glMultiDrawElementsIndirect_t>(Self::ptr_filter(f(b"glMultiDrawElementsIndirect\0".as_ptr())).ok_or("glMultiDrawElementsIndirect")?);
    let glMultiDrawElementsIndirectCount_p = transmute::<nn_cv, glMultiDrawElementsIndirectCount_t>(Self::ptr_filter(f(b"glMultiDrawElementsIndirectCount\0".as_ptr())).ok_or("glMultiDrawElementsIndirectCount")?);
    let glMultiTexCoordP1ui_p = transmute::<nn_cv, glMultiTexCoordP1ui_t>(Self::ptr_filter(f(b"glMultiTexCoordP1ui\0".as_ptr())).ok_or("glMultiTexCoordP1ui")?);
    let glMultiTexCoordP1uiv_p = transmute::<nn_cv, glMultiTexCoordP1uiv_t>(Self::ptr_filter(f(b"glMultiTexCoordP1uiv\0".as_ptr())).ok_or("glMultiTexCoordP1uiv")?);
    let glMultiTexCoordP2ui_p = transmute::<nn_cv, glMultiTexCoordP2ui_t>(Self::ptr_filter(f(b"glMultiTexCoordP2ui\0".as_ptr())).ok_or("glMultiTexCoordP2ui")?);
    let glMultiTexCoordP2uiv_p = transmute::<nn_cv, glMultiTexCoordP2uiv_t>(Self::ptr_filter(f(b"glMultiTexCoordP2uiv\0".as_ptr())).ok_or("glMultiTexCoordP2uiv")?);
    let glMultiTexCoordP3ui_p = transmute::<nn_cv, glMultiTexCoordP3ui_t>(Self::ptr_filter(f(b"glMultiTexCoordP3ui\0".as_ptr())).ok_or("glMultiTexCoordP3ui")?);
    let glMultiTexCoordP3uiv_p = transmute::<nn_cv, glMultiTexCoordP3uiv_t>(Self::ptr_filter(f(b"glMultiTexCoordP3uiv\0".as_ptr())).ok_or("glMultiTexCoordP3uiv")?);
    let glMultiTexCoordP4ui_p = transmute::<nn_cv, glMultiTexCoordP4ui_t>(Self::ptr_filter(f(b"glMultiTexCoordP4ui\0".as_ptr())).ok_or("glMultiTexCoordP4ui")?);
    let glMultiTexCoordP4uiv_p = transmute::<nn_cv, glMultiTexCoordP4uiv_t>(Self::ptr_filter(f(b"glMultiTexCoordP4uiv\0".as_ptr())).ok_or("glMultiTexCoordP4uiv")?);
    let glNamedBufferData_p = transmute::<nn_cv, glNamedBufferData_t>(Self::ptr_filter(f(b"glNamedBufferData\0".as_ptr())).ok_or("glNamedBufferData")?);
    let glNamedBufferStorage_p = transmute::<nn_cv, glNamedBufferStorage_t>(Self::ptr_filter(f(b"glNamedBufferStorage\0".as_ptr())).ok_or("glNamedBufferStorage")?);
    let glNamedBufferSubData_p = transmute::<nn_cv, glNamedBufferSubData_t>(Self::ptr_filter(f(b"glNamedBufferSubData\0".as_ptr())).ok_or("glNamedBufferSubData")?);
    let glNamedFramebufferDrawBuffer_p = transmute::<nn_cv, glNamedFramebufferDrawBuffer_t>(Self::ptr_filter(f(b"glNamedFramebufferDrawBuffer\0".as_ptr())).ok_or("glNamedFramebufferDrawBuffer")?);
    let glNamedFramebufferDrawBuffers_p = transmute::<nn_cv, glNamedFramebufferDrawBuffers_t>(Self::ptr_filter(f(b"glNamedFramebufferDrawBuffers\0".as_ptr())).ok_or("glNamedFramebufferDrawBuffers")?);
    let glNamedFramebufferParameteri_p = transmute::<nn_cv, glNamedFramebufferParameteri_t>(Self::ptr_filter(f(b"glNamedFramebufferParameteri\0".as_ptr())).ok_or("glNamedFramebufferParameteri")?);
    let glNamedFramebufferReadBuffer_p = transmute::<nn_cv, glNamedFramebufferReadBuffer_t>(Self::ptr_filter(f(b"glNamedFramebufferReadBuffer\0".as_ptr())).ok_or("glNamedFramebufferReadBuffer")?);
    let glNamedFramebufferRenderbuffer_p = transmute::<nn_cv, glNamedFramebufferRenderbuffer_t>(Self::ptr_filter(f(b"glNamedFramebufferRenderbuffer\0".as_ptr())).ok_or("glNamedFramebufferRenderbuffer")?);
    let glNamedFramebufferTexture_p = transmute::<nn_cv, glNamedFramebufferTexture_t>(Self::ptr_filter(f(b"glNamedFramebufferTexture\0".as_ptr())).ok_or("glNamedFramebufferTexture")?);
    let glNamedFramebufferTextureLayer_p = transmute::<nn_cv, glNamedFramebufferTextureLayer_t>(Self::ptr_filter(f(b"glNamedFramebufferTextureLayer\0".as_ptr())).ok_or("glNamedFramebufferTextureLayer")?);
    let glNamedRenderbufferStorage_p = transmute::<nn_cv, glNamedRenderbufferStorage_t>(Self::ptr_filter(f(b"glNamedRenderbufferStorage\0".as_ptr())).ok_or("glNamedRenderbufferStorage")?);
    let glNamedRenderbufferStorageMultisample_p = transmute::<nn_cv, glNamedRenderbufferStorageMultisample_t>(Self::ptr_filter(f(b"glNamedRenderbufferStorageMultisample\0".as_ptr())).ok_or("glNamedRenderbufferStorageMultisample")?);
    let glNormalP3ui_p = transmute::<nn_cv, glNormalP3ui_t>(Self::ptr_filter(f(b"glNormalP3ui\0".as_ptr())).ok_or("glNormalP3ui")?);
    let glNormalP3uiv_p = transmute::<nn_cv, glNormalP3uiv_t>(Self::ptr_filter(f(b"glNormalP3uiv\0".as_ptr())).ok_or("glNormalP3uiv")?);
    let glObjectLabel_p = transmute::<nn_cv, glObjectLabel_t>(Self::ptr_filter(f(b"glObjectLabel\0".as_ptr())).ok_or("glObjectLabel")?);
    let glObjectPtrLabel_p = transmute::<nn_cv, glObjectPtrLabel_t>(Self::ptr_filter(f(b"glObjectPtrLabel\0".as_ptr())).ok_or("glObjectPtrLabel")?);
    let glPatchParameterfv_p = transmute::<nn_cv, glPatchParameterfv_t>(Self::ptr_filter(f(b"glPatchParameterfv\0".as_ptr())).ok_or("glPatchParameterfv")?);
    let glPatchParameteri_p = transmute::<nn_cv, glPatchParameteri_t>(Self::ptr_filter(f(b"glPatchParameteri\0".as_ptr())).ok_or("glPatchParameteri")?);
    let glPauseTransformFeedback_p = transmute::<nn_cv, glPauseTransformFeedback_t>(Self::ptr_filter(f(b"glPauseTransformFeedback\0".as_ptr())).ok_or("glPauseTransformFeedback")?);
    let glPixelStoref_p = transmute::<nn_cv, glPixelStoref_t>(Self::ptr_filter(f(b"glPixelStoref\0".as_ptr())).ok_or("glPixelStoref")?);
    let glPixelStorei_p = transmute::<nn_cv, glPixelStorei_t>(Self::ptr_filter(f(b"glPixelStorei\0".as_ptr())).ok_or("glPixelStorei")?);
    let glPointParameterf_p = transmute::<nn_cv, glPointParameterf_t>(Self::ptr_filter(f(b"glPointParameterf\0".as_ptr())).ok_or("glPointParameterf")?);
    let glPointParameterfv_p = transmute::<nn_cv, glPointParameterfv_t>(Self::ptr_filter(f(b"glPointParameterfv\0".as_ptr())).ok_or("glPointParameterfv")?);
    let glPointParameteri_p = transmute::<nn_cv, glPointParameteri_t>(Self::ptr_filter(f(b"glPointParameteri\0".as_ptr())).ok_or("glPointParameteri")?);
    let glPointParameteriv_p = transmute::<nn_cv, glPointParameteriv_t>(Self::ptr_filter(f(b"glPointParameteriv\0".as_ptr())).ok_or("glPointParameteriv")?);
    let glPointSize_p = transmute::<nn_cv, glPointSize_t>(Self::ptr_filter(f(b"glPointSize\0".as_ptr())).ok_or("glPointSize")?);
    let glPolygonMode_p = transmute::<nn_cv, glPolygonMode_t>(Self::ptr_filter(f(b"glPolygonMode\0".as_ptr())).ok_or("glPolygonMode")?);
    let glPolygonOffset_p = transmute::<nn_cv, glPolygonOffset_t>(Self::ptr_filter(f(b"glPolygonOffset\0".as_ptr())).ok_or("glPolygonOffset")?);
    let glPolygonOffsetClamp_p = transmute::<nn_cv, glPolygonOffsetClamp_t>(Self::ptr_filter(f(b"glPolygonOffsetClamp\0".as_ptr())).ok_or("glPolygonOffsetClamp")?);
    let glPopDebugGroup_p = transmute::<nn_cv, glPopDebugGroup_t>(Self::ptr_filter(f(b"glPopDebugGroup\0".as_ptr())).ok_or("glPopDebugGroup")?);
    let glPrimitiveRestartIndex_p = transmute::<nn_cv, glPrimitiveRestartIndex_t>(Self::ptr_filter(f(b"glPrimitiveRestartIndex\0".as_ptr())).ok_or("glPrimitiveRestartIndex")?);
    let glProgramBinary_p = transmute::<nn_cv, glProgramBinary_t>(Self::ptr_filter(f(b"glProgramBinary\0".as_ptr())).ok_or("glProgramBinary")?);
    let glProgramParameteri_p = transmute::<nn_cv, glProgramParameteri_t>(Self::ptr_filter(f(b"glProgramParameteri\0".as_ptr())).ok_or("glProgramParameteri")?);
    let glProgramUniform1d_p = transmute::<nn_cv, glProgramUniform1d_t>(Self::ptr_filter(f(b"glProgramUniform1d\0".as_ptr())).ok_or("glProgramUniform1d")?);
    let glProgramUniform1dv_p = transmute::<nn_cv, glProgramUniform1dv_t>(Self::ptr_filter(f(b"glProgramUniform1dv\0".as_ptr())).ok_or("glProgramUniform1dv")?);
    let glProgramUniform1f_p = transmute::<nn_cv, glProgramUniform1f_t>(Self::ptr_filter(f(b"glProgramUniform1f\0".as_ptr())).ok_or("glProgramUniform1f")?);
    let glProgramUniform1fv_p = transmute::<nn_cv, glProgramUniform1fv_t>(Self::ptr_filter(f(b"glProgramUniform1fv\0".as_ptr())).ok_or("glProgramUniform1fv")?);
    let glProgramUniform1i_p = transmute::<nn_cv, glProgramUniform1i_t>(Self::ptr_filter(f(b"glProgramUniform1i\0".as_ptr())).ok_or("glProgramUniform1i")?);
    let glProgramUniform1iv_p = transmute::<nn_cv, glProgramUniform1iv_t>(Self::ptr_filter(f(b"glProgramUniform1iv\0".as_ptr())).ok_or("glProgramUniform1iv")?);
    let glProgramUniform1ui_p = transmute::<nn_cv, glProgramUniform1ui_t>(Self::ptr_filter(f(b"glProgramUniform1ui\0".as_ptr())).ok_or("glProgramUniform1ui")?);
    let glProgramUniform1uiv_p = transmute::<nn_cv, glProgramUniform1uiv_t>(Self::ptr_filter(f(b"glProgramUniform1uiv\0".as_ptr())).ok_or("glProgramUniform1uiv")?);
    let glProgramUniform2d_p = transmute::<nn_cv, glProgramUniform2d_t>(Self::ptr_filter(f(b"glProgramUniform2d\0".as_ptr())).ok_or("glProgramUniform2d")?);
    let glProgramUniform2dv_p = transmute::<nn_cv, glProgramUniform2dv_t>(Self::ptr_filter(f(b"glProgramUniform2dv\0".as_ptr())).ok_or("glProgramUniform2dv")?);
    let glProgramUniform2f_p = transmute::<nn_cv, glProgramUniform2f_t>(Self::ptr_filter(f(b"glProgramUniform2f\0".as_ptr())).ok_or("glProgramUniform2f")?);
    let glProgramUniform2fv_p = transmute::<nn_cv, glProgramUniform2fv_t>(Self::ptr_filter(f(b"glProgramUniform2fv\0".as_ptr())).ok_or("glProgramUniform2fv")?);
    let glProgramUniform2i_p = transmute::<nn_cv, glProgramUniform2i_t>(Self::ptr_filter(f(b"glProgramUniform2i\0".as_ptr())).ok_or("glProgramUniform2i")?);
    let glProgramUniform2iv_p = transmute::<nn_cv, glProgramUniform2iv_t>(Self::ptr_filter(f(b"glProgramUniform2iv\0".as_ptr())).ok_or("glProgramUniform2iv")?);
    let glProgramUniform2ui_p = transmute::<nn_cv, glProgramUniform2ui_t>(Self::ptr_filter(f(b"glProgramUniform2ui\0".as_ptr())).ok_or("glProgramUniform2ui")?);
    let glProgramUniform2uiv_p = transmute::<nn_cv, glProgramUniform2uiv_t>(Self::ptr_filter(f(b"glProgramUniform2uiv\0".as_ptr())).ok_or("glProgramUniform2uiv")?);
    let glProgramUniform3d_p = transmute::<nn_cv, glProgramUniform3d_t>(Self::ptr_filter(f(b"glProgramUniform3d\0".as_ptr())).ok_or("glProgramUniform3d")?);
    let glProgramUniform3dv_p = transmute::<nn_cv, glProgramUniform3dv_t>(Self::ptr_filter(f(b"glProgramUniform3dv\0".as_ptr())).ok_or("glProgramUniform3dv")?);
    let glProgramUniform3f_p = transmute::<nn_cv, glProgramUniform3f_t>(Self::ptr_filter(f(b"glProgramUniform3f\0".as_ptr())).ok_or("glProgramUniform3f")?);
    let glProgramUniform3fv_p = transmute::<nn_cv, glProgramUniform3fv_t>(Self::ptr_filter(f(b"glProgramUniform3fv\0".as_ptr())).ok_or("glProgramUniform3fv")?);
    let glProgramUniform3i_p = transmute::<nn_cv, glProgramUniform3i_t>(Self::ptr_filter(f(b"glProgramUniform3i\0".as_ptr())).ok_or("glProgramUniform3i")?);
    let glProgramUniform3iv_p = transmute::<nn_cv, glProgramUniform3iv_t>(Self::ptr_filter(f(b"glProgramUniform3iv\0".as_ptr())).ok_or("glProgramUniform3iv")?);
    let glProgramUniform3ui_p = transmute::<nn_cv, glProgramUniform3ui_t>(Self::ptr_filter(f(b"glProgramUniform3ui\0".as_ptr())).ok_or("glProgramUniform3ui")?);
    let glProgramUniform3uiv_p = transmute::<nn_cv, glProgramUniform3uiv_t>(Self::ptr_filter(f(b"glProgramUniform3uiv\0".as_ptr())).ok_or("glProgramUniform3uiv")?);
    let glProgramUniform4d_p = transmute::<nn_cv, glProgramUniform4d_t>(Self::ptr_filter(f(b"glProgramUniform4d\0".as_ptr())).ok_or("glProgramUniform4d")?);
    let glProgramUniform4dv_p = transmute::<nn_cv, glProgramUniform4dv_t>(Self::ptr_filter(f(b"glProgramUniform4dv\0".as_ptr())).ok_or("glProgramUniform4dv")?);
    let glProgramUniform4f_p = transmute::<nn_cv, glProgramUniform4f_t>(Self::ptr_filter(f(b"glProgramUniform4f\0".as_ptr())).ok_or("glProgramUniform4f")?);
    let glProgramUniform4fv_p = transmute::<nn_cv, glProgramUniform4fv_t>(Self::ptr_filter(f(b"glProgramUniform4fv\0".as_ptr())).ok_or("glProgramUniform4fv")?);
    let glProgramUniform4i_p = transmute::<nn_cv, glProgramUniform4i_t>(Self::ptr_filter(f(b"glProgramUniform4i\0".as_ptr())).ok_or("glProgramUniform4i")?);
    let glProgramUniform4iv_p = transmute::<nn_cv, glProgramUniform4iv_t>(Self::ptr_filter(f(b"glProgramUniform4iv\0".as_ptr())).ok_or("glProgramUniform4iv")?);
    let glProgramUniform4ui_p = transmute::<nn_cv, glProgramUniform4ui_t>(Self::ptr_filter(f(b"glProgramUniform4ui\0".as_ptr())).ok_or("glProgramUniform4ui")?);
    let glProgramUniform4uiv_p = transmute::<nn_cv, glProgramUniform4uiv_t>(Self::ptr_filter(f(b"glProgramUniform4uiv\0".as_ptr())).ok_or("glProgramUniform4uiv")?);
    let glProgramUniformMatrix2dv_p = transmute::<nn_cv, glProgramUniformMatrix2dv_t>(Self::ptr_filter(f(b"glProgramUniformMatrix2dv\0".as_ptr())).ok_or("glProgramUniformMatrix2dv")?);
    let glProgramUniformMatrix2fv_p = transmute::<nn_cv, glProgramUniformMatrix2fv_t>(Self::ptr_filter(f(b"glProgramUniformMatrix2fv\0".as_ptr())).ok_or("glProgramUniformMatrix2fv")?);
    let glProgramUniformMatrix2x3dv_p = transmute::<nn_cv, glProgramUniformMatrix2x3dv_t>(Self::ptr_filter(f(b"glProgramUniformMatrix2x3dv\0".as_ptr())).ok_or("glProgramUniformMatrix2x3dv")?);
    let glProgramUniformMatrix2x3fv_p = transmute::<nn_cv, glProgramUniformMatrix2x3fv_t>(Self::ptr_filter(f(b"glProgramUniformMatrix2x3fv\0".as_ptr())).ok_or("glProgramUniformMatrix2x3fv")?);
    let glProgramUniformMatrix2x4dv_p = transmute::<nn_cv, glProgramUniformMatrix2x4dv_t>(Self::ptr_filter(f(b"glProgramUniformMatrix2x4dv\0".as_ptr())).ok_or("glProgramUniformMatrix2x4dv")?);
    let glProgramUniformMatrix2x4fv_p = transmute::<nn_cv, glProgramUniformMatrix2x4fv_t>(Self::ptr_filter(f(b"glProgramUniformMatrix2x4fv\0".as_ptr())).ok_or("glProgramUniformMatrix2x4fv")?);
    let glProgramUniformMatrix3dv_p = transmute::<nn_cv, glProgramUniformMatrix3dv_t>(Self::ptr_filter(f(b"glProgramUniformMatrix3dv\0".as_ptr())).ok_or("glProgramUniformMatrix3dv")?);
    let glProgramUniformMatrix3fv_p = transmute::<nn_cv, glProgramUniformMatrix3fv_t>(Self::ptr_filter(f(b"glProgramUniformMatrix3fv\0".as_ptr())).ok_or("glProgramUniformMatrix3fv")?);
    let glProgramUniformMatrix3x2dv_p = transmute::<nn_cv, glProgramUniformMatrix3x2dv_t>(Self::ptr_filter(f(b"glProgramUniformMatrix3x2dv\0".as_ptr())).ok_or("glProgramUniformMatrix3x2dv")?);
    let glProgramUniformMatrix3x2fv_p = transmute::<nn_cv, glProgramUniformMatrix3x2fv_t>(Self::ptr_filter(f(b"glProgramUniformMatrix3x2fv\0".as_ptr())).ok_or("glProgramUniformMatrix3x2fv")?);
    let glProgramUniformMatrix3x4dv_p = transmute::<nn_cv, glProgramUniformMatrix3x4dv_t>(Self::ptr_filter(f(b"glProgramUniformMatrix3x4dv\0".as_ptr())).ok_or("glProgramUniformMatrix3x4dv")?);
    let glProgramUniformMatrix3x4fv_p = transmute::<nn_cv, glProgramUniformMatrix3x4fv_t>(Self::ptr_filter(f(b"glProgramUniformMatrix3x4fv\0".as_ptr())).ok_or("glProgramUniformMatrix3x4fv")?);
    let glProgramUniformMatrix4dv_p = transmute::<nn_cv, glProgramUniformMatrix4dv_t>(Self::ptr_filter(f(b"glProgramUniformMatrix4dv\0".as_ptr())).ok_or("glProgramUniformMatrix4dv")?);
    let glProgramUniformMatrix4fv_p = transmute::<nn_cv, glProgramUniformMatrix4fv_t>(Self::ptr_filter(f(b"glProgramUniformMatrix4fv\0".as_ptr())).ok_or("glProgramUniformMatrix4fv")?);
    let glProgramUniformMatrix4x2dv_p = transmute::<nn_cv, glProgramUniformMatrix4x2dv_t>(Self::ptr_filter(f(b"glProgramUniformMatrix4x2dv\0".as_ptr())).ok_or("glProgramUniformMatrix4x2dv")?);
    let glProgramUniformMatrix4x2fv_p = transmute::<nn_cv, glProgramUniformMatrix4x2fv_t>(Self::ptr_filter(f(b"glProgramUniformMatrix4x2fv\0".as_ptr())).ok_or("glProgramUniformMatrix4x2fv")?);
    let glProgramUniformMatrix4x3dv_p = transmute::<nn_cv, glProgramUniformMatrix4x3dv_t>(Self::ptr_filter(f(b"glProgramUniformMatrix4x3dv\0".as_ptr())).ok_or("glProgramUniformMatrix4x3dv")?);
    let glProgramUniformMatrix4x3fv_p = transmute::<nn_cv, glProgramUniformMatrix4x3fv_t>(Self::ptr_filter(f(b"glProgramUniformMatrix4x3fv\0".as_ptr())).ok_or("glProgramUniformMatrix4x3fv")?);
    let glProvokingVertex_p = transmute::<nn_cv, glProvokingVertex_t>(Self::ptr_filter(f(b"glProvokingVertex\0".as_ptr())).ok_or("glProvokingVertex")?);
    let glPushDebugGroup_p = transmute::<nn_cv, glPushDebugGroup_t>(Self::ptr_filter(f(b"glPushDebugGroup\0".as_ptr())).ok_or("glPushDebugGroup")?);
    let glQueryCounter_p = transmute::<nn_cv, glQueryCounter_t>(Self::ptr_filter(f(b"glQueryCounter\0".as_ptr())).ok_or("glQueryCounter")?);
    let glReadBuffer_p = transmute::<nn_cv, glReadBuffer_t>(Self::ptr_filter(f(b"glReadBuffer\0".as_ptr())).ok_or("glReadBuffer")?);
    let glReadPixels_p = transmute::<nn_cv, glReadPixels_t>(Self::ptr_filter(f(b"glReadPixels\0".as_ptr())).ok_or("glReadPixels")?);
    let glReadnPixels_p = transmute::<nn_cv, glReadnPixels_t>(Self::ptr_filter(f(b"glReadnPixels\0".as_ptr())).ok_or("glReadnPixels")?);
    let glReleaseShaderCompiler_p = transmute::<nn_cv, glReleaseShaderCompiler_t>(Self::ptr_filter(f(b"glReleaseShaderCompiler\0".as_ptr())).ok_or("glReleaseShaderCompiler")?);
    let glRenderbufferStorage_p = transmute::<nn_cv, glRenderbufferStorage_t>(Self::ptr_filter(f(b"glRenderbufferStorage\0".as_ptr())).ok_or("glRenderbufferStorage")?);
    let glRenderbufferStorageMultisample_p = transmute::<nn_cv, glRenderbufferStorageMultisample_t>(Self::ptr_filter(f(b"glRenderbufferStorageMultisample\0".as_ptr())).ok_or("glRenderbufferStorageMultisample")?);
    let glResumeTransformFeedback_p = transmute::<nn_cv, glResumeTransformFeedback_t>(Self::ptr_filter(f(b"glResumeTransformFeedback\0".as_ptr())).ok_or("glResumeTransformFeedback")?);
    let glSampleCoverage_p = transmute::<nn_cv, glSampleCoverage_t>(Self::ptr_filter(f(b"glSampleCoverage\0".as_ptr())).ok_or("glSampleCoverage")?);
    let glSampleMaski_p = transmute::<nn_cv, glSampleMaski_t>(Self::ptr_filter(f(b"glSampleMaski\0".as_ptr())).ok_or("glSampleMaski")?);
    let glSamplerParameterIiv_p = transmute::<nn_cv, glSamplerParameterIiv_t>(Self::ptr_filter(f(b"glSamplerParameterIiv\0".as_ptr())).ok_or("glSamplerParameterIiv")?);
    let glSamplerParameterIuiv_p = transmute::<nn_cv, glSamplerParameterIuiv_t>(Self::ptr_filter(f(b"glSamplerParameterIuiv\0".as_ptr())).ok_or("glSamplerParameterIuiv")?);
    let glSamplerParameterf_p = transmute::<nn_cv, glSamplerParameterf_t>(Self::ptr_filter(f(b"glSamplerParameterf\0".as_ptr())).ok_or("glSamplerParameterf")?);
    let glSamplerParameterfv_p = transmute::<nn_cv, glSamplerParameterfv_t>(Self::ptr_filter(f(b"glSamplerParameterfv\0".as_ptr())).ok_or("glSamplerParameterfv")?);
    let glSamplerParameteri_p = transmute::<nn_cv, glSamplerParameteri_t>(Self::ptr_filter(f(b"glSamplerParameteri\0".as_ptr())).ok_or("glSamplerParameteri")?);
    let glSamplerParameteriv_p = transmute::<nn_cv, glSamplerParameteriv_t>(Self::ptr_filter(f(b"glSamplerParameteriv\0".as_ptr())).ok_or("glSamplerParameteriv")?);
    let glScissor_p = transmute::<nn_cv, glScissor_t>(Self::ptr_filter(f(b"glScissor\0".as_ptr())).ok_or("glScissor")?);
    let glScissorArrayv_p = transmute::<nn_cv, glScissorArrayv_t>(Self::ptr_filter(f(b"glScissorArrayv\0".as_ptr())).ok_or("glScissorArrayv")?);
    let glScissorIndexed_p = transmute::<nn_cv, glScissorIndexed_t>(Self::ptr_filter(f(b"glScissorIndexed\0".as_ptr())).ok_or("glScissorIndexed")?);
    let glScissorIndexedv_p = transmute::<nn_cv, glScissorIndexedv_t>(Self::ptr_filter(f(b"glScissorIndexedv\0".as_ptr())).ok_or("glScissorIndexedv")?);
    let glSecondaryColorP3ui_p = transmute::<nn_cv, glSecondaryColorP3ui_t>(Self::ptr_filter(f(b"glSecondaryColorP3ui\0".as_ptr())).ok_or("glSecondaryColorP3ui")?);
    let glSecondaryColorP3uiv_p = transmute::<nn_cv, glSecondaryColorP3uiv_t>(Self::ptr_filter(f(b"glSecondaryColorP3uiv\0".as_ptr())).ok_or("glSecondaryColorP3uiv")?);
    let glShaderBinary_p = transmute::<nn_cv, glShaderBinary_t>(Self::ptr_filter(f(b"glShaderBinary\0".as_ptr())).ok_or("glShaderBinary")?);
    let glShaderSource_p = transmute::<nn_cv, glShaderSource_t>(Self::ptr_filter(f(b"glShaderSource\0".as_ptr())).ok_or("glShaderSource")?);
    let glShaderStorageBlockBinding_p = transmute::<nn_cv, glShaderStorageBlockBinding_t>(Self::ptr_filter(f(b"glShaderStorageBlockBinding\0".as_ptr())).ok_or("glShaderStorageBlockBinding")?);
    let glSpecializeShader_p = transmute::<nn_cv, glSpecializeShader_t>(Self::ptr_filter(f(b"glSpecializeShader\0".as_ptr())).ok_or("glSpecializeShader")?);
    let glStencilFunc_p = transmute::<nn_cv, glStencilFunc_t>(Self::ptr_filter(f(b"glStencilFunc\0".as_ptr())).ok_or("glStencilFunc")?);
    let glStencilFuncSeparate_p = transmute::<nn_cv, glStencilFuncSeparate_t>(Self::ptr_filter(f(b"glStencilFuncSeparate\0".as_ptr())).ok_or("glStencilFuncSeparate")?);
    let glStencilMask_p = transmute::<nn_cv, glStencilMask_t>(Self::ptr_filter(f(b"glStencilMask\0".as_ptr())).ok_or("glStencilMask")?);
    let glStencilMaskSeparate_p = transmute::<nn_cv, glStencilMaskSeparate_t>(Self::ptr_filter(f(b"glStencilMaskSeparate\0".as_ptr())).ok_or("glStencilMaskSeparate")?);
    let glStencilOp_p = transmute::<nn_cv, glStencilOp_t>(Self::ptr_filter(f(b"glStencilOp\0".as_ptr())).ok_or("glStencilOp")?);
    let glStencilOpSeparate_p = transmute::<nn_cv, glStencilOpSeparate_t>(Self::ptr_filter(f(b"glStencilOpSeparate\0".as_ptr())).ok_or("glStencilOpSeparate")?);
    let glTexBuffer_p = transmute::<nn_cv, glTexBuffer_t>(Self::ptr_filter(f(b"glTexBuffer\0".as_ptr())).ok_or("glTexBuffer")?);
    let glTexBufferRange_p = transmute::<nn_cv, glTexBufferRange_t>(Self::ptr_filter(f(b"glTexBufferRange\0".as_ptr())).ok_or("glTexBufferRange")?);
    let glTexCoordP1ui_p = transmute::<nn_cv, glTexCoordP1ui_t>(Self::ptr_filter(f(b"glTexCoordP1ui\0".as_ptr())).ok_or("glTexCoordP1ui")?);
    let glTexCoordP1uiv_p = transmute::<nn_cv, glTexCoordP1uiv_t>(Self::ptr_filter(f(b"glTexCoordP1uiv\0".as_ptr())).ok_or("glTexCoordP1uiv")?);
    let glTexCoordP2ui_p = transmute::<nn_cv, glTexCoordP2ui_t>(Self::ptr_filter(f(b"glTexCoordP2ui\0".as_ptr())).ok_or("glTexCoordP2ui")?);
    let glTexCoordP2uiv_p = transmute::<nn_cv, glTexCoordP2uiv_t>(Self::ptr_filter(f(b"glTexCoordP2uiv\0".as_ptr())).ok_or("glTexCoordP2uiv")?);
    let glTexCoordP3ui_p = transmute::<nn_cv, glTexCoordP3ui_t>(Self::ptr_filter(f(b"glTexCoordP3ui\0".as_ptr())).ok_or("glTexCoordP3ui")?);
    let glTexCoordP3uiv_p = transmute::<nn_cv, glTexCoordP3uiv_t>(Self::ptr_filter(f(b"glTexCoordP3uiv\0".as_ptr())).ok_or("glTexCoordP3uiv")?);
    let glTexCoordP4ui_p = transmute::<nn_cv, glTexCoordP4ui_t>(Self::ptr_filter(f(b"glTexCoordP4ui\0".as_ptr())).ok_or("glTexCoordP4ui")?);
    let glTexCoordP4uiv_p = transmute::<nn_cv, glTexCoordP4uiv_t>(Self::ptr_filter(f(b"glTexCoordP4uiv\0".as_ptr())).ok_or("glTexCoordP4uiv")?);
    let glTexImage1D_p = transmute::<nn_cv, glTexImage1D_t>(Self::ptr_filter(f(b"glTexImage1D\0".as_ptr())).ok_or("glTexImage1D")?);
    let glTexImage2D_p = transmute::<nn_cv, glTexImage2D_t>(Self::ptr_filter(f(b"glTexImage2D\0".as_ptr())).ok_or("glTexImage2D")?);
    let glTexImage2DMultisample_p = transmute::<nn_cv, glTexImage2DMultisample_t>(Self::ptr_filter(f(b"glTexImage2DMultisample\0".as_ptr())).ok_or("glTexImage2DMultisample")?);
    let glTexImage3D_p = transmute::<nn_cv, glTexImage3D_t>(Self::ptr_filter(f(b"glTexImage3D\0".as_ptr())).ok_or("glTexImage3D")?);
    let glTexImage3DMultisample_p = transmute::<nn_cv, glTexImage3DMultisample_t>(Self::ptr_filter(f(b"glTexImage3DMultisample\0".as_ptr())).ok_or("glTexImage3DMultisample")?);
    let glTexParameterIiv_p = transmute::<nn_cv, glTexParameterIiv_t>(Self::ptr_filter(f(b"glTexParameterIiv\0".as_ptr())).ok_or("glTexParameterIiv")?);
    let glTexParameterIuiv_p = transmute::<nn_cv, glTexParameterIuiv_t>(Self::ptr_filter(f(b"glTexParameterIuiv\0".as_ptr())).ok_or("glTexParameterIuiv")?);
    let glTexParameterf_p = transmute::<nn_cv, glTexParameterf_t>(Self::ptr_filter(f(b"glTexParameterf\0".as_ptr())).ok_or("glTexParameterf")?);
    let glTexParameterfv_p = transmute::<nn_cv, glTexParameterfv_t>(Self::ptr_filter(f(b"glTexParameterfv\0".as_ptr())).ok_or("glTexParameterfv")?);
    let glTexParameteri_p = transmute::<nn_cv, glTexParameteri_t>(Self::ptr_filter(f(b"glTexParameteri\0".as_ptr())).ok_or("glTexParameteri")?);
    let glTexParameteriv_p = transmute::<nn_cv, glTexParameteriv_t>(Self::ptr_filter(f(b"glTexParameteriv\0".as_ptr())).ok_or("glTexParameteriv")?);
    let glTexStorage1D_p = transmute::<nn_cv, glTexStorage1D_t>(Self::ptr_filter(f(b"glTexStorage1D\0".as_ptr())).ok_or("glTexStorage1D")?);
    let glTexStorage2D_p = transmute::<nn_cv, glTexStorage2D_t>(Self::ptr_filter(f(b"glTexStorage2D\0".as_ptr())).ok_or("glTexStorage2D")?);
    let glTexStorage2DMultisample_p = transmute::<nn_cv, glTexStorage2DMultisample_t>(Self::ptr_filter(f(b"glTexStorage2DMultisample\0".as_ptr())).ok_or("glTexStorage2DMultisample")?);
    let glTexStorage3D_p = transmute::<nn_cv, glTexStorage3D_t>(Self::ptr_filter(f(b"glTexStorage3D\0".as_ptr())).ok_or("glTexStorage3D")?);
    let glTexStorage3DMultisample_p = transmute::<nn_cv, glTexStorage3DMultisample_t>(Self::ptr_filter(f(b"glTexStorage3DMultisample\0".as_ptr())).ok_or("glTexStorage3DMultisample")?);
    let glTexSubImage1D_p = transmute::<nn_cv, glTexSubImage1D_t>(Self::ptr_filter(f(b"glTexSubImage1D\0".as_ptr())).ok_or("glTexSubImage1D")?);
    let glTexSubImage2D_p = transmute::<nn_cv, glTexSubImage2D_t>(Self::ptr_filter(f(b"glTexSubImage2D\0".as_ptr())).ok_or("glTexSubImage2D")?);
    let glTexSubImage3D_p = transmute::<nn_cv, glTexSubImage3D_t>(Self::ptr_filter(f(b"glTexSubImage3D\0".as_ptr())).ok_or("glTexSubImage3D")?);
    let glTextureBarrier_p = transmute::<nn_cv, glTextureBarrier_t>(Self::ptr_filter(f(b"glTextureBarrier\0".as_ptr())).ok_or("glTextureBarrier")?);
    let glTextureBuffer_p = transmute::<nn_cv, glTextureBuffer_t>(Self::ptr_filter(f(b"glTextureBuffer\0".as_ptr())).ok_or("glTextureBuffer")?);
    let glTextureBufferRange_p = transmute::<nn_cv, glTextureBufferRange_t>(Self::ptr_filter(f(b"glTextureBufferRange\0".as_ptr())).ok_or("glTextureBufferRange")?);
    let glTextureParameterIiv_p = transmute::<nn_cv, glTextureParameterIiv_t>(Self::ptr_filter(f(b"glTextureParameterIiv\0".as_ptr())).ok_or("glTextureParameterIiv")?);
    let glTextureParameterIuiv_p = transmute::<nn_cv, glTextureParameterIuiv_t>(Self::ptr_filter(f(b"glTextureParameterIuiv\0".as_ptr())).ok_or("glTextureParameterIuiv")?);
    let glTextureParameterf_p = transmute::<nn_cv, glTextureParameterf_t>(Self::ptr_filter(f(b"glTextureParameterf\0".as_ptr())).ok_or("glTextureParameterf")?);
    let glTextureParameterfv_p = transmute::<nn_cv, glTextureParameterfv_t>(Self::ptr_filter(f(b"glTextureParameterfv\0".as_ptr())).ok_or("glTextureParameterfv")?);
    let glTextureParameteri_p = transmute::<nn_cv, glTextureParameteri_t>(Self::ptr_filter(f(b"glTextureParameteri\0".as_ptr())).ok_or("glTextureParameteri")?);
    let glTextureParameteriv_p = transmute::<nn_cv, glTextureParameteriv_t>(Self::ptr_filter(f(b"glTextureParameteriv\0".as_ptr())).ok_or("glTextureParameteriv")?);
    let glTextureStorage1D_p = transmute::<nn_cv, glTextureStorage1D_t>(Self::ptr_filter(f(b"glTextureStorage1D\0".as_ptr())).ok_or("glTextureStorage1D")?);
    let glTextureStorage2D_p = transmute::<nn_cv, glTextureStorage2D_t>(Self::ptr_filter(f(b"glTextureStorage2D\0".as_ptr())).ok_or("glTextureStorage2D")?);
    let glTextureStorage2DMultisample_p = transmute::<nn_cv, glTextureStorage2DMultisample_t>(Self::ptr_filter(f(b"glTextureStorage2DMultisample\0".as_ptr())).ok_or("glTextureStorage2DMultisample")?);
    let glTextureStorage3D_p = transmute::<nn_cv, glTextureStorage3D_t>(Self::ptr_filter(f(b"glTextureStorage3D\0".as_ptr())).ok_or("glTextureStorage3D")?);
    let glTextureStorage3DMultisample_p = transmute::<nn_cv, glTextureStorage3DMultisample_t>(Self::ptr_filter(f(b"glTextureStorage3DMultisample\0".as_ptr())).ok_or("glTextureStorage3DMultisample")?);
    let glTextureSubImage1D_p = transmute::<nn_cv, glTextureSubImage1D_t>(Self::ptr_filter(f(b"glTextureSubImage1D\0".as_ptr())).ok_or("glTextureSubImage1D")?);
    let glTextureSubImage2D_p = transmute::<nn_cv, glTextureSubImage2D_t>(Self::ptr_filter(f(b"glTextureSubImage2D\0".as_ptr())).ok_or("glTextureSubImage2D")?);
    let glTextureSubImage3D_p = transmute::<nn_cv, glTextureSubImage3D_t>(Self::ptr_filter(f(b"glTextureSubImage3D\0".as_ptr())).ok_or("glTextureSubImage3D")?);
    let glTextureView_p = transmute::<nn_cv, glTextureView_t>(Self::ptr_filter(f(b"glTextureView\0".as_ptr())).ok_or("glTextureView")?);
    let glTransformFeedbackBufferBase_p = transmute::<nn_cv, glTransformFeedbackBufferBase_t>(Self::ptr_filter(f(b"glTransformFeedbackBufferBase\0".as_ptr())).ok_or("glTransformFeedbackBufferBase")?);
    let glTransformFeedbackBufferRange_p = transmute::<nn_cv, glTransformFeedbackBufferRange_t>(Self::ptr_filter(f(b"glTransformFeedbackBufferRange\0".as_ptr())).ok_or("glTransformFeedbackBufferRange")?);
    let glTransformFeedbackVaryings_p = transmute::<nn_cv, glTransformFeedbackVaryings_t>(Self::ptr_filter(f(b"glTransformFeedbackVaryings\0".as_ptr())).ok_or("glTransformFeedbackVaryings")?);
    let glUniform1d_p = transmute::<nn_cv, glUniform1d_t>(Self::ptr_filter(f(b"glUniform1d\0".as_ptr())).ok_or("glUniform1d")?);
    let glUniform1dv_p = transmute::<nn_cv, glUniform1dv_t>(Self::ptr_filter(f(b"glUniform1dv\0".as_ptr())).ok_or("glUniform1dv")?);
    let glUniform1f_p = transmute::<nn_cv, glUniform1f_t>(Self::ptr_filter(f(b"glUniform1f\0".as_ptr())).ok_or("glUniform1f")?);
    let glUniform1fv_p = transmute::<nn_cv, glUniform1fv_t>(Self::ptr_filter(f(b"glUniform1fv\0".as_ptr())).ok_or("glUniform1fv")?);
    let glUniform1i_p = transmute::<nn_cv, glUniform1i_t>(Self::ptr_filter(f(b"glUniform1i\0".as_ptr())).ok_or("glUniform1i")?);
    let glUniform1iv_p = transmute::<nn_cv, glUniform1iv_t>(Self::ptr_filter(f(b"glUniform1iv\0".as_ptr())).ok_or("glUniform1iv")?);
    let glUniform1ui_p = transmute::<nn_cv, glUniform1ui_t>(Self::ptr_filter(f(b"glUniform1ui\0".as_ptr())).ok_or("glUniform1ui")?);
    let glUniform1uiv_p = transmute::<nn_cv, glUniform1uiv_t>(Self::ptr_filter(f(b"glUniform1uiv\0".as_ptr())).ok_or("glUniform1uiv")?);
    let glUniform2d_p = transmute::<nn_cv, glUniform2d_t>(Self::ptr_filter(f(b"glUniform2d\0".as_ptr())).ok_or("glUniform2d")?);
    let glUniform2dv_p = transmute::<nn_cv, glUniform2dv_t>(Self::ptr_filter(f(b"glUniform2dv\0".as_ptr())).ok_or("glUniform2dv")?);
    let glUniform2f_p = transmute::<nn_cv, glUniform2f_t>(Self::ptr_filter(f(b"glUniform2f\0".as_ptr())).ok_or("glUniform2f")?);
    let glUniform2fv_p = transmute::<nn_cv, glUniform2fv_t>(Self::ptr_filter(f(b"glUniform2fv\0".as_ptr())).ok_or("glUniform2fv")?);
    let glUniform2i_p = transmute::<nn_cv, glUniform2i_t>(Self::ptr_filter(f(b"glUniform2i\0".as_ptr())).ok_or("glUniform2i")?);
    let glUniform2iv_p = transmute::<nn_cv, glUniform2iv_t>(Self::ptr_filter(f(b"glUniform2iv\0".as_ptr())).ok_or("glUniform2iv")?);
    let glUniform2ui_p = transmute::<nn_cv, glUniform2ui_t>(Self::ptr_filter(f(b"glUniform2ui\0".as_ptr())).ok_or("glUniform2ui")?);
    let glUniform2uiv_p = transmute::<nn_cv, glUniform2uiv_t>(Self::ptr_filter(f(b"glUniform2uiv\0".as_ptr())).ok_or("glUniform2uiv")?);
    let glUniform3d_p = transmute::<nn_cv, glUniform3d_t>(Self::ptr_filter(f(b"glUniform3d\0".as_ptr())).ok_or("glUniform3d")?);
    let glUniform3dv_p = transmute::<nn_cv, glUniform3dv_t>(Self::ptr_filter(f(b"glUniform3dv\0".as_ptr())).ok_or("glUniform3dv")?);
    let glUniform3f_p = transmute::<nn_cv, glUniform3f_t>(Self::ptr_filter(f(b"glUniform3f\0".as_ptr())).ok_or("glUniform3f")?);
    let glUniform3fv_p = transmute::<nn_cv, glUniform3fv_t>(Self::ptr_filter(f(b"glUniform3fv\0".as_ptr())).ok_or("glUniform3fv")?);
    let glUniform3i_p = transmute::<nn_cv, glUniform3i_t>(Self::ptr_filter(f(b"glUniform3i\0".as_ptr())).ok_or("glUniform3i")?);
    let glUniform3iv_p = transmute::<nn_cv, glUniform3iv_t>(Self::ptr_filter(f(b"glUniform3iv\0".as_ptr())).ok_or("glUniform3iv")?);
    let glUniform3ui_p = transmute::<nn_cv, glUniform3ui_t>(Self::ptr_filter(f(b"glUniform3ui\0".as_ptr())).ok_or("glUniform3ui")?);
    let glUniform3uiv_p = transmute::<nn_cv, glUniform3uiv_t>(Self::ptr_filter(f(b"glUniform3uiv\0".as_ptr())).ok_or("glUniform3uiv")?);
    let glUniform4d_p = transmute::<nn_cv, glUniform4d_t>(Self::ptr_filter(f(b"glUniform4d\0".as_ptr())).ok_or("glUniform4d")?);
    let glUniform4dv_p = transmute::<nn_cv, glUniform4dv_t>(Self::ptr_filter(f(b"glUniform4dv\0".as_ptr())).ok_or("glUniform4dv")?);
    let glUniform4f_p = transmute::<nn_cv, glUniform4f_t>(Self::ptr_filter(f(b"glUniform4f\0".as_ptr())).ok_or("glUniform4f")?);
    let glUniform4fv_p = transmute::<nn_cv, glUniform4fv_t>(Self::ptr_filter(f(b"glUniform4fv\0".as_ptr())).ok_or("glUniform4fv")?);
    let glUniform4i_p = transmute::<nn_cv, glUniform4i_t>(Self::ptr_filter(f(b"glUniform4i\0".as_ptr())).ok_or("glUniform4i")?);
    let glUniform4iv_p = transmute::<nn_cv, glUniform4iv_t>(Self::ptr_filter(f(b"glUniform4iv\0".as_ptr())).ok_or("glUniform4iv")?);
    let glUniform4ui_p = transmute::<nn_cv, glUniform4ui_t>(Self::ptr_filter(f(b"glUniform4ui\0".as_ptr())).ok_or("glUniform4ui")?);
    let glUniform4uiv_p = transmute::<nn_cv, glUniform4uiv_t>(Self::ptr_filter(f(b"glUniform4uiv\0".as_ptr())).ok_or("glUniform4uiv")?);
    let glUniformBlockBinding_p = transmute::<nn_cv, glUniformBlockBinding_t>(Self::ptr_filter(f(b"glUniformBlockBinding\0".as_ptr())).ok_or("glUniformBlockBinding")?);
    let glUniformMatrix2dv_p = transmute::<nn_cv, glUniformMatrix2dv_t>(Self::ptr_filter(f(b"glUniformMatrix2dv\0".as_ptr())).ok_or("glUniformMatrix2dv")?);
    let glUniformMatrix2fv_p = transmute::<nn_cv, glUniformMatrix2fv_t>(Self::ptr_filter(f(b"glUniformMatrix2fv\0".as_ptr())).ok_or("glUniformMatrix2fv")?);
    let glUniformMatrix2x3dv_p = transmute::<nn_cv, glUniformMatrix2x3dv_t>(Self::ptr_filter(f(b"glUniformMatrix2x3dv\0".as_ptr())).ok_or("glUniformMatrix2x3dv")?);
    let glUniformMatrix2x3fv_p = transmute::<nn_cv, glUniformMatrix2x3fv_t>(Self::ptr_filter(f(b"glUniformMatrix2x3fv\0".as_ptr())).ok_or("glUniformMatrix2x3fv")?);
    let glUniformMatrix2x4dv_p = transmute::<nn_cv, glUniformMatrix2x4dv_t>(Self::ptr_filter(f(b"glUniformMatrix2x4dv\0".as_ptr())).ok_or("glUniformMatrix2x4dv")?);
    let glUniformMatrix2x4fv_p = transmute::<nn_cv, glUniformMatrix2x4fv_t>(Self::ptr_filter(f(b"glUniformMatrix2x4fv\0".as_ptr())).ok_or("glUniformMatrix2x4fv")?);
    let glUniformMatrix3dv_p = transmute::<nn_cv, glUniformMatrix3dv_t>(Self::ptr_filter(f(b"glUniformMatrix3dv\0".as_ptr())).ok_or("glUniformMatrix3dv")?);
    let glUniformMatrix3fv_p = transmute::<nn_cv, glUniformMatrix3fv_t>(Self::ptr_filter(f(b"glUniformMatrix3fv\0".as_ptr())).ok_or("glUniformMatrix3fv")?);
    let glUniformMatrix3x2dv_p = transmute::<nn_cv, glUniformMatrix3x2dv_t>(Self::ptr_filter(f(b"glUniformMatrix3x2dv\0".as_ptr())).ok_or("glUniformMatrix3x2dv")?);
    let glUniformMatrix3x2fv_p = transmute::<nn_cv, glUniformMatrix3x2fv_t>(Self::ptr_filter(f(b"glUniformMatrix3x2fv\0".as_ptr())).ok_or("glUniformMatrix3x2fv")?);
    let glUniformMatrix3x4dv_p = transmute::<nn_cv, glUniformMatrix3x4dv_t>(Self::ptr_filter(f(b"glUniformMatrix3x4dv\0".as_ptr())).ok_or("glUniformMatrix3x4dv")?);
    let glUniformMatrix3x4fv_p = transmute::<nn_cv, glUniformMatrix3x4fv_t>(Self::ptr_filter(f(b"glUniformMatrix3x4fv\0".as_ptr())).ok_or("glUniformMatrix3x4fv")?);
    let glUniformMatrix4dv_p = transmute::<nn_cv, glUniformMatrix4dv_t>(Self::ptr_filter(f(b"glUniformMatrix4dv\0".as_ptr())).ok_or("glUniformMatrix4dv")?);
    let glUniformMatrix4fv_p = transmute::<nn_cv, glUniformMatrix4fv_t>(Self::ptr_filter(f(b"glUniformMatrix4fv\0".as_ptr())).ok_or("glUniformMatrix4fv")?);
    let glUniformMatrix4x2dv_p = transmute::<nn_cv, glUniformMatrix4x2dv_t>(Self::ptr_filter(f(b"glUniformMatrix4x2dv\0".as_ptr())).ok_or("glUniformMatrix4x2dv")?);
    let glUniformMatrix4x2fv_p = transmute::<nn_cv, glUniformMatrix4x2fv_t>(Self::ptr_filter(f(b"glUniformMatrix4x2fv\0".as_ptr())).ok_or("glUniformMatrix4x2fv")?);
    let glUniformMatrix4x3dv_p = transmute::<nn_cv, glUniformMatrix4x3dv_t>(Self::ptr_filter(f(b"glUniformMatrix4x3dv\0".as_ptr())).ok_or("glUniformMatrix4x3dv")?);
    let glUniformMatrix4x3fv_p = transmute::<nn_cv, glUniformMatrix4x3fv_t>(Self::ptr_filter(f(b"glUniformMatrix4x3fv\0".as_ptr())).ok_or("glUniformMatrix4x3fv")?);
    let glUniformSubroutinesuiv_p = transmute::<nn_cv, glUniformSubroutinesuiv_t>(Self::ptr_filter(f(b"glUniformSubroutinesuiv\0".as_ptr())).ok_or("glUniformSubroutinesuiv")?);
    let glUnmapBuffer_p = transmute::<nn_cv, glUnmapBuffer_t>(Self::ptr_filter(f(b"glUnmapBuffer\0".as_ptr())).ok_or("glUnmapBuffer")?);
    let glUnmapNamedBuffer_p = transmute::<nn_cv, glUnmapNamedBuffer_t>(Self::ptr_filter(f(b"glUnmapNamedBuffer\0".as_ptr())).ok_or("glUnmapNamedBuffer")?);
    let glUseProgram_p = transmute::<nn_cv, glUseProgram_t>(Self::ptr_filter(f(b"glUseProgram\0".as_ptr())).ok_or("glUseProgram")?);
    let glUseProgramStages_p = transmute::<nn_cv, glUseProgramStages_t>(Self::ptr_filter(f(b"glUseProgramStages\0".as_ptr())).ok_or("glUseProgramStages")?);
    let glValidateProgram_p = transmute::<nn_cv, glValidateProgram_t>(Self::ptr_filter(f(b"glValidateProgram\0".as_ptr())).ok_or("glValidateProgram")?);
    let glValidateProgramPipeline_p = transmute::<nn_cv, glValidateProgramPipeline_t>(Self::ptr_filter(f(b"glValidateProgramPipeline\0".as_ptr())).ok_or("glValidateProgramPipeline")?);
    let glVertexArrayAttribBinding_p = transmute::<nn_cv, glVertexArrayAttribBinding_t>(Self::ptr_filter(f(b"glVertexArrayAttribBinding\0".as_ptr())).ok_or("glVertexArrayAttribBinding")?);
    let glVertexArrayAttribFormat_p = transmute::<nn_cv, glVertexArrayAttribFormat_t>(Self::ptr_filter(f(b"glVertexArrayAttribFormat\0".as_ptr())).ok_or("glVertexArrayAttribFormat")?);
    let glVertexArrayAttribIFormat_p = transmute::<nn_cv, glVertexArrayAttribIFormat_t>(Self::ptr_filter(f(b"glVertexArrayAttribIFormat\0".as_ptr())).ok_or("glVertexArrayAttribIFormat")?);
    let glVertexArrayAttribLFormat_p = transmute::<nn_cv, glVertexArrayAttribLFormat_t>(Self::ptr_filter(f(b"glVertexArrayAttribLFormat\0".as_ptr())).ok_or("glVertexArrayAttribLFormat")?);
    let glVertexArrayBindingDivisor_p = transmute::<nn_cv, glVertexArrayBindingDivisor_t>(Self::ptr_filter(f(b"glVertexArrayBindingDivisor\0".as_ptr())).ok_or("glVertexArrayBindingDivisor")?);
    let glVertexArrayElementBuffer_p = transmute::<nn_cv, glVertexArrayElementBuffer_t>(Self::ptr_filter(f(b"glVertexArrayElementBuffer\0".as_ptr())).ok_or("glVertexArrayElementBuffer")?);
    let glVertexArrayVertexBuffer_p = transmute::<nn_cv, glVertexArrayVertexBuffer_t>(Self::ptr_filter(f(b"glVertexArrayVertexBuffer\0".as_ptr())).ok_or("glVertexArrayVertexBuffer")?);
    let glVertexArrayVertexBuffers_p = transmute::<nn_cv, glVertexArrayVertexBuffers_t>(Self::ptr_filter(f(b"glVertexArrayVertexBuffers\0".as_ptr())).ok_or("glVertexArrayVertexBuffers")?);
    let glVertexAttrib1d_p = transmute::<nn_cv, glVertexAttrib1d_t>(Self::ptr_filter(f(b"glVertexAttrib1d\0".as_ptr())).ok_or("glVertexAttrib1d")?);
    let glVertexAttrib1dv_p = transmute::<nn_cv, glVertexAttrib1dv_t>(Self::ptr_filter(f(b"glVertexAttrib1dv\0".as_ptr())).ok_or("glVertexAttrib1dv")?);
    let glVertexAttrib1f_p = transmute::<nn_cv, glVertexAttrib1f_t>(Self::ptr_filter(f(b"glVertexAttrib1f\0".as_ptr())).ok_or("glVertexAttrib1f")?);
    let glVertexAttrib1fv_p = transmute::<nn_cv, glVertexAttrib1fv_t>(Self::ptr_filter(f(b"glVertexAttrib1fv\0".as_ptr())).ok_or("glVertexAttrib1fv")?);
    let glVertexAttrib1s_p = transmute::<nn_cv, glVertexAttrib1s_t>(Self::ptr_filter(f(b"glVertexAttrib1s\0".as_ptr())).ok_or("glVertexAttrib1s")?);
    let glVertexAttrib1sv_p = transmute::<nn_cv, glVertexAttrib1sv_t>(Self::ptr_filter(f(b"glVertexAttrib1sv\0".as_ptr())).ok_or("glVertexAttrib1sv")?);
    let glVertexAttrib2d_p = transmute::<nn_cv, glVertexAttrib2d_t>(Self::ptr_filter(f(b"glVertexAttrib2d\0".as_ptr())).ok_or("glVertexAttrib2d")?);
    let glVertexAttrib2dv_p = transmute::<nn_cv, glVertexAttrib2dv_t>(Self::ptr_filter(f(b"glVertexAttrib2dv\0".as_ptr())).ok_or("glVertexAttrib2dv")?);
    let glVertexAttrib2f_p = transmute::<nn_cv, glVertexAttrib2f_t>(Self::ptr_filter(f(b"glVertexAttrib2f\0".as_ptr())).ok_or("glVertexAttrib2f")?);
    let glVertexAttrib2fv_p = transmute::<nn_cv, glVertexAttrib2fv_t>(Self::ptr_filter(f(b"glVertexAttrib2fv\0".as_ptr())).ok_or("glVertexAttrib2fv")?);
    let glVertexAttrib2s_p = transmute::<nn_cv, glVertexAttrib2s_t>(Self::ptr_filter(f(b"glVertexAttrib2s\0".as_ptr())).ok_or("glVertexAttrib2s")?);
    let glVertexAttrib2sv_p = transmute::<nn_cv, glVertexAttrib2sv_t>(Self::ptr_filter(f(b"glVertexAttrib2sv\0".as_ptr())).ok_or("glVertexAttrib2sv")?);
    let glVertexAttrib3d_p = transmute::<nn_cv, glVertexAttrib3d_t>(Self::ptr_filter(f(b"glVertexAttrib3d\0".as_ptr())).ok_or("glVertexAttrib3d")?);
    let glVertexAttrib3dv_p = transmute::<nn_cv, glVertexAttrib3dv_t>(Self::ptr_filter(f(b"glVertexAttrib3dv\0".as_ptr())).ok_or("glVertexAttrib3dv")?);
    let glVertexAttrib3f_p = transmute::<nn_cv, glVertexAttrib3f_t>(Self::ptr_filter(f(b"glVertexAttrib3f\0".as_ptr())).ok_or("glVertexAttrib3f")?);
    let glVertexAttrib3fv_p = transmute::<nn_cv, glVertexAttrib3fv_t>(Self::ptr_filter(f(b"glVertexAttrib3fv\0".as_ptr())).ok_or("glVertexAttrib3fv")?);
    let glVertexAttrib3s_p = transmute::<nn_cv, glVertexAttrib3s_t>(Self::ptr_filter(f(b"glVertexAttrib3s\0".as_ptr())).ok_or("glVertexAttrib3s")?);
    let glVertexAttrib3sv_p = transmute::<nn_cv, glVertexAttrib3sv_t>(Self::ptr_filter(f(b"glVertexAttrib3sv\0".as_ptr())).ok_or("glVertexAttrib3sv")?);
    let glVertexAttrib4Nbv_p = transmute::<nn_cv, glVertexAttrib4Nbv_t>(Self::ptr_filter(f(b"glVertexAttrib4Nbv\0".as_ptr())).ok_or("glVertexAttrib4Nbv")?);
    let glVertexAttrib4Niv_p = transmute::<nn_cv, glVertexAttrib4Niv_t>(Self::ptr_filter(f(b"glVertexAttrib4Niv\0".as_ptr())).ok_or("glVertexAttrib4Niv")?);
    let glVertexAttrib4Nsv_p = transmute::<nn_cv, glVertexAttrib4Nsv_t>(Self::ptr_filter(f(b"glVertexAttrib4Nsv\0".as_ptr())).ok_or("glVertexAttrib4Nsv")?);
    let glVertexAttrib4Nub_p = transmute::<nn_cv, glVertexAttrib4Nub_t>(Self::ptr_filter(f(b"glVertexAttrib4Nub\0".as_ptr())).ok_or("glVertexAttrib4Nub")?);
    let glVertexAttrib4Nubv_p = transmute::<nn_cv, glVertexAttrib4Nubv_t>(Self::ptr_filter(f(b"glVertexAttrib4Nubv\0".as_ptr())).ok_or("glVertexAttrib4Nubv")?);
    let glVertexAttrib4Nuiv_p = transmute::<nn_cv, glVertexAttrib4Nuiv_t>(Self::ptr_filter(f(b"glVertexAttrib4Nuiv\0".as_ptr())).ok_or("glVertexAttrib4Nuiv")?);
    let glVertexAttrib4Nusv_p = transmute::<nn_cv, glVertexAttrib4Nusv_t>(Self::ptr_filter(f(b"glVertexAttrib4Nusv\0".as_ptr())).ok_or("glVertexAttrib4Nusv")?);
    let glVertexAttrib4bv_p = transmute::<nn_cv, glVertexAttrib4bv_t>(Self::ptr_filter(f(b"glVertexAttrib4bv\0".as_ptr())).ok_or("glVertexAttrib4bv")?);
    let glVertexAttrib4d_p = transmute::<nn_cv, glVertexAttrib4d_t>(Self::ptr_filter(f(b"glVertexAttrib4d\0".as_ptr())).ok_or("glVertexAttrib4d")?);
    let glVertexAttrib4dv_p = transmute::<nn_cv, glVertexAttrib4dv_t>(Self::ptr_filter(f(b"glVertexAttrib4dv\0".as_ptr())).ok_or("glVertexAttrib4dv")?);
    let glVertexAttrib4f_p = transmute::<nn_cv, glVertexAttrib4f_t>(Self::ptr_filter(f(b"glVertexAttrib4f\0".as_ptr())).ok_or("glVertexAttrib4f")?);
    let glVertexAttrib4fv_p = transmute::<nn_cv, glVertexAttrib4fv_t>(Self::ptr_filter(f(b"glVertexAttrib4fv\0".as_ptr())).ok_or("glVertexAttrib4fv")?);
    let glVertexAttrib4iv_p = transmute::<nn_cv, glVertexAttrib4iv_t>(Self::ptr_filter(f(b"glVertexAttrib4iv\0".as_ptr())).ok_or("glVertexAttrib4iv")?);
    let glVertexAttrib4s_p = transmute::<nn_cv, glVertexAttrib4s_t>(Self::ptr_filter(f(b"glVertexAttrib4s\0".as_ptr())).ok_or("glVertexAttrib4s")?);
    let glVertexAttrib4sv_p = transmute::<nn_cv, glVertexAttrib4sv_t>(Self::ptr_filter(f(b"glVertexAttrib4sv\0".as_ptr())).ok_or("glVertexAttrib4sv")?);
    let glVertexAttrib4ubv_p = transmute::<nn_cv, glVertexAttrib4ubv_t>(Self::ptr_filter(f(b"glVertexAttrib4ubv\0".as_ptr())).ok_or("glVertexAttrib4ubv")?);
    let glVertexAttrib4uiv_p = transmute::<nn_cv, glVertexAttrib4uiv_t>(Self::ptr_filter(f(b"glVertexAttrib4uiv\0".as_ptr())).ok_or("glVertexAttrib4uiv")?);
    let glVertexAttrib4usv_p = transmute::<nn_cv, glVertexAttrib4usv_t>(Self::ptr_filter(f(b"glVertexAttrib4usv\0".as_ptr())).ok_or("glVertexAttrib4usv")?);
    let glVertexAttribBinding_p = transmute::<nn_cv, glVertexAttribBinding_t>(Self::ptr_filter(f(b"glVertexAttribBinding\0".as_ptr())).ok_or("glVertexAttribBinding")?);
    let glVertexAttribDivisor_p = transmute::<nn_cv, glVertexAttribDivisor_t>(Self::ptr_filter(f(b"glVertexAttribDivisor\0".as_ptr())).ok_or("glVertexAttribDivisor")?);
    let glVertexAttribFormat_p = transmute::<nn_cv, glVertexAttribFormat_t>(Self::ptr_filter(f(b"glVertexAttribFormat\0".as_ptr())).ok_or("glVertexAttribFormat")?);
    let glVertexAttribI1i_p = transmute::<nn_cv, glVertexAttribI1i_t>(Self::ptr_filter(f(b"glVertexAttribI1i\0".as_ptr())).ok_or("glVertexAttribI1i")?);
    let glVertexAttribI1iv_p = transmute::<nn_cv, glVertexAttribI1iv_t>(Self::ptr_filter(f(b"glVertexAttribI1iv\0".as_ptr())).ok_or("glVertexAttribI1iv")?);
    let glVertexAttribI1ui_p = transmute::<nn_cv, glVertexAttribI1ui_t>(Self::ptr_filter(f(b"glVertexAttribI1ui\0".as_ptr())).ok_or("glVertexAttribI1ui")?);
    let glVertexAttribI1uiv_p = transmute::<nn_cv, glVertexAttribI1uiv_t>(Self::ptr_filter(f(b"glVertexAttribI1uiv\0".as_ptr())).ok_or("glVertexAttribI1uiv")?);
    let glVertexAttribI2i_p = transmute::<nn_cv, glVertexAttribI2i_t>(Self::ptr_filter(f(b"glVertexAttribI2i\0".as_ptr())).ok_or("glVertexAttribI2i")?);
    let glVertexAttribI2iv_p = transmute::<nn_cv, glVertexAttribI2iv_t>(Self::ptr_filter(f(b"glVertexAttribI2iv\0".as_ptr())).ok_or("glVertexAttribI2iv")?);
    let glVertexAttribI2ui_p = transmute::<nn_cv, glVertexAttribI2ui_t>(Self::ptr_filter(f(b"glVertexAttribI2ui\0".as_ptr())).ok_or("glVertexAttribI2ui")?);
    let glVertexAttribI2uiv_p = transmute::<nn_cv, glVertexAttribI2uiv_t>(Self::ptr_filter(f(b"glVertexAttribI2uiv\0".as_ptr())).ok_or("glVertexAttribI2uiv")?);
    let glVertexAttribI3i_p = transmute::<nn_cv, glVertexAttribI3i_t>(Self::ptr_filter(f(b"glVertexAttribI3i\0".as_ptr())).ok_or("glVertexAttribI3i")?);
    let glVertexAttribI3iv_p = transmute::<nn_cv, glVertexAttribI3iv_t>(Self::ptr_filter(f(b"glVertexAttribI3iv\0".as_ptr())).ok_or("glVertexAttribI3iv")?);
    let glVertexAttribI3ui_p = transmute::<nn_cv, glVertexAttribI3ui_t>(Self::ptr_filter(f(b"glVertexAttribI3ui\0".as_ptr())).ok_or("glVertexAttribI3ui")?);
    let glVertexAttribI3uiv_p = transmute::<nn_cv, glVertexAttribI3uiv_t>(Self::ptr_filter(f(b"glVertexAttribI3uiv\0".as_ptr())).ok_or("glVertexAttribI3uiv")?);
    let glVertexAttribI4bv_p = transmute::<nn_cv, glVertexAttribI4bv_t>(Self::ptr_filter(f(b"glVertexAttribI4bv\0".as_ptr())).ok_or("glVertexAttribI4bv")?);
    let glVertexAttribI4i_p = transmute::<nn_cv, glVertexAttribI4i_t>(Self::ptr_filter(f(b"glVertexAttribI4i\0".as_ptr())).ok_or("glVertexAttribI4i")?);
    let glVertexAttribI4iv_p = transmute::<nn_cv, glVertexAttribI4iv_t>(Self::ptr_filter(f(b"glVertexAttribI4iv\0".as_ptr())).ok_or("glVertexAttribI4iv")?);
    let glVertexAttribI4sv_p = transmute::<nn_cv, glVertexAttribI4sv_t>(Self::ptr_filter(f(b"glVertexAttribI4sv\0".as_ptr())).ok_or("glVertexAttribI4sv")?);
    let glVertexAttribI4ubv_p = transmute::<nn_cv, glVertexAttribI4ubv_t>(Self::ptr_filter(f(b"glVertexAttribI4ubv\0".as_ptr())).ok_or("glVertexAttribI4ubv")?);
    let glVertexAttribI4ui_p = transmute::<nn_cv, glVertexAttribI4ui_t>(Self::ptr_filter(f(b"glVertexAttribI4ui\0".as_ptr())).ok_or("glVertexAttribI4ui")?);
    let glVertexAttribI4uiv_p = transmute::<nn_cv, glVertexAttribI4uiv_t>(Self::ptr_filter(f(b"glVertexAttribI4uiv\0".as_ptr())).ok_or("glVertexAttribI4uiv")?);
    let glVertexAttribI4usv_p = transmute::<nn_cv, glVertexAttribI4usv_t>(Self::ptr_filter(f(b"glVertexAttribI4usv\0".as_ptr())).ok_or("glVertexAttribI4usv")?);
    let glVertexAttribIFormat_p = transmute::<nn_cv, glVertexAttribIFormat_t>(Self::ptr_filter(f(b"glVertexAttribIFormat\0".as_ptr())).ok_or("glVertexAttribIFormat")?);
    let glVertexAttribIPointer_p = transmute::<nn_cv, glVertexAttribIPointer_t>(Self::ptr_filter(f(b"glVertexAttribIPointer\0".as_ptr())).ok_or("glVertexAttribIPointer")?);
    let glVertexAttribL1d_p = transmute::<nn_cv, glVertexAttribL1d_t>(Self::ptr_filter(f(b"glVertexAttribL1d\0".as_ptr())).ok_or("glVertexAttribL1d")?);
    let glVertexAttribL1dv_p = transmute::<nn_cv, glVertexAttribL1dv_t>(Self::ptr_filter(f(b"glVertexAttribL1dv\0".as_ptr())).ok_or("glVertexAttribL1dv")?);
    let glVertexAttribL2d_p = transmute::<nn_cv, glVertexAttribL2d_t>(Self::ptr_filter(f(b"glVertexAttribL2d\0".as_ptr())).ok_or("glVertexAttribL2d")?);
    let glVertexAttribL2dv_p = transmute::<nn_cv, glVertexAttribL2dv_t>(Self::ptr_filter(f(b"glVertexAttribL2dv\0".as_ptr())).ok_or("glVertexAttribL2dv")?);
    let glVertexAttribL3d_p = transmute::<nn_cv, glVertexAttribL3d_t>(Self::ptr_filter(f(b"glVertexAttribL3d\0".as_ptr())).ok_or("glVertexAttribL3d")?);
    let glVertexAttribL3dv_p = transmute::<nn_cv, glVertexAttribL3dv_t>(Self::ptr_filter(f(b"glVertexAttribL3dv\0".as_ptr())).ok_or("glVertexAttribL3dv")?);
    let glVertexAttribL4d_p = transmute::<nn_cv, glVertexAttribL4d_t>(Self::ptr_filter(f(b"glVertexAttribL4d\0".as_ptr())).ok_or("glVertexAttribL4d")?);
    let glVertexAttribL4dv_p = transmute::<nn_cv, glVertexAttribL4dv_t>(Self::ptr_filter(f(b"glVertexAttribL4dv\0".as_ptr())).ok_or("glVertexAttribL4dv")?);
    let glVertexAttribLFormat_p = transmute::<nn_cv, glVertexAttribLFormat_t>(Self::ptr_filter(f(b"glVertexAttribLFormat\0".as_ptr())).ok_or("glVertexAttribLFormat")?);
    let glVertexAttribLPointer_p = transmute::<nn_cv, glVertexAttribLPointer_t>(Self::ptr_filter(f(b"glVertexAttribLPointer\0".as_ptr())).ok_or("glVertexAttribLPointer")?);
    let glVertexAttribP1ui_p = transmute::<nn_cv, glVertexAttribP1ui_t>(Self::ptr_filter(f(b"glVertexAttribP1ui\0".as_ptr())).ok_or("glVertexAttribP1ui")?);
    let glVertexAttribP1uiv_p = transmute::<nn_cv, glVertexAttribP1uiv_t>(Self::ptr_filter(f(b"glVertexAttribP1uiv\0".as_ptr())).ok_or("glVertexAttribP1uiv")?);
    let glVertexAttribP2ui_p = transmute::<nn_cv, glVertexAttribP2ui_t>(Self::ptr_filter(f(b"glVertexAttribP2ui\0".as_ptr())).ok_or("glVertexAttribP2ui")?);
    let glVertexAttribP2uiv_p = transmute::<nn_cv, glVertexAttribP2uiv_t>(Self::ptr_filter(f(b"glVertexAttribP2uiv\0".as_ptr())).ok_or("glVertexAttribP2uiv")?);
    let glVertexAttribP3ui_p = transmute::<nn_cv, glVertexAttribP3ui_t>(Self::ptr_filter(f(b"glVertexAttribP3ui\0".as_ptr())).ok_or("glVertexAttribP3ui")?);
    let glVertexAttribP3uiv_p = transmute::<nn_cv, glVertexAttribP3uiv_t>(Self::ptr_filter(f(b"glVertexAttribP3uiv\0".as_ptr())).ok_or("glVertexAttribP3uiv")?);
    let glVertexAttribP4ui_p = transmute::<nn_cv, glVertexAttribP4ui_t>(Self::ptr_filter(f(b"glVertexAttribP4ui\0".as_ptr())).ok_or("glVertexAttribP4ui")?);
    let glVertexAttribP4uiv_p = transmute::<nn_cv, glVertexAttribP4uiv_t>(Self::ptr_filter(f(b"glVertexAttribP4uiv\0".as_ptr())).ok_or("glVertexAttribP4uiv")?);
    let glVertexAttribPointer_p = transmute::<nn_cv, glVertexAttribPointer_t>(Self::ptr_filter(f(b"glVertexAttribPointer\0".as_ptr())).ok_or("glVertexAttribPointer")?);
    let glVertexBindingDivisor_p = transmute::<nn_cv, glVertexBindingDivisor_t>(Self::ptr_filter(f(b"glVertexBindingDivisor\0".as_ptr())).ok_or("glVertexBindingDivisor")?);
    let glVertexP2ui_p = transmute::<nn_cv, glVertexP2ui_t>(Self::ptr_filter(f(b"glVertexP2ui\0".as_ptr())).ok_or("glVertexP2ui")?);
    let glVertexP2uiv_p = transmute::<nn_cv, glVertexP2uiv_t>(Self::ptr_filter(f(b"glVertexP2uiv\0".as_ptr())).ok_or("glVertexP2uiv")?);
    let glVertexP3ui_p = transmute::<nn_cv, glVertexP3ui_t>(Self::ptr_filter(f(b"glVertexP3ui\0".as_ptr())).ok_or("glVertexP3ui")?);
    let glVertexP3uiv_p = transmute::<nn_cv, glVertexP3uiv_t>(Self::ptr_filter(f(b"glVertexP3uiv\0".as_ptr())).ok_or("glVertexP3uiv")?);
    let glVertexP4ui_p = transmute::<nn_cv, glVertexP4ui_t>(Self::ptr_filter(f(b"glVertexP4ui\0".as_ptr())).ok_or("glVertexP4ui")?);
    let glVertexP4uiv_p = transmute::<nn_cv, glVertexP4uiv_t>(Self::ptr_filter(f(b"glVertexP4uiv\0".as_ptr())).ok_or("glVertexP4uiv")?);
    let glViewport_p = transmute::<nn_cv, glViewport_t>(Self::ptr_filter(f(b"glViewport\0".as_ptr())).ok_or("glViewport")?);
    let glViewportArrayv_p = transmute::<nn_cv, glViewportArrayv_t>(Self::ptr_filter(f(b"glViewportArrayv\0".as_ptr())).ok_or("glViewportArrayv")?);
    let glViewportIndexedf_p = transmute::<nn_cv, glViewportIndexedf_t>(Self::ptr_filter(f(b"glViewportIndexedf\0".as_ptr())).ok_or("glViewportIndexedf")?);
    let glViewportIndexedfv_p = transmute::<nn_cv, glViewportIndexedfv_t>(Self::ptr_filter(f(b"glViewportIndexedfv\0".as_ptr())).ok_or("glViewportIndexedfv")?);
    let glWaitSync_p = transmute::<nn_cv, glWaitSync_t>(Self::ptr_filter(f(b"glWaitSync\0".as_ptr())).ok_or("glWaitSync")?);
    // nullable loads
    let glDebugMessageCallbackARB_p = transmute::<Option<nn_cv>, Option<glDebugMessageCallbackARB_t>>(Self::ptr_filter(f(b"glDebugMessageCallbackARB\0".as_ptr())));
    let glDebugMessageControlARB_p = transmute::<Option<nn_cv>, Option<glDebugMessageControlARB_t>>(Self::ptr_filter(f(b"glDebugMessageControlARB\0".as_ptr())));
    let glDebugMessageInsertARB_p = transmute::<Option<nn_cv>, Option<glDebugMessageInsertARB_t>>(Self::ptr_filter(f(b"glDebugMessageInsertARB\0".as_ptr())));
    let glGetDebugMessageLogARB_p = transmute::<Option<nn_cv>, Option<glGetDebugMessageLogARB_t>>(Self::ptr_filter(f(b"glGetDebugMessageLogARB\0".as_ptr())));
    let glDebugMessageCallbackKHR_p = transmute::<Option<nn_cv>, Option<glDebugMessageCallbackKHR_t>>(Self::ptr_filter(f(b"glDebugMessageCallbackKHR\0".as_ptr())));
    let glDebugMessageControlKHR_p = transmute::<Option<nn_cv>, Option<glDebugMessageControlKHR_t>>(Self::ptr_filter(f(b"glDebugMessageControlKHR\0".as_ptr())));
    let glDebugMessageInsertKHR_p = transmute::<Option<nn_cv>, Option<glDebugMessageInsertKHR_t>>(Self::ptr_filter(f(b"glDebugMessageInsertKHR\0".as_ptr())));
    let glGetDebugMessageLogKHR_p = transmute::<Option<nn_cv>, Option<glGetDebugMessageLogKHR_t>>(Self::ptr_filter(f(b"glGetDebugMessageLogKHR\0".as_ptr())));
    let glGetObjectLabelKHR_p = transmute::<Option<nn_cv>, Option<glGetObjectLabelKHR_t>>(Self::ptr_filter(f(b"glGetObjectLabelKHR\0".as_ptr())));
    let glGetObjectPtrLabelKHR_p = transmute::<Option<nn_cv>, Option<glGetObjectPtrLabelKHR_t>>(Self::ptr_filter(f(b"glGetObjectPtrLabelKHR\0".as_ptr())));
    let glGetPointervKHR_p = transmute::<Option<nn_cv>, Option<glGetPointervKHR_t>>(Self::ptr_filter(f(b"glGetPointervKHR\0".as_ptr())));
    let glObjectLabelKHR_p = transmute::<Option<nn_cv>, Option<glObjectLabelKHR_t>>(Self::ptr_filter(f(b"glObjectLabelKHR\0".as_ptr())));
    let glObjectPtrLabelKHR_p = transmute::<Option<nn_cv>, Option<glObjectPtrLabelKHR_t>>(Self::ptr_filter(f(b"glObjectPtrLabelKHR\0".as_ptr())));
    let glPopDebugGroupKHR_p = transmute::<Option<nn_cv>, Option<glPopDebugGroupKHR_t>>(Self::ptr_filter(f(b"glPopDebugGroupKHR\0".as_ptr())));
    let glPushDebugGroupKHR_p = transmute::<Option<nn_cv>, Option<glPushDebugGroupKHR_t>>(Self::ptr_filter(f(b"glPushDebugGroupKHR\0".as_ptr())));
    // we're all good!
    Ok(Self {
      glActiveShaderProgram_p,
      glActiveTexture_p,
      glAttachShader_p,
      glBeginConditionalRender_p,
      glBeginQuery_p,
      glBeginQueryIndexed_p,
      glBeginTransformFeedback_p,
      glBindAttribLocation_p,
      glBindBuffer_p,
      glBindBufferBase_p,
      glBindBufferRange_p,
      glBindBuffersBase_p,
      glBindBuffersRange_p,
      glBindFragDataLocation_p,
      glBindFragDataLocationIndexed_p,
      glBindFramebuffer_p,
      glBindImageTexture_p,
      glBindImageTextures_p,
      glBindProgramPipeline_p,
      glBindRenderbuffer_p,
      glBindSampler_p,
      glBindSamplers_p,
      glBindTexture_p,
      glBindTextureUnit_p,
      glBindTextures_p,
      glBindTransformFeedback_p,
      glBindVertexArray_p,
      glBindVertexBuffer_p,
      glBindVertexBuffers_p,
      glBlendColor_p,
      glBlendEquation_p,
      glBlendEquationSeparate_p,
      glBlendEquationSeparatei_p,
      glBlendEquationi_p,
      glBlendFunc_p,
      glBlendFuncSeparate_p,
      glBlendFuncSeparatei_p,
      glBlendFunci_p,
      glBlitFramebuffer_p,
      glBlitNamedFramebuffer_p,
      glBufferData_p,
      glBufferStorage_p,
      glBufferSubData_p,
      glCheckFramebufferStatus_p,
      glCheckNamedFramebufferStatus_p,
      glClampColor_p,
      glClear_p,
      glClearBufferData_p,
      glClearBufferSubData_p,
      glClearBufferfi_p,
      glClearBufferfv_p,
      glClearBufferiv_p,
      glClearBufferuiv_p,
      glClearColor_p,
      glClearDepth_p,
      glClearDepthf_p,
      glClearNamedBufferData_p,
      glClearNamedBufferSubData_p,
      glClearNamedFramebufferfi_p,
      glClearNamedFramebufferfv_p,
      glClearNamedFramebufferiv_p,
      glClearNamedFramebufferuiv_p,
      glClearStencil_p,
      glClearTexImage_p,
      glClearTexSubImage_p,
      glClientWaitSync_p,
      glClipControl_p,
      glColorMask_p,
      glColorMaski_p,
      glColorP3ui_p,
      glColorP3uiv_p,
      glColorP4ui_p,
      glColorP4uiv_p,
      glCompileShader_p,
      glCompressedTexImage1D_p,
      glCompressedTexImage2D_p,
      glCompressedTexImage3D_p,
      glCompressedTexSubImage1D_p,
      glCompressedTexSubImage2D_p,
      glCompressedTexSubImage3D_p,
      glCompressedTextureSubImage1D_p,
      glCompressedTextureSubImage2D_p,
      glCompressedTextureSubImage3D_p,
      glCopyBufferSubData_p,
      glCopyImageSubData_p,
      glCopyNamedBufferSubData_p,
      glCopyTexImage1D_p,
      glCopyTexImage2D_p,
      glCopyTexSubImage1D_p,
      glCopyTexSubImage2D_p,
      glCopyTexSubImage3D_p,
      glCopyTextureSubImage1D_p,
      glCopyTextureSubImage2D_p,
      glCopyTextureSubImage3D_p,
      glCreateBuffers_p,
      glCreateFramebuffers_p,
      glCreateProgram_p,
      glCreateProgramPipelines_p,
      glCreateQueries_p,
      glCreateRenderbuffers_p,
      glCreateSamplers_p,
      glCreateShader_p,
      glCreateShaderProgramv_p,
      glCreateTextures_p,
      glCreateTransformFeedbacks_p,
      glCreateVertexArrays_p,
      glCullFace_p,
      glDebugMessageCallback_p,
      glDebugMessageControl_p,
      glDebugMessageInsert_p,
      glDeleteBuffers_p,
      glDeleteFramebuffers_p,
      glDeleteProgram_p,
      glDeleteProgramPipelines_p,
      glDeleteQueries_p,
      glDeleteRenderbuffers_p,
      glDeleteSamplers_p,
      glDeleteShader_p,
      glDeleteSync_p,
      glDeleteTextures_p,
      glDeleteTransformFeedbacks_p,
      glDeleteVertexArrays_p,
      glDepthFunc_p,
      glDepthMask_p,
      glDepthRange_p,
      glDepthRangeArrayv_p,
      glDepthRangeIndexed_p,
      glDepthRangef_p,
      glDetachShader_p,
      glDisable_p,
      glDisableVertexArrayAttrib_p,
      glDisableVertexAttribArray_p,
      glDisablei_p,
      glDispatchCompute_p,
      glDispatchComputeIndirect_p,
      glDrawArrays_p,
      glDrawArraysIndirect_p,
      glDrawArraysInstanced_p,
      glDrawArraysInstancedBaseInstance_p,
      glDrawBuffer_p,
      glDrawBuffers_p,
      glDrawElements_p,
      glDrawElementsBaseVertex_p,
      glDrawElementsIndirect_p,
      glDrawElementsInstanced_p,
      glDrawElementsInstancedBaseInstance_p,
      glDrawElementsInstancedBaseVertex_p,
      glDrawElementsInstancedBaseVertexBaseInstance_p,
      glDrawRangeElements_p,
      glDrawRangeElementsBaseVertex_p,
      glDrawTransformFeedback_p,
      glDrawTransformFeedbackInstanced_p,
      glDrawTransformFeedbackStream_p,
      glDrawTransformFeedbackStreamInstanced_p,
      glEnable_p,
      glEnableVertexArrayAttrib_p,
      glEnableVertexAttribArray_p,
      glEnablei_p,
      glEndConditionalRender_p,
      glEndQuery_p,
      glEndQueryIndexed_p,
      glEndTransformFeedback_p,
      glFenceSync_p,
      glFinish_p,
      glFlush_p,
      glFlushMappedBufferRange_p,
      glFlushMappedNamedBufferRange_p,
      glFramebufferParameteri_p,
      glFramebufferRenderbuffer_p,
      glFramebufferTexture_p,
      glFramebufferTexture1D_p,
      glFramebufferTexture2D_p,
      glFramebufferTexture3D_p,
      glFramebufferTextureLayer_p,
      glFrontFace_p,
      glGenBuffers_p,
      glGenFramebuffers_p,
      glGenProgramPipelines_p,
      glGenQueries_p,
      glGenRenderbuffers_p,
      glGenSamplers_p,
      glGenTextures_p,
      glGenTransformFeedbacks_p,
      glGenVertexArrays_p,
      glGenerateMipmap_p,
      glGenerateTextureMipmap_p,
      glGetActiveAtomicCounterBufferiv_p,
      glGetActiveAttrib_p,
      glGetActiveSubroutineName_p,
      glGetActiveSubroutineUniformName_p,
      glGetActiveSubroutineUniformiv_p,
      glGetActiveUniform_p,
      glGetActiveUniformBlockName_p,
      glGetActiveUniformBlockiv_p,
      glGetActiveUniformName_p,
      glGetActiveUniformsiv_p,
      glGetAttachedShaders_p,
      glGetAttribLocation_p,
      glGetBooleani_v_p,
      glGetBooleanv_p,
      glGetBufferParameteri64v_p,
      glGetBufferParameteriv_p,
      glGetBufferPointerv_p,
      glGetBufferSubData_p,
      glGetCompressedTexImage_p,
      glGetCompressedTextureImage_p,
      glGetCompressedTextureSubImage_p,
      glGetDebugMessageLog_p,
      glGetDoublei_v_p,
      glGetDoublev_p,
      glGetError_p,
      glGetFloati_v_p,
      glGetFloatv_p,
      glGetFragDataIndex_p,
      glGetFragDataLocation_p,
      glGetFramebufferAttachmentParameteriv_p,
      glGetFramebufferParameteriv_p,
      glGetGraphicsResetStatus_p,
      glGetInteger64i_v_p,
      glGetInteger64v_p,
      glGetIntegeri_v_p,
      glGetIntegerv_p,
      glGetInternalformati64v_p,
      glGetInternalformativ_p,
      glGetMultisamplefv_p,
      glGetNamedBufferParameteri64v_p,
      glGetNamedBufferParameteriv_p,
      glGetNamedBufferPointerv_p,
      glGetNamedBufferSubData_p,
      glGetNamedFramebufferAttachmentParameteriv_p,
      glGetNamedFramebufferParameteriv_p,
      glGetNamedRenderbufferParameteriv_p,
      glGetObjectLabel_p,
      glGetObjectPtrLabel_p,
      glGetPointerv_p,
      glGetProgramBinary_p,
      glGetProgramInfoLog_p,
      glGetProgramInterfaceiv_p,
      glGetProgramPipelineInfoLog_p,
      glGetProgramPipelineiv_p,
      glGetProgramResourceIndex_p,
      glGetProgramResourceLocation_p,
      glGetProgramResourceLocationIndex_p,
      glGetProgramResourceName_p,
      glGetProgramResourceiv_p,
      glGetProgramStageiv_p,
      glGetProgramiv_p,
      glGetQueryBufferObjecti64v_p,
      glGetQueryBufferObjectiv_p,
      glGetQueryBufferObjectui64v_p,
      glGetQueryBufferObjectuiv_p,
      glGetQueryIndexediv_p,
      glGetQueryObjecti64v_p,
      glGetQueryObjectiv_p,
      glGetQueryObjectui64v_p,
      glGetQueryObjectuiv_p,
      glGetQueryiv_p,
      glGetRenderbufferParameteriv_p,
      glGetSamplerParameterIiv_p,
      glGetSamplerParameterIuiv_p,
      glGetSamplerParameterfv_p,
      glGetSamplerParameteriv_p,
      glGetShaderInfoLog_p,
      glGetShaderPrecisionFormat_p,
      glGetShaderSource_p,
      glGetShaderiv_p,
      glGetString_p,
      glGetStringi_p,
      glGetSubroutineIndex_p,
      glGetSubroutineUniformLocation_p,
      glGetSynciv_p,
      glGetTexImage_p,
      glGetTexLevelParameterfv_p,
      glGetTexLevelParameteriv_p,
      glGetTexParameterIiv_p,
      glGetTexParameterIuiv_p,
      glGetTexParameterfv_p,
      glGetTexParameteriv_p,
      glGetTextureImage_p,
      glGetTextureLevelParameterfv_p,
      glGetTextureLevelParameteriv_p,
      glGetTextureParameterIiv_p,
      glGetTextureParameterIuiv_p,
      glGetTextureParameterfv_p,
      glGetTextureParameteriv_p,
      glGetTextureSubImage_p,
      glGetTransformFeedbackVarying_p,
      glGetTransformFeedbacki64_v_p,
      glGetTransformFeedbacki_v_p,
      glGetTransformFeedbackiv_p,
      glGetUniformBlockIndex_p,
      glGetUniformIndices_p,
      glGetUniformLocation_p,
      glGetUniformSubroutineuiv_p,
      glGetUniformdv_p,
      glGetUniformfv_p,
      glGetUniformiv_p,
      glGetUniformuiv_p,
      glGetVertexArrayIndexed64iv_p,
      glGetVertexArrayIndexediv_p,
      glGetVertexArrayiv_p,
      glGetVertexAttribIiv_p,
      glGetVertexAttribIuiv_p,
      glGetVertexAttribLdv_p,
      glGetVertexAttribPointerv_p,
      glGetVertexAttribdv_p,
      glGetVertexAttribfv_p,
      glGetVertexAttribiv_p,
      glGetnColorTable_p,
      glGetnCompressedTexImage_p,
      glGetnConvolutionFilter_p,
      glGetnHistogram_p,
      glGetnMapdv_p,
      glGetnMapfv_p,
      glGetnMapiv_p,
      glGetnMinmax_p,
      glGetnPixelMapfv_p,
      glGetnPixelMapuiv_p,
      glGetnPixelMapusv_p,
      glGetnPolygonStipple_p,
      glGetnSeparableFilter_p,
      glGetnTexImage_p,
      glGetnUniformdv_p,
      glGetnUniformfv_p,
      glGetnUniformiv_p,
      glGetnUniformuiv_p,
      glHint_p,
      glInvalidateBufferData_p,
      glInvalidateBufferSubData_p,
      glInvalidateFramebuffer_p,
      glInvalidateNamedFramebufferData_p,
      glInvalidateNamedFramebufferSubData_p,
      glInvalidateSubFramebuffer_p,
      glInvalidateTexImage_p,
      glInvalidateTexSubImage_p,
      glIsBuffer_p,
      glIsEnabled_p,
      glIsEnabledi_p,
      glIsFramebuffer_p,
      glIsProgram_p,
      glIsProgramPipeline_p,
      glIsQuery_p,
      glIsRenderbuffer_p,
      glIsSampler_p,
      glIsShader_p,
      glIsSync_p,
      glIsTexture_p,
      glIsTransformFeedback_p,
      glIsVertexArray_p,
      glLineWidth_p,
      glLinkProgram_p,
      glLogicOp_p,
      glMapBuffer_p,
      glMapBufferRange_p,
      glMapNamedBuffer_p,
      glMapNamedBufferRange_p,
      glMemoryBarrier_p,
      glMemoryBarrierByRegion_p,
      glMinSampleShading_p,
      glMultiDrawArrays_p,
      glMultiDrawArraysIndirect_p,
      glMultiDrawArraysIndirectCount_p,
      glMultiDrawElements_p,
      glMultiDrawElementsBaseVertex_p,
      glMultiDrawElementsIndirect_p,
      glMultiDrawElementsIndirectCount_p,
      glMultiTexCoordP1ui_p,
      glMultiTexCoordP1uiv_p,
      glMultiTexCoordP2ui_p,
      glMultiTexCoordP2uiv_p,
      glMultiTexCoordP3ui_p,
      glMultiTexCoordP3uiv_p,
      glMultiTexCoordP4ui_p,
      glMultiTexCoordP4uiv_p,
      glNamedBufferData_p,
      glNamedBufferStorage_p,
      glNamedBufferSubData_p,
      glNamedFramebufferDrawBuffer_p,
      glNamedFramebufferDrawBuffers_p,
      glNamedFramebufferParameteri_p,
      glNamedFramebufferReadBuffer_p,
      glNamedFramebufferRenderbuffer_p,
      glNamedFramebufferTexture_p,
      glNamedFramebufferTextureLayer_p,
      glNamedRenderbufferStorage_p,
      glNamedRenderbufferStorageMultisample_p,
      glNormalP3ui_p,
      glNormalP3uiv_p,
      glObjectLabel_p,
      glObjectPtrLabel_p,
      glPatchParameterfv_p,
      glPatchParameteri_p,
      glPauseTransformFeedback_p,
      glPixelStoref_p,
      glPixelStorei_p,
      glPointParameterf_p,
      glPointParameterfv_p,
      glPointParameteri_p,
      glPointParameteriv_p,
      glPointSize_p,
      glPolygonMode_p,
      glPolygonOffset_p,
      glPolygonOffsetClamp_p,
      glPopDebugGroup_p,
      glPrimitiveRestartIndex_p,
      glProgramBinary_p,
      glProgramParameteri_p,
      glProgramUniform1d_p,
      glProgramUniform1dv_p,
      glProgramUniform1f_p,
      glProgramUniform1fv_p,
      glProgramUniform1i_p,
      glProgramUniform1iv_p,
      glProgramUniform1ui_p,
      glProgramUniform1uiv_p,
      glProgramUniform2d_p,
      glProgramUniform2dv_p,
      glProgramUniform2f_p,
      glProgramUniform2fv_p,
      glProgramUniform2i_p,
      glProgramUniform2iv_p,
      glProgramUniform2ui_p,
      glProgramUniform2uiv_p,
      glProgramUniform3d_p,
      glProgramUniform3dv_p,
      glProgramUniform3f_p,
      glProgramUniform3fv_p,
      glProgramUniform3i_p,
      glProgramUniform3iv_p,
      glProgramUniform3ui_p,
      glProgramUniform3uiv_p,
      glProgramUniform4d_p,
      glProgramUniform4dv_p,
      glProgramUniform4f_p,
      glProgramUniform4fv_p,
      glProgramUniform4i_p,
      glProgramUniform4iv_p,
      glProgramUniform4ui_p,
      glProgramUniform4uiv_p,
      glProgramUniformMatrix2dv_p,
      glProgramUniformMatrix2fv_p,
      glProgramUniformMatrix2x3dv_p,
      glProgramUniformMatrix2x3fv_p,
      glProgramUniformMatrix2x4dv_p,
      glProgramUniformMatrix2x4fv_p,
      glProgramUniformMatrix3dv_p,
      glProgramUniformMatrix3fv_p,
      glProgramUniformMatrix3x2dv_p,
      glProgramUniformMatrix3x2fv_p,
      glProgramUniformMatrix3x4dv_p,
      glProgramUniformMatrix3x4fv_p,
      glProgramUniformMatrix4dv_p,
      glProgramUniformMatrix4fv_p,
      glProgramUniformMatrix4x2dv_p,
      glProgramUniformMatrix4x2fv_p,
      glProgramUniformMatrix4x3dv_p,
      glProgramUniformMatrix4x3fv_p,
      glProvokingVertex_p,
      glPushDebugGroup_p,
      glQueryCounter_p,
      glReadBuffer_p,
      glReadPixels_p,
      glReadnPixels_p,
      glReleaseShaderCompiler_p,
      glRenderbufferStorage_p,
      glRenderbufferStorageMultisample_p,
      glResumeTransformFeedback_p,
      glSampleCoverage_p,
      glSampleMaski_p,
      glSamplerParameterIiv_p,
      glSamplerParameterIuiv_p,
      glSamplerParameterf_p,
      glSamplerParameterfv_p,
      glSamplerParameteri_p,
      glSamplerParameteriv_p,
      glScissor_p,
      glScissorArrayv_p,
      glScissorIndexed_p,
      glScissorIndexedv_p,
      glSecondaryColorP3ui_p,
      glSecondaryColorP3uiv_p,
      glShaderBinary_p,
      glShaderSource_p,
      glShaderStorageBlockBinding_p,
      glSpecializeShader_p,
      glStencilFunc_p,
      glStencilFuncSeparate_p,
      glStencilMask_p,
      glStencilMaskSeparate_p,
      glStencilOp_p,
      glStencilOpSeparate_p,
      glTexBuffer_p,
      glTexBufferRange_p,
      glTexCoordP1ui_p,
      glTexCoordP1uiv_p,
      glTexCoordP2ui_p,
      glTexCoordP2uiv_p,
      glTexCoordP3ui_p,
      glTexCoordP3uiv_p,
      glTexCoordP4ui_p,
      glTexCoordP4uiv_p,
      glTexImage1D_p,
      glTexImage2D_p,
      glTexImage2DMultisample_p,
      glTexImage3D_p,
      glTexImage3DMultisample_p,
      glTexParameterIiv_p,
      glTexParameterIuiv_p,
      glTexParameterf_p,
      glTexParameterfv_p,
      glTexParameteri_p,
      glTexParameteriv_p,
      glTexStorage1D_p,
      glTexStorage2D_p,
      glTexStorage2DMultisample_p,
      glTexStorage3D_p,
      glTexStorage3DMultisample_p,
      glTexSubImage1D_p,
      glTexSubImage2D_p,
      glTexSubImage3D_p,
      glTextureBarrier_p,
      glTextureBuffer_p,
      glTextureBufferRange_p,
      glTextureParameterIiv_p,
      glTextureParameterIuiv_p,
      glTextureParameterf_p,
      glTextureParameterfv_p,
      glTextureParameteri_p,
      glTextureParameteriv_p,
      glTextureStorage1D_p,
      glTextureStorage2D_p,
      glTextureStorage2DMultisample_p,
      glTextureStorage3D_p,
      glTextureStorage3DMultisample_p,
      glTextureSubImage1D_p,
      glTextureSubImage2D_p,
      glTextureSubImage3D_p,
      glTextureView_p,
      glTransformFeedbackBufferBase_p,
      glTransformFeedbackBufferRange_p,
      glTransformFeedbackVaryings_p,
      glUniform1d_p,
      glUniform1dv_p,
      glUniform1f_p,
      glUniform1fv_p,
      glUniform1i_p,
      glUniform1iv_p,
      glUniform1ui_p,
      glUniform1uiv_p,
      glUniform2d_p,
      glUniform2dv_p,
      glUniform2f_p,
      glUniform2fv_p,
      glUniform2i_p,
      glUniform2iv_p,
      glUniform2ui_p,
      glUniform2uiv_p,
      glUniform3d_p,
      glUniform3dv_p,
      glUniform3f_p,
      glUniform3fv_p,
      glUniform3i_p,
      glUniform3iv_p,
      glUniform3ui_p,
      glUniform3uiv_p,
      glUniform4d_p,
      glUniform4dv_p,
      glUniform4f_p,
      glUniform4fv_p,
      glUniform4i_p,
      glUniform4iv_p,
      glUniform4ui_p,
      glUniform4uiv_p,
      glUniformBlockBinding_p,
      glUniformMatrix2dv_p,
      glUniformMatrix2fv_p,
      glUniformMatrix2x3dv_p,
      glUniformMatrix2x3fv_p,
      glUniformMatrix2x4dv_p,
      glUniformMatrix2x4fv_p,
      glUniformMatrix3dv_p,
      glUniformMatrix3fv_p,
      glUniformMatrix3x2dv_p,
      glUniformMatrix3x2fv_p,
      glUniformMatrix3x4dv_p,
      glUniformMatrix3x4fv_p,
      glUniformMatrix4dv_p,
      glUniformMatrix4fv_p,
      glUniformMatrix4x2dv_p,
      glUniformMatrix4x2fv_p,
      glUniformMatrix4x3dv_p,
      glUniformMatrix4x3fv_p,
      glUniformSubroutinesuiv_p,
      glUnmapBuffer_p,
      glUnmapNamedBuffer_p,
      glUseProgram_p,
      glUseProgramStages_p,
      glValidateProgram_p,
      glValidateProgramPipeline_p,
      glVertexArrayAttribBinding_p,
      glVertexArrayAttribFormat_p,
      glVertexArrayAttribIFormat_p,
      glVertexArrayAttribLFormat_p,
      glVertexArrayBindingDivisor_p,
      glVertexArrayElementBuffer_p,
      glVertexArrayVertexBuffer_p,
      glVertexArrayVertexBuffers_p,
      glVertexAttrib1d_p,
      glVertexAttrib1dv_p,
      glVertexAttrib1f_p,
      glVertexAttrib1fv_p,
      glVertexAttrib1s_p,
      glVertexAttrib1sv_p,
      glVertexAttrib2d_p,
      glVertexAttrib2dv_p,
      glVertexAttrib2f_p,
      glVertexAttrib2fv_p,
      glVertexAttrib2s_p,
      glVertexAttrib2sv_p,
      glVertexAttrib3d_p,
      glVertexAttrib3dv_p,
      glVertexAttrib3f_p,
      glVertexAttrib3fv_p,
      glVertexAttrib3s_p,
      glVertexAttrib3sv_p,
      glVertexAttrib4Nbv_p,
      glVertexAttrib4Niv_p,
      glVertexAttrib4Nsv_p,
      glVertexAttrib4Nub_p,
      glVertexAttrib4Nubv_p,
      glVertexAttrib4Nuiv_p,
      glVertexAttrib4Nusv_p,
      glVertexAttrib4bv_p,
      glVertexAttrib4d_p,
      glVertexAttrib4dv_p,
      glVertexAttrib4f_p,
      glVertexAttrib4fv_p,
      glVertexAttrib4iv_p,
      glVertexAttrib4s_p,
      glVertexAttrib4sv_p,
      glVertexAttrib4ubv_p,
      glVertexAttrib4uiv_p,
      glVertexAttrib4usv_p,
      glVertexAttribBinding_p,
      glVertexAttribDivisor_p,
      glVertexAttribFormat_p,
      glVertexAttribI1i_p,
      glVertexAttribI1iv_p,
      glVertexAttribI1ui_p,
      glVertexAttribI1uiv_p,
      glVertexAttribI2i_p,
      glVertexAttribI2iv_p,
      glVertexAttribI2ui_p,
      glVertexAttribI2uiv_p,
      glVertexAttribI3i_p,
      glVertexAttribI3iv_p,
      glVertexAttribI3ui_p,
      glVertexAttribI3uiv_p,
      glVertexAttribI4bv_p,
      glVertexAttribI4i_p,
      glVertexAttribI4iv_p,
      glVertexAttribI4sv_p,
      glVertexAttribI4ubv_p,
      glVertexAttribI4ui_p,
      glVertexAttribI4uiv_p,
      glVertexAttribI4usv_p,
      glVertexAttribIFormat_p,
      glVertexAttribIPointer_p,
      glVertexAttribL1d_p,
      glVertexAttribL1dv_p,
      glVertexAttribL2d_p,
      glVertexAttribL2dv_p,
      glVertexAttribL3d_p,
      glVertexAttribL3dv_p,
      glVertexAttribL4d_p,
      glVertexAttribL4dv_p,
      glVertexAttribLFormat_p,
      glVertexAttribLPointer_p,
      glVertexAttribP1ui_p,
      glVertexAttribP1uiv_p,
      glVertexAttribP2ui_p,
      glVertexAttribP2uiv_p,
      glVertexAttribP3ui_p,
      glVertexAttribP3uiv_p,
      glVertexAttribP4ui_p,
      glVertexAttribP4uiv_p,
      glVertexAttribPointer_p,
      glVertexBindingDivisor_p,
      glVertexP2ui_p,
      glVertexP2uiv_p,
      glVertexP3ui_p,
      glVertexP3uiv_p,
      glVertexP4ui_p,
      glVertexP4uiv_p,
      glViewport_p,
      glViewportArrayv_p,
      glViewportIndexedf_p,
      glViewportIndexedfv_p,
      glWaitSync_p,
      glDebugMessageCallbackARB_p,
      glDebugMessageControlARB_p,
      glDebugMessageInsertARB_p,
      glGetDebugMessageLogARB_p,
      glDebugMessageCallbackKHR_p,
      glDebugMessageControlKHR_p,
      glDebugMessageInsertKHR_p,
      glGetDebugMessageLogKHR_p,
      glGetObjectLabelKHR_p,
      glGetObjectPtrLabelKHR_p,
      glGetPointervKHR_p,
      glObjectLabelKHR_p,
      glObjectPtrLabelKHR_p,
      glPopDebugGroupKHR_p,
      glPushDebugGroupKHR_p,
    })
  }
}

impl GlFns46 {
  /// [glActiveShaderProgram](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glActiveShaderProgram.xhtml)
  /// * `pipeline` class: program pipeline
  /// * `program` class: program
  pub unsafe fn ActiveShaderProgram(&self, pipeline: GLuint, program: GLuint) {
    (self.glActiveShaderProgram_p)(pipeline, program)
  }
  /// [glActiveTexture](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glActiveTexture.xhtml)
  /// * `texture` group: TextureUnit
  pub unsafe fn ActiveTexture(&self, texture: TextureUnit) {
    (self.glActiveTexture_p)(texture)
  }
  /// [glAttachShader](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glAttachShader.xhtml)
  /// * `program` class: program
  /// * `shader` class: shader
  pub unsafe fn AttachShader(&self, program: GLuint, shader: GLuint) {
    (self.glAttachShader_p)(program, shader)
  }
  /// [glBeginConditionalRender](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBeginConditionalRender.xhtml)
  /// * `mode` group: ConditionalRenderMode
  pub unsafe fn BeginConditionalRender(&self, id: GLuint, mode: ConditionalRenderMode) {
    (self.glBeginConditionalRender_p)(id, mode)
  }
  /// [glBeginQuery](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBeginQuery.xhtml)
  /// * `target` group: QueryTarget
  /// * `id` class: query
  pub unsafe fn BeginQuery(&self, target: QueryTarget, id: GLuint) {
    (self.glBeginQuery_p)(target, id)
  }
  /// [glBeginQueryIndexed](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBeginQueryIndexed.xhtml)
  /// * `target` group: QueryTarget
  /// * `id` class: query
  pub unsafe fn BeginQueryIndexed(&self, target: QueryTarget, index: GLuint, id: GLuint) {
    (self.glBeginQueryIndexed_p)(target, index, id)
  }
  /// [glBeginTransformFeedback](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBeginTransformFeedback.xhtml)
  /// * `primitiveMode` group: PrimitiveType
  pub unsafe fn BeginTransformFeedback(&self, primitiveMode: PrimitiveType) {
    (self.glBeginTransformFeedback_p)(primitiveMode)
  }
  /// [glBindAttribLocation](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindAttribLocation.xhtml)
  /// * `program` class: program
  pub unsafe fn BindAttribLocation(&self, program: GLuint, index: GLuint, name: *const GLchar) {
    (self.glBindAttribLocation_p)(program, index, name)
  }
  /// [glBindBuffer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindBuffer.xhtml)
  /// * `target` group: BufferTargetARB
  /// * `buffer` class: buffer
  pub unsafe fn BindBuffer(&self, target: BufferTargetARB, buffer: GLuint) {
    (self.glBindBuffer_p)(target, buffer)
  }
  /// [glBindBufferBase](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindBufferBase.xhtml)
  /// * `target` group: BufferTargetARB
  /// * `buffer` class: buffer
  pub unsafe fn BindBufferBase(&self, target: BufferTargetARB, index: GLuint, buffer: GLuint) {
    (self.glBindBufferBase_p)(target, index, buffer)
  }
  /// [glBindBufferRange](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindBufferRange.xhtml)
  /// * `target` group: BufferTargetARB
  /// * `buffer` class: buffer
  /// * `offset` group: BufferOffset
  /// * `size` group: BufferSize
  pub unsafe fn BindBufferRange(&self, target: BufferTargetARB, index: GLuint, buffer: GLuint, offset: GLintptr, size: GLsizeiptr) {
    (self.glBindBufferRange_p)(target, index, buffer, offset, size)
  }
  /// [glBindBuffersBase](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindBuffersBase.xhtml)
  /// * `target` group: BufferTargetARB
  /// * `buffers` class: buffer
  /// * `buffers` len: count
  pub unsafe fn BindBuffersBase(&self, target: BufferTargetARB, first: GLuint, count: GLsizei, buffers: *const GLuint) {
    (self.glBindBuffersBase_p)(target, first, count, buffers)
  }
  /// [glBindBuffersRange](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindBuffersRange.xhtml)
  /// * `target` group: BufferTargetARB
  /// * `buffers` class: buffer
  /// * `buffers` len: count
  /// * `offsets` len: count
  /// * `sizes` len: count
  pub unsafe fn BindBuffersRange(&self, target: BufferTargetARB, first: GLuint, count: GLsizei, buffers: *const GLuint, offsets: *const GLintptr, sizes: *const GLsizeiptr) {
    (self.glBindBuffersRange_p)(target, first, count, buffers, offsets, sizes)
  }
  /// [glBindFragDataLocation](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindFragDataLocation.xhtml)
  /// * `program` class: program
  /// * `name` len: COMPSIZE(name)
  pub unsafe fn BindFragDataLocation(&self, program: GLuint, color: GLuint, name: *const GLchar) {
    (self.glBindFragDataLocation_p)(program, color, name)
  }
  /// [glBindFragDataLocationIndexed](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindFragDataLocationIndexed.xhtml)
  /// * `program` class: program
  pub unsafe fn BindFragDataLocationIndexed(&self, program: GLuint, colorNumber: GLuint, index: GLuint, name: *const GLchar) {
    (self.glBindFragDataLocationIndexed_p)(program, colorNumber, index, name)
  }
  /// [glBindFramebuffer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindFramebuffer.xhtml)
  /// * `target` group: FramebufferTarget
  /// * `framebuffer` class: framebuffer
  pub unsafe fn BindFramebuffer(&self, target: FramebufferTarget, framebuffer: GLuint) {
    (self.glBindFramebuffer_p)(target, framebuffer)
  }
  /// [glBindImageTexture](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindImageTexture.xhtml)
  /// * `texture` class: texture
  /// * `layered` group: Boolean
  /// * `access` group: BufferAccessARB
  /// * `format` group: InternalFormat
  pub unsafe fn BindImageTexture(&self, unit: GLuint, texture: GLuint, level: GLint, layered: GLboolean, layer: GLint, access: BufferAccessARB, format: InternalFormat) {
    (self.glBindImageTexture_p)(unit, texture, level, layered, layer, access, format)
  }
  /// [glBindImageTextures](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindImageTextures.xhtml)
  /// * `textures` class: texture
  /// * `textures` len: count
  pub unsafe fn BindImageTextures(&self, first: GLuint, count: GLsizei, textures: *const GLuint) {
    (self.glBindImageTextures_p)(first, count, textures)
  }
  /// [glBindProgramPipeline](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindProgramPipeline.xhtml)
  /// * `pipeline` class: program pipeline
  pub unsafe fn BindProgramPipeline(&self, pipeline: GLuint) {
    (self.glBindProgramPipeline_p)(pipeline)
  }
  /// [glBindRenderbuffer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindRenderbuffer.xhtml)
  /// * `target` group: RenderbufferTarget
  /// * `renderbuffer` class: renderbuffer
  pub unsafe fn BindRenderbuffer(&self, target: RenderbufferTarget, renderbuffer: GLuint) {
    (self.glBindRenderbuffer_p)(target, renderbuffer)
  }
  /// [glBindSampler](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindSampler.xhtml)
  /// * `sampler` class: sampler
  pub unsafe fn BindSampler(&self, unit: GLuint, sampler: GLuint) {
    (self.glBindSampler_p)(unit, sampler)
  }
  /// [glBindSamplers](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindSamplers.xhtml)
  /// * `samplers` class: sampler
  /// * `samplers` len: count
  pub unsafe fn BindSamplers(&self, first: GLuint, count: GLsizei, samplers: *const GLuint) {
    (self.glBindSamplers_p)(first, count, samplers)
  }
  /// [glBindTexture](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindTexture.xhtml)
  /// * `target` group: TextureTarget
  /// * `texture` group: Texture
  /// * `texture` class: texture
  pub unsafe fn BindTexture(&self, target: TextureTarget, texture: GLuint) {
    (self.glBindTexture_p)(target, texture)
  }
  /// [glBindTextureUnit](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindTextureUnit.xhtml)
  /// * `texture` class: texture
  pub unsafe fn BindTextureUnit(&self, unit: GLuint, texture: GLuint) {
    (self.glBindTextureUnit_p)(unit, texture)
  }
  /// [glBindTextures](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindTextures.xhtml)
  /// * `textures` class: texture
  /// * `textures` len: count
  pub unsafe fn BindTextures(&self, first: GLuint, count: GLsizei, textures: *const GLuint) {
    (self.glBindTextures_p)(first, count, textures)
  }
  /// [glBindTransformFeedback](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindTransformFeedback.xhtml)
  /// * `target` group: BindTransformFeedbackTarget
  /// * `id` class: transform feedback
  pub unsafe fn BindTransformFeedback(&self, target: BindTransformFeedbackTarget, id: GLuint) {
    (self.glBindTransformFeedback_p)(target, id)
  }
  /// [glBindVertexArray](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindVertexArray.xhtml)
  /// * `array` class: vertex array
  pub unsafe fn BindVertexArray(&self, array: GLuint) {
    (self.glBindVertexArray_p)(array)
  }
  /// [glBindVertexBuffer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindVertexBuffer.xhtml)
  /// * `buffer` class: buffer
  /// * `offset` group: BufferOffset
  pub unsafe fn BindVertexBuffer(&self, bindingindex: GLuint, buffer: GLuint, offset: GLintptr, stride: GLsizei) {
    (self.glBindVertexBuffer_p)(bindingindex, buffer, offset, stride)
  }
  /// [glBindVertexBuffers](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindVertexBuffers.xhtml)
  /// * `buffers` class: buffer
  /// * `buffers` len: count
  /// * `offsets` len: count
  /// * `strides` len: count
  pub unsafe fn BindVertexBuffers(&self, first: GLuint, count: GLsizei, buffers: *const GLuint, offsets: *const GLintptr, strides: *const GLsizei) {
    (self.glBindVertexBuffers_p)(first, count, buffers, offsets, strides)
  }
  /// [glBlendColor](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBlendColor.xhtml)
  /// * `red` group: ColorF
  /// * `green` group: ColorF
  /// * `blue` group: ColorF
  /// * `alpha` group: ColorF
  pub unsafe fn BlendColor(&self, red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat) {
    (self.glBlendColor_p)(red, green, blue, alpha)
  }
  /// [glBlendEquation](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBlendEquation.xhtml)
  /// * `mode` group: BlendEquationModeEXT
  pub unsafe fn BlendEquation(&self, mode: BlendEquationModeEXT) {
    (self.glBlendEquation_p)(mode)
  }
  /// [glBlendEquationSeparate](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBlendEquationSeparate.xhtml)
  /// * `modeRGB` group: BlendEquationModeEXT
  /// * `modeAlpha` group: BlendEquationModeEXT
  pub unsafe fn BlendEquationSeparate(&self, modeRGB: BlendEquationModeEXT, modeAlpha: BlendEquationModeEXT) {
    (self.glBlendEquationSeparate_p)(modeRGB, modeAlpha)
  }
  /// [glBlendEquationSeparatei](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBlendEquationSeparatei.xhtml)
  /// * `modeRGB` group: BlendEquationModeEXT
  /// * `modeAlpha` group: BlendEquationModeEXT
  pub unsafe fn BlendEquationSeparatei(&self, buf: GLuint, modeRGB: BlendEquationModeEXT, modeAlpha: BlendEquationModeEXT) {
    (self.glBlendEquationSeparatei_p)(buf, modeRGB, modeAlpha)
  }
  /// [glBlendEquationi](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBlendEquationi.xhtml)
  /// * `mode` group: BlendEquationModeEXT
  pub unsafe fn BlendEquationi(&self, buf: GLuint, mode: BlendEquationModeEXT) {
    (self.glBlendEquationi_p)(buf, mode)
  }
  /// [glBlendFunc](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBlendFunc.xhtml)
  /// * `sfactor` group: BlendingFactor
  /// * `dfactor` group: BlendingFactor
  pub unsafe fn BlendFunc(&self, sfactor: BlendingFactor, dfactor: BlendingFactor) {
    (self.glBlendFunc_p)(sfactor, dfactor)
  }
  /// [glBlendFuncSeparate](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBlendFuncSeparate.xhtml)
  /// * `sfactorRGB` group: BlendingFactor
  /// * `dfactorRGB` group: BlendingFactor
  /// * `sfactorAlpha` group: BlendingFactor
  /// * `dfactorAlpha` group: BlendingFactor
  pub unsafe fn BlendFuncSeparate(&self, sfactorRGB: BlendingFactor, dfactorRGB: BlendingFactor, sfactorAlpha: BlendingFactor, dfactorAlpha: BlendingFactor) {
    (self.glBlendFuncSeparate_p)(sfactorRGB, dfactorRGB, sfactorAlpha, dfactorAlpha)
  }
  /// [glBlendFuncSeparatei](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBlendFuncSeparatei.xhtml)
  /// * `srcRGB` group: BlendingFactor
  /// * `dstRGB` group: BlendingFactor
  /// * `srcAlpha` group: BlendingFactor
  /// * `dstAlpha` group: BlendingFactor
  pub unsafe fn BlendFuncSeparatei(&self, buf: GLuint, srcRGB: BlendingFactor, dstRGB: BlendingFactor, srcAlpha: BlendingFactor, dstAlpha: BlendingFactor) {
    (self.glBlendFuncSeparatei_p)(buf, srcRGB, dstRGB, srcAlpha, dstAlpha)
  }
  /// [glBlendFunci](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBlendFunci.xhtml)
  /// * `src` group: BlendingFactor
  /// * `dst` group: BlendingFactor
  pub unsafe fn BlendFunci(&self, buf: GLuint, src: BlendingFactor, dst: BlendingFactor) {
    (self.glBlendFunci_p)(buf, src, dst)
  }
  /// [glBlitFramebuffer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBlitFramebuffer.xhtml)
  /// * `mask` group: ClearBufferMask
  /// * `filter` group: BlitFramebufferFilter
  pub unsafe fn BlitFramebuffer(&self, srcX0: GLint, srcY0: GLint, srcX1: GLint, srcY1: GLint, dstX0: GLint, dstY0: GLint, dstX1: GLint, dstY1: GLint, mask: GLbitfield, filter: BlitFramebufferFilter) {
    (self.glBlitFramebuffer_p)(srcX0, srcY0, srcX1, srcY1, dstX0, dstY0, dstX1, dstY1, mask, filter)
  }
  /// [glBlitNamedFramebuffer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBlitNamedFramebuffer.xhtml)
  /// * `readFramebuffer` class: framebuffer
  /// * `drawFramebuffer` class: framebuffer
  /// * `mask` group: ClearBufferMask
  /// * `filter` group: BlitFramebufferFilter
  pub unsafe fn BlitNamedFramebuffer(&self, readFramebuffer: GLuint, drawFramebuffer: GLuint, srcX0: GLint, srcY0: GLint, srcX1: GLint, srcY1: GLint, dstX0: GLint, dstY0: GLint, dstX1: GLint, dstY1: GLint, mask: GLbitfield, filter: BlitFramebufferFilter) {
    (self.glBlitNamedFramebuffer_p)(readFramebuffer, drawFramebuffer, srcX0, srcY0, srcX1, srcY1, dstX0, dstY0, dstX1, dstY1, mask, filter)
  }
  /// [glBufferData](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBufferData.xhtml)
  /// * `target` group: BufferTargetARB
  /// * `size` group: BufferSize
  /// * `data` len: size
  /// * `usage` group: BufferUsageARB
  pub unsafe fn BufferData(&self, target: BufferTargetARB, size: GLsizeiptr, data: *const void, usage: BufferUsageARB) {
    (self.glBufferData_p)(target, size, data, usage)
  }
  /// [glBufferStorage](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBufferStorage.xhtml)
  /// * `target` group: BufferStorageTarget
  /// * `data` len: size
  /// * `flags` group: BufferStorageMask
  pub unsafe fn BufferStorage(&self, target: BufferStorageTarget, size: GLsizeiptr, data: *const void, flags: GLbitfield) {
    (self.glBufferStorage_p)(target, size, data, flags)
  }
  /// [glBufferSubData](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBufferSubData.xhtml)
  /// * `target` group: BufferTargetARB
  /// * `offset` group: BufferOffset
  /// * `size` group: BufferSize
  /// * `data` len: size
  pub unsafe fn BufferSubData(&self, target: BufferTargetARB, offset: GLintptr, size: GLsizeiptr, data: *const void) {
    (self.glBufferSubData_p)(target, offset, size, data)
  }
  /// [glCheckFramebufferStatus](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCheckFramebufferStatus.xhtml)
  /// * `target` group: FramebufferTarget
  pub unsafe fn CheckFramebufferStatus(&self, target: FramebufferTarget) -> FramebufferStatus {
    (self.glCheckFramebufferStatus_p)(target)
  }
  /// [glCheckNamedFramebufferStatus](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCheckNamedFramebufferStatus.xhtml)
  /// * `framebuffer` class: framebuffer
  /// * `target` group: FramebufferTarget
  pub unsafe fn CheckNamedFramebufferStatus(&self, framebuffer: GLuint, target: FramebufferTarget) -> FramebufferStatus {
    (self.glCheckNamedFramebufferStatus_p)(framebuffer, target)
  }
  /// [glClampColor](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClampColor.xhtml)
  /// * `target` group: ClampColorTargetARB
  /// * `clamp` group: ClampColorModeARB
  pub unsafe fn ClampColor(&self, target: ClampColorTargetARB, clamp: ClampColorModeARB) {
    (self.glClampColor_p)(target, clamp)
  }
  /// [glClear](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClear.xhtml)
  /// * `mask` group: ClearBufferMask
  pub unsafe fn Clear(&self, mask: GLbitfield) {
    (self.glClear_p)(mask)
  }
  /// [glClearBufferData](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClearBufferData.xhtml)
  /// * `target` group: BufferStorageTarget
  /// * `internalformat` group: InternalFormat
  /// * `format` group: PixelFormat
  /// * `type` group: PixelType
  /// * `data` len: COMPSIZE(format,type)
  pub unsafe fn ClearBufferData(&self, target: BufferStorageTarget, internalformat: InternalFormat, format: PixelFormat, type_: PixelType, data: *const void) {
    (self.glClearBufferData_p)(target, internalformat, format, type_, data)
  }
  /// [glClearBufferSubData](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClearBufferSubData.xhtml)
  /// * `target` group: BufferTargetARB
  /// * `internalformat` group: InternalFormat
  /// * `offset` group: BufferOffset
  /// * `size` group: BufferSize
  /// * `format` group: PixelFormat
  /// * `type` group: PixelType
  /// * `data` len: COMPSIZE(format,type)
  pub unsafe fn ClearBufferSubData(&self, target: BufferTargetARB, internalformat: InternalFormat, offset: GLintptr, size: GLsizeiptr, format: PixelFormat, type_: PixelType, data: *const void) {
    (self.glClearBufferSubData_p)(target, internalformat, offset, size, format, type_, data)
  }
  /// [glClearBufferfi](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClearBufferfi.xhtml)
  /// * `buffer` group: Buffer
  /// * `drawbuffer` group: DrawBufferName
  pub unsafe fn ClearBufferfi(&self, buffer: Buffer, drawbuffer: GLint, depth: GLfloat, stencil: GLint) {
    (self.glClearBufferfi_p)(buffer, drawbuffer, depth, stencil)
  }
  /// [glClearBufferfv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClearBufferfv.xhtml)
  /// * `buffer` group: Buffer
  /// * `drawbuffer` group: DrawBufferName
  /// * `value` len: COMPSIZE(buffer)
  pub unsafe fn ClearBufferfv(&self, buffer: Buffer, drawbuffer: GLint, value: *const GLfloat) {
    (self.glClearBufferfv_p)(buffer, drawbuffer, value)
  }
  /// [glClearBufferiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClearBufferiv.xhtml)
  /// * `buffer` group: Buffer
  /// * `drawbuffer` group: DrawBufferName
  /// * `value` len: COMPSIZE(buffer)
  pub unsafe fn ClearBufferiv(&self, buffer: Buffer, drawbuffer: GLint, value: *const GLint) {
    (self.glClearBufferiv_p)(buffer, drawbuffer, value)
  }
  /// [glClearBufferuiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClearBufferuiv.xhtml)
  /// * `buffer` group: Buffer
  /// * `drawbuffer` group: DrawBufferName
  /// * `value` len: COMPSIZE(buffer)
  pub unsafe fn ClearBufferuiv(&self, buffer: Buffer, drawbuffer: GLint, value: *const GLuint) {
    (self.glClearBufferuiv_p)(buffer, drawbuffer, value)
  }
  /// [glClearColor](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClearColor.xhtml)
  /// * `red` group: ColorF
  /// * `green` group: ColorF
  /// * `blue` group: ColorF
  /// * `alpha` group: ColorF
  pub unsafe fn ClearColor(&self, red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat) {
    (self.glClearColor_p)(red, green, blue, alpha)
  }
  /// [glClearDepth](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClearDepth.xhtml)
  pub unsafe fn ClearDepth(&self, depth: GLdouble) {
    (self.glClearDepth_p)(depth)
  }
  /// [glClearDepthf](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClearDepthf.xhtml)
  pub unsafe fn ClearDepthf(&self, d: GLfloat) {
    (self.glClearDepthf_p)(d)
  }
  /// [glClearNamedBufferData](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClearNamedBufferData.xhtml)
  /// * `buffer` class: buffer
  /// * `internalformat` group: InternalFormat
  /// * `format` group: PixelFormat
  /// * `type` group: PixelType
  pub unsafe fn ClearNamedBufferData(&self, buffer: GLuint, internalformat: InternalFormat, format: PixelFormat, type_: PixelType, data: *const void) {
    (self.glClearNamedBufferData_p)(buffer, internalformat, format, type_, data)
  }
  /// [glClearNamedBufferSubData](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClearNamedBufferSubData.xhtml)
  /// * `buffer` class: buffer
  /// * `internalformat` group: InternalFormat
  /// * `size` group: BufferSize
  /// * `format` group: PixelFormat
  /// * `type` group: PixelType
  pub unsafe fn ClearNamedBufferSubData(&self, buffer: GLuint, internalformat: InternalFormat, offset: GLintptr, size: GLsizeiptr, format: PixelFormat, type_: PixelType, data: *const void) {
    (self.glClearNamedBufferSubData_p)(buffer, internalformat, offset, size, format, type_, data)
  }
  /// [glClearNamedFramebufferfi](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClearNamedFramebufferfi.xhtml)
  /// * `framebuffer` class: framebuffer
  /// * `buffer` group: Buffer
  pub unsafe fn ClearNamedFramebufferfi(&self, framebuffer: GLuint, buffer: Buffer, drawbuffer: GLint, depth: GLfloat, stencil: GLint) {
    (self.glClearNamedFramebufferfi_p)(framebuffer, buffer, drawbuffer, depth, stencil)
  }
  /// [glClearNamedFramebufferfv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClearNamedFramebufferfv.xhtml)
  /// * `framebuffer` class: framebuffer
  /// * `buffer` group: Buffer
  pub unsafe fn ClearNamedFramebufferfv(&self, framebuffer: GLuint, buffer: Buffer, drawbuffer: GLint, value: *const GLfloat) {
    (self.glClearNamedFramebufferfv_p)(framebuffer, buffer, drawbuffer, value)
  }
  /// [glClearNamedFramebufferiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClearNamedFramebufferiv.xhtml)
  /// * `framebuffer` class: framebuffer
  /// * `buffer` group: Buffer
  pub unsafe fn ClearNamedFramebufferiv(&self, framebuffer: GLuint, buffer: Buffer, drawbuffer: GLint, value: *const GLint) {
    (self.glClearNamedFramebufferiv_p)(framebuffer, buffer, drawbuffer, value)
  }
  /// [glClearNamedFramebufferuiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClearNamedFramebufferuiv.xhtml)
  /// * `framebuffer` class: framebuffer
  /// * `buffer` group: Buffer
  pub unsafe fn ClearNamedFramebufferuiv(&self, framebuffer: GLuint, buffer: Buffer, drawbuffer: GLint, value: *const GLuint) {
    (self.glClearNamedFramebufferuiv_p)(framebuffer, buffer, drawbuffer, value)
  }
  /// [glClearStencil](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClearStencil.xhtml)
  /// * `s` group: StencilValue
  pub unsafe fn ClearStencil(&self, s: GLint) {
    (self.glClearStencil_p)(s)
  }
  /// [glClearTexImage](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClearTexImage.xhtml)
  /// * `texture` class: texture
  /// * `format` group: PixelFormat
  /// * `type` group: PixelType
  /// * `data` len: COMPSIZE(format,type)
  pub unsafe fn ClearTexImage(&self, texture: GLuint, level: GLint, format: PixelFormat, type_: PixelType, data: *const void) {
    (self.glClearTexImage_p)(texture, level, format, type_, data)
  }
  /// [glClearTexSubImage](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClearTexSubImage.xhtml)
  /// * `texture` class: texture
  /// * `format` group: PixelFormat
  /// * `type` group: PixelType
  /// * `data` len: COMPSIZE(format,type)
  pub unsafe fn ClearTexSubImage(&self, texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: PixelFormat, type_: PixelType, data: *const void) {
    (self.glClearTexSubImage_p)(texture, level, xoffset, yoffset, zoffset, width, height, depth, format, type_, data)
  }
  /// [glClientWaitSync](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClientWaitSync.xhtml)
  /// * `sync` group: sync
  /// * `sync` class: sync
  /// * `flags` group: SyncObjectMask
  pub unsafe fn ClientWaitSync(&self, sync: GLsync, flags: GLbitfield, timeout: GLuint64) -> SyncStatus {
    (self.glClientWaitSync_p)(sync, flags, timeout)
  }
  /// [glClipControl](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClipControl.xhtml)
  /// * `origin` group: ClipControlOrigin
  /// * `depth` group: ClipControlDepth
  pub unsafe fn ClipControl(&self, origin: ClipControlOrigin, depth: ClipControlDepth) {
    (self.glClipControl_p)(origin, depth)
  }
  /// [glColorMask](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glColorMask.xhtml)
  /// * `red` group: Boolean
  /// * `green` group: Boolean
  /// * `blue` group: Boolean
  /// * `alpha` group: Boolean
  pub unsafe fn ColorMask(&self, red: GLboolean, green: GLboolean, blue: GLboolean, alpha: GLboolean) {
    (self.glColorMask_p)(red, green, blue, alpha)
  }
  /// [glColorMaski](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glColorMaski.xhtml)
  /// * `r` group: Boolean
  /// * `g` group: Boolean
  /// * `b` group: Boolean
  /// * `a` group: Boolean
  pub unsafe fn ColorMaski(&self, index: GLuint, r: GLboolean, g: GLboolean, b: GLboolean, a: GLboolean) {
    (self.glColorMaski_p)(index, r, g, b, a)
  }
  /// [glColorP3ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glColorP3ui.xhtml)
  /// * `type` group: ColorPointerType
  pub unsafe fn ColorP3ui(&self, type_: ColorPointerType, color: GLuint) {
    (self.glColorP3ui_p)(type_, color)
  }
  /// [glColorP3uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glColorP3uiv.xhtml)
  /// * `type` group: ColorPointerType
  pub unsafe fn ColorP3uiv(&self, type_: ColorPointerType, color: *const GLuint) {
    (self.glColorP3uiv_p)(type_, color)
  }
  /// [glColorP4ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glColorP4ui.xhtml)
  /// * `type` group: ColorPointerType
  pub unsafe fn ColorP4ui(&self, type_: ColorPointerType, color: GLuint) {
    (self.glColorP4ui_p)(type_, color)
  }
  /// [glColorP4uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glColorP4uiv.xhtml)
  /// * `type` group: ColorPointerType
  pub unsafe fn ColorP4uiv(&self, type_: ColorPointerType, color: *const GLuint) {
    (self.glColorP4uiv_p)(type_, color)
  }
  /// [glCompileShader](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCompileShader.xhtml)
  /// * `shader` class: shader
  pub unsafe fn CompileShader(&self, shader: GLuint) {
    (self.glCompileShader_p)(shader)
  }
  /// [glCompressedTexImage1D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCompressedTexImage1D.xhtml)
  /// * `target` group: TextureTarget
  /// * `level` group: CheckedInt32
  /// * `internalformat` group: InternalFormat
  /// * `border` group: CheckedInt32
  /// * `data` group: CompressedTextureARB
  /// * `data` len: imageSize
  pub unsafe fn CompressedTexImage1D(&self, target: TextureTarget, level: GLint, internalformat: InternalFormat, width: GLsizei, border: GLint, imageSize: GLsizei, data: *const void) {
    (self.glCompressedTexImage1D_p)(target, level, internalformat, width, border, imageSize, data)
  }
  /// [glCompressedTexImage2D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCompressedTexImage2D.xhtml)
  /// * `target` group: TextureTarget
  /// * `level` group: CheckedInt32
  /// * `internalformat` group: InternalFormat
  /// * `border` group: CheckedInt32
  /// * `data` group: CompressedTextureARB
  /// * `data` len: imageSize
  pub unsafe fn CompressedTexImage2D(&self, target: TextureTarget, level: GLint, internalformat: InternalFormat, width: GLsizei, height: GLsizei, border: GLint, imageSize: GLsizei, data: *const void) {
    (self.glCompressedTexImage2D_p)(target, level, internalformat, width, height, border, imageSize, data)
  }
  /// [glCompressedTexImage3D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCompressedTexImage3D.xhtml)
  /// * `target` group: TextureTarget
  /// * `level` group: CheckedInt32
  /// * `internalformat` group: InternalFormat
  /// * `border` group: CheckedInt32
  /// * `data` group: CompressedTextureARB
  /// * `data` len: imageSize
  pub unsafe fn CompressedTexImage3D(&self, target: TextureTarget, level: GLint, internalformat: InternalFormat, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, imageSize: GLsizei, data: *const void) {
    (self.glCompressedTexImage3D_p)(target, level, internalformat, width, height, depth, border, imageSize, data)
  }
  /// [glCompressedTexSubImage1D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCompressedTexSubImage1D.xhtml)
  /// * `target` group: TextureTarget
  /// * `level` group: CheckedInt32
  /// * `xoffset` group: CheckedInt32
  /// * `format` group: PixelFormat
  /// * `data` group: CompressedTextureARB
  /// * `data` len: imageSize
  pub unsafe fn CompressedTexSubImage1D(&self, target: TextureTarget, level: GLint, xoffset: GLint, width: GLsizei, format: PixelFormat, imageSize: GLsizei, data: *const void) {
    (self.glCompressedTexSubImage1D_p)(target, level, xoffset, width, format, imageSize, data)
  }
  /// [glCompressedTexSubImage2D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCompressedTexSubImage2D.xhtml)
  /// * `target` group: TextureTarget
  /// * `level` group: CheckedInt32
  /// * `xoffset` group: CheckedInt32
  /// * `yoffset` group: CheckedInt32
  /// * `format` group: PixelFormat
  /// * `data` group: CompressedTextureARB
  /// * `data` len: imageSize
  pub unsafe fn CompressedTexSubImage2D(&self, target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: PixelFormat, imageSize: GLsizei, data: *const void) {
    (self.glCompressedTexSubImage2D_p)(target, level, xoffset, yoffset, width, height, format, imageSize, data)
  }
  /// [glCompressedTexSubImage3D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCompressedTexSubImage3D.xhtml)
  /// * `target` group: TextureTarget
  /// * `level` group: CheckedInt32
  /// * `xoffset` group: CheckedInt32
  /// * `yoffset` group: CheckedInt32
  /// * `zoffset` group: CheckedInt32
  /// * `format` group: PixelFormat
  /// * `data` group: CompressedTextureARB
  /// * `data` len: imageSize
  pub unsafe fn CompressedTexSubImage3D(&self, target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: PixelFormat, imageSize: GLsizei, data: *const void) {
    (self.glCompressedTexSubImage3D_p)(target, level, xoffset, yoffset, zoffset, width, height, depth, format, imageSize, data)
  }
  /// [glCompressedTextureSubImage1D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCompressedTextureSubImage1D.xhtml)
  /// * `texture` class: texture
  /// * `format` group: PixelFormat
  pub unsafe fn CompressedTextureSubImage1D(&self, texture: GLuint, level: GLint, xoffset: GLint, width: GLsizei, format: PixelFormat, imageSize: GLsizei, data: *const void) {
    (self.glCompressedTextureSubImage1D_p)(texture, level, xoffset, width, format, imageSize, data)
  }
  /// [glCompressedTextureSubImage2D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCompressedTextureSubImage2D.xhtml)
  /// * `texture` class: texture
  /// * `format` group: PixelFormat
  pub unsafe fn CompressedTextureSubImage2D(&self, texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: PixelFormat, imageSize: GLsizei, data: *const void) {
    (self.glCompressedTextureSubImage2D_p)(texture, level, xoffset, yoffset, width, height, format, imageSize, data)
  }
  /// [glCompressedTextureSubImage3D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCompressedTextureSubImage3D.xhtml)
  /// * `texture` class: texture
  /// * `format` group: PixelFormat
  pub unsafe fn CompressedTextureSubImage3D(&self, texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: PixelFormat, imageSize: GLsizei, data: *const void) {
    (self.glCompressedTextureSubImage3D_p)(texture, level, xoffset, yoffset, zoffset, width, height, depth, format, imageSize, data)
  }
  /// [glCopyBufferSubData](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCopyBufferSubData.xhtml)
  /// * `readTarget` group: CopyBufferSubDataTarget
  /// * `writeTarget` group: CopyBufferSubDataTarget
  /// * `readOffset` group: BufferOffset
  /// * `writeOffset` group: BufferOffset
  /// * `size` group: BufferSize
  pub unsafe fn CopyBufferSubData(&self, readTarget: CopyBufferSubDataTarget, writeTarget: CopyBufferSubDataTarget, readOffset: GLintptr, writeOffset: GLintptr, size: GLsizeiptr) {
    (self.glCopyBufferSubData_p)(readTarget, writeTarget, readOffset, writeOffset, size)
  }
  /// [glCopyImageSubData](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCopyImageSubData.xhtml)
  /// * `srcTarget` group: CopyImageSubDataTarget
  /// * `dstTarget` group: CopyImageSubDataTarget
  pub unsafe fn CopyImageSubData(&self, srcName: GLuint, srcTarget: CopyImageSubDataTarget, srcLevel: GLint, srcX: GLint, srcY: GLint, srcZ: GLint, dstName: GLuint, dstTarget: CopyImageSubDataTarget, dstLevel: GLint, dstX: GLint, dstY: GLint, dstZ: GLint, srcWidth: GLsizei, srcHeight: GLsizei, srcDepth: GLsizei) {
    (self.glCopyImageSubData_p)(srcName, srcTarget, srcLevel, srcX, srcY, srcZ, dstName, dstTarget, dstLevel, dstX, dstY, dstZ, srcWidth, srcHeight, srcDepth)
  }
  /// [glCopyNamedBufferSubData](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCopyNamedBufferSubData.xhtml)
  /// * `readBuffer` class: buffer
  /// * `writeBuffer` class: buffer
  /// * `size` group: BufferSize
  pub unsafe fn CopyNamedBufferSubData(&self, readBuffer: GLuint, writeBuffer: GLuint, readOffset: GLintptr, writeOffset: GLintptr, size: GLsizeiptr) {
    (self.glCopyNamedBufferSubData_p)(readBuffer, writeBuffer, readOffset, writeOffset, size)
  }
  /// [glCopyTexImage1D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCopyTexImage1D.xhtml)
  /// * `target` group: TextureTarget
  /// * `level` group: CheckedInt32
  /// * `internalformat` group: InternalFormat
  /// * `x` group: WinCoord
  /// * `y` group: WinCoord
  /// * `border` group: CheckedInt32
  pub unsafe fn CopyTexImage1D(&self, target: TextureTarget, level: GLint, internalformat: InternalFormat, x: GLint, y: GLint, width: GLsizei, border: GLint) {
    (self.glCopyTexImage1D_p)(target, level, internalformat, x, y, width, border)
  }
  /// [glCopyTexImage2D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCopyTexImage2D.xhtml)
  /// * `target` group: TextureTarget
  /// * `level` group: CheckedInt32
  /// * `internalformat` group: InternalFormat
  /// * `x` group: WinCoord
  /// * `y` group: WinCoord
  /// * `border` group: CheckedInt32
  pub unsafe fn CopyTexImage2D(&self, target: TextureTarget, level: GLint, internalformat: InternalFormat, x: GLint, y: GLint, width: GLsizei, height: GLsizei, border: GLint) {
    (self.glCopyTexImage2D_p)(target, level, internalformat, x, y, width, height, border)
  }
  /// [glCopyTexSubImage1D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCopyTexSubImage1D.xhtml)
  /// * `target` group: TextureTarget
  /// * `level` group: CheckedInt32
  /// * `xoffset` group: CheckedInt32
  /// * `x` group: WinCoord
  /// * `y` group: WinCoord
  pub unsafe fn CopyTexSubImage1D(&self, target: TextureTarget, level: GLint, xoffset: GLint, x: GLint, y: GLint, width: GLsizei) {
    (self.glCopyTexSubImage1D_p)(target, level, xoffset, x, y, width)
  }
  /// [glCopyTexSubImage2D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCopyTexSubImage2D.xhtml)
  /// * `target` group: TextureTarget
  /// * `level` group: CheckedInt32
  /// * `xoffset` group: CheckedInt32
  /// * `yoffset` group: CheckedInt32
  /// * `x` group: WinCoord
  /// * `y` group: WinCoord
  pub unsafe fn CopyTexSubImage2D(&self, target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
    (self.glCopyTexSubImage2D_p)(target, level, xoffset, yoffset, x, y, width, height)
  }
  /// [glCopyTexSubImage3D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCopyTexSubImage3D.xhtml)
  /// * `target` group: TextureTarget
  /// * `level` group: CheckedInt32
  /// * `xoffset` group: CheckedInt32
  /// * `yoffset` group: CheckedInt32
  /// * `zoffset` group: CheckedInt32
  /// * `x` group: WinCoord
  /// * `y` group: WinCoord
  pub unsafe fn CopyTexSubImage3D(&self, target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
    (self.glCopyTexSubImage3D_p)(target, level, xoffset, yoffset, zoffset, x, y, width, height)
  }
  /// [glCopyTextureSubImage1D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCopyTextureSubImage1D.xhtml)
  /// * `texture` class: texture
  pub unsafe fn CopyTextureSubImage1D(&self, texture: GLuint, level: GLint, xoffset: GLint, x: GLint, y: GLint, width: GLsizei) {
    (self.glCopyTextureSubImage1D_p)(texture, level, xoffset, x, y, width)
  }
  /// [glCopyTextureSubImage2D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCopyTextureSubImage2D.xhtml)
  /// * `texture` class: texture
  pub unsafe fn CopyTextureSubImage2D(&self, texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
    (self.glCopyTextureSubImage2D_p)(texture, level, xoffset, yoffset, x, y, width, height)
  }
  /// [glCopyTextureSubImage3D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCopyTextureSubImage3D.xhtml)
  /// * `texture` class: texture
  pub unsafe fn CopyTextureSubImage3D(&self, texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
    (self.glCopyTextureSubImage3D_p)(texture, level, xoffset, yoffset, zoffset, x, y, width, height)
  }
  /// [glCreateBuffers](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCreateBuffers.xhtml)
  /// * `buffers` class: buffer
  /// * `buffers` len: n
  pub unsafe fn CreateBuffers(&self, n: GLsizei, buffers: *mut GLuint) {
    (self.glCreateBuffers_p)(n, buffers)
  }
  /// [glCreateFramebuffers](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCreateFramebuffers.xhtml)
  /// * `framebuffers` class: framebuffer
  /// * `framebuffers` len: n
  pub unsafe fn CreateFramebuffers(&self, n: GLsizei, framebuffers: *mut GLuint) {
    (self.glCreateFramebuffers_p)(n, framebuffers)
  }
  /// [glCreateProgram](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCreateProgram.xhtml)
  pub unsafe fn CreateProgram(&self) -> GLuint {
    (self.glCreateProgram_p)()
  }
  /// [glCreateProgramPipelines](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCreateProgramPipelines.xhtml)
  /// * `pipelines` class: program pipeline
  /// * `pipelines` len: n
  pub unsafe fn CreateProgramPipelines(&self, n: GLsizei, pipelines: *mut GLuint) {
    (self.glCreateProgramPipelines_p)(n, pipelines)
  }
  /// [glCreateQueries](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCreateQueries.xhtml)
  /// * `target` group: QueryTarget
  /// * `ids` class: query
  /// * `ids` len: n
  pub unsafe fn CreateQueries(&self, target: QueryTarget, n: GLsizei, ids: *mut GLuint) {
    (self.glCreateQueries_p)(target, n, ids)
  }
  /// [glCreateRenderbuffers](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCreateRenderbuffers.xhtml)
  /// * `renderbuffers` class: renderbuffer
  /// * `renderbuffers` len: n
  pub unsafe fn CreateRenderbuffers(&self, n: GLsizei, renderbuffers: *mut GLuint) {
    (self.glCreateRenderbuffers_p)(n, renderbuffers)
  }
  /// [glCreateSamplers](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCreateSamplers.xhtml)
  /// * `samplers` class: sampler
  /// * `samplers` len: n
  pub unsafe fn CreateSamplers(&self, n: GLsizei, samplers: *mut GLuint) {
    (self.glCreateSamplers_p)(n, samplers)
  }
  /// [glCreateShader](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCreateShader.xhtml)
  /// * `type` group: ShaderType
  pub unsafe fn CreateShader(&self, type_: ShaderType) -> GLuint {
    (self.glCreateShader_p)(type_)
  }
  /// [glCreateShaderProgramv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCreateShaderProgramv.xhtml)
  /// * `type` group: ShaderType
  /// * `strings` len: count
  pub unsafe fn CreateShaderProgramv(&self, type_: ShaderType, count: GLsizei, strings: *const *const GLchar) -> GLuint {
    (self.glCreateShaderProgramv_p)(type_, count, strings)
  }
  /// [glCreateTextures](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCreateTextures.xhtml)
  /// * `target` group: TextureTarget
  /// * `textures` class: texture
  /// * `textures` len: n
  pub unsafe fn CreateTextures(&self, target: TextureTarget, n: GLsizei, textures: *mut GLuint) {
    (self.glCreateTextures_p)(target, n, textures)
  }
  /// [glCreateTransformFeedbacks](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCreateTransformFeedbacks.xhtml)
  /// * `ids` class: transform feedback
  /// * `ids` len: n
  pub unsafe fn CreateTransformFeedbacks(&self, n: GLsizei, ids: *mut GLuint) {
    (self.glCreateTransformFeedbacks_p)(n, ids)
  }
  /// [glCreateVertexArrays](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCreateVertexArrays.xhtml)
  /// * `arrays` class: vertex array
  /// * `arrays` len: n
  pub unsafe fn CreateVertexArrays(&self, n: GLsizei, arrays: *mut GLuint) {
    (self.glCreateVertexArrays_p)(n, arrays)
  }
  /// [glCullFace](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCullFace.xhtml)
  /// * `mode` group: CullFaceMode
  pub unsafe fn CullFace(&self, mode: CullFaceMode) {
    (self.glCullFace_p)(mode)
  }
  /// [glDebugMessageCallback](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDebugMessageCallback.xhtml)
  pub unsafe fn DebugMessageCallback(&self, callback: GLDEBUGPROC, userParam: *const void) {
    (self.glDebugMessageCallback_p)(callback, userParam)
  }
  /// [glDebugMessageControl](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDebugMessageControl.xhtml)
  /// * `source` group: DebugSource
  /// * `type` group: DebugType
  /// * `severity` group: DebugSeverity
  /// * `ids` len: count
  /// * `enabled` group: Boolean
  pub unsafe fn DebugMessageControl(&self, source: DebugSource, type_: DebugType, severity: DebugSeverity, count: GLsizei, ids: *const GLuint, enabled: GLboolean) {
    (self.glDebugMessageControl_p)(source, type_, severity, count, ids, enabled)
  }
  /// [glDebugMessageInsert](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDebugMessageInsert.xhtml)
  /// * `source` group: DebugSource
  /// * `type` group: DebugType
  /// * `severity` group: DebugSeverity
  /// * `buf` len: COMPSIZE(buf,length)
  pub unsafe fn DebugMessageInsert(&self, source: DebugSource, type_: DebugType, id: GLuint, severity: DebugSeverity, length: GLsizei, buf: *const GLchar) {
    (self.glDebugMessageInsert_p)(source, type_, id, severity, length, buf)
  }
  /// [glDeleteBuffers](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeleteBuffers.xhtml)
  /// * `buffers` class: buffer
  /// * `buffers` len: n
  pub unsafe fn DeleteBuffers(&self, n: GLsizei, buffers: *const GLuint) {
    (self.glDeleteBuffers_p)(n, buffers)
  }
  /// [glDeleteFramebuffers](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeleteFramebuffers.xhtml)
  /// * `framebuffers` class: framebuffer
  /// * `framebuffers` len: n
  pub unsafe fn DeleteFramebuffers(&self, n: GLsizei, framebuffers: *const GLuint) {
    (self.glDeleteFramebuffers_p)(n, framebuffers)
  }
  /// [glDeleteProgram](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeleteProgram.xhtml)
  /// * `program` class: program
  pub unsafe fn DeleteProgram(&self, program: GLuint) {
    (self.glDeleteProgram_p)(program)
  }
  /// [glDeleteProgramPipelines](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeleteProgramPipelines.xhtml)
  /// * `pipelines` class: program pipeline
  /// * `pipelines` len: n
  pub unsafe fn DeleteProgramPipelines(&self, n: GLsizei, pipelines: *const GLuint) {
    (self.glDeleteProgramPipelines_p)(n, pipelines)
  }
  /// [glDeleteQueries](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeleteQueries.xhtml)
  /// * `ids` class: query
  /// * `ids` len: n
  pub unsafe fn DeleteQueries(&self, n: GLsizei, ids: *const GLuint) {
    (self.glDeleteQueries_p)(n, ids)
  }
  /// [glDeleteRenderbuffers](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeleteRenderbuffers.xhtml)
  /// * `renderbuffers` class: renderbuffer
  /// * `renderbuffers` len: n
  pub unsafe fn DeleteRenderbuffers(&self, n: GLsizei, renderbuffers: *const GLuint) {
    (self.glDeleteRenderbuffers_p)(n, renderbuffers)
  }
  /// [glDeleteSamplers](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeleteSamplers.xhtml)
  /// * `samplers` class: sampler
  /// * `samplers` len: count
  pub unsafe fn DeleteSamplers(&self, count: GLsizei, samplers: *const GLuint) {
    (self.glDeleteSamplers_p)(count, samplers)
  }
  /// [glDeleteShader](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeleteShader.xhtml)
  /// * `shader` class: shader
  pub unsafe fn DeleteShader(&self, shader: GLuint) {
    (self.glDeleteShader_p)(shader)
  }
  /// [glDeleteSync](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeleteSync.xhtml)
  /// * `sync` group: sync
  /// * `sync` class: sync
  pub unsafe fn DeleteSync(&self, sync: GLsync) {
    (self.glDeleteSync_p)(sync)
  }
  /// [glDeleteTextures](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeleteTextures.xhtml)
  /// * `textures` group: Texture
  /// * `textures` class: texture
  /// * `textures` len: n
  pub unsafe fn DeleteTextures(&self, n: GLsizei, textures: *const GLuint) {
    (self.glDeleteTextures_p)(n, textures)
  }
  /// [glDeleteTransformFeedbacks](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeleteTransformFeedbacks.xhtml)
  /// * `ids` class: transform feedback
  /// * `ids` len: n
  pub unsafe fn DeleteTransformFeedbacks(&self, n: GLsizei, ids: *const GLuint) {
    (self.glDeleteTransformFeedbacks_p)(n, ids)
  }
  /// [glDeleteVertexArrays](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeleteVertexArrays.xhtml)
  /// * `arrays` class: vertex array
  /// * `arrays` len: n
  pub unsafe fn DeleteVertexArrays(&self, n: GLsizei, arrays: *const GLuint) {
    (self.glDeleteVertexArrays_p)(n, arrays)
  }
  /// [glDepthFunc](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDepthFunc.xhtml)
  /// * `func` group: DepthFunction
  pub unsafe fn DepthFunc(&self, func: DepthFunction) {
    (self.glDepthFunc_p)(func)
  }
  /// [glDepthMask](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDepthMask.xhtml)
  /// * `flag` group: Boolean
  pub unsafe fn DepthMask(&self, flag: GLboolean) {
    (self.glDepthMask_p)(flag)
  }
  /// [glDepthRange](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDepthRange.xhtml)
  pub unsafe fn DepthRange(&self, n: GLdouble, f: GLdouble) {
    (self.glDepthRange_p)(n, f)
  }
  /// [glDepthRangeArrayv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDepthRangeArrayv.xhtml)
  /// * `v` len: COMPSIZE(count)
  pub unsafe fn DepthRangeArrayv(&self, first: GLuint, count: GLsizei, v: *const GLdouble) {
    (self.glDepthRangeArrayv_p)(first, count, v)
  }
  /// [glDepthRangeIndexed](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDepthRangeIndexed.xhtml)
  pub unsafe fn DepthRangeIndexed(&self, index: GLuint, n: GLdouble, f: GLdouble) {
    (self.glDepthRangeIndexed_p)(index, n, f)
  }
  /// [glDepthRangef](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDepthRangef.xhtml)
  pub unsafe fn DepthRangef(&self, n: GLfloat, f: GLfloat) {
    (self.glDepthRangef_p)(n, f)
  }
  /// [glDetachShader](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDetachShader.xhtml)
  /// * `program` class: program
  /// * `shader` class: shader
  pub unsafe fn DetachShader(&self, program: GLuint, shader: GLuint) {
    (self.glDetachShader_p)(program, shader)
  }
  /// [glDisable](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDisable.xhtml)
  /// * `cap` group: EnableCap
  pub unsafe fn Disable(&self, cap: EnableCap) {
    (self.glDisable_p)(cap)
  }
  /// [glDisableVertexArrayAttrib](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDisableVertexArrayAttrib.xhtml)
  /// * `vaobj` class: vertex array
  pub unsafe fn DisableVertexArrayAttrib(&self, vaobj: GLuint, index: GLuint) {
    (self.glDisableVertexArrayAttrib_p)(vaobj, index)
  }
  /// [glDisableVertexAttribArray](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDisableVertexAttribArray.xhtml)
  pub unsafe fn DisableVertexAttribArray(&self, index: GLuint) {
    (self.glDisableVertexAttribArray_p)(index)
  }
  /// [glDisablei](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDisablei.xhtml)
  /// * `target` group: EnableCap
  pub unsafe fn Disablei(&self, target: EnableCap, index: GLuint) {
    (self.glDisablei_p)(target, index)
  }
  /// [glDispatchCompute](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDispatchCompute.xhtml)
  pub unsafe fn DispatchCompute(&self, num_groups_x: GLuint, num_groups_y: GLuint, num_groups_z: GLuint) {
    (self.glDispatchCompute_p)(num_groups_x, num_groups_y, num_groups_z)
  }
  /// [glDispatchComputeIndirect](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDispatchComputeIndirect.xhtml)
  /// * `indirect` group: BufferOffset
  pub unsafe fn DispatchComputeIndirect(&self, indirect: GLintptr) {
    (self.glDispatchComputeIndirect_p)(indirect)
  }
  /// [glDrawArrays](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawArrays.xhtml)
  /// * `mode` group: PrimitiveType
  pub unsafe fn DrawArrays(&self, mode: PrimitiveType, first: GLint, count: GLsizei) {
    (self.glDrawArrays_p)(mode, first, count)
  }
  /// [glDrawArraysIndirect](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawArraysIndirect.xhtml)
  /// * `mode` group: PrimitiveType
  pub unsafe fn DrawArraysIndirect(&self, mode: PrimitiveType, indirect: *const void) {
    (self.glDrawArraysIndirect_p)(mode, indirect)
  }
  /// [glDrawArraysInstanced](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawArraysInstanced.xhtml)
  /// * `mode` group: PrimitiveType
  pub unsafe fn DrawArraysInstanced(&self, mode: PrimitiveType, first: GLint, count: GLsizei, instancecount: GLsizei) {
    (self.glDrawArraysInstanced_p)(mode, first, count, instancecount)
  }
  /// [glDrawArraysInstancedBaseInstance](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawArraysInstancedBaseInstance.xhtml)
  /// * `mode` group: PrimitiveType
  pub unsafe fn DrawArraysInstancedBaseInstance(&self, mode: PrimitiveType, first: GLint, count: GLsizei, instancecount: GLsizei, baseinstance: GLuint) {
    (self.glDrawArraysInstancedBaseInstance_p)(mode, first, count, instancecount, baseinstance)
  }
  /// [glDrawBuffer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawBuffer.xhtml)
  /// * `buf` group: DrawBufferMode
  pub unsafe fn DrawBuffer(&self, buf: DrawBufferMode) {
    (self.glDrawBuffer_p)(buf)
  }
  /// [glDrawBuffers](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawBuffers.xhtml)
  /// * `bufs` group: DrawBufferMode
  /// * `bufs` len: n
  pub unsafe fn DrawBuffers(&self, n: GLsizei, bufs: *const DrawBufferMode) {
    (self.glDrawBuffers_p)(n, bufs)
  }
  /// [glDrawElements](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawElements.xhtml)
  /// * `mode` group: PrimitiveType
  /// * `type` group: DrawElementsType
  /// * `indices` len: COMPSIZE(count,type)
  pub unsafe fn DrawElements(&self, mode: PrimitiveType, count: GLsizei, type_: DrawElementsType, indices: *const void) {
    (self.glDrawElements_p)(mode, count, type_, indices)
  }
  /// [glDrawElementsBaseVertex](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawElementsBaseVertex.xhtml)
  /// * `mode` group: PrimitiveType
  /// * `type` group: DrawElementsType
  /// * `indices` len: COMPSIZE(count,type)
  pub unsafe fn DrawElementsBaseVertex(&self, mode: PrimitiveType, count: GLsizei, type_: DrawElementsType, indices: *const void, basevertex: GLint) {
    (self.glDrawElementsBaseVertex_p)(mode, count, type_, indices, basevertex)
  }
  /// [glDrawElementsIndirect](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawElementsIndirect.xhtml)
  /// * `mode` group: PrimitiveType
  /// * `type` group: DrawElementsType
  pub unsafe fn DrawElementsIndirect(&self, mode: PrimitiveType, type_: DrawElementsType, indirect: *const void) {
    (self.glDrawElementsIndirect_p)(mode, type_, indirect)
  }
  /// [glDrawElementsInstanced](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawElementsInstanced.xhtml)
  /// * `mode` group: PrimitiveType
  /// * `type` group: DrawElementsType
  /// * `indices` len: COMPSIZE(count,type)
  pub unsafe fn DrawElementsInstanced(&self, mode: PrimitiveType, count: GLsizei, type_: DrawElementsType, indices: *const void, instancecount: GLsizei) {
    (self.glDrawElementsInstanced_p)(mode, count, type_, indices, instancecount)
  }
  /// [glDrawElementsInstancedBaseInstance](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawElementsInstancedBaseInstance.xhtml)
  /// * `mode` group: PrimitiveType
  /// * `type` group: PrimitiveType
  /// * `indices` len: count
  pub unsafe fn DrawElementsInstancedBaseInstance(&self, mode: PrimitiveType, count: GLsizei, type_: PrimitiveType, indices: *const void, instancecount: GLsizei, baseinstance: GLuint) {
    (self.glDrawElementsInstancedBaseInstance_p)(mode, count, type_, indices, instancecount, baseinstance)
  }
  /// [glDrawElementsInstancedBaseVertex](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawElementsInstancedBaseVertex.xhtml)
  /// * `mode` group: PrimitiveType
  /// * `type` group: DrawElementsType
  /// * `indices` len: COMPSIZE(count,type)
  pub unsafe fn DrawElementsInstancedBaseVertex(&self, mode: PrimitiveType, count: GLsizei, type_: DrawElementsType, indices: *const void, instancecount: GLsizei, basevertex: GLint) {
    (self.glDrawElementsInstancedBaseVertex_p)(mode, count, type_, indices, instancecount, basevertex)
  }
  /// [glDrawElementsInstancedBaseVertexBaseInstance](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawElementsInstancedBaseVertexBaseInstance.xhtml)
  /// * `mode` group: PrimitiveType
  /// * `type` group: DrawElementsType
  /// * `indices` len: count
  pub unsafe fn DrawElementsInstancedBaseVertexBaseInstance(&self, mode: PrimitiveType, count: GLsizei, type_: DrawElementsType, indices: *const void, instancecount: GLsizei, basevertex: GLint, baseinstance: GLuint) {
    (self.glDrawElementsInstancedBaseVertexBaseInstance_p)(mode, count, type_, indices, instancecount, basevertex, baseinstance)
  }
  /// [glDrawRangeElements](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawRangeElements.xhtml)
  /// * `mode` group: PrimitiveType
  /// * `type` group: DrawElementsType
  /// * `indices` len: COMPSIZE(count,type)
  pub unsafe fn DrawRangeElements(&self, mode: PrimitiveType, start: GLuint, end: GLuint, count: GLsizei, type_: DrawElementsType, indices: *const void) {
    (self.glDrawRangeElements_p)(mode, start, end, count, type_, indices)
  }
  /// [glDrawRangeElementsBaseVertex](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawRangeElementsBaseVertex.xhtml)
  /// * `mode` group: PrimitiveType
  /// * `type` group: DrawElementsType
  /// * `indices` len: COMPSIZE(count,type)
  pub unsafe fn DrawRangeElementsBaseVertex(&self, mode: PrimitiveType, start: GLuint, end: GLuint, count: GLsizei, type_: DrawElementsType, indices: *const void, basevertex: GLint) {
    (self.glDrawRangeElementsBaseVertex_p)(mode, start, end, count, type_, indices, basevertex)
  }
  /// [glDrawTransformFeedback](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawTransformFeedback.xhtml)
  /// * `mode` group: PrimitiveType
  /// * `id` class: transform feedback
  pub unsafe fn DrawTransformFeedback(&self, mode: PrimitiveType, id: GLuint) {
    (self.glDrawTransformFeedback_p)(mode, id)
  }
  /// [glDrawTransformFeedbackInstanced](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawTransformFeedbackInstanced.xhtml)
  /// * `mode` group: PrimitiveType
  /// * `id` class: transform feedback
  pub unsafe fn DrawTransformFeedbackInstanced(&self, mode: PrimitiveType, id: GLuint, instancecount: GLsizei) {
    (self.glDrawTransformFeedbackInstanced_p)(mode, id, instancecount)
  }
  /// [glDrawTransformFeedbackStream](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawTransformFeedbackStream.xhtml)
  /// * `mode` group: PrimitiveType
  /// * `id` class: transform feedback
  pub unsafe fn DrawTransformFeedbackStream(&self, mode: PrimitiveType, id: GLuint, stream: GLuint) {
    (self.glDrawTransformFeedbackStream_p)(mode, id, stream)
  }
  /// [glDrawTransformFeedbackStreamInstanced](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawTransformFeedbackStreamInstanced.xhtml)
  /// * `mode` group: PrimitiveType
  /// * `id` class: transform feedback
  pub unsafe fn DrawTransformFeedbackStreamInstanced(&self, mode: PrimitiveType, id: GLuint, stream: GLuint, instancecount: GLsizei) {
    (self.glDrawTransformFeedbackStreamInstanced_p)(mode, id, stream, instancecount)
  }
  /// [glEnable](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEnable.xhtml)
  /// * `cap` group: EnableCap
  pub unsafe fn Enable(&self, cap: EnableCap) {
    (self.glEnable_p)(cap)
  }
  /// [glEnableVertexArrayAttrib](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEnableVertexArrayAttrib.xhtml)
  /// * `vaobj` class: vertex array
  pub unsafe fn EnableVertexArrayAttrib(&self, vaobj: GLuint, index: GLuint) {
    (self.glEnableVertexArrayAttrib_p)(vaobj, index)
  }
  /// [glEnableVertexAttribArray](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEnableVertexAttribArray.xhtml)
  pub unsafe fn EnableVertexAttribArray(&self, index: GLuint) {
    (self.glEnableVertexAttribArray_p)(index)
  }
  /// [glEnablei](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEnablei.xhtml)
  /// * `target` group: EnableCap
  pub unsafe fn Enablei(&self, target: EnableCap, index: GLuint) {
    (self.glEnablei_p)(target, index)
  }
  /// [glEndConditionalRender](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEndConditionalRender.xhtml)
  pub unsafe fn EndConditionalRender(&self) {
    (self.glEndConditionalRender_p)()
  }
  /// [glEndQuery](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEndQuery.xhtml)
  /// * `target` group: QueryTarget
  pub unsafe fn EndQuery(&self, target: QueryTarget) {
    (self.glEndQuery_p)(target)
  }
  /// [glEndQueryIndexed](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEndQueryIndexed.xhtml)
  /// * `target` group: QueryTarget
  pub unsafe fn EndQueryIndexed(&self, target: QueryTarget, index: GLuint) {
    (self.glEndQueryIndexed_p)(target, index)
  }
  /// [glEndTransformFeedback](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEndTransformFeedback.xhtml)
  pub unsafe fn EndTransformFeedback(&self) {
    (self.glEndTransformFeedback_p)()
  }
  /// [glFenceSync](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFenceSync.xhtml)
  /// * `condition` group: SyncCondition
  /// * `flags` group: SyncBehaviorFlags
  pub unsafe fn FenceSync(&self, condition: SyncCondition, flags: GLbitfield) -> GLsync {
    (self.glFenceSync_p)(condition, flags)
  }
  /// [glFinish](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFinish.xhtml)
  pub unsafe fn Finish(&self) {
    (self.glFinish_p)()
  }
  /// [glFlush](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFlush.xhtml)
  pub unsafe fn Flush(&self) {
    (self.glFlush_p)()
  }
  /// [glFlushMappedBufferRange](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFlushMappedBufferRange.xhtml)
  /// * `target` group: BufferTargetARB
  /// * `offset` group: BufferOffset
  /// * `length` group: BufferSize
  pub unsafe fn FlushMappedBufferRange(&self, target: BufferTargetARB, offset: GLintptr, length: GLsizeiptr) {
    (self.glFlushMappedBufferRange_p)(target, offset, length)
  }
  /// [glFlushMappedNamedBufferRange](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFlushMappedNamedBufferRange.xhtml)
  /// * `buffer` class: buffer
  /// * `length` group: BufferSize
  pub unsafe fn FlushMappedNamedBufferRange(&self, buffer: GLuint, offset: GLintptr, length: GLsizeiptr) {
    (self.glFlushMappedNamedBufferRange_p)(buffer, offset, length)
  }
  /// [glFramebufferParameteri](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFramebufferParameteri.xhtml)
  /// * `target` group: FramebufferTarget
  /// * `pname` group: FramebufferParameterName
  pub unsafe fn FramebufferParameteri(&self, target: FramebufferTarget, pname: FramebufferParameterName, param: GLint) {
    (self.glFramebufferParameteri_p)(target, pname, param)
  }
  /// [glFramebufferRenderbuffer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFramebufferRenderbuffer.xhtml)
  /// * `target` group: FramebufferTarget
  /// * `attachment` group: FramebufferAttachment
  /// * `renderbuffertarget` group: RenderbufferTarget
  /// * `renderbuffer` class: renderbuffer
  pub unsafe fn FramebufferRenderbuffer(&self, target: FramebufferTarget, attachment: FramebufferAttachment, renderbuffertarget: RenderbufferTarget, renderbuffer: GLuint) {
    (self.glFramebufferRenderbuffer_p)(target, attachment, renderbuffertarget, renderbuffer)
  }
  /// [glFramebufferTexture](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFramebufferTexture.xhtml)
  /// * `target` group: FramebufferTarget
  /// * `attachment` group: FramebufferAttachment
  /// * `texture` class: texture
  pub unsafe fn FramebufferTexture(&self, target: FramebufferTarget, attachment: FramebufferAttachment, texture: GLuint, level: GLint) {
    (self.glFramebufferTexture_p)(target, attachment, texture, level)
  }
  /// [glFramebufferTexture1D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFramebufferTexture1D.xhtml)
  /// * `target` group: FramebufferTarget
  /// * `attachment` group: FramebufferAttachment
  /// * `textarget` group: TextureTarget
  /// * `texture` class: texture
  pub unsafe fn FramebufferTexture1D(&self, target: FramebufferTarget, attachment: FramebufferAttachment, textarget: TextureTarget, texture: GLuint, level: GLint) {
    (self.glFramebufferTexture1D_p)(target, attachment, textarget, texture, level)
  }
  /// [glFramebufferTexture2D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFramebufferTexture2D.xhtml)
  /// * `target` group: FramebufferTarget
  /// * `attachment` group: FramebufferAttachment
  /// * `textarget` group: TextureTarget
  /// * `texture` class: texture
  pub unsafe fn FramebufferTexture2D(&self, target: FramebufferTarget, attachment: FramebufferAttachment, textarget: TextureTarget, texture: GLuint, level: GLint) {
    (self.glFramebufferTexture2D_p)(target, attachment, textarget, texture, level)
  }
  /// [glFramebufferTexture3D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFramebufferTexture3D.xhtml)
  /// * `target` group: FramebufferTarget
  /// * `attachment` group: FramebufferAttachment
  /// * `textarget` group: TextureTarget
  /// * `texture` class: texture
  pub unsafe fn FramebufferTexture3D(&self, target: FramebufferTarget, attachment: FramebufferAttachment, textarget: TextureTarget, texture: GLuint, level: GLint, zoffset: GLint) {
    (self.glFramebufferTexture3D_p)(target, attachment, textarget, texture, level, zoffset)
  }
  /// [glFramebufferTextureLayer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFramebufferTextureLayer.xhtml)
  /// * `target` group: FramebufferTarget
  /// * `attachment` group: FramebufferAttachment
  /// * `texture` group: Texture
  /// * `texture` class: texture
  /// * `level` group: CheckedInt32
  /// * `layer` group: CheckedInt32
  pub unsafe fn FramebufferTextureLayer(&self, target: FramebufferTarget, attachment: FramebufferAttachment, texture: GLuint, level: GLint, layer: GLint) {
    (self.glFramebufferTextureLayer_p)(target, attachment, texture, level, layer)
  }
  /// [glFrontFace](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glFrontFace.xhtml)
  /// * `mode` group: FrontFaceDirection
  pub unsafe fn FrontFace(&self, mode: FrontFaceDirection) {
    (self.glFrontFace_p)(mode)
  }
  /// [glGenBuffers](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGenBuffers.xhtml)
  /// * `buffers` class: buffer
  /// * `buffers` len: n
  pub unsafe fn GenBuffers(&self, n: GLsizei, buffers: *mut GLuint) {
    (self.glGenBuffers_p)(n, buffers)
  }
  /// [glGenFramebuffers](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGenFramebuffers.xhtml)
  /// * `framebuffers` class: framebuffer
  /// * `framebuffers` len: n
  pub unsafe fn GenFramebuffers(&self, n: GLsizei, framebuffers: *mut GLuint) {
    (self.glGenFramebuffers_p)(n, framebuffers)
  }
  /// [glGenProgramPipelines](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGenProgramPipelines.xhtml)
  /// * `pipelines` class: program pipeline
  /// * `pipelines` len: n
  pub unsafe fn GenProgramPipelines(&self, n: GLsizei, pipelines: *mut GLuint) {
    (self.glGenProgramPipelines_p)(n, pipelines)
  }
  /// [glGenQueries](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGenQueries.xhtml)
  /// * `ids` class: query
  /// * `ids` len: n
  pub unsafe fn GenQueries(&self, n: GLsizei, ids: *mut GLuint) {
    (self.glGenQueries_p)(n, ids)
  }
  /// [glGenRenderbuffers](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGenRenderbuffers.xhtml)
  /// * `renderbuffers` class: renderbuffer
  /// * `renderbuffers` len: n
  pub unsafe fn GenRenderbuffers(&self, n: GLsizei, renderbuffers: *mut GLuint) {
    (self.glGenRenderbuffers_p)(n, renderbuffers)
  }
  /// [glGenSamplers](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGenSamplers.xhtml)
  /// * `samplers` class: sampler
  /// * `samplers` len: count
  pub unsafe fn GenSamplers(&self, count: GLsizei, samplers: *mut GLuint) {
    (self.glGenSamplers_p)(count, samplers)
  }
  /// [glGenTextures](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGenTextures.xhtml)
  /// * `textures` group: Texture
  /// * `textures` class: texture
  /// * `textures` len: n
  pub unsafe fn GenTextures(&self, n: GLsizei, textures: *mut GLuint) {
    (self.glGenTextures_p)(n, textures)
  }
  /// [glGenTransformFeedbacks](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGenTransformFeedbacks.xhtml)
  /// * `ids` class: transform feedback
  /// * `ids` len: n
  pub unsafe fn GenTransformFeedbacks(&self, n: GLsizei, ids: *mut GLuint) {
    (self.glGenTransformFeedbacks_p)(n, ids)
  }
  /// [glGenVertexArrays](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGenVertexArrays.xhtml)
  /// * `arrays` class: vertex array
  /// * `arrays` len: n
  pub unsafe fn GenVertexArrays(&self, n: GLsizei, arrays: *mut GLuint) {
    (self.glGenVertexArrays_p)(n, arrays)
  }
  /// [glGenerateMipmap](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGenerateMipmap.xhtml)
  /// * `target` group: TextureTarget
  pub unsafe fn GenerateMipmap(&self, target: TextureTarget) {
    (self.glGenerateMipmap_p)(target)
  }
  /// [glGenerateTextureMipmap](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGenerateTextureMipmap.xhtml)
  /// * `texture` class: texture
  pub unsafe fn GenerateTextureMipmap(&self, texture: GLuint) {
    (self.glGenerateTextureMipmap_p)(texture)
  }
  /// [glGetActiveAtomicCounterBufferiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetActiveAtomicCounterBufferiv.xhtml)
  /// * `program` class: program
  /// * `pname` group: AtomicCounterBufferPName
  /// * `params` len: COMPSIZE(pname)
  pub unsafe fn GetActiveAtomicCounterBufferiv(&self, program: GLuint, bufferIndex: GLuint, pname: AtomicCounterBufferPName, params: *mut GLint) {
    (self.glGetActiveAtomicCounterBufferiv_p)(program, bufferIndex, pname, params)
  }
  /// [glGetActiveAttrib](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetActiveAttrib.xhtml)
  /// * `program` class: program
  /// * `type` group: AttributeType
  /// * `name` len: bufSize
  pub unsafe fn GetActiveAttrib(&self, program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLint, type_: *mut AttributeType, name: *mut GLchar) {
    (self.glGetActiveAttrib_p)(program, index, bufSize, length, size, type_, name)
  }
  /// [glGetActiveSubroutineName](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetActiveSubroutineName.xhtml)
  /// * `program` class: program
  /// * `shadertype` group: ShaderType
  /// * `name` len: bufSize
  pub unsafe fn GetActiveSubroutineName(&self, program: GLuint, shadertype: ShaderType, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, name: *mut GLchar) {
    (self.glGetActiveSubroutineName_p)(program, shadertype, index, bufSize, length, name)
  }
  /// [glGetActiveSubroutineUniformName](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetActiveSubroutineUniformName.xhtml)
  /// * `program` class: program
  /// * `shadertype` group: ShaderType
  /// * `name` len: bufSize
  pub unsafe fn GetActiveSubroutineUniformName(&self, program: GLuint, shadertype: ShaderType, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, name: *mut GLchar) {
    (self.glGetActiveSubroutineUniformName_p)(program, shadertype, index, bufSize, length, name)
  }
  /// [glGetActiveSubroutineUniformiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetActiveSubroutineUniformiv.xhtml)
  /// * `program` class: program
  /// * `shadertype` group: ShaderType
  /// * `pname` group: SubroutineParameterName
  /// * `values` len: COMPSIZE(pname)
  pub unsafe fn GetActiveSubroutineUniformiv(&self, program: GLuint, shadertype: ShaderType, index: GLuint, pname: SubroutineParameterName, values: *mut GLint) {
    (self.glGetActiveSubroutineUniformiv_p)(program, shadertype, index, pname, values)
  }
  /// [glGetActiveUniform](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetActiveUniform.xhtml)
  /// * `program` class: program
  /// * `type` group: UniformType
  /// * `name` len: bufSize
  pub unsafe fn GetActiveUniform(&self, program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLint, type_: *mut UniformType, name: *mut GLchar) {
    (self.glGetActiveUniform_p)(program, index, bufSize, length, size, type_, name)
  }
  /// [glGetActiveUniformBlockName](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetActiveUniformBlockName.xhtml)
  /// * `program` class: program
  /// * `uniformBlockName` len: bufSize
  pub unsafe fn GetActiveUniformBlockName(&self, program: GLuint, uniformBlockIndex: GLuint, bufSize: GLsizei, length: *mut GLsizei, uniformBlockName: *mut GLchar) {
    (self.glGetActiveUniformBlockName_p)(program, uniformBlockIndex, bufSize, length, uniformBlockName)
  }
  /// [glGetActiveUniformBlockiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetActiveUniformBlockiv.xhtml)
  /// * `program` class: program
  /// * `pname` group: UniformBlockPName
  /// * `params` len: COMPSIZE(program,uniformBlockIndex,pname)
  pub unsafe fn GetActiveUniformBlockiv(&self, program: GLuint, uniformBlockIndex: GLuint, pname: UniformBlockPName, params: *mut GLint) {
    (self.glGetActiveUniformBlockiv_p)(program, uniformBlockIndex, pname, params)
  }
  /// [glGetActiveUniformName](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetActiveUniformName.xhtml)
  /// * `program` class: program
  /// * `uniformName` len: bufSize
  pub unsafe fn GetActiveUniformName(&self, program: GLuint, uniformIndex: GLuint, bufSize: GLsizei, length: *mut GLsizei, uniformName: *mut GLchar) {
    (self.glGetActiveUniformName_p)(program, uniformIndex, bufSize, length, uniformName)
  }
  /// [glGetActiveUniformsiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetActiveUniformsiv.xhtml)
  /// * `program` class: program
  /// * `uniformIndices` len: uniformCount
  /// * `pname` group: UniformPName
  /// * `params` len: COMPSIZE(uniformCount,pname)
  pub unsafe fn GetActiveUniformsiv(&self, program: GLuint, uniformCount: GLsizei, uniformIndices: *const GLuint, pname: UniformPName, params: *mut GLint) {
    (self.glGetActiveUniformsiv_p)(program, uniformCount, uniformIndices, pname, params)
  }
  /// [glGetAttachedShaders](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetAttachedShaders.xhtml)
  /// * `program` class: program
  /// * `shaders` class: shader
  /// * `shaders` len: maxCount
  pub unsafe fn GetAttachedShaders(&self, program: GLuint, maxCount: GLsizei, count: *mut GLsizei, shaders: *mut GLuint) {
    (self.glGetAttachedShaders_p)(program, maxCount, count, shaders)
  }
  /// [glGetAttribLocation](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetAttribLocation.xhtml)
  /// * `program` class: program
  pub unsafe fn GetAttribLocation(&self, program: GLuint, name: *const GLchar) -> GLint {
    (self.glGetAttribLocation_p)(program, name)
  }
  /// [glGetBooleani_v](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetBooleani_v.xhtml)
  /// * `target` group: BufferTargetARB
  /// * `data` group: Boolean
  /// * `data` len: COMPSIZE(target)
  pub unsafe fn GetBooleani_v(&self, target: BufferTargetARB, index: GLuint, data: *mut GLboolean) {
    (self.glGetBooleani_v_p)(target, index, data)
  }
  /// [glGetBooleanv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetBooleanv.xhtml)
  /// * `pname` group: GetPName
  /// * `data` group: Boolean
  /// * `data` len: COMPSIZE(pname)
  pub unsafe fn GetBooleanv(&self, pname: GetPName, data: *mut GLboolean) {
    (self.glGetBooleanv_p)(pname, data)
  }
  /// [glGetBufferParameteri64v](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetBufferParameteri64v.xhtml)
  /// * `target` group: BufferTargetARB
  /// * `pname` group: BufferPNameARB
  /// * `params` len: COMPSIZE(pname)
  pub unsafe fn GetBufferParameteri64v(&self, target: BufferTargetARB, pname: BufferPNameARB, params: *mut GLint64) {
    (self.glGetBufferParameteri64v_p)(target, pname, params)
  }
  /// [glGetBufferParameteriv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetBufferParameteriv.xhtml)
  /// * `target` group: BufferTargetARB
  /// * `pname` group: BufferPNameARB
  /// * `params` len: COMPSIZE(pname)
  pub unsafe fn GetBufferParameteriv(&self, target: BufferTargetARB, pname: BufferPNameARB, params: *mut GLint) {
    (self.glGetBufferParameteriv_p)(target, pname, params)
  }
  /// [glGetBufferPointerv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetBufferPointerv.xhtml)
  /// * `target` group: BufferTargetARB
  /// * `pname` group: BufferPointerNameARB
  pub unsafe fn GetBufferPointerv(&self, target: BufferTargetARB, pname: BufferPointerNameARB, params: *mut *mut void) {
    (self.glGetBufferPointerv_p)(target, pname, params)
  }
  /// [glGetBufferSubData](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetBufferSubData.xhtml)
  /// * `target` group: BufferTargetARB
  /// * `offset` group: BufferOffset
  /// * `size` group: BufferSize
  /// * `data` len: size
  pub unsafe fn GetBufferSubData(&self, target: BufferTargetARB, offset: GLintptr, size: GLsizeiptr, data: *mut void) {
    (self.glGetBufferSubData_p)(target, offset, size, data)
  }
  /// [glGetCompressedTexImage](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetCompressedTexImage.xhtml)
  /// * `target` group: TextureTarget
  /// * `level` group: CheckedInt32
  /// * `img` group: CompressedTextureARB
  /// * `img` len: COMPSIZE(target,level)
  pub unsafe fn GetCompressedTexImage(&self, target: TextureTarget, level: GLint, img: *mut void) {
    (self.glGetCompressedTexImage_p)(target, level, img)
  }
  /// [glGetCompressedTextureImage](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetCompressedTextureImage.xhtml)
  /// * `texture` class: texture
  pub unsafe fn GetCompressedTextureImage(&self, texture: GLuint, level: GLint, bufSize: GLsizei, pixels: *mut void) {
    (self.glGetCompressedTextureImage_p)(texture, level, bufSize, pixels)
  }
  /// [glGetCompressedTextureSubImage](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetCompressedTextureSubImage.xhtml)
  /// * `texture` class: texture
  pub unsafe fn GetCompressedTextureSubImage(&self, texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, bufSize: GLsizei, pixels: *mut void) {
    (self.glGetCompressedTextureSubImage_p)(texture, level, xoffset, yoffset, zoffset, width, height, depth, bufSize, pixels)
  }
  /// [glGetDebugMessageLog](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetDebugMessageLog.xhtml)
  /// * `sources` group: DebugSource
  /// * `sources` len: count
  /// * `types` group: DebugType
  /// * `types` len: count
  /// * `ids` len: count
  /// * `severities` group: DebugSeverity
  /// * `severities` len: count
  /// * `lengths` len: count
  /// * `messageLog` len: bufSize
  pub unsafe fn GetDebugMessageLog(&self, count: GLuint, bufSize: GLsizei, sources: *mut DebugSource, types: *mut DebugType, ids: *mut GLuint, severities: *mut DebugSeverity, lengths: *mut GLsizei, messageLog: *mut GLchar) -> GLuint {
    (self.glGetDebugMessageLog_p)(count, bufSize, sources, types, ids, severities, lengths, messageLog)
  }
  /// [glGetDoublei_v](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetDoublei_v.xhtml)
  /// * `target` group: GetPName
  /// * `data` len: COMPSIZE(target)
  pub unsafe fn GetDoublei_v(&self, target: GetPName, index: GLuint, data: *mut GLdouble) {
    (self.glGetDoublei_v_p)(target, index, data)
  }
  /// [glGetDoublev](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetDoublev.xhtml)
  /// * `pname` group: GetPName
  /// * `data` len: COMPSIZE(pname)
  pub unsafe fn GetDoublev(&self, pname: GetPName, data: *mut GLdouble) {
    (self.glGetDoublev_p)(pname, data)
  }
  /// [glGetError](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetError.xhtml)
  pub unsafe fn GetError(&self) -> ErrorCode {
    (self.glGetError_p)()
  }
  /// [glGetFloati_v](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetFloati_v.xhtml)
  /// * `target` group: GetPName
  /// * `data` len: COMPSIZE(target)
  pub unsafe fn GetFloati_v(&self, target: GetPName, index: GLuint, data: *mut GLfloat) {
    (self.glGetFloati_v_p)(target, index, data)
  }
  /// [glGetFloatv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetFloatv.xhtml)
  /// * `pname` group: GetPName
  /// * `data` len: COMPSIZE(pname)
  pub unsafe fn GetFloatv(&self, pname: GetPName, data: *mut GLfloat) {
    (self.glGetFloatv_p)(pname, data)
  }
  /// [glGetFragDataIndex](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetFragDataIndex.xhtml)
  /// * `program` class: program
  pub unsafe fn GetFragDataIndex(&self, program: GLuint, name: *const GLchar) -> GLint {
    (self.glGetFragDataIndex_p)(program, name)
  }
  /// [glGetFragDataLocation](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetFragDataLocation.xhtml)
  /// * `program` class: program
  /// * `name` len: COMPSIZE(name)
  pub unsafe fn GetFragDataLocation(&self, program: GLuint, name: *const GLchar) -> GLint {
    (self.glGetFragDataLocation_p)(program, name)
  }
  /// [glGetFramebufferAttachmentParameteriv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetFramebufferAttachmentParameteriv.xhtml)
  /// * `target` group: FramebufferTarget
  /// * `attachment` group: FramebufferAttachment
  /// * `pname` group: FramebufferAttachmentParameterName
  /// * `params` len: COMPSIZE(pname)
  pub unsafe fn GetFramebufferAttachmentParameteriv(&self, target: FramebufferTarget, attachment: FramebufferAttachment, pname: FramebufferAttachmentParameterName, params: *mut GLint) {
    (self.glGetFramebufferAttachmentParameteriv_p)(target, attachment, pname, params)
  }
  /// [glGetFramebufferParameteriv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetFramebufferParameteriv.xhtml)
  /// * `target` group: FramebufferTarget
  /// * `pname` group: FramebufferAttachmentParameterName
  /// * `params` len: COMPSIZE(pname)
  pub unsafe fn GetFramebufferParameteriv(&self, target: FramebufferTarget, pname: FramebufferAttachmentParameterName, params: *mut GLint) {
    (self.glGetFramebufferParameteriv_p)(target, pname, params)
  }
  /// [glGetGraphicsResetStatus](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetGraphicsResetStatus.xhtml)
  pub unsafe fn GetGraphicsResetStatus(&self) -> GraphicsResetStatus {
    (self.glGetGraphicsResetStatus_p)()
  }
  /// [glGetInteger64i_v](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetInteger64i_v.xhtml)
  /// * `target` group: GetPName
  /// * `data` len: COMPSIZE(target)
  pub unsafe fn GetInteger64i_v(&self, target: GetPName, index: GLuint, data: *mut GLint64) {
    (self.glGetInteger64i_v_p)(target, index, data)
  }
  /// [glGetInteger64v](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetInteger64v.xhtml)
  /// * `pname` group: GetPName
  /// * `data` len: COMPSIZE(pname)
  pub unsafe fn GetInteger64v(&self, pname: GetPName, data: *mut GLint64) {
    (self.glGetInteger64v_p)(pname, data)
  }
  /// [glGetIntegeri_v](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetIntegeri_v.xhtml)
  /// * `target` group: GetPName
  /// * `data` len: COMPSIZE(target)
  pub unsafe fn GetIntegeri_v(&self, target: GetPName, index: GLuint, data: *mut GLint) {
    (self.glGetIntegeri_v_p)(target, index, data)
  }
  /// [glGetIntegerv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetIntegerv.xhtml)
  /// * `pname` group: GetPName
  /// * `data` len: COMPSIZE(pname)
  pub unsafe fn GetIntegerv(&self, pname: GetPName, data: *mut GLint) {
    (self.glGetIntegerv_p)(pname, data)
  }
  /// [glGetInternalformati64v](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetInternalformati64v.xhtml)
  /// * `target` group: TextureTarget
  /// * `internalformat` group: InternalFormat
  /// * `pname` group: InternalFormatPName
  /// * `params` len: count
  pub unsafe fn GetInternalformati64v(&self, target: TextureTarget, internalformat: InternalFormat, pname: InternalFormatPName, count: GLsizei, params: *mut GLint64) {
    (self.glGetInternalformati64v_p)(target, internalformat, pname, count, params)
  }
  /// [glGetInternalformativ](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetInternalformativ.xhtml)
  /// * `target` group: TextureTarget
  /// * `internalformat` group: InternalFormat
  /// * `pname` group: InternalFormatPName
  /// * `params` len: count
  pub unsafe fn GetInternalformativ(&self, target: TextureTarget, internalformat: InternalFormat, pname: InternalFormatPName, count: GLsizei, params: *mut GLint) {
    (self.glGetInternalformativ_p)(target, internalformat, pname, count, params)
  }
  /// [glGetMultisamplefv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetMultisamplefv.xhtml)
  /// * `pname` group: GetMultisamplePNameNV
  /// * `val` len: COMPSIZE(pname)
  pub unsafe fn GetMultisamplefv(&self, pname: GetMultisamplePNameNV, index: GLuint, val: *mut GLfloat) {
    (self.glGetMultisamplefv_p)(pname, index, val)
  }
  /// [glGetNamedBufferParameteri64v](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetNamedBufferParameteri64v.xhtml)
  /// * `buffer` class: buffer
  /// * `pname` group: BufferPNameARB
  pub unsafe fn GetNamedBufferParameteri64v(&self, buffer: GLuint, pname: BufferPNameARB, params: *mut GLint64) {
    (self.glGetNamedBufferParameteri64v_p)(buffer, pname, params)
  }
  /// [glGetNamedBufferParameteriv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetNamedBufferParameteriv.xhtml)
  /// * `buffer` class: buffer
  /// * `pname` group: BufferPNameARB
  pub unsafe fn GetNamedBufferParameteriv(&self, buffer: GLuint, pname: BufferPNameARB, params: *mut GLint) {
    (self.glGetNamedBufferParameteriv_p)(buffer, pname, params)
  }
  /// [glGetNamedBufferPointerv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetNamedBufferPointerv.xhtml)
  /// * `buffer` class: buffer
  /// * `pname` group: BufferPointerNameARB
  pub unsafe fn GetNamedBufferPointerv(&self, buffer: GLuint, pname: BufferPointerNameARB, params: *mut *mut void) {
    (self.glGetNamedBufferPointerv_p)(buffer, pname, params)
  }
  /// [glGetNamedBufferSubData](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetNamedBufferSubData.xhtml)
  /// * `buffer` class: buffer
  /// * `size` group: BufferSize
  pub unsafe fn GetNamedBufferSubData(&self, buffer: GLuint, offset: GLintptr, size: GLsizeiptr, data: *mut void) {
    (self.glGetNamedBufferSubData_p)(buffer, offset, size, data)
  }
  /// [glGetNamedFramebufferAttachmentParameteriv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetNamedFramebufferAttachmentParameteriv.xhtml)
  /// * `framebuffer` class: framebuffer
  /// * `attachment` group: FramebufferAttachment
  /// * `pname` group: FramebufferAttachmentParameterName
  pub unsafe fn GetNamedFramebufferAttachmentParameteriv(&self, framebuffer: GLuint, attachment: FramebufferAttachment, pname: FramebufferAttachmentParameterName, params: *mut GLint) {
    (self.glGetNamedFramebufferAttachmentParameteriv_p)(framebuffer, attachment, pname, params)
  }
  /// [glGetNamedFramebufferParameteriv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetNamedFramebufferParameteriv.xhtml)
  /// * `framebuffer` class: framebuffer
  /// * `pname` group: GetFramebufferParameter
  pub unsafe fn GetNamedFramebufferParameteriv(&self, framebuffer: GLuint, pname: GetFramebufferParameter, param: *mut GLint) {
    (self.glGetNamedFramebufferParameteriv_p)(framebuffer, pname, param)
  }
  /// [glGetNamedRenderbufferParameteriv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetNamedRenderbufferParameteriv.xhtml)
  /// * `renderbuffer` class: renderbuffer
  /// * `pname` group: RenderbufferParameterName
  pub unsafe fn GetNamedRenderbufferParameteriv(&self, renderbuffer: GLuint, pname: RenderbufferParameterName, params: *mut GLint) {
    (self.glGetNamedRenderbufferParameteriv_p)(renderbuffer, pname, params)
  }
  /// [glGetObjectLabel](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetObjectLabel.xhtml)
  /// * `identifier` group: ObjectIdentifier
  /// * `label` len: bufSize
  pub unsafe fn GetObjectLabel(&self, identifier: ObjectIdentifier, name: GLuint, bufSize: GLsizei, length: *mut GLsizei, label: *mut GLchar) {
    (self.glGetObjectLabel_p)(identifier, name, bufSize, length, label)
  }
  /// [glGetObjectPtrLabel](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetObjectPtrLabel.xhtml)
  /// * `label` len: bufSize
  pub unsafe fn GetObjectPtrLabel(&self, ptr: *const void, bufSize: GLsizei, length: *mut GLsizei, label: *mut GLchar) {
    (self.glGetObjectPtrLabel_p)(ptr, bufSize, length, label)
  }
  /// [glGetPointerv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetPointerv.xhtml)
  /// * `pname` group: GetPointervPName
  pub unsafe fn GetPointerv(&self, pname: GetPointervPName, params: *mut *mut void) {
    (self.glGetPointerv_p)(pname, params)
  }
  /// [glGetProgramBinary](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetProgramBinary.xhtml)
  /// * `program` class: program
  /// * `binary` len: bufSize
  pub unsafe fn GetProgramBinary(&self, program: GLuint, bufSize: GLsizei, length: *mut GLsizei, binaryFormat: *mut GLenum, binary: *mut void) {
    (self.glGetProgramBinary_p)(program, bufSize, length, binaryFormat, binary)
  }
  /// [glGetProgramInfoLog](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetProgramInfoLog.xhtml)
  /// * `program` class: program
  /// * `infoLog` len: bufSize
  pub unsafe fn GetProgramInfoLog(&self, program: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar) {
    (self.glGetProgramInfoLog_p)(program, bufSize, length, infoLog)
  }
  /// [glGetProgramInterfaceiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetProgramInterfaceiv.xhtml)
  /// * `program` class: program
  /// * `programInterface` group: ProgramInterface
  /// * `pname` group: ProgramInterfacePName
  /// * `params` len: COMPSIZE(pname)
  pub unsafe fn GetProgramInterfaceiv(&self, program: GLuint, programInterface: ProgramInterface, pname: ProgramInterfacePName, params: *mut GLint) {
    (self.glGetProgramInterfaceiv_p)(program, programInterface, pname, params)
  }
  /// [glGetProgramPipelineInfoLog](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetProgramPipelineInfoLog.xhtml)
  /// * `pipeline` class: program pipeline
  /// * `infoLog` len: bufSize
  pub unsafe fn GetProgramPipelineInfoLog(&self, pipeline: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar) {
    (self.glGetProgramPipelineInfoLog_p)(pipeline, bufSize, length, infoLog)
  }
  /// [glGetProgramPipelineiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetProgramPipelineiv.xhtml)
  /// * `pipeline` class: program pipeline
  /// * `pname` group: PipelineParameterName
  /// * `params` len: COMPSIZE(pname)
  pub unsafe fn GetProgramPipelineiv(&self, pipeline: GLuint, pname: PipelineParameterName, params: *mut GLint) {
    (self.glGetProgramPipelineiv_p)(pipeline, pname, params)
  }
  /// [glGetProgramResourceIndex](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetProgramResourceIndex.xhtml)
  /// * `program` class: program
  /// * `programInterface` group: ProgramInterface
  /// * `name` len: COMPSIZE(name)
  pub unsafe fn GetProgramResourceIndex(&self, program: GLuint, programInterface: ProgramInterface, name: *const GLchar) -> GLuint {
    (self.glGetProgramResourceIndex_p)(program, programInterface, name)
  }
  /// [glGetProgramResourceLocation](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetProgramResourceLocation.xhtml)
  /// * `program` class: program
  /// * `programInterface` group: ProgramInterface
  /// * `name` len: COMPSIZE(name)
  pub unsafe fn GetProgramResourceLocation(&self, program: GLuint, programInterface: ProgramInterface, name: *const GLchar) -> GLint {
    (self.glGetProgramResourceLocation_p)(program, programInterface, name)
  }
  /// [glGetProgramResourceLocationIndex](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetProgramResourceLocationIndex.xhtml)
  /// * `program` class: program
  /// * `programInterface` group: ProgramInterface
  /// * `name` len: COMPSIZE(name)
  pub unsafe fn GetProgramResourceLocationIndex(&self, program: GLuint, programInterface: ProgramInterface, name: *const GLchar) -> GLint {
    (self.glGetProgramResourceLocationIndex_p)(program, programInterface, name)
  }
  /// [glGetProgramResourceName](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetProgramResourceName.xhtml)
  /// * `program` class: program
  /// * `programInterface` group: ProgramInterface
  /// * `name` len: bufSize
  pub unsafe fn GetProgramResourceName(&self, program: GLuint, programInterface: ProgramInterface, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, name: *mut GLchar) {
    (self.glGetProgramResourceName_p)(program, programInterface, index, bufSize, length, name)
  }
  /// [glGetProgramResourceiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetProgramResourceiv.xhtml)
  /// * `program` class: program
  /// * `programInterface` group: ProgramInterface
  /// * `props` group: ProgramResourceProperty
  /// * `props` len: propCount
  /// * `params` len: count
  pub unsafe fn GetProgramResourceiv(&self, program: GLuint, programInterface: ProgramInterface, index: GLuint, propCount: GLsizei, props: *const ProgramResourceProperty, count: GLsizei, length: *mut GLsizei, params: *mut GLint) {
    (self.glGetProgramResourceiv_p)(program, programInterface, index, propCount, props, count, length, params)
  }
  /// [glGetProgramStageiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetProgramStageiv.xhtml)
  /// * `program` class: program
  /// * `shadertype` group: ShaderType
  /// * `pname` group: ProgramStagePName
  pub unsafe fn GetProgramStageiv(&self, program: GLuint, shadertype: ShaderType, pname: ProgramStagePName, values: *mut GLint) {
    (self.glGetProgramStageiv_p)(program, shadertype, pname, values)
  }
  /// [glGetProgramiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetProgramiv.xhtml)
  /// * `program` class: program
  /// * `pname` group: ProgramPropertyARB
  /// * `params` len: COMPSIZE(pname)
  pub unsafe fn GetProgramiv(&self, program: GLuint, pname: ProgramPropertyARB, params: *mut GLint) {
    (self.glGetProgramiv_p)(program, pname, params)
  }
  /// [glGetQueryBufferObjecti64v](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetQueryBufferObjecti64v.xhtml)
  /// * `id` class: query
  /// * `buffer` class: buffer
  /// * `pname` group: QueryObjectParameterName
  pub unsafe fn GetQueryBufferObjecti64v(&self, id: GLuint, buffer: GLuint, pname: QueryObjectParameterName, offset: GLintptr) {
    (self.glGetQueryBufferObjecti64v_p)(id, buffer, pname, offset)
  }
  /// [glGetQueryBufferObjectiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetQueryBufferObjectiv.xhtml)
  /// * `id` class: query
  /// * `buffer` class: buffer
  /// * `pname` group: QueryObjectParameterName
  pub unsafe fn GetQueryBufferObjectiv(&self, id: GLuint, buffer: GLuint, pname: QueryObjectParameterName, offset: GLintptr) {
    (self.glGetQueryBufferObjectiv_p)(id, buffer, pname, offset)
  }
  /// [glGetQueryBufferObjectui64v](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetQueryBufferObjectui64v.xhtml)
  /// * `id` class: query
  /// * `buffer` class: buffer
  /// * `pname` group: QueryObjectParameterName
  pub unsafe fn GetQueryBufferObjectui64v(&self, id: GLuint, buffer: GLuint, pname: QueryObjectParameterName, offset: GLintptr) {
    (self.glGetQueryBufferObjectui64v_p)(id, buffer, pname, offset)
  }
  /// [glGetQueryBufferObjectuiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetQueryBufferObjectuiv.xhtml)
  /// * `id` class: query
  /// * `buffer` class: buffer
  /// * `pname` group: QueryObjectParameterName
  pub unsafe fn GetQueryBufferObjectuiv(&self, id: GLuint, buffer: GLuint, pname: QueryObjectParameterName, offset: GLintptr) {
    (self.glGetQueryBufferObjectuiv_p)(id, buffer, pname, offset)
  }
  /// [glGetQueryIndexediv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetQueryIndexediv.xhtml)
  /// * `target` group: QueryTarget
  /// * `pname` group: QueryParameterName
  /// * `params` len: COMPSIZE(pname)
  pub unsafe fn GetQueryIndexediv(&self, target: QueryTarget, index: GLuint, pname: QueryParameterName, params: *mut GLint) {
    (self.glGetQueryIndexediv_p)(target, index, pname, params)
  }
  /// [glGetQueryObjecti64v](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetQueryObjecti64v.xhtml)
  /// * `id` class: query
  /// * `pname` group: QueryObjectParameterName
  /// * `params` len: COMPSIZE(pname)
  pub unsafe fn GetQueryObjecti64v(&self, id: GLuint, pname: QueryObjectParameterName, params: *mut GLint64) {
    (self.glGetQueryObjecti64v_p)(id, pname, params)
  }
  /// [glGetQueryObjectiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetQueryObjectiv.xhtml)
  /// * `id` class: query
  /// * `pname` group: QueryObjectParameterName
  /// * `params` len: COMPSIZE(pname)
  pub unsafe fn GetQueryObjectiv(&self, id: GLuint, pname: QueryObjectParameterName, params: *mut GLint) {
    (self.glGetQueryObjectiv_p)(id, pname, params)
  }
  /// [glGetQueryObjectui64v](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetQueryObjectui64v.xhtml)
  /// * `id` class: query
  /// * `pname` group: QueryObjectParameterName
  /// * `params` len: COMPSIZE(pname)
  pub unsafe fn GetQueryObjectui64v(&self, id: GLuint, pname: QueryObjectParameterName, params: *mut GLuint64) {
    (self.glGetQueryObjectui64v_p)(id, pname, params)
  }
  /// [glGetQueryObjectuiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetQueryObjectuiv.xhtml)
  /// * `id` class: query
  /// * `pname` group: QueryObjectParameterName
  /// * `params` len: COMPSIZE(pname)
  pub unsafe fn GetQueryObjectuiv(&self, id: GLuint, pname: QueryObjectParameterName, params: *mut GLuint) {
    (self.glGetQueryObjectuiv_p)(id, pname, params)
  }
  /// [glGetQueryiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetQueryiv.xhtml)
  /// * `target` group: QueryTarget
  /// * `pname` group: QueryParameterName
  /// * `params` len: COMPSIZE(pname)
  pub unsafe fn GetQueryiv(&self, target: QueryTarget, pname: QueryParameterName, params: *mut GLint) {
    (self.glGetQueryiv_p)(target, pname, params)
  }
  /// [glGetRenderbufferParameteriv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetRenderbufferParameteriv.xhtml)
  /// * `target` group: RenderbufferTarget
  /// * `pname` group: RenderbufferParameterName
  /// * `params` len: COMPSIZE(pname)
  pub unsafe fn GetRenderbufferParameteriv(&self, target: RenderbufferTarget, pname: RenderbufferParameterName, params: *mut GLint) {
    (self.glGetRenderbufferParameteriv_p)(target, pname, params)
  }
  /// [glGetSamplerParameterIiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetSamplerParameterIiv.xhtml)
  /// * `sampler` class: sampler
  /// * `pname` group: SamplerParameterI
  /// * `params` len: COMPSIZE(pname)
  pub unsafe fn GetSamplerParameterIiv(&self, sampler: GLuint, pname: SamplerParameterI, params: *mut GLint) {
    (self.glGetSamplerParameterIiv_p)(sampler, pname, params)
  }
  /// [glGetSamplerParameterIuiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetSamplerParameterIuiv.xhtml)
  /// * `sampler` class: sampler
  /// * `pname` group: SamplerParameterI
  /// * `params` len: COMPSIZE(pname)
  pub unsafe fn GetSamplerParameterIuiv(&self, sampler: GLuint, pname: SamplerParameterI, params: *mut GLuint) {
    (self.glGetSamplerParameterIuiv_p)(sampler, pname, params)
  }
  /// [glGetSamplerParameterfv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetSamplerParameterfv.xhtml)
  /// * `sampler` class: sampler
  /// * `pname` group: SamplerParameterF
  /// * `params` len: COMPSIZE(pname)
  pub unsafe fn GetSamplerParameterfv(&self, sampler: GLuint, pname: SamplerParameterF, params: *mut GLfloat) {
    (self.glGetSamplerParameterfv_p)(sampler, pname, params)
  }
  /// [glGetSamplerParameteriv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetSamplerParameteriv.xhtml)
  /// * `sampler` class: sampler
  /// * `pname` group: SamplerParameterI
  /// * `params` len: COMPSIZE(pname)
  pub unsafe fn GetSamplerParameteriv(&self, sampler: GLuint, pname: SamplerParameterI, params: *mut GLint) {
    (self.glGetSamplerParameteriv_p)(sampler, pname, params)
  }
  /// [glGetShaderInfoLog](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetShaderInfoLog.xhtml)
  /// * `shader` class: shader
  /// * `infoLog` len: bufSize
  pub unsafe fn GetShaderInfoLog(&self, shader: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar) {
    (self.glGetShaderInfoLog_p)(shader, bufSize, length, infoLog)
  }
  /// [glGetShaderPrecisionFormat](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetShaderPrecisionFormat.xhtml)
  /// * `shadertype` group: ShaderType
  /// * `precisiontype` group: PrecisionType
  pub unsafe fn GetShaderPrecisionFormat(&self, shadertype: ShaderType, precisiontype: PrecisionType, range: *mut [GLint; 2], precision: *mut GLint) {
    (self.glGetShaderPrecisionFormat_p)(shadertype, precisiontype, range, precision)
  }
  /// [glGetShaderSource](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetShaderSource.xhtml)
  /// * `shader` class: shader
  /// * `source` len: bufSize
  pub unsafe fn GetShaderSource(&self, shader: GLuint, bufSize: GLsizei, length: *mut GLsizei, source: *mut GLchar) {
    (self.glGetShaderSource_p)(shader, bufSize, length, source)
  }
  /// [glGetShaderiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetShaderiv.xhtml)
  /// * `shader` class: shader
  /// * `pname` group: ShaderParameterName
  /// * `params` len: COMPSIZE(pname)
  pub unsafe fn GetShaderiv(&self, shader: GLuint, pname: ShaderParameterName, params: *mut GLint) {
    (self.glGetShaderiv_p)(shader, pname, params)
  }
  /// [glGetString](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetString.xhtml)
  /// * `name` group: StringName
  pub unsafe fn GetString(&self, name: StringName) -> GLubyte {
    (self.glGetString_p)(name)
  }
  /// [glGetStringi](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetStringi.xhtml)
  /// * `name` group: StringName
  pub unsafe fn GetStringi(&self, name: StringName, index: GLuint) -> GLubyte {
    (self.glGetStringi_p)(name, index)
  }
  /// [glGetSubroutineIndex](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetSubroutineIndex.xhtml)
  /// * `program` class: program
  /// * `shadertype` group: ShaderType
  pub unsafe fn GetSubroutineIndex(&self, program: GLuint, shadertype: ShaderType, name: *const GLchar) -> GLuint {
    (self.glGetSubroutineIndex_p)(program, shadertype, name)
  }
  /// [glGetSubroutineUniformLocation](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetSubroutineUniformLocation.xhtml)
  /// * `program` class: program
  /// * `shadertype` group: ShaderType
  pub unsafe fn GetSubroutineUniformLocation(&self, program: GLuint, shadertype: ShaderType, name: *const GLchar) -> GLint {
    (self.glGetSubroutineUniformLocation_p)(program, shadertype, name)
  }
  /// [glGetSynciv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetSynciv.xhtml)
  /// * `sync` group: sync
  /// * `sync` class: sync
  /// * `pname` group: SyncParameterName
  /// * `values` len: count
  pub unsafe fn GetSynciv(&self, sync: GLsync, pname: SyncParameterName, count: GLsizei, length: *mut GLsizei, values: *mut GLint) {
    (self.glGetSynciv_p)(sync, pname, count, length, values)
  }
  /// [glGetTexImage](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTexImage.xhtml)
  /// * `target` group: TextureTarget
  /// * `level` group: CheckedInt32
  /// * `format` group: PixelFormat
  /// * `type` group: PixelType
  /// * `pixels` len: COMPSIZE(target,level,format,type)
  pub unsafe fn GetTexImage(&self, target: TextureTarget, level: GLint, format: PixelFormat, type_: PixelType, pixels: *mut void) {
    (self.glGetTexImage_p)(target, level, format, type_, pixels)
  }
  /// [glGetTexLevelParameterfv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTexLevelParameterfv.xhtml)
  /// * `target` group: TextureTarget
  /// * `level` group: CheckedInt32
  /// * `pname` group: GetTextureParameter
  /// * `params` len: COMPSIZE(pname)
  pub unsafe fn GetTexLevelParameterfv(&self, target: TextureTarget, level: GLint, pname: GetTextureParameter, params: *mut GLfloat) {
    (self.glGetTexLevelParameterfv_p)(target, level, pname, params)
  }
  /// [glGetTexLevelParameteriv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTexLevelParameteriv.xhtml)
  /// * `target` group: TextureTarget
  /// * `level` group: CheckedInt32
  /// * `pname` group: GetTextureParameter
  /// * `params` len: COMPSIZE(pname)
  pub unsafe fn GetTexLevelParameteriv(&self, target: TextureTarget, level: GLint, pname: GetTextureParameter, params: *mut GLint) {
    (self.glGetTexLevelParameteriv_p)(target, level, pname, params)
  }
  /// [glGetTexParameterIiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTexParameterIiv.xhtml)
  /// * `target` group: TextureTarget
  /// * `pname` group: GetTextureParameter
  /// * `params` len: COMPSIZE(pname)
  pub unsafe fn GetTexParameterIiv(&self, target: TextureTarget, pname: GetTextureParameter, params: *mut GLint) {
    (self.glGetTexParameterIiv_p)(target, pname, params)
  }
  /// [glGetTexParameterIuiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTexParameterIuiv.xhtml)
  /// * `target` group: TextureTarget
  /// * `pname` group: GetTextureParameter
  /// * `params` len: COMPSIZE(pname)
  pub unsafe fn GetTexParameterIuiv(&self, target: TextureTarget, pname: GetTextureParameter, params: *mut GLuint) {
    (self.glGetTexParameterIuiv_p)(target, pname, params)
  }
  /// [glGetTexParameterfv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTexParameterfv.xhtml)
  /// * `target` group: TextureTarget
  /// * `pname` group: GetTextureParameter
  /// * `params` len: COMPSIZE(pname)
  pub unsafe fn GetTexParameterfv(&self, target: TextureTarget, pname: GetTextureParameter, params: *mut GLfloat) {
    (self.glGetTexParameterfv_p)(target, pname, params)
  }
  /// [glGetTexParameteriv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTexParameteriv.xhtml)
  /// * `target` group: TextureTarget
  /// * `pname` group: GetTextureParameter
  /// * `params` len: COMPSIZE(pname)
  pub unsafe fn GetTexParameteriv(&self, target: TextureTarget, pname: GetTextureParameter, params: *mut GLint) {
    (self.glGetTexParameteriv_p)(target, pname, params)
  }
  /// [glGetTextureImage](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTextureImage.xhtml)
  /// * `texture` class: texture
  /// * `format` group: PixelFormat
  /// * `type` group: PixelType
  pub unsafe fn GetTextureImage(&self, texture: GLuint, level: GLint, format: PixelFormat, type_: PixelType, bufSize: GLsizei, pixels: *mut void) {
    (self.glGetTextureImage_p)(texture, level, format, type_, bufSize, pixels)
  }
  /// [glGetTextureLevelParameterfv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTextureLevelParameterfv.xhtml)
  /// * `texture` class: texture
  /// * `pname` group: GetTextureParameter
  pub unsafe fn GetTextureLevelParameterfv(&self, texture: GLuint, level: GLint, pname: GetTextureParameter, params: *mut GLfloat) {
    (self.glGetTextureLevelParameterfv_p)(texture, level, pname, params)
  }
  /// [glGetTextureLevelParameteriv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTextureLevelParameteriv.xhtml)
  /// * `texture` class: texture
  /// * `pname` group: GetTextureParameter
  pub unsafe fn GetTextureLevelParameteriv(&self, texture: GLuint, level: GLint, pname: GetTextureParameter, params: *mut GLint) {
    (self.glGetTextureLevelParameteriv_p)(texture, level, pname, params)
  }
  /// [glGetTextureParameterIiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTextureParameterIiv.xhtml)
  /// * `texture` class: texture
  /// * `pname` group: GetTextureParameter
  pub unsafe fn GetTextureParameterIiv(&self, texture: GLuint, pname: GetTextureParameter, params: *mut GLint) {
    (self.glGetTextureParameterIiv_p)(texture, pname, params)
  }
  /// [glGetTextureParameterIuiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTextureParameterIuiv.xhtml)
  /// * `texture` class: texture
  /// * `pname` group: GetTextureParameter
  pub unsafe fn GetTextureParameterIuiv(&self, texture: GLuint, pname: GetTextureParameter, params: *mut GLuint) {
    (self.glGetTextureParameterIuiv_p)(texture, pname, params)
  }
  /// [glGetTextureParameterfv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTextureParameterfv.xhtml)
  /// * `texture` class: texture
  /// * `pname` group: GetTextureParameter
  pub unsafe fn GetTextureParameterfv(&self, texture: GLuint, pname: GetTextureParameter, params: *mut GLfloat) {
    (self.glGetTextureParameterfv_p)(texture, pname, params)
  }
  /// [glGetTextureParameteriv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTextureParameteriv.xhtml)
  /// * `texture` class: texture
  /// * `pname` group: GetTextureParameter
  pub unsafe fn GetTextureParameteriv(&self, texture: GLuint, pname: GetTextureParameter, params: *mut GLint) {
    (self.glGetTextureParameteriv_p)(texture, pname, params)
  }
  /// [glGetTextureSubImage](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTextureSubImage.xhtml)
  /// * `texture` class: texture
  /// * `format` group: PixelFormat
  /// * `type` group: PixelType
  pub unsafe fn GetTextureSubImage(&self, texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: PixelFormat, type_: PixelType, bufSize: GLsizei, pixels: *mut void) {
    (self.glGetTextureSubImage_p)(texture, level, xoffset, yoffset, zoffset, width, height, depth, format, type_, bufSize, pixels)
  }
  /// [glGetTransformFeedbackVarying](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTransformFeedbackVarying.xhtml)
  /// * `program` class: program
  /// * `type` group: AttributeType
  /// * `name` len: bufSize
  pub unsafe fn GetTransformFeedbackVarying(&self, program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLsizei, type_: *mut AttributeType, name: *mut GLchar) {
    (self.glGetTransformFeedbackVarying_p)(program, index, bufSize, length, size, type_, name)
  }
  /// [glGetTransformFeedbacki64_v](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTransformFeedbacki64_v.xhtml)
  /// * `xfb` class: transform feedback
  /// * `pname` group: TransformFeedbackPName
  pub unsafe fn GetTransformFeedbacki64_v(&self, xfb: GLuint, pname: TransformFeedbackPName, index: GLuint, param: *mut GLint64) {
    (self.glGetTransformFeedbacki64_v_p)(xfb, pname, index, param)
  }
  /// [glGetTransformFeedbacki_v](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTransformFeedbacki_v.xhtml)
  /// * `xfb` class: transform feedback
  /// * `pname` group: TransformFeedbackPName
  pub unsafe fn GetTransformFeedbacki_v(&self, xfb: GLuint, pname: TransformFeedbackPName, index: GLuint, param: *mut GLint) {
    (self.glGetTransformFeedbacki_v_p)(xfb, pname, index, param)
  }
  /// [glGetTransformFeedbackiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetTransformFeedbackiv.xhtml)
  /// * `xfb` class: transform feedback
  /// * `pname` group: TransformFeedbackPName
  pub unsafe fn GetTransformFeedbackiv(&self, xfb: GLuint, pname: TransformFeedbackPName, param: *mut GLint) {
    (self.glGetTransformFeedbackiv_p)(xfb, pname, param)
  }
  /// [glGetUniformBlockIndex](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetUniformBlockIndex.xhtml)
  /// * `program` class: program
  /// * `uniformBlockName` len: COMPSIZE()
  pub unsafe fn GetUniformBlockIndex(&self, program: GLuint, uniformBlockName: *const GLchar) -> GLuint {
    (self.glGetUniformBlockIndex_p)(program, uniformBlockName)
  }
  /// [glGetUniformIndices](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetUniformIndices.xhtml)
  /// * `program` class: program
  /// * `uniformNames` len: COMPSIZE(uniformCount)
  /// * `uniformIndices` len: COMPSIZE(uniformCount)
  pub unsafe fn GetUniformIndices(&self, program: GLuint, uniformCount: GLsizei, uniformNames: *const *const GLchar, uniformIndices: *mut GLuint) {
    (self.glGetUniformIndices_p)(program, uniformCount, uniformNames, uniformIndices)
  }
  /// [glGetUniformLocation](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetUniformLocation.xhtml)
  /// * `program` class: program
  pub unsafe fn GetUniformLocation(&self, program: GLuint, name: *const GLchar) -> GLint {
    (self.glGetUniformLocation_p)(program, name)
  }
  /// [glGetUniformSubroutineuiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetUniformSubroutineuiv.xhtml)
  /// * `shadertype` group: ShaderType
  pub unsafe fn GetUniformSubroutineuiv(&self, shadertype: ShaderType, location: GLint, params: *mut GLuint) {
    (self.glGetUniformSubroutineuiv_p)(shadertype, location, params)
  }
  /// [glGetUniformdv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetUniformdv.xhtml)
  /// * `program` class: program
  /// * `params` len: COMPSIZE(program,location)
  pub unsafe fn GetUniformdv(&self, program: GLuint, location: GLint, params: *mut GLdouble) {
    (self.glGetUniformdv_p)(program, location, params)
  }
  /// [glGetUniformfv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetUniformfv.xhtml)
  /// * `program` class: program
  /// * `params` len: COMPSIZE(program,location)
  pub unsafe fn GetUniformfv(&self, program: GLuint, location: GLint, params: *mut GLfloat) {
    (self.glGetUniformfv_p)(program, location, params)
  }
  /// [glGetUniformiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetUniformiv.xhtml)
  /// * `program` class: program
  /// * `params` len: COMPSIZE(program,location)
  pub unsafe fn GetUniformiv(&self, program: GLuint, location: GLint, params: *mut GLint) {
    (self.glGetUniformiv_p)(program, location, params)
  }
  /// [glGetUniformuiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetUniformuiv.xhtml)
  /// * `program` class: program
  /// * `params` len: COMPSIZE(program,location)
  pub unsafe fn GetUniformuiv(&self, program: GLuint, location: GLint, params: *mut GLuint) {
    (self.glGetUniformuiv_p)(program, location, params)
  }
  /// [glGetVertexArrayIndexed64iv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetVertexArrayIndexed64iv.xhtml)
  /// * `vaobj` class: vertex array
  /// * `pname` group: VertexArrayPName
  pub unsafe fn GetVertexArrayIndexed64iv(&self, vaobj: GLuint, index: GLuint, pname: VertexArrayPName, param: *mut GLint64) {
    (self.glGetVertexArrayIndexed64iv_p)(vaobj, index, pname, param)
  }
  /// [glGetVertexArrayIndexediv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetVertexArrayIndexediv.xhtml)
  /// * `vaobj` class: vertex array
  /// * `pname` group: VertexArrayPName
  pub unsafe fn GetVertexArrayIndexediv(&self, vaobj: GLuint, index: GLuint, pname: VertexArrayPName, param: *mut GLint) {
    (self.glGetVertexArrayIndexediv_p)(vaobj, index, pname, param)
  }
  /// [glGetVertexArrayiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetVertexArrayiv.xhtml)
  /// * `vaobj` class: vertex array
  /// * `pname` group: VertexArrayPName
  pub unsafe fn GetVertexArrayiv(&self, vaobj: GLuint, pname: VertexArrayPName, param: *mut GLint) {
    (self.glGetVertexArrayiv_p)(vaobj, pname, param)
  }
  /// [glGetVertexAttribIiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetVertexAttribIiv.xhtml)
  /// * `pname` group: VertexAttribEnum
  pub unsafe fn GetVertexAttribIiv(&self, index: GLuint, pname: VertexAttribEnum, params: *mut GLint) {
    (self.glGetVertexAttribIiv_p)(index, pname, params)
  }
  /// [glGetVertexAttribIuiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetVertexAttribIuiv.xhtml)
  /// * `pname` group: VertexAttribEnum
  pub unsafe fn GetVertexAttribIuiv(&self, index: GLuint, pname: VertexAttribEnum, params: *mut GLuint) {
    (self.glGetVertexAttribIuiv_p)(index, pname, params)
  }
  /// [glGetVertexAttribLdv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetVertexAttribLdv.xhtml)
  /// * `pname` group: VertexAttribEnum
  /// * `params` len: COMPSIZE(pname)
  pub unsafe fn GetVertexAttribLdv(&self, index: GLuint, pname: VertexAttribEnum, params: *mut GLdouble) {
    (self.glGetVertexAttribLdv_p)(index, pname, params)
  }
  /// [glGetVertexAttribPointerv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetVertexAttribPointerv.xhtml)
  /// * `pname` group: VertexAttribPointerPropertyARB
  pub unsafe fn GetVertexAttribPointerv(&self, index: GLuint, pname: VertexAttribPointerPropertyARB, pointer: *mut *mut void) {
    (self.glGetVertexAttribPointerv_p)(index, pname, pointer)
  }
  /// [glGetVertexAttribdv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetVertexAttribdv.xhtml)
  /// * `pname` group: VertexAttribPropertyARB
  pub unsafe fn GetVertexAttribdv(&self, index: GLuint, pname: VertexAttribPropertyARB, params: *mut [GLdouble; 4]) {
    (self.glGetVertexAttribdv_p)(index, pname, params)
  }
  /// [glGetVertexAttribfv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetVertexAttribfv.xhtml)
  /// * `pname` group: VertexAttribPropertyARB
  pub unsafe fn GetVertexAttribfv(&self, index: GLuint, pname: VertexAttribPropertyARB, params: *mut [GLfloat; 4]) {
    (self.glGetVertexAttribfv_p)(index, pname, params)
  }
  /// [glGetVertexAttribiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetVertexAttribiv.xhtml)
  /// * `pname` group: VertexAttribPropertyARB
  pub unsafe fn GetVertexAttribiv(&self, index: GLuint, pname: VertexAttribPropertyARB, params: *mut [GLint; 4]) {
    (self.glGetVertexAttribiv_p)(index, pname, params)
  }
  /// [glGetnColorTable](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetnColorTable.xhtml)
  /// * `target` group: ColorTableTarget
  /// * `format` group: PixelFormat
  /// * `type` group: PixelType
  /// * `table` len: bufSize
  pub unsafe fn GetnColorTable(&self, target: ColorTableTarget, format: PixelFormat, type_: PixelType, bufSize: GLsizei, table: *mut void) {
    (self.glGetnColorTable_p)(target, format, type_, bufSize, table)
  }
  /// [glGetnCompressedTexImage](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetnCompressedTexImage.xhtml)
  /// * `target` group: TextureTarget
  /// * `pixels` len: bufSize
  pub unsafe fn GetnCompressedTexImage(&self, target: TextureTarget, lod: GLint, bufSize: GLsizei, pixels: *mut void) {
    (self.glGetnCompressedTexImage_p)(target, lod, bufSize, pixels)
  }
  /// [glGetnConvolutionFilter](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetnConvolutionFilter.xhtml)
  /// * `target` group: ConvolutionTarget
  /// * `format` group: PixelFormat
  /// * `type` group: PixelType
  /// * `image` len: bufSize
  pub unsafe fn GetnConvolutionFilter(&self, target: ConvolutionTarget, format: PixelFormat, type_: PixelType, bufSize: GLsizei, image: *mut void) {
    (self.glGetnConvolutionFilter_p)(target, format, type_, bufSize, image)
  }
  /// [glGetnHistogram](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetnHistogram.xhtml)
  /// * `target` group: HistogramTarget
  /// * `reset` group: Boolean
  /// * `format` group: PixelFormat
  /// * `type` group: PixelType
  /// * `values` len: bufSize
  pub unsafe fn GetnHistogram(&self, target: HistogramTarget, reset: GLboolean, format: PixelFormat, type_: PixelType, bufSize: GLsizei, values: *mut void) {
    (self.glGetnHistogram_p)(target, reset, format, type_, bufSize, values)
  }
  /// [glGetnMapdv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetnMapdv.xhtml)
  /// * `target` group: MapTarget
  /// * `query` group: MapQuery
  /// * `v` len: COMPSIZE(bufSize)
  pub unsafe fn GetnMapdv(&self, target: MapTarget, query: MapQuery, bufSize: GLsizei, v: *mut GLdouble) {
    (self.glGetnMapdv_p)(target, query, bufSize, v)
  }
  /// [glGetnMapfv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetnMapfv.xhtml)
  /// * `target` group: MapTarget
  /// * `query` group: MapQuery
  pub unsafe fn GetnMapfv(&self, target: MapTarget, query: MapQuery, bufSize: GLsizei, v: *mut GLfloat) {
    (self.glGetnMapfv_p)(target, query, bufSize, v)
  }
  /// [glGetnMapiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetnMapiv.xhtml)
  /// * `target` group: MapTarget
  /// * `query` group: MapQuery
  pub unsafe fn GetnMapiv(&self, target: MapTarget, query: MapQuery, bufSize: GLsizei, v: *mut GLint) {
    (self.glGetnMapiv_p)(target, query, bufSize, v)
  }
  /// [glGetnMinmax](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetnMinmax.xhtml)
  /// * `target` group: MinmaxTarget
  /// * `reset` group: Boolean
  /// * `format` group: PixelFormat
  /// * `type` group: PixelType
  /// * `values` len: bufSize
  pub unsafe fn GetnMinmax(&self, target: MinmaxTarget, reset: GLboolean, format: PixelFormat, type_: PixelType, bufSize: GLsizei, values: *mut void) {
    (self.glGetnMinmax_p)(target, reset, format, type_, bufSize, values)
  }
  /// [glGetnPixelMapfv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetnPixelMapfv.xhtml)
  /// * `map` group: PixelMap
  /// * `values` len: COMPSIZE(bufSize)
  pub unsafe fn GetnPixelMapfv(&self, map: PixelMap, bufSize: GLsizei, values: *mut GLfloat) {
    (self.glGetnPixelMapfv_p)(map, bufSize, values)
  }
  /// [glGetnPixelMapuiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetnPixelMapuiv.xhtml)
  /// * `map` group: PixelMap
  pub unsafe fn GetnPixelMapuiv(&self, map: PixelMap, bufSize: GLsizei, values: *mut GLuint) {
    (self.glGetnPixelMapuiv_p)(map, bufSize, values)
  }
  /// [glGetnPixelMapusv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetnPixelMapusv.xhtml)
  /// * `map` group: PixelMap
  pub unsafe fn GetnPixelMapusv(&self, map: PixelMap, bufSize: GLsizei, values: *mut GLushort) {
    (self.glGetnPixelMapusv_p)(map, bufSize, values)
  }
  /// [glGetnPolygonStipple](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetnPolygonStipple.xhtml)
  /// * `pattern` len: bufSize
  pub unsafe fn GetnPolygonStipple(&self, bufSize: GLsizei, pattern: *mut GLubyte) {
    (self.glGetnPolygonStipple_p)(bufSize, pattern)
  }
  /// [glGetnSeparableFilter](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetnSeparableFilter.xhtml)
  /// * `target` group: SeparableTarget
  /// * `format` group: PixelFormat
  /// * `type` group: PixelType
  /// * `row` len: rowBufSize
  /// * `column` len: columnBufSize
  pub unsafe fn GetnSeparableFilter(&self, target: SeparableTarget, format: PixelFormat, type_: PixelType, rowBufSize: GLsizei, row: *mut void, columnBufSize: GLsizei, column: *mut void, span: *mut void) {
    (self.glGetnSeparableFilter_p)(target, format, type_, rowBufSize, row, columnBufSize, column, span)
  }
  /// [glGetnTexImage](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetnTexImage.xhtml)
  /// * `target` group: TextureTarget
  /// * `format` group: PixelFormat
  /// * `type` group: PixelType
  /// * `pixels` len: bufSize
  pub unsafe fn GetnTexImage(&self, target: TextureTarget, level: GLint, format: PixelFormat, type_: PixelType, bufSize: GLsizei, pixels: *mut void) {
    (self.glGetnTexImage_p)(target, level, format, type_, bufSize, pixels)
  }
  /// [glGetnUniformdv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetnUniformdv.xhtml)
  /// * `program` class: program
  /// * `params` len: bufSize / 8
  pub unsafe fn GetnUniformdv(&self, program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLdouble) {
    (self.glGetnUniformdv_p)(program, location, bufSize, params)
  }
  /// [glGetnUniformfv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetnUniformfv.xhtml)
  /// * `program` class: program
  /// * `params` len: bufSize / 4
  pub unsafe fn GetnUniformfv(&self, program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLfloat) {
    (self.glGetnUniformfv_p)(program, location, bufSize, params)
  }
  /// [glGetnUniformiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetnUniformiv.xhtml)
  /// * `program` class: program
  /// * `params` len: bufSize / 4
  pub unsafe fn GetnUniformiv(&self, program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLint) {
    (self.glGetnUniformiv_p)(program, location, bufSize, params)
  }
  /// [glGetnUniformuiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetnUniformuiv.xhtml)
  /// * `program` class: program
  /// * `params` len: bufSize / 4
  pub unsafe fn GetnUniformuiv(&self, program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLuint) {
    (self.glGetnUniformuiv_p)(program, location, bufSize, params)
  }
  /// [glHint](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glHint.xhtml)
  /// * `target` group: HintTarget
  /// * `mode` group: HintMode
  pub unsafe fn Hint(&self, target: HintTarget, mode: HintMode) {
    (self.glHint_p)(target, mode)
  }
  /// [glInvalidateBufferData](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glInvalidateBufferData.xhtml)
  /// * `buffer` class: buffer
  pub unsafe fn InvalidateBufferData(&self, buffer: GLuint) {
    (self.glInvalidateBufferData_p)(buffer)
  }
  /// [glInvalidateBufferSubData](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glInvalidateBufferSubData.xhtml)
  /// * `buffer` class: buffer
  /// * `offset` group: BufferOffset
  /// * `length` group: BufferSize
  pub unsafe fn InvalidateBufferSubData(&self, buffer: GLuint, offset: GLintptr, length: GLsizeiptr) {
    (self.glInvalidateBufferSubData_p)(buffer, offset, length)
  }
  /// [glInvalidateFramebuffer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glInvalidateFramebuffer.xhtml)
  /// * `target` group: FramebufferTarget
  /// * `attachments` group: InvalidateFramebufferAttachment
  /// * `attachments` len: numAttachments
  pub unsafe fn InvalidateFramebuffer(&self, target: FramebufferTarget, numAttachments: GLsizei, attachments: *const InvalidateFramebufferAttachment) {
    (self.glInvalidateFramebuffer_p)(target, numAttachments, attachments)
  }
  /// [glInvalidateNamedFramebufferData](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glInvalidateNamedFramebufferData.xhtml)
  /// * `framebuffer` class: framebuffer
  /// * `attachments` group: FramebufferAttachment
  pub unsafe fn InvalidateNamedFramebufferData(&self, framebuffer: GLuint, numAttachments: GLsizei, attachments: *const FramebufferAttachment) {
    (self.glInvalidateNamedFramebufferData_p)(framebuffer, numAttachments, attachments)
  }
  /// [glInvalidateNamedFramebufferSubData](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glInvalidateNamedFramebufferSubData.xhtml)
  /// * `framebuffer` class: framebuffer
  /// * `attachments` group: FramebufferAttachment
  pub unsafe fn InvalidateNamedFramebufferSubData(&self, framebuffer: GLuint, numAttachments: GLsizei, attachments: *const FramebufferAttachment, x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
    (self.glInvalidateNamedFramebufferSubData_p)(framebuffer, numAttachments, attachments, x, y, width, height)
  }
  /// [glInvalidateSubFramebuffer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glInvalidateSubFramebuffer.xhtml)
  /// * `target` group: FramebufferTarget
  /// * `attachments` group: InvalidateFramebufferAttachment
  /// * `attachments` len: numAttachments
  pub unsafe fn InvalidateSubFramebuffer(&self, target: FramebufferTarget, numAttachments: GLsizei, attachments: *const InvalidateFramebufferAttachment, x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
    (self.glInvalidateSubFramebuffer_p)(target, numAttachments, attachments, x, y, width, height)
  }
  /// [glInvalidateTexImage](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glInvalidateTexImage.xhtml)
  /// * `texture` class: texture
  pub unsafe fn InvalidateTexImage(&self, texture: GLuint, level: GLint) {
    (self.glInvalidateTexImage_p)(texture, level)
  }
  /// [glInvalidateTexSubImage](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glInvalidateTexSubImage.xhtml)
  /// * `texture` class: texture
  pub unsafe fn InvalidateTexSubImage(&self, texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei) {
    (self.glInvalidateTexSubImage_p)(texture, level, xoffset, yoffset, zoffset, width, height, depth)
  }
  /// [glIsBuffer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIsBuffer.xhtml)
  /// * `buffer` class: buffer
  pub unsafe fn IsBuffer(&self, buffer: GLuint) -> GLboolean {
    (self.glIsBuffer_p)(buffer)
  }
  /// [glIsEnabled](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIsEnabled.xhtml)
  /// * `cap` group: EnableCap
  pub unsafe fn IsEnabled(&self, cap: EnableCap) -> GLboolean {
    (self.glIsEnabled_p)(cap)
  }
  /// [glIsEnabledi](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIsEnabledi.xhtml)
  /// * `target` group: EnableCap
  pub unsafe fn IsEnabledi(&self, target: EnableCap, index: GLuint) -> GLboolean {
    (self.glIsEnabledi_p)(target, index)
  }
  /// [glIsFramebuffer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIsFramebuffer.xhtml)
  /// * `framebuffer` class: framebuffer
  pub unsafe fn IsFramebuffer(&self, framebuffer: GLuint) -> GLboolean {
    (self.glIsFramebuffer_p)(framebuffer)
  }
  /// [glIsProgram](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIsProgram.xhtml)
  /// * `program` class: program
  pub unsafe fn IsProgram(&self, program: GLuint) -> GLboolean {
    (self.glIsProgram_p)(program)
  }
  /// [glIsProgramPipeline](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIsProgramPipeline.xhtml)
  /// * `pipeline` class: program pipeline
  pub unsafe fn IsProgramPipeline(&self, pipeline: GLuint) -> GLboolean {
    (self.glIsProgramPipeline_p)(pipeline)
  }
  /// [glIsQuery](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIsQuery.xhtml)
  /// * `id` class: query
  pub unsafe fn IsQuery(&self, id: GLuint) -> GLboolean {
    (self.glIsQuery_p)(id)
  }
  /// [glIsRenderbuffer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIsRenderbuffer.xhtml)
  /// * `renderbuffer` class: renderbuffer
  pub unsafe fn IsRenderbuffer(&self, renderbuffer: GLuint) -> GLboolean {
    (self.glIsRenderbuffer_p)(renderbuffer)
  }
  /// [glIsSampler](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIsSampler.xhtml)
  /// * `sampler` class: sampler
  pub unsafe fn IsSampler(&self, sampler: GLuint) -> GLboolean {
    (self.glIsSampler_p)(sampler)
  }
  /// [glIsShader](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIsShader.xhtml)
  /// * `shader` class: shader
  pub unsafe fn IsShader(&self, shader: GLuint) -> GLboolean {
    (self.glIsShader_p)(shader)
  }
  /// [glIsSync](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIsSync.xhtml)
  /// * `sync` group: sync
  /// * `sync` class: sync
  pub unsafe fn IsSync(&self, sync: GLsync) -> GLboolean {
    (self.glIsSync_p)(sync)
  }
  /// [glIsTexture](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIsTexture.xhtml)
  /// * `texture` group: Texture
  /// * `texture` class: texture
  pub unsafe fn IsTexture(&self, texture: GLuint) -> GLboolean {
    (self.glIsTexture_p)(texture)
  }
  /// [glIsTransformFeedback](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIsTransformFeedback.xhtml)
  /// * `id` class: transform feedback
  pub unsafe fn IsTransformFeedback(&self, id: GLuint) -> GLboolean {
    (self.glIsTransformFeedback_p)(id)
  }
  /// [glIsVertexArray](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glIsVertexArray.xhtml)
  /// * `array` class: vertex array
  pub unsafe fn IsVertexArray(&self, array: GLuint) -> GLboolean {
    (self.glIsVertexArray_p)(array)
  }
  /// [glLineWidth](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glLineWidth.xhtml)
  /// * `width` group: CheckedFloat32
  pub unsafe fn LineWidth(&self, width: GLfloat) {
    (self.glLineWidth_p)(width)
  }
  /// [glLinkProgram](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glLinkProgram.xhtml)
  /// * `program` class: program
  pub unsafe fn LinkProgram(&self, program: GLuint) {
    (self.glLinkProgram_p)(program)
  }
  /// [glLogicOp](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glLogicOp.xhtml)
  /// * `opcode` group: LogicOp
  pub unsafe fn LogicOp(&self, opcode: LogicOp) {
    (self.glLogicOp_p)(opcode)
  }
  /// [glMapBuffer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMapBuffer.xhtml)
  /// * `target` group: BufferTargetARB
  /// * `access` group: BufferAccessARB
  pub unsafe fn MapBuffer(&self, target: BufferTargetARB, access: BufferAccessARB) -> *mut void {
    (self.glMapBuffer_p)(target, access)
  }
  /// [glMapBufferRange](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMapBufferRange.xhtml)
  /// * `target` group: BufferTargetARB
  /// * `offset` group: BufferOffset
  /// * `length` group: BufferSize
  /// * `access` group: MapBufferAccessMask
  pub unsafe fn MapBufferRange(&self, target: BufferTargetARB, offset: GLintptr, length: GLsizeiptr, access: GLbitfield) -> *mut void {
    (self.glMapBufferRange_p)(target, offset, length, access)
  }
  /// [glMapNamedBuffer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMapNamedBuffer.xhtml)
  /// * `buffer` class: buffer
  /// * `access` group: BufferAccessARB
  pub unsafe fn MapNamedBuffer(&self, buffer: GLuint, access: BufferAccessARB) -> *mut void {
    (self.glMapNamedBuffer_p)(buffer, access)
  }
  /// [glMapNamedBufferRange](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMapNamedBufferRange.xhtml)
  /// * `buffer` class: buffer
  /// * `length` group: BufferSize
  /// * `access` group: MapBufferAccessMask
  pub unsafe fn MapNamedBufferRange(&self, buffer: GLuint, offset: GLintptr, length: GLsizeiptr, access: GLbitfield) -> *mut void {
    (self.glMapNamedBufferRange_p)(buffer, offset, length, access)
  }
  /// [glMemoryBarrier](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMemoryBarrier.xhtml)
  /// * `barriers` group: MemoryBarrierMask
  pub unsafe fn MemoryBarrier(&self, barriers: GLbitfield) {
    (self.glMemoryBarrier_p)(barriers)
  }
  /// [glMemoryBarrierByRegion](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMemoryBarrierByRegion.xhtml)
  /// * `barriers` group: MemoryBarrierMask
  pub unsafe fn MemoryBarrierByRegion(&self, barriers: GLbitfield) {
    (self.glMemoryBarrierByRegion_p)(barriers)
  }
  /// [glMinSampleShading](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMinSampleShading.xhtml)
  /// * `value` group: ColorF
  pub unsafe fn MinSampleShading(&self, value: GLfloat) {
    (self.glMinSampleShading_p)(value)
  }
  /// [glMultiDrawArrays](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiDrawArrays.xhtml)
  /// * `mode` group: PrimitiveType
  /// * `first` len: COMPSIZE(drawcount)
  /// * `count` len: COMPSIZE(drawcount)
  pub unsafe fn MultiDrawArrays(&self, mode: PrimitiveType, first: *const GLint, count: *const GLsizei, drawcount: GLsizei) {
    (self.glMultiDrawArrays_p)(mode, first, count, drawcount)
  }
  /// [glMultiDrawArraysIndirect](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiDrawArraysIndirect.xhtml)
  /// * `mode` group: PrimitiveType
  /// * `indirect` len: COMPSIZE(drawcount,stride)
  pub unsafe fn MultiDrawArraysIndirect(&self, mode: PrimitiveType, indirect: *const void, drawcount: GLsizei, stride: GLsizei) {
    (self.glMultiDrawArraysIndirect_p)(mode, indirect, drawcount, stride)
  }
  /// [glMultiDrawArraysIndirectCount](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiDrawArraysIndirectCount.xhtml)
  /// * `mode` group: PrimitiveType
  pub unsafe fn MultiDrawArraysIndirectCount(&self, mode: PrimitiveType, indirect: *const void, drawcount: GLintptr, maxdrawcount: GLsizei, stride: GLsizei) {
    (self.glMultiDrawArraysIndirectCount_p)(mode, indirect, drawcount, maxdrawcount, stride)
  }
  /// [glMultiDrawElements](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiDrawElements.xhtml)
  /// * `mode` group: PrimitiveType
  /// * `count` len: COMPSIZE(drawcount)
  /// * `type` group: DrawElementsType
  /// * `indices` len: COMPSIZE(drawcount)
  pub unsafe fn MultiDrawElements(&self, mode: PrimitiveType, count: *const GLsizei, type_: DrawElementsType, indices: *const *const void, drawcount: GLsizei) {
    (self.glMultiDrawElements_p)(mode, count, type_, indices, drawcount)
  }
  /// [glMultiDrawElementsBaseVertex](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiDrawElementsBaseVertex.xhtml)
  /// * `mode` group: PrimitiveType
  /// * `count` len: COMPSIZE(drawcount)
  /// * `type` group: DrawElementsType
  /// * `indices` len: COMPSIZE(drawcount)
  /// * `basevertex` len: COMPSIZE(drawcount)
  pub unsafe fn MultiDrawElementsBaseVertex(&self, mode: PrimitiveType, count: *const GLsizei, type_: DrawElementsType, indices: *const *const void, drawcount: GLsizei, basevertex: *const GLint) {
    (self.glMultiDrawElementsBaseVertex_p)(mode, count, type_, indices, drawcount, basevertex)
  }
  /// [glMultiDrawElementsIndirect](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiDrawElementsIndirect.xhtml)
  /// * `mode` group: PrimitiveType
  /// * `type` group: DrawElementsType
  /// * `indirect` len: COMPSIZE(drawcount,stride)
  pub unsafe fn MultiDrawElementsIndirect(&self, mode: PrimitiveType, type_: DrawElementsType, indirect: *const void, drawcount: GLsizei, stride: GLsizei) {
    (self.glMultiDrawElementsIndirect_p)(mode, type_, indirect, drawcount, stride)
  }
  /// [glMultiDrawElementsIndirectCount](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiDrawElementsIndirectCount.xhtml)
  /// * `mode` group: PrimitiveType
  /// * `type` group: DrawElementsType
  pub unsafe fn MultiDrawElementsIndirectCount(&self, mode: PrimitiveType, type_: DrawElementsType, indirect: *const void, drawcount: GLintptr, maxdrawcount: GLsizei, stride: GLsizei) {
    (self.glMultiDrawElementsIndirectCount_p)(mode, type_, indirect, drawcount, maxdrawcount, stride)
  }
  /// [glMultiTexCoordP1ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoordP1ui.xhtml)
  /// * `texture` group: TextureUnit
  /// * `type` group: TexCoordPointerType
  pub unsafe fn MultiTexCoordP1ui(&self, texture: TextureUnit, type_: TexCoordPointerType, coords: GLuint) {
    (self.glMultiTexCoordP1ui_p)(texture, type_, coords)
  }
  /// [glMultiTexCoordP1uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoordP1uiv.xhtml)
  /// * `texture` group: TextureUnit
  /// * `type` group: TexCoordPointerType
  pub unsafe fn MultiTexCoordP1uiv(&self, texture: TextureUnit, type_: TexCoordPointerType, coords: *const GLuint) {
    (self.glMultiTexCoordP1uiv_p)(texture, type_, coords)
  }
  /// [glMultiTexCoordP2ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoordP2ui.xhtml)
  /// * `texture` group: TextureUnit
  /// * `type` group: TexCoordPointerType
  pub unsafe fn MultiTexCoordP2ui(&self, texture: TextureUnit, type_: TexCoordPointerType, coords: GLuint) {
    (self.glMultiTexCoordP2ui_p)(texture, type_, coords)
  }
  /// [glMultiTexCoordP2uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoordP2uiv.xhtml)
  /// * `texture` group: TextureUnit
  /// * `type` group: TexCoordPointerType
  pub unsafe fn MultiTexCoordP2uiv(&self, texture: TextureUnit, type_: TexCoordPointerType, coords: *const GLuint) {
    (self.glMultiTexCoordP2uiv_p)(texture, type_, coords)
  }
  /// [glMultiTexCoordP3ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoordP3ui.xhtml)
  /// * `texture` group: TextureUnit
  /// * `type` group: TexCoordPointerType
  pub unsafe fn MultiTexCoordP3ui(&self, texture: TextureUnit, type_: TexCoordPointerType, coords: GLuint) {
    (self.glMultiTexCoordP3ui_p)(texture, type_, coords)
  }
  /// [glMultiTexCoordP3uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoordP3uiv.xhtml)
  /// * `texture` group: TextureUnit
  /// * `type` group: TexCoordPointerType
  pub unsafe fn MultiTexCoordP3uiv(&self, texture: TextureUnit, type_: TexCoordPointerType, coords: *const GLuint) {
    (self.glMultiTexCoordP3uiv_p)(texture, type_, coords)
  }
  /// [glMultiTexCoordP4ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoordP4ui.xhtml)
  /// * `texture` group: TextureUnit
  /// * `type` group: TexCoordPointerType
  pub unsafe fn MultiTexCoordP4ui(&self, texture: TextureUnit, type_: TexCoordPointerType, coords: GLuint) {
    (self.glMultiTexCoordP4ui_p)(texture, type_, coords)
  }
  /// [glMultiTexCoordP4uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glMultiTexCoordP4uiv.xhtml)
  /// * `texture` group: TextureUnit
  /// * `type` group: TexCoordPointerType
  pub unsafe fn MultiTexCoordP4uiv(&self, texture: TextureUnit, type_: TexCoordPointerType, coords: *const GLuint) {
    (self.glMultiTexCoordP4uiv_p)(texture, type_, coords)
  }
  /// [glNamedBufferData](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNamedBufferData.xhtml)
  /// * `buffer` class: buffer
  /// * `size` group: BufferSize
  /// * `usage` group: VertexBufferObjectUsage
  pub unsafe fn NamedBufferData(&self, buffer: GLuint, size: GLsizeiptr, data: *const void, usage: VertexBufferObjectUsage) {
    (self.glNamedBufferData_p)(buffer, size, data, usage)
  }
  /// [glNamedBufferStorage](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNamedBufferStorage.xhtml)
  /// * `buffer` class: buffer
  /// * `size` group: BufferSize
  /// * `data` len: size
  /// * `flags` group: BufferStorageMask
  pub unsafe fn NamedBufferStorage(&self, buffer: GLuint, size: GLsizeiptr, data: *const void, flags: GLbitfield) {
    (self.glNamedBufferStorage_p)(buffer, size, data, flags)
  }
  /// [glNamedBufferSubData](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNamedBufferSubData.xhtml)
  /// * `buffer` class: buffer
  /// * `size` group: BufferSize
  /// * `data` len: COMPSIZE(size)
  pub unsafe fn NamedBufferSubData(&self, buffer: GLuint, offset: GLintptr, size: GLsizeiptr, data: *const void) {
    (self.glNamedBufferSubData_p)(buffer, offset, size, data)
  }
  /// [glNamedFramebufferDrawBuffer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNamedFramebufferDrawBuffer.xhtml)
  /// * `framebuffer` class: framebuffer
  /// * `buf` group: ColorBuffer
  pub unsafe fn NamedFramebufferDrawBuffer(&self, framebuffer: GLuint, buf: ColorBuffer) {
    (self.glNamedFramebufferDrawBuffer_p)(framebuffer, buf)
  }
  /// [glNamedFramebufferDrawBuffers](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNamedFramebufferDrawBuffers.xhtml)
  /// * `framebuffer` class: framebuffer
  /// * `bufs` group: ColorBuffer
  pub unsafe fn NamedFramebufferDrawBuffers(&self, framebuffer: GLuint, n: GLsizei, bufs: *const ColorBuffer) {
    (self.glNamedFramebufferDrawBuffers_p)(framebuffer, n, bufs)
  }
  /// [glNamedFramebufferParameteri](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNamedFramebufferParameteri.xhtml)
  /// * `framebuffer` class: framebuffer
  /// * `pname` group: FramebufferParameterName
  pub unsafe fn NamedFramebufferParameteri(&self, framebuffer: GLuint, pname: FramebufferParameterName, param: GLint) {
    (self.glNamedFramebufferParameteri_p)(framebuffer, pname, param)
  }
  /// [glNamedFramebufferReadBuffer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNamedFramebufferReadBuffer.xhtml)
  /// * `framebuffer` class: framebuffer
  /// * `src` group: ColorBuffer
  pub unsafe fn NamedFramebufferReadBuffer(&self, framebuffer: GLuint, src: ColorBuffer) {
    (self.glNamedFramebufferReadBuffer_p)(framebuffer, src)
  }
  /// [glNamedFramebufferRenderbuffer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNamedFramebufferRenderbuffer.xhtml)
  /// * `framebuffer` class: framebuffer
  /// * `attachment` group: FramebufferAttachment
  /// * `renderbuffertarget` group: RenderbufferTarget
  /// * `renderbuffer` class: renderbuffer
  pub unsafe fn NamedFramebufferRenderbuffer(&self, framebuffer: GLuint, attachment: FramebufferAttachment, renderbuffertarget: RenderbufferTarget, renderbuffer: GLuint) {
    (self.glNamedFramebufferRenderbuffer_p)(framebuffer, attachment, renderbuffertarget, renderbuffer)
  }
  /// [glNamedFramebufferTexture](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNamedFramebufferTexture.xhtml)
  /// * `framebuffer` class: framebuffer
  /// * `attachment` group: FramebufferAttachment
  /// * `texture` class: texture
  pub unsafe fn NamedFramebufferTexture(&self, framebuffer: GLuint, attachment: FramebufferAttachment, texture: GLuint, level: GLint) {
    (self.glNamedFramebufferTexture_p)(framebuffer, attachment, texture, level)
  }
  /// [glNamedFramebufferTextureLayer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNamedFramebufferTextureLayer.xhtml)
  /// * `framebuffer` class: framebuffer
  /// * `attachment` group: FramebufferAttachment
  /// * `texture` class: texture
  pub unsafe fn NamedFramebufferTextureLayer(&self, framebuffer: GLuint, attachment: FramebufferAttachment, texture: GLuint, level: GLint, layer: GLint) {
    (self.glNamedFramebufferTextureLayer_p)(framebuffer, attachment, texture, level, layer)
  }
  /// [glNamedRenderbufferStorage](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNamedRenderbufferStorage.xhtml)
  /// * `renderbuffer` class: renderbuffer
  /// * `internalformat` group: InternalFormat
  pub unsafe fn NamedRenderbufferStorage(&self, renderbuffer: GLuint, internalformat: InternalFormat, width: GLsizei, height: GLsizei) {
    (self.glNamedRenderbufferStorage_p)(renderbuffer, internalformat, width, height)
  }
  /// [glNamedRenderbufferStorageMultisample](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNamedRenderbufferStorageMultisample.xhtml)
  /// * `renderbuffer` class: renderbuffer
  /// * `internalformat` group: InternalFormat
  pub unsafe fn NamedRenderbufferStorageMultisample(&self, renderbuffer: GLuint, samples: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei) {
    (self.glNamedRenderbufferStorageMultisample_p)(renderbuffer, samples, internalformat, width, height)
  }
  /// [glNormalP3ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNormalP3ui.xhtml)
  /// * `type` group: NormalPointerType
  pub unsafe fn NormalP3ui(&self, type_: NormalPointerType, coords: GLuint) {
    (self.glNormalP3ui_p)(type_, coords)
  }
  /// [glNormalP3uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glNormalP3uiv.xhtml)
  /// * `type` group: NormalPointerType
  pub unsafe fn NormalP3uiv(&self, type_: NormalPointerType, coords: *const GLuint) {
    (self.glNormalP3uiv_p)(type_, coords)
  }
  /// [glObjectLabel](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glObjectLabel.xhtml)
  /// * `identifier` group: ObjectIdentifier
  /// * `label` len: COMPSIZE(label,length)
  pub unsafe fn ObjectLabel(&self, identifier: ObjectIdentifier, name: GLuint, length: GLsizei, label: *const GLchar) {
    (self.glObjectLabel_p)(identifier, name, length, label)
  }
  /// [glObjectPtrLabel](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glObjectPtrLabel.xhtml)
  /// * `label` len: COMPSIZE(label,length)
  pub unsafe fn ObjectPtrLabel(&self, ptr: *const void, length: GLsizei, label: *const GLchar) {
    (self.glObjectPtrLabel_p)(ptr, length, label)
  }
  /// [glPatchParameterfv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPatchParameterfv.xhtml)
  /// * `pname` group: PatchParameterName
  /// * `values` len: COMPSIZE(pname)
  pub unsafe fn PatchParameterfv(&self, pname: PatchParameterName, values: *const GLfloat) {
    (self.glPatchParameterfv_p)(pname, values)
  }
  /// [glPatchParameteri](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPatchParameteri.xhtml)
  /// * `pname` group: PatchParameterName
  pub unsafe fn PatchParameteri(&self, pname: PatchParameterName, value: GLint) {
    (self.glPatchParameteri_p)(pname, value)
  }
  /// [glPauseTransformFeedback](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPauseTransformFeedback.xhtml)
  pub unsafe fn PauseTransformFeedback(&self) {
    (self.glPauseTransformFeedback_p)()
  }
  /// [glPixelStoref](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPixelStoref.xhtml)
  /// * `pname` group: PixelStoreParameter
  /// * `param` group: CheckedFloat32
  pub unsafe fn PixelStoref(&self, pname: PixelStoreParameter, param: GLfloat) {
    (self.glPixelStoref_p)(pname, param)
  }
  /// [glPixelStorei](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPixelStorei.xhtml)
  /// * `pname` group: PixelStoreParameter
  /// * `param` group: CheckedInt32
  pub unsafe fn PixelStorei(&self, pname: PixelStoreParameter, param: GLint) {
    (self.glPixelStorei_p)(pname, param)
  }
  /// [glPointParameterf](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPointParameterf.xhtml)
  /// * `pname` group: PointParameterNameARB
  /// * `param` group: CheckedFloat32
  pub unsafe fn PointParameterf(&self, pname: PointParameterNameARB, param: GLfloat) {
    (self.glPointParameterf_p)(pname, param)
  }
  /// [glPointParameterfv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPointParameterfv.xhtml)
  /// * `pname` group: PointParameterNameARB
  /// * `params` group: CheckedFloat32
  /// * `params` len: COMPSIZE(pname)
  pub unsafe fn PointParameterfv(&self, pname: PointParameterNameARB, params: *const GLfloat) {
    (self.glPointParameterfv_p)(pname, params)
  }
  /// [glPointParameteri](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPointParameteri.xhtml)
  /// * `pname` group: PointParameterNameARB
  pub unsafe fn PointParameteri(&self, pname: PointParameterNameARB, param: GLint) {
    (self.glPointParameteri_p)(pname, param)
  }
  /// [glPointParameteriv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPointParameteriv.xhtml)
  /// * `pname` group: PointParameterNameARB
  /// * `params` len: COMPSIZE(pname)
  pub unsafe fn PointParameteriv(&self, pname: PointParameterNameARB, params: *const GLint) {
    (self.glPointParameteriv_p)(pname, params)
  }
  /// [glPointSize](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPointSize.xhtml)
  /// * `size` group: CheckedFloat32
  pub unsafe fn PointSize(&self, size: GLfloat) {
    (self.glPointSize_p)(size)
  }
  /// [glPolygonMode](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPolygonMode.xhtml)
  /// * `face` group: MaterialFace
  /// * `mode` group: PolygonMode
  pub unsafe fn PolygonMode(&self, face: MaterialFace, mode: PolygonMode) {
    (self.glPolygonMode_p)(face, mode)
  }
  /// [glPolygonOffset](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPolygonOffset.xhtml)
  pub unsafe fn PolygonOffset(&self, factor: GLfloat, units: GLfloat) {
    (self.glPolygonOffset_p)(factor, units)
  }
  /// [glPolygonOffsetClamp](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPolygonOffsetClamp.xhtml)
  pub unsafe fn PolygonOffsetClamp(&self, factor: GLfloat, units: GLfloat, clamp: GLfloat) {
    (self.glPolygonOffsetClamp_p)(factor, units, clamp)
  }
  /// [glPopDebugGroup](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPopDebugGroup.xhtml)
  pub unsafe fn PopDebugGroup(&self) {
    (self.glPopDebugGroup_p)()
  }
  /// [glPrimitiveRestartIndex](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPrimitiveRestartIndex.xhtml)
  pub unsafe fn PrimitiveRestartIndex(&self, index: GLuint) {
    (self.glPrimitiveRestartIndex_p)(index)
  }
  /// [glProgramBinary](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramBinary.xhtml)
  /// * `program` class: program
  /// * `binary` len: length
  pub unsafe fn ProgramBinary(&self, program: GLuint, binaryFormat: GLenum, binary: *const void, length: GLsizei) {
    (self.glProgramBinary_p)(program, binaryFormat, binary, length)
  }
  /// [glProgramParameteri](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramParameteri.xhtml)
  /// * `program` class: program
  /// * `pname` group: ProgramParameterPName
  pub unsafe fn ProgramParameteri(&self, program: GLuint, pname: ProgramParameterPName, value: GLint) {
    (self.glProgramParameteri_p)(program, pname, value)
  }
  /// [glProgramUniform1d](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform1d.xhtml)
  /// * `program` class: program
  pub unsafe fn ProgramUniform1d(&self, program: GLuint, location: GLint, v0: GLdouble) {
    (self.glProgramUniform1d_p)(program, location, v0)
  }
  /// [glProgramUniform1dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform1dv.xhtml)
  /// * `program` class: program
  /// * `value` len: count
  pub unsafe fn ProgramUniform1dv(&self, program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble) {
    (self.glProgramUniform1dv_p)(program, location, count, value)
  }
  /// [glProgramUniform1f](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform1f.xhtml)
  /// * `program` class: program
  pub unsafe fn ProgramUniform1f(&self, program: GLuint, location: GLint, v0: GLfloat) {
    (self.glProgramUniform1f_p)(program, location, v0)
  }
  /// [glProgramUniform1fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform1fv.xhtml)
  /// * `program` class: program
  /// * `value` len: count
  pub unsafe fn ProgramUniform1fv(&self, program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat) {
    (self.glProgramUniform1fv_p)(program, location, count, value)
  }
  /// [glProgramUniform1i](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform1i.xhtml)
  /// * `program` class: program
  pub unsafe fn ProgramUniform1i(&self, program: GLuint, location: GLint, v0: GLint) {
    (self.glProgramUniform1i_p)(program, location, v0)
  }
  /// [glProgramUniform1iv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform1iv.xhtml)
  /// * `program` class: program
  /// * `value` len: count
  pub unsafe fn ProgramUniform1iv(&self, program: GLuint, location: GLint, count: GLsizei, value: *const GLint) {
    (self.glProgramUniform1iv_p)(program, location, count, value)
  }
  /// [glProgramUniform1ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform1ui.xhtml)
  /// * `program` class: program
  pub unsafe fn ProgramUniform1ui(&self, program: GLuint, location: GLint, v0: GLuint) {
    (self.glProgramUniform1ui_p)(program, location, v0)
  }
  /// [glProgramUniform1uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform1uiv.xhtml)
  /// * `program` class: program
  /// * `value` len: count
  pub unsafe fn ProgramUniform1uiv(&self, program: GLuint, location: GLint, count: GLsizei, value: *const GLuint) {
    (self.glProgramUniform1uiv_p)(program, location, count, value)
  }
  /// [glProgramUniform2d](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform2d.xhtml)
  /// * `program` class: program
  pub unsafe fn ProgramUniform2d(&self, program: GLuint, location: GLint, v0: GLdouble, v1: GLdouble) {
    (self.glProgramUniform2d_p)(program, location, v0, v1)
  }
  /// [glProgramUniform2dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform2dv.xhtml)
  /// * `program` class: program
  /// * `value` len: count*2
  pub unsafe fn ProgramUniform2dv(&self, program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble) {
    (self.glProgramUniform2dv_p)(program, location, count, value)
  }
  /// [glProgramUniform2f](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform2f.xhtml)
  /// * `program` class: program
  pub unsafe fn ProgramUniform2f(&self, program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat) {
    (self.glProgramUniform2f_p)(program, location, v0, v1)
  }
  /// [glProgramUniform2fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform2fv.xhtml)
  /// * `program` class: program
  /// * `value` len: count*2
  pub unsafe fn ProgramUniform2fv(&self, program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat) {
    (self.glProgramUniform2fv_p)(program, location, count, value)
  }
  /// [glProgramUniform2i](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform2i.xhtml)
  /// * `program` class: program
  pub unsafe fn ProgramUniform2i(&self, program: GLuint, location: GLint, v0: GLint, v1: GLint) {
    (self.glProgramUniform2i_p)(program, location, v0, v1)
  }
  /// [glProgramUniform2iv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform2iv.xhtml)
  /// * `program` class: program
  /// * `value` len: count*2
  pub unsafe fn ProgramUniform2iv(&self, program: GLuint, location: GLint, count: GLsizei, value: *const GLint) {
    (self.glProgramUniform2iv_p)(program, location, count, value)
  }
  /// [glProgramUniform2ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform2ui.xhtml)
  /// * `program` class: program
  pub unsafe fn ProgramUniform2ui(&self, program: GLuint, location: GLint, v0: GLuint, v1: GLuint) {
    (self.glProgramUniform2ui_p)(program, location, v0, v1)
  }
  /// [glProgramUniform2uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform2uiv.xhtml)
  /// * `program` class: program
  /// * `value` len: count*2
  pub unsafe fn ProgramUniform2uiv(&self, program: GLuint, location: GLint, count: GLsizei, value: *const GLuint) {
    (self.glProgramUniform2uiv_p)(program, location, count, value)
  }
  /// [glProgramUniform3d](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform3d.xhtml)
  /// * `program` class: program
  pub unsafe fn ProgramUniform3d(&self, program: GLuint, location: GLint, v0: GLdouble, v1: GLdouble, v2: GLdouble) {
    (self.glProgramUniform3d_p)(program, location, v0, v1, v2)
  }
  /// [glProgramUniform3dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform3dv.xhtml)
  /// * `program` class: program
  /// * `value` len: count*3
  pub unsafe fn ProgramUniform3dv(&self, program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble) {
    (self.glProgramUniform3dv_p)(program, location, count, value)
  }
  /// [glProgramUniform3f](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform3f.xhtml)
  /// * `program` class: program
  pub unsafe fn ProgramUniform3f(&self, program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat) {
    (self.glProgramUniform3f_p)(program, location, v0, v1, v2)
  }
  /// [glProgramUniform3fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform3fv.xhtml)
  /// * `program` class: program
  /// * `value` len: count*3
  pub unsafe fn ProgramUniform3fv(&self, program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat) {
    (self.glProgramUniform3fv_p)(program, location, count, value)
  }
  /// [glProgramUniform3i](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform3i.xhtml)
  /// * `program` class: program
  pub unsafe fn ProgramUniform3i(&self, program: GLuint, location: GLint, v0: GLint, v1: GLint, v2: GLint) {
    (self.glProgramUniform3i_p)(program, location, v0, v1, v2)
  }
  /// [glProgramUniform3iv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform3iv.xhtml)
  /// * `program` class: program
  /// * `value` len: count*3
  pub unsafe fn ProgramUniform3iv(&self, program: GLuint, location: GLint, count: GLsizei, value: *const GLint) {
    (self.glProgramUniform3iv_p)(program, location, count, value)
  }
  /// [glProgramUniform3ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform3ui.xhtml)
  /// * `program` class: program
  pub unsafe fn ProgramUniform3ui(&self, program: GLuint, location: GLint, v0: GLuint, v1: GLuint, v2: GLuint) {
    (self.glProgramUniform3ui_p)(program, location, v0, v1, v2)
  }
  /// [glProgramUniform3uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform3uiv.xhtml)
  /// * `program` class: program
  /// * `value` len: count*3
  pub unsafe fn ProgramUniform3uiv(&self, program: GLuint, location: GLint, count: GLsizei, value: *const GLuint) {
    (self.glProgramUniform3uiv_p)(program, location, count, value)
  }
  /// [glProgramUniform4d](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform4d.xhtml)
  /// * `program` class: program
  pub unsafe fn ProgramUniform4d(&self, program: GLuint, location: GLint, v0: GLdouble, v1: GLdouble, v2: GLdouble, v3: GLdouble) {
    (self.glProgramUniform4d_p)(program, location, v0, v1, v2, v3)
  }
  /// [glProgramUniform4dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform4dv.xhtml)
  /// * `program` class: program
  /// * `value` len: count*4
  pub unsafe fn ProgramUniform4dv(&self, program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble) {
    (self.glProgramUniform4dv_p)(program, location, count, value)
  }
  /// [glProgramUniform4f](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform4f.xhtml)
  /// * `program` class: program
  pub unsafe fn ProgramUniform4f(&self, program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat) {
    (self.glProgramUniform4f_p)(program, location, v0, v1, v2, v3)
  }
  /// [glProgramUniform4fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform4fv.xhtml)
  /// * `program` class: program
  /// * `value` len: count*4
  pub unsafe fn ProgramUniform4fv(&self, program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat) {
    (self.glProgramUniform4fv_p)(program, location, count, value)
  }
  /// [glProgramUniform4i](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform4i.xhtml)
  /// * `program` class: program
  pub unsafe fn ProgramUniform4i(&self, program: GLuint, location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint) {
    (self.glProgramUniform4i_p)(program, location, v0, v1, v2, v3)
  }
  /// [glProgramUniform4iv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform4iv.xhtml)
  /// * `program` class: program
  /// * `value` len: count*4
  pub unsafe fn ProgramUniform4iv(&self, program: GLuint, location: GLint, count: GLsizei, value: *const GLint) {
    (self.glProgramUniform4iv_p)(program, location, count, value)
  }
  /// [glProgramUniform4ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform4ui.xhtml)
  /// * `program` class: program
  pub unsafe fn ProgramUniform4ui(&self, program: GLuint, location: GLint, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint) {
    (self.glProgramUniform4ui_p)(program, location, v0, v1, v2, v3)
  }
  /// [glProgramUniform4uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniform4uiv.xhtml)
  /// * `program` class: program
  /// * `value` len: count*4
  pub unsafe fn ProgramUniform4uiv(&self, program: GLuint, location: GLint, count: GLsizei, value: *const GLuint) {
    (self.glProgramUniform4uiv_p)(program, location, count, value)
  }
  /// [glProgramUniformMatrix2dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniformMatrix2dv.xhtml)
  /// * `program` class: program
  /// * `transpose` group: Boolean
  /// * `value` len: count*4
  pub unsafe fn ProgramUniformMatrix2dv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) {
    (self.glProgramUniformMatrix2dv_p)(program, location, count, transpose, value)
  }
  /// [glProgramUniformMatrix2fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniformMatrix2fv.xhtml)
  /// * `program` class: program
  /// * `transpose` group: Boolean
  /// * `value` len: count*4
  pub unsafe fn ProgramUniformMatrix2fv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
    (self.glProgramUniformMatrix2fv_p)(program, location, count, transpose, value)
  }
  /// [glProgramUniformMatrix2x3dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniformMatrix2x3dv.xhtml)
  /// * `program` class: program
  /// * `transpose` group: Boolean
  /// * `value` len: count*6
  pub unsafe fn ProgramUniformMatrix2x3dv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) {
    (self.glProgramUniformMatrix2x3dv_p)(program, location, count, transpose, value)
  }
  /// [glProgramUniformMatrix2x3fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniformMatrix2x3fv.xhtml)
  /// * `program` class: program
  /// * `transpose` group: Boolean
  /// * `value` len: count*6
  pub unsafe fn ProgramUniformMatrix2x3fv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
    (self.glProgramUniformMatrix2x3fv_p)(program, location, count, transpose, value)
  }
  /// [glProgramUniformMatrix2x4dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniformMatrix2x4dv.xhtml)
  /// * `program` class: program
  /// * `transpose` group: Boolean
  /// * `value` len: count*8
  pub unsafe fn ProgramUniformMatrix2x4dv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) {
    (self.glProgramUniformMatrix2x4dv_p)(program, location, count, transpose, value)
  }
  /// [glProgramUniformMatrix2x4fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniformMatrix2x4fv.xhtml)
  /// * `program` class: program
  /// * `transpose` group: Boolean
  /// * `value` len: count*8
  pub unsafe fn ProgramUniformMatrix2x4fv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
    (self.glProgramUniformMatrix2x4fv_p)(program, location, count, transpose, value)
  }
  /// [glProgramUniformMatrix3dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniformMatrix3dv.xhtml)
  /// * `program` class: program
  /// * `transpose` group: Boolean
  /// * `value` len: count*9
  pub unsafe fn ProgramUniformMatrix3dv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) {
    (self.glProgramUniformMatrix3dv_p)(program, location, count, transpose, value)
  }
  /// [glProgramUniformMatrix3fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniformMatrix3fv.xhtml)
  /// * `program` class: program
  /// * `transpose` group: Boolean
  /// * `value` len: count*9
  pub unsafe fn ProgramUniformMatrix3fv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
    (self.glProgramUniformMatrix3fv_p)(program, location, count, transpose, value)
  }
  /// [glProgramUniformMatrix3x2dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniformMatrix3x2dv.xhtml)
  /// * `program` class: program
  /// * `transpose` group: Boolean
  /// * `value` len: count*6
  pub unsafe fn ProgramUniformMatrix3x2dv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) {
    (self.glProgramUniformMatrix3x2dv_p)(program, location, count, transpose, value)
  }
  /// [glProgramUniformMatrix3x2fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniformMatrix3x2fv.xhtml)
  /// * `program` class: program
  /// * `transpose` group: Boolean
  /// * `value` len: count*6
  pub unsafe fn ProgramUniformMatrix3x2fv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
    (self.glProgramUniformMatrix3x2fv_p)(program, location, count, transpose, value)
  }
  /// [glProgramUniformMatrix3x4dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniformMatrix3x4dv.xhtml)
  /// * `program` class: program
  /// * `transpose` group: Boolean
  /// * `value` len: count*12
  pub unsafe fn ProgramUniformMatrix3x4dv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) {
    (self.glProgramUniformMatrix3x4dv_p)(program, location, count, transpose, value)
  }
  /// [glProgramUniformMatrix3x4fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniformMatrix3x4fv.xhtml)
  /// * `program` class: program
  /// * `transpose` group: Boolean
  /// * `value` len: count*12
  pub unsafe fn ProgramUniformMatrix3x4fv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
    (self.glProgramUniformMatrix3x4fv_p)(program, location, count, transpose, value)
  }
  /// [glProgramUniformMatrix4dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniformMatrix4dv.xhtml)
  /// * `program` class: program
  /// * `transpose` group: Boolean
  /// * `value` len: count*16
  pub unsafe fn ProgramUniformMatrix4dv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) {
    (self.glProgramUniformMatrix4dv_p)(program, location, count, transpose, value)
  }
  /// [glProgramUniformMatrix4fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniformMatrix4fv.xhtml)
  /// * `program` class: program
  /// * `transpose` group: Boolean
  /// * `value` len: count*16
  pub unsafe fn ProgramUniformMatrix4fv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
    (self.glProgramUniformMatrix4fv_p)(program, location, count, transpose, value)
  }
  /// [glProgramUniformMatrix4x2dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniformMatrix4x2dv.xhtml)
  /// * `program` class: program
  /// * `transpose` group: Boolean
  /// * `value` len: count*8
  pub unsafe fn ProgramUniformMatrix4x2dv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) {
    (self.glProgramUniformMatrix4x2dv_p)(program, location, count, transpose, value)
  }
  /// [glProgramUniformMatrix4x2fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniformMatrix4x2fv.xhtml)
  /// * `program` class: program
  /// * `transpose` group: Boolean
  /// * `value` len: count*8
  pub unsafe fn ProgramUniformMatrix4x2fv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
    (self.glProgramUniformMatrix4x2fv_p)(program, location, count, transpose, value)
  }
  /// [glProgramUniformMatrix4x3dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniformMatrix4x3dv.xhtml)
  /// * `program` class: program
  /// * `transpose` group: Boolean
  /// * `value` len: count*12
  pub unsafe fn ProgramUniformMatrix4x3dv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) {
    (self.glProgramUniformMatrix4x3dv_p)(program, location, count, transpose, value)
  }
  /// [glProgramUniformMatrix4x3fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProgramUniformMatrix4x3fv.xhtml)
  /// * `program` class: program
  /// * `transpose` group: Boolean
  /// * `value` len: count*12
  pub unsafe fn ProgramUniformMatrix4x3fv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
    (self.glProgramUniformMatrix4x3fv_p)(program, location, count, transpose, value)
  }
  /// [glProvokingVertex](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glProvokingVertex.xhtml)
  /// * `mode` group: VertexProvokingMode
  pub unsafe fn ProvokingVertex(&self, mode: VertexProvokingMode) {
    (self.glProvokingVertex_p)(mode)
  }
  /// [glPushDebugGroup](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPushDebugGroup.xhtml)
  /// * `source` group: DebugSource
  /// * `message` len: COMPSIZE(message,length)
  pub unsafe fn PushDebugGroup(&self, source: DebugSource, id: GLuint, length: GLsizei, message: *const GLchar) {
    (self.glPushDebugGroup_p)(source, id, length, message)
  }
  /// [glQueryCounter](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glQueryCounter.xhtml)
  /// * `id` class: query
  /// * `target` group: QueryCounterTarget
  pub unsafe fn QueryCounter(&self, id: GLuint, target: QueryCounterTarget) {
    (self.glQueryCounter_p)(id, target)
  }
  /// [glReadBuffer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glReadBuffer.xhtml)
  /// * `src` group: ReadBufferMode
  pub unsafe fn ReadBuffer(&self, src: ReadBufferMode) {
    (self.glReadBuffer_p)(src)
  }
  /// [glReadPixels](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glReadPixels.xhtml)
  /// * `x` group: WinCoord
  /// * `y` group: WinCoord
  /// * `format` group: PixelFormat
  /// * `type` group: PixelType
  /// * `pixels` len: COMPSIZE(format,type,width,height)
  pub unsafe fn ReadPixels(&self, x: GLint, y: GLint, width: GLsizei, height: GLsizei, format: PixelFormat, type_: PixelType, pixels: *mut void) {
    (self.glReadPixels_p)(x, y, width, height, format, type_, pixels)
  }
  /// [glReadnPixels](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glReadnPixels.xhtml)
  /// * `format` group: PixelFormat
  /// * `type` group: PixelType
  /// * `data` len: bufSize
  pub unsafe fn ReadnPixels(&self, x: GLint, y: GLint, width: GLsizei, height: GLsizei, format: PixelFormat, type_: PixelType, bufSize: GLsizei, data: *mut void) {
    (self.glReadnPixels_p)(x, y, width, height, format, type_, bufSize, data)
  }
  /// [glReleaseShaderCompiler](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glReleaseShaderCompiler.xhtml)
  pub unsafe fn ReleaseShaderCompiler(&self) {
    (self.glReleaseShaderCompiler_p)()
  }
  /// [glRenderbufferStorage](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glRenderbufferStorage.xhtml)
  /// * `target` group: RenderbufferTarget
  /// * `internalformat` group: InternalFormat
  pub unsafe fn RenderbufferStorage(&self, target: RenderbufferTarget, internalformat: InternalFormat, width: GLsizei, height: GLsizei) {
    (self.glRenderbufferStorage_p)(target, internalformat, width, height)
  }
  /// [glRenderbufferStorageMultisample](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glRenderbufferStorageMultisample.xhtml)
  /// * `target` group: RenderbufferTarget
  /// * `internalformat` group: InternalFormat
  pub unsafe fn RenderbufferStorageMultisample(&self, target: RenderbufferTarget, samples: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei) {
    (self.glRenderbufferStorageMultisample_p)(target, samples, internalformat, width, height)
  }
  /// [glResumeTransformFeedback](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glResumeTransformFeedback.xhtml)
  pub unsafe fn ResumeTransformFeedback(&self) {
    (self.glResumeTransformFeedback_p)()
  }
  /// [glSampleCoverage](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSampleCoverage.xhtml)
  /// * `invert` group: Boolean
  pub unsafe fn SampleCoverage(&self, value: GLfloat, invert: GLboolean) {
    (self.glSampleCoverage_p)(value, invert)
  }
  /// [glSampleMaski](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSampleMaski.xhtml)
  pub unsafe fn SampleMaski(&self, maskNumber: GLuint, mask: GLbitfield) {
    (self.glSampleMaski_p)(maskNumber, mask)
  }
  /// [glSamplerParameterIiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSamplerParameterIiv.xhtml)
  /// * `sampler` class: sampler
  /// * `pname` group: SamplerParameterI
  /// * `param` len: COMPSIZE(pname)
  pub unsafe fn SamplerParameterIiv(&self, sampler: GLuint, pname: SamplerParameterI, param: *const GLint) {
    (self.glSamplerParameterIiv_p)(sampler, pname, param)
  }
  /// [glSamplerParameterIuiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSamplerParameterIuiv.xhtml)
  /// * `sampler` class: sampler
  /// * `pname` group: SamplerParameterI
  /// * `param` len: COMPSIZE(pname)
  pub unsafe fn SamplerParameterIuiv(&self, sampler: GLuint, pname: SamplerParameterI, param: *const GLuint) {
    (self.glSamplerParameterIuiv_p)(sampler, pname, param)
  }
  /// [glSamplerParameterf](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSamplerParameterf.xhtml)
  /// * `sampler` class: sampler
  /// * `pname` group: SamplerParameterF
  pub unsafe fn SamplerParameterf(&self, sampler: GLuint, pname: SamplerParameterF, param: GLfloat) {
    (self.glSamplerParameterf_p)(sampler, pname, param)
  }
  /// [glSamplerParameterfv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSamplerParameterfv.xhtml)
  /// * `sampler` class: sampler
  /// * `pname` group: SamplerParameterF
  /// * `param` len: COMPSIZE(pname)
  pub unsafe fn SamplerParameterfv(&self, sampler: GLuint, pname: SamplerParameterF, param: *const GLfloat) {
    (self.glSamplerParameterfv_p)(sampler, pname, param)
  }
  /// [glSamplerParameteri](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSamplerParameteri.xhtml)
  /// * `sampler` class: sampler
  /// * `pname` group: SamplerParameterI
  pub unsafe fn SamplerParameteri(&self, sampler: GLuint, pname: SamplerParameterI, param: GLint) {
    (self.glSamplerParameteri_p)(sampler, pname, param)
  }
  /// [glSamplerParameteriv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSamplerParameteriv.xhtml)
  /// * `sampler` class: sampler
  /// * `pname` group: SamplerParameterI
  /// * `param` len: COMPSIZE(pname)
  pub unsafe fn SamplerParameteriv(&self, sampler: GLuint, pname: SamplerParameterI, param: *const GLint) {
    (self.glSamplerParameteriv_p)(sampler, pname, param)
  }
  /// [glScissor](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glScissor.xhtml)
  /// * `x` group: WinCoord
  /// * `y` group: WinCoord
  pub unsafe fn Scissor(&self, x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
    (self.glScissor_p)(x, y, width, height)
  }
  /// [glScissorArrayv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glScissorArrayv.xhtml)
  /// * `v` len: COMPSIZE(count)
  pub unsafe fn ScissorArrayv(&self, first: GLuint, count: GLsizei, v: *const GLint) {
    (self.glScissorArrayv_p)(first, count, v)
  }
  /// [glScissorIndexed](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glScissorIndexed.xhtml)
  pub unsafe fn ScissorIndexed(&self, index: GLuint, left: GLint, bottom: GLint, width: GLsizei, height: GLsizei) {
    (self.glScissorIndexed_p)(index, left, bottom, width, height)
  }
  /// [glScissorIndexedv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glScissorIndexedv.xhtml)
  pub unsafe fn ScissorIndexedv(&self, index: GLuint, v: *const [GLint; 4]) {
    (self.glScissorIndexedv_p)(index, v)
  }
  /// [glSecondaryColorP3ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSecondaryColorP3ui.xhtml)
  /// * `type` group: ColorPointerType
  pub unsafe fn SecondaryColorP3ui(&self, type_: ColorPointerType, color: GLuint) {
    (self.glSecondaryColorP3ui_p)(type_, color)
  }
  /// [glSecondaryColorP3uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSecondaryColorP3uiv.xhtml)
  /// * `type` group: ColorPointerType
  pub unsafe fn SecondaryColorP3uiv(&self, type_: ColorPointerType, color: *const GLuint) {
    (self.glSecondaryColorP3uiv_p)(type_, color)
  }
  /// [glShaderBinary](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glShaderBinary.xhtml)
  /// * `shaders` class: shader
  /// * `shaders` len: count
  /// * `binaryFormat` group: ShaderBinaryFormat
  /// * `binary` len: length
  pub unsafe fn ShaderBinary(&self, count: GLsizei, shaders: *const GLuint, binaryFormat: ShaderBinaryFormat, binary: *const void, length: GLsizei) {
    (self.glShaderBinary_p)(count, shaders, binaryFormat, binary, length)
  }
  /// [glShaderSource](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glShaderSource.xhtml)
  /// * `shader` class: shader
  /// * `string` len: count
  /// * `length` len: count
  pub unsafe fn ShaderSource(&self, shader: GLuint, count: GLsizei, string: *const *const GLchar, length: *const GLint) {
    (self.glShaderSource_p)(shader, count, string, length)
  }
  /// [glShaderStorageBlockBinding](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glShaderStorageBlockBinding.xhtml)
  /// * `program` class: program
  pub unsafe fn ShaderStorageBlockBinding(&self, program: GLuint, storageBlockIndex: GLuint, storageBlockBinding: GLuint) {
    (self.glShaderStorageBlockBinding_p)(program, storageBlockIndex, storageBlockBinding)
  }
  /// [glSpecializeShader](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glSpecializeShader.xhtml)
  /// * `shader` class: shader
  pub unsafe fn SpecializeShader(&self, shader: GLuint, pEntryPoint: *const GLchar, numSpecializationConstants: GLuint, pConstantIndex: *const GLuint, pConstantValue: *const GLuint) {
    (self.glSpecializeShader_p)(shader, pEntryPoint, numSpecializationConstants, pConstantIndex, pConstantValue)
  }
  /// [glStencilFunc](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glStencilFunc.xhtml)
  /// * `func` group: StencilFunction
  /// * `ref` group: StencilValue
  /// * `mask` group: MaskedStencilValue
  pub unsafe fn StencilFunc(&self, func: StencilFunction, ref_: GLint, mask: GLuint) {
    (self.glStencilFunc_p)(func, ref_, mask)
  }
  /// [glStencilFuncSeparate](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glStencilFuncSeparate.xhtml)
  /// * `face` group: StencilFaceDirection
  /// * `func` group: StencilFunction
  /// * `ref` group: StencilValue
  /// * `mask` group: MaskedStencilValue
  pub unsafe fn StencilFuncSeparate(&self, face: StencilFaceDirection, func: StencilFunction, ref_: GLint, mask: GLuint) {
    (self.glStencilFuncSeparate_p)(face, func, ref_, mask)
  }
  /// [glStencilMask](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glStencilMask.xhtml)
  /// * `mask` group: MaskedStencilValue
  pub unsafe fn StencilMask(&self, mask: GLuint) {
    (self.glStencilMask_p)(mask)
  }
  /// [glStencilMaskSeparate](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glStencilMaskSeparate.xhtml)
  /// * `face` group: StencilFaceDirection
  /// * `mask` group: MaskedStencilValue
  pub unsafe fn StencilMaskSeparate(&self, face: StencilFaceDirection, mask: GLuint) {
    (self.glStencilMaskSeparate_p)(face, mask)
  }
  /// [glStencilOp](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glStencilOp.xhtml)
  /// * `fail` group: StencilOp
  /// * `zfail` group: StencilOp
  /// * `zpass` group: StencilOp
  pub unsafe fn StencilOp(&self, fail: StencilOp, zfail: StencilOp, zpass: StencilOp) {
    (self.glStencilOp_p)(fail, zfail, zpass)
  }
  /// [glStencilOpSeparate](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glStencilOpSeparate.xhtml)
  /// * `face` group: StencilFaceDirection
  /// * `sfail` group: StencilOp
  /// * `dpfail` group: StencilOp
  /// * `dppass` group: StencilOp
  pub unsafe fn StencilOpSeparate(&self, face: StencilFaceDirection, sfail: StencilOp, dpfail: StencilOp, dppass: StencilOp) {
    (self.glStencilOpSeparate_p)(face, sfail, dpfail, dppass)
  }
  /// [glTexBuffer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexBuffer.xhtml)
  /// * `target` group: TextureTarget
  /// * `internalformat` group: InternalFormat
  /// * `buffer` class: buffer
  pub unsafe fn TexBuffer(&self, target: TextureTarget, internalformat: InternalFormat, buffer: GLuint) {
    (self.glTexBuffer_p)(target, internalformat, buffer)
  }
  /// [glTexBufferRange](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexBufferRange.xhtml)
  /// * `target` group: TextureTarget
  /// * `internalformat` group: InternalFormat
  /// * `buffer` class: buffer
  /// * `offset` group: BufferOffset
  /// * `size` group: BufferSize
  pub unsafe fn TexBufferRange(&self, target: TextureTarget, internalformat: InternalFormat, buffer: GLuint, offset: GLintptr, size: GLsizeiptr) {
    (self.glTexBufferRange_p)(target, internalformat, buffer, offset, size)
  }
  /// [glTexCoordP1ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoordP1ui.xhtml)
  /// * `type` group: TexCoordPointerType
  pub unsafe fn TexCoordP1ui(&self, type_: TexCoordPointerType, coords: GLuint) {
    (self.glTexCoordP1ui_p)(type_, coords)
  }
  /// [glTexCoordP1uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoordP1uiv.xhtml)
  /// * `type` group: TexCoordPointerType
  pub unsafe fn TexCoordP1uiv(&self, type_: TexCoordPointerType, coords: *const GLuint) {
    (self.glTexCoordP1uiv_p)(type_, coords)
  }
  /// [glTexCoordP2ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoordP2ui.xhtml)
  /// * `type` group: TexCoordPointerType
  pub unsafe fn TexCoordP2ui(&self, type_: TexCoordPointerType, coords: GLuint) {
    (self.glTexCoordP2ui_p)(type_, coords)
  }
  /// [glTexCoordP2uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoordP2uiv.xhtml)
  /// * `type` group: TexCoordPointerType
  pub unsafe fn TexCoordP2uiv(&self, type_: TexCoordPointerType, coords: *const GLuint) {
    (self.glTexCoordP2uiv_p)(type_, coords)
  }
  /// [glTexCoordP3ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoordP3ui.xhtml)
  /// * `type` group: TexCoordPointerType
  pub unsafe fn TexCoordP3ui(&self, type_: TexCoordPointerType, coords: GLuint) {
    (self.glTexCoordP3ui_p)(type_, coords)
  }
  /// [glTexCoordP3uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoordP3uiv.xhtml)
  /// * `type` group: TexCoordPointerType
  pub unsafe fn TexCoordP3uiv(&self, type_: TexCoordPointerType, coords: *const GLuint) {
    (self.glTexCoordP3uiv_p)(type_, coords)
  }
  /// [glTexCoordP4ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoordP4ui.xhtml)
  /// * `type` group: TexCoordPointerType
  pub unsafe fn TexCoordP4ui(&self, type_: TexCoordPointerType, coords: GLuint) {
    (self.glTexCoordP4ui_p)(type_, coords)
  }
  /// [glTexCoordP4uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexCoordP4uiv.xhtml)
  /// * `type` group: TexCoordPointerType
  pub unsafe fn TexCoordP4uiv(&self, type_: TexCoordPointerType, coords: *const GLuint) {
    (self.glTexCoordP4uiv_p)(type_, coords)
  }
  /// [glTexImage1D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexImage1D.xhtml)
  /// * `target` group: TextureTarget
  /// * `level` group: CheckedInt32
  /// * `internalformat` group: InternalFormat
  /// * `border` group: CheckedInt32
  /// * `format` group: PixelFormat
  /// * `type` group: PixelType
  /// * `pixels` len: COMPSIZE(format,type,width)
  pub unsafe fn TexImage1D(&self, target: TextureTarget, level: GLint, internalformat: GLint, width: GLsizei, border: GLint, format: PixelFormat, type_: PixelType, pixels: *const void) {
    (self.glTexImage1D_p)(target, level, internalformat, width, border, format, type_, pixels)
  }
  /// [glTexImage2D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexImage2D.xhtml)
  /// * `target` group: TextureTarget
  /// * `level` group: CheckedInt32
  /// * `internalformat` group: InternalFormat
  /// * `border` group: CheckedInt32
  /// * `format` group: PixelFormat
  /// * `type` group: PixelType
  /// * `pixels` len: COMPSIZE(format,type,width,height)
  pub unsafe fn TexImage2D(&self, target: TextureTarget, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, border: GLint, format: PixelFormat, type_: PixelType, pixels: *const void) {
    (self.glTexImage2D_p)(target, level, internalformat, width, height, border, format, type_, pixels)
  }
  /// [glTexImage2DMultisample](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexImage2DMultisample.xhtml)
  /// * `target` group: TextureTarget
  /// * `internalformat` group: InternalFormat
  /// * `fixedsamplelocations` group: Boolean
  pub unsafe fn TexImage2DMultisample(&self, target: TextureTarget, samples: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei, fixedsamplelocations: GLboolean) {
    (self.glTexImage2DMultisample_p)(target, samples, internalformat, width, height, fixedsamplelocations)
  }
  /// [glTexImage3D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexImage3D.xhtml)
  /// * `target` group: TextureTarget
  /// * `level` group: CheckedInt32
  /// * `internalformat` group: InternalFormat
  /// * `border` group: CheckedInt32
  /// * `format` group: PixelFormat
  /// * `type` group: PixelType
  /// * `pixels` len: COMPSIZE(format,type,width,height,depth)
  pub unsafe fn TexImage3D(&self, target: TextureTarget, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, format: PixelFormat, type_: PixelType, pixels: *const void) {
    (self.glTexImage3D_p)(target, level, internalformat, width, height, depth, border, format, type_, pixels)
  }
  /// [glTexImage3DMultisample](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexImage3DMultisample.xhtml)
  /// * `target` group: TextureTarget
  /// * `internalformat` group: InternalFormat
  /// * `fixedsamplelocations` group: Boolean
  pub unsafe fn TexImage3DMultisample(&self, target: TextureTarget, samples: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean) {
    (self.glTexImage3DMultisample_p)(target, samples, internalformat, width, height, depth, fixedsamplelocations)
  }
  /// [glTexParameterIiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexParameterIiv.xhtml)
  /// * `target` group: TextureTarget
  /// * `pname` group: TextureParameterName
  /// * `params` len: COMPSIZE(pname)
  pub unsafe fn TexParameterIiv(&self, target: TextureTarget, pname: TextureParameterName, params: *const GLint) {
    (self.glTexParameterIiv_p)(target, pname, params)
  }
  /// [glTexParameterIuiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexParameterIuiv.xhtml)
  /// * `target` group: TextureTarget
  /// * `pname` group: TextureParameterName
  /// * `params` len: COMPSIZE(pname)
  pub unsafe fn TexParameterIuiv(&self, target: TextureTarget, pname: TextureParameterName, params: *const GLuint) {
    (self.glTexParameterIuiv_p)(target, pname, params)
  }
  /// [glTexParameterf](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexParameterf.xhtml)
  /// * `target` group: TextureTarget
  /// * `pname` group: TextureParameterName
  /// * `param` group: CheckedFloat32
  pub unsafe fn TexParameterf(&self, target: TextureTarget, pname: TextureParameterName, param: GLfloat) {
    (self.glTexParameterf_p)(target, pname, param)
  }
  /// [glTexParameterfv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexParameterfv.xhtml)
  /// * `target` group: TextureTarget
  /// * `pname` group: TextureParameterName
  /// * `params` group: CheckedFloat32
  /// * `params` len: COMPSIZE(pname)
  pub unsafe fn TexParameterfv(&self, target: TextureTarget, pname: TextureParameterName, params: *const GLfloat) {
    (self.glTexParameterfv_p)(target, pname, params)
  }
  /// [glTexParameteri](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexParameteri.xhtml)
  /// * `target` group: TextureTarget
  /// * `pname` group: TextureParameterName
  /// * `param` group: CheckedInt32
  pub unsafe fn TexParameteri(&self, target: TextureTarget, pname: TextureParameterName, param: GLint) {
    (self.glTexParameteri_p)(target, pname, param)
  }
  /// [glTexParameteriv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexParameteriv.xhtml)
  /// * `target` group: TextureTarget
  /// * `pname` group: TextureParameterName
  /// * `params` group: CheckedInt32
  /// * `params` len: COMPSIZE(pname)
  pub unsafe fn TexParameteriv(&self, target: TextureTarget, pname: TextureParameterName, params: *const GLint) {
    (self.glTexParameteriv_p)(target, pname, params)
  }
  /// [glTexStorage1D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexStorage1D.xhtml)
  /// * `target` group: TextureTarget
  /// * `internalformat` group: InternalFormat
  pub unsafe fn TexStorage1D(&self, target: TextureTarget, levels: GLsizei, internalformat: InternalFormat, width: GLsizei) {
    (self.glTexStorage1D_p)(target, levels, internalformat, width)
  }
  /// [glTexStorage2D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexStorage2D.xhtml)
  /// * `target` group: TextureTarget
  /// * `internalformat` group: InternalFormat
  pub unsafe fn TexStorage2D(&self, target: TextureTarget, levels: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei) {
    (self.glTexStorage2D_p)(target, levels, internalformat, width, height)
  }
  /// [glTexStorage2DMultisample](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexStorage2DMultisample.xhtml)
  /// * `target` group: TextureTarget
  /// * `internalformat` group: InternalFormat
  /// * `fixedsamplelocations` group: Boolean
  pub unsafe fn TexStorage2DMultisample(&self, target: TextureTarget, samples: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei, fixedsamplelocations: GLboolean) {
    (self.glTexStorage2DMultisample_p)(target, samples, internalformat, width, height, fixedsamplelocations)
  }
  /// [glTexStorage3D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexStorage3D.xhtml)
  /// * `target` group: TextureTarget
  /// * `internalformat` group: InternalFormat
  pub unsafe fn TexStorage3D(&self, target: TextureTarget, levels: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei, depth: GLsizei) {
    (self.glTexStorage3D_p)(target, levels, internalformat, width, height, depth)
  }
  /// [glTexStorage3DMultisample](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexStorage3DMultisample.xhtml)
  /// * `target` group: TextureTarget
  /// * `internalformat` group: InternalFormat
  /// * `fixedsamplelocations` group: Boolean
  pub unsafe fn TexStorage3DMultisample(&self, target: TextureTarget, samples: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean) {
    (self.glTexStorage3DMultisample_p)(target, samples, internalformat, width, height, depth, fixedsamplelocations)
  }
  /// [glTexSubImage1D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexSubImage1D.xhtml)
  /// * `target` group: TextureTarget
  /// * `level` group: CheckedInt32
  /// * `xoffset` group: CheckedInt32
  /// * `format` group: PixelFormat
  /// * `type` group: PixelType
  /// * `pixels` len: COMPSIZE(format,type,width)
  pub unsafe fn TexSubImage1D(&self, target: TextureTarget, level: GLint, xoffset: GLint, width: GLsizei, format: PixelFormat, type_: PixelType, pixels: *const void) {
    (self.glTexSubImage1D_p)(target, level, xoffset, width, format, type_, pixels)
  }
  /// [glTexSubImage2D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexSubImage2D.xhtml)
  /// * `target` group: TextureTarget
  /// * `level` group: CheckedInt32
  /// * `xoffset` group: CheckedInt32
  /// * `yoffset` group: CheckedInt32
  /// * `format` group: PixelFormat
  /// * `type` group: PixelType
  /// * `pixels` len: COMPSIZE(format,type,width,height)
  pub unsafe fn TexSubImage2D(&self, target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: PixelFormat, type_: PixelType, pixels: *const void) {
    (self.glTexSubImage2D_p)(target, level, xoffset, yoffset, width, height, format, type_, pixels)
  }
  /// [glTexSubImage3D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexSubImage3D.xhtml)
  /// * `target` group: TextureTarget
  /// * `level` group: CheckedInt32
  /// * `xoffset` group: CheckedInt32
  /// * `yoffset` group: CheckedInt32
  /// * `zoffset` group: CheckedInt32
  /// * `format` group: PixelFormat
  /// * `type` group: PixelType
  /// * `pixels` len: COMPSIZE(format,type,width,height,depth)
  pub unsafe fn TexSubImage3D(&self, target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: PixelFormat, type_: PixelType, pixels: *const void) {
    (self.glTexSubImage3D_p)(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type_, pixels)
  }
  /// [glTextureBarrier](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureBarrier.xhtml)
  pub unsafe fn TextureBarrier(&self) {
    (self.glTextureBarrier_p)()
  }
  /// [glTextureBuffer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureBuffer.xhtml)
  /// * `texture` class: texture
  /// * `internalformat` group: InternalFormat
  /// * `buffer` class: buffer
  pub unsafe fn TextureBuffer(&self, texture: GLuint, internalformat: InternalFormat, buffer: GLuint) {
    (self.glTextureBuffer_p)(texture, internalformat, buffer)
  }
  /// [glTextureBufferRange](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureBufferRange.xhtml)
  /// * `texture` class: texture
  /// * `internalformat` group: InternalFormat
  /// * `buffer` class: buffer
  /// * `size` group: BufferSize
  pub unsafe fn TextureBufferRange(&self, texture: GLuint, internalformat: InternalFormat, buffer: GLuint, offset: GLintptr, size: GLsizeiptr) {
    (self.glTextureBufferRange_p)(texture, internalformat, buffer, offset, size)
  }
  /// [glTextureParameterIiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureParameterIiv.xhtml)
  /// * `texture` class: texture
  /// * `pname` group: TextureParameterName
  pub unsafe fn TextureParameterIiv(&self, texture: GLuint, pname: TextureParameterName, params: *const GLint) {
    (self.glTextureParameterIiv_p)(texture, pname, params)
  }
  /// [glTextureParameterIuiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureParameterIuiv.xhtml)
  /// * `texture` class: texture
  /// * `pname` group: TextureParameterName
  pub unsafe fn TextureParameterIuiv(&self, texture: GLuint, pname: TextureParameterName, params: *const GLuint) {
    (self.glTextureParameterIuiv_p)(texture, pname, params)
  }
  /// [glTextureParameterf](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureParameterf.xhtml)
  /// * `texture` class: texture
  /// * `pname` group: TextureParameterName
  pub unsafe fn TextureParameterf(&self, texture: GLuint, pname: TextureParameterName, param: GLfloat) {
    (self.glTextureParameterf_p)(texture, pname, param)
  }
  /// [glTextureParameterfv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureParameterfv.xhtml)
  /// * `texture` class: texture
  /// * `pname` group: TextureParameterName
  pub unsafe fn TextureParameterfv(&self, texture: GLuint, pname: TextureParameterName, param: *const GLfloat) {
    (self.glTextureParameterfv_p)(texture, pname, param)
  }
  /// [glTextureParameteri](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureParameteri.xhtml)
  /// * `texture` class: texture
  /// * `pname` group: TextureParameterName
  pub unsafe fn TextureParameteri(&self, texture: GLuint, pname: TextureParameterName, param: GLint) {
    (self.glTextureParameteri_p)(texture, pname, param)
  }
  /// [glTextureParameteriv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureParameteriv.xhtml)
  /// * `texture` class: texture
  /// * `pname` group: TextureParameterName
  pub unsafe fn TextureParameteriv(&self, texture: GLuint, pname: TextureParameterName, param: *const GLint) {
    (self.glTextureParameteriv_p)(texture, pname, param)
  }
  /// [glTextureStorage1D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureStorage1D.xhtml)
  /// * `texture` class: texture
  /// * `internalformat` group: InternalFormat
  pub unsafe fn TextureStorage1D(&self, texture: GLuint, levels: GLsizei, internalformat: InternalFormat, width: GLsizei) {
    (self.glTextureStorage1D_p)(texture, levels, internalformat, width)
  }
  /// [glTextureStorage2D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureStorage2D.xhtml)
  /// * `texture` class: texture
  /// * `internalformat` group: InternalFormat
  pub unsafe fn TextureStorage2D(&self, texture: GLuint, levels: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei) {
    (self.glTextureStorage2D_p)(texture, levels, internalformat, width, height)
  }
  /// [glTextureStorage2DMultisample](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureStorage2DMultisample.xhtml)
  /// * `texture` class: texture
  /// * `internalformat` group: InternalFormat
  /// * `fixedsamplelocations` group: Boolean
  pub unsafe fn TextureStorage2DMultisample(&self, texture: GLuint, samples: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei, fixedsamplelocations: GLboolean) {
    (self.glTextureStorage2DMultisample_p)(texture, samples, internalformat, width, height, fixedsamplelocations)
  }
  /// [glTextureStorage3D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureStorage3D.xhtml)
  /// * `texture` class: texture
  /// * `internalformat` group: InternalFormat
  pub unsafe fn TextureStorage3D(&self, texture: GLuint, levels: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei, depth: GLsizei) {
    (self.glTextureStorage3D_p)(texture, levels, internalformat, width, height, depth)
  }
  /// [glTextureStorage3DMultisample](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureStorage3DMultisample.xhtml)
  /// * `texture` class: texture
  /// * `internalformat` group: InternalFormat
  /// * `fixedsamplelocations` group: Boolean
  pub unsafe fn TextureStorage3DMultisample(&self, texture: GLuint, samples: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean) {
    (self.glTextureStorage3DMultisample_p)(texture, samples, internalformat, width, height, depth, fixedsamplelocations)
  }
  /// [glTextureSubImage1D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureSubImage1D.xhtml)
  /// * `texture` class: texture
  /// * `format` group: PixelFormat
  /// * `type` group: PixelType
  pub unsafe fn TextureSubImage1D(&self, texture: GLuint, level: GLint, xoffset: GLint, width: GLsizei, format: PixelFormat, type_: PixelType, pixels: *const void) {
    (self.glTextureSubImage1D_p)(texture, level, xoffset, width, format, type_, pixels)
  }
  /// [glTextureSubImage2D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureSubImage2D.xhtml)
  /// * `texture` class: texture
  /// * `format` group: PixelFormat
  /// * `type` group: PixelType
  pub unsafe fn TextureSubImage2D(&self, texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: PixelFormat, type_: PixelType, pixels: *const void) {
    (self.glTextureSubImage2D_p)(texture, level, xoffset, yoffset, width, height, format, type_, pixels)
  }
  /// [glTextureSubImage3D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureSubImage3D.xhtml)
  /// * `texture` class: texture
  /// * `format` group: PixelFormat
  /// * `type` group: PixelType
  pub unsafe fn TextureSubImage3D(&self, texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: PixelFormat, type_: PixelType, pixels: *const void) {
    (self.glTextureSubImage3D_p)(texture, level, xoffset, yoffset, zoffset, width, height, depth, format, type_, pixels)
  }
  /// [glTextureView](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTextureView.xhtml)
  /// * `texture` class: texture
  /// * `target` group: TextureTarget
  /// * `origtexture` class: texture
  /// * `internalformat` group: InternalFormat
  pub unsafe fn TextureView(&self, texture: GLuint, target: TextureTarget, origtexture: GLuint, internalformat: InternalFormat, minlevel: GLuint, numlevels: GLuint, minlayer: GLuint, numlayers: GLuint) {
    (self.glTextureView_p)(texture, target, origtexture, internalformat, minlevel, numlevels, minlayer, numlayers)
  }
  /// [glTransformFeedbackBufferBase](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTransformFeedbackBufferBase.xhtml)
  /// * `xfb` class: transform feedback
  /// * `buffer` class: buffer
  pub unsafe fn TransformFeedbackBufferBase(&self, xfb: GLuint, index: GLuint, buffer: GLuint) {
    (self.glTransformFeedbackBufferBase_p)(xfb, index, buffer)
  }
  /// [glTransformFeedbackBufferRange](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTransformFeedbackBufferRange.xhtml)
  /// * `xfb` class: transform feedback
  /// * `buffer` class: buffer
  /// * `size` group: BufferSize
  pub unsafe fn TransformFeedbackBufferRange(&self, xfb: GLuint, index: GLuint, buffer: GLuint, offset: GLintptr, size: GLsizeiptr) {
    (self.glTransformFeedbackBufferRange_p)(xfb, index, buffer, offset, size)
  }
  /// [glTransformFeedbackVaryings](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTransformFeedbackVaryings.xhtml)
  /// * `program` class: program
  /// * `varyings` len: count
  /// * `bufferMode` group: TransformFeedbackBufferMode
  pub unsafe fn TransformFeedbackVaryings(&self, program: GLuint, count: GLsizei, varyings: *const *const GLchar, bufferMode: TransformFeedbackBufferMode) {
    (self.glTransformFeedbackVaryings_p)(program, count, varyings, bufferMode)
  }
  /// [glUniform1d](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform1d.xhtml)
  pub unsafe fn Uniform1d(&self, location: GLint, x: GLdouble) {
    (self.glUniform1d_p)(location, x)
  }
  /// [glUniform1dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform1dv.xhtml)
  /// * `value` len: count*1
  pub unsafe fn Uniform1dv(&self, location: GLint, count: GLsizei, value: *const GLdouble) {
    (self.glUniform1dv_p)(location, count, value)
  }
  /// [glUniform1f](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform1f.xhtml)
  pub unsafe fn Uniform1f(&self, location: GLint, v0: GLfloat) {
    (self.glUniform1f_p)(location, v0)
  }
  /// [glUniform1fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform1fv.xhtml)
  /// * `value` len: count*1
  pub unsafe fn Uniform1fv(&self, location: GLint, count: GLsizei, value: *const GLfloat) {
    (self.glUniform1fv_p)(location, count, value)
  }
  /// [glUniform1i](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform1i.xhtml)
  pub unsafe fn Uniform1i(&self, location: GLint, v0: GLint) {
    (self.glUniform1i_p)(location, v0)
  }
  /// [glUniform1iv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform1iv.xhtml)
  /// * `value` len: count*1
  pub unsafe fn Uniform1iv(&self, location: GLint, count: GLsizei, value: *const GLint) {
    (self.glUniform1iv_p)(location, count, value)
  }
  /// [glUniform1ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform1ui.xhtml)
  pub unsafe fn Uniform1ui(&self, location: GLint, v0: GLuint) {
    (self.glUniform1ui_p)(location, v0)
  }
  /// [glUniform1uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform1uiv.xhtml)
  /// * `value` len: count*1
  pub unsafe fn Uniform1uiv(&self, location: GLint, count: GLsizei, value: *const GLuint) {
    (self.glUniform1uiv_p)(location, count, value)
  }
  /// [glUniform2d](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform2d.xhtml)
  pub unsafe fn Uniform2d(&self, location: GLint, x: GLdouble, y: GLdouble) {
    (self.glUniform2d_p)(location, x, y)
  }
  /// [glUniform2dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform2dv.xhtml)
  /// * `value` len: count*2
  pub unsafe fn Uniform2dv(&self, location: GLint, count: GLsizei, value: *const GLdouble) {
    (self.glUniform2dv_p)(location, count, value)
  }
  /// [glUniform2f](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform2f.xhtml)
  pub unsafe fn Uniform2f(&self, location: GLint, v0: GLfloat, v1: GLfloat) {
    (self.glUniform2f_p)(location, v0, v1)
  }
  /// [glUniform2fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform2fv.xhtml)
  /// * `value` len: count*2
  pub unsafe fn Uniform2fv(&self, location: GLint, count: GLsizei, value: *const GLfloat) {
    (self.glUniform2fv_p)(location, count, value)
  }
  /// [glUniform2i](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform2i.xhtml)
  pub unsafe fn Uniform2i(&self, location: GLint, v0: GLint, v1: GLint) {
    (self.glUniform2i_p)(location, v0, v1)
  }
  /// [glUniform2iv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform2iv.xhtml)
  /// * `value` len: count*2
  pub unsafe fn Uniform2iv(&self, location: GLint, count: GLsizei, value: *const GLint) {
    (self.glUniform2iv_p)(location, count, value)
  }
  /// [glUniform2ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform2ui.xhtml)
  pub unsafe fn Uniform2ui(&self, location: GLint, v0: GLuint, v1: GLuint) {
    (self.glUniform2ui_p)(location, v0, v1)
  }
  /// [glUniform2uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform2uiv.xhtml)
  /// * `value` len: count*2
  pub unsafe fn Uniform2uiv(&self, location: GLint, count: GLsizei, value: *const GLuint) {
    (self.glUniform2uiv_p)(location, count, value)
  }
  /// [glUniform3d](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform3d.xhtml)
  pub unsafe fn Uniform3d(&self, location: GLint, x: GLdouble, y: GLdouble, z: GLdouble) {
    (self.glUniform3d_p)(location, x, y, z)
  }
  /// [glUniform3dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform3dv.xhtml)
  /// * `value` len: count*3
  pub unsafe fn Uniform3dv(&self, location: GLint, count: GLsizei, value: *const GLdouble) {
    (self.glUniform3dv_p)(location, count, value)
  }
  /// [glUniform3f](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform3f.xhtml)
  pub unsafe fn Uniform3f(&self, location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat) {
    (self.glUniform3f_p)(location, v0, v1, v2)
  }
  /// [glUniform3fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform3fv.xhtml)
  /// * `value` len: count*3
  pub unsafe fn Uniform3fv(&self, location: GLint, count: GLsizei, value: *const GLfloat) {
    (self.glUniform3fv_p)(location, count, value)
  }
  /// [glUniform3i](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform3i.xhtml)
  pub unsafe fn Uniform3i(&self, location: GLint, v0: GLint, v1: GLint, v2: GLint) {
    (self.glUniform3i_p)(location, v0, v1, v2)
  }
  /// [glUniform3iv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform3iv.xhtml)
  /// * `value` len: count*3
  pub unsafe fn Uniform3iv(&self, location: GLint, count: GLsizei, value: *const GLint) {
    (self.glUniform3iv_p)(location, count, value)
  }
  /// [glUniform3ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform3ui.xhtml)
  pub unsafe fn Uniform3ui(&self, location: GLint, v0: GLuint, v1: GLuint, v2: GLuint) {
    (self.glUniform3ui_p)(location, v0, v1, v2)
  }
  /// [glUniform3uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform3uiv.xhtml)
  /// * `value` len: count*3
  pub unsafe fn Uniform3uiv(&self, location: GLint, count: GLsizei, value: *const GLuint) {
    (self.glUniform3uiv_p)(location, count, value)
  }
  /// [glUniform4d](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform4d.xhtml)
  pub unsafe fn Uniform4d(&self, location: GLint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble) {
    (self.glUniform4d_p)(location, x, y, z, w)
  }
  /// [glUniform4dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform4dv.xhtml)
  /// * `value` len: count*4
  pub unsafe fn Uniform4dv(&self, location: GLint, count: GLsizei, value: *const GLdouble) {
    (self.glUniform4dv_p)(location, count, value)
  }
  /// [glUniform4f](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform4f.xhtml)
  pub unsafe fn Uniform4f(&self, location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat) {
    (self.glUniform4f_p)(location, v0, v1, v2, v3)
  }
  /// [glUniform4fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform4fv.xhtml)
  /// * `value` len: count*4
  pub unsafe fn Uniform4fv(&self, location: GLint, count: GLsizei, value: *const GLfloat) {
    (self.glUniform4fv_p)(location, count, value)
  }
  /// [glUniform4i](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform4i.xhtml)
  pub unsafe fn Uniform4i(&self, location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint) {
    (self.glUniform4i_p)(location, v0, v1, v2, v3)
  }
  /// [glUniform4iv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform4iv.xhtml)
  /// * `value` len: count*4
  pub unsafe fn Uniform4iv(&self, location: GLint, count: GLsizei, value: *const GLint) {
    (self.glUniform4iv_p)(location, count, value)
  }
  /// [glUniform4ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform4ui.xhtml)
  pub unsafe fn Uniform4ui(&self, location: GLint, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint) {
    (self.glUniform4ui_p)(location, v0, v1, v2, v3)
  }
  /// [glUniform4uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniform4uiv.xhtml)
  /// * `value` len: count*4
  pub unsafe fn Uniform4uiv(&self, location: GLint, count: GLsizei, value: *const GLuint) {
    (self.glUniform4uiv_p)(location, count, value)
  }
  /// [glUniformBlockBinding](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniformBlockBinding.xhtml)
  /// * `program` class: program
  pub unsafe fn UniformBlockBinding(&self, program: GLuint, uniformBlockIndex: GLuint, uniformBlockBinding: GLuint) {
    (self.glUniformBlockBinding_p)(program, uniformBlockIndex, uniformBlockBinding)
  }
  /// [glUniformMatrix2dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniformMatrix2dv.xhtml)
  /// * `transpose` group: Boolean
  /// * `value` len: count*4
  pub unsafe fn UniformMatrix2dv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) {
    (self.glUniformMatrix2dv_p)(location, count, transpose, value)
  }
  /// [glUniformMatrix2fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniformMatrix2fv.xhtml)
  /// * `transpose` group: Boolean
  /// * `value` len: count*4
  pub unsafe fn UniformMatrix2fv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
    (self.glUniformMatrix2fv_p)(location, count, transpose, value)
  }
  /// [glUniformMatrix2x3dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniformMatrix2x3dv.xhtml)
  /// * `transpose` group: Boolean
  /// * `value` len: count*6
  pub unsafe fn UniformMatrix2x3dv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) {
    (self.glUniformMatrix2x3dv_p)(location, count, transpose, value)
  }
  /// [glUniformMatrix2x3fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniformMatrix2x3fv.xhtml)
  /// * `transpose` group: Boolean
  /// * `value` len: count*6
  pub unsafe fn UniformMatrix2x3fv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
    (self.glUniformMatrix2x3fv_p)(location, count, transpose, value)
  }
  /// [glUniformMatrix2x4dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniformMatrix2x4dv.xhtml)
  /// * `transpose` group: Boolean
  /// * `value` len: count*8
  pub unsafe fn UniformMatrix2x4dv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) {
    (self.glUniformMatrix2x4dv_p)(location, count, transpose, value)
  }
  /// [glUniformMatrix2x4fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniformMatrix2x4fv.xhtml)
  /// * `transpose` group: Boolean
  /// * `value` len: count*8
  pub unsafe fn UniformMatrix2x4fv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
    (self.glUniformMatrix2x4fv_p)(location, count, transpose, value)
  }
  /// [glUniformMatrix3dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniformMatrix3dv.xhtml)
  /// * `transpose` group: Boolean
  /// * `value` len: count*9
  pub unsafe fn UniformMatrix3dv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) {
    (self.glUniformMatrix3dv_p)(location, count, transpose, value)
  }
  /// [glUniformMatrix3fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniformMatrix3fv.xhtml)
  /// * `transpose` group: Boolean
  /// * `value` len: count*9
  pub unsafe fn UniformMatrix3fv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
    (self.glUniformMatrix3fv_p)(location, count, transpose, value)
  }
  /// [glUniformMatrix3x2dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniformMatrix3x2dv.xhtml)
  /// * `transpose` group: Boolean
  /// * `value` len: count*6
  pub unsafe fn UniformMatrix3x2dv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) {
    (self.glUniformMatrix3x2dv_p)(location, count, transpose, value)
  }
  /// [glUniformMatrix3x2fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniformMatrix3x2fv.xhtml)
  /// * `transpose` group: Boolean
  /// * `value` len: count*6
  pub unsafe fn UniformMatrix3x2fv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
    (self.glUniformMatrix3x2fv_p)(location, count, transpose, value)
  }
  /// [glUniformMatrix3x4dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniformMatrix3x4dv.xhtml)
  /// * `transpose` group: Boolean
  /// * `value` len: count*12
  pub unsafe fn UniformMatrix3x4dv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) {
    (self.glUniformMatrix3x4dv_p)(location, count, transpose, value)
  }
  /// [glUniformMatrix3x4fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniformMatrix3x4fv.xhtml)
  /// * `transpose` group: Boolean
  /// * `value` len: count*12
  pub unsafe fn UniformMatrix3x4fv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
    (self.glUniformMatrix3x4fv_p)(location, count, transpose, value)
  }
  /// [glUniformMatrix4dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniformMatrix4dv.xhtml)
  /// * `transpose` group: Boolean
  /// * `value` len: count*16
  pub unsafe fn UniformMatrix4dv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) {
    (self.glUniformMatrix4dv_p)(location, count, transpose, value)
  }
  /// [glUniformMatrix4fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniformMatrix4fv.xhtml)
  /// * `transpose` group: Boolean
  /// * `value` len: count*16
  pub unsafe fn UniformMatrix4fv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
    (self.glUniformMatrix4fv_p)(location, count, transpose, value)
  }
  /// [glUniformMatrix4x2dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniformMatrix4x2dv.xhtml)
  /// * `transpose` group: Boolean
  /// * `value` len: count*8
  pub unsafe fn UniformMatrix4x2dv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) {
    (self.glUniformMatrix4x2dv_p)(location, count, transpose, value)
  }
  /// [glUniformMatrix4x2fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniformMatrix4x2fv.xhtml)
  /// * `transpose` group: Boolean
  /// * `value` len: count*8
  pub unsafe fn UniformMatrix4x2fv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
    (self.glUniformMatrix4x2fv_p)(location, count, transpose, value)
  }
  /// [glUniformMatrix4x3dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniformMatrix4x3dv.xhtml)
  /// * `transpose` group: Boolean
  /// * `value` len: count*12
  pub unsafe fn UniformMatrix4x3dv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) {
    (self.glUniformMatrix4x3dv_p)(location, count, transpose, value)
  }
  /// [glUniformMatrix4x3fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniformMatrix4x3fv.xhtml)
  /// * `transpose` group: Boolean
  /// * `value` len: count*12
  pub unsafe fn UniformMatrix4x3fv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
    (self.glUniformMatrix4x3fv_p)(location, count, transpose, value)
  }
  /// [glUniformSubroutinesuiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUniformSubroutinesuiv.xhtml)
  /// * `shadertype` group: ShaderType
  /// * `indices` len: count
  pub unsafe fn UniformSubroutinesuiv(&self, shadertype: ShaderType, count: GLsizei, indices: *const GLuint) {
    (self.glUniformSubroutinesuiv_p)(shadertype, count, indices)
  }
  /// [glUnmapBuffer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUnmapBuffer.xhtml)
  /// * `target` group: BufferTargetARB
  pub unsafe fn UnmapBuffer(&self, target: BufferTargetARB) -> GLboolean {
    (self.glUnmapBuffer_p)(target)
  }
  /// [glUnmapNamedBuffer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUnmapNamedBuffer.xhtml)
  /// * `buffer` class: buffer
  pub unsafe fn UnmapNamedBuffer(&self, buffer: GLuint) -> GLboolean {
    (self.glUnmapNamedBuffer_p)(buffer)
  }
  /// [glUseProgram](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUseProgram.xhtml)
  /// * `program` class: program
  pub unsafe fn UseProgram(&self, program: GLuint) {
    (self.glUseProgram_p)(program)
  }
  /// [glUseProgramStages](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUseProgramStages.xhtml)
  /// * `pipeline` class: program pipeline
  /// * `stages` group: UseProgramStageMask
  /// * `program` class: program
  pub unsafe fn UseProgramStages(&self, pipeline: GLuint, stages: GLbitfield, program: GLuint) {
    (self.glUseProgramStages_p)(pipeline, stages, program)
  }
  /// [glValidateProgram](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glValidateProgram.xhtml)
  /// * `program` class: program
  pub unsafe fn ValidateProgram(&self, program: GLuint) {
    (self.glValidateProgram_p)(program)
  }
  /// [glValidateProgramPipeline](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glValidateProgramPipeline.xhtml)
  /// * `pipeline` class: program pipeline
  pub unsafe fn ValidateProgramPipeline(&self, pipeline: GLuint) {
    (self.glValidateProgramPipeline_p)(pipeline)
  }
  /// [glVertexArrayAttribBinding](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexArrayAttribBinding.xhtml)
  /// * `vaobj` class: vertex array
  pub unsafe fn VertexArrayAttribBinding(&self, vaobj: GLuint, attribindex: GLuint, bindingindex: GLuint) {
    (self.glVertexArrayAttribBinding_p)(vaobj, attribindex, bindingindex)
  }
  /// [glVertexArrayAttribFormat](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexArrayAttribFormat.xhtml)
  /// * `vaobj` class: vertex array
  /// * `type` group: VertexAttribType
  /// * `normalized` group: Boolean
  pub unsafe fn VertexArrayAttribFormat(&self, vaobj: GLuint, attribindex: GLuint, size: GLint, type_: VertexAttribType, normalized: GLboolean, relativeoffset: GLuint) {
    (self.glVertexArrayAttribFormat_p)(vaobj, attribindex, size, type_, normalized, relativeoffset)
  }
  /// [glVertexArrayAttribIFormat](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexArrayAttribIFormat.xhtml)
  /// * `vaobj` class: vertex array
  /// * `type` group: VertexAttribIType
  pub unsafe fn VertexArrayAttribIFormat(&self, vaobj: GLuint, attribindex: GLuint, size: GLint, type_: VertexAttribIType, relativeoffset: GLuint) {
    (self.glVertexArrayAttribIFormat_p)(vaobj, attribindex, size, type_, relativeoffset)
  }
  /// [glVertexArrayAttribLFormat](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexArrayAttribLFormat.xhtml)
  /// * `vaobj` class: vertex array
  /// * `type` group: VertexAttribLType
  pub unsafe fn VertexArrayAttribLFormat(&self, vaobj: GLuint, attribindex: GLuint, size: GLint, type_: VertexAttribLType, relativeoffset: GLuint) {
    (self.glVertexArrayAttribLFormat_p)(vaobj, attribindex, size, type_, relativeoffset)
  }
  /// [glVertexArrayBindingDivisor](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexArrayBindingDivisor.xhtml)
  /// * `vaobj` class: vertex array
  pub unsafe fn VertexArrayBindingDivisor(&self, vaobj: GLuint, bindingindex: GLuint, divisor: GLuint) {
    (self.glVertexArrayBindingDivisor_p)(vaobj, bindingindex, divisor)
  }
  /// [glVertexArrayElementBuffer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexArrayElementBuffer.xhtml)
  /// * `vaobj` class: vertex array
  /// * `buffer` class: buffer
  pub unsafe fn VertexArrayElementBuffer(&self, vaobj: GLuint, buffer: GLuint) {
    (self.glVertexArrayElementBuffer_p)(vaobj, buffer)
  }
  /// [glVertexArrayVertexBuffer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexArrayVertexBuffer.xhtml)
  /// * `vaobj` class: vertex array
  /// * `buffer` class: buffer
  pub unsafe fn VertexArrayVertexBuffer(&self, vaobj: GLuint, bindingindex: GLuint, buffer: GLuint, offset: GLintptr, stride: GLsizei) {
    (self.glVertexArrayVertexBuffer_p)(vaobj, bindingindex, buffer, offset, stride)
  }
  /// [glVertexArrayVertexBuffers](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexArrayVertexBuffers.xhtml)
  /// * `vaobj` class: vertex array
  /// * `buffers` class: buffer
  pub unsafe fn VertexArrayVertexBuffers(&self, vaobj: GLuint, first: GLuint, count: GLsizei, buffers: *const GLuint, offsets: *const GLintptr, strides: *const GLsizei) {
    (self.glVertexArrayVertexBuffers_p)(vaobj, first, count, buffers, offsets, strides)
  }
  /// [glVertexAttrib1d](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib1d.xhtml)
  pub unsafe fn VertexAttrib1d(&self, index: GLuint, x: GLdouble) {
    (self.glVertexAttrib1d_p)(index, x)
  }
  /// [glVertexAttrib1dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib1dv.xhtml)
  pub unsafe fn VertexAttrib1dv(&self, index: GLuint, v: *const GLdouble) {
    (self.glVertexAttrib1dv_p)(index, v)
  }
  /// [glVertexAttrib1f](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib1f.xhtml)
  pub unsafe fn VertexAttrib1f(&self, index: GLuint, x: GLfloat) {
    (self.glVertexAttrib1f_p)(index, x)
  }
  /// [glVertexAttrib1fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib1fv.xhtml)
  pub unsafe fn VertexAttrib1fv(&self, index: GLuint, v: *const GLfloat) {
    (self.glVertexAttrib1fv_p)(index, v)
  }
  /// [glVertexAttrib1s](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib1s.xhtml)
  pub unsafe fn VertexAttrib1s(&self, index: GLuint, x: GLshort) {
    (self.glVertexAttrib1s_p)(index, x)
  }
  /// [glVertexAttrib1sv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib1sv.xhtml)
  pub unsafe fn VertexAttrib1sv(&self, index: GLuint, v: *const GLshort) {
    (self.glVertexAttrib1sv_p)(index, v)
  }
  /// [glVertexAttrib2d](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib2d.xhtml)
  pub unsafe fn VertexAttrib2d(&self, index: GLuint, x: GLdouble, y: GLdouble) {
    (self.glVertexAttrib2d_p)(index, x, y)
  }
  /// [glVertexAttrib2dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib2dv.xhtml)
  pub unsafe fn VertexAttrib2dv(&self, index: GLuint, v: *const [GLdouble; 2]) {
    (self.glVertexAttrib2dv_p)(index, v)
  }
  /// [glVertexAttrib2f](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib2f.xhtml)
  pub unsafe fn VertexAttrib2f(&self, index: GLuint, x: GLfloat, y: GLfloat) {
    (self.glVertexAttrib2f_p)(index, x, y)
  }
  /// [glVertexAttrib2fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib2fv.xhtml)
  pub unsafe fn VertexAttrib2fv(&self, index: GLuint, v: *const [GLfloat; 2]) {
    (self.glVertexAttrib2fv_p)(index, v)
  }
  /// [glVertexAttrib2s](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib2s.xhtml)
  pub unsafe fn VertexAttrib2s(&self, index: GLuint, x: GLshort, y: GLshort) {
    (self.glVertexAttrib2s_p)(index, x, y)
  }
  /// [glVertexAttrib2sv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib2sv.xhtml)
  pub unsafe fn VertexAttrib2sv(&self, index: GLuint, v: *const [GLshort; 2]) {
    (self.glVertexAttrib2sv_p)(index, v)
  }
  /// [glVertexAttrib3d](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib3d.xhtml)
  pub unsafe fn VertexAttrib3d(&self, index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble) {
    (self.glVertexAttrib3d_p)(index, x, y, z)
  }
  /// [glVertexAttrib3dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib3dv.xhtml)
  pub unsafe fn VertexAttrib3dv(&self, index: GLuint, v: *const [GLdouble; 3]) {
    (self.glVertexAttrib3dv_p)(index, v)
  }
  /// [glVertexAttrib3f](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib3f.xhtml)
  pub unsafe fn VertexAttrib3f(&self, index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat) {
    (self.glVertexAttrib3f_p)(index, x, y, z)
  }
  /// [glVertexAttrib3fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib3fv.xhtml)
  pub unsafe fn VertexAttrib3fv(&self, index: GLuint, v: *const [GLfloat; 3]) {
    (self.glVertexAttrib3fv_p)(index, v)
  }
  /// [glVertexAttrib3s](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib3s.xhtml)
  pub unsafe fn VertexAttrib3s(&self, index: GLuint, x: GLshort, y: GLshort, z: GLshort) {
    (self.glVertexAttrib3s_p)(index, x, y, z)
  }
  /// [glVertexAttrib3sv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib3sv.xhtml)
  pub unsafe fn VertexAttrib3sv(&self, index: GLuint, v: *const [GLshort; 3]) {
    (self.glVertexAttrib3sv_p)(index, v)
  }
  /// [glVertexAttrib4Nbv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib4Nbv.xhtml)
  pub unsafe fn VertexAttrib4Nbv(&self, index: GLuint, v: *const [GLbyte; 4]) {
    (self.glVertexAttrib4Nbv_p)(index, v)
  }
  /// [glVertexAttrib4Niv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib4Niv.xhtml)
  pub unsafe fn VertexAttrib4Niv(&self, index: GLuint, v: *const [GLint; 4]) {
    (self.glVertexAttrib4Niv_p)(index, v)
  }
  /// [glVertexAttrib4Nsv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib4Nsv.xhtml)
  pub unsafe fn VertexAttrib4Nsv(&self, index: GLuint, v: *const [GLshort; 4]) {
    (self.glVertexAttrib4Nsv_p)(index, v)
  }
  /// [glVertexAttrib4Nub](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib4Nub.xhtml)
  pub unsafe fn VertexAttrib4Nub(&self, index: GLuint, x: GLubyte, y: GLubyte, z: GLubyte, w: GLubyte) {
    (self.glVertexAttrib4Nub_p)(index, x, y, z, w)
  }
  /// [glVertexAttrib4Nubv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib4Nubv.xhtml)
  pub unsafe fn VertexAttrib4Nubv(&self, index: GLuint, v: *const [GLubyte; 4]) {
    (self.glVertexAttrib4Nubv_p)(index, v)
  }
  /// [glVertexAttrib4Nuiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib4Nuiv.xhtml)
  pub unsafe fn VertexAttrib4Nuiv(&self, index: GLuint, v: *const [GLuint; 4]) {
    (self.glVertexAttrib4Nuiv_p)(index, v)
  }
  /// [glVertexAttrib4Nusv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib4Nusv.xhtml)
  pub unsafe fn VertexAttrib4Nusv(&self, index: GLuint, v: *const [GLushort; 4]) {
    (self.glVertexAttrib4Nusv_p)(index, v)
  }
  /// [glVertexAttrib4bv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib4bv.xhtml)
  pub unsafe fn VertexAttrib4bv(&self, index: GLuint, v: *const [GLbyte; 4]) {
    (self.glVertexAttrib4bv_p)(index, v)
  }
  /// [glVertexAttrib4d](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib4d.xhtml)
  pub unsafe fn VertexAttrib4d(&self, index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble) {
    (self.glVertexAttrib4d_p)(index, x, y, z, w)
  }
  /// [glVertexAttrib4dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib4dv.xhtml)
  pub unsafe fn VertexAttrib4dv(&self, index: GLuint, v: *const [GLdouble; 4]) {
    (self.glVertexAttrib4dv_p)(index, v)
  }
  /// [glVertexAttrib4f](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib4f.xhtml)
  pub unsafe fn VertexAttrib4f(&self, index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat) {
    (self.glVertexAttrib4f_p)(index, x, y, z, w)
  }
  /// [glVertexAttrib4fv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib4fv.xhtml)
  pub unsafe fn VertexAttrib4fv(&self, index: GLuint, v: *const [GLfloat; 4]) {
    (self.glVertexAttrib4fv_p)(index, v)
  }
  /// [glVertexAttrib4iv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib4iv.xhtml)
  pub unsafe fn VertexAttrib4iv(&self, index: GLuint, v: *const [GLint; 4]) {
    (self.glVertexAttrib4iv_p)(index, v)
  }
  /// [glVertexAttrib4s](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib4s.xhtml)
  pub unsafe fn VertexAttrib4s(&self, index: GLuint, x: GLshort, y: GLshort, z: GLshort, w: GLshort) {
    (self.glVertexAttrib4s_p)(index, x, y, z, w)
  }
  /// [glVertexAttrib4sv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib4sv.xhtml)
  pub unsafe fn VertexAttrib4sv(&self, index: GLuint, v: *const [GLshort; 4]) {
    (self.glVertexAttrib4sv_p)(index, v)
  }
  /// [glVertexAttrib4ubv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib4ubv.xhtml)
  pub unsafe fn VertexAttrib4ubv(&self, index: GLuint, v: *const [GLubyte; 4]) {
    (self.glVertexAttrib4ubv_p)(index, v)
  }
  /// [glVertexAttrib4uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib4uiv.xhtml)
  pub unsafe fn VertexAttrib4uiv(&self, index: GLuint, v: *const [GLuint; 4]) {
    (self.glVertexAttrib4uiv_p)(index, v)
  }
  /// [glVertexAttrib4usv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttrib4usv.xhtml)
  pub unsafe fn VertexAttrib4usv(&self, index: GLuint, v: *const [GLushort; 4]) {
    (self.glVertexAttrib4usv_p)(index, v)
  }
  /// [glVertexAttribBinding](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribBinding.xhtml)
  pub unsafe fn VertexAttribBinding(&self, attribindex: GLuint, bindingindex: GLuint) {
    (self.glVertexAttribBinding_p)(attribindex, bindingindex)
  }
  /// [glVertexAttribDivisor](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribDivisor.xhtml)
  pub unsafe fn VertexAttribDivisor(&self, index: GLuint, divisor: GLuint) {
    (self.glVertexAttribDivisor_p)(index, divisor)
  }
  /// [glVertexAttribFormat](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribFormat.xhtml)
  /// * `type` group: VertexAttribType
  /// * `normalized` group: Boolean
  pub unsafe fn VertexAttribFormat(&self, attribindex: GLuint, size: GLint, type_: VertexAttribType, normalized: GLboolean, relativeoffset: GLuint) {
    (self.glVertexAttribFormat_p)(attribindex, size, type_, normalized, relativeoffset)
  }
  /// [glVertexAttribI1i](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribI1i.xhtml)
  pub unsafe fn VertexAttribI1i(&self, index: GLuint, x: GLint) {
    (self.glVertexAttribI1i_p)(index, x)
  }
  /// [glVertexAttribI1iv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribI1iv.xhtml)
  pub unsafe fn VertexAttribI1iv(&self, index: GLuint, v: *const GLint) {
    (self.glVertexAttribI1iv_p)(index, v)
  }
  /// [glVertexAttribI1ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribI1ui.xhtml)
  pub unsafe fn VertexAttribI1ui(&self, index: GLuint, x: GLuint) {
    (self.glVertexAttribI1ui_p)(index, x)
  }
  /// [glVertexAttribI1uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribI1uiv.xhtml)
  pub unsafe fn VertexAttribI1uiv(&self, index: GLuint, v: *const GLuint) {
    (self.glVertexAttribI1uiv_p)(index, v)
  }
  /// [glVertexAttribI2i](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribI2i.xhtml)
  pub unsafe fn VertexAttribI2i(&self, index: GLuint, x: GLint, y: GLint) {
    (self.glVertexAttribI2i_p)(index, x, y)
  }
  /// [glVertexAttribI2iv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribI2iv.xhtml)
  pub unsafe fn VertexAttribI2iv(&self, index: GLuint, v: *const [GLint; 2]) {
    (self.glVertexAttribI2iv_p)(index, v)
  }
  /// [glVertexAttribI2ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribI2ui.xhtml)
  pub unsafe fn VertexAttribI2ui(&self, index: GLuint, x: GLuint, y: GLuint) {
    (self.glVertexAttribI2ui_p)(index, x, y)
  }
  /// [glVertexAttribI2uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribI2uiv.xhtml)
  pub unsafe fn VertexAttribI2uiv(&self, index: GLuint, v: *const [GLuint; 2]) {
    (self.glVertexAttribI2uiv_p)(index, v)
  }
  /// [glVertexAttribI3i](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribI3i.xhtml)
  pub unsafe fn VertexAttribI3i(&self, index: GLuint, x: GLint, y: GLint, z: GLint) {
    (self.glVertexAttribI3i_p)(index, x, y, z)
  }
  /// [glVertexAttribI3iv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribI3iv.xhtml)
  pub unsafe fn VertexAttribI3iv(&self, index: GLuint, v: *const [GLint; 3]) {
    (self.glVertexAttribI3iv_p)(index, v)
  }
  /// [glVertexAttribI3ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribI3ui.xhtml)
  pub unsafe fn VertexAttribI3ui(&self, index: GLuint, x: GLuint, y: GLuint, z: GLuint) {
    (self.glVertexAttribI3ui_p)(index, x, y, z)
  }
  /// [glVertexAttribI3uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribI3uiv.xhtml)
  pub unsafe fn VertexAttribI3uiv(&self, index: GLuint, v: *const [GLuint; 3]) {
    (self.glVertexAttribI3uiv_p)(index, v)
  }
  /// [glVertexAttribI4bv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribI4bv.xhtml)
  pub unsafe fn VertexAttribI4bv(&self, index: GLuint, v: *const [GLbyte; 4]) {
    (self.glVertexAttribI4bv_p)(index, v)
  }
  /// [glVertexAttribI4i](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribI4i.xhtml)
  pub unsafe fn VertexAttribI4i(&self, index: GLuint, x: GLint, y: GLint, z: GLint, w: GLint) {
    (self.glVertexAttribI4i_p)(index, x, y, z, w)
  }
  /// [glVertexAttribI4iv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribI4iv.xhtml)
  pub unsafe fn VertexAttribI4iv(&self, index: GLuint, v: *const [GLint; 4]) {
    (self.glVertexAttribI4iv_p)(index, v)
  }
  /// [glVertexAttribI4sv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribI4sv.xhtml)
  pub unsafe fn VertexAttribI4sv(&self, index: GLuint, v: *const [GLshort; 4]) {
    (self.glVertexAttribI4sv_p)(index, v)
  }
  /// [glVertexAttribI4ubv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribI4ubv.xhtml)
  pub unsafe fn VertexAttribI4ubv(&self, index: GLuint, v: *const [GLubyte; 4]) {
    (self.glVertexAttribI4ubv_p)(index, v)
  }
  /// [glVertexAttribI4ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribI4ui.xhtml)
  pub unsafe fn VertexAttribI4ui(&self, index: GLuint, x: GLuint, y: GLuint, z: GLuint, w: GLuint) {
    (self.glVertexAttribI4ui_p)(index, x, y, z, w)
  }
  /// [glVertexAttribI4uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribI4uiv.xhtml)
  pub unsafe fn VertexAttribI4uiv(&self, index: GLuint, v: *const [GLuint; 4]) {
    (self.glVertexAttribI4uiv_p)(index, v)
  }
  /// [glVertexAttribI4usv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribI4usv.xhtml)
  pub unsafe fn VertexAttribI4usv(&self, index: GLuint, v: *const [GLushort; 4]) {
    (self.glVertexAttribI4usv_p)(index, v)
  }
  /// [glVertexAttribIFormat](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribIFormat.xhtml)
  /// * `type` group: VertexAttribIType
  pub unsafe fn VertexAttribIFormat(&self, attribindex: GLuint, size: GLint, type_: VertexAttribIType, relativeoffset: GLuint) {
    (self.glVertexAttribIFormat_p)(attribindex, size, type_, relativeoffset)
  }
  /// [glVertexAttribIPointer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribIPointer.xhtml)
  /// * `type` group: VertexAttribIType
  /// * `pointer` len: COMPSIZE(size,type,stride)
  pub unsafe fn VertexAttribIPointer(&self, index: GLuint, size: GLint, type_: VertexAttribIType, stride: GLsizei, pointer: *const void) {
    (self.glVertexAttribIPointer_p)(index, size, type_, stride, pointer)
  }
  /// [glVertexAttribL1d](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribL1d.xhtml)
  pub unsafe fn VertexAttribL1d(&self, index: GLuint, x: GLdouble) {
    (self.glVertexAttribL1d_p)(index, x)
  }
  /// [glVertexAttribL1dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribL1dv.xhtml)
  pub unsafe fn VertexAttribL1dv(&self, index: GLuint, v: *const GLdouble) {
    (self.glVertexAttribL1dv_p)(index, v)
  }
  /// [glVertexAttribL2d](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribL2d.xhtml)
  pub unsafe fn VertexAttribL2d(&self, index: GLuint, x: GLdouble, y: GLdouble) {
    (self.glVertexAttribL2d_p)(index, x, y)
  }
  /// [glVertexAttribL2dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribL2dv.xhtml)
  pub unsafe fn VertexAttribL2dv(&self, index: GLuint, v: *const [GLdouble; 2]) {
    (self.glVertexAttribL2dv_p)(index, v)
  }
  /// [glVertexAttribL3d](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribL3d.xhtml)
  pub unsafe fn VertexAttribL3d(&self, index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble) {
    (self.glVertexAttribL3d_p)(index, x, y, z)
  }
  /// [glVertexAttribL3dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribL3dv.xhtml)
  pub unsafe fn VertexAttribL3dv(&self, index: GLuint, v: *const [GLdouble; 3]) {
    (self.glVertexAttribL3dv_p)(index, v)
  }
  /// [glVertexAttribL4d](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribL4d.xhtml)
  pub unsafe fn VertexAttribL4d(&self, index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble) {
    (self.glVertexAttribL4d_p)(index, x, y, z, w)
  }
  /// [glVertexAttribL4dv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribL4dv.xhtml)
  pub unsafe fn VertexAttribL4dv(&self, index: GLuint, v: *const [GLdouble; 4]) {
    (self.glVertexAttribL4dv_p)(index, v)
  }
  /// [glVertexAttribLFormat](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribLFormat.xhtml)
  /// * `type` group: VertexAttribLType
  pub unsafe fn VertexAttribLFormat(&self, attribindex: GLuint, size: GLint, type_: VertexAttribLType, relativeoffset: GLuint) {
    (self.glVertexAttribLFormat_p)(attribindex, size, type_, relativeoffset)
  }
  /// [glVertexAttribLPointer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribLPointer.xhtml)
  /// * `type` group: VertexAttribLType
  /// * `pointer` len: size
  pub unsafe fn VertexAttribLPointer(&self, index: GLuint, size: GLint, type_: VertexAttribLType, stride: GLsizei, pointer: *const void) {
    (self.glVertexAttribLPointer_p)(index, size, type_, stride, pointer)
  }
  /// [glVertexAttribP1ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribP1ui.xhtml)
  /// * `type` group: VertexAttribPointerType
  /// * `normalized` group: Boolean
  pub unsafe fn VertexAttribP1ui(&self, index: GLuint, type_: VertexAttribPointerType, normalized: GLboolean, value: GLuint) {
    (self.glVertexAttribP1ui_p)(index, type_, normalized, value)
  }
  /// [glVertexAttribP1uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribP1uiv.xhtml)
  /// * `type` group: VertexAttribPointerType
  /// * `normalized` group: Boolean
  pub unsafe fn VertexAttribP1uiv(&self, index: GLuint, type_: VertexAttribPointerType, normalized: GLboolean, value: *const GLuint) {
    (self.glVertexAttribP1uiv_p)(index, type_, normalized, value)
  }
  /// [glVertexAttribP2ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribP2ui.xhtml)
  /// * `type` group: VertexAttribPointerType
  /// * `normalized` group: Boolean
  pub unsafe fn VertexAttribP2ui(&self, index: GLuint, type_: VertexAttribPointerType, normalized: GLboolean, value: GLuint) {
    (self.glVertexAttribP2ui_p)(index, type_, normalized, value)
  }
  /// [glVertexAttribP2uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribP2uiv.xhtml)
  /// * `type` group: VertexAttribPointerType
  /// * `normalized` group: Boolean
  pub unsafe fn VertexAttribP2uiv(&self, index: GLuint, type_: VertexAttribPointerType, normalized: GLboolean, value: *const GLuint) {
    (self.glVertexAttribP2uiv_p)(index, type_, normalized, value)
  }
  /// [glVertexAttribP3ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribP3ui.xhtml)
  /// * `type` group: VertexAttribPointerType
  /// * `normalized` group: Boolean
  pub unsafe fn VertexAttribP3ui(&self, index: GLuint, type_: VertexAttribPointerType, normalized: GLboolean, value: GLuint) {
    (self.glVertexAttribP3ui_p)(index, type_, normalized, value)
  }
  /// [glVertexAttribP3uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribP3uiv.xhtml)
  /// * `type` group: VertexAttribPointerType
  /// * `normalized` group: Boolean
  pub unsafe fn VertexAttribP3uiv(&self, index: GLuint, type_: VertexAttribPointerType, normalized: GLboolean, value: *const GLuint) {
    (self.glVertexAttribP3uiv_p)(index, type_, normalized, value)
  }
  /// [glVertexAttribP4ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribP4ui.xhtml)
  /// * `type` group: VertexAttribPointerType
  /// * `normalized` group: Boolean
  pub unsafe fn VertexAttribP4ui(&self, index: GLuint, type_: VertexAttribPointerType, normalized: GLboolean, value: GLuint) {
    (self.glVertexAttribP4ui_p)(index, type_, normalized, value)
  }
  /// [glVertexAttribP4uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribP4uiv.xhtml)
  /// * `type` group: VertexAttribPointerType
  /// * `normalized` group: Boolean
  pub unsafe fn VertexAttribP4uiv(&self, index: GLuint, type_: VertexAttribPointerType, normalized: GLboolean, value: *const GLuint) {
    (self.glVertexAttribP4uiv_p)(index, type_, normalized, value)
  }
  /// [glVertexAttribPointer](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexAttribPointer.xhtml)
  /// * `type` group: VertexAttribPointerType
  /// * `normalized` group: Boolean
  /// * `pointer` len: COMPSIZE(size,type,stride)
  pub unsafe fn VertexAttribPointer(&self, index: GLuint, size: GLint, type_: VertexAttribPointerType, normalized: GLboolean, stride: GLsizei, pointer: *const void) {
    (self.glVertexAttribPointer_p)(index, size, type_, normalized, stride, pointer)
  }
  /// [glVertexBindingDivisor](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexBindingDivisor.xhtml)
  pub unsafe fn VertexBindingDivisor(&self, bindingindex: GLuint, divisor: GLuint) {
    (self.glVertexBindingDivisor_p)(bindingindex, divisor)
  }
  /// [glVertexP2ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexP2ui.xhtml)
  /// * `type` group: VertexPointerType
  pub unsafe fn VertexP2ui(&self, type_: VertexPointerType, value: GLuint) {
    (self.glVertexP2ui_p)(type_, value)
  }
  /// [glVertexP2uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexP2uiv.xhtml)
  /// * `type` group: VertexPointerType
  pub unsafe fn VertexP2uiv(&self, type_: VertexPointerType, value: *const GLuint) {
    (self.glVertexP2uiv_p)(type_, value)
  }
  /// [glVertexP3ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexP3ui.xhtml)
  /// * `type` group: VertexPointerType
  pub unsafe fn VertexP3ui(&self, type_: VertexPointerType, value: GLuint) {
    (self.glVertexP3ui_p)(type_, value)
  }
  /// [glVertexP3uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexP3uiv.xhtml)
  /// * `type` group: VertexPointerType
  pub unsafe fn VertexP3uiv(&self, type_: VertexPointerType, value: *const GLuint) {
    (self.glVertexP3uiv_p)(type_, value)
  }
  /// [glVertexP4ui](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexP4ui.xhtml)
  /// * `type` group: VertexPointerType
  pub unsafe fn VertexP4ui(&self, type_: VertexPointerType, value: GLuint) {
    (self.glVertexP4ui_p)(type_, value)
  }
  /// [glVertexP4uiv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glVertexP4uiv.xhtml)
  /// * `type` group: VertexPointerType
  pub unsafe fn VertexP4uiv(&self, type_: VertexPointerType, value: *const GLuint) {
    (self.glVertexP4uiv_p)(type_, value)
  }
  /// [glViewport](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glViewport.xhtml)
  /// * `x` group: WinCoord
  /// * `y` group: WinCoord
  pub unsafe fn Viewport(&self, x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
    (self.glViewport_p)(x, y, width, height)
  }
  /// [glViewportArrayv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glViewportArrayv.xhtml)
  /// * `v` len: COMPSIZE(count)
  pub unsafe fn ViewportArrayv(&self, first: GLuint, count: GLsizei, v: *const GLfloat) {
    (self.glViewportArrayv_p)(first, count, v)
  }
  /// [glViewportIndexedf](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glViewportIndexedf.xhtml)
  pub unsafe fn ViewportIndexedf(&self, index: GLuint, x: GLfloat, y: GLfloat, w: GLfloat, h: GLfloat) {
    (self.glViewportIndexedf_p)(index, x, y, w, h)
  }
  /// [glViewportIndexedfv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glViewportIndexedfv.xhtml)
  pub unsafe fn ViewportIndexedfv(&self, index: GLuint, v: *const [GLfloat; 4]) {
    (self.glViewportIndexedfv_p)(index, v)
  }
  /// [glWaitSync](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glWaitSync.xhtml)
  /// * `sync` group: sync
  /// * `sync` class: sync
  /// * `flags` group: SyncBehaviorFlags
  pub unsafe fn WaitSync(&self, sync: GLsync, flags: GLbitfield, timeout: GLuint64) {
    (self.glWaitSync_p)(sync, flags, timeout)
  }
  /// [glDebugMessageCallbackARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDebugMessageCallbackARB.xhtml)
  /// * `userParam` len: COMPSIZE(callback)
  #[cfg_attr(feature = "track_caller", track_caller)]
  pub unsafe fn DebugMessageCallbackARB(&self, callback: GLDEBUGPROCARB, userParam: *const void) {
    match self.glDebugMessageCallbackARB_p {
      Some(f) => f(callback, userParam),
      None => Self::not_loaded("glDebugMessageCallbackARB"),
    }
  }
  #[doc(hidden)]
  pub fn DebugMessageCallbackARB_is_loaded(&self) -> bool {
    self.glDebugMessageCallbackARB_p.is_some()
  }
  /// [glDebugMessageControlARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDebugMessageControlARB.xhtml)
  /// * `source` group: DebugSource
  /// * `type` group: DebugType
  /// * `severity` group: DebugSeverity
  /// * `ids` len: count
  /// * `enabled` group: Boolean
  #[cfg_attr(feature = "track_caller", track_caller)]
  pub unsafe fn DebugMessageControlARB(&self, source: DebugSource, type_: DebugType, severity: DebugSeverity, count: GLsizei, ids: *const GLuint, enabled: GLboolean) {
    match self.glDebugMessageControlARB_p {
      Some(f) => f(source, type_, severity, count, ids, enabled),
      None => Self::not_loaded("glDebugMessageControlARB"),
    }
  }
  #[doc(hidden)]
  pub fn DebugMessageControlARB_is_loaded(&self) -> bool {
    self.glDebugMessageControlARB_p.is_some()
  }
  /// [glDebugMessageInsertARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDebugMessageInsertARB.xhtml)
  /// * `source` group: DebugSource
  /// * `type` group: DebugType
  /// * `severity` group: DebugSeverity
  /// * `buf` len: length
  #[cfg_attr(feature = "track_caller", track_caller)]
  pub unsafe fn DebugMessageInsertARB(&self, source: DebugSource, type_: DebugType, id: GLuint, severity: DebugSeverity, length: GLsizei, buf: *const GLchar) {
    match self.glDebugMessageInsertARB_p {
      Some(f) => f(source, type_, id, severity, length, buf),
      None => Self::not_loaded("glDebugMessageInsertARB"),
    }
  }
  #[doc(hidden)]
  pub fn DebugMessageInsertARB_is_loaded(&self) -> bool {
    self.glDebugMessageInsertARB_p.is_some()
  }
  /// [glGetDebugMessageLogARB](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetDebugMessageLogARB.xhtml)
  /// * `sources` group: DebugSource
  /// * `sources` len: count
  /// * `types` group: DebugType
  /// * `types` len: count
  /// * `ids` len: count
  /// * `severities` group: DebugSeverity
  /// * `severities` len: count
  /// * `lengths` len: count
  /// * `messageLog` len: bufSize
  #[cfg_attr(feature = "track_caller", track_caller)]
  pub unsafe fn GetDebugMessageLogARB(&self, count: GLuint, bufSize: GLsizei, sources: *mut DebugSource, types: *mut DebugType, ids: *mut GLuint, severities: *mut DebugSeverity, lengths: *mut GLsizei, messageLog: *mut GLchar) -> GLuint {
    match self.glGetDebugMessageLogARB_p {
      Some(f) => f(count, bufSize, sources, types, ids, severities, lengths, messageLog),
      None => Self::not_loaded("glGetDebugMessageLogARB"),
    }
  }
  #[doc(hidden)]
  pub fn GetDebugMessageLogARB_is_loaded(&self) -> bool {
    self.glGetDebugMessageLogARB_p.is_some()
  }
  /// [glDebugMessageCallbackKHR](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDebugMessageCallbackKHR.xhtml)
  #[cfg_attr(feature = "track_caller", track_caller)]
  pub unsafe fn DebugMessageCallbackKHR(&self, callback: GLDEBUGPROCKHR, userParam: *const void) {
    match self.glDebugMessageCallbackKHR_p {
      Some(f) => f(callback, userParam),
      None => Self::not_loaded("glDebugMessageCallbackKHR"),
    }
  }
  #[doc(hidden)]
  pub fn DebugMessageCallbackKHR_is_loaded(&self) -> bool {
    self.glDebugMessageCallbackKHR_p.is_some()
  }
  /// [glDebugMessageControlKHR](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDebugMessageControlKHR.xhtml)
  /// * `source` group: DebugSource
  /// * `type` group: DebugType
  /// * `severity` group: DebugSeverity
  /// * `enabled` group: Boolean
  #[cfg_attr(feature = "track_caller", track_caller)]
  pub unsafe fn DebugMessageControlKHR(&self, source: DebugSource, type_: DebugType, severity: DebugSeverity, count: GLsizei, ids: *const GLuint, enabled: GLboolean) {
    match self.glDebugMessageControlKHR_p {
      Some(f) => f(source, type_, severity, count, ids, enabled),
      None => Self::not_loaded("glDebugMessageControlKHR"),
    }
  }
  #[doc(hidden)]
  pub fn DebugMessageControlKHR_is_loaded(&self) -> bool {
    self.glDebugMessageControlKHR_p.is_some()
  }
  /// [glDebugMessageInsertKHR](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDebugMessageInsertKHR.xhtml)
  /// * `source` group: DebugSource
  /// * `type` group: DebugType
  /// * `severity` group: DebugSeverity
  #[cfg_attr(feature = "track_caller", track_caller)]
  pub unsafe fn DebugMessageInsertKHR(&self, source: DebugSource, type_: DebugType, id: GLuint, severity: DebugSeverity, length: GLsizei, buf: *const GLchar) {
    match self.glDebugMessageInsertKHR_p {
      Some(f) => f(source, type_, id, severity, length, buf),
      None => Self::not_loaded("glDebugMessageInsertKHR"),
    }
  }
  #[doc(hidden)]
  pub fn DebugMessageInsertKHR_is_loaded(&self) -> bool {
    self.glDebugMessageInsertKHR_p.is_some()
  }
  /// [glGetDebugMessageLogKHR](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetDebugMessageLogKHR.xhtml)
  /// * `sources` group: DebugSource
  /// * `sources` len: count
  /// * `types` group: DebugType
  /// * `types` len: count
  /// * `ids` len: count
  /// * `severities` group: DebugSeverity
  /// * `severities` len: count
  /// * `lengths` len: count
  /// * `messageLog` len: bufSize
  #[cfg_attr(feature = "track_caller", track_caller)]
  pub unsafe fn GetDebugMessageLogKHR(&self, count: GLuint, bufSize: GLsizei, sources: *mut DebugSource, types: *mut DebugType, ids: *mut GLuint, severities: *mut DebugSeverity, lengths: *mut GLsizei, messageLog: *mut GLchar) -> GLuint {
    match self.glGetDebugMessageLogKHR_p {
      Some(f) => f(count, bufSize, sources, types, ids, severities, lengths, messageLog),
      None => Self::not_loaded("glGetDebugMessageLogKHR"),
    }
  }
  #[doc(hidden)]
  pub fn GetDebugMessageLogKHR_is_loaded(&self) -> bool {
    self.glGetDebugMessageLogKHR_p.is_some()
  }
  /// [glGetObjectLabelKHR](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetObjectLabelKHR.xhtml)
  /// * `label` len: bufSize
  #[cfg_attr(feature = "track_caller", track_caller)]
  pub unsafe fn GetObjectLabelKHR(&self, identifier: GLenum, name: GLuint, bufSize: GLsizei, length: *mut GLsizei, label: *mut GLchar) {
    match self.glGetObjectLabelKHR_p {
      Some(f) => f(identifier, name, bufSize, length, label),
      None => Self::not_loaded("glGetObjectLabelKHR"),
    }
  }
  #[doc(hidden)]
  pub fn GetObjectLabelKHR_is_loaded(&self) -> bool {
    self.glGetObjectLabelKHR_p.is_some()
  }
  /// [glGetObjectPtrLabelKHR](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetObjectPtrLabelKHR.xhtml)
  /// * `label` len: bufSize
  #[cfg_attr(feature = "track_caller", track_caller)]
  pub unsafe fn GetObjectPtrLabelKHR(&self, ptr: *const void, bufSize: GLsizei, length: *mut GLsizei, label: *mut GLchar) {
    match self.glGetObjectPtrLabelKHR_p {
      Some(f) => f(ptr, bufSize, length, label),
      None => Self::not_loaded("glGetObjectPtrLabelKHR"),
    }
  }
  #[doc(hidden)]
  pub fn GetObjectPtrLabelKHR_is_loaded(&self) -> bool {
    self.glGetObjectPtrLabelKHR_p.is_some()
  }
  /// [glGetPointervKHR](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetPointervKHR.xhtml)
  #[cfg_attr(feature = "track_caller", track_caller)]
  pub unsafe fn GetPointervKHR(&self, pname: GLenum, params: *mut *mut void) {
    match self.glGetPointervKHR_p {
      Some(f) => f(pname, params),
      None => Self::not_loaded("glGetPointervKHR"),
    }
  }
  #[doc(hidden)]
  pub fn GetPointervKHR_is_loaded(&self) -> bool {
    self.glGetPointervKHR_p.is_some()
  }
  /// [glObjectLabelKHR](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glObjectLabelKHR.xhtml)
  /// * `identifier` group: ObjectIdentifier
  #[cfg_attr(feature = "track_caller", track_caller)]
  pub unsafe fn ObjectLabelKHR(&self, identifier: ObjectIdentifier, name: GLuint, length: GLsizei, label: *const GLchar) {
    match self.glObjectLabelKHR_p {
      Some(f) => f(identifier, name, length, label),
      None => Self::not_loaded("glObjectLabelKHR"),
    }
  }
  #[doc(hidden)]
  pub fn ObjectLabelKHR_is_loaded(&self) -> bool {
    self.glObjectLabelKHR_p.is_some()
  }
  /// [glObjectPtrLabelKHR](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glObjectPtrLabelKHR.xhtml)
  #[cfg_attr(feature = "track_caller", track_caller)]
  pub unsafe fn ObjectPtrLabelKHR(&self, ptr: *const void, length: GLsizei, label: *const GLchar) {
    match self.glObjectPtrLabelKHR_p {
      Some(f) => f(ptr, length, label),
      None => Self::not_loaded("glObjectPtrLabelKHR"),
    }
  }
  #[doc(hidden)]
  pub fn ObjectPtrLabelKHR_is_loaded(&self) -> bool {
    self.glObjectPtrLabelKHR_p.is_some()
  }
  /// [glPopDebugGroupKHR](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPopDebugGroupKHR.xhtml)
  #[cfg_attr(feature = "track_caller", track_caller)]
  pub unsafe fn PopDebugGroupKHR(&self) {
    match self.glPopDebugGroupKHR_p {
      Some(f) => f(),
      None => Self::not_loaded("glPopDebugGroupKHR"),
    }
  }
  #[doc(hidden)]
  pub fn PopDebugGroupKHR_is_loaded(&self) -> bool {
    self.glPopDebugGroupKHR_p.is_some()
  }
  /// [glPushDebugGroupKHR](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPushDebugGroupKHR.xhtml)
  /// * `source` group: DebugSource
  #[cfg_attr(feature = "track_caller", track_caller)]
  pub unsafe fn PushDebugGroupKHR(&self, source: DebugSource, id: GLuint, length: GLsizei, message: *const GLchar) {
    match self.glPushDebugGroupKHR_p {
      Some(f) => f(source, id, length, message),
      None => Self::not_loaded("glPushDebugGroupKHR"),
    }
  }
  #[doc(hidden)]
  pub fn PushDebugGroupKHR_is_loaded(&self) -> bool {
    self.glPushDebugGroupKHR_p.is_some()
  }
}
